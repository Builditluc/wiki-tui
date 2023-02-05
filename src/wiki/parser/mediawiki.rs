use anyhow::{Context, Result};

use cursive::theme::Effect;
use select::{document::Document, predicate::Class};

use crate::wiki::article_new::Section;

use super::{
    elements::Text,
    traits::{Element, ElementParser, Parser},
};

pub struct MediawikiParser {
    effects: Vec<Effect>,
    elements: Vec<Box<dyn Element>>,
}

impl MediawikiParser {
    pub fn new() -> Self {
        MediawikiParser {
            effects: Vec::new(),
            elements: Vec::new(),
        }
    }
}

impl Parser for MediawikiParser {
    fn parse_document<'a>(
        mut self,
        doc: &'a [u8],
        sections: &Vec<Section>,
    ) -> Result<Vec<Box<dyn Element>>> {
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

    fn next_id(&mut self) -> u32 {
        (self.elements.len() + 1) as u32
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

struct MediawikiUnsupportedElementParser;

impl ElementParser for MediawikiUnsupportedElementParser {
    fn parse_node(&self, node: select::node::Node, parser: &mut dyn Parser) {
        let id = parser.next_id();
        let content = format!("<Unsupported Element '{}'>", node.name().unwrap_or("NONE"));
        let effects = vec![Effect::Italic];

        parser.push_element(Box::new(Text::new(id, content, effects)))
    }
}
