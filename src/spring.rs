use crate::math::*;
use crate::ode_solver::{rk4_angular_step, rk4_step};
use crate::rigidbody::Rigidbody;

#[derive(Clone)] #[derive(Default)]
pub struct Spring {
    pub body_a: Rigidbody,         // Index or ID of first body
    pub body_b: Rigidbody,         // Index or ID of second body
    pub connector: Rigidbody,
    anchor_a: Vec2,  // Local offset on body A
    anchor_b: Vec2,  // Local offset on body B
    rest_length: f32,
    stiffness: f32,
    damping: f32,
}

impl Spring {
    pub fn new(body_a: Rigidbody, body_b: Rigidbody, anchor_a: Vec2, anchor_b: Vec2, rest_length: f32, stiffness: f32, damping: f32) -> Self {
        let a = &body_a;
        let b = &body_b;

        // Rotate anchors into world space
        let world_anchor_a = a.center + anchor_a;
        let world_anchor_b = b.center + anchor_b;

        let distance = world_anchor_a.distance(&world_anchor_b);

        let connector = Rigidbody::rectangle(0.1, distance, Vec2::new((world_anchor_a.x + world_anchor_b.x) / 2.0, (world_anchor_a.y + world_anchor_b.y) / 2.0));
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
    pub fn apply(&mut self, dt: f32) {
        let a = &mut self.body_a;
        let b = &mut self.body_b;

        // Rotate anchors into world space
        let world_anchor_a = a.center + self.anchor_a;
        let world_anchor_b = b.center + self.anchor_b;
        
        let delta = world_anchor_b - world_anchor_a;
        let distance = world_anchor_a.distance(&world_anchor_b);

        let direction = delta / distance;

        self.connector = Rigidbody::rectangle(0.01, distance, Vec2::new((world_anchor_a.x + world_anchor_b.x) / 2.0, (world_anchor_a.y + world_anchor_b.y) / 2.0));
        let angle = direction.angle(&Vec2::new(0.0, -1.0));
        if (direction.x < 0.0 && direction.y < 0.0){
            self.connector.rotate(-angle);
        }
        else{
            self.connector.rotate(angle);
        }


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
        let force_a = move |_t: f32, _x: Vec2, _v: Vec2| -> Vec2 {
            -spring_force - damping_force // constant during step
        };

        let force_b = move |_t: f32, _x: Vec2, _v: Vec2| -> Vec2 {
            spring_force + damping_force
        };

        let (new_pos_a, new_vel_a) = rk4_step(0.0, a.center, a.velocity, dt, a.mass, &force_a);
        let (new_pos_b, new_vel_b) = rk4_step(0.0, b.center, b.velocity, dt, b.mass, &force_b);
        
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

        let (new_angle_a, new_omega_a) = rk4_angular_step(0.0, a.angle, a.angular_velocity, dt, a.moment_of_inertia, &torque_fn_a);
        let (new_angle_b, new_omega_b) = rk4_angular_step(0.0, b.angle, b.angular_velocity, dt, b.moment_of_inertia, &torque_fn_b);
        
        let old_angle_a = a.angle;
        a.angle = new_angle_a;
        let diff = new_angle_a - old_angle_a;
        a.rotate(new_angle_a - old_angle_a);
        a.angular_velocity = new_omega_a;
        self.anchor_a.rotate(&Vec2::zero(), diff);
        
        let old_angle_b = b.angle;
        b.angle = new_angle_b;
        let diff = new_angle_b - old_angle_b;
        b.rotate(new_angle_b - old_angle_b);
        b.angular_velocity = new_omega_b;
        self.anchor_b.rotate(&Vec2::zero(), diff);
    }

}