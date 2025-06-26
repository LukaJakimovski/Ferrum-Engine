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
use miniquad::conf::Platform;

fn main() {
    let conf = conf::Conf {
        window_title: "Ferrum Engine".to_string(),
        window_height: 1440,
        window_width: 1440,
        high_dpi: false,
        sample_count: 16,
        window_resizable: false,
        icon: None,
        platform: Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        fullscreen: false
    }
        ;
    let mut polygons = vec![
    ];
    for i in 0..64 {
        for j in 0..64{
            polygons.push(Polygon::polygon(rand::random::<u32>() % 3 + 3, 0.3533, Vec2 { x: i as f32 * 0.67, y: j as f32 * 0.67 }));
        }
    }

    
    start(conf, move || Box::new(World::new(polygons)));
}