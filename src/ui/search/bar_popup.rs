use super::on_search;
use crate::ui::utils::percentage;
use cursive::{
    view::Resizable,
    views::{EditView, Panel},
    Cursive,
};

const PROMPT_WIDTH_PERCENTAGE: f32 = 0.6;

pub fn open_search_bar(siv: &mut Cursive) {
    siv.add_layer(
        Panel::new(
            EditView::new()
                .on_submit(|s, q| {
                    s.pop_layer();
                    on_search(s, q)
                })
                .full_width(),
        )
        .title("Search")
        .fixed_width(percentage(siv.screen_size().x, PROMPT_WIDTH_PERCENTAGE)),
    )
}
