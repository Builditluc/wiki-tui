pub mod test_renderer;

use ratatui::style::{Style, Styled};
use textwrap::core::Fragment;
use wiki_api::document::{Document, Node};

#[derive(Debug)]
pub struct Word<'a> {
    pub node: Node<'a>,
    pub content: String,
    pub style: Style,
    pub width: f64,
    pub whitespace_width: f64,
    pub penalty_width: f64,
}

impl<'a> Fragment for Word<'a> {
    #[inline]
    fn width(&self) -> f64 {
        self.width
    }

    #[inline]
    fn whitespace_width(&self) -> f64 {
        self.whitespace_width
    }

    #[inline]
    fn penalty_width(&self) -> f64 {
        self.penalty_width
    }
}

pub type Line<'a> = Vec<Word<'a>>;

#[derive(Debug)]
pub struct RenderedDocument<'a> {
    pub lines: Vec<Line<'a>>,
}

pub trait Renderer {
    fn render<'a>(&self, document: &'a Document, width: u16) -> RenderedDocument<'a>;
}
