use std::collections::HashMap;

use anyhow::{Context, Result};
use cursive::theme::{Effect, Style};
use select::{document::Document, node::Node, predicate::Class};
use snafu::Snafu;
use url::Url;

use crate::{
    config,
    wiki::{
        article::link_data::{AnchorData, ExternalData, InternalData, RedLinkData},
        search::Namespace,
    },
};

use super::{
    article::{Element, ElementType, Link, Section},
    language::Language,
};

const SHOW_UNSUPPORTED: bool = false;
const LIST_MARKER: char = '-';

pub struct Parser<'a> {
    endpoint: Url,
    language: Language,
    elements: Vec<Element>,
    current_effects: Vec<Effect>,
    sections: Option<&'a Vec<Section>>,
}

impl<'a> Parser<'a> {
    pub fn parse_document(
        document: &'a str,
        title: &'a str,
        sections: Option<&Vec<Section>>,
        endpoint: Url,
        language: Language,
    ) -> Result<Vec<Element>> {
        let document = Document::from(document);

        let mut parser = Parser {
            endpoint,
            elements: Vec::new(),
            current_effects: Vec::new(),
            sections,
            language,
        };

        parser.elements.push(Element::new(
            parser.next_id(),
            ElementType::Header,
            title.to_string(),
            config::CONFIG.theme.title,
            {
                let mut attrs = HashMap::new();
                attrs.insert("anchor".to_string(), "Content_Top".to_string());
                attrs
            },
        ));
        parser.push_newline();
        parser.push_newline();

        let _ = &document
            .find(Class("mw-parser-output"))
            .into_selection()
            .children()
            .into_iter()
            .map(|x| parser.parse_node(x))
            .count();

        Ok(parser.elements)
    }

    fn parse_node(&mut self, node: Node) {
        let name = node.name().unwrap_or_default();
        match name {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => self.parse_header(node),
            "p" => self.parse_paragraph(node),
            "a" => self.parse_link(node),
            "b" => self.parse_effect(node, Effect::Bold),
            "i" => self.parse_effect(node, Effect::Italic),
            "ul" if node
                .attr("class")
                .map(|x| x.contains("portalbox"))
                .unwrap_or(false) => {}
            "ul" => self.parse_list(node),
            "div"
                if node
                    .attr("class")
                    .map(|x| x.contains("hatnote"))
                    .unwrap_or(false) =>
            {
                self.parse_disambiguation(node)
            }
            "div"
                if node
                    .attr("class")
                    .map(|x| x.contains("redirectMsg"))
                    .unwrap_or(false) =>
            {
                self.parse_redirect_msg(node)
            }
            "div"
                if node
                    .attr("class")
                    .map(|x| x.contains("toc") | x.contains("quotebox"))
                    .unwrap_or(false) => {}
            "" => (),
            "dl" => self.parse_description_list(node),
            "div" => {
                node.children().map(|node| self.parse_node(node)).count();
            }
            _ if SHOW_UNSUPPORTED => {
                self.elements.push(Element::new(
                    self.next_id(),
                    ElementType::Unsupported,
                    format!("<Unsupported Element '{}'>", name),
                    Effect::Italic,
                    HashMap::new(),
                ));
            }
            _ => (),
        }
    }

    fn next_id(&self) -> usize {
        self.elements.len().saturating_sub(1)
    }

    fn combine_effects(&self, mut style: Style) -> Style {
        self.current_effects.iter().for_each(|effect| {
            style = style.combine(*effect);
        });
        style
    }

    fn push_newline(&mut self) {
        self.elements.push(Element::new(
            self.next_id(),
            ElementType::Newline,
            "",
            Style::none(),
            HashMap::new(),
        ));
    }

    fn push_kind(&mut self, kind: ElementType) {
        self.elements.push(Element::new(
            self.next_id(),
            kind,
            "",
            Style::none(),
            HashMap::new(),
        ))
    }

    fn is_last_newline(&self) -> bool {
        self.elements
            .last()
            .map(|x| x.kind == ElementType::Newline)
            .unwrap_or(false)
    }

    fn parse_header(&mut self, node: Node) {
        if let Some(headline_node) = node.find(Class("mw-headline")).into_selection().first() {
            let mut attributes = HashMap::new();

            if let Some(anchor) = headline_node.attr("id") {
                attributes.insert("anchor".to_string(), anchor.to_string());
            }

            let mut header = headline_node.text();

            if let Some(sections) = self.sections {
                if let Some(section) = sections
                    .iter()
                    .find(|&section| Some(section.anchor()) == headline_node.attr("id"))
                {
                    header.insert_str(0, &format!("{} ", section.number()))
                };
            }

            self.push_newline();
            self.elements.push(Element::new(
                self.next_id(),
                ElementType::Header,
                header,
                Style::from(config::CONFIG.theme.title).combine(Effect::Bold),
                attributes,
            ));
            self.push_newline();
            self.push_newline();
        }
    }

    fn parse_paragraph(&mut self, node: Node) {
        if let Some("mw-empty-elt") = node.attr("class") {
            return;
        }
        self.parse_text(node);
        self.push_newline();
        self.push_newline();
    }

    fn parse_text(&mut self, node: Node) {
        for child in node.children() {
            if child.name().is_some() {
                self.parse_node(child);
                continue;
            }

            self.elements.push(Element::new(
                self.next_id(),
                ElementType::Text,
                child.text(),
                self.combine_effects(Style::from(config::CONFIG.theme.text)),
                HashMap::new(),
            ))
        }
    }

    fn parse_link(&mut self, node: Node) {
        let target = node.attr("href");
        let title = node.attr("title");

        if target.is_none() {
            warn!("'target' missing from link");
            return;
        }

        let target = target.expect("'title' missing after check").to_string();
        let link = match parse_href_to_link(
            self.endpoint.clone(),
            target.clone(),
            title,
            self.language.clone(),
        )
        .with_context(move || format!("failed parsing the link '{}'", target))
        {
            Ok(link) => link,
            Err(error) => {
                warn!("{:?}", error);
                self.elements.push(Element::new(
                    self.next_id(),
                    ElementType::Text,
                    node.text(),
                    self.combine_effects(Style::from(config::CONFIG.theme.text)),
                    HashMap::new(),
                ));
                return;
            }
        };

        self.elements.push(Element::new(
            self.next_id(),
            ElementType::Link(link),
            node.text(),
            self.combine_effects(Style::from(config::CONFIG.theme.text).combine(Effect::Underline)),
            HashMap::new(),
        ));
    }

    fn parse_effect(&mut self, node: Node, effect: Effect) {
        self.current_effects.push(effect);
        self.parse_text(node);
        self.current_effects.pop();
    }

    fn parse_description_list(&mut self, node: Node) {
        for child in node.children() {
            if !self.is_last_newline() {
                self.push_newline();
            }
            match child.name().unwrap_or_default() {
                "dt" => {
                    self.push_kind(ElementType::DescriptionListTermStart);
                    self.parse_text(child);
                    self.push_kind(ElementType::DescriptionListTermEnd);
                }
                "dd" => {
                    self.push_kind(ElementType::DescriptionListDescriptionStart);
                    self.parse_text(child);
                    self.push_kind(ElementType::DescriptionListDescriptionEnd);
                }
                _ => continue,
            }
        }
        self.push_newline();
        self.push_newline();
    }

    fn parse_list(&mut self, node: Node) {
        for child in node
            .children()
            .filter(|x| x.name().unwrap_or_default() == "li")
        {
            // to avoid having large gaps between lists and other elements, we only want to add a
            // newline when there isn't another one already added
            if !self.is_last_newline() {
                self.push_newline();
            }

            self.elements.push(Element::new(
                self.next_id(),
                ElementType::ListMarker,
                format!("\t{} ", LIST_MARKER),
                self.combine_effects(Style::from(config::CONFIG.theme.text)),
                HashMap::new(),
            ));

            self.push_kind(ElementType::ListItemStart);
            self.parse_text(child);
            self.push_kind(ElementType::ListItemEnd);
        }
        self.push_newline();
    }

    fn parse_redirect_msg(&mut self, node: Node) {
        for child in node.children() {
            self.parse_node(child)
        }
    }

    fn parse_disambiguation(&mut self, node: Node) {
        self.push_kind(ElementType::DisambiguationStart);
        self.parse_effect(node, Effect::Italic);
        self.push_kind(ElementType::DisambiguationEnd);

        self.push_newline();
        self.push_newline();
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
