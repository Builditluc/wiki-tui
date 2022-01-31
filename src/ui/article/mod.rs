use crate::{config, ui, view_with_theme, wiki};

use anyhow::Context;
use cursive::align::HAlign;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

pub mod lines;
pub mod links;
pub mod view;
pub type ArticleView = view::ArticleView;

pub fn on_article_submit(siv: &mut Cursive, article_preview: &ui::models::ArticleResultPreview) {
    log::info!("Opening the article '{}'", article_preview.title);

    if siv
        .find_name::<TextView>("search_results_preview")
        .is_some()
    {
        // remove the results layer
        log::debug!("Removing the search layer");
        siv.pop_layer();
    } else {
        log::debug!("The search layer doesn't exist ");
    }

    // get the article from wikipedia

    // let article = match wiki.get_article(&article_preview.page_id) {
    //     Ok(article) => article,
    //     Err(error) => {
    //         // log an error_message
    //         log::error!("{:?}", error);
    //
    //         // display an error_message
    //         siv.add_layer(
    //             Dialog::info(
    //                 "A Problem occurred while fetching the article.\nCheck the logs for further information",
    //             )
    //             .title("Error")
    //             .title_position(HAlign::Center),
    //         );
    //         return;
    //     }
    // };
    //
    // // remove views
    // remove_view_from_article_layout(siv, "logo_view");
    // remove_view_from_article_layout(siv, "article_view");
    // remove_view_from_article_layout(siv, "toc_view");
    //
    // let mut article_view = ui::article::ArticleView::new().on_link_submit(on_link_submit);
    //
    // // set the contents of the article_view to the article
    // log::debug!("Setting the content of the article view");
    // article_view.set_article(article.clone().article);
    //
    // // add the article_view to the article_layout]
    // siv.call_on_name("article_layout", |view: &mut LinearLayout| {
    //     view.insert_child(
    //         0,
    //         view_with_theme!(
    //             config::CONFIG.theme.article_view,
    //             Dialog::around(article_view.with_name("article_view").scrollable())
    //         ),
    //     );
    // });
    // log::debug!("Added the article_view to the article_layout");
    //
    // // does this article have a table of contents?
    // if article.toc.is_some() {
    //     log::debug!("The article contains a table of contents");
    //     ui::toc::add_table_of_contents(siv, article.toc.unwrap());
    // } else {
    //     log::debug!("The article doesn't contain a table of contents");
    // }
    //
    // // focus the article view
    // let result = siv
    //     .focus_name("article_view")
    //     .context("Failed to focus the article view");
    //
    // match result {
    //     Ok(_) => log::debug!("Successfully focussed the article view"),
    //     Err(error) => log::warn!("{:?}", error),
    // }
}

fn on_link_submit(siv: &mut Cursive, target: &str) {
    let target = target.to_string();
    let target_title = {
        let target = target.strip_prefix("/wiki/").unwrap_or(&target);
        target.replace("_", " ")
    };

    siv.add_layer(
        Dialog::around(TextView::new(format!(
            "Do you want to open the article '{}'?",
            target_title
        )))
        .button("Yes", move |s| show_article_from_link(s, target.clone()))
        .button("No", |s| {
            s.pop_layer();
        }),
    )
}

fn show_article_from_link(siv: &mut Cursive, target: String) {
    // siv.pop_layer();
    //
    // // remove views
    // remove_view_from_article_layout(siv, "logo_view");
    // remove_view_from_article_layout(siv, "article_view");
    // remove_view_from_article_layout(siv, "toc_view");
    //
    // // get the article from wikipedia
    // let wiki: &wiki::WikiApi = siv.user_data().unwrap();
    // let article = match wiki.open_article(&target) {
    //     Ok(article) => article,
    //     Err(error) => {
    //         // log an error_message
    //         log::error!("url: {},\t{:?}", &target, error);
    //
    //         // display an error message
    //         siv.add_layer(
    //             Dialog::info(
    //                 "An error occurred while fetching the article.\nCheck the logs for further information"
    //             )
    //             .title("Error")
    //             .title_position(HAlign::Center),
    //         );
    //         return;
    //     }
    // };
    //
    // let mut article_view = ui::article::ArticleView::new().on_link_submit(on_link_submit);
    //
    // // set the contents of the article_view to the article
    // log::debug!("Setting the content of the article view");
    // article_view.set_article(article.clone().article);
    //
    // // add the article_view to the article_layout]
    // siv.call_on_name("article_layout", |view: &mut LinearLayout| {
    //     view.insert_child(
    //         0,
    //         view_with_theme!(
    //             config::CONFIG.theme.article_view,
    //             Dialog::around(article_view.with_name("article_view").scrollable())
    //         ),
    //     );
    // });
    // log::debug!("Added the article_view to the article_layout");
    //
    // // does this article have a table of contents?
    // if article.toc.is_some() {
    //     log::debug!("The article contains a table of contents");
    //     ui::toc::add_table_of_contents(siv, article.toc.unwrap());
    // } else {
    //     log::debug!("The article doesn't contain a table of contents");
    // }
    //
    // // focus the article view
    // let result = siv
    //     .focus_name("article_view")
    //     .context("Failed to focus the article view");
    //
    // match result {
    //     Ok(_) => log::info!("Successfully focussed the article view"),
    //     Err(error) => log::warn!("{:?}", error),
    // }
}
