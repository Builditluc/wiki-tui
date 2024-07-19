use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use tracing::warn;
use wiki_api::{languages::Language, Endpoint};

use tokio::sync::mpsc;

use crate::{
    action::{Action, ActionPacket, ActionResult},
    components::{
        logger::LoggerComponent,
        message_popup::MessagePopupComponent,
        page_viewer::PageViewer,
        search::SearchComponent,
        search_bar::{SearchBarComponent, SEARCH_BAR_HEIGTH},
        search_language_popup::SearchLanguageSelectionComponent,
        Component,
    },
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
}

impl Component for AppComponent {
    fn init(&mut self, action_tx: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.search.init(action_tx.clone())?;
        self.page.init(action_tx.clone())?;
        self.search_bar.init(action_tx.clone())?;

        let endpoint = Endpoint::parse("https://en.wikipedia.org/w/api.php").unwrap();
        let language = Language::English;

        self.page_loader = Some(PageLoader::new(action_tx.clone()));

        self.search.endpoint = Some(endpoint);
        self.search.language = Some(language);

        action_tx.send(Action::EnterSearchBar).unwrap();
        self.action_tx = Some(action_tx);

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        // we need to always handle CTRL-C
        if matches!(key.code, KeyCode::Char('c') if has_modifier!(key, Modifier::CONTROL)) {
            return Action::Quit.into();
        }

        if self.search_bar.is_focussed {
            return self.search_bar.handle_key_events(key);
        }

        if let Some(ref mut popup) = self.popups.last_mut() {
            let result = popup.handle_key_events(key);
            if result.is_consumed() {
                return result;
            }
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

        match key.code {
            KeyCode::Char('q') => Action::Quit.into(),
            KeyCode::Esc => Action::PopPopup.into(),

            KeyCode::Char('l') => Action::ToggleShowLogger.into(),

            KeyCode::Char('s') => Action::SwitchContextSearch.into(),
            KeyCode::Char('p') => Action::SwitchContextPage.into(),

            KeyCode::Char('j') => Action::ScrollDown(1).into(),
            KeyCode::Char('k') => Action::ScrollUp(1).into(),

            KeyCode::Char('g') => Action::ScrollToTop.into(),
            KeyCode::Char('G') => Action::ScrollToBottom.into(),

            KeyCode::Char('d') if has_modifier!(key, Modifier::CONTROL) => {
                Action::ScrollHalfDown.into()
            }
            KeyCode::Char('u') if has_modifier!(key, Modifier::CONTROL) => {
                Action::ScrollHalfUp.into()
            }

            KeyCode::Char('h') => Action::UnselectScroll.into(),

            KeyCode::Char('i') => Action::EnterSearchBar.into(),

            KeyCode::F(2) => {
                self.popups
                    .push(Box::<SearchLanguageSelectionComponent>::default());
                ActionResult::consumed()
            }
            _ => ActionResult::Ignored,
        }
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

            Action::PopupMessage(title, content) => self
                .popups
                .push(Box::new(MessagePopupComponent::new(title, content))),
            _ => {
                if let Some(ref mut popup) = self.popups.last_mut() {
                    let result = popup.update(action.clone());
                    if result.is_consumed() {
                        return result;
                    }
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

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
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

        let area = if self.is_logger {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);
            self.logger.render(f, chunks[1]);
            chunks[0]
        } else {
            area
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
