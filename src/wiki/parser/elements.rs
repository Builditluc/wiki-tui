use cursive::theme::Effect;

use crate::wiki::article_new::{Article, Section};

use super::traits::Element;

macro_rules! elem_impl {
    ($type: ty) => {
        impl Element for $type {
            fn id(&self) -> u32 {
                self.id
            }

            fn content(&self) -> &str {
                &self.content
            }

            fn content_width(&self) -> usize {
                self.content_width
            }

            fn effects(&self) -> &Vec<Effect> {
                &self.effects
            }
        }
    };
}

pub struct Text {
    id: u32,
    content: String,
    content_width: usize,
    effects: Vec<Effect>,
}

impl Text {
    pub fn new(id: u32, content: String, effects: Vec<Effect>) -> Self {
        Text {
            id,
            content_width: content.chars().count(),
            content,
            effects,
        }
    }
}

elem_impl!(Text);

pub struct Header {
    id: u32,
    section_id: u32,
    content: String,
    content_width: usize,
    effects: Vec<Effect>,
}

impl Header {
    pub fn new(id: u32, section_id: u32, content: String, effects: Vec<Effect>) -> Self {
        Header {
            id,
            section_id,
            content_width: content.chars().count(),
            content,
            effects,
        }
    }

    pub fn section<'a>(&self, article: &'a Article) -> Option<&'a Section> {
        article.section_from_id(self.section_id)
    }
}

elem_impl!(Header);

pub struct Link {
    id: u32,
    content: String,
    content_width: usize,
    effects: Vec<Effect>,
    title: String,
    target: String,
    link_type: LinkType,
}

pub enum LinkType {
    Wiki,
    External,
}

impl Link {
    pub fn new(
        id: u32,
        content: String,
        effects: Vec<Effect>,
        title: String,
        target: String,
        link_type: LinkType,
    ) -> Self {
        Link {
            id,
            content_width: content.chars().count(),
            content,
            effects,
            title,
            target,
            link_type,
        }
    }
}

elem_impl!(Link);
