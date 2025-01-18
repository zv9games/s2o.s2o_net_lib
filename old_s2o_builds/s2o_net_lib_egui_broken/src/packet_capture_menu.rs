use eframe::egui;
use crate::settings::Settings;

pub fn show_menu(ui: &mut egui::Ui, settings: &Settings) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("Packet Capture")
            .font(egui::FontId::proportional(settings.font_size))
            .color(egui::Color32::from_rgb(0, 255, 0))
            .strong()
        );
        ui.add_space(20.0);

        ui.label("Capture network packets here.");
        // Add functionality to start/stop capture, view packets, etc.
        if ui.button("Back").clicked() {
            // Handle returning to admin menu
        }
    });
}