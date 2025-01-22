use crate::menu_rendering::{MenuItem, MenuState, render_menu};
use crate::app_state::{SharedAppState, AppState}; // Import SharedAppState and AppState
use crate::logging::LogBuffers; // Import LogBuffers
use std::sync::{Arc, Mutex};
use log::{info, error};

pub fn render_ds_menu(ctx: &eframe::egui::Context, state: &mut MenuState, shared_state: &SharedAppState, log_buffers: &LogBuffers, log_buffer: Arc<Mutex<Vec<String>>>) {
    // Define the menu items for the Data Speed menu
    let menu_items = vec![
        MenuItem {
            label: "Check Speed".to_string(),
            action: Some(Box::new(|| {
                info!("Check Speed selected");
                // TODO: Add actual logic for Check Speed here
            })),
        },
        MenuItem {
            label: "History".to_string(),
            action: Some(Box::new(|| {
                info!("History selected");
                // TODO: Add actual logic for History here
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new({
                let shared_state = shared_state.clone();
                move || {
                    shared_state.set_state(AppState::ProgramMenu);
                }
            })),
        },
    ];

    // Render the Data Speed menu using the render_menu function
    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("S2O's s2o_net_lib Crate");
        ui.add_space(20.0);  // Add space between the heading and the options
        if let Err(e) = render_menu(ctx, "S2O's s2o_net_lib Crate", &menu_items, state, log_buffers, log_buffer) {
            error!("Failed to render DS menu: {:?}", e);
        }
    });
}
