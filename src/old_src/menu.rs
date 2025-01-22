use eframe::egui::{self, Color32, RichText, CentralPanel, SidePanel, Align2, Area, Id, Key, ScrollArea};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::process;
use crate::logging::log_info;  // Import the log_info function

// Struct to define menu settings
pub struct MenuSettings {
    pub font_size: f32,
    pub title_color: Color32,
    pub option_color_unselected: Color32,
    pub selected_font_color: Color32,
}

impl MenuSettings {
    // Apply settings to a menu item label
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

    // Apply settings to the menu title
    pub fn apply_title(&self, text: &str) -> RichText {
        RichText::new(text)
            .font(egui::FontId::proportional(self.font_size * 1.5))
            .color(self.title_color)
            .strong()
    }
}

// Define default menu settings
pub fn default_settings() -> MenuSettings {
    MenuSettings {
        font_size: 24.0,
        title_color: Color32::from_rgb(0, 255, 0),
        option_color_unselected: Color32::from_rgb(255, 255, 255),
        selected_font_color: Color32::from_rgb(0, 255, 0),
    }
}

// Struct to define a menu item
pub struct MenuItem {
    pub label: String,
    pub action: Option<Box<dyn Fn()>>,
}

// Struct to define the menu state
pub struct MenuState {
    pub selected: usize,
    pub settings: MenuSettings,
    pub start_time: Instant,
}

impl MenuState {
    // Create a new menu state with given settings
    pub fn new(settings: MenuSettings) -> Self {
        MenuState {
            selected: 0,
            settings,
            start_time: Instant::now(),
        }
    }

    // Calculate runtime duration
    pub fn runtime(&self) -> Duration {
        self.start_time.elapsed()
    }

    // Format runtime duration as a string
    pub fn format_runtime(&self) -> String {
        let runtime = self.runtime();
        format!("{:02}:{:02}:{:02}.{:02}", 
                runtime.as_secs() / 3600, 
                (runtime.as_secs() / 60) % 60, 
                runtime.as_secs() % 60,
                runtime.subsec_millis() / 10)
    }
}

// Function to render the menu
pub fn render_menu(
    ctx: &eframe::egui::Context,
    title: &str,
    menu_items: &[MenuItem],
    state: &mut MenuState,
    log_buffer: Arc<Mutex<Vec<String>>>,
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

    // Render the menu in the central panel
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
                                log_info(&format!("Action executed for: {}", item.label));
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

    // Display the logging panel
    SidePanel::left("logging_panel")
        .show(ctx, |ui| {
            ui.heading("Debug Info Log");
            ScrollArea::vertical().show(ui, |ui| {
                let buffer = log_buffer.lock().unwrap();
                for log_entry in buffer.iter() {
                    ui.label(log_entry);
                }
            });
        });

    // Request repaint every 100 milliseconds
    ctx.request_repaint_after(Duration::from_millis(100));
}
