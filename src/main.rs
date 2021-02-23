#[macro_use] extern crate log;


pub mod tui;
pub mod tests;
pub mod logging;

fn main() {
    logging::Logger::new();
}