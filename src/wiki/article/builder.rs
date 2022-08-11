use crate::wiki::article::{compiled_article::Article, parser::Parser};

use anyhow::Result;
use reqwest::blocking::{get, Response};

/// A Builder which fetches and parses an article. Can work with either an article id or a link
pub struct ArticleBuilder {
    /// The id of the article to be fetched
    page_id: i32,
    /// The optional link of the article to be fetched
    target: Option<String>,
    /// The url of wikipedia
    base_url: String,
}

impl ArticleBuilder {
    /// Creates a new Articlebuilder
    pub fn new(page_id: i32, target: Option<String>, base_url: &str) -> ArticleBuilder {
        log::debug!("creating a new instance of ArticleBuilder");
        ArticleBuilder {
            page_id,
            target,
            base_url: base_url.to_string(),
        }
    }

    /// Fetches the article and parses it with a given parser. Any errors it encounters will be returned
    pub fn build(&self, parser: &mut impl Parser) -> Result<Article> {
        log::info!("beginning the build process");
        let url = self.build_url();

        log::info!("making the request to '{}'", url);
        let response = self.make_request(&url)?;

        log::info!("parsing the article");
        self.parse_response(parser, response)
    }

    /// Creates a url from the link
    fn build_url(&self) -> String {
        match self.target {
            Some(ref target) => format!("{}{}", self.base_url, target),
            None => format!("{}?curid={}", self.base_url, self.page_id),
        }
    }

    /// Makes the request to wikipedia and checks the response for errors
    fn make_request(&self, url: &str) -> Result<Response> {
        Ok(get(url)?.error_for_status()?)
    }

    /// Parses the response with a given parser
    fn parse_response(&self, parser: &mut impl Parser, response: Response) -> Result<Article> {
        parser.parse(response)
    }
}

#[cfg(test)]
mod tests {
    const BASE_URL: &str = "https://en.wikipedia.org/";

    #[test]
    fn correct_url() {
        use super::ArticleBuilder;
        assert_eq!(
            ArticleBuilder::new(1234, None, BASE_URL).build_url(),
            format!("{}?curid=1234", BASE_URL)
        );
        assert_eq!(
            ArticleBuilder::new(1234, Some("/wiki/Software".to_string()), BASE_URL).build_url(),
            format!("{}/wiki/Software", BASE_URL)
        );
    }
}
