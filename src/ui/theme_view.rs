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
        printer
            .theme(&self.theme)
            .with_style(ColorStyle::primary(), |printer| self.view.draw(printer))
    }
}
