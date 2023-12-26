use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use tokio::sync::mpsc;

use crate::{
    action::{Action, ActionResult},
    event::Event,
    terminal::Frame,
};

pub mod logger;
pub mod page;
pub mod page_viewer;
pub mod search;
pub mod search_bar;
pub mod status;

pub trait Component {
    // TODO: use custom error type
    #[allow(unused_variables)]
    fn init(&mut self, action_tx: mpsc::UnboundedSender<Action>) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn handle_events(&mut self, event: Option<Event>) -> ActionResult {
        match event {
            Some(Event::Quit) => Action::Quit.into(),
            Some(Event::RenderTick) => Action::RenderTick.into(),
            Some(Event::Key(key_event)) => self.handle_key_events(key_event),
            Some(Event::Resize(x, y)) => Action::Resize(x, y).into(),
            None => ActionResult::Ignored,
        }
    }

    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        ActionResult::Ignored
    }

    #[allow(unused_variables)]
    fn update(&mut self, action: Action) -> ActionResult {
        ActionResult::Ignored
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect);
}
