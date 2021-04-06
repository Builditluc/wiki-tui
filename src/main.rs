#[macro_use] extern crate log;
extern crate ini;

use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use cursive::theme::*;
use cursive::utils::*;
use cursive::view::{Scrollable, Resizable};

pub mod tests;
pub mod logging;
pub mod wiki;
pub mod structs;

fn main() {
    logging::Logger::new();

    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);

    let search_bar = EditView::new()
        .on_submit(on_search)
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

fn on_search(siv: &mut Cursive, search_query: &str) {
    log::trace!("on_search was called");

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
    
    let mut results_view = SelectView::<structs::wiki::ArticleResultPreview>::new()
        .on_select(|s, item| {on_result_select(s, item)})
        .on_submit(|s, a| {on_article_submit(s, a)});

    let results_preview = TextView::new("Please select an Article")
        .h_align(cursive::align::HAlign::Left)
        .with_name("results_preview");

    for search_result in search_results.into_iter() {
        results_view.add_item(search_result.title.to_string(), search_result);
    }

    let results_layout = LinearLayout::horizontal()
        .child(results_view)
        .child(results_preview);

    siv.add_layer(Dialog::around(results_layout)
                  .title(format!("Results for {}", search_query))
                  .dismiss_button("Back")
                  .button("Quit", Cursive::quit));
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

    siv.call_on_name("results_preview", |view: &mut TextView| {
        view.set_content(styled_snippet);
    });
}

fn on_article_submit(siv: &mut Cursive, article_preview: &structs::wiki::ArticleResultPreview) {
    siv.pop_layer();

    // get the article
    let wiki = wiki::Wiki::new();
    let article_response = wiki.get_article(&article_preview.page_id);

    // convert the article into the right format
    let mut article = structs::wiki::Article::from(article_response); 

    siv.call_on_name("article", |view: &mut TextView| {
        view.set_content(article.content);
    });

    siv.focus_name("article");
}
