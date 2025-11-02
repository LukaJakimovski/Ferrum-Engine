use crate::{Rigidbody, Spring};

#[derive(Default)]
pub struct Energy {
    pub kinetic_energy: f64,
    pub potential_energy: f64,
    pub spring_energy: f64,
}


impl Energy {
    pub fn calculate_kinetic_energy(rigidbody: &Rigidbody) -> f64 {
        let mut kinetic_energy = 0.0;
        kinetic_energy += 0.5 * rigidbody.mass * rigidbody.velocity.dot(rigidbody.velocity);
        kinetic_energy +=
            0.5 * rigidbody.moment_of_inertia * rigidbody.angular_velocity * rigidbody.angular_velocity;
        if kinetic_energy < 0.0 {
            return 0.0;
        }
        kinetic_energy as f64
    }
    pub fn calculate_gravitational_energy(rigidbody: &Rigidbody, gravity: f32, origin: f32) -> f64 {
        let height = origin - rigidbody.center.y;
        rigidbody.mass as f64 * gravity as f64 * rigidbody.gravity_multiplier as f64 * height as f64
    }

    pub fn calculate_spring_energy(spring: &Spring, rigidbodys: &Vec<Rigidbody>) -> f64 {
        let a = &rigidbodys[spring.body_a];
        let b = &rigidbodys[spring.body_b];

        let world_anchor_a = a.center + spring.anchor_a;
        let world_anchor_b = b.center + spring.anchor_b;

        let distance = world_anchor_a.distance(world_anchor_b);
        let stretch = distance - spring.rest_length;

        (0.5 * spring.stiffness * stretch * stretch) as f64
    }

    pub fn update_energy(&mut self, rigidbodys: &Vec<Rigidbody>, springs: &Vec<Spring>, gravity: f32, origin: f32) {
        self.kinetic_energy = 0.0;
        self.spring_energy = 0.0;
        self.potential_energy = 0.0;
        for polygon in rigidbodys {
            self.kinetic_energy += Self::calculate_kinetic_energy(polygon);
            self.potential_energy += Self::calculate_gravitational_energy(polygon, gravity, origin);
        }
        for spring in springs {
            self.spring_energy += Self::calculate_spring_energy(spring, rigidbodys);
        }
    }

    pub fn get_energy(&self) -> f64 {
        self.kinetic_energy + self.spring_energy + self.potential_energy
    }
}

