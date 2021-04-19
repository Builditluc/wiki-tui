#[allow(unused_imports)]
#[macro_use] extern crate log;
extern crate ini;
extern crate anyhow;

use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use cursive::theme::*;
use cursive::utils::*;
use cursive::event::*;
use cursive::view::{Scrollable, Resizable};
use anyhow::*;

pub mod tests;
pub mod logging;
pub mod wiki;
pub mod structs;
pub mod config;

fn main() {
    let mut config: config::Config = config::Config::new();
    let wiki = wiki::Wiki::new(config.get_api_config());
    logging::Logger::new(config.get_logging_config());

    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);

    siv.set_user_data(wiki);
    
    let search_bar = EditView::new()
        .on_submit(|s, q| { 
            on_search(s, q.to_string())
        })
        .with_name("search")
        .full_width();
    let search_layout = Dialog::around(LinearLayout::horizontal()
                                       .child(search_bar))
        .title("Search")
        .title_position(cursive::align::HAlign::Left);

    let article_view = TextView::new("Welcome to wiki-tui!")
        .with_name("article")
        .full_screen()
        .scrollable();

    siv.add_fullscreen_layer(Dialog::around(LinearLayout::vertical()
                                            .child(search_layout)
                                            .child(Dialog::around(article_view)))
                             .title("wiki-tui")
                             .button("Quit", Cursive::quit)
                             .full_screen());
    siv.run();
}

fn on_search(siv: &mut Cursive, search_query: String) {
    log::trace!("on_search was called");
    let wiki: &wiki::Wiki = siv.user_data().unwrap();

    if search_query.is_empty() {
        log::warn!("No Search Query, aborting Search");
        return;
    }

    log::trace!("The Search Query is {}", search_query);

    let search_response = wiki.search(&search_query);

    let mut results_view = SelectView::<structs::wiki::ArticleResultPreview>::new()
        .on_select(|s, item| {on_result_select(s, item)})
        .on_submit(|s, a| {on_article_submit(s, a)});

    let results_preview = TextView::new("")
        .h_align(cursive::align::HAlign::Left)
        .with_name("results_preview")
        .fixed_width(50);


    // convert the search results into Article Result Previews
    // and then add them to the results_view
    for search_result in search_response.query.search.clone() {
        let search_result = structs::wiki::ArticleResultPreview::from(search_result);
        results_view.add_item(search_result.title.to_string(), search_result);
    }

    let query = search_query.to_string();

    let search_info = TextView::new(format!("Found {} articles matching your search", &search_response.clone().query.search_info.total_hits));
    let continue_button = Button::new("Show more results...", move |s| {
        continue_search(s, query.clone(), &search_response.continue_code)
    }).with_name("continue_button");

    let results_layout = LinearLayout::horizontal()
        .child(Dialog::around(LinearLayout::vertical()
                              .child(results_view.with_name("results_view").scrollable())
                              .child(continue_button)))
        .child(Dialog::around(results_preview));

    siv.add_layer(Dialog::around(LinearLayout::vertical()
                                 .child(results_layout)
                                 .child(search_info))
                  .title(format!("Results for {}", search_query))
                  .dismiss_button("Back")
                  .button("Quit", Cursive::quit)
                  .max_height(20));
 }

fn on_result_select(siv: &mut Cursive, item: &structs::wiki::ArticleResultPreview) {
    let title = &item.title;
    let snippet = &item.snippet;

    // formatting the snippet for styled text
    let split_snippet: Vec<&str> = snippet.split(r#"<span class="searchmatch">"#).collect();

    let mut styled_snippet = markup::StyledString::new();
    styled_snippet.append_plain(format!("{}\n", title));

    for slice in split_snippet {
        if slice.contains("</span>") {
            let split_slice: Vec<&str> = slice.split("</span>").collect();
            
            styled_snippet.append(markup::StyledString::styled(split_slice[0], Color::Dark(BaseColor::Red)));
            styled_snippet.append_plain(split_slice[1]);
        } else {
            styled_snippet.append_plain(slice);
        }
    }

    styled_snippet.append_plain("...");
    siv.call_on_name("results_preview", |view: &mut TextView| {
        view.set_content(styled_snippet);
    });
}

fn on_article_submit(siv: &mut Cursive, article_preview: &structs::wiki::ArticleResultPreview) {
    // remoe the results layer and the paging callbacks
    siv.clear_global_callbacks(Key::Left);
    siv.clear_global_callbacks(Key::Right);

    siv.pop_layer();

    // get the article
    let wiki: &wiki::Wiki = siv.user_data().unwrap();
    let article_response = wiki.get_article(&article_preview.page_id);

    // convert the article into the right format
    let article = structs::wiki::Article::from(article_response); 

    siv.call_on_name("article", |view: &mut TextView| {
        view.set_content(article.content);
    });

    let result = siv.focus_name("article").context("Failed to focus the article view");
    match result {
        Ok(_) => log::info!("Successfully focussed the article view"),
        Err(error) => log::warn!("{:?}", error),
    }

}

fn continue_search(siv: &mut Cursive, search_query: String, continue_code: &structs::wiki::search::ContinueCode) {
    let wiki: &wiki::Wiki = siv.user_data().unwrap();
    let search_response = wiki.continue_search(&search_query, continue_code);

    let mut results_view = siv.find_name::<SelectView::<structs::wiki::ArticleResultPreview>>("results_view").unwrap();
    for search_result in search_response.query.search.clone() {
        let search_result = structs::wiki::ArticleResultPreview::from(search_result);
        results_view.add_item(search_result.title.clone(), search_result);
    }

    let mut continue_button = siv.find_name::<Button>("continue_button").unwrap();
    continue_button.set_callback(move |s| {
        continue_search(s, search_query.clone(), &search_response.continue_code);
    });

    let result = siv.focus_name("results_view").context("Failed to focus the results view");
    match result {
        Ok(_) => log::info!("Successfully focussed the results view"),
        Err(error) => log::warn!("{:?}", error),
    }
}
