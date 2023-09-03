use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::Rect, widgets::Paragraph};

use crate::{action::Action, terminal::Frame};

use super::Component;

#[derive(Default)]
pub struct Root;

impl Root {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Root {
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('q') => Action::Quit,
            _ => Action::Noop,
        }
    }

    fn render(&mut self, frame: &mut Frame<'_>, size: Rect) {
        frame.render_widget(Paragraph::new("Hello World!"), size);
    }
}
