// This is mostly original panel code (https://docs.rs/cursive_core/0.3.5/src/cursive_core/views/panel.rs.html) but with a few things changed

use cursive::{
    align::HAlign,
    event::{Event, EventResult},
    theme::ColorStyle,
    view::ViewWrapper,
    Printer, Rect, Vec2, View,
};

use crate::config::{BorderStyle, CONFIG};

/// Draws a border around a wrapped view
pub struct Panel<V> {
    /// Inner View
    view: V,
    /// Possibly empty title
    title: String,
    /// Where to put the title position
    title_position: HAlign,
    /// Characters for the border
    border: Border,
    /// `true` when it needs relayout
    invalidated: bool,
}

/// Holds the characters required for the border
pub struct Border {
    upper_left: String,
    upper_right: String,

    lower_left: String,
    lower_right: String,

    horizontal: String,
    vertical: String,

    title_left: String,
    title_right: String,
}

impl From<BorderStyle> for Border {
    fn from(style: BorderStyle) -> Self {
        match style {
            BorderStyle::Light | BorderStyle::Default => Border {
                upper_left: "\u{250C}".to_string(),
                upper_right: "\u{2510}".to_string(),
                lower_left: "\u{2514}".to_string(),
                lower_right: "\u{2518}".to_string(),
                horizontal: "\u{2500}".to_string(),
                vertical: "\u{2502}".to_string(),
                title_left: "\u{2524}".to_string(),
                title_right: "\u{251C}".to_string(),
            },
            BorderStyle::Heavy => Border {
                upper_left: "\u{2554}".to_string(),
                upper_right: "\u{2557}".to_string(),
                lower_left: "\u{255A}".to_string(),
                lower_right: "\u{255D}".to_string(),
                horizontal: "\u{2550}".to_string(),
                vertical: "\u{2551}".to_string(),
                title_left: "\u{2563}".to_string(),
                title_right: "\u{2560}".to_string(),
            },
            BorderStyle::Round => Border {
                upper_left: "\u{256D}".to_string(),
                upper_right: "\u{256E}".to_string(),
                lower_left: "\u{2570}".to_string(),
                lower_right: "\u{256F}".to_string(),
                horizontal: "\u{2500}".to_string(),
                vertical: "\u{2502}".to_string(),
                title_left: "\u{2524}".to_string(),
                title_right: "\u{251C}".to_string(),
            },
        }
    }
}

/// Minimum distance between title and borders
const TITLE_SPACING: usize = 3;

impl<V> Panel<V> {
    /// Creates a new panel around the given view
    pub fn new<B: Into<Border>>(view: V, border: B) -> Self {
        Panel {
            view,
            title: String::new(),
            title_position: HAlign::Center,
            border: border.into(),
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
            printer.print((x - 2, 0), &format!("{} ", self.border.title_left));
            printer.print((x + len, 0), &format!(" {}", self.border.title_right));
        });
    }

    fn draw_border(&self, printer: &Printer) {
        let start: Vec2 = (0, 0).into();
        let size: Vec2 = printer.size;

        if size.x < 2 || size.y < 2 {
            return;
        }

        let size = size.saturating_sub((1, 1));

        printer.with_high_border(true, |s| {
            s.print(start, &self.border.upper_left);
            s.print(start + size.keep_y(), &self.border.lower_left);
            s.print_hline(start + (1, 0), size.x - 1, &self.border.horizontal);
            s.print_vline(start + (0, 1), size.y - 1, &self.border.vertical);
        });

        printer.with_low_border(true, |s| {
            s.print(start + size.keep_x(), &self.border.upper_right);
            s.print(start + size, &self.border.lower_right);
            s.print_hline(
                start + (1, 0) + size.keep_y(),
                size.x - 1,
                &self.border.horizontal,
            );
            s.print_vline(
                start + (0, 1) + size.keep_x(),
                size.y - 1,
                &self.border.vertical,
            );
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
        self.draw_border(printer);
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

pub trait WithPanel: View + Sized {
    fn with_panel(self) -> Panel<Self> {
        Panel::new(self, CONFIG.theme.border)
    }
}

impl<T: View> WithPanel for T {}
