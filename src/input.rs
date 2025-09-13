use glam::Vec2;
use crate::body_builder::{BodyBuilder, SpringParams};
use crate::enums::{BodyType, DraggingState, InputMode, Keys, Menu, Mouse};
use crate::spring::Spring;
use crate::{Camera, ColorRGBA, Parameters, PivotJoint, Rigidbody, WeldJoint};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::ActiveEventLoop;
use crate::color::ColorSystem;
use crate::physics::PhysicsSystem;

pub struct UiSystem{
    pub pressed_keys: [u8; 64],
    pub pressed_buttons: [u8; 3],
    pub mouse_pos: Vec2,
    pub is_pointer_used: bool,

    pub selected_polygon: Option<usize>,
    pub selected_spring: Option<usize>,
    pub spring_polygon: Option<usize>,
    pub mouse_spring: Option<usize>,
    pub spawn_ghost_polygon: Option<usize>,

    pub input_mode: InputMode,
    pub dragging: DraggingState,
    pub menus: [bool; 16],

    pub drag_params: SpringParams,
    pub spawn_parameters: BodyBuilder,

    pub camera: Camera,
    pub window_dimensions: Vec2,
}
impl UiSystem {
    pub(crate) fn handle_key(
        &mut self,
        physics: &mut PhysicsSystem,
        color_system: &mut ColorSystem,
        parameters: &mut Parameters,
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
                    for i in 0..physics.polygons.len() {
                        physics.polygons[i].angular_velocity = 20.0;
                    }
                }
            }
            winit::keyboard::KeyCode::KeyP => {
                if _pressed && self.pressed_keys[Keys::P as usize] == 0 {
                    self.pressed_keys[Keys::P as usize] = 1;
                    parameters.is_running = true;
                } else if _pressed && self.pressed_keys[Keys::P as usize] == 1 {
                    self.pressed_keys[Keys::P as usize] = 0;
                    parameters.is_running = false;
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
                for i in 0..physics.polygons.len() {
                    physics.polygons[i].angular_velocity = 0.0;
                    physics.polygons[i].velocity = Vec2::ZERO;
                }
            }
            winit::keyboard::KeyCode::Digit1 => {
                if _pressed && self.pressed_keys[Keys::Num1 as usize] == 0 {
                    self.pressed_keys[Keys::Num1 as usize] = 1;
                    self.input_mode = InputMode::Spawn;
                    self.menus[Menu::Spawner as usize] = true;
                    self.menus[Menu::Editor as usize] = false;
                } else if _pressed && self.pressed_keys[Keys::Num1 as usize] == 1 {
                    self.pressed_keys[Keys::Num1 as usize] = 0;
                    self.input_mode = InputMode::Spawn;
                    self.menus[Menu::Spawner as usize] = true;
                    self.menus[Menu::Editor as usize] = false;
                }
            }
            winit::keyboard::KeyCode::Digit2 => {
                if _pressed && self.pressed_keys[Keys::Num2 as usize] == 0 {
                    self.pressed_keys[Keys::Num2 as usize] = 1;
                    self.input_mode = InputMode::Edit;
                    self.menus[Menu::Editor as usize] = true;
                    self.menus[Menu::Spawner as usize] = false;
                } else if _pressed && self.pressed_keys[Keys::Num2 as usize] == 1 {
                    self.pressed_keys[Keys::Num2 as usize] = 0;
                    self.input_mode = InputMode::Edit;
                    self.menus[Menu::Editor as usize] = true;
                    self.menus[Menu::Spawner as usize] = false;
                }
            }
            winit::keyboard::KeyCode::Digit3 => {
                if _pressed && self.pressed_keys[Keys::Num3 as usize] == 0 {
                    self.pressed_keys[Keys::Num3 as usize] = 1;
                    self.input_mode = InputMode::Drag;
                    self.menus[Menu::Editor as usize] = false;
                    self.menus[Menu::Spawner as usize] = false;
                } else if _pressed && self.pressed_keys[Keys::Num3 as usize] == 1 {
                    self.pressed_keys[Keys::Num3 as usize] = 0;
                    self.input_mode = InputMode::Drag;
                    self.menus[Menu::Editor as usize] = false;
                    self.menus[Menu::Spawner as usize] = false;
                }
            }
            winit::keyboard::KeyCode::KeyC => {
                if _pressed && self.pressed_keys[Keys::C as usize] == 0 {
                    self.pressed_keys[Keys::C as usize] = 1;
                    color_system.regenerate_colors(&mut physics.polygons);
                } else if _pressed && self.pressed_keys[Keys::C as usize] == 1 {
                    self.pressed_keys[Keys::C as usize] = 0;
                    color_system.regenerate_colors(&mut physics.polygons);
                }
            }
            winit::keyboard::KeyCode::KeyV => {
                if _pressed && self.pressed_keys[Keys::C as usize] == 0 {
                    self.pressed_keys[Keys::V as usize] = 1;
                    color_system.view_random_palette(&mut physics.polygons);
                } else if _pressed && self.pressed_keys[Keys::C as usize] == 1 {
                    self.pressed_keys[Keys::V as usize] = 0;
                    color_system.view_random_palette(&mut physics.polygons);
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
    pub fn handle_input(&mut self, physics_system: &mut PhysicsSystem, color_system: &mut ColorSystem) {
        let position = self.get_mouse_world_position();
        if self.pressed_keys[Keys::L as usize] == 1 {
            if self.under_mouse_is_clear(physics_system) == true{
                physics_system.polygons
                    .push(BodyBuilder::create_rigidbody(&self.spawn_parameters, &color_system.color_palette));
                let length = physics_system.polygons.len() - 1;
                physics_system.polygons[length].translate(position);
            }
        }

        if self.pressed_keys[Keys::W as usize] == 1 {
            self.camera.camera_pos.y += 5.0 * physics_system.dt;
        }
        if self.pressed_keys[Keys::A as usize] == 1 {
            self.camera.camera_pos.x -= 5.0 * physics_system.dt;
        }
        if self.pressed_keys[Keys::S as usize] == 1 {
            self.camera.camera_pos.y -= 5.0 * physics_system.dt;
        }
        if self.pressed_keys[Keys::D as usize] == 1 {
            self.camera.camera_pos.x += 5.0 * physics_system.dt;
        }
        if self.pressed_keys[Keys::Plus as usize] == 1 {
            self.camera.camera_pos.w += 5.0 * self.camera.scaling_factor * physics_system.dt;
        }
        if self.pressed_keys[Keys::Minus as usize] == 1 {
            self.camera.camera_pos.w -= 5.0 * self.camera.scaling_factor * physics_system.dt;
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
                    Rigidbody::rectangle(0.03, 0.03, position, f32::MAX / 10000.0, 1.0, ColorRGBA::white());
                mouse_polygon.collision = false;
                let selected_polygon;
                if self.selected_polygon.unwrap() < physics_system.polygons.len(){
                    selected_polygon = &mut physics_system.polygons[self.selected_polygon.unwrap()];
                } else {
                    self.selected_polygon = None;
                    return
                }

                if self.dragging == DraggingState::StartDragging {
                    let anchor_pos = mouse_polygon.center - selected_polygon.center;
                    physics_system.polygons.push(mouse_polygon);
                    let length = physics_system.polygons.len() - 1;
                    let spring;
                    if self.input_mode == InputMode::Spawn {
                        spring = Spring::new(
                            self.selected_polygon.unwrap(),
                            length,
                            anchor_pos,
                            Vec2::ZERO,
                            self.spawn_parameters.spring_params.rest_length,
                            self.spawn_parameters.spring_params.stiffness,
                            self.spawn_parameters.spring_params.dampening,
                            &physics_system.polygons,
                        );
                    } else  {
                        spring = Spring::new(
                            self.selected_polygon.unwrap(),
                            length,
                            anchor_pos,
                            Vec2::ZERO,
                            0.0,
                            physics_system.polygons[self.selected_polygon.unwrap()].mass * 5.0,
                            1.0 * (11.0 * physics_system.polygons[self.selected_polygon.unwrap()].mass).sqrt() ,
                            &physics_system.polygons,
                        );
                    }
                    physics_system.springs.push(spring);
                    self.mouse_spring = Some(physics_system.springs.len() - 1);
                    self.spring_polygon = Some(physics_system.polygons.len() - 1);
                }
                if self.spring_polygon.is_some() {
                    let index = self.spring_polygon.unwrap();
                    if index < physics_system.polygons.len() {
                        let diff = position - physics_system.polygons[index].center;
                        physics_system.polygons[index].translate(diff);
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
        if !self.is_pointer_used{
            self.camera.camera_pos.w += change * self.camera.scaling_factor / 10.0;
        }

    }

    pub fn handle_cursor_movement(&mut self, position: PhysicalPosition<f64>) {
        let dx = position.x as f32 - self.mouse_pos.x;
        let dy = position.y as f32 - self.mouse_pos.y;
        if self.pressed_buttons[Mouse::Middle as usize] == 1 {
            self.camera.camera_pos.x -= dx * 2.0 * (-self.camera.camera_pos.w + 1.0) / self.window_dimensions.x;
            self.camera.camera_pos.y += dy * 2.0 * (-self.camera.camera_pos.w + 1.0) / self.window_dimensions.y;
        }
        self.mouse_pos = (position.x as f32, position.y as f32).into();
    }

    pub fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton, physics_system: &mut PhysicsSystem, color_system: &ColorSystem) {
        let position = self.get_mouse_world_position();
        if button == MouseButton::Left {
            if state.is_pressed() && !self.is_pointer_used {
                if self.input_mode == InputMode::Spawn{
                    if self.spawn_parameters.body_type == BodyType::Spring {
                        self.pressed_buttons[Mouse::Left as usize] = 1;
                        self.selected_polygon = self.get_polygon_under_mouse(physics_system);
                        if self.selected_polygon.is_some() { self.dragging = DraggingState::StartDragging }
                    } else if self.spawn_parameters.body_type == BodyType::Rectangle || self.spawn_parameters.body_type == BodyType::RegularPolygon{
                        if self.under_mouse_is_clear(physics_system) == true {
                            self.pressed_buttons[Mouse::Left as usize] = 1;
                            physics_system.polygons
                                .push(BodyBuilder::create_rigidbody(&self.spawn_parameters, &color_system.color_palette));
                            let length = physics_system.polygons.len() - 1;
                            physics_system.polygons[length].translate(position);
                        }
                    } else if self.spawn_parameters.body_type == BodyType::WeldJoint || self.spawn_parameters.body_type == BodyType::PivotJoint{
                        let under_mouse = self.get_all_polygons_under_mouse(physics_system);
                        if under_mouse.len() >= 2 {
                            for i in 0..under_mouse.len() {
                                for j in i+1..under_mouse.len() {
                                    let anchor_a = physics_system.polygons[under_mouse[i]].center - position;
                                    let anchor_b = physics_system.polygons[under_mouse[j]].center - position;
                                    if self.spawn_parameters.body_type == BodyType::WeldJoint {
                                        physics_system.weld_joints.push(WeldJoint::new(anchor_a, anchor_b, &mut physics_system.polygons, under_mouse[i], under_mouse[j]));
                                    } else if self.spawn_parameters.body_type == BodyType::PivotJoint{
                                        physics_system.pivot_joints.push(PivotJoint::new(anchor_a, anchor_b, &mut physics_system.polygons, under_mouse[i], under_mouse[j]));
                                    }
                                }
                            }
                        }
                    }
                }
                else if self.input_mode == InputMode::Edit {
                    self.pressed_buttons[Mouse::Left as usize] = 1;
                    self.selected_polygon = self.get_polygon_under_mouse(physics_system);
                    if self.selected_polygon.is_some() { self.menus[Menu::Editor as usize] = true}
                    else {
                        self.selected_polygon = None;
                        self.selected_spring = self.get_spring_under_mouse(physics_system);
                    }
                } else if self.input_mode == InputMode::Drag {
                    self.pressed_buttons[Mouse::Left as usize] = 1;
                    self.selected_polygon = self.get_polygon_under_mouse(physics_system);
                    if self.selected_polygon.is_some() { self.dragging = DraggingState::StartDragging }
                }
            }  else if !state.is_pressed() || self.dragging == DraggingState::StopDragging {
                if self.input_mode == InputMode::Drag {
                    if self.spring_polygon.is_some() {physics_system.remove_rigidbody(self.spring_polygon.unwrap(), self); }
                    if self.mouse_spring.is_some() {physics_system.remove_spring(self.mouse_spring.unwrap(), self); }
                    self.mouse_spring = None;
                    self.spring_polygon = None;
                    self.dragging = DraggingState::NotDragging;
                    self.selected_polygon = None;
                    self.pressed_buttons[Mouse::Left as usize] = 0;
                } else if self.input_mode == InputMode::Spawn && self.dragging == DraggingState::Dragging {
                    self.pressed_buttons[Mouse::Left as usize] = 1;
                    let mouse_polygon = Rigidbody::rectangle(0.03, 0.03, position, f32::MAX / 100000.0, 1.0, ColorRGBA::white());
                    let polygon2_index = self.get_polygon_under_mouse(physics_system);
                    if polygon2_index.is_some() && self.mouse_spring.is_some() && self.selected_polygon.unwrap() != polygon2_index.unwrap() {
                        let polygon2 = &mut physics_system.polygons[polygon2_index.unwrap()];
                        let anchor_pos = mouse_polygon.center - polygon2.center;
                        physics_system.springs[self.mouse_spring.unwrap()].body_b = polygon2_index.unwrap();
                        physics_system.springs[self.mouse_spring.unwrap()].anchor_b = anchor_pos;
                    } else if self.mouse_spring.is_some() {
                        physics_system.remove_spring(self.mouse_spring.unwrap(), self);
                    }
                    if self.spring_polygon.is_some() {physics_system.remove_rigidbody(self.spring_polygon.unwrap(), self); }
                    self.mouse_spring = None;
                    self.spring_polygon = None;
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
                let mut erased = false;
                for i in (0..physics_system.weld_joints.len()).rev() {
                    let mut polygon = BodyBuilder::create_joint();
                    let position = physics_system.weld_joints[i].get_anchor_world_position(&physics_system.polygons);
                    polygon.move_to(position);
                    if self.is_colliding_mouse(&polygon){
                        physics_system.remove_weld_joint(i);
                        erased = true;
                        break;
                    }
                }

                for i in (0..physics_system.pivot_joints.len()).rev() {
                    let mut polygon = BodyBuilder::create_joint();
                    let position = physics_system.pivot_joints[i].get_anchor_world_position(&physics_system.polygons);
                    polygon.move_to(position);
                    if self.is_colliding_mouse(&polygon) && !erased{
                        physics_system.remove_pivot_joint(i);
                        erased = true;
                        break;
                    }
                }

                let spring_under_mouse = self.get_spring_under_mouse(physics_system);
                if spring_under_mouse.is_some() && !erased{
                    physics_system.remove_spring(spring_under_mouse.unwrap(), self);
                    erased = true;
                }

                let polygon_under_mouse = self.get_polygon_under_mouse(physics_system);
                if polygon_under_mouse.is_some() && !erased{
                    physics_system.remove_rigidbody(polygon_under_mouse.unwrap(), self);
                }
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
