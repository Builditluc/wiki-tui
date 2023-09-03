use crate::terminal::Tui;
use better_panic::Settings;
use tracing::error;

pub fn initialize_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        match Tui::new() {
            Ok(tui) => {
                if let Err(error) = tui.exit() {
                    error!("unable to exit terminal: {error:?}");
                }
            }
            Err(error) => error!("unable to exit terminal {error:?}"),
        }
        Settings::auto()
            .most_recent_first(false)
            .lineno_suffix(true)
            .create_panic_handler()(panic_info);
        std::process::exit(libc::EXIT_FAILURE);
    }))
}
