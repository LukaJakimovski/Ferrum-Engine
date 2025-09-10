use std::f32::consts::PI;
use std::ops::Range;
use std::sync::Arc;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::util::DeviceExt;
use glam::Vec2;
use winit::window::Window;
use crate::{ColorRGBA, Parameters, Rigidbody, World};
use crate::body_builder::{BodyBuilder, RigidbodyParams, SpringParams};
use crate::color::{ColorRange, ColorSystem, PaletteParams};
use crate::egui_tools::EguiRenderer;
use crate::enums::{BodyType, ColorType, DraggingState, InputMode, Menu};
use crate::input::UiSystem;
use crate::physics::PhysicsSystem;
use crate::render::{RenderSystem, Uniforms, Vertex};
use crate::spring::Spring;
use crate::timing::Timing;
use crate::weld_joint::WeldJoint;

impl World{
    pub(crate) async fn new(
        window: Arc<Window>,
        polygons: Vec<Rigidbody>,
        springs: Vec<Spring>,
        weld_joints: Vec<WeldJoint>,
        mut parameters: Parameters,
    ) -> anyhow::Result<World> {
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;
        let uniforms = Uniforms {
            camera_pos: parameters.initial_camera.camera_pos,
            aspect_ratio,
            padding: [0.0; 7],
        };
        let (vertices, indices) = World::get_vertices_and_indices(&polygons, &springs);
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                None,
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes a@ Srgb surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        let uniforms_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniforms_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniforms_bind_group_layout"),
            });
        let uniforms_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniforms_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniforms_buffer.as_entire_binding(),
            }],
            label: Some("uniforms_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniforms_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
            // Useful for optimizing shader compilation on Android
            cache: None,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        });
        let num_indices = indices.len() as u32;

        let egui_renderer = EguiRenderer::new(&device, config.format, None, 1, &window);

        let spawn_parameters = BodyBuilder {
            body_type: BodyType::RegularPolygon,

            rigidbody_params: RigidbodyParams {
                sides: 32,
                radius: 0.3533,
                pos: Vec2::ZERO,
                mass: 1.0,
                width: 0.5,
                height: 0.5,
                restitution: 0.8,
                color: None,
                collides: true,
                rotation: 0.0,
                angular_velocity: 0.0,
                velocity: Vec2::ZERO,
                color_type: ColorType::Random,
                gravity_multiplier: 1.0,
                eternal: false,
            },

            spring_params: SpringParams {
                dampening: 1.0,
                stiffness: 10.0,
                rest_length: 0.0,
                body_a: 0,
                body_b: 0,
                anchor_a: Default::default(),
                anchor_b: Default::default(),
            },
        };
        let palette_params = PaletteParams {
            start_range: ColorRange { x: Range{start: 0.15, end: 0.25}, y: Range{start: 0.15, end: 0.20}, z: Range{start: 0.0, end: 2.0 * PI} },
            end_range: ColorRange { x: Range{start: 0.25, end: 0.30}, y: Range{start: 0.20, end: 0.25}, z: Range{start: 2.0 * PI, end: 4.0 * PI} },
            color_count: 32,
        };
        #[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu"))]
        let scale_divider = 100.0;
        #[cfg(not(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu")))]
        let scale_divider = 1.0;
        parameters.initial_camera.scaling_factor /= scale_divider;
        let mut menus = [false; 16];
        menus[Menu::Input as usize] = true;
        menus[Menu::Config as usize] = true;

        let render: RenderSystem = RenderSystem {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_indices,
            window,
            vertices,
            uniforms,
            uniforms_buffer,
            uniforms_bind_group,
            egui_renderer,
        };
        let timing: Timing = Timing {
            start_time: 0.0,
            timer: 0.0,
            frame_count: 0,
            fps: 0.0,
        };
        let physics: PhysicsSystem = PhysicsSystem{
            springs,
            polygons,
            weld_joints,
            dt: 0.0,
            total_energy: 0.0,
        };

        let color_system: ColorSystem = ColorSystem {
            palette_params,
            color_palette: None,
            clear_color: ColorRGBA { r: 0.0, g: 0.0, b: 0.0, a: 0.0 },
        };

        let ui: UiSystem = UiSystem {
            pressed_keys: [0; 64],
            pressed_buttons: [0; 3],
            mouse_pos: Vec2::ZERO,
            is_pointer_used: false,
            selected_polygon: None,
            selected_spring: None,
            spring_polygon: None,
            mouse_spring: None,
            spawn_ghost_polygon: None,
            input_mode: InputMode::Spawn,
            dragging: DraggingState::NotDragging,
            menus,
            drag_params: SpringParams {
                stiffness: 10.0,
                dampening: 1.0,
                ..Default::default()
            },
            spawn_parameters,
            camera: parameters.initial_camera.clone(),
            window_dimensions: Vec2::new(render.config.width as f32, render.config.height as f32),
        };





        Ok(Self {
            render,
            physics,
            timing,
            color_system,
            ui,
            parameters,
        })
    }
}