use miniquad::{window, Bindings, BufferSource, BufferType, BufferUsage, UniformsSource};
use crate::{shader, RenderObject, Rigidbody, Vertex, World};

impl World{
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