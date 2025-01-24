use crate::menu_rendering::{MenuItem, MenuContext, render_menu, default_style, has_admin_rights};
use crate::app_state::SharedAppState;
use crate::logging::{LogBuffers, log_info};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::io::Error as IoError;

static LOGGED_ACTIONS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn log_action(log_buffers: &LogBuffers, action: &str) {
    let mut actions = LOGGED_ACTIONS.lock().expect("Failed to lock LOGGED_ACTIONS");
    if !actions.contains(action) {
        actions.insert(action.to_string());
        log_info(log_buffers, action);
    }
}

pub fn security_menu(log_buffers: &LogBuffers, _shared_state: &SharedAppState) {
    log_action(log_buffers, "Entering security interface.");

    let menu_items = vec![
        MenuItem {
            label: "Admin Access".to_string(),
            action: Some(Box::new({
                let log_buffers = log_buffers.clone();
                move || initiate_admin_access(&log_buffers)
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new({
                let log_buffers = log_buffers.clone();
                move || {
                    log_action(&log_buffers, "Exiting from security interface.");
                    std::process::exit(0);
                }
            })),
        },
    ];

    log_action(log_buffers, "Menu options configured.");

    let state = MenuContext::new(default_style(), has_admin_rights());
    log_action(log_buffers, "Menu context initialized.");

    let log_buffer = log_buffers.info_buffer.clone();

    log_action(log_buffers, "Launching security application.");
    match eframe::run_native(
        "Security Interface",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(SecurityApp {
                state,
                menu_items,
                log_buffer,
                log_buffers: log_buffers.clone(),
            }))
        }),
    ) {
        Err(e) => {
            log_action(log_buffers, &format!("Failed to start security app: {}", e));
            std::process::exit(1);
        },
        _ => log_action(log_buffers, "Security application running."),
    }
}

struct SecurityApp {
    state: MenuContext,
    menu_items: Vec<MenuItem>,
    log_buffer: Arc<Mutex<Vec<String>>>,
    log_buffers: LogBuffers,
}

impl eframe::App for SecurityApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        log_action(&self.log_buffers, "Refreshing security interface.");
        if let Err(e) = render_menu(ctx, "Security Interface", &self.menu_items, &mut self.state, &self.log_buffers, self.log_buffer.clone()) {
            log_action(&self.log_buffers, &format!("Error rendering menu: {}", e));
        }
        log_action(&self.log_buffers, "Security interface refreshed.");
    }
}

fn initiate_admin_access(log_buffers: &LogBuffers) {
    log_action(log_buffers, "Attempting to gain admin access.");

    let result = std::process::Command::new("powershell")
        .arg("-File")
        .arg("uac.ps1")
        .arg(format!("{}", std::env::current_exe().unwrap().display()))
        .arg("--admin")
        .output();

    match result {
        Ok(output) => {
            let stdout = std::str::from_utf8(&output.stdout).unwrap_or("<non-utf8 data>");
            let stderr = std::str::from_utf8(&output.stderr).unwrap_or("<non-utf8 data>");
            log_action(log_buffers, &format!("Admin script ran. Output: stdout: {}, stderr: {}", stdout, stderr));
        },
        Err(e) => {
            log_action(log_buffers, &format!("Admin access initiation failed: {}", e));
        }
    }
}

pub fn initialize_security_menu(log_buffers: &LogBuffers) -> Result<(), IoError> {
    log_info(log_buffers, "Initializing security interface...");
    // Placeholder for further initialization if needed
    Ok(())
}