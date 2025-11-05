use anyhow::{Context, Result};
use std::sync::Mutex;
use tracing::level_filters::LevelFilter;
use tracing_log::AsLog;
use tracing_subscriber::{self, prelude::*, EnvFilter};

use crate::config::{self, load_logging_config};

const LOG_ENV: &str = "WIKI_TUI_LOG";

pub fn initialize_logging(level: Option<LevelFilter>) -> Result<()> {
    let logging_config = match load_logging_config() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{:?}", error.context("failed loading the logging config"));
            Default::default()
        }
    };

    if !logging_config.enabled {
        return Ok(());
    }

    let directory = config::cache_dir()?;
    std::fs::create_dir_all(directory.clone())
        .context(format!("{directory:?} could not be created"))?;
    let log_path = directory.join("wiki-tui.log");
    let log_file = std::fs::File::create(log_path)?;
    let log_writer = Mutex::new(log_file);

    let env_filter = EnvFilter::builder()
        .with_default_directive(logging_config.level.into())
        .with_env_var(LOG_ENV)
        .from_env()?;
    let level = match level {
        Some(level) => level,
        None => env_filter.max_level_hint().unwrap_or(logging_config.level),
    };

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_writer)
        .with_target(false)
        .with_ansi(false)
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(tui_logger::TuiTracingSubscriberLayer)
        .init();

    tui_logger::set_default_level(level.as_log());

    Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr, $ex:expr) => {
        $crate::trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        $crate::trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        $crate::trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}
