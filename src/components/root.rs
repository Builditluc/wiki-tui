use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use tokio::sync::mpsc;

use crate::components::home::Home;
use crate::{action::Action, terminal::Frame};

use super::{logger::Logger, page::PageComponent, search::Search, Component};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Context {
    #[default]
    Home,
    Search,
    Page,
}

#[derive(Default)]
pub struct Root {
    home: Home,
    search: Search,
    page: PageComponent,
    logger: Logger,

    show_logger: bool,
    is_input: bool,
    context: Context,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl Root {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Root {
    fn init(&mut self, sender: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.home.init(sender.clone())?;
        self.search.init(sender.clone())?;
        self.page.init(sender.clone())?;
        self.action_tx = Some(sender);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            // HACK: handle global events after the component handled it
            // When we are in the input mode, we don't want to handle the global events
            KeyCode::Char('l') if !self.is_input => Action::ToggleShowLogger,
            KeyCode::Char('q') if !self.is_input => Action::Quit,
            KeyCode::Char('j') if !self.is_input => Action::ScrollDown(1),
            KeyCode::Char('k') if !self.is_input => Action::ScrollUp(1),
            KeyCode::Char('h') if !self.is_input => Action::UnselectScroll,
            KeyCode::Char('p') => {
                let action_tx = self.action_tx.as_ref().unwrap();
                action_tx.send(Action::EnterContext(Context::Page)).unwrap();
                action_tx
                    .send(Action::OpenPage("Linux".to_string()))
                    .unwrap();
                Action::Noop
            }
            _ => match self.context {
                Context::Home => self.home.handle_key_events(key),
                Context::Search => self.search.handle_key_events(key),
                Context::Page => self.page.handle_key_events(key),
            },
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        // global actions
        if let Some(_action) = match action {
            Action::ToggleShowLogger => {
                self.show_logger = !self.show_logger;
                None
            }
            Action::EnterContext(ref context) => {
                self.context = context.to_owned();
                None
            }
            Action::EnterInsert => {
                self.is_input = true;
                None
            }
            Action::ExitInsert => {
                self.is_input = false;
                None
            }
            _ => None,
        } {
            return Some(_action);
        }

        // all other actions are passed on to the current component
        if let Some(_action) = match self.context {
            Context::Home => self.home.dispatch(action),
            Context::Search => self.search.dispatch(action),
            Context::Page => self.page.dispatch(action),
        } {
            return Some(_action);
        }

        None
    }

    fn render(&mut self, frame: &mut Frame<'_>, size: Rect) {
        let size = if self.show_logger {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);
            self.logger.render(frame, chunks[1]);
            chunks[0]
        } else {
            size
        };

        match self.context {
            Context::Home => self.home.render(frame, size),
            Context::Search => self.search.render(frame, size),
            Context::Page => self.page.render(frame, size),
        }
    }
}
