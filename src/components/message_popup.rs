use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Clear},
};

use crate::{
    action::{Action, ActionPacket, ActionResult},
    config::Theme,
    ui::centered_rect,
};

use super::Component;

pub struct MessagePopupComponent {
    title: String,
    content: String,
    content_alignment: Alignment,

    theme: Arc<Theme>,

    confirmation: Option<ActionPacket>,
}

impl MessagePopupComponent {
    pub fn new_raw(title: String, content: String, theme: Arc<Theme>) -> Self {
        Self {
            title,
            content,
            content_alignment: Alignment::Center,

            theme,

            confirmation: None,
        }
    }

    pub fn new_error(error: String, theme: Arc<Theme>) -> Self {
        const ERROR_MESSAGE: &str =
            "An error occurred\nCheck the logs for further information\n\nError: {ERROR}";

        Self {
            title: "Error".to_string(),
            content: ERROR_MESSAGE.replace("{ERROR}", &error),
            content_alignment: Alignment::Left,

            theme,

            confirmation: None,
        }
    }

    pub fn new_confirmation(
        title: String,
        content: String,
        cb: ActionPacket,
        theme: Arc<Theme>,
    ) -> Self {
        Self {
            title,
            content,
            content_alignment: Alignment::Center,

            theme,

            confirmation: Some(cb),
        }
    }
}

impl Component for MessagePopupComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        match key.code {
            KeyCode::Char('y') if self.confirmation.is_some() => self
                .confirmation
                .take()
                .unwrap()
                .action(Action::PopPopup)
                .into(),
            KeyCode::Char('n') if self.confirmation.is_some() => Action::PopPopup.into(),

            KeyCode::Esc => Action::PopPopup.into(),
            _ => ActionResult::Ignored,
        }
    }

    fn render(&mut self, f: &mut crate::terminal::Frame<'_>, area: ratatui::prelude::Rect) {
        let max_area = centered_rect(area, 50, 80);

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
        f.render_widget(
            Block::default().style(Style::default().bg(self.theme.bg)),
            area,
        );

        let title_line = if self.title == "Error" {
            Line::from("Error".bold().red()).centered()
        } else {
            Line::from(self.title.clone()).centered()
        };
        
        let mut block = self.theme.default_block().title_top(title_line);

        block = if self.confirmation.is_some() {
            block
                .title_bottom(Line::from(vec!["Y".bold(), "es".into()]).right_aligned())
                .title_bottom(Line::from(vec!["N".bold(), "o".into()]).right_aligned())
        } else {
            block.title_bottom(Line::from("<ESC> Dismiss").right_aligned())
        };

        let message_widget = self
            .theme
            .default_paragraph(
                wrapped_message
                    .iter()
                    .map(|x| Line::from(x.to_string()))
                    .collect::<Vec<Line>>(),
            )
            .alignment(self.content_alignment)
            .block(block);
        f.render_widget(message_widget, area);
    }
}
