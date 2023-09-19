use anyhow::{anyhow, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};
use tokio::sync::mpsc;
use tracing::error;
use tui_input::{backend::crossterm::EventHandler, Input};
use wiki_api::{
    languages::Language,
    search::{Search as ApiSearch, SearchContinue, SearchInfo, SearchRequest, SearchResult},
    Endpoint,
};

use crate::{action::Action, terminal::Frame};

use super::{root::Context, Component};

struct ResultsList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> ResultsList<T> {
    fn with_items(items: Vec<T>) -> Self {
        ResultsList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    0
                } else {
                    i.saturating_add(1)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len().saturating_sub(1)
                } else {
                    i.saturating_sub(1)
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    fn unselect(&mut self) {
        self.state.select(None)
    }

    fn is_selected(&self) -> bool {
        self.state.selected().is_some()
    }

    fn selected(&self) -> Option<&T> {
        self.state.selected().map(|i| &self.items[i])
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Insert,
    Processing,
}

pub struct Search {
    mode: Mode,
    input: Input,
    endpoint: Option<Endpoint>,
    language: Option<Language>,

    search_results: ResultsList<SearchResult>,
    search_info: Option<SearchInfo>,
    continue_search: Option<SearchContinue>,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl Default for Search {
    fn default() -> Search {
        Search {
            mode: Mode::default(),
            input: Input::default(),
            endpoint: None,
            language: None,

            search_results: ResultsList::with_items(Vec::new()),
            search_info: None,
            continue_search: None,

            action_tx: None,
        }
    }
}

impl Search {
    fn build_search(&self, query: String) -> Result<SearchRequest> {
        let endpoint = self
            .endpoint
            .clone()
            .ok_or(anyhow!("No Endpoint configured"))?;
        let language = self
            .language
            .clone()
            .ok_or(anyhow!("No Language configured"))?;

        Ok(ApiSearch::builder()
            .query(query)
            .endpoint(endpoint)
            .language(language))
    }

    fn execute_search(&mut self, query: String) {
        let tx = self.action_tx.clone().unwrap();
        let search_request = match self.build_search(query) {
            Ok(search_request) => search_request,
            Err(error) => {
                error!("Unable to build the search request: {:?}", error);
                return;
            }
        };
        tokio::spawn(async move {
            tx.send(Action::EnterProcessing).unwrap();
            match search_request.search().await {
                Ok(search) => tx.send(Action::FinshSearch(search)).unwrap(),
                Err(error) => error!("Unable to complete the search: {:?}", error),
            };
            tx.send(Action::ExitProcessing).unwrap();
        });
    }

    fn finish_search(&mut self, mut search: ApiSearch) {
        self.search_results.items.append(&mut search.results);
        self.continue_search = search.continue_data().take();
        self.search_info = Some(search.info);
    }

    fn open_selected_result(&self) {
        if let Some(selected_result) = self.search_results.selected() {
            let action_tx = self.action_tx.clone().unwrap();
            action_tx.send(Action::EnterContext(Context::Page)).unwrap();
            action_tx
                .send(Action::OpenPage(selected_result.title.to_string()))
                .unwrap();
        }
    }
}

impl Component for Search {
    fn init(&mut self, sender: mpsc::UnboundedSender<Action>) -> anyhow::Result<()> {
        self.action_tx = Some(sender);
        // FIXME: the endpoint and language should be set by the root component
        self.endpoint = Some(Endpoint::parse("https://en.wikipedia.org/w/api.php").unwrap());
        self.language = Some(Language::default());
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Char('h') if key.modifiers == KeyModifiers::CONTROL => {
                    Action::EnterContext(Context::Home)
                }
                KeyCode::Char('i') => Action::EnterInsert,
                KeyCode::Enter if self.search_results.is_selected() => {
                    self.open_selected_result();
                    Action::Noop
                }
                _ => Action::Noop,
            },
            Mode::Insert => match key.code {
                KeyCode::Esc => Action::ExitInsert,
                KeyCode::Enter => Action::StartSearch(self.input.value().to_string()),
                _ => {
                    self.input.handle_event(&crossterm::event::Event::Key(key));
                    Action::Noop
                }
            },
            Mode::Processing => Action::Noop,
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        // FIXME: make this cleaner
        match action {
            Action::EnterNormal => {
                self.mode = Mode::Normal;
                None
            }
            Action::EnterInsert => {
                self.mode = Mode::Insert;
                None
            }
            Action::ExitInsert => {
                self.mode = Mode::Normal;
                None
            }
            Action::EnterProcessing => {
                self.mode = Mode::Processing;
                None
            }
            Action::ExitProcessing => {
                // TODO: make this exit to the previous mode
                self.mode = Mode::Normal;
                None
            }
            Action::StartSearch(query) => {
                self.execute_search(query);
                if self.mode == Mode::Insert {
                    Some(Action::ExitInsert)
                } else {
                    None
                }
            }
            Action::FinshSearch(search) => {
                self.finish_search(search);
                None
            }
            Action::ScrollUp(n) => {
                for _ in 0..n {
                    self.search_results.previous();
                }
                None
            }
            Action::ScrollDown(n) => {
                for _ in 0..n {
                    self.search_results.next();
                }
                None
            }
            Action::UnselectScroll => {
                self.search_results.unselect();
                None
            }
            _ => None,
        }
    }

    fn render(&mut self, frame: &mut Frame<'_>, size: Rect) {
        let [input_area, results_area, info_area] = {
            let rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Percentage(100),
                    Constraint::Min(1),
                ])
                .split(size);
            [rects[0], rects[1], rects[2]]
        };

        let width = input_area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let input = Paragraph::new(self.input.value())
            .style(match self.mode {
                Mode::Insert => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .scroll((0, scroll as u16))
            .block(
                Block::new()
                    .title("Search")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            );

        frame.render_widget(input, input_area);

        if self.search_results.items.is_empty() {
            return;
        }

        // TODO: Somehow implement list item margin
        let results_list_width = results_area.width;
        let items: Vec<ListItem> = self
            .search_results
            .items
            .iter()
            .map(|result| {
                let snippet = result.snippet.clone().unwrap();
                let mut cleaned_snippet = String::new();
                for slice in snippet
                    .split(r#"<span class="searchmatch">"#)
                    .collect::<Vec<&str>>()
                {
                    let split_slice: Vec<&str> = slice.split("</span>").collect();
                    cleaned_snippet.push_str(&split_slice.join(""));
                }

                let mut text = Text::from(Span::raw(result.title.clone()).red());
                text.lines.append(
                    &mut textwrap::wrap(&cleaned_snippet, results_list_width as usize)
                        .iter()
                        .map(|s| Line::from(s.to_string()))
                        .collect(),
                );
                ListItem::new(text)
            })
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Results"))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(items, results_area, &mut self.search_results.state);

        if let Some(ref search_info) = self.search_info {
            let info = Paragraph::new(format!(
                "Results: {} | Language: {}",
                search_info.total_hits.unwrap_or_default(),
                search_info.language.name()
            ));

            frame.render_widget(info, info_area);
        }
    }
}
