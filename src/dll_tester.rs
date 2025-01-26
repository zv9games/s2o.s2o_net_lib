use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use libloading::{Library, Symbol};
use once_cell::sync::Lazy;

type InitSniffer = unsafe extern "C" fn() -> i32;
type RunSniffer = unsafe extern "C" fn(callback: extern "C" fn(data: *const u8, length: i32));
type StopSniffer = unsafe extern "C" fn() -> i32;

static CAPTURED_PACKETS: Lazy<Arc<Mutex<Vec<Vec<u8>>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

extern "C" fn packet_callback(data: *const u8, length: i32) {
    println!("Packet captured: length = {}", length);
    if length > 0 {
        let packet = unsafe { std::slice::from_raw_parts(data, length as usize) };
        let mut packets = CAPTURED_PACKETS.lock().expect("Failed to lock captured packets");
        packets.push(packet.to_vec());
    }
}

fn test_invalid_dll_path() {
    let library_path = "invalid/path/to/dll.dll";

    unsafe {
        if let Err(e) = Library::new(library_path) {
            println!("Failed to load DLL as expected: {:?}", e);
        } else {
            eprintln!("Error: DLL loaded unexpectedly");
        }
    }
}

fn test_missing_exports() {
    let library_path = "c:/S2O/s2o_net_lib/src/s2o_dll/packet_sniffer.dll";

    unsafe {
        let lib = Library::new(library_path).expect("Failed to load DLL");

        if let Err(e) = lib.get::<Symbol<unsafe extern "C" fn()>>(b"non_existent_function\0") {
            println!("Failed to load missing function as expected: {:?}", e);
        } else {
            eprintln!("Error: Missing function loaded unexpectedly");
        }
    }
}

fn test_packet_data_integrity() {
    let library_path = "c:/S2O/s2o_net_lib/src/s2o_dll/packet_sniffer.dll";

    unsafe {
        let lib = Library::new(library_path).expect("Failed to load DLL");

        let init_sniffer: Symbol<InitSniffer> = lib.get(b"init_sniffer\0").expect("Failed to load init_sniffer function");
        let run_sniffer: Symbol<RunSniffer> = lib.get(b"run_sniffer\0").expect("Failed to load run_sniffer function");
        let stop_sniffer: Symbol<StopSniffer> = lib.get(b"stop_sniffer\0").expect("Failed to load stop_sniffer function");

        if init_sniffer() != 0 {
            eprintln!("Failed to initialize sniffer");
            return;
        }

        run_sniffer(packet_callback);

        // Run sniffer for a short duration, then stop
        thread::sleep(Duration::from_secs(10));

        if stop_sniffer() != 0 {
            eprintln!("Failed to stop sniffer");
        }

        // Verify packet data
        let packets = CAPTURED_PACKETS.lock().expect("Failed to lock captured packets");
        for packet in packets.iter() {
            assert_eq!(packet.len(), packet.iter().count(), "Packet data integrity check failed");
            println!("Captured packet: {:?}", packet);
        }
    }
}

fn main() {
    // Run the original test
    let library_path = "c:/S2O/s2o_net_lib/src/s2o_dll/packet_sniffer.dll";

    unsafe {
        let lib = Library::new(library_path).expect("Failed to load DLL");

        let init_sniffer: Symbol<InitSniffer> = lib.get(b"init_sniffer\0").expect("Failed to load init_sniffer function");
        let run_sniffer: Symbol<RunSniffer> = lib.get(b"run_sniffer\0").expect("Failed to load run_sniffer function");
        let stop_sniffer: Symbol<StopSniffer> = lib.get(b"stop_sniffer\0").expect("Failed to load stop_sniffer function");

        if init_sniffer() != 0 {
            eprintln!("Failed to initialize sniffer");
            return;
        }

        run_sniffer(packet_callback);

        // Run sniffer for a short duration, then stop
        thread::sleep(Duration::from_secs(10));

        if stop_sniffer() != 0 {
            eprintln!("Failed to stop sniffer");
        }

        // Print captured packets
        let packets = CAPTURED_PACKETS.lock().expect("Failed to lock captured packets");
        for packet in packets.iter() {
            println!("Captured packet: {:?}", packet);
        }
    }

    // Run additional tests
    test_invalid_dll_path();
    test_missing_exports();
    test_packet_data_integrity();
}
