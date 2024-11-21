use std::sync::Arc;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    widgets::{Clear, List, ListItem},
};
use tui_input::{backend::crossterm::EventHandler, Input};
use wiki_api::page::LanguageLink;

use crate::{
    action::{Action, ActionPacket, ActionResult},
    config::{Config, Theme},
    terminal::Frame,
    ui::{centered_rect, StatefulList},
};

use super::Component;

const FOCUS_INPUT: u8 = 0;
const FOCUS_LIST: u8 = 1;

pub struct PageLanguageSelectionComponent {
    input: Input,
    focus: u8,
    list: StatefulList<LanguageLink>,
    language_links: Vec<LanguageLink>,

    config: Arc<Config>,
    theme: Arc<Theme>,
}

impl PageLanguageSelectionComponent {
    pub fn new(language_links: Vec<LanguageLink>, config: Arc<Config>, theme: Arc<Theme>) -> Self {
        Self {
            input: Input::default(),
            list: StatefulList::with_items(language_links.clone()),
            language_links,
            focus: 0,

            config,
            theme,
        }
    }

    fn update_list(&mut self) {
        let input_value = self.input.value();
        let sorted_languages = self
            .language_links
            .iter()
            .filter(|lang_link| {
                let lang = lang_link.language.name().to_lowercase();
                let query = input_value.to_lowercase();
                lang.contains(&query)
            })
            .map(|x| x.to_owned())
            .collect::<Vec<LanguageLink>>();
        self.list = StatefulList::with_items(sorted_languages);
    }
}

impl Component for PageLanguageSelectionComponent {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        if self.config.bindings.global.submit.matches_event(key) {
            if let Some(link) = self.list.selected() {
                return ActionPacket::single(Action::PopPopup)
                    .action(Action::PopupMessage(
                        "Information".to_string(),
                        format!(
                            "Changing the language of the page to '{}'",
                            link.language.name()
                        ),
                    ))
                    .action(Action::LoadLangaugeLink(link.to_owned()))
                    .into();
            }
            return ActionResult::Ignored;
        }

        if self.config.bindings.global.pop_popup.matches_event(key) {
            return Action::PopPopup.into();
        }

        match key.code {
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
            KeyCode::F(3) => Action::PopPopup.into(),
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
        let popup_block = self
            .theme
            .default_block()
            .title("Switch Page Language")
            .style(Style::default().bg(self.theme.bg));
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

        let input_widget = self
            .theme
            .default_paragraph(format!(
                "{}{}",
                value,
                "_".repeat((input_area.width as usize).saturating_sub(value.len()))
            ))
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
            .map(|x| ListItem::new(x.language.name().to_owned()).fg(self.theme.fg));
        let list_widget = List::new(list_items).highlight_style(if self.focus == FOCUS_LIST {
            Style::default()
                .fg(self.theme.selected_fg)
                .bg(self.theme.selected_bg)
                .add_modifier(Modifier::ITALIC)
        } else {
            Style::default()
        });
        f.render_stateful_widget(list_widget, list_area, self.list.get_state_mut());
    }
}
