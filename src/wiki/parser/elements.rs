use cursive::theme::Effect;

use super::traits::Element;

pub struct Text {
    id: u32,
    content: String,
    content_width: u32,
    effects: Vec<Effect>,
}

impl Element for Text {
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
