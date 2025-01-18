use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{self, Write};
use crate::analytics_menu::analytics_menu;

static DEFAULT_CAPTURE_DURATION: &str = "10 seconds";

pub fn packet_capture_menu() {
    let mut capture_duration = DEFAULT_CAPTURE_DURATION.to_string();

    loop {
        // Clear the screen
        clear_screen();

        // Check for existing export file and print its location if found
        let export_file_path = "C:\\Temp\\NetworkCaptureParsed.cap";
        if Path::new(export_file_path).exists() {
            println!("An existing export file was found at: {}", export_file_path);
        }

        println!("Packet Capture Menu:");
        println!("1. Start Packet Capture");
        println!("2. Stop Packet Capture");
        println!("3. Export Captured Data");
        println!("4. Set Capture Duration");
        println!("5. View Network Adapters");
        println!("6. Analytics Menu");
        println!("9. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => start_packet_capture(&capture_duration),
            2 => stop_packet_capture(),
            3 => export_capture(),
            4 => capture_duration = set_capture_duration(),
            5 => view_network_adapters(),
            6 => analytics_menu(),
            9 => {
                println!("Exiting packet capture menu...");
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn start_packet_capture(capture_duration: &str) {
    println!("Starting packet scan using Microsoft Network Monitor...");

    let nmcap_path = "C:\\Program Files\\Microsoft Network Monitor 3\\NMCap.exe";
    if !Path::new(nmcap_path).exists() {
        println!("NMCap.exe not found. Please ensure Microsoft Network Monitor is installed and NMCap.exe is accessible.");
        return;
    }

    let network_adapter = match find_active_adapter() {
        Some(adapter) => adapter,
        None => {
            println!("No active network adapter found.");
            return;
        }
    };

    println!("Using network adapter: {}", network_adapter);

    let mut capture_command = Command::new(nmcap_path)
        .args(&[
            "/network",
            &network_adapter,
            "/capture",
            "/file",
            "C:\\Temp\\NetworkCapture.cap",
            "/StopWhen",
            "/TimeAfter",
            capture_duration,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start capture command");

    let status = capture_command.wait().expect("Failed to wait on capture process");
    println!("Packet capture process exited with status: {}", status);
}

fn set_capture_duration() -> String {
    println!("Enter capture duration value: ");
    let mut duration_value = String::new();
    io::stdin().read_line(&mut duration_value).unwrap();
    let duration_value = duration_value.trim();

    println!("Select duration unit:");
    println!("1. Seconds");
    println!("2. Minutes");
    println!("3. Hours");
    let mut unit_choice = String::new();
    io::stdin().read_line(&mut unit_choice).unwrap();
    let unit_choice = unit_choice.trim().parse::<u32>().unwrap_or(0);

    let duration_unit = match unit_choice {
        1 => "seconds",
        2 => "minutes",
        3 => "hours",
        _ => {
            println!("Invalid choice, defaulting to seconds.");
            "seconds"
        }
    };

    let new_duration = format!("{} {}", duration_value, duration_unit);
    println!("Capture duration set to {}", new_duration);

    new_duration
}

fn stop_packet_capture() {
    println!("Stopping packet capture...");
    // Logic to stop the packet capture
}

fn export_capture() {
    let capture_file_path = "C:\\Temp\\NetworkCapture.cap";
    if !Path::new(capture_file_path).exists() {
        println!("Capture file not found. Please ensure the file exists and try again.");
        return;
    }

    let nmcap_path = "C:\\Program Files\\Microsoft Network Monitor 3\\NMCap.exe";
    if !Path::new(nmcap_path).exists() {
        println!("NMCap.exe not found. Please ensure Microsoft Network Monitor is installed and NMCap.exe is accessible.");
        return;
    }

    let export_command = Command::new(nmcap_path)
        .args(&[
            "/InputCapture",
            capture_file_path,
            "/ReassembleCapture",
            "/File",
            "C:\\Temp\\NetworkCaptureParsed.cap",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute export command");

    if export_command.status.success() {
        println!("Network data export completed successfully.");
        println!("Results saved to C:\\Temp\\NetworkCaptureParsed.cap");
    } else {
        println!("Network data export failed.");
        println!("Status: {}", export_command.status);
        println!("Stdout: {}", String::from_utf8_lossy(&export_command.stdout));
        println!("Stderr: {}", String::from_utf8_lossy(&export_command.stderr));
    }
    
    // Log the file location
    let output_file_path = "C:\\Temp\\NetworkCaptureParsed.cap";
    println!("The exported file is located at: {}", output_file_path);
}

fn view_network_adapters() {
    let adapters = list_network_adapters();
    println!("Available network adapters:");
    for adapter in adapters {
        println!("- {}", adapter);
    }
}

fn find_active_adapter() -> Option<String> {
    list_network_adapters().into_iter().next()
}

fn list_network_adapters() -> Vec<String> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | Select-Object -Property Name")
        .output()
        .expect("Failed to execute PowerShell command");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .skip(2) // Skip the header lines
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && line != "----")
            .collect()
    } else {
        println!("Failed to list network adapters.");
        println!("Status: {}", output.status);
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        vec![]
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}
