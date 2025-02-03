use crate::logging;
use crate::s_menu;
use crate::p_menu;
use crate::app_state::{self, AppState};
use crate::gui_engine_animation::AnimationState;
use eframe::egui::{self, Context, FontDefinitions, FontData, FontFamily};
use std::sync::{Arc, Mutex};

pub fn init_module() -> Result<(), String> {
    let initialization_passed = true;
    if initialization_passed {
        logging::debug_info("gui_engine module is online");
        Ok(())
    } else {
        Err("gui_engine module initialization failed".to_string())
    }
}

struct MyApp {
    app_state: Arc<Mutex<AppState>>,
    menu_settings: crate::gui_engine_style::MenuSettings,
    menu_items: Vec<crate::gui_engine_menu::MenuItem>,
    selected_index: usize,
    is_elevated: bool,
    menu_state: crate::gui_engine_menu::MenuState,
    animation_state: AnimationState,
}

impl MyApp {
    fn set_app_state(&mut self, new_state: AppState) {
        logging::debug_info(&format!("Setting app state to: {:?}", new_state));
        let mut app_state = self.app_state.lock().unwrap();
        *app_state = new_state;
        self.menu_items = match new_state {
            AppState::SMenu => s_menu::menu_items(self.get_set_app_state_closure()),
            AppState::PMenu => p_menu::menu_items(self.get_set_app_state_closure()),
            _ => vec![],
        };
        logging::debug_info("App state set successfully");
    }

    fn get_set_app_state_closure(&self) -> impl Fn(AppState) + 'static + Clone {
        let app_state_clone = self.app_state.clone();
        move |state| {
            logging::debug_info(&format!("Updating app state to: {:?}", state));
            let mut app_state = app_state_clone.lock().unwrap();
            *app_state = state;
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle keyboard input
        ctx.input(|input| {
            if input.key_pressed(egui::Key::ArrowDown) {
                logging::debug_info("ArrowDown key pressed");
                self.selected_index = (self.selected_index + 1) % self.menu_items.len();
            }
            if input.key_pressed(egui::Key::ArrowUp) {
                logging::debug_info("ArrowUp key pressed");
                if self.selected_index == 0 {
                    self.selected_index = self.menu_items.len() - 1;
                } else {
                    self.selected_index -= 1;
                }
            }
            if input.key_pressed(egui::Key::Enter) {
                logging::debug_info("Enter key pressed");
                if let Some(action) = &self.menu_items[self.selected_index].action {
                    action();
                }
            }
        });

        // Update the animation
        self.animation_state.update();

        // Schedule the next update
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        // Draw the background animation
        let painter = ctx.layer_painter(eframe::egui::LayerId::background());
        let rect = ctx.screen_rect();
        self.animation_state.draw_background(&painter, rect);

        let app_state = self.app_state.lock().unwrap().clone();
        logging::debug_info("Rendering app state");

        crate::gui_engine_menu::render_app_state(
            ctx,
            &app_state,
            &self.menu_settings,
            self.selected_index,
            self.is_elevated,
            self.menu_state.format_runtime(),
            self.get_set_app_state_closure(),
        );
        logging::debug_info("App state rendered successfully");
    }
}

pub fn start_gui() {
    logging::debug_info("Starting GUI");

    // Initialize app_state
    app_state::init_module().unwrap_or_else(|err| {
        logging::debug_error(&format!("Failed to initialize app_state module: {}", err));
        panic!("Failed to initialize app_state module: {}", err);
    });
    logging::debug_info("App state module initialized");

    // Initialize s_menu and p_menu
    s_menu::init_module().unwrap_or_else(|err| {
        logging::debug_error(&format!("Failed to initialize s_menu module: {}", err));
        panic!("Failed to initialize s_menu module: {}", err);
    });
    logging::debug_info("s_menu module initialized");
    p_menu::init_module().unwrap_or_else(|err| {
        logging::debug_error(&format!("Failed to initialize p_menu module: {}", err));
        panic!("Failed to initialize p_menu module: {}", err);
    });
    logging::debug_info("p_menu module initialized");

    // Initialize animation module
    crate::gui_engine_animation::init_module().unwrap_or_else(|err| {
        logging::debug_error(&format!("Failed to initialize animation module: {}", err));
        panic!("Failed to initialize animation module: {}", err);
    });
    logging::debug_info("animation module initialized");

    // Load a font that supports the characters (NotoSansJP-Bold.ttf)
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "noto_sans_jp".to_owned(),
        FontData::from_static(include_bytes!("../NotoSansJP-Bold.ttf")).into(), // Convert to Arc<FontData>
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "noto_sans_jp".to_owned());

    logging::debug_info("Font loaded successfully: NotoSansJP-Bold.ttf");

    // Define the menu settings and items from s_menu
    let menu_settings = crate::gui_engine_style::default_settings();
    let app_state = Arc::new(Mutex::new(AppState::SMenu));
    let app_state_clone = app_state.clone();
    let menu_items = s_menu::menu_items(Box::new(move |state| {
        let mut app_state = app_state_clone.lock().unwrap();
        *app_state = state;
    }));
    logging::debug_info("Menu items initialized");

    // Create the app instance
    let app = MyApp {
        app_state,
        menu_settings,
        menu_items,
        selected_index: 0,
        is_elevated: false,
        menu_state: crate::gui_engine_menu::MenuState::new(crate::gui_engine_style::default_settings()),
        animation_state: AnimationState::new(),
    };

    // Define native options
    let native_options = eframe::NativeOptions::default();

    // Run the GUI application
    eframe::run_native(
        "S2O's s2o_net_lib Crate",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(app))
        }),
    );
    logging::debug_info("GUI started successfully");
}
