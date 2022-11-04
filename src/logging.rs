use crate::config::CONFIG;

use anyhow::{Context, Result};
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
                    .build(log::LevelFilter::Info),
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

        // disable logging if not enabled in the config
        if !CONFIG.logging.enabled {
            return Ok(());
        }

        let wiki_tui = FileAppender::builder()
            .append(false)
            .build(CONFIG.logging.log_dir.as_path())
            .context("failed building the FileAppender")?;

        let default_config = Config::builder()
            .appender(Appender::builder().build("wiki_tui", Box::new(wiki_tui)))
            .build(
                Root::builder()
                    .appender("wiki_tui")
                    .build(CONFIG.logging.log_level),
            )
            .context("failed building the default config")?;

        self.handle.set_config(default_config);
        log::info!("successfully initialized the logging system");
        Ok(())
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}
