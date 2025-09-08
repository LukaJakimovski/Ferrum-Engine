use crate::body_builder::{BodyBuilder, SpringParams};
use crate::egui_tools::EguiRenderer;
use crate::enums::{DraggingState, InputMode};
use crate::spring::Spring;
use crate::utility::date;
use crate::{ColorRGBA, Rigidbody};
use egui_wgpu::wgpu;
use std::sync::Arc;
use glam::{Vec2, Vec4};
use winit::window::Window;
use crate::render::{Uniforms, Vertex};

#[derive(Clone)]
pub struct Parameters {
    pub delta_time: f64,
    pub updates_per_frame: u32,
    pub angular_velocity: bool,
    pub camera_pos: Vec4,
    pub gravity: bool,
    pub world_size: f32,
    pub gravity_force: Vec2,
    pub time_multiplier: f32,
    pub clear_color: ColorRGBA,
}
pub struct World {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub window: Arc<Window>,

    pub vertices: Vec<Vertex>,
    pub uniforms: Uniforms,
    pub camera_pos: Vec4,
    pub uniforms_buffer: wgpu::Buffer,
    pub uniforms_bind_group: wgpu::BindGroup,
    pub scaling_factor: f32,
    pub mouse_pos: (f32, f32),

    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    pub previous_polygon_count: usize,

    pub collisions: usize,

    pub pressed_keys: [u8; 64],
    pub pressed_buttons: [u8; 3],

    pub start_time: f64,
    pub delta_time: f64,
    pub(crate) frame_count: u32,
    pub fps: f64,
    pub is_running: bool,
    pub total_energy: f64,

    pub parameters: Parameters,
    pub timer: f64,
    pub egui_renderer: EguiRenderer,
    pub is_pointer_used: bool,

    pub menus: [bool; 16],
    pub spawn_parameters: BodyBuilder,

    pub input_mode: InputMode,
    pub selected_polygon: Option<usize>,
    pub selected_spring: Option<usize>,
    pub spring_polygon: Option<usize>,
    pub mouse_spring: Option<usize>,
    pub spawn_ghost_polygon: Option<usize>,
    pub anchor_pos: Vec2,
    pub dragging: DraggingState,
    pub drag_params: SpringParams,
    pub colors: Option<Vec<ColorRGBA>>
}

impl World {
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub(crate) fn update(&mut self) {
        if self.parameters.delta_time == 0.0 {
            self.delta_time = date::now() - self.start_time;
            self.start_time = date::now();
            self.delta_time *= self.parameters.time_multiplier as f64;
        } else {
            self.delta_time = self.parameters.delta_time;
        }
        for spring in &mut self.springs {
            spring.update_connector(&mut self.polygons);
        }
        
        if self.is_pointer_used == false {
            self.handle_input();
        }
        
        for i in 0..self.polygons.len() {
            if i < self.polygons.len()
                && self.parameters.world_size > 0.0
                && self.polygons[i].center.distance(Vec2::ZERO) > self.parameters.world_size
                && self.polygons[i].eternal == false
            {
                self.remove_rigidbody(i);
            }
        }
        if self.is_running == true {
            for _i in 0..self.parameters.updates_per_frame {
                self.update_physics();
            }
        }
        self.frame_count += 1;
        if self.frame_count % 10 == 0 {
            self.fps = 10.0 / (date::now() - self.timer);
            self.timer = date::now();
        }

        self.total_energy = 0.0;
        for polygon in &self.polygons {
            self.total_energy += polygon.calculate_energy();
        }
        for spring in &mut self.springs {
            self.total_energy += spring.calculate_energy(&self.polygons);
        }
        self.create_mouse_ghost();
    }
}
