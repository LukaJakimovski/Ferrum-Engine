pub struct Timing{
    pub start_time: f64,
    pub frame_count: u32,
    pub timer: f64,
    pub fps: f64,
}


impl Timing {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn now() -> f64 {
        use std::time::SystemTime;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| panic!("{}", e));
        time.as_secs_f64()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn now() -> f32 {
        use crate::native;

        unsafe { native::wasm::now() }
    }
}
