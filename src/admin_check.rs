use crate::logging;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use winapi::um::handleapi::CloseHandle;
use std::ptr::null_mut;

pub fn init_module() -> Result<(), String> {
    // Placeholder for actual initialization logic
    let initialization_passed = true;

    if initialization_passed {
        logging::debug_info("admin_check module is online");
        Ok(())
    } else {
        Err("admin_check module initialization failed".to_string())
    }
}

#[allow(dead_code)]
pub fn is_admin_user() -> bool {
    unsafe {
        let mut token: winapi::um::winnt::HANDLE = null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION {
            TokenIsElevated: 0,
        };
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
        let result = winapi::um::securitybaseapi::GetTokenInformation(
            token,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut size,
        );

        CloseHandle(token);

        result != 0 && elevation.TokenIsElevated != 0
    }
}