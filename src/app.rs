use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::{Constraint, Direction, Layout, Rect};
use std::sync::Arc;
use tracing::warn;
use wiki_api::{languages::Language, Endpoint};

use tokio::sync::{mpsc, Mutex};

use crate::{
    action::{Action, ActionPacket, ActionResult},
    components::{
        help::{HelpComponent, Keymap},
        logger::LoggerComponent,
        page_viewer::PageViewer,
        search::SearchComponent,
        search_bar::{SearchBarComponent, SEARCH_BAR_HEIGTH},
        status::{StatusComponent, STATUS_HEIGHT},
        Component,
    },
    event::EventHandler,
    key_event,
    page_loader::PageLoader,
    terminal::{Frame, Tui},
    trace_dbg,
    ui::centered_rect,
};

const CONTEXT_SEARCH: u8 = 0;
const CONTEXT_PAGE: u8 = 1;

#[derive(Default)]
pub struct AppComponent {
    search: SearchComponent,
    page: PageViewer,
    logger: LoggerComponent,
    status: StatusComponent,
    search_bar: SearchBarComponent,
    help: HelpComponent,

    page_loader: Option<PageLoader>,

    is_logger: bool,
    is_help: bool,

    context: u8,
    prev_context: u8,

    action_tx: Option<mpsc::UnboundedSender<Action>>,
}

impl AppComponent {
    fn switch_context(&mut self, context: u8) {
        self.prev_context = context;
        std::mem::swap(&mut self.prev_context, &mut self.context);
    }

    fn toggle_show_help(&mut self) {
        self.is_help = !self.is_help;

        if !self.is_help {
            return;
        }

        let mut keymap = self.keymap();
        keymap.append(&mut match self.context {
            CONTEXT_SEARCH => self.search.keymap(),
            CONTEXT_PAGE => self.page.keymap(),
            _ => return warn!("unknown context"),
        });
        self.help.set_keymap(keymap);
    }
}

impl Component for AppComponent {
    fn init(&mut self, action_tx: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.search.init(action_tx.clone())?;
        self.page.init(action_tx.clone())?;
        self.search_bar.init(action_tx.clone())?;

        self.page_loader = Some(PageLoader::new(
            Endpoint::parse("https://en.wikipedia.org/w/api.php").unwrap(),
            Language::default(),
            action_tx.clone(),
        ));

        action_tx.send(Action::EnterSearchBar).unwrap();
        self.action_tx = Some(action_tx);

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> ActionResult {
        if self.search_bar.is_focussed {
            return self.search_bar.handle_key_events(key);
        }

        let result = match self.context {
            CONTEXT_SEARCH => self.search.handle_key_events(key),
            CONTEXT_PAGE => self.page.handle_key_events(key),
            _ => {
                warn!("unknown context");
                return ActionResult::Ignored;
            }
        };

        if result.is_consumed() {
            return result;
        }

        match key.code {
            KeyCode::Char('l') => Action::ToggleShowLogger.into(),
            KeyCode::Char('?') => Action::ToggleShowHelp.into(),
            KeyCode::Char('q') => Action::Quit.into(),

            KeyCode::Char('s') => Action::SwitchContextSearch.into(),
            KeyCode::Char('p') => Action::SwitchContextPage.into(),

            KeyCode::Char('j') => Action::ScrollDown(1).into(),
            KeyCode::Char('k') => Action::ScrollUp(1).into(),
            KeyCode::Char('h') => Action::UnselectScroll.into(),
            KeyCode::Char('i') => Action::EnterSearchBar.into(),

            _ => ActionResult::Ignored,
        }
    }

    fn keymap(&self) -> Keymap {
        vec![
            (
                key_event!('l'),
                ActionPacket::single(Action::ToggleShowLogger),
            ),
            (
                key_event!('?'),
                ActionPacket::single(Action::ToggleShowHelp),
            ),
            (key_event!('q'), ActionPacket::single(Action::Quit)),
            (
                key_event!('s'),
                ActionPacket::single(Action::SwitchContextSearch),
            ),
            (
                key_event!('p'),
                ActionPacket::single(Action::SwitchContextPage),
            ),
            (key_event!('j'), ActionPacket::single(Action::ScrollDown(1))),
            (key_event!('k'), ActionPacket::single(Action::ScrollUp(1))),
            (
                key_event!('h'),
                ActionPacket::single(Action::UnselectScroll),
            ),
            (
                key_event!('i'),
                ActionPacket::single(Action::EnterSearchBar),
            ),
        ]
    }

    fn update(&mut self, action: Action) -> ActionResult {
        let result = if self.is_help {
            self.help.update(action.clone())
        } else {
            match self.context {
                CONTEXT_SEARCH => self.search.update(action.clone()),
                CONTEXT_PAGE => self.page.update(action.clone()),
                _ => {
                    warn!("unknown context");
                    return ActionResult::Ignored;
                }
            }
        };

        if result.is_consumed() {
            return result;
        }

        // global actions
        match action {
            Action::ToggleShowLogger => self.is_logger = !self.is_logger,
            Action::ToggleShowHelp => self.toggle_show_help(),

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

            Action::LoadPage(title) => self.page_loader.as_ref().unwrap().load_page(title),
            _ => return ActionResult::Ignored,
        };

        ActionResult::consumed()
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let (search_bar_area, area, status_area) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(SEARCH_BAR_HEIGTH),
                    Constraint::Percentage(100),
                    Constraint::Min(STATUS_HEIGHT),
                ])
                .split(area);
            (chunks[0], chunks[1], chunks[2])
        };

        self.status.render(f, status_area);

        if self.is_help {
            self.help.render(f, centered_rect(area, 30, 50));
            return;
        }

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

    pub async fn run(&mut self, actions: Option<ActionPacket>) -> Result<()> {
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
                if let ActionResult::Consumed(action) = _root.lock().await.handle_events(event) {
                    action.send(&_action_tx);
                }
            }
        });

        if let Some(actions) = actions {
            let _action_tx = action_tx.clone();
            tokio::spawn(async move {
                actions.send(&_action_tx);
            });
        }

        loop {
            if let Some(action) = action_rx.recv().await {
                if !matches!(action, Action::RenderTick) {
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
                    action => match self.app_component.lock().await.update(action) {
                        ActionResult::Consumed(action) => action.send(&action_tx),
                        ActionResult::Ignored => {}
                    },
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
