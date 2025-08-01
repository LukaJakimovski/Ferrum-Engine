use ferrum_engine::*;
use ferrum_engine::spring::Spring;

fn main() {
    let conf = conf::Conf {
        window_title: "Bouncy Castle Example".to_string(),
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
    let mut rigidbodies = vec![];
    rigidbodies.push(Rigidbody::rectangle(
        10.0,
        0.5,
        Vec2::new(0.0, -6.0),
        f32::MAX / 1000.0,
        0.9,
        Color::random(),
    ));
    rigidbodies.push(Rigidbody::rectangle(
        10.0,
        0.5,
        Vec2::new(0.0, 0.0),
        f32::MAX / 10000000000.0,
        1.1,
        Color::random(),
    ));

    let mut springs= vec![];
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(5.0, 0.0),
        Vec2::new(5.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(4.0, 0.0),
        Vec2::new(4.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(3.0, 0.0),
        Vec2::new(3.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(-1.0, 0.0),
        Vec2::new(-1.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(-2.0, 0.0),
        Vec2::new(-2.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(-3.0, 0.0),
        Vec2::new(-3.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(-4.0, 0.0),
        Vec2::new(-4.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(-5.0, 0.0),
        Vec2::new(-5.0, 0.0),
        3.5,
        10000000000000000000000000000.0,
        0.0,
        &rigidbodies,
    ));
    let parameters = Parameters {delta_time: 0.0, updates_per_frame: 1, angular_velocity: true, camera_pos: (0.0, 0.0, 0.0, -6.0), gravity: true, world_size: 500.0 };
    start(conf, move || Box::new(World::new(rigidbodies, springs, parameters)));
}