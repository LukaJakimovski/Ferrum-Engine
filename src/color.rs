use std::ops::Range;
use rand;
use colors_transform::{Color, Hsl};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct ColorHSVA {
    pub h: f32,
    pub s: f32,
    pub v: f32,
    pub a: f32,
}

#[allow(dead_code)]
impl ColorRGBA {
    pub const fn red() -> Self {
        Self {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
    pub const fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
    pub const fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
    pub const fn gray() -> Self {
        Self {
            r: 0.5,
            g: 0.5,
            b: 0.50,
            a: 1.0,
        }
    }
    pub const fn orange() -> Self {
        Self {
            r: 1.0,
            g: 0.5,
            b: 0.2,
            a: 1.0,
        }
    }
    pub const fn blue() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        }
    }
    pub const fn transparent() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.5,
        }
    }
    pub fn random() -> Self {
        let h = rand::random_range::<f64, Range<f64>, >(0.0..360.0);
        let s = rand::random_range::<f64, Range<f64>, >(50.0..70.0);
        let l = rand::random_range::<f64, Range<f64>, >(40.0..60.0);
        let hsl = Hsl::from(h as f32, s as f32, l as f32);
        let rgb = hsl.to_rgb();
        let mut rgb_tuple = rgb.as_tuple();
        rgb_tuple.0 /= 255.0;
        rgb_tuple.1 /= 255.0;
        rgb_tuple.2 /= 255.0;
        Self {

            r: rgb_tuple.0,
            g: rgb_tuple.1,
            b: rgb_tuple.2,
            a: 1.0,
        }
    }
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}


