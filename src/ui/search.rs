use crate::{
    config,
    ui::{self, RootLayout},
    view_with_theme,
    wiki::search::{
        SearchBuilder, SearchMetadata, SearchProperties, SearchResult, SearchSortOrder,
    },
    CONFIG,
};

use anyhow::{Context, Result};
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Button, Dialog, EditView, LinearLayout, SelectView, TextView};
use cursive::{align::HAlign, utils::markup::StyledString, Cursive};

/// Returns the default SearchBuilder
fn build_search() -> SearchBuilder {
    SearchBuilder::new(&config::CONFIG.api_config.base_url)
        .info(SearchMetadata::new().total_hits())
        .prop(SearchProperties::new().snippet())
        .sort(SearchSortOrder::JustMatch)
}

/// Searches for a given query and displays the results. Returns an error if something went wrong.
pub fn on_search(siv: &mut Cursive, search_query: String) {
    log::info!("on_search was called");

    // do the search and if something went wrong, display an error message to the user
    log::info!("searching for '{}'", search_query);
    let search = match build_search().query(search_query.clone()).search() {
        Ok(search) => search,
        Err(error) => {
            // log the error
            log::warn!("{}", error);

            // display an error message
            siv.add_layer(
                Dialog::info(
                    "A Problem occurred while searching. \nCheck the logs for further information",
                )
                .title("Error")
                .title_position(HAlign::Center),
            );
            log::info!("on_search failed to finish");
            return;
        }
    };

    // clear the search bar
    log::debug!("clearing the search bar");
    siv.call_on_name("search_bar", |view: &mut EditView| {
        view.set_content("");
    });

    // Create the views

    // create the results view letting the user select an result
    log::info!(
        "displaying '{}' out of '{}' search results",
        search.results().count(),
        search.info().total_hits().unwrap_or(&-1),
    );
    let mut search_results_view = SelectView::<SearchResult>::new()
        .on_select(on_result_select)
        .on_submit(ui::article::on_article_submit);

    // create the continue button
    let search_continue_button = {
        let query = search_query.to_string();
        let offset = search.search_offset().to_owned();
        Button::new("Show more results...", move |s| {
            if let Err(error) = continue_search(s, &query, &offset) {
                log::warn!("{:?}", error);
            }
        })
        .with_name("search_continue_button")
    };

    // create the results preview displaying previews of the currently selected article
    let search_results_preview = TextView::empty()
        .h_align(cursive::align::HAlign::Left)
        .with_name("search_results_preview")
        .fixed_width(50);

    // create the info view showing the total hits
    let mut search_info_view = TextView::empty();
    log::debug!("created the search results view, the search continue button, the search results preview and the search info view");
    if let Some(total_hits) = search.info().total_hits() {
        search_info_view.set_content(format!(
            "Found {} articles matching your search",
            total_hits
        ));
    }

    // save the first result so we can display its preview
    let first_result = search.results().next().cloned();

    // add the search results to the results view
    log::debug!("adding the results to the search results view");
    for search_result in search.results() {
        search_results_view.add_item(search_result.title().to_string(), search_result.to_owned())
    }

    // create the search results layout
    let search_results_layout = RootLayout::horizontal(CONFIG.keybindings.clone())
        .child(view_with_theme!(
            config::CONFIG.theme.search_results,
            Dialog::around(
                LinearLayout::vertical()
                    .child(
                        search_results_view
                            .with_name("search_results_view")
                            .scrollable()
                            .min_height(10)
                    )
                    .child(search_continue_button),
            )
        ))
        .child(view_with_theme!(
            config::CONFIG.theme.search_preview,
            Dialog::around(search_results_preview)
        ));
    log::debug!("created the search results layout");

    // finally, add the whole thing as a new layer
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(search_results_layout)
                .child(search_info_view),
        )
        .title(format!("Results for \"{}\"", search_query))
        .dismiss_button("Back")
        .button("Quit", Cursive::quit)
        .max_height(20),
    );
    log::debug!("added the search view to the screen");

    // send a callback selecting the first search result
    log::debug!("sending the callback to select the first search result");
    if let Err(error) = siv.cb_sink().send(Box::new(|s| {
        if let Some(search_result) = first_result {
            on_result_select(s, &search_result);
        }
    })) {
        log::warn!("{:?}", error);
        log::info!("on_search failed to finish");
        return;
    }

    log::info!("on_search finished successfully");
}

/// Generates and displays a preview of a given search result. It's used as a callback for the
/// search results view
fn on_result_select(siv: &mut Cursive, item: &SearchResult) {
    log::info!(
        "on_result_select was called with the item '{}', page id: '{}'",
        item.title(),
        item.page_id()
    );

    log::debug!("generating the preview");
    let mut preview = StyledString::new();

    // add the title to the preview
    log::debug!("adding the title to the preview");
    preview.append_plain(format!("{}\n", item.title()));

    // only go through this if we have a snippet
    if let Some(snippet) = item.snippet() {
        log::debug!("found a snippet for the result, adding it to the preview now");
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
    }

    // set the content of the preview view to the generated preview
    log::debug!("displaying the generated preivew");
    let result = siv.call_on_name("search_results_preview", |view: &mut TextView| {
        view.set_content(preview);
    });
    if result.is_none() {
        log::warn!("couldn't find the search results preview view");
        log::info!("on_result_select failed to finish");
        return;
    }

    log::info!("on_result_select finished successfully");
}

/// Searches for more results at a given offset and adds them to the results view. It's a callback
/// for the continue button and returns an error if something went wrong
fn continue_search(siv: &mut Cursive, search_query: &str, search_offset: &usize) -> Result<()> {
    log::info!(
        "continue_search was called for the query '{}' with the offset '{}'",
        search_query,
        search_offset
    );

    // fetch more results
    log::info!("fetching more results");
    let search = build_search()
        .query(search_query.to_string())
        .offset(*search_offset)
        .search()?;

    // get the results view so we can add some results to it
    log::debug!("getting the search results view");
    let mut search_results_views = siv
        .find_name::<SelectView<SearchResult>>("search_results_view")
        .with_context(|| {
            log::info!("continue_search failed to finish");
            "Couldn't find the search results view"
        })?;

    // add the new results to the view
    log::info!(
        "adding '{}' results to the search results view",
        search.results().count()
    );
    for search_result in search.results() {
        search_results_views.add_item(search_result.title(), search_result.clone())
    }

    // get the continue button so we can change its callback
    log::debug!("modifying the callback of the search continue button");
    let mut search_continue_button = siv
        .find_name::<Button>("search_continue_button")
        .with_context(|| {
            log::info!("continue_search failed to finish");
            "Couldn't find the search continue button"
        })?;

    // modify the callback of the continue button so we don't search for the same thing again
    {
        let query = search_query.to_string();
        search_continue_button.set_callback(move |s| {
            if let Err(error) = continue_search(s, &query, search.search_offset()) {
                log::warn!("{:?}", error);
            }
        });
    }

    // focus the results view
    siv.focus_name("search_results_view").with_context(|| {
        log::info!("continue_search failed to finish");
        "Failed to focus the search results view"
    })?;
    log::debug!("focussed the search results view");

    log::info!("continue_search finished successfully");
    Ok(())
}
