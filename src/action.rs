use std::fmt::Debug;

use tokio::sync::mpsc;
use wiki_api::{
    languages::Language,
    page::{LanguageLink, Link, Page},
    search::{Search, SearchResult},
    Endpoint,
};

use crate::components::page::Renderer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Resume,
    Suspend,
    RenderTick,
    Resize(u16, u16),

    // View Focus
    ToggleShowLogger,
    ShowPageLanguageSelection,
    ShowHelp,

    /// PopupMessage(Title, Content)
    PopupMessage(String, String),
    /// PopupError(Error)
    PopupError(String),
    /// PopupError(Title, Content, Callback)
    PopupDialog(String, String, Box<ActionPacket>),
    PopPopup,

    SwitchContextSearch,
    SwitchContextPage,
    SwitchPreviousContext,

    // Scrolling
    ScrollUp(u16),
    ScrollDown(u16),

    ScrollToTop,
    ScrollToBottom,

    ScrollHalfUp,
    ScrollHalfDown,

    UnselectScroll,

    // Mode
    EnterInsert,
    EnterNormal,
    EnterProcessing,

    // Search Bar
    EnterSearchBar,
    ClearSearchBar,
    SubmitSearchBar,
    ExitSearchBar,

    // Page loading
    LoadSearchResult(SearchResult),
    LoadLink(Link),
    LoadLangaugeLink(LanguageLink),
    /// Try to load a page, checking cache first
    TryLoadPage(String, Language, Endpoint),

    Search(SearchAction),
    Page(PageAction),
    PageViewer(PageViewerAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchAction {
    StartSearch(String),
    FinshSearch(Search),
    ContinueSearch,
    ClearSearchResults,
    OpenSearchResult,
    ChangeMode(crate::components::search::Mode),
    ChangeLanguage(Language),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageAction {
    SwitchRenderer(Renderer),
    ToggleContents,

    SelectFirstLink,
    SelectLastLink,

    SelectTopLink,
    SelectBottomLink,

    SelectPrevLink,
    SelectNextLink,

    GoToHeader(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageViewerAction {
    DisplayPage(Page),
    PopPage,
    ExitLoading,
    SaveCache,
}

pub enum ActionResult {
    Ignored,
    Consumed(ActionPacket),
}

impl ActionResult {
    pub fn consumed() -> Self {
        Self::Consumed(ActionPacket::default())
    }

    pub fn is_consumed(&self) -> bool {
        matches!(self, ActionResult::Consumed { .. })
    }
}

impl From<Action> for ActionResult {
    fn from(value: Action) -> Self {
        ActionResult::Consumed(ActionPacket::single(value))
    }
}

impl From<ActionPacket> for ActionResult {
    fn from(value: ActionPacket) -> Self {
        ActionResult::Consumed(value)
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ActionPacket {
    actions: Vec<Action>,
}

impl ActionPacket {
    pub fn single(action: Action) -> Self {
        Self {
            actions: vec![action],
        }
    }

    pub fn action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn send(self, action_tx: &mpsc::UnboundedSender<Action>) {
        for action in self.actions {
            action_tx.send(action).unwrap();
        }
    }
}

impl From<Action> for ActionPacket {
    fn from(value: Action) -> Self {
        ActionPacket::single(value)
    }
}

impl Debug for ActionPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.actions.len() == 1 {
            return write!(f, "{:?}", self.actions.first().unwrap());
        } else if self.actions.is_empty() {
            return write!(f, "Nothing");
        }

        f.debug_list().entries(self.actions.iter()).finish()
    }
}
