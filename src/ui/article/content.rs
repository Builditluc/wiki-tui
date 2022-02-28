use cursive::{direction::Absolute, Vec2};
use std::rc::Rc;

use crate::wiki::article::{Article, ArticleElement};
use crate::{
    config::CONFIG,
    ui::article::{
        lines::{Line, LinesWrapper},
        links::LinkHandler,
    },
};

/// The content of an ArticleView. Handles text formatting
pub struct ArticleContent {
    /// The article
    article: Article,
    /// Wrapped lines, ready for drawing
    rendered_lines: Vec<Line>,

    /// The LinkHandler, only created and used when it's enabled in the configuration
    link_handler: Option<LinkHandler>,
}

impl ArticleContent {
    /// Creates a new ArticleContent with the given article
    pub fn new(article: Article) -> Self {
        log::debug!("creating a new instance of ArticleContent");
        ArticleContent {
            article,
            rendered_lines: Vec::new(),
            link_handler: None,
        }
    }

    /// Returns the ArticleElement from a given id
    /// Accepts an optional id so it can be easily linked with current_link
    pub fn element_by_id(&self, id: Option<i32>) -> Option<&ArticleElement> {
        if let Some(id) = id {
            // get every element with that id and return the first one
            return self.article.elements().filter(|e| e.id() == &id).next();
        }
        None
    }

    /// Returns the id of the current link
    pub fn current_link(&self) -> Option<i32> {
        if let Some(ref link_handler) = self.link_handler {
            return Some(link_handler.get_current_link());
        }
        None
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
        let lines_wrapper = LinesWrapper::new(
            size.x,
            // we have to clone all the elements
            Rc::new(
                self.article
                    .elements()
                    .cloned()
                    .collect::<Vec<ArticleElement>>(),
            ),
        )
        .wrap_lines();

        self.link_handler = lines_wrapper.link_handler;
        self.rendered_lines = lines_wrapper.rendered_lines;

        log::debug!(
            "compute_lines finished successfully, rendering '{}' lines",
            self.rendered_lines.len()
        );
    }

    /// Returns an iterator over the rendered lines
    pub fn get_rendered_lines(&self) -> impl Iterator<Item = &Line> {
        self.rendered_lines.iter()
    }

    /// Moves the selected link by in a direction by a given amount
    pub fn move_selected_link(&mut self, direction: Absolute, amount: usize) {
        if !CONFIG.features.links {
            return;
        }

        if let Some(ref mut link_handler) = self.link_handler {
            match direction {
                Absolute::Left => link_handler.move_left(amount),
                Absolute::Up => link_handler.move_up(amount),
                Absolute::Right => link_handler.move_right(amount),
                Absolute::Down => link_handler.move_down(amount),
                Absolute::None => return,
            }
        }
    }
}
