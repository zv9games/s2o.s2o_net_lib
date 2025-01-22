use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    ProgramMenu,
    DSMenu,
    NSMenu,
    PacketCaptureMenu,
    SecurityMenu,
}

#[derive(Clone, Debug)]
pub struct SharedAppState {
    pub app_state: Arc<Mutex<AppState>>,
}

impl SharedAppState {
    pub fn new(initial_state: AppState) -> Self {
        Self {
            app_state: Arc::new(Mutex::new(initial_state)),
        }
    }

    pub fn set_state(&self, new_state: AppState) {
        let mut state = self.app_state.lock().expect("Failed to lock app state for setting");
        *state = new_state;
    }

    pub fn get_state(&self) -> AppState {
        let state = self.app_state.lock().expect("Failed to lock app state for getting");
        state.clone()
    }
}

