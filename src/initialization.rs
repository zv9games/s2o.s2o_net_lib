use crate::admin_check;
use crate::logging;
use crate::app_state;
use crate::gui_engine;
use crate::gui_engine_animation;
use crate::s_menu;
use crate::p_menu;
use crate::pc_menu;
use crate::ds_menu;
use crate::ns_menu;



pub fn s2o_bootup() -> Result<(), String> {
    // Print a message to indicate the initialization process has started
    logging::debug_info("Initialization is being called");

    
    // Step 1: Check modules
    check_modules()?;

    // Step 2: Set up configurations
    if let Err(e) = setup_configurations() {
        return Err(format!("Configuration setup failed: {}", e));
    }

    // Step 3: Check environment settings
    if let Err(e) = check_environment() {
        return Err(format!("Environment check failed: {}", e));
    }

    // Step 4: Initialize core components
    if let Err(e) = initialize_components() {
        return Err(format!("Component initialization failed: {}", e));
    }

    logging::debug_info("Initialization complete");
    gui_engine::start_gui();

    Ok(())
}

fn check_modules() -> Result<(), String> {
    if let Err(e) = logging::init_module() {
        logging::debug_info("Failed to initialize logging module...");
        return Err(format!("Failed to initialize logging module: {}", e));
    }

    admin_check::init_module()?;
    app_state::init_module()?;
    gui_engine::init_module()?;
	gui_engine_animation::init_module()?;
    s_menu::init_module()?;
    p_menu::init_module()?;
    pc_menu::init_module()?;
    ns_menu::init_module()?;
    ds_menu::init_module()?;
    // packet_capture::init_module()?;
    // nc::init_module()?;
    // module_12::init_module()?;
    // module_13::init_module()?;
    // module_14::init_module()?;
    // module_15::init_module()?;
    // module_16::init_module()?;

    logging::debug_info("All modules initialized successfully");

    Ok(())
}

fn setup_configurations() -> Result<(), String> {
    // Add your configuration setup logic here
	
	// Load fonts
    gui_engine::load_fonts()?;
    logging::debug_info("Fonts setup complete.");

    
    logging::debug_info("Check 1");
    // Simulate configuration setup
    Ok(())
}

fn check_environment() -> Result<(), String> {
    // Add your environment check logic here
    logging::debug_info("Check 2");
    // Simulate environment check
    Ok(())
}

fn initialize_components() -> Result<(), String> {
    // Add your core component initialization logic here
    logging::debug_info("Check 3");
    // Simulate component initialization
    Ok(())
}
