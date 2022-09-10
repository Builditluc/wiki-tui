use cursive::{
    align::HAlign,
    view::Resizable,
    views::{DummyView, EditView, Panel, TextView},
    Cursive,
};
use cursive_aligned_view::Alignable;

use crate::{
    config::CONFIG,
    ui::{search::on_search, RootLayout},
    LOGO,
};

const PROMT_WIDTH_PERCENTAGE: f32 = 0.6;
const LOGO_PROMT_SPACING: usize = 5;

pub fn display_home() -> Box<dyn FnOnce(&mut Cursive) + Send> {
    Box::new(move |siv| {
        let logo_view = TextView::new(LOGO).h_align(HAlign::Center).full_width();
        let filler_view = DummyView::fixed_height(DummyView {}, LOGO_PROMT_SPACING);
        let search_promt = Panel::new(EditView::new().on_submit(on_search))
            .title("Search")
            .fixed_width((siv.screen_size().x as f32 * PROMT_WIDTH_PERCENTAGE) as usize)
            .align_center();

        siv.add_fullscreen_layer(
            Panel::new(
                RootLayout::vertical(CONFIG.keybindings.clone())
                    .child(logo_view)
                    .child(filler_view)
                    .child(search_promt)
                    .input(true)
                    .full_screen(),
            )
            .title("wiki-tui"),
        )
    })
}
