use miniquad::{date, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, KeyCode, KeyMods, MouseButton, Pipeline, PipelineParams, RenderingBackend, ShaderSource, UniformsSource, VertexAttribute, VertexFormat};
use crate::rigidbody::*;
use crate::math::*;
use crate::collision_detection::*;
use crate::shader::{FRAGMENT, VERTEX};
use crate::{shader, Color};
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
    ctx: Box<dyn RenderingBackend>,
    render_object: RenderObject,
    pipeline: Pipeline,

    pub scaling_factor: f32,
    camera_pos: (f32, f32, f32, f32),
    mouse_pos: (f32, f32),
    
    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    previous_polygon_count: usize,
    
    pub collisions: usize,
    
    pub pressed_keys: [u8; 16],
    pub pressed_buttons: [u8; 3],
    
    pub start_time: f64,
    pub delta_time: f64,
    frame_count: u32,

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
            pressed_keys: [0; 16],
            pressed_buttons: [0, 0, 0],
            start_time: date::now(),
            mouse_pos: (0.0, 0.0),
            scaling_factor,
            frame_count: 0,
            previous_polygon_count: 0,
            parameters
        }
    }

    pub fn create_render_object(&mut self, polygons: Vec<Rigidbody>) {
        let mut indices: Vec<u32> = vec![];
        let mut vertices: Vec<Vertex> = vec![];
        let mut start_index: u32 = 0;
        for polygon in polygons.clone() {
            let length = polygon.vertices.len() as u32;
            let color = polygon.vertices[0].color;
            vertices.extend(polygon.vertices);
            vertices.push(Vertex{pos: polygon.center, color});
            let mut new_indices= polygon.indices;
            for i in 0..new_indices.len() {
                new_indices[i] += start_index;
            }
            indices.extend(new_indices);
            start_index += length + 1;
        }
        if self.previous_polygon_count != polygons.len() {
            self.ctx.delete_buffer(self.render_object.bindings.vertex_buffers[0]);
            self.ctx.delete_buffer(self.render_object.bindings.index_buffer);
            let vertex_buffer = self.ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Dynamic,
                BufferSource::slice(&vertices),
            );

            let index_buffer = self.ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Dynamic,
                BufferSource::slice(&indices),
            );

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![],
            };

            self.render_object = RenderObject {
                bindings,
                indices,
            };
            return
        }
        self.ctx.buffer_update(
            self.render_object.bindings.vertex_buffers[0],
            BufferSource::slice(&vertices),
        );

        self.ctx.buffer_update(
            self.render_object.bindings.index_buffer,
            BufferSource::slice(&indices),
        );
        self.previous_polygon_count = polygons.len();
    }

    pub fn render(&mut self){
        self.ctx.begin_default_pass(Default::default());
        let mut render_polygon: Vec<Rigidbody> = self.polygons.clone();
        self.create_render_object(render_polygon);
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.render_object.bindings);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                camera_pos: self.camera_pos,
                aspect_ratio: window::screen_size().0 / window::screen_size().1,
            }));
        self.ctx.draw(0, self.render_object.clone().indices.len() as i32, 1);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
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

        if self.pressed_keys[4] == 1 {
            for i in 0..self.polygons.len() {
                self.polygons[i].angular_velocity = 5.0;
            }
        }

        if self.pressed_keys[0] == 1 {self.camera_pos.1 += 5.0 * self.delta_time as f32;}
        if self.pressed_keys[1] == 1 {self.camera_pos.0 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[2] == 1 {self.camera_pos.1 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[3] == 1 {self.camera_pos.0 += 5.0 * self.delta_time as f32;}
        if self.pressed_keys[5] == 1 {
            for _i in 0..self.parameters.updates_per_frame {
                self.update_physics();
            }
        }
        //println!("Collisions: {}", self.collisions);

        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - window::screen_size().0)/ window::screen_size().0 + self.camera_pos.0 / (-self.camera_pos.3 + 1.0)) * (-self.camera_pos.3 + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - window::screen_size().1)/ window::screen_size().1 + self.camera_pos.1 / -(-self.camera_pos.3 + 1.0)) * -(-self.camera_pos.3 + 1.0) * window::screen_size().1 / window::screen_size().0,};
        if self.pressed_keys[7] == 1 {
            self.polygons.push(Rigidbody::polygon(16, 0.3533, position.clone()));
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

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - window::screen_size().0)/ window::screen_size().0 + self.camera_pos.0 / (-self.camera_pos.3 + 1.0)) * (-self.camera_pos.3 + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - window::screen_size().1)/ window::screen_size().1 + self.camera_pos.1 / -(-self.camera_pos.3 + 1.0)) * -(-self.camera_pos.3 + 1.0) * window::screen_size().1 / window::screen_size().0,};
        if _button == MouseButton::Left {
            self.pressed_buttons[0] = 1;
            self.polygons.push(Rigidbody::polygon(16, 0.3533, position.clone()));
            let length = self.polygons.len();
            self.polygons[length - 1].restitution = 0.95;
        }
        if _button == MouseButton::Right {
            self.pressed_buttons[1] = 1;
            let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position);
            for i in 0..self.polygons.len() {
                let result = sat_collision(&self.polygons[i], &mouse_polygon);
                if result[1].y != 0.0{
                    self.polygons.remove(i);
                    break;
                }
            }
        }
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 1 }
    }
    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 0 }
        if _button == MouseButton::Right { self.pressed_buttons[1] = 0 }
        if _button == MouseButton::Left { self.pressed_buttons[0] = 0 }
    }
    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match _keycode {
            KeyCode::Key1 => window::show_mouse(false),
            KeyCode::Key2 => window::show_mouse(true),
            _ => (),
        }
        if _keycode == KeyCode::W{self.pressed_keys[0] = 1 }
        if _keycode == KeyCode::A{self.pressed_keys[1] = 1 }
        if _keycode == KeyCode::S{self.pressed_keys[2] = 1 }
        if _keycode == KeyCode::D{self.pressed_keys[3] = 1 }
        if _keycode == KeyCode::R{self.pressed_keys[4] = 1 }
        if _keycode == KeyCode::P{self.pressed_keys[5] = 1 }
        if _keycode == KeyCode::LeftControl || _keycode == KeyCode::RightControl { self.pressed_keys[6] = 1 }
        if _keycode == KeyCode::L{self.pressed_keys[7] = 1 }
    }
    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        if _keycode == KeyCode::W{self.pressed_keys[0] = 0 }
        if _keycode == KeyCode::A{self.pressed_keys[1] = 0 }
        if _keycode == KeyCode::S{self.pressed_keys[2] = 0}
        if _keycode == KeyCode::D{self.pressed_keys[3] = 0 }
        if _keycode == KeyCode::R{self.pressed_keys[4] = 0 }
        //if _keycode == KeyCode::P{self.pressed_keys[5] = 0 }
        if _keycode == KeyCode::LeftControl || _keycode == KeyCode::RightControl { self.pressed_keys[6] = 0 }
        if _keycode == KeyCode::L{self.pressed_keys[7] = 0 }
    }

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {
        if self.pressed_buttons[2] == 1 {
            self.camera_pos.0 -= _dx * (-self.camera_pos.3 + 1.0) / window::screen_size().0;
            self.camera_pos.1 += _dy *  (-self.camera_pos.3 + 1.0) / window::screen_size().1;
        }
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        if self.pressed_keys[6] == 1 {
            self.scaling_factor += _y * 0.1;
            self.scaling_factor += _x * 0.1;
            println!("Scaling factor: {}", self.scaling_factor);
        }
        else {
            self.camera_pos.3 += _y * 0.1 * self.scaling_factor;
            self.camera_pos.3 += _x * 0.1 * self.scaling_factor;
        }
    }
}