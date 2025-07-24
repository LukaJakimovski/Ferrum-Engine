use miniquad::*;

pub const VERTEX: &str = r#"#version 330
    in vec2 in_pos;
    in vec3 in_color;
    
    out lowp vec3 color;
    
    uniform vec4 camera_pos;
    uniform float aspect_ratio;

    void main() {
        gl_Position = vec4( in_pos.x, in_pos.y  * aspect_ratio, 0, 1) - camera_pos;
        color = in_color;
    }"#;

pub const FRAGMENT: &str = r#"#version 120
    in lowp vec3 color;

    void main()
    {
        gl_FragColor = vec4 ( color, 1.0);
    } "#;

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("camera_pos", UniformType::Float4),
                UniformDesc::new("aspect_ratio", UniformType::Float1)]
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub camera_pos: (f32, f32, f32, f32),
    pub aspect_ratio: f32,
}