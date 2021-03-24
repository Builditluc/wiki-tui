use serde::*;
use ini::{Properties, Ini};

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SearchResponse {
    continue_code: ContinueCode,
    query: QuerySearchResponse
}

#[derive(Deserialize, Debug)]
struct ContinueCode {
    #[serde(rename="continue")]
    continue_code: String,
    #[serde(rename="sroffset")]
    scroll_offset: i32
}

#[derive(Deserialize, Debug)]
struct QuerySearchResponse {
    search: Vec<SearchResult>,
    search_info: SearchInfo 
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct SearchResult {
    #[serde(rename="pageid")]
    page_id: i32,
    size: i32,
    snippet: String,
    timestamp: String,
    title: String,
    #[serde(rename="wordcount")]
    word_count: i32
}

#[derive(Deserialize, Debug)]
struct SearchInfo {
    #[serde(rename="totalhits")]
    total_hits: i32
}

pub struct Wiki {
    client: reqwest::blocking::Client,
    api_config: Properties
}

impl Wiki {
    pub fn new() -> Self {
        let config = Ini::load_from_file("config.ini").unwrap();
        let config = config.clone();
        Wiki {
            client: reqwest::blocking::Client::new(),
            api_config: config.section(Some("Api")).unwrap().clone()
        }
    }

    fn search(&self, title: &str) {
        let base_url = &self.api_config
            .get("BASE_URL");
        let url = format!("{}?action=query&list=searcg&srwhat=text&srsearch={}&format=json", base_url.unwrap(), title);

        let result = self.client.get(&url)
            .send()
            .unwrap();
    }
}
