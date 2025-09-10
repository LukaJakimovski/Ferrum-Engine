use glam::{Mat2, Vec2};
use crate::{ ColorRGBA, Rigidbody};
use crate::body_builder::BodyBuilder;
use crate::collision_detection::sat_collision;
use crate::enums::{BodyType, InputMode};
use crate::input::UiSystem;
use crate::physics::PhysicsSystem;

impl PhysicsSystem {
    pub fn remove_rigidbody(&mut self, index: usize, ui_system: &mut UiSystem) {
        self.polygons.remove(index);
        let mut i = 0;
        loop {
            if i >= self.springs.len() {
                break;
            }
            if  self.springs[i].body_a == index || self.springs[i].body_b == index {
                self.remove_spring(i, ui_system);
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
        if ui_system.mouse_spring.is_some() && ui_system.mouse_spring.unwrap() > index {
            ui_system.mouse_spring = Some(ui_system.mouse_spring.unwrap() - 1);
        }

        Self::move_indices(&mut ui_system.selected_polygon, index);
        Self::move_indices(&mut ui_system.spring_polygon, index);
        Self::move_indices(&mut ui_system.spawn_ghost_polygon, index);
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

    pub fn remove_spring(&mut self, index: usize, ui_system: &mut UiSystem) {
        self.springs.remove(index);
        if ui_system.mouse_spring.is_some() {
            if ui_system.mouse_spring.unwrap() == index{
                ui_system.mouse_spring = None;
            }
        }
        if ui_system.selected_spring.is_some() {
            ui_system.selected_spring = None;
        }
    }
}


impl UiSystem {
    pub fn get_mouse_world_position(&self) -> Vec2 {
        Vec2 {
            x: ((self.mouse_pos.x * 2.0 - self.window_dimensions.x)
                / self.window_dimensions.x
                + self.camera.camera_pos.x / (-self.camera.camera_pos.w + 1.0))
                * (-self.camera.camera_pos.w + 1.0),
            y: ((self.mouse_pos.y * 2.0 - self.window_dimensions.y)
                / self.window_dimensions.y
                + self.camera.camera_pos.y / -(-self.camera.camera_pos.w + 1.0))
                * -(-self.camera.camera_pos.w + 1.0)
                * self.window_dimensions.y
                / self.window_dimensions.x,
        }
    }

    pub fn get_polygon_under_mouse(&self, physics_system: &mut PhysicsSystem) -> Option<usize>{
        let mut polygon_index = None;
        let position = self.get_mouse_world_position();
        let mouse_polygon =
            Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, ColorRGBA::white());
        for i in 0..physics_system.polygons.len() {
            let result = sat_collision(&physics_system.polygons[i], &mouse_polygon);
            if result[1].y != 0.0 && (self.spawn_ghost_polygon == None || i != self.spawn_ghost_polygon.unwrap()) && (self.spring_polygon == None || i != self.spring_polygon.unwrap()) {
                polygon_index = Some(i);
                break;
            }
        }
        polygon_index
    }

    pub fn under_mouse_is_clear(&self, physics_system: &mut PhysicsSystem) -> bool{
        let position = self.get_mouse_world_position();
        let mut mouse_polygon =
            BodyBuilder::create_rigidbody(&self.spawn_parameters, &None);
        mouse_polygon.translate(position);
        for i in 0..physics_system.polygons.len() {
            let result = sat_collision(&physics_system.polygons[i], &mouse_polygon);
            if result[1].y != 0.0 && (self.spawn_ghost_polygon == None || i != self.spawn_ghost_polygon.unwrap()) && (self.spring_polygon == None || i != self.spring_polygon.unwrap()) {
                return false;
            }
        }
        true
    }

    pub fn get_spring_under_mouse(&self, physics_system: &mut PhysicsSystem) -> Option<usize> {
        let mut spring_index = None;
        let position = self.get_mouse_world_position();
        let mouse_spring =
            Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, ColorRGBA::white());
        for i in 0..physics_system.springs.len() {
            let result = sat_collision(&mouse_spring, &physics_system.springs[i].connector);
            if result[1].y != 0.0 && (self.spring_polygon == None || i != self.spring_polygon.unwrap()) {
                spring_index = Some(i);
                break;
            }
        }
        spring_index
    }

    pub fn create_mouse_ghost(&mut self, physics_system: &mut PhysicsSystem) {
        if self.spawn_ghost_polygon.is_some() {
            physics_system.polygons.remove(self.spawn_ghost_polygon.unwrap());
        }
        self.spawn_ghost_polygon = None;
        if self.input_mode == InputMode::Spawn && self.spawn_parameters.body_type != BodyType::Spring {
            physics_system.polygons.push(BodyBuilder::create_rigidbody(&self.spawn_parameters, &None));
            let length = physics_system.polygons.len() - 1;
            let position = self.get_mouse_world_position();
            physics_system.polygons[length].move_to(position);
            physics_system.polygons[length].change_color(ColorRGBA::new(1.0, 1.0, 1.0, 0.3));
            physics_system.polygons[length].collision = false;
            self.spawn_ghost_polygon = Some(length);
        }
    }
}

pub fn rotate(to_rotate: &mut Vec2, center: Vec2, angle: f32) -> &mut Vec2 {
    let rot = Mat2::from_angle(angle);
    *to_rotate = rot.mul_vec2(*to_rotate - center) + center;
    to_rotate
}
