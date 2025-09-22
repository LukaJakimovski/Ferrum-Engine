use glam::{Mat2, Vec2};
use crate::Rigidbody;
use crate::utility::rotate;

/// 2D Ball-and-Socket (pivot/pin) joint: constrains anchors to coincide, allows free rotation.
#[derive(Clone)]
pub struct PivotJoint {
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    pub(crate) body_a: usize,
    pub(crate) body_b: usize,
    start_angle: f32,
    pub a_index: usize,
    pub b_index: usize,

    /// Baumgarte stabilization factor for positional drift (small: 0.01..0.2)
    pub beta: f32,
}

impl PivotJoint {
    pub fn new(local_anchor_a: Vec2, local_anchor_b: Vec2, rigidbodys: &mut Vec<Rigidbody>, body_a: usize, body_b: usize) -> Self {
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
        
        Self {
            local_anchor_a,
            local_anchor_b,
            body_a,
            body_b,
            a_index,
            b_index,
            start_angle: a.angle,
            beta: 0.12,
        }
    }

    /// Solve velocity-level linear constraints (2D block solve).
    /// Call multiple times per physics step inside the solver iteration loop.
    pub fn solve_velocity_constraints(&self, rigidbodys: &mut Vec<Rigidbody>, dt: f32) {
        let a;
        let b;
        if self.body_a > self.body_b {
            let (left, right) = rigidbodys.split_at_mut(self.body_a);
            b = &mut left[self.body_b];
            a = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(self.body_b);
            a = &mut left[self.body_a];
            b = &mut right[0];
        }
        // world-space anchor offsets
        let ra = a.rotation_matrix() * self.local_anchor_a;
        let rb = b.rotation_matrix() * self.local_anchor_b;

        let inv_ma = 1.0 / a.mass;
        let inv_mb = 1.0 / b.mass;
        let inv_ia = 1.0 / a.moment_of_inertia;
        let inv_ib = 1.0 / b.moment_of_inertia;

        // relative velocity at anchors
        let va_anchor = a.velocity + Vec2::new(-a.angular_velocity * ra.y, a.angular_velocity * ra.x);
        let vb_anchor = b.velocity + Vec2::new(-b.angular_velocity * rb.y, b.angular_velocity * rb.x);
        let v_rel = vb_anchor - va_anchor;

        // position error and bias (Baumgarte)
        let pa = a.center + ra;
        let pb = b.center + rb;
        let pos_err = pb - pa;
        let bias = (self.beta / dt) * pos_err; // 2-vector

        // Build 2x2 effective mass matrix K = J M^{-1} J^T
        // Base linear terms:
        let mut k = Mat2::from_diagonal(Vec2::splat(inv_ma + inv_mb));

        // rotational contributions: for 2D, these are scalar but produce 2x2 additions.
        // For a given ra, the term is inv_ia * ([ -ra.y; ra.x ] * [ -ra.y, ra.x ]) = inv_ia * (ra_perp * ra_perp^T)
        // where ra_perp = perpendicular vector = (-ra.y, ra.x)
        let ra_perp = Vec2::new(-ra.y, ra.x);
        let rb_perp = Vec2::new(-rb.y, rb.x);

        k += Mat2::from_cols(ra_perp * ra_perp.x * inv_ia, ra_perp * ra_perp.y * inv_ia);
        k += Mat2::from_cols(rb_perp * rb_perp.x * inv_ib, rb_perp * rb_perp.y * inv_ib);

        // threshold for singularity
        const EPS: f32 = 1e-6;
        let rhs = -(v_rel + bias);

        // matrix entries (Mat2 stores columns as x_axis, y_axis)
        let a_k = k.x_axis.x;
        let b_k = k.y_axis.x;
        let c = k.x_axis.y;
        let d = k.y_axis.y;

        // determinant
        let det = a_k * d - b_k * c;

        let lambda = if det.abs() > EPS {
            // Cramer's rule (explicit solve)
            let inv_det = 1.0 / det;
            Vec2::new(
                ( d * rhs.x - b_k * rhs.y) * inv_det,
                (-c * rhs.x + a_k * rhs.y) * inv_det,
            )
        } else {
            // fallback: diagonal approximation (safe)
            let diag_a = a_k;
            let diag_d = d;
            let mut lx = 0.0;
            let mut ly = 0.0;
            if diag_a.abs() > EPS { lx = rhs.x / diag_a; }
            if diag_d.abs() > EPS { ly = rhs.y / diag_d; }
            Vec2::new(lx, ly)
        };

        // Apply linear impulse and corresponding angular effect
        // impulse is applied as +lambda to B, -lambda to A
        let impulse = lambda;

        a.velocity -= impulse * inv_ma;
        // torque change = r x F = perp_dot(r, F)
        a.angular_velocity -= inv_ia * ra.perp_dot(impulse);

        b.velocity += impulse * inv_mb;
        b.angular_velocity += inv_ib * rb.perp_dot(impulse);
    }

    pub fn get_anchor_world_position(&self, rigidbodys: &Vec<Rigidbody>) -> Vec2 {
        rigidbodys[self.body_a].center + rotate(self.local_anchor_a, Vec2::ZERO, rigidbodys[self.body_a].angle - self.start_angle)
    }
}