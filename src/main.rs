#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

pub mod db;
pub mod tui;
pub mod tests;
pub mod logging;
pub mod traits;

fn main() {
    logging::Logger::new();

    let wiki = db::wiki::WikiSql::new();
}