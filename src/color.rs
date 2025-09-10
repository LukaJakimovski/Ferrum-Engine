use std::f32::consts::PI;
use std::ops::Range;
use rand;
use colors_transform::{Color, Hsl};
use oklab::*;
use crate::Rigidbody;

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

pub struct OkLCH {
    pub l: f32,
    pub c: f32,
    pub h: f32,
}

pub fn oklch_to_rgb(oklch: OkLCH) -> ColorRGBA {
    let a = oklch.c * oklch.h.cos();
    let b = oklch.c * oklch.h.sin();
    let oklab = Oklab { l: oklch.l, a, b };
    let rgb = oklab_to_srgb_f32(oklab);
    ColorRGBA {
        r: rgb.r,
        g: rgb.g,
        b: rgb.b,
        a: 1.0,
    }
}

#[derive(Clone)]
pub struct ColorRange {
    pub x: Range<f32>,
    pub y: Range<f32>,
    pub z: Range<f32>,
}

pub struct PaletteParams {
    pub start_range: ColorRange,
    pub end_range: ColorRange,
    pub color_count: usize,
}

impl PaletteParams {
    pub fn create_palette(&mut self) -> Vec<ColorRGBA> {
        let mut l = rand::random_range::<f32, Range<f32>, >(self.start_range.x.clone());
        let mut c = rand::random_range::<f32, Range<f32>, >(self.start_range.y.clone());
        let mut h = rand::random_range::<f32, Range<f32>, >(self.start_range.z.clone());
        let ln = rand::random_range::<f32, Range<f32>, >(self.end_range.x.clone());
        let cn = rand::random_range::<f32, Range<f32>, >(self.end_range.y.clone());
        let hn = rand::random_range::<f32, Range<f32>, >(self.end_range.z.clone());
        let ls = (ln - l) / (self.color_count as f32);
        let cs = (cn - c) / (self.color_count as f32);
        let hs = (hn - h) / (self.color_count as f32);
        let mut palette = vec![];
        for _i in 0..self.color_count {
            if _i == 0 {
                palette.push(oklch_to_rgb(OkLCH {l: l / 2.0 , c: c / 3.0, h}));
            }
            palette.push(oklch_to_rgb(OkLCH {l, c, h}));
            l += ls;
            c += cs;
            h += hs;
        }
        palette
    }
}

pub struct ColorSystem {
    pub palette_params: PaletteParams,
    pub color_palette: Option<Vec<ColorRGBA>>,
    pub clear_color: ColorRGBA,
}

impl ColorSystem{
    pub fn regenerate_colors(&mut self, polygons: &mut Vec<Rigidbody>){
        self.color_palette = Some(self.palette_params.create_palette());
        for polygon in polygons{
            let rand = rand::random_range::<usize, Range<usize>, >(1..self.color_palette.as_ref().unwrap().len());
            polygon.color = self.color_palette.clone().unwrap()[rand];
        }
        self.clear_color = self.color_palette.clone().unwrap()[0];
    }

    pub fn view_random_palette(&mut self, polygons: &mut Vec<Rigidbody>){
        self.color_palette = Some(self.palette_params.create_palette());
        let mut i = 1;
        for polygon in polygons{
            //let rand = rand::random_range::<usize, Range<usize>, >(1..self.color_palette.as_ref().unwrap().len());
            let rand = i % self.color_palette.as_ref().unwrap().len();
            polygon.color = self.color_palette.clone().unwrap()[rand];
            i += 1;
        }
        self.clear_color = self.color_palette.clone().unwrap()[0];
    }
}


