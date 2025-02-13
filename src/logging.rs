use log::{info, error};
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode, ColorChoice};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;

static LOGGED_MESSAGES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn init_module() -> Result<(), String> {
    // Initialize the logger
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        .map_err(|e| format!("Failed to boot logger: {}", e))?;

    let initialization_passed = true;

    if initialization_passed {
        debug_info("logging module is online");
        Ok(())
    } else {
        Err("logging module boot failed".to_string())
    }
}

pub fn debug_info(message: &str) {
    let mut logged_messages = LOGGED_MESSAGES.lock().expect("Failed to lock LOGGED_MESSAGES");
    if !logged_messages.contains(message) {
        logged_messages.insert(message.to_string());
        info!("{}", message);
    }
}

pub fn debug_error(message: &str) {
    error!("{}", message);
}