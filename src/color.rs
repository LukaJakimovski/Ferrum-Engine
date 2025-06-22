use crate::math::Vec4;
use rand;

#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[allow(dead_code)]
impl Color{
    pub fn red() -> Self{
        Self{r: 1.0, g: 0.0, b: 0.0, a: 1.0}
    }
    pub fn white() -> Self{
        Self{r: 1.0, g: 1.0, b: 1.0, a: 1.0}
    }
    pub fn black() -> Self{
        Self{r: 0.0, g: 0.0, b: 0.0, a: 1.0}
    }
    pub fn gray() -> Self{
        Self{r: 0.5, g: 0.5, b: 0.5, a: 1.0}
    }
    pub fn orange() -> Self{
        Self{r: 1.0, g: 0.5, b: 0.2, a: 1.0}
    }
    pub fn transparent() -> Self{
        Self{r: 0.0, g: 0.0, b: 0.0, a: 0.0}
    }
    pub fn random() -> Self{
        Self{ r: rand::random::<f32>(), g: rand::random::<f32>(), b: rand::random::<f32>(), a: 1.0}
    }
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self{
        Self{r, g, b, a}
    }
    pub fn to_vec4(&self) -> Vec4{
        Vec4 {x: self.r, y: self.g, z: self.b, w: self.a}
    }
}