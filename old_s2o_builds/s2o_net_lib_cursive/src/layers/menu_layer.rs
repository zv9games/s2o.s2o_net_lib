use cursive::views::{Dialog, LinearLayout, SelectView, TextView, ScrollView};
use cursive::view::{Nameable, Resizable, ViewWrapper, View};
use cursive::wrap_impl;
use cursive::Cursive;
use cursive::event::{Event, Key, EventResult};
use std::sync::{Arc, Mutex};
use chrono;

struct SafeClosure<T>(T);

unsafe impl<T> Send for SafeClosure<T> {}
unsafe impl<T> Sync for SafeClosure<T> {}

impl<T> SafeClosure<T> {
    fn new(f: T) -> Self {
        SafeClosure(f)
    }
}

type ClosureType = Box<dyn Fn(&mut Cursive)>;

struct MenuView {
    select_view: SelectView<SafeClosure<ClosureType>>,
    debug_info: Arc<Mutex<String>>,
}

impl MenuView {
    fn new(debug_info: Arc<Mutex<String>>) -> Self {
        let mut select_view = SelectView::new()
            .autojump()
            .item("1. Admin Menu", SafeClosure::new(Box::new(|s: &mut Cursive| {
                s.add_layer(
                    Dialog::around(TextView::new("Admin Menu not implemented yet!"))
                        .button("Back", |s| {
                            s.pop_layer();
                        })
                        .title("Admin Menu")
                );
            }) as ClosureType))
            .item("2. Exit", SafeClosure::new(Box::new(|s: &mut Cursive| {
                s.quit();
            }) as ClosureType));

        select_view.set_on_submit(|s, item: &SafeClosure<ClosureType>| {
            let callback = &item.0;
            callback(s);
        });

        MenuView { select_view, debug_info }
    }
}

impl ViewWrapper for MenuView {
    wrap_impl!(self.select_view: SelectView<SafeClosure<ClosureType>>);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Up) => {
                self.update_log("Up Arrow key pressed.");
                self.select_view.on_event(event)
            },
            Event::Key(Key::Down) => {
                self.update_log("Down Arrow key pressed.");
                self.select_view.on_event(event)
            },
            _ => self.select_view.on_event(event),
        }
    }
}

impl MenuView {
    fn update_log(&mut self, message: &str) {
        let mut info = self.debug_info.lock().unwrap();
        let current_content = info.clone();
        let updated_content = format!("{}: {}\n{}", chrono::Local::now().format("%H:%M:%S"), message, current_content);
        info.clear();
        info.push_str(&updated_content);
        info.insert_str(0, &updated_content); // Prepend new content at the top
    }
}

pub fn setup_ui(siv: &mut Cursive, debug_info: &Arc<Mutex<String>>, _debug_error: &Arc<Mutex<String>>) {
    let debug_info_view = Dialog::around(ScrollView::new(TextView::new("").with_name("debug_info")));
    let debug_error_view = Dialog::around(ScrollView::new(TextView::new("").with_name("debug_error")));

    let menu = MenuView::new(Arc::clone(debug_info)).with_name("menu");

    siv.add_layer(
        LinearLayout::horizontal()
            .child(debug_info_view.full_screen())
            .child(Dialog::around(menu).full_screen())
            .child(debug_error_view.full_screen())
    );

    // Update the debug_info view when the log changes
    let debug_info_clone = Arc::clone(debug_info);
    siv.add_global_callback(Event::Refresh, move |s| {
        s.call_on_name("debug_info", |view: &mut TextView| {
            view.set_content(debug_info_clone.lock().unwrap().clone());
        });
    });
}
