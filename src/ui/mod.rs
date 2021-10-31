pub mod article;
pub mod models;

pub mod search;
pub mod toc;

mod theme_view;

pub type ThemedView<T> = theme_view::ThemedView<T>;

#[macro_export]
macro_rules! change_theme {
    ($theme:expr,$view:expr) => {
        if let Some(theme) = $theme.as_ref() {
            crate::ui::ThemedView::new(theme.to_theme(), $view)
        } else {
            crate::ui::ThemedView::new(config::CONFIG.theme.to_theme(), $view)
        }
    };
}
