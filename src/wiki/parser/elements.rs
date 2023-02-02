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

            fn content_width(&self) -> u32 {
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
    content_width: u32,
    effects: Vec<Effect>,
}

elem_impl!(Text);

pub struct Header {
    id: u32,
    section_id: u32,
    content: String,
    content_width: u32,
    effects: Vec<Effect>,
}

impl Header {
    pub fn section<'a>(&self, article: &'a Article) -> Option<&'a Section> {
        article.section_from_id(self.section_id)
    }
}

elem_impl!(Header);
