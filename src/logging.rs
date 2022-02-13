use crate::config::CONFIG;

use log4rs::config::{Appender, Config, Root};
use log4rs::Handle;

pub struct Logger {
    handle: Handle,
}
impl Logger {
    pub fn new() -> Self {
        use log4rs::append::console::ConsoleAppender;

        let wiki_tui = ConsoleAppender::builder().build();

        let default_config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .build(
                Root::builder()
                    .appender("wiki_tui")
                    .build(log::LevelFilter::Info),
            )
            .unwrap();

        Logger {
            handle: log4rs::init_config(default_config).unwrap(),
        }
    }
    pub fn initialize(&self) {
        use log4rs::append::file::FileAppender;

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

        self.handle.set_config(default_config);
        log::info!("successfully initialized the logging system");
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
