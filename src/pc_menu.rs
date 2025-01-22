use crate::menu_rendering::{MenuItem, MenuState, render_menu};
use crate::app_state::{SharedAppState, AppState}; 
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use log::{info, error, debug};
use eframe::egui::{Context, CentralPanel, SidePanel, ScrollArea};
use crate::logging::{LogBuffers, log_info, log_error};
use once_cell::sync::Lazy;
use crate::packet_sniffer;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

// Shared static variables for capturing packets
static CAPTURED_PACKETS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));
static CAPTURING: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));
static PROCESS_LOGS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new()))); // Logs for process steps
static SETUP_LOGGED: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false))); // Flag to log setup steps only once
static LOGGED_STEPS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new())); // Flag to ensure logs are entered once

// Helper function to add logs to PROCESS_LOGS
fn log_process_step(log_buffers: &LogBuffers, step: &str) {
    let mut logs = PROCESS_LOGS.lock().expect("Failed to lock process logs");
    let mut logged_steps = LOGGED_STEPS.lock().expect("Failed to lock logged steps");
    if !logged_steps.contains(step) {
        logs.push(step.to_string());
        logged_steps.insert(step.to_string());
        log_info(log_buffers, step);
    }
}

// Function to run the packet capture UI
pub fn run_ui(ctx: &Context, shared_state: &SharedAppState, state: &mut MenuState, log_buffers: &LogBuffers) {
    if shared_state.get_state() == AppState::PacketCaptureMenu {
        render_pc_menu(ctx, shared_state, state, log_buffers);
    }
}

// Function to render process logs
fn render_process_logs(ctx: &Context) {
    SidePanel::right("process_logs").show(ctx, |ui| {
        ui.heading("Process Logs");
        ScrollArea::vertical().show(ui, |ui| {
            let logs = PROCESS_LOGS.lock().expect("Failed to lock process logs");
            for log in logs.iter().rev().take(20) { // Show last 20 logs
                ui.label(log);
            }
        });
    });
}

// Function to render captured packets
fn render_packets(ctx: &Context) {
    SidePanel::right("captured_packets").show(ctx, |ui| {
        ui.heading("Captured Packets");
        ScrollArea::vertical().show(ui, |ui| {
            let packets = CAPTURED_PACKETS.lock().expect("Failed to lock captured packets");
            for packet in packets.iter().rev().take(20) { // Show last 20 packets
                ui.label(packet);
            }
        });
    });
}

static RENDER_LOGGED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

pub fn render_pc_menu(ctx: &Context, shared_state: &SharedAppState, state: &mut MenuState, log_buffers: &LogBuffers) {
    // Log the setup steps only once
    if !SETUP_LOGGED.load(Ordering::SeqCst) {
        let setup_steps = [
            "DLL found...", 
            "Aligning pc_menu...", 
            "Aligning packet sniffer...", 
            "Aligning DLL...", 
            "Prepping packet capture...", 
            "Packet capture prepped. Select Start Capture."
        ];
        for step in setup_steps.iter() {
            log_process_step(log_buffers, step);
        }
        SETUP_LOGGED.store(true, Ordering::SeqCst);
    }

    // Ensure rendering log is only entered once per cycle
    if !RENDER_LOGGED.load(Ordering::SeqCst) {
        info!("Rendering packet capture menu...");
        log_process_step(log_buffers, "Rendering packet capture menu...");
        RENDER_LOGGED.store(true, Ordering::SeqCst);
    }

    let title = "S2O's s2o_net_lib Crate";
    let menu_items = vec![
        MenuItem {
            label: "Start Capture".to_string(),
            action: Some(Box::new({
                let capturing = Arc::clone(&CAPTURING);
                let log_buffers = log_buffers.clone();
                let ctx = ctx.clone();
                move || {
                    if !capturing.load(Ordering::SeqCst) {
                        debug!("Starting packet capture...");
                        log_info(&log_buffers, "Starting packet capture...");
                        log_process_step(&log_buffers, "Starting packet capture...");
                        match packet_sniffer::start_packet_sniffer(&log_buffers) {
                            Ok(()) => {
                                log_info(&log_buffers, "Sniffer started successfully.");
                                log_process_step(&log_buffers, "Sniffer started successfully.");
                                capturing.store(true, Ordering::SeqCst);
                                log_info(&log_buffers, "Spawning packet capture thread...");
                                spawn_capture_thread(capturing.clone(), ctx.clone(), log_buffers.clone());
                            },
                            Err(e) => {
                                error!("Failed to start sniffer: {:?}", e);
                                log_error(&log_buffers, &format!("Failed to start sniffer: {:?}", e));
                                log_process_step(&log_buffers, &format!("Failed to start sniffer: {:?}", e));
                            }
                        }
                    } else {
                        error!("Capture already running.");
                        log_info(&log_buffers, "Capture already running.");
                        log_process_step(&log_buffers, "Capture already running.");
                    }
                }
            })),
        },
        MenuItem {
            label: "Stop Capture".to_string(),
            action: Some(Box::new({
                let capturing = Arc::clone(&CAPTURING);
                let log_buffers = log_buffers.clone();
                move || {
                    if capturing.load(Ordering::SeqCst) {
                        debug!("Stopping packet capture...");
                        log_info(&log_buffers, "Stopping packet capture...");
                        log_process_step(&log_buffers, "Stopping packet capture...");
                        if let Err(e) = packet_sniffer::stop_packet_sniffer(&log_buffers) {
                            error!("Failed to stop sniffer: {:?}", e);
                            log_error(&log_buffers, &format!("Failed to stop sniffer: {:?}", e));
                            log_process_step(&log_buffers, &format!("Failed to stop sniffer: {:?}", e));
                        }
                        capturing.store(false, Ordering::SeqCst);
                        info!("Capture stopped successfully.");
                        log_info(&log_buffers, "Capture stopped successfully.");
                        log_process_step(&log_buffers, "Capture stopped successfully.");
                    } else {
                        error!("No capture to stop.");
                        log_info(&log_buffers, "No capture to stop.");
                        log_process_step(&log_buffers, "No capture to stop.");
                    }
                }
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new({
                let shared_state = shared_state.clone();
                let log_buffers = log_buffers.clone();
                move || {
                    debug!("Exiting packet capture menu...");
                    log_info(&log_buffers, "Exiting packet capture menu...");
                    log_process_step(&log_buffers, "Exiting packet capture menu.");
                    shared_state.set_state(AppState::ProgramMenu);
                    log_info(&log_buffers, "Transitioned to Program Menu");
                    log_process_step(&log_buffers, "Transitioned to Program Menu.");
                }
            })),
        },
    ];

    debug!("Rendering the main menu with title: {}", title);
    log_process_step(log_buffers, &format!("Rendering the main menu with title: {}", title));

    // Render the main menu
    CentralPanel::default().show(ctx, |_ui| {
        debug!("Calling render_menu...");
        log_process_step(log_buffers, "Calling render_menu...");
        if let Err(e) = render_menu(ctx, title, &menu_items, state, log_buffers, log_buffers.info_buffer.clone()) {
            error!("Failed to render menu: {:?}", e);
            log_error(&log_buffers, &format!("Failed to render menu: {:?}", e));
            log_process_step(&log_buffers, &format!("Failed to render menu: {:?}", e));
        }
        debug!("render_menu completed.");
        log_process_step(log_buffers, "render_menu completed.");
    });

    // Render process logs
    debug!("Rendering process logs...");
    log_process_step(log_buffers, "Rendering process logs...");
    render_process_logs(ctx);
    debug!("Rendering process logs completed.");
    log_process_step(log_buffers, "Rendering process logs completed.");

    // Render captured packets
    debug!("Rendering captured packets...");
    log_process_step(log_buffers, "Rendering captured packets...");
    render_packets(ctx);
    debug!("Rendering captured packets completed.");
    log_process_step(log_buffers, "Rendering captured packets completed.");
}

fn spawn_capture_thread(capturing: Arc<AtomicBool>, ctx: Context, log_buffers: LogBuffers) {
    log_process_step(&log_buffers, "Inside spawn_capture_thread function...");

    thread::spawn(move || {
        while capturing.load(Ordering::SeqCst) {
            log_process_step(&log_buffers, "Attempting to capture packet...");
            if let Err(e) = packet_sniffer::capture_packet_data(&log_buffers) {
                error!("Failed to capture packet: {:?}", e);
                log_error(&log_buffers, &format!("Failed to capture packet: {:?}", e));
                log_process_step(&log_buffers, &format!("Failed to capture packet: {:?}", e));
                break;
            }

            log_info(&log_buffers, "Calling GET_PACKET_COUNT function...");
            log_process_step(&log_buffers, "Calling GET_PACKET_COUNT function...");
            let count = match packet_sniffer::get_captured_packet_count(&log_buffers) {
                Ok(count) => count,
                Err(e) => {
                    error!("Failed to get packet count: {:?}", e);
                    log_error(&log_buffers, &format!("Failed to get packet count: {:?}", e));
                    log_process_step(&log_buffers, &format!("Failed to get packet count: {:?}", e));
                    break;
                }
            };

            log_info(&log_buffers, &format!("Packet count: {}", count));
            log_process_step(&log_buffers, &format!("Packet count: {}", count));
            let mut packets = CAPTURED_PACKETS.lock().expect("Failed to lock captured packets for writing");
            for i in 0..count {
                log_info(&log_buffers, &format!("Getting packet at index {}", i));
                log_process_step(&log_buffers, &format!("Getting packet at index {}", i));
                match packet_sniffer::get_captured_packet(i, &log_buffers) {
                    Ok(Some(packet)) => {
                        let packet_data = packet_sniffer::human_readable_packet_data(&packet);
                        log_info(&log_buffers, &format!("Captured packet:\n{}", &packet_data));
                        packets.push(packet_data.clone()); // Clone the string before pushing
                        info!("Captured packet:\n{}", packet_data);
                        log_process_step(&log_buffers, &format!("Captured packet:\n{}", packet_data));
                    },
                    Ok(None) => {
                        error!("Failed to get packet: pointer is null at index {}", i);
                        log_error(&log_buffers, &format!("Failed to get packet: pointer is null at index {}", i));
                        log_process_step(&log_buffers, &format!("Failed to get packet: pointer is null at index {}", i));
                    },
                    Err(e) => {
                        error!("Failed to access get_packet function: {:?}", e);
                        log_error(&log_buffers, &format!("Failed to access get_packet function: {:?}", e));
                        log_process_step(&log_buffers, &format!("Failed to access get_packet function: {:?}", e));
                        break;
                    }
                }
            }
            drop(packets); // Explicitly drop lock to ensure it is released
            ctx.request_repaint(); // Request UI repaint to update packet list
            log_info(&log_buffers, "Sleeping for 500 ms...");
            log_process_step(&log_buffers, "Sleeping for 500 ms...");
            thread::sleep(Duration::from_millis(500));
        }

        log_info(&log_buffers, "Exiting packet capture thread...");
        log_process_step(&log_buffers, "Exiting packet capture thread...");
        ctx.request_repaint(); // Ensure UI repaint after capture thread ends
    });
}
