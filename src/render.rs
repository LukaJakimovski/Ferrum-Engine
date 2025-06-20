use miniquad::{window, Bindings, BufferLayout, BufferSource, BufferType, BufferUsage, EventHandler, PipelineParams, RenderingBackend, ShaderSource, UniformsSource, VertexAttribute, VertexFormat};
use crate::shader;
use crate::square::*;

impl Stage {
    pub fn new(polygons: Vec<Polygon>) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let mut indices: Vec<u16> = vec![];
        let mut vertices: Vec<crate::square::Vertex> = vec![];
        let mut start_index: u16 = 0;
        for polygon in polygons {
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
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                camera_pos: (0.0, 0.0, 0.0, -1.0)
            }));

        self.ctx.draw(0, 15, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}