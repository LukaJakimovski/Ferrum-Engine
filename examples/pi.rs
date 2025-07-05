use ferrum_engine::*;

fn main() {
    let conf = conf::Conf {
        window_title: "Pi Example".to_string(),
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

    let mut polygons = vec![
    ];
    polygons.push(Polygon::rectangle(5.0, 1000.0, Vec2{x: -5.0, y: 0.0}));
    polygons[0].mass = f32::MAX / 10000000.0;
    polygons[0].calculate_moment_of_inertia();
    polygons[0].change_color(Color::new(1.0, 0.7, 0.1));

    polygons.push(Polygon::rectangle(0.5, 0.5, Vec2{x: 0.0, y: 0.0}));
    polygons[1].mass = 1.0;
    polygons[1].change_color(Color::blue());

    polygons.push(Polygon::rectangle(0.5, 0.5, Vec2{x: 5.0, y: 0.0}));
    polygons[2].mass = 10000000000.0;
    polygons[2].change_color(Color::new(0.0, 0.7, 1.0));
    polygons[2].velocity = Vec2{x: -0.0005, y: 0.0};


    let parameters = Parameters {delta_time: 0.003, updates_per_frame: 10000, angular_velocity: false, camera_pos: (0.0, 0.0, 0.0, -5.0), gravity: false };
    start(conf, move || Box::new(World::new(polygons, parameters)));
}
