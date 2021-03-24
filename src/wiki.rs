use serde::*;
use ini::{Properties, Ini};

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    #[serde(rename="continue")]
    continue_code: ContinueCode,
    query: QuerySearchResponse
}

#[derive(Deserialize, Debug)]
pub struct ContinueCode {
    #[serde(rename="continue")]
    continue_code: String,
    #[serde(rename="sroffset")]
    scroll_offset: i32
}

#[derive(Deserialize, Debug)]
struct QuerySearchResponse {
    search: Vec<SearchResult>,
    #[serde(rename="searchinfo")]
    search_info: SearchInfo 
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct ArticleResponse {
    #[serde(rename="parse")]
    parsed_content: ParseArticle
}

#[derive(Deserialize, Debug)]
struct ParseArticle {
    text: ParseArticleText 
}

#[derive(Deserialize, Debug)]
struct ParseArticleText {
    #[serde(rename="*")]
    content: String
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

    pub fn search(&self, title: &str) -> SearchResponse {
        self.search_articles(title, None)
    }

    pub fn get_article(&self, page_id: &i32) -> ArticleResponse {
        let base_url = &self.api_config
            .get("BASE_URL");
        let url = format!("{}?action=parse&prop=text&pageid={}&format=json", base_url.unwrap(), page_id);
        println!("{}", &url);
        self.client.get(&url)
            .send()
            .unwrap()
            .json::<ArticleResponse>()
            .unwrap()
    }
    
    fn search_articles(&self, title: &str, continue_code: Option<&ContinueCode>) -> SearchResponse {
        let base_url = &self.api_config
            .get("BASE_URL");
        let url = format!("{}?action=query&list=search&srwhat=text&srsearch={}&format=json", base_url.unwrap(), title);
        if (continue_code.is_some()) {
            let continue_unwrapped = continue_code.unwrap();
            let continue_code_unwrapped = &continue_unwrapped.continue_code;
            let continue_scroll_offset_unwrapped = continue_unwrapped.scroll_offset;
            let url = format!("{}?action=query&list=search&srwhat=text&srsearch={}&format=json&continue={}&sroffset={}", base_url.unwrap(), title, continue_code_unwrapped, continue_scroll_offset_unwrapped);
        }

        self.client.get(&url)
            .send()
            .unwrap()
            .json::<SearchResponse>()
            .unwrap()
    }
    pub fn continue_search(&self, title: &str, continue_code: &ContinueCode) -> SearchResponse {
        self.search_articles(title, Some(continue_code))
    }
}
