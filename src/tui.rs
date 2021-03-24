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

        let search_bar = EditView::new();
        let search_button = Button::new("Search", |cb| {});

        let search_layout = LinearLayout::horizontal()
            .child(search_bar)
            .child(search_button);

        siv.add_layer(Dialog::around(search_layout)
                      .title("wiki-tui"));

        siv.run();
    }
}
