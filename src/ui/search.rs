use crate::{
    config, ui, view_with_theme,
    wiki::search::{
        SearchBuilder, SearchMetadata, SearchProperties, SearchResult, SearchSortOrder,
    },
};

use anyhow::{Context, Result};
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Button, Dialog, EditView, LinearLayout, SelectView, TextView};
use cursive::{align::HAlign, utils::markup::StyledString, Cursive};

/// Returns the default SearchBuilder
fn build_search() -> SearchBuilder {
    SearchBuilder::new()
        .info(SearchMetadata::new().total_hits())
        .prop(SearchProperties::new().snippet())
        .sort(SearchSortOrder::JustMatch)
}

/// Searches for a given query and displays the result
/// Returns an error if something went wrong
pub fn on_search(siv: &mut Cursive, search_query: String) -> Result<()> {
    // do the search and if something went wrong, display an error message to the user
    let search = match build_search().query(search_query.clone()).search() {
        Ok(search) => search,
        Err(error) => {
            siv.add_layer(
                Dialog::info(
                    "A Problem occurred while searching. \nCheck the logs for further information",
                )
                .title("Error")
                .title_position(HAlign::Center),
            );
            return Err(error);
        }
    };

    // clear the search bar
    siv.call_on_name("search_bar", |view: &mut EditView| {
        view.set_content("");
    });

    // Create the views
    
    // create the results view letting the user select an result
    let mut search_results_view = SelectView::<SearchResult>::new().on_select(on_result_select);
    //.on_submit(ui::article::on_article_submit);

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
    if let Some(total_hits) = search.info().total_hits() {
        search_info_view.set_content(format!(
            "Found {} articles matching your search",
            total_hits
        ));
    }

    // save the first result so we can display its preview
    let first_result = search.results().cloned().next();

    // add the search results to the results view
    for search_result in search.results() {
        search_results_view.add_item(search_result.title().to_string(), search_result.to_owned())
    }

    // create the search results layout
    let search_results_layout = LinearLayout::horizontal()
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

    log::info!("Finished the search, displaying the results");

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

    // send a callback selecting the first search result
    if let Err(error) = siv.cb_sink().send(Box::new(|s| {
        if let Some(search_result) = first_result {
            on_result_select(s, &search_result);
        }
    })) {
        log::warn!("{:?}", error);
    }

    Ok(())
}

// Takes a search result and shows its formatted snippet in the search preview
// If the search result doesn't have a snippet, then it'll only show the title
fn on_result_select(siv: &mut Cursive, item: &SearchResult) {
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
    }

    // set the content of the preview view to the generated preview
    siv.call_on_name("search_results_preview", |view: &mut TextView| {
        view.set_content(preview);
    });
}

// This is a Callback for the continue button, searching for more results at the given offset and
// displays them
// Returns an error if something went wrong
fn continue_search(siv: &mut Cursive, search_query: &str, search_offset: &usize) -> Result<()> {
    // fetch more results
    let search = build_search()
        .query(search_query.to_string())
        .offset(*search_offset)
        .search()?;

    // get the results view so we can add some results to it
    let mut search_results_views = siv
        .find_name::<SelectView<SearchResult>>("search_results_view")
        .context("Couldn't find the search results view")?;

    // add the new results to the view
    for search_result in search.results() {
        search_results_views.add_item(search_result.title(), search_result.clone())
    }

    // get the continue button so we can change its callback
    let mut search_continue_button = siv
        .find_name::<Button>("search_continue_button")
        .context("Couldn't find the search continue button")?;

    // modify the callback of the continue button so we don't search for the same thing again
    {
        let query = search_query.to_string();
        search_continue_button.set_callback(move |s| {
            if let Err(error) = continue_search(s, &query, search.search_offset()) {
                log::warn!("{:?}", error);
            }
        });
    }

    log::info!("Finished the search, displaying the article now");

    // focus the results view
    siv.focus_name("search_results_view")
        .context("Failed to focus the search results view")?;

    Ok(())
}
