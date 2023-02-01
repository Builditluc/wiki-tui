use reqwest::blocking::Client;

#[derive(Debug)]
pub enum Error {
    HTTPError,
    JSONError,
}

#[derive(Debug)]
pub struct Search {
    pub offset: Option<u64>,
    pub info: SearchInfo,
    pub result: Vec<SearchResult>,
}

#[derive(Debug)]
pub struct SearchInfo {
    pub total_hits: Option<u64>,
    pub suggestion: Option<String>,
    pub rewritten_query: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchResult {
    pub title: String,
    #[serde(rename = "pageid")]
    pub id: u64,
    pub size: Option<u64>,
    pub wordcount: Option<u64>,
    pub snippet: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug)]
pub struct Article {
    pub title: String,
    pub id: u64,
    pub text: Option<String>,
    pub sections: Option<Vec<Section>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Section {
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

    pub fn search(&self, query: &str) -> Result<Search, Error> {
        self.search_at_offset(query, 0)
    }

    pub fn search_at_offset(&self, query: &str, offset: u64) -> Result<Search, Error> {
        let res_json: serde_json::Value = serde_json::from_str(
            &self
                .client
                .get(self.url.to_owned())
                .query(&[
                    ("format", "json"),
                    ("action", "query"),
                    ("list", "search"),
                    ("srsearch", query),
                    ("sroffset", &offset.to_string()),
                ])
                .send()
                .map_err(|_| Error::HTTPError)?
                .text()
                .map_err(|_| Error::HTTPError)?,
        )
        .map_err(|_| Error::JSONError)?;
        self.search_from_json(res_json)
    }

    fn search_from_json(&self, json: serde_json::Value) -> Result<Search, Error> {
        let search_offset = json
            .as_object()
            .ok_or(Error::JSONError)?
            .get("continue")
            .and_then(|x| x.get("sroffset"))
            .and_then(|x| x.as_u64())
            .map(|x| x.to_owned());

        let query_json = json
            .as_object()
            .ok_or(Error::JSONError)?
            .get("query")
            .ok_or(Error::JSONError)?;

        let search_info = SearchInfo {
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
        let search_results: Vec<SearchResult> =
            serde_json::from_value(query_json.get("search").ok_or(Error::JSONError)?.to_owned())
                .map_err(|x| {
                    println!("{:?}", x);
                    Error::JSONError
                })?;

        Ok(Search {
            offset: search_offset,
            info: search_info,
            result: search_results,
        })
    }

    pub fn article_from_title(&self, title: &str) -> Result<Article, Error> {
        let res_json: serde_json::Value = serde_json::from_str(
            &self
                .client
                .get(self.url.to_owned())
                .query(&[
                    ("format", "json"),
                    ("action", "parse"),
                    ("page", title),
                    ("prop", "sections|text"),
                ])
                .send()
                .map_err(|_| Error::HTTPError)?
                .text()
                .map_err(|_| Error::HTTPError)?,
        )
        .map_err(|_| Error::JSONError)?;

        self.article_from_json(res_json)
    }

    pub fn article_from_id(&self, id: u64) -> Result<Article, Error> {
        let res_json: serde_json::Value = serde_json::from_str(
            &self
                .client
                .get(self.url.to_owned())
                .query(&[
                    ("format", "json"),
                    ("action", "parse"),
                    ("pageid", &id.to_string()),
                    ("prop", "sections|text"),
                ])
                .send()
                .map_err(|_| Error::HTTPError)?
                .text()
                .map_err(|_| Error::HTTPError)?,
        )
        .map_err(|_| Error::JSONError)?;

        self.article_from_json(res_json)
    }

    fn article_from_json(&self, json: serde_json::Value) -> Result<Article, Error> {
        let parse_json = json
            .as_object()
            .ok_or(Error::JSONError)?
            .get("parse")
            .ok_or(Error::JSONError)?;

        let article_title = parse_json.get("title").ok_or(Error::JSONError)?.to_string();

        let article_id = parse_json
            .get("pageid")
            .and_then(|x| x.as_u64())
            .ok_or(Error::JSONError)?;

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
                    .collect::<Vec<Section>>()
            });

        Ok(Article {
            title: article_title,
            id: article_id,
            text: article_text,
            sections: article_sections,
        })
    }
}
