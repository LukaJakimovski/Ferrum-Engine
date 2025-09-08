use std::ops::Range;
use ferrum_engine::*;
use ferrum_engine::color::create_palette;

fn main() {
    let mut polygons = vec![];
    let palette = create_palette(10, Range {start: 0.025, end: 0.05}, Range {start: 0.025, end: 0.05},Range {start: 0.025, end: 0.05});
    for i in 0..64 {
        for j in 0..64 {
            polygons.push(Rigidbody::polygon(
                rand::random::<u32>() % 3 + 3,
                0.3533,
                Vec2 {
                    x: i as f32 * 0.5,
                    y: j as f32 * 0.5,
                },
                1.0,
                1.01,
                ColorRGBA::random_from_palette(&palette),
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
        camera_pos: Vec4::new(9.0, 9.0, 0.0, -50.0),
        time_multiplier: 1.0,
        gravity: false,
        world_size: 300.0,
        gravity_force: Vec2::new(0.0, -9.81),
        clear_color: ColorRGBA::new(0.0, 0.0, 0.0, 1.0),
    };
    run(polygons, vec![], parameters).unwrap();
}
