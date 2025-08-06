use crate::{Color, Rigidbody, Vec2, World};
use crate::collision_detection::sat_collision;

impl World {
    pub fn remove_rigidbody(&mut self, index: usize) {
        self.polygons.remove(index);
        for i in 0..self.springs.len() {
            if self.springs[i].body_a == index || self.springs[i].body_b == index {
                self.remove_spring(i);
            } else if self.springs[i].body_a > index {
                self.springs[i].body_a -= 1;
            } else if self.springs[i].body_b > index {
                self.springs[i].body_b -= 1;
            }
        }
        for i in 0..self.temp_springs.len() {
            if self.temp_springs[i] > index {
                self.temp_springs[i] -= 1;
            }
        }
        for i in 0..self.temp_polygons.len() {
            if self.temp_polygons[i] > index {
                self.temp_polygons[i] -= 1;
            }
        }
    }

    pub fn remove_spring(&mut self, index: usize) {
        self.springs.remove(index);
        for i in 0..self.temp_springs.len() {
            if self.temp_springs[i] == index{
                self.temp_springs.remove(i);
            }
        }
    }

    pub fn get_mouse_world_position(&self) -> Vec2 {
        Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - self.config.width as f32)
                / self.config.width as f32
                + self.camera_pos.x / (-self.camera_pos.w + 1.0))
                * (-self.camera_pos.w + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - self.config.height as f32)
                / self.config.height as f32
                + self.camera_pos.y / -(-self.camera_pos.w + 1.0))
                * -(-self.camera_pos.w + 1.0)
                * self.config.height as f32
                / self.config.width as f32,
        }
    }

    pub fn get_polygon_under_mouse(&self) -> Option<usize>{
        let mut polygon_index = None;
        let position = self.get_mouse_world_position();
        let mouse_polygon =
            Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
        for i in 0..self.polygons.len() {
            let result = sat_collision(&self.polygons[i], &mouse_polygon);
            if result[1].y != 0.0 && (self.temp_polygons.len() == 0 || i != self.temp_polygons[0]) {
                polygon_index = Some(i);
                break;
            }
        }
        polygon_index
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
