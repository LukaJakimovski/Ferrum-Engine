
use crate::body_builder::BodyBuilder;
use crate::enums::{BodyType, DraggingState, InputMode, Keys, Menu, Mouse};
use crate::spring::Spring;
use crate::{Color, Rigidbody, Vec2, World};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;

impl World {
    pub(crate) fn handle_key(
        &mut self,
        event_loop: &ActiveEventLoop,
        key: winit::keyboard::KeyCode,
        _pressed: bool,
    ) {
        match key {
            winit::keyboard::KeyCode::KeyW | winit::keyboard::KeyCode::ArrowUp => {
                if _pressed {
                    self.pressed_keys[Keys::W as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::W as usize] = 0
                }
            }
            winit::keyboard::KeyCode::KeyA | winit::keyboard::KeyCode::ArrowLeft => {
                if _pressed {
                    self.pressed_keys[Keys::A as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::A as usize] = 0
                }
            }
            winit::keyboard::KeyCode::KeyS | winit::keyboard::KeyCode::ArrowDown => {
                if _pressed {
                    self.pressed_keys[Keys::S as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::S as usize] = 0
                }
            }
            winit::keyboard::KeyCode::KeyD | winit::keyboard::KeyCode::ArrowRight => {
                if _pressed {
                    self.pressed_keys[Keys::D as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::D as usize] = 0
                }
            }
            winit::keyboard::KeyCode::KeyR => {
                if _pressed {
                    self.pressed_keys[Keys::R as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::R as usize] = 0
                }
                if _pressed {
                    for i in 0..self.polygons.len() {
                        self.polygons[i].angular_velocity = 20.0;
                    }
                }
            }
            winit::keyboard::KeyCode::KeyP => {
                if _pressed && self.pressed_keys[Keys::P as usize] == 0 {
                    self.pressed_keys[Keys::P as usize] = 1;
                    self.is_running = true;
                } else if _pressed && self.pressed_keys[Keys::P as usize] == 1 {
                    self.pressed_keys[Keys::P as usize] = 0;
                    self.is_running = false;
                }
            }
            winit::keyboard::KeyCode::KeyL => {
                if _pressed {
                    self.pressed_keys[Keys::L as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::L as usize] = 0
                }
            }
            winit::keyboard::KeyCode::KeyM => {
                for i in 0..self.polygons.len() {
                    self.polygons[i].angular_velocity = 0.0;
                    self.polygons[i].velocity = Vec2::zero();
                }
            }
            winit::keyboard::KeyCode::Digit1 => {
                if _pressed && self.pressed_keys[Keys::Num1 as usize] == 0 {
                    self.pressed_keys[Keys::Num1 as usize] = 1;
                    self.input_mode = InputMode::Spawn;
                } else if _pressed && self.pressed_keys[Keys::Num1 as usize] == 1 {
                    self.pressed_keys[Keys::Num1 as usize] = 0;
                    self.input_mode = InputMode::Spawn;
                }
            }
            winit::keyboard::KeyCode::Digit2 => {
                if _pressed && self.pressed_keys[Keys::Num2 as usize] == 0 {
                    self.pressed_keys[Keys::Num2 as usize] = 1;
                    self.input_mode = InputMode::Select;
                } else if _pressed && self.pressed_keys[Keys::Num2 as usize] == 1 {
                    self.pressed_keys[Keys::Num2 as usize] = 0;
                    self.input_mode = InputMode::Select;
                }
            }
            winit::keyboard::KeyCode::Digit3 => {
                if _pressed && self.pressed_keys[Keys::Num3 as usize] == 0 {
                    self.pressed_keys[Keys::Num3 as usize] = 1;
                    self.input_mode = InputMode::Drag;
                } else if _pressed && self.pressed_keys[Keys::Num3 as usize] == 1 {
                    self.pressed_keys[Keys::Num3 as usize] = 0;
                    self.input_mode = InputMode::Drag;
                }
            }
            winit::keyboard::KeyCode::Equal => {
                if _pressed {
                    self.pressed_keys[Keys::Plus as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::Plus as usize] = 0
                }
            }
            winit::keyboard::KeyCode::Minus => {
                if _pressed {
                    self.pressed_keys[Keys::Minus as usize] = 1
                } else if !_pressed {
                    self.pressed_keys[Keys::Minus as usize] = 0
                }
            }
            winit::keyboard::KeyCode::Escape => {
                event_loop.exit();
            }
            _ => {}
        }
    }
    pub fn handle_input(&mut self) {
        let position = self.get_mouse_world_position();
        if self.pressed_keys[Keys::L as usize] == 1 {
            self.polygons
                .push(BodyBuilder::create_rigidbody(&self.spawn_parameters));
            let length = self.polygons.len() - 1;
            self.polygons[length].translate(position);
        }

        if self.pressed_keys[Keys::W as usize] == 1 {
            self.camera_pos.y += 5.0 * self.delta_time as f32;
        }
        if self.pressed_keys[Keys::A as usize] == 1 {
            self.camera_pos.x -= 5.0 * self.delta_time as f32;
        }
        if self.pressed_keys[Keys::S as usize] == 1 {
            self.camera_pos.y -= 5.0 * self.delta_time as f32;
        }
        if self.pressed_keys[Keys::D as usize] == 1 {
            self.camera_pos.x += 5.0 * self.delta_time as f32;
        }
        if self.pressed_keys[Keys::Plus as usize] == 1 {
            self.camera_pos.w += 5.0 * self.scaling_factor * self.delta_time as f32;
        }
        if self.pressed_keys[Keys::Minus as usize] == 1 {
            self.camera_pos.w -= 5.0 * self.scaling_factor * self.delta_time as f32;
        }

        if self.pressed_buttons[Mouse::Left as usize] == 1
            && (self.input_mode == InputMode::Drag
                || (self.input_mode == InputMode::Spawn
                    && self.spawn_parameters.body_type == BodyType::Spring))
            &&  (self.dragging == DraggingState::Dragging
                || self.dragging == DraggingState::StartDragging)
        {
            if self.selected_polygon.is_some() {
                let position = self.get_mouse_world_position();
                let mut mouse_polygon =
                    Rigidbody::rectangle(0.03, 0.03, position, f32::MAX / 10000.0, 1.0, Color::random());
                mouse_polygon.collision = false;
                let selected_polygon;
                if self.selected_polygon.unwrap() < self.polygons.len(){
                    selected_polygon = &mut self.polygons[self.selected_polygon.unwrap()];
                } else {
                    self.selected_polygon = None;
                    return
                }

                if self.dragging == DraggingState::StartDragging {
                    self.anchor_pos = mouse_polygon.center - selected_polygon.center;
                    self.polygons.push(mouse_polygon);
                    let length = self.polygons.len() - 1;
                    let spring;
                    if self.input_mode == InputMode::Spawn {
                        spring = Spring::new(
                            self.selected_polygon.unwrap(),
                            length,
                            self.anchor_pos,
                            Vec2::zero(),
                            self.spawn_parameters.spring_params.rest_length,
                            self.spawn_parameters.spring_params.stiffness,
                            self.spawn_parameters.spring_params.dampening,
                            &self.polygons,
                        );
                    } else  {
                        spring = Spring::new(
                            self.selected_polygon.unwrap(),
                            length,
                            self.anchor_pos,
                            Vec2::zero(),
                            0.0,
                            self.polygons[self.selected_polygon.unwrap()].mass * 5.0,
                            1.0 * (11.0 * self.polygons[self.selected_polygon.unwrap()].mass).sqrt() ,
                            &self.polygons,
                        );
                    }

                    self.springs.push(spring);
                    self.temp_springs.push(self.springs.len() - 1);
                    self.temp_polygons.push(self.polygons.len() - 1);
                }
                if self.temp_polygons.len() > 0 {
                    let index = self.temp_polygons[0];
                    if index < self.polygons.len() {
                        let diff = position - self.polygons[index].center;
                        self.polygons[index].translate(diff);
                    }
                } else {
                    self.dragging = DraggingState::StopDragging;
                }
                self.dragging = DraggingState::Dragging;
            }
        }
    }

    pub(crate) fn handle_scroll(&mut self, delta: MouseScrollDelta) {
        let change = match delta {
            MouseScrollDelta::LineDelta(_, y) => y,
            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 20.0,
        };
        self.camera_pos.w += change * self.scaling_factor / 10.0;
    }

    pub fn handle_cursor_movement(&mut self, position: PhysicalPosition<f64>) {
        let dx = position.x as f32 - self.mouse_pos.0;
        let dy = position.y as f32 - self.mouse_pos.1;
        let size = self.window.inner_size();
        if self.pressed_buttons[Mouse::Middle as usize] == 1 {
            self.camera_pos.x -= dx * 2.0 * (-self.camera_pos.w + 1.0) / size.width as f32;
            self.camera_pos.y += dy * 2.0 * (-self.camera_pos.w + 1.0) / size.height as f32;
        }
        self.mouse_pos = (position.x as f32, position.y as f32).into();
    }

    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
        let position = self.get_mouse_world_position();
        if button == MouseButton::Left {
            if state.is_pressed() && !self.is_pointer_used {
                if self.input_mode == InputMode::Spawn{
                    if self.spawn_parameters.body_type == BodyType::Spring {
                        self.pressed_buttons[Mouse::Left as usize] = 1;
                        self.selected_polygon = self.get_polygon_under_mouse();
                        if self.selected_polygon.is_some() { self.dragging = DraggingState::StartDragging }
                    } else if self.spawn_parameters.body_type == BodyType::Rectangle || self.spawn_parameters.body_type == BodyType::RegularPolygon{
                        self.pressed_buttons[Mouse::Left as usize] = 1;
                        self.polygons
                            .push(BodyBuilder::create_rigidbody(&self.spawn_parameters));
                        let length = self.polygons.len() - 1;
                        self.polygons[length].translate(position);
                    }
                }
                else if self.input_mode == InputMode::Select {
                    self.pressed_buttons[Mouse::Left as usize] = 1;
                    self.selected_polygon = self.get_polygon_under_mouse();
                    if self.selected_polygon.is_some() { self.menus[Menu::Editor as usize] = true}
                    else {
                        self.selected_polygon = None;
                        self.selected_spring = self.get_spring_under_mouse();
                    }
                } else if self.input_mode == InputMode::Drag {
                    self.pressed_buttons[Mouse::Left as usize] = 1;
                    self.selected_polygon = self.get_polygon_under_mouse();
                    if self.selected_polygon.is_some() { self.dragging = DraggingState::StartDragging }
                }
            }  else if !state.is_pressed() || self.dragging == DraggingState::StopDragging {
                if self.input_mode == InputMode::Drag {
                    for index in self.temp_polygons.clone() { self.remove_rigidbody(index); }
                    for index in self.temp_springs.clone() { self.remove_spring(index); }
                    self.temp_springs.clear();
                    self.temp_polygons.clear();
                    self.dragging = DraggingState::NotDragging;
                    self.selected_polygon = None;
                    self.pressed_buttons[Mouse::Left as usize] = 0;
                } else if self.input_mode == InputMode::Spawn && self.dragging == DraggingState::Dragging {
                    self.pressed_buttons[Mouse::Left as usize] = 1;
                    let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, f32::MAX / 100000.0, 1.0, Color::random());
                    let polygon2_index = self.get_polygon_under_mouse();
                    if polygon2_index.is_some() && self.temp_springs.len() > 0 && self.selected_polygon.unwrap() != polygon2_index.unwrap() {
                        let polygon2 = &mut self.polygons[polygon2_index.unwrap()];
                        let anchor_pos = mouse_polygon.center - polygon2.center;
                        self.springs[self.temp_springs[0]].body_b = polygon2_index.unwrap();
                        self.springs[self.temp_springs[0]].anchor_b = anchor_pos;
                    } else {
                        for spring_index in self.temp_springs.clone() { self.remove_spring(spring_index); }
                    }
                    for rigidbody_index in self.temp_polygons.clone() { self.remove_rigidbody(rigidbody_index); }
                    self.temp_springs.clear();
                    self.temp_polygons.clear();
                    self.dragging = DraggingState::NotDragging;
                    self.pressed_buttons[Mouse::Left as usize] = 0;
                } else {
                    self.pressed_buttons[Mouse::Left as usize] = 0;
                }
            }
        }
        if button == MouseButton::Right {
            if state.is_pressed() && !self.is_pointer_used {
                self.pressed_buttons[Mouse::Right as usize] = 1;
                let polygon_under_mouse = self.get_polygon_under_mouse();
                if polygon_under_mouse.is_some() {self.remove_rigidbody(polygon_under_mouse.unwrap()); }
                let spring_under_mouse = self.get_spring_under_mouse();
                if spring_under_mouse.is_some() {self.remove_spring(spring_under_mouse.unwrap());}
            } else {
                self.pressed_buttons[Mouse::Right as usize] = 0;
            }
        }
        if button == MouseButton::Middle {
            if state.is_pressed() && !self.is_pointer_used {
                self.pressed_buttons[Mouse::Middle as usize] = 1;
            } else {
                self.pressed_buttons[Mouse::Middle as usize] = 0;
            }
        }
    }
}
