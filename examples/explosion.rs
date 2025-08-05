use ferrum_engine::*;

fn main() {
    let mut polygons = vec![];
    for i in 0..64 {
        for j in 0..64 {
            polygons.push(Rigidbody::polygon(
                rand::random::<u32>() % 3 + 3,
                0.3533,
                Vec2 {
                    x: i as f32 * 0.3,
                    y: j as f32 * 0.3,
                },
                1.0,
                1.01,
                Color::random(),
            ));
        }
    }
    for i in 0..polygons.len() {
        polygons[i].velocity = Vec2 {
            x: (rand::random::<f32>() * 2.0 - 1.0) * 10.0,
            y: (rand::random::<f32>() * 2.0 - 1.0) * 10.0,
        };
        polygons[i].angular_velocity = (rand::random::<f32>() * 2.0 - 1.0) * 500.0;
    }
    let parameters = Parameters {
        delta_time: 0.0,
        updates_per_frame: 1,
        angular_velocity: true,
        camera_pos: Vec4 {
            x: 9.0,
            y: 9.0,
            z: 0.0,
            w: -50.0,
        },
        gravity: false,
        world_size: 10000.0,
        gravity_force: Vec2::new(0.0, -9.81),
    };
    run(polygons, vec![], parameters).unwrap();
}
