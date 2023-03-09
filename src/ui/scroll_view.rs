use cursive::{direction::Direction, event::EventResult, view::CannotFocus, View};

use super::scroll;

pub struct ScrollView<V> {
    inner: V,
    core: scroll::Core,
}

impl_scroller!(ScrollView<V>::core);

impl<V> ScrollView<V> {
    /// Creates a new ScrollView around `view`
    pub fn new(inner: V) -> Self {
        ScrollView {
            inner,
            core: scroll::Core::new(),
        }
    }

    #[must_use]
    pub fn scroll_x(mut self, enabled: bool) -> Self {
        self.core.set_scroll_x(enabled);
        self
    }

    #[must_use]
    pub fn scroll_y(mut self, enabled: bool) -> Self {
        self.core.set_scroll_y(enabled);
        self
    }

    fn scroll_to_important_area(&mut self) -> EventResult
    where
        V: View,
    {
        self.scroll_operation(|s| {
            let important_area = s.inner.important_area(s.core.last_outer_size());
            s.core.scroll_to_rect(important_area)
        })
    }

    fn scroll_operation<F>(&mut self, f: F) -> EventResult
    where
        V: View,
        F: FnOnce(&mut Self),
    {
        self.refresh();
        f(self);
        EventResult::consumed()
    }

    fn refresh(&mut self)
    where
        V: View,
    {
        self.layout(self.core.last_outer_size());
    }
}

impl<V> View for ScrollView<V>
where
    V: View,
{
    fn draw(&self, printer: &cursive::Printer) {
        scroll::draw(self, printer, |s, p| s.inner.draw(p));
    }

    fn on_event(&mut self, event: cursive::event::Event) -> cursive::event::EventResult {
        match scroll::on_event(
            self,
            event,
            |s, e| s.inner.on_event(e),
            |_| {},
            |s, si| s.inner.important_area(si),
        ) {
            EventResult::Ignored => EventResult::Ignored,
            other => other,
        }
    }

    fn layout(&mut self, size: cursive::Vec2) {
        scroll::layout(
            self,
            size,
            self.inner.needs_relayout(),
            |s, si| s.inner.layout(si),
            |s, c| s.inner.required_size(c),
        );
    }

    fn needs_relayout(&self) -> bool {
        self.core.needs_relayout() || self.inner.needs_relayout()
    }

    fn required_size(&mut self, constraint: cursive::Vec2) -> cursive::Vec2 {
        scroll::required_size(self, constraint, self.inner.needs_relayout(), |s, c| {
            s.inner.required_size(c)
        })
    }

    fn call_on_any(&mut self, selector: &cursive::view::Selector, cb: cursive::event::AnyCb) {
        self.inner.call_on_any(selector, cb)
    }

    fn focus_view(
        &mut self,
        selector: &cursive::view::Selector,
    ) -> Result<EventResult, cursive::view::ViewNotFound> {
        self.inner.focus_view(selector).map(|res| {
            self.scroll_to_important_area();
            res
        })
    }

    fn take_focus(
        &mut self,
        source: cursive::direction::Direction,
    ) -> Result<EventResult, cursive::view::CannotFocus> {
        match self.inner.take_focus(source) {
            Ok(res) => {
                if source != Direction::none() {
                    self.scroll_to_important_area();
                }
                Ok(res)
            }
            Err(CannotFocus) => self
                .core
                .is_scrolling()
                .any()
                .then(EventResult::consumed)
                .ok_or(CannotFocus),
        }
    }

    fn important_area(&self, view_size: cursive::Vec2) -> cursive::Rect {
        scroll::important_area(self, view_size, |s, si| s.inner.important_area(si))
    }
}

pub trait Scrollable: View + Sized {
    fn scrollable(self) -> ScrollView<Self> {
        ScrollView::new(self)
    }
}

impl<T: View> Scrollable for T {}
