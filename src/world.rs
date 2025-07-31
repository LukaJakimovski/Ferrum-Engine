use miniquad::{date, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, KeyCode, KeyMods, MouseButton, Pipeline, PipelineParams, RenderingBackend, ShaderSource, VertexAttribute, VertexFormat};
use crate::rigidbody::*;
use crate::math::*;
use crate::shader::{FRAGMENT, VERTEX};
use crate::{shader};
use crate::spring::*;

#[derive(Clone)]
pub struct RenderObject {
    pub bindings: Bindings,
    pub indices: Vec<u32>,
}

pub struct Parameters{
    pub delta_time: f32,
    pub updates_per_frame: u32,
    pub angular_velocity: bool,
    pub camera_pos: (f32, f32, f32, f32),
    pub gravity: bool,
    pub world_size: f32,
}
pub struct World {
    pub(crate) ctx: Box<dyn RenderingBackend>,
    pub(crate) render_object: RenderObject,
    pub(crate) pipeline: Pipeline,

    pub scaling_factor: f32,
    pub(crate) camera_pos: (f32, f32, f32, f32),
    pub(crate) mouse_pos: (f32, f32),
    
    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    pub(crate) previous_polygon_count: usize,
    
    pub collisions: usize,
    
    pub pressed_keys: [u8; 64],
    pub pressed_buttons: [u8; 3],
    
    pub start_time: f64,
    pub delta_time: f64,
    frame_count: u32,
    pub is_running: bool,

    pub parameters: Parameters,

}
impl World {
    pub fn new(polygons: Vec<Rigidbody>,
               springs: Vec<Spring>,
               parameters: Parameters,
                ) -> Self {
        let mut ctx = window::new_rendering_backend();
        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: &VERTEX,
                    fragment: &FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float3),
            ],
            shader,
            PipelineParams::default(),
        );
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vec![0]),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vec![0]),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };
        
        #[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu"))]
        let scaling_factor = 0.1;
        #[cfg(not(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu")))]
        let scaling_factor = 10.0;
        World {
            ctx,
            pipeline,
            polygons,
            render_object: RenderObject {bindings, indices: vec![]},
            springs,
            collisions: 0,
            delta_time: 0.0,
            camera_pos: parameters.camera_pos,
            pressed_keys: [0; 64],
            pressed_buttons: [0, 0, 0],
            start_time: date::now(),
            mouse_pos: (0.0, 0.0),
            scaling_factor,
            frame_count: 0,
            previous_polygon_count: 0,
            parameters,
            is_running: false,
        }
    }
}

impl EventHandler for World {
    fn update(&mut self) {
        if self.parameters.delta_time == 0.0 {
            self.delta_time = date::now() - self.start_time;
            self.start_time = date::now()
        }
        else{
            self.delta_time = self.parameters.delta_time as f64;
        }

        self.handle_input();

        if self.is_running == true{
            for _i in 0..self.parameters.updates_per_frame {
                self.update_physics();
            }
        }

        for i in 0..self.polygons.len() {
            if self.polygons[i].center.distance(&Vec2::zero()) > self.parameters.world_size {
                self.polygons.remove(i);
                break;
            }
        }

        self.frame_count += 1;
    }

    fn draw(&mut self) {
        self.render();
    }

    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {
        self.mouse_pos = (_x, _y);
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.mouse_wheel_eventhandler(_x, _y);
    }
    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.mouse_button_down_eventhandler(_button, _x, _y);
    }
    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.mouse_button_up_eventhandler(_button, _x, _y);
    }
    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.key_down_eventhandler(_keycode, _keymods, _repeat);
    }

    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        self.key_up_eventhandler(_keycode, _keymods);
    }

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {
        self.raw_mouse_motionhandler(_dx, _dy);
    }
}