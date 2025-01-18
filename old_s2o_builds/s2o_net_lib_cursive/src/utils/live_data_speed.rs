use tokio::time::{interval, Duration};
use cursive::Cursive;
use cursive::views::{Dialog, TextView};
use cursive::view::Nameable;
use winapi::um::pdh::{
    PdhAddEnglishCounterW, PdhCloseQuery, PdhCollectQueryData, PdhGetFormattedCounterValue,
    PdhOpenQueryW, PDH_FMT_DOUBLE, PDH_FMT_COUNTERVALUE,
};
use winapi::shared::ntdef::WCHAR;
use winapi::ctypes::c_void;
use winapi::shared::winerror::ERROR_SUCCESS;
use std::ptr::null_mut;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use log::{info, warn, error};

// Helper function to convert string to WCHAR
fn to_wide_chars(s: &str) -> Vec<WCHAR> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn start_live_data_speed_monitor(siv: &mut Cursive, interface_name: String) {
    // Check if the interface name is valid before proceeding
    if !is_valid_interface(&interface_name) {
        siv.add_layer(Dialog::info(format!("Invalid interface name: {}", interface_name)));
        return;
    }

    let text_view = TextView::new("Fetching data speed...").with_name("speed_display");
    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = stop_flag.clone();

    siv.add_layer(Dialog::around(text_view)
        .title(format!("Live Data Speed for {}", interface_name))
        .button("Stop", move |s| {
            stop_flag_clone.store(true, Ordering::Relaxed);
            s.pop_layer();
        }));

    let cb_sink = siv.cb_sink().clone();

    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            if stop_flag.load(Ordering::Relaxed) {
                info!("Live data speed monitoring stopped for interface: {}", interface_name);
                break;
            }
            interval.tick().await;

            let mut query: *mut c_void = null_mut();
            if let Some(speed) = unsafe {
                if PdhOpenQueryW(null_mut(), 0, &mut query) != ERROR_SUCCESS as i32 {
                    error!("Failed to open PDH query for interface: {}", interface_name);
                    continue;
                }
                if let Some(counter) = get_counter_for_interface(&interface_name, query) {
                    let speed = get_network_speed(counter, query);
                    PdhCloseQuery(query);
                    speed
                } else {
                    PdhCloseQuery(query);
                    None
                }
            } {
                cb_sink.send(Box::new(move |s: &mut Cursive| {
                    if let Some(mut display) = s.find_name::<TextView>("speed_display") {
                        display.set_content(format!("Current Speed: {} Mbps", speed));
                    } else {
                        warn!("Failed to find speed display view.");
                    }
                })).map_err(|e| error!("Failed to send speed update: {}", e)).ok();
            } else {
                warn!("Failed to get network speed for interface: {}", interface_name);
            }
        }
    });
}

// Check if the interface name is valid
fn is_valid_interface(interface_name: &str) -> bool {
    // TODO: Implement real check for interface existence
    // For now, just check if the interface name isn't empty as a placeholder
    !interface_name.is_empty()
}

// Unsafe function to get counter for interface
unsafe fn get_counter_for_interface(interface_name: &str, query: *mut c_void) -> Option<*mut c_void> {
    let mut counter: *mut c_void = null_mut();
    let in_counter_path = format!("\\Network Interface({})\\Bytes Received/sec", interface_name);
    if PdhAddEnglishCounterW(query, to_wide_chars(&in_counter_path).as_ptr(), 0, &mut counter) == ERROR_SUCCESS as i32 {
        Some(counter)
    } else {
        error!("Failed to add counter for interface: {}", interface_name);
        None
    }
}

// Unsafe function to get network speed
unsafe fn get_network_speed(counter: *mut c_void, query: *mut c_void) -> Option<f64> {
    let mut pdh_status: u32 = 0;
    let mut counter_value = PDH_FMT_COUNTERVALUE {
        CStatus: 0,
        u: std::mem::MaybeUninit::zeroed().assume_init(),
    };

    if PdhCollectQueryData(query) == ERROR_SUCCESS as i32 && 
       PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, &mut pdh_status, &mut counter_value) == ERROR_SUCCESS as i32 {
        Some(*(&counter_value.u as *const _ as *const f64))
    } else {
        error!("Failed to collect or format counter data.");
        None
    }
}