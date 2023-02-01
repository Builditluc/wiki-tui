use crate::wiki::{article_new::Article, section::Section};
use std::io::Read;

use cursive::theme::Effect;
use select::node::Node;

pub trait Parser {
    fn parse_document<R: Read>(doc: R, sections: Vec<Section>) -> Article
    where
        Self: Sized;

    fn push_element(&mut self, element: dyn Element);

    fn push_effect(&mut self, effect: Effect);
    fn pop_effect(&mut self);

    fn effects(&self) -> Vec<Effect>;
    fn next_id(&mut self) -> u32;
}

pub trait ElementParser {
    fn parse_node(node: Node, parser: &mut dyn Parser);
}

pub trait Element {
    fn id(&self) -> u32;
    fn content(&self) -> &str;
    fn content_width(&self) -> u32;
    fn effects(&self) -> Vec<Effect>;
}
