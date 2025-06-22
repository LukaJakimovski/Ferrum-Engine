mod square;
mod render;
mod shader;
mod collision_detection;
mod math;
mod color;

use crate::square::*;
use crate::render::*;
use crate::math::*;
use miniquad::*;

fn main() {
    let conf = conf::Conf::default();
    let polygons = vec![
        Polygon::rectangle(0.5, 0.5, Vec2 { x: 0.0, y: 0.0 }),
        Polygon::rectangle(0.5, 0.5, Vec2 { x: -0.6, y: -0.6 }),
        Polygon::triangle(0.5, 0.5, Vec2 { x: 0.6, y: 0.6 })
    ];
    start(conf, move || Box::new(World::new(polygons)));
}