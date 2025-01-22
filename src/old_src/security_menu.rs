use crate::menu::{MenuItem, MenuState, render_menu, default_settings};
use std::sync::{Arc, Mutex};
use crate::logging::init_logging;  // Use the logging module

pub fn security_menu_loop() {
    let menu_items = vec![
        MenuItem {
            label: "Admin Menu".to_string(),
            action: Some(Box::new(|| run_admin_menu())),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new(|| std::process::exit(0))),
        },
    ];

    let state = MenuState::new(default_settings());

    let log_buffer = init_logging();  // Initialize logging here

    // Run the native application for the security menu
    if let Err(e) = eframe::run_native(
        "Security Menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(SecurityMenuApp { state, menu_items, log_buffer }))),
    ) {
        eprintln!("Failed to run security menu: {}", e);
        std::process::exit(1);
    }
}

struct SecurityMenuApp {
    state: MenuState,
    menu_items: Vec<MenuItem>,
    log_buffer: Arc<Mutex<Vec<String>>>,
}

impl eframe::App for SecurityMenuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        render_menu(ctx, "Security Menu", &self.menu_items, &mut self.state, self.log_buffer.clone());
    }
}

fn run_admin_menu() {
    // Call PowerShell script to request elevated permissions
    let output = std::process::Command::new("powershell")
        .arg("-File")
        .arg("uac.ps1")
        .arg(format!("{}", std::env::current_exe().unwrap().display()))
        .arg("--admin")
        .output()
        .expect("Failed to execute script");

    println!("{:?}", output);
}
