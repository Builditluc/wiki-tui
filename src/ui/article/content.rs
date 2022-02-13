use cursive::Vec2;
use std::rc::Rc;

use crate::ui::article::lines::{Line, LinesWrapper};
use crate::wiki::article::{Article, ArticleElement};

/// The content of an ArticleView. Handles text formatting
pub struct ArticleContent {
    /// The article
    article: Article,
    /// Wrapped lines, ready for drawing
    rendered_lines: Vec<Line>,
}

impl ArticleContent {
    /// Creates a new ArticleContent with the given article
    pub fn new(article: Article) -> Self {
        log::debug!("creating a new instance of ArticleContent");
        ArticleContent {
            article,
            rendered_lines: Vec::new(),
        }
    }

    /// Calculates and returns the required size
    pub fn required_size(&mut self, size: Vec2) -> Vec2 {
        log::debug!(
            "required_size was called with a size of '({},{})'",
            size.x,
            size.y
        );

        // get the required width from a LinesWrapper
        let required_width = LinesWrapper::new(
            size.x,
            // we have to clone all of the elements
            Rc::new(
                self.article
                    .elements()
                    .cloned()
                    .collect::<Vec<ArticleElement>>(),
            ),
        )
        .required_width();

        // if the rendered lines are empty, render them
        if self.rendered_lines.is_empty() {
            self.compute_lines(size);
        }

        // the required width is 0, when any of the lines are wrapped. When this happens we
        // require the full width
        if required_width == 0 {
            log::debug!("lines are wrapped, requiring the full width");
            log::debug!("required_size finished successfully");
            return Vec2::new(size.x, self.rendered_lines.len());
        }

        // if the lines are not wrapped, then return the required width
        log::debug!(
            "required_size finished successfully width a width of '{}'",
            required_width
        );
        Vec2::new(required_width, self.rendered_lines.len())
    }

    /// Renders the article with a given constraint
    pub fn compute_lines(&mut self, size: Vec2) {
        log::debug!(
            "compute_lines was called with a size of '({},{})'",
            size.x,
            size.y
        );

        // render the lines
        self.rendered_lines = LinesWrapper::new(
            size.x,
            // we have to clone all the elements
            Rc::new(
                self.article
                    .elements()
                    .cloned()
                    .collect::<Vec<ArticleElement>>(),
            ),
        )
        .wrap_lines()
        .rendered_lines;

        log::debug!(
            "compute_lines finished successfully, rendering '{}' lines",
            self.rendered_lines.len()
        );
    }

    /// Returns an iterator over the rendered lines
    pub fn get_rendered_lines(&self) -> impl Iterator<Item = &Line> {
        self.rendered_lines.iter()
    }
}
