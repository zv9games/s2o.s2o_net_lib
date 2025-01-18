use crate::ds_menu::data_speed_menu;
use crate::block_all::{block_all_traffic, unblock_all_traffic};
use crate::pc_menu::packet_capture_menu;
use std::io::{Write};

pub fn admin_menu() {
    loop {
        // Clear the screen
        clear_screen();

        println!("Welcome to s2o_net_lib administrative menu.");
        println!("Press 1 for data speeds.");
        println!("Press 2 for packet scanning.");
        println!("Press 3 to block all traffic.");
        println!("Press 4 to unblock all traffic.");
        println!("Press 9 to exit.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => data_speed_menu(),
            "2" => packet_capture_menu(),
            "3" => block_all_traffic(),
            "4" => unblock_all_traffic(),
            "9" => {
                println!("Exiting administrative menu...");
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
