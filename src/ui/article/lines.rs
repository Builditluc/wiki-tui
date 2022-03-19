use crate::config::CONFIG;
use crate::ui::article::links::LinkHandler;
use crate::wiki::article::ArticleElement;

use cursive::theme::Style;
use std::mem;
use std::rc::Rc;

/// An element only containing the neccessary information for rendering (and an id so that it can
/// be referenced to an article element
pub struct RenderedElement {
    /// The id of the ArticleElement this element belongs to
    pub id: i32,
    /// The content of the element
    pub content: String,
    /// The style of the element
    pub style: Style,
    /// The width of the element. Measured by the amount of characters in the content
    pub width: usize,
}

impl RenderedElement {
    /// Appends a string to the content of the element
    pub fn push_str(&mut self, string: &str) {
        self.width += string.chars().count();
        self.content.push_str(string);
    }

    /// Appends a character to the content of the element
    pub fn push(&mut self, char: char) {
        self.width += 1;
        self.content.push(char);
    }
}

pub type Line = Vec<RenderedElement>;

/// Generates lines of elements in constrained width
pub struct LinesWrapper {
    /// The line that is currently being rendered
    current_line: Line,
    /// The width of the current line
    current_width: usize,

    /// The maximal width a line can have
    width: usize,
    /// The length of the longest rendered line
    pub max_width: usize,

    /// Are any lines wrapped?
    pub is_wrapped: bool,

    /// A referece to the article elements
    elements: Rc<Vec<ArticleElement>>,
    /// The rendered lines
    pub rendered_lines: Vec<Line>,

    /// The link handler, it is only created and used when enabled in the config
    pub link_handler: Option<LinkHandler>,

    /// The y coordinates of the headers, it is only created and used when enabled in the config
    pub header_y_coords: Option<Vec<usize>>,
}

impl LinesWrapper {
    /// Creates a new LinesWrapper with a content and constraint
    pub fn new(width: usize, elements: Rc<Vec<ArticleElement>>) -> Self {
        log::debug!(
            "creating a new LinesWrapper with '{}' elements and a width of '{}'",
            elements.len(),
            width
        );
        LinesWrapper {
            current_line: Line::new(),
            current_width: 0,

            width,
            max_width: 0,

            is_wrapped: false,

            elements,
            rendered_lines: Vec::new(),

            link_handler: {
                if CONFIG.features.links {
                    Some(LinkHandler::new())
                } else {
                    None
                }
            },

            header_y_coords: {
                if CONFIG.features.headers {
                    Some(Vec::new())
                } else {
                    None
                }
            },
        }
    }

    /// Wraps the lines and returns the required width. This method is way cheaper than wrap_lines
    /// because it only calculates the required width and nothing else
    pub fn required_width(mut self) -> usize {
        log::debug!("required_width was called");
        // go through every elment
        for element in self.elements.iter() {
            // does this element go onto a new line?
            if element.get_attribute("type").unwrap_or("text") == "newline" {
                // "add" the element onto a new line
                self.current_line = Line::new();
                self.current_width = *element.width();

                // store the width of the element if it is the biggest one yet
                if element.width() > &self.max_width {
                    self.max_width = *element.width();
                    continue;
                }
            }

            // does it fit into the current line?
            if element.width() + self.current_width < self.width {
                // yay, it fits
                // add its width to the current line
                self.current_width += element.width();

                // store the width of the element if it is the biggest one yet
                if element.width() > &self.max_width {
                    self.max_width = *element.width();
                    continue;
                }
            }

            // if it doesn't fit, return 0
            log::debug!("required_width finished successfully with a width of '0'");
            return 0;
        }

        log::debug!(
            "required_width finished successfully with a width of '{}'",
            self.max_width
        );
        self.max_width
    }

    /// Starts the wrapping process
    #[must_use]
    pub fn wrap_lines(mut self) -> Self {
        log::debug!("wrap_lines was called");

        // go through every element
        for element in self.elements.clone().iter() {
            // get the type of the element
            let element_type = element.get_attribute("type").unwrap_or("text");

            // is this element a link?
            let is_link = element_type == "link" && element.get_attribute("target").is_some();

            // is this element a header?
            let is_header = element_type == "header";

            // does this element go onto a new line?
            if element_type == "newline" {
                // fill the current line and make the next one blank
                self.fill_line();
                self.newline();

                self.fill_line();
                self.newline();

                continue;
            }

            // does it fit into the current line?
            if element.width() + self.current_width < self.width {
                // yay, it fits!
                // convert and add it to the current line
                self.create_rendered_element(
                    element.id(),
                    element.style(),
                    element.content(),
                    element.width(),
                );

                // if it's a link, register it
                if is_link {
                    self.register_link(*element.id());
                }

                // if it's a header, save its coordinates
                if is_header && CONFIG.features.headers {
                    if let Some(ref mut header_y_coords) = self.header_y_coords {
                        header_y_coords.push(self.rendered_lines.len());
                    }
                }

                continue;
            }

            // oh no, it doesn't fit!
            // well, then split it
            self.split_element(element);
        }

        log::debug!(
            "wrap_lines finished successfully, wrapping '{}' lines",
            self.rendered_lines.len()
        );

        if let Some(ref link_handler) = self.link_handler {
            log::debug!("total links found: '{}'", link_handler.registered_links());
        }

        self
    }

    /// Registers a new link with the given id
    fn register_link(&mut self, id: i32) {
        if let Some(ref mut link_handler) = self.link_handler {
            link_handler.push_link(
                id,
                self.current_line.len().saturating_sub(1),
                self.rendered_lines.len().saturating_sub(1),
            );
        }
    }
    /// Adds an element to the current line and if needed, registers a link to it
    fn push_element(&mut self, element: RenderedElement) {
        self.current_width += element.width;
        self.current_line.push(element);
    }

    /// Adds the current line to the rendered lines and replaces it with a new, empty one
    fn newline(&mut self) {
        // add the current line to the rendered lines
        self.rendered_lines.push(mem::take(&mut self.current_line));

        // and reset the current line afterwards
        self.current_width = 0;
    }

    /// Fills the remaining space of the line with spaces
    fn fill_line(&mut self) {
        // if our current line is wider than allowed, we really messed up
        assert!(self.current_width <= self.width);

        // change the max width, if neccessary
        if self.current_width > self.max_width {
            self.max_width = self.current_width;
        }

        // just create an empty element that filles the whole line
        let remaining_width = self.width - self.current_width;
        self.create_rendered_element(
            &-1,
            &Style::none(),
            &" ".repeat(remaining_width),
            &remaining_width,
        );
    }

    /// Creates a rendered element and adds it to the current line
    fn create_rendered_element(&mut self, id: &i32, style: &Style, content: &str, width: &usize) {
        // we can just clone the whole thing and call the push_element function
        self.push_element(RenderedElement {
            id: *id,
            style: *style,
            content: {
                // if the line is empty, remove leading whitespace
                if self.current_line.is_empty() {
                    content.trim_start().to_string()
                } else {
                    content.to_string()
                }
            },
            width: *width,
        });
    }

    /// Splits the element into multiple (if required) lines. Pollutes the line with as few
    /// rendered elements as possible
    fn split_element(&mut self, element: &ArticleElement) {
        // what we do here is fairly simple:
        // First, we split the content into words and then we merge these words together until the
        // line is full. Then we create a new one and do the same thing over and over again until
        // we run out of words.

        // is this a link?
        let is_link = element.get_attribute("type") == Some("link")
            && element.get_attribute("target").is_some();

        // is this a header?
        let is_header = element.get_attribute("header") == Some("header");

        let mut merged_element = RenderedElement {
            id: *element.id(),
            style: *element.style(),
            content: String::new(),
            width: 0,
        };

        // now our lines are wrapped
        self.is_wrapped = true;

        for span in element.content().split_whitespace() {
            // does the span fit onto the current line?
            if span.chars().count() + merged_element.width + self.current_width < self.width {
                // then add it to the merged element
                merged_element.push_str(span);
                merged_element.push(' ');
                continue;
            }

            // now we have to do the following things:
            // - add the merged element to the current line
            // - fill the current line and replace it with a new one
            // - add the span to a new merged element
            self.current_width += merged_element.width;
            self.current_line.push(merged_element);

            // if its a link, add it
            if is_link {
                self.register_link(*element.id())
            }

            // if its a header, save its y position
            if is_header && CONFIG.features.headers {
                if let Some(ref mut header_y_coords) = self.header_y_coords {
                    header_y_coords.push(self.rendered_lines.len());
                }
            }

            self.fill_line();
            self.newline();

            merged_element = RenderedElement {
                id: *element.id(),
                style: *element.style(),
                content: String::new(),
                width: 0,
            };
        }

        // if there are still some spans in the merged_element, add it to the current line and
        // register a link if it is one
        if !merged_element.content.is_empty() {
            self.current_width += merged_element.width;
            self.current_line.push(merged_element);

            if is_link {
                self.register_link(*element.id());
            }

            if is_header && CONFIG.features.headers {
                if let Some(ref mut header_y_coords) = self.header_y_coords {
                    header_y_coords.push(self.rendered_lines.len());
                }
            }
        }
    }
}
