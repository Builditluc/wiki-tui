use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::{
    action::{Action, ActionPacket, ActionResult},
    ui::{ScrollBehaviour, StatefulList},
};

use super::Component;

const INFO_TEXT: &str = "Below are the keybindings for the current context";
const INFO_LIST_SPACING: u16 = 1;

pub type Keybinding = (KeyEvent, ActionPacket);
pub type Keymap = Vec<Keybinding>;

pub struct HelpComponent {
    keymap: StatefulList<Keybinding>,
}

impl HelpComponent {
    pub fn set_keymap(&mut self, keymap: Keymap) {
        self.keymap =
            StatefulList::with_items(keymap).scroll_behavior(ScrollBehaviour::StickToEnds);
    }
}

impl Component for HelpComponent {
    fn update(&mut self, action: Action) -> ActionResult {
        match action {
            Action::ScrollUp(n) => {
                for _ in 0..n {
                    self.keymap.previous();
                }
            }
            Action::ScrollDown(n) => {
                for _ in 0..n {
                    self.keymap.next();
                }
            }
            Action::UnselectScroll => {
                self.keymap.unselect();
            }
            _ => return ActionResult::Ignored,
        };
        ActionResult::consumed()
    }

    fn render(&mut self, f: &mut crate::terminal::Frame<'_>, area: Rect) {
        f.render_widget(Block::default().borders(Borders::ALL), area);
        let area = area.inner(&Margin::new(1, 1));

        let (info_area, keymap_area) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Min(INFO_LIST_SPACING),
                    Constraint::Percentage(100),
                ])
                .split(area);
            (chunks[0], chunks[2])
        };

        let info_widget = Paragraph::new(INFO_TEXT).alignment(Alignment::Center);
        f.render_widget(info_widget, info_area);

        let (actions_area, spacer_area, events_area) = {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Min(1),
                    Constraint::Percentage(50),
                ])
                .split(keymap_area);
            (chunks[0], chunks[1], chunks[2])
        };

        let spacer_widget = Block::default().borders(Borders::LEFT);
        f.render_widget(spacer_widget, spacer_area);

        let mut actions_items = Vec::new();
        let mut event_items = Vec::new();

        for (event, action) in self.keymap.get_items() {
            actions_items.push(ListItem::new(format!("{:?}", action)));
            event_items.push(ListItem::new(format!("{:?}", event.code)));
        }

        let actions_widget = List::new(actions_items)
            .block(Block::default().title("Action(s)"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            );
        let event_widget = List::new(event_items)
            .block(Block::default().title("Keybinding"))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            );

        f.render_stateful_widget(actions_widget, actions_area, self.keymap.get_state_mut());
        f.render_stateful_widget(event_widget, events_area, self.keymap.get_state_mut());
    }
}

impl Default for HelpComponent {
    fn default() -> Self {
        Self {
            keymap: StatefulList::with_items(Vec::new()),
        }
    }
}
