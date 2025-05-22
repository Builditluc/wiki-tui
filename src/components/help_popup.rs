use std::{cmp::max, sync::Arc};

use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Flex::Center, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

use crate::{
    action::{Action, ActionResult},
    config::{Config, Keybinding, Theme},
    terminal::Frame,
    ui::centered_rect,
};

use super::Component;

struct Binding {
    pub bindings: Box<[Box<str>]>,
    pub name: Box<str>,
}

impl Binding {
    pub fn keys_len(&self) -> usize {
        self.bindings.iter().fold(0, |acc, elm| acc + elm.len() + 2) - 2
    }

    pub fn to_line(&self, gap: usize, highlight: Color) -> Line {
        Itertools::intersperse(
            self.bindings
                .iter()
                .map(|b| Span::styled(b.as_ref(), Style::default().fg(highlight))),
            Span::raw(", "),
        )
        .take(self.bindings.len() * 2 - 1)
        .chain([
            (self.keys_len()..gap)
                .map(|_| ".")
                .collect::<String>()
                .into(),
            Span::raw(self.name.as_ref()),
        ])
        .collect()
    }
}

impl Binding {
    pub fn from_keys(key: &Keybinding, desc: &str) -> Self {
        let bindings = key
            .bindings()
            .iter()
            .map(|b| b.to_string().into())
            .collect();
        Self {
            bindings,
            name: desc.into(),
        }
    }
}

pub struct HelpPopupComponent {
    line: u16,

    global_bindings_list: Arc<[Binding]>,
    search_bindings_list: Arc<[Binding]>,
    page_bindings_list: Arc<[Binding]>,
    config: Arc<Config>,
    theme: Arc<Theme>,
}

impl HelpPopupComponent {
    pub fn new(config: Arc<Config>, theme: Arc<Theme>) -> Self {
        macro_rules! convert_binding {
            ($binding:expr) => {
                Binding::from_keys(&$binding, {
                    let string_name = stringify!($binding);
                    &match string_name.rsplit_once('.') {
                        None => string_name,
                        Some((_, name)) => name,
                    }
                    .replace('_', " ")
                })
            };
        }

        let global_bindings_list = vec![
            convert_binding!(config.bindings.global.scroll_down),
            convert_binding!(config.bindings.global.scroll_up),
            convert_binding!(config.bindings.global.scroll_to_top),
            convert_binding!(config.bindings.global.scroll_to_bottom),
            convert_binding!(config.bindings.global.pop_popup),
            convert_binding!(config.bindings.global.half_down),
            convert_binding!(config.bindings.global.half_up),
            convert_binding!(config.bindings.global.unselect_scroll),
            convert_binding!(config.bindings.global.submit),
            convert_binding!(config.bindings.global.quit),
            convert_binding!(config.bindings.global.enter_search_bar),
            convert_binding!(config.bindings.global.exit_search_bar),
            convert_binding!(config.bindings.global.switch_context_search),
            convert_binding!(config.bindings.global.switch_context_page),
            convert_binding!(config.bindings.global.toggle_search_language_selection),
            convert_binding!(config.bindings.global.toggle_logger),
            convert_binding!(config.bindings.global.help),
        ]
        .into();

        let search_bindings_list =
            vec![convert_binding!(config.bindings.search.continue_search)].into();

        let page_bindings_list = vec![
            convert_binding!(config.bindings.page.pop_page),
            convert_binding!(config.bindings.page.jump_to_header),
            convert_binding!(config.bindings.page.select_first_link),
            convert_binding!(config.bindings.page.select_last_link),
            convert_binding!(config.bindings.page.select_prev_link),
            convert_binding!(config.bindings.page.select_next_link),
            convert_binding!(config.bindings.page.open_link),
            convert_binding!(config.bindings.page.toggle_page_language_selection),
            convert_binding!(config.bindings.page.toggle_zen_mode),
            convert_binding!(config.bindings.page.toggle_toc),
        ]
        .into();

        Self {
            line: 0,

            global_bindings_list,
            search_bindings_list,
            page_bindings_list,
            config,
            theme,
        }
    }

    pub fn data_size(&self) -> u16 {
        (self.global_bindings_list.len()
            + self.search_bindings_list.len()
            + self.page_bindings_list.len()) as u16
    }
}

impl Component for HelpPopupComponent {
    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> ActionResult {
        if self.config.bindings.global.pop_popup.matches_event(key)
            | self.config.bindings.global.quit.matches_event(key)
        {
            return Action::PopPopup.into();
        }

        ActionResult::Ignored
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::ScrollUp(n) => {
                self.line = self.line.saturating_sub(n);

                ActionResult::consumed()
            }
            Action::ScrollDown(n) => {
                self.line += n;
                ActionResult::consumed()
            }
            Action::Quit => Action::PopPopup.into(),
            _ => ActionResult::Ignored,
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let popup_block = self
            .theme
            .default_block()
            .title("Help")
            .style(Style::default().bg(self.theme.bg));
        let area = centered_rect(area, 95, 95);

        f.render_widget(Clear, area);
        f.render_widget(popup_block, area);

        let [text_area] = Layout::vertical([Constraint::Percentage(100)])
            .margin(1)
            .flex(Center)
            .areas(area);

        let gap = self
            .global_bindings_list
            .iter()
            .chain(self.search_bindings_list.iter())
            .chain(self.page_bindings_list.iter())
            .fold(0, |acc, elm| max(acc, elm.keys_len()))
            + 1;
        let highlight_color = self.theme.search_title_fg;
        macro_rules! to_line {
            ($bindings:expr) => {
                $bindings
                    .iter()
                    .map(|x| x.to_line(gap, highlight_color))
                    .collect()
            };
        }

        let consolidated_list: Vec<Line> = vec![
            vec![Line::raw("Global").underlined()],
            to_line!(self.global_bindings_list),
            vec![Line::default(), Line::raw("Search").underlined()],
            to_line!(self.search_bindings_list),
            vec![Line::default(), Line::raw("Page").underlined()],
            to_line!(self.page_bindings_list),
        ]
        .concat();

        if self.data_size() < text_area.height {
            self.line = 0;
        } else if self.line > self.data_size() - text_area.height {
            self.line = self.data_size() - text_area.height;
        };

        let paragraph_widget = Paragraph::new(consolidated_list).scroll((self.line, 0));

        f.render_widget(paragraph_widget, text_area);
    }
}
