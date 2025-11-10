use std::sync::Arc;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::Block,
};
use tracing::warn;

use tokio::sync::mpsc;

use crate::{
    action::{Action, ActionPacket, ActionResult},
    components::{
        help_popup::HelpPopupComponent,
        logger::LoggerComponent,
        message_popup::MessagePopupComponent,
        page_viewer::PageViewer,
        search::SearchComponent,
        search_bar::{SearchBarComponent, SEARCH_BAR_HEIGTH},
        search_language_popup::SearchLanguageSelectionComponent,
        Component,
    },
    config::{Config, Theme, ZenModeComponents},
    has_modifier,
    page_loader::PageLoader,
    terminal::Frame,
};

const CONTEXT_SEARCH: u8 = 0;
const CONTEXT_PAGE: u8 = 1;

#[derive(Default)]
pub struct AppComponent {
    search: SearchComponent,
    page: PageViewer,
    logger: LoggerComponent,
    search_bar: SearchBarComponent,
    page_loader: Option<PageLoader>,

    is_logger: bool,

    popups: Vec<Box<dyn Component + Send>>,
    config: Arc<Config>,
    theme: Arc<Theme>,

    context: u8,
    prev_context: u8,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl AppComponent {
    fn switch_context(&mut self, context: u8) {
        self.prev_context = context;
        std::mem::swap(&mut self.prev_context, &mut self.context);
    }

    fn show_page_language(&mut self) {
        let selection_widget = self.page.get_page_language_selection_popup();
        self.popups.push(Box::new(selection_widget));
    }

    fn render_search_bar(&mut self, f: &mut Frame<'_>, area: Rect) -> Rect {
        let (search_bar_area, area) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(SEARCH_BAR_HEIGTH),
                    Constraint::Percentage(100),
                ])
                .split(area);
            (chunks[0], chunks[1])
        };

        self.search_bar.render(f, search_bar_area);
        area
    }
}

impl Component for AppComponent {
    fn init(
        &mut self,
        action_tx: mpsc::UnboundedSender<Action>,
        config: Arc<Config>,
        theme: Arc<Theme>,
    ) -> Result<()> {
        self.search
            .init(action_tx.clone(), config.clone(), theme.clone())?;
        self.page
            .init(action_tx.clone(), config.clone(), theme.clone())?;
        self.search_bar
            .init(action_tx.clone(), config.clone(), theme.clone())?;

        self.page_loader = Some(PageLoader::new(config.clone(), action_tx.clone()));

        action_tx.send(Action::EnterSearchBar).unwrap();
        self.action_tx = Some(action_tx);

        self.config = config;
        self.theme = theme;

        Ok(())
    }
    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        // we need to always handle CTRL-C
        if matches!(key.code, KeyCode::Char('c') if has_modifier!(key, Modifier::CONTROL)) {
            return Action::Quit.into();
        }

        if let Some(ref mut popup) = self.popups.last_mut() {
            let result = popup.handle_key_events(key);
            if result.is_consumed() {
                return result;
            }
        }

        if self.search_bar.is_focussed {
            return self.search_bar.handle_key_events(key);
        }

        let result = match self.context {
            CONTEXT_SEARCH => self.search.handle_key_events(key),
            CONTEXT_PAGE => self.page.handle_key_events(key),
            _ => {
                warn!("unknown context");
                ActionResult::Ignored
            }
        };

        if result.is_consumed() {
            return result;
        }

        let global_bindings = &self.config.bindings.global;
        macro_rules! match_bindings {
            ($($bind:ident => $action:expr),+) => {
                $(if global_bindings.$bind.matches_event(key) {
                    return $action.into();
                })+
            };
        }

        match_bindings!(
            quit => Action::Quit,
            pop_popup => Action::PopPopup,

            toggle_logger => Action::ToggleShowLogger,

            switch_context_search => Action::SwitchContextSearch,
            switch_context_page => Action::SwitchContextPage,

            scroll_down => Action::ScrollDown(1),
            scroll_up => Action::ScrollUp(1),

            scroll_to_top => Action::ScrollToTop,
            scroll_to_bottom => Action::ScrollToBottom,

            half_up => Action::ScrollHalfUp,
            half_down => Action::ScrollHalfDown,

            unselect_scroll => Action::UnselectScroll,
            enter_search_bar => Action::EnterSearchBar,

            toggle_search_language_selection => {
                self.popups
                    .push(Box::new(SearchLanguageSelectionComponent::new(
                        self.config.clone(),
                        self.theme.clone(),
                    )));
                ActionResult::consumed()
            },

            help => {
                self.popups
                    .push(Box::new(HelpPopupComponent::new(
                        self.config.clone(),
                        self.theme.clone(),
                    )));
                ActionResult::consumed()
            }
        );

        ActionResult::Ignored
    }

    fn update(&mut self, action: Action) -> ActionResult {
        // global actions
        match action {
            Action::PopPopup => {
                self.popups.pop();
            }

            Action::ToggleShowLogger => self.is_logger = !self.is_logger,
            Action::ShowPageLanguageSelection => self.show_page_language(),

            Action::SwitchContextSearch => self.switch_context(CONTEXT_SEARCH),
            Action::SwitchContextPage => self.switch_context(CONTEXT_PAGE),
            Action::SwitchPreviousContext => self.switch_context(self.prev_context),

            Action::EnterSearchBar => self.search_bar.is_focussed = true,
            Action::ExitSearchBar => self.search_bar.is_focussed = false,
            Action::ClearSearchBar => self.search_bar.clear(),
            Action::SubmitSearchBar => {
                return ActionPacket::default()
                    .action(Action::ExitSearchBar)
                    .action(Action::SwitchContextSearch)
                    .action(self.search_bar.submit())
                    .into()
            }

            Action::LoadSearchResult(title) => {
                self.page_loader.as_ref().unwrap().load_search_result(title)
            }
            Action::LoadLink(link) => self.page_loader.as_ref().unwrap().load_link(link),
            Action::LoadLangaugeLink(link) => {
                self.page_loader.as_ref().unwrap().load_language_link(link)
            }

            Action::PopupMessage(title, content) => self.popups.push(Box::new(
                MessagePopupComponent::new_raw(title, content, self.theme.clone()),
            )),
            Action::PopupError(error) => self.popups.push(Box::new(
                MessagePopupComponent::new_error(error, self.theme.clone()),
            )),
            Action::PopupDialog(title, content, cb) => {
                self.popups
                    .push(Box::new(MessagePopupComponent::new_confirmation(
                        title,
                        content,
                        *cb,
                        self.theme.clone(),
                    )))
            }
            _ => {
                if let Some(ref mut popup) = self.popups.last_mut() {
                    let result = popup.update(action.clone());
                    if result.is_consumed() {
                        return result;
                    }
                }

                if matches!(action, Action::PageViewer(_)) {
                    return self.page.update(action);
                }

                if matches!(action, Action::Search(_)) {
                    return self.search.update(action);
                }

                let result = match self.context {
                    CONTEXT_SEARCH => self.search.update(action.clone()),
                    CONTEXT_PAGE => self.page.update(action.clone()),
                    _ => {
                        warn!("unknown context");
                        return ActionResult::Ignored;
                    }
                };
                if result.is_consumed() {
                    return result;
                }
            }
        };

        ActionResult::consumed()
    }

    fn render(&mut self, f: &mut Frame<'_>, mut area: Rect) {
        f.render_widget(
            Block::default().style(Style::default().bg(self.theme.bg)),
            area,
        );

        // don't render the search bar when we're in zen-mode and the config doesn't include the
        // search bar in the zen-mode settings
        match self.page.current_page() {
            // always render the searchbar if its focussed
            Some(_) if self.search_bar.is_focussed => area = self.render_search_bar(f, area),
            Some(page) if self.context == CONTEXT_PAGE => {
                if !page.is_zen_mode()
                    || self
                        .config
                        .page
                        .zen_mode
                        .contains(ZenModeComponents::SEARCH_BAR)
                {
                    area = self.render_search_bar(f, area);
                }
            }
            _ => area = self.render_search_bar(f, area),
        }

        if self.is_logger {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);
            self.logger.render(f, chunks[1]);
            area = chunks[0];
        };

        match self.context {
            CONTEXT_SEARCH => self.search.render(f, area),
            CONTEXT_PAGE => self.page.render(f, area),
            _ => warn!("unknown context"),
        }

        if let Some(ref mut popup) = self.popups.last_mut() {
            popup.render(f, area);
        }
    }
}
