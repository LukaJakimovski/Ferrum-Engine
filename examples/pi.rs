use ferrum_engine::*;

fn main() {
    let mut polygons = vec![
    ];
    polygons.push(Rigidbody::rectangle(5.0, 1000.0, Vec2{x: -5.0, y: 0.0}, f32::MAX / 10000000.0, 1.0, Color::orange()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2{x: 0.0, y: 0.0}, 1.0, 1.0, Color::blue()));
    polygons.push(Rigidbody::rectangle(0.5, 0.5, Vec2{x: 5.0, y: 0.0}, 10000000000.0, 1.0, Color::new(0.0, 0.7, 1.0)));
    polygons[2].velocity = Vec2{x: -0.0005, y: 0.0};
    
    let parameters = Parameters {delta_time: 0.003, updates_per_frame: 10000, angular_velocity: false, camera_pos: Vec4{ x: 0.0, y: 0.0, z: 0.0, w: -5.0}, gravity: false, world_size: 300.0, gravity_force: Vec2::new(0.0, 0.0)  };
    run(polygons, vec![], parameters).unwrap();
}
