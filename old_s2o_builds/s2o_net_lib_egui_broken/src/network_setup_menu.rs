use eframe::egui;
use crate::settings::Settings;

pub fn show_menu(ui: &mut egui::Ui, settings: &Settings) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("Network Setup")
            .font(egui::FontId::proportional(settings.font_size))
            .color(egui::Color32::from_rgb(0, 255, 0))
            .strong()
        );
        ui.add_space(20.0);

        ui.label("Configure your network settings here.");
        // Add actual network setup UI elements here
        if ui.button("Back").clicked() {
            // Here, you might want to handle returning to the admin menu
            // This could mean changing some state in AdminApp or closing this window
        }
    });
}