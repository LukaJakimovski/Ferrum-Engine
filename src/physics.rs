use glam::Vec2;
use crate::{Parameters, Rigidbody, Spring};
use crate::pivot_joint::PivotJoint;
use crate::weld_joint::WeldJoint;

pub struct PhysicsSystem {
    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    pub(crate) weld_joints: Vec<WeldJoint>,
    pub(crate) pivot_joints: Vec<PivotJoint>,
    pub dt: f32,
    pub total_energy: f64,

}

impl PhysicsSystem {
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

        for weld_joint in &mut self.weld_joints {
            for _ in 0..50 {
                weld_joint.solve_velocity_constraints(&mut self.polygons, self.dt);
            }
        }

        for pivot_joint in &mut self.pivot_joints {
            for _ in 0..50 {
                pivot_joint.solve_velocity_constraints(&mut self.polygons, self.dt);
            }
        }

        for polygon in &mut self.polygons {
            polygon.update_rigidbody(g, self.dt);
        }
    }
}
