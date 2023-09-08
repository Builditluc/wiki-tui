use anyhow::Result;
use log::LevelFilter;
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    widgets::{Block, Borders},
};
use tokio::sync::mpsc;
use tui_logger::{TuiLoggerWidget, TuiWidgetState};

use crate::{action::Action, terminal::Frame};

use super::Component;

#[derive(Default)]
pub struct Logger {
    state: TuiWidgetState,
}

impl Component for Logger {
    fn init(&mut self, _: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.state = TuiWidgetState::new().set_default_display_level(LevelFilter::Debug);
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame<'_>, size: Rect) {
        let widget = TuiLoggerWidget::default()
            .block(Block::new().title("Log").borders(Borders::ALL))
            .style_error(Style::default().fg(Color::Red))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_info(Style::default().fg(Color::Cyan))
            .style_debug(Style::default().fg(Color::Green))
            .style_trace(Style::default().fg(Color::Magenta))
            .state(&self.state);
        frame.render_widget(widget, size)
    }
}
