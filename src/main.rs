use anyhow::Result;
use wiki_tui::{app::App, logging::initialize_logging, panic_handler::initialize_panic_handler};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;
    initialize_panic_handler();

    let mut app = App::new();
    app.run().await?;

    Ok(())
}
