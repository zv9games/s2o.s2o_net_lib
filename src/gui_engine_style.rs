use eframe::egui::{self, Color32, RichText};

// MenuSettings struct definition
pub struct MenuSettings {
    pub font_size: f32,
    pub title_color: Color32,
    pub option_color_unselected: Color32,
    pub selected_font_color: Color32,
}

impl MenuSettings {
    // Applies styles to labels
    pub fn apply_label(&self, text: &str, selected: bool) -> RichText {
        let color = if selected { self.selected_font_color } else { self.option_color_unselected };
        RichText::new(text)
            .font(egui::FontId::proportional(self.font_size))
            .color(color)
    }

    // Applies styles to titles
    pub fn apply_title(&self, text: &str) -> RichText {
        RichText::new(text)
            .font(egui::FontId::proportional(self.font_size * 1.5))
            .color(self.title_color)
            .strong()
    }
}

// Function to return default menu settings
pub fn default_settings() -> MenuSettings {
    MenuSettings {
        font_size: 24.0,
        title_color: Color32::from_rgb(0, 255, 0),
        option_color_unselected: Color32::from_rgb(255, 255, 255),
        selected_font_color: Color32::from_rgb(0, 255, 0),
    }
}
