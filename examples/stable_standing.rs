use ferrum_engine::*;

fn main() {
    let conf = conf::Conf {
        window_title: "Stable Standing Example".to_string(),
        window_height: 800,
        window_width: 800,
        high_dpi: false,
        sample_count: 4,
        window_resizable: true,
        icon: None,
        platform: Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        fullscreen: false
    };

    let mut polygons = vec![];
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: -1.0, y: 5.0}));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: 1.0, y: 5.0}));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: -2.0, y: 5.0}));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: 2.0, y: 5.0}));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: -0.00001, y: 5.0}));
    polygons.push(Rigidbody::rectangle(1000.0, 1.0, Vec2 { x: 1.0, y: 0.0}));

    polygons[5].mass = f32::MAX / 100000000000.0;
    polygons[5].calculate_moment_of_inertia();
    polygons[5].rotate(-1.0);
    for polygon in &mut polygons {
        polygon.restitution = 0.6;
        polygon.rotate(1.0);
    }
    let parameters = Parameters {delta_time: 0.00001, updates_per_frame: 25, angular_velocity: true, camera_pos: (0.0, 2.5, 0.0, -3.0), gravity: true, world_size: 300.0 };
    start(conf, move || Box::new(World::new(polygons, vec![], parameters)));
}