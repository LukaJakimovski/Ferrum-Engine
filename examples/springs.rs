use ferrum_engine::*;

fn main() {
    let mut polygons = vec![];
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2::new(0.0, 1.0),
        1.0,
        1.0,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2::new(0.0, -1.0),
        1.0,
        1.0,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2::new(-3.0, -5.0),
        1.0,
        1.0,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2::new(-3.0, 5.0),
        1.0,
        1.0,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2::new(2.5, 3.0),
        1.0,
        1.0,
        ColorRGBA::random_hsl(),
    ));
    polygons.push(Rigidbody::rectangle(
        0.5,
        0.5,
        Vec2::new(3.5, -3.0),
        1.0,
        1.0,
        ColorRGBA::random_hsl(),
    ));

    let mut springs = vec![];
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

    let parameters = Parameters {
        delta_time: 0.0,
        updates_per_frame: 1,
        angular_velocity: true,
        initial_camera: Camera {
            camera_pos: Vec4::new(0.0, 0.0, 0.0, -6.0),
            scaling_factor: 10.0,
        },
        time_multiplier: 1.0,
        gravity: false,
        world_size: 300.0,
        gravity_force: Vec2::new(0.0, -9.81),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
        is_running: false,
    };
    run(polygons, springs, vec![], vec![], parameters);
}
