use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use snafu::Snafu;
use std::str::FromStr;
use tracing::{debug, trace, warn};

use crate::{
    document::{Data, HeaderKind, Raw},
    languages::Language,
    page::{
        link_data::{AnchorData, ExternalData, InternalData, RedLinkData},
        Link,
    },
    search::Namespace,
    Endpoint,
};

// TODO: remove Parser and replace it with normal functions and helper functions
pub trait Parser {
    fn parse_document(document: &str, endpoint: Endpoint, language: Language) -> Self;
    fn nodes(self) -> Vec<Raw>;
}

pub struct WikipediaParser {
    endpoint: Endpoint,
    language: Language,
    nodes: Vec<Raw>,
}

impl WikipediaParser {
    fn parse_node(
        &mut self,
        node: &Handle,
        parent: Option<usize>,
        prev: Option<usize>,
    ) -> Option<usize> {
        match node.data {
            NodeData::Document => {
                let mut prev = None;
                for child in node.children.borrow().iter() {
                    prev = self.parse_node(child, parent, prev)
                }
                None
            }
            NodeData::Text { ref contents } => {
                let data = Data::Text {
                    contents: contents.borrow().to_string(),
                };
                Some(self.push_node(data, parent, prev))
            }
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                let name = name.local.to_string();
                let attrs: Vec<(String, String)> = attrs
                    .borrow()
                    .iter()
                    .map(|attr| (attr.name.local.to_string(), attr.value.to_string()))
                    .collect();

                let data = match name.as_str() {
                    "head" | "style" | "link" => return prev,

                    "table" | "img" | "figure" => {
                        warn!("unsupported node '{name}'");
                        return prev;
                    }

                    "ul" if attrs.iter().any(|(name, value)| {
                        name.as_str() == "class" && value.contains("portalbox")
                    }) =>
                    {
                        trace!("ignoring 'ul' class: 'portalbox'");
                        return prev;
                    }

                    "div"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class"
                                && (value.contains("toc") || value.contains("quotebox"))
                        }) =>
                    {
                        trace!("ignoring 'div': class: 'toc' || 'quotebox'");
                        return prev;
                    }

                    "div"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class" && value.contains("mw-empty-elt")
                        }) =>
                    {
                        trace!("ignoring 'div': class: 'mw-empty-elt'");
                        return prev;
                    }

                    "span"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class" && value.contains("cs1-maint")
                        }) =>
                    {
                        trace!("ignoring 'span': class: 'cs1-maint'");
                        return prev;
                    }

                    _ if attrs.iter().any(|(name, value)| {
                        name.as_str() == "class" && value.contains("noprint")
                    }) =>
                    {
                        trace!("ignoring '{name}': class: 'noprint'");
                        return prev;
                    }

                    "span"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class" && value.contains("mw-editsection")
                        }) =>
                    {
                        trace!("ignoring 'span': class: 'mw-editsection'");
                        return prev;
                    }

                    "span"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "typeof" && value.contains("mw:Nowiki")
                        }) =>
                    {
                        trace!("ignoring 'span': class: 'mw:Nowiki'");
                        return prev;
                    }

                    "span"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class" && value.contains("mw-reflink-text")
                        }) =>
                    {
                        Data::Reflink
                    }

                    "section" => self.parse_section(attrs.iter()).unwrap_or_default(),
                    "h1" => self
                        .parse_header(attrs.iter(), HeaderKind::Main)
                        .unwrap_or_default(),

                    "h2" => self
                        .parse_header(attrs.iter(), HeaderKind::Sub)
                        .unwrap_or_default(),
                    "h3" => self
                        .parse_header(attrs.iter(), HeaderKind::Section)
                        .unwrap_or_default(),
                    "h4" => self
                        .parse_header(attrs.iter(), HeaderKind::Subsection)
                        .unwrap_or_default(),
                    "h5" => self
                        .parse_header(attrs.iter(), HeaderKind::Minor)
                        .unwrap_or_default(),
                    "h6" => self
                        .parse_header(attrs.iter(), HeaderKind::Detail)
                        .unwrap_or_default(),

                    "ol" => Data::OrderedList,
                    "ul" => Data::UnorderedList,
                    "li" => Data::ListItem,

                    "dl" => Data::DescriptionList,
                    "dt" => Data::DescriptionListTerm,
                    "dd" => Data::DerscriptionListDescription,

                    "b" => Data::Bold,
                    "i" => Data::Italic,

                    "p" => Data::Paragraph,
                    "span" => Data::Span,

                    "div"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class" && value.contains("redirectMsg")
                        }) =>
                    {
                        Data::RedirectMessage
                    }

                    "div"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class" && value.contains("hatnote")
                        }) =>
                    {
                        Data::Disambiguation
                    }

                    "a" => self.parse_link(attrs.iter()).unwrap_or_default(),

                    /*
                    "a" if attrs.iter().any(|(name, value)| {
                        name.as_str() == "rel" && value.as_str() == "mw:WikiLink"
                    }) =>
                    {
                        self.parse_wiki_link(attrs.iter()).unwrap_or_default()
                    }

                    "a" if attrs.iter().any(|(name, value)| {
                        name.as_str() == "rel" && value.as_str() == "mw:MediaLink"
                    }) =>
                    {
                        self.parse_media_link(attrs.iter()).unwrap_or_default()
                    }

                    "a" if attrs.iter().any(|(name, value)| {
                        name.as_str() == "rel" && value.as_str() == "mw:ExtLink"
                    }) =>
                    {
                        self.parse_external_link(attrs.iter()).unwrap_or_default()
                    }
                    */
                    "div" => Data::Division,
                    _ => {
                        warn!("unknown node '{name}'");
                        Data::Unknown
                    }
                };

                let index = self.push_node(data, parent, prev);
                let mut prev = None;
                for child in node.children.borrow().iter() {
                    prev = self.parse_node(child, Some(index), prev)
                }
                Some(index)
            }
            NodeData::ProcessingInstruction { .. }
            | NodeData::Doctype { .. }
            | NodeData::Comment { .. } => prev,
        }
    }

    fn push_node(&mut self, data: Data, parent: Option<usize>, prev: Option<usize>) -> usize {
        let index = self.nodes.len();

        self.nodes.push(Raw {
            index,
            parent,
            prev,
            next: None,
            first_child: None,
            last_child: None,
            data,
        });

        if let Some(parent) = parent {
            let parent = &mut self.nodes[parent];
            if parent.first_child.is_none() {
                parent.first_child = Some(index);
            }
            parent.last_child = Some(index);
        }

        if let Some(prev) = prev {
            self.nodes[prev].next = Some(index);
        }

        index
    }

    fn parse_section<'a>(
        &mut self,
        mut attrs: impl Iterator<Item = &'a (String, String)>,
    ) -> Option<Data> {
        let section_id = attrs
            .find(|(name, _)| name.as_str() == "data-mw-section-id")
            .map(|(_, value)| value)?;
        let section_id = usize::from_str(section_id)
            .map_err(|err| warn!("section-id not a usize, '{err:?}'"))
            .ok()?;

        Some(Data::Section { id: section_id })
    }

    fn parse_header<'a>(
        &mut self,
        mut attrs: impl Iterator<Item = &'a (String, String)>,
        kind: HeaderKind,
    ) -> Option<Data> {
        let header_id = attrs
            .find(|(name, _)| name.as_str() == "id")
            .map(|(_, value)| value.to_owned())?;

        Some(Data::Header {
            id: header_id,
            kind,
        })
    }

    fn parse_link<'a>(
        &mut self,
        mut attrs: impl Iterator<Item = &'a (String, String)>,
    ) -> Option<Data> {
        let href = attrs
            .find(|(name, _)| name.as_str() == "href")
            .map(|(_, value)| value.to_owned())?;

        let title = attrs
            .find(|(name, _)| name.as_str() == "title")
            .map(|(_, value)| value.to_owned());

        let link = match parse_href_to_link(
            self.endpoint.clone(),
            href.strip_prefix("//en.wikipedia.org").unwrap_or_default(),
            title,
            self.language.clone(),
        ) {
            Ok(link) => link,
            Err(error) => {
                warn!("{:?}", error);
                return None;
            }
        };

        Some(Data::Link(link))
    }
}

impl Parser for WikipediaParser {
    fn parse_document(document: &str, endpoint: Endpoint, language: Language) -> Self {
        let mut parser = WikipediaParser {
            nodes: Vec::new(),
            endpoint,
            language,
        };

        let rc_dom = parse_document(RcDom::default(), Default::default()).one(document);
        parser.parse_node(&rc_dom.document, None, None);

        parser
    }

    fn nodes(self) -> Vec<Raw> {
        self.nodes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
enum ParsingError {
    #[snafu(display("The link leads to an invalid namespace: '{namespace}"))]
    InvalidNamespace { namespace: String },
    #[snafu(display("The link is missing data: '{data}'"))]
    MissingData { data: String },
    #[snafu(display("Error while processing the link: '{process}'"))]
    ProcessingFailure { process: String },
    #[snafu(display("Link is not UTF-8 encoded"))]
    InvalidEncoding,
}

fn parse_href_to_link(
    endpoint: Endpoint,
    href: impl Into<String>,
    title: Option<impl Into<String>>,
    language: Language,
) -> Result<Link, ParsingError> {
    let href: String = match urlencoding::decode(&href.into()) {
        Ok(href) => href.into_owned(),
        Err(_) => return Err(ParsingError::InvalidEncoding),
    };

    let title: Option<String> = title.map(|title| title.into());

    debug!("parsing the link '{}'", href);
    debug!("link title: '{:?}'", title);
    debug!("link endpoint: '{}'", endpoint.as_str());

    // the prefix /wiki/ indicates that the link is a internal link
    const INTERNAL_LINK_PREFIX: &str = "/wiki/";
    // the character used to separate the namespace and the page
    const NAMESPACE_DELIMITER: char = ':';
    // the character used to separate the page and the anchor
    const ANCHOR_DELIMITER: char = '#';
    // the parameter indicating a redlink (non existent link)
    const REDLINK_PARAM: &str = "redlink=1";

    if href.starts_with(INTERNAL_LINK_PREFIX) {
        let title = title.ok_or(ParsingError::MissingData {
            data: "title".to_string(),
        })?;
        return parse_internal_link(href, title, endpoint, language);
    }

    if href.starts_with(ANCHOR_DELIMITER) {
        let anchor_str =
            href.strip_prefix(ANCHOR_DELIMITER)
                .ok_or(ParsingError::ProcessingFailure {
                    process: "removing ANCHOR_DELIMITER prefix".to_string(),
                })?;
        return Ok(Link::Anchor(AnchorData {
            anchor: anchor_str.to_string(),
            title: anchor_str.replace('_', " "),
        }));
    }

    if href.contains(REDLINK_PARAM) {
        let url = endpoint
            .join(&href)
            .map_err(|_| ParsingError::ProcessingFailure {
                process: "joining endpoint and href for REDLINK".to_string(),
            })?;
        let title = title.ok_or(ParsingError::MissingData {
            data: "title".to_string(),
        })?;
        return Ok(Link::RedLink(RedLinkData { url, title }));
    }

    if let Ok(url) = Endpoint::parse(&href) {
        return Ok(Link::External(ExternalData { url }));
    }

    fn parse_internal_link(
        href: String,
        title: String,
        endpoint: Endpoint,
        language: Language,
    ) -> Result<Link, ParsingError> {
        let mut href =
            href.strip_prefix(INTERNAL_LINK_PREFIX)
                .ok_or(ParsingError::ProcessingFailure {
                    process: "removing INTERNAL_LINK_PREFIX".to_string(),
                })?;
        let mut namespace = Namespace::Main;
        let mut anchor: Option<AnchorData> = None;

        if href.contains(NAMESPACE_DELIMITER) {
            debug!("link contains a namespace");
            let (namespace_str, href_split) =
                href.split_once(NAMESPACE_DELIMITER)
                    .ok_or(ParsingError::ProcessingFailure {
                        process: "splitting at NAMESPACE_DELIMITER".to_string(),
                    })?;

            href = href_split;
            namespace =
                Namespace::from_string(namespace_str).ok_or(ParsingError::InvalidNamespace {
                    namespace: namespace_str.to_string(),
                })?;

            debug!("link namespace: '{}'", namespace);
        }

        if href.contains(ANCHOR_DELIMITER) {
            debug!("link contains an anchor");
            let (page_ref, anchor_str) =
                href.split_once(ANCHOR_DELIMITER)
                    .ok_or(ParsingError::ProcessingFailure {
                        process: "splitting at ANCHOR_DELIMITER".to_string(),
                    })?;

            href = page_ref;
            anchor = Some(AnchorData {
                anchor: anchor_str.to_string(),
                title: anchor_str.replace('_', " "),
            });

            debug!("link anchor: '{}'", anchor_str);
        }

        Ok(Link::Internal(InternalData {
            namespace,
            page: href.to_string(),
            title,
            language,
            endpoint,
            anchor,
        }))
    }

    Err(ParsingError::ProcessingFailure {
        process: "invalid link".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::{
        languages::Language,
        page::{
            link_data::{AnchorData, ExternalData, InternalData, RedLinkData},
            Link,
        },
        parser::ParsingError,
        search::Namespace,
    };

    use super::parse_href_to_link;

    const ENDPOINT: &str = "https://en.wikipedia.org/w/api.php";
    const LANGUAGE: Language = Language::English;

    fn internal_link(
        namespace: Namespace,
        page: impl Into<String>,
        title: impl Into<String>,
        endpoint: Url,
        anchor: Option<AnchorData>,
    ) -> Link {
        Link::Internal(InternalData {
            namespace,
            page: page.into(),
            title: title.into(),
            endpoint,
            anchor,
            language: LANGUAGE.clone(),
        })
    }

    fn anchor_data(anchor: impl Into<String>, title: impl Into<String>) -> AnchorData {
        AnchorData {
            anchor: anchor.into(),
            title: title.into(),
        }
    }

    fn endpoint() -> Url {
        Url::parse(ENDPOINT).expect("hard-coded endpoint should be valid")
    }

    #[test]
    fn test_parse_link_unknown_namespace() {
        assert!(matches!(
            parse_href_to_link(
                endpoint(),
                "/wiki/UnknownNamespace:Main_Page",
                Some("Main Page"),
                LANGUAGE
            ),
            Err(ParsingError::InvalidNamespace { .. })
        ))
    }

    #[test]
    fn test_parse_link_invalid_link() {
        assert!(matches!(
            parse_href_to_link(endpoint(), "/invalid/hello", Some("hello"), LANGUAGE),
            Err(ParsingError::ProcessingFailure { .. })
        ))
    }

    #[test]
    fn test_parse_internal_link_no_namespace() {
        assert_eq!(
            parse_href_to_link(endpoint(), "/wiki/Main_Page", Some("Main Page"), LANGUAGE),
            Ok(internal_link(
                Namespace::Main,
                "Main_Page",
                "Main Page",
                endpoint(),
                None
            ))
        )
    }

    #[test]
    fn test_parse_internal_link_with_namespace() {
        assert_eq!(
            parse_href_to_link(
                endpoint(),
                "/wiki/Help:Contents",
                Some("Help:Contents"),
                LANGUAGE
            ),
            Ok(internal_link(
                Namespace::Help,
                "Contents",
                "Help:Contents",
                endpoint(),
                None
            ))
        );

        assert_eq!(
            parse_href_to_link(
                endpoint(),
                "/wiki/Help:Editing_pages",
                Some("Help:Editing pages"),
                LANGUAGE
            ),
            Ok(internal_link(
                Namespace::Help,
                "Editing_pages",
                "Help:Editing pages",
                endpoint(),
                None
            ))
        );
    }

    #[test]
    fn test_parse_internal_link_with_anchor() {
        assert_eq!(
            parse_href_to_link(
                endpoint(),
                "/wiki/Help:Editing_pages#Preview",
                Some("Help:Editing pages"),
                LANGUAGE
            ),
            Ok(internal_link(
                Namespace::Help,
                "Editing_pages",
                "Help:Editing pages",
                endpoint(),
                Some(anchor_data("Preview", "Preview"))
            ))
        );
    }

    #[test]
    fn test_parse_internal_link_with_anchor_whitespace() {
        assert_eq!(
            parse_href_to_link(
                endpoint(),
                "/wiki/Help:Editing_pages#See_also",
                Some("Help:Editing pages"),
                LANGUAGE
            ),
            Ok(internal_link(
                Namespace::Help,
                "Editing_pages",
                "Help:Editing pages",
                endpoint(),
                Some(anchor_data("See_also", "See also"))
            ))
        );
    }

    #[test]
    fn test_parse_internal_link_with_subpage() {
        assert_eq!(
            parse_href_to_link(
                endpoint(),
                "/wiki/Help:Links/example",
                Some("Help:Links/example"),
                LANGUAGE
            ),
            Ok(internal_link(
                Namespace::Help,
                "Links/example",
                "Help:Links/example",
                endpoint(),
                None,
            ))
        )
    }

    #[test]
    fn test_parse_anchor_link() {
        assert_eq!(
            parse_href_to_link(endpoint(), "#See_also", None::<String>, LANGUAGE),
            Ok(Link::Anchor(anchor_data("See_also", "See also")))
        )
    }

    #[test]
    fn test_parse_redlink() {
        let link = "/w/index.php?title=Help:Links/example2&action=edit&redlink=1";
        let title = "Help:Links/example2 (page does not exist)";
        assert_eq!(
            parse_href_to_link(endpoint(), link, Some(title), LANGUAGE),
            Ok(Link::RedLink(RedLinkData {
                url: endpoint().join(link).unwrap(),
                title: title.to_string(),
            }))
        )
    }

    #[test]
    fn test_parse_external_link() {
        let link = "https://mediawiki.org/";
        assert_eq!(
            parse_href_to_link(endpoint(), link, None::<String>, LANGUAGE),
            Ok(Link::External(ExternalData {
                url: Url::parse(link).expect("hard-coded url should be valid")
            }))
        );
    }

    #[test]
    fn test_parse_external_link_with_params() {
        let link = "https://google.com/search?q=link";
        assert_eq!(
            parse_href_to_link(endpoint(), link, None::<String>, LANGUAGE),
            Ok(Link::External(ExternalData {
                url: Url::parse(link).expect("hard-coded url should be valid")
            }))
        )
    }

    #[test]
    fn test_parse_external_link_with_mailto() {
        let link = "mailto:info@example.org";
        assert_eq!(
            parse_href_to_link(endpoint(), link, None::<String>, LANGUAGE),
            Ok(Link::External(ExternalData {
                url: Url::parse(link).expect("hard-coded url should be valid")
            }))
        )
    }
}
