use ferrum_engine::*;
use std::f32::consts::PI;

fn main() {
    let mut polygons = vec![];

    polygons.push(Rigidbody::rectangle(
        10.0,
        1.0,
        Vec2 { x: 0.0, y: 0.0 },
        f32::MAX / 10.0,
        0.6,
        ColorRGBA::orange(),
    ));
    polygons[0].rotate(PI / 4.0);
    polygons[0].gravity_multiplier = 0.0;

    for i in 0..10 {
        polygons.push(Rigidbody::polygon(
            32,
            0.3533,
            Vec2 {
                x: 2.5,
                y: 6.0 + i as f32 * 2.0,
            },
            1.0,
            1.0,
            ColorRGBA::random_hsl(),
        ));
    }

    let parameters = Parameters {
        delta_time: 0.0,
        updates_per_frame: 1,
        angular_velocity: true,
        initial_camera: Camera {
            camera_pos: Vec4::new(0.0, 0.0, 0.0, -10.0),
            scaling_factor: 10.0,
        },
        time_multiplier: 1.0,
        gravity: true,
        world_size: 300.0,
        gravity_force: Vec2::new(0.0, -9.81),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
        is_running: false,
    };
    run(polygons, vec![], vec![], vec![], parameters);
}
