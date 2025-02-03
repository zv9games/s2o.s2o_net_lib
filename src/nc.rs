use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use libloading::{Library, Symbol};
use once_cell::sync::Lazy;
use std::thread::JoinHandle;
use crate::logging::{log_info, log_error, LogBuffers}; // Correct import for LogBuffers
use std::io::Error;

// Define function types for WinDivert
type WinDivertOpen = unsafe extern "C" fn(filter: *const i8, layer: i32, priority: i16, flags: u64) -> *mut std::ffi::c_void;
type WinDivertRecv = unsafe extern "C" fn(handle: *mut std::ffi::c_void, p_packet: *mut u8, packet_len: u32, addr: *mut u8) -> bool;
type WinDivertClose = unsafe extern "C" fn(handle: *mut std::ffi::c_void) -> bool;

static CAPTURED_PACKETS: Lazy<Arc<Mutex<Vec<Vec<u8>>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

pub extern "C" fn packet_callback(data: *const u8, length: i32) {
    println!("Packet captured: length = {}", length);
    if length > 0 {
        let packet = unsafe { std::slice::from_raw_parts(data, length as usize) };
        let mut packets = CAPTURED_PACKETS.lock().unwrap();
        packets.push(packet.to_vec());
        println!("Packet added. New count: {}", packets.len());
    }
}

pub fn init_nc(log_buffers: &Arc<LogBuffers>) -> Result<(), Error> {
    log_info(log_buffers, "Initializing nc.", false);
    
    Ok(())
}

pub fn load_dll(log_buffers: &LogBuffers) -> Result<(Arc<Library>, CString), String> {
    let library_path = "c:/s2o/dll_tester/src/windivert/windivert.dll";
    log_info(log_buffers, &format!("Attempting to load DLL from path: {}", library_path), false); // Logging the attempt to load DLL

    unsafe {
        match Library::new(library_path) {
            Ok(lib) => {
                log_info(log_buffers, "DLL loaded successfully.", false); // Logging successful DLL load
                match CString::new("true") {
                    Ok(filter) => {
                        log_info(log_buffers, "Filter created successfully.", false); // Logging successful filter creation
                        Ok((Arc::new(lib), filter))
                    },
                    Err(e) => {
                        log_error(log_buffers, &format!("Failed to create filter: {}", e), false); // Logging filter creation failure
                        Err(format!("CString::new failed: {}", e))
                    }
                }
            },
            Err(e) => {
                log_error(log_buffers, &format!("Failed to load DLL: {}", e), false); // Logging DLL load failure
                Err(format!("Failed to load DLL: {}", e))
            }
        }
    }
}

pub fn unload_dll(log_buffers: &LogBuffers, lib: Option<Arc<Library>>) {
    if let Some(lib) = lib {
        drop(lib);
        log_info(log_buffers, "DLL unloaded successfully.", false); // Logging successful DLL unload
    } else {
        log_error(log_buffers, "DLL is not loaded.", false); // Logging DLL not loaded
    }
}

pub fn start_packet_capture(log_buffers: &LogBuffers, lib: Arc<Library>, filter: CString, stop_flag: Arc<Mutex<bool>>) -> JoinHandle<()> {
    let log_buffers = log_buffers.clone();
    thread::spawn(move || {
        log_info(&log_buffers, "Capture thread started", false); // Logging the start of the capture thread
        unsafe {
            let win_divert_open: Symbol<WinDivertOpen> = match lib.get(b"WinDivertOpen\0") {
                Ok(symbol) => symbol,
                Err(e) => {
                    log_error(&log_buffers, &format!("Failed to load WinDivertOpen function: {:?}", e), false); // Logging function load failure
                    return;
                }
            };
            let win_divert_recv: Symbol<WinDivertRecv> = match lib.get(b"WinDivertRecv\0") {
                Ok(symbol) => symbol,
                Err(e) => {
                    log_error(&log_buffers, &format!("Failed to load WinDivertRecv function: {:?}", e), false); // Logging function load failure
                    return;
                }
            };
            let win_divert_close: Symbol<WinDivertClose> = match lib.get(b"WinDivertClose\0") {
                Ok(symbol) => symbol,
                Err(e) => {
                    log_error(&log_buffers, &format!("Failed to load WinDivertClose function: {:?}", e), false); // Logging function load failure
                    return;
                }
            };

            let layer = 0; // Use the appropriate layer as needed
            let handle = win_divert_open(filter.as_ptr(), layer, 0, 0);
            if handle.is_null() {
                log_error(&log_buffers, "Failed to open WinDivert handle.", false); // Logging handle open failure
                return;
            }
            log_info(&log_buffers, "WinDivert handle opened successfully.", false); // Logging handle open success

            let mut packet = vec![0u8; 1500];
            let mut addr = vec![0u8; 64];

            log_info(&log_buffers, "Entering packet capture loop...", false); // Logging capture loop entry
            while !*stop_flag.lock().unwrap() {
                let received = win_divert_recv(handle, packet.as_mut_ptr(), packet.len() as u32, addr.as_mut_ptr());
                log_info(&log_buffers, &format!("win_divert_recv returned: {}", received), false); // Logging packet receive result

                if received {
                    log_info(&log_buffers, "Packet received.", false); // Logging packet receipt
                    packet_callback(packet.as_ptr(), packet.len() as i32);
                } else {
                    log_info(&log_buffers, "No packet received.", false); // Logging no packet received
                }
                thread::sleep(Duration::from_millis(100));
            }

            win_divert_close(handle);
            log_info(&log_buffers, "Capture thread stopped.", false); // Logging capture thread stop
        }
    })
}

pub fn stop_packet_capture(log_buffers: &LogBuffers, stop_flag: Arc<Mutex<bool>>) {
    *stop_flag.lock().unwrap() = true;
    log_info(log_buffers, "Packet capture stop signal sent.", false); // Logging stop signal sent
}

pub fn count_packets(log_buffers: &LogBuffers) {
    let packets = CAPTURED_PACKETS.lock().unwrap();
    log_info(log_buffers, &format!("Total packets captured: {}", packets.len()), false); // Logging packet count
}

pub fn print_packet_data(log_buffers: &LogBuffers) {
    let packets = CAPTURED_PACKETS.lock().unwrap();
    log_info(log_buffers, &format!("Total packets captured: {}", packets.len()), false); // Logging total packet count
    for packet in packets.iter() {
        log_info(log_buffers, &format!("Captured packet: {:?}", packet), false); // Logging each captured packet
    }
}

pub fn run_preliminary_tests(log_buffers: &LogBuffers) -> Result<(), String> {
    log_info(log_buffers, "Running preliminary tests...", false); // Logging start of preliminary tests

    match load_dll(log_buffers) {
        Ok((lib, filter)) => {
            log_info(log_buffers, "DLL loaded successfully for preliminary tests.", false); // Logging DLL load success

            unsafe {
                let win_divert_open: Symbol<WinDivertOpen> = match lib.get(b"WinDivertOpen\0") {
                    Ok(symbol) => symbol,
                    Err(e) => {
                        log_error(log_buffers, &format!("Failed to load WinDivertOpen function: {:?}", e), false); // Logging function load failure
                        return Err(format!("Failed to load WinDivertOpen function: {:?}", e));
                    }
                };
                let handle = win_divert_open(filter.as_ptr(), 0, 0, 0);
                if handle.is_null() {
                    log_error(log_buffers, "Failed to open WinDivert handle in preliminary test.", false); // Logging handle open failure
                    return Err("Failed to open WinDivert handle in preliminary test.".to_string());
                } else {
                    log_info(log_buffers, "WinDivert handle opened successfully in preliminary test.", false); // Logging handle open success
                }
            }

            unload_dll(log_buffers, Some(lib));
        },
        Err(e) => {
            log_error(log_buffers, &format!("Failed to load DLL during preliminary tests: {}", e), false); // Logging DLL load failure
            return Err(format!("Failed to load DLL during preliminary tests: {}", e));
        }
    }

    log_info(log_buffers, "Preliminary tests completed successfully.", false); // Logging test completion
    Ok(())
}
