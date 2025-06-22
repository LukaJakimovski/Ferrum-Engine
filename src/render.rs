use miniquad::{date, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, KeyCode, KeyMods, MouseButton, Pipeline, PipelineParams, RenderingBackend, ShaderSource, UniformsSource, VertexAttribute, VertexFormat};
use crate::shader;
use crate::square::*;
use crate::math::*;
use crate::collision_detection::*;
use crate::color::Color;
use crate::shader::{FRAGMENT, VERTEX};

pub struct RenderObject {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
    pub indices: Vec<u16>,
}

pub struct World {
    pub ctx: Box<dyn RenderingBackend>,
    pub camera_pos: (f32, f32, f32, f32),
    pub polygons: Vec<Polygon>,
    pub colliding_polygons: Vec<Polygon>,
    pub pressed_keys: [u8; 16],
    pub pressed_buttons: [u8; 3],
    pub mouse_pos: (f32, f32),
    pub start_time: f64,
    pub delta_time: f64,
}
impl World {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        World {
            ctx: window::new_rendering_backend(),
            polygons,
            colliding_polygons: vec![],
            delta_time: 0.0,
            camera_pos: (0.0, 0.0, 0.0, -2.0),
            pressed_keys: [0; 16],
            pressed_buttons: [0, 0, 0],
            start_time: date::now(),
            mouse_pos: (0.0, 0.0),
        }
    }

    pub fn create_render_object(&mut self, vertex_shader: &&str, fragment_shader: &&str, polygons: Vec<Polygon>) -> RenderObject {
        let mut indices: Vec<u16> = vec![];
        let mut vertices: Vec<crate::square::Vertex> = vec![];
        let mut start_index: u16 = 0;
        for polygon in polygons.clone() {
            let length = polygon.vertices.len() as u16;
            vertices.extend(polygon.vertices);

            let mut new_indices = polygon.indices;
            for i in 0..new_indices.len() {
                new_indices[i] += start_index as u16;
            }
            indices.extend(new_indices);
            start_index += length as u16;
        }
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

        let shader = self.ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: vertex_shader,
                    fragment: fragment_shader,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = self.ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
            ],
            shader,
            PipelineParams::default(),
        );

        // Pipeline Bindings Indices
        RenderObject {
            pipeline,
            bindings,
            indices,
        }
    }

    pub fn render(&mut self){
        self.ctx.begin_default_pass(Default::default());
        let mut render_polygon: Vec<Polygon> = self.polygons.clone();
        render_polygon.extend(self.colliding_polygons.clone());
        let render_object = self.create_render_object(&VERTEX, &FRAGMENT, render_polygon);
        self.ctx.apply_pipeline(&render_object.pipeline);
        self.ctx.apply_bindings(&render_object.bindings);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                camera_pos: self.camera_pos
            }));
        self.ctx.draw(0, render_object.indices.len() as i32, 1);
        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}

impl EventHandler for World {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.delta_time = date::now() - self.start_time;
        self.start_time = date::now();

        for i in 0..self.polygons.len() {
            for j in 0..self.polygons.len(){
                if i != j {
                    let result = sat_collision(&self.polygons[i], &self.polygons[j]);
                    if result[1].y == 1.0{
                        self.colliding_polygons.push(self.polygons[i].clone());
                        let vertex_count = self.colliding_polygons[self.colliding_polygons.len() - 1].vertices.len();
                        let length = self.colliding_polygons.len();
                        for k in 0..vertex_count {
                            self.colliding_polygons[length - 1].vertices[k].color = Color::red();
                        }
                    }
                };
            }
        }
        if self.pressed_keys[0] == 1 {self.camera_pos.1 += 5.0 * self.delta_time as f32;}
        if self.pressed_keys[1] == 1 {self.camera_pos.0 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[2] == 1 {self.camera_pos.1 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[3] == 1 {self.camera_pos.0 += 5.0 * self.delta_time as f32;}

        self.render();

        self.colliding_polygons.clear();
    }

    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {
        self.mouse_pos = (_x, _y);
    }

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - window::screen_size().0)/ window::screen_size().0 + self.camera_pos.0 / (-self.camera_pos.3 + 1.0)) * (-self.camera_pos.3 + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - window::screen_size().1)/ window::screen_size().1 + self.camera_pos.1 / -(-self.camera_pos.3 + 1.0)) * -(-self.camera_pos.3 + 1.0)};
        if _button == MouseButton::Left {
            self.pressed_buttons[0] = 1;
            self.polygons.push(Polygon::rectangle(0.5, 0.5, position.clone()));
        }
        if _button == MouseButton::Right {
            self.pressed_buttons[1] = 1;
            let mouse_polygon = Polygon::rectangle(0.01, 0.01, position);
            for i in 0..self.polygons.len() {
                let result = sat_collision(&self.polygons[i], &mouse_polygon);
                if result[1].y == 1.0{
                    self.polygons.remove(i);
                    break;
                }
            }
        }
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 1 }
    }
    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 0 }
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
        if _keycode == KeyCode::S{self.pressed_keys[2] = 1}
        if _keycode == KeyCode::D{self.pressed_keys[3] = 1 }

    }
    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        if _keycode == KeyCode::W{self.pressed_keys[0] = 0 }
        if _keycode == KeyCode::A{self.pressed_keys[1] = 0 }
        if _keycode == KeyCode::S{self.pressed_keys[2] = 0}
        if _keycode == KeyCode::D{self.pressed_keys[3] = 0 }
    }

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {
        if self.pressed_buttons[2] == 1 {
            self.camera_pos.0 -= _dx * (-self.camera_pos.3 + 1.0) / window::screen_size().0;
            self.camera_pos.1 += _dy *  (-self.camera_pos.3 + 1.0) / window::screen_size().1;
        }
    }

    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {
        self.camera_pos.3 += _y * 0.3;
        self.camera_pos.3 += _x * 0.3;
    }
}