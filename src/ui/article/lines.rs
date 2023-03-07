use crate::config::CONFIG;
use crate::wiki::article::{Element, ElementType};

use cursive::theme::Style;
use cursive::Vec2;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

/// An element only containing the neccessary information for rendering (and an id so that it can
/// be referenced to an article element
#[derive(Debug)]
pub struct RenderedElement {
    /// The id of the ArticleElement this element belongs to
    pub id: usize,
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
    elements: Rc<Vec<Element>>,
    /// The rendered lines
    pub rendered_lines: Vec<Line>,

    /// The y-coordinates of every anchor encountered
    pub anchors: HashMap<String, usize>,

    /// The ids and positions of the links
    pub links: Vec<(usize, Vec2)>,
}

impl LinesWrapper {
    /// Creates a new LinesWrapper with a content and constraint
    pub fn new(width: usize, elements: Rc<Vec<Element>>) -> Self {
        LinesWrapper {
            current_line: Line::new(),
            current_width: 0,

            width,
            max_width: 0,

            is_wrapped: false,

            elements,
            rendered_lines: Vec::new(),

            links: Vec::new(),
            anchors: HashMap::new(),
        }
    }

    /// Wraps the lines and returns the required width. This method is way cheaper than wrap_lines
    /// because it only calculates the required width and nothing else
    pub fn required_width(mut self) -> usize {
        debug!("calculating the required with");
        // go through every elment
        for element in self.elements.iter() {
            // does this element go onto a new line?
            if element.kind() == ElementType::Newline {
                // "add" the element onto a new line
                self.current_line = Line::new();
                self.current_width = element.width();

                // store the width of the element if it is the biggest one yet
                if element.width() > self.max_width {
                    self.max_width = element.width();
                    continue;
                }
            }

            // does it fit into the current line?
            if element.width() + self.current_width < self.width {
                // yay, it fits
                // add its width to the current line
                self.current_width += element.width();

                // store the width of the element if it is the biggest one yet
                if element.width() > self.max_width {
                    self.max_width = element.width();
                    continue;
                }
            }

            // if it doesn't fit, return 0
            debug!("the lines are wrapped, returning 0");
            return 0;
        }

        debug!("finished with a required size of '{}'", self.max_width);
        self.max_width
    }

    /// Starts the wrapping process
    #[must_use]
    pub fn wrap_lines(mut self) -> Self {
        debug!("wrapping the lines");

        let mut last_type: ElementType = ElementType::Text;

        // go through every element
        for element in self.elements.clone().iter() {
            // is this a link?
            let is_link = element.kind() == ElementType::Link;

            // does this element go onto a new line?
            if element.kind() == ElementType::Newline {
                // fill the current line and make the next one blank
                self.fill_line();
                self.newline();

                last_type = element.kind();
                continue;
            }

            // what we do here is fairly simple:
            // First, we split the content into words and then we merge these words together until the
            // line is full. Then we create a new one and do the same thing over and over again until
            // we run out of words.

            let mut merged_element = RenderedElement {
                id: element.id(),
                style: element.style(),
                content: String::new(),
                width: 0,
            };

            // now our lines are wrapped
            self.is_wrapped = true;

            // if the element does not have a leading special character and we are not at the beginning
            // of a line, add a leading whitespace
            if (!element.content().starts_with([',', '.', ';', ':'])
                && !self.current_line.is_empty())
                || last_type == ElementType::ListMarker
            {
                self.push_whitespace();
            }

            for span in element.content().split_whitespace() {
                // does the span fit onto the current line?
                if span.chars().count() + merged_element.width + self.current_width < self.width {
                    // only add a leading whitespace if the merged element is not empty
                    if !merged_element.content.is_empty() {
                        merged_element.push(' ');
                    }
                    // then add it to the merged element
                    merged_element.push_str(span);
                    last_type = element.kind();
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
                    self.register_link(element.id())
                }

                if let Some(anchor) = element.attr("anchor") {
                    self.anchors
                        .insert(anchor.to_string(), self.rendered_lines.len());
                }

                self.fill_line();
                self.newline();

                merged_element = RenderedElement {
                    id: element.id(),
                    style: element.style(),
                    content: String::new(),
                    width: 0,
                };

                // does the span fit onto the current line?
                if span.chars().count() + merged_element.width + self.current_width < self.width {
                    // only add a leading whitespace if the merged element is not empty
                    if !merged_element.content.is_empty() {
                        merged_element.push(' ');
                    }
                    // then add it to the merged element
                    merged_element.push_str(span);
                    last_type = element.kind();
                    continue;
                }
            }

            // if there are still some spans in the merged_element, add it to the current line and
            // register a link if it is one
            if !merged_element.content.is_empty() {
                self.current_width += merged_element.width;
                self.current_line.push(merged_element);

                if is_link {
                    self.register_link(element.id());
                }

                if let Some(anchor) = element.attr("anchor") {
                    self.anchors
                        .insert(anchor.to_string(), self.rendered_lines.len());
                }
            }
        }

        debug!("'{}' anchors found and registered", self.anchors.len());
        debug!("'{}' links found", self.links.len());
        debug!("wrapped '{}' lines", self.rendered_lines.len());
        self
    }

    /// Registers a new link with the given id
    fn register_link(&mut self, id: usize) {
        self.links
            .push((id, Vec2::new(self.current_width, self.rendered_lines.len())))
    }
    /// Adds an element to the current line and if needed, registers a link to it
    fn push_element(&mut self, element: RenderedElement) {
        self.current_width += element.width;
        self.current_line.push(element);
    }

    /// Adds a whitespacde to the current line
    fn push_whitespace(&mut self) {
        // check if we can add a whitespace
        if self.current_width == self.width {
            return;
        }

        // create a rendered element with the id -1 and push it to the current line
        self.push_element(RenderedElement {
            id: usize::MAX,
            content: " ".to_string(),
            style: Style::from(CONFIG.theme.text),
            width: 1,
        });
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
            &usize::MAX,
            &Style::none(),
            &" ".repeat(remaining_width),
            &remaining_width,
        );
    }

    /// Creates a rendered element and adds it to the current line
    fn create_rendered_element(&mut self, id: &usize, style: &Style, content: &str, width: &usize) {
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
}
