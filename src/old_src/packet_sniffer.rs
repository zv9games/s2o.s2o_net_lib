use crate::logging::{LogBuffers, log_info};
use std::cell::RefCell;

#[link(name = "packet_sniffer")]
extern "C" {
    fn initialize_sniffer() -> i32;
    fn start_sniffer(callback: extern "C" fn(packet: *const u8, length: usize));
}

thread_local! {
    static LOG_BUFFERS: RefCell<Option<LogBuffers>> = RefCell::new(None);
}

/// Initializes the packet sniffer and returns a result indicating success or failure.
pub fn init_sniffer() -> Result<(), String> {
    let result = unsafe { initialize_sniffer() };
    if result == 0 {
        Ok(())
    } else {
        Err("Sniffer initialization failed".to_string())
    }
}

/// Callback function to handle captured packets.
extern "C" fn handle_packet(packet: *const u8, length: usize) {
    let data = unsafe { std::slice::from_raw_parts(packet, length) };
    LOG_BUFFERS.with(|log_buffers| {
        log_buffers.borrow().as_ref().map_or_else(
            || eprintln!("Log buffers not available"),
            |buffers| log_info(buffers, &format!("Packet captured: {:?}", data)),
        );
    });
}

/// Starts the packet sniffer with the provided logging buffers.
pub fn run_sniffer(log_buffers: &LogBuffers) {
    LOG_BUFFERS.with(|lb| {
        *lb.borrow_mut() = Some(log_buffers.clone());
    });

    let callback: extern "C" fn(packet: *const u8, length: usize) = handle_packet;
    unsafe { start_sniffer(callback) };
}