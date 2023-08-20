use anyhow::{Context, Result};
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
        self.raw().prev.map(|index| self.page.nth(index).unwrap())
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

#[derive(Debug, Clone)]
pub struct Page {
    pub nodes: Vec<Raw>,
}

impl Page {
    pub fn nth(&self, n: usize) -> Option<Node> {
        Node::new(self, n)
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
