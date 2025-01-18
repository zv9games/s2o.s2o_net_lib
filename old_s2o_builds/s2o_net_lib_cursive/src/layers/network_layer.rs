// src/layers/network_layer.rs
use cursive::Cursive;
use cursive::views::{LinearLayout, Dialog, SelectView};

pub fn network_menu(siv: &mut Cursive) {
    let mut layout = LinearLayout::vertical();

    let mut sub_menu = SelectView::new()
        .item("Data Speed Menu", "data_speed")
        .item("Packet Capture Menu", "packet_capture")
        .item("Back", "back");

    sub_menu.set_on_submit(move |siv, item| {
        match item {
            "data_speed" => {
                // TODO: Implement data speed menu
                siv.add_layer(Dialog::text("Data Speed Menu - Coming Soon!").button("Back", |s| {s.pop_layer();}));
            },
            "packet_capture" => {
                // TODO: Implement packet capture menu
                siv.add_layer(Dialog::text("Packet Capture Menu - Coming Soon!").button("Back", |s| {s.pop_layer();}));
            },
            "back" => {
                siv.pop_layer(); // Return to admin menu
            },
            _ => {},
        }
    });

    layout.add_child(sub_menu); // Removed fixed_size

    // Add everything in a dialog without setting a fixed size
    siv.add_layer(Dialog::around(layout).title("Network Menu"));
}