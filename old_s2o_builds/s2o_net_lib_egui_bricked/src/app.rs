use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::fs::OpenOptions;
use std::io::Write;
use crate::menu_settings::{MenuSettings, default_settings};

pub struct AdminApp {
    debug_log_admin: Arc<Mutex<String>>, // Admin debug log
    debug_log_error: Arc<Mutex<String>>, // Error log
    admin_submenu: Option<usize>,        // Currently selected submenu
    show_submenu: bool,                  // Flag to control submenu visibility
    start_time: Arc<Mutex<Instant>>,     // Application start time
    last_log_time: Instant,              // Last time a log message was written
    menu_settings: MenuSettings,         // Menu settings
}

impl AdminApp {
    // Constructor to initialize a new AdminApp instance
    pub fn new(start_time: Arc<Mutex<Instant>>) -> Self {
        Self {
            debug_log_admin: Arc::new(Mutex::new(String::new())),
            debug_log_error: Arc::new(Mutex::new(String::new())), // Initialize error log
            admin_submenu: Some(0), // Initialize with the first submenu selected
            show_submenu: true, // Set to true to show the submenu
            start_time,
            last_log_time: Instant::now(),
            menu_settings: default_settings(),
        }
    }

    // Function to log messages with a timestamp, placing the most recent entry at the top
    pub fn log_message(&mut self, message: &str) {
        let elapsed_time = Instant::now().duration_since(self.last_log_time);
        if elapsed_time < Duration::from_millis(500) {
            return; // Skip logging if less than 500 milliseconds have passed
        }
        self.last_log_time = Instant::now();

        let mut log = self.debug_log_admin.lock().expect("Failed to lock debug log");
        let elapsed = {
            let start_time = self.start_time.lock().expect("Failed to lock start time mutex");
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

        // Append log entry to debug.log
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug.log")
            .expect("Failed to open debug log file");
        file.write_all(new_entry.as_bytes())
            .expect("Failed to write to debug log file");
    }

    // Function to handle displaying the submenu
    fn handle_submenu(&mut self, ui: &mut egui::Ui) {
        self.log_message("handle_submenu called");

        if self.show_submenu {
            if let Some(submenu) = self.admin_submenu {
                match submenu {
                    0 => {
                        ui.add(self.menu_settings.apply_label("Network Setup menu", submenu == 0));
                        self.log_message("Network Setup menu selected");
                    },
                    1 => {
                        ui.add(self.menu_settings.apply_label("Data Speed menu", submenu == 1));
                        self.log_message("Data Speed menu selected");
                    },
                    2 => {
                        ui.add(self.menu_settings.apply_label("Packet Capture menu", submenu == 2));
                        self.log_message("Packet Capture menu selected");
                    },
                    _ => {}
                }
            } else {
                self.log_message("No submenu selected");
            }
        } else {
            self.log_message("show_submenu is false");
        }
    }
}

// Implement the eframe::App trait for AdminApp
impl eframe::App for AdminApp {
    // Update function to handle UI rendering and logic
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Log a message to show that the update function is called
        self.log_message("Admin menu update function called");

        // Display the debug info log in the left side panel
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Debug Info Log");
            let debug_info = self.debug_log_admin.lock().expect("Failed to lock debug log admin mutex");
            ui.label(debug_info.to_string());
        });

        // Display the debug error log in the right side panel
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Debug Error Log");
            let debug_error = self.debug_log_error.lock().expect("Failed to lock debug log error mutex");
            ui.label(debug_error.to_string());
        });

        // Display the central panel elements
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_space(ui.available_size());
            egui::Area::new("centered_area".into())
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(self.menu_settings.apply_title("Admin Menu"));
                        ui.add_space(20.0);
                        self.handle_submenu(ui); // Properly handle submenu here
                    });
                });
        });

        // Request a repaint every 500 milliseconds to update the UI
        ctx.request_repaint_after(Duration::from_millis(500));
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
