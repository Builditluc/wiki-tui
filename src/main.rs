#[macro_use] extern crate log;
extern crate ini;

use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;

pub mod tests;
pub mod logging;
pub mod wiki;
pub mod structs;

fn main() {
    logging::Logger::new();

    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);

    let search_bar = EditView::new()
        .with_name("search")
        .full_width();
    let search_button = Button::new("Search", |s| {on_search(s)});
    let search_layout = Dialog::around(LinearLayout::horizontal()
                                       .child(search_bar)
                                       .child(search_button));

    let mut search_results = SelectView::<structs::wiki::ArticleResultPreview>::new()
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

fn on_search(siv: &mut Cursive) {
    log::trace!("on_search was called");

    let search_query = siv.call_on_name("search", |view: &mut EditView| {
        view.get_content()
    }).unwrap();

    log::trace!("The Search Query is {}", search_query);

    let wiki = wiki::Wiki::new();
    let search_response = wiki.search(&search_query);

    log::trace!("got the wikipedia response");

    // until the convert function is properly implemented, these default response are being
    // displayed in the SelectView when searchnig
    let search_results = vec![
        structs::wiki::ArticleResultPreview {page_id: 0001, snippet: "This is a test".to_string(), title: "Title of an Article".to_string()}
    ];
    log::trace!("Adding the items to the Results View");
    siv.call_on_name("results", |view: &mut SelectView<structs::wiki::ArticleResultPreview>| {
        for search_result in search_results.into_iter() {
            log::trace!("Added {} to the Results View", search_result.title);
            //view.add_item("This is a Test", search_result);
        }
    });
}
