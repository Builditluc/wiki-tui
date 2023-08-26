use std::str::FromStr;

use anyhow::{Context, Result};
use html5ever::{tendril::StrTendril, QualName};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use snafu::Snafu;
use url::Url;

use crate::{
    config,
    wiki::{
        article::link_data::{AnchorData, ExternalData, InternalData, RedLinkData},
        search::Namespace,
    },
};

use super::{article::Link, language::Language};

#[derive(Debug, Clone)]
pub enum Data {
    Section { id: usize },
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Raw {
    pub index: usize,
    pub parent: Option<usize>,
    pub prev: Option<usize>,
    pub next: Option<usize>,
    pub first_child: Option<usize>,
    pub last_child: Option<usize>,
    pub data: Data,
}

#[derive(Copy, Debug, Clone)]
pub struct Node<'a> {
    page: &'a Page,
    index: usize,
}

impl<'a> Node<'a> {
    pub fn new(page: &'a Page, index: usize) -> Option<Self> {
        if page.nodes.len() > index {
            return Some(Node { page, index });
        }
        None
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn raw(&self) -> &'a Raw {
        &self.page.nodes[self.index]
    }

    pub fn data(&self) -> &'a Data {
        &self.raw().data
    }

    pub fn parent(&self) -> Option<Node<'a>> {
        self.raw().parent.map(|index| self.page.nth(index).unwrap())
    }

    pub fn prev(&self) -> Option<Node<'a>> {
        self.raw().prev.map(|index| self.page.nth(index).unwrap())
    }

    pub fn next(&self) -> Option<Node<'a>> {
        self.raw().next.map(|index| self.page.nth(index).unwrap())
    }

    pub fn first_child(&self) -> Option<Node<'a>> {
        self.raw()
            .first_child
            .map(|index| self.page.nth(index).unwrap())
    }

    pub fn last_child(&self) -> Option<Node<'a>> {
        self.raw()
            .last_child
            .map(|index| self.page.nth(index).unwrap())
    }

    pub fn children(&self) -> Children<'a> {
        Children {
            page: self.page,
            next: self.first_child(),
        }
    }

    pub fn descendants(&self) -> Descendants<'a> {
        Descendants {
            start: *self,
            current: *self,
            done: false,
        }
    }
}

pub struct Children<'a> {
    page: &'a Page,
    next: Option<Node<'a>>,
}

impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Node<'a>> {
        if let Some(next) = self.next {
            self.next = next.next();
            return Some(next);
        }
        None
    }
}

pub struct Descendants<'a> {
    start: Node<'a>,
    current: Node<'a>,
    done: bool,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Node<'a>> {
        if self.done {
            return None;
        }

        if self.start.index() == self.current.index() {
            if let Some(first_child) = self.current.first_child() {
                self.current = first_child;
            } else {
                self.done = true;
                return None;
            }
        } else {
            if let Some(first_child) = self.current.first_child() {
                self.current = first_child;
            } else if let Some(next) = self.current.next() {
                self.current = next;
            } else {
                loop {
                    let parent = self.current.parent().unwrap();
                    if parent.index() == self.start.index() {
                        self.done = true;
                        return None;
                    }
                    if let Some(next) = parent.next() {
                        self.current = next;
                        break;
                    }
                    self.current = parent;
                }
            }
        }

        Some(self.current)
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub nodes: Vec<Raw>,
}

impl Page {
    pub fn nth(&self, n: usize) -> Option<Node> {
        Node::new(self, n)
    }

    pub fn from_string(str: &str) -> Page {
        use html5ever::parse_document;
        use html5ever::tendril::stream::TendrilSink;

        let tendril = StrTendril::from(str);
        let rc_dom = parse_document(RcDom::default(), Default::default()).one(tendril);
        return Parser::parse_rc_dom(rc_dom);
    }
}

macro_rules! has_attribute {
    ($attrs: expr, $attr: expr) => {
        $attrs
            .iter()
            .find(|(name, _)| name.local.as_ref() == $attr)
            .is_some()
    };
}

macro_rules! attribute {
    ($attrs: expr, $attr: expr) => {
        $attrs.find(|(name, _)| name.local.as_ref() == $attr)
    };
}

mod attrs {
    pub const SECTION_ID_ATTR: &str = "data-mw-section-id";
}

struct Parser {
    nodes: Vec<Raw>,
}

impl Parser {
    fn parse_rc_dom(rc_dom: RcDom) -> Page {
        let mut parser = Parser { nodes: Vec::new() };
        parser.parse_node(&rc_dom.document, None, None);
        Page {
            nodes: parser.nodes,
        }
    }

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
                    prev = self.parse_node(child, None, prev)
                }
                None
            }
            NodeData::Text { ref contents } => Some(self.push_node(Data::Unknown, parent, prev)),
            NodeData::Element {
                ref name,
                ref attrs,
                ..
            } => {
                let attributes: Vec<(QualName, StrTendril)> = attrs
                    .borrow()
                    .iter()
                    .map(|attr| (attr.name.clone(), attr.value.clone()))
                    .collect();

                let index = match name.local.as_ref() {
                    "section" if has_attribute!(attributes, attrs::SECTION_ID_ATTR) => {
                        self.parse_section(parent, prev, attributes.iter())
                    }
                    _ => self.push_node(Data::Unknown, parent, prev),
                };
                let mut prev = None;
                for child in node.children.borrow().iter() {
                    prev = self.parse_node(child, Some(index), prev)
                }
                Some(index)
            }
            NodeData::Doctype { .. }
            | NodeData::Comment { .. }
            | NodeData::ProcessingInstruction { .. } => None,
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
        parent: Option<usize>,
        prev: Option<usize>,
        mut attributes: impl Iterator<Item = &'a (QualName, StrTendril)>,
    ) -> usize {
        let id = attribute!(attributes, attrs::SECTION_ID_ATTR).and_then(|(_, id_str)| {
            usize::from_str(id_str)
                .map_err(|err| warn!("section_id not a number"))
                .ok()
        });

        if id.is_none() {
            return self.push_node(Data::Unknown, parent, prev);
        }

        self.push_node(Data::Section { id: id.unwrap() }, parent, prev)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Snafu)]
pub enum ParsingError {
    #[snafu(display("The link leads to an invalid namespace: '{namespace}'"))]
    InvalidNamespace { namespace: String },

    #[snafu(display("The link is missing data: '{data}'"))]
    MissingData { data: String },

    #[snafu(display("Error while processing the link: '{process}'"))]
    ProcessingFailure { process: String },

    #[snafu(display("Link is not UTF-8 encoded"))]
    InvalidEncoding,
}

fn parse_href_to_link(
    endpoint: Url,
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

    if let Ok(url) = Url::parse(&href) {
        return Ok(Link::External(ExternalData { url }));
    }

    fn parse_internal_link(
        href: String,
        title: String,
        endpoint: Url,
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
                Namespace::from_str(namespace_str).ok_or(ParsingError::InvalidNamespace {
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

    use crate::wiki::{
        article::{
            link_data::{AnchorData, ExternalData, InternalData, RedLinkData},
            Link,
        },
        language::Language,
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
