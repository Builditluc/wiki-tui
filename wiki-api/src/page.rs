use crate::{
    document::{Document, HeaderKind},
    parser::{Parser, WikipediaParser},
    Endpoint,
};
use anyhow::{anyhow, Context, Result};
use reqwest::{Client, Response};
use scraper::Html;
use serde::{de, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;
use tracing::{debug, warn};
use url::Url;

use super::languages::Language;

pub mod link_data {
    use crate::{languages::Language, search::Namespace, Endpoint};
    use url::Url;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InternalData {
        pub namespace: Namespace,
        pub page: String,
        pub title: String,
        pub endpoint: Endpoint,
        pub language: Language,
        pub anchor: Option<AnchorData>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AnchorData {
        pub anchor: String,
        pub title: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RedLinkData {
        pub url: Url,
        pub title: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct MediaData {
        pub url: Url,
        pub title: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ExternalData {
        pub url: Url,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ExternalToInteralData {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Link {
    /// Interal link to another page in the same wiki
    Internal(link_data::InternalData),
    /// Anchor to a specific section in the current page
    /// Note: this only corresponds to anchors on the current page. For anchors in another page on
    /// the same wiki, `LinkType::Internal` is used
    Anchor(link_data::AnchorData),
    /// A special type of link that leads to an internal page that doesn't exist yet
    RedLink(link_data::RedLinkData),
    /// Link pointing to a media
    MediaLink(link_data::MediaData),
    /// External link to a page at another website
    External(link_data::ExternalData),
    /// External link to an interal page in the same wiki
    ExternalToInternal(link_data::ExternalToInteralData),
}

impl Link {
    pub fn title(&self) -> Option<&str> {
        match self {
            Link::Anchor(link_data) => Some(&link_data.title),
            Link::RedLink(link_data) => Some(&link_data.title),
            &Link::External(_) => None,
            &Link::ExternalToInternal(_) => None,
            Link::MediaLink(link_data) => Some(&link_data.title),
            Link::Internal(link_data) => Some(&link_data.title),
        }
    }
}

// TODO: replace this with Link::Internal
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageLink {
    pub name: String,
    pub language: Language,
    pub autonym: String,
    pub title: String,
    pub url: Url,
    pub endpoint: Endpoint,
}

#[derive(Deserialize)]
struct LanguageLinkInt {
    #[serde(rename = "langname")]
    name: String,
    #[serde(rename = "lang")]
    #[serde(deserialize_with = "language_from_str")]
    language: Language,
    autonym: String,
    title: String,
    url: Url,
}

fn language_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(de::Error::custom)
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Section {
    #[serde(skip_deserializing)]
    pub index: usize,
    #[serde(rename = "toclevel")]
    pub header_kind: HeaderKind,
    #[serde(rename = "line")]
    pub text: String,
    pub number: String,
    pub anchor: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Page {
    pub title: String,
    pub pageid: usize,
    pub content: Document,
    pub language: Language,
    pub language_links: Option<Vec<LanguageLink>>,
    pub sections: Option<Vec<Section>>,
    pub revision_id: Option<usize>,
}

impl Page {
    #[cfg(debug_assertions)]
    pub fn from_path(path: &std::path::PathBuf) -> Option<Page> {
        if !path.exists() {
            return None;
        }

        let content = std::fs::read_to_string(path).ok()?;
        let nodes = WikipediaParser::parse_document(
            &content,
            url::Url::parse("https://en.wikipedia.org/w/api.php").ok()?,
            Language::default(),
        )
        .nodes();

        Some(Page {
            title: "DEBUG: FILE".to_string(),
            pageid: 0,
            content: Document { nodes },
            language: Language::default(),
            language_links: None,
            sections: None,
            revision_id: None,
        })
    }

    pub fn builder() -> PageBuilder<NoPageID, NoPage, NoEndpoint, NoLanguage> {
        PageBuilder::default()
    }

    pub fn available_languages(&self) -> Option<usize> {
        if let Some(ref links) = self.language_links {
            return Some(links.len());
        }
        None
    }

    pub fn sections(&self) -> Option<&Vec<Section>> {
        if let Some(ref sections) = self.sections {
            return Some(sections);
        }
        None
    }
}

impl std::fmt::Debug for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Page")
            .field("title", &self.title)
            .field("pageid", &self.pageid)
            .field("content", &self.content)
            .field("language", &self.language)
            .field("language_links", &self.language_links.is_some())
            .field("sections", &self.sections.is_some())
            .field("revision_id", &self.revision_id)
            .finish()
    }
}

#[derive(Clone)]
/// Which pieces of information to get about the article
pub enum Property {
    /// Gives the parsed text of the wikitext
    Text,
    /// Gives the language links in the parsed wikitext
    LangLinks,
    /// Gives the categories in the parsed wikitext
    Categories,
    /// Gives the HTML version of the categories
    CategoriesHTML,
    /// Gives the templates in the parsed wikitext
    Templates,
    /// Gives the images in the parsed wikitext
    Images,
    /// Gives the external links in the parsed wikitext
    ExternalLinks,
    /// Gives the sections in the parsed wikitext
    Sections,
    /// Adds the revision ID of the parsed page
    RevID,
    /// Adds the title of the parsed wikitext
    DisplayTitle,
    /// Adds the page subtitle for the parsed page
    Subtitle,
    /// Gives parsed doctype, opening `<html>`, `<head>` and opening `<body>` of the page
    HeadHTML,
    /// Gives the HTML of page status indicators used on the page
    Indicators,
    /// Gives interwiki links in the parsed wikitext
    InterwikiLinks,
    /// Gives the original wikitext that was parsed
    Wikitext,
    /// Gives various properties defined in the parsed wikitext
    Properties,
    /// Gives the limit report in a structured way
    LimitReportData,
    /// Gives the HTML version of the limit report
    LimitReportHTML,
    /// The XML parse tree of revision content (requires content model `wikitext`)
    ParseTree,
    /// Gives the warnings that occurred while parsing content (as wikitext)
    ParseWarnings,
    /// Gives the warnings that occurred while parsing content (as HTML)
    ParseWarningsHTML,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Text => write!(f, "text"),
            Property::LangLinks => write!(f, "langlinks"),
            Property::Categories => write!(f, "categories"),
            Property::CategoriesHTML => write!(f, "categorieshtml"),
            Property::Templates => write!(f, "templates"),
            Property::Images => write!(f, "images"),
            Property::ExternalLinks => write!(f, "externallinks"),
            Property::Sections => write!(f, "sections"),
            Property::RevID => write!(f, "revid"),
            Property::DisplayTitle => write!(f, "displaytitle"),
            Property::Subtitle => write!(f, "subtitle"),
            Property::HeadHTML => write!(f, "headhtml"),
            Property::Indicators => write!(f, "indicators"),
            Property::InterwikiLinks => write!(f, "iwlinks"),
            Property::Wikitext => write!(f, "wikitext"),
            Property::Properties => write!(f, "properties"),
            Property::LimitReportData => write!(f, "limitreportdata"),
            Property::LimitReportHTML => write!(f, "limitreporthtml"),
            Property::ParseTree => write!(f, "parsetree"),
            Property::ParseWarnings => write!(f, "parsewarnings"),
            Property::ParseWarningsHTML => write!(f, "parsewarningshtml"),
        }
    }
}

pub struct WithPageID(usize);
#[derive(Default)]
pub struct NoPageID;

pub struct WithPage(String);
#[derive(Default)]
pub struct NoPage;

pub struct WithEndpoint(Url);
#[derive(Default)]
pub struct NoEndpoint;

pub struct WithLanguage(Language);
#[derive(Default)]
pub struct NoLanguage;

#[derive(Default)]
pub struct PageBuilder<I, P, E, L> {
    pageid: I,
    page: P,
    endpoint: E,
    language: L,
    revision: Option<usize>,
    redirects: Option<bool>,
    properties: Option<Vec<Property>>,
}

pub type PageRequest = PageBuilder<NoPageID, WithPage, WithEndpoint, WithLanguage>;
pub type PageRequestID = PageBuilder<WithPageID, NoPage, WithEndpoint, WithLanguage>;

impl<E, L> PageBuilder<NoPageID, NoPage, E, L> {
    /// Parse content of this page
    pub fn pageid(self, pageid: usize) -> PageBuilder<WithPageID, NoPage, E, L> {
        PageBuilder {
            pageid: WithPageID(pageid),
            page: self.page,
            endpoint: self.endpoint,
            revision: self.revision,
            redirects: self.redirects,
            properties: self.properties,
            language: self.language,
        }
    }

    /// Parse content of this page
    pub fn page(self, page: impl Into<String>) -> PageBuilder<NoPageID, WithPage, E, L> {
        PageBuilder {
            pageid: self.pageid,
            page: WithPage(page.into()),
            endpoint: self.endpoint,
            revision: self.revision,
            redirects: self.redirects,
            properties: self.properties,
            language: self.language,
        }
    }
}

impl<I, P, L> PageBuilder<I, P, NoEndpoint, L> {
    pub fn url(self, url: impl Into<Url>) -> PageBuilder<I, P, WithEndpoint, L> {
        PageBuilder {
            pageid: self.pageid,
            page: self.page,
            endpoint: WithEndpoint(url.into()),
            revision: self.revision,
            redirects: self.redirects,
            properties: self.properties,
            language: self.language,
        }
    }

    pub fn endpoint(self, endpoint: Url) -> PageBuilder<I, P, WithEndpoint, L> {
        PageBuilder {
            pageid: self.pageid,
            page: self.page,
            endpoint: WithEndpoint(endpoint),
            revision: self.revision,
            redirects: self.redirects,
            properties: self.properties,
            language: self.language,
        }
    }
}

impl<I, P, E> PageBuilder<I, P, E, NoLanguage> {
    pub fn language(self, language: Language) -> PageBuilder<I, P, E, WithLanguage> {
        PageBuilder {
            pageid: self.pageid,
            page: self.page,
            endpoint: self.endpoint,
            language: WithLanguage(language),
            revision: self.revision,
            redirects: self.redirects,
            properties: self.properties,
        }
    }
}

impl<I, P, U, L> PageBuilder<I, P, U, L> {
    /// Revision ID, for `{{REVISIONID}}` and similar variables
    pub fn revision(mut self, revision: usize) -> Self {
        self.revision = Some(revision);
        self
    }

    /// If page or pageid is set to a redirect, resolve it
    pub fn redirects(mut self, redirects: bool) -> Self {
        self.redirects = Some(redirects);
        self
    }

    /// Which pieces of information to get
    pub fn properties(mut self, properties: Vec<Property>) -> Self {
        self.properties = Some(properties);
        self
    }
}

impl<I, P> PageBuilder<I, P, WithEndpoint, WithLanguage> {
    async fn fetch_with_params(self, mut params: Vec<(&str, String)>) -> Result<Page> {
        async fn action_parse(params: Vec<(&str, String)>, endpoint: Url) -> Result<Response> {
            Client::new()
                .get(endpoint)
                .header("User-Agent", "wiki-tui/0.9.1")
                .query(&[
                    ("action", "parse"),
                    ("format", "json"),
                    ("formatversion", "2"),
                    ("parsoid", "true"),
                ])
                .query(&params)
                .send()
                .await
                .inspect(|response| {
                    debug!("response url: '{}'", response.url().as_str());
                })
                .context("failed sending the request")
        }

        if let Some(revision) = self.revision {
            params.push(("revid", revision.to_string()));
        }

        if let Some(redirects) = self.redirects {
            params.push(("redirects", redirects.to_string()));
        }

        if let Some(ref prop) = self.properties {
            let mut prop_str = String::new();
            for prop in prop {
                prop_str.push('|');
                prop_str.push_str(&prop.to_string())
            }
            params.push(("prop", prop_str));
        }

        let response = action_parse(params, self.endpoint.0.clone())
            .await?
            .error_for_status()
            .context("the server returned an error")?;

        let res_json: serde_json::Value = serde_json::from_str(
            &response
                .text()
                .await
                .context("failed reading the response")?,
        )
        .context("failed interpreting the response as json")?;

        self.serialize_result(res_json)
            .context("failed serializing the returned response")
    }

    fn serialize_result(self, res_json: serde_json::Value) -> Result<Page> {
        let title = res_json
            .get("parse")
            .and_then(|x| x.get("title"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_string())
            .ok_or_else(|| anyhow!("missing the title"))?;

        let pageid = res_json
            .get("parse")
            .and_then(|x| x.get("pageid"))
            .and_then(|x| x.as_u64())
            .map(|x| x as usize)
            .ok_or_else(|| anyhow!("missing the pageid"))?;

        let endpoint = self.endpoint.0;
        let language = self.language.0;
        let content = res_json
            .get("parse")
            .and_then(|x| x.get("text"))
            .and_then(|x| x.as_str())
            .map(|x| {
                let parser = WikipediaParser::parse_document(x, endpoint.clone(), language);
                Document {
                    nodes: parser.nodes(),
                }
            })
            // HACK: implement correct errors
            .ok_or(anyhow!("missing the content or failed parsing the content"))?;

        let language_links = res_json
            .get("parse")
            .and_then(|x| x.get("langlinks"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| {
                        let language_int: LanguageLinkInt = serde_json::from_value(x)
                            .map_err(|err| warn!("language_link parsing error: {:?}", err))
                            .ok()?;
                        let mut endpoint = endpoint.clone();
                        let _ = endpoint.set_host(Some(language_int.url.host_str().unwrap()));
                        Some(LanguageLink {
                            name: language_int.name,
                            language: language_int.language,
                            autonym: language_int.autonym,
                            title: language_int.title,
                            url: language_int.url,
                            endpoint,
                        })
                    })
                    .collect::<Vec<LanguageLink>>()
            })
            .inspect(|x| {
                debug!("language_links: '{}'", x.len());
            });

        let sections = res_json
            .get("parse")
            .and_then(|x| x.get("sections"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        serde_json::from_value(x).ok().map(|mut x: Section| {
                            x.index = i + 1;
                            // TODO: render html tags in the toc
                            let fragment = Html::parse_document(&x.text);
                            x.text = fragment.root_element().text().collect();
                            x
                        })
                    })
                    .collect::<Vec<Section>>()
            })
            .map(|mut x| {
                x.insert(
                    0,
                    Section {
                        index: 0,
                        header_kind: HeaderKind::Main,
                        text: "(Top)".to_string(),
                        number: "".to_string(),
                        anchor: "Content_Top".to_string(),
                    },
                );
                x
            });

        let revision_id = res_json
            .get("parse")
            .and_then(|x| x.get("revid"))
            .and_then(|x| x.as_u64())
            .map(|x| x as usize);

        Ok(Page {
            title,
            pageid,
            content,
            language,
            language_links,
            sections,
            revision_id,
        })
    }
}

impl PageBuilder<WithPageID, NoPage, WithEndpoint, WithLanguage> {
    pub async fn fetch(self) -> Result<Page> {
        let param = vec![("pageid", self.pageid.0.to_string())];
        self.fetch_with_params(param).await
    }
}

impl PageBuilder<NoPageID, WithPage, WithEndpoint, WithLanguage> {
    pub async fn fetch(self) -> Result<Page> {
        let param = vec![("page", self.page.0.to_string())];
        self.fetch_with_params(param).await
    }
}
