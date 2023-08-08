use cursive::Vec2;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ui::article::lines::{Line, LinesWrapper};
use crate::wiki::article::{Article, Element, LanguageLink};

/// The content of an ArticleView. Handles text formatting
pub struct ArticleContent {
    /// The article
    article: Article,

    /// Wrapped lines, ready for drawing
    rendered_lines: Vec<Line>,

    /// The coordinates of every anchor found
    anchors: HashMap<String, usize>,

    /// All of the links, with their position and element id
    links: Vec<(usize, Vec2)>,

    /// The index of the currently selected link
    current_link: usize,
}

impl ArticleContent {
    /// Creates a new ArticleContent with the given article
    pub fn new(article: Article) -> Self {
        ArticleContent {
            article,
            rendered_lines: Vec::new(),
            anchors: HashMap::new(),

            links: Vec::new(),
            current_link: 0,
        }
    }

    /// Calculates and returns the required size
    pub fn required_size(&mut self, size: Vec2) -> Vec2 {
        let content = self
            .article
            .content()
            .map(|x| x.cloned())
            .map(|x| x.collect::<Vec<Element>>());

        if content.is_none() {
            return Vec2::zero();
        }

        // get the required width from a LinesWrapper
        let required_width = LinesWrapper::new(
            size.x,
            // we have to clone all of the elements
            Rc::new(content.unwrap()),
        )
        .required_width();

        // if the rendered lines are empty, render them
        if self.rendered_lines.is_empty() {
            self.compute_lines(size);
        }

        // the required width is 0, when any of the lines are wrapped. When this happens we
        // require the full width
        if required_width == 0 {
            return Vec2::new(size.x, self.rendered_lines.len());
        }

        // if the lines are not wrapped, then return the required width
        debug!(
            "required size is ({},{})",
            required_width,
            self.rendered_lines.len()
        );
        Vec2::new(required_width, self.rendered_lines.len())
    }

    /// Renders the article with a given constraint
    pub fn compute_lines(&mut self, size: Vec2) {
        debug!("rendering the article with a size constraint of {:?}", size);

        let content = self
            .article
            .content()
            .map(|x| x.cloned())
            .map(|x| x.collect::<Vec<Element>>());

        if content.is_none() {
            return;
        }

        // render the lines
        let lines_wrapper = LinesWrapper::new(
            size.x.saturating_sub(1),
            // we have to clone all the elements
            Rc::new(content.unwrap()),
        )
        .wrap_lines();

        self.rendered_lines = lines_wrapper.rendered_lines;
        self.anchors = lines_wrapper.anchors;

        // try to keep the link selection after layout change
        let link_id = self.current_link_element_id();
        self.links = lines_wrapper.links;
        self.select_link_by_id(link_id);

        debug!("sucessfully rendered '{}' lines", self.rendered_lines.len());
    }

    /// Returns an iterator over the rendered lines
    pub fn get_rendered_lines(&self) -> impl Iterator<Item = &Line> {
        self.rendered_lines.iter()
    }

    /// Returns the coordinate of an anchor, if it could be found
    pub fn anchor(&self, anchor: &str) -> Option<usize> {
        self.anchors.get(anchor).copied()
    }

    /// Returns the element id of the currently selected link
    pub fn current_link_element_id(&self) -> usize {
        if self.links.len() <= self.current_link {
            return 0;
        }
        self.links[self.current_link].0
    }

    /// Return the coordinates of the currently selected link
    pub fn current_link_coords(&self) -> Vec2 {
        if self.links.len() <= self.current_link {
            return Vec2::zero();
        }
        self.links[self.current_link].1
    }

    /// Selects the link corredsponding to the element id
    pub fn select_link_by_id(&mut self, id: usize) {
        if let Some(new_link) = self.links.iter().position(|x| x.0 == id) {
            self.current_link = new_link;
        }
    }

    /// Returns an iterator over every link
    pub fn links(&self) -> impl Iterator<Item = &(usize, Vec2)> + DoubleEndedIterator {
        self.links.iter()
    }

    /// Returns the index of the current link
    pub fn current_link_idx(&self) -> usize {
        self.current_link
    }

    /// Returns true if we have links
    pub fn has_links(&self) -> bool {
        !self.links.is_empty()
    }

    /// Selects the next link, if selection is possible
    pub fn select_next_link(&mut self) {
        if self.current_link + 1 >= self.links.len() {
            return;
        }
        self.current_link = self
            .links
            .iter()
            .position(|(id, _)| id > &self.current_link_element_id())
            .unwrap_or(self.current_link);
    }

    /// Selects the previous link, if selection is possible
    pub fn select_prev_link(&mut self) {
        if let Some(link) = self
            .links
            .iter()
            .rev()
            .skip(self.links.len().saturating_sub(self.current_link))
            .find(|(id, _)| id < &self.current_link_element_id())
        {
            let position = self.links.iter().position(|x| x == link).unwrap();
            self.current_link = position;
        }
    }

    /// Retrieves the Element with the id
    pub fn element_by_id(&self, id: usize) -> Option<Element> {
        self.article
            .content()
            .and_then(|mut x| x.find(|e| e.id() == id))
            .cloned()
    }

    /// Retrieves the Element at the given position
    pub fn element_by_pos(&self, pos: Vec2) -> Option<Element> {
        if pos.y >= self.rendered_lines.len() || self.rendered_lines.is_empty() {
            return None;
        }

        let line = &self.rendered_lines[pos.y];
        let mut x_offset = 0;
        for element in line {
            if pos.x >= x_offset && pos.x <= x_offset.saturating_add(element.width) {
                return self.element_by_id(element.id);
            }
            x_offset += element.width;
        }
        None
    }

    pub fn language_links(&self) -> Option<Vec<LanguageLink>> {
        self.article.language_links.clone()
    }
}
