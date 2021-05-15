use crate::config::CONFIG;
use anyhow::*;
pub mod article;
pub mod parser;
pub mod search;

pub struct WikiApi {
    client: reqwest::blocking::Client,
    parser: Box<dyn parser::Parser>,
}

impl WikiApi {
    pub fn new() -> Self {
        log::info!("[wiki::WikiApi::new] Creating a instance of Wiki");
        let default_parser = parser::Default {};
        WikiApi {
            client: reqwest::blocking::Client::new(),
            parser: Box::new(default_parser),
        }
    }

    pub fn search(&self, title: &str) -> search::SearchResponse {
        self.search_articles(title, None)
    }
    pub fn continue_search(
        &self,
        title: &str,
        continue_code: &search::ContinueCode,
    ) -> search::SearchResponse {
        self.search_articles(title, Some(continue_code))
    }

    fn search_articles(
        &self,
        title: &str,
        continue_code: Option<&search::ContinueCode>,
    ) -> search::SearchResponse {
        // creating the url for the request
        let mut url = format!(
            "{}/w/api.php?action=query&list=search&srwhat=text&srsearch={}&format=json",
            CONFIG.api_config.base_url.clone(),
            title
        );

        if continue_code.is_some() {
            let continue_unwrapped = continue_code.unwrap();
            let continue_code_unwrapped = &continue_unwrapped.continue_code;
            let continue_scroll_offset_unwrapped = continue_unwrapped.scroll_offset;
            url = format!("{}/w/api.php?action=query&list=search&srwhat=text&srsearch={}&format=json&continue={}&sroffset={}", CONFIG.api_config.base_url.clone(), title, continue_code_unwrapped, continue_scroll_offset_unwrapped);
        }

        // making the request
        let response = match self
            .client
            .get(&url)
            .send()
            .context("Failed to send the request")
        {
            Ok(response) => {
                log::info!("[wiki::WikiApi::search_articles] Successfully sent the request");
                response
            }
            Err(error) => {
                log::error!("[wiki::WikiApi::search_articles], {:?}", error);
                panic!("Something happened, please check your logs")
            }
        };

        // serializing the response
        let serde_result = response
            .json::<search::SearchResponse>()
            .context("Failed serializing the response");

        match serde_result {
            Ok(result) => {
                log::info!("[wiki::WikiApi::search_articles] Successfully serialized the response");
                result
            }
            Err(error) => {
                log::error!("[wiki::WikiApi::search_articles] {:?}", error);
                panic!("Something weird happened, please check your logs");
            }
        }
    }

    pub fn get_article(&self, page_id: &i32) -> article::ParsedArticle {
        // creating the url and making the request
        let url = format!("{}?curid={}", CONFIG.api_config.base_url.clone(), page_id);
        let article_html = self.client.get(&url).send().unwrap();

        // parsing the html response into a Article
        self.parser.parse(article_html)
    }
}
