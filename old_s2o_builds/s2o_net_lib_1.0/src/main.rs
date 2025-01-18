mod bootup;
mod block_all;
mod data_speeds;
mod menu;
mod permissions;
mod admin_menu;
mod ds_menu;
mod pc_menu;
mod analytics_menu;

use menu::main_menu;

fn main() {
    // Load bootup module to check for Network Monitor
    bootup::check_network_monitor();

    // List available network adapters at startup and store the result
    let adapters = bootup::list_network_adapters_with_powershell();
    if adapters.is_empty() {
        println!("No network adapters found. Exiting program.");
        return;
    }

    // Store the adapter list or handle it as needed
    println!("Available network adapters:");
    for adapter in &adapters {
        println!("- {}", adapter);
    }

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "admin" {
        admin_menu::admin_menu();
    } else {
        main_menu();
    }
}
