use glam::{Mat2, Vec2};
use crate::{ColorRGBA, Rigidbody, World};
use crate::body_builder::BodyBuilder;
use crate::collision_detection::sat_collision;
use crate::enums::{BodyType, InputMode};

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
                continue;
            }
            if self.springs[i].body_a > index {
                self.springs[i].body_a -= 1;
            }
            if self.springs[i].body_b > index {
                self.springs[i].body_b -= 1;
            }
            i += 1;
        }
        if self.mouse_spring.is_some() && self.mouse_spring.unwrap() > index {
            self.mouse_spring = Some(self.mouse_spring.unwrap() - 1);
        }

        Self::move_indices(&mut self.selected_polygon, index);
        Self::move_indices(&mut self.spring_polygon, index);
        Self::move_indices(&mut self.spawn_ghost_polygon, index);
    }

    fn move_indices(option: &mut Option<usize>, index: usize){
        if option.is_some() {
            if option.unwrap() > index {
                *option = Some(option.unwrap() - 1);
            }
            else if option.unwrap() == index {
                *option = None;
            }
        }
    }

    pub fn remove_spring(&mut self, index: usize) {
        self.springs.remove(index);
        if self.mouse_spring.is_some() {
            if self.mouse_spring.unwrap() == index{
                self.mouse_spring = None;
            }
        }
        if self.selected_spring.is_some() {
            self.selected_spring = None;
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
            Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, ColorRGBA::white());
        for i in 0..self.polygons.len() {
            let result = sat_collision(&self.polygons[i], &mouse_polygon);
            if result[1].y != 0.0 && (self.spawn_ghost_polygon == None || i != self.spawn_ghost_polygon.unwrap()) && (self.spring_polygon == None || i != self.spring_polygon.unwrap()) {
                polygon_index = Some(i);
                break;
            }
        }
        polygon_index
    }

    pub fn under_mouse_is_clear(&self) -> Option<usize>{
        let mut polygon_index = None;
        let position = self.get_mouse_world_position();
        let mut mouse_polygon =
            BodyBuilder::create_rigidbody(&self.spawn_parameters, &self.colors);
        mouse_polygon.translate(position);
        for i in 0..self.polygons.len() {
            let result = sat_collision(&self.polygons[i], &mouse_polygon);
            if result[1].y != 0.0 && (self.spawn_ghost_polygon == None || i != self.spawn_ghost_polygon?) && (self.spring_polygon == None || i != self.spring_polygon.unwrap()) {
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
            Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, ColorRGBA::white());
        for i in 0..self.springs.len() {
            let result = sat_collision(&mouse_spring, &self.springs[i].connector);
            if result[1].y != 0.0 && (self.spring_polygon == None || i != self.spring_polygon.unwrap()) {
                spring_index = Some(i);
                break;
            }
        }
        spring_index
    }

    pub fn create_mouse_ghost(&mut self) {
        if self.spawn_ghost_polygon.is_some() {
            self.polygons.remove(self.spawn_ghost_polygon.unwrap());
        }
        self.spawn_ghost_polygon = None;
        if self.input_mode == InputMode::Spawn && self.spawn_parameters.body_type != BodyType::Spring {
            self.polygons.push(BodyBuilder::create_rigidbody(&self.spawn_parameters, &self.colors));
            let length = self.polygons.len() - 1;
            let position = self.get_mouse_world_position();
            self.polygons[length].move_to(position);
            self.polygons[length].change_color(ColorRGBA::new(1.0, 1.0, 1.0, 0.3));
            self.polygons[length].collision = false;
            self.spawn_ghost_polygon = Some(length);
        }
    }
}

pub fn rotate(to_rotate: &mut Vec2, center: Vec2, angle: f32) -> &mut Vec2 {
    let rot = Mat2::from_angle(angle);
    *to_rotate = rot.mul_vec2(*to_rotate - center) + center;
    to_rotate
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
