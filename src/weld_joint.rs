use glam::{Vec2, Vec3, Mat3};
use crate::Rigidbody;

#[derive(Clone)]
pub struct WeldJoint {
    body_a: usize,
    body_b: usize,
    local_anchor_a: Vec2,
    local_anchor_b: Vec2,
    reference_angle: f32,

    pub beta: f32,
}

impl WeldJoint {
    pub fn new(local_anchor_a: Vec2, local_anchor_b: Vec2, rigidbodys: &mut Vec<Rigidbody>, body_a: usize, body_b: usize) -> Self {
        let a;
        let b;
        if body_a > body_b {
            let (left, right) = rigidbodys.split_at_mut(body_a);
            a = &mut left[body_b];
            b = &mut right[0];
        } else {
            let (left, right) = rigidbodys.split_at_mut(body_b);
            a = &mut left[body_a];
            b = &mut right[0];
        }
        a.connected_anchors.push(body_b);
        b.connected_anchors.push(body_a);
        
        let reference_angle = b.angle - a.angle;
        Self {
            body_a,
            body_b,
            local_anchor_a,
            local_anchor_b,
            reference_angle,
            beta: 0.05,
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

        // Jacobian builds constraints:
        // Cdot = J * v
        // v = [va, wa, vb, wb]
        //
        // We collapse this into a 3x3 effective mass matrix.

        // relative velocity at anchors
        let va_anchor = a.velocity + Vec2::new(-a.angular_velocity * ra.y, a.angular_velocity * ra.x);
        let vb_anchor = b.velocity + Vec2::new(-b.angular_velocity * rb.y, b.angular_velocity * rb.x);
        let v_rel = vb_anchor - va_anchor;
        let w_rel = b.angular_velocity - a.angular_velocity;

        // Bias term (Baumgarte)
        let pa = a.center + ra;
        let pb = b.center + rb;
        let pos_err = pb - pa;
        let ang_err = (b.angle - a.angle) - self.reference_angle;

        let bias_lin = (self.beta / dt) * pos_err;
        let bias_ang = (self.beta / dt) * ang_err;

        // Effective mass matrix (3x3)
        // [ M11  M12  M13 ]
        // [ M21  M22  M23 ]
        // [ M31  M32  M33 ]

        // Row/col ordering: [linear.x, linear.y, angular]

        // Start with zeros
        let mut k = Mat3::ZERO;

        // Linear terms
        k.x_axis.x = inv_ma + inv_mb;
        k.y_axis.y = inv_ma + inv_mb;

        // Rotational contributions from anchors
        // In 2D, r x f = perp_dot(r, f)
        k.x_axis.x += inv_ia * ra.y * ra.y + inv_ib * rb.y * rb.y;
        k.x_axis.y += -inv_ia * ra.x * ra.y - inv_ib * rb.x * rb.y;
        k.y_axis.x = k.x_axis.y;
        k.y_axis.y += inv_ia * ra.x * ra.x + inv_ib * rb.x * rb.x;

        // Cross terms with angular constraint
        k.z_axis.x = -inv_ia * ra.y - inv_ib * rb.y;
        k.z_axis.y = inv_ia * ra.x + inv_ib * rb.x;
        k.x_axis.z = k.z_axis.x;
        k.y_axis.z = k.z_axis.y;

        // Angular constraint diagonal
        k.z_axis.z = inv_ia + inv_ib;

        // Now invert K
        let k_inv = k.inverse();

        // Build constraint velocity error vector
        let c_dot = Vec3::new(v_rel.x + bias_lin.x,
                              v_rel.y + bias_lin.y,
                              w_rel + bias_ang);

        // Solve for impulses
        let lambda = -k_inv * c_dot;

        // Apply impulses
        let lin_impulse = Vec2::new(lambda.x, lambda.y);
        let ang_impulse = lambda.z;

        a.velocity -= lin_impulse * inv_ma;
        a.angular_velocity -= inv_ia * (ra.perp_dot(lin_impulse) + ang_impulse);

        b.velocity += lin_impulse * inv_mb;
        b.angular_velocity += inv_ib * (rb.perp_dot(lin_impulse) + ang_impulse);
    }
}
