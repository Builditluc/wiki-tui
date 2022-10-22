use crate::{
    config,
    ui::utils::display_error,
    wiki::search::{
        Search, SearchBuilder, SearchMetadata, SearchProperties, SearchResult, SearchSortOrder,
    },
};

use anyhow::{anyhow, Context, Result};
use chrono::DateTime;
use cursive::views::{Button, SelectView, TextView};
use cursive::{utils::markup::StyledString, Cursive};

mod display;

/// Returns the default SearchBuilder
fn build_search() -> SearchBuilder {
    SearchBuilder::new(&config::CONFIG.api_config.base_url)
        .info(SearchMetadata::new().total_hits())
        .prop(SearchProperties::new().snippet().wordcount().timestamp())
        .sort(SearchSortOrder::JustMatch)
}

/// Calback that searches for a given query and adds the results to a new layer
/// Displays any error that occurred and aborts the search (does not crash)
pub fn on_search(siv: &mut Cursive, query: &str) {
    // search for the query
    let search = match search(query).with_context(|| format!("failed to search for '{}'", query)) {
        Ok(search) => search,
        Err(error) => {
            warn!("{:?}", error);
            display_error(siv, error);
            return;
        }
    };

    // display the found search results
    if let Err(error) = display::display_search_results(siv, search, query)
        .with_context(|| format!("failed to display the search results for '{}'", query))
    {
        warn!("{:?}", error);
        display_error(siv, error);
        return;
    }
}

/// Searches for a given query and returns the results. Returns an error if something went wrong.
fn search(query: &str) -> Result<Search> {
    info!("searching for '{}'", query);
    // do the search and if something went wrong, return the error
    build_search().query(query.to_string()).search()
}

/// Generates and displays a preview of a given search result. It's used as a callback for the
/// search results view
fn on_result_select(siv: &mut Cursive, item: &SearchResult) {
    info!(
        "selecting the item '{}', page id: '{}'",
        item.title(),
        item.page_id()
    );

    let mut preview = StyledString::new();

    // add the title to the preview
    preview.append_plain(format!("{}\n", item.title()));

    // only go through this if we have a snippet
    if let Some(snippet) = item.snippet() {
        let splitted_snippet: Vec<&str> = snippet.split(r#"<span class="searchmatch">"#).collect();

        // go through every slice of the splitted_snippet and if it contains </span>,
        // split the slice again and make the first split red
        for slice in splitted_snippet {
            if slice.contains("</span>") {
                let split_slice: Vec<&str> = slice.split("</span>").collect();

                preview.append(StyledString::styled(
                    split_slice[0],
                    config::CONFIG.theme.search_match,
                ));
                preview.append_plain(split_slice[1]);
            } else {
                preview.append_plain(slice);
            }
        }
        preview.append_plain("...");
        debug!("added the snippet to the preview");
    }
    debug!("generated the preview");

    // generate the info text
    let mut info_text = String::new();

    info_text.push_str(&format!("Title: {}", item.title()));

    // add the wordcount to the info if available
    if let Some(wordcount) = item.wordcount() {
        info_text.push_str(&format!("\nWord count: {} words", wordcount.to_string()));
        debug!("added the wordcount to the info");
    }

    // add the formatted timestamp to the info if available
    if let Some(timestamp) = item.timestamp() {
        match DateTime::parse_from_rfc3339(timestamp) {
            Ok(formatted_time) => info_text.push_str(&format!(
                "\nLast Edited: {}",
                formatted_time.format("%H:%M:%S %d/%m/%Y ")
            )),
            Err(error) => warn!("failed formatting the found timestamp '{}'", error),
        }
        debug!("added the timestamp to the info")
    }

    // set the content of the info view to the generated info text
    if siv
        .call_on_name("search_result_info", |view: &mut TextView| {
            view.set_content(info_text);
        })
        .is_none()
    {
        let error = anyhow!("couldn't find the search info view")
            .context("failed displaying the generated result info");
        warn!("{:?}", error);
        display_error(siv, error);
    };
    debug!("displayed the generated result info");

    // set the content of the result preview view to the generated preview
    if siv
        .call_on_name("search_result_preview", |view: &mut TextView| {
            view.set_content(preview);
        })
        .is_none()
    {
        let error = anyhow!("couldn't find the search result view")
            .context("failed displaying the generated preview");
        warn!("{:?}", error);
        display_error(siv, error);
    };
    debug!("displayed the generated result preview");
}

/// Searches for more results at a given offset and adds them to the results view. It's a callback
/// for the continue button and displays an error if something went wrong
fn on_continue_submit(siv: &mut Cursive, search_query: &str, search_offset: &usize) {
    // continue the search and fetch more results
    let search = match continue_search(search_query, search_offset)
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
    if let Err(error) = display_more_search_results(siv, search, search_query).with_context(|| {
        format!(
            "failed displaying more search results for '{}'",
            search_query
        )
    }) {
        warn!("{:?}", error);
        display_error(siv, error);
        return;
    }
}

/// Continues the search and fetches more search results for a given query. Returns an error
/// if something went wrong
fn continue_search(search_query: &str, search_offset: &usize) -> Result<Search> {
    info!("fetching more search results for '{}'", search_query);

    // fetch more results
    build_search()
        .query(search_query.to_string())
        .offset(*search_offset)
        .search()
}

/// Adds more search results to the already existing search panel
fn display_more_search_results(
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
