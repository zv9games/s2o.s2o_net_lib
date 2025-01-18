// menu_settings.rs
use eframe::egui;

pub struct MenuSettings {
    pub header_font_size: f32,
    pub option_font_size: f32,
    pub selected_color: egui::Color32,
    pub default_color: egui::Color32,
    pub hover_color: egui::Color32,
    pub hover_transparent: bool, // New field to control transparency
}

impl Default for MenuSettings {
    fn default() -> Self {
        Self {
            header_font_size: 27.0,
            option_font_size: 18.0,
            selected_color: egui::Color32::from_rgb(0, 255, 0),
            default_color: egui::Color32::from_rgb(255, 255, 255),
            hover_color: egui::Color32::from_rgba_premultiplied(0, 255, 0, 50),
            hover_transparent: true, // Set to true for transparency by default
        }
    }
}

pub fn get_menu_settings() -> MenuSettings {
    MenuSettings::default()
}

pub fn create_button(ui: &mut egui::Ui, text: egui::RichText, settings: &MenuSettings, is_selected: bool) -> egui::Response {
    let mut button = egui::Button::new(text).frame(true);
    if settings.hover_transparent {
        button = button.fill(egui::Color32::TRANSPARENT);
    }
    let response = ui.add(button);

    if is_selected {
        if !settings.hover_transparent {
            ui.painter().rect_filled(response.rect.expand(2.0), 5.0, settings.hover_color);
        } else {
            ui.painter().rect_stroke(response.rect.expand(2.0), 1.0, egui::Stroke::new(2.0, settings.selected_color));
        }
    } else if !settings.hover_transparent {
        if response.hovered() {
            ui.painter().rect_filled(response.rect, 5.0, settings.hover_color);
        }
    }

    response
}

pub fn handle_menu_navigation(ui: &mut egui::Ui, state: &mut usize, options_len: usize) {
    ui.input(|i| {
        if i.key_pressed(egui::Key::ArrowDown) {
            *state = (*state + 1) % options_len;
            ui.ctx().request_repaint(); // Ensure UI updates to reflect new selection
        } else if i.key_pressed(egui::Key::ArrowUp) {
            *state = (*state + options_len - 1) % options_len;
            ui.ctx().request_repaint(); // Ensure UI updates to reflect new selection
        }
    });
}
