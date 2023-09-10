#[cfg(debug_assertions)]
pub mod test_renderer;

use ratatui::style::Style;
use textwrap::core::Fragment;
use wiki_api::document::Node;

#[derive(Debug)]
pub struct Word {
    pub index: usize,
    pub content: String,
    pub style: Style,
    pub width: f64,
    pub whitespace_width: f64,
    pub penalty_width: f64,
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

pub type Line = Vec<Word>;

#[derive(Debug)]
pub struct RenderedDocument {
    pub lines: Vec<Line>,
}
