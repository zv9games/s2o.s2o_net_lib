use crate::s_menu;
use crate::p_menu;

use crate::app_state::{SharedAppState, AppState};
use crate::logging::{LogBuffers, log_info, log_error};
use std::env;
use std::io::{self, Error as IoError};

pub fn initialize_application(log_buffers: &LogBuffers) -> Result<(), IoError> {
    log_info(log_buffers, "Starting application initialization...");

    log_info(log_buffers, "Setting DLL path...");
    set_dll_path(log_buffers)?;
    log_info(log_buffers, "DLL path set successfully.");

    let shared_state = SharedAppState::new(AppState::ProgramMenu); // Initialize shared application state

    log_info(log_buffers, "Initializing security menu...");
    s_menu::init_s_menu(log_buffers)?;

    log_info(log_buffers, "Checking for elevated privileges...");
    if is_elevated(log_buffers) {
        log_info(log_buffers, "Application is running with elevated privileges.");
        p_menu::program_menu_loop(&shared_state, log_buffers);
    } else {
        log_info(log_buffers, "Application is running without elevated privileges.");
        s_menu::security_menu_loop(log_buffers, &shared_state);
    }

    log_info(log_buffers, "Initialization process completed successfully.");
    Ok(())
}

fn set_dll_path(log_buffers: &LogBuffers) -> Result<(), IoError> {
    log_info(log_buffers, "Determining current directory for DLL path...");
    let current_dir = env::current_dir()?;
    let dll_dir = current_dir.join("src/s2o_dll");

    if !dll_dir.exists() {
        log_error(log_buffers, &format!("DLL directory not found: {:?}", dll_dir));
        return Err(IoError::new(io::ErrorKind::NotFound, "DLL directory not found"));
    }

    log_info(log_buffers, "Updating PATH environment variable...");
    let old_path = env::var("PATH").unwrap_or_default();
    if !old_path.contains(&dll_dir.display().to_string()) {
        env::set_var("PATH", format!("{};{}", dll_dir.display(), old_path));
    }

    log_info(log_buffers, &format!("DLL Directory set to: {:?}", dll_dir));
    log_info(log_buffers, &format!("PATH updated to: {}", env::var("PATH").unwrap()));
    Ok(())
}

fn is_elevated(log_buffers: &LogBuffers) -> bool {
    log_info(log_buffers, "Checking if the application is running with elevated privileges...");
    use winapi::um::processthreadsapi::OpenProcessToken;
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TOKEN_ELEVATION, HANDLE};
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::shared::minwindef::{DWORD, FALSE};

    unsafe {
        let mut token_handle: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), 0x0008 /* TOKEN_QUERY */, &mut token_handle) == FALSE {
            log_error(log_buffers, "Failed to open process token.");
            return false;
        }

        let mut token_elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut return_length: DWORD = 0;
        let result = GetTokenInformation(
            token_handle,
            winapi::um::winnt::TokenElevation,
            &mut token_elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as DWORD,
            &mut return_length,
        );

        if result == FALSE {
            log_error(log_buffers, "Failed to get token information.");
            return false;
        }

        let is_elevated = token_elevation.TokenIsElevated != 0;
        log_info(log_buffers, &format!("Token elevation status: {}", is_elevated));
        is_elevated
    }
}