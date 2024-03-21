use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{mpsc, Mutex};
use wiki_tui::{
    action::{Action, ActionResult},
    app::AppComponent,
    cli::match_cli,
    components::Component,
    event::EventHandler,
    logging::initialize_logging,
    panic_handler::initialize_panic_handler,
    terminal::Tui,
    trace_dbg,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        r#"
IMPORTANT:
wiki-tui is going through a major rewrite (we're going async and switching backends, among other 
things). Please note that this is a DEVELOPMENT version and can / will include:
    - BUGS
    - MISSING FEATURES
    - other issues...
For more information about the rewrite, please refer to PR#226 (and its linked issues):
    https://github.com/Builditluc/wiki-tui/pull/226

Please feel free to try out this version, and report bugs / suggestions / etc.!
Thank you!
- Builditluc
    "#
    );

    let actions = match_cli();

    initialize_logging()?;
    initialize_panic_handler()?;

    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    let app_component = Arc::new(Mutex::new(AppComponent::default()));
    let mut should_quit = false;

    app_component.lock().await.init(action_tx.clone())?;

    let mut tui = Tui::new()?;
    tui.enter()?;

    let _action_tx = action_tx.clone();
    let _root = app_component.clone();

    // Event Thread
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

    // Send actions to be run at startup
    if let Some(actions) = actions {
        let _action_tx = action_tx.clone();
        tokio::spawn(async move {
            actions.send(&_action_tx);
        });
    }

    // Main Loop
    loop {
        if let Some(action) = action_rx.recv().await {
            if !matches!(action, Action::RenderTick) {
                trace_dbg!(&action);
            }

            match action {
                Action::RenderTick => {
                    let mut app_component = app_component.lock().await;
                    tui.terminal
                        .draw(|frame| app_component.render(frame, frame.size()))
                        .unwrap();
                }
                Action::Quit => should_quit = true,
                action => match app_component.lock().await.update(action) {
                    ActionResult::Consumed(action) => action.send(&action_tx),
                    ActionResult::Ignored => {}
                },
            }
        }

        if should_quit {
            break;
        }
    }

    tui.exit()?;
    Ok(())
}
