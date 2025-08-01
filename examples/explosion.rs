use ferrum_engine::*;

fn main() {
    let conf = conf::Conf {
        window_title: "Explosion Example".to_string(),
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
    for i in 0..64 {
        for j in 0..64 {
            polygons.push(Rigidbody::polygon(rand::random::<u32>() % 3 + 3, 0.3533, Vec2 { x: i as f32 * 0.3, y: j as f32 * 0.3 }, 1.0, 1.0, Color::random()));
        }
    }
    for i in 0..polygons.len() {
        polygons[i].velocity = Vec2{x: (rand::random::<f32>() * 2.0 - 1.0) * 10.0, y: (rand::random::<f32>() * 2.0 - 1.0) * 10.0};
        polygons[i].angular_velocity = (rand::random::<f32>() * 2.0 - 1.0) * 100.0;
    }
    let parameters = Parameters {delta_time: 0.0, updates_per_frame: 1, angular_velocity: true, camera_pos: (9.0, 9.0, 0.0, -75.0), gravity: false, world_size: 300.0 };
    start(conf, move || Box::new(World::new(polygons, vec![], parameters)));
}