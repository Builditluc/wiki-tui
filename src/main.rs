#[macro_use] extern crate log;
extern crate ini;

use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;

pub mod tests;
pub mod logging;
pub mod wiki;
pub mod structs;

struct ArticlePreview {
    title: String
}

impl ArticlePreview {
    pub fn new(title: String) -> Self {
        ArticlePreview { title  }
    }
}

fn main() {
    logging::Logger::new();

    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);

    let search_bar = EditView::new().full_width();
    let search_button = Button::new("Search", |s| {on_search(s)});
    let search_layout = Dialog::around(LinearLayout::horizontal()
                                       .child(search_bar)
                                       .child(search_button));

    let mut search_results = SelectView::<ArticlePreview>::new();
    search_results.add_all(vec![("Article 1", ArticlePreview::new("Article 1".to_string())), ("Article 2", ArticlePreview::new("Article 2".to_string())), ("Article 3", ArticlePreview::new("Article 3".to_string()))]);
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

}
