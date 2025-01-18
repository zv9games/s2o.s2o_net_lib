use std::process::Command;

pub fn block_all_traffic() {
    Command::new("powershell")
        .arg("-Command")
        .arg("New-NetFirewallRule -DisplayName 'Block All Inbound Traffic' -Direction Inbound -Action Block")
        .output()
        .expect("Failed to execute command");

    Command::new("powershell")
        .arg("-Command")
        .arg("New-NetFirewallRule -DisplayName 'Block All Outbound Traffic' -Direction Outbound -Action Block")
        .output()
        .expect("Failed to execute command");

    println!("All inbound and outbound traffic is now blocked.");
}

pub fn unblock_all_traffic() {
    Command::new("powershell")
        .arg("-Command")
        .arg("Remove-NetFirewallRule -DisplayName 'Block All Inbound Traffic'")
        .output()
        .expect("Failed to execute command");

    Command::new("powershell")
        .arg("-Command")
        .arg("Remove-NetFirewallRule -DisplayName 'Block All Outbound Traffic'")
        .output()
        .expect("Failed to execute command");

    println!("All inbound and outbound traffic is now unblocked.");
}
