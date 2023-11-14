use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::prelude::Rect;
use tokio::sync::mpsc;

use crate::{action::Action, event::Event, terminal::Frame};

pub mod home;
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
    fn handle_events(&mut self, event: Option<Event>) -> Action {
        match event {
            Some(Event::Quit) => Action::Quit,
            Some(Event::RenderTick) => Action::RenderTick,
            Some(Event::Key(key_event)) => self.handle_key_events(key_event),
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event),
            Some(Event::Resize(x, y)) => Action::Resize(x, y),
            None => Action::Noop,
        }
    }

    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        Action::Noop
    }

    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Action {
        Action::Noop
    }

    #[allow(unused_variables)]
    fn dispatch(&mut self, action: Action) -> Option<Action> {
        None
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect);
}
