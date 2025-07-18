use rand;

#[repr(C)] #[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[allow(dead_code)]
impl Color{
    pub fn red() -> Self{
        Self{r: 1.0, g: 0.0, b: 0.0}
    }
    pub fn white() -> Self{
        Self{r: 1.0, g: 1.0, b: 1.0}
    }
    pub fn black() -> Self{
        Self{r: 0.0, g: 0.0, b: 0.0}
    }
    pub fn gray() -> Self{
        Self{r: 0.5, g: 0.5, b: 0.50}
    }
    pub fn orange() -> Self{
        Self{r: 1.0, g: 0.5, b: 0.2}
    }
    pub fn blue() -> Self{ Self{r: 0.0, g: 0.0, b: 1.0} }
    pub fn transparent() -> Self{
        Self{r: 0.0, g: 0.0, b: 0.0}
    }
    pub fn random() -> Self{ Self{ r: rand::random::<f32>(), g: rand::random::<f32>(), b: rand::random::<f32>()} }
    pub fn new(r: f32, g: f32, b: f32) -> Self{
        Self{r, g, b}
    }
}