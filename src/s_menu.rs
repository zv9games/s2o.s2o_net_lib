use crate::gui_engine::{MenuItem, MenuState, render_menu, default_settings};
use crate::app_state::SharedAppState;
use std::sync::{Arc, Mutex};
use crate::logging::{LogBuffers, log_info};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::io::Error as IoError; // Import IoError for error conversion

// Shared static variable for logging steps
static LOGGED_STEPS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

pub fn log_process_step(log_buffers: &LogBuffers, step: &str) {
    let mut logged_steps = LOGGED_STEPS.lock().expect("Failed to lock LOGGED_STEPS");
    if !logged_steps.contains(step) {
        logged_steps.insert(step.to_string());
        log_info(log_buffers, step, false); // Allow repeated log
    }
}

pub fn security_menu_loop(log_buffers: &LogBuffers, _shared_state: &SharedAppState) {
    log_process_step(log_buffers, "Security menu loop started.");

    let menu_items = vec![
        MenuItem {
            label: "Admin Menu".to_string(),
            action: Some(Box::new({
                let log_buffers = log_buffers.clone();
                move || run_admin_menu(&log_buffers)
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new({
                let log_buffers = log_buffers.clone();
                move || {
                    log_process_step(&log_buffers, "Exiting application from security menu.");
                    std::process::exit(0);
                }
            })),
        },
    ];

    log_process_step(log_buffers, "Menu items initialized.");

    let state = MenuState::new(default_settings());
    log_process_step(log_buffers, "Menu state initialized with default settings.");

    let log_buffer = log_buffers.info_buffer.clone();  // Use the existing log buffer

    log_process_step(log_buffers, "Starting native application for security menu.");
    if let Err(e) = eframe::run_native(
        "Security Menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(SecurityMenuApp {
                state,
                menu_items,
                log_buffer,
                log_buffers: log_buffers.clone(),
                is_admin: false, // Default to false, can be set to true if needed
            }))
        }),
    ) {
        log_process_step(log_buffers, &format!("Failed to run security menu: {}", e));
        std::process::exit(1);
    }
    log_process_step(log_buffers, "Security menu running.");
}

struct SecurityMenuApp {
    state: MenuState,
    menu_items: Vec<MenuItem>,
    log_buffer: Arc<Mutex<Vec<String>>>,
    log_buffers: LogBuffers,
    is_admin: bool, // Add is_admin field
}

impl eframe::App for SecurityMenuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        log_process_step(&self.log_buffers, "Update method called for security menu.");
        if let Err(e) = render_menu(ctx, "Security Menu", &self.menu_items, &mut self.state, &self.log_buffers, self.log_buffer.clone(), self.is_admin) {
            log_process_step(&self.log_buffers, &format!("Failed to render menu: {}", e));
        }
        log_process_step(&self.log_buffers, "Security menu rendered.");
    }
}

fn run_admin_menu(log_buffers: &LogBuffers) {
    log_process_step(log_buffers, "Admin Menu action triggered. Requesting elevated permissions.");
    
    // Call PowerShell script to request elevated permissions
    let output = std::process::Command::new("powershell")
        .arg("-File")
        .arg("uac.ps1")
        .arg(format!("{}", std::env::current_exe().unwrap().display()))
        .arg("--admin")
        .output();

    match output {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or("<non-utf8 data>");
            let stderr = std::str::from_utf8(&output.stderr).unwrap_or("<non-utf8 data>");
            log_process_step(log_buffers, &format!("Admin Menu script executed successfully. Output: stdout: {}, stderr: {}", stdout, stderr));
        },
        Err(e) => {
            log_process_step(log_buffers, &format!("Failed to execute admin menu script: {}", e));
        }
    }
}

pub fn init_s_menu(log_buffers: &LogBuffers) -> Result<(), IoError> {
    log_info(log_buffers, "Initializing security menu...", false); // Allow repeated log
    // Implement security menu initialization here
    Ok(())
}
