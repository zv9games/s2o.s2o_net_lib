use crate::pcnc::handle_packet_capture;
use crate::menu_rendering::{MenuItem, MenuContext, render_menu};
use crate::app_state::SharedAppState;
use crate::logging::{LogBuffers, log_info};
use std::env;

pub fn run_packet_capture_ui(ctx: &eframe::egui::Context, _shared_state: &SharedAppState, state: &mut MenuContext, log_buffers: &LogBuffers) {
    log_info(log_buffers, "Launching Packet Capture Interface");

    // Check and log the current PATH for DLL access
    let path = env::var("PATH").unwrap_or_default();
    log_info(log_buffers, &format!("Current system PATH: {}", path));

    let menu_options = vec![
        MenuItem::new("Begin Capture", move || handle_packet_capture(log_buffers)),
        MenuItem::new("Exit", move || {
            log_info(log_buffers, "Exiting Packet Capture Interface.");
            std::process::exit(0);
        }),
    ];

    if let Err(e) = render_menu(ctx, "Packet Capture Interface", &menu_options, state, log_buffers, log_buffers.info_buffer.clone()) {
        log_info(log_buffers, &format!("Failed to display menu: {}", e));
    }
}

impl MenuItem {
    fn new(label: &str, action: impl Fn() + 'static) -> Self {
        MenuItem {
            label: label.to_string(),
            action: Some(Box::new(action)),
        }
    }
}