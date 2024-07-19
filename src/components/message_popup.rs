use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

use crate::{
    action::{Action, ActionResult},
    ui::centered_rect,
};

use super::Component;

pub struct MessagePopupComponent {
    title: String,
    content: String,
}

impl MessagePopupComponent {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}

impl Component for MessagePopupComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        match key.code {
            KeyCode::Esc => Action::PopPopup.into(),
            _ => ActionResult::Ignored,
        }
    }

    fn render(&mut self, f: &mut crate::terminal::Frame<'_>, area: ratatui::prelude::Rect) {
        let max_area = centered_rect(area, 30, 50);

        let width = (max_area.width as usize).min(self.content.chars().count() + 2) as usize;
        let wrapped_message = textwrap::wrap(&self.content, width);

        let height = (max_area.height as usize).min(wrapped_message.len() + 2);

        let area = Rect {
            x: area.x + (area.width - width as u16) / 2,
            y: area.y + (area.height - height as u16) / 2,
            width: width as u16,
            height: height as u16,
        };

        f.render_widget(Clear, area);
        let message_widget = Paragraph::new(self.content.as_str())
            .centered()
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .title(self.title.as_str())
                    .title_bottom(Line::from("<ESC> Dismiss").right_aligned())
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );
        f.render_widget(message_widget, area);
    }
}
