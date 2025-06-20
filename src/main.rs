mod square;
mod render;

use crate::square::*;
use crate::render::*;
use miniquad::*;



fn main() {
    let conf = conf::Conf::default();
    let polygons = vec![
        draw_rectangle(0.5, 0.5, Vec2 {x: 0.0, y: 0.0}), 
        draw_rectangle(0.5, 0.5, Vec2 {x: -0.5, y: -0.5}),
        draw_triangle(0.5, 0.5, Vec2 {x: 0.5, y: 0.5})
    ];
    start(conf, move || Box::new(Stage::new(polygons)));
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 330 core
    attribute vec2 in_pos;
    
    uniform vec4 camera_pos;
    
    void main() {
        gl_Position = vec4( in_pos, 0, 1) - camera_pos;
    }"#;

    pub const FRAGMENT: &str = r#"#version 330 core
    out vec4 FragColor;

    void main()
    {   
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    } "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("camera_pos", UniformType::Float4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub camera_pos: (f32, f32, f32, f32),
    }
}