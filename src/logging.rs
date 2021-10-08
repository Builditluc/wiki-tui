use crate::config::CONFIG;

pub struct Logger;
impl Logger {
    pub fn initialize() {
        use log4rs::append::file::FileAppender;
        use log4rs::config::{Appender, Config, Root};

        let wiki_tui = FileAppender::builder()
            .append(false)
            .build(CONFIG.logging.log_dir.as_path())
            .unwrap();

        let default_config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .build(
                Root::builder()
                    .appender("wiki_tui")
                    .build(CONFIG.logging.log_level),
            )
            .unwrap();

        // try loading the config from a yml file
        log4rs::init_config(default_config).unwrap();
        log::info!("Successfully initialized the logging system");
    }
}
