use egui::Align2;
use egui_wgpu::{wgpu, ScreenDescriptor};
use crate::enums::{ColorType, Menu, RigidBodyType};
use crate::{Color, World};

impl World{
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
            .show(self.egui_renderer.context(), | ui | {
                ui.heading("Menu Selector");
                ui.checkbox(&mut self.menus[Menu::Config as usize], "World Config");
                ui.checkbox(&mut self.menus[Menu::Energy as usize], "Kinetic Energy Info");
                ui.checkbox(&mut self.menus[Menu::FPS as usize], "Show FPS");
                ui.checkbox(&mut self.menus[Menu::Camera as usize], "Camera Position");
                ui.checkbox(&mut self.menus[Menu::Spawner as usize], "Spawned Body Properties");
            });

        if self.menus[Menu::Config as usize] { self.config_menu() }
        if self.menus[Menu::Energy as usize] { self.energy_menu() }
        if self.menus[Menu::FPS as usize] { self.fps_menu() }
        if self.menus[Menu::Camera as usize] { self.camera_menu() }
        if self.menus[Menu::Spawner as usize] { self.spawner_menu() }


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

    fn camera_menu(&mut self){
        egui::Window::new("Camera")
            .resizable(true)
            .vscroll(true)
            .default_open(true)
            .title_bar(false)
            .max_height(100.0)
            .resizable(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Camera");
                ui.columns(3, | ui |{
                    ui[0].label("X");
                    ui[1].label("Y");
                    ui[2].label("Z");
                });
                ui.end_row();
                ui.columns(3, | ui |{
                    ui[0].add(egui::DragValue::new(&mut self.camera_pos.x, ).speed(0.1));
                    ui[1].add(egui::DragValue::new(&mut self.camera_pos.y).speed(0.1));
                    ui[2].add(egui::DragValue::new(&mut self.camera_pos.w).speed(0.1));
                });
                ui.columns(2, | ui |{
                    ui[0].label("Scroll Speed");
                    ui[1].add(egui::DragValue::new(&mut self.scaling_factor).speed(0.1));
                })
            });
    }
    fn energy_menu(&mut self){
        egui::Window::new("Energy")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(25.0)
            .max_width(200.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Energy");
                ui.label(format!(
                    "Energy: {:.3} Joules",
                    self.total_energy
                ));
            });
    }
    fn fps_menu(&mut self){
        egui::Window::new("FPS")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(50.0)
            .max_width(100.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("FPS");
                ui.label(format!(
                    "FPS: {:.3}",
                    self.fps
                ));
                ui.label(format!(
                    "ms/frame {:.3}",
                    1000.0 / self.fps
                ));
            });
    }
    fn config_menu(&mut self){
        egui::Window::new("Config")
            .resizable(false)
            .vscroll(true)
            .default_open(true)
            .max_height(200.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Config");
                ui.checkbox(&mut self.is_running,
                            "Running"
                );
                ui.checkbox(&mut self.parameters.gravity,
                            "Gravity"
                );
                ui.columns(2, | ui |{
                    ui[0].label("World Radius");
                    ui[1].add(egui::DragValue::new(&mut self.parameters.world_size).speed(0.1));
                    if self.parameters.world_size < 0.0 {
                        self.parameters.world_size = 0.0;
                    }
                });
                ui.columns(2, | ui |{
                    ui[0].label("Time Step");
                    ui[1].add(egui::DragValue::new(&mut self.parameters.delta_time).speed(0.0001));
                    if self.parameters.delta_time < 0.0 {
                        self.parameters.delta_time = 0.0;
                    }
                });
                ui.columns(2, | ui |{
                    ui[0].label("World Radius");
                    ui[1].add(egui::DragValue::new(&mut self.parameters.world_size).speed(0.1));
                    if self.parameters.world_size < 0.0 {
                        self.parameters.world_size = 0.0;
                    }
                });
                ui.columns(2, | ui |{
                    ui[0].label("Physics Updates Per Frame");
                    ui[1].add(egui::DragValue::new(&mut self.parameters.updates_per_frame).speed(1));
                });
            });
    }

    fn spawner_menu(&mut self){
        egui::Window::new("Spawner")
            .resizable(false)
            .vscroll(false)
            .default_open(true)
            .default_height(275.0)
            .title_bar(false)
            .show(self.egui_renderer.context(), |ui| {
                ui.heading("Spawner");
                egui::ComboBox::from_label("Color")
                    .selected_text(format!("{:?}", self.spawn_parameters.color_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.spawn_parameters.color_type, ColorType::Random, "Random Color");
                        ui.selectable_value(&mut self.spawn_parameters.color_type, ColorType::Set, "Set Color");
                        }
                    );

                egui::ComboBox::from_label("Body Type")
                    .selected_text(format!("{:?}", self.spawn_parameters.body_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.spawn_parameters.body_type, RigidBodyType::RegularPolygon, "Regular Polygon");
                        ui.selectable_value(&mut self.spawn_parameters.body_type, RigidBodyType::Rectangle, "Rectangle");
                    }
                    );

                match self.spawn_parameters.body_type {
                    RigidBodyType::RegularPolygon => {
                        ui.columns(2, | ui |{
                            ui[0].label("Side Count");
                            ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.sides).speed(1))
                            //.show_tooltip_text("Changes the amount of sides of the spawned polygon");;
                        });
                        ui.columns(2, | ui |{
                            ui[0].label("Radius");
                            ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.radius).speed(0.01))
                            //.show_tooltip_text("Changes the radius of the spawned polygon. Value in meters");;
                        });
                    }
                    RigidBodyType::Rectangle => {
                        ui.columns(2, | ui |{
                            ui[0].label("Width");
                            ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.width).speed(0.01))
                            //.show_tooltip_text("Changes the amount of sides of the spawned polygon");;
                        });
                        ui.columns(2, | ui |{
                            ui[0].label("Height");
                            ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.height).speed(0.01))
                            //.show_tooltip_text("Changes the radius of the spawned polygon. Value in meters");;
                        });
                    }
                }


                ui.columns(2, | ui |{
                    ui[0].label("Restitution/Bounciness");
                    ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.restitution).speed(0.01))
                        //.show_tooltip_text("Changes the amount of energy conserved in a collision\n0.0 -> No bounce, 1.0 -> Perfectly elastic >1.0 -> Gains energy <0.0 -> Accelerates into collision");;
                });
                ui.columns(3, | ui |{
                    ui[0].label("Velocity");
                    ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.velocity.x).speed(0.01));
                        //.show_tooltip_text("Changes the horizontal velocity of the spawned polygon. Value in m/s");;
                    ui[2].add(egui::DragValue::new(&mut self.spawn_parameters.velocity.y).speed(0.01));
                        //.show_tooltip_text("Changes the vertical velocity of the spawned polygon. Value in m/s");;
                });
                ui.columns(2, | ui |{
                    ui[0].label("Angular Velocity");
                    ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.angular_velocity).speed(0.01))
                        //.show_tooltip_text("Changes the orientation of the spawned polygon. Value in radians/sec");;
                });
                ui.columns(2, | ui |{
                    ui[0].label("Mass");
                    ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.mass).speed(0.01))
                        //.show_tooltip_text("Changes the mass of the spawned polygon. Value in kg");;
                });
                ui.columns(2, | ui |{
                    ui[0].label("Rotation");
                    ui[1].add(egui::DragValue::new(&mut self.spawn_parameters.rotation)
                        .speed(0.01))
                        //.show_tooltip_text("Changes the orientation of the spawned polygon. Value in radians");
                });
                ui.columns(2, | ui |{
                    ui[0].label("Collides");
                    ui[1].add(egui::Checkbox::new(&mut self.spawn_parameters.collides, "Collides"));
                });
                match self.spawn_parameters.color_type {
                    ColorType::Random => {
                        self.spawn_parameters.color = None;
                    },
                    ColorType::Set => {
                        if self.spawn_parameters.color.is_none() {
                            self.spawn_parameters.color = Some(Color::random());
                        }
                        let param_color = &mut self.spawn_parameters.color.unwrap();
                        let mut color: [f32; 3] = [param_color.r, param_color.g, param_color.b];
                        ui.columns(2, | ui |{
                            ui[0].label("Color");
                            ui[1].color_edit_button_rgb(&mut color);
                        });

                        self.spawn_parameters.color = Some(Color::new(color[0], color[1], color[2]));
                    }
                }
            });
    }
}