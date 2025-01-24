use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub enum AppState {
    Main,
    Setup,
    Network,
    Capture,
    Security,
}

pub struct AppStateController {
    state: Arc<Mutex<AppState>>,
}

impl AppStateController {
    /// Creates a new `AppStateController` with the provided initial state.
    pub fn new(initial_state: AppState) -> Self {
        AppStateController {
            state: Arc::new(Mutex::new(initial_state)),
        }
    }

    /// Updates the current application state.
    pub fn set_state(&self, new_state: AppState) {
        let mut state = self.state.lock().unwrap_or_else(|_| panic!("Failed to lock state for update"));
        *state = new_state;
    }

    /// Retrieves the current application state.
    pub fn get_state(&self) -> AppState {
        let state = self.state.lock().unwrap_or_else(|_| panic!("Failed to lock state for retrieval"));
        state.clone()
    }
}