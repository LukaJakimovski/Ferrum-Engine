mod square;
mod render;
mod shader;
mod collision_detection;
mod math;
mod color;
mod ode_solver;
mod physics;

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
        window_resizable: true,
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
    polygons.push(Polygon::rectangle(10.0, 1.0, Vec2 { x: 0.0, y: 0.0}));
    polygons.push(Polygon::rectangle(1.0, 1.0, Vec2 { x: 0.0, y: 5.0}));
    start(conf, move || Box::new(World::new(polygons)));
}