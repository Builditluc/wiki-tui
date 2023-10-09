use crate::action::Action;
use crate::components::root::Context;
use crate::components::Component;
use crate::terminal::Frame;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Wrap};
use tokio::sync::mpsc::UnboundedSender;

const INFO_TEXT: &'static str = r#"Welcome to wiki-tui. This version is a part of the rewrite of wiki-tui in ratatui.
Since this conversion is ongoing, there are going to be bugs and missing features. To get going, here are a few tipps:

Keybinding help:
- Global keybindings (work except when in input mode):
    - 'l' Action::ToggleShowLogger
    - 'q' Action::Quit
    - 'j' Action::ScrollDown(1)
    - 'k' Action::ScrollUp(1)
    - 'h' Action::UnselectScroll
    - 'p' Shortcut for opening the article 'Linux'
- Context::Home keybindings:
    - 's' Action::EnterContext(Context::Search)
- Context::Search keybindings:
    Mode::Normal:
    - 'h' if KeyModifiers::CONTROL Action::EnterContext(Context::Home)
    - 'i' Action::EnterInsert
    - Enter if search_results.is_selected() -> Open Search Result

    Mode::Insert:
    - Esc Action::ExitInsert
    - Enter Action::StartSearch(input.value())
    - other: forward to input widget

    Mode::Processing:
    - no events (global keybindings still work)
- Context::Page keybindings:
    - 's' Action::EnterContext(Context::Search)
    - 'h' if KeyModifiers::CONTROL Action::EnterContext(Context::Home)
    - 'r' if KeyModifiers::CONTROL Action::SwitchRenderer(renderer.next())
"#;

#[derive(Default)]
pub struct Home {
    scroll: usize,
    action_tx: Option<UnboundedSender<Action>>,
}

impl Home {
    fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    fn scroll_down(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_add(amount);
    }
}

impl Component for Home {
    fn init(&mut self, sender: UnboundedSender<Action>) -> anyhow::Result<()> {
        self.action_tx = Some(sender);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('s') => Action::EnterContext(Context::Search),
            _ => Action::Noop,
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::ScrollUp(amount) => self.scroll_up(amount),
            Action::ScrollDown(amount) => self.scroll_down(amount),
            _ => {}
        };

        None
    }

    fn render(&mut self, frame: &mut Frame<'_>, size: Rect) {
        let info_widget = Paragraph::new(INFO_TEXT)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll as u16, 0));
        frame.render_widget(info_widget, size);
    }
}
