use ferrum_engine::*;
use ferrum_engine::spring::Spring;

fn main() {
    let conf = conf::Conf {
        window_title: "Springs Example".to_string(),
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

    let mut springs= vec![];
    springs.push(Spring::new(
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(0.0, 1.0)),
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(0.0, -1.0)),
        Vec2::new(0.0, -0.0),
        Vec2::new(0.0, 0.0),
        7.0,
        10.0,
        1.0,
    ));
    springs.push(Spring::new(
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(-3.0, -5.0)),
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(-3.0, 5.0)),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, -0.0),
        2.0,
        10.0,
        2.0,
    ));
    springs.push(Spring::new(
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(2.5, 3.0)),
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(3.5, -3.0)),
        Vec2::new(0.0, -0.0),
        Vec2::new(0.0, 0.0),
        5.0,
        10.0,
        1.0,
    ));

    
    let parameters = Parameters {delta_time: 0.0, updates_per_frame: 1, angular_velocity: true, camera_pos: (0.0, 0.0, 0.0, -6.0), gravity: false, world_size: 300.0 };
    start(conf, move || Box::new(World::new(vec![], springs, parameters)));
}