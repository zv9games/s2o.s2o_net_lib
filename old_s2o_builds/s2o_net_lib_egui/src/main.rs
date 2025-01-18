mod initialization;
mod security_menu;
mod program_menu;
mod menu;
mod ds_menu;
mod ns_menu;
mod pc_menu;


fn main() {
    if std::env::args().any(|arg| arg == "--admin") {
        initialization::initialize_cloud_environment();
        program_menu::program_menu_loop();
    } else {
        security_menu::security_menu_loop();
    }
}
