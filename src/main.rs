use miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    ctx: Box<dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [Vertex; 3] = [
            Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.0, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 3] = [0, 1, 2];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let pixels: [u8; 1 * 1 * 4] = [
            0xFF, 0xFF, 0xFF, 0xFF,
        ];
        let texture = ctx.new_texture_from_rgba8(1, 1, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
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
        self.ctx.draw(0, 3, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {
    let mut conf = conf::Conf::default();
    miniquad::start(conf, move || Box::new(Stage::new()));
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
    }"#;

    pub const FRAGMENT: &str = r#"#version 330 core
    out vec4 FragColor;

    void main()
    {   
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    } "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["empty".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("empty", UniformType::Float2)],
            },
        }
    }
}