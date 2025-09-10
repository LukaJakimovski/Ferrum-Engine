use ferrum_engine::*;

fn main() {
    let mut rigidbodies = vec![];
    rigidbodies.push(Rigidbody::rectangle(
        0.01,
        0.01,
        Vec2::new(0.0, 0.0),
        f32::MAX / 100000000000000.0,
        1.0,
        ColorRGBA::white(),
    ));
    rigidbodies[0].collision = false;
    rigidbodies[0].eternal = true;
    rigidbodies.push(Rigidbody::polygon(
        64,
        3.533,
        Vec2::new(25.0, 0.0),
        5000000.0,
        1.0,
        ColorRGBA::white(),
    ));
    rigidbodies[1].eternal = true;
    let mut springs = vec![];
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        0.0,
        500000000.0,
        10.0,
        &rigidbodies,
    ));

    let parameters = Parameters {
        delta_time: 0.0,
        updates_per_frame: 1,
        angular_velocity: true,
        initial_camera: Camera {
            camera_pos: Vec4::new(0.0, 0.0, 0.0, -30.0),
            scaling_factor: 10.0,
        },
        time_multiplier: 1.0,
        gravity: false,
        world_size: 100.0,
        gravity_force: Vec2::new(0.0, -9.81),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
        is_running: true,
    };
    run(rigidbodies, springs, vec![], parameters).unwrap();
}
