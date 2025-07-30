use ferrum_engine::*;
use ferrum_engine::spring::Spring;

fn main() {
    let conf = conf::Conf {
        window_title: "Explosion Example".to_string(),
        window_height: 800,
        window_width: 800,
        high_dpi: false,
        sample_count: 4,
        window_resizable: true,
        icon: None,
        platform: Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        fullscreen: false
    };

    let mut springs= vec![];
    springs.push(Spring::new());
    
    let parameters = Parameters {delta_time: 0.000001, updates_per_frame: 25, angular_velocity: true, camera_pos: (0.0, 2.5, 0.0, -3.0), gravity: false };
    start(conf, move || Box::new(World::new(vec![], springs, parameters)));
}