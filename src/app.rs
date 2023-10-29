use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::{
    action::Action,
    components::{
        home::HomeComponent,
        logger::LoggerComponent,
        page::PageComponent,
        search::SearchComponent,
        status::{StatusComponent, STATUS_HEIGHT},
        Component,
    },
    event::EventHandler,
    terminal::{Frame, Tui},
    trace_dbg,
};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Context {
    #[default]
    Home,
    Search,
    Page,
}

#[derive(Default)]
pub struct AppComponent {
    home: HomeComponent,
    search: SearchComponent,
    page: PageComponent,
    logger: LoggerComponent,
    status: StatusComponent,

    is_logger: bool,
    is_input: bool,

    context: Context,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl AppComponent {
    fn enter_context(&mut self, context: Context) {
        self.context = context.clone();
        self.status.set_focus(context)
    }
}

impl Component for AppComponent {
    fn init(&mut self, action_tx: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.home.init(action_tx.clone())?;
        self.search.init(action_tx.clone())?;
        self.page.init(action_tx.clone())?;

        self.action_tx = Some(action_tx);
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
            // TEST: this is just for quickly opening a page
            // will be removed before release
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
        // HACK: handle global actions after the component handlet it
        match action {
            Action::ToggleShowLogger => self.is_logger = !self.is_logger,
            Action::EnterContext(ref context) => self.enter_context(context.to_owned()),
            Action::EnterInsert => self.is_input = true,
            Action::ExitInsert => self.is_input = false,
            _ => {}
        }

        // all other actions are passed on to the current component
        if let Some(action_cb) = match self.context {
            Context::Home => self.home.dispatch(action),
            Context::Search => self.search.dispatch(action),
            Context::Page => self.page.dispatch(action),
        } {
            return Some(action_cb);
        }

        None
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let (area, status_area) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100), Constraint::Min(STATUS_HEIGHT)])
                .split(area);
            (chunks[0], chunks[1])
        };

        self.status.render(f, status_area);

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
            Context::Home => self.home.render(f, area),
            Context::Search => self.search.render(f, area),
            Context::Page => self.page.render(f, area),
        }
    }
}

pub struct App {
    pub app_component: Arc<Mutex<AppComponent>>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            app_component: Arc::new(Mutex::new(AppComponent::default())),
            should_quit: false,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        self.app_component.lock().await.init(action_tx.clone())?;

        let mut tui = Tui::new()?;
        tui.enter()?;

        let _action_tx = action_tx.clone();
        let _root = self.app_component.clone();

        tokio::spawn(async move {
            let render_tick = 20;
            let mut event_handler = EventHandler::new(render_tick);
            loop {
                let event = event_handler.next().await;
                let action = _root.lock().await.handle_events(event);
                _action_tx.send(action).unwrap();
            }
        });

        loop {
            if let Some(action) = action_rx.recv().await {
                if !matches!(
                    action,
                    Action::RenderTick
                        | Action::Noop
                        | Action::ScrollDown(..)
                        | Action::ScrollUp(..)
                        | Action::UnselectScroll
                ) {
                    trace_dbg!(&action);
                }

                match action {
                    Action::RenderTick => {
                        let mut app_component = self.app_component.lock().await;
                        tui.terminal
                            .draw(|frame| app_component.render(frame, frame.size()))
                            .unwrap();
                    }
                    Action::Quit => self.should_quit = true,
                    action => {
                        if let Some(action_cb) = self.app_component.lock().await.dispatch(action) {
                            action_tx.send(action_cb).unwrap()
                        }
                    }
                }
            }

            if self.should_quit {
                break;
            }
        }

        tui.exit()?;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
