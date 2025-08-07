use ferrum_engine::{run, Parameters, Vec2, Vec4};

fn main(){
    let parameters = Parameters {
        delta_time: 0.0,
        updates_per_frame: 1,
        angular_velocity: false,
        camera_pos: Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: -5.0,
        },
        gravity: false,
        world_size: 300.0,
        gravity_force: Vec2::new(0.0, 0.0),
    };
    run(vec![], vec![], parameters).unwrap();
}