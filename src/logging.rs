use simplelog::*;
use std::fs::{File, OpenOptions};
use std::sync::{Arc, Mutex};
use log::{LevelFilter, info, error};
use std::sync::Once;
use std::collections::HashSet;
use std::io::{Write, Error};

static INIT_LOG: Once = Once::new();

#[derive(Clone)]
pub struct LogBuffers {
    pub info_buffer: Arc<Mutex<Vec<String>>>,
    pub error_buffer: Arc<Mutex<Vec<String>>>,
    pub info_log_set: Arc<Mutex<HashSet<String>>>,
    pub error_log_set: Arc<Mutex<HashSet<String>>>,
}

pub fn init_logging() -> Result<(), Error> {
    INIT_LOG.call_once(|| {
        if let Err(e) = File::create("debug_info.log") {
            error!("Failed to create info log file: {}", e);
            return;
        }
        if let Err(e) = File::create("debug_error.log") {
            error!("Failed to create error log file: {}", e);
            return;
        }

        let log_file_info = OpenOptions::new().write(true).create(true).append(true).open("debug_info.log").unwrap();
        let log_file_error = OpenOptions::new().write(true).create(true).append(true).open("debug_error.log").unwrap();

        if let Err(e) = CombinedLogger::init(vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), log_file_info),
            WriteLogger::new(LevelFilter::Error, Config::default(), log_file_error),
        ]) {
            error!("Failed to initialize logging: {}", e);
        }
    });

    Ok(())
}

pub fn log_info(log_buffers: &LogBuffers, message: &str, restrict: bool) {
    if restrict {
        let mut log_set = log_buffers.info_log_set.lock().unwrap_or_else(|_| {
            error!("Failed to lock info log set");
            std::process::exit(1);
        });

        if log_set.contains(message) {
            return;
        }
        log_set.insert(message.to_string());
    }

    let mut buffer = log_buffers.info_buffer.lock().unwrap_or_else(|_| {
        error!("Failed to lock info buffer");
        std::process::exit(1);
    });
    buffer.push(message.to_string());
    if buffer.len() > 1000 {
        buffer.clear();
    }
    info!("{}", message);

    // Write to file directly to ensure log persistence
    if let Err(e) = File::options().append(true).open("debug_info.log").and_then(|mut file| writeln!(file, "{}", message)) {
        error!("Failed to write to info log file: {}", e);
    }
}

pub fn log_error(log_buffers: &LogBuffers, message: &str, restrict: bool) {
    if restrict {
        let mut log_set = log_buffers.error_log_set.lock().unwrap_or_else(|_| {
            error!("Failed to lock error log set");
            std::process::exit(1);
        });

        if log_set.contains(message) {
            return;
        }
        log_set.insert(message.to_string());
    }

    let mut buffer = log_buffers.error_buffer.lock().unwrap_or_else(|_| {
        error!("Failed to lock error buffer");
        std::process::exit(1);
    });
    buffer.push(message.to_string());
    if buffer.len() > 1000 {
        buffer.clear();
    }
    error!("{}", message);

    // Write to file directly to ensure log persistence
    if let Err(e) = File::options().append(true).open("debug_error.log").and_then(|mut file| writeln!(file, "{}", message)) {
        error!("Failed to write to error log file: {}", e);
    }
}
