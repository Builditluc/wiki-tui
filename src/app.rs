use anyhow::Result;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::{
    action::Action,
    components::{root::Root, Component},
    event::EventHandler,
    terminal::Tui,
    trace_dbg,
};

pub struct App {
    pub root: Arc<Mutex<Root>>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            root: Arc::new(Mutex::new(Root::default())),
            should_quit: false,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        self.root.lock().await.init(action_tx.clone())?;

        let mut tui = Tui::new()?;
        tui.enter()?;

        let _action_tx = action_tx.clone();
        let _root = self.root.clone();

        tokio::spawn(async move {
            let mut event_handler = EventHandler::new(20);
            loop {
                let event = event_handler.next().await;
                let action = _root.lock().await.handle_events(event);
                _action_tx.send(action).unwrap();
            }
        });

        loop {
            if let Some(action) = action_rx.recv().await {
                if action != Action::RenderTick && action != Action::Noop {
                    trace_dbg!(&action);
                }

                match action {
                    Action::RenderTick => {
                        let mut root = self.root.lock().await;
                        tui.terminal
                            .draw(|frame| root.render(frame, frame.size()))
                            .unwrap();
                    }
                    Action::Quit => self.should_quit = true,
                    action => {
                        if let Some(_action) = self.root.lock().await.dispatch(action) {
                            action_tx.send(_action).unwrap()
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
