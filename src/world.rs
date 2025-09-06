use crate::body_builder::{BodyBuilder, RigidbodyParams, SpringParams};
use crate::egui_tools::EguiRenderer;
use crate::enums::{BodyType, ColorType, DraggingState, InputMode, Menu};
use crate::spring::Spring;
use crate::utility::date;
use crate::{Color, Rigidbody, Vec2, Vec4};
use egui_wgpu::wgpu;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[derive(Clone)]
pub struct Parameters {
    pub delta_time: f64,
    pub updates_per_frame: u32,
    pub angular_velocity: bool,
    pub camera_pos: Vec4,
    pub gravity: bool,
    pub world_size: f32,
    pub gravity_force: Vec2,
    pub time_multiplier: f32
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub camera_pos: Vec4,
    pub aspect_ratio: f32,
    pub padding: [f32; 7],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub(crate) pos: Vec2,
    pub(crate) color: Color,
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
pub struct World {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub render_pipeline: wgpu::RenderPipeline,
    // NEW!
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub window: Arc<Window>,

    //Us
    pub vertices: Vec<Vertex>,
    pub uniforms: Uniforms,
    pub camera_pos: Vec4,
    pub uniforms_buffer: wgpu::Buffer,
    pub uniforms_bind_group: wgpu::BindGroup,
    // From World
    pub scaling_factor: f32,
    pub mouse_pos: (f32, f32),

    pub springs: Vec<Spring>,
    pub polygons: Vec<Rigidbody>,
    pub previous_polygon_count: usize,

    pub collisions: usize,

    pub pressed_keys: [u8; 64],
    pub pressed_buttons: [u8; 3],

    pub start_time: f64,
    pub delta_time: f64,
    frame_count: u32,
    pub fps: f64,
    pub is_running: bool,
    pub total_energy: f64,

    pub parameters: Parameters,
    pub timer: f64,
    pub egui_renderer: EguiRenderer,
    pub is_pointer_used: bool,

    pub menus: [bool; 16],
    pub spawn_parameters: BodyBuilder,

    pub input_mode: InputMode,
    pub selected_polygon: Option<usize>,
    pub selected_spring: Option<usize>,
    pub spring_polygon: Option<usize>,
    pub mouse_spring: Option<usize>,
    pub spawn_ghost_polygon: Option<usize>,
    pub anchor_pos: Vec2,
    pub dragging: DraggingState,
    pub drag_params: SpringParams,
}

impl World {
    pub(crate) async fn new(
        window: Arc<Window>,
        polygons: Vec<Rigidbody>,
        springs: Vec<Spring>,
        parameters: Parameters,
    ) -> anyhow::Result<World> {
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;
        let uniforms = Uniforms {
            camera_pos: parameters.camera_pos,
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
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
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
                pos: Vec2::zero(),
                mass: 1.0,
                width: 0.5,
                height: 0.5,
                restitution: 0.8,
                color: None,
                collides: true,
                rotation: 0.0,
                angular_velocity: 0.0,
                velocity: Vec2::zero(),
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
        #[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu"))]
        let scaling_factor = 0.1;
        #[cfg(not(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu")))]
        let scaling_factor = 10.0;
        let mut menus = [false; 16];
        menus[Menu::Input as usize] = true;
        menus[Menu::Config as usize] = true;
        Ok(Self {
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

            scaling_factor,
            mouse_pos: (0.0, 0.0),
            springs,
            polygons,
            previous_polygon_count: 0,
            collisions: 0,
            pressed_keys: [0; 64],
            pressed_buttons: [0; 3],
            start_time: 0.0,
            delta_time: 0.0,
            frame_count: 0,
            fps: 0.0,
            is_running: false,
            total_energy: 0.0,
            parameters,
            camera_pos: uniforms.camera_pos,
            timer: 0.0,
            egui_renderer,
            is_pointer_used: false,
            menus,
            spawn_parameters,
            input_mode: InputMode::Spawn,
            selected_polygon: None,
            selected_spring: None,
            spring_polygon: None,
            mouse_spring: None,
            spawn_ghost_polygon: None,
            anchor_pos: Vec2::new(0.0, 0.0),
            dragging: DraggingState::NotDragging,
            drag_params: SpringParams {
                stiffness: 10.0,
                dampening: 1.0,
                ..Default::default()
            },
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub(crate) fn update(&mut self) {
        if self.parameters.delta_time == 0.0 {
            self.delta_time = date::now() - self.start_time;
            self.start_time = date::now();
            self.delta_time *= self.parameters.time_multiplier as f64;
        } else {
            self.delta_time = self.parameters.delta_time;
        }
        for spring in &mut self.springs {
            spring.update_connector(&mut self.polygons);
        }
        
        if self.is_pointer_used == false {
            self.handle_input();
        }
        
        for i in 0..self.polygons.len() {
            if i < self.polygons.len()
                && self.parameters.world_size > 0.0
                && self.polygons[i].center.distance(&Vec2::zero()) > self.parameters.world_size
                && self.polygons[i].eternal == false
            {
                self.remove_rigidbody(i);
            }
        }
        if self.is_running == true {
            for _i in 0..self.parameters.updates_per_frame {
                self.update_physics();
            }
        }
        self.frame_count += 1;
        if self.frame_count % 10 == 0 {
            self.fps = 10.0 / (date::now() - self.timer);
            self.timer = date::now();
        }

        self.total_energy = 0.0;
        for polygon in &self.polygons {
            self.total_energy += polygon.calculate_energy();
        }
        for spring in &mut self.springs {
            self.total_energy += spring.calculate_energy(&self.polygons);
        }
        self.create_mouse_ghost();
    }
}
