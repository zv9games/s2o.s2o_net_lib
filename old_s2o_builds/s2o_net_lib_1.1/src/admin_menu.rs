use crate::data_speed; // Ensure this matches the actual file name
use cursive::Cursive;
use cursive::views::{Dialog, SelectView, TextView};

pub fn create_admin_menu(siv: &mut Cursive) {
    let mut select = SelectView::new()
        .item("Data Speed Menu", "data_speed")
        .item("Packet Capture Menu", "packet_capture")
        .item("Firewall Menu", "firewall")
        .item("Exit", "exit");

    select.set_on_submit(|siv, item| match item {
        "data_speed" => data_speed::create_data_speed_menu(siv),
        "packet_capture" => show_packet_capture_menu(siv),
        "firewall" => show_firewall_menu(siv),
        "exit" => {
            siv.quit(); // Ensure this cleanly exits the admin process
            std::process::exit(0); // Exit the elevated process entirely
        }
        _ => (),
    });

    siv.add_layer(Dialog::around(select).title("Admin Menu").button("Back", |siv| {siv.pop_layer(); }));
}

fn show_packet_capture_menu(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(TextView::new("Packet Capture Functions"))
            .title("Packet Capture Menu")
            .button("Back", |siv| {siv.pop_layer(); })
    );
}

fn show_firewall_menu(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(TextView::new("Firewall Functions"))
            .title("Firewall Menu")
            .button("Back", |siv| {siv.pop_layer(); })
    );
}