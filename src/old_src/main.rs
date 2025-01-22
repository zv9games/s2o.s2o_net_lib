mod security_menu;
mod program_menu;
mod menu;
mod ds_menu;
mod ns_menu;
mod pc_menu;
mod packet_sniffer;
mod logging;
mod app_state;

use std::env;
use std::path::PathBuf;
use std::process;
use log::{info, error};

fn main() {
    // Initialize logging
    logging::init_logging();
    info!("Logging initialized successfully.");

    // Set the DLL path before any other initializations
    if let Err(e) = set_dll_path() {
        error!("Failed to set DLL path: {}", e);
        process::exit(1);
    }
    info!("DLL path set.");

    info!("Starting main...");

    if is_elevated() {
        info!("Running with elevated privileges...");
        // Run elevated logic
        program_menu::program_menu_loop();
    } else {
        info!("Running without elevated privileges...");
        // Not elevated, run the security menu
        security_menu::security_menu_loop();
    }
}

pub fn set_dll_path() -> Result<(), std::io::Error> {
    let current_dir = env::current_dir()?;
    let dll_dir = current_dir.join("src/s2o_dll");

    if !dll_dir.exists() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "DLL directory not found"));
    }

    let old_path = env::var("PATH").unwrap_or_default();
    if !old_path.contains(&dll_dir.display().to_string()) {
        env::set_var("PATH", format!("{};{}", dll_dir.display(), old_path));
    }

    // Log the DLL path and the current PATH
    info!("DLL Directory: {:?}", dll_dir);
    info!("PATH set to: {}", env::var("PATH").unwrap());
    Ok(())
}

pub fn is_elevated() -> bool {
    use winapi::um::processthreadsapi::OpenProcessToken;
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TOKEN_ELEVATION, HANDLE};
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::shared::minwindef::{DWORD, FALSE};

    unsafe {
        let mut token_handle: HANDLE = std::ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), 0x0008 /* TOKEN_QUERY */, &mut token_handle) == FALSE {
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
            return false;
        }

        token_elevation.TokenIsElevated != 0
    }
}