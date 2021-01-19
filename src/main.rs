
pub mod db;
pub mod tui;
pub mod tests;

fn main() {
    let api = db::api::Api::new();
    api.fetch_all_articles();
}