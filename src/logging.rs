pub struct Logger;

impl Logger {
    pub fn new() {
        use simple_logging::log_to_file;

        let log_level = std::env::var("LOG_LEVEL").unwrap();
        let max_log_level: log::LevelFilter = match &log_level[..] {
            "OFF" => log::LevelFilter::Off,
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "WARN" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,

            _ => log::LevelFilter::Off
        };

        dotenv::dotenv().ok();
        log_to_file(std::env::var("LOG_OUTPUT").unwrap(), max_log_level);

        log::info!("Successfully initialized the Logging Module");
    }
}