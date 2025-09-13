use std::f32::consts::PI;
use glam::Vec2;
use crate::ode_solver::{rk4_angular_step, rk4_step};
use crate::rigidbody::Rigidbody;
use crate::{ColorRGBA};
use crate::utility::rotate_in_place;

#[derive(Clone, Debug, Default)]
pub struct Spring {
    pub body_a: usize, // Index or ID of first body
    pub body_b: usize, // Index or ID of second body
    pub connector: Rigidbody,
    pub(crate) anchor_a: Vec2,            // Local offset on body A
    pub(crate) anchor_b: Vec2, // Local offset on body B
    angle_a: f32,
    angle_b: f32,
    pub(crate) rest_length: f32,
    pub(crate) stiffness: f32,
    pub(crate) damping: f32,
}

impl Spring {
    pub fn new(
        body_a: usize,
        body_b: usize,
        anchor_a: Vec2,
        anchor_b: Vec2,
        rest_length: f32,
        stiffness: f32,
        damping: f32,
        rigidbodys: &Vec<Rigidbody>,
    ) -> Self {
        let a = &rigidbodys[body_a];
        let b = &rigidbodys[body_b];

        // Rotate anchors into world space
        let world_anchor_a = a.center + anchor_a;
        let world_anchor_b = b.center + anchor_b;

        let delta = world_anchor_b - world_anchor_a;
        let distance = world_anchor_a.distance(world_anchor_b);

        let direction = delta / distance;

        let mut connector = Rigidbody::rectangle(
            0.1,
            distance,
            Vec2::new(
                (world_anchor_a.x + world_anchor_b.x) / 2.0,
                (world_anchor_a.y + world_anchor_b.y) / 2.0,
            ),
            1.0,
            1.0,
            ColorRGBA::white(),
        );
        let angle = direction.angle_to(Vec2::new(0.0, -1.0));
        connector.rotate(angle);
        let angle_a = 0.0;
        let angle_b = 0.0;

        Spring {
            body_a,
            body_b,
            connector,
            anchor_a,
            anchor_b,
            angle_a,
            angle_b,
            rest_length,
            stiffness,
            damping,
        }
    }
    pub fn apply(&mut self, dt: f32, rigidbodys: &mut Vec<Rigidbody>) {
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

        // Rotate anchors into world space
        let world_anchor_a = a.center + self.anchor_a;
        let world_anchor_b = b.center + self.anchor_b;

        let delta = world_anchor_b - world_anchor_a;
        let distance = world_anchor_a.distance(world_anchor_b);
        let direction;
        if distance == 0.0 {
            direction = Vec2::new(rand::random::<f32>(), rand::random::<f32>()).normalize();
        } else {
            direction = delta / distance;
        }
        let stretch = distance - self.rest_length;

        let vel_a = a.velocity + a.angular_velocity * (world_anchor_a - a.center).perp();
        let vel_b = b.velocity + b.angular_velocity * (world_anchor_b - b.center).perp();
        let relative_velocity = vel_b - vel_a;

        let spring_force = -self.stiffness * stretch * direction;
        let damping_force = -self.damping * relative_velocity.dot(direction) * direction;
        let total_force = spring_force + damping_force;

        // Torques
        let r_a = world_anchor_a - a.center;
        let r_b = world_anchor_b - b.center;
        let torque_a = r_a.perp_dot(-total_force);
        let torque_b = r_b.perp_dot(total_force);

        // Step linear motion using RK4
        let force_a = move |_t: f32, _x: Vec2, _v: Vec2| -> Vec2 {
            -total_force
        };

        let force_b = move |_t: f32, _x: Vec2, _v: Vec2| -> Vec2 {
            total_force
        };

        let (_new_pos_a, new_vel_a) = rk4_step(0.0, a.center, a.velocity, dt, a.mass, &force_a);
        let (_new_pos_b, new_vel_b) = rk4_step(0.0, b.center, b.velocity, dt, b.mass, &force_b);

        a.velocity = new_vel_a;
        b.velocity = new_vel_b;

        // Step angular motion using RK4
        let torque_fn_a = move |_t: f32, _theta: f32, _omega: f32| -> f32 {
            torque_a
        };
        let torque_fn_b = move |_t: f32, _theta: f32, _omega: f32| -> f32 {
            torque_b
        };

        let (new_angle_a, new_omega_a) = rk4_angular_step(
            0.0,
            a.angle,
            a.angular_velocity,
            dt,
            a.moment_of_inertia,
            &torque_fn_a,
        );
        let (new_angle_b, new_omega_b) = rk4_angular_step(
            0.0,
            b.angle,
            b.angular_velocity,
            dt,
            b.moment_of_inertia,
            &torque_fn_b,
        );

        a.angular_velocity = new_omega_a;
        let diff = new_angle_a - self.angle_a;
        self.angle_a = new_angle_a;
        rotate_in_place(&mut self.anchor_a, Vec2::ZERO, diff);

        b.angular_velocity = new_omega_b;
        let diff = new_angle_b - self.angle_b;
        self.angle_b = new_angle_b;
        rotate_in_place(&mut self.anchor_b, Vec2::ZERO, diff);
    }

    pub fn update_connector(&mut self, rigidbodys: &Vec<Rigidbody>) {
        let a = &rigidbodys[self.body_a];
        let b = &rigidbodys[self.body_b];
        let world_anchor_a = a.center + self.anchor_a;
        let world_anchor_b = b.center + self.anchor_b;

        let delta = world_anchor_b - world_anchor_a;
        let distance = delta.length();
        let direction = delta.to_angle();

        self.connector = Rigidbody::rectangle(
            0.1,
            distance,
            Vec2::new(
                (world_anchor_a.x + world_anchor_b.x) / 2.0,
                (world_anchor_a.y + world_anchor_b.y) / 2.0,
            ),
            1.0,
            1.0,
            ColorRGBA::white(),
        );
        self.connector.rotate(direction + PI / 2.0);
    }

    pub fn calculate_energy(&self, rigidbodys: &Vec<Rigidbody>) -> f64 {
        let a = &rigidbodys[self.body_a];
        let b = &rigidbodys[self.body_b];

        let world_anchor_a = a.center + self.anchor_a;
        let world_anchor_b = b.center + self.anchor_b;

        let distance = world_anchor_a.distance(world_anchor_b);
        let stretch = distance - self.rest_length;

        (0.5 * self.stiffness * stretch * stretch) as f64
    }
}
