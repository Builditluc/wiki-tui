use std::collections::HashMap;

use cursive::theme::Effect;

use crate::wiki::article_new::Section;

use super::{
    elements::Text,
    traits::{Element, ElementParser, Parser},
};

pub struct MediawikiParser {
    element_parser: HashMap<String, Box<dyn ElementParser>>,
    unsupported_parser: Box<dyn ElementParser>,
    effects: Vec<Effect>,
    elements: Vec<Box<dyn Element>>,
}

impl MediawikiParser {
    pub fn new() -> Self {
        let mut element_parser = HashMap::new();
        element_parser.insert(
            "p".to_string(),
            Box::new(MediawikiParagraphParser) as Box<dyn ElementParser>,
        );

        MediawikiParser {
            element_parser,
            unsupported_parser: Box::new(MediawikiUnsupportedElementParser),
            effects: Vec::new(),
            elements: Vec::new(),
        }
    }
}

impl Parser for MediawikiParser {
    fn parse_document<'a>(
        &mut self,
        doc: &'a [u8],
        sections: &Vec<Section>,
    ) -> Vec<Box<dyn Element>> {
        todo!()
    }

    fn get_parser(&self, node_name: &str) -> Box<dyn ElementParser> {
        todo!()
    }
    fn push_element(&mut self, element: Box<dyn Element>) {
        todo!()
    }

    fn push_effect(&mut self, effect: cursive::theme::Effect) {
        todo!()
    }

    fn pop_effect(&mut self) {
        todo!()
    }

    fn effects(&self) -> Vec<cursive::theme::Effect> {
        todo!()
    }

    fn next_id(&mut self) -> u32 {
        todo!()
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
