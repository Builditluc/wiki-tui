use url::Url;

pub mod document;
pub mod languages;
pub mod page;
pub mod parser;
pub mod search;

// TODO: Make Endpoint a real struct
pub type Endpoint = Url;
