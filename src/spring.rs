use crate::color::Color;
use crate::math::*;
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
            rest_length: 1.0,
            stiffness: 10.0,
            damping: 5.0,
        }
    }
    pub fn apply(&mut self) {
        let a = &self.body_a;
        let b = &self.body_b;

        let world_anchor_a = a.center + self.local_anchor_a;
        let world_anchor_b = b.center + self.local_anchor_b;
        
        self.connector = Rigidbody::rectangle(0.1, (world_anchor_a.y - world_anchor_b.y).abs(), Vec2::new((world_anchor_a.x + world_anchor_b.x) / 2.0, (world_anchor_a.y + world_anchor_b.y) / 2.0));

        let delta = world_anchor_b - world_anchor_a;
        let distance = delta.magnitude();
        if distance == 0.0 { return; }

        let direction = delta.normalized();
        let stretch = distance - self.rest_length;

        let spring_force = direction * (-self.stiffness * stretch);
        let relative_velocity = b.velocity - a.velocity;
        let damping_force = direction * (-self.damping * relative_velocity.dot(&direction));
        let total_force = spring_force + damping_force;

        // Apply forces
        self.body_a.torque -= (world_anchor_a - a.center).cross(&total_force);
        self.body_b.torque += (world_anchor_b - b.center).cross(&total_force);
        
        self.body_a.force -= total_force;
        self.body_b.force += total_force;
    }
}