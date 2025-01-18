use std::process::Command;
use std::{thread, time};
use crossterm::{cursor, event, execute, terminal};
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io::{stdout};

const BYTES_TO_MB: f64 = 1_048_576.0;
const INTERVAL_MILLIS: u64 = 250; // Measure every 250 milliseconds for more accurate readings

fn get_network_stats(adapter: &str) -> Option<(u64, u64)> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(format!(
            "Get-NetAdapterStatistics -Name '{}' | Select-Object ReceivedBytes, SentBytes | ConvertTo-Json",
            adapter
        ))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout_content = String::from_utf8_lossy(&output.stdout);
        let stats: serde_json::Value = serde_json::from_str(&stdout_content).unwrap();
        let received = stats["ReceivedBytes"].as_u64().unwrap_or(0);
        let sent = stats["SentBytes"].as_u64().unwrap_or(0);
        return Some((received, sent));
    }
    None
}

fn get_active_network_adapter() -> Option<String> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(r#"
            Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | Select-Object -ExpandProperty Name | ConvertTo-Json
        "#)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout_content = String::from_utf8_lossy(&output.stdout);
        let adapter: serde_json::Value = serde_json::from_str(&stdout_content).unwrap();
        if let Some(name) = adapter.as_str() {
            return Some(name.to_string());
        }
    }
    None
}

pub fn data_speeds() {
    let adapter = match get_active_network_adapter() {
        Some(name) => name,
        None => {
            println!("No active network adapter found.");
            return;
        }
    };

    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    terminal::enable_raw_mode().unwrap();

    println!("Fetching live data speeds...");
    println!("Press '0' to stop.");

    let mut previous_stats = get_network_stats(&adapter);

    loop {
        execute!(stdout, cursor::MoveTo(0, 2)).unwrap();
        let current_stats = get_network_stats(&adapter);

        if let (Some((prev_received, prev_sent)), Some((curr_received, curr_sent))) = (previous_stats.clone(), current_stats.clone()) {
            let received_diff = curr_received as f64 - prev_received as f64;
            let sent_diff = curr_sent as f64 - prev_sent as f64;

            let received_speed = (received_diff / BYTES_TO_MB) / (INTERVAL_MILLIS as f64 / 1000.0);
            let sent_speed = (sent_diff / BYTES_TO_MB) / (INTERVAL_MILLIS as f64 / 1000.0);

            println!("{:<15} {:<15.4} {:<15.4}", "Real-Time Network", received_speed, sent_speed);

            // Additional debug output
            println!("DEBUG: ReceivedBytesPrev: {} SentBytesPrev: {}", prev_received, prev_sent);
            println!("DEBUG: ReceivedBytesCurr: {} SentBytesCurr: {}", curr_received, curr_sent);
            println!("DEBUG: ReceivedDiff: {} SentDiff: {}", received_diff, sent_diff);
            println!("DEBUG: ReceivedSpeed: {:.4} SentSpeed: {:.4}", received_speed, sent_speed);
        }

        previous_stats = current_stats;

        if event::poll(time::Duration::from_millis(100)).unwrap() {
            if let Event::Key(KeyEvent { code: KeyCode::Char('0'), .. }) = event::read().unwrap() {
                println!("Stopping live data speeds.");
                break;
            }
        }

        thread::sleep(time::Duration::from_millis(INTERVAL_MILLIS));
    }

    terminal::disable_raw_mode().unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
}
