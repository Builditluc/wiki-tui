use anyhow::Error;
use cursive::{
    views::{Button, LinearLayout, TextView},
    Cursive,
};
use cursive_aligned_view::Alignable;

use crate::{config::CONFIG, ui::panel::WithPanel};

use super::root::RootLayout;

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
            ui::views::ThemedView::new(theme.to_theme(), $view)
        } else {
            ui::views::ThemedView::new(config::CONFIG.theme.to_theme(), $view)
        }
    };
}

/// Displays a given error
pub fn display_error(siv: &mut Cursive, error: Error) {
    const ERROR_MESSAGE: &str = "An error occurred during the search\nCheck the logs for more information\n\nError: {ERROR}";

    let title = "Warning";
    let body = ERROR_MESSAGE.replace("{ERROR}", &error.to_string());

    display_message(siv, title, &body);
}

/// Displays a popup message
pub fn display_message(siv: &mut Cursive, title: &str, body: &str) {
    siv.add_layer(
        LinearLayout::vertical()
            .child(TextView::new(body))
            .child(
                Button::new("Dismiss", |s| {
                    s.pop_layer();
                })
                .align_bottom_right(),
            )
            .with_panel()
            .title(title),
    );
}

/// Displays a basic yes/no dialog
pub fn display_dialog<F>(siv: &mut Cursive, title: &str, body: &str, cb: F)
where
    F: 'static + Fn(&mut Cursive),
{
    siv.add_layer(
        LinearLayout::vertical()
            .child(TextView::new(body))
            .child(
                RootLayout::horizontal(CONFIG.keybindings.clone())
                    .child(Button::new("No", |s| {
                        s.pop_layer();
                    }))
                    .child(Button::new("Yes", move |s| {
                        s.pop_layer();
                        cb(s)
                    })),
            )
            .with_panel()
            .title(title),
    );
}
