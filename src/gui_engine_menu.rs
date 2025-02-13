use crate::logging;
use crate::app_state::AppState;
use eframe::egui::{self, RichText, CentralPanel, Align2, Area, Id, Context};
use std::time::{Instant, Duration};

pub struct MenuItem {
    pub label: String,
    pub action: Option<Box<dyn Fn() + 'static>>,
}

pub struct MenuState {
    pub selected: usize,
    pub settings: crate::gui_engine_style::MenuSettings,
    pub start_time: Instant,
}

impl MenuState {
    pub fn new(settings: crate::gui_engine_style::MenuSettings) -> Self {
        logging::debug_info("MenuState initialized with default settings");
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
        format!(
            "{:02}:{:02}:{:02}.{:02}",
            runtime.as_secs() / 3600,
            (runtime.as_secs() % 3600) / 60,
            runtime.as_secs() % 60,
            runtime.subsec_millis() / 10
        )
    }

    // Method to update selected index
    pub fn set_selected(&mut self, index: usize) {
        self.selected = index;
    }

    // Method to access settings
    pub fn get_settings(&self) -> &crate::gui_engine_style::MenuSettings {
        &self.settings
    }
}

pub fn render_menu(
    ctx: &Context,
    title: &str,
    menu_items: &[MenuItem],
    menu_state: &MenuState,
    is_elevated: bool,
    runtime: String,
) {
    logging::debug_info("Rendering menu");
    let settings = &menu_state.settings;
    let selected_index = menu_state.selected;

    CentralPanel::default()
        .frame(egui::Frame::none()) // Make the menu panel transparent
        .show(ctx, |ui| {
            Area::new(Id::new("title_area"))
                .anchor(Align2::CENTER_TOP, (0.0, 50.0))
                .show(&ui.ctx(), |ui| {
                    ui.heading(settings.apply_title(title));
                });

            Area::new(Id::new("menu_area"))
                .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
                .show(&ui.ctx(), |ui| {
                    ui.vertical_centered(|ui| {
                        for (index, item) in menu_items.iter().enumerate() {
                            let selected = index == selected_index;
                            ui.label(settings.apply_label(&item.label, selected));
                        }
                    });
                });

            Area::new(Id::new("status_area"))
                .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0))
                .show(&ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("■")
                                .color(if is_elevated { settings.selected_font_color } else { settings.option_color_unselected })
                                .font(egui::FontId::monospace(24.0))
                        );
                        ui.label(
                            RichText::new(runtime)
                                .font(egui::FontId::proportional(18.0))
                        );
                    });
                });
        });
    logging::debug_info("Menu rendered successfully");
}

pub fn render_app_state(
    ctx: &Context,
    app_state: &AppState,
    menu_state: &MenuState,
    is_elevated: bool,
    runtime: String,
    set_app_state: impl Fn(AppState) + 'static + Clone,
) {
    let set_app_state = set_app_state.clone();
    logging::debug_info(&format!("Rendering app state: {:?}", app_state));
    match app_state {
        AppState::SMenu => {
            render_menu(
                ctx,
                "Security Menu",
                &crate::s_menu::menu_items(set_app_state.clone(), &menu_state.get_settings()),
                menu_state,
                is_elevated,
                runtime,
            );
        }
        AppState::PMenu => {
            render_menu(
                ctx,
                "P Menu",
                &crate::p_menu::menu_items(set_app_state.clone(), &menu_state.get_settings()),
                menu_state,
                is_elevated,
                runtime,
            );
        }
        AppState::PCMenu => {
            // Add rendering for PCMenu
        }
        AppState::NSMenu => {
            // Add rendering for NSMenu
        }
        AppState::DSMenu => {
            // Add rendering for DSMenu
        }
    }
    logging::debug_info("App state rendered successfully");
}
