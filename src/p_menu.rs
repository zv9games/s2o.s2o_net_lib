use crate::logging;
use crate::app_state::AppState;
use crate::gui_engine_menu::MenuItem;
use crate::gui_engine_style::MenuSettings;  // Import MenuSettings

pub fn init_module() -> Result<(), String> {
    // Placeholder for actual initialization logic
    let initialization_passed = true;

    if initialization_passed {
        logging::debug_info("p_menu module is online");
        Ok(())
    } else {
        Err("p_menu module initialization failed".to_string())
    }
}

// Placeholder menu items for p_menu
pub fn menu_items<S: Fn(AppState) + Clone + 'static>(
    set_app_state: S,
    menu_settings: &MenuSettings,  // Add menu_settings parameter
) -> Vec<MenuItem> {
    vec![
        MenuItem {
            label: menu_settings.apply_label("PC Menu", false).text().to_string(),  // Use menu_settings and convert to String
            action: Some(Box::new({
                let set_app_state = set_app_state.clone();
                move || set_app_state(AppState::PCMenu)
            })),
        },
        MenuItem {
            label: menu_settings.apply_label("DS Menu", false).text().to_string(),  // Use menu_settings and convert to String
            action: Some(Box::new({
                let set_app_state = set_app_state.clone();
                move || set_app_state(AppState::DSMenu)
            })),
        },
        MenuItem {
            label: menu_settings.apply_label("NS Menu", false).text().to_string(),  // Use menu_settings and convert to String
            action: Some(Box::new({
                let set_app_state = set_app_state.clone();
                move || set_app_state(AppState::NSMenu)
            })),
        },
        MenuItem {
            label: "Exit".to_string(),  // No need to use menu_settings for a simple label
            action: Some(Box::new(|| std::process::exit(0))),
        },
    ]
}
