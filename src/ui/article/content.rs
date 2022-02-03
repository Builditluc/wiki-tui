use std::rc::Rc;

use cursive::Vec2;

use crate::ui::article::lines::{Line, LinesWrapper};
use crate::wiki::article::{Article, ArticleElement};

/// The content of an ArticleView. Handles text formatting and rendering
pub struct ArticleContent {
    /// The article
    article: Article,
    /// Wrapped lines, ready for drawing
    rendered_lines: Vec<Line>,
}

impl ArticleContent {
    /// Creates a new ArticleContent with the given article
    pub fn new(article: Article) -> Self {
        ArticleContent {
            article,
            rendered_lines: Vec::new(),
        }
    }

    /// Calculates and returns the required size for the content
    pub fn required_size(&self, size: Vec2) -> Vec2 {
        let required_width = LinesWrapper::new(
            size.x,
            Rc::new(
                self.article
                    .elements()
                    .cloned()
                    .collect::<Vec<ArticleElement>>(),
            ),
        ).required_width();

        if required_width == 0 {
            return Vec2::new(size.x, 1);
        }

        return Vec2::new(required_width, 1);
    }

    /// Renders the article with a given constraint
    pub fn compute_lines(&mut self, size: Vec2) {
        self.rendered_lines = LinesWrapper::new(
            size.x,
            Rc::new(
                self.article
                .elements()
                .cloned()
                .collect::<Vec<ArticleElement>>(),
            ),
        ).rendered_lines;
    }

    /// Returns an iterator over the rendered lines
    pub fn get_rendered_lines(&mut self) -> impl Iterator<Item = &Line> {
        self.rendered_lines.iter()
    }
}
