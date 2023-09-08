use wiki_api::search::Search;

use crate::components::root::Context;

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

    // Search
    StartSearch(String),
    FinshSearch(Search),
}
