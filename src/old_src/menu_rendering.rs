use eframe::egui::{self, Color32, RichText, CentralPanel, SidePanel, Align2, Area, Id, Key, ScrollArea};
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use crate::logging::{LogBuffers, log_info};
use winapi::um::winnt::TOKEN_QUERY;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION};

pub struct MenuStyle {
    pub font_size: f32,
    pub header_color: Color32,
    pub option_color: Color32,
    pub selected_color: Color32,
}

impl MenuStyle {
    pub fn style_label(&self, text: &str, selected: bool) -> RichText {
        RichText::new(text)
            .font(egui::FontId::proportional(self.font_size))
            .color(if selected { self.selected_color } else { self.option_color })
    }

    pub fn style_header(&self, text: &str) -> RichText {
        RichText::new(text)
            .font(egui::FontId::proportional(self.font_size * 1.5))
            .color(self.header_color)
            .strong()
    }
}

pub fn default_style() -> MenuStyle {
    MenuStyle {
        font_size: 24.0,
        header_color: Color32::GREEN,
        option_color: Color32::WHITE,
        selected_color: Color32::GREEN,
    }
}

pub struct MenuOption {
    pub label: String,
    pub action: Option<Box<dyn Fn() + 'static>>,
}

pub struct MenuContext {
    pub selection: usize,
    pub style: MenuStyle,
    pub launch_time: Instant,
    pub is_admin: bool,
}

impl MenuContext {
    pub fn new(style: MenuStyle, is_admin: bool) -> Self {
        MenuContext {
            selection: 0,
            style,
            launch_time: Instant::now(),
            is_admin,
        }
    }

    pub fn runtime(&self) -> Duration {
        self.launch_time.elapsed()
    }

    pub fn format_runtime(&self) -> String {
        let runtime = self.runtime();
        format!("{:02}:{:02}:{:02}.{:02}", 
                runtime.as_secs() / 3600, 
                (runtime.as_secs() % 3600) / 60, 
                runtime.as_secs() % 60, 
                runtime.subsec_millis() / 10)
    }
}

pub fn render_menu(
    ctx: &eframe::egui::Context,
    title: &str,
    menu_items: &[MenuOption],
    state: &mut MenuContext,
    log_buffers: &LogBuffers,
    log_buffer: Arc<Mutex<Vec<String>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    ctx.input(|input| {
        if input.key_pressed(Key::ArrowDown) {
            state.selection = (state.selection + 1) % menu_items.len();
        } else if input.key_pressed(Key::ArrowUp) {
            state.selection = state.selection.checked_sub(1).unwrap_or(menu_items.len() - 1);
        }

        if input.key_pressed(Key::Enter) {
            if let Some(action) = &menu_items[state.selection].action {
                action();
                log_info(log_buffers, &format!("Action triggered for: {}", menu_items[state.selection].label));
            }
        }
    });

    CentralPanel::default().show(ctx, |ui| {
        Area::new(Id::new("title_area"))
            .anchor(Align2::CENTER_TOP, (0.0, 50.0))
            .show(&ui.ctx(), |ui| {
                ui.heading(state.style.style_header(title));
            });

        Area::new(Id::new("menu_options"))
            .anchor(Align2::CENTER_CENTER, (0.0, 0.0))
            .show(&ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(30.0);
                    for (index, item) in menu_items.iter().enumerate() {
                        let selected = state.selection == index;
                        let item_ui = ui.add(egui::Label::new(state.style.style_label(&item.label, selected)));

                        if item_ui.clicked() {
                            if let Some(action) = &item.action {
                                action();
                                log_info(log_buffers, &format!("Option selected: {}", item.label));
                            }
                        }
                    }
                });
            });

        Area::new(Id::new("admin_status"))
            .anchor(Align2::RIGHT_TOP, (-20.0, 20.0))
            .show(&ui.ctx(), |ui| {
                let color = if state.is_admin { Color32::GREEN } else { Color32::RED };
                ui.colored_label(color, "●");
            });

        Area::new(Id::new("runtime_display"))
            .anchor(Align2::RIGHT_BOTTOM, (-10.0, -10.0))
            .show(&ui.ctx(), |ui| {
                ui.label(state.format_runtime());
            });
    });

    SidePanel::left("log_view")
        .show(ctx, |ui| {
            ui.heading("Debug Logs");
            ScrollArea::vertical().show(ui, |ui| {
                if let Ok(buffer) = log_buffer.lock() {
                    for log in buffer.iter().rev().take(20) {
                        ui.label(log);
                    }
                } else {
                    ui.label("Log access failed");
                }
            });
        });

    ctx.request_repaint_after(Duration::from_millis(100));
    Ok(())
}

pub fn has_admin_rights() -> bool {
    use winapi::shared::minwindef::FALSE;
    unsafe {
        let mut token = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == FALSE {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length = 0;
        if GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length
        ) == FALSE {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}