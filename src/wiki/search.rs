use anyhow::{Context, Result};
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::fmt::Display;

fn action_query(params: Vec<(&str, String)>, url: String) -> Result<Response> {
    Client::new()
        .get(url)
        .query(&[
            ("action", "query"),
            ("format", "json"),
            ("formatversion", "2"),
        ])
        .query(&params)
        .send()
        .context("failed sending the request")
}

/// A finished search containing the results and additional information
#[derive(Debug)]
pub struct Search {
    complete: bool,
    continue_offset: Option<usize>,
    total_hits: Option<usize>,
    suggestion: Option<String>,
    rewritten_query: Option<String>,
    results: Vec<SearchResult>,
}

impl Search {
    pub fn builder() -> SearchBuilder<NoQuery, NoUrl> {
        SearchBuilder::default()
    }

    pub fn complete(&self) -> bool {
        self.complete
    }

    pub fn continue_offset(&self) -> Option<usize> {
        self.continue_offset
    }

    pub fn total_hits(&self) -> Option<usize> {
        self.total_hits
    }

    pub fn suggestion(&self) -> Option<&str> {
        match self.suggestion {
            Some(ref x) => Some(x as &str),
            None => None,
        }
    }

    pub fn rewritten_query(&self) -> Option<&str> {
        match self.rewritten_query {
            Some(ref x) => Some(x as &str),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    pub fn results(&self) -> &Vec<SearchResult> {
        &self.results
    }

    pub fn take_results(&mut self) -> Vec<SearchResult> {
        std::mem::take(&mut self.results)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResult {
    #[serde(rename = "ns")]
    namespace: Namespace,
    title: String,
    pageid: usize,
    size: Option<usize>,
    wordcount: Option<usize>,
    snippet: Option<String>,
    timestamp: Option<String>,
}

impl SearchResult {
    pub fn namespace(&self) -> Namespace {
        self.namespace.clone()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn pageid(&self) -> usize {
        self.pageid
    }

    pub fn size(&self) -> Option<usize> {
        self.size
    }

    pub fn wordcount(&self) -> Option<usize> {
        self.wordcount
    }

    pub fn snippet(&self) -> Option<&str> {
        self.snippet.as_ref().map(|x| x as _)
    }

    pub fn timestamp(&self) -> Option<&str> {
        self.timestamp.as_ref().map(|x| x as _)
    }
}

#[derive(Deserialize_repr, Debug, Clone)]
#[repr(usize)]
/// The 16 "real" namespaces, corresponding to actual pages. This only includes the default
/// namespaces as defined by MediaWiki. They are:
/// - Main
/// - User
/// - Project
/// - File
/// - MediaWiki
/// - Template
/// - Help
/// - Category
///
/// All of those namespaces also include a "Talk" namespaces
pub enum Namespace {
    Main = 0,
    MainTalk = 1,
    User = 2,
    UserTalk = 3,
    Project = 4,
    ProjectTalk = 5,
    File = 6,
    FileTalk = 7,
    MediaWiki = 8,
    MediaWikiTalk = 9,
    Template = 10,
    TemplateTalk = 11,
    Help = 12,
    HelpTalk = 13,
    Category = 14,
    CategoryTalk = 15,
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone())
    }
}

pub enum QiProfile {
    Classic,
    ClassicNoBoostLinks,
    WSumIncLinks,
    WSumIncLinksPV,
    PopularIncLinksPV,
    PopularIncLinks,
    EngineAutoselect,
}

impl Display for QiProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QiProfile::Classic => write!(f, "classic"),
            QiProfile::ClassicNoBoostLinks => write!(f, "classic_noboostlinks"),
            QiProfile::WSumIncLinks => write!(f, "wsum_inclinks"),
            QiProfile::WSumIncLinksPV => write!(f, "wsum_inclinks_pv"),
            QiProfile::PopularIncLinksPV => write!(f, "popular_inclinks_pv"),
            QiProfile::PopularIncLinks => write!(f, "popular_inclinks"),
            QiProfile::EngineAutoselect => write!(f, "engine_autoselect"),
        }
    }
}

pub enum SearchType {
    NearMatch,
    Text,
    Title,
}

impl Display for SearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchType::NearMatch => write!(f, "nearmatch"),
            SearchType::Text => write!(f, "text"),
            SearchType::Title => write!(f, "title"),
        }
    }
}

pub enum Info {
    RewrittenQuery,
    Suggestion,
    TotalHits,
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Info::RewrittenQuery => write!(f, "rewrittenquery"),
            Info::Suggestion => write!(f, "suggestion"),
            Info::TotalHits => write!(f, "totalhits"),
        }
    }
}

pub enum Property {
    Size,
    WordCount,
    Timestamp,
    Snippet,
    TitleSnippet,
    RedirectTitle,
    RedirectSnippet,
    SectionTitle,
    SectionSnippet,
    IsFileMatch,
    CategorySnippet,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Size => write!(f, "size"),
            Property::WordCount => write!(f, "wordcount"),
            Property::Timestamp => write!(f, "timestamp"),
            Property::Snippet => write!(f, "snippet"),
            Property::TitleSnippet => write!(f, "titlesnippet"),
            Property::RedirectTitle => write!(f, "redirecttitle"),
            Property::RedirectSnippet => write!(f, "redirectsnippet"),
            Property::SectionTitle => write!(f, "sectiontitle"),
            Property::SectionSnippet => write!(f, "sectionsnippet"),
            Property::IsFileMatch => write!(f, "isfilematch"),
            Property::CategorySnippet => write!(f, "categorysnippet"),
        }
    }
}

pub enum SortOrder {
    CreateTimestampAscending,
    CreateTimestampDescending,
    IncomingLinksAscending,
    IncomingLinksDescending,
    JustMatch,
    LastEditAscending,
    LastEditDescending,
    NoSort,
    Random,
    Relevance,
    UserRandom,
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::CreateTimestampAscending => write!(f, "create_timestamp_asc"),
            SortOrder::CreateTimestampDescending => write!(f, "create_timestamp_desc"),
            SortOrder::IncomingLinksAscending => write!(f, "incoming_links_asc"),
            SortOrder::IncomingLinksDescending => write!(f, "incoming_links_desc"),
            SortOrder::JustMatch => write!(f, "just_match"),
            SortOrder::LastEditAscending => write!(f, "last_edit_asc"),
            SortOrder::LastEditDescending => write!(f, "last_edit_desc"),
            SortOrder::NoSort => write!(f, "none"),
            SortOrder::Random => write!(f, "random"),
            SortOrder::Relevance => write!(f, "relevance"),
            SortOrder::UserRandom => write!(f, "user_random"),
        }
    }
}

pub struct WithQuery(String);
#[derive(Default)]
pub struct NoQuery;

pub struct WithUrl(String);
#[derive(Default)]
pub struct NoUrl;

#[derive(Default)]
pub struct SearchBuilder<Q, U> {
    query: Q,
    url: U,
    namespace: Option<Namespace>,
    limit: Option<usize>,
    offset: Option<usize>,
    qiprofile: Option<QiProfile>,
    search_type: Option<SearchType>,
    info: Option<Vec<Info>>,
    properties: Option<Vec<Property>>,
    interwiki: Option<bool>,
    rewrites: Option<bool>,
    sort_order: Option<SortOrder>,
}

impl<U> SearchBuilder<NoQuery, U> {
    pub fn query(self, query: impl Into<String>) -> SearchBuilder<WithQuery, U> {
        SearchBuilder {
            query: WithQuery(query.into()),
            url: self.url,
            namespace: self.namespace,
            limit: self.limit,
            offset: self.offset,
            qiprofile: self.qiprofile,
            search_type: self.search_type,
            info: self.info,
            properties: self.properties,
            interwiki: self.interwiki,
            rewrites: self.rewrites,
            sort_order: self.sort_order,
        }
    }
}

impl<Q> SearchBuilder<Q, NoUrl> {
    pub fn url(self, url: impl Into<String>) -> SearchBuilder<Q, WithUrl> {
        SearchBuilder {
            query: self.query,
            url: WithUrl(url.into()),
            namespace: self.namespace,
            limit: self.limit,
            offset: self.offset,
            qiprofile: self.qiprofile,
            search_type: self.search_type,
            info: self.info,
            properties: self.properties,
            interwiki: self.interwiki,
            rewrites: self.rewrites,
            sort_order: self.sort_order,
        }
    }
}

impl<Q, U> SearchBuilder<Q, U> {
    pub fn namespace(mut self, namespace: Namespace) -> Self {
        self.namespace = Some(namespace);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn qiprofile(mut self, qiprofile: QiProfile) -> Self {
        self.qiprofile = Some(qiprofile);
        self
    }

    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }

    pub fn info(mut self, info: Vec<Info>) -> Self {
        self.info = Some(info);
        self
    }

    pub fn properties(mut self, properties: Vec<Property>) -> Self {
        self.properties = Some(properties);
        self
    }

    pub fn interwiki(mut self, interwiki: bool) -> Self {
        self.interwiki = Some(interwiki);
        self
    }

    pub fn rewrites(mut self, rewrites: bool) -> Self {
        self.rewrites = Some(rewrites);
        self
    }

    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }
}

impl SearchBuilder<WithQuery, WithUrl> {
    pub fn search(self) -> Result<Search> {
        let mut params = vec![("list", "search".to_string()), ("srsearch", self.query.0)];

        if let Some(namespace) = self.namespace {
            params.push(("srnamespace", namespace.to_string()));
        }

        if let Some(limit) = self.limit {
            params.push(("srlimit", limit.to_string()));
        }

        if let Some(offset) = self.offset {
            params.push(("sroffset", offset.to_string()));
        }

        if let Some(qiprofile) = self.qiprofile {
            params.push(("srqiprofile", qiprofile.to_string()));
        }

        if let Some(search_type) = self.search_type {
            params.push(("srwhat", search_type.to_string()));
        }

        if let Some(info) = self.info {
            let mut info_str = String::new();
            for info in info {
                info_str.push('|');
                info_str.push_str(&info.to_string());
            }
            params.push(("srinfo", info_str));
        }

        if let Some(prop) = self.properties {
            let mut prop_str = String::new();
            for prop in prop {
                prop_str.push('|');
                prop_str.push_str(&prop.to_string());
            }
            params.push(("srprop", prop_str));
        }

        if let Some(interwiki) = self.interwiki {
            params.push(("srinterwiki", interwiki.to_string()));
        }

        if let Some(rewrites) = self.rewrites {
            params.push(("srenablerewrites", rewrites.to_string()));
        }

        if let Some(sort_order) = self.sort_order {
            params.push(("srsort", sort_order.to_string()));
        }

        let response = action_query(params, self.url.0)?
            .error_for_status()
            .context("recieved an error")?;

        let res_json: serde_json::Value =
            serde_json::from_str(&response.text().context("failed reading the response")?)
                .context("failed reading the response as json")?;

        let continue_offset = res_json
            .get("continue")
            .and_then(|x| x.get("sroffset"))
            .and_then(|x| x.as_u64().map(|x| x as usize));

        let total_hits = res_json
            .get("query")
            .and_then(|x| x.get("searchinfo"))
            .and_then(|x| x.get("totalhits"))
            .and_then(|x| x.as_u64().map(|x| x as usize));

        let suggestion = res_json
            .get("query")
            .and_then(|x| x.get("searchinfo"))
            .and_then(|x| x.get("suggestion"))
            .and_then(|x| x.as_str().map(|x| x.to_string()));

        let rewritten_query = res_json
            .get("query")
            .and_then(|x| x.get("searchinfo"))
            .and_then(|x| x.get("rewrittenquery"))
            .and_then(|x| x.as_str().map(|x| x.to_string()));

        let results: Vec<SearchResult> = serde_json::from_value(
            res_json
                .get("query")
                .and_then(|x| x.get("search"))
                .ok_or_else(|| anyhow!("missing the search results"))?
                .to_owned(),
        )
        .context("failed deserializing the search results")?;

        Ok(Search {
            complete: continue_offset.is_none(),
            continue_offset,
            total_hits,
            suggestion,
            rewritten_query,
            results,
        })
    }
}
