mod logging;
mod initialization;
mod app_state;
mod gui_engine;
mod s_menu;
mod p_menu;
mod ns_menu;
mod ds_menu;
mod pc_menu;
mod packet_capture;
mod packet_sniffer;

use log::{info, error};
use std::sync::{Arc, Mutex};

fn main() {
    info!("Initializing logging...");
    if let Err(e) = logging::init_logging() {
        error!("Failed to initialize logging: {}", e);
        std::process::exit(1);
    }
    info!("Logging initialized successfully.");

    let log_buffers = logging::LogBuffers {
        info_buffer: Arc::new(Mutex::new(Vec::new())),
        error_buffer: Arc::new(Mutex::new(Vec::new())),
        info_log_set: Arc::new(Mutex::new(std::collections::HashSet::new())),
        error_log_set: Arc::new(Mutex::new(std::collections::HashSet::new())),
    };

    info!("Running the application initialization process...");
    if let Err(e) = initialization::initialize_application(&log_buffers) {
        error!("Failed to initialize application: {}", e);
        std::process::exit(1);
    }
    info!("Application initialization completed successfully.");
}