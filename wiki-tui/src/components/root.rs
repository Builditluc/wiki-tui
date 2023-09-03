use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

use crate::{action::Action, terminal::Frame};

use super::{logger::Logger, Component};

#[derive(Default)]
pub struct Root {
    logger: Logger,
    show_logger: bool,
}

impl Root {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Root {
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('l') => Action::ToggleShowLogger,
            KeyCode::Char('q') => Action::Quit,
            _ => Action::Noop,
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::ToggleShowLogger => {
                self.show_logger = !self.show_logger;
                None
            }
            _ => None,
        }
    }

    fn render(&mut self, frame: &mut Frame<'_>, size: Rect) {
        let size = if self.show_logger {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);
            self.logger.render(frame, chunks[1]);
            chunks[0]
        } else {
            size
        };

        frame.render_widget(Paragraph::new("Hello World!"), size);
    }
}
