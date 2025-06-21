mod square;
mod render;
mod shader;
mod collision_detection;
mod math;

use crate::square::*;
use crate::render::*;
use crate::math::*;
use miniquad::*;

fn main() {
    let conf = conf::Conf::default();
    let polygons = vec![
        rectangle(0.5, 0.5, Vec2 { x: 0.0, y: 0.0 }),
        rectangle(0.5, 0.5, Vec2 { x: -0.6, y: -0.6 }),
        triangle(0.5, 0.5, Vec2 { x: 0.6, y: 0.6 })
    ];
    start(conf, move || Box::new(Stage::new(polygons)));
}