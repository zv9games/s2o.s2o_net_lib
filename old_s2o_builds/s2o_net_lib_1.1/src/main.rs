mod menu; // Declare the menu module
mod admin_menu; // Declare the admin_menu module
mod data_speed; // Declare the data_speed module

use cursive::Cursive;
use cursive::CursiveExt; // Import the CursiveExt trait
use menu::create_menu;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"admin".to_string()) {
        // Run Admin Menu directly if launched with "admin" argument
        let mut siv = Cursive::default();
        admin_menu::create_admin_menu(&mut siv);
        siv.run();
        std::process::exit(0); // Ensure the elevated process ends properly
    } else {
        loop {
            let mut siv = Cursive::default();
            create_menu(&mut siv);
            siv.run();

            // Check the flag file before running the UAC prompt
            let run_uac = std::fs::read_to_string("admin_flag.txt").unwrap_or_else(|_| "exit".to_string()) == "admin";

            if run_uac {
                clear_terminal(); // Clear the terminal screen after elevated process exits

                // Wait for the elevated process to complete
                let status = run_uac_bat();
                if status.is_err() || !status.unwrap() {
                    break; // Exit the loop if the UAC process was not successful
                }
            } else {
                break; // Exit the loop if not prompted for UAC
            }
        }
    }
}

fn run_uac_bat() -> Result<bool, std::io::Error> {
    let status = std::process::Command::new("cmd")
        .args(&["/C", "uac.bat"])
        .status()?;

    Ok(status.success()) // Return true if the UAC was successfully handled
}

fn clear_terminal() {
    // Command to clear the terminal screen based on the operating system
    if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear terminal");
    } else {
        std::process::Command::new("clear")
            .status()
            .expect("Failed to clear terminal");
    }
}