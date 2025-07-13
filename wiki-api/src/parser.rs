use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::str::FromStr;
use tracing::{trace, warn};
use url::Url;

use crate::{
    document::{Data, HeaderKind, Raw, UnsupportedElement},
    languages::Language,
    page::{
        link_data::{AnchorData, ExternalData, ExternalToInternalData, InternalData, MediaData},
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
    nodes: Vec<Raw>,
    endpoint: Endpoint,
    language: Language,
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

                let mut ignore_children = false;

                let data = match name.as_str() {
                    "head" | "style" | "link" => return prev,

                    "table" => {
                        ignore_children = true;
                        Data::Unsupported(UnsupportedElement::Table)
                    }
                    "image" => {
                        ignore_children = true;
                        Data::Unsupported(UnsupportedElement::Image)
                    }
                    "figure" => {
                        ignore_children = true;
                        Data::Unsupported(UnsupportedElement::Figure)
                    }
                    "pre" => {
                        ignore_children = true;
                        Data::Unsupported(UnsupportedElement::PreformattedText)
                    }

                    "span"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class"
                                && (value.contains("texhtml") || value.contains("mwe-math-element"))
                        }) =>
                    {
                        ignore_children = true;
                        Data::UnsupportedInline(UnsupportedElement::MathElement)
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

                    "blockquote" => Data::Blockquote,

                    "ol" => Data::OrderedList,
                    "ul" => Data::UnorderedList,
                    "li" => Data::ListItem,

                    "dl" => Data::DescriptionList,
                    "dt" => Data::DescriptionListTerm,
                    "dd" => Data::DerscriptionListDescription,

                    "br" => Data::Linebreak,

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

                    "a" => {
                        Self::parse_link(&self.endpoint, self.language, &attrs).unwrap_or_default()
                    }

                    "div" => Data::Division,
                    _ => {
                        warn!("unknown node '{name}'");
                        Data::Unknown
                    }
                };

                let index = self.push_node(data, parent, prev);

                if ignore_children {
                    return Some(index);
                }

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

    fn parse_link(endpoint: &Url, language: Language, attrs: &[(String, String)]) -> Option<Data> {
        let href = attrs
            .iter()
            .find(|(name, _)| name.as_str() == "href")
            .map(|(_, value)| value.to_owned())?;

        let title = attrs
            .iter()
            .find(|(name, _)| name.as_str() == "title")
            .map(|(_, value)| value.to_owned())
            .unwrap_or_default();

        let link_url = endpoint.join(&href).ok()?;
        let link_type: &str = match attrs
            .iter()
            .find(|(name, _)| name.as_str() == "rel")
            .map(|(_, value)| value.to_owned())?
            .as_str()
        {
            "mw:WikiLink" => "wiki",
            "mw:MediaLink" => "media",
            "mw:ExtLink" => "external",
            _ => "",
        };

        let anchor = link_url.fragment().map(|fragment| AnchorData {
            title: title.to_string(),
            anchor: fragment.to_string(),
        });

        if link_type == "wiki" {
            let namespace = Namespace::Main;

            let is_same_wiki = link_url.domain() == endpoint.domain();
            if !is_same_wiki {
                return Some(Data::Link(Link::ExternalToInternal(
                    ExternalToInternalData {},
                )));
            }

            let page = link_url.path_segments()?.last()?;

            const NAMESPACE_DELIMITER: char = ':';
            let (namespace, page) =
                if let Some((ns_str, page_str)) = page.split_once(NAMESPACE_DELIMITER) {
                    (
                        Namespace::from_string(ns_str).unwrap_or_else(|| {
                            warn!("invalid namespace '{}', using default", ns_str);
                            namespace
                        }),
                        page_str,
                    )
                } else {
                    (namespace, page)
                };

            // we get the language from the host
            // for wikipedia, the host looks like this
            //      [lang].wikipedia.org/
            // where [lang] is the language code, for example
            //      en.wikipedia.org/
            // for the english wikipedia

            let lang_str = link_url
                .host_str()
                .and_then(|x| x.split_once('.').map(|x| x.0));

            let language = match lang_str {
                Some(str) => Language::from_str(str).unwrap_or(language),
                None => language,
            };

            let link_data = InternalData {
                namespace,
                page: page.to_string(),
                title,
                endpoint: endpoint.clone(),
                language,
                anchor,
            };

            return Some(Data::Link(Link::Internal(link_data)));
        }

        if link_type == "media" {
            return Some(Data::Link(Link::MediaLink(MediaData {
                url: link_url,
                title,
            })));
        }

        if link_type == "external" {
            return Some(Data::Link(Link::External(ExternalData { url: link_url })));
        }

        None
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
