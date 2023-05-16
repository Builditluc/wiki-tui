use crate::config::CONFIG;

use anyhow::{Context, Result};
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
        let default_config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .build(
                #[cfg(debug_assertions)]
                Root::builder()
                    .appender("wiki_tui")
                    .build(log::LevelFilter::Debug),
                #[cfg(not(debug_assertions))]
                Root::builder()
                    .appender("wiki_tui")
                    .build(log::LevelFilter::Off),
            )
            .unwrap();

        Logger {
            handle: log4rs::init_config(default_config).unwrap(),
        }
    }
    pub fn initialize(&self) -> Result<()> {
        use log4rs::append::file::FileAppender;
        use log4rs::encode::pattern::PatternEncoder;

        // disable logging if not enabled in the config
        if !CONFIG.logging.enabled {
            return Ok(());
        }

        let wiki_tui = FileAppender::builder()
            .append(false)
            .encoder(Box::new(PatternEncoder::new("{d} {l} {M} - {m}{n}")))
            .build(CONFIG.logging.log_path.as_path())
            .context("failed building the FileAppender")?;

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
            .context("failed building the default config")?;

        self.handle.set_config(default_config);
        info!("successfully initialized the logging system");
        Ok(())
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
