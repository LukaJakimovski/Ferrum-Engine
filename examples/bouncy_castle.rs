use ferrum_engine::*;
use ferrum_engine::spring::Spring;

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
    let mut rigidbodies = vec![];
    /*
    rigidbodies.push(Rigidbody::rectangle(
        6.0,
        0.5,
        Vec2::new(0.0, 0.0)
    ));
    */
    rigidbodies.push(Rigidbody::rectangle(
        60.0,
        0.5,
        Vec2::new(0.0, -6.0)
    ));
    rigidbodies[0].mass = f32::MAX / 10000000.0;
    rigidbodies[0].calculate_moment_of_inertia();

    let mut springs= vec![];
    springs.push(Spring::new(
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(2.5, -0.0)),
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(2.5, -1.5)),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        5.0,
        9.0,
        0.0,
    ));
    springs.push(Spring::new(
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(-2.5, -0.0)),
        Rigidbody::rectangle(0.5, 0.5, Vec2::new(-2.5, -1.5)),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, -0.0),
        5.0,
        9.0,
        0.0,
    ));
    let parameters = Parameters {delta_time: 0.0001, updates_per_frame: 2, angular_velocity: true, camera_pos: (0.0, 0.0, 0.0, -6.0), gravity: true, world_size: 500.0 };
    start(conf, move || Box::new(World::new(rigidbodies, springs, parameters)));
}