// Import necessary modules
mod logging;
mod initialization;
mod app_state;
mod admin_check;
mod gui_engine;
mod gui_engine_animation;
mod gui_engine_menu;
mod gui_engine_style;
mod s_menu;
mod p_menu;
mod pc_menu;
mod ns_menu;
mod ds_menu;

//mod packet_capture;
//mod nc;

//use std::sync::Arc;
//use std::sync::Mutex;

fn main() {
    // Call the s2o_bootup function
    if let Err(e) = initialization::s2o_bootup() {
        logging::debug_error(&format!("Failed to boot application: {}", e));
        std::process::exit(1);
    }
}
    // Example usage of logging
    //logging::debug_info("Application has started.");
    //logging::debug_error("No errors encountered.");

