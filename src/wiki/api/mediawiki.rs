use reqwest::blocking::Client;

use crate::wiki::article_new::{Article, HeaderType, Section};

#[derive(Debug)]
pub enum Error {
    HTTPError(reqwest::Error),
    JSONError(serde_json::Error),
    JSONPathError(String),
}

#[derive(Debug)]
pub struct MediawikiSearch {
    pub offset: Option<u64>,
    pub info: MediawikiSearchInfo,
    pub result: Vec<MediawikiSearchResult>,
}

#[derive(Debug)]
pub struct MediawikiSearchInfo {
    pub total_hits: Option<u64>,
    pub suggestion: Option<String>,
    pub rewritten_query: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MediawikiSearchResult {
    pub title: String,
    #[serde(rename = "pageid")]
    pub id: u64,
    pub size: Option<u64>,
    pub wordcount: Option<u64>,
    pub snippet: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug)]
pub struct MediawikiArticle {
    pub title: String,
    pub id: u64,
    pub text: Option<String>,
    pub sections: Option<Vec<MediawikiSection>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MediawikiSection {
    #[serde(rename = "toclevel")]
    pub level: usize,
    pub number: String,
    #[serde(rename = "line")]
    pub text: String,
    pub anchor: String,
}

#[derive(Debug)]
pub struct Mediawiki {
    url: String,
    client: Client,
}

impl Mediawiki {
    pub fn new(url: &str) -> Self {
        Mediawiki {
            url: url.to_string(),
            client: Client::new(),
        }
    }

    fn query(&self, parameters: &[(&str, &str)]) -> Result<serde_json::Value, Error> {
        serde_json::from_str(
            &self
                .client
                .get(self.url.to_owned())
                .query(&[("format", "json")])
                .query(parameters)
                .send()
                .map_err(Error::HTTPError)?
                .text()
                .map_err(Error::HTTPError)?,
        )
        .map_err(Error::JSONError)
    }

    pub fn search(&self, query: &str) -> Result<MediawikiSearch, Error> {
        self.search_at_offset(query, 0)
    }

    pub fn search_at_offset(&self, query: &str, offset: u64) -> Result<MediawikiSearch, Error> {
        let res_json = self.query(&[
            ("action", "query"),
            ("list", "search"),
            ("srsearch", query),
            ("sroffset", &offset.to_string()),
        ])?;
        self.search_from_json(res_json)
    }

    fn search_from_json(&self, json: serde_json::Value) -> Result<MediawikiSearch, Error> {
        let search_offset = json
            .get("continue")
            .and_then(|x| x.get("sroffset"))
            .and_then(|x| x.as_u64())
            .map(|x| x.to_owned());

        let query_json = json
            .get("query")
            .ok_or(Error::JSONPathError("missing element '$.query'".into()))?;

        let search_info = MediawikiSearchInfo {
            total_hits: query_json
                .get("searchinfo")
                .and_then(|x| x.get("totalhits"))
                .and_then(|x| x.as_u64()),
            suggestion: query_json
                .get("searchinfo")
                .and_then(|x| x.get("suggestion"))
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
            rewritten_query: query_json
                .get("searchinfo")
                .and_then(|x| x.get("rewrittenquery"))
                .and_then(|x| x.as_str())
                .map(|x| x.to_string()),
        };

        // retrieve the search results
        let search_results: Vec<MediawikiSearchResult> = serde_json::from_value(
            query_json
                .get("search")
                .ok_or(Error::JSONPathError(
                    "missing element '$query.search'".into(),
                ))?
                .to_owned(),
        )
        .map_err(Error::JSONError)?;

        Ok(MediawikiSearch {
            offset: search_offset,
            info: search_info,
            result: search_results,
        })
    }

    pub fn article_from_title(&self, title: &str) -> Result<MediawikiArticle, Error> {
        let res_json = self.query(&[
            ("action", "parse"),
            ("page", title),
            ("prop", "sections|text"),
        ])?;
        self.article_from_json(res_json)
    }

    pub fn article_from_id(&self, id: u64) -> Result<MediawikiArticle, Error> {
        let res_json = self.query(&[
            ("format", "json"),
            ("action", "parse"),
            ("pageid", &id.to_string()),
            ("prop", "sections|text"),
        ])?;
        self.article_from_json(res_json)
    }

    fn article_from_json(&self, json: serde_json::Value) -> Result<MediawikiArticle, Error> {
        let parse_json = json
            .get("parse")
            .ok_or(Error::JSONPathError("missing element '$.parse'".into()))?;

        let article_title = parse_json
            .get("title")
            .ok_or(Error::JSONPathError(
                "missing element '$.parse.title'".into(),
            ))?
            .to_string();

        let article_id =
            parse_json
                .get("pageid")
                .and_then(|x| x.as_u64())
                .ok_or(Error::JSONPathError(
                    "missing or invalid element '$.parse.pageid'".into(),
                ))?;

        let article_text = parse_json
            .get("text")
            .and_then(|x| x.get("*"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());

        let article_sections = parse_json
            .get("sections")
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| serde_json::from_value(x).ok())
                    .collect::<Vec<MediawikiSection>>()
            });

        Ok(MediawikiArticle {
            title: article_title,
            id: article_id,
            text: article_text,
            sections: article_sections,
        })
    }
}
