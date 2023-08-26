use anyhow::{Context, Result};
use cursive::theme::Style;
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::{collections::HashMap, fmt::Display};
use syn::parse::Parser;
use url::Url;

use super::{language::Language, parser::Page, search::Namespace};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ElementType {
    Text,
    Newline,
    Link(Link),
    Header,
    Unsupported,

    ListMarker,
    ListItemStart,
    ListItemEnd,

    DescriptionListTermStart,
    DescriptionListTermEnd,
    DescriptionListDescriptionStart,
    DescriptionListDescriptionEnd,

    DisambiguationStart,
    DisambiguationEnd,
}

#[derive(Debug, Clone)]
pub struct Element {
    id: usize,
    pub kind: ElementType,
    content: String,
    style: Style,
    width: usize,
    attributes: HashMap<String, String>,
}

impl Element {
    pub fn new(
        id: usize,
        kind: ElementType,
        content: impl Into<String>,
        style: impl Into<Style>,
        attributes: HashMap<String, String>,
    ) -> Self {
        let content = content.into();
        Element {
            id,
            kind,
            width: content.len(),
            content,
            style: style.into(),
            attributes,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn style(&self) -> Style {
        self.style
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn attr(&self, name: &str) -> Option<&str> {
        match self.attributes.get(name) {
            Some(value) => Some(value),
            None => None,
        }
    }
}

pub mod link_data {
    use url::Url;

    use crate::wiki::{language::Language, search::Namespace};

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct InternalData {
        pub namespace: Namespace,
        pub page: String,
        pub title: String,
        pub endpoint: Url,
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
    /// External link to a page at another website
    External(link_data::ExternalData),
    /// External link to an interal page in the same wiki
    ExternalToInternal(link_data::ExternalToInteralData),
}

#[derive(Debug, Deserialize, Clone)]
pub struct LanguageLink {
    #[serde(rename = "langname")]
    pub name: String,
    #[serde(rename = "lang")]
    pub language: Language,
    pub autonym: String,
    pub title: String,
    pub url: Url,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    sortkey: String,
    category: String,
    hidden: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Template {
    #[serde(rename = "ns")]
    namespace: Namespace,
    title: String,
    exists: bool,
}

#[derive(Debug, Clone, Deserialize_repr)]
#[repr(usize)]
pub enum HeaderType {
    Main = 1,
    Sub = 2,
    Section = 3,
    Subsection = 4,
    Minor = 5,
    Detail = 6,
}

#[derive(Debug, Deserialize)]
pub struct Section {
    #[serde(skip_deserializing)]
    index: usize,
    #[serde(rename = "toclevel")]
    header_type: HeaderType,
    #[serde(rename = "line")]
    text: String,
    number: String,
    anchor: String,
}

impl Section {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn header_type(&self) -> HeaderType {
        self.header_type.clone()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn anchor(&self) -> &str {
        &self.anchor
    }
}

#[derive(Debug, Deserialize)]
pub struct InterwikiLink {
    prefix: String,
    url: String,
    title: String,
}

#[derive(Debug)]
pub struct LimitReportData {
    name: String,
    data: Vec<String>,
}

#[derive(Debug)]
pub struct Article {
    title: String,
    pageid: usize,
    content: Page,
    pub language: Language,
    pub language_links: Option<Vec<LanguageLink>>,
    categories: Option<Vec<Category>>,
    categories_html: Option<String>,
    templates: Option<Vec<Template>>,
    images: Option<Vec<String>>,
    external_links: Option<Vec<String>>,
    sections: Option<Vec<Section>>,
    revision_id: Option<usize>,
    display_title: Option<String>,
    subtitle: Option<String>,
    head_html: Option<String>,
    indicators: Option<String>,
    interwiki_links: Option<Vec<InterwikiLink>>,
    wikitext: Option<String>,
    properties: Option<HashMap<String, String>>,
    limit_report_data: Option<Vec<LimitReportData>>,
    limit_report_html: Option<String>,
    parse_tree: Option<String>,
    parse_warnings: Option<Vec<String>>,
    parse_warnings_html: Option<Vec<String>>,
}

impl Article {
    pub fn builder() -> ArticleBuilder<NoPageID, NoPage, NoEndpoint, NoLanguage> {
        ArticleBuilder::default()
    }

    pub fn content(&self) -> &Page {
        &self.content
    }

    pub fn sections(&self) -> Option<impl Iterator<Item = &Section>> {
        self.sections.as_ref().map(|x| x.iter())
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn available_languages(&self) -> Option<usize> {
        if let Some(ref links) = self.language_links {
            return Some(links.len());
        }
        None
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
pub struct ArticleBuilder<I, P, E, L> {
    pageid: I,
    page: P,
    endpoint: E,
    language: L,
    revision: Option<usize>,
    redirects: Option<bool>,
    properties: Option<Vec<Property>>,
}

impl<E, L> ArticleBuilder<NoPageID, NoPage, E, L> {
    /// Parse content of this page
    pub fn pageid(self, pageid: usize) -> ArticleBuilder<WithPageID, NoPage, E, L> {
        ArticleBuilder {
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
    pub fn page(self, page: impl Into<String>) -> ArticleBuilder<NoPageID, WithPage, E, L> {
        ArticleBuilder {
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

impl<I, P, L> ArticleBuilder<I, P, NoEndpoint, L> {
    pub fn url(self, url: impl Into<Url>) -> ArticleBuilder<I, P, WithEndpoint, L> {
        ArticleBuilder {
            pageid: self.pageid,
            page: self.page,
            endpoint: WithEndpoint(url.into()),
            revision: self.revision,
            redirects: self.redirects,
            properties: self.properties,
            language: self.language,
        }
    }

    pub fn endpoint(self, endpoint: Url) -> ArticleBuilder<I, P, WithEndpoint, L> {
        ArticleBuilder {
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

impl<I, P, E> ArticleBuilder<I, P, E, NoLanguage> {
    pub fn language(self, language: Language) -> ArticleBuilder<I, P, E, WithLanguage> {
        ArticleBuilder {
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

impl<I, P, U, L> ArticleBuilder<I, P, U, L> {
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

impl<I, P> ArticleBuilder<I, P, WithEndpoint, WithLanguage> {
    fn fetch_with_params(self, mut params: Vec<(&str, String)>) -> Result<Article> {
        fn action_parse(params: Vec<(&str, String)>, endpoint: Url) -> Result<Response> {
            Client::new()
                .get(endpoint)
                .query(&[
                    ("action", "parse"),
                    ("format", "json"),
                    ("formatversion", "2"),
                    ("parsoid", "true"),
                ])
                .query(&params)
                .send()
                .map(|response| {
                    debug!("response url: '{}'", response.url().as_str());
                    response
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

        let response = action_parse(params, self.endpoint.0.clone())?
            .error_for_status()
            .context("the server returned an error")?;

        let res_json: serde_json::Value =
            serde_json::from_str(&response.text().context("failed reading the response")?)
                .context("failed interpreting the response as json")?;

        self.serialize_result(res_json)
            .context("failed serializing the returned response")
    }

    fn serialize_result(self, res_json: serde_json::Value) -> Result<Article> {
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

        let language_links = res_json
            .get("parse")
            .and_then(|x| x.get("langlinks"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| {
                        serde_json::from_value(x)
                            .map_err(|err| warn!("language_link parsing error: {:?}", err))
                            .ok()
                    })
                    .collect::<Vec<LanguageLink>>()
            })
            .map(|x| {
                debug!("language_links: '{}'", x.len());
                x
            });

        let categories = res_json
            .get("parse")
            .and_then(|x| x.get("categories"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| serde_json::from_value(x).ok())
                    .collect::<Vec<Category>>()
            });

        let categories_html = res_json
            .get("parse")
            .and_then(|x| x.get("categorieshtml"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_owned());

        let templates = res_json
            .get("parse")
            .and_then(|x| x.get("templates"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| serde_json::from_value(x).ok())
                    .collect::<Vec<Template>>()
            });

        let images = res_json
            .get("parse")
            .and_then(|x| x.get("images"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| x.as_str().map(|x| x.to_owned()))
                    .collect::<Vec<String>>()
            });

        let external_links = res_json
            .get("parse")
            .and_then(|x| x.get("externallinks"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| x.as_str().map(|x| x.to_owned()))
                    .collect::<Vec<String>>()
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
                            // let doc = Document::from(x.text());
                            // // TODO: render html tags in the toc
                            // if let Some(text) = doc.nth(0).map(|x| x.text()) {
                            //     x.text = text;
                            // }
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
                        header_type: HeaderType::Main,
                        text: "(Top)".to_string(),
                        number: "".to_string(),
                        anchor: "Content_Top".to_string(),
                    },
                );
                x
            });

        let content = res_json
            .get("parse")
            .and_then(|x| x.get("text"))
            .and_then(|x| x.as_str())
            .map(|x| Page::from_string(x))
            .ok_or(anyhow!("no page found"))?;

        let revision_id = res_json
            .get("parse")
            .and_then(|x| x.get("revid"))
            .and_then(|x| x.as_u64())
            .map(|x| x as usize);

        let display_title = res_json
            .get("parse")
            .and_then(|x| x.get("displaytitle"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());

        let subtitle = res_json
            .get("parse")
            .and_then(|x| x.get("subtitle"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());

        let head_html = res_json
            .get("parse")
            .and_then(|x| x.get("headhtml"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());

        let interwiki_links = res_json
            .get("parse")
            .and_then(|x| x.get("iwlinks"))
            .and_then(|x| x.as_array())
            .map(|x| x.to_owned())
            .map(|x| {
                x.into_iter()
                    .filter_map(|x| serde_json::from_value(x).ok())
                    .collect::<Vec<InterwikiLink>>()
            });

        let wikitext = res_json
            .get("parse")
            .and_then(|x| x.get("wikitext"))
            .and_then(|x| x.as_str())
            .map(|x| x.to_string());

        Ok(Article {
            title,
            pageid,
            content,
            language: self.language.0,
            language_links,
            categories,
            categories_html,
            templates,
            images,
            external_links,
            sections,
            revision_id,
            display_title,
            subtitle,
            head_html,
            indicators: None,
            interwiki_links,
            wikitext,
            properties: None,
            limit_report_data: None,
            limit_report_html: None,
            parse_tree: None,
            parse_warnings: None,
            parse_warnings_html: None,
        })
    }
}

impl ArticleBuilder<WithPageID, NoPage, WithEndpoint, WithLanguage> {
    pub fn fetch(self) -> Result<Article> {
        let param = vec![("pageid", self.pageid.0.to_string())];
        self.fetch_with_params(param)
    }
}

impl ArticleBuilder<NoPageID, WithPage, WithEndpoint, WithLanguage> {
    pub fn fetch(self) -> Result<Article> {
        let param = vec![("page", self.page.0.to_string())];
        self.fetch_with_params(param)
    }
}
