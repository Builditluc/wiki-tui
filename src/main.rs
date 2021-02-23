#[macro_use] extern crate log;
extern crate ini;

pub mod tui;
pub mod tests;
pub mod logging;
pub mod wiki;

fn main() {
    logging::Logger::new();
}