use crate::config::LoggingConfig;

pub struct Logger;

impl Logger {
    pub fn new(logging_config: &LoggingConfig) {
        use simple_logging::log_to_file;

        log_to_file(logging_config.log_output.clone(), logging_config.log_level);
        log::info!("Successfully initialized the Logging Module");
    }
}
