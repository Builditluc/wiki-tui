use crate::{
    config::CONFIG,
    wiki::article::{compiled_article::Article, parser::Parser},
};
use anyhow::Result;
use reqwest::blocking::{get, Response};

pub struct ArticleBuilder {
    page_id: i32,
    target: Option<String>,
}

impl ArticleBuilder {
    pub fn new(page_id: i32, target: Option<String>) -> ArticleBuilder {
        ArticleBuilder { page_id, target }
    }

    pub fn build<'a>(&self, parser: &mut impl Parser<'a>) -> Result<Article> {
        let url = self.build_url();
        let response = self.make_request(&url)?;

        self.parse_response(parser, response)
    }

    fn build_url(&self) -> String {
        match self.target {
            Some(ref target) => format!("{}{}", CONFIG.api_config.base_url, target),
            None => format!("{}?curid={}", CONFIG.api_config.base_url, self.page_id),
        }
    }

    fn make_request(&self, url: &str) -> Result<Response> {
        Ok(get(url)?.error_for_status()?)
    }

    fn parse_response<'a>(
        &self,
        parser: &mut impl Parser<'a>,
        response: Response,
    ) -> Result<Article> {
        parser.parse(response)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn correct_url() {
        use super::ArticleBuilder;
        assert_eq!(
            ArticleBuilder::new(1234, None).build_url(),
            "https://en.wikipedia.org/?curid=1234".to_string()
        );
        assert_eq!(
            ArticleBuilder::new(1234, Some("/wiki/Software".to_string())).build_url(),
            "https://en.wikipedia.org//wiki/Software".to_string()
        );
    }
}
