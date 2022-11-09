pub mod article;
pub mod models;
mod panel;
mod root;
pub mod search;
mod theme_view;
pub mod toc;
pub mod utils;

pub mod views {
    pub type ThemedView<T> = super::theme_view::ThemedView<T>;
    pub type RootLayout = super::root::RootLayout;
    pub type Panel<V> = super::panel::Panel<V>;
}
