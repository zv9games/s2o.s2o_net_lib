use crate::permissions::{is_user_admin, elevate_process, graceful_shutdown};
use crate::admin_menu::admin_menu;

pub fn main_menu() {
    loop {
        println!("Welcome to s2o_net_lib main menu.");
        println!("***IMPORTANT NOTICE:***");
        println!("You may encounter a User Account Control (UAC) prompt during operation.");
        println!("Please click **Yes** to allow the changes.");
        println!("Press 1 to enter administrative mode.");
        println!("Press 9 to exit.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                if !is_user_admin() {
                    elevate_process();
                } else {
                    admin_menu();
                }
            },
            "9" => {
                println!("Exiting the program...");
                graceful_shutdown();
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}
