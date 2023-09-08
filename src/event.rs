use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use futures::FutureExt;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::warn;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Quit,
    RenderTick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    task: Option<JoinHandle<()>>,
    stop_cancellation_token: CancellationToken,
}

impl EventHandler {
    pub fn new(render_tick_rate: u64) -> Self {
        let render_tick_rate = std::time::Duration::from_millis(render_tick_rate);

        let (event_tx, event_rx) = mpsc::unbounded_channel();

        let stop_cancellation_token = CancellationToken::new();
        let _stop_cancellation_token = stop_cancellation_token.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut render_interval = tokio::time::interval(render_tick_rate);

            loop {
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    _ = _stop_cancellation_token.cancelled() => break,
                    maybe_event = crossterm_event => match maybe_event {
                        Some(Ok(evt)) => match evt {
                            CrosstermEvent::Key(key) => if key.kind == KeyEventKind::Press {
                                event_tx.send(Event::Key(key)).unwrap();
                            },
                            CrosstermEvent::Resize(x, y) => {
                                event_tx.send(Event::Resize(x, y)).unwrap();
                            },
                            _ => {}
                        }
                        Some(Err(error)) => {
                            warn!("crossterm event error: {error:?}");
                        }
                        None => {}
                    },
                    _ = render_delay => event_tx.send(Event::RenderTick).unwrap(),
                }
            }
        });

        Self {
            rx: event_rx,
            task: Some(task),
            stop_cancellation_token,
        }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }

    pub async fn stop(&mut self) {
        self.stop_cancellation_token.cancel();
        if let Some(handle) = self.task.take() {
            handle.await.unwrap();
        }
    }
}
