use crate::config::ApiConfig;
use crate::structs::wiki::article::*;
use crate::structs::wiki::parser::*;
use crate::structs::wiki::search::*;
use crate::structs::Parser;
use anyhow::*;

pub struct Wiki {
    client: reqwest::blocking::Client,
    api_config: ApiConfig,
    parser: Box<dyn Parser>,
}

impl Wiki {
    pub fn new(config: ApiConfig) -> Self {
        log::info!("Creating a instance of Wiki");
        let default_parser = Default {};
        Wiki {
            client: reqwest::blocking::Client::new(),
            api_config: config,
            parser: Box::new(default_parser),
        }
    }

    pub fn search(&self, title: &str) -> SearchResponse {
        self.search_articles(title, None)
    }
    pub fn continue_search(&self, title: &str, continue_code: &ContinueCode) -> SearchResponse {
        self.search_articles(title, Some(continue_code))
    }

    fn search_articles(&self, title: &str, continue_code: Option<&ContinueCode>) -> SearchResponse {
        // creating the url for the request
        let mut url = format!(
            "{}?action=query&list=search&srwhat=text&srsearch={}&format=json",
            self.api_config.base_url.clone(),
            title
        );

        if continue_code.is_some() {
            let continue_unwrapped = continue_code.unwrap();
            let continue_code_unwrapped = &continue_unwrapped.continue_code;
            let continue_scroll_offset_unwrapped = continue_unwrapped.scroll_offset;
            url = format!("{}?action=query&list=search&srwhat=text&srsearch={}&format=json&continue={}&sroffset={}", self.api_config.base_url.clone(), title, continue_code_unwrapped, continue_scroll_offset_unwrapped);
        }

        // making the request
        let response = match self
            .client
            .get(&url)
            .send()
            .context("Failed to send the request")
        {
            Ok(response) => {
                log::info!("Successfully sent the request");
                response
            }
            Err(error) => {
                log::error!("Failed sending the request, {:?}", error);
                panic!("Something happened, please check your logs")
            }
        };

        // serializing the response
        let serde_result = response
            .json::<SearchResponse>()
            .context("Failed serializing the response");

        match serde_result {
            Ok(result) => {
                log::info!("Successfully serialized the response");
                result
            }
            Err(error) => {
                log::error!("Failed serializing the response, {:?}", error);
                panic!("Something weird happened, please check your logs");
            }
        }
    }

    pub fn get_article(&self, page_id: &i32) -> Article {
        // creating the url and making the request
        let url = format!("http://en.wikipedia.org/?curid={}", page_id);
        let article_html = self.client.get(&url).send().unwrap();

        // parsing the html response into a Article
        self.parser.parse(article_html)
    }
}
