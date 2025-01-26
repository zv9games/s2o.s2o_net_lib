use crate::gui_engine::{MenuItem, MenuState, render_menu};
use crate::app_state::{SharedAppState, AppState};
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use log::{info, error, debug};
use eframe::egui::{Context, CentralPanel};
use crate::logging::{LogBuffers, log_info, log_error};
use once_cell::sync::Lazy;
use crate::packet_sniffer;
use crate::packet_capture::{CAPTURING, spawn_capture_thread, render_packets};
use crate::process_log::{log_process_step, render_process_logs};
use std::env;
use std::path::Path;
use libloading::Library;
use egui::Window;
use crate::packet_capture::CAPTURED_PACKETS;

// Shared static variables
static SETUP_LOGGED: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false))); // Flag to log setup steps only once
static RENDER_LOGGED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));




use std::fs;

fn check_dll_loaded() -> Result<(), String> {
    // Path to the DLL
    let dll_path = "C:/S2O/s2o_net_lib/src/s2o_dll/packet_sniffer.dll";

    // Log the DLL path
    debug!("DLL path: {}", dll_path);

    // Update the environment path to include the DLL directory
    if let Some(dll_dir) = Path::new(dll_path).parent() {
        let current_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{};{}", current_path, dll_dir.to_string_lossy());
        env::set_var("PATH", new_path.clone());  // Clone the new_path before moving
        debug!("Updated PATH: {}", new_path);  // Log the updated PATH

        // List the contents of the directory
        if let Ok(entries) = fs::read_dir(dll_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    debug!("Directory entry: {}", entry.path().display());
                }
            }
        } else {
            debug!("Failed to read directory: {}", dll_dir.display());
        }
    } else {
        let error_message = format!("Failed to get parent directory for path: {}", dll_path);
        debug!("{}", error_message);
        return Err(error_message);
    }

    // Log the current directory
    if let Ok(current_dir) = env::current_dir() {
        debug!("Current directory: {}", current_dir.display());
    }

    // Check if DLL exists at the specified path
    if !Path::new(dll_path).exists() {
        let error_message = format!("DLL not found at path: {}", dll_path);
        debug!("{}", error_message);
        return Err(error_message);
    } else {
        debug!("DLL found at path: {}", dll_path);
    }

    // Use libloading to attempt to load the DLL and log any errors
    match unsafe { Library::new(dll_path) } {
        Ok(_) => debug!("DLL loaded successfully."),
        Err(e) => {
            // Capture and log specific error details
            let error_message = format!("Failed to load DLL: {:?}", e);
            debug!("{}", error_message);
            return Err(error_message);
        },
    }

    Ok(())
}

// Shared static variable for showing packet popup
static SHOW_PACKET_POPUP: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));


pub fn render_pc_menu(ctx: &eframe::egui::Context, shared_state: &SharedAppState, state: &mut MenuState, log_buffers: &LogBuffers) {
    // Log the setup steps only once
    if !SETUP_LOGGED.load(Ordering::SeqCst) {
        // Check if DLL is loaded
        if let Err(e) = check_dll_loaded() {
            log_process_step("DLL load failed.");
            log_error(log_buffers, &format!("DLL load failed: {:?}", e), true); // Restrict repeated log
            error!("DLL load failed: {:?}", e);

            // Display error message in the UI with detailed error info
            CentralPanel::default().show(ctx, |ui| {
                ui.heading("Error");
                ui.label(&format!("Failed to load DLL: {:?}", e));
            });

            ctx.request_repaint();
            return;
        } else {
            log_process_step("DLL found...");
        }

        let setup_steps = [
            "Aligning pc_menu...",
            "Aligning packet sniffer...",
            "Aligning DLL...",
            "Prepping packet capture...",
            "Packet capture prepped. Select Start Capture."
        ];

        for step in setup_steps.iter() {
            log_process_step(step);
        }

        SETUP_LOGGED.store(true, Ordering::SeqCst);
    }

    // Ensure rendering log is only entered once per cycle
    if !RENDER_LOGGED.load(Ordering::SeqCst) {
        info!("Rendering packet capture menu...");
        log_process_step("Rendering packet capture menu...");
        RENDER_LOGGED.store(true, Ordering::SeqCst);
    }

    let title = "S2O's s2o_net_lib Crate";
    let menu_items = vec![
        MenuItem {
            label: "Start Capture".to_string(),
            action: Some(Box::new({
                let capturing: Arc<AtomicBool> = Arc::clone(&CAPTURING);
                let log_buffers = log_buffers.clone();
                let ctx = ctx.clone();
                move || {
                    if (!capturing.load(Ordering::SeqCst)) {
                        debug!("Starting packet capture...");
                        log_info(&log_buffers, "Starting packet capture...", false); // Allow repeated log
                        log_process_step("Starting packet capture...");
                        match packet_sniffer::start_packet_sniffer(&log_buffers) {
                            Ok(()) => {
                                log_info(&log_buffers, "Sniffer started successfully.", false); // Allow repeated log
                                log_process_step("Sniffer started successfully.");
                                capturing.store(true, Ordering::SeqCst);
                                log_info(&log_buffers, "Spawning packet capture thread...", false); // Allow repeated log
                                spawn_capture_thread(capturing.clone(), ctx.clone(), log_buffers.clone());
                                SHOW_PACKET_POPUP.store(true, Ordering::SeqCst); // Show the popup once sniffer starts successfully
                            },
                            Err(e) => {
                                error!("Failed to start sniffer: {:?}", e);
                                log_error(&log_buffers, &format!("Failed to start sniffer: {:?}", e), true); // Restrict repeated log
                                log_process_step(&format!("Failed to start sniffer: {:?}", e));
                            }
                        }
                    } else {
                        error!("Capture already running.");
                        log_info(&log_buffers, "Capture already running.", false); // Allow repeated log
                        log_process_step("Capture already running.");
                    }
                }
            })),
        },
        MenuItem {
            label: "Stop Capture".to_string(),
            action: Some(Box::new({
                let capturing: Arc<AtomicBool> = Arc::clone(&CAPTURING);
                let log_buffers = log_buffers.clone();
                move || {
                    if capturing.load(Ordering::SeqCst) {
                        debug!("Stopping packet capture...");
                        log_info(&log_buffers, "Stopping packet capture...", false); // Allow repeated log
                        log_process_step("Stopping packet capture...");
                        if let Err(e) = packet_sniffer::stop_packet_sniffer(&log_buffers) {
                            error!("Failed to stop sniffer: {:?}", e);
                            log_error(&log_buffers, &format!("Failed to stop sniffer: {:?}", e), true); // Restrict repeated log
                            log_process_step(&format!("Failed to stop sniffer: {:?}", e));
                        }
                        capturing.store(false, Ordering::SeqCst);
                        SHOW_PACKET_POPUP.store(false, Ordering::SeqCst); // Reset popup flag
                        info!("Capture stopped successfully.");
                        log_info(&log_buffers, "Capture stopped successfully.", false); // Allow repeated log
                        log_process_step("Capture stopped successfully.");
                    } else {
                        error!("No capture to stop.");
                        log_info(&log_buffers, "No capture to stop.", false); // Allow repeated log
                        log_process_step("No capture to stop.");
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
                    log_info(&log_buffers, "Exiting packet capture menu...", false); // Allow repeated log
                    log_process_step("Exiting packet capture menu.");
                    shared_state.set_state(AppState::ProgramMenu);
                    log_info(&log_buffers, "Transitioned to Program Menu", false); // Allow repeated log
                    log_process_step("Transitioned to Program Menu.");
                }
            })),
        },
    ];

    debug!("Rendering the main menu with title: {}", title);
    log_process_step(&format!("Rendering the main menu with title: {}", title));

    // Define the is_admin flag based on your logic
    let is_admin = true; // Set this according to your logic

    // Render the main menu
    CentralPanel::default().show(ctx, |_ui| {
        debug!("Calling render_menu...");
        log_process_step("Calling render_menu...");
        if let Err(e) = render_menu(ctx, title, &menu_items, state, log_buffers, log_buffers.info_buffer.clone(), is_admin) {
            error!("Failed to render menu: {:?}", e);
            log_error(&log_buffers, &format!("Failed to render menu: {:?}", e), true); // Restrict repeated log
            log_process_step(&format!("Failed to render menu: {:?}", e));
        }
        debug!("render_menu completed.");
        log_process_step("render_menu completed.");
    });

    // Render process logs
    debug!("Rendering process logs...");
    log_process_step("Rendering process logs...");
    render_process_logs(ctx);
    debug!("Rendering process logs completed.");
    log_process_step("Rendering process logs completed.");

    // Render captured packets
    debug!("Rendering captured packets...");
    log_process_step("Rendering captured packets...");
    render_packets(ctx);
    debug!("Rendering captured packets completed.");
    log_process_step("Rendering captured packets completed.");

    // Show popup with packet data if flag is set
    if SHOW_PACKET_POPUP.load(Ordering::SeqCst) {
        Window::new("Packet Data")
            .collapsible(false)
            .resizable(true)
            .show(ctx, |ui| {
                let packets = CAPTURED_PACKETS.lock().expect("Failed to lock captured packets");
                for packet in packets.iter().take(1) { // Show the first captured packet
                    ui.label(packet);
                }
            });
    }
}

// Define the run_ui function here
pub fn run_ui(ctx: &Context, shared_state: &SharedAppState, state: &mut MenuState, log_buffers: &LogBuffers) {
    if shared_state.get_state() == AppState::PacketCaptureMenu {
        render_pc_menu(ctx, shared_state, state, log_buffers);
    }
}
