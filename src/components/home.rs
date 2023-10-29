use crate::action::Action;
use crate::components::Component;
use crate::terminal::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Paragraph, Wrap};
use tokio::sync::mpsc::UnboundedSender;

const INFO_TEXT: &str = r#"Welcome to wiki-tui. This version is a part of the rewrite of wiki-tui.
Since this conversion is ongoing, there are going to be bugs and missing features. To get going, here are a few tipps:

Key Events and Keybindings:
===========================

KeyEvents are ordered in the following order:
1. if the search bar is focussed, forward the key event to it
2. forward the event to the currently focussed context (can be seen in the status bar)
3. if the event was ignored, handle the global keybindings

Below is a list of the keybindings:
Global keybindings:
- Char('l') => Action::ToggleShowLogger
- Char('q') => Action::Quit
- Char('s') => Action::EnterContext(Context::Search)
- Char('h') if KeyModifiers::CONTROL => Action::EnterContext(Context::Home)
- Char('j') => Action::ScrollDown(1)
- Char('k') => Action::ScrollUp(1)
- Char('h') => Action::UnselectScroll
- Char('i') => Action::EnterSearchBar
- Char('p') => PageAction::OpenPage("Linux") // just for testing purposes

Search focus keybindings:
- Enter if search_results.is_selected() => open_selected_result

Page focus keybindings:
- Char('q') if KeyModifiers::CONTROL => PageAction::SwitchRenderer // switches through the
                                                                   // renderers

SearchBar keybindings:
- Enter => disable searchbar focus & SearchAction::StartSearch
- Esc => Action::ExitSearchBar
"#;

#[derive(Default)]
pub struct HomeComponent {
    scroll: usize,
    action_tx: Option<UnboundedSender<Action>>,
}

impl HomeComponent {
    fn scroll_up(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_sub(amount);
    }

    fn scroll_down(&mut self, amount: usize) {
        self.scroll = self.scroll.saturating_add(amount);
    }
}

impl Component for HomeComponent {
    fn init(&mut self, sender: UnboundedSender<Action>) -> anyhow::Result<()> {
        self.action_tx = Some(sender);
        Ok(())
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
