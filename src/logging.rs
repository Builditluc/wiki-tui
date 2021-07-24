pub struct Logger;
impl Logger {
    pub fn new() {
        use log::LevelFilter;
        use log4rs::append::file::FileAppender;
        use log4rs::config::{Appender, Config, Logger, Root};

        let wiki_tui = FileAppender::builder()
            .append(false)
            .build("wiki_tui.log")
            .unwrap();

        let config = Config::builder()
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
                    .build("wiki_tui::ui::article::view", LevelFilter::Info),
            )
            .build(
                Root::builder()
                    .appender("wiki_tui")
                    .build(log::LevelFilter::Info),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();
        log::info!("Successfully initialized the logging system");
    }
}
