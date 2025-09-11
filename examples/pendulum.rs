use glam::{Vec2, Vec4};
use ferrum_engine::{run, Camera, ColorRGBA, Parameters, PivotJoint, Rigidbody, WeldJoint};

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
        gravity_force: Vec2::new(0.0, -9.81),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
        is_running: false,
    };
    let mut polygons = vec![];

    polygons.push(Rigidbody::rectangle(
        0.05,
        0.05,
        Vec2 {
            x: 0.0,
            y: 0.0,
        },
        f32::MAX / 10000.0,
        1.01,
        ColorRGBA::random_oklab(),
    ));
    polygons[0].collision = false;
    polygons[0].gravity_multiplier = 0.0;

    polygons.push(Rigidbody::rectangle(
        0.25,
        2.0,
        Vec2 {
            x: 0.0,
            y: 1.0,
        },
        1.0,
        1.01,
        ColorRGBA::random_oklab(),
    ));
    polygons[1].collision = false;

    let mut pivot_joints = vec![];

    pivot_joints.push(
        PivotJoint::new(Vec2::new(0.0, 0.0), Vec2::new(0.0, -1.0), 0, 1)
    );

    polygons.push(Rigidbody::rectangle(
        2.0,
        0.25,
        Vec2 {
            x: 1.0,
            y: 2.0,
        },
        1.0,
        1.01,
        ColorRGBA::random_oklab(),
    ));

    pivot_joints.push(
        PivotJoint::new(Vec2::new(0.0, 1.0), Vec2::new(-1.0, 0.0), 1, 2)
    );




    run(polygons, vec![], vec![], pivot_joints, parameters).unwrap();
}