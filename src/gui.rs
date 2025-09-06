use crate::enums::{BodyType, ColorType, InputMode, Menu};
use crate::{Color, World};
use egui::Align2;
use egui_wgpu::{ScreenDescriptor, wgpu};
use std::f32::consts::PI;

impl World {
    pub fn create_gui(&mut self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
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
                ui.checkbox(&mut self.menus[Menu::Config as usize], "World Config");
                ui.checkbox(&mut self.menus[Menu::Energy as usize], "Kinetic Energy Info", );
                ui.checkbox(&mut self.menus[Menu::FPS as usize], "Show FPS");
                ui.checkbox(&mut self.menus[Menu::Camera as usize], "Camera Position");
                ui.checkbox(&mut self.menus[Menu::Spawner as usize], "Spawned Body Properties", );
                ui.checkbox(&mut self.menus[Menu::Input as usize], "Change Input Mode");
                ui.checkbox(&mut self.menus[Menu::Editor as usize], "Edit Selected Polygon", );
                ui.checkbox(&mut self.menus[Menu::Advanced as usize], "Advanced Settings");
            });

        if self.menus[Menu::Config as usize] {
            self.config_menu()
        }
        if self.menus[Menu::Energy as usize] {
            self.energy_menu()
        }
        if self.menus[Menu::FPS as usize] {
            self.fps_menu()
        }
        if self.menus[Menu::Camera as usize] {
            self.camera_menu()
        }
        if self.menus[Menu::Spawner as usize] {
            self.spawner_menu()
        }
        if self.menus[Menu::Input as usize] {
            self.input_menu()
        }
        if self.menus[Menu::Editor as usize] {
            self.editor_menu()
        }
        if self.menus[Menu::Advanced as usize] {
            self.advanced_menu()
        }

        self.is_pointer_used = self.egui_renderer.context().is_pointer_over_area();
        self.egui_renderer.end_frame_and_draw(
            &self.device,
            &self.queue,
            encoder,
            self.window.as_ref(),
            &view,
            screen_descriptor,
        );
    }

    fn camera_menu(&mut self) {
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
                    ui[0].add(egui::DragValue::new(&mut self.camera_pos.x).speed(0.1));
                    ui[1].add(egui::DragValue::new(&mut self.camera_pos.y).speed(0.1));
                    ui[2].add(egui::DragValue::new(&mut self.camera_pos.w).speed(0.1));
                    if self.camera_pos.w > 0.0 {
                        self.camera_pos.w = 0.0
                    }
                });
                ui.columns(2, |ui| {
                    ui[0].label("Scroll Speed");
                    ui[1].add(egui::DragValue::new(&mut self.scaling_factor).speed(0.1));
                    if self.scaling_factor < 0.0 {
                        self.scaling_factor = 0.0
                    }
                })
            });
    }
    fn energy_menu(&mut self) {
        egui::Window::new("Energy")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(25.0)
            .max_width(200.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Energy");
                ui.label(format!("Energy: {:.3} Joules", self.total_energy));
            });
    }
    fn fps_menu(&mut self) {
        egui::Window::new("FPS")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(50.0)
            .max_width(100.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("FPS");
                ui.label(format!("FPS: {:.3}", self.fps));
                ui.label(format!("ms/frame {:.3}", 1000.0 / self.fps));
            });
    }
    fn config_menu(&mut self) {
        egui::Window::new("Config")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(200.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Config");
                ui.checkbox(&mut self.is_running, "Running");
                ui.checkbox(&mut self.parameters.gravity, "Gravity");
                ui.columns(2, |ui| {
                    ui[0].label("World Radius");
                    ui[1].add(egui::DragValue::new(&mut self.parameters.world_size).speed(0.1));
                    if self.parameters.world_size < 0.0 {
                        self.parameters.world_size = 0.0;
                    }
                });
                if self.parameters.delta_time == 0.0 {
                    ui.columns(2, |ui| {
                        ui[0].label("Time multiplier");
                        ui[1].add(egui::DragValue::new(&mut self.parameters.time_multiplier).speed(0.01));
                        if self.parameters.time_multiplier < 0.0 {
                            self.parameters.time_multiplier = 0.0;
                        }
                    });
                }
                ui.columns(3, |ui| {
                    ui[0].label("Gravity Force");
                    ui[1]
                        .add(egui::DragValue::new(&mut self.parameters.gravity_force.x).speed(0.1));
                    ui[1]
                        .add(egui::DragValue::new(&mut self.parameters.gravity_force.y).speed(0.1));
                });
            });
    }

    fn advanced_menu(&mut self) {
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
                    ui[1].add(egui::DragValue::new(&mut self.parameters.delta_time).speed(0.00001));
                    if self.parameters.delta_time < 0.0 {
                        self.parameters.delta_time = 0.0;
                    }
                    if self.parameters.delta_time > 0.005 {
                        self.parameters.delta_time = 0.005;
                    }
                });
                ui.columns(2, |ui| {
                    ui[0].label("Physics Updates Per Frame");
                    ui[1]
                        .add(egui::DragValue::new(&mut self.parameters.updates_per_frame).speed(1));
                });
            });
    }

    fn spawner_menu(&mut self) {
        egui::Window::new("Spawner")
            .resizable(false)
            .vscroll(false)
            .default_open(true)
            .default_height(275.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Spawner");
                egui::ComboBox::from_label("Body Type")
                    .selected_text(format!("{:?}", self.spawn_parameters.body_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.spawn_parameters.body_type,
                            BodyType::RegularPolygon,
                            "Regular Polygon",
                        );
                        ui.selectable_value(
                            &mut self.spawn_parameters.body_type,
                            BodyType::Rectangle,
                            "Rectangle",
                        );
                        ui.selectable_value(
                            &mut self.spawn_parameters.body_type,
                            BodyType::Spring,
                            "Spring",
                        );
                    });
                match self.spawn_parameters.body_type {
                    BodyType::RegularPolygon => {
                        egui::ComboBox::from_label("Color")
                            .selected_text(format!(
                                "{:?}",
                                self.spawn_parameters.rigidbody_params.color_type
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Random,
                                    "Random Color",
                                );
                                ui.selectable_value(
                                    &mut self.spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Set,
                                    "Set Color",
                                );
                            });
                        ui.columns(2, |ui| {
                            ui[0].label("Side Count");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut self.spawn_parameters.rigidbody_params.sides,
                                )
                                .speed(1),
                            )
                            //.show_tooltip_text("Changes the amount of sides of the spawned polygon");;
                        });
                        ui.columns(2, |ui| {
                            ui[0].label("Radius");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut self.spawn_parameters.rigidbody_params.radius,
                                )
                                .speed(0.01),
                            );
                            if self.spawn_parameters.rigidbody_params.radius < 0.0 {
                                self.spawn_parameters.rigidbody_params.radius = 0.0
                            };
                            //.show_tooltip_text("Changes the radius of the spawned polygon. Value in meters");;
                        });
                    }
                    BodyType::Rectangle => {
                        egui::ComboBox::from_label("Color")
                            .selected_text(format!(
                                "{:?}",
                                self.spawn_parameters.rigidbody_params.color_type
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Random,
                                    "Random Color",
                                );
                                ui.selectable_value(
                                    &mut self.spawn_parameters.rigidbody_params.color_type,
                                    ColorType::Set,
                                    "Set Color",
                                );
                            });
                        ui.columns(2, |ui| {
                            ui[0].label("Width");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut self.spawn_parameters.rigidbody_params.width,
                                )
                                .speed(0.01),
                            );
                            if self.spawn_parameters.rigidbody_params.width < 0.0 {
                                self.spawn_parameters.rigidbody_params.width = 0.0
                            };
                            //.show_tooltip_text("Changes the amount of sides of the spawned polygon");;
                        });
                        ui.columns(2, |ui| {
                            ui[0].label("Height");
                            ui[1].add(
                                egui::DragValue::new(
                                    &mut self.spawn_parameters.rigidbody_params.height,
                                )
                                .speed(0.01),
                            );
                            if self.spawn_parameters.rigidbody_params.height < 0.0 {
                                self.spawn_parameters.rigidbody_params.height = 0.0
                            };
                            //.show_tooltip_text("Changes the radius of the spawned polygon. Value in meters");;
                        });
                    }
                    _ => {}
                }

                if self.spawn_parameters.body_type != BodyType::Spring {
                    ui.columns(2, |ui| {
                        ui[0].label("Restitution/Bounciness");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.rigidbody_params.restitution,
                            )
                            .speed(0.01),
                        )
                        //.show_tooltip_text("Changes the amount of energy conserved in a collision\n0.0 -> No bounce, 1.0 -> Perfectly elastic >1.0 -> Gains energy <0.0 -> Accelerates into collision");;
                    });
                    ui.columns(3, |ui| {
                        ui[0].label("Velocity");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.rigidbody_params.velocity.x,
                            )
                            .speed(0.01),
                        );
                        //.show_tooltip_text("Changes the horizontal velocity of the spawned polygon. Value in m/s");;
                        ui[2].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.rigidbody_params.velocity.y,
                            )
                            .speed(0.01),
                        );
                        //.show_tooltip_text("Changes the vertical velocity of the spawned polygon. Value in m/s");;
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Angular Velocity");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.rigidbody_params.angular_velocity,
                            )
                            .speed(0.01),
                        )
                        //.show_tooltip_text("Changes the orientation of the spawned polygon. Value in radians/sec");;
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Mass");
                        ui[1].add(
                            egui::DragValue::new(&mut self.spawn_parameters.rigidbody_params.mass)
                                .speed(0.01),
                        );
                        if self.spawn_parameters.rigidbody_params.mass < 0.0 {
                            self.spawn_parameters.rigidbody_params.mass = 0.0
                        };
                        //.show_tooltip_text("Changes the mass of the spawned polygon. Value in kg");;
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Rotation");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.rigidbody_params.rotation,
                            )
                            .speed(0.01),
                        );
                        if self.spawn_parameters.rigidbody_params.rotation > PI {
                            self.spawn_parameters.rigidbody_params.rotation = PI
                        };
                        if self.spawn_parameters.rigidbody_params.rotation < -PI {
                            self.spawn_parameters.rigidbody_params.rotation = -PI
                        };
                        //.show_tooltip_text("Changes the orientation of the spawned polygon. Value in radians");
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Gravity Multiplier");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.rigidbody_params.gravity_multiplier,
                            )
                            .speed(0.01),
                        )
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Collides");
                        ui[1].add(egui::Checkbox::new(
                            &mut self.spawn_parameters.rigidbody_params.collides,
                            "Collides",
                        ));
                    });
                    match self.spawn_parameters.rigidbody_params.color_type {
                        ColorType::Random => {
                            self.spawn_parameters.rigidbody_params.color = None;
                        }
                        ColorType::Set => {
                            if self.spawn_parameters.rigidbody_params.color.is_none() {
                                self.spawn_parameters.rigidbody_params.color =
                                    Some(Color::random());
                            }
                            let param_color =
                                &mut self.spawn_parameters.rigidbody_params.color.unwrap();
                            let mut color: [f32; 3] = [param_color.r, param_color.g, param_color.b];
                            ui.columns(2, |ui| {
                                ui[0].label("Color");
                                ui[1].color_edit_button_rgb(&mut color);
                            });

                            self.spawn_parameters.rigidbody_params.color =
                                Some(Color::new(color[0], color[1], color[2]));
                        }
                    }
                } else {
                    ui.columns(2, |ui| {
                        ui[0].label("Pull Strength/Stiffness");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.spring_params.stiffness,
                            )
                                .speed(0.01),
                        );
                        if self.spawn_parameters.spring_params.stiffness < 0.0 {
                            self.spawn_parameters.spring_params.stiffness = 0.0
                        };
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Dampening");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.spring_params.dampening,
                            )
                                .speed(0.01),
                        );
                        if self.spawn_parameters.spring_params.dampening < 0.0 {
                            self.spawn_parameters.spring_params.dampening = 0.0
                        };
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Rest length");
                        ui[1].add(
                            egui::DragValue::new(
                                &mut self.spawn_parameters.spring_params.rest_length,
                            )
                                .speed(0.01),
                        );
                        if self.spawn_parameters.spring_params.rest_length < 0.0 {
                            self.spawn_parameters.spring_params.rest_length = 0.0
                        };
                    });
                }
            });
    }

    fn input_menu(&mut self) {
        egui::Window::new("Input Mode")
            .resizable(false)
            .vscroll(false)
            .default_open(true)
            .default_height(275.0)
            .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Input Mode Selector");
                egui::ComboBox::from_label("Mode")
                    .selected_text(format!("{:?}", self.input_mode))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.input_mode,
                            InputMode::Spawn,
                            "Spawn/Despawn Bodies",
                        );
                        ui.selectable_value(
                            &mut self.input_mode,
                            InputMode::Select,
                            "Select/Deselect Bodies",
                        );
                        ui.selectable_value(&mut self.input_mode, InputMode::Drag, "Drag Bodies");
                    });
            });
        if self.input_mode == InputMode::Drag {
            self.menus[Menu::DragParams as usize] = true;
        }
    }

    fn editor_menu(&mut self) {
        if self.selected_polygon.is_some() {
            let selected_polygon = &mut self.polygons[self.selected_polygon.unwrap()];
            egui::Window::new("Body Editor")
                .resizable(false)
                .vscroll(false)
                .default_open(true)
                .default_height(275.0)
                .title_bar(false)
                .show(self.egui_renderer.context(), |ui| {
                    ui.heading("Rigidbody Editor");
                    ui.columns(3, |ui| {
                        let mut new_center = selected_polygon.center;
                        ui[0].label("Position");
                        ui[1].add(
                            egui::DragValue::new(&mut new_center.x).speed(0.01),
                        );
                        //.show_tooltip_text("Changes the horizontal velocity of the spawned polygon. Value in m/s");;
                        ui[2].add(
                            egui::DragValue::new(&mut new_center.y).speed(0.01),
                        );
                        if new_center != selected_polygon.center {
                            selected_polygon.move_to(new_center);
                        }
                        //.show_tooltip_text("Changes the vertical velocity of the spawned polygon. Value in m/s");;
                    });
                    ui.columns(3, |ui| {
                        ui[0].label("Velocity");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_polygon.velocity.x).speed(0.01),
                        );
                        //.show_tooltip_text("Changes the horizontal velocity of the spawned polygon. Value in m/s");;
                        ui[2].add(
                            egui::DragValue::new(&mut selected_polygon.velocity.y).speed(0.01),
                        );
                        //.show_tooltip_text("Changes the vertical velocity of the spawned polygon. Value in m/s");;
                    });
                    ui.columns(2, |ui| {
                        let mut angle_degrees = selected_polygon.angle * 360.0 / (2.0 * std::f32::consts::PI);
                        let old_angle = selected_polygon.angle;
                        ui[0].label("Angle");
                        ui[1].add(
                            egui::DragValue::new(&mut angle_degrees)
                                .speed(0.1),
                        );
                        let angle_radians = angle_degrees * 2.0 * std::f32::consts::PI / 360.0;
                        selected_polygon.rotate(angle_radians - old_angle);
                        selected_polygon.angle += angle_radians - old_angle;
                        //.show_tooltip_text("Changes the orientation of the spawned polygon. Value in radians/sec");;
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Angular Velocity");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_polygon.angular_velocity)
                                .speed(0.01),
                        )
                        //.show_tooltip_text("Changes the orientation of the spawned polygon. Value in radians/sec");;
                    });
                    ui.columns(2, |ui| {
                        let old_mass = selected_polygon.mass;
                        ui[0].label("Mass");
                        ui[1].add(egui::DragValue::new(&mut selected_polygon.mass).speed(0.01));
                        if old_mass != selected_polygon.mass {
                            selected_polygon.calculate_moment_of_inertia();
                        }
                        if self.spawn_parameters.rigidbody_params.mass < 0.0 {
                            selected_polygon.mass = 0.0
                        };
                    });
                    ui.columns(2, |ui| {
                        ui[0].label("Restitution/Bounciness");
                        ui[1].add(
                            egui::DragValue::new(&mut selected_polygon.restitution).speed(0.01),
                        )
                        //.show_tooltip_text("Changes the amount of energy conserved in a collision\n0.0 -> No bounce, 1.0 -> Perfectly elastic >1.0 -> Gains energy <0.0 -> Accelerates into collision");;
                    });
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
                    let param_color = &mut selected_polygon.vertices[0].color;
                    let mut color: [f32; 3] = [param_color.r, param_color.g, param_color.b];
                    ui.columns(2, |ui| {
                        ui[0].label("Color");
                        ui[1].color_edit_button_rgb(&mut color);
                    });
                    selected_polygon.change_color(Color::new(color[0], color[1], color[2]));
                });
        } else if self.selected_spring.is_some() {
            let selected_spring = &mut self.springs[self.selected_spring.unwrap()];
            egui::Window::new("Spring Editor")
                .resizable(false)
                .vscroll(false)
                .default_open(true)
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
            egui::Window::new("Rigidbody Editor")
                .resizable(false)
                .vscroll(false)
                .default_open(true)
                .default_height(275.0)
                .title_bar(false)
                .show(self.egui_renderer.context(), |ui| {
                    ui.heading("Rigidbody Editor");
                    ui.label("No Rigidbody Selected");
                });
        }
    }
}
