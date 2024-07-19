use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Rect},
    style::Stylize,
    text::Line,
    widgets::{block::Title, Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{
    action::{Action, ActionResult},
    ui::centered_rect,
};

use super::Component;

pub struct MessagePopupComponent<'a> {
    title: Title<'a>,
    content: String,
    content_alignment: Alignment,
}

impl<'a> MessagePopupComponent<'a> {
    pub fn new_raw(title: String, content: String) -> Self {
        Self {
            title: Title::from(title).alignment(Alignment::Center),
            content,
            content_alignment: Alignment::Center,
        }
    }

    pub fn new_error(error: String) -> Self {
        const ERROR_MESSAGE: &str =
            "An error occurred\nCheck the logs for further information\n\nError: {ERROR}";

        Self {
            title: Title::from("Error".bold().red()).alignment(Alignment::Center),
            content: ERROR_MESSAGE.replace("{ERROR}", &error),
            content_alignment: Alignment::Left,
        }
    }
}

impl<'a> Component for MessagePopupComponent<'a> {
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
        let message_widget = Paragraph::new(
            wrapped_message
                .iter()
                .map(|x| Line::from(x.to_string()))
                .collect::<Vec<Line>>(),
        )
        .alignment(self.content_alignment)
        .block(
            Block::default()
                .title(self.title.clone())
                .title_bottom(Line::from("<ESC> Dismiss").right_aligned())
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
        f.render_widget(message_widget, area);
    }
}
