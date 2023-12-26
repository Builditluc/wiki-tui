use tokio::sync::mpsc;
use wiki_api::{page::Page, search::Search};

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

    SwitchContextSearch,
    SwitchContextPage,

    // Scrolling
    ScrollUp(u16),
    ScrollDown(u16),
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
    LoadPage(String),

    Search(SearchAction),
    Page(PageAction),
    PageViewer(PageViewerAction),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchAction {
    StartSearch(String),
    FinshSearch(Search),
    ClearSearchResults,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageAction {
    SwitchRenderer(Renderer),

    SelectFirstLink,
    SelectLastLink,

    SelectTopLink,
    SelectBottomLink,

    SelectPrevLink,
    SelectNextLink,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageViewerAction {
    DisplayPage(Page),
    PopPage,
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

#[derive(Default)]
pub struct ActionPacket {
    actions: Vec<Action>,
}

impl ActionPacket {
    pub fn single(action: Action) -> Self {
        Self {
            actions: vec![action],
        }
    }

    pub fn append(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn send(self, action_tx: &mpsc::UnboundedSender<Action>) {
        for action in self.actions {
            action_tx.send(action).unwrap();
        }
    }
}
