use crate::menu::{MenuItem, MenuState, render_menu};

pub fn render_ns_menu(ctx: &eframe::egui::Context, state: &mut MenuState) {
    let menu_items = vec![
        MenuItem {
            label: "Wi-Fi".to_string(),
            action: Some(Box::new(|| println!("Wi-Fi Settings selected"))),  // Placeholder action
        },
        MenuItem {
            label: "Ethernet".to_string(),
            action: Some(Box::new(|| println!("Ethernet Settings selected"))),  // Placeholder action
        },
        MenuItem {
            label: "Diagnostics".to_string(),
            action: Some(Box::new(|| println!("Network Diagnostics selected"))),  // Placeholder action
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
