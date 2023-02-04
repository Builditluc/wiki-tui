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
    fn parse_node(&self, node: select::node::Node, parser: &mut dyn Parser)
    where
        Self: Sized,
    {
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
