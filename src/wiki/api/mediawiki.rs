use reqwest::blocking::Client;

#[derive(Debug)]
pub enum Error {
    HTTPError,
    JSONError,
}

#[derive(Debug)]
pub struct Search<'a> {
    offset: Option<u64>,
    info: SearchInfo,
    result: Vec<SearchResult>,
    origin: &'a Mediawiki,
}

#[derive(Debug)]
pub struct SearchInfo {
    total_hits: Option<u64>,
    suggestion: Option<String>,
    rewritten_query: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchResult {
    #[serde(rename = "ns")]
    namespace: u64,
    title: String,
    #[serde(rename = "pageid")]
    id: u64,
    size: Option<u64>,
    wordcount: Option<u64>,
    snippet: Option<String>,
    timestamp: Option<String>,
}

#[derive(Debug)]
pub struct Article {
    title: String,
    id: u64,
    text: Option<String>,
    pub sections: Option<Vec<Section>>,
    // pub sections: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Section {
    number: String,
    #[serde(rename = "line")]
    text: String,
    anchor: String,
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
        // retrieve the search offset, if there is one
        let continue_json = json.as_object().ok_or(Error::JSONError)?.get("continue");

        let search_offset = match continue_json {
            Some(json) => Some(
                json.get("sroffset")
                    .ok_or(Error::JSONError)?
                    .as_u64()
                    .ok_or(Error::JSONError)?
                    .to_owned(),
            ),
            None => None,
        };

        let query_json = json
            .as_object()
            .ok_or(Error::JSONError)?
            .get("query")
            .ok_or(Error::JSONError)?; // the query argument must always be there

        // retrieve the info about the search
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
            origin: &self,
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

        // retrieve the title
        let article_title = parse_json.get("title").ok_or(Error::JSONError)?.to_string();

        // retrieve the id
        let article_id = parse_json
            .get("pageid")
            .and_then(|x| x.as_u64())
            .ok_or(Error::JSONError)?;

        // retrieve the text
        let article_text = parse_json
            .get("text")
            .and_then(|x| x.get("*"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());

        // retrieve the sections
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
