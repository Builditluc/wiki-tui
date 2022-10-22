use crate::config::Keybindings;
use cursive::direction::Orientation;
use cursive::event::{Event, EventResult, Key};
use cursive::view::{IntoBoxedView, View, ViewWrapper};
use cursive::views::LinearLayout;
use cursive::Vec2;

pub struct RootLayout {
    layout: LinearLayout,
    keybindings: Keybindings,
    input_mode: bool,
}

impl RootLayout {
    pub fn new(orientation: Orientation, keybindings: Keybindings) -> Self {
        RootLayout {
            layout: LinearLayout::new(orientation),
            keybindings,
            input_mode: false,
        }
    }

    pub fn horizontal(keybindings: Keybindings) -> Self {
        RootLayout::new(Orientation::Horizontal, keybindings)
    }

    pub fn vertical(keybindings: Keybindings) -> Self {
        RootLayout::new(Orientation::Vertical, keybindings)
    }

    pub fn input(mut self, state: bool) -> Self {
        self.input_mode = state;
        self
    }

    pub fn child<V: IntoBoxedView + 'static>(mut self, view: V) -> Self {
        self.add_child(view);
        self
    }

    pub fn insert_child<V: IntoBoxedView + 'static>(&mut self, i: usize, view: V) {
        self.layout.insert_child(i, view);
    }

    pub fn add_child<V: IntoBoxedView + 'static>(&mut self, view: V) {
        self.layout.add_child(view);
    }
}

impl ViewWrapper for RootLayout {
    wrap_impl!(self.layout: LinearLayout);

    fn wrap_on_event(&mut self, ch: Event) -> EventResult {
        match ch {
            // input mode
            Event::Char(_) if self.input_mode => self.layout.on_event(ch),
            Event::Key(Key::Home) if self.input_mode => self.layout.on_event(ch),
            Event::Key(Key::End) if self.input_mode => self.layout.on_event(ch),
            Event::Key(Key::Left) if self.input_mode => self.layout.on_event(ch),
            Event::Key(Key::Right) if self.input_mode => self.layout.on_event(ch),
            Event::Key(Key::Backspace) if self.input_mode => self.layout.on_event(ch),
            Event::Key(Key::Del) if self.input_mode => self.layout.on_event(ch),
            Event::Key(Key::Enter) if self.input_mode => self.layout.on_event(ch),

            // movement
            key if key == self.keybindings.up => self.layout.on_event(Event::Key(Key::Up)),
            key if key == self.keybindings.down => self.layout.on_event(Event::Key(Key::Down)),
            key if key == self.keybindings.left => self.layout.on_event(Event::Key(Key::Left)),
            key if key == self.keybindings.right => self.layout.on_event(Event::Key(Key::Right)),

            // focus
            key if key == self.keybindings.focus_next => self.layout.on_event(Event::Key(Key::Tab)),
            key if key == self.keybindings.focus_prev => {
                self.layout.on_event(Event::Shift(Key::Tab))
            }

            _ => self.layout.on_event(ch),
        }
    }

    fn wrap_layout(&mut self, size: Vec2) {
        self.layout.layout(size);
    }
}
