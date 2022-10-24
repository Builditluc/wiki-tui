use anyhow::Error;
use cursive::{views::Dialog, Cursive};

/// Returns the percentage of a given value
pub fn percentage(value: usize, percentage: f32) -> usize {
    (value as f32 * percentage) as usize
}

/// Wraps a view into a ThemedView with the given theme. If the macro is used without a theme,
/// it'll just apply the default one to the view
#[macro_export]
macro_rules! view_with_theme {
    ($theme: expr, $view: expr) => {
        if let Some(theme) = $theme.as_ref() {
            ui::ThemedView::new(theme.to_theme(), $view)
        } else {
            ui::ThemedView::new(config::CONFIG.theme.to_theme(), $view)
        }
    };
}

/// Displays a given error
pub fn display_error(siv: &mut Cursive, error: Error) {
    const ERROR_MESSAGE: &str = "An error occurred during the search\nCheck the logs for more information\n\nError: {ERROR}";

    siv.add_layer(
        Dialog::text(ERROR_MESSAGE.replace("{ERROR}", &error.to_string()))
            .title("Warning")
            .dismiss_button("Dismiss"),
    );
}
