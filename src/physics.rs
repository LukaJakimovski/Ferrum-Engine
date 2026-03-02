use glam::{DVec2, Vec2};
use crate::{Parameters, Rigidbody, Spring};
use crate::energy::Energy;
use crate::ode_solver::{dormand_prince_step};
use crate::pivot_joint::PivotJoint;
use crate::weld_joint::WeldJoint;

//const G: f64 = 6.674 * 0.00000000001;
pub struct PhysicsSystem {
    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    pub(crate) weld_joints: Vec<WeldJoint>,
    pub(crate) pivot_joints: Vec<PivotJoint>,
    pub dt: f64,
    pub energy: Energy,
}

impl PhysicsSystem {
    pub fn calculate_gravitational_energy(rigidbodys: &Vec<Rigidbody>, g: f64) -> f64{
        let mut potential = 0.0;
        for i in 0..rigidbodys.len() {
            for j in (i + 1)..rigidbodys.len() {
                let r = rigidbodys[j].center - rigidbodys[i].center;
                let distance = r.length();
                potential -= g * rigidbodys[i].mass * rigidbodys[j].mass / distance * rigidbodys[i].gravity_multiplier * rigidbodys[j].gravity_multiplier;
            }
        }
        potential
    }

    pub fn gravity_step(&mut self, g: f64){
        let snapshot = self.polygons.clone();
        let mut next_bodies: Vec<Rigidbody> = Vec::with_capacity(self.polygons.len());

        for i in 0..self.polygons.len() {
            let p = &self.polygons[i];

            let compute_accel = |dt_offset: f64, my_pos: DVec2, _my_vel: DVec2| {
                let mut accel = DVec2::ZERO;
                for (j, other) in snapshot.iter().enumerate() {
                    if i == j {continue; }

                    let other_pos_at_t = other.center + other.velocity * dt_offset;

                    let diff = other_pos_at_t - my_pos;
                    let dist_sq = diff.length_squared() + 1e-14;
                    accel = accel + diff * (g * other.gravity_multiplier * other.mass / (dist_sq * dist_sq.sqrt()));
                }
                accel
            };
            // Inside your loop
            let (new_pos, new_vel) = dormand_prince_step(0.0, p.center, p.velocity, self.dt, p.mass, &compute_accel);
            //self.dt = suggested_dt; // Update global simulation speed based on need
            let mut p1 = p.clone();
            p1.move_to(new_pos);
            p1.velocity = new_vel;
            next_bodies.push(p1);
        }
        self.polygons = next_bodies;
    }

    pub fn update_physics(&mut self, parameters: &Parameters) {
        self.collision_resolution();
        let g: DVec2;
        if parameters.gravity == true {
            g = parameters.gravity_force;
        } else {
            g = DVec2 { x: 0.0, y: 0.0 };
        }
        for spring in &mut self.springs {
            spring.apply(self.dt, &mut self.polygons);
        }

        //self.get_gravity(parameters.gravitational_constant);
        self.gravity_step(parameters.gravitational_constant);
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
