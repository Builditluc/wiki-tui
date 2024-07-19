use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
};
use tui_input::{backend::crossterm::EventHandler, Input};
use wiki_api::languages::{Language, LANGUAGES};

use crate::{
    action::{Action, ActionPacket, ActionResult, SearchAction},
    terminal::Frame,
    ui::{centered_rect, StatefulList},
};

use super::Component;

const FOCUS_INPUT: u8 = 0;
const FOCUS_LIST: u8 = 1;

pub struct SearchLanguageSelectionComponent {
    input: Input,
    focus: u8,
    list: StatefulList<Language>,
}

impl SearchLanguageSelectionComponent {
    fn update_list(&mut self) {
        let input_value = self.input.value();
        let sorted_languages = LANGUAGES
            .iter()
            .filter(|lang| {
                let lang = lang.name().to_lowercase();
                let query = input_value.to_lowercase();
                lang.contains(&query)
            })
            .map(|x| x.to_owned())
            .collect::<Vec<Language>>();
        self.list = StatefulList::with_items(sorted_languages);
    }
}

impl Component for SearchLanguageSelectionComponent {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        match key.code {
            KeyCode::Enter => {
                if let Some(lang) = self.list.selected() {
                    return ActionPacket::single(Action::SwitchContextSearch)
                        .action(Action::PopPopup)
                        .action(Action::PopupMessage(
                            "Information".to_string(),
                            format!("Changed the language for searches to '{}'", lang.name()),
                        ))
                        .action(Action::Search(SearchAction::ChangeLanguage(
                            lang.to_owned(),
                        )))
                        .into();
                }
                ActionResult::Ignored
            }
            KeyCode::Tab | KeyCode::BackTab => {
                if self.focus == FOCUS_INPUT {
                    self.focus = FOCUS_LIST;
                } else if self.focus == FOCUS_LIST {
                    self.focus = FOCUS_INPUT;
                }

                tracing::debug!("focus now: '{}'", self.focus);

                ActionResult::consumed()
            }
            KeyCode::Char('i') if self.focus == FOCUS_LIST => {
                self.focus = FOCUS_INPUT;
                ActionResult::consumed()
            }

            KeyCode::Esc | KeyCode::F(2) => Action::PopPopup.into(),

            _ if self.focus == FOCUS_INPUT => {
                self.input.handle_event(&crossterm::event::Event::Key(key));
                self.update_list();
                ActionResult::consumed()
            }
            _ => ActionResult::Ignored,
        }
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::ScrollUp(n) => {
                for _ in 0..n {
                    self.list.previous()
                }
                ActionResult::consumed()
            }
            Action::ScrollDown(n) => {
                for _ in 0..n {
                    self.list.next()
                }
                ActionResult::consumed()
            }
            Action::UnselectScroll => {
                self.list.unselect();
                ActionResult::consumed()
            }
            _ => ActionResult::Ignored,
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let popup_block = Block::default()
            .title("Switch Search Language")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default());
        let area = centered_rect(area, 25, 60);
        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let (input_area, list_area) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(1), Constraint::Percentage(100)])
                .split(area);
            (chunks[0], chunks[1])
        };

        let scroll = self.input.visual_scroll(input_area.width as usize);
        let cursor = self.input.visual_cursor();
        let value = self.input.value();

        let input_widget = Paragraph::new(format!(
            "{}{}",
            value,
            "_".repeat((input_area.width as usize).saturating_sub(value.len()))
        ))
        .style(Style::default().bg(Color::Blue))
        .scroll((0, scroll as u16));
        f.render_widget(input_widget, input_area);

        if self.focus == FOCUS_INPUT {
            f.set_cursor(
                input_area.x + (cursor.max(scroll) - scroll) as u16,
                input_area.y,
            );
        }

        let list_items = self
            .list
            .get_items()
            .iter()
            .map(|x| ListItem::new(x.name().to_owned()));
        let list_widget = List::new(list_items).highlight_style(if self.focus == FOCUS_LIST {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC)
        } else {
            Style::default()
        });
        f.render_stateful_widget(list_widget, list_area, self.list.get_state_mut());
    }
}

impl Default for SearchLanguageSelectionComponent {
    fn default() -> Self {
        Self {
            input: Input::default(),
            list: StatefulList::with_items(Vec::new()),
            focus: 0,
        }
    }
}
