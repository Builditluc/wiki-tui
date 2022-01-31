use cursive::theme::{ColorStyle, Theme};
use cursive::view::{View, ViewWrapper};
use cursive::{inner_getters, wrap_impl};

pub struct ThemedView<T> {
    theme: Theme,
    view: T,
}

impl<T> ThemedView<T> {
    pub fn new(theme: Theme, view: T) -> Self {
        ThemedView { theme, view }
    }

    inner_getters!(self.view: T);
}

impl<T: View> ViewWrapper for ThemedView<T> {
    wrap_impl!(self.view: T);

    fn wrap_draw(&self, printer: &cursive::Printer) {
        printer.with_theme(&self.theme, |printer| {
            printer.with_color(ColorStyle::background(), |printer| {
                for y in 0..printer.size.y {
                    printer.print_hline((0, y), printer.size.x, " ");
                }
            });
            printer.with_style(ColorStyle::primary(), |printer| self.view.draw(printer));
        });
    }
}
