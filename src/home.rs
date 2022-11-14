use cursive::{
    align::HAlign,
    view::Resizable,
    views::{DummyView, EditView, TextView},
    Cursive,
};
use cursive_aligned_view::Alignable;

use crate::{
    config::CONFIG,
    ui::{panel::WithPanel, search::on_search, utils::percentage, views::RootLayout},
    LOGO,
};

const PROMT_WIDTH_PERCENTAGE: f32 = 0.6;
const LOGO_PROMT_SPACING: usize = 5;

pub fn display_home() -> Box<dyn FnOnce(&mut Cursive) + Send> {
    Box::new(move |siv| {
        let logo_view = TextView::new(LOGO).h_align(HAlign::Center).full_width();
        let spacer_view = DummyView::fixed_height(DummyView {}, LOGO_PROMT_SPACING);
        let search_promt = EditView::new()
            .on_submit(on_search)
            .with_panel()
            .title("Search")
            .fixed_width(percentage(siv.screen_size().x, PROMT_WIDTH_PERCENTAGE))
            .align_center();

        siv.add_fullscreen_layer(
            RootLayout::vertical(CONFIG.keybindings.clone())
                .child(logo_view)
                .child(spacer_view)
                .child(search_promt)
                .input(true)
                .full_screen()
                .with_panel()
                .title("wiki-tui"),
        )
    })
}
