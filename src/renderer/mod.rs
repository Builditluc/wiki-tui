pub mod default_renderer;
#[cfg(debug_assertions)]
pub mod test_renderer;

use ratatui::style::Style;
use textwrap::core::Fragment;
use wiki_api::document::{Document, Node};

#[derive(Debug, Clone)]
pub struct Word {
    pub index: usize,
    pub content: String,
    pub style: Style,
    // TODO: Change width type to u64
    pub width: f64,
    // TODO: Change whitespace_width type to u8
    pub whitespace_width: f64,
    // TODO: Change penalty_width type to u8
    pub penalty_width: f64,
}

impl<'a> Word {
    pub fn node(&self, document: &'a Document) -> Option<Node<'a>> {
        document.nth(self.index)
    }
}

impl Fragment for Word {
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

#[derive(Debug, Clone)]
pub struct RenderedDocument {
    pub lines: Vec<Vec<Word>>,
    /// Vec<(y-Coord, idx)>
    pub links: Vec<(usize, usize)>,
}
