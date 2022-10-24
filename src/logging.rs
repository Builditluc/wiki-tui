use crate::config::CONFIG;

use log::LevelFilter;
use log4rs::config::{Appender, Config, Root};
use log4rs::Handle;

pub struct Logger {
    handle: Handle,
}
impl Logger {
    pub fn new() -> Self {
        use log4rs::append::console::ConsoleAppender;

        let wiki_tui = ConsoleAppender::builder().build();

        #[cfg(debug_assertions)]
        let log_level = log::LevelFilter::Debug;

        #[cfg(not(debug_assertions))]
        let log_level = log::LevelFilter::Warn;

        let default_config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .build(Root::builder().appender("wiki_tui").build(log_level))
            .unwrap();

        Logger {
            handle: log4rs::init_config(default_config).unwrap(),
        }
    }
    pub fn initialize(&self) {
        use log4rs::append::file::FileAppender;
        use log4rs::encode::pattern::PatternEncoder;

        let wiki_tui = FileAppender::builder()
            .append(false)
            .encoder(Box::new(PatternEncoder::new("{d} {l} {M} - {m}{n}")))
            .build(CONFIG.logging.log_dir.as_path())
            .unwrap();

        // disable logging for specific crates
        let default_config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .logger(log4rs::config::Logger::builder().build("cursive_core", LevelFilter::Off))
            .logger(log4rs::config::Logger::builder().build("html5ever", LevelFilter::Off))
            .logger(
                log4rs::config::Logger::builder()
                    .build("cursive_buffered_backend", LevelFilter::Off),
            )
            .build(
                Root::builder()
                    .appender("wiki_tui")
                    .build(CONFIG.logging.log_level),
            )
            .unwrap();

        self.handle.set_config(default_config);
        info!("successfully initialized the logging system");
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
