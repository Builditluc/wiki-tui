use crate::{
    config::CONFIG,
    wiki::search::{
        compiled_search::Search, info::SearchInfo, metadata::SearchMetadata,
        properties::SearchProperties, result::SearchResult, sort_order::SearchSortOrder,
    },
};
use anyhow::{bail, Context, Result};
use reqwest::blocking::{get, Response};
use serde::Deserialize;

pub struct SearchBuilder {
    query: String,
    namespace: usize,
    limit: usize,
    offset: usize,

    info: SearchMetadata,
    prop: SearchProperties,
    sort: SearchSortOrder,
}

#[derive(Deserialize)]
struct JsonResponse {
    #[serde(rename = "continue")]
    continue_code: JsonResponseContinue,

    query: JsonResponseQuery,
}

#[derive(Deserialize)]
struct JsonResponseContinue {
    #[serde(rename = "sroffset")]
    offset: i32,
}

#[derive(Deserialize)]
struct JsonResponseQuery {
    #[serde(rename = "searchinfo")]
    info: Option<JsonResponseInfo>,
    search: Vec<JsonResponseResult>,
}

#[derive(Deserialize)]
struct JsonResponseInfo {
    #[serde(rename = "totalhits")]
    total_hits: Option<i32>,
    suggestion: Option<String>,
    #[serde(rename = "rewrittenquery")]
    rewritten_query: Option<String>,
}

#[derive(Deserialize)]
struct JsonResponseResult {
    #[serde(rename = "ns")]
    namespace: usize,
    title: String,
    #[serde(rename = "pageid")]
    page_id: i32,
    size: Option<i32>,

    wordcount: Option<i32>,
    timestamp: Option<String>,

    snippet: Option<String>,
    #[serde(rename = "titlesnippet")]
    title_snippet: Option<String>,
    #[serde(rename = "categorysnippet")]
    category_snippet: Option<String>,

    #[serde(rename = "redirecttitle")]
    redirect_title: Option<String>,
    #[serde(rename = "redirectsnippet")]
    redirect_snippet: Option<String>,

    #[serde(rename = "sectiontitle")]
    section_title: Option<String>,
    #[serde(rename = "sectionsnippet")]
    section_snippet: Option<String>,

    #[serde(rename = "isfilematch")]
    is_file_match: Option<bool>,
}

macro_rules! build_setter {
    ($value: ident, $type: ty) => {
        #[must_use]
        pub fn $value(mut self, value: $type) -> Self {
            self.$value = value;
            self
        }
    };
}

impl SearchBuilder {
    pub fn new() -> Self {
        SearchBuilder {
            query: String::new(),
            namespace: 0,
            limit: 10,
            offset: 0,

            info: SearchMetadata::new()
                .total_hits()
                .suggestion()
                .rewritten_query(),
            prop: SearchProperties::new()
                .size()
                .wordcount()
                .timestamp()
                .snippet(),
            sort: SearchSortOrder::default(),
        }
    }

    build_setter!(query, String);
    build_setter!(namespace, usize);
    build_setter!(limit, usize);
    build_setter!(offset, usize);

    build_setter!(info, SearchMetadata);
    build_setter!(prop, SearchProperties);
    build_setter!(sort, SearchSortOrder);

    pub fn search(&self) -> Result<Search> {
        self.invalid_fields()?;
        let url = self.build_url(&CONFIG.api_config.base_url);
        let response = self.make_request(&url)?;

        self.deserialize_response(response.text()?)
    }

    fn build_url(&self, base_url: &str) -> String {
        if self.query.is_empty() {
            return String::new();
        }
        format!(
            "{}w/api.php?action=query&format=json&list=search&srsearch={}&srnamespace={}&srlimit={}&sroffset={}{}{}{}",
            base_url,
            self.query,
            self.namespace,
            self.limit,
            self.offset,
            self.info.build(),
            self.prop.build(),
            self.sort.to_string(),
        )
    }

    fn make_request(&self, url: &str) -> Result<Response> {
        Ok(get(url)?.error_for_status()?)
    }

    fn deserialize_response(&self, json: String) -> Result<Search> {
        let mut deserialized_json: JsonResponse =
            serde_json::from_str(&json).context("Failed to deserialize the response")?;

        let search_offset = deserialized_json.continue_code.offset as usize;
        let search_info = self.deserialize_search_info(deserialized_json.query.info.take());
        let search_results =
            self.deserialize_search_results(std::mem::take(&mut deserialized_json.query.search));

        Ok(Search::new(search_offset, search_info, search_results))
    }

    fn deserialize_search_info(&self, search_info: Option<JsonResponseInfo>) -> SearchInfo {
        let mut total_hits: Option<i32> = None;
        let mut suggestion: Option<String> = None;
        let mut rewritten_query: Option<String> = None;

        if let Some(mut info) = search_info {
            total_hits = info.total_hits.take();
            suggestion = info.suggestion.take();
            rewritten_query = info.rewritten_query.take();
        }

        SearchInfo::new(total_hits, suggestion, rewritten_query)
    }

    fn deserialize_search_results(
        &self,
        search_results: Vec<JsonResponseResult>,
    ) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = Vec::new();

        for search_result in search_results.into_iter() {
            results.push(self.deserialize_search_result(search_result));
        }

        results
    }

    fn deserialize_search_result(&self, search_result: JsonResponseResult) -> SearchResult {
        SearchResult::new(
            search_result.title,
            search_result.namespace,
            search_result.page_id,
            search_result.size,
            search_result.wordcount,
            search_result.timestamp,
            search_result.snippet,
            search_result.title_snippet,
            search_result.category_snippet,
            search_result.redirect_title,
            search_result.redirect_snippet,
            search_result.section_title,
            search_result.section_snippet,
            search_result.is_file_match,
        )
    }

    fn invalid_fields(&self) -> Result<()> {
        if self.limit < 1 || self.limit > 500 {
            bail!("limit must be between 1 and 500")
        }
        if ![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 100, 101, 118, 119, 710, 711,
            828, 829, 2300, 2301, 2302, 2303,
        ]
        .contains(&self.namespace)
        {
            bail!("namespace invalid")
        }

        Ok(())
    }
}

impl Default for SearchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    const SEARCH_RESPONSE: &str = r#"{"batchcomplete":"","continue":{"sroffset":2,"continue":"-||"},"query":{"searchinfo":{"totalhits":232618,"suggestion":"mening","suggestionsnippet":"mening"},"search":[{"ns":0,"title":"Meaning","pageid":18916,"size":1645,"wordcount":215,"snippet":"<span class=\"searchmatch\">Meaning</span> most commonly refers to: <span class=\"searchmatch\">Meaning</span> (linguistics), <span class=\"searchmatch\">meaning</span> which is communicated through the use of language <span class=\"searchmatch\">Meaning</span> (philosophy), definition, elements","timestamp":"2021-10-19T21:30:54Z"},{"ns":0,"title":"The Meaning of Meaning","pageid":1754283,"size":4359,"wordcount":470,"snippet":"The <span class=\"searchmatch\">Meaning</span> of <span class=\"searchmatch\">Meaning</span>: A Study of the Influence of Language upon Thought and of the Science of Symbolism (1923) is a book by C. K. Ogden and I. A. Richards","timestamp":"2022-01-07T23:20:19Z"}]}}"#;

    #[test]
    fn correct_url() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new()
            .build_url("https://en.wikipedia.org/")
            .is_empty());
        assert_eq!(SearchBuilder::new().query("meaning".to_string()).build_url("https://en.wikipedia.org/"), "https://en.wikipedia.org/w/api.php?action=query&format=json&list=search&srsearch=meaning&srnamespace=0&srlimit=10&sroffset=0&srinfo=totalhits|suggestion|rewrittenquery&srprop=size|wordcount|timestamp|snippet&srsort=relevance".to_string());
    }

    #[test]
    fn deserialize_correct() -> anyhow::Result<()> {
        use super::SearchBuilder;
        SearchBuilder::new().deserialize_response(SEARCH_RESPONSE.to_string())?;

        Ok(())
    }

    #[test]
    fn deserialize_missing_fields() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new()
            .deserialize_response("{}".to_string())
            .is_err());
    }

    #[test]
    fn namespace_invalid() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new()
            .namespace(2304)
            .invalid_fields()
            .is_err());
        assert!(SearchBuilder::new().namespace(16).invalid_fields().is_err());
    }

    #[test]
    fn limit_invalid() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new().limit(0).invalid_fields().is_err());
        assert!(SearchBuilder::new().limit(501).invalid_fields().is_err());
    }
}
