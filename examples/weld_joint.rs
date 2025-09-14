use glam::{Vec2, Vec4};
use ferrum_engine::{run, Camera, ColorRGBA, Parameters, Rigidbody, WeldJoint};

fn main() {
    let parameters = Parameters {
        delta_time: 0.0,
        updates_per_frame: 1,
        angular_velocity: false,
        initial_camera: Camera {
            camera_pos: Vec4::new(0.0, 0.0, 0.0, -5.0),
            scaling_factor: 10.0,
        },
        time_multiplier: 1.0,
        gravity: false,
        world_size: 300.0,
        gravity_force: Vec2::new(0.0, 0.0),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
        is_running: false,
    };
    let mut polygons = vec![];

    polygons.push(Rigidbody::polygon(
        4,
        0.3533,
        Vec2 {
            x: 0.0,
            y: 0.0,
        },
        1.0,
        1.01,
        ColorRGBA::random_hsl(),
    ));

    polygons.push(Rigidbody::polygon(
        32,
        0.3533,
        Vec2 {
            x: 0.4,
            y: 0.0,
        },
        1.0,
        1.01,
        ColorRGBA::random_hsl(),
    ));

    let mut weld_joints = vec![];

    weld_joints.push(
        WeldJoint::new(Vec2::new(0.2, 0.0), Vec2::new(-0.2, 0.0), &mut polygons, 1, 0)
    );



    run(polygons, vec![], weld_joints, vec![], parameters);
}