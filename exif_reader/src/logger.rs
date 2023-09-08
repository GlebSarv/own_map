use env_logger::Env;
use log::{info, error, debug};


pub fn init_logger() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

pub fn log_info(message: &str) {
    info!("{message}"); 
}

pub fn log_debug(message: &str) {
    debug!("{message}");
}

pub fn log_error(message: &str) {
    error!("{message}"); 
}