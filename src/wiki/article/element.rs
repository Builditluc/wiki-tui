use cursive::theme::Style;
use std::collections::HashMap;

/// A struct representing an element in an article. The type of the element is defined by the
/// 'type' attribute
#[derive(PartialEq, Debug, Clone)]
pub struct ArticleElement {
    /// The id of the element
    id: i32,
    /// The width of the element. Measured by the amount of characters
    width: usize,
    /// The style of the element
    style: Style,
    /// The content of the element
    content: String,
    /// The attributes of the element
    attributes: HashMap<String, String>,
}

impl ArticleElement {
    /// Creates a new element. This should not be used directly
    pub fn new(id: i32, width: usize, style: Style, content: String) -> Self {
        let mut element = ArticleElement {
            id,
            width,
            style,
            content,
            attributes: HashMap::new(),
        };

        element.set_attribute("type", "text");
        element
    }

    /// Quickly creates a new element representing a newline
    pub fn newline(id: i32) -> Self {
        let mut element = ArticleElement::new(id, 0, Style::none(), String::new());

        element.set_attribute("type", "newline");
        element
    }

    /// Creates a new empty element filled with blank spaces
    pub fn empty_width(id: i32, width: usize) -> Self {
        ArticleElement::new(id, width, Style::none(), " ".repeat(width))
    }

    /// Add a new attribute to the element
    pub fn set_attribute<'a>(&mut self, key: &'a str, value: &'a str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    /// Add a new attribute to the element. Chainable variant
    #[must_use]
    pub fn attribute<'a>(mut self, key: &'a str, value: &'a str) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }

    /// Retrieve an attribute from the element. Returns None is the attribute could not be found
    pub fn get_attribute<'a>(&'a self, key: &str) -> Option<&'a str> {
        match self.attributes.get(key) {
            Some(value) => Some(value),
            None => None,
        }
    }

    /// The id of the element
    pub fn id(&self) -> &i32 {
        &self.id
    }

    /// The width of the element. Measured by the amount of characters
    pub fn width(&self) -> &usize {
        &self.width
    }

    /// The style of the element
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// The content of the element
    pub fn content(&self) -> &str {
        &self.content
    }
}
