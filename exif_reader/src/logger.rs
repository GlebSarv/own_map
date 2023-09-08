use env_logger::Env;
use log::{info, error, debug};

// init_logger is a function that initializes the logger based on environment variables.
// It sets the log level to "info" by default.
pub fn init_logger() {
    // Initialize the logger using the environment's default log level or "info" if not specified.
    env_logger::init_from_env(Env::default().default_filter_or("info"));
}

// log_info is a function that logs an informational message.
// Parameters:
// - message: A string representing the informational message to be logged.
pub fn log_info(message: &str) {
    // Log the provided informational message.
    info!("{message}"); 
}

// log_debug is a function that logs a debug message.
// Parameters:
// - message: A string representing the debug message to be logged.
pub fn log_debug(message: &str) {
    // Log the provided debug message.
    debug!("{message}");
}

// log_error is a function that logs an error message.
// Parameters:
// - message: A string representing the error message to be logged.
pub fn log_error(message: &str) {
    // Log the provided error message.
    error!("{message}"); 
}
