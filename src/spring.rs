use crate::color::Color;
use crate::math::*;
use crate::ode_solver::{rk4_angular_step, rk4_step};
use crate::rigidbody::Rigidbody;

#[derive(Clone)] #[derive(Default)]
pub struct Spring {
    pub body_a: Rigidbody,         // Index or ID of first body
    pub body_b: Rigidbody,         // Index or ID of second body
    pub connector: Rigidbody,
    local_anchor_a: Vec2,  // Local offset on body A
    local_anchor_b: Vec2,  // Local offset on body B
    rest_length: f32,
    stiffness: f32,
    damping: f32,
}

impl Spring {
    pub fn new() -> Self {
        Spring {
            body_a: Rigidbody::rectangle(0.5, 0.5, Vec2::new(0.0, 5.0)),
            body_b: Rigidbody::rectangle(0.5, 0.5, Vec2::new(0.0, 0.0)),
            connector: Rigidbody::rectangle(0.1, 4.5, Vec2::new(0.0, 2.5)),
            local_anchor_a: Vec2::new(0.0, -0.25),
            local_anchor_b: Vec2::new(0.0, 0.25),
            rest_length: 10.0,
            stiffness: 1000.0,
            damping: 0.1,
        }
    }
    pub fn apply(&mut self, t: f32, dt: f32) {
        let a = &mut self.body_a;
        let b = &mut self.body_b;

        // Rotate anchors into world space
        let world_anchor_a = a.center + self.local_anchor_a;
        let world_anchor_b = b.center + self.local_anchor_b;
        
        let delta = world_anchor_b - world_anchor_a;
        let distance = world_anchor_a.distance(&world_anchor_b);

        self.connector = Rigidbody::rectangle(0.1, distance, Vec2::new((world_anchor_a.x + world_anchor_b.x) / 2.0, (world_anchor_a.y + world_anchor_b.y) / 2.0));


        let direction = delta / distance;
        let stretch = distance - self.rest_length;

        let relative_velocity = b.velocity - a.velocity;
        let spring_force = -self.stiffness * stretch * direction;
        let damping_force = -self.damping * relative_velocity.dot(&direction) * direction;
        let total_force = spring_force + damping_force;

        // Torques
        let r_a = world_anchor_a - a.center;
        let r_b = world_anchor_b - b.center;
        let torque_a = r_a.cross(&-total_force);
        let torque_b = r_b.cross(&total_force);

        // Step linear motion using RK4
        let force_a = move |t: f32, x: Vec2, v: Vec2| -> Vec2 {
            -spring_force - damping_force // constant during step
        };

        let force_b = move |t: f32, x: Vec2, v: Vec2| -> Vec2 {
            spring_force + damping_force
        };

        let (new_pos_a, new_vel_a) = rk4_step(t, a.center, a.velocity, dt, a.mass, &force_a);
        let (new_pos_b, new_vel_b) = rk4_step(t, b.center, b.velocity, dt, b.mass, &force_b);
        
        a.velocity = new_vel_a;

        let diff = new_pos_a - a.center;
        a.translate(diff);
        
        b.velocity = new_vel_b;

        let diff = new_pos_b - b.center;
        b.translate(diff);

        // Step angular motion using RK4
        let torque_fn_a = move |_t: f32, _theta: f32, _omega: f32| -> f32 {
            torque_a // constant during dt
        };
        let torque_fn_b = move |_t: f32, _theta: f32, _omega: f32| -> f32 {
            torque_b
        };

        let (new_angle_a, new_omega_a) = rk4_angular_step(t, a.angle, a.angular_velocity, dt, a.moment_of_inertia, &torque_fn_a);
        let (new_angle_b, new_omega_b) = rk4_angular_step(t, b.angle, b.angular_velocity, dt, b.moment_of_inertia, &torque_fn_b);

        a.angle = new_angle_a;
        a.angular_velocity = new_omega_a;

        b.angle = new_angle_b;
        b.angular_velocity = new_omega_b;

    }

}