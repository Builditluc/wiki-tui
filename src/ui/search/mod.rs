use crate::{
    config::Config,
    ui::utils::display_error,
    wiki::search::{Search, SearchResult},
};

use anyhow::Context;
use cursive::Cursive;

use super::utils::{display_dialog, display_message};

pub mod bar_popup;
mod display;
mod select;

/// Callback that searches for a given query and adds the results to a new layer
/// Displays any error that occurred and aborts the search (does not crash)
pub fn on_search(siv: &mut Cursive, query: &str) {
    // search for the query
    let search = match Search::builder()
        .query(query)
        .url(Config::from_siv(siv).borrow().api_config.url())
        .search()
        .with_context(|| format!("failed to search for '{}'", query))
    {
        Ok(search) => search,
        Err(error) => {
            warn!("{:?}", error);
            display_error(siv, error);
            return;
        }
    };

    // if we've found no results, display the appropriate message
    if search.is_empty() && search.suggestion().is_none() {
        warn!("could'nt find any search results and no query suggestion was given");
        display_message(
            siv,
            "Warning",
            &format!("Couldn't find any results for '{}'", query),
        );
        return;
    }

    // if we've found no results, but have a suggestion for another query, display a dialog
    if search.is_empty() && search.suggestion().is_some() {
        info!(
            "no results are available, suggesting a new query, '{}'",
            search.suggestion().unwrap()
        );

        display_dialog(
            siv,
            "Information",
            &format!(
                "No results for '{}' were found. Do you want to search for '{}' instead?",
                query,
                search.suggestion().unwrap()
            ),
            move |siv| on_search(siv, search.suggestion().unwrap()),
        );
        return;
    }

    // display the found search results
    if let Err(error) = display::display_search_results(siv, search, query)
        .with_context(|| format!("failed to display the search results for '{}'", query))
    {
        warn!("{:?}", error);
        display_error(siv, error);
    }
}

/// Generates and displays a preview of a given search result. It's used as a callback for the
/// search results view
fn on_result_select(siv: &mut Cursive, item: &SearchResult) {
    info!(
        "selecting the item '{}', page id: '{}'",
        item.title(),
        item.pageid()
    );

    let layer_len = siv.screen_mut().len();

    let search_result_preview_name = format!("search_result_preview-{}", layer_len);
    let search_result_info_name = format!("search_result_info-{}", layer_len);

    // generate and display the preview of the search result
    if let Err(error) = select::generate_and_display_preview(siv, item, &search_result_preview_name)
        .context("failed generating and displaying the preview")
    {
        // only log the error and don't display it
        warn!("{:?}", error);
    }

    // generate and display the info of the search result
    if let Err(error) = select::generate_and_display_info(siv, item, &search_result_info_name)
        .context("failed generating and displaying the info")
    {
        // only log the error and don't display it
        warn!("{:?}", error);
    }
}

/// Searches for more results at a given offset and adds them to the results view. It's a callback
/// for the continue button and displays an error if something went wrong
fn on_continue_submit(siv: &mut Cursive, search_query: &str, search_offset: &usize) {
    // continue the search and fetch more results
    let search = match Search::builder()
        .query(search_query)
        .url(Config::from_siv(siv).borrow().api_config.url())
        .offset(*search_offset)
        .search()
        .with_context(|| format!("failed to fetch more search results for '{}'", search_query))
    {
        Ok(search) => search,
        Err(error) => {
            warn!("{:?}", error);
            display_error(siv, error);
            return;
        }
    };

    // display the search results
    if let Err(error) = display::display_more_search_results(siv, search, search_query)
        .with_context(|| {
            format!(
                "failed displaying more search results for '{}'",
                search_query
            )
        })
    {
        warn!("{:?}", error);
        display_error(siv, error);
    }
}
