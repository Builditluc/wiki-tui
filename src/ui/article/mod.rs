use crate::ui::utils::remove_view_from_layout;
use crate::wiki::{
    article::{
        parser::{DefaultParser, Parser},
        Article, ArticleBuilder,
    },
    search::SearchResult,
};
use crate::{config, ui, view_with_theme};

use anyhow::{bail, Context, Result};
use cursive::align::HAlign;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

mod content;
mod lines;
mod links;
mod view;
pub type ArticleView = view::ArticleView;

/// Fetches an article from a given SearchResult and displays it. It's the on_submit callback for
/// the search results view
pub fn on_article_submit(siv: &mut Cursive, search_result: &SearchResult) {
    log::info!("on_article_submit was called");

    // fetch the article
    log::info!(
        "fetching the article '{}' with the id '{}'",
        search_result.title(),
        search_result.page_id()
    );
    let article = match ArticleBuilder::new(*search_result.page_id(), None)
        .build(&mut DefaultParser::new(&config::CONFIG.parser))
    {
        Ok(article) => article,
        Err(error) => {
            // log the error
            log::warn!("{}", error);

            // display an error message
            siv.add_layer(
                Dialog::info("A Problem occurred while fetching the article.\nCheck the logs for further information")
                    .title("Error")
                    .title_position(HAlign::Center)
            );
            log::info!("on_article_submit failed to finish");
            return;
        }
    };

    // display the article
    log::info!(
        "displaying the article '{}'",
        if search_result.title().is_empty() {
            search_result.page_id().to_string()
        } else {
            search_result.title().to_string()
        }
    );
    if let Err(error) = display_article(siv, article.clone()) {
        // log the error
        log::warn!("{}", error);

        // display an error message
        siv.add_layer(
            Dialog::info("A Problem occurred while displaying the article.\nCheck the logs for further information")
                .title("Error")
                .title_position(HAlign::Center)
        );
        log::info!("on_article_submit failed to finish");
        return;
    }

    // display the toc if there is one
    if let Some(toc) = article.toc() {
        log::info!("displaying the table of contents");
        ui::toc::add_table_of_contents(siv, toc);
    }

    log::info!("on_article_submit finished successfully");
}

/// Fetches an article from a given link and displays it. It's the on_submit callback for the
/// article view
pub fn on_link_submit(siv: &mut Cursive, target: String) {
    log::info!(
        "on_link_submit was called with the target link '{}'",
        target
    );

    // convert the target into a human-friendly format
    let target_human = {
        let target = target.strip_prefix("/wiki/").unwrap_or(&target);
        target.replace('_', " ")
    };

    log::info!("requesting confirmation from the user");
    siv.add_layer(
        // create a dialog that asks the user for confirmation whether he really wants to open this
        // link
        Dialog::around(TextView::new(format!(
            "Do you want to open the article '{}'?",
            target_human
        )))
        .button("Yep", move |s| {
            log::info!("on_link_submit - user said yes :) continuing...");
            // the human wants us to open the link for him... we will comply...
            open_link(s, target.clone())
        })
        .button("Nope", move |s| {
            log::info!("on_link_submit - said no :/ aborting...");
            // so he doesn't want us to open the link... delete the whole dialog and pretend it
            // didn't happen
            s.pop_layer();
        }),
    );

    log::info!("on_link_submit finished successfully");
}

/// Helper function for fetching and displaying an article from a given link
fn open_link(siv: &mut Cursive, target: String) {
    log::debug!("open_link was called");

    // hide the confirmation dialog
    siv.pop_layer();

    // fetch the article
    log::debug!("fetching the article");
    let article = match ArticleBuilder::new(0, Some(target))
        .build(&mut DefaultParser::new(&config::CONFIG.parser))
    {
        Ok(article) => article,
        Err(error) => {
            log::warn!("{:?}", error);

            // display an error message
            siv.add_layer(
                Dialog::info("A Problem occurred while fetching the article.\nCheck the logs for further information")
                    .title("Error")
                    .title_position(HAlign::Center)
            );

            log::debug!("open_link failed to finish");
            return;
        }
    };

    // display the article
    log::debug!("displaying the article");
    if let Err(error) = display_article(siv, article.clone()) {
        log::warn!("{:?}", error);

        // display an error message
        siv.add_layer(
            Dialog::info("A Problem occurred while displaying the article.\nCheck the logs for further information")
                .title("Error")
                .title_position(HAlign::Center)
        );

        log::debug!("open_link failed to finish");
        return;
    }

    // display the toc if there is one
    if let Some(toc) = article.toc() {
        log::debug!("displaying the table of contents");
        ui::toc::add_table_of_contents(siv, toc);
    }

    log::debug!("open_link finished successfully");
}

/// Helper function for displaying an article on the screen. This includes creating an article view
/// and any errors it encountred are returned
fn display_article(siv: &mut Cursive, article: Article) -> Result<()> {
    log::debug!("display_article was called");

    // if the search layer still exists, then remove it
    if siv
        .find_name::<TextView>("search_results_preview")
        .is_some()
    {
        siv.pop_layer();
        log::debug!("removed the search_results_preview layer");
    }

    // remove views
    remove_view_from_layout(siv, "logo_view", "article_layout");
    remove_view_from_layout(siv, "article_view", "article_layout");
    remove_view_from_layout(siv, "toc_view", "article_layout");

    // create the article view
    let article_view = ArticleView::new(article);
    log::debug!("created an instance of ArticleView");

    // add the article view to the screen
    let result = siv.call_on_name("article_layout", |view: &mut LinearLayout| {
        view.insert_child(
            0,
            view_with_theme!(
                config::CONFIG.theme.article_view,
                Dialog::around(article_view.with_name("article_view").scrollable())
            ),
        );
    });
    if result.is_none() {
        log::debug!("display_article failed to finish");
        bail!("Couldn't find the article layout");
    }
    log::debug!("added the ArticleView to the screen");

    // focus the article view
    siv.focus_name("article_view").with_context(|| {
        log::debug!("display_article failed to finish");
        "Failed to focus the article view"
    })?;

    log::debug!("display_article finished successfully");
    Ok(())
}
