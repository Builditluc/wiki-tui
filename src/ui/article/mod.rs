use crate::wiki::article::parser::{DefaultParser, Parser};
use crate::wiki::article::{Article, ArticleBuilder};
use crate::wiki::search::SearchResult;
use crate::ui::utils::remove_view_from_layout;
use crate::{config::CONFIG, ui, view_with_theme, wiki};

use anyhow::{Result, Context};
use cursive::align::HAlign;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

// pub mod lines;
// pub mod links;
// pub mod view;
// pub type ArticleView = view::ArticleView;

/// Fetches an article from a given SearchResult and displays it. It's the on_submit callback for
/// the search results view
pub fn on_article_submit(siv: &mut Cursive, search_result: &SearchResult) {

}

/// Fetches an article from a given link and displays it. It's the on_submit callback for the
/// article view
pub fn on_link_submit(siv: &mut Cursive, target: String) {

}

/// Helper function for fetching and displaying an artile from a given link. Any errors it
/// encounters are returned
fn open_link(siv: &mut Cursive, target: String) -> Result<()> {
    // fetch the article
    let article = ArticleBuilder::new(0, Some(target)).build(&mut DefaultParser::new(&CONFIG.parser))?;

    // display the article
    display_article(siv, article)?;

    Ok(())
}

/// Helper function for displaying an article on the screen. This includes creating an article view
/// if neccessary and any errors it encountred are returned
fn display_article(siv: &mut Cursive, article: Article) -> Result<()> {
    // if the search layer still exists, then remove it
    if siv.find_name::<TextView>("search_results_preview").is_some() {
        siv.pop_layer();
    }

    // remove views
    remove_view_from_layout(siv, "logo_view", "article_layout");
    remove_view_from_layout(siv, "article_view", "article_layout");
    remove_view_from_layout(siv, "toc_view", "article_layout");

    // create the article view
    // let mut article_view = ArticleView::new().on_link_submit(on_link_submit);

    // add the article view to the screen
    Ok(())
}
