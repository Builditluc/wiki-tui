use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;

pub struct Tui;

impl Tui {
    pub fn new() -> Self {
        Tui{}
    }

    pub fn run(&self) {
        let mut siv = cursive::default();

        let search_bar = EditView::new()
            .full_width();
        let search_button = Button::new("Search", |cb| {});
        let search_layout = LinearLayout::horizontal()
            .child(search_bar)
            .child(search_button);

        let mut search_results = SelectView::new();
        search_results.add_item("Article 1", 1);
        search_results.add_item("Article 2", 2);
        let search_results = search_results.full_screen();

        siv.add_fullscreen_layer(Dialog::around(LinearLayout::vertical()
                                                .child(search_layout)
                                                .child(search_results))
                      .title("wiki-tui")
                      .button("Quit", Cursive::quit)
                      .full_screen());

        siv.run();
    }
}
