use crate::{Color, Rigidbody, Vec2, World};
use crate::collision_detection::sat_collision;

impl World {
    pub fn remove_rigidbody(&mut self, index: usize) {
        self.polygons.remove(index);
        let mut i = 0;
        loop {
            if i >= self.springs.len() {
                break;
            }
            if  self.springs[i].body_a == index || self.springs[i].body_b == index {
                self.remove_spring(i);
                if i > 0 {
                    i -= 1;
                }
                continue;
            }
            if self.springs[i].body_a > index {
                self.springs[i].body_a -= 1;
            }
            if self.springs[i].body_b > index {
                self.springs[i].body_b -= 1;
            }
            if self.springs[i].body_a == self.springs[i].body_b{
                self.remove_spring(i);
            }
            i += 1;
        }
        for i in 0..self.temp_springs.len() {
            if self.temp_springs[i] > index {
                self.temp_springs[i] -= 1;
            }
        }
        for i in 0..self.temp_polygons.len() {
            if self.temp_polygons[i] > index {
                self.temp_polygons[i] -= 1;
            } else if self.temp_polygons[i] == index {
                self.temp_polygons.remove(i);
            }

        }
        
        if self.selected_polygon.is_some() {
            if self.selected_polygon.unwrap() > index {
                self.selected_polygon = Some(self.selected_polygon.unwrap() - 1);
            }
            if self.selected_polygon.unwrap() == index {
                self.selected_polygon = None;
            }
        }
    }

    pub fn remove_spring(&mut self, index: usize) {
        self.polygons[self.springs[index].body_a].gravity_divider -= 1.0;
        self.polygons[self.springs[index].body_b].gravity_divider -= 1.0;
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
    
    pub fn get_spring_under_mouse(&self) -> Option<usize> {
        let mut spring_index = None;
        let position = self.get_mouse_world_position();
        let mouse_spring =
            Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
        for i in 0..self.springs.len() {
            let result = sat_collision(&mouse_spring, &self.springs[i].connector);
            if result[1].y != 0.0 && (self.temp_polygons.len() == 0 || i != self.temp_polygons[0]) {
                spring_index = Some(i);
                break;
            }
        }
        spring_index
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
