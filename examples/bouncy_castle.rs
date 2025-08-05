use ferrum_engine::*;
use ferrum_engine::spring::Spring;

fn main() {
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

    let parameters = Parameters {delta_time: 0.0, updates_per_frame: 1, angular_velocity: true, camera_pos: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: -6.0 }, gravity: true, world_size: 500.0, gravity_force: Vec2::new(0.0, -9.81)  };
    run(rigidbodies, springs, parameters).unwrap();
}