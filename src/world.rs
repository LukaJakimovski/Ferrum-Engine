use crate::{ColorRGBA};
use glam::{DVec2, Vec2, Vec4};
use crate::color::ColorSystem;
use crate::input::UiSystem;
use crate::physics::PhysicsSystem;
use crate::render::RenderSystem;
use crate::timing::Timing;

#[derive(Clone)]
pub struct Camera {
    pub camera_pos: Vec4,
    pub scaling_factor: f32,
}
#[derive(Clone)]
pub struct Parameters {
    pub delta_time: f64,
    pub updates_per_frame: u32,
    pub time_multiplier: f32,
    pub angular_velocity: bool,
    pub gravity: bool,
    pub world_size: f32,
    pub gravity_force: DVec2,
    pub clear_color: ColorRGBA,
    pub is_running: bool,
    pub initial_camera: Camera,
    pub gravitational_constant: f64,
}

pub struct World {
    pub render: RenderSystem,
    pub timing: Timing,
    pub parameters: Parameters,
    pub physics: PhysicsSystem,
    pub color_system: ColorSystem,
    pub ui: UiSystem,
}

impl World {
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.render.config.width = width;
            self.render.config.height = height;
            self.render.surface.configure(&self.render.device, &self.render.config);
            self.render.is_surface_configured = true;
        }
    }

    pub(crate) fn physics_update(physics: &mut PhysicsSystem, timing: &mut Timing, parameters: &Parameters) {

        if parameters.delta_time == 0.0 {
            let mut dt = Timing::now() - timing.start_time;
            timing.start_time = Timing::now();
            dt *= parameters.time_multiplier as f64;
            physics.dt = dt;
        } else {
            physics.dt = parameters.delta_time;
        }

        for spring in &mut physics.springs {
            spring.update_connector(&mut physics.polygons);
        }

        if parameters.is_running == true { 
            physics.update_physics(&parameters);
        }
        physics.energy.update_energy(&physics.polygons, &physics.springs, parameters);
    }

    pub(crate) fn update(&mut self) {
        for _i in 0..self.parameters.updates_per_frame {
            World::physics_update(&mut self.physics, &mut self.timing, &self.parameters);
        }


        if self.parameters.is_running {
            if self.parameters.delta_time != 0.0 {
                self.timing.runtime += self.parameters.delta_time * self.parameters.updates_per_frame as f64;
            } else {
                let dt = Timing::now() - self.timing.timer;
                self.timing.runtime += dt * self.parameters.time_multiplier as f64;
                self.timing.frame_count += 1;
            }
        }

        self.timing.fps = 1.0 / (Timing::now() - self.timing.timer);
        self.timing.timer = Timing::now();
        if self.timing.frame_count >= 1000000{
            self.parameters.is_running = false;
        }


        if self.ui.is_pointer_used == false {
            self.ui.handle_input(&mut self.physics, &mut self.color_system);
        }

        for i in 0..self.physics.polygons.len() {
            if i < self.physics.polygons.len()
                && self.parameters.world_size > 0.0
                && self.physics.polygons[i].center.distance(DVec2::ZERO) > self.parameters.world_size as f64
                && self.physics.polygons[i].eternal == false
            {
                self.physics.remove_rigidbody(i, &mut self.ui);
            }
        }
        
        self.ui.create_mouse_ghost(&mut self.physics);
    }
    
    
}
