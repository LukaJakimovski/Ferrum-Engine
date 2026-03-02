use std::f64::consts::PI;
use glam::{DMat2, DVec2};
use crate::ode_solver::{rk4_angular_step, rk4_step};
use crate::rigidbody::Rigidbody;
use crate::{ColorRGBA};

#[derive(Clone, Debug, Default)]
pub struct Spring {
    pub body_a: usize,
    pub body_b: usize,
    pub connector: Rigidbody,
    pub(crate) anchor_a: DVec2, // Local offset on body A
    pub(crate) anchor_b: DVec2, // Local offset on body B
    pub(crate) rest_length: f32,
    pub(crate) stiffness: f64,
    pub(crate) damping: f64,
}

impl Spring {
    pub fn new(
        body_a: usize,
        body_b: usize,
        anchor_a: DVec2,
        anchor_b: DVec2,
        rest_length: f32,
        stiffness: f64,
        damping: f64,
        rigidbodys: &Vec<Rigidbody>,
    ) -> Self {
        let a = &rigidbodys[body_a];
        let b = &rigidbodys[body_b];

        // Compute world anchors using rotation matrices
        let rot_a = DMat2::from_angle(a.angle);
        let rot_b = DMat2::from_angle(b.angle);
        let world_anchor_a = a.center + rot_a * anchor_a;
        let world_anchor_b = b.center + rot_b * anchor_b;

        let delta = world_anchor_b - world_anchor_a;
        let distance = delta.length();
        let direction = delta.normalize_or_zero();

        // Create the connector body
        let mut connector = Rigidbody::rectangle(
            0.1,
            distance as f32,
            (world_anchor_a + world_anchor_b) * 0.5,
            1.0,
            1.0,
            ColorRGBA::white(),
        );

        // Align connector visually with spring
        let angle = direction.angle_to(DVec2::new(0.0, -1.0));
        connector.rotate(angle as f32);

        Spring {
            body_a,
            body_b,
            connector,
            anchor_a,
            anchor_b,
            rest_length,
            stiffness,
            damping,
        }
    }

    pub fn apply(&mut self, dt: f64, rigidbodys: &mut Vec<Rigidbody>) {
        let (a, b) = {
            let (low, high) = if self.body_a > self.body_b {
                let (left, right) = rigidbodys.split_at_mut(self.body_a);
                (&mut right[0], &mut left[self.body_b])
            } else {
                let (left, right) = rigidbodys.split_at_mut(self.body_b);
                (&mut left[self.body_a], &mut right[0])
            };
            (low, high)
        };

        // --- Compute world-space anchors using rotation matrices ---
        let rot_a = DMat2::from_angle(a.angle);
        let rot_b = DMat2::from_angle(b.angle);
        let world_anchor_a = a.center + rot_a * self.anchor_a;
        let world_anchor_b = b.center + rot_b * self.anchor_b;

        // --- Compute spring physics ---
        let delta = world_anchor_b - world_anchor_a;
        let distance = delta.length();
        let direction = if distance != 0.0 {
            delta / distance
        } else {
            DVec2::new(rand::random::<f64>(), rand::random::<f64>()).normalize()
        };

        let stretch = distance - self.rest_length as f64;

        let vel_a = a.velocity + a.angular_velocity * (world_anchor_a - a.center).perp();
        let vel_b = b.velocity + b.angular_velocity * (world_anchor_b - b.center).perp();
        let relative_velocity = vel_b - vel_a;

        let spring_force = -self.stiffness * stretch * direction;
        let damping_force = -self.damping * relative_velocity.dot(direction) * direction;
        let total_force = spring_force + damping_force;

        // --- Apply forces and torques ---
        let r_a = world_anchor_a - a.center;
        let r_b = world_anchor_b - b.center;
        let torque_a = r_a.perp_dot(-total_force);
        let torque_b = r_b.perp_dot(total_force);

        // Linear motion (RK4)
        let force_a = |_t: f64, _x: DVec2, _v: DVec2| -total_force;
        let force_b = |_t: f64, _x: DVec2, _v: DVec2| total_force;
        let (_pos_a, new_vel_a) = rk4_step(0.0, a.center, a.velocity, dt, a.mass, &force_a);
        let (_pos_b, new_vel_b) = rk4_step(0.0, b.center, b.velocity, dt, b.mass, &force_b);

        a.velocity = new_vel_a;
        b.velocity = new_vel_b;

        // Angular motion (RK4)
        let torque_fn_a = |_t: f64, _theta: f64, _omega: f64| torque_a;
        let torque_fn_b = |_t: f64, _theta: f64, _omega: f64| torque_b;

        let (_angle_a, new_omega_a) =
            rk4_angular_step(0.0, a.angle, a.angular_velocity, dt, a.moment_of_inertia, &torque_fn_a);
        let (_angle_b, new_omega_b) =
            rk4_angular_step(0.0, b.angle, b.angular_velocity, dt, b.moment_of_inertia, &torque_fn_b);

        a.angular_velocity = new_omega_a;
        b.angular_velocity = new_omega_b;
    }

    pub fn update_connector(&mut self, rigidbodys: &Vec<Rigidbody>) {
        let a = &rigidbodys[self.body_a];
        let b = &rigidbodys[self.body_b];

        // Compute rotated world anchors
        let rot_a = DMat2::from_angle(a.angle);
        let rot_b = DMat2::from_angle(b.angle);
        let world_anchor_a = a.center + rot_a * self.anchor_a;
        let world_anchor_b = b.center + rot_b * self.anchor_b;

        let delta = world_anchor_b - world_anchor_a;
        let distance = delta.length();
        let direction = delta.to_angle();

        self.connector = Rigidbody::rectangle(
            0.1,
            distance as f32,
            (world_anchor_a + world_anchor_b) * 0.5,
            1.0,
            1.0,
            ColorRGBA::white(),
        );
        self.connector.rotate((direction + PI / 2.0) as f32);
    }
}