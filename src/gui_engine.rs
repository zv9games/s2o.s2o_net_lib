use crate::logging;
use crate::admin_check;
use crate::s_menu;
use crate::p_menu;
use crate::app_state::AppState;
use crate::gui_engine_animation::{AnimationState, speedometer};
use crate::gui_engine_menu::MenuItem;
use crate::gui_engine_style::MenuSettings;
use eframe::egui::{self, Context, FontDefinitions, FontData, FontFamily};
use std::sync::{Arc, Mutex};
use std::io::{self, Write};

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
    #[allow(dead_code)]
    fn set_app_state(&mut self, new_state: AppState) {
        logging::debug_info(&format!("Setting app state to: {:?}", new_state));
        let mut app_state = self.app_state.lock().unwrap();
        *app_state = new_state;
        self.menu_items = match new_state {
            AppState::SMenu => s_menu::menu_items(self.get_set_app_state_closure(), &self.menu_settings),
            AppState::PMenu => p_menu::menu_items(self.get_set_app_state_closure(), &self.menu_settings),
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
        ctx.input(|input| handle_input(input, self));

        // Update the animation
        self.animation_state.update();

        // Schedule the next update
        ctx.request_repaint_after(std::time::Duration::from_millis(16)); // Approx. 60 FPS

        // Draw the background animation
        let painter = ctx.layer_painter(eframe::egui::LayerId::background());
        let rect = ctx.screen_rect();
        self.animation_state.draw_background(&painter, rect);

        let app_state = self.app_state.lock().unwrap().clone();
        logging::debug_info("Rendering app state");

        crate::gui_engine_menu::render_app_state(
            ctx,
            &app_state,
            &self.menu_state,
            self.is_elevated,
            self.menu_state.format_runtime(),
            self.get_set_app_state_closure(),
        );
        logging::debug_info("App state rendered successfully");
    }
}

pub fn start_gui() {
    logging::debug_info("Starting GUI");

    // Add a pause and wait for any key to be pressed
    println!("Starting GUI application pause");
    println!("Press any key to continue...");
    io::stdout().flush().unwrap(); // Ensure the message is printed before waiting
    let _ = io::stdin().read_line(&mut String::new()); // Wait for any key press

    // Load menu settings
    let menu_settings = load_menu_settings();
    
    // Load app state and menu items
    let (app_state, menu_items) = load_app_state();

    // Create the app instance
    let app = create_app_instance(app_state, menu_settings, menu_items);

    // Define native options
    let native_options = eframe::NativeOptions::default();

    // Load fonts
    let fonts = load_fonts().unwrap(); // Ensure fonts are loaded

    // Run the GUI application
    eframe::run_native(
        "S2O's s2o_net_lib Crate",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(fonts); // Set fonts in GUI context
            Ok(Box::new(app))
        }),
    ).unwrap_or_else(|err| {
        logging::debug_error(&format!("Failed to start GUI: {}", err));
        panic!("Failed to start GUI: {}", err);
    });
    logging::debug_info("GUI started successfully");
}



pub fn load_fonts() -> Result<FontDefinitions, String> {
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
    Ok(fonts)
}


fn load_menu_settings() -> MenuSettings {
    crate::gui_engine_style::default_settings()
}

fn load_app_state() -> (Arc<Mutex<AppState>>, Vec<MenuItem>) {
    let initial_app_state = if admin_check::is_admin_user() {
        logging::debug_info("User is elevated. Starting with PMenu.");
        AppState::PMenu
    } else {
        logging::debug_info("User is not elevated. Starting with SMenu.");
        AppState::SMenu
    };

    let app_state = Arc::new(Mutex::new(initial_app_state));
    let app_state_clone = app_state.clone(); // Clone to use within closures

    let menu_settings = load_menu_settings();
    let menu_items = match initial_app_state {
        AppState::SMenu => s_menu::menu_items(Box::new(move |state| {
            let mut app_state = app_state_clone.lock().unwrap();
            *app_state = state;
        }), &menu_settings),
        AppState::PMenu => p_menu::menu_items(Box::new(move |state| {
            let mut app_state = app_state_clone.lock().unwrap();
            *app_state = state;
        }), &menu_settings),
        _ => vec![],
    };

    (app_state, menu_items)
}

fn create_app_instance(
    app_state: Arc<Mutex<AppState>>,
    menu_settings: MenuSettings,
    menu_items: Vec<MenuItem>
) -> MyApp {
    let initial_app_state = *app_state.lock().unwrap();
    
    let mut animation_state = AnimationState::new();

    // Set the speed using a two-digit number (e.g., 50)
    let speed = 255; // Set this value as needed
    let speed_factor = speedometer(speed);
    animation_state.set_speed_factor(speed_factor);

    MyApp {
        app_state,
        menu_settings: menu_settings.clone(), // Use menu_settings here
        menu_items,
        selected_index: 0,
        is_elevated: initial_app_state == AppState::PMenu,
        menu_state: crate::gui_engine_menu::MenuState::new(menu_settings),
        animation_state,
    }
}

fn handle_input(input: &egui::InputState, app: &mut MyApp) {
    if input.key_pressed(egui::Key::ArrowDown) {
        logging::debug_info("ArrowDown key pressed");
        app.selected_index = (app.selected_index + 1) % app.menu_items.len();
        app.menu_state.set_selected(app.selected_index); // Update menu_state
    }
    if input.key_pressed(egui::Key::ArrowUp) {
        logging::debug_info("ArrowUp key pressed");
        if app.selected_index == 0 {
            app.selected_index = app.menu_items.len() - 1;
        } else {
            app.selected_index -= 1;
        }
        app.menu_state.set_selected(app.selected_index); // Update menu_state
    }
    if input.key_pressed(egui::Key::Enter) {
        logging::debug_info("Enter key pressed");
        if let Some(action) = &app.menu_items[app.selected_index].action {
            action();
        }
    }
}
