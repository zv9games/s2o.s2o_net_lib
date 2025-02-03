extern crate libc;

use libc::{c_char, c_void};
use std::ffi::CString;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use log::{info, error};
use once_cell::sync::Lazy;
use crate::logging::LogBuffers; // Import LogBuffers
use eframe::egui::Window;

// Declare external WinDivert functions
#[link(name = "WinDivert")]
extern "C" {
    fn WinDivertOpen(filter: *const c_char, layer: u8, priority: i16, flags: u64) -> *mut c_void;
    fn WinDivertRecv(handle: *mut c_void, p_packet: *mut c_void, packet_len: u32, p_addr: *mut c_void, read_len: *mut u32) -> bool;
    fn WinDivertClose(handle: *mut c_void) -> bool;
}

static CAPTURING: AtomicBool = AtomicBool::new(false);
static mut HANDLE: *mut c_void = ptr::null_mut();
static CAPTURED_PACKETS: Lazy<Arc<Mutex<Vec<Vec<u8>>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub fn start_capture(log_buffers: LogBuffers) -> Result<(), String> {
    if CAPTURING.load(Ordering::SeqCst) {
        return Err("Capture already running.".to_string());
    }

    let filter = CString::new("true").unwrap();
    unsafe {
        HANDLE = WinDivertOpen(filter.as_ptr(), 0, 0, 0);
        if HANDLE == ptr::null_mut() {
            let error_msg = format!("Failed to open WinDivert handle. Error: {}", std::io::Error::last_os_error());
            error!("{}", error_msg);
            return Err(error_msg);
        }

        CAPTURING.store(true, Ordering::SeqCst);
        std::thread::spawn(move || capture_packets(log_buffers));
        info!("Packet capturing started.");
        Ok(())
    }
}

pub fn stop_capture(_log_buffers: &LogBuffers) -> Result<(), String> {
    if !CAPTURING.load(Ordering::SeqCst) {
        return Err("No capture to stop.".to_string());
    }

    unsafe {
        if !WinDivertClose(HANDLE) {
            let error_msg = format!("Failed to close WinDivert handle. Error: {}", std::io::Error::last_os_error());
            error!("{}", error_msg);
            return Err(error_msg);
        }

        CAPTURING.store(false, Ordering::SeqCst);
        info!("Packet capturing stopped.");
        Ok(())
    }
}

pub fn capture_packets(_log_buffers: LogBuffers) {
    unsafe {
        let mut packet: [u8; 65535] = [0; 65535];
        let mut read_len: u32 = 0;
        let mut addr: [u8; 16] = [0; 16];

        while CAPTURING.load(Ordering::SeqCst) {
            if WinDivertRecv(HANDLE, packet.as_mut_ptr() as *mut c_void, packet.len() as u32, addr.as_mut_ptr() as *mut c_void, &mut read_len) {
                info!("Captured packet of size: {}", read_len);
                let mut packets = CAPTURED_PACKETS.lock().unwrap();
                packets.push(packet[..read_len as usize].to_vec());
            } else {
                error!("Failed to capture packet. Error: {}", std::io::Error::last_os_error());
            }
        }
    }
}

pub fn display_captured_packets(ctx: &eframe::egui::Context) {
    let packets = CAPTURED_PACKETS.lock().unwrap();
    Window::new("Captured Packet Data").show(ctx, |ui| {
        for packet in packets.iter() {
            ui.label(format!("{:?}", packet));
        }
    });
}
