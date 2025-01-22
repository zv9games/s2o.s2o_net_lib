use crate::menu::{MenuItem, MenuState, render_menu, default_settings};
use crate::logging::init_logging;  
use std::sync::{Arc, Mutex};
use eframe::egui;

#[derive(Debug)]
enum AppState {
    ProgramMenu,
    DSMenu,
    NSMenu,
    PCMenu,
}

pub fn program_menu_loop() {
    let state = MenuState::new(default_settings());
    let app_state = AppState::ProgramMenu;  // Removed underscore as it's now used
    let log_buffer = init_logging();

    if let Err(e) = eframe::run_native(
        "Program Menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Ok(Box::new(ProgramMenuApp {
                state,
                log_buffer,
            }))
        }),
    ) {
        eprintln!("Failed to run program menu: {}", e);
        std::process::exit(1);
    }
}

struct ProgramMenuApp {
    state: MenuState,
    log_buffer: Arc<Mutex<Vec<String>>>,
}

impl eframe::App for ProgramMenuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let menu_items = self.menu_items();
        render_menu(ctx, "S2O's s2o_net_lib Crate", &menu_items, &mut self.state, self.log_buffer.clone());

        // Check for state transitions
        if let Some(selected) = self.state.selected {
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                match selected {
                    0 => self.state.selected = None, // Reset selector for NSMenu
                    1 => self.state.selected = None, // Reset selector for DSMenu
                    2 => self.state.selected = None, // Reset selector for PCMenu
                    3 => std::process::exit(0),      // Exit option
                    _ => (), // Do nothing for unknown selections
                }
            }
        }
    }
}

impl ProgramMenuApp {
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