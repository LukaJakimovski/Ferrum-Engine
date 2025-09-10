use ferrum_engine::*;

fn main() {
    let mut polygons = vec![];
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2 { x: -1.0, y: 5.0 },
        1.0,
        0.6,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2 { x: 1.0, y: 5.0 },
        1.0,
        0.6,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2 { x: -2.0, y: 5.0 },
        1.0,
        0.6,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2 { x: 2.0, y: 5.0 },
        1.0,
        0.6,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2 {
            x: -0.00001,
            y: 5.0,
        },
        1.0,
        0.6,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        1000.0,
        1.0,
        Vec2 { x: 1.0, y: 0.0 },
        f32::MAX / 100000000000.0,
        0.6,
        ColorRGBA::random_hsl(),
    ));
    polygons[5].rotate(-1.0);
    polygons[5].gravity_multiplier = 0.0;
    for polygon in &mut polygons {
        polygon.rotate(1.0);
    }
    let parameters = Parameters {
        delta_time: 0.00001,
        updates_per_frame: 25,
        angular_velocity: true,
        initial_camera: Camera {
            camera_pos: Vec4::new(0.0, 2.5, 0.0, -3.0),
            scaling_factor: 10.0,
        },
        time_multiplier: 1.0,
        gravity: true,
        world_size: 300.0,
        gravity_force: Vec2::new(0.0, -9.81),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
        is_running: false,
    };
    run(polygons, vec![], vec![], parameters).unwrap();
}
