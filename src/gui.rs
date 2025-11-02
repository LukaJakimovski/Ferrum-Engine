use crate::enums::{BodyType, ColorType, InputMode, Menu};
use crate::{Camera, ColorRGBA, Parameters};
use egui::{Align2};
use egui_wgpu::{ScreenDescriptor, wgpu};
use std::f32::consts::PI;
use crate::body_builder::BodyBuilder;
use crate::color::ColorSystem;
use crate::input::UiSystem;
use crate::physics::PhysicsSystem;
use crate::render::RenderSystem;
use crate::timing::Timing;

impl RenderSystem {
    pub fn create_gui(&mut self, ui_system: &mut UiSystem, physics_system: &mut PhysicsSystem, color_system: &mut ColorSystem, timing: &mut Timing, parameters: &mut Parameters, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window.as_ref().scale_factor() as f32 * 1.0,
        };
        self.egui_renderer.begin_frame(self.window.as_ref());
        egui::Window::new("Menu Selector")
            .resizable(false)
            .vscroll(false)
            .anchor(Align2::LEFT_TOP, [0.0, 0.0])
            .default_open(true)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Menu Selector");
                ui.checkbox(&mut ui_system.menus[Menu::Config as usize], "World Config");
                ui.checkbox(&mut ui_system.menus[Menu::Input as usize], "Change Input Mode");
                ui.checkbox(&mut ui_system.menus[Menu::Camera as usize], "Camera Position");
                ui.checkbox(&mut ui_system.menus[Menu::Spawner as usize], "Spawned Body Properties", );
                ui.checkbox(&mut ui_system.menus[Menu::Editor as usize], "Edit Selected Polygon", );
                ui.checkbox(&mut ui_system.menus[Menu::Energy as usize], "Kinetic Energy Info", );
                ui.checkbox(&mut ui_system.menus[Menu::FPS as usize], "Show FPS");
                ui.checkbox(&mut ui_system.menus[Menu::Color as usize], "Color Menu");
                ui.checkbox(&mut ui_system.menus[Menu::Advanced as usize], "Advanced Settings");
            });

        if ui_system.menus[Menu::Config as usize] {
            self.config_menu(parameters)
        }
        if ui_system.menus[Menu::Energy as usize] {
            self.energy_menu(physics_system)
        }
        if ui_system.menus[Menu::FPS as usize] {
            self.fps_menu(timing)
        }
        if ui_system.menus[Menu::Camera as usize] {
            self.camera_menu(&mut ui_system.camera)
        }
        if ui_system.menus[Menu::Spawner as usize] {
            self.spawner_menu(&mut ui_system.spawn_parameters, &mut color_system.color_palette)
        }
        if ui_system.menus[Menu::Input as usize] {
            self.input_menu(ui_system)
        }
        if ui_system.menus[Menu::Editor as usize] {
            self.editor_menu(physics_system, ui_system)
        }
        if ui_system.menus[Menu::Advanced as usize] {
            self.advanced_menu(parameters)
        }
        if ui_system.menus[Menu::Color as usize] {
            self.color_menu(color_system)
        }

        ui_system.is_pointer_used = self.egui_renderer.context().is_pointer_over_area();
        self.egui_renderer.end_frame_and_draw(
            &self.device,
            &self.queue,
            encoder,
            self.window.as_ref(),
            &view,
            screen_descriptor,
        );
    }

    fn camera_menu(&mut self, camera: &mut Camera) {
        egui::Window::new("Camera")
            .resizable(true)
            .vscroll(true)
            .default_open(true)
            .title_bar(false)
            .max_height(100.0)
            .resizable(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Camera");
                ui.columns(3, |ui| {
                    ui[0].label("X");
                    ui[1].label("Y");
                    ui[2].label("Z");
                });
                ui.end_row();
                ui.columns(3, |ui| {
                    ui[0].add(egui::DragValue::new(&mut camera.camera_pos.x).speed(0.1));
                    ui[1].add(egui::DragValue::new(&mut camera.camera_pos.y).speed(0.1));
                    ui[2].add(egui::DragValue::new(&mut camera.camera_pos.w).speed(0.1));
                    if camera.camera_pos.w > 0.9999 {
                        camera.camera_pos.w = 0.9999
                    }
                });
                ui.columns(2, |ui| {
                    ui[0].label("Scroll Speed");
                    ui[1].add(egui::DragValue::new(&mut camera.scaling_factor).speed(0.1));
                    if camera.scaling_factor < 0.0 {
                        camera.scaling_factor = 0.0
                    }
                })
            });
    }
    fn energy_menu(&mut self, physics_system: &mut PhysicsSystem) {
        egui::Window::new("Energy")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(100.0)
            .max_width(300.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Energy");
                ui.label(format!("Spring Energy: {:.3} Joules", physics_system.energy.spring_energy));
                ui.label(format!("Kinetic Energy: {:.3} Joules", physics_system.energy.kinetic_energy));
                ui.label(format!("Potential Energy: {:.3} Joules", physics_system.energy.potential_energy));
                ui.label(format!("Total Energy: {:.3} Joules", physics_system.energy.get_energy()));


            });
    }
    fn fps_menu(&mut self, timing: &mut Timing) {
        egui::Window::new("FPS")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(50.0)
            .max_width(100.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("FPS");
                ui.label(format!("FPS: {:.3}", timing.fps));
                ui.label(format!("ms/frame {:.3}", 1000.0 / timing.fps));
            });
    }
    fn config_menu(&mut self, parameters: &mut Parameters) {
        egui::Window::new("Config")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(200.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Config");
                ui.checkbox(&mut parameters.is_running, "Running");
                ui.checkbox(&mut parameters.gravity, "Gravity");
                ui.columns(2, |ui| {
                    ui[0].label("World Radius");
                    ui[1].add(egui::DragValue::new(&mut parameters.world_size).speed(0.1));
                    if parameters.world_size < 0.0 {
                        parameters.world_size = 0.0;
                    }
                });
                if parameters.delta_time == 0.0 {
                    ui.columns(2, |ui| {
                        ui[0].label("Time multiplier");
                        ui[1].add(egui::DragValue::new(&mut parameters.time_multiplier).speed(0.01));
                        if parameters.time_multiplier < 0.0 {
                            parameters.time_multiplier = 0.0;
                        }
                    });
                }
                ui.columns(3, |ui| {
                    ui[0].label("Gravity Force");
                    ui[1]
                        .add(egui::DragValue::new(&mut parameters.gravity_force.x).speed(0.1));
                    ui[1]
                        .add(egui::DragValue::new(&mut parameters.gravity_force.y).speed(0.1));
                });
            });
    }

    fn advanced_menu(&mut self, parameters: &mut Parameters) {
        egui::Window::new("Spawner")
            .resizable(false)
            .vscroll(false)
            .default_open(true)
            .default_height(275.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Advanced");
                ui.label("If you don't know what these mean don't change them");
                ui.columns(2, |ui| {
                    ui[0].label("Time Step");
                    ui[1].add(egui::DragValue::new(&mut parameters.delta_time).speed(0.00001));
                    if parameters.delta_time < 0.0 {
                        parameters.delta_time = 0.0;
                    }
                    if parameters.delta_time > 0.005 {
                        parameters.delta_time = 0.005;
                    }
                });
                ui.columns(2, |ui| {
                    ui[0].label("Physics Updates Per Frame");
                    ui[1]
                        .add(egui::DragValue::new(&mut parameters.updates_per_frame).speed(1));
                });
            });
    }

    fn spawner_menu(&mut self, spawn_parameters: &mut BodyBuilder, color_palette: &mut Option<Vec<ColorRGBA>>) {
        egui::Window::new("Spawner")
            .resizable(false)
            .vscroll(false)
            .default_open(true)
            .default_height(275.0)
            .anchor(Align2::LEFT_CENTER, [0.0, 0.0])
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Spawner");
                egui::ComboBox::from_label("Body Type")
                    .selected_text(format!("{:?}", spawn_parameters.body_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut spawn_parameters.body_type,
                            BodyType::RegularPolygon,
                            "Regular Polygon",
                        );
                        ui.selectable_value(
                            &mut spawn_parameters.body_type,
                            BodyType::Rectangle,
                            "Rectangle",
                        );
                        ui.selectable_value(
                            &mut spawn_parameters.body_type,
                            BodyType::Spring,
                            "Spring",
                        );
                        ui.selectable_value(
                            &mut spawn_parameters.body_type,
                            BodyType::WeldJoint,
                            "Weld Joint",
                        );
                        ui.selectable_value(
                            &mut spawn_parameters.body_type,
                            BodyType::PivotJoint,
                            "Pivot Joint"
                        );
                    });
                match spawn_parameters.body_type {
                    BodyType::RegularPolygon => {
                        ui.label("Click anywhere on the screen not overlapping another rigidbody to spawn a rigidbody");
                        egui::ComboBox::from_label("Color")
                            .selected_text(format!(
                                "{:?}",
                                spawn_parameters.rigidbody_params.color_type
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Random,
                                    "Random Color",
                                );
                                ui.selectable_value(
                                    &mut spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Set,
                                    "Set Color",
                                );
                            });
                        ui.columns(2, |ui| {
                            ui[0].label("Side Count");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut spawn_parameters.rigidbody_params.sides,
                                )
                                .speed(1),
                            );
                            if spawn_parameters.rigidbody_params.sides < 3 {
                                spawn_parameters.rigidbody_params.sides = 3
                            }
                            if spawn_parameters.rigidbody_params.sides > 128 {
                                spawn_parameters.rigidbody_params.sides = 128
                            }
                        });
                        ui.columns(2, |ui| {
                            ui[0].label("Radius");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut spawn_parameters.rigidbody_params.radius,
                                )
                                .speed(0.01),
                            );
                            if spawn_parameters.rigidbody_params.radius < 0.0 {
                                spawn_parameters.rigidbody_params.radius = 0.0
                            };
                        });
                    }
                    BodyType::Rectangle => {
                        ui.label("Click anywhere on the screen not overlapping another rigidbody to spawn another");
                        egui::ComboBox::from_label("Color")
                            .selected_text(format!(
                                "{:?}",
                                spawn_parameters.rigidbody_params.color_type
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Random,
                                    "Random Color",
                                );
                                ui.selectable_value(
                                    &mut spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Set,
                                    "Set Color",
                                );
                            });
                        ui.columns(2, |ui| {
                            ui[0].label("Width");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut spawn_parameters.rigidbody_params.width,
                                )
                                .speed(0.01),
                            );
                            if spawn_parameters.rigidbody_params.width < 0.0 {
                                spawn_parameters.rigidbody_params.width = 0.0
                            };
                        });
                        ui.columns(2, |ui| {
                            ui[0].label("Height");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut spawn_parameters.rigidbody_params.height,
                                )
                                .speed(0.01),
                            );
                            if spawn_parameters.rigidbody_params.height < 0.0 {
                                spawn_parameters.rigidbody_params.height = 0.0
                            };
                        });
                    }
                    _ => {}
                }

                if spawn_parameters.body_type == BodyType::Rectangle || spawn_parameters.body_type == BodyType::RegularPolygon {
                    ui.columns(2, |ui| {
                        ui[0].label("Restitution/Bounciness");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.rigidbody_params.restitution,
                            )
                            .speed(0.01),
                        )
                    });
                    if spawn_parameters.rigidbody_params.restitution > 1.5 {
                        spawn_parameters.rigidbody_params.restitution = 1.5;
                    }
                    if spawn_parameters.rigidbody_params.restitution < 0.0 {
                        spawn_parameters.rigidbody_params.restitution = 0.0;
                    }
                    ui.columns(3, |ui| {
                        ui[0].label("Velocity");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.rigidbody_params.velocity.x,
                            )
                            .speed(0.01),
                        );
                        ui[2].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.rigidbody_params.velocity.y,
                            )
                            .speed(0.01),
                        );
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Angular Velocity");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.rigidbody_params.angular_velocity,
                            )
                            .speed(0.01),
                        )
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Mass");
                        ui[1].add(
                            egui::DragValue::new(&mut spawn_parameters.rigidbody_params.mass)
                                .speed(0.01),
                        );
                        if spawn_parameters.rigidbody_params.mass <= 0.0 {
                            spawn_parameters.rigidbody_params.mass = 0.0000000001
                        };
                    });
                    ui.columns(2, |ui| {
                        let mut rotation_degrees = spawn_parameters.rigidbody_params.rotation.to_degrees();
                        ui[0].label("Rotation");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut rotation_degrees,
                            )
                            .speed(0.01),
                        );
                        spawn_parameters.rigidbody_params.rotation = rotation_degrees.to_radians();
                        if spawn_parameters.rigidbody_params.rotation > PI {
                            spawn_parameters.rigidbody_params.rotation = PI
                        };
                        if spawn_parameters.rigidbody_params.rotation < -PI {
                            spawn_parameters.rigidbody_params.rotation = -PI
                        };
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Gravity Multiplier");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.rigidbody_params.gravity_multiplier,
                            )
                            .speed(0.01),
                        )
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Collides");
                        ui[1].add(egui::Checkbox::new(
                            &mut spawn_parameters.rigidbody_params.collides,
                            "Collides",
                        ));
                    });
                    match spawn_parameters.rigidbody_params.color_type {
                        ColorType::Random => {
                            spawn_parameters.rigidbody_params.color = None;
                        }
                        ColorType::Set => {
                            if spawn_parameters.rigidbody_params.color.is_none() {
                                spawn_parameters.rigidbody_params.color =
                                    Some(ColorRGBA::random_from_palette(&color_palette.clone().unwrap()));
                            }
                            let param_color =
                                &mut spawn_parameters.rigidbody_params.color.unwrap();
                            let mut color: [f32; 3] = [param_color.r, param_color.g, param_color.b];
                            ui.columns(2, |ui| {
                                ui[0].label("Color");
                                ui[1].color_edit_button_rgb(&mut color);
                            });

                            spawn_parameters.rigidbody_params.color =
                                Some(ColorRGBA::new(color[0], color[1], color[2], 1.0));
                        }
                    }
                } else if spawn_parameters.body_type == BodyType::Spring {
                    ui.label("To spawn a spring:");
                    ui.label("Click on a rigidbody and drag.");
                    ui.label("Drop the spring on a different rigidbody to spawn a spring.");
                    ui.label("Will drag rigidbody if simulation is running");
                    ui.columns(2, |ui| {
                        ui[0].label("Pull Strength/Stiffness");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.spring_params.stiffness,
                            )
                                .speed(0.01),
                        );
                        if spawn_parameters.spring_params.stiffness < 0.0 {
                            spawn_parameters.spring_params.stiffness = 0.0
                        };
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Dampening");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.spring_params.dampening,
                            )
                                .speed(0.01),
                        );
                        if spawn_parameters.spring_params.dampening < 0.0 {
                            spawn_parameters.spring_params.dampening = 0.0
                        };
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Rest length");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut spawn_parameters.spring_params.rest_length,
                            )
                                .speed(0.01),
                        );
                        if spawn_parameters.spring_params.rest_length < 0.0 {
                            spawn_parameters.spring_params.rest_length = 0.0
                        };
                    });
                } else if spawn_parameters.body_type == BodyType::WeldJoint || spawn_parameters.body_type == BodyType::PivotJoint {
                    ui.label("To spawn a joint:");
                    ui.label("Stop the simulation using the config menu.");
                    ui.label("Go to select mode.");
                    ui.label("Select a rigidbody.");
                    ui.label("Change it's position to be overlapping another rigidbody.");
                    ui.label("Change to spawn mode.");
                    ui.label("Click an overlapping section to spawn a rigidbody");
                }
            });
    }

    fn input_menu(&mut self, ui_system: &mut UiSystem) {
        egui::Window::new("Left Click Mode")
            .resizable(false)
            .vscroll(false)
            .default_open(true)
            .default_height(275.0)
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                let current_mode = ui_system.input_mode;
                ui.heading("Left Click Mode Selector");
                egui::ComboBox::from_label("Mode")
                    .selected_text(format!("{:?}", ui_system.input_mode))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut ui_system.input_mode, InputMode::Spawn, "Spawn Bodies", );
                        ui.selectable_value(&mut ui_system.input_mode, InputMode::Edit, "Edit Bodies", );
                        ui.selectable_value(&mut ui_system.input_mode, InputMode::Drag, "Drag Body w/ Spring");
                        //ui.selectable_value(&mut ui_system.input_mode, InputMode::Move, "Move Body w/ Mouse");
                        ui.selectable_value(&mut ui_system.input_mode, InputMode::Nothing, "Nothing", );
                    });
                if ui_system.input_mode != current_mode {
                    if ui_system.input_mode == InputMode::Spawn {
                        ui_system.menus[Menu::Spawner as usize] = true;
                        ui_system.menus[Menu::Editor as usize] = false;
                    } else if ui_system.input_mode == InputMode::Edit {
                        ui_system.menus[Menu::Spawner as usize] = false;
                        ui_system.menus[Menu::Editor as usize] = true;
                    } else if ui_system.input_mode == InputMode::Spawn {
                        ui_system.menus[Menu::Spawner as usize] = false;
                        ui_system.menus[Menu::Editor as usize] = false;
                    }
                }
            });
        if ui_system.input_mode == InputMode::Drag {
            ui_system.menus[Menu::DragParams as usize] = true;
        }
    }

    fn editor_menu(&mut self, physics_system: &mut PhysicsSystem, ui_system: &mut UiSystem) {
        if ui_system.selected_polygon.is_some() {
            let selected_polygon = &mut physics_system.polygons[ui_system.selected_polygon.unwrap()];
            egui::Window::new("Body Editor")
                .resizable(false)
                .vscroll(false)
                .default_open(true)
                .default_height(275.0)
                .anchor(Align2::LEFT_CENTER, [0.0, 0.0])
                .title_bar(false)
                .show(self.egui_renderer.context(), |ui| {
                    ui.heading("Rigidbody Editor");
                    ui.columns(3, |ui| {
                        let mut new_center = selected_polygon.center;
                        ui[0].label("Position");
                        ui[1].add(
                            egui::DragValue::new(&mut new_center.x).speed(0.01),
                        );
                        ui[2].add(
                            egui::DragValue::new(&mut new_center.y).speed(0.01),
                        );
                        if new_center != selected_polygon.center {
                            selected_polygon.move_to(new_center);
                        }
                    });
                    ui.columns(3, |ui| {
                        ui[0].label("Velocity");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_polygon.velocity.x).speed(0.01),
                        );
                        ui[2].add(
                            egui::DragValue::new(&mut selected_polygon.velocity.y).speed(0.01),
                        );
                    });
                    ui.columns(2, |ui| {
                        let mut angle_degrees = selected_polygon.angle.to_degrees();
                        let old_angle = selected_polygon.angle;
                        ui[0].label("Angle");
                        ui[1].add(
                            egui::DragValue::new(&mut angle_degrees)
                                .speed(0.1),
                        );
                        let angle_radians = angle_degrees.to_radians();
                        selected_polygon.rotate(angle_radians - old_angle);
                        selected_polygon.angle += angle_radians - old_angle;
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Angular Velocity");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_polygon.angular_velocity)
                                .speed(0.01),
                        )
                    });
                    ui.columns(2, |ui| {
                        let old_mass = selected_polygon.mass;
                        ui[0].label("Mass");
                        ui[1].add(egui::DragValue::new(&mut selected_polygon.mass).speed(0.01));
                        if old_mass != selected_polygon.mass {
                            selected_polygon.calculate_moment_of_inertia();
                        }
                        if selected_polygon.mass < 0.0 {
                            selected_polygon.mass = 0.000000000001
                        };
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Restitution/Bounciness");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_polygon.restitution).speed(0.01),
                        )
                    });
                    if selected_polygon.restitution < 0.0 {
                        selected_polygon.restitution = 0.0
                    }
                    if selected_polygon.restitution > 1.5 {
                        selected_polygon.restitution = 1.5
                    }
                    ui.columns(2, |ui| {
                        ui[0].label("Gravity Multiplier");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_polygon.gravity_multiplier)
                                .speed(0.01),
                        )
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Collides");
                        ui[1].add(egui::Checkbox::new(
                            &mut selected_polygon.collision,
                            "Collides",
                        ));
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Eternal");
                        ui[1].add(egui::Checkbox::new(&mut selected_polygon.eternal, "Eternal"));
                    });
                    let param_color = &mut selected_polygon.color;
                    let mut color: [f32; 3] = [param_color.r, param_color.g, param_color.b];
                    ui.columns(2, |ui| {
                        ui[0].label("Color");
                        ui[1].color_edit_button_rgb(&mut color);
                    });
                    selected_polygon.change_color(ColorRGBA::new(color[0], color[1], color[2], 1.0));
                });
        } else if ui_system.selected_spring.is_some() {
            let selected_spring = &mut physics_system.springs[ui_system.selected_spring.unwrap()];
            egui::Window::new("Spring Editor")
                .resizable(false)
                .vscroll(false)
                .default_open(true)
                .anchor(Align2::LEFT_CENTER, [0.0, 0.0])
                .default_height(275.0)
                .title_bar(false)
                .show(self.egui_renderer.context(), |ui| {
                    ui.heading("Spring Editor");
                    ui.columns(3, |ui| {
                        ui[0].label("Anchor A");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_spring.anchor_a.x).speed(0.01),
                        );
                        ui[2].add(
                            egui::DragValue::new(&mut selected_spring.anchor_a.y).speed(0.01),
                        );
                    });
                    ui.columns(3, |ui| {
                        ui[0].label("Anchor B");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_spring.anchor_b.x).speed(0.01),
                        );
                        ui[2].add(
                            egui::DragValue::new(&mut selected_spring.anchor_b.y).speed(0.01),
                        );
                    });
                    ui.columns(3, |ui| {
                        ui[0].label("Stiffness");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_spring.stiffness).speed(0.01),
                        );
                    });
                    ui.columns(3, |ui| {
                        ui[0].label("Damping");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_spring.damping).speed(0.01),
                        );
                    });
                    ui.columns(3, |ui| {
                        ui[0].label("Rest Length");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_spring.rest_length).speed(0.01),
                        );
                    });
                });
        } else {
            egui::Window::new("Physics Body Editor")
                .resizable(false)
                .vscroll(false)
                .default_open(true)
                .anchor(Align2::LEFT_CENTER, [0.0, 0.0])
                .default_height(275.0)
                .title_bar(false)
                .show(self.egui_renderer.context(), |ui| {
                    ui.heading("Physics Body Editor");
                    ui.label("No Body Selected");
                    ui.label("Try left clicking one");
                });
        }
    }
    
    fn color_menu(&mut self, color_system: &mut ColorSystem) {
        egui::Window::new("Color Menu")
            .resizable(false)
            .vscroll(false)
            .default_open(true)
            .default_height(275.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Color Menu");
                ui.columns(3, |ui| {
                    let old_start = color_system.palette_params.start_range.x.start;
                    let old_end = color_system.palette_params.start_range.x.end;
                    ui[0].label("Hue Start Range");
                    ui[1].add(
                        egui::DragValue::new(&mut color_system.palette_params.start_range.x.start).speed(0.1),
                    );
                    ui[2].add(
                        egui::DragValue::new(&mut color_system.palette_params.start_range.x.end).speed(0.1),
                    );
                    if  color_system.palette_params.start_range.x.start >= color_system.palette_params.start_range.x.end {
                        if old_start != color_system.palette_params.start_range.x.start { color_system.palette_params.start_range.x.start = color_system.palette_params.start_range.x.end.next_down();}
                        if old_end != color_system.palette_params.start_range.x.end { color_system.palette_params.start_range.x.end = color_system.palette_params.start_range.x.start.next_up();}
                    }
                    if color_system.palette_params.start_range.x.start < 0.0 { color_system.palette_params.start_range.x.start = 0.0; }
                    if color_system.palette_params.start_range.x.end < 0.0 { color_system.palette_params.start_range.x.end = 0.0f32.next_up(); }
                });
                ui.columns(3, |ui| {
                    let old_start = color_system.palette_params.start_range.y.start;
                    let old_end = color_system.palette_params.start_range.y.end;
                    ui[0].label("Saturation Start Range");
                    ui[1].add(
                        egui::DragValue::new(&mut color_system.palette_params.start_range.y.start).speed(0.1),
                    );
                    ui[2].add(
                        egui::DragValue::new(&mut color_system.palette_params.start_range.y.end).speed(0.1),
                    );
                    if color_system.palette_params.start_range.y.start >= color_system.palette_params.start_range.y.end {
                        if old_start != color_system.palette_params.start_range.y.start { color_system.palette_params.start_range.y.start = color_system.palette_params.start_range.y.end.next_down();}
                        if old_end != color_system.palette_params.start_range.y.end { color_system.palette_params.start_range.y.end = color_system.palette_params.start_range.y.start.next_up();}
                    }
                    if color_system.palette_params.start_range.y.start < 0.0 { color_system.palette_params.start_range.y.start = 0.0; }
                    if color_system.palette_params.start_range.y.end < 0.0 { color_system.palette_params.start_range.y.end = 0.0f32.next_up(); }
                });
                ui.columns(3, |ui| {
                    let old_start = color_system.palette_params.start_range.z.start;
                    let old_end = color_system.palette_params.start_range.z.end;
                    ui[0].label("Light Start Range");
                    ui[1].add(
                        egui::DragValue::new(&mut color_system.palette_params.start_range.z.start).speed(0.1),
                    );
                    ui[2].add(
                        egui::DragValue::new(&mut color_system.palette_params.start_range.z.end).speed(0.1),
                    );
                    if color_system.palette_params.start_range.z.start >= color_system.palette_params.start_range.z.end {
                        if old_start != color_system.palette_params.start_range.z.start { color_system.palette_params.start_range.z.start = color_system.palette_params.start_range.z.end.next_down();}
                        if old_end != color_system.palette_params.start_range.z.end { color_system.palette_params.start_range.z.end = color_system.palette_params.start_range.z.start.next_up();}
                    }
                    if color_system.palette_params.start_range.z.start < 0.0 { color_system.palette_params.start_range.z.start = 0.0; }
                    if color_system.palette_params.start_range.z.end < 0.0 { color_system.palette_params.start_range.z.end = 0.0f32.next_up(); }
                });
                ui.columns(3, |ui| {
                    let old_start = color_system.palette_params.end_range.x.start;
                    let old_end = color_system.palette_params.end_range.x.end;
                    ui[0].label("Hue End Range");
                    ui[1].add(
                        egui::DragValue::new(&mut color_system.palette_params.end_range.x.start).speed(0.1),
                    );
                    ui[2].add(
                        egui::DragValue::new(&mut color_system.palette_params.end_range.x.end).speed(0.1),
                    );
                    if color_system.palette_params.end_range.x.start >= color_system.palette_params.end_range.x.end {
                        if old_start != color_system.palette_params.end_range.x.start { color_system.palette_params.end_range.x.start = color_system.palette_params.end_range.x.end.next_down();}
                        if old_end != color_system.palette_params.end_range.x.end { color_system.palette_params.end_range.x.end = color_system.palette_params.end_range.x.start.next_up();}
                    }
                    if color_system.palette_params.end_range.x.start < 0.0 { color_system.palette_params.end_range.x.start = 0.0; }
                    if color_system.palette_params.end_range.x.end < 0.0 { color_system.palette_params.end_range.x.end = 0.0f32.next_up(); }
                });
                ui.columns(3, |ui| {
                    let old_start = color_system.palette_params.end_range.y.start;
                    let old_end = color_system.palette_params.end_range.y.end;
                    ui[0].label("Saturation End Range");
                    ui[1].add(
                        egui::DragValue::new(&mut color_system.palette_params.end_range.y.start).speed(0.1),
                    );
                    ui[2].add(
                        egui::DragValue::new(&mut color_system.palette_params.end_range.y.end).speed(0.1),
                    );
                    if color_system.palette_params.end_range.y.start >= color_system.palette_params.end_range.y.end {
                        if old_start != color_system.palette_params.end_range.y.start { color_system.palette_params.end_range.y.start = color_system.palette_params.end_range.y.end.next_down();}
                        if old_end != color_system.palette_params.end_range.y.end { color_system.palette_params.end_range.y.end = color_system.palette_params.end_range.y.start.next_up();}
                    }
                    if color_system.palette_params.end_range.y.start < 0.0 { color_system.palette_params.end_range.y.start = 0.0; }
                    if color_system.palette_params.end_range.y.end < 0.0 { color_system.palette_params.end_range.y.end = 0.0f32.next_up(); }
                });
                ui.columns(3, |ui| {
                    let old_start = color_system.palette_params.end_range.z.start;
                    let old_end = color_system.palette_params.end_range.z.end;
                    ui[0].label("Light End Range");
                    ui[1].add(
                        egui::DragValue::new(&mut color_system.palette_params.end_range.z.start).speed(0.1),
                    );
                    ui[2].add(
                        egui::DragValue::new(&mut color_system.palette_params.end_range.z.end).speed(0.1),
                    );
                    if color_system.palette_params.end_range.z.start >= color_system.palette_params.end_range.z.end {
                        if old_start != color_system.palette_params.end_range.z.start { color_system.palette_params.end_range.z.start = color_system.palette_params.end_range.z.end.next_down();}
                        if old_end != color_system.palette_params.end_range.z.end { color_system.palette_params.end_range.z.end = color_system.palette_params.end_range.z.start.next_up();}
                    }
                    if color_system.palette_params.end_range.z.start < 0.0 { color_system.palette_params.end_range.z.start = 0.0; }
                    if color_system.palette_params.end_range.z.end < 0.0 { color_system.palette_params.end_range.z.end = 0.0f32.next_up(); }
                });
                ui.columns(2, |ui|{
                    ui[0].label("Color Count");
                    ui[1].add(
                        egui::DragValue::new(&mut color_system.palette_params.color_count).speed(1),
                    );
                    if color_system.palette_params.color_count < 1 { color_system.palette_params.color_count = 1; }
                });
                ui.columns(1, |ui|{
                    ui[0].add(
                        egui::Checkbox::new(&mut color_system.update_clear_color, "Update Background Color"),
                    );
                });
                let param_color =
                    &mut color_system.clear_color;
                let mut color: [f32; 3] = [param_color.r, param_color.g, param_color.b];
                ui.columns(2, |ui| {
                    ui[0].label("Background Color");
                    ui[1].color_edit_button_rgb(&mut color);
                });
                color_system.clear_color = ColorRGBA {r: color[0], b: color[1], g : color[2], a: 1.0 }
            });
    }
}
