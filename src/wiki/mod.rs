use crate::config::CONFIG;
use anyhow::{Context, Result};
pub mod article;
pub mod parser;
pub mod search;

pub struct WikiApi {
    client: reqwest::blocking::Client,
    parser: Box<dyn parser::Parser>,
}

impl WikiApi {
    pub fn new() -> Self {
        let default_parser = parser::Default {};
        WikiApi {
            client: reqwest::blocking::ClientBuilder::new().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:90.0) Gecko/20100101 Firefox/90.0").build().unwrap(),
            parser: Box::new(default_parser),
        }
    }

    pub fn search(&self, query: &str) -> Result<search::SearchResponse> {
        self.search_articles(query, None)
    }

    pub fn continue_search(
        &self,
        query: &str,
        continue_code: &search::ContinueCode,
    ) -> Result<search::SearchResponse> {
        self.search_articles(query, Some(continue_code))
    }

    fn search_articles(
        &self,
        query: &str,
        continue_code: Option<&search::ContinueCode>,
    ) -> Result<search::SearchResponse> {
        // creating the url for the request
        let mut url = format!(
            "{}/w/api.php?action=query&list=search&srwhat=text&srsearch={}&format=json",
            CONFIG.api_config.base_url, 
            query
        );

        if let Some(_continue) = continue_code {
            let continue_code = &_continue.continue_code;
            let continue_scroll_offset = _continue.scroll_offset;
            url = format!(
                "{}/w/api.php?action=query&list=search&srwhat=text&srsearch={}&format=json&continue={}&sroffset={}",
                CONFIG.api_config.base_url, 
                query, 
                continue_code, 
                continue_scroll_offset
            );
        }

        let response = self
            .client
            .get(&url)
            .send()
            .context("Failed sending the search request")?;
        
        // serializing the response
        response
            .json::<search::SearchResponse>()
            .context("Failed serializing the search response")
    }

    pub fn get_article(&self, page_id: &i32) -> Result<article::ParsedArticle> {
        // creating the url and making the request
        self.parse_article(&format!(
            "{}?curid={}",
            CONFIG.api_config.base_url.clone(),
            page_id
        ))
    }

    fn parse_article(&self, url: &str) -> Result<article::ParsedArticle> {
        let article_html = self.client.get(url).send().unwrap();

        // parsing the html response into a Article
        self.parser.parse(article_html)
    }

    pub fn open_article(&self, target: &str) -> Result<article::ParsedArticle> {
        self.parse_article(&format!("{}{}", CONFIG.api_config.base_url.clone(), target))
    }
}

impl Default for WikiApi {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for WikiApi {}
unsafe impl Sync for WikiApi {}
