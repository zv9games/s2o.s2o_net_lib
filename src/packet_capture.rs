use std::sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}};
use std::sync::atomic::{AtomicBool};
use eframe::egui::Context;
use once_cell::sync::Lazy;
use crate::logging::{LogBuffers, log_info, log_error};
use crate::nc::{start_packet_capture, stop_packet_capture, count_packets, print_packet_data};
use std::io::Error;

// Wrapping AtomicBool in Lazy for proper initialization
pub static CAPTURING: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
pub static SHOW_PACKET_POPUP: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
pub static SHOW_PACKET_COUNT_POPUP: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

// Channel for sending stop signals
pub static STOP_CAPTURE_SENDER: Lazy<Mutex<Option<Sender<bool>>>> = Lazy::new(|| Mutex::new(None));

pub fn init_packet(log_buffers: &Arc<LogBuffers>) -> Result<(), Error> {
    log_info(log_buffers, "Initializing packet_capture.", false);
    
    Ok(())
}

pub fn start_capture(log_buffers: &LogBuffers) {
    if let Err(e) = crate::nc::run_preliminary_tests(log_buffers) {
        log_error(log_buffers, &format!("Failed preliminary tests: {}", e), false);
    } else {
        let mut capturing = CAPTURING.lock().unwrap();
        *capturing = true;
        log_info(log_buffers, "Packet capture started.", false);
    }
}

pub fn stop_capture(log_buffers: &LogBuffers) {
    let sender = STOP_CAPTURE_SENDER.lock().unwrap().clone();
    let log_buffers = log_buffers.clone();
    std::thread::spawn(move || {
        if let Some(sender) = sender {
            sender.send(true).expect("Failed to send stop signal");
            log_info(&log_buffers, "Packet capture stopped.", false);
        } else {
            log_error(&log_buffers, "Stop sender not initialized.", false);
        }
    });
}

pub fn spawn_capture_thread(ctx: Arc<Context>, log_buffers: Arc<LogBuffers>) {
    log_info(&log_buffers, "Spawning packet capture thread...", false);
    let (sender, receiver): (Sender<bool>, Receiver<bool>) = channel(); // Create a new channel for each thread
    {
        let mut stop_sender = STOP_CAPTURE_SENDER.lock().unwrap();
        *stop_sender = Some(sender.clone()); // Use the sender to send the stop signal
    }
    std::thread::spawn(move || {
        let (lib, filter) = match crate::nc::load_dll(&log_buffers) {
            Ok((lib, filter)) => {
                log_info(&log_buffers, "DLL loaded successfully.", false);
                (lib, filter)
            },
            Err(e) => {
                log_error(&log_buffers, &format!("Failed to load DLL: {}", e), false);
                return;
            }
        };

        start_packet_capture(&log_buffers, lib.clone(), filter, CAPTURING.clone());
        log_info(&log_buffers, "Packet capture started in thread.", false);

        loop {
            ctx.request_repaint();
            if let Ok(_) = receiver.try_recv() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
            log_info(&log_buffers, "Packet capture thread running...", false);
        }

        stop_packet_capture(&log_buffers, CAPTURING.clone());
        crate::nc::unload_dll(&log_buffers, Some(lib));
        log_info(&log_buffers, "Packet capture thread terminated.", false);
    });
}

pub fn get_packet_data(log_buffers: &LogBuffers) -> Vec<String> {
    // Capture the output of print_packet_data and return as Vec<String>
    let mut buffer = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buffer);

    // Redirect stdout temporarily
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    std::io::copy(&mut writer, &mut handle).unwrap();

    // Execute function to print packet data
    print_packet_data(log_buffers);

    // Restore stdout
    std::io::copy(&mut writer, &mut handle).unwrap();

    // Convert buffer to Vec<String>
    String::from_utf8(buffer).unwrap().lines().map(|s| s.to_string()).collect()
}

pub fn get_packet_count(log_buffers: &LogBuffers) -> usize {
    let mut buffer = Vec::new();
    let mut writer = std::io::Cursor::new(&mut buffer);

    // Redirect stdout temporarily
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    std::io::copy(&mut writer, &mut handle).unwrap();

    // Execute function to count packets
    count_packets(log_buffers);

    // Restore stdout
    std::io::copy(&mut writer, &mut handle).unwrap();

    // Convert buffer to usize
    String::from_utf8(buffer).unwrap().trim().parse::<usize>().unwrap_or(0)
}
