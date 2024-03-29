use anyhow::{anyhow, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, Paragraph},
};
use tokio::sync::mpsc;
use tracing::error;
use wiki_api::{
    languages::Language,
    search::{Search as ApiSearch, SearchContinue, SearchInfo, SearchRequest, SearchResult},
    Endpoint,
};

use crate::{
    action::{Action, ActionPacket, ActionResult, SearchAction},
    key_event,
    terminal::Frame,
    ui::{centered_rect, ScrollBehaviour, StatefulList},
};

use super::Component;

#[derive(Default, Debug, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Processing,
}

pub struct SearchComponent {
    mode: Mode,
    endpoint: Option<Endpoint>,
    language: Option<Language>,

    search_results: StatefulList<SearchResult>,
    search_info: Option<SearchInfo>,
    continue_search: Option<SearchContinue>,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl Default for SearchComponent {
    fn default() -> SearchComponent {
        SearchComponent {
            mode: Mode::default(),
            endpoint: None,
            language: None,

            search_results: StatefulList::with_items(Vec::new())
                .scroll_behavior(ScrollBehaviour::StickToEnds),
            search_info: None,
            continue_search: None,

            action_tx: None,
        }
    }
}

impl SearchComponent {
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

    fn start_search(&mut self, query: String) -> ActionResult {
        let tx = self.action_tx.clone().unwrap();
        let search_request = match self.build_search(query) {
            Ok(search_request) => search_request,
            Err(error) => {
                error!("Unable to build the search request: {:?}", error);
                return ActionResult::consumed();
            }
        };
        tokio::spawn(async move {
            tx.send(Action::EnterProcessing).unwrap();
            tx.send(Action::Search(SearchAction::ClearSearchResults))
                .unwrap();
            match search_request.search().await {
                Ok(search) => tx
                    .send(Action::Search(SearchAction::FinshSearch(search)))
                    .unwrap(),
                Err(error) => error!("Unable to complete the search: {:?}", error),
            };
            tx.send(Action::EnterNormal).unwrap();
        });

        ActionResult::consumed()
    }

    fn finish_search(&mut self, mut search: ApiSearch) -> ActionResult {
        self.search_results
            .get_items_mut()
            .append(&mut search.results);
        self.search_results.next();

        self.continue_search = search.continue_data().take();
        self.search_info = Some(search.info);

        ActionResult::consumed()
    }

    fn open_selected_result(&self) -> ActionResult {
        if let Some(selected_result) = self.search_results.selected() {
            return ActionPacket::default()
                .action(Action::ClearSearchBar)
                .action(Action::LoadPage(selected_result.title.clone()))
                .into();
        }
        ActionResult::Ignored
    }

    fn clear_search_results(&mut self) -> ActionResult {
        self.search_results = StatefulList::with_items(Vec::new());
        self.continue_search = None;
        self.search_info = None;

        ActionResult::consumed()
    }
}

impl Component for SearchComponent {
    fn init(&mut self, sender: mpsc::UnboundedSender<Action>) -> anyhow::Result<()> {
        self.action_tx = Some(sender);
        // FIXME: the endpoint and language should be set by the root component
        self.endpoint = Some(Endpoint::parse("https://en.wikipedia.org/w/api.php").unwrap());
        self.language = Some(Language::default());
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Enter if self.search_results.is_selected() => {
                    Action::Search(SearchAction::OpenSearchResult).into()
                }
                _ => ActionResult::Ignored,
            },
            Mode::Processing => ActionResult::Ignored,
        }
    }

    fn keymap(&self) -> super::help::Keymap {
        vec![(
            key_event!(Key::Enter),
            ActionPacket::single(Action::Search(SearchAction::OpenSearchResult)),
        )]
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::Search(search_action) => match search_action {
                SearchAction::StartSearch(query) => self.start_search(query),
                SearchAction::FinshSearch(search) => self.finish_search(search),
                SearchAction::ClearSearchResults => self.clear_search_results(),
                SearchAction::OpenSearchResult => self.open_selected_result(),
            },
            Action::EnterNormal => {
                self.mode = Mode::Normal;
                ActionResult::consumed()
            }
            Action::EnterProcessing => {
                self.mode = Mode::Processing;
                ActionResult::consumed()
            }
            Action::ScrollUp(n) => {
                for _ in 0..n {
                    self.search_results.previous()
                }
                ActionResult::consumed()
            }
            Action::ScrollDown(n) => {
                for _ in 0..n {
                    self.search_results.next()
                }
                ActionResult::consumed()
            }
            Action::UnselectScroll => {
                self.search_results.unselect();
                ActionResult::consumed()
            }
            _ => ActionResult::Ignored,
        }
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        if self.mode == Mode::Processing {
            f.render_widget(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow)),
                area,
            );
            f.render_widget(
                Paragraph::new("Processing Search. Please wait...").alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        if self.search_results.get_items().is_empty() {
            f.render_widget(
                Paragraph::new("Start a search to view the results!").alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        let [info_area, results_area] = {
            let rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Percentage(100)])
                .split(area);
            [rects[0], rects[1]]
        };

        if let Some(ref search_info) = self.search_info {
            let info = Paragraph::new(format!(
                "Results: {} | Language: {}",
                search_info.total_hits.unwrap_or_default(),
                search_info.language.name()
            ));

            f.render_widget(info, info_area);
        }

        // TODO: Somehow implement list item margin
        let results_list_width = results_area.width.saturating_sub(3); // HACK: subtract 3 for
                                                                       // border and highlight symbol
        let items: Vec<ListItem> = self
            .search_results
            .get_items()
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
            .repeat_highlight_symbol(true)
            .highlight_symbol("| ")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            );
        f.render_stateful_widget(items, results_area, self.search_results.get_state_mut());
    }
}
