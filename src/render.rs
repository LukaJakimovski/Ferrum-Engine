use crate::spring::Spring;
use crate::{Color, Rigidbody, Vertex, World};
use egui_wgpu::wgpu;
use std::iter;
use glam::Vec2;
use wgpu::util::DeviceExt;

impl World {
    pub fn get_vertices_and_indices(
        polygons: &Vec<Rigidbody>,
        springs: &Vec<Spring>,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(
            polygons.iter().map(|p| p.vertices.len() + 1).sum::<usize>()
                + springs
                    .iter()
                    .map(|s| s.connector.vertices.len() + 1)
                    .sum::<usize>(),
        );

        let mut indices: Vec<u32> = Vec::with_capacity(
            polygons.iter().map(|p| p.indices.len()).sum::<usize>()
                + springs
                    .iter()
                    .map(|s| s.connector.indices.len())
                    .sum::<usize>(),
        );
        let mut start_index: u32 = 0;
        // Helper closure to process each connector-like structure
        let mut process = |verts: &[Vec2], color: Color, center: Vec2, indices_src: &[u32]| {
            for vert in verts {
                vertices.push(Vertex {pos: vert.clone(), color});
            }
            vertices.push(Vertex { pos: center, color });

            indices.extend(indices_src.iter().map(|i| i + start_index));
            start_index += verts.len() as u32 + 1;
        };

        for polygon in polygons {
            process(&polygon.vertices, polygon.color, polygon.center, &polygon.indices);
        }

        for spring in springs {
            process(
                &spring.connector.vertices,
                spring.connector.color,
                spring.connector.center,
                &spring.connector.indices,
            );
        }

        (vertices, indices)
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let (vertices, indices) = &Self::get_vertices_and_indices(&self.polygons, &self.springs);

        self.window.request_redraw();

        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            // Always update uniforms
            let size = self.window.inner_size();
            self.uniforms.camera_pos = self.camera_pos;
            self.uniforms.aspect_ratio = size.width as f32 / size.height as f32;
            self.queue.write_buffer(
                &self.uniforms_buffer,
                0,
                bytemuck::cast_slice(&[self.uniforms]),
            );
            self.queue.submit(None);

            // Only update and draw if there are vertices/indices
            if !vertices.is_empty() && !indices.is_empty() {
                self.vertex_buffer.destroy();
                self.index_buffer.destroy();

                self.vertex_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(&vertices),
                            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        });

                self.index_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(&indices),
                            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                        });

                self.num_indices = indices.len() as u32;
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, &self.uniforms_bind_group, &[]);
                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass
                    .set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
        }

        self.create_gui(&mut encoder, &view);

        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
