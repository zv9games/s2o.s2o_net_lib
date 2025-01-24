use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use log::{info, error};
use eframe::egui::{Context, SidePanel, ScrollArea};
use crate::logging::{LogBuffers, log_info, log_error};
use once_cell::sync::Lazy;
use crate::packet_sniffer;
use std::thread;
use std::time::Duration;
use crate::s_menu::log_process_step;

// Shared static variables for capturing packets
static CAPTURED_PACKETS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new()))); // Logs for captured packets
pub(crate) static CAPTURING: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false))); // Flag for capturing state

// Function to render captured packets
pub fn render_packets(ctx: &Context) {
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

// Function to spawn a thread for capturing packets
pub fn spawn_capture_thread(capturing: Arc<AtomicBool>, ctx: Context, log_buffers: LogBuffers) {
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
