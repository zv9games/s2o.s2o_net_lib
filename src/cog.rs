// Import necessary modules for logging, state management, and menu handling
use crate::{
    logging,
    logging::debug_info,
    app_state::{AppState, get_app_state, check_admin_and_set_menu},
    s_menu,
    p_menu,
};
use std::{
    thread,
    time::Duration,
};

/// Simulates initialization of the cog module.
pub fn init_module() -> Result<(), String> {
    debug_info("cog module is online");
    Ok(())
}

/// Main loop that manages the application's state machine.
fn cog_loop() {
    loop {
        match get_app_state() {
            Some(AppState::SMenu) => {
                debug_info("Rendering SMenu");
                s_menu::init_module().expect("Failed to initialize SMenu");
            },
            Some(AppState::PMenu) => {
                debug_info("Rendering PMenu");
                p_menu::init_module().expect("Failed to initialize PMenu");
            },
            _ => {
                logging::debug_error("Unsupported or uninitialized app state");
            }
        }

        // Pause to prevent excessive CPU usage
        thread::sleep(Duration::from_secs(1));
    }
}

/// Starts the cog loop in a new thread to handle application states asynchronously.
pub fn start_cog_loop() {
    thread::spawn(cog_loop);
}