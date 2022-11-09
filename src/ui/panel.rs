// This is mostly original panel code (https://docs.rs/cursive_core/0.3.5/src/cursive_core/views/panel.rs.html) but with a few things changed

use cursive::{
    align::HAlign,
    event::{Event, EventResult},
    theme::ColorStyle,
    view::ViewWrapper,
    Printer, Rect, Vec2, View,
};

/// Draws a border around a wrapped view
pub struct Panel<V> {
    /// Inner View
    view: V,
    /// Possibly empty title
    title: String,
    /// Where to put the title position
    title_position: HAlign,
    /// `true` when it needs relayout
    invalidated: bool,
}

/// Minimum distance between title and borders
const TITLE_SPACING: usize = 3;

impl<V> Panel<V> {
    /// Creates a new panel around the given view
    pub fn new(view: V) -> Self {
        Panel {
            view,
            title: String::new(),
            title_position: HAlign::Center,
            invalidated: true,
        }
    }

    /// Sets the title of the dialog
    /// If not empty, it will be visible at the top
    #[must_use]
    pub fn title<S: Into<String>>(mut self, label: S) -> Self {
        self.title = label.into();
        self.invalidated = true;
        self
    }

    fn draw_title(&self, printer: &Printer) {
        if self.title.is_empty() {
            return;
        }

        let available = match printer.size.x.checked_sub(2 * TITLE_SPACING) {
            Some(available) => available,
            None => return,
        };
        let len = std::cmp::min(self.title.chars().count(), available);
        let x = TITLE_SPACING + self.title_position.get_offset(len, available);

        printer
            .offset((x, 0))
            .cropped((len, 1))
            .with_color(ColorStyle::title_primary(), |p| {
                p.print((0, 0), &self.title)
            });
        printer.with_high_border(false, |printer| {
            printer.print((x - 2, 0), "┤ ");
            printer.print((x + len, 0), " ├");
        });
    }

    inner_getters!(self.view: V);
}

impl<V: View> ViewWrapper for Panel<V> {
    wrap_impl!(self.view: V);

    fn wrap_on_event(&mut self, event: Event) -> EventResult {
        self.view.on_event(event.relativized((1, 1)))
    }

    fn wrap_required_size(&mut self, req: Vec2) -> Vec2 {
        let req = req.saturating_sub((2, 2));
        let size = self.view.required_size(req).saturating_add((2, 2));
        if self.title.is_empty() {
            return size;
        }

        let title_width = self.title.chars().count() + 2 * TITLE_SPACING;
        size.or_max((title_width, 0))
    }

    fn wrap_draw(&self, printer: &Printer) {
        printer.print_box((0, 0), printer.size, true);
        self.draw_title(printer);

        let printer = printer.offset((1, 1)).shrinked((1, 1));
        self.view.draw(&printer);
    }

    fn wrap_layout(&mut self, size: Vec2) {
        self.view.layout(size.saturating_sub((2, 2)));
    }

    fn wrap_important_area(&self, size: Vec2) -> Rect {
        let inner_size = size.saturating_sub((2, 2));
        self.view.important_area(inner_size) + (1, 1)
    }

    fn wrap_needs_relayout(&self) -> bool {
        self.invalidated || self.view.needs_relayout()
    }
}
