use miniquad::{window, Bindings, BufferSource, BufferType, BufferUsage, UniformsSource};
use crate::{shader, RenderObject, Vec2, Vertex, World};

impl World{
    pub fn get_vertices_and_indices(&mut self) -> (Vec<Vertex>, Vec<u32>){
        let mut vertices: Vec<Vertex> = Vec::with_capacity(
            self.polygons.iter().map(|p| p.vertices.len() + 1).sum::<usize>() +
                self.springs.iter().map(|s| s.connector.vertices.len() + 1).sum::<usize>()
        );

        let mut indices: Vec<u32> = Vec::with_capacity(
            self.polygons.iter().map(|p| p.indices.len()).sum::<usize>() +
                self.springs.iter().map(|s| s.connector.indices.len()).sum::<usize>()
        );
        let mut start_index: u32 = 0;

        // Helper closure to process each connector-like structure
        let mut process = |verts: &[Vertex], center: Vec2, indices_src: &[u32]| {
            let color = verts[0].color;
            vertices.extend_from_slice(verts);
            vertices.push(Vertex { pos: center, color });

            indices.extend(indices_src.iter().map(|i| i + start_index));
            start_index += verts.len() as u32 + 1;
        };

        for polygon in &self.polygons {
            process(&polygon.vertices, polygon.center, &polygon.indices);
        }

        for spring in &self.springs {
            process(&spring.connector.vertices, spring.connector.center, &spring.connector.indices);
        }

        (vertices, indices)
    }

    pub fn create_render_object(&mut self) {
        let (vertices, indices) = self.get_vertices_and_indices();
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
            egui::Window::new("Kinetic Energy").show(egui_ctx, |ui| {
                ui.label(format!("{:.3}J", self.total_energy));
            });

            egui::Window::new("FPS").show(egui_ctx, |ui| {
                ui.label(format!("{:.3}fps", self.fps));
                ui.label(format!("{:.3}ms per frame", 1000.0 / self.fps));
            });

            egui::Window::new("Using Pointer?").show(egui_ctx, |ui| {
                ui.label(format!("Is using pointer: {}", egui_ctx.is_pointer_over_area()));

                if egui_ctx.is_pointer_over_area() { self.pointer_used = true; }
                else { self.pointer_used = false; }
            });
        });


        // Draw things behind egui here

        self.egui.draw(&mut *self.ctx);
        //self.egui.egui_ctx().is_using_pointer();
        self.ctx.commit_frame();
    }
}