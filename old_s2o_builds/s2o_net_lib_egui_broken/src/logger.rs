// logger.rs

use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger {
    debug_log_admin: Arc<Mutex<VecDeque<String>>>, // Admin debug log with a fixed-size queue
    debug_log_error: Arc<Mutex<String>>,           // Error log
    start_time: Arc<Mutex<Instant>>,               // Application start time
}

impl Logger {
    pub fn new(start_time: Arc<Mutex<Instant>>) -> Self {
        Self {
            debug_log_admin: Arc::new(Mutex::new(VecDeque::with_capacity(50))), // Initialize with capacity 50
            debug_log_error: Arc::new(Mutex::new(String::new())),               // Initialize error log
            start_time,
        }
    }

    // Function to log messages with a timestamp, placing the most recent entry at the top
    pub fn log_message(&self, message: &str) {
        let mut log = self.debug_log_admin.lock().unwrap();
        let elapsed = {
            let start_time = self.start_time.lock().unwrap();
            start_time.elapsed()
        };
        let timestamp = format!("{:02}:{:02}:{:02}.{:03}", 
            elapsed.as_secs() / 3600,
            (elapsed.as_secs() % 3600) / 60,
            elapsed.as_secs() % 60,
            elapsed.subsec_millis()
        );
        let new_entry = format!("[{}] {}\n", timestamp, message);

        // Prepend the new log entry and maintain the limit of 50 entries
        if log.len() == 50 {
            log.pop_back(); // Remove the oldest entry if the limit is reached
        }
        log.push_front(new_entry.clone()); // Add the new entry at the front

        // Save the log to a file
        if let Err(e) = self.save_log_to_file(&new_entry) {
            eprintln!("Failed to save log entry: {:?}", e);
        }
    }

    // Function to save log entry to a file
    fn save_log_to_file(&self, entry: &str) -> std::io::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open("debug_info_log.txt")?;
        file.write_all(entry.as_bytes())?;
        Ok(())
    }

    // Function to get the debug log as a single string
    pub fn get_log(&self) -> String {
        let log = self.debug_log_admin.lock().unwrap();
        log.iter().cloned().collect::<String>()
    }

    // Function to get the error log
    pub fn get_error_log(&self) -> String {
        let log = self.debug_log_error.lock().unwrap();
        log.clone()
    }
}
