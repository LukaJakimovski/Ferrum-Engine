use crate::{ColorRGBA};
use glam::{Vec2, Vec4};
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
    pub gravity_force: Vec2,
    pub clear_color: ColorRGBA,
    pub is_running: bool,
    pub initial_camera: Camera,
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

    pub(crate) fn update(&mut self) {
        if self.parameters.delta_time == 0.0 {
            let mut dt = Timing::now() - self.timing.start_time;
            self.timing.start_time = Timing::now();
            dt *= self.parameters.time_multiplier as f64;
            self.physics.dt = dt as f32;
        } else {
            self.physics.dt = self.parameters.delta_time as f32;
        }
        for spring in &mut self.physics.springs {
            spring.update_connector(&mut self.physics.polygons);
        }
        
        if self.ui.is_pointer_used == false {
            self.ui.handle_input(&mut self.physics, &mut self.color_system);
        }
        
        for i in 0..self.physics.polygons.len() {
            if i < self.physics.polygons.len()
                && self.parameters.world_size > 0.0
                && self.physics.polygons[i].center.distance(Vec2::ZERO) > self.parameters.world_size
                && self.physics.polygons[i].eternal == false
            {
                self.physics.remove_rigidbody(i, &mut self.ui);
            }
        }
        if self.parameters.is_running == true {
            for _i in 0..self.parameters.updates_per_frame {
                self.physics.update_physics(&self.parameters);
            }
        }
        self.timing.frame_count += 1;
        if self.timing.frame_count % 10 == 0 {
            self.timing.fps = 10.0 / (Timing::now() - self.timing.timer);
            self.timing.timer = Timing::now();
        }

        self.physics.energy.update_energy(&self.physics.polygons, &self.physics.springs, self.parameters.gravity_force.y,  -self.parameters.world_size);

        self.ui.create_mouse_ghost(&mut self.physics);
    }
}
