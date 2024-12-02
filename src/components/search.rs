use std::sync::Arc;

use anyhow::{Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{HighlightSpacing, List, ListItem, Wrap},
};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use wiki_api::{
    languages::Language,
    search::{Search as ApiSearch, SearchContinue, SearchInfo, SearchRequest, SearchResult},
    Endpoint,
};

use crate::{
    action::{Action, ActionPacket, ActionResult, SearchAction},
    config::{Config, Theme},
    terminal::Frame,
    ui::{centered_rect, ScrollBehaviour, StatefulList},
};

use super::Component;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum Mode {
    #[default]
    NoSearch,
    FinishedSearch,

    Suggestion,

    Searching,
}

pub struct SearchComponent {
    mode: Mode,
    pub endpoint: Option<Endpoint>,
    pub language: Option<Language>,

    search_results: StatefulList<SearchResult>,
    search_info: Option<SearchInfo>,
    continue_search: Option<SearchContinue>,

    config: Arc<Config>,
    theme: Arc<Theme>,

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

            config: Arc::new(Config::default()),
            theme: Arc::new(Theme::default()),

            action_tx: None,
        }
    }
}

impl SearchComponent {
    fn build_search(&self, query: String) -> Result<SearchRequest> {
        let api_config = &self.config.api;

        let endpoint = self.endpoint.clone().unwrap_or(api_config.endpoint.clone());
        let language = self.language.unwrap_or(api_config.language);

        Ok(ApiSearch::builder()
            .query(query)
            .endpoint(endpoint)
            .language(language)
            .limit(api_config.search_limit)
            .qiprofile(api_config.search_qiprofile.clone())
            .search_type(api_config.search_type.clone())
            .info(api_config.search_info.clone())
            .rewrites(api_config.search_rewrites)
            .sort_order(api_config.search_sort_order.clone()))
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
            tx.send(Action::Search(SearchAction::ChangeMode(Mode::Searching)))
                .unwrap();
            tx.send(Action::Search(SearchAction::ClearSearchResults))
                .unwrap();
            match search_request.search().await {
                Ok(search) => tx
                    .send(Action::Search(SearchAction::FinshSearch(search)))
                    .unwrap(),
                Err(error) => {
                    let error = error.context("Unable to execute the search");
                    tx.send(Action::PopupError(error.to_string())).unwrap();
                    error!("{:?}", error);
                }
            };
        });

        ActionResult::consumed()
    }

    fn finish_search(&mut self, mut search: ApiSearch) -> ActionResult {
        let has_results = search.info.total_hits.unwrap_or_default() != 0;
        let has_suggestion = search.info.suggestion.is_some();

        self.search_results
            .get_items_mut()
            .append(&mut search.results);

        self.continue_search = search.continue_data().take();
        self.search_info = Some(search.info);

        crate::trace_dbg!(has_results);
        crate::trace_dbg!(has_suggestion);

        if !has_results && !has_suggestion {
            warn!("could not find any results and no suggestion was given");
            return ActionPacket::single(Action::PopupMessage(
                "Warning".to_string(),
                "Could not find any search results and no suggestion could be made".to_string(),
            ))
            .action(Action::Search(SearchAction::ChangeMode(Mode::NoSearch)))
            .into();
        }

        if !has_results && has_suggestion {
            info!("could not find any results, but a suggestion was given",);
            return Action::Search(SearchAction::ChangeMode(Mode::Suggestion)).into();
        }

        Action::Search(SearchAction::ChangeMode(Mode::FinishedSearch)).into()
    }

    fn continue_search(&mut self) -> ActionResult {
        if self.continue_search.is_none() {
            return ActionPacket::single(Action::PopupMessage(
                "Warning".to_string(),
                "Could not find any search results and no suggestion could be made".to_string(),
            ))
            .action(Action::Search(SearchAction::ChangeMode(Mode::NoSearch)))
            .into();
        }

        let code = self.continue_search.as_ref().unwrap();
        let tx = self.action_tx.clone().unwrap();
        let search_request = ApiSearch::builder()
            .query(code.query.clone())
            .endpoint(code.endpoint.clone())
            .language(code.language)
            .offset(code.offset);
        tokio::spawn(async move {
            tx.send(Action::Search(SearchAction::ChangeMode(Mode::Searching)))
                .unwrap();
            match search_request.search().await {
                Ok(search) => tx
                    .send(Action::Search(SearchAction::FinshSearch(search)))
                    .unwrap(),
                Err(error) => {
                    let error = error.context("Unable to continue the search");
                    tx.send(Action::PopupError(error.to_string())).unwrap();
                    error!("{:?}", error)
                }
            };
        });

        ActionResult::consumed()
    }

    fn open_selected_result(&self) -> ActionResult {
        if let Some(selected_result) = self.search_results.selected() {
            return ActionPacket::default()
                .action(Action::ClearSearchBar)
                .action(Action::LoadSearchResult(selected_result.clone()))
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

    fn change_mode(&mut self, mode: Mode) -> ActionResult {
        self.mode = mode;
        ActionResult::consumed()
    }

    fn change_language(&mut self, lang: Language) -> ActionResult {
        self.endpoint = Some(
            // HACK: we need a way to get the API endpoint from the language
            Endpoint::parse(&format!("https://{}.wikipedia.org/w/api.php", lang.code())).unwrap(),
        );
        self.language = Some(lang);
        ActionResult::consumed()
    }
}

impl Component for SearchComponent {
    fn init(
        &mut self,
        sender: mpsc::UnboundedSender<Action>,
        config: Arc<Config>,
        theme: Arc<Theme>,
    ) -> anyhow::Result<()> {
        self.action_tx = Some(sender);
        self.config = config;
        self.theme = theme;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        match self.mode {
            Mode::Searching => ActionResult::Ignored,
            Mode::Suggestion => {
                match key.code {
                    KeyCode::Char('y') => {
                        // HACK: well, some checks would be cool
                        let suggestion = self
                            .search_info
                            .as_ref()
                            .unwrap()
                            .suggestion
                            .as_ref()
                            .unwrap()
                            .as_str();
                        Action::Search(SearchAction::StartSearch(suggestion.to_string())).into()
                    }
                    KeyCode::Char('n') => {
                        Action::Search(SearchAction::ChangeMode(Mode::NoSearch)).into()
                    }
                    _ => ActionResult::Ignored,
                }
            }
            Mode::FinishedSearch => match key.code {
                _ if self.search_results.is_selected()
                    && self.config.bindings.global.submit.matches_event(key) =>
                {
                    Action::Search(SearchAction::OpenSearchResult).into()
                }
                _ if self
                    .config
                    .bindings
                    .search
                    .continue_search
                    .matches_event(key) =>
                {
                    Action::Search(SearchAction::ContinueSearch).into()
                }
                _ => ActionResult::Ignored,
            },
            _ => ActionResult::Ignored,
        }
    }

    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::Search(search_action) => match search_action {
                SearchAction::StartSearch(query) => self.start_search(query),
                SearchAction::FinshSearch(search) => self.finish_search(search),
                SearchAction::ContinueSearch => self.continue_search(),
                SearchAction::ClearSearchResults => self.clear_search_results(),
                SearchAction::OpenSearchResult => self.open_selected_result(),
                SearchAction::ChangeMode(mode) => self.change_mode(mode),
                SearchAction::ChangeLanguage(lang) => self.change_language(lang),
            },

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
        if self.mode == Mode::Searching {
            f.render_widget(
                self.theme
                    .default_block()
                    .border_style(Style::default().fg(Color::Yellow)),
                area,
            );
            f.render_widget(
                self.theme
                    .default_paragraph("Searching. Please wait...")
                    .alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        if self.mode == Mode::NoSearch {
            f.render_widget(
                self.theme
                    .default_paragraph("Start a search!")
                    .alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        if self.mode == Mode::Suggestion {
            if self.search_info.is_none() {
                return;
            }

            let block = self.theme.default_block().title("Information");
            let msg = format!(
                "No results for '{}' were found. Do you want to search for '{}' instead?\n\n[y]/[n]",
                self.search_info.as_ref().unwrap().query.as_str(),
                self.search_info
                    .as_ref()
                    .unwrap()
                    .suggestion
                    .as_ref()
                    .unwrap()
            );
            let area = centered_rect(area, 60, 25);
            f.render_widget(
                self.theme
                    .default_paragraph(msg)
                    .block(block)
                    .wrap(Wrap { trim: true }),
                area,
            );
        }

        // below this we should be in FinishedSearch mode
        if self.mode != Mode::FinishedSearch {
            return;
        }

        if self.search_results.get_items().is_empty() {
            f.render_widget(
                self.theme
                    .default_paragraph("Start a search to view the results!")
                    .alignment(Alignment::Center),
                centered_rect(area, 100, 50),
            );
            return;
        }

        let [info_area, results_area] = {
            let rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100), Constraint::Min(1)])
                .split(area);
            [rects[1], rects[0]]
        };

        if let Some(ref search_info) = self.search_info {
            let info = self
                .theme
                .default_paragraph(format!(
                    " wiki-tui | Results: '{}' | Language: '{}' | [c]ontinue",
                    search_info.total_hits.unwrap_or_default(),
                    search_info.language.name()
                ))
                .style(
                    Style::default()
                        .fg(self.theme.status_bar_fg)
                        .bg(self.theme.status_bar_bg),
                );

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

                let mut text =
                    Text::from(Span::raw(result.title.clone()).fg(self.theme.search_title_fg));
                text.lines.append(
                    &mut textwrap::wrap(&cleaned_snippet, results_list_width as usize)
                        .iter()
                        .map(|s| {
                            Line::from(s.to_string()).style(Style::default().fg(self.theme.fg))
                        })
                        .collect(),
                );
                ListItem::new(text)
            })
            .collect();

        let items = List::new(items)
            .block(self.theme.default_block().title("Results"))
            .repeat_highlight_symbol(true)
            .highlight_symbol("| ")
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_style(
                Style::default()
                    .fg(self.theme.selected_fg)
                    .bg(self.theme.selected_bg)
                    .add_modifier(Modifier::ITALIC),
            );
        f.render_stateful_widget(items, results_area, self.search_results.get_state_mut());
    }

    fn handle_events(&mut self, event: Option<crate::event::Event>) -> ActionResult {
        match event {
            Some(crate::event::Event::Quit) => Action::Quit.into(),
            Some(crate::event::Event::RenderTick) => Action::RenderTick.into(),
            Some(crate::event::Event::Key(key_event)) => self.handle_key_events(key_event),
            Some(crate::event::Event::Resize(x, y)) => Action::Resize(x, y).into(),
            None => ActionResult::Ignored,
        }
    }
}
