use glam::{Mat2, Vec2};
use crate::Rigidbody;

/// 2D Pivot joint that constrains world(anchor_a) == world(anchor_b)
#[derive(Clone)]
pub struct PivotJoint {
    pub body_a: usize,
    pub body_b: usize,
    pub a_index: usize,
    pub b_index: usize,
    pub local_anchor_a: Vec2,
    pub local_anchor_b: Vec2,

    // solver state (for warm starting)
    impulse: Vec2,

    // precomputed per-solve step:
    r_a: Vec2,
    r_b: Vec2,
    effective_mass: Mat2, // inverse of K (2x2)
    bias: Vec2,
}

impl PivotJoint {
    pub fn new( world_anchor: Vec2, rigidbodys: &mut Vec<Rigidbody>, body_a: usize, body_b: usize) -> Self {
        let a;
        let b;
        if body_a > body_b {
            let (left, right) = rigidbodys.split_at_mut(body_a);
            b = &mut left[body_b];
            a = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(body_b);
            a = &mut left[body_a];
            b = &mut right[0];
        }
        a.connected_anchors.push(body_b);
        let a_index = a.connected_anchors.len() - 1;
        b.connected_anchors.push(body_a);
        let b_index = b.connected_anchors.len() - 1;
        let local_anchor_a = world_anchor - a.center;
        let local_anchor_b = world_anchor - b.center;
        println!("local_anchor_a: {:?}", local_anchor_a);
        println!("local_anchor_b: {:?}", local_anchor_b);
        println!("world_anchor: {:?}", world_anchor);
        Self {
            body_a,
            body_b,
            a_index,
            b_index,
            local_anchor_a,
            local_anchor_b,
            impulse: Vec2::ZERO,
            r_a: Vec2::ZERO,
            r_b: Vec2::ZERO,
            effective_mass: Mat2::ZERO,
            bias: Vec2::ZERO,
        }
    }

    /// rotate local -> world
    fn rotate(v: Vec2, angle: f32) -> Vec2 {
        let (s, c) = angle.sin_cos();
        Vec2::new(c * v.x - s * v.y, s * v.x + c * v.y)
    }

    /// pre_solve: compute effective mass, bias, warm-start
    /// - dt: timestep
    /// - baumgarte: typically 0.05..0.2 (fraction)
    pub fn pre_solve(&mut self, rigidbodys: &mut Vec<Rigidbody>, dt: f32, baumgarte: f32) {
        let body_a;
        let body_b;
        if self.body_a > self.body_b {
            let (left, right) = rigidbodys.split_at_mut(self.body_a);
            body_b = &mut left[self.body_b];
            body_a = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(self.body_b);
            body_a = &mut left[self.body_a];
            body_b = &mut right[0];
        }

        // r_a/r_b are offsets from COM in world
        self.r_a = Self::rotate(self.local_anchor_a, body_a.angle);
        self.r_b = Self::rotate(self.local_anchor_b, body_b.angle);

        // world anchor positions
        let p_a = body_a.center + self.r_a;
        let p_b = body_b.center + self.r_b;

        // NOTE: use Box2D sign convention: C = pB - pA
        let c = p_b - p_a;

        // mass / inertia
        let m_a = 1.0 / body_a.mass;
        let m_b = 1.0 / body_b.mass;
        let i_a = 1.0 / body_a.inertia;
        let i_b = 1.0 / body_b.inertia;

        // Build K = J * invM * J^T (2x2). See Box2D source for same formula.
        // Using r.x and r.y directly (Box2D uses r.y^2 etc)
        let k11 = m_a + m_b + i_a * self.r_a.y * self.r_a.y + i_b * self.r_b.y * self.r_b.y;
        let k12 = -i_a * self.r_a.x * self.r_a.y - i_b * self.r_b.x * self.r_b.y;
        let k22 = m_a + m_b + i_a * self.r_a.x * self.r_a.x + i_b * self.r_b.x * self.r_b.x;

        // invert 2x2 with regularization if nearly singular
        let det = k11 * k22 - k12 * k12;
        let eps = 1e-9_f32;
        if det.abs() > eps {
            let inv_det = 1.0 / det;
            self.effective_mass = Mat2::from_cols(
                Vec2::new(k22 * inv_det, -k12 * inv_det),
                Vec2::new(-k12 * inv_det, k11 * inv_det),
            );
        } else {
            // small regularizer to avoid dividing by zero (common practice)
            let reg = 1e-6_f32;
            let k11r = k11 + reg;
            let k22r = k22 + reg;
            let det_r = k11r * k22r - k12 * k12;
            let inv_det = 1.0 / det_r.max(eps);
            self.effective_mass = Mat2::from_cols(
                Vec2::new(k22r * inv_det, -k12 * inv_det),
                Vec2::new(-k12 * inv_det, k11r * inv_det),
            );
        }

        // Baumgarte positional bias: bias = -(beta / dt) * C
        let beta = baumgarte.clamp(0.0, 0.2);
        self.bias = -(beta / dt) * c;

        // Warm starting: apply last frame's accumulated impulse
        if self.impulse != Vec2::ZERO {
            // apply -P to A and +P to B (Box2D convention)
            body_a.velocity -= self.impulse * m_a;
            body_a.angular_velocity -= i_a * cross(self.r_a, self.impulse);

            body_b.velocity += self.impulse * m_b;
            body_b.angular_velocity += i_b * cross(self.r_b, self.impulse);
        }
    }

    /// velocity solver: iterative sequential impulse
    pub fn solve_velocity(&mut self, rigidbodys: &mut Vec<Rigidbody>) {
        let body_a;
        let body_b;
        if self.body_a > self.body_b {
            let (left, right) = rigidbodys.split_at_mut(self.body_a);
            body_b = &mut left[self.body_b];
            body_a = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(self.body_b);
            body_a = &mut left[self.body_a];
            body_b = &mut right[0];
        }


        let m_a = 1.0 / body_a.mass;
        let m_b = 1.0 / body_b.mass;
        let i_a = 1.0 / body_a.inertia;
        let i_b = 1.0 / body_b.inertia;
        
        // relative velocity at anchors: Jv = vB + ωB×rB - vA - ωA×rA (Box2D)
        let vel_a_anchor = body_a.velocity + cross_scalar_vec(body_a.angular_velocity, self.r_a);
        let vel_b_anchor = body_b.velocity + cross_scalar_vec(body_b.angular_velocity, self.r_b);
        let rel_vel = vel_b_anchor - vel_a_anchor;

        // solve: P = -M * (Jv + bias)
        let rhs = -(rel_vel + self.bias);
        let p = self.effective_mass * rhs;

        // accumulate impulse (no limits here; you can clamp if needed)
        self.impulse += p;

        // apply impulses: A -= P, B += P  (linear) & angular updates
        body_a.velocity -= p * m_a;
        body_a.angular_velocity -= i_a * cross(self.r_a, p);

        body_b.velocity += p * m_b;
        body_b.angular_velocity += i_b * cross(self.r_b, p);
    }

    /// position correction pass (recompute K here). Returns remaining error magnitude.
    pub fn solve_position(&mut self, rigidbodys: &mut Vec<Rigidbody>) -> f32 {
        let body_a;
        let body_b;
        if self.body_a > self.body_b {
            let (left, right) = rigidbodys.split_at_mut(self.body_a);
            body_b = &mut left[self.body_b];
            body_a = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(self.body_b);
            body_a = &mut left[self.body_a];
            body_b = &mut right[0];
        }

        // recompute offsets (angles might have changed)
        self.r_a = Self::rotate(self.local_anchor_a, body_a.angle);
        self.r_b = Self::rotate(self.local_anchor_b, body_b.angle);

        let p_a = body_a.center + self.r_a;
        let p_b = body_b.center + self.r_b;

        // Box2D convention: C = pB - pA
        let c = p_b - p_a;

        // small tolerance: linear slop (Box2D uses ~0.005)
        const LINEAR_SLOP: f32 = 0.005;
        const MAX_CORRECTION: f32 = 0.2;

        let err = c.length();
        if err <= LINEAR_SLOP {
            return err;
        }

        // clamp the correction to avoid overshoot
        let correction = if err > MAX_CORRECTION {
            -c.normalize() * MAX_CORRECTION
        } else {
            -c
        };

        // recompute K and invert (same formula)
        let m_a = 1.0 / body_a.mass;
        let m_b = 1.0 / body_b.mass;
        let i_a = 1.0 / body_a.inertia;
        let i_b = 1.0 / body_b.inertia;

        let k11 = m_a + m_b + i_a * self.r_a.y * self.r_a.y + i_b * self.r_b.y * self.r_b.y;
        let k12 = -i_a * self.r_a.x * self.r_a.y - i_b * self.r_b.x * self.r_b.y;
        let k22 = m_a + m_b + i_a * self.r_a.x * self.r_a.x + i_b * self.r_b.x * self.r_b.x;

        let det = k11 * k22 - k12 * k12;
        let eps = 1e-9_f32;
        let k_inv = if det.abs() > eps {
            let inv_det = 1.0 / det;
            Mat2::from_cols(
                Vec2::new(k22 * inv_det, -k12 * inv_det),
                Vec2::new(-k12 * inv_det, k11 * inv_det),
            )
        } else {
            let reg = 1e-6_f32;
            let k11r = k11 + reg;
            let k22r = k22 + reg;
            let det_r = k11r * k22r - k12 * k12;
            let inv_det = 1.0 / det_r.max(eps);
            Mat2::from_cols(
                Vec2::new(k22r * inv_det, -k12 * inv_det),
                Vec2::new(-k12 * inv_det, k11r * inv_det),
            )
        };

        // position impulse: p = K^{-1} * correction
        let p = k_inv * correction;

        // apply positional correction: A -= inv_mass * p, angle -= inv_inertia * (rA × p)
        let translation = p * m_a;
        body_a.translate(-translation);
        let rotation = i_a * cross(self.r_a, p);
        body_a.rotate(-rotation);
        body_a.angle -= rotation;

        let translation = p * m_b;
        body_b.translate(translation);
        let rotation = i_b * cross(self.r_b, p);
        body_b.rotate(rotation);
        body_b.angle += rotation;

        err
    }

    pub fn get_anchor_world_position_a(&self, rigidbodys: &Vec<Rigidbody>) -> Vec2 {
        rigidbodys[self.body_a].center + Self::rotate(self.local_anchor_a, rigidbodys[self.a_index].angle )
    }
    pub fn get_anchor_world_position_b(&self, rigidbodys: &Vec<Rigidbody>) -> Vec2 {
        rigidbodys[self.body_b].center + Self::rotate(self.local_anchor_b, rigidbodys[self.b_index].angle )
    }
}

/// 2D cross helpers

/// 2D cross: scalar x vec -> vec (ω × r): (-ω * r.y, ω * r.x)
fn cross_scalar_vec(s: f32, v: Vec2) -> Vec2 {
    Vec2::new(-s * v.y, s * v.x)
}

/// vec x vec -> scalar (a × b)
fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

