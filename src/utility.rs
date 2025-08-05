use crate::World;

impl World{
    pub fn remove_rigidbody(&mut self, index: usize){
        self.polygons.remove(index);
        for i in 0..self.springs.len(){
            if self.springs[i].body_a == index || self.springs[i].body_b == index{
                self.springs.remove(i);
            }
            else if self.springs[i].body_a > index{
                self.springs[i].body_a -= 1;
            }
            else if self.springs[i].body_b > index{
                self.springs[i].body_b -= 1;
            }
        }
        for i in 0..self.temp_springs.len(){
            if self.temp_springs[i] > index{
                self.temp_springs[i] -= 1;
            }
        }
        for i in 0..self.temp_polygons.len(){
            if self.temp_polygons[i] > index{
                self.temp_polygons[i] -= 1;
            }
        }
    }
}

pub mod date {
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