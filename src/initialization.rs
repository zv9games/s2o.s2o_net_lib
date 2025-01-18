use std::env;
use std::path::PathBuf;
use crate::security_menu;
use crate::program_menu;
use crate::ds_menu;
use crate::ns_menu;
use crate::pc_menu;
use crate::packet_sniffer;

pub fn initialize_cloud_environment() {
    // Get the current directory and join with "s2o_dll"
    let current_dir: PathBuf = env::current_dir().unwrap();
    let dll_dir = current_dir.join("s2o_dll");  // Make sure this matches the directory where packet_sniffer.dll is located
    
    // Set the library path to the directory containing packet_sniffer.dll
    env::set_var("PATH", format!("{};{}", dll_dir.display(), env::var("PATH").unwrap_or_default()));
    
    println!("Cloud environment initialized. PATH set to: {}", env::var("PATH").unwrap());
}

fn main() {
    if std::env::args().any(|arg| arg == "--admin") {
        initialize_cloud_environment();  // Call the initialization function
        program_menu::program_menu_loop();
    } else {
        security_menu::security_menu_loop();
    }
}
