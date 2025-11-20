use crate::terminal::Tui;
use anyhow::Result;
use tracing::error;

pub fn initialize_panic_handler() -> Result<()> {
    #[allow(unused_variables)]
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .display_location_section(true)
        .display_env_section(true)
        .into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |panic_info| {
        match Tui::new() {
            Ok(mut tui) => {
                if let Err(error) = tui.exit() {
                    error!("unable to exit terminal: {error:?}");
                }
            }
            Err(error) => error!("unable to exit terminal {error:?}"),
        }

        #[cfg(not(debug_assertions))]
        {
            eprintln!("{}", panic_hook.panic_report(panic_info));
            use human_panic::{handle_dump, print_msg, Metadata};
            let meta = Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                .authors(env!("CARGO_PKG_AUTHORS").replace(':', ", "))
                .homepage(env!("CARGO_PKG_HOMEPAGE"));

            let file_path = handle_dump(&meta, panic_info);
            print_msg(file_path, &meta)
                .expect("human-panic: printing error message to console failed");
        }

        #[cfg(debug_assertions)]
        {
            better_panic::Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .verbosity(better_panic::Verbosity::Full)
                .create_panic_handler()(panic_info);
        }

        std::process::exit(libc::EXIT_FAILURE);
    }));

    Ok(())
}
