mod builder;
mod metadata;
mod properties;
mod sort_order;

mod compiled_search;
mod info;
mod result;

pub type Search = compiled_search::Search;
#[allow(dead_code)]
pub type SearchInfo = info::SearchInfo;
pub type SearchResult = result::SearchResult;

pub type SearchBuilder = builder::SearchBuilder;
pub type SearchMetadata = metadata::SearchMetadata;
pub type SearchSortOrder = sort_order::SearchSortOrder;
pub type SearchProperties = properties::SearchProperties;
