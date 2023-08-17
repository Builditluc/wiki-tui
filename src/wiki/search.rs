///! Search API
///!
///! A search can be configured and executed using [`Search::builder()`]. Available options for
///! configuration within the search include:
///!
///! - Specify which Search metadata to return using ([`Info`])
///! - Specify which Properties to return for each result ([`Property`])
///! - Change the query independent profile, affecting the ranking algorithm ([`QiProfile`])
///! - Change the type of search ([`SearchType`])
///! - Modify the sort order of returned results ([`SortOrder`])
///!
///! Apart from the customization, the API also supports continuing a search when more results are
///! available using [`SearchContinue`]
///!
///! # Making a Search
///!
///! ```
///! // Search for `meaning` in the english wikipedia with the default settings
///! let search = Search::builder()
///!    .query("meaning")
///!    .endpoint(Url::parse("https://en.wikipedia.org/w/api.php")?)
///!    .language(Language::English)
///!    .search()?;
///! ```
///!
///! Additionally, we can continue the search using [`Search::continue_data`]
///! ```
///! let continue_data = search.continue_data()
///!    .expect("no more search results");
///! let continued_search = Search::builder()
///!    .query(continue_data.query)
///!    .endpoint(continue_data.endpoint)
///!    .language(continue_data.language)
///!    .search()?;
///! ```
///!
///! [`Search::builder()`]: Search::builder
///! [`Info`]: Info
///! [`Property`]: Property
///! [`QiProfile`]: QiProfile
///! [`SearchType`]: SearchType
///! [`SortOrder`]: SortOrder
///! [`SearchContinue`]: SearchContinue
///! [`Search::continue_data`]: Search::continue_data
use anyhow::{Context, Result};
use reqwest::blocking::{Client, Response};
use serde_repr::Deserialize_repr;
use std::fmt::Display;
use url::Url;

use super::language::Language;

/// A finished search containing the found results and additional optional information regarding
/// the search
#[derive(Debug, Clone)]
pub struct Search {
    /// Whether the search is complete and no more results are available
    pub complete: bool,
    /// If more results are available, use this offset to continue the search
    pub continue_offset: Option<usize>,
    /// Optional: Total amount of results found
    pub total_hits: Option<usize>,
    /// Optional: Suggestion for a different query
    pub suggestion: Option<String>,
    /// Optional: The query rewritten by the search backend (See [`SearchBuilder::rewrites`] for
    /// more)
    ///
    /// [`SearchBuilder::rewrites`]: SearchBuilder::rewrites
    pub rewritten_query: Option<String>,
    /// Searched value
    pub query: String,
    /// The found results in this batch
    pub results: Vec<SearchResult>,
    /// API endpoint of the MediaWiki site where the search was performed on
    pub endpoint: Url,
    /// In what language the search was made
    pub language: Language,
}

impl Search {
    /// Creates a [`SearchBuilder`] to configure and perform a search
    ///
    /// [`SearchBuilder`]: SearchBuilder
    pub fn builder() -> SearchBuilder<NoQuery, NoEndpoint, NoLanguage> {
        SearchBuilder::default()
    }

    /// If available, returns the the data necessary for continuing the current search
    ///
    /// When are more results available for the search, which can be checked via the
    /// `Search::complete` field, creates a [`SearchContinue`] data
    /// struct that contains all of the necessary information to continue the search at the
    /// correct offset.
    ///
    /// [`SearchContinue`]: SearchContinue
    pub fn continue_data(&self) -> Option<SearchContinue> {
        if let Some(ref offset) = self.continue_offset {
            return Some(SearchContinue {
                query: self.query.clone(),
                endpoint: self.endpoint.clone(),
                language: self.language.clone(),
                offset: *offset,
            });
        }
        None
    }
}

/// Contains the necessary data for continuing a Search at a given offset. This data can be
/// extracted from a already existing search with [`Search::continue_data`]
///
/// # Example
///
/// ```
/// // This will continue the already completed search
/// let continue_data = search.continue_data()?;
/// let continued_search = Search::builder()
///     .query(continue_data.query)
///     .endpoint(continue_data.endpoint)
///     .langauge(continue_data.language)
///     .offset(continue_data.offset)
///     .search()?;
/// ```
///
/// [`Search::continue_data`]: Search::continue_data
#[derive(Debug, Clone)]
pub struct SearchContinue {
    /// Search for page titles or content matching this value
    pub query: String,
    /// API endpoint of the MediaWiki site to perform the search on
    pub endpoint: Url,
    /// In what language to perform the search
    pub language: Language,
    /// Offset where the search will continue
    pub offset: usize,
}

/// A single search result containing additional optional properties if they were added in the
/// search
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Namespace where the page belongs to
    pub namespace: Namespace,
    /// Title of the page
    pub title: String,
    /// PageId of the page
    pub pageid: usize,

    /// Language, the page is written in
    pub language: Language,
    /// API endpoint of the MediaWiki site this page belongs to
    pub endpoint: Url,

    /// Optional: Size in bytes of the page
    pub size: Option<usize>,
    /// Optional: Word count of the page
    pub wordcount: Option<usize>,
    /// Optional: Snippet of the page, with query term highlighting markup
    pub snippet: Option<String>,
    /// Optional: Timestamp of when the page was last edited
    pub timestamp: Option<String>,
}

/// The 16 built-in namespaces (excluding two "virtual" namespaces) of MediaWiki
///
/// A namespace is a collection of pages which have content with a similar purposek, i. e. pages
/// where the intended use is the same. Namespaces can be thought of as partitions of different
/// types of information within the same wiki, and keep "real" content separate from user profiles,
/// help pages, etc.
///
/// These are the 16 built-in "real" namespaces, meaning namespaces corresponding to actual pages.
/// They each have a unique number (0 to 15) and are grouped in subject/talk pairs
///
/// Read more in the [MediaWiki API docs](https://www.mediawiki.org/wiki/Manual:Namespace)
#[derive(Deserialize_repr, Debug, Clone, PartialEq, Eq)]
#[repr(usize)]
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

/// Query independent profile which affects the ranking algorithm
pub enum QiProfile {
    /// Ranking based on the number of incoming links, some templates, page language and recency
    /// (templates / language / recency may not be activated on the wiki where the search is
    /// performed on)
    Classic,
    /// Ranking based on some templates, page language and recency when activated on the wiki where
    /// the search is performed on
    ClassicNoBoostLinks,
    /// Weighted sum based on incoming links
    WSumIncLinks,
    /// Weighted sum based on incoming links and weekly pageviews
    WSumIncLinksPV,
    /// Ranking based primarily on page views
    PopularIncLinksPV,
    /// Ranking based primarily on incoming link counts
    PopularIncLinks,
    /// Let the search engine decide on the best profile to use
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

/// The type of search
pub enum SearchType {
    /// Search just by a match  
    NearMatch,
    /// Search the content of the page
    Text,
    /// Search the title of the page
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

/// A Search metadata
pub enum Info {
    /// The query if rewritten by the search backend. Refer to [`SearchBuilder::rewrites`] for more
    /// information about rewrites by the search backend
    ///
    /// [`SearchBuilder::rewrites`]: SearchBuilder::rewrites
    RewrittenQuery,
    /// Another query to search instead for. This might include grammatical fixes
    Suggestion,
    /// The total amount of pages found for the query
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

/// A Page property
pub enum Property {
    /// The size of the page in bytes
    Size,
    /// The word count of the page
    WordCount,
    /// The timestamp wof when the page was last edited
    Timestamp,
    /// Snippet of the page, with query term highlighting markup
    Snippet,
    /// Page title, with query term highlighting markup
    TitleSnippet,
    /// Title of the matching redirect
    RedirectTitle,
    /// Title of the matching redirect, with query term highlighting markup
    RedirectSnippet,
    /// Title of the matching section
    SectionTitle,
    /// Title of the matching section, with query term highlighting markup
    SectionSnippet,
    /// Indicator whether the search matched file content
    IsFileMatch,
    /// Matching category name, with query term highlighting markup
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

/// The sort order of returned search results
pub enum SortOrder {
    /// Sort the results by their creation date in ascending order
    CreateTimestampAscending,
    /// Sort the results by their creation date in descending order
    CreateTimestampDescending,
    /// Sort the results by their amount of pages linking to it in ascending order
    IncomingLinksAscending,
    /// Sort the results by their amount of pages linking to it in descending order
    IncomingLinksDescending,
    /// Sort the results only by their match to the query
    JustMatch,
    /// Sort the results by the time of their last edit in ascending order
    LastEditAscending,
    /// Sort the results by the time of their last edit in descending order
    LastEditDescending,
    /// Do not sort the search results
    NoSort,
    /// Arrange the results in a random order
    Random,
    /// Sort the results by relevance
    Relevance,
    /// Arrange the results in a random order depending on the current user
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

#[doc(hidden)]
pub struct WithQuery(String);

#[doc(hidden)]
#[derive(Default)]
pub struct NoQuery;

#[doc(hidden)]
pub struct WithEndpoint(Url);

#[doc(hidden)]
#[derive(Default)]
pub struct NoEndpoint;

#[doc(hidden)]
pub struct WithLanguage(Language);

#[doc(hidden)]
#[derive(Default)]
pub struct NoLanguage;

/// A `SearchBuilder` can be used to configure and perform a search
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
    /// Search for page titles or content matching this value
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
    /// API endpoint for the MediaWiki site to perform the search on
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
    /// Language where the search will be performed in
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
    /// Search only in this specific namespace
    pub fn namespace(mut self, namespace: Namespace) -> Self {
        self.namespace = Some(namespace);
        self
    }

    /// How many total pages to return. The value must be between 1 and 500
    ///
    /// Default: `10`
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// When more results are available, use the offset to continue
    /// Default: `0`
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Query independent profile to use which affects the ranking algorithm
    ///
    /// Default: [`QiProfile::EngineAutoselect`]
    ///
    /// [`QiProfile::EngineAutoselect`]: QiProfile::EngineAutoselect
    pub fn qiprofile(mut self, qiprofile: QiProfile) -> Self {
        self.qiprofile = Some(qiprofile);
        self
    }

    /// Which search to perform
    ///
    /// Default: [`SearchType::Text`]
    ///
    /// [`SearchType::Text`]: SearchType::Text
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }

    /// Which metadata to return
    ///
    /// Default: [[`Info::TotalHits`], [`Info::Suggestion`], [`Info::RewrittenQuery`]]
    ///
    /// [`Info::TotalHits`]: Info::TotalHits
    /// [`Info::Suggestion`]: Info::Suggestion
    /// [`Info::RewrittenQuery`]: Info::RewrittenQuery
    pub fn info(mut self, info: Vec<Info>) -> Self {
        self.info = Some(info);
        self
    }

    /// Which properties about the search results to return
    ///
    /// Default: [[`Property::Size`], [`Property::WordCount`], [`Property::Timestamp`],
    /// [`Property::Snippet`]]
    ///
    /// [`Property::Size`]: Property::Size
    /// [`Property::WordCount`]: Property::WordCount
    /// [`Property::Timestamp`]: Property::Timestamp
    /// [`Property::Snippet`]: Property::Snippet
    pub fn properties(mut self, properties: Vec<Property>) -> Self {
        self.properties = Some(properties);
        self
    }

    /// Include interwiki results in the search, if available
    ///
    /// Default: `false`
    pub fn interwiki(mut self, interwiki: bool) -> Self {
        self.interwiki = Some(interwiki);
        self
    }

    /// Enable internal query rewriting. Some search backends can rewrite the query into another
    /// which is thought to provide better results, for instance by correcting spelling errors
    ///
    /// Default: `false`
    pub fn rewrites(mut self, rewrites: bool) -> Self {
        self.rewrites = Some(rewrites);
        self
    }

    /// Set the sort order of returend results
    ///
    /// Default: [`SortOrder::Relevance`]
    ///
    /// [`SortOrder::Relevance`]: SortOrder::Relevance
    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }
}

impl SearchBuilder<WithQuery, WithEndpoint, WithLanguage> {
    /// Performes the search and returns the result. The search can only be made when the query,
    /// endpoint and language are set
    ///
    /// # Example
    ///
    /// ```
    /// // This searches for the pages containing 'meaning' in the english wikipedia
    /// let search = Search::builder()
    ///     .query("meaning")
    ///     .endpoint(Url::from("https://en.wikipedia.org/w/api.php")?)
    ///     .language(Language::English)
    ///     .search()?;
    /// ```
    ///
    /// # Error
    ///
    /// This function returns an error when one of the following things happens:
    /// - The request to the server could not be made
    /// - The server returned an error
    /// - The returned result could not interpreted as a `Search`
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

        let results: Vec<SearchResult> = {
            let mut results: Vec<SearchResult> = Vec::new();
            let results_json = res_json
                .get("query")
                .and_then(|x| x.get("search"))
                .and_then(|x| x.as_array())
                .ok_or_else(|| anyhow!("missing the search results"))?
                .to_owned();

            macro_rules! value_from_json {
                ($result: ident, $val: expr) => {
                    serde_json::from_value($result.get($val).map(|x| x.to_owned()).ok_or_else(
                        || anyhow!("couldn't find '{}' in the result", stringify!($val)),
                    )?)?
                };
            }

            for result in results_json.into_iter() {
                results.push(SearchResult {
                    namespace: value_from_json!(result, "ns"),
                    title: value_from_json!(result, "title"),
                    pageid: value_from_json!(result, "pageid"),
                    language: self.language.0.clone(),
                    endpoint: self.endpoint.0.clone(),
                    size: value_from_json!(result, "size"),
                    wordcount: value_from_json!(result, "wordcount"),
                    snippet: value_from_json!(result, "snippet"),
                    timestamp: value_from_json!(result, "timestamp"),
                })
            }

            results
        };

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
