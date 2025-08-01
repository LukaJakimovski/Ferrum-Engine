use miniquad::{window, Bindings, BufferSource, BufferType, BufferUsage, UniformsSource};
use crate::{shader, RenderObject, Vertex, World};

impl World{
    pub fn create_render_object(&mut self) {
        let mut indices: Vec<u32> = vec![];
        let mut vertices: Vec<Vertex> = vec![];
        let mut start_index: u32 = 0;
        for polygon in &self.polygons {
            let length = polygon.vertices.len() as u32;
            let color = polygon.vertices[0].color;
            vertices.extend(polygon.vertices.clone());
            vertices.push(Vertex{pos: polygon.center, color});
            let mut new_indices= polygon.indices.clone();
            for i in 0..new_indices.len() {
                new_indices[i] += start_index;
            }
            indices.extend(new_indices);
            start_index += length + 1;
        }
        for spring in &self.springs {
            let length = spring.connector.vertices.len() as u32;
            let color = spring.connector.vertices[0].color;
            vertices.extend(spring.connector.vertices.clone());
            vertices.push(Vertex{pos: spring.connector.center, color});
            let mut new_indices= spring.connector.indices.clone();
            for i in 0..new_indices.len() {
                new_indices[i] += start_index;
            }
            indices.extend(new_indices);
            start_index += length + 1;
        }

        if self.previous_polygon_count != self.polygons.len() {
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
        self.previous_polygon_count = self.polygons.len();
    }

    pub fn render(&mut self){
        self.ctx.begin_default_pass(miniquad::PassAction::clear_color(0.0, 0.0, 0.0, 1.0));
        self.create_render_object();
        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.render_object.bindings);
        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                camera_pos: self.camera_pos,
                aspect_ratio: window::screen_size().0 / window::screen_size().1,
            }));
        self.ctx.draw(0, self.render_object.clone().indices.len() as i32, 1);
        self.ctx.end_render_pass();

        // Run the UI code:
        self.egui.run(&mut *self.ctx, |_mq_ctx, egui_ctx|{
            egui::Window::new("Display Window").show(egui_ctx, |ui| {
                ui.heading("Hello UI!");
            });
        });


        // Draw things behind egui here

        self.egui.draw(&mut *self.ctx);

        self.ctx.commit_frame();
    }
}