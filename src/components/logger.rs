use std::sync::Arc;

use anyhow::Result;
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
};
use tokio::sync::mpsc;
use tui_logger::{TuiLoggerWidget, TuiWidgetState};

use crate::{
    action::Action,
    config::{Config, Theme},
    terminal::Frame,
};

use super::Component;

#[derive(Default)]
pub struct LoggerComponent {
    state: TuiWidgetState,
    config: Arc<Config>,
    theme: Arc<Theme>,
}

impl Component for LoggerComponent {
    fn init(
        &mut self,
        _: mpsc::UnboundedSender<Action>,
        config: Arc<Config>,
        theme: Arc<Theme>,
    ) -> Result<()> {
        self.state = TuiWidgetState::new();
        self.config = config;
        self.theme = theme;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame<'_>, size: Rect) {
        let widget = TuiLoggerWidget::default()
            .block(self.theme.default_block().title("Log"))
            .style_error(Style::default().fg(Color::Red))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_info(Style::default().fg(Color::Cyan))
            .style_debug(Style::default().fg(Color::Green))
            .style_trace(Style::default().fg(Color::Magenta))
            .state(&self.state);
        frame.render_widget(widget, size)
    }
}
