use ferrum_engine::*;
use ferrum_engine::spring::Spring;

fn main() {
    let mut polygons = vec![];
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2::new(0.0, 1.0), 1.0, 1.0, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2::new(0.0, -1.0), 1.0, 1.0, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2::new(-3.0, -5.0), 1.0, 1.0, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2::new(-3.0, 5.0), 1.0, 1.0, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2::new(2.5, 3.0), 1.0, 1.0, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2::new(3.5, -3.0), 1.0, 1.0, Color::random()));

    let mut springs= vec![];
    springs.push(Spring::new(
        0,
        1,
        Vec2::new(0.0, -0.0),
        Vec2::new(0.0, 0.0),
        7.0,
        10.0,
        1.0,
        &polygons,
    ));
    springs.push(Spring::new(
        2,
        3,
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, -0.0),
        2.0,
        10.0,
        2.0,
        &polygons,
    ));
    springs.push(Spring::new(
        4,
        5,
        Vec2::new(0.0, -0.0),
        Vec2::new(0.0, 0.0),
        5.0,
        10.0,
        1.0,
        &polygons,
    ));

    
    let parameters = Parameters {delta_time: 0.0, updates_per_frame: 1, angular_velocity: true, camera_pos: Vec4{ x: 0.0, y: 0.0, z: 0.0, w: -6.0}, gravity: false, world_size: 300.0 };
    run(polygons, springs, parameters).unwrap();
}