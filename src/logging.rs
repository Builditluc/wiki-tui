use crate::config::CONFIG;

pub struct Logger;
impl Logger {
    pub fn initialize() {
        use log::LevelFilter;
        use log4rs::append::file::FileAppender;
        use log4rs::config::{Appender, Config, Logger, Root};

        let wiki_tui = FileAppender::builder()
            .append(false)
            .build(CONFIG.logging.log_dir.as_path())
            .unwrap();

        let default_config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .logger(
                Logger::builder()
                    .appender("wiki_tui")
                    .additive(false)
                    .build("wiki_tui::config", LevelFilter::Info),
            )
            .logger(
                Logger::builder()
                    .appender("wiki_tui")
                    .additive(false)
                    .build("wiki_tui::wiki::parser", LevelFilter::Info),
            )
            .logger(
                Logger::builder()
                    .appender("wiki_tui")
                    .additive(false)
                    .build("wiki_tui::ui::article::view", LevelFilter::Debug),
            )
            .build(
                Root::builder()
                    .appender("wiki_tui")
                    .build(log::LevelFilter::Info),
            )
            .unwrap();

        // try loading the config from a yml file
        log4rs::init_file(&CONFIG.logging.log_config_dir, Default::default()).unwrap_or_else(
            |_| {
                log4rs::init_config(default_config).unwrap();
            },
        );
        log::info!("Successfully initialized the logging system");
    }
}
