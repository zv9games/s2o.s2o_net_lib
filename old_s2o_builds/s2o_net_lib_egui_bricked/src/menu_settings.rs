use eframe::egui::{Color32, FontId, RichText, Label};

// Define a struct to hold the menu settings
pub struct MenuSettings {
    pub font_size: f32,
    pub title_color: Color32,
    pub option_color_selected: Color32,
    pub option_color_unselected: Color32,
}

// Implement a method to apply the settings to a menu item
impl MenuSettings {
    pub fn apply_label(&self, text: &str, selected: bool) -> Label {
        let color = if selected {
            self.option_color_selected
        } else {
            self.option_color_unselected
        };
        Label::new(
            RichText::new(text)
                .font(FontId::proportional(self.font_size))
                .color(color)
        )
    }

    pub fn apply_title(&self, text: &str) -> Label {
        Label::new(
            RichText::new(text)
                .font(FontId::proportional(self.font_size * 1.5))
                .color(self.title_color)
                .strong()
        )
    }
}


// Define default settings
pub fn default_settings() -> MenuSettings {
    MenuSettings {
        font_size: 24.0,
        title_color: Color32::from_rgb(0, 255, 0),
        option_color_selected: Color32::from_rgb(0, 255, 0),
        option_color_unselected: Color32::from_rgb(255, 255, 255),
    }
}
