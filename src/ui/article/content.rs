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

    /// The y-coordinates of the headers, only created and used when it's enabled in the
    /// configuration
    header_y_coords: Option<Vec<usize>>,

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
            header_y_coords: None,
            link_handler: None,
        }
    }

    /// Returns the ArticleElement from a given id
    /// Accepts an optional id so it can be easily linked with current_link
    pub fn element_by_id(&self, id: Option<i32>) -> Option<&ArticleElement> {
        if let Some(id) = id {
            // get every element with that id and return the first one
            return self.article.elements().find(|e| e.id() == &id);
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

    /// Overrides the current link
    pub fn set_current_link(&mut self, id: i32) {
        if let Some(ref mut link_handler) = self.link_handler {
            link_handler.set_current_link(id);
        }
    }

    /// Returns the position of the current link
    pub fn current_link_pos(&self) -> Option<Vec2> {
        if let Some(ref link_handler) = self.link_handler {
            return Some(link_handler.get_current_link_pos());
        }
        None
    }

    /// Returns the y-position of a given header
    pub fn header_y_pos(&self, index: usize) -> Option<usize> {
        if let Some(ref header_y_coords) = self.header_y_coords {
            assert!(header_y_coords.len() > index);
            return Some(header_y_coords[index]);
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
        self.header_y_coords = lines_wrapper.header_y_coords;

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
                Absolute::None => {}
            }
        }
    }

    /// Retrieves the element at the given position. If no element could be found at that position,
    /// none is returned
    pub fn get_element_at_position(&self, position: Vec2) -> Option<&ArticleElement> {
        // check if y-pos is outside of the bounds
        if position.y >= self.rendered_lines.len() || self.rendered_lines.is_empty() {
            return None;
        }

        // get the line at the given y coordinate
        let line = &self.rendered_lines[position.y];
        let mut x_offset = 0;

        // iterate through every element in that line
        for element in line {
            // if the x position is inside of the bounds of the element, get its ArticleElement and
            // return it
            if position.x >= x_offset && position.x <= x_offset.saturating_add(element.width) {
                return self.element_by_id(Some(element.id));
            }

            // increment the offset
            x_offset += element.width;
        }

        // no element could be found at that position
        None
    }
}
