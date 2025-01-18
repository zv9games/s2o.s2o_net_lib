use windows::Win32::System::Threading::{OpenProcessToken, GetCurrentProcess};
use windows::Win32::Security::{TOKEN_QUERY, GetTokenInformation};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use runas::Command;
use std::env;

#[repr(C)]
struct TokenElevation {
    token_is_elevated: u32,
}

pub fn is_user_admin() -> bool {
    unsafe {
        let process_handle = GetCurrentProcess();
        let mut token_handle: HANDLE = HANDLE::default();
        if OpenProcessToken(
            process_handle,
            TOKEN_QUERY,
            &mut token_handle,
        ).is_ok() {
            let mut elevation = TokenElevation { token_is_elevated: 0 };
            let mut ret_len = 0;
            if GetTokenInformation(
                token_handle,
                windows::Win32::Security::TokenElevation,
                Some(&mut elevation as *mut _ as *mut _),
                std::mem::size_of::<TokenElevation>() as u32,
                &mut ret_len,
            ).is_ok() {
                let _ = CloseHandle(token_handle);
                return elevation.token_is_elevated != 0;
            }
            let _ = CloseHandle(token_handle);
        }
        false
    }
}

pub fn elevate_process() {
    println!("Attempting to elevate permissions...");
    println!("Please accept the User Account Control (UAC) prompt to continue.");

    let status = Command::new(env::current_exe().unwrap())
        .arg("admin")
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("Restarting with elevated privileges...");
            std::process::exit(0);
        }
        Ok(status) => println!("Failed to elevate permissions. Status code: {}", status),
        Err(e) => println!("Failed to elevate permissions. Error: {}", e),
    }
}

pub fn graceful_shutdown() {
    println!("Cleaning up resources before exiting...");
    std::process::exit(0);
}
