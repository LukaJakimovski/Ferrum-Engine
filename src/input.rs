use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;
use crate::enums::{BodyType, InputMode, Keys, Mouse};
use crate::{Color, Rigidbody, Vec2, World};
use crate::body_builder::BodyBuilder;
use crate::collision_detection::sat_collision;
use crate::spring::Spring;

impl World {
    pub(crate) fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: winit::keyboard::KeyCode, _pressed: bool) {

        match key {
            winit::keyboard::KeyCode::KeyW | winit::keyboard::KeyCode::ArrowUp => {
                if _pressed {self.pressed_keys[Keys::W as usize] = 1}
                else if !_pressed {self.pressed_keys[Keys::W as usize] = 0}
            },
            winit::keyboard::KeyCode::KeyA | winit::keyboard::KeyCode::ArrowLeft => {
                if _pressed {self.pressed_keys[Keys::A as usize] = 1}
                else if !_pressed {self.pressed_keys[Keys::A as usize] = 0}
            },
            winit::keyboard::KeyCode::KeyS | winit::keyboard::KeyCode::ArrowDown => {
                if _pressed {self.pressed_keys[Keys::S as usize] = 1}
                else if !_pressed {self.pressed_keys[Keys::S as usize] = 0}
            },
            winit::keyboard::KeyCode::KeyD | winit::keyboard::KeyCode::ArrowRight => {
                if _pressed {self.pressed_keys[Keys::D as usize] = 1}
                else if !_pressed {self.pressed_keys[Keys::D as usize] = 0}
            },
            winit::keyboard::KeyCode::KeyR => {
                if _pressed {self.pressed_keys[Keys::R as usize] = 1}
                else if !_pressed {self.pressed_keys[Keys::R as usize] = 0}
                if _pressed {
                    for i in 0..self.polygons.len() {
                        self.polygons[i].angular_velocity = 20.0;
                    }
                }
            },
            winit::keyboard::KeyCode::KeyP => {
                if _pressed && self.pressed_keys[Keys::P as usize] == 0 {self.pressed_keys[Keys::P as usize] = 1; self.is_running = true;}
                else if _pressed && self.pressed_keys[Keys::P as usize] == 1 {self.pressed_keys[Keys::P as usize] = 0; self.is_running = false;}
            },
            winit::keyboard::KeyCode::KeyL => {
                if _pressed {self.pressed_keys[Keys::L as usize] = 1}
                else if !_pressed {self.pressed_keys[Keys::L as usize] = 0}
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
    pub fn handle_input(&mut self){
        let size = self.window.inner_size();
        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - size.width as f32) / size.width as f32+ self.camera_pos.x / (-self.camera_pos.w + 1.0)) * (-self.camera_pos.w + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - size.height as f32) /size.height as f32 + self.camera_pos.y / -(-self.camera_pos.w + 1.0)) * -(-self.camera_pos.w + 1.0) * size.height as f32 / size.width as f32};
        if self.pressed_keys[Keys::L as usize] == 1 {
            self.polygons.push(BodyBuilder::create_rigidbody(&self.spawn_parameters));
            let length = self.polygons.len() - 1;
            self.polygons[length].translate(position);
        }

        if self.pressed_keys[Keys::W as usize] == 1 {self.camera_pos.y += 5.0 * self.delta_time as f32;}
        if self.pressed_keys[Keys::A as usize] == 1 {self.camera_pos.x -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[Keys::S as usize] == 1 {self.camera_pos.y -= 5.0 * self.delta_time as f32;}
        if self.pressed_keys[Keys::D as usize] == 1 {self.camera_pos.x += 5.0 * self.delta_time as f32;}

        if self.pressed_buttons[Mouse::Left as usize] == 1 && (self.input_mode == InputMode::Drag || (self.input_mode == InputMode::Select && self.spawn_parameters.body_type == BodyType::Spring)) {
            if self.selected_polygon.is_some(){
                let position = Vec2 {
                    x: ((self.mouse_pos.0 * 2.0 - self.config.width as f32) / self.config.width as f32+ self.camera_pos.x / (-self.camera_pos.w + 1.0)) * (-self.camera_pos.w + 1.0),
                    y: ((self.mouse_pos.1 * 2.0 - self.config.height as f32) /self.config.height as f32 + self.camera_pos.y / -(-self.camera_pos.w + 1.0)) * -(-self.camera_pos.w + 1.0) * self.config.height as f32 / self.config.width as f32};
                let mut mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1000.0, 1.0, Color::random());
                mouse_polygon.collision = false;
                let selected_polygon = &mut self.polygons[self.selected_polygon.unwrap()];
                if !self.dragging{
                    self.anchor_pos = mouse_polygon.center - selected_polygon.center;
                    self.polygons.push(mouse_polygon);
                    let length = self.polygons.len() - 1;
                    let spring = Spring::new(self.selected_polygon.unwrap(), length, self.anchor_pos, Vec2::zero(), 0.0, 10.0, 0.0, &self.polygons);
                    self.springs.push(spring);
                    self.temp_springs.push(self.springs.len() - 1);
                    self.temp_polygons.push(self.polygons.len() - 1);
                }
                let index = self.temp_polygons[0];
                if index < self.polygons.len() {
                    let diff = position - self.polygons[index].center;
                    self.polygons[index].translate(diff);
                }
                self.dragging = true;

            }
        }
    }

    pub(crate) fn handle_scroll(&mut self, delta: MouseScrollDelta) {
        let change = match delta {
            MouseScrollDelta::LineDelta(_, y) => y,
            MouseScrollDelta::PixelDelta(pos) => {
                pos.y as f32 / 20.0
            }
        };
        self.camera_pos.w += change * self.scaling_factor/10.0;
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
        let position = Vec2 {
            x: ((self.mouse_pos.0 * 2.0 - size.width as f32) / size.width as f32+ self.camera_pos.x / (-self.camera_pos.w + 1.0)) * (-self.camera_pos.w + 1.0),
            y: ((self.mouse_pos.1 * 2.0 - size.height as f32) /size.height as f32 + self.camera_pos.y / -(-self.camera_pos.w + 1.0)) * -(-self.camera_pos.w + 1.0) * size.height as f32 / size.width as f32};
        if button == MouseButton::Left {
            if state.is_pressed() && !self.is_pointer_used && self.input_mode == InputMode::Spawn {
                self.pressed_buttons[Mouse::Left as usize] = 1;
                self.polygons.push(BodyBuilder::create_rigidbody(&self.spawn_parameters));
                let length = self.polygons.len() - 1;
                self.polygons[length].translate(position);
            } else if state.is_pressed() && !self.is_pointer_used && self.input_mode == InputMode::Select{
                self.pressed_buttons[Mouse::Left as usize] = 1;
                let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
                for i in 0..self.polygons.len() {
                    let result = sat_collision(&self.polygons[i], &mouse_polygon);
                    if result[1].y != 0.0{
                        self.selected_polygon = Some(i);
                        break;
                    }
                }
            } else if state.is_pressed() && !self.is_pointer_used && self.input_mode == InputMode::Drag {
                self.pressed_buttons[Mouse::Left as usize] = 1;
                let position = Vec2 {
                    x: ((self.mouse_pos.0 * 2.0 - self.config.width as f32) / self.config.width as f32 + self.camera_pos.x / (-self.camera_pos.w + 1.0)) * (-self.camera_pos.w + 1.0),
                    y: ((self.mouse_pos.1 * 2.0 - self.config.height as f32) / self.config.height as f32 + self.camera_pos.y / -(-self.camera_pos.w + 1.0)) * -(-self.camera_pos.w + 1.0) * self.config.height as f32 / self.config.width as f32
                };
                let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
                for i in 0..self.polygons.len() {
                    let result = sat_collision(&self.polygons[i], &mouse_polygon);
                    if result[1].y != 0.0 {
                        self.selected_polygon = Some(i);
                        break;
                    }
                }
            } else if state.is_pressed() && !self.is_pointer_used && self.input_mode == InputMode::Select && self.spawn_parameters.body_type == BodyType::Spring {
                self.pressed_buttons[Mouse::Left as usize] = 1;
                let position = Vec2 {
                    x: ((self.mouse_pos.0 * 2.0 - self.config.width as f32) / self.config.width as f32 + self.camera_pos.x / (-self.camera_pos.w + 1.0)) * (-self.camera_pos.w + 1.0),
                    y: ((self.mouse_pos.1 * 2.0 - self.config.height as f32) / self.config.height as f32 + self.camera_pos.y / -(-self.camera_pos.w + 1.0)) * -(-self.camera_pos.w + 1.0) * self.config.height as f32 / self.config.width as f32
                };
                let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
                for i in 0..self.polygons.len() {
                    let result = sat_collision(&self.polygons[i], &mouse_polygon);
                    if result[1].y != 0.0 {
                        self.selected_polygon = Some(i);
                        break;
                    }
                }
            } else if !state.is_pressed() && self.input_mode != InputMode::Select {
                for index in &self.temp_polygons{
                    self.polygons.remove(*index);
                }
                for index in &self.temp_springs{
                    self.springs.remove(*index);
                }
                self.temp_springs.clear();
                self.temp_polygons.clear();
                self.dragging = false;
                self.selected_polygon = None;
                self.pressed_buttons[Mouse::Left as usize] = 0;
            } else if !state.is_pressed() && self.input_mode == InputMode::Select {
                self.pressed_buttons[Mouse::Left as usize] = 1;
                let position = Vec2 {
                    x: ((self.mouse_pos.0 * 2.0 - self.config.width as f32) / self.config.width as f32 + self.camera_pos.x / (-self.camera_pos.w + 1.0)) * (-self.camera_pos.w + 1.0),
                    y: ((self.mouse_pos.1 * 2.0 - self.config.height as f32) / self.config.height as f32 + self.camera_pos.y / -(-self.camera_pos.w + 1.0)) * -(-self.camera_pos.w + 1.0) * self.config.height as f32 / self.config.width as f32
                };
                for polygon_index in &self.temp_polygons{
                    self.polygons.remove(*polygon_index);
                }
                let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
                let mut polygon2_index = None;
                for i in 0..self.polygons.len() {
                    let result = sat_collision(&self.polygons[i], &mouse_polygon);
                    if result[1].y != 0.0 {
                        polygon2_index = Some(i);
                        break;
                    }
                }
                if polygon2_index.is_some() && self.temp_springs.len() > 0 {
                    let polygon2 = &mut self.polygons[polygon2_index.unwrap()];

                    let anchor_pos = mouse_polygon.center - polygon2.center;
                    self.springs[self.temp_springs[0]].body_b = polygon2_index.unwrap();
                    self.springs[self.temp_springs[0]].anchor_b = anchor_pos;
                }
                else{
                    for spring_index in &self.temp_springs{
                        self.springs.remove(*spring_index);
                    }
                }
                self.temp_springs.clear();
                self.temp_polygons.clear();
                self.dragging = false;
                self.pressed_buttons[Mouse::Left as usize] = 0;
            } else { self.pressed_buttons[Mouse::Left as usize] = 0; }

        }
        if button == MouseButton::Right {
            if state.is_pressed() && !self.is_pointer_used {
                self.pressed_buttons[Mouse::Right as usize] = 1;
                let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, 1.0, 1.0, Color::random());
                for i in 0..self.polygons.len() {
                    let result = sat_collision(&self.polygons[i], &mouse_polygon);
                    if result[1].y != 0.0{
                        self.remove_rigidbody(i);
                        break;
                    }
                }
            }
            else { self.pressed_buttons[Mouse::Right as usize] = 0; }
        }
        if button == MouseButton::Middle {
            if state.is_pressed() && !self.is_pointer_used{ self.pressed_buttons[Mouse::Middle as usize] = 1; }
            else { self.pressed_buttons[Mouse::Middle as usize] = 0; }
        }
    }
}