use crate::wiki::search::{
    compiled_search::Search, info::SearchInfo, metadata::SearchMetadata,
    properties::SearchProperties, result::SearchResult, sort_order::SearchSortOrder,
};

use anyhow::{bail, Context, Result};
use reqwest::blocking::{get, Response};
use serde::Deserialize;

/// A SearchBuilder can be used to do a search with custom configuration
pub struct SearchBuilder {
    /// Search for this page title or content matching this value
    query: String,
    /// Search only with these namespaces. Accepted namespaces are:
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 100, 101, 118, 119,
    /// 710, 711, 828, 829, 2300, 2301, 2302, 2303
    namespace: usize,
    /// How many total pages to return. The value must be between 1 and 500
    limit: usize,
    /// When more results are available, use this to continue
    offset: usize,

    /// Which metadata to return
    info: SearchMetadata,
    /// Which properties to return
    prop: SearchProperties,
    /// Set the sort order of returned results
    sort: SearchSortOrder,
    /// The url of wikipedia
    base_url: String,
}

// NOTE: The following structs are only used for deserializing the json response
#[derive(Deserialize)]
#[doc(hidden)]
struct JsonResponse {
    #[serde(rename = "continue")]
    continue_code: JsonResponseContinue,

    query: JsonResponseQuery,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct JsonResponseContinue {
    #[serde(rename = "sroffset")]
    offset: i32,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct JsonResponseQuery {
    #[serde(rename = "searchinfo")]
    info: Option<JsonResponseInfo>,
    search: Vec<JsonResponseResult>,
}

#[derive(Deserialize)]
#[doc(hidden)]
struct JsonResponseInfo {
    #[serde(rename = "totalhits")]
    total_hits: Option<i32>,
    suggestion: Option<String>,
    #[serde(rename = "rewrittenquery")]
    rewritten_query: Option<String>,
}

#[derive(Deserialize)]
#[doc(hidden)]
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

/// A helper macro for building a setter function
macro_rules! build_setter {
    ($(#[$meta :meta])* $value: ident, $type: ty) => {
        #[must_use]
        $(#[$meta])*
        pub fn $value(mut self, value: $type) -> Self {
            self.$value = value;
            self
        }
    };
}

#[allow(dead_code)]
impl SearchBuilder {
    /// Creates a new SearchBuilder
    pub fn new(base_url: &str) -> Self {
        log::debug!("creating a new SearchBuilder");
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
            base_url: base_url.to_string(),
        }
    }

    build_setter!(
        /// Search for this page title or content matching this value
        query,
        String
    );
    build_setter!(
        /// Search only with these namespaces. Accepted namespaces are:
        /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 100, 101, 118, 119,
        /// 710, 711, 828, 829, 2300, 2301, 2302, 2303
        namespace,
        usize
    );
    build_setter!(
        /// How many total pages to return. The value must be between 1 and 500
        limit,
        usize
    );
    build_setter!(
        /// When more results are available, use this to continue
        offset,
        usize
    );

    build_setter!(
        /// Which metadata to return
        info,
        SearchMetadata
    );
    build_setter!(
        /// Which properties to return
        prop,
        SearchProperties
    );
    build_setter!(
        /// Set the sort order of returned results
        sort,
        SearchSortOrder
    );

    /// Begin the search. This will return either a Search or an error
    pub fn search(&self) -> Result<Search> {
        log::info!("search was called");

        // check for invalid fields
        log::debug!("checking for invalid fields");
        self.invalid_fields().context("there are invalid fields")?;

        // build the url
        log::debug!("building the url");
        let url = self.build_url().context("failed building the url")?;

        // make the request
        log::debug!("making the request to '{}'", url);
        let response = self
            .make_request(&url)
            .context("failed sending the request")?;

        // deserialize the response and return the finished search
        log::debug!("deserializing the resposne");
        let search = self.deserialize_response(
            response
                .text()
                .context("failed getting the response content")?,
        );
        if search.is_ok() {
            log::info!("search finished successfully");
        }

        search
    }

    /// A helper function that builds the search url. It fails if the query is empty
    fn build_url(&self) -> Result<String> {
        // if the query is empty, then do nothing
        if self.query.is_empty() {
            bail!("the query is empty. we don't do that here!")
        }

        // just build the url, very simple
        Ok(format!(
            "{}w/api.php?action=query&format=json&list=search&srsearch={}&srnamespace={}&srlimit={}&sroffset={}{}{}{}",
            self.base_url,
            self.query,
            self.namespace,
            self.limit,
            self.offset,
            self.info.build(),
            self.prop.build(),
            self.sort,
        ))
    }

    /// A helper function that makes a get request to a given url and returns its response
    fn make_request(&self, url: &str) -> Result<Response> {
        // just do the request, nothing special here
        Ok(get(url)?.error_for_status()?)
    }

    /// A helper function that deserializes a json string into a Search. Any errors it encounters
    /// will be returned
    fn deserialize_response(&self, json: String) -> Result<Search> {
        // deserialize the response into a temporary struct
        let mut deserialized_json: JsonResponse =
            serde_json::from_str(&json).context("failed to deserialize the response")?;

        // retrieve the values of importance
        let search_offset = deserialized_json.continue_code.offset as usize;
        let search_info = self.deserialize_search_info(deserialized_json.query.info.take());
        let search_results =
            self.deserialize_search_results(std::mem::take(&mut deserialized_json.query.search));

        // return the search
        Ok(Search::new(search_offset, search_info, search_results))
    }

    /// A helper function that converts a JsonResponseInfo into a SearchInfo
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

    /// A helper function that converts an array of JsonResponseResult into an array of SearchResult
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

    /// A helper function that converts a JsonResponseResult into a SearchResult
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

    /// A helper function that checks all fields have correct values. Returns Ok(()) if all fields
    /// are valid
    fn invalid_fields(&self) -> Result<()> {
        // validate the limit
        if self.limit < 1 || self.limit > 500 {
            bail!("limit must be between 1 and 500")
        }

        // validate the namespace
        let valid_namespaces = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 100, 101, 118, 119, 710, 711,
            828, 829, 2300, 2301, 2302, 2303,
        ];

        if !valid_namespaces.contains(&self.namespace) {
            bail!(
                "namespace invalid. Expected one of '{:?}', got '{}' instead",
                valid_namespaces,
                &self.namespace
            )
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    const SEARCH_RESPONSE: &str = r#"{"batchcomplete":"","continue":{"sroffset":2,"continue":"-||"},"query":{"searchinfo":{"totalhits":232618,"suggestion":"mening","suggestionsnippet":"mening"},"search":[{"ns":0,"title":"Meaning","pageid":18916,"size":1645,"wordcount":215,"snippet":"<span class=\"searchmatch\">Meaning</span> most commonly refers to: <span class=\"searchmatch\">Meaning</span> (linguistics), <span class=\"searchmatch\">meaning</span> which is communicated through the use of language <span class=\"searchmatch\">Meaning</span> (philosophy), definition, elements","timestamp":"2021-10-19T21:30:54Z"},{"ns":0,"title":"The Meaning of Meaning","pageid":1754283,"size":4359,"wordcount":470,"snippet":"The <span class=\"searchmatch\">Meaning</span> of <span class=\"searchmatch\">Meaning</span>: A Study of the Influence of Language upon Thought and of the Science of Symbolism (1923) is a book by C. K. Ogden and I. A. Richards","timestamp":"2022-01-07T23:20:19Z"}]}}"#;
    const BASE_URL: &str = "https://en.wikipedia.org/";

    #[test]
    fn correct_url() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new(BASE_URL).build_url().is_err());
        assert_eq!(SearchBuilder::new(BASE_URL).query("meaning".to_string()).build_url().unwrap(), "https://en.wikipedia.org/w/api.php?action=query&format=json&list=search&srsearch=meaning&srnamespace=0&srlimit=10&sroffset=0&srinfo=totalhits|suggestion|rewrittenquery&srprop=size|wordcount|timestamp|snippet&srsort=relevance".to_string());
    }

    #[test]
    fn deserialize_correct() -> anyhow::Result<()> {
        use super::SearchBuilder;
        SearchBuilder::new(BASE_URL).deserialize_response(SEARCH_RESPONSE.to_string())?;

        Ok(())
    }

    #[test]
    fn deserialize_missing_fields() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new(BASE_URL)
            .deserialize_response("{}".to_string())
            .is_err());
    }

    #[test]
    fn namespace_invalid() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new(BASE_URL)
            .namespace(2304)
            .invalid_fields()
            .is_err());
        assert!(SearchBuilder::new(BASE_URL)
            .namespace(16)
            .invalid_fields()
            .is_err());
    }

    #[test]
    fn limit_invalid() {
        use super::SearchBuilder;
        assert!(SearchBuilder::new(BASE_URL)
            .limit(0)
            .invalid_fields()
            .is_err());
        assert!(SearchBuilder::new(BASE_URL)
            .limit(501)
            .invalid_fields()
            .is_err());
    }
}
