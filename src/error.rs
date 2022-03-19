use panic_message::panic_info_message;
use std::collections::HashMap;
use std::env;
use std::panic::{set_hook, PanicInfo};
use uuid::Uuid;

use crate::config::CONFIG;

pub const PANIC: &str = "
Well, this is embarrassing...
%NAME% had a problem and crashed spectacularly. To help us diagnose the problem, you can send us a crash report.
We have generated a report file in the current working directory. Submit an issue with the subjet of \"%NAME% Crash Report\"
and describe what you did before the crash. Also include the report as an attachment.

Github: %GITHUB%
";

pub fn print_panic() {
    let data = {
        let mut data = HashMap::new();
        data.insert("%NAME%", env!("CARGO_PKG_NAME"));
        data.insert("%GITHUB%", env!("CARGO_PKG_REPOSITORY"));
        data
    };

    let text = {
        let mut text = String::from(PANIC);
        for (k, v) in &data {
            text = text.replace(k, v);
        }
        text
    };

    println!("{}", text);
}

pub fn create_hook<F>(f: F)
where
    F: 'static + Fn(Option<std::path::PathBuf>, String) + Send + Sync,
{
    if std::env::var("RUST_BACKTRACE").is_ok() {
        return;
    }

    set_hook(Box::new(move |info: &PanicInfo| {
        let path = env::current_dir()
            .unwrap_or_else(|_| env::temp_dir())
            .join(format!(
                "crash_report-{}.log",
                Uuid::new_v4().to_hyphenated()
            ));
        log::error!("panic occurred, crash log is at: {}", path.display());

        let mut payload = String::new();

        let os = if cfg!(target_os = "windows") {
            "Windows"
        } else if cfg!(target_os = "linux") {
            "Linux"
        } else if cfg!(target_os = "macos") {
            "Mac OS"
        } else {
            "Unknown"
        };

        payload.push_str(&format!("Name: {}\n", env!("CARGO_PKG_NAME")));
        payload.push_str(&format!("Version: {}\n", env!("CARGO_PKG_VERSION")));
        payload.push_str(&format!("Operating System: {}\n", os));

        payload.push_str(&format!("Cause: {}.\n", panic_info_message(info)));

        match info.location() {
            Some(location) => payload.push_str(&format!(
                "Panic occurred in file '{}' at line {}\n",
                location.file(),
                location.line()
            )),
            None => payload.push_str("Panic location unknown.\n"),
        }

        let logs = match std::fs::read_to_string(&CONFIG.logging.log_dir) {
            Ok(logs) => logs,
            Err(_) => "No logs available.".to_string(),
        };

        payload.push_str(&format!("\n\nLogs: \n{}", logs));

        f(Some(path), payload);
    }))
}
