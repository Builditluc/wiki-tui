pub mod metadata;
pub mod properties;

// Temporary
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponse {
    #[serde(rename = "continue")]
    #[serde(default)]
    pub continue_code: ContinueCode,
    pub query: QuerySearch,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContinueCode {
    #[serde(rename = "continue")]
    pub continue_code: String,
    #[serde(rename = "sroffset")]
    pub scroll_offset: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuerySearch {
    pub search: Vec<SearchResult>,
    #[serde(rename = "searchinfo")]
    pub search_info: SearchInfo,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    #[serde(rename = "pageid")]
    pub page_id: i32,
    pub size: i32,
    pub snippet: String,
    pub timestamp: String,
    pub title: String,
    #[serde(rename = "wordcount")]
    pub word_count: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchInfo {
    #[serde(rename = "totalhits")]
    pub total_hits: i32,
}

impl Default for ContinueCode {
    fn default() -> ContinueCode {
        ContinueCode {
            continue_code: "".to_string(),
            scroll_offset: 0,
        }
    }
}
