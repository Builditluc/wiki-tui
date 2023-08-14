use anyhow::{Context, Result};
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::fmt::Display;
use url::Url;

use super::language::Language;

/// A finished search containing the results and additional information
#[derive(Debug, Clone)]
pub struct Search {
    pub complete: bool,
    pub continue_offset: Option<usize>,
    pub total_hits: Option<usize>,
    pub suggestion: Option<String>,
    pub rewritten_query: Option<String>,
    pub query: String,
    pub results: Vec<SearchResult>,
    pub endpoint: Url,
    pub language: Language,
}

impl Search {
    pub fn builder() -> SearchBuilder<NoQuery, NoEndpoint, NoLanguage> {
        SearchBuilder::default()
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

#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq)]
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
        match self {
            Namespace::Main => write!(f, "Main"),
            Namespace::MainTalk => write!(f, "Main_talk"),
            Namespace::User => write!(f, "User"),
            Namespace::UserTalk => write!(f, "User_talk"),
            Namespace::Project => write!(f, "Project"),
            Namespace::ProjectTalk => write!(f, "Project_talk"),
            Namespace::File => write!(f, "File"),
            Namespace::FileTalk => write!(f, "File_talk"),
            Namespace::MediaWiki => write!(f, "Mediawiki"),
            Namespace::MediaWikiTalk => write!(f, "Mediawiki_talk"),
            Namespace::Template => write!(f, "Template"),
            Namespace::TemplateTalk => write!(f, "Template_talk"),
            Namespace::Help => write!(f, "Help"),
            Namespace::HelpTalk => write!(f, "Help_talk"),
            Namespace::Category => write!(f, "Category"),
            Namespace::CategoryTalk => write!(f, "Category_talk"),
        }
    }
}

impl Namespace {
    pub fn from_str(namespace: &str) -> Option<Namespace> {
        match namespace.to_lowercase().as_str() {
            "main" => Some(Namespace::Main),
            "main_talk" => Some(Namespace::MainTalk),
            "user" => Some(Namespace::User),
            "user_talk" => Some(Namespace::UserTalk),
            "project" => Some(Namespace::Project),
            "project_talk" => Some(Namespace::ProjectTalk),
            "file" => Some(Namespace::File),
            "file_talk" => Some(Namespace::FileTalk),
            "mediawiki" => Some(Namespace::MediaWiki),
            "mediawiki_talk" => Some(Namespace::MediaWikiTalk),
            "template" => Some(Namespace::Template),
            "template_talk" => Some(Namespace::TemplateTalk),
            "help" => Some(Namespace::Help),
            "help_talk" => Some(Namespace::HelpTalk),
            "category" => Some(Namespace::Category),
            "category_talk" => Some(Namespace::CategoryTalk),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Namespace;
    #[test]
    fn test_namespace_display_and_str() {
        macro_rules! test_namespace {
            ($namespace: ident, $namespace_talk: ident) => {
                let namespace_str = format!("{}", Namespace::$namespace);
                assert_eq!(
                    Namespace::from_str(&namespace_str),
                    Some(Namespace::$namespace)
                );

                let namespace_str = format!("{}", Namespace::$namespace_talk);
                assert_eq!(
                    Namespace::from_str(&namespace_str),
                    Some(Namespace::$namespace_talk)
                );
            };
        }

        test_namespace!(Main, MainTalk);
        test_namespace!(User, UserTalk);
        test_namespace!(Project, ProjectTalk);
        test_namespace!(File, FileTalk);
        test_namespace!(MediaWiki, MediaWikiTalk);
        test_namespace!(Template, TemplateTalk);
        test_namespace!(Help, HelpTalk);
        test_namespace!(Category, CategoryTalk);
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

pub struct WithEndpoint(Url);
#[derive(Default)]
pub struct NoEndpoint;

pub struct WithLanguage(Language);
#[derive(Default)]
pub struct NoLanguage;

#[derive(Default)]
pub struct SearchBuilder<Q, E, L> {
    query: Q,
    endpoint: E,
    language: L,
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

impl<E, L> SearchBuilder<NoQuery, E, L> {
    pub fn query(self, query: impl Into<String>) -> SearchBuilder<WithQuery, E, L> {
        SearchBuilder {
            query: WithQuery(query.into()),
            endpoint: self.endpoint,
            language: self.language,
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

impl<Q, L> SearchBuilder<Q, NoEndpoint, L> {
    pub fn endpoint(self, endpoint: Url) -> SearchBuilder<Q, WithEndpoint, L> {
        SearchBuilder {
            query: self.query,
            endpoint: WithEndpoint(endpoint),
            language: self.language,
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

impl<Q, E> SearchBuilder<Q, E, NoLanguage> {
    pub fn language(self, language: Language) -> SearchBuilder<Q, E, WithLanguage> {
        SearchBuilder {
            query: self.query,
            endpoint: self.endpoint,
            language: WithLanguage(language),
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

impl<Q, E, L> SearchBuilder<Q, E, L> {
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

impl SearchBuilder<WithQuery, WithEndpoint, WithLanguage> {
    pub fn search(self) -> Result<Search> {
        fn action_query(params: Vec<(&str, String)>, endpoint: Url) -> Result<Response> {
            Client::new()
                .get(endpoint)
                .query(&[
                    ("action", "query"),
                    ("format", "json"),
                    ("formatversion", "2"),
                ])
                .query(&params)
                .send()
                .context("failed sending the request")
        }

        let mut params = vec![
            ("list", "search".to_string()),
            ("srsearch", self.query.0.clone()),
        ];

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

        let response = action_query(params, self.endpoint.0.clone())?
            .error_for_status()
            .context("the server returned an error")?;

        let res_json: serde_json::Value =
            serde_json::from_str(&response.text().context("failed reading the response")?)
                .context("failed interpreting the response as json")?;

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
            endpoint: self.endpoint.0,
            query: self.query.0,
            language: self.language.0,
        })
    }
}
