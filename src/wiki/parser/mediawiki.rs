use cursive::theme::Effect;

use crate::wiki::article_new::Section;

use super::{
    elements::Text,
    traits::{Element, ElementParser, Parser},
};

pub struct MediawikiParser;

impl Parser for MediawikiParser {
    fn parse_document<R: std::io::Read>(doc: R, sections: &Vec<Section>) -> Vec<Box<dyn Element>>
    where
        Self: Sized,
    {
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
