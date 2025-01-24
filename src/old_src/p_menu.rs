use crate::{
    menu_rendering::{MenuItem, MenuContext, render_menu, default_style, has_admin_rights},
    app_state::{SharedAppState, AppState},
    logging::{LogBuffers, log_info},
    pc_menu::run_ui,
    ns_menu::render_ns_menu,
    ds_menu::render_ds_menu,
};
use std::{sync::Mutex, collections::HashSet, env, path::PathBuf, ffi::OsString, os::windows::ffi::OsStrExt};
use once_cell::sync::Lazy;
use winapi::um::libloaderapi::{AddDllDirectory, SetDefaultDllDirectories, LOAD_LIBRARY_SEARCH_APPLICATION_DIR, LOAD_LIBRARY_SEARCH_SYSTEM32};

// Configure DLL path and environment
fn configure_dll_env(log_buffers: &LogBuffers) {
    let dll_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string())).join("src/s2o_dll");
    let dll_dir_str = dll_dir.to_str().unwrap_or("");
    env::set_var("PATH", format!("{};{}", dll_dir_str, env::var("PATH").unwrap_or_default()));
    log_process_step(log_buffers, &format!("PATH now includes: {}", dll_dir_str));

    unsafe {
        let wide: Vec<u16> = OsString::from(dll_dir_str).encode_wide().chain(std::iter::once(0)).collect();
        let handle = AddDllDirectory(wide.as_ptr());
        let log_msg = if handle.is_null() { "DLL directory addition failed." } else { format!("DLL directory added: {}", dll_dir_str) };
        log_process_step(log_buffers, log_msg);
        if handle.is_null() || SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_APPLICATION_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32) == 0 {
            log_process_step(log_buffers, "Setting default DLL directories failed.");
        } else {
            log_process_step(log_buffers, "Default DLL directories configured.");
        }
    }
}

// Shared static for tracking logged steps
static TRACKED_STEPS: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

// Log a new step in the process
fn log_process_step(log_buffers: &LogBuffers, step: &str) {
    let mut tracked_steps = TRACKED_STEPS.lock().expect("Failed to track step");
    if tracked_steps.insert(step.to_string()) {
        log_info(log_buffers, step);
    }
}

fn main() {
    let log_buffers = LogBuffers::new();
    configure_dll_env(&log_buffers);
    run_program_menu(&SharedAppState::new(AppState::SecurityMenu), &log_buffers);
}

// Handle the program menu loop and state changes
pub fn run_program_menu(shared_state: &SharedAppState, log_buffers: &LogBuffers) {
    log_process_step(log_buffers, "Program menu initiated.");
    if !has_admin_rights() {
        log_process_step(log_buffers, "Admin rights missing, redirecting to security menu.");
        shared_state.set_state(AppState::SecurityMenu);
        return;
    }

    let state = MenuContext::new(default_style(), has_admin_rights());
    log_process_step(log_buffers, "Menu context initialized.");
    configure_dll_env(log_buffers);

    if let Err(e) = eframe::run_native("Main Menu", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(ProgramMenuApp { state, shared_state: shared_state.clone(), log_buffers: log_buffers.clone() })))) {
        log_process_step(log_buffers, &format!("Failed to run main menu: {}", e));
        std::process::exit(1);
    }
    log_process_step(log_buffers, "Main menu running.");
}

struct ProgramMenuApp {
    state: MenuContext,
    shared_state: SharedAppState,
    log_buffers: LogBuffers,
}

impl eframe::App for ProgramMenuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        log_process_step(&self.log_buffers, "Updating UI.");
        let current_state = self.shared_state.get_state();
        log_process_step(&self.log_buffers, &format!("Current state: {:?}", current_state));

        match current_state {
            AppState::ProgramMenu => {
                log_process_step(&self.log_buffers, "Displaying program menu.");
                match render_menu(ctx, "S2O's Network Lib", &self.menu_items(), &mut self.state, &self.log_buffers, self.log_buffers.info_buffer.clone()) {
                    Err(e) => log_process_step(&self.log_buffers, &format!("Menu display failed: {}", e)),
                    Ok(_) => {
                        log_process_step(&self.log_buffers, "Checking for menu interactions.");
                        if ctx.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                            match self.state.selection {
                                0 => self.transition_to(AppState::NSMenu, "Network Settings"),
                                1 => self.transition_to(AppState::DSMenu, "Data Speed"),
                                2 => self.transition_to(AppState::PacketCaptureMenu, "Packet Capture"),
                                _ => {}
                            }
                        }
                    }
                }
            },
            AppState::NSMenu => {
                log_process_step(&self.log_buffers, "Rendering Network Settings.");
                render_ns_menu(ctx, &mut self.state, &self.shared_state, &self.log_buffers, self.log_buffers.info_buffer.clone());
            },
            AppState::DSMenu => {
                log_process_step(&self.log_buffers, "Rendering Data Speed.");
                render_ds_menu(ctx, &mut self.state, &self.shared_state, &self.log_buffers, self.log_buffers.info_buffer.clone());
            },
            AppState::PacketCaptureMenu => {
                log_process_step(&self.log_buffers, "Rendering Packet Capture.");
                run_ui(ctx, &self.shared_state, &mut self.state, &self.log_buffers);
            },
            _ => log_process_step(&self.log_buffers, "Unknown state encountered."),
        }
    }
}

impl ProgramMenuApp {
    fn menu_items(&self) -> Vec<MenuItem> {
        log_process_step(&self.log_buffers, "Generating menu options.");

        vec![
            MenuItem::new("Network Settings", move || log_process_step(&self.log_buffers, "Network Settings chosen.")),
            MenuItem::new("Data Speed", move || log_process_step(&self.log_buffers, "Data Speed chosen.")),
            MenuItem::new("Packet Capture", move || log_process_step(&self.log_buffers, "Packet Capture chosen.")),
            MenuItem::new("Exit", move || {
                log_process_step(&self.log_buffers, "Exiting application.");
                std::process::exit(0);
            }),
        ]
    }

    fn transition_to(&mut self, state: AppState, log_msg: &str) {
        self.state.selection = 0;
        self.shared_state.set_state(state);
        log_process_step(&self.log_buffers, log_msg);
    }
}

impl MenuItem {
    fn new(label: &str, action: impl Fn() + 'static) -> Self {
        MenuItem {
            label: label.to_string(),
            action: Some(Box::new(action)),
        }
    }
}