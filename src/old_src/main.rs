mod logging;
mod initialization;
mod app_state;
mod menu_rendering;
mod s_menu;
mod p_menu;
mod ns_menu;
mod ds_menu;
mod pc_menu;
mod packet_sniffer;
mod pcnc;

use log::{info, error};
use std::sync::{Arc, Mutex};
use std::env;

fn main() {
    // Ensure the PATH is set early
    let dll_path = "C:\\s2o\\s2o_net_lib\\src\\s2o_dll"; // Ensure this path is correct and accessible
    let current_path = env::var("PATH").unwrap_or_default();
    let new_path = format!("{};{}", dll_path, current_path);
    env::set_var("PATH", new_path.clone());
    println!("Setting PATH: {}", new_path); // Debug print
    println!("PATH set to: {}", new_path);

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
