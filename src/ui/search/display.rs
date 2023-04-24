use std::{cell::RefCell, rc::Rc};

use anyhow::{Context, Result};
use cursive::{
    align::HAlign,
    view::{Nameable, Resizable},
    views::{Button, LinearLayout, TextView},
    Cursive,
};

use crate::{
    config::{Config, CONFIG},
    ui::{
        article::on_article_submit,
        panel::WithPanel,
        scroll_view::Scrollable,
        search::on_continue_submit,
        utils::percentage,
        views::{RootLayout, SelectView},
    },
    wiki::search::{Search, SearchResult},
};

use super::on_result_select;

const SEARCH_WIDTH_PERCENTAGE: f32 = 0.7;
const SEARCH_HEIGHT_PERCENTAGE: f32 = 0.6;

const PREVIEW_HEIGHT_PERCENTAGE: f32 = 0.5;
const SEARCH_RESULTS_PERCENTAGE: f32 = 0.3;

/// Displays the search results and returns an error if anything went wrong
pub fn display_search_results(siv: &mut Cursive, mut search: Search, query: &str) -> Result<()> {
    info!("displaying '{}' search results", search.results().len());

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

    let layer_len = siv.screen_mut().len() + 1;

    let search_results_view_name = format!("search_results_view-{}", layer_len);
    let search_continue_button_name = format!("search_continue_button-{}", layer_len);
    let search_result_preview_name = format!("search_result_preview-{}", layer_len);
    let search_result_info_name = format!("search_result_info-{}", layer_len);

    debug!("search_results_view name '{}'", search_results_view_name);
    debug!(
        "search_continue_button name '{}'",
        search_continue_button_name
    );
    debug!(
        "search_result_preview name '{}'",
        search_result_preview_name
    );
    debug!("search_result_info name '{}'", search_result_info_name);

    // create the results view (SelectView)
    let search_results_view = {
        let mut search_results_view = SelectView::<SearchResult>::new()
            .on_select(on_result_select)
            .on_submit(|siv, x| on_article_submit(siv, x.pageid()));

        // fill results view with results
        for search_result in search.take_results() {
            search_results_view.add_item(search_result.title().to_string(), search_result);
        }
        search_results_view
    }
    .with_name(search_results_view_name)
    .full_height()
    .fixed_width(search_results_width)
    .scrollable();

    // create the continue button (Button)
    let search_continue_button = {
        let query = query.to_string();
        let offset = search.continue_offset().unwrap_or(0);
        Button::new("Show more results...", move |s| {
            on_continue_submit(s, &query, &offset)
        })
        .with_name(search_continue_button_name)
    };

    // create the preview view (TextView)
    let search_result_preview = TextView::new("")
        .h_align(HAlign::Left)
        .with_name(search_result_preview_name)
        .with_panel()
        .fixed_height(search_preview_height)
        .full_width();

    // create the info view (TextView)
    let search_result_info = TextView::empty()
        .with_name(search_result_info_name)
        .with_panel()
        .full_height();

    // create the status view (TextView)
    let language = siv
        .with_user_data(|config: &mut Rc<RefCell<Config>>| {
            config.borrow().api_config.language.clone()
        })
        .unwrap_or_default();

    let search_status_view = {
        let mut search_status_view = TextView::empty();

        // fill status view with the status
        if let Some(total_hits) = search.total_hits() {
            search_status_view.set_content(format!(
                "Found {} articles on the {} Wikipedia matching your search",
                total_hits,
                language.name()
            ));
        }
        search_status_view
    };

    debug!("created the views for the search results layout");

    // pack results view and continue button in a layout
    let search_results_layout = LinearLayout::vertical()
        .child(search_results_view)
        .child(search_continue_button)
        .with_panel();

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
        LinearLayout::vertical()
            .child(search_layout)
            .child(search_status_view)
            .with_panel()
            .title(format!("Results for '{}'", query))
            .fixed_width(search_width)
            .fixed_height(search_height),
    );

    debug!("added the layouts to a new layer");

    siv.on_event(CONFIG.keybindings.down.clone());
    siv.on_event(CONFIG.keybindings.up.clone());

    Ok(())
}

/// Adds more search results to the already existing search panel
pub fn display_more_search_results(
    siv: &mut Cursive,
    mut search: Search,
    search_query: &str,
) -> Result<()> {
    info!(
        "displaying '{}' more search results",
        search.results().len()
    );

    let layer_len = siv.screen_mut().len();

    let search_results_view_name = format!("search_results_view-{}", layer_len);
    let search_continue_button_name = format!("search_continue_button-{}", layer_len);

    // get the results view so we can add some results to it
    let mut search_results_views = siv
        .find_name::<SelectView<SearchResult>>(&search_results_view_name)
        .context(format!("couldn't find '{}'", search_results_view_name))?;
    debug!("found the search_results_view");

    // get the continue button so we can change its callback
    let mut search_continue_button = siv
        .find_name::<Button>(&search_continue_button_name)
        .context(format!("couldn't find '{}'", search_continue_button_name))?;
    debug!("found the search_continue_button");

    // add the new results to the view
    for search_result in search.take_results() {
        search_results_views.add_item(search_result.title().to_string(), search_result)
    }
    debug!("added the results to the results view");

    // modify the callback of the continue button so we don't search for the same thing again
    {
        let query = search_query.to_string();
        search_continue_button.set_callback(move |s| {
            on_continue_submit(s, &query, &search.continue_offset().unwrap_or(0))
        });
    }
    debug!("set the new callback of the continue button");

    // focus the results view
    siv.focus_name(&search_results_view_name)
        .context(format!("failed to focus '{}'", search_results_view_name))?;
    debug!("focussed the search_results_view");

    Ok(())
}
