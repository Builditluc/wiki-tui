use anyhow::{Context, Result};

use cursive::theme::Effect;
use select::{document::Document, predicate::Class};

use crate::wiki::article_new::Section;

use super::{
    elements::{Header, Link, LinkType, Text},
    traits::{Element, ElementParser, Parser},
};

pub struct MediawikiParser {
    effects: Vec<Effect>,
    elements: Vec<Box<dyn Element>>,
    sections: Vec<Section>,
}

impl MediawikiParser {
    pub fn new() -> Self {
        MediawikiParser {
            effects: Vec::new(),
            elements: Vec::new(),
            sections: Vec::new(),
        }
    }
}

impl Parser for MediawikiParser {
    fn parse_document<'a>(
        mut self,
        doc: &'a [u8],
        sections: &Vec<Section>,
    ) -> Result<Vec<Box<dyn Element>>> {
        self.sections = sections.to_vec();

        Document::from_read(doc)?
            .find(Class("mw-parser-output"))
            .into_selection()
            .first()
            .context("Couldn't find the node 'mw-parser-output'")?
            .children()
            .map(|child| {
                self.get_parser(child.name().unwrap_or("NONE"))
                    .parse_node(child, &mut self);
            })
            .count();

        Ok(self.elements)
    }

    fn get_parser(&self, node_name: &str) -> Box<dyn ElementParser> {
        match node_name {
            "p" => Box::new(MediawikiParagraphParser),
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => Box::new(MediawikiHeaderParser),
            "a" => Box::new(MediawikiLinkParser),
            _ => Box::new(MediawikiUnsupportedElementParser),
        }
    }

    fn push_element(&mut self, element: Box<dyn Element>) {
        self.elements.push(element)
    }

    fn push_effect(&mut self, effect: cursive::theme::Effect) {
        self.effects.push(effect)
    }

    fn pop_effect(&mut self) {
        self.effects.pop();
    }

    fn effects(&self) -> Vec<cursive::theme::Effect> {
        self.effects.clone()
    }

    fn next_id(&self) -> u32 {
        (self.elements.len() + 1) as u32
    }

    fn section_from_anchor(&self, anchor: &str) -> Option<&Section> {
        self.sections.iter().find(|x| x.anchor == anchor)
    }
}

struct MediawikiParagraphParser;

impl ElementParser for MediawikiParagraphParser {
    fn parse_node(&self, node: select::node::Node, parser: &mut dyn Parser) {
        for child in node.children() {
            if let Some(name) = child.name() {
                let element_parser = parser.get_parser(name);
                element_parser.parse_node(child, parser);
                continue;
            }

            let id = parser.next_id();
            let content = child.text();
            let effects = parser.effects();

            parser.push_element(Box::new(Text::new(id, content, effects)))
        }
    }
}

struct MediawikiHeaderParser;

impl ElementParser for MediawikiHeaderParser {
    fn parse_node(&self, node: select::node::Node, parser: &mut dyn Parser) {
        if let Some(headline_node) = node.find(Class("mw-headline")).into_selection().first() {
            if let Some(section) =
                parser.section_from_anchor(headline_node.attr("id").unwrap_or_default())
            {
                parser.push_element(Box::new(Header::new(
                    parser.next_id(),
                    section.id,
                    headline_node.text(),
                    parser.effects(),
                )))
            }
        }
    }
}

struct MediawikiLinkParser;

impl ElementParser for MediawikiLinkParser {
    fn parse_node(&self, node: select::node::Node, parser: &mut dyn Parser) {
        let target = node.attr("href");
        let title = node.attr("title");

        if target.is_some() && title.is_some() {
            let mut link_type = LinkType::Wiki;
            if target.unwrap().starts_with("https://") || target.unwrap().starts_with("http://") {
                link_type = LinkType::External;
            }

            let id = parser.next_id();
            let content = node.text();
            let effects = parser.effects();

            parser.push_element(Box::new(Link::new(
                id,
                content,
                effects,
                target.unwrap().to_string(),
                title.unwrap().to_string(),
                link_type,
            )))
        }
    }
}

struct MediawikiUnsupportedElementParser;

impl ElementParser for MediawikiUnsupportedElementParser {
    fn parse_node(&self, node: select::node::Node, parser: &mut dyn Parser) {
        let id = parser.next_id();
        let content = format!("<Unsupported Element '{}'>", node.name().unwrap_or("NONE"));
        let effects = vec![Effect::Italic];

        parser.push_element(Box::new(Text::new(id, content, effects)))
    }
}
