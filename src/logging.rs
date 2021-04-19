use crate::config::LoggingConfig;
use anyhow::*;

pub struct Logger;

impl Logger {
    pub fn new(logging_config: &LoggingConfig) {
        use simple_logging::log_to_file;

        let result = log_to_file(logging_config.log_output.clone(), logging_config.log_level).context("Failed to initialize the logging");
        match result {
            Ok(_) => log::info!("Successfully initialized the Logging Module"),
            Err(error) => panic!("{:?}", error),
        }
    }
}
