use ratatui::widgets::ListState;

#[derive(Default)]
pub enum ScrollBehaviour {
    StickToTop,
    StickToBottom,
    StickToEnds,
    #[default]
    Circle,
}

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
    behvaiour: ScrollBehaviour,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        StatefulList {
            state: ListState::default().with_selected(Some(0)),
            items,
            behvaiour: ScrollBehaviour::default(),
        }
    }

    pub fn scroll_behavior(mut self, behavior: ScrollBehaviour) -> Self {
        self.behvaiour = behavior;
        self
    }

    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn get_items_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }

    pub fn get_state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len().saturating_sub(1) {
                    match self.behvaiour {
                        ScrollBehaviour::StickToBottom | ScrollBehaviour::StickToEnds => i,
                        ScrollBehaviour::StickToTop | ScrollBehaviour::Circle => 0,
                    }
                } else {
                    i.saturating_add(1)
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    match self.behvaiour {
                        ScrollBehaviour::StickToBottom | ScrollBehaviour::Circle => {
                            self.items.len().saturating_sub(1)
                        }
                        ScrollBehaviour::StickToTop | ScrollBehaviour::StickToEnds => i,
                    }
                } else {
                    i.saturating_sub(1)
                }
            }
            None => 0,
        };
        self.state.select(Some(i))
    }

    pub fn unselect(&mut self) {
        self.state.select(None)
    }

    pub fn is_selected(&self) -> bool {
        self.state.selected().is_some()
    }

    pub fn selected(&self) -> Option<&T> {
        self.state.selected().map(|i| &self.items[i])
    }
}
