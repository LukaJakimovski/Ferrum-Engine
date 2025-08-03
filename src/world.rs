use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use crate::{Color, Rigidbody, Vec2, Vec4};
use crate::spring::Spring;
use crate::utility::date;

#[derive(Clone)]
pub struct Parameters{
    pub delta_time: f64,
    pub updates_per_frame: u32,
    pub angular_velocity: bool,
    pub camera_pos: Vec4,
    pub gravity: bool,
    pub world_size: f32,
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
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
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
    config: wgpu::SurfaceConfiguration,
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
}

impl World {
    pub(crate) async fn new(window: Arc<Window>,
                            polygons: Vec<Rigidbody>,
                            springs: Vec<Spring>,
                            parameters: Parameters,) -> anyhow::Result<World> {
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;
        let uniforms = Uniforms { camera_pos: parameters.camera_pos, aspect_ratio, padding: [0.0; 7] };
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
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
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
                trace: wgpu::Trace::Off, // Trace path
            })
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


        #[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu"))]
        let scaling_factor = 0.1;
        #[cfg(not(all(target_os = "windows", target_arch = "x86_64", target_env = "gnu")))]
        let scaling_factor = 10.0;
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

        }
        else{
            self.delta_time = self.parameters.delta_time;
        }


        if self.is_running == true{
            for _i in 0..self.parameters.updates_per_frame {
                self.update_physics();
            }
        }

        for i in 0..self.polygons.len() {
            if self.polygons[i].center.distance(&Vec2::zero()) > self.parameters.world_size {
                self.remove_rigidbody(i);
                break;
            }
        }

        self.frame_count += 1;
        if self.frame_count % 10 == 0 {
            self.fps = 10.0 / (date::now() - self.timer);
            self.timer = date::now();
            println!("FPS: {}", self.fps);
        }

        self.total_energy = 0.0;
        for polygon in &self.polygons {
            self.total_energy += polygon.calculate_energy();
        }
    }
}

