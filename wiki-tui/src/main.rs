use anyhow::Result;
use tracing::info;
use wiki_tui::{logging::initialize_logging, panic_handler::initialize_panic_handler};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging();
    initialize_panic_handler();

    println!("Hello, world!");
    Ok(())
}
