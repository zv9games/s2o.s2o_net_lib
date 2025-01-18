use cursive::event::{Event, Key};
use cursive::Cursive;
use std::sync::{Arc, Mutex, atomic::AtomicBool};



pub fn setup_controls(siv: &mut Cursive, debug_info: Arc<Mutex<String>>, info_updated: Arc<AtomicBool>) {
    let debug_info_clone = debug_info.clone();
    let info_updated_clone = info_updated.clone();

    siv.add_global_callback(Event::CtrlChar('c'), move |s| {
        log_to_info(&debug_info_clone, &info_updated_clone, "Ctrl+C pressed, exiting program.");
        s.quit();
    });

    siv.set_fps(20); // Increased to 20 FPS for more frequent updates

    let debug_info_clone = debug_info.clone();
    let info_updated_clone = info_updated.clone();
    siv.add_global_callback('q', move |s| {
        log_to_info(&debug_info_clone, &info_updated_clone, "Q key pressed, quitting.");
        s.quit();
    });

    // Capture necessary key events
    let debug_info_clone = debug_info.clone();
    let info_updated_clone = info_updated.clone();
    siv.add_global_callback(Event::Key(Key::Enter), move |_s| {
        log_to_info(&debug_info_clone, &info_updated_clone, "Enter key pressed.");
    });

    let debug_info_clone = debug_info.clone();
    let info_updated_clone = info_updated.clone();
    siv.add_global_callback(Event::Key(Key::Left), move |_s| {
        log_to_info(&debug_info_clone, &info_updated_clone, "Left Arrow key pressed.");
    });

    let debug_info_clone = debug_info.clone();
    let info_updated_clone = info_updated.clone();
    siv.add_global_callback(Event::Key(Key::Right), move |_s| {
        log_to_info(&debug_info_clone, &info_updated_clone, "Right Arrow key pressed.");
    });

    let debug_info_clone = debug_info.clone();
    let info_updated_clone = info_updated.clone();
    siv.add_global_callback(Event::Key(Key::Up), move |_s| {
        log_to_info(&debug_info_clone, &info_updated_clone, "Up Arrow key pressed.");
    });

    let debug_info_clone = debug_info.clone();
    let info_updated_clone = info_updated.clone();
    siv.add_global_callback(Event::Key(Key::Down), move |_s| {
        log_to_info(&debug_info_clone, &info_updated_clone, "Down Arrow key pressed.");
    });
}

pub fn log_to_info(debug_info: &Arc<Mutex<String>>, info_updated: &Arc<AtomicBool>, message: &str) {
    let mut info = debug_info.lock().unwrap();
    info.push_str(&format!("{}: {}\n", chrono::Local::now().format("%H:%M:%S"), message));
    info_updated.store(true, std::sync::atomic::Ordering::Relaxed);
}
