use glam::Vec2;
use crate::{Parameters, Rigidbody, Spring};
use crate::energy::Energy;
use crate::pivot_joint::PivotJoint;
use crate::weld_joint::WeldJoint;

//const G: f64 = 6.674 * 0.00000000001;
const G: f64 = 0.0;
pub struct PhysicsSystem {
    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    pub(crate) weld_joints: Vec<WeldJoint>,
    pub(crate) pivot_joints: Vec<PivotJoint>,
    pub dt: f32,
    pub energy: Energy,
}

impl PhysicsSystem {
    pub fn get_gravity(&mut self) {
        for polygon in &mut self.polygons {
            polygon.gravity_force = Vec2::ZERO;
        }
        for i in 0..self.polygons.len() {
            for j in (i + 1)..self.polygons.len() {

                let pos_i = self.polygons[i].center;
                let pos_j = self.polygons[j].center;

                let direction = pos_j - pos_i;
                let distance = direction.length();

                // Prevent division by zero or extremely small distances
                if distance <= 0.0001 {
                    continue;
                }

                let mass_product = self.polygons[i].mass * self.polygons[j].mass;

                // Correct gravity formula: F = G * m1 * m2 / r^2
                let force_magnitude = G as f32 * mass_product / (distance * distance);

                // Normalize direction vector
                let force = direction.normalize() * force_magnitude;

                self.polygons[i].gravity_force += force;
                self.polygons[j].gravity_force -= force;
            }
        }
    }


    pub fn update_physics(&mut self, parameters: &Parameters) {
        self.collision_resolution();
        let g: Vec2;
        if parameters.gravity == true {
            g = parameters.gravity_force;
        } else {
            g = Vec2 { x: 0.0, y: 0.0 };
        }
        for spring in &mut self.springs {
            spring.apply(self.dt, &mut self.polygons);
        }

        self.get_gravity();
        for polygon in &mut self.polygons {
            polygon.update_rigidbody(g, self.dt);
        }
        for weld_joint in &mut self.weld_joints {
            for _ in 0..1 {
                weld_joint.solve_velocity_constraints(&mut self.polygons, self.dt);
            }
        }

        for pivot_joint in &mut self.pivot_joints {
            for _ in 0..1 {
                pivot_joint.solve_velocity_constraints(&mut self.polygons, self.dt);
            }
        }

    }
}
