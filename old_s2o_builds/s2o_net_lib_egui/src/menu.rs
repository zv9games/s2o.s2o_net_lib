use eframe::egui::{self, Color32, RichText, CentralPanel, Align2, Area, Id, Key};
use std::time::{Instant, Duration};

pub struct MenuSettings {
    pub font_size: f32,
    pub title_color: Color32,
    pub option_color_unselected: Color32,
    pub selected_font_color: Color32,
}

impl MenuSettings {
    pub fn apply_label(&self, text: &str, selected: bool) -> RichText {
        let color = if selected {
            self.selected_font_color
        } else {
            self.option_color_unselected
        };
        RichText::new(text)
            .font(egui::FontId::proportional(self.font_size))
            .color(color)
    }

    pub fn apply_title(&self, text: &str) -> RichText {
        RichText::new(text)
            .font(egui::FontId::proportional(self.font_size * 1.5))
            .color(self.title_color)
            .strong()
    }
}

// Define default settings
pub fn default_settings() -> MenuSettings {
    MenuSettings {
        font_size: 24.0,
        title_color: Color32::from_rgb(0, 255, 0),
        option_color_unselected: Color32::from_rgb(255, 255, 255),
        selected_font_color: Color32::from_rgb(0, 255, 0),
    }
}

pub struct MenuItem {
    pub label: String,
    pub action: Option<Box<dyn Fn()>>,
}

pub struct MenuState {
    pub selected: usize,
    pub settings: MenuSettings,
    pub start_time: Instant,
}

impl MenuState {
    pub fn new(settings: MenuSettings) -> Self {
        MenuState {
            selected: 0,
            settings,
            start_time: Instant::now(),
        }
    }

    pub fn runtime(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn format_runtime(&self) -> String {
        let runtime = self.runtime();
        format!("{:02}:{:02}:{:02}.{:02}", 
                runtime.as_secs() / 3600, 
                (runtime.as_secs() / 60) % 60, 
                runtime.as_secs() % 60,
                runtime.subsec_millis() / 10)
    }
}

pub fn render_menu(
    ctx: &eframe::egui::Context,
    title: &str,
    menu_items: &[MenuItem],
    state: &mut MenuState,
) {
    // Handle keyboard navigation
    ctx.input(|input| {
        if input.key_pressed(Key::ArrowDown) {
            state.selected = (state.selected + 1) % menu_items.len();
        } else if input.key_pressed(Key::ArrowUp) {
            state.selected = if state.selected == 0 {
                menu_items.len() - 1
            } else {
                state.selected - 1
            };
        }

        // Handle Enter key press
        if input.key_pressed(Key::Enter) {
            if let Some(action) = &menu_items[state.selected].action {
                action();
            }
        }
    });

    CentralPanel::default().show(ctx, |_ui| {
        // Create a centered area for the title
        Area::new(Id::new("title_area"))
            .anchor(Align2::CENTER_TOP, (0.0, 50.0))
            .show(ctx, |ui| {
                ui.heading(state.settings.apply_title(title));
            });

        // Create a centered area for the menu items
        Area::new(Id::new("menu_area"))
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);  // Add space between the title and the options
                    for (index, item) in menu_items.iter().enumerate() {
                        let selected = state.selected == index;
                        let item_response = ui.add(egui::Label::new(state.settings.apply_label(&item.label, selected)));

                        if item_response.clicked() {
                            if let Some(action) = &item.action {
                                action();
                            }
                        }
                    }
                });
            });

        // Display the runtime counter in the bottom right corner
        Area::new(Id::new("runtime_counter"))
            .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0))
            .show(ctx, |ui| {
                ui.label(state.format_runtime());
            });
    });

    // Request repaint every 100 milliseconds
    ctx.request_repaint_after(Duration::from_millis(100));
}
