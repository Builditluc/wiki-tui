#[macro_use] extern crate log;

pub mod db;
pub mod tui;
pub mod tests;
pub mod logging;
pub mod traits;

fn main() {
    logging::Logger::new();
}