use cursive::view::Nameable; // Import the Nameable trait
use cursive::Cursive;
use cursive::views::{Dialog, SelectView, TextView};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use winapi::um::iphlpapi::GetIfTable;
use winapi::shared::ifmib::{MIB_IFTABLE, MIB_IFROW};
use winapi::shared::minwindef::ULONG;

pub fn create_data_speed_menu(siv: &mut Cursive) {
    let mut select = SelectView::new()
        .item("Start Monitoring", "start")
        .item("Stop Monitoring", "stop")
        .item("Set Refresh Interval", "set_interval")
        .item("Log Speeds to File", "log")
        .item("Show Historical Data", "historical")
        .item("Show Average Speed", "average")
        .item("Set Speed Alert", "alert")
        .item("Back", "back");

    select.set_on_submit(|siv, item| match item {
        "start" => start_monitoring(siv),
        "stop" => stop_monitoring(siv),
        "set_interval" => set_refresh_interval(siv),
        "log" => log_speeds(siv),
        "historical" => show_historical_data(siv),
        "average" => show_average_speed(siv),
        "alert" => set_speed_alert(siv),
        "back" => {siv.pop_layer();},
        _ => (),
    });

    siv.add_layer(Dialog::around(select).title("Data Speed Menu"));
}

fn start_monitoring(siv: &mut Cursive) {
    let speed_data = Arc::new(Mutex::new((0.0, 0.0))); // Store speed data in a thread-safe way
    let siv_clone = siv.cb_sink().clone();
    thread::spawn(move || {
        loop {
            let speeds = get_current_speeds(); // Function to get current speeds
            {
                let mut data = speed_data.lock().unwrap();
                *data = speeds;
            }
            let speeds_clone = speeds.clone(); // Clone speeds to move into the closure
            siv_clone.send(Box::new(move |s| {
                s.call_on_name("speed_view", move |view: &mut TextView| {
                    view.set_content(format!("Download: {} Mbps, Upload: {} Mbps", speeds_clone.0, speeds_clone.1));
                });
            })).unwrap();
            thread::sleep(Duration::from_millis(200)); // Update every 0.2 seconds
        }
    });

    siv.add_layer(Dialog::around(TextView::new("Monitoring Speed...").with_name("speed_view"))
        .title("Live Network Speed")
        .button("Stop", |s| {s.pop_layer();}));
}

fn stop_monitoring(_siv: &mut Cursive) {
    // Implement logic to stop monitoring
}

fn set_refresh_interval(_siv: &mut Cursive) {
    // Implement logic to set refresh interval
}

fn log_speeds(_siv: &mut Cursive) {
    // Implement logic to log speeds to a file
}

fn show_historical_data(_siv: &mut Cursive) {
    // Implement logic to show historical data
}

fn show_average_speed(_siv: &mut Cursive) {
    // Implement logic to show average speed
}

fn set_speed_alert(_siv: &mut Cursive) {
    // Implement logic to set speed alert
}

fn get_current_speeds() -> (f64, f64) {
    // Use winapi to get current download and upload speeds
    unsafe {
        let mut table_size: ULONG = 0;
        GetIfTable(std::ptr::null_mut(), &mut table_size, 0);
        let mut table: Vec<u8> = vec![0; table_size as usize];
        let table_ptr = table.as_mut_ptr() as *mut MIB_IFTABLE;

        if GetIfTable(table_ptr, &mut table_size, 0) == 0 {
            let table = &*table_ptr;
            for i in 0..table.dwNumEntries {
                let row = table.table[i as usize];
                let download_speed = (row.dwInOctets * 8) as f64 / (1024.0 * 1024.0); // Convert to Mbps
                let upload_speed = (row.dwOutOctets * 8) as f64 / (1024.0 * 1024.0); // Convert to Mbps

                // Return the first active interface speeds
                return (download_speed, upload_speed);
            }
        }
    }

    (0.0, 0.0) // Default to 0 if no data is available
}