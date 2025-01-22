use crate::menu::{MenuItem, MenuState, render_menu, default_settings};
use crate::app_state::{SharedAppState, AppState};
use std::sync::{Arc, Mutex};
use crate::logging::init_logging;
use crate::logging::log_info;
use crate::pc_menu::run_ui;

// Function to run the program menu loop
pub fn program_menu_loop() {
    let state = MenuState::new(default_settings());  // Initialize menu state with default settings
    let shared_state = SharedAppState::new(AppState::ProgramMenu);  // Initialize shared application state
    let log_buffer = init_logging();  // Initialize logging

    // Run the native application for the program menu
    if let Err(e) = eframe::run_native(
        "Program Menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ProgramMenuApp {
            state,
            shared_state,
            log_buffer,
        }))),
    ) {
        eprintln!("Failed to run program menu: {}", e);
        std::process::exit(1);
    }
}

// Structure to manage the program menu application state
struct ProgramMenuApp {
    state: MenuState,
    shared_state: SharedAppState,
    log_buffer: Arc<Mutex<Vec<String>>>,
}

impl eframe::App for ProgramMenuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let current_app_state = self.shared_state.get_state();

        match current_app_state {
            AppState::ProgramMenu => {
                render_menu(ctx, "S2O's s2o_net_lib Crate", &self.menu_items(), &mut self.state, self.log_buffer.clone());

                // Check for state transitions
                if self.state.selected == 1 && ctx.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                    self.state.selected = 0;
                    log_info("Transition to DSMenu");
                    self.shared_state.set_state(AppState::DSMenu);
                } else if self.state.selected == 0 && ctx.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                    self.state.selected = 0;
                    log_info("Transition to NSMenu");
                    self.shared_state.set_state(AppState::NSMenu);
                } else if self.state.selected == 2 && ctx.input(|i| i.key_pressed(eframe::egui::Key::Enter)) {
                    self.state.selected = 0;
                    log_info("Transition to PacketCaptureMenu");
                    self.shared_state.set_state(AppState::PacketCaptureMenu);
                }
            },
            AppState::PacketCaptureMenu => {
                log_info("Rendering PCMenu");
                run_ui(ctx, &self.shared_state, &mut self.state, self.log_buffer.clone());
            },
            _ => {}
        }
    }
}

impl ProgramMenuApp {
    // Function to define menu items
    fn menu_items(&self) -> Vec<MenuItem> {
        vec![
            MenuItem {
                label: "Network Settings".to_string(),
                action: Some(Box::new(|| println!("Network Settings selected"))),
            },
            MenuItem {
                label: "Data Speed".to_string(),
                action: Some(Box::new(|| println!("Data Speed selected"))),
            },
            MenuItem {
                label: "Packet Capture".to_string(),
                action: Some(Box::new(|| println!("Packet Capture selected"))),
            },
            MenuItem {
                label: "Exit".to_string(),
                action: Some(Box::new(|| std::process::exit(0))),
            },
        ]
    }
}
