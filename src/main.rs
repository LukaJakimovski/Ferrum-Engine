mod square;
mod render;
mod shader;

use crate::square::*;
use crate::render::*;
use miniquad::*;

fn main() {
    let conf = conf::Conf::default();
    let polygons = vec![
        rectangle(0.5, 0.5, Vec2 { x: 0.0, y: 0.0 }),
        rectangle(0.5, 0.5, Vec2 { x: -0.5, y: -0.5 }),
        triangle(0.5, 0.5, Vec2 { x: 0.5, y: 0.5 })
    ];
    start(conf, move || Box::new(Stage::new(polygons)));
}