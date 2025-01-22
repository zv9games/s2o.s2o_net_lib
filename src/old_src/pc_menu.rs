use crate::menu::{MenuItem, MenuState, render_menu};
use crate::app_state::{SharedAppState, AppState};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use log::{info, error, debug};
use eframe::egui::{CentralPanel, SidePanel};
use crate::logging::log_info;
use once_cell::sync::Lazy;
use crate::packet_sniffer;

// Shared static variables for capturing packets
static CAPTURED_PACKETS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new()))); 
static CAPTURING: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

// Function to run the packet capture UI
pub fn run_ui(ctx: &eframe::egui::Context, shared_state: &SharedAppState, state: &mut MenuState, log_buffer: Arc<Mutex<Vec<String>>>) {
    log_info("Running UI for packet capture menu...");
    render_pc_menu(ctx, shared_state, state, log_buffer);
}

// Function to render captured packets
fn render_packets(ctx: &eframe::egui::Context) {
    log_info("Rendering packets...");
    SidePanel::right("captured_packets").show(ctx, |ui| {
        ui.heading("Captured Packets");
        let packets = CAPTURED_PACKETS.lock().unwrap();
        log_info(&format!("Number of captured packets: {}", packets.len()));
        for packet in packets.iter() {
            ui.label(packet);
            log_info(&format!("Rendering packet: {}", packet));
        }
    });
    log_info("Rendered packets.");
}

// Function to render the packet capture menu
pub fn render_pc_menu(ctx: &eframe::egui::Context, shared_state: &SharedAppState, state: &mut MenuState, log_buffer: Arc<Mutex<Vec<String>>>) {
    info!("Rendering packet capture menu...");
    log_info("Rendering packet capture menu...");

    if shared_state.get_state() != AppState::PacketCaptureMenu {
        return;
    }

    let title = "S2O's s2o_net_lib Crate";
    let menu_items = vec![
        MenuItem {
            label: "Start Capture".to_string(),
            action: Some(Box::new({
                let capturing: Arc<AtomicBool> = Arc::clone(&CAPTURING);
                let ctx = ctx.clone();
                move || {
                    if !capturing.load(Ordering::SeqCst) {
                        debug!("Starting packet capture...");
                        log_info("Starting packet capture...");
                        match packet_sniffer::start_packet_sniffer() {
                            Ok(0) => {
                                log_info("Sniffer started successfully.");
                                capturing.store(true, Ordering::SeqCst);
                                log_info("Spawning packet capture thread...");
                                spawn_capture_thread(capturing.clone(), ctx.clone());
                            },
                            Ok(err) => {
                                error!("Failed to start sniffer with error code: {}", err);
                                log_info(&format!("Failed to start sniffer with error code: {}", err));
                            },
                            Err(e) => {
                                error!("Failed to access start_sniffer function: {:?}", e);
                                log_info(&format!("Failed to access start_sniffer function: {:?}", e));
                            }
                        }
                    } else {
                        error!("Capture already running.");
                        log_info("Capture already running.");
                    }
                }
            })),
        },
        MenuItem {
            label: "Stop Capture".to_string(),
            action: Some(Box::new({
                let capturing: Arc<AtomicBool> = Arc::clone(&CAPTURING);
                move || {
                    if capturing.load(Ordering::SeqCst) {
                        debug!("Stopping packet capture...");
                        log_info("Stopping packet capture...");
                        if let Err(e) = packet_sniffer::stop_packet_sniffer() {
                            error!("Failed to access stop_sniffer function: {:?}", e);
                            log_info(&format!("Failed to access stop_sniffer function: {:?}", e));
                        }
                        capturing.store(false, Ordering::SeqCst);
                        info!("Capture stopped successfully.");
                        log_info("Capture stopped successfully.");
                    } else {
                        error!("No capture to stop.");
                        log_info("No capture to stop.");
                    }
                }
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new({
                let shared_state = shared_state.clone();
                move || {
                    debug!("Exiting packet capture menu...");
                    log_info("Exiting packet capture menu...");
                    shared_state.set_state(AppState::ProgramMenu);
                    log_info("Transitioned to Program Menu");
                }
            })),
        },
    ];

    debug!("Rendering the main menu with title: {}", title);
    log_info(&format!("Rendering the main menu with title: {}", title));

    // Render the main menu
    CentralPanel::default().show(ctx, |_ui| {
        debug!("Calling render_menu...");
        log_info("Calling render_menu...");
        render_menu(ctx, title, &menu_items[..], state, log_buffer);
        debug!("render_menu completed.");
        log_info("render_menu completed.");
    });

    // Render captured packets
    debug!("Rendering captured packets...");
    log_info("Rendering captured packets...");
    render_packets(ctx);
    debug!("Rendering captured packets completed.");
    log_info("Rendering captured packets completed.");
}

// Function to spawn a thread for packet capture
// Function to spawn a thread for packet capture
fn spawn_capture_thread(capturing: Arc<AtomicBool>, ctx: eframe::egui::Context) {
    log_info("Inside spawn_capture_thread function...");
    std::thread::spawn(move || {
        while capturing.load(Ordering::SeqCst) {
            log_info("Attempting to capture packet...");
            if let Err(e) = packet_sniffer::capture_packet_data() {
                error!("Failed to capture packet: {:?}", e);
                log_info(&format!("Failed to capture packet: {:?}", e));
                break;
            }

            log_info("Calling GET_PACKET_COUNT function...");
            let count = match packet_sniffer::get_captured_packet_count() {
                Ok(count) => count,
                Err(e) => {
                    error!("Failed to get packet count: {:?}", e);
                    log_info(&format!("Failed to get packet count: {:?}", e));
                    break;
                }
            };
            log_info(&format!("Packet count: {}", count));
            let mut packets = CAPTURED_PACKETS.lock().unwrap();
            for i in 0..count {
                log_info(&format!("Getting packet at index {}", i));
                match packet_sniffer::get_captured_packet(i) {
                    Ok(Some(packet)) => {
                        let packet_data = packet_sniffer::human_readable_packet_data(&packet);
                        log_info(&format!("Captured packet:\n{}", &packet_data));
                        packets.push(packet_data.clone()); // Clone the string before pushing
                        info!("Captured packet:\n{}", packet_data);
                        log_info(&format!("Added packet to CAPTURED_PACKETS:\n{}", packet_data));
                    },
                    Ok(None) => {
                        error!("Failed to get packet: pointer is null at index {}", i);
                        log_info(&format!("Failed to get packet: pointer is null at index {}", i));
                    },
                    Err(e) => {
                        error!("Failed to access get_packet function: {:?}", e);
                        log_info(&format!("Failed to access get_packet function: {:?}", e));
                        break;
                    }
                }
            }
            drop(packets); // Explicitly drop lock to ensure it is released
            ctx.request_repaint(); // Request UI repaint to update packet list
            log_info("Sleeping for 500 ms...");
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        log_info("Exiting packet capture thread...");
        ctx.request_repaint(); // Ensure UI repaint after capture thread ends
    });
}