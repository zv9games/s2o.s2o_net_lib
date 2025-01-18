use cursive::Cursive;
use cursive::views::{Dialog, SelectView};

pub fn create_menu(siv: &mut Cursive) {
    let mut select = SelectView::new()
        .item("Admin Menu", "admin")
        .item("Exit", "exit");

    select.set_on_submit(|siv, item| match item {
        "admin" => {
            // Create a flag file to indicate admin selection
            std::fs::write("admin_flag.txt", "admin").expect("Unable to write flag file");
            siv.quit(); // Exit the current instance to trigger UAC
        }
        "exit" => {
            // Clear any flag to ensure regular exit
            std::fs::write("admin_flag.txt", "exit").expect("Unable to write flag file");
            siv.quit();
        }
        _ => (),
    });

    siv.add_layer(Dialog::around(select).title("Main Menu"));
}