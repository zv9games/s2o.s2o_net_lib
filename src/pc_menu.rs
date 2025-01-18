use crate::menu::{MenuItem, MenuState, render_menu};
use crate::packet_sniffer::start_packet_sniffer;

pub fn render_pc_menu(ctx: &eframe::egui::Context, state: &mut MenuState) {
    let menu_items = vec![
        MenuItem {
            label: "Start Capture".to_string(),
            action: Some(Box::new(|| {
                println!("Starting packet capture...");
                start_packet_sniffer("192.168.0.101"); // Example IP address
                println!("Started packet capture");
            })),
        },
        MenuItem {
            label: "Stop Capture".to_string(),
            action: Some(Box::new(|| {
                // Implement stopping capture logic
                println!("Stopped packet capture");
            })),
        },
        MenuItem {
            label: "View Captures".to_string(),
            action: Some(Box::new(|| println!("View Captures selected"))),  // Placeholder action for viewing captures
        },
        MenuItem {
            label: "Capture Settings".to_string(),
            action: Some(Box::new(|| println!("Capture Settings selected"))),  // Placeholder action for capture settings
        },
        MenuItem {
            label: "Exit".to_string(),
            action: None,  // No action needed, handled in program_menu.rs
        },
    ];

    eframe::egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("S2O's net_lib Crate");
        ui.add_space(20.0);  // Add space between the heading and the options
        render_menu(ctx, "S2O's net_lib Crate", &menu_items, state);
    });
}
