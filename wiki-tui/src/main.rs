use anyhow::Result;
use wiki_tui::panic_handler::initialize_panic_handler;

#[tokio::main]
async fn main() -> Result<()> {
    initialize_panic_handler();

    println!("Hello, world!");
    Ok(())
}
