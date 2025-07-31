use miniquad::{window, KeyCode, KeyMods, MouseButton};
use crate::enums::Keys;
use crate::{Color, Rigidbody, Vec2, World};
use crate::collision_detection::sat_collision;

impl World{
    pub fn handle_input(&mut self){
        if self.pressed_keys[Keys::R as usize] == 1 {
            for i in 0..self.polygons.len() {
                self.polygons[i].angular_velocity = 5.0;
            }
        }

        if self.pressed_keys[Keys::W as usize] == 1 {self.camera_pos.1 += 5.0 * self.delta_time as f32;}
        if self.pressed_keys[Keys::A as usize] == 1 {self.camera_pos.0 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[Keys::S as usize] == 1 {self.camera_pos.1 -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[Keys::D as usize] == 1 {self.camera_pos.0 += 5.0 * self.delta_time as f32;}
        if self.pressed_keys[Keys::P as usize] == 1 {
            self.is_running = true;
        }
        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - window::screen_size().0)/ window::screen_size().0 + self.camera_pos.0 / (-self.camera_pos.3 + 1.0)) * (-self.camera_pos.3 + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - window::screen_size().1)/ window::screen_size().1 + self.camera_pos.1 / -(-self.camera_pos.3 + 1.0)) * -(-self.camera_pos.3 + 1.0) * window::screen_size().1 / window::screen_size().0,};
        if self.pressed_keys[Keys::L as usize] == 1 {
            self.polygons.push(Rigidbody::polygon(16, 0.3533, position.clone(), 1.0, 1.0, Color::random()));
        }
    }

    pub fn mouse_motion_eventhandler(&mut self, _x: f32, _y: f32) {
        self.mouse_pos = (_x, _y);
    }

    pub fn mouse_wheel_eventhandler(&mut self, _x: f32, _y: f32) {
        if self.pressed_keys[6] == 1 {
            self.scaling_factor += _y * 0.1;
            self.scaling_factor += _x * 0.1;
            println!("Scaling factor: {}", self.scaling_factor);
        }
        else {
            self.camera_pos.3 += _y * 0.1 * self.scaling_factor;
            self.camera_pos.3 += _x * 0.1 * self.scaling_factor;
        }
    }
    pub fn mouse_button_down_eventhandler(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - window::screen_size().0)/ window::screen_size().0 + self.camera_pos.0 / (-self.camera_pos.3 + 1.0)) * (-self.camera_pos.3 + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - window::screen_size().1)/ window::screen_size().1 + self.camera_pos.1 / -(-self.camera_pos.3 + 1.0)) * -(-self.camera_pos.3 + 1.0) * window::screen_size().1 / window::screen_size().0,};
        if _button == MouseButton::Left {
            self.pressed_buttons[0] = 1;
            self.polygons.push(Rigidbody::polygon(16, 0.3533, position.clone(), 1.0, 1.0, Color::random()));
            let length = self.polygons.len();
            self.polygons[length - 1].restitution = 0.95;
        }
        if _button == MouseButton::Right {
            self.pressed_buttons[1] = 1;
            let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
            for i in 0..self.polygons.len() {
                let result = sat_collision(&self.polygons[i], &mouse_polygon);
                if result[1].y != 0.0{
                    self.polygons.remove(i);
                    break;
                }
            }
        }
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 1 }
    }
    pub fn mouse_button_up_eventhandler(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        if _button == MouseButton::Middle { self.pressed_buttons[2] = 0 }
        if _button == MouseButton::Right { self.pressed_buttons[1] = 0 }
        if _button == MouseButton::Left { self.pressed_buttons[0] = 0 }
    }
    pub fn key_down_eventhandler(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match _keycode {
            KeyCode::Key1 => window::show_mouse(false),
            KeyCode::Key2 => window::show_mouse(true),
            _ => (),
        }
        if _keycode == KeyCode::W{self.pressed_keys[Keys::W as usize] = 1 }
        if _keycode == KeyCode::A{self.pressed_keys[Keys::A as usize] = 1 }
        if _keycode == KeyCode::S{self.pressed_keys[Keys::S as usize] = 1 }
        if _keycode == KeyCode::D{self.pressed_keys[Keys::D as usize] = 1 }
        if _keycode == KeyCode::R{self.pressed_keys[Keys::R as usize] = 1 }
        if _keycode == KeyCode::P{self.pressed_keys[Keys::P as usize] = 1 }
        if _keycode == KeyCode::LeftControl || _keycode == KeyCode::RightControl { self.pressed_keys[Keys::LeftCtrl as usize] = 1 }
        if _keycode == KeyCode::L{self.pressed_keys[Keys::L as usize] = 1 }
    }

    pub fn key_up_eventhandler(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        if _keycode == KeyCode::W{self.pressed_keys[Keys::W as usize] = 0 }
        if _keycode == KeyCode::A{self.pressed_keys[Keys::A as usize] = 0 }
        if _keycode == KeyCode::S{self.pressed_keys[Keys::S as usize] = 0}
        if _keycode == KeyCode::D{self.pressed_keys[Keys::D as usize] = 0 }
        if _keycode == KeyCode::R{self.pressed_keys[Keys::R as usize] = 0 }
        //if _keycode == KeyCode::P{self.pressed_keys[Keys::P] = 0 }
        if _keycode == KeyCode::LeftControl || _keycode == KeyCode::RightControl { self.pressed_keys[Keys::LeftCtrl as usize] = 0 }
        if _keycode == KeyCode::L{self.pressed_keys[Keys::L as usize] = 0 }
    }

    pub fn raw_mouse_motionhandler(&mut self, _dx: f32, _dy: f32) {
        if self.pressed_buttons[2] == 1 {
            self.camera_pos.0 -= _dx * (-self.camera_pos.3 + 1.0) / window::screen_size().0;
            self.camera_pos.1 += _dy *  (-self.camera_pos.3 + 1.0) / window::screen_size().1;
        }
    }
}