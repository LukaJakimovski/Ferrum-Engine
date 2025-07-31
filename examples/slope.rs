use ferrum_engine::*;
use std::f32::consts::PI;

fn main() {
    let conf = conf::Conf {
        window_title: "Slope Example".to_string(),
        window_height: 800,
        window_width: 800,
        high_dpi: false,
        sample_count: 4,
        window_resizable: true,
        icon: None,
        platform: Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        fullscreen: false
    };

    let mut polygons = vec![];
    
    polygons.push(Rigidbody::rectangle(10.0, 1.0, Vec2{x: 0.0, y: 0.0}, f32::MAX, 0.6, Color::orange()));
    polygons[0].rotate(PI / 4.0);

    for i in 0..10 {
        polygons.push(Rigidbody::polygon(32, 0.3533, Vec2{x: 2.5, y: 6.0 + i as f32 * 2.0}, 1.0, 1.0, Color::random()));
    }
    
    let parameters = Parameters {delta_time: 0.0001, updates_per_frame: 165, angular_velocity: true, camera_pos: (0.0, 0.0, 0.0, -10.0), gravity: true, world_size: 300.0 };
    start(conf, move || Box::new(World::new(polygons, vec![], parameters)));
}