use std::sync::Arc;

use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::prelude::Rect;
use tokio::sync::mpsc;

use crate::{
    action::{Action, ActionResult},
    config::{Config, Theme},
    event::Event,
    terminal::Frame,
};

pub mod help_popup;
pub mod logger;
pub mod message_popup;
pub mod page;
pub mod page_language_popup;
pub mod page_viewer;
pub mod search;
pub mod search_bar;
pub mod search_language_popup;

#[macro_export]
macro_rules! key_event {
    (Key::$key: ident, Modifier::$modifier: ident) => {
        crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::$key,
            crossterm::event::KeyModifiers::$modifier,
        )
    };
    (Key::$key: ident) => {
        key_event!(Key::$key, Modifier::NONE)
    };
    ($char: expr, Modifier::$modifier: ident) => {
        crossterm::event::KeyEvent::new(
            crossterm::event::KeyCode::Char($char),
            crossterm::event::KeyModifiers::$modifier,
        )
    };
    ($char: expr) => {
        key_event!($char, Modifier::NONE)
    };
}

pub trait Component {
    // TODO: use custom error type
    #[allow(unused_variables)]
    fn init(
        &mut self,
        action_tx: mpsc::UnboundedSender<Action>,
        config: Arc<Config>,
        theme: Arc<Theme>,
    ) -> Result<()> {
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
