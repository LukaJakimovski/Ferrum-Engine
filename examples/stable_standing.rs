use ferrum_engine::*;

fn main() {
    let mut polygons = vec![];
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: -1.0, y: 5.0}, 1.0, 0.6, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: 1.0, y: 5.0}, 1.0, 0.6, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: -2.0, y: 5.0}, 1.0, 0.6, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: 2.0, y: 5.0}, 1.0, 0.6, Color::random()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2 { x: -0.00001, y: 5.0}, 1.0, 0.6, Color::random()));
    polygons.push(Rigidbody::rectangle(1000.0, 1.0, Vec2 { x: 1.0, y: 0.0}, f32::MAX / 100000000000.0, 0.6, Color::random()));
    polygons[5].rotate(-1.0);
    for polygon in &mut polygons {
        polygon.rotate(1.0);
    }
    let parameters = Parameters {delta_time: 0.00001, updates_per_frame: 25, angular_velocity: true, camera_pos: Vec4{ x: 0.0, y: 2.5, z: 0.0, w: -3.0}, gravity: true, world_size: 300.0 };
    run(polygons, vec![], parameters).unwrap();
}