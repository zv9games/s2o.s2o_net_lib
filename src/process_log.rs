use std::sync::{Arc, Mutex};
use log::info;
use once_cell::sync::Lazy;
use std::io::Error;
use crate::logging::{LogBuffers, log_info};

static PROCESS_LOGS: Lazy<Arc<Mutex<Vec<String>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new()))); // Logs for process steps
static LOGGED_STEPS: Lazy<Mutex<std::collections::HashSet<String>>> = Lazy::new(|| Mutex::new(std::collections::HashSet::new())); // Flag to ensure logs are entered once

pub fn init_process(log_buffers: &Arc<LogBuffers>) -> Result<(), Error> {
    log_info(log_buffers, "Initializing process_log.", false);
    
    Ok(())
}

// Helper function to add logs to PROCESS_LOGS
pub fn log_process_step(step: &str) {
    let mut logs = PROCESS_LOGS.lock().expect("Failed to lock process logs");
    let mut logged_steps = LOGGED_STEPS.lock().expect("Failed to lock logged steps");
    if !logged_steps.contains(step) {
        logs.push(step.to_string());
        logged_steps.insert(step.to_string());
        info!("{}", step);
    }
}

// Function to render process logs
pub fn render_process_logs(ctx: &eframe::egui::Context) {
    eframe::egui::SidePanel::right("process_logs").show(ctx, |ui| {
        ui.heading("Process Logs");
        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
            let logs = PROCESS_LOGS.lock().expect("Failed to lock process logs");
            for log in logs.iter().rev().take(20) { // Show last 20 logs
                ui.label(log);
            }
        });
    });
}
