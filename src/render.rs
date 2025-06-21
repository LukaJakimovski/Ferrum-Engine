use miniquad::{conf, date, start, window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, KeyCode, KeyMods, MouseButton, Pipeline, PipelineParams, RenderingBackend, ShaderSource, UniformsSource, VertexAttribute, VertexFormat};
use crate::shader;
use crate::square::*;

pub struct Stage {
    pub ctx: Box<dyn RenderingBackend>,
    pub pipeline: Pipeline,
    pub bindings: Bindings,
    pub camera_pos: (f32, f32, f32, f32),
    pub polygons: Vec<Polygon>,
    pub pressed_keys: Vec<KeyCode>,
    pub pressed_buttons: [u8; 3],
    pub start_time: f64,
    pub frame_count: u32,
    pub mouse_pos: (f32, f32),
    pub vertex_count: i32,
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
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
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
            camera_pos: (0.0, 0.0, 0.0, -1.0),
            pressed_keys: vec![],
            pressed_buttons: [0, 0, 0],
            start_time,
            frame_count: 0,
            mouse_pos: (0.0, 0.0),
            vertex_count: 0,
        }
    }

    pub fn refresh(&self) -> Self {
        let start_time = date::now();
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let mut indices: Vec<u16> = vec![];
        let mut vertices: Vec<crate::square::Vertex> = vec![];
        let mut start_index: u16 = 0;
        for i in 0..self.polygons.len() {
            let length = self.polygons[i].vertices.len() as u16;
            vertices.extend(self.polygons[i].clone().vertices);

            let mut new_indices = self.polygons[i].clone().indices;
            for i in 0..new_indices.len() {
                new_indices[i] += start_index as u16;
            }
            indices.extend(new_indices);
            start_index += length as u16;
        }

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
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
            pipeline: self.pipeline.clone(),
            bindings: self.bindings.clone(),
            ctx,
            polygons: self.polygons.clone(),
            camera_pos: self.camera_pos,
            pressed_keys: vec![],
            pressed_buttons: self.pressed_buttons,
            start_time: self.start_time,
            frame_count: self.frame_count,
            mouse_pos: self.mouse_pos,
            vertex_count: self.vertex_count,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
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
        if self.frame_count % 60 == 0 {
            let time = date::now() - self.start_time;
            self.start_time = date::now();
            println!("FPS: {}", 60.0 / time);
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
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 1 }
        if _button == MouseButton::Left { 
            self.pressed_buttons[0] = 1;
            self.polygons.push(rectangle(0.5, 0.5, Vec2 {x: self.mouse_pos.0 / window::screen_size().0, y: self.mouse_pos.1 / window::screen_size().1}));
            self.refresh();
            let conf = conf::Conf::default();
            start(conf, move || Box::new(Stage::new(polygons)));
        }
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
        if _keycode == KeyCode::W{ self.camera_pos.1 += 0.03; self.pressed_keys.push(_keycode) }
        if _keycode == KeyCode::S{ self.camera_pos.1 -= 0.03; self.pressed_keys.push(_keycode)}
        if _keycode == KeyCode::D{ self.camera_pos.0 += 0.03; self.pressed_keys.push(_keycode) }
        if _keycode == KeyCode::A{ self.camera_pos.0 -= 0.03; self.pressed_keys.push(_keycode) }
    }
}