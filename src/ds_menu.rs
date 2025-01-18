use crate::menu::{MenuItem, MenuState, render_menu};

pub fn render_ds_menu(ctx: &eframe::egui::Context, state: &mut MenuState) {
    let menu_items = vec![
        MenuItem {
            label: "Speed Test".to_string(),
            action: Some(Box::new(|| println!("Speed Test selected"))),  // Placeholder action
        },
        MenuItem {
            label: "History".to_string(),
            action: Some(Box::new(|| println!("History selected"))),     // Placeholder action
        },
        MenuItem {
            label: "Settings".to_string(),
            action: Some(Box::new(|| println!("Settings selected"))),    // Placeholder action
        },
        MenuItem {
            label: "Exit".to_string(),
            action: None,  // No action needed, handled in program_menu.rs
        },
    ];

    render_menu(ctx, "S2O's s2o_net_lib Crate", &menu_items, state);
}
