use std::f32::consts::PI;
use std::ops::Range;
use rand;
use colors_transform::{Color, Hsl};
use oklab::*;
use crate::World;

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
    pub fn random_hsl() -> Self {
        let h = rand::random_range::<f64, Range<f64>, >(0.0..360.0);
        let s = rand::random_range::<f64, Range<f64>, >(70.0..90.0);
        let l = rand::random_range::<f64, Range<f64>, >(30.0..50.0);
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

    pub fn random_oklab() -> Self {
        let l = rand::random_range::<f32, Range<f32>, >(0.4..0.50);
        let c = rand::random_range::<f32, Range<f32>, >(0.15..0.25);
        let h = rand::random_range::<f32, Range<f32>, >(0.0..PI / 2.0);
        let a = c * h.cos();
        let b = c * h.sin();
        let oklab = Oklab {l, a, b};
        let rgb = oklab_to_srgb_f32(oklab);
        Self {

            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 1.0,
        }
    }

    pub fn random_from_palette(palette: &Vec<ColorRGBA>) -> Self {
        let rand = rand::random_range::<usize, Range<usize>, >(0..palette.len());
        let rgb = palette[rand];
        Self {

            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 1.0,
        }
    }
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

pub fn create_palette(size: u8) -> Vec<ColorRGBA> {
    let mut l = rand::random_range::<f32, Range<f32>, >(0.1..0.15);
    let c = rand::random_range::<f32, Range<f32>, >(0.05..0.15);
    let mut h = rand::random_range::<f32, Range<f32>, >(0.0..PI * 2.0);
    let ls = rand::random_range::<f32, Range<f32>, >(0.025..0.05);
    let cs = rand::random_range::<f32, Range<f32>, >(0.025..0.05);
    let mut palette = vec![];
    for _ in 0..size {
        let a = c * h.cos();
        let b = c * h.sin();
        let oklab = Oklab {l, a, b};
        let rgb = oklab_to_srgb_f32(oklab);
        palette.push(ColorRGBA {

            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
            a: 1.0,
        });
        l += ls;
        h += cs;
    }
    palette
}

impl World{
    pub fn regenerate_colors(&mut self){
        self.colors = Some(create_palette(16));
        for polygon in &mut self.polygons{
            let rand = rand::random_range::<usize, Range<usize>, >(0..self.colors.as_ref().unwrap().len());
            polygon.color = self.colors.clone().unwrap()[rand];
        }
    }
}


