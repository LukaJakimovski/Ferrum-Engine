use miniquad::*;

pub const VERTEX: &str = r#"#version 330 core
    attribute vec2 in_pos;
    attribute vec4 in_color;
    
    varying lowp vec4 color;
    
    uniform vec4 camera_pos;

    void main() {
        gl_Position = vec4( in_pos, 0, 1) - camera_pos;
        color = in_color;
    }"#;

pub const FRAGMENT: &str = r#"#version 330 core
    varying lowp vec4 color;

    void main()
    {
        gl_FragColor = color;
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