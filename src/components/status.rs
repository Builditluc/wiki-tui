use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    widgets::Paragraph,
};

use crate::{app::Context, terminal::Frame};

use super::Component;

const HELP_MSG: &str = "Press [?] for help";
const HELP_MSG_LEN: u16 = HELP_MSG.len() as u16;

pub const STATUS_HEIGHT: u16 = 1;

#[derive(Default)]
pub struct StatusComponent {
    focus: Context,
}

impl StatusComponent {
    pub fn set_focus(&mut self, focus: Context) {
        self.focus = focus
    }
}

impl Component for StatusComponent {
    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100), Constraint::Min(HELP_MSG_LEN)])
            .split(area);

        f.render_widget(Paragraph::new(HELP_MSG), chunks[1]);
    }
}
