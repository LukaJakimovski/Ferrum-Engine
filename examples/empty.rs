use ferrum_engine::{run, ColorRGBA, Parameters, Vec2, Vec4};

fn main(){
    let parameters = Parameters {
        delta_time: 0.0,
        updates_per_frame: 1,
        angular_velocity: false,
        camera_pos: Vec4::new(0.0, 0.0, 0.0, -5.0),
        time_multiplier: 1.0,
        gravity: false,
        world_size: 300.0,
        gravity_force: Vec2::new(0.0, 0.0),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
    };
    run(vec![], vec![], parameters).unwrap();
}