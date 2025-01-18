// Import necessary modules and traits
use eframe::egui;
use crate::admin_app::AdminApp;
use crate::network_setup_menu::{self, MenuState as NetworkSetupMenuState};
use crate::data_speed_menu::{self, MenuState as DataSpeedMenuState};
use crate::packet_capture_menu::{self, MenuState as PacketCaptureMenuState};
use crate::menu_settings; // Import the menu_settings module

pub fn handle_submenu(app: &mut AdminApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    if app.get_show_submenu() {
        if let Some(submenu) = app.get_admin_submenu() {
            match submenu {
                0 => {
                    let mut network_setup_state = NetworkSetupMenuState::default();
                    network_setup_menu::show_menu(ui, &mut network_setup_state);
                    if network_setup_state.selected_option == 3 {
                        app.set_show_submenu(false);
                    }
                },
                1 => {
                    let mut data_speed_state = DataSpeedMenuState::default();
                    data_speed_menu::show_menu(ui, |ui| {
                        if ui.add(egui::Button::new(
                            egui::RichText::new("Back")
                                .font(egui::FontId::proportional(app.menu_settings.option_font_size))
                                .color(app.menu_settings.default_color)
                                .strong()
                        )).clicked() {
                            app.set_show_submenu(false);
                        }
                    }, &mut data_speed_state);
                },
                2 => {
                    let mut packet_capture_state = PacketCaptureMenuState::default();
                    packet_capture_menu::show_menu(ui, &mut packet_capture_state);
                    if packet_capture_state.selected_option == 3 {
                        app.set_show_submenu(false);
                    }
                },
                _ => {}
            }
        }
    } else {
        // Define the admin menu options
        let admin_options = ["Network Setup", "Data Speed", "Packet Capture", "Exit"];
        let mut local_submenu = app.get_admin_submenu().unwrap_or(3);

        for (i, &option) in admin_options.iter().enumerate() {
            let text = if i == local_submenu {
                egui::RichText::new(option)
                    .font(egui::FontId::proportional(app.menu_settings.option_font_size))
                    .color(app.menu_settings.selected_color)
                    .strong()
            } else {
                egui::RichText::new(option)
                    .font(egui::FontId::proportional(app.menu_settings.option_font_size))
                    .color(app.menu_settings.default_color)
            };

            let button_response = menu_settings::create_button(
                ui, 
                text, 
                &app.menu_settings, 
                i == local_submenu
            );

            if button_response.clicked() || (ctx.input(|i| i.key_pressed(egui::Key::Enter)) && i == local_submenu) {
                match i {
                    0 | 1 | 2 => {
                        app.set_admin_submenu(Some(i));
                        app.set_show_submenu(true); // Show submenu when Enter is pressed or button clicked
                        app.get_logger().log_message(&format!("Selected submenu: {}", option)); // Log submenu selection
                    },
                    3 => {
                        app.get_logger().log_message("Exiting the application"); // Log exit action
                        std::process::exit(0); // Exit the program when "Exit" is selected
                    },
                    _ => {}
                }
            }
        }

        // Use global menu navigation
        menu_settings::handle_menu_navigation(ui, &mut local_submenu, admin_options.len());
        
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            if local_submenu == 3 {
                app.get_logger().log_message("Exiting the application via Enter key"); // Log exit action
                std::process::exit(0); // Exit the program when "Exit" is selected
            }
            app.set_show_submenu(true); // Only show submenu when Enter is pressed
        }
        app.set_admin_submenu(Some(local_submenu)); // Update selection
    }
}
