use crate::menu_rendering::{MenuItem, MenuContext, render_menu};
use crate::app_state::{SharedAppState, AppState};
use crate::logging::LogBuffers;
use std::sync::{Arc, Mutex};
use log::{info, error};

pub fn render_network_settings_menu(ctx: &eframe::egui::Context, state: &mut MenuContext, shared_state: &SharedAppState, log_buffers: &LogBuffers, log_buffer: Arc<Mutex<Vec<String>>>) {
    let menu_items = vec![
        MenuItem::new("Wi-Fi", || {
            info!("Wi-Fi settings accessed");
            // TODO: Implement Wi-Fi settings logic
        }),
        MenuItem::new("Ethernet", || {
            info!("Ethernet settings accessed");
            // TODO: Implement Ethernet settings logic
        }),
        MenuItem::new("Diagnostics", || {
            info!("Network diagnostics started");
            // TODO: Implement network diagnostics logic
        }),
        MenuItem::new("Exit", {
            let shared_state = shared_state.clone();
            move || {
                shared_state.set_state(AppState::ProgramMenu);
            }
        }),
    ];

    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("S2O's Network Configuration Tool");
        ui.add_space(20.0);
        if let Err(e) = render_menu(ctx, "Network Options", &menu_items, state, log_buffers, log_buffer) {
            error!("Failed to render Network Settings menu: {}", e);
        }
    });
}

impl MenuItem {
    fn new(label: &str, action: impl Fn() + 'static) -> Self {
        MenuItem {
            label: label.to_string(),
            action: Some(Box::new(action)),
        }
    }
}