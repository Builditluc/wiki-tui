use serde::Serialize;
use wiki_tui::parse_languages;

// This macro parses all the available wikipedia languages into an enum
parse_languages!("./data/languages.json");
