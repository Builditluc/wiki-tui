use crate::ui::panel::WithPanel;
use crate::ui::search::bar_popup::open_search_bar;
use crate::ui::utils::display_error;
use crate::wiki::{
    article::{parser::DefaultParser, Article, ArticleBuilder},
    search::SearchResult,
};
use crate::{
    config::CONFIG,
    ui::{self, views::RootLayout},
};

use anyhow::{Context, Result};
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{Dialog, OnEventView, TextView};
use cursive::Cursive;

mod content;
mod lines;
mod links;
mod view;
pub type ArticleView = view::ArticleView;

/// Fetches an article from a given SearchResult and displays it. It's the on_submit callback for
/// the search results view
pub fn on_article_submit(siv: &mut Cursive, search_result: &SearchResult) {
    let article = match fetch_article(*search_result.page_id(), None) {
        Ok(article) => article,
        Err(error) => {
            warn!("{:?}", error);
            display_error(siv, error);
            return;
        }
    };
    if let Err(error) = display_article(siv, article) {
        warn!("{:?}", error);
        display_error(siv, error)
    }
}

fn fetch_article(page_id: i32, target: Option<String>) -> Result<Article> {
    info!("fetching the article");
    ArticleBuilder::new(page_id, target, &CONFIG.api_config.base_url)
        .build(&mut DefaultParser::new(&CONFIG.settings.toc))
}

/// Fetches an article from a given link and displays it. It's the on_submit callback for the
/// article view
pub fn on_link_submit(siv: &mut Cursive, target: String) {
    // convert the target into a human-friendly format
    let target_human = {
        let target = target.strip_prefix("/wiki/").unwrap_or(&target);
        target.replace('_', " ")
    };

    info!("requesting confirmation to open the link '{}'", target);
    siv.add_layer(
        // create a dialog that asks the user for confirmation whether he really wants to open this
        // link
        RootLayout::new(Orientation::Vertical, CONFIG.keybindings.clone()).child(
            Dialog::around(TextView::new(format!(
                "Do you want to open the article '{}'?",
                target_human
            )))
            .button("Yep", move |s| {
                info!("opening the link '{}'", target);

                // hide the confirmation dialog
                s.pop_layer();
                debug!("removed the confirmation dialog");

                // open the link
                open_link(s, target.clone())
            })
            .button("Nope", move |s| {
                // so he doesn't want us to open the link... delete the whole dialog and pretend it
                // didn't happen
                s.pop_layer();
            }),
        ),
    );
}

/// Helper function for fetching and displaying an article from a given link
fn open_link(siv: &mut Cursive, target: String) {
    // fetch the article
    let article = match ArticleBuilder::new(0, Some(target), &CONFIG.api_config.base_url)
        .build(&mut DefaultParser::new(&CONFIG.settings.toc))
    {
        Ok(article) => article,
        Err(error) => {
            warn!("{:?}", error);

            // display an error message
            siv.add_layer(
                Dialog::info("A Problem occurred while fetching the article.\nCheck the logs for further information")
                    .title("Error")
                    .title_position(HAlign::Center)
            );
            return;
        }
    };
    debug!("fetched the article");

    // display the article
    if let Err(error) = display_article(siv, article) {
        warn!("{:?}", error);

        // display an error message
        siv.add_layer(
            Dialog::info("A Problem occurred while displaying the article.\nCheck the logs for further information")
                .title("Error")
                .title_position(HAlign::Center)
        );

        return;
    }
    debug!("displayed the article");
}

/// Helper function for displaying an article on the screen. This includes creating an article view
/// and any errors it encountred are returned
fn display_article(siv: &mut Cursive, article: Article) -> Result<()> {
    // if the search layer still exists, then remove it
    if siv
        .find_name::<TextView>("search_results_preview")
        .is_some()
    {
        siv.pop_layer();
        debug!("removed the last layer")
    }

    // get the amount of layers, this is used as an id for the new article layout so we don't have
    // multiple layouts with the same name

    let layer_len = siv.screen_mut().len() + 1;

    let article_layout_name = format!("article_layout-{}", layer_len);
    let article_view_name = format!("article_view-{}", layer_len);

    debug!("article_layout name '{}'", article_layout_name);
    debug!("artilce_view name '{}'", article_view_name);

    // create the article view
    let article_view = ArticleView::new(article.clone())
        .with_name(&article_view_name)
        .scrollable()
        .with_panel()
        .title("wiki-tui");
    debug!("created the article view");

    let article_layout = RootLayout::horizontal(CONFIG.keybindings.clone())
        .child(article_view)
        .with_name(&article_layout_name);
    debug!("created the article layout");

    siv.add_fullscreen_layer(OnEventView::new(article_layout).on_event('S', open_search_bar));
    debug!("created a new fullscreen layer and added the article layout to it");

    // display the toc if there is one
    if let Some(toc) = article.toc() {
        if let Err(error) = ui::toc::add_table_of_contents(siv, toc)
            .context("failed displaying the table of contents")
        {
            warn!("{:?}", error);
            display_error(siv, error);
        } else {
            debug!("displayed the table of contents");
        }
    }

    // focus the article view
    siv.focus_name(&article_view_name)
        .context("failed focussing the article view")?;

    debug!("focussed the article view");
    Ok(())
}
