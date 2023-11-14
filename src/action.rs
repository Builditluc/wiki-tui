use wiki_api::{page::Page, search::Search};

use crate::{app::Context, components::page::Renderer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Resume,
    Suspend,
    RenderTick,
    Resize(u16, u16),
    Noop,

    ToggleShowLogger,
    EnterContext(Context),

    // Scrolling
    ScrollUp(usize),
    ScrollDown(usize),
    UnselectScroll,

    // Mode
    EnterInsert,
    ExitInsert,

    EnterNormal,

    EnterProcessing,
    ExitProcessing,

    // Search Bar
    EnterSearchBar,
    ClearSearchBar,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageViewerAction {
    DisplayPage(Page),
}
