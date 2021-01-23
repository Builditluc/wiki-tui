#[macro_use] extern crate log;
#[macro_use] extern crate diesel;

use crate::traits::wiki::Fetchable;

pub mod db;
pub mod tui;
pub mod tests;
pub mod logging;
pub mod traits;

fn main() {
    logging::Logger::new();

    let wiki = db::wiki::WikiSql::new();
    wiki.get_all_articles();
}