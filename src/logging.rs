pub struct Logger;

impl Logger {
    pub fn new() {
        use simple_logging::log_to_file;
        use ini::Ini;

        let log_conf = Ini::load_from_file("config.ini")
            .unwrap()
            .section(Some("Logging"))
            .unwrap();

        let log_level = log_conf.get("LOG_LEVEL").unwrap();
        let max_log_level: log::LevelFilter = match &log_level[..] {
            "OFF" => log::LevelFilter::Off,
            "TRACE" => log::LevelFilter::Trace,
            "DEBUG" => log::LevelFilter::Debug,
            "INFO" => log::LevelFilter::Info,
            "WARN" => log::LevelFilter::Warn,
            "ERROR" => log::LevelFilter::Error,

            _ => log::LevelFilter::Off
        };

        log_to_file(log_conf.get("LOG_OUTPUT").unwrap(), max_log_level);
        log::info!("Successfully initialized the Logging Module");
    }
}