mod layers;

use layers::{menu_layer, debug_info, debug_errors};
use cursive::views::TextView;
use cursive::Cursive;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

fn main() {
    let mut siv = cursive::default();
    let debug_info = Arc::new(Mutex::new(String::new()));
    let debug_error = Arc::new(Mutex::new(String::new()));
    let info_updated = Arc::new(AtomicBool::new(false));
    let error_updated = Arc::new(AtomicBool::new(false));

    menu_layer::setup_ui(&mut siv, &debug_info, &debug_error);
    debug_info::setup_controls(&mut siv, debug_info.clone(), info_updated.clone());
    debug_errors::start_error_thread(&error_updated, &debug_error);

    let info_updated_clone = Arc::clone(&info_updated);
    let error_updated_clone = Arc::clone(&error_updated);
    let debug_info_clone = Arc::clone(&debug_info);
    let debug_error_clone = Arc::clone(&debug_error);
    let siv_clone = siv.cb_sink().clone();

    thread::spawn(move || {
        ui_update_thread(info_updated_clone, error_updated_clone, debug_info_clone, debug_error_clone, siv_clone);
    });

    siv.run();
}

fn ui_update_thread(info_updated: Arc<AtomicBool>, error_updated: Arc<AtomicBool>, debug_info: Arc<Mutex<String>>, debug_error: Arc<Mutex<String>>, siv_clone: cursive::reexports::crossbeam_channel::Sender<Box<dyn FnOnce(&mut Cursive) + Send>>) {
    loop {
        let mut info_needs_update = false;
        let mut error_needs_update = false;

        if info_updated.load(Ordering::Relaxed) {
            info_needs_update = true;
            info_updated.store(false, Ordering::Relaxed);
        }

        if error_updated.load(Ordering::Relaxed) {
            error_needs_update = true;
            error_updated.store(false, Ordering::Relaxed);
        }

        if info_needs_update || error_needs_update {
            let debug_info = Arc::clone(&debug_info);
            let debug_error = Arc::clone(&debug_error);
            siv_clone.send(Box::new(move |s: &mut Cursive| {
                if info_needs_update {
                    update_display_info(s, debug_info);
                }
                if error_needs_update {
                    update_display_error(s, debug_error);
                }
            })).unwrap();
        }

        thread::sleep(Duration::from_millis(33)); // Check for updates every 33 milliseconds
    }
}

fn update_display_info(s: &mut Cursive, debug_info: Arc<Mutex<String>>) {
    if let Some(mut view) = s.find_name::<TextView>("debug_info") {
        view.set_content(debug_info.lock().unwrap().clone());
    }
}

fn update_display_error(s: &mut Cursive, debug_error: Arc<Mutex<String>>) {
    if let Some(mut view) = s.find_name::<TextView>("debug_error") {
        view.set_content(debug_error.lock().unwrap().clone());
    }
}
