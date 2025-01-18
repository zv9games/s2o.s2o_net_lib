use std::sync::{Arc, Mutex, atomic::AtomicBool};
use std::thread;
use std::time::Duration;

pub fn start_error_thread(error_updated: &Arc<AtomicBool>, debug_error: &Arc<Mutex<String>>) {
    let error_updated_clone = Arc::clone(error_updated);
    let debug_error_clone = Arc::clone(debug_error);
    thread::spawn(move || {
        loop {
            if rand::random::<f64>() < 0.5 { // Increased probability to 50%
                {
                    let mut error = debug_error_clone.lock().unwrap();
                    error.push_str(&format!("Error: Something went wrong! at {}\n", chrono::Local::now()));
                }
                error_updated_clone.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            thread::sleep(Duration::from_millis(500)); // Reduced sleep time to 500 milliseconds
        }
    });
}
