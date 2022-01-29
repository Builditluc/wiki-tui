use std::collections::HashMap;

use cursive::theme::Style;

#[derive(PartialEq, Debug)]
pub struct ArticleElement {
    id: i32,
    width: usize,
    style: Style,
    content: String,
    attributes: HashMap<String, String>,
}

impl ArticleElement {
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

    pub fn newline(id: i32) -> Self {
        let mut element = ArticleElement::new(id, 0, Style::none(), String::new());

        element.set_attribute("type", "newline");
        element
    }

    pub fn empty_width(id: i32, width: usize) -> Self {
        ArticleElement::new(id, width, Style::none(), " ".repeat(width))
    }

    pub fn set_attribute<'a>(&mut self, key: &'a str, value: &'a str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }

    pub fn get_attribute<'a>(&'a self, key: &str) -> Option<&'a str> {
        match self.attributes.get(key) {
            Some(value) => Some(value),
            None => None,
        }
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn width(&self) -> &usize {
        &self.width
    }
    pub fn style(&self) -> &Style {
        &self.style
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
