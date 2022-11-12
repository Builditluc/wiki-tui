use super::on_search;
use crate::{
    config::CONFIG,
    ui::{utils::percentage, views::Panel},
};
use cursive::{view::Resizable, views::EditView, Cursive};

const PROMPT_WIDTH_PERCENTAGE: f32 = 0.6;

pub fn open_search_bar(siv: &mut Cursive) {
    let search_bar = Panel::new(
        EditView::new()
            .on_submit(|s, q| {
                s.pop_layer();
                on_search(s, q)
            })
            .full_width(),
        CONFIG.theme.border,
    )
    .title("Search")
    .fixed_width(percentage(siv.screen_size().x, PROMPT_WIDTH_PERCENTAGE));

    siv.add_layer(search_bar)
}
