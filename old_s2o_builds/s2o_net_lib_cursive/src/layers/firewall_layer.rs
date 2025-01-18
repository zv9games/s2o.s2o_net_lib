use cursive::Cursive;
use cursive::views::{Dialog, SelectView};
use std::process::Command;
use crate::layers::network_layer::network_menu;

pub fn admin_menu_with_pid(siv: &mut Cursive, pid: String) {
    // Store pid in user_data for access in other layers
    siv.set_user_data(pid.clone());

    let mut select = SelectView::new()
        .item("Network Menu", "network_menu")
        .item("Firewall Menu", "firewall_menu")
        .item("Exit to Main Menu", "exit");

    select.set_on_submit(move |siv, item| {
        match item {
            "network_menu" => {
                // Both network_menu and firewall_menu will add their own layer
                network_menu(siv);
            },
            "firewall_menu" => {
                
            },
            "exit" => {
                // Logic to terminate the session
                match Command::new("cmd")
                    .args(&["/C", "taskkill", "/F", "/T", "/PID", &pid])
                    .spawn() {
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(status) if status.success() => {
                                // Process terminated successfully, but double-check
                                match Command::new("cmd").args(&["/C", "tasklist", "/FI", "PID eq", &pid]).output() {
                                    Ok(output) if !String::from_utf8_lossy(&output.stdout).contains(&pid) => {
                                        // The PID is no longer in the task list, assume it's terminated
                                        while siv.screen().len() > 1 {
                                            siv.pop_layer();
                                        }
                                    },
                                    _ => {
                                        siv.add_layer(Dialog::info("Admin session did not close as expected.").dismiss_button("OK"));
                                    }
                                }
                            },
                            Ok(_) => {
                                siv.add_layer(Dialog::info("Failed to terminate admin session cleanly.").dismiss_button("OK"));
                            },
                            Err(e) => {
                                siv.add_layer(Dialog::info(format!("Error waiting for process termination: {}", e)).dismiss_button("OK"));
                            }
                        }
                    },
                    Err(e) => {
                        siv.add_layer(Dialog::info(format!("Failed to close admin session: {}", e)).dismiss_button("OK"));
                    }
                }
                // Optionally quit if this is the last step before actual program exit
                // siv.quit();
            },
            _ => {},
        }
    });

    siv.add_layer(Dialog::around(select).title("Admin Menu"));
}