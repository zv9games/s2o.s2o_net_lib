use crate::menu::{MenuItem, MenuState, render_menu, default_settings};
use crate::ds_menu::render_ds_menu;
use crate::ns_menu::render_ns_menu;
use crate::pc_menu::render_pc_menu;
use eframe::egui::Key;

enum AppState {
    ProgramMenu,
    DSMenu,
    NSMenu,
    PCMenu,
}

pub fn program_menu_loop() {
    let state = MenuState::new(default_settings());
    let app_state = AppState::ProgramMenu;

    eframe::run_native(
        "Program Menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(ProgramMenuApp {
            state,
            app_state,
        }))),
    ).unwrap();
}

struct ProgramMenuApp {
    state: MenuState,
    app_state: AppState,
}

impl eframe::App for ProgramMenuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        match self.app_state {
            AppState::ProgramMenu => {
                render_menu(ctx, "S2O's s2o_net_lib Crate", &self.menu_items(), &mut self.state);

                // Check for state transitions
                if self.state.selected == 1 && ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.app_state = AppState::DSMenu;
                    self.state.selected = 0;  // Reset selector when transitioning to DSMenu
                } else if self.state.selected == 0 && ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.app_state = AppState::NSMenu;
                    self.state.selected = 0;  // Reset selector when transitioning to NSMenu
                } else if self.state.selected == 2 && ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.app_state = AppState::PCMenu;
                    self.state.selected = 0;  // Reset selector when transitioning to PCMenu
                }
            }
            AppState::DSMenu => {
                render_ds_menu(ctx, &mut self.state);

                // Check for state transition back to ProgramMenu
                if self.state.selected == 3 && ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.app_state = AppState::ProgramMenu;
                    self.state.selected = 0;  // Reset selector when returning to ProgramMenu
                }
            }
            AppState::NSMenu => {
                render_ns_menu(ctx, &mut self.state);

                // Check for state transition back to ProgramMenu
                if self.state.selected == 3 && ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.app_state = AppState::ProgramMenu;
                    self.state.selected = 0;  // Reset selector when returning to ProgramMenu
                }
            }
            AppState::PCMenu => {
                render_pc_menu(ctx, &mut self.state);

                // Check for state transition back to ProgramMenu
                if self.state.selected == 3 && ctx.input(|i| i.key_pressed(Key::Enter)) {
                    self.app_state = AppState::ProgramMenu;
                    self.state.selected = 0;  // Reset selector when returning to ProgramMenu
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
                action: Some(Box::new(|| println!("Network Settings selected"))),  // Placeholder action
            },
            MenuItem {
                label: "Data Speed".to_string(),
                action: Some(Box::new(|| println!("Data Speed selected"))),        // Placeholder action
            },
            MenuItem {
                label: "Packet Capture".to_string(),
                action: Some(Box::new(|| println!("Packet Capture selected"))),    // Placeholder action
            },
            MenuItem {
                label: "Exit".to_string(),
                action: Some(Box::new(|| std::process::exit(0))),                  // Exit action
            },
        ]
    }
}
