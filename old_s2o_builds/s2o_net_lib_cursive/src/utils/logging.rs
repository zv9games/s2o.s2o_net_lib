// utils/logging.rs
use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::fs::File;

pub fn init_module_logger(module_name: &str) -> Result<(), log::SetLoggerError> {
    let log_file = format!("{}.log", module_name);
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create(log_file).expect(&format!("Failed to create log file for {}", module_name))
    )
}