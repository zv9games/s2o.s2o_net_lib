mod initialization;
mod security_menu;
mod program_menu;
mod menu;
mod ds_menu;
mod ns_menu;
mod pc_menu;
mod packet_sniffer;

use std::env;
use std::path::PathBuf;
use std::process;
use libloading::{Library, Symbol};

fn main() {
    // Get the current directory and join with "s2o_dll"
    let current_dir: PathBuf = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            process::exit(1);
        }
    };
    
    let dll_dir = current_dir.join("s2o_dll");

    // Debug: Print the DLL directory
    println!("DLL Directory: {:?}", dll_dir);

    // Set the library path to the directory containing packet_sniffer.dll
    let old_path = env::var("PATH").unwrap_or_default();
    env::set_var("PATH", format!("{};{}", dll_dir.display(), old_path));

    println!("PATH set to: {}", env::var("PATH").unwrap());

    // Attempt to load the DLL directly to check if it's accessible
    match unsafe { Library::new(dll_dir.join("packet_sniffer.dll")) } {
        Ok(lib) => {
            println!("DLL 'packet_sniffer.dll' loaded successfully.");
        },
        Err(e) => {
            eprintln!("Error loading DLL: {}", e);
            process::exit(1);
        }
    }

    if std::env::args().any(|arg| arg == "--admin") {
        initialization::initialize_cloud_environment();
        program_menu::program_menu_loop();
    } else {
        security_menu::security_menu_loop();
    }
}
