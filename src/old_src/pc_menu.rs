use crate::pcnc::handle_packet_capture;
use crate::menu_rendering::{MenuItem, MenuState, render_menu};
use crate::app_state::SharedAppState;
use crate::logging::{LogBuffers, log_info};
use std::env;

pub fn execute_ui(ctx: &eframe::egui::Context, _shared_state: &SharedAppState, state: &mut MenuState, log_buffers: &LogBuffers) {
    log_info(log_buffers, "Initiating Packet Capture Menu UI");

    // Validate the current PATH before using the DLL
    let current_path = env::var("PATH").unwrap_or_default();
    log_info(log_buffers, &format!("Current PATH: {}", current_path));

    let menu_items = vec![
        MenuItem {
            label: "Start Capture".to_string(),
            action: Some(Box::new({
                let log_buffers = log_buffers.clone();
                move || handle_packet_capture(&log_buffers)
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new({
                let log_buffers = log_buffers.clone();
                move || {
                    log_info(&log_buffers, "Exiting Packet Capture Menu.");
                    std::process::exit(0);
                }
            })),
        },
    ];

    render_menu(ctx, "Packet Capture Menu", &menu_items, state, log_buffers, log_buffers.info_buffer.clone())
        .expect("Failed to render menu");
}
