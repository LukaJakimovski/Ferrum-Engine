use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;
use crate::enums::{Keys, Mouse};
use crate::{Color, Rigidbody, Vec2, World};
use crate::collision_detection::sat_collision;

impl World {
    pub(crate) fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: winit::keyboard::KeyCode, _pressed: bool) {

        match key {
            winit::keyboard::KeyCode::KeyW | winit::keyboard::KeyCode::ArrowUp => {
                self.camera_pos.y += 5.0 * self.delta_time as f32;
            },
            winit::keyboard::KeyCode::KeyA | winit::keyboard::KeyCode::ArrowLeft => {
                self.camera_pos.x -= 5.0 * self.delta_time as f32;
            },
            winit::keyboard::KeyCode::KeyS | winit::keyboard::KeyCode::ArrowDown => {
                self.camera_pos.y -= 5.0 * self.delta_time as f32;
            },
            winit::keyboard::KeyCode::KeyD | winit::keyboard::KeyCode::ArrowRight => {
                self.camera_pos.x += 5.0 * self.delta_time as f32;
            },
            winit::keyboard::KeyCode::KeyR => {
                for i in 0..self.polygons.len() {
                    self.polygons[i].angular_velocity = 5.0;
                }
            },
            winit::keyboard::KeyCode::KeyP => {
                self.is_running = true;
            },
            winit::keyboard::KeyCode::KeyL => {
                let size = self.window.inner_size();
                if _pressed {self.pressed_keys[Keys::L as usize] = 1}
                else if !_pressed {self.pressed_keys[Keys::L as usize] = 0}
                let position = Vec2 {
                    x: ((self.mouse_pos.0 * 2.0 - size.width as f32) / size.width as f32+ self.camera_pos.x / (-self.camera_pos.w + 1.0)) * (-self.camera_pos.w + 1.0),
                    y: ((self.mouse_pos.1 * 2.0 - size.height as f32) /size.height as f32 + self.camera_pos.y / -(-self.camera_pos.w + 1.0)) * -(-self.camera_pos.w + 1.0) * size.height as f32 / size.width as f32};
                if self.pressed_keys[Keys::L as usize] == 1 {
                    self.polygons.push(Rigidbody::polygon(32, 0.3533, position.clone(), 1.0, 1.0, Color::random()));
                }
            }
            winit::keyboard::KeyCode::KeyM => {
                for i in 0..self.polygons.len() {
                    self.polygons[i].angular_velocity = 0.0;
                    self.polygons[i].velocity = Vec2::zero();
                }
            },
            winit::keyboard::KeyCode::Escape => {
                event_loop.exit();
            },
            _ => {},
        }
    }
    pub(crate) fn handle_scroll(&mut self, delta: MouseScrollDelta) {
        let change = match delta {
            MouseScrollDelta::LineDelta(_, y) => y,
            MouseScrollDelta::PixelDelta(pos) => {
                pos.y as f32 / 20.0
            }
        };
        println!("{}", self.is_running);
        self.camera_pos.w += change / 10.0;
    }

    pub fn handle_cursor_movement(&mut self, position: PhysicalPosition<f64>){
        let dx = position.x as f32 - self.mouse_pos.0;
        let dy =  position.y as f32 - self.mouse_pos.1;
        let size = self.window.inner_size();
        if self.pressed_buttons[Mouse::Middle as usize] == 1 {
            self.camera_pos.x -= dx * 2.0 * (-self.camera_pos.w + 1.0) / size.width as f32;
            self.camera_pos.y += dy * 2.0 * (-self.camera_pos.w + 1.0) / size.height as f32;
        }
        self.mouse_pos = (position.x as f32, position.y as f32).into();
    }

    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        let size = self.window.inner_size();
        let new_pos = (-self.camera_pos.w + 1.0) * 1.0;
        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - size.width as f32) / size.width as f32 + self.camera_pos.x / new_pos) * new_pos,
            y: ((self.mouse_pos.1 * 2.0 - size.height as f32)/ size.height as f32 + self.camera_pos.y / new_pos) * -new_pos * size.height as f32 / size.width as f32};
        if button == MouseButton::Left {
            if state.is_pressed(){
                self.pressed_buttons[Mouse::Left as usize] = 1;
                self.polygons.push(Rigidbody::polygon(16, 0.3533, position.clone(), 1.0, 0.8, Color::random()));
            }
            else { self.pressed_buttons[Mouse::Left as usize] = 0; }

        }
        if button == MouseButton::Right {
            if state.is_pressed(){ self.pressed_buttons[Mouse::Right as usize] = 1; }
            else { self.pressed_buttons[Mouse::Right as usize] = 0; }
            let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
            for i in 0..self.polygons.len() {
                let result = sat_collision(&self.polygons[i], &mouse_polygon);
                if result[1].y != 0.0{
                    self.remove_rigidbody(i);
                    break;
                }
            }
        }
        if button == MouseButton::Middle {
            if state.is_pressed(){ self.pressed_buttons[Mouse::Middle as usize] = 1; }
            else { self.pressed_buttons[Mouse::Middle as usize] = 0; }
        }
    }
}