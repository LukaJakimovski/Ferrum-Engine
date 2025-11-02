use glam::Vec2;
use crate::{Parameters, Rigidbody, Spring};
use crate::energy::Energy;
use crate::pivot_joint::PivotJoint;
use crate::weld_joint::WeldJoint;

pub struct PhysicsSystem {
    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    pub(crate) weld_joints: Vec<WeldJoint>,
    pub(crate) pivot_joints: Vec<PivotJoint>,
    pub dt: f32,
    pub energy: Energy,
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
