#[macro_use] extern crate log;
extern crate ini;

use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use cursive::view::{Scrollable, Resizable};

use html2text::from_read;

pub mod tests;
pub mod logging;
pub mod wiki;
pub mod structs;

fn main() {
    logging::Logger::new();

    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);

    let search_bar = EditView::new()
        .on_submit(on_bar_search)
        .with_name("search")
        .full_width();
    let search_button = Button::new("Search", |s| {on_search(s)});
    let search_layout = Dialog::around(LinearLayout::horizontal()
                                       .child(search_bar)
                                       .child(search_button));

    let search_results = SelectView::<structs::wiki::ArticleResultPreview>::new()
        .on_submit(|s, a| {on_article_submit(s, a)})
        .with_name("results");
    let search_results = search_results.full_screen();

    siv.add_fullscreen_layer(Dialog::around(LinearLayout::vertical()
                                            .child(search_layout)
                                            .child(Dialog::around(search_results)))
                             .title("wiki-tui")
                             .button("Quit", Cursive::quit)
                             .full_screen());
    siv.run();
}

fn on_bar_search(siv: &mut Cursive, text: &str) {
    on_search(siv);
}

fn on_search(siv: &mut Cursive) {
    log::trace!("on_search was called");

    let search_query = siv.call_on_name("search", |view: &mut EditView| {
        view.get_content()
    }).unwrap();

    if search_query.is_empty() {
        log::warn!("No Search Query, aborting Search");
        return;
    }

    log::trace!("The Search Query is {}", search_query);

    let wiki = wiki::Wiki::new();
    let search_response = wiki.search(&search_query);
    let mut search_results: Vec<structs::wiki::ArticleResultPreview> = Vec::new();

    // convert the search results into Article Result Previews
    for search_result in search_response.query.search.into_iter() {
        search_results.push(structs::wiki::ArticleResultPreview::from(search_result));
    }

    siv.call_on_name("results", |view: &mut SelectView::<structs::wiki::ArticleResultPreview>| {
        view.clear();
        for search_result in search_results.into_iter() {
            view.add_item(search_result.title.to_string(), search_result);
        }
    });
}

fn on_article_submit(siv: &mut Cursive, article_preview: &structs::wiki::ArticleResultPreview) {
    // get the article
    let wiki = wiki::Wiki::new();
    let article_response = wiki.get_article(&article_preview.page_id);

    // convert the article into the right format
    let mut article = structs::wiki::Article::from(article_response); 

    let article_view = TextView::new(article.content)
        .full_width()
        .scrollable();

    siv.add_fullscreen_layer(Dialog::around(article_view)
                             .title(article.title)
                             .dismiss_button("Back")
                             .button("Quit", Cursive::quit)
                             .full_screen());
}
