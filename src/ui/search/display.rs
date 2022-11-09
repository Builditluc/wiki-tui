use anyhow::{Context, Result};
use cursive::{
    align::HAlign,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, LinearLayout, SelectView, TextView},
    Cursive,
};

use crate::{
    config::CONFIG,
    ui::{
        article::on_article_submit,
        search::on_continue_submit,
        utils::percentage,
        views::{Panel, RootLayout},
    },
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

    debug!("screen_size: '{}' '{}'", screen_size.x, screen_size.y);

    let search_width = percentage(screen_size.x, SEARCH_WIDTH_PERCENTAGE);
    let search_height = percentage(screen_size.y, SEARCH_HEIGHT_PERCENTAGE);

    debug!("search_width: '{}'", search_width);
    debug!("search_height: '{}'", search_height);

    let search_results_width = percentage(search_width, SEARCH_RESULTS_PERCENTAGE);
    let search_preview_height = percentage(search_height, PREVIEW_HEIGHT_PERCENTAGE);

    debug!("search_results_width: '{}'", search_results_width);
    debug!("search_preview_height: '{}'", search_preview_height);

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
    .fixed_height(search_preview_height)
    .full_width();

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

    debug!("created the views for the search results layout");

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

    debug!("added the views to the search layout");

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

    debug!("added the layouts to a new layer");

    // send the callback to select the first search result
    if let Err(error) = siv.cb_sink().send(Box::new(move |s| {
        if let Some(search_result) = search.results().next() {
            on_result_select(s, search_result);
        }
    })) {
        warn!("{:?}", error);
        bail!("couldn't send the callback to select the first search result");
    }

    debug!("send the callback for selecting the first search result");

    Ok(())
}

/// Adds more search results to the already existing search panel
pub fn display_more_search_results(
    siv: &mut Cursive,
    search: Search,
    search_query: &str,
) -> Result<()> {
    info!(
        "displaying '{}' more search results",
        search.results().count()
    );

    // get the results view so we can add some results to it
    let mut search_results_views = siv
        .find_name::<SelectView<SearchResult>>("search_results_view")
        .context("couldn't find the search_results_view view")?;
    debug!("found the search_results_view");

    // get the continue button so we can change its callback
    let mut search_continue_button = siv
        .find_name::<Button>("search_continue_button")
        .context("couldn't find the search_continue_button view")?;
    debug!("found the search_continue_button");

    // add the new results to the view
    for search_result in search.results() {
        search_results_views.add_item(search_result.title(), search_result.clone())
    }
    debug!("added the results to the results view");

    // modify the callback of the continue button so we don't search for the same thing again
    {
        let query = search_query.to_string();
        search_continue_button
            .set_callback(move |s| on_continue_submit(s, &query, search.search_offset()));
    }
    debug!("set the new callback of the continue button");

    // focus the results view
    siv.focus_name("search_results_view")
        .context("failed to focus the search_results_view")?;
    debug!("focussed the search_results_view");

    Ok(())
}
