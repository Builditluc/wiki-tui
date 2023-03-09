pub mod article;
pub mod models;
pub mod panel;
mod root;
mod scroll;
mod scroll_view;
pub mod search;
#[allow(dead_code)]
mod select_view;
mod theme_view;
pub mod toc;
pub mod utils;

pub mod views {
    pub type ThemedView<T> = super::theme_view::ThemedView<T>;
    pub type RootLayout = super::root::RootLayout;
    pub type SelectView<T> = super::select_view::SelectView<T>;
}
