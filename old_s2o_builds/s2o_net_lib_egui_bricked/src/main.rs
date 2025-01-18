use eframe::egui::{self};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

// Declare the app and menu_settings modules
mod app;
mod menu_settings;

use menu_settings::{MenuSettings, default_settings};

struct MyApp {
    selected_option: Option<usize>,
    menu_settings: MenuSettings,
}

impl MyApp {
    fn new() -> Self {
        Self {
            selected_option: Some(0),
            menu_settings: default_settings(),
        }
    }

    fn log_message(&self, message: &str) {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug.log")
            .expect("Failed to open debug log file");
        let new_entry = format!("{}\n", message);
        file.write_all(new_entry.as_bytes())
            .expect("Failed to write to debug log file");
    }

    fn run_admin_menu(&self) {
        self.log_message("run_admin_menu called");

        // Call PowerShell script to request elevated permissions
        let output = Command::new("powershell")
            .arg("-File")
            .arg("uac.ps1")
            .arg(format!("{}", std::env::current_exe().unwrap().display()))
            .arg("--admin")
            .output()
            .expect("Failed to execute script");

        self.log_message(&format!("PowerShell output: {:?}", output));
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let options = ["Admin Menu", "Exit"];

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_space(ui.available_size());

            egui::Area::new("centered_area".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(self.menu_settings.apply_title("S2O's s2o_net_lib crate"));
                        ui.add_space(40.0);

                        for (i, &option) in options.iter().enumerate() {
                            let label = self.menu_settings.apply_label(option, self.selected_option == Some(i));

                            if ui.add(label).clicked() {
                                self.selected_option = Some(i);
                                match i {
                                    0 => {
                                        // Call the run_admin_menu function
                                        self.log_message("Admin Menu selected in main menu");
                                        self.run_admin_menu();
                                    },
                                    1 => {
                                        println!("Exiting the application");
                                        self.log_message("Exiting the application");
                                        std::process::exit(0); // Exit the program when "Exit" is selected
                                    },
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
                            // Call the run_admin_menu function
                            self.log_message("Admin Menu selected via Enter key");
                            self.run_admin_menu();
                        },
                        1 => {
                            println!("Exiting the application via Enter key");
                            self.log_message("Exiting the application via Enter key");
                            std::process::exit(0); // Exit the program when "Exit" is selected
                        },
                        _ => {}
                    }
                }
            }
        });
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--admin") {
        app::run_admin_menu(Arc::new(Mutex::new(Instant::now())));
    } else {
        let app = MyApp::new();
        let native_options = eframe::NativeOptions {
            ..Default::default()
        };
        let _ = eframe::run_native("s2o_net_lib", native_options, Box::new(|_cc| Ok(Box::new(app))));
    }
}
