use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::menu_settings::Settings; // Use the common Settings module

pub struct AdminApp {
    debug_log_admin: Arc<Mutex<String>>,
    admin_submenu: Option<usize>,
    show_submenu: bool, // New field to control when to show the submenu
    start_time: Arc<Mutex<Instant>>,
    settings: Arc<Settings>,
}

impl AdminApp {
    fn new(start_time: Arc<Mutex<Instant>>, settings: Arc<Settings>) -> Self {
        Self {
            debug_log_admin: Arc::new(Mutex::new(String::new())),
            admin_submenu: None,
            show_submenu: false, // Initially, no submenu is shown
            start_time,
            settings,
        }
    }
}

impl eframe::App for AdminApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let elapsed_time = {
            let start_time = self.start_time.lock().unwrap();
            start_time.elapsed()
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_space(ui.available_size());
            
            egui::Area::new("centered_area".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new("S2O's s2o_net_lib crate")
                                .font(egui::FontId::proportional(self.settings.font_size * 1.5))
                                .color(egui::Color32::from_rgb(0, 255, 0))
                                .strong()
                        ));
                        ui.add_space(20.0);

                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("Runtime: {:02}:{:02}:{:02}", 
                                elapsed_time.as_secs() / 3600,
                                (elapsed_time.as_secs() % 3600) / 60,
                                elapsed_time.as_secs() % 60))
                                .font(egui::FontId::proportional(self.settings.font_size))
                                .color(egui::Color32::from_rgb(255, 255, 255))
                        ));
                        ui.add_space(20.0);
                        
                        let admin_options = ["Exit"];
                        let mut local_submenu = self.admin_submenu.unwrap_or(0);

                        for (i, &option) in admin_options.iter().enumerate() {
                            let text = if i == local_submenu {
                                egui::RichText::new(option)
                                    .font(egui::FontId::proportional(self.settings.font_size))
                                    .color(egui::Color32::from_rgb(0, 255, 0))
                                    .strong()
                            } else {
                                egui::RichText::new(option)
                                    .font(egui::FontId::proportional(self.settings.font_size))
                                    .color(egui::Color32::from_rgb(255, 255, 255))
                            };

                            let button = ui.add(egui::Button::new(text));
                            if button.clicked() || (ctx.input(|i| i.key_pressed(egui::Key::Enter)) && i == local_submenu) {
                                match i {
                                    0 => frame.quit(),
                                    _ => {}
                                }
                            }
                        }

                        // Handle keyboard navigation for selection
                        ctx.input(|i| {
                            let len = admin_options.len();
                            if i.key_pressed(egui::Key::ArrowDown) {
                                local_submenu = (local_submenu + 1) % len;
                            } else if i.key_pressed(egui::Key::ArrowUp) {
                                local_submenu = (local_submenu + len - 1) % len;
                            } else if i.key_pressed(egui::Key::Enter) {
                                // Reset show_submenu flag when changing selection, to hide old submenu
                                self.show_submenu = false;
                            }
                            self.admin_submenu = Some(local_submenu); // Update selection
                        });
                    });
                });
        });

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }
}

pub fn run_admin_menu(start_time: Arc<Mutex<Instant>>, settings: Arc<Settings>) {
    let app = AdminApp::new(start_time.clone(), settings.clone());
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(settings.window_size.0, settings.window_size.1)),
        ..Default::default()
    };
    if let Err(e) = eframe::run_native("Admin Menu", native_options, Box::new(|_cc| Ok(Box::new(app)))) {
        eprintln!("Failed to start admin menu: {:?}", e);
    }
}
