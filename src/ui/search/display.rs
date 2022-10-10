use anyhow::Result;
use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, LinearLayout, Panel, SelectView, TextView},
    Cursive,
};

use crate::{
    config::CONFIG,
    ui::{article::on_article_submit, search::on_continue_submit, utils::percentage, RootLayout},
    wiki::search::{Search, SearchResult},
};

use super::on_result_select;

const SEARCH_WIDTH_PERCENTAGE: f32 = 0.7;
const SEARCH_HEIGHT_PERCENTAGE: f32 = 0.6;

const PREVIEW_HEIGHT_PERCENTAGE: f32 = 0.5;
const SEARCH_RESULTS_PERCENTAGE: f32 = 0.3;

/// Displays the search results and returns an error if anything went wrong
pub fn display_search_results(siv: &mut Cursive, search: Search, query: &str) -> Result<()> {
    info!("displaying '{}' search results", search.results().count());

    // calculate the necessary size values
    let screen_size = siv.screen_size();

    let search_width = percentage(screen_size.x, SEARCH_WIDTH_PERCENTAGE);
    let search_height = percentage(screen_size.y, SEARCH_HEIGHT_PERCENTAGE);

    let search_results_width = percentage(search_width, SEARCH_RESULTS_PERCENTAGE);
    let search_preview_height = percentage(search_height, PREVIEW_HEIGHT_PERCENTAGE);

    // create the results view (SelectView)
    let search_results_view = {
        let mut search_results_view = SelectView::<SearchResult>::new()
            .on_select(on_result_select)
            .on_submit(on_article_submit);

        // fill results view with results
        for search_result in search.results() {
            search_results_view
                .add_item(search_result.title().to_string(), search_result.to_owned());
        }
        search_results_view
    }
    .with_name("search_results_view")
    .full_height()
    .fixed_width(search_results_width)
    .scrollable();

    // create the continue button (Button)
    let search_continue_button = {
        let query = query.to_string();
        let offset = search.search_offset().to_owned();
        Button::new("Show more results...", move |s| {
            on_continue_submit(s, &query, &offset)
        })
        .with_name("search_continue_button")
    };

    // create the preview view (TextView)
    let search_result_preview = Panel::new(
        TextView::new("Test")
            .h_align(HAlign::Left)
            .with_name("search_result_preview"),
    )
    .fixed_height(search_preview_height);

    // create the info view (TextView)
    let search_result_info =
        Panel::new(TextView::empty().with_name("search_result_info")).full_height();

    // create the status view (TextView)
    let search_status_view = {
        let mut search_status_view = TextView::empty();

        // fill status view with the status
        if let Some(total_hits) = search.info().total_hits() {
            search_status_view.set_content(format!(
                "Found {} articles matching your search",
                total_hits
            ));
        }
        search_status_view
    };

    // pack results view and continue button in a layout
    let search_results_layout = Panel::new(
        LinearLayout::vertical()
            .child(search_results_view)
            .child(search_continue_button),
    );

    // pack preview view and info view in a layout
    let search_result_detail_layout = LinearLayout::vertical()
        .child(search_result_preview)
        .child(search_result_info);

    // pack these two layouts into a RootLayout
    let search_layout = RootLayout::horizontal(CONFIG.keybindings.clone())
        .child(search_results_layout)
        .child(search_result_detail_layout);

    // add the whole thing as a layer
    siv.add_layer(
        Panel::new(
            LinearLayout::vertical()
                .child(search_layout)
                .child(search_status_view),
        )
        .title(format!("Results for '{}'", query))
        .fixed_width(search_width)
        .fixed_height(search_height),
    );

    // send the callback to select the first search result
    if let Err(error) = siv.cb_sink().send(Box::new(move |s| {
        if let Some(search_result) = search.results().next() {
            on_result_select(s, &search_result);
        }
    })) {
        warn!("{:?}", error);
        bail!("couldn't send the callback to select the first search result");
    }

    Ok(())
}
