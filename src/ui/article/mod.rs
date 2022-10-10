use crate::ui::utils::display_error;
use crate::wiki::{
    article::{parser::DefaultParser, Article, ArticleBuilder},
    search::SearchResult,
};
use crate::{
    config::CONFIG,
    ui::{self, RootLayout},
};

use anyhow::{Context, Result};
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{Dialog, Panel, TextView};
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
    info!(
        "on_link_submit was called with the target link '{}'",
        target
    );

    // convert the target into a human-friendly format
    let target_human = {
        let target = target.strip_prefix("/wiki/").unwrap_or(&target);
        target.replace('_', " ")
    };

    info!("requesting confirmation from the user");
    siv.add_layer(
        // create a dialog that asks the user for confirmation whether he really wants to open this
        // link
        RootLayout::new(Orientation::Vertical, CONFIG.keybindings.clone()).child(
            Diaaround(TextView::new(format!(
                "Do you want to open the article '{}'?",
                target_human
            )))
            .button("Yep", move |s| {
                info!("on_link_submit - user said yes :) continuing...");
                // the human wants us to open the link for him... we will comply...
                open_link(s, target.clone())
            })
            .button("Nope", move |s| {
                info!("on_link_submit - said no :/ aborting...");
                // so he doesn't want us to open the link... delete the whole dialog and pretend it
                // didn't happen
                s.pop_layer();
            }),
        ),
    );

    info!("on_link_submit finished successfully");
}

/// Helper function for fetching and displaying an article from a given link
fn open_link(siv: &mut Cursive, target: String) {
    debug!("open_link was called");

    // hide the confirmation dialog
    siv.pop_layer();

    // fetch the article
    debug!("fetching the article");
    let article = match ArticleBuilder::new(0, Some(target), &CONFIG.api_config.base_url)
        .build(&mut DefaultParser::new(&CONFIG.settings.toc))
    {
        Ok(article) => article,
        Err(error) => {
            warn!("{:?}", error);

            // display an error message
            siv.add_layer(
                Diainfo("A Problem occurred while fetching the article.\nCheck the logs for further information")
                    .title("Error")
                    .title_position(HAlign::Center)
            );

            debug!("open_link failed to finish");
            return;
        }
    };

    // display the article
    debug!("displaying the article");
    if let Err(error) = display_article(siv, article) {
        warn!("{:?}", error);

        // display an error message
        siv.add_layer(
            Diainfo("A Problem occurred while displaying the article.\nCheck the logs for further information")
                .title("Error")
                .title_position(HAlign::Center)
        );

        debug!("open_link failed to finish");
        return;
    }

    debug!("open_link finished successfully");
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
        debug!("removed the search_results_preview layer");
    }

    // create the article view
    let article_view = Panel::new(
        ArticleView::new(article.clone())
            .with_name("article_view")
            .scrollable(),
    )
    .title("wiki-tui");
    debug!("created the article view");

    let article_layout = RootLayout::horizontal(CONFIG.keybindings.clone())
        .child(article_view)
        .with_name("article_layout");
    debug!("created the article layout");

    siv.add_fullscreen_layer(article_layout);

    // display the toc if there is one
    if let Some(toc) = article.toc() {
        debug!("displaying the table of contents");
        ui::toc::add_table_of_contents(siv, toc, "article_layout");
    }

    // focus the article view
    siv.focus_name("article_view").with_context(|| {
        debug!("display_article failed to finish");
        "Failed to focus the article view"
    })?;

    debug!("display_article finished successfully");
    Ok(())
}
