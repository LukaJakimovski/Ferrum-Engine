use miniquad::{BufferId, date, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, KeyCode, KeyMods, MouseButton, Pipeline, PipelineParams, RenderingBackend, ShaderSource, UniformsSource, VertexAttribute, VertexFormat};
use crate::shader;
use crate::square::*;
use crate::math::*;
use crate::collision_detection::*;

pub struct Stage {
    pub ctx: Box<dyn RenderingBackend>,
    pub pipeline: Pipeline,
    pub bindings: Bindings,
    pub camera_pos: (f32, f32, f32, f32),
    pub polygons: Vec<Polygon>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub pressed_keys: [u8; 16],
    pub pressed_buttons: [u8; 3],
    pub frame_count: u32,
    pub mouse_pos: (f32, f32),
    pub vertex_count: i32,
    pub start_time: f64,
    pub delta_time: f64,
    pub index_buffers: Vec<BufferId>,
    pub vertex_buffers: Vec<BufferId>,
}
impl Stage {
    pub fn new(polygons: Vec<Polygon>) -> Self {
        let start_time = date::now();
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

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

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Dynamic,
            BufferSource::slice(&vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Dynamic,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::VERTEX,
                    fragment: shader::FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
            polygons,
            delta_time: 0.0,
            camera_pos: (0.0, 0.0, 0.0, -1.0),
            pressed_keys: [0; 16],
            pressed_buttons: [0, 0, 0],
            start_time,
            frame_count: 0,
            mouse_pos: (0.0, 0.0),
            vertex_count: 0,
            vertices,
            indices,
            index_buffers: vec![index_buffer],
            vertex_buffers: vec![vertex_buffer],
        }
    }

    pub fn add_polygon(&mut self, polygon: Polygon) {
        self.polygons.push(polygon);
        let current_indices = self.vertices.len() as u16;
        let mut new_indices = self.polygons[self.polygons.len() - 1].indices.clone();
        for i in 0..new_indices.len() {
            new_indices[i] += current_indices;
        }
        self.indices.extend(new_indices);
        self.vertices.extend(self.polygons[self.polygons.len() - 1].vertices.clone());
    }
    pub fn refresh(&mut self) {
        self.ctx.delete_buffer(self.bindings.vertex_buffers[0]);
        self.ctx.delete_buffer(self.bindings.index_buffer);
        self.bindings.vertex_buffers[0] = self.ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Dynamic,
            BufferSource::slice(&self.vertices),
        );
        self.bindings.index_buffer = self.ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Dynamic,
            BufferSource::slice(&self.indices),
        );
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.refresh();
        self.delta_time = date::now() - self.start_time;
        self.start_time = date::now();
        
        for i in 0..self.polygons.len() {
            for j in 0..self.polygons.len(){
                if i != j {
                    let result = sat_collision(&self.polygons[i], &self.polygons[j]);
                    if result[1].y == 1.0{
                        println!("Collision!");
                    }
                };
            }
        }

        if self.pressed_keys[0] == 1 {self.camera_pos.1 += 5.0 * self.delta_time as f32;}
        if self.pressed_keys[1] == 1 {self.camera_pos.0 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[2] == 1 {self.camera_pos.1 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[3] == 1 {self.camera_pos.0 += 5.0 * self.delta_time as f32;}

        self.vertex_count = 0;
        for polygon in self.polygons.clone() {
            self.vertex_count += polygon.indices.len() as i32;
        }
        self.ctx.begin_default_pass(Default::default());
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                camera_pos: self.camera_pos
            }));

        self.ctx.draw(0, self.vertex_count, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
        self.frame_count += 1;
        if self.frame_count % 165 == 0 {

            println!("Frame time: {}ms", self.delta_time * 1000.0);
            println!("Polygons: {}", self.polygons.len());
        }
    }

    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {
        if self.pressed_buttons[2] == 1 {
            self.camera_pos.0 -= _dx * 2.0 / window::screen_size().0;
            self.camera_pos.1 += _dy * 2.0 / window::screen_size().1;
        }
    }

    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {
        self.mouse_pos = (_x, _y);
    }
    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        if _button == MouseButton::Left {
            self.pressed_buttons[0] = 1;
            let position = Vec2 {x: ((self.mouse_pos.0 * 2.0 - window::screen_size().0)/ window::screen_size().0 + self.camera_pos.0 / 2.0) * -self.camera_pos.3 * 2.0, y: ((self.mouse_pos.1 * 2.0 - window::screen_size().1)/ window::screen_size().1 + self.camera_pos.1 / -2.0) * self.camera_pos.3 * 2.0};
            self.add_polygon(rectangle(0.5, 0.5, position));
        }
        if _button == MouseButton::Right { 
            self.pressed_buttons[1] = 1;
            let position = Vec2 {x: ((self.mouse_pos.0 * 2.0 - window::screen_size().0)/ window::screen_size().0 + self.camera_pos.0 / 2.0) * -self.camera_pos.3 * 2.0, y: ((self.mouse_pos.1 * 2.0 - window::screen_size().1)/ window::screen_size().1 + self.camera_pos.1 / -2.0) * self.camera_pos.3 * 2.0};
            let mouse_polygon = rectangle(0.01, 0.01, position);
            for i in 0..self.polygons.len() {
                let result = sat_collision(&self.polygons[i], &mouse_polygon);
                if result[1].y == 1.0{
                    self.polygons.remove(i);
                    self.indices = vec![];
                    self.vertices = vec![];
                    let mut start_index: u16 = 0;
                    for polygon in self.polygons.clone() {
                        let length = polygon.vertices.len() as u16;
                        self.vertices.extend(polygon.vertices);

                        let mut new_indices = polygon.indices;
                        for i in 0..new_indices.len() {
                            new_indices[i] += start_index as u16;
                        }
                        self.indices.extend(new_indices);
                        start_index += length as u16;
                    }
                    break
                }
            }
            self.refresh();
        }
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 1 }
    }
    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 0 }
        if _button == MouseButton::Left { self.pressed_buttons[0] = 0 }
    }
    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, mut _repeat: bool) {
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
}