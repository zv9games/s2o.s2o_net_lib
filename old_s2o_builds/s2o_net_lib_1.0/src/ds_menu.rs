use std::io::Write;
use crate::data_speeds::data_speeds;

pub fn data_speed_menu() {
    loop {
        // Clear the screen
        clear_screen();

        println!("Data Speed Menu:");
        println!("1. Test Data Speeds");
        println!("9. Exit");
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => data_speeds(),
            9 => {
                println!("Exiting data speed menu...");
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
