use ferrum_engine::{run, Camera, ColorRGBA, Parameters, PivotJoint, Rigidbody, Spring, Vec2, Vec4, WeldJoint};

fn main(){
    let mut polygons = vec![];
    let mut springs: Vec<Spring> = vec![];
    let mut weld_joints: Vec<WeldJoint> = vec![];
    let mut pivot_joints: Vec<PivotJoint> = vec![];
    
    polygons.push(Rigidbody::rectangle(10.0, 0.5, Vec2::new(0.0, 0.0), 1000000.0, 0.0, ColorRGBA::random_hsl()));
    polygons.push(Rigidbody::polygon(3, 3.0, Vec2::new(0.0, 1.5), 100000.0, 0.0, ColorRGBA::random_hsl()));
    polygons[1].rotate(90f32.to_radians());
    polygons[1].gravity_multiplier = 0.0;
    polygons.push(Rigidbody::rectangle(20.0, 0.5, Vec2::new(-5.0, 4.5), 1.0, 0.0, ColorRGBA::random_hsl()));
    pivot_joints.push(PivotJoint::new(Vec2::new(0.0, 4.5), &mut polygons, 1, 2));
    //weld_joints.push(WeldJoint::new(Vec2::new(0.0, 0.0), &mut polygons, 0, 1));
    
    
    
    
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
    run(polygons, springs, weld_joints, pivot_joints, parameters);
}