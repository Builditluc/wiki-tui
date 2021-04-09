use crate::structs::wiki::search::*;
use crate::structs::wiki::article::*;

use crate::config::ApiConfig;

pub struct Wiki<'a> {
    client: reqwest::blocking::Client,
    api_config: &'a ApiConfig,
}

impl<'a> Wiki<'a> {
    pub fn new(config: &'a ApiConfig) -> Self {
        Wiki {
            client: reqwest::blocking::Client::new(),
            api_config: config
        }
    }

    pub fn search(&self, title: &str) -> SearchResponse {
        self.search_articles(title, None)
    }

    pub fn get_article(&self, page_id: &i32) -> ArticleResponse {
        let url = format!("{}?action=query&prop=extracts&pageids={}&formatversion=2&explaintext=true&exsectionformat=plain&format=json", self.api_config.base_url.clone(), page_id);
        println!("{}", &url);
        self.client.get(&url)
            .send()
            .unwrap()
            .json::<ArticleResponse>()
            .unwrap()
    }
    
    fn search_articles(&self, title: &str, continue_code: Option<&ContinueCode>) -> SearchResponse {
        let url = format!("{}?action=query&list=search&srwhat=text&srsearch={}&format=json", self.api_config.base_url.clone(), title);
        if continue_code.is_some() {
            let continue_unwrapped = continue_code.unwrap();
            let continue_code_unwrapped = &continue_unwrapped.continue_code;
            let continue_scroll_offset_unwrapped = continue_unwrapped.scroll_offset;
            let _url = format!("{}?action=query&list=search&srwhat=text&srsearch={}&format=json&continue={}&sroffset={}", self.api_config.base_url.clone(), title, continue_code_unwrapped, continue_scroll_offset_unwrapped);
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
