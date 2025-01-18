use eframe::egui;
use crate::settings::Settings;

pub fn show_menu(ui: &mut egui::Ui, settings: &Settings) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("Data Speed")
            .font(egui::FontId::proportional(settings.font_size))
            .color(egui::Color32::from_rgb(0, 255, 0))
            .strong()
        );
        ui.add_space(20.0);

        ui.label("Monitor your data speed here.");
        // Add real-time speed indicators or graphs here
        if ui.button("Back").clicked() {
            // Handle returning to admin menu
        }
    });
}

// menu_settings.rs

pub struct Settings {
    pub font_size: f32,
    pub window_size: (f32, f32),
    pub horizontal_pos: f32,
    pub vertical_pos: f32,
}
