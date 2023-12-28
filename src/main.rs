use anyhow::Result;
use wiki_tui::{
    app::App, cli::match_cli, logging::initialize_logging, panic_handler::initialize_panic_handler,
};

#[tokio::main]
async fn main() -> Result<()> {
    let actions = match_cli();

    initialize_logging()?;
    initialize_panic_handler()?;

    let mut app = App::new();
    app.run(actions).await?;

    Ok(())
}
