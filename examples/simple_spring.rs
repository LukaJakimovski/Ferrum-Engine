use ferrum_engine::*;
use ferrum_engine::spring::Spring;

fn main() {
    let mut rigidbodies = vec![];
    rigidbodies.push(Rigidbody::rectangle(
        0.01,
        0.01,
        Vec2::new(0.0, 0.0),
        f32::MAX / 100000000000000.0,
        1.0,
        Color::white(),
    ));
    rigidbodies[0].collision = false;
    rigidbodies.push(Rigidbody::polygon(
        64,
        3.533,
        Vec2::new(25.0, 0.0),
        50000.0,
        1.0,
        Color::white(),
    ));

    let mut springs= vec![];
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        0.0,
        1000000.0,
        10.0,
        &rigidbodies,
    ));

    let parameters = Parameters {delta_time: 0.0, updates_per_frame: 1, angular_velocity: true, camera_pos: Vec4{ x: 0.0, y: 0.0, z: 0.0, w: -1.0}, gravity: false, world_size: 100.0, gravity_force: Vec2::new(0.0, -9.81)  };
    run(rigidbodies, springs, parameters).unwrap();
}