use glam::{Vec2, Mat2};
use crate::Rigidbody;

#[derive(Clone)]
pub struct WeldJoint {
    body_a: usize,
    body_b: usize,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    reference_angle: f32,

    pub beta: f32,
    pub solver_iters: usize,
}

impl WeldJoint {
    pub fn new(local_anchor_a: Vec2, local_anchor_b: Vec2, rigidbodys: &Vec<Rigidbody>, body_a: usize, body_b: usize) -> Self {
        let a = &rigidbodys[body_a];
        let b = &rigidbodys[body_b];
        let reference_angle = b.angle - a.angle;
        Self {
            body_a,
            body_b,
            local_anchor_a,
            local_anchor_b,
            reference_angle,
            beta: 0.2,
            solver_iters: 10,
        }
    }

    pub fn solve_velocity_constraints(&self, rigidbodys: &mut Vec<Rigidbody>, dt: f32) {
        let a;
        let b;
        if self.body_a > self.body_b {
            let (left, right) = rigidbodys.split_at_mut(self.body_a);
            a = &mut left[self.body_b];
            b = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(self.body_b);
            a = &mut left[self.body_a];
            b = &mut right[0];
        }

        let ra = a.rotation_matrix() * self.local_anchor_a;
        let rb = b.rotation_matrix() * self.local_anchor_b;

        let inv_ma = 1.0 / a.mass;
        let inv_mb = 1.0 / b.mass;
        let inv_ia = 1.0 / a.moment_of_inertia;
        let inv_ib = 1.0 / b.moment_of_inertia;

        // constraint errors
        let pa = a.center + ra;
        let pb = b.center + rb;
        let c_pos = pb - pa;

        let c_ang = (b.angle - a.angle) - self.reference_angle;

        // Baumgarte stabilization
        let bias_pos = (self.beta / dt) * c_pos;
        let bias_ang = (self.beta / dt) * c_ang;

        for _ in 0..self.solver_iters {
            // ---- linear 2D constraint (2x2) ----
            let va_anchor = a.velocity + Vec2::new(-a.angular_velocity * ra.y, a.angular_velocity * ra.x);
            let vb_anchor = b.velocity + Vec2::new(-b.angular_velocity * rb.y, b.angular_velocity * rb.x);
            let v_rel = vb_anchor - va_anchor;

            // effective mass matrix for linear constraint
            let mut k = Mat2::from_diagonal(Vec2::splat(inv_ma + inv_mb));

            // add rotational terms: r x invI x r^T
            // In 2D, torque response is scalar, so add scalar terms
            let ra_skew = Mat2::from_cols(Vec2::new(0.0, -ra.x), Vec2::new(ra.y, 0.0));
            let rb_skew = Mat2::from_cols(Vec2::new(0.0, -rb.x), Vec2::new(rb.y, 0.0));

            k += ra_skew * ra_skew.transpose() * inv_ia;
            k += rb_skew * rb_skew.transpose() * inv_ib;

            let rhs = -(v_rel + bias_pos);
            let lambda_lin = k.inverse() * rhs;

            // apply impulses
            a.velocity -= lambda_lin * inv_ma;
            a.angular_velocity -= inv_ia * ra.perp_dot(lambda_lin);
            b.velocity += lambda_lin * inv_mb;
            b.angular_velocity += inv_ib * rb.perp_dot(lambda_lin);

            // ---- angular 1D constraint ----
            let w_rel = b.angular_velocity - a.angular_velocity;
            let k_ang = inv_ia + inv_ib;
            if k_ang > 0.0 {
                let lambda_ang = -(w_rel + bias_ang) / k_ang;
                a.angular_velocity -= inv_ia * lambda_ang;
                b.angular_velocity += inv_ib * lambda_ang;
            }
        }
    }

    /// Optional position projection
    pub fn positional_correction(&self, rigidbodys: &mut Vec<Rigidbody>) {
        let a;
        let b;
        if self.body_a > self.body_b {
            let (left, right) = rigidbodys.split_at_mut(self.body_a);
            a = &mut left[self.body_b];
            b = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(self.body_b);
            a = &mut left[self.body_a];
            b = &mut right[0];
        }

        let ra = a.rotation_matrix() * self.local_anchor_a;
        let rb = b.rotation_matrix() * self.local_anchor_b;

        let pa = a.center + ra;
        let pb = b.center + rb;
        let c_pos = pb - pa;
        let c_ang = (b.angle - a.angle) - self.reference_angle;

        let k_pos = 0.2;
        let k_ang = 0.2;

        let inv_ma = 1.0 / a.mass;
        let inv_mb = 1.0 / b.mass;
        let inv_ia = 1.0 / a.moment_of_inertia;
        let inv_ib = 1.0 / b.moment_of_inertia;

        let inv_m_total = inv_ma + inv_mb;
        if inv_m_total > 0.0 {
            let corr = (k_pos / inv_m_total) * c_pos;
            a.translate(-corr * inv_ma);
                b.translate(corr * inv_mb);
        }

        a.angle -= k_ang * c_ang * (inv_ia / (inv_ia + inv_ib));
        a.rotate(-k_ang * c_ang * (inv_ia / (inv_ia + inv_ib)));
        b.angle += k_ang * c_ang * (inv_ib / (inv_ia + inv_ib));
        b.rotate(k_ang * c_ang * (inv_ib / (inv_ia + inv_ib)));
    }
}
