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
        Polygon::polygon(3, 0.3533, Vec2 { x: -0.31421356237309504, y: 0.0 }),
        Polygon::polygon(4, 0.3533, Vec2 { x: 0.0, y: 0.55 }),
        Polygon::polygon(5, 0.3533, Vec2 { x: 0.31421356237309504, y: 0.0 }),
        Polygon::polygon(32, 0.3533, Vec2 { x: -1.0, y: 1.0 }),
    ];
    
    start(conf, move || Box::new(World::new(polygons)));
}