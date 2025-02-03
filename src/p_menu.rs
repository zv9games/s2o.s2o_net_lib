use crate::logging;
use crate::app_state::AppState;
use crate::gui_engine;
use crate::gui_engine_menu::MenuItem;

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
pub fn menu_items<S: Fn(AppState) + Clone + 'static>(set_app_state: S) -> Vec<MenuItem> {
    vec![
        MenuItem {
            label: "PC Menu".to_string(),
            action: Some(Box::new({
                let set_app_state = set_app_state.clone();
                move || set_app_state(AppState::PCMenu)
            })),
        },
        MenuItem {
            label: "DS Menu".to_string(),
            action: Some(Box::new({
                let set_app_state = set_app_state.clone();
                move || set_app_state(AppState::DSMenu)
            })),
        },
        MenuItem {
            label: "NS Menu".to_string(),
            action: Some(Box::new({
                let set_app_state = set_app_state.clone();
                move || set_app_state(AppState::NSMenu)
            })),
        },
        MenuItem {
            label: "Exit".to_string(),
            action: Some(Box::new(|| std::process::exit(0))),
        },
    ]
}
