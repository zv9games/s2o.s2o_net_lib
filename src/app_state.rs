use crate::logging;

pub fn init_module() -> Result<(), String> {
    // Placeholder for actual initialization logic
    let initialization_passed = true;

    if initialization_passed {
        logging::debug_info("app_state module is online");
        Ok(())
    } else {
        Err("app_state module initialization failed".to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    SMenu,
    PMenu,
    PCMenu,
    NSMenu,
    DSMenu,
}

pub trait SetAppState: Fn(AppState) {}
impl<T> SetAppState for T where T: Fn(AppState) {}
