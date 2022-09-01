pub mod article;
pub mod models;
mod root;
pub mod search;
mod theme_view;
pub mod toc;
pub mod utils;

pub type ThemedView<T> = theme_view::ThemedView<T>;
pub type RootLayout = root::RootLayout;
