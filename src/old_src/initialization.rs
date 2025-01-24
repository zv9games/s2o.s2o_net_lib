use crate::security_menu;
use crate::program_menu;
use crate::app_state::{SharedAppState, AppState};
use crate::logging::{LogBuffers, log_info, log_error};
use std::env;
use std::io::{self, Error as IoError};
use std::path::PathBuf;

pub fn initialize_application(log_buffers: &LogBuffers) -> Result<(), IoError> {
    log_info(log_buffers, "Initiating application setup...");

    configure_dll_path(log_buffers)?;
    log_info(log_buffers, "DLL path configuration successful.");

    let shared_state = SharedAppState::new(AppState::ProgramMenu);

    if check_elevation(log_buffers) {
        log_info(log_buffers, "Running with elevated permissions.");
        program_menu::run_menu(&shared_state, log_buffers);
    } else {
        log_info(log_buffers, "Running without elevated permissions.");
        security_menu::run_menu(log_buffers, &shared_state);
    }

    log_info(log_buffers, "Application setup completed.");
    Ok(())
}

/// Configures the DLL path in the environment for runtime.
fn configure_dll_path(log_buffers: &LogBuffers) -> Result<(), IoError> {
    log_info(log_buffers, "Locating DLL directory...");

    let dll_path = get_dll_path()?;
    ensure_path_exists(&dll_path, log_buffers)?;

    update_environment_path(&dll_path, log_buffers)?;
    Ok(())
}

/// Attempts to find and return the path to the DLL directory.
fn get_dll_path() -> Result<PathBuf, IoError> {
    let mut dll_path = env::current_dir()?;
    dll_path.push("src/s2o_dll");
    dll_path.canonicalize().map_err(|_| IoError::new(io::ErrorKind::NotFound, "DLL directory not found"))
}

/// Ensures the DLL path exists, logging an error if it doesn't.
fn ensure_path_exists(path: &PathBuf, log_buffers: &LogBuffers) -> Result<(), IoError> {
    if !path.exists() {
        log_error(log_buffers, &format!("DLL directory does not exist: {:?}", path));
        return Err(IoError::new(io::ErrorKind::NotFound, "DLL directory not found"));
    }
    Ok(())
}

/// Updates the PATH environment variable to include the DLL directory.
fn update_environment_path(path: &PathBuf, log_buffers: &LogBuffers) -> Result<(), IoError> {
    let old_path = env::var("PATH").unwrap_or_default();
    if !old_path.contains(path.to_str().ok_or_else(|| IoError::new(io::ErrorKind::InvalidData, "Invalid path"))?) {
        env::set_var("PATH", format!("{};{}", path.display(), old_path));
    }
    log_info(log_buffers, &format!("PATH updated to include: {}", path.display()));
    Ok(())
}

/// Checks if the current process has elevated (administrative) privileges.
fn check_elevation(log_buffers: &LogBuffers) -> bool {
    log_info(log_buffers, "Verifying administrative privileges...");

    unsafe {
        use winapi::um::processthreadsapi::OpenProcessToken;
        use winapi::um::securitybaseapi::GetTokenInformation;
        use winapi::um::winnt::{TOKEN_ELEVATION, HANDLE};
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::shared::minwindef::{DWORD, FALSE};

        let mut token_handle: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), 0x0008 /* TOKEN_QUERY */, &mut token_handle) == FALSE {
            log_error(log_buffers, "Unable to access process token for elevation check.");
            return false;
        }

        let mut token_elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length: DWORD = 0;
        if GetTokenInformation(
            token_handle,
            winapi::um::winnt::TokenElevation,
            &mut token_elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as DWORD,
            &mut return_length,
        ) == FALSE {
            log_error(log_buffers, "Failed to retrieve token elevation information.");
            return false;
        }

        let elevated = token_elevation.TokenIsElevated != 0;
        log_info(log_buffers, &format!("Elevation status: {}", elevated));
        elevated
    }
}