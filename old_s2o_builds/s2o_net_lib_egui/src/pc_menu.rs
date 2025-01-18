use crate::menu::{MenuItem, MenuState, render_menu};

pub fn render_pc_menu(ctx: &eframe::egui::Context, state: &mut MenuState) {
    let menu_items = vec![
        MenuItem {
            label: "Start Capture".to_string(),
            action: Some(Box::new(|| println!("Start Capture selected"))),  // Placeholder action
        },
        MenuItem {
            label: "View Captures".to_string(),
            action: Some(Box::new(|| println!("View Captures selected"))),  // Placeholder action
        },
        MenuItem {
            label: "Capture Settings".to_string(),
            action: Some(Box::new(|| println!("Capture Settings selected"))),  // Placeholder action
        },
        MenuItem {
            label: "Exit".to_string(),
            action: None,  // No action needed, handled in program_menu.rs
        },
    ];

    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("S2O's s2o_net_lib Crate");
        ui.add_space(20.0);  // Add space between the heading and the options
        render_menu(ctx, "S2O's s2o_net_lib Crate", &menu_items, state);
    });
}
