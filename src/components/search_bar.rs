use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_input::{backend::crossterm::EventHandler, Input};

use crate::{
    action::{Action, SearchAction},
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
    pub is_focussed: bool,
}

impl SearchBarComponent {
    pub fn clear(&mut self) {
        self.input = Input::default();
    }
}

impl Component for SearchBarComponent {
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Enter => {
                self.is_focussed = false;
                Action::Search(SearchAction::StartSearch(self.input.value().to_string()))
            }
            KeyCode::Esc => Action::ExitSearchBar,
            _ => {
                self.input.handle_event(&crossterm::event::Event::Key(key));
                Action::Noop
            }
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let scroll = self.input.visual_scroll(area.width as usize);
        let value = self.input.value();

        let input = if value.is_empty() {
            Paragraph::new(Text::styled(
                EMPTY_PROMPT,
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ))
        } else {
            Paragraph::new(self.input.value()).scroll((0, scroll as u16))
        }
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(match self.is_focussed {
                    true => Style::default().fg(Color::Yellow),
                    false => Style::default(),
                }),
        );

        f.render_widget(input, centered_rect(area, SEARCH_BAR_X, 100));
    }
}
