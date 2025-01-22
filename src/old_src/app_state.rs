use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    ProgramMenu,
    DSMenu,
    NSMenu,
    PacketCaptureMenu,  // Added PacketCaptureMenu variant
}

#[derive(Clone, Debug)]
pub struct SharedAppState {
    pub app_state: Arc<Mutex<AppState>>,
}

impl SharedAppState {
    // Create a new SharedAppState with the initial state
    pub fn new(initial_state: AppState) -> Self {
        Self {
            app_state: Arc::new(Mutex::new(initial_state)),
        }
    }

    // Set a new state
    pub fn set_state(&self, new_state: AppState) {
        let mut state = self.app_state.lock().unwrap();
        *state = new_state;
    }

    // Get the current state
    pub fn get_state(&self) -> AppState {
        let state = self.app_state.lock().unwrap();
        state.clone()
    }
}
