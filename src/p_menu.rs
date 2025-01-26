use crate::gui_engine::{MenuItem, MenuState, render_menu, default_settings};
use crate::app_state::{SharedAppState, AppState};
use crate::logging::{LogBuffers, log_info};
use crate::pc_menu::run_ui;
use crate::ns_menu::render_ns_menu;
use crate::ds_menu::render_ds_menu;
use winapi::um::winnt::TOKEN_QUERY;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashSet;

// Shared static variable for logging steps
static LOGGED_STEPS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

fn log_process_step(log_buffers: &LogBuffers, step: &str) {
    let mut logged_steps = LOGGED_STEPS.lock().expect("Failed to lock LOGGED_STEPS");
    if !logged_steps.contains(step) {
        logged_steps.insert(step.to_string());
        log_info(log_buffers, step, false); // Allow repeated log
    }
}

pub fn program_menu_loop(shared_state: &SharedAppState, log_buffers: &LogBuffers) {
    log_process_step(log_buffers, "Program menu loop started.");

    // Verify administrative privileges
    if !is_running_as_admin() {
        log_process_step(log_buffers, "Administrative privileges not granted. Redirecting to security menu.");
        shared_state.set_state(AppState::SecurityMenu);
        return;
    }

    let state = MenuState::new(default_settings());
    log_process_step(log_buffers, "Menu state initialized with default settings.");

    if let Err(e) = eframe::run_native(
        "Program Menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ProgramMenuApp {
            state,
            shared_state: shared_state.clone(),
            log_buffers: log_buffers.clone(),
        }))),
    ) {
        log_process_step(log_buffers, &format!("Error running native program menu: {}", e));
        std::process::exit(1);
    }
    log_process_step(log_buffers, "Native program menu running.");
}

struct ProgramMenuApp {
    state: MenuState,
    shared_state: SharedAppState,
    log_buffers: LogBuffers,
}

impl eframe::App for ProgramMenuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        log_process_step(&self.log_buffers, "Update method called.");

        let current_app_state = self.shared_state.get_state();
        log_process_step(&self.log_buffers, &format!("Current app state: {:?}", current_app_state));

        match current_app_state {
            AppState::ProgramMenu => {
                log_process_step(&self.log_buffers, "Rendering program menu.");
                // Add the missing is_admin argument here
                let is_admin = true; // Set this based on your logic
                if let Err(e) = render_menu(ctx, "S2O's s2o_net_lib Crate", &self.menu_items(), &mut self.state, &self.log_buffers, self.log_buffers.info_buffer.clone(), is_admin) {
                    log_process_step(&self.log_buffers, &format!("Failed to render menu: {}", e));
                    return;
                }

                log_process_step(&self.log_buffers, "Checking for state transitions.");
                if ctx.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                    match self.state.selected {
                        0 => {
                            self.state.selected = 0;
                            log_process_step(&self.log_buffers, "Transitioning to NSMenu.");
                            self.shared_state.set_state(AppState::NSMenu);
                        },
                        1 => {
                            self.state.selected = 0;
                            log_process_step(&self.log_buffers, "Transitioning to DSMenu.");
                            self.shared_state.set_state(AppState::DSMenu);
                        },
                        2 => {
                            self.state.selected = 0;
                            log_process_step(&self.log_buffers, "Transitioning to PacketCaptureMenu.");
                            self.shared_state.set_state(AppState::PacketCaptureMenu);
                        },
                        _ => {} // Handle other cases or do nothing
                    }
                }
            },
            AppState::NSMenu => {
                log_process_step(&self.log_buffers, "Rendering Network Settings Menu.");
                render_ns_menu(ctx, &mut self.state, &self.shared_state, &self.log_buffers, self.log_buffers.info_buffer.clone());
            },
            AppState::DSMenu => {
                log_process_step(&self.log_buffers, "Rendering Data Speed Menu.");
                render_ds_menu(ctx, &mut self.state, &self.shared_state, &self.log_buffers, self.log_buffers.info_buffer.clone());
            },
            AppState::PacketCaptureMenu => {
                log_process_step(&self.log_buffers, "Rendering Packet Capture Menu.");
                run_ui(ctx, &self.shared_state, &mut self.state, &self.log_buffers);
            },
            _ => {
                log_process_step(&self.log_buffers, "Unknown app state encountered.");
            }
        }
    }
}

impl ProgramMenuApp {
    fn menu_items(&self) -> Vec<MenuItem> {
        log_process_step(&self.log_buffers, "Creating menu items.");

        vec![
            MenuItem {
                label: "Network Settings".to_string(),
                action: {
                    let log_buffers = self.log_buffers.clone();
                    Some(Box::new(move || {
                        log_process_step(&log_buffers, "Network Settings selected.");
                    }))
                },
            },
            MenuItem {
                label: "Data Speed".to_string(),
                action: {
                    let log_buffers = self.log_buffers.clone();
                    Some(Box::new(move || {
                        log_process_step(&log_buffers, "Data Speed selected.");
                    }))
                },
            },
            MenuItem {
                label: "Packet Capture".to_string(),
                action: {
                    let log_buffers = self.log_buffers.clone();
                    Some(Box::new(move || {
                        log_process_step(&log_buffers, "Packet Capture selected.");
                    }))
                },
            },
            MenuItem {
                label: "Exit".to_string(),
                action: {
                    let log_buffers = self.log_buffers.clone();
                    Some(Box::new(move || {
                        log_process_step(&log_buffers, "Exiting application.");
                        std::process::exit(0);
                    }))
                },
            },
        ]
    }
}

// Function to check if the application is running with administrative privileges
fn is_running_as_admin() -> bool {
    use winapi::shared::minwindef::FALSE;
    unsafe {
        let mut token_handle = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == FALSE {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0;
        if GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length
        ) == FALSE {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}
