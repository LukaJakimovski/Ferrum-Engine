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
    let mut polygons = vec![
    ];
    for i in 0..46 {
        for j in 0..46{
            polygons.push(Polygon::polygon(5, 0.3533, Vec2 { x: i as f32 * 0.7, y: j as f32 * 0.7 }));
        }
    }

    
    start(conf, move || Box::new(World::new(polygons)));
}