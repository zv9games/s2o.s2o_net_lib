use std::path::Path;
use std::process::{Command, exit};
use std::thread::sleep;
use std::time::Duration;

pub fn check_network_monitor() {
    let path_to_netmon = "C:\\Program Files\\Microsoft Network Monitor 3\\netmon.exe";

    if Path::new(path_to_netmon).exists() {
        println!("Microsoft Network Monitor is installed.");
    } else {
        println!("***IMPORTANT NOTICE:***");
        println!("To proceed, it is necessary to install Microsoft Network Monitor. Please follow these steps meticulously:");

        println!("\n1. **Allow Permission:**");
        println!("   - **Prompt:** You may be asked to allow Microsoft Network Monitor to make changes to your system.");
        println!("   - **Action:** Click **Yes** to continue.");

        println!("\n2. **Installation Steps:**");
        println!("   - **Step 1:** Click **Next** to begin the installation process.");
        println!("   - **Step 2:** Accept the Microsoft Software License Terms and click **Next**.");
        println!("   - **Step 3:** Use Microsoft Update to ensure you have the latest updates, then click **Next**.");
        println!("   - **Step 4:** Select **Complete** as the setup type and click **Next**.");
        println!("   - **Step 5:** Deselect the option to create a shortcut, then click **Install**.");

        println!("\n**Note:** You are in control throughout the installation. Ensure all prompts are followed to complete the setup successfully.");
        println!("\nInstallation will begin in 30 seconds...");

        determine_architecture_and_install();
        loop_check_installation(path_to_netmon);
    }
}

fn determine_architecture_and_install() {
    let arch = get_system_architecture();
    let installer_name = match arch.as_str() {
        "x86_64" => "NM34_x64.exe",
        "ia64" => "NM34_ia64.exe",
        "x86" => "NM34_x86.exe",
        _ => {
            println!("Unsupported architecture: {}", arch);
            exit(1);
        }
    };
    let installer_path = format!("C:\\S2O\\s2o_net_lib\\installers\\{}", installer_name);
    download_and_install_network_monitor(installer_path);
}

fn get_system_architecture() -> String {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-WmiObject Win32_OperatingSystem | Select-Object OSArchitecture")
        .output()
        .expect("Failed to detect system architecture");

    let architecture = String::from_utf8_lossy(&output.stdout);
    if architecture.contains("64-bit") {
        "x86_64".to_string()
    } else if architecture.contains("Itanium-based") {
        "ia64".to_string()
    } else {
        "x86".to_string()
    }
}

fn download_and_install_network_monitor(installer_path: String) {
    if !Path::new(&installer_path).exists() {
        println!("Installer not found at {}. Please ensure the installers are correctly placed.", installer_path);
        exit(1);
    }

    println!("Installer file found at {}.", installer_path);
    println!("Starting installation in 30 seconds...");

    // Countdown timer
    for i in (1..=30).rev() {
        println!("{}...", i);
        sleep(Duration::from_secs(1));
    }

    println!("Starting installation...");

    // Execute the installer with appropriate permissions
    let install_output = Command::new("powershell")
        .arg("-Command")
        .arg(format!("Start-Process -FilePath '{}' -Wait -Verb runAs", installer_path))
        .output()
        .expect("Failed to start the installer");

    println!("Installer output: {:?}", install_output);

    if install_output.status.success() {
        println!("Installer started successfully. Please complete the installation.");
        warn_and_restart();
    } else {
        println!("Failed to start installer. Please try running the installer manually.");
        exit(1);
    }
}

fn warn_and_restart() {
    println!("***IMPORTANT NOTICE:***");
    println!("After installation, you will encounter a User Account Control (UAC) prompt.");
    println!("Please click **Yes** to allow the changes.");

    // Brief delay to read the message
    sleep(Duration::from_secs(10));

    println!("Restarting the program...");

    // Restart the program
    let output = Command::new("cmd")
        .args(&["/C", "start", "C:\\S2O\\s2o_net_lib\\target\\debug\\s2o_net_lib.exe"])
        .output()
        .expect("Failed to restart the program");

    println!("Restart output: {:?}", output);

    if output.status.success() {
        println!("Program restarted successfully.");
    } else {
        println!("Failed to restart the program. Please restart manually.");
    }
    exit(0);
}

fn loop_check_installation(path: &str) {
    println!("Waiting for installation to complete...");
    while !Path::new(path).exists() {
        println!("Checking if Microsoft Network Monitor is installed...");
        sleep(Duration::from_secs(10)); // Check every 10 seconds
    }
    println!("Microsoft Network Monitor installation completed. Resuming program...");
}

// Function to list network adapters using PowerShell
pub fn list_network_adapters_with_powershell() -> Vec<String> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-NetAdapter | Select-Object -Property Name")
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        let adapters = String::from_utf8_lossy(&output.stdout);
        adapters
            .lines()
            .skip(2) // Skip the header lines
            .map(|line| line.trim().to_string())
            .collect()
    } else {
        println!("Failed to list network adapters.");
        println!("Status: {}", output.status);
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        vec![]
    }
}
