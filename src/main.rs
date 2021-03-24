#[macro_use] extern crate log;
extern crate ini;

pub mod tui;
pub mod tests;
pub mod logging;
pub mod wiki;

fn main() {
    logging::Logger::new();
    let wiki_struct = wiki::Wiki::new();
    let search_response = wiki_struct.search("meeting");
    println!("{:?}", search_response)
}
