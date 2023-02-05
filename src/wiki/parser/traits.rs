use crate::wiki::article_new::Section;

use anyhow::Result;
use cursive::theme::Effect;
use select::node::Node;

pub trait Parser {
    fn parse_document<'a>(
        self,
        doc: &'a [u8],
        sections: &Vec<Section>,
    ) -> Result<Vec<Box<dyn Element>>>;
    fn get_parser(&self, node_name: &str) -> Box<dyn ElementParser>;

    fn push_element(&mut self, element: Box<dyn Element>);

    fn push_effect(&mut self, effect: Effect);
    fn pop_effect(&mut self);

    fn effects(&self) -> Vec<Effect>;
    fn next_id(&mut self) -> u32;
}

pub trait ElementParser {
    fn parse_node(&self, node: Node, parser: &mut dyn Parser);
}

pub trait Element {
    fn id(&self) -> u32;
    fn content(&self) -> &str;
    fn content_width(&self) -> usize;
    fn effects(&self) -> &Vec<Effect>;
}