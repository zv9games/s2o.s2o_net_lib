use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::process::Command;

mod admin_app;
mod menu_settings;

use menu_settings::Settings;

struct MyApp {
    debug_log_main: Arc<Mutex<String>>,
    debug_log_admin: Arc<Mutex<String>>,
    selected_option: Option<usize>,
    start_time: Arc<Mutex<Instant>>,
    settings: Arc<Settings>,
}

impl MyApp {
    fn new(start_time: Arc<Mutex<Instant>>, settings: Arc<Settings>) -> Self {
        Self {
            debug_log_main: Arc::new(Mutex::new(String::new())),
            debug_log_admin: Arc::new(Mutex::new(String::new())),
            selected_option: Some(0),
            start_time,
            settings,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let options = ["Admin Menu", "Exit"];
        
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
                        ui.add_space(40.0);
                        
                        for (i, &option) in options.iter().enumerate() {
                            let text = if self.selected_option == Some(i) {
                                egui::RichText::new(option)
                                    .font(egui::FontId::proportional(self.settings.font_size))
                                    .color(egui::Color32::from_rgb(0, 255, 0))
                                    .strong()
                            } else {
                                egui::RichText::new(option)
                                    .font(egui::FontId::proportional(self.settings.font_size))
                                    .color(egui::Color32::from_rgb(255, 255, 255))
                            };
                            
                            if ui.add(egui::Button::new(text)).clicked() {
                                self.selected_option = Some(i);
                                match i {
                                    0 => {
                                        self.run_admin_menu();
                                        frame.close();
                                    },
                                    1 => frame.close(),
                                    _ => {}
                                }
                            }
                        }
                    });
                });
        });

        // Handle keyboard navigation and selection
        ctx.input(|i| {
            if let Some(selected) = self.selected_option {
                if i.key_pressed(egui::Key::ArrowDown) && selected < options.len() - 1 {
                    self.selected_option = Some(selected + 1);
                }
                if i.key_pressed(egui::Key::ArrowUp) && selected > 0 {
                    self.selected_option = Some(selected - 1);
                }
                if i.key_pressed(egui::Key::Enter) {
                    match selected {
                        0 => {
                            self.run_admin_menu();
                            frame.close();
                        },
                        1 => frame.close(),
                        _ => {}
                    }
                }
            }
        });
    }
}

impl MyApp {
    fn run_admin_menu(&self) {
        // Call PowerShell script to request elevated permissions
        let output = Command::new("powershell")
            .arg("-File")
            .arg("uac.ps1")
            .arg(format!("{}", std::env::current_exe().unwrap().display()))
            .arg("--admin")
            .output()
            .expect("Failed to execute script");

        // Log the output for debugging
        println!("{:?}", output);
    }
}

fn main() {
    let start_time = Arc::new(Mutex::new(Instant::now()));
    let settings = Arc::new(Settings { 
        font_size: 24.0,
        window_size: (800.0, 600.0),
        horizontal_pos: 0.5,
        vertical_pos: 0.5,
    });

    if std::env::args().any(|arg| arg == "--admin") {
        admin_app::run_admin_menu(start_time.clone(), settings.clone());
    } else {
        let app = MyApp::new(start_time.clone(), settings.clone());
        let native_options = eframe::NativeOptions {
            initial_window_size: Some(egui::vec2(settings.window_size.0, settings.window_size.1)),
            ..Default::default()
        };
        eframe::run_native("s2o_net_lib", native_options, Box::new(|_cc| Ok(Box::new(app))));
    }
}
