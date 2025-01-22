use crate::menu_rendering::{MenuItem, MenuState, render_menu};
use std::sync::{Arc, Mutex};
use log::{info, error};

pub fn render_ns_menu(ctx: &eframe::egui::Context, state: &mut MenuState, log_buffer: Arc<Mutex<Vec<String>>>) {
    // Define the menu items for the Network Settings menu
    let menu_items = vec![
        MenuItem {
            label: "Wi-Fi".to_string(),
            action: Some(Box::new(|| {
                info!("Wi-Fi Settings selected");
                // TODO: Add actual logic for Wi-Fi Settings here
            })),
        },
        MenuItem {
            label: "Ethernet".to_string(),
            action: Some(Box::new(|| {
                info!("Ethernet Settings selected");
                // TODO: Add actual logic for Ethernet Settings here
            })),
        },
        MenuItem {
            label: "Diagnostics".to_string(),
            action: Some(Box::new(|| {
                info!("Network Diagnostics selected");
                // TODO: Add actual logic for Network Diagnostics here
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: None,  // No action needed, handled in program_menu.rs
        },
    ];

    // Render the Network Settings menu using the render_menu function
    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("S2O's s2o_net_lib Crate");
        ui.add_space(20.0);  // Add space between the heading and the options
        if let Err(e) = render_menu(ctx, "S2O's s2o_net_lib Crate", &menu_items, state, log_buffer) {
            error!("Failed to render NS menu: {:?}", e);
        }
    });
}
