use anyhow::{Context, Result};
use tracing_subscriber::{
    self, filter::EnvFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
    Layer,
};

use crate::config;

pub fn initialize_logging() -> Result<()> {
    let directory = config::data_dir()?;
    std::fs::create_dir_all(directory.clone())
        .context(format!("{directory:?} could not be created"))?;
    let log_path = directory.join("wiki-tui.log");
    let log_file = std::fs::File::create(log_path)?;
    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(EnvFilter::from_default_env());

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(tui_logger::tracing_subscriber_layer())
        .init();

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
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}
