// Import necessary modules and traits
use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// Import submodules for different menus
use crate::network_setup_menu;
use crate::data_speed_menu;
use crate::packet_capture_menu;

// Define the AdminApp struct to hold the application state
pub struct AdminApp {
    debug_log_admin: Arc<Mutex<String>>, // Admin debug log
    debug_log_error: Arc<Mutex<String>>, // Error log
    admin_submenu: Option<usize>,        // Currently selected submenu
    show_submenu: bool,                  // Flag to control submenu visibility
    start_time: Arc<Mutex<Instant>>,     // Application start time
}

impl AdminApp {
    // Constructor to initialize a new AdminApp instance
    fn new(start_time: Arc<Mutex<Instant>>) -> Self {
        Self {
            debug_log_admin: Arc::new(Mutex::new(String::new())),
            debug_log_error: Arc::new(Mutex::new(String::new())), // Initialize error log
            admin_submenu: None,
            show_submenu: false, // Initially, no submenu is shown
            start_time,
        }
    }

    // Function to log messages with a timestamp, placing the most recent entry at the top
    fn log_message(&self, message: &str) {
        let mut log = self.debug_log_admin.lock().unwrap();
        let elapsed = {
            let start_time = self.start_time.lock().unwrap();
            start_time.elapsed()
        };
        let timestamp = format!("{:02}:{:02}:{:02}.{:03}", 
            elapsed.as_secs() / 3600,
            (elapsed.as_secs() % 3600) / 60,
            elapsed.as_secs() % 60,
            elapsed.subsec_millis()
        );
        let new_entry = format!("[{}] {}\n", timestamp, message);
        log.insert_str(0, &new_entry); // Prepend the new log entry
    }

    // Function to handle displaying the submenu
    fn handle_submenu(&mut self, ui: &mut egui::Ui) {
        if self.show_submenu {
            if let Some(submenu) = self.admin_submenu {
                match submenu {
                    0 => network_setup_menu::show_menu(ui, 12.0, &mut network_setup_menu::MenuState::default()), // Show Network Setup menu
                    1 => data_speed_menu::show_menu(ui, 12.0, |ui| { ui.label("Back button clicked"); }),         // Show Data Speed menu
                    2 => packet_capture_menu::show_menu(ui, 12.0, |ui| { ui.label("Back button clicked"); }),     // Show Packet Capture menu
                    _ => {}
                }
            }
        }
    }
}

// Implement the eframe::App trait for AdminApp
impl eframe::App for AdminApp {
    // Update function to handle UI rendering and logic
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Calculate the elapsed time since the application started
        let elapsed_time = {
            let start_time = self.start_time.lock().unwrap();
            start_time.elapsed()
        };

        // Log a message to show that the update function is called
        self.log_message("Update function called");

        // Display the debug info log in the left side panel
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Debug Info Log");
            let debug_info = self.debug_log_admin.lock().unwrap();
            ui.label(debug_info.to_string());
        });

        // Display the debug error log in the right side panel
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Debug Error Log");
            let debug_error = self.debug_log_error.lock().unwrap();
            ui.label(debug_error.to_string());
        });

        // Display the main UI elements in the central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_space(ui.available_size());

            egui::Area::new("centered_area".into()) // Create a centered area
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(egui::Label::new(
                            egui::RichText::new("S2O's s2o_net_lib crate")
                                .font(egui::FontId::proportional(18.0 * 1.5))
                                .color(egui::Color32::from_rgb(0, 255, 0))
                                .strong()
                        ));
                        ui.add_space(20.0);

                        // Display runtime in hours, minutes, seconds, and milliseconds
                        ui.add(egui::Label::new(
                            egui::RichText::new(format!("Runtime: {:02}:{:02}:{:02}.{:03}", 
                                elapsed_time.as_secs() / 3600,
                                (elapsed_time.as_secs() % 3600) / 60,
                                elapsed_time.as_secs() % 60,
                                elapsed_time.subsec_millis()))
                                .font(egui::FontId::proportional(18.0))
                                .color(egui::Color32::from_rgb(255, 255, 255))
                        ));
                        ui.add_space(20.0);

                        // Define the admin menu options
                        let admin_options = ["Network Setup", "Data Speed", "Packet Capture", "Exit"];
                        let mut local_submenu = self.admin_submenu.unwrap_or(3);

                        // Iterate through the admin options and create buttons for each
                        for (i, &option) in admin_options.iter().enumerate() {
                            let text = if i == local_submenu {
                                egui::RichText::new(option)
                                    .font(egui::FontId::proportional(18.0))
                                    .color(egui::Color32::from_rgb(0, 255, 0))
                                    .strong()
                            } else {
                                egui::RichText::new(option)
                                    .font(egui::FontId::proportional(18.0))
                                    .color(egui::Color32::from_rgb(255, 255, 255))
                            };

                            // Add button and handle button click or Enter key press
                            let button = ui.add(egui::Button::new(text));
                            if button.clicked() || (ctx.input(|i| i.key_pressed(egui::Key::Enter)) && i == local_submenu) {
                                match i {
                                    0 | 1 | 2 => {
                                        self.admin_submenu = Some(i);
                                        self.show_submenu = true; // Show submenu when Enter is pressed or button clicked
                                        self.log_message(&format!("Selected submenu: {}", option)); // Log submenu selection
                                    },
                                    3 => {
                                        self.log_message("Exiting the application"); // Log exit action
                                        std::process::exit(0); // Exit the program when "Exit" is selected
                                    },
                                    _ => {}
                                }
                            }
                        }

                        // Handle keyboard navigation for selection
                        ctx.input(|i| {
                            let len = admin_options.len();
                            if i.key_pressed(egui::Key::ArrowDown) {
                                local_submenu = (local_submenu + 1) % len;
                                self.log_message("Navigated down in the menu"); // Log navigation action
                            } else if i.key_pressed(egui::Key::ArrowUp) {
                                local_submenu = (local_submenu + len - 1) % len;
                                self.log_message("Navigated up in the menu"); // Log navigation action
                            } else if i.key_pressed(egui::Key::Enter) {
                                if local_submenu == 3 {
                                    self.log_message("Exiting the application via Enter key"); // Log exit action
                                    std::process::exit(0); // Exit the program when "Exit" is selected
                                }
                                self.show_submenu = false; // Reset show_submenu flag when changing selection
                            }
                            self.admin_submenu = Some(local_submenu); // Update selection
                        });

                        // Call handle_submenu to display the selected submenu
                        self.handle_submenu(ui);
                    });
                });
        });

        // Request a repaint every 10 milliseconds to update the UI
        ctx.request_repaint_after(Duration::from_millis(10));
    }
}

// Function to run the admin menu
pub fn run_admin_menu(start_time: Arc<Mutex<Instant>>) {
    let app = AdminApp::new(start_time.clone());
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };
    if let Err(e) = eframe::run_native("Admin Menu", native_options, Box::new(|_cc| Ok(Box::new(app)))) {
        eprintln!("Failed to start admin menu: {:?}", e);
    }
}
