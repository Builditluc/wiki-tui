use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::str::FromStr;
use tracing::{debug, warn};

use crate::document::{Data, HeaderKind, Raw};

// TODO: remove Parser and replace it with normal functions and helper functions
pub trait Parser {
    fn parse_document(docuemnt: &str) -> Self;
    fn nodes(self) -> Vec<Raw>;
}

pub struct WikipediaParser {
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
                        debug!("ignoring portalbox");
                        return prev;
                    }

                    "div"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class"
                                && (value.contains("toc")
                                    || value.contains("quotebox")
                                    || value.contains("noprint"))
                        }) =>
                    {
                        debug!("ignoring toc or quotebox");
                        return prev;
                    }

                    "div"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "class" && value.contains("mw-empty-elt")
                        }) =>
                    {
                        return prev
                    }

                    "span"
                        if attrs.iter().any(|(name, value)| {
                            name.as_str() == "typeof" && value.contains("mw:Transclusion")
                        }) =>
                    {
                        return prev
                    }

                    _ if attrs.iter().any(|(name, value)| {
                        name.as_str() == "class" && value.contains("noprint")
                    }) =>
                    {
                        return prev
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

    fn parse_wiki_link<'a>(
        &mut self,
        mut attrs: impl Iterator<Item = &'a (String, String)>,
    ) -> Option<Data> {
        let href = attrs
            .find(|(name, _)| name.as_str() == "href")
            .map(|(_, value)| value.to_owned())?;

        let title = attrs
            .find(|(name, _)| name.as_str() == "title")
            .map(|(_, value)| value.to_owned());

        if attrs.any(|(name, value)| name.as_str() == "class" && value.contains("new")) {
            return Some(Data::RedLink { title });
        }

        Some(Data::WikiLink { href, title })
    }

    fn parse_media_link<'a>(
        &mut self,
        mut attrs: impl Iterator<Item = &'a (String, String)>,
    ) -> Option<Data> {
        let href = attrs
            .find(|(name, _)| name.as_str() == "href")
            .map(|(_, value)| value.to_owned())?;

        let title = attrs
            .find(|(name, _)| name.as_str() == "title")
            .map(|(_, value)| value.to_owned());

        if attrs.any(|(name, value)| name.as_str() == "class" && value.contains("new")) {
            return Some(Data::RedLink { title });
        }

        Some(Data::MediaLink { href, title })
    }

    fn parse_external_link<'a>(
        &self,
        mut attrs: impl Iterator<Item = &'a (String, String)>,
    ) -> Option<Data> {
        let href = attrs
            .find(|(name, _)| name.as_str() == "href")
            .map(|(_, value)| value.to_owned())?;

        let title = attrs
            .find(|(name, _)| name.as_str() == "title")
            .map(|(_, value)| value.to_owned());

        let autonumber =
            attrs.any(|(name, value)| name.as_str() == "class" && value.contains("autonumber"));

        Some(Data::ExternalLink {
            href,
            title,
            autonumber,
        })
    }
}

impl Parser for WikipediaParser {
    fn parse_document(document: &str) -> Self {
        let mut parser = WikipediaParser { nodes: Vec::new() };

        let rc_dom = parse_document(RcDom::default(), Default::default()).one(document);
        parser.parse_node(&rc_dom.document, None, None);

        parser
    }

    fn nodes(self) -> Vec<Raw> {
        self.nodes
    }
}
