use ferrum_engine::*;
use std::f32::consts::PI;

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
            swap_interval: Some(1),
            ..Default::default()
        },
        fullscreen: false
    }
        ;
    let mut polygons = vec![
    ];
    polygons.push(Polygon::rectangle(10.0, 5.0, Vec2{x: 0.0, y: 0.0}));
    polygons[0].mass = f32::MAX / 10000000.0;
    polygons[0].calculate_moment_of_inertia();
    polygons[0].change_color(Color::new(1.0, 0.7, 0.1));
    polygons[0].rotate(PI/6.0);

    polygons.push(Polygon::polygon(32, 2.0, Vec2{x: 1.0, y: 20.0}));
    let parameters = Parameters {delta_time: 0.0003, updates_per_frame: 100, angular_velocity: true, camera_pos: (0.0, 0.0, 0.0, 0.0), gravity: true };
    
    start(conf, move || Box::new(World::new(polygons, parameters)));
}