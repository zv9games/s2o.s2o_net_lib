use crate::menu::{MenuItem, MenuState, render_menu, default_settings};

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

    let _ = eframe::run_native(
        "Security Menu",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(SecurityMenuApp { state, menu_items }))),
    );
}

struct SecurityMenuApp {
    state: MenuState,
    menu_items: Vec<MenuItem>,
}

impl eframe::App for SecurityMenuApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        render_menu(ctx, "Security Menu", &self.menu_items, &mut self.state);
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
