// menu_utils.rs
use eframe::egui;
use crate::settings;

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

pub fn create_button(ui: &mut egui::Ui, text: egui::RichText, settings: &settings::MenuSettings, is_selected: bool) -> egui::Response {
    let mut button = egui::Button::new(text).frame(true);
    if settings.hover_transparent {
        button = button.fill(egui::Color32::TRANSPARENT);
    }
    let response = ui.add(button);

    // Ensure the visual feedback for selection is always shown, even if hover is transparent
    if is_selected {
        if !settings.hover_transparent {
            ui.painter().rect_filled(response.rect.expand(2.0), 5.0, settings.hover_color);
        } else {
            ui.painter().rect_stroke(response.rect.expand(2.0), 1.0, egui::Stroke::new(2.0, settings.selected_color));
        }
    } 
    // Add hover effect if not selected, but only if hover transparency isn't set
    else if !settings.hover_transparent {
        if response.hovered() {
            ui.painter().rect_filled(response.rect, 5.0, settings.hover_color);
        }
    }

    response
}