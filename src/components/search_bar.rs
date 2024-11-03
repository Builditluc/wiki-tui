use std::sync::Arc;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::Text,
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{
    action::{Action, ActionResult, SearchAction},
    config::{Config, Theme},
    terminal::Frame,
    ui::centered_rect,
};

use super::Component;

const EMPTY_PROMPT: &str = "Search Wikipedia";
const SEARCH_BAR_X: u16 = 50;

pub const SEARCH_BAR_HEIGTH: u16 = 3;

#[derive(Default)]
pub struct SearchBarComponent {
    input: Input,
    config: Arc<Config>,
    theme: Arc<Theme>,
    pub is_focussed: bool,
}

impl SearchBarComponent {
    pub fn clear(&mut self) {
        self.input = Input::default();
    }

    pub fn submit(&self) -> Action {
        Action::Search(SearchAction::StartSearch(self.input.value().to_string()))
    }
}

impl Component for SearchBarComponent {
    fn init(
        &mut self,
        _: tokio::sync::mpsc::UnboundedSender<Action>,
        config: Arc<Config>,
        theme: Arc<Theme>,
    ) -> anyhow::Result<()> {
        self.config = config;
        self.theme = theme;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        match key.code {
            KeyCode::Enter => Action::SubmitSearchBar.into(),
            KeyCode::Esc => Action::ExitSearchBar.into(),
            _ => {
                self.input.handle_event(&crossterm::event::Event::Key(key));
                ActionResult::consumed()
            }
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let scroll = self.input.visual_scroll(area.width as usize);
        let value = self.input.value();

        let mut block = self.theme.default_block();
        if self.is_focussed {
            block = block.border_style(
                Style::default()
                    .fg(self.theme.border_highlight_fg)
                    .bg(self.theme.border_highlight_bg),
            )
        }

        let input = if value.is_empty() {
            self.theme.default_paragraph(Text::styled(
                EMPTY_PROMPT,
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ))
        } else {
            self.theme
                .default_paragraph(self.input.value())
                .scroll((0, scroll as u16))
        }
        .block(block);

        let input_area = centered_rect(area, SEARCH_BAR_X, 100);
        f.render_widget(input, input_area);
        if self.is_focussed {
            f.set_cursor(
                // Put cursor past the end of the input text
                input_area.x + ((self.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            );
        }
    }
}
