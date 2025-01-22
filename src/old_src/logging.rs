use simplelog::*;
use std::fs::File;
use std::sync::{Arc, Mutex};
use log::{LevelFilter, info, error};
use std::sync::Once;
use chrono::Local;

static INIT_LOG: Once = Once::new();

// Shared log buffers for info and error
#[derive(Clone)]
pub struct LogBuffers {
    pub info_buffer: Arc<Mutex<Vec<String>>>,
    pub error_buffer: Arc<Mutex<Vec<String>>>,
}

// Function to initialize logging
pub fn init_logging() -> LogBuffers {
    // Create shared log buffers for info and error
    let log_buffer_info = Arc::new(Mutex::new(Vec::new()));
    let log_buffer_error = Arc::new(Mutex::new(Vec::new()));

    // Initialize logging only once
    INIT_LOG.call_once(|| {
        let log_file_info = File::create("debug_info.log").unwrap_or_else(|e| {
            eprintln!("Failed to create info log file: {}", e);
            std::process::exit(1);
        });
        let log_file_error = File::create("debug_error.log").unwrap_or_else(|e| {
            eprintln!("Failed to create error log file: {}", e);
            std::process::exit(1);
        });

        CombinedLogger::init(vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file_info),
            WriteLogger::new(LevelFilter::Error, Config::default(), log_file_error),
        ])
        .unwrap_or_else(|e| {
            eprintln!("Failed to initialize logging: {}", e);
            std::process::exit(1);
        });
    });

    // Start a background thread to periodically update the log buffers
    start_log_buffer_update_thread(log_buffer_info.clone(), "debug_info.log");
    start_log_buffer_update_thread(log_buffer_error.clone(), "debug_error.log");

    LogBuffers {
        info_buffer: log_buffer_info,
        error_buffer: log_buffer_error,
    }
}

// Function to start a background thread to update log buffers
fn start_log_buffer_update_thread(buffer: Arc<Mutex<Vec<String>>>, file_path: &str) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            if let Ok(lines) = std::fs::read_to_string(file_path) {
                let mut buffer = buffer.lock().unwrap();
                buffer.clear();
                for line in lines.lines().rev() {
                    buffer.push(line.to_string());
                }
            }
        }
    });
}

// Function to log informational messages with timestamps
pub fn log_info(message: &str) {
    let timestamped_message = format!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message);
    info!("{}", timestamped_message);
}

// Function to log error messages with timestamps
pub fn log_error(message: &str) {
    let timestamped_message = format!("[{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), message);
    error!("{}", timestamped_message);
}
