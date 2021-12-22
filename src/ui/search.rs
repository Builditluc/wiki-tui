use crate::{change_theme, config, ui, wiki};

use anyhow::{Context, Result};
use cursive::align::HAlign;
use cursive::utils::markup;
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Button, Dialog, EditView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

pub fn on_search(siv: &mut Cursive, search_query: String) -> Result<()> {
    log::info!("Beginning search");
    let wiki: &wiki::WikiApi = siv.user_data().with_context(|| {
        "the user_data is incomplete. Couldn't find the wikipedia interface in it".to_string()
    })?;

    if search_query.is_empty() {
        log::warn!("Empty search query, aborting Search");
        return Ok(());
    }
    log::info!("The search query is \"{}\"", search_query);

    // Search wikipedia for the search query and the response
    let search_response = match wiki.search(&search_query) {
        Ok(response) => response,
        Err(error) => {
            log::warn!("{:?}", error);
            // display an error_message
            siv.add_layer(
                Dialog::info(
                    "A Problem occurred while searching.\nCheck the logs for further information",
                )
                .title("Error")
                .title_position(HAlign::Center),
            );
            return Ok(());
        }
    };

    // are there any results?
    let search_results_exist = if search_response.continue_code.continue_code == *"" {
        log::warn!("No articles were found with the given query");
        false
    } else {
        true
    };

    // clear the search bar
    siv.call_on_name("search_bar", |view: &mut EditView| {
        view.set_content("");
    });

    // Create the views
    let mut search_results_view = SelectView::<ui::models::ArticleResultPreview>::new()
        .on_select(on_result_select)
        .on_submit(ui::article::on_article_submit);

    let search_results_preview = TextView::new("")
        .h_align(cursive::align::HAlign::Left)
        .with_name("search_results_preview")
        .fixed_width(50);

    let search_details_view = TextView::new(format!(
        "Found {} articles matching your search",
        &search_response.query.search_info.total_hits
    ));

    // convert the search results into Article Result Previews
    // and then add them to the results_view
    for search_result in search_response.query.search.clone() {
        let search_result = ui::models::ArticleResultPreview::from(search_result);
        search_results_view.add_item(search_result.title.to_string(), search_result);
    }

    // store the first search result to preview it
    let first_search_result = if search_results_exist {
        Some(
            search_results_view
                .iter()
                .next()
                .with_context(|| {
                    "Couldn't access the first search result. Is it missing?".to_string()
                })?
                .1
                .clone(),
        )
    } else {
        None
    };

    // create the button which continues the search when clicked
    let query = search_query.to_string();
    let search_continue_button = Button::new("Show more results...", move |s| {
        continue_search(s, query.clone(), &search_response.clone().continue_code)
    })
    .with_name("search_continue_button");

    // create the search results layout and add it as a new layer to the application
    let search_results_layout = LinearLayout::horizontal()
        .child(change_theme!(
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
        .child(change_theme!(
            config::CONFIG.theme.search_preview,
            Dialog::around(search_results_preview)
        ));

    log::info!("Finished the search, displaying the results");
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(search_results_layout)
                .child(search_details_view),
        )
        .title(format!("Results for \"{}\"", search_query))
        .dismiss_button("Back")
        .button("Quit", Cursive::quit)
        .max_height(20),
    );

    if search_results_exist {
        siv.cb_sink()
            .send(Box::new(|s| {
                on_result_select(s, &first_search_result.unwrap());
            }))
            .unwrap();
    }

    Ok(())
}

fn on_result_select(siv: &mut Cursive, item: &ui::models::ArticleResultPreview) {
    // create references for the item title and snippet
    let title = &item.title;
    let snippet = &item.snippet;

    // format the snippet for styled text
    let splitted_snippet: Vec<&str> = snippet.split(r#"<span class="searchmatch">"#).collect();

    let mut styled_snippet = markup::StyledString::new();
    styled_snippet.append_plain(format!("{}\n", title));

    // go through every slice of the splitted_snippet and if it contains </span>,
    // split the slice again and make the first split red
    for slice in splitted_snippet {
        if slice.contains("</span>") {
            let split_slice: Vec<&str> = slice.split("</span>").collect();

            styled_snippet.append(markup::StyledString::styled(
                split_slice[0],
                config::CONFIG.theme.search_match,
            ));
            styled_snippet.append_plain(split_slice[1]);
        } else {
            styled_snippet.append_plain(slice);
        }
    }
    styled_snippet.append_plain("...");

    // set the content of the result_preview view to the generated styled snippet
    siv.call_on_name("search_results_preview", |view: &mut TextView| {
        view.set_content(styled_snippet);
    });
}

fn continue_search(
    siv: &mut Cursive,
    search_query: String,
    continue_code: &wiki::search::ContinueCode,
) {
    // if there is no valid continue code, abort
    if continue_code.continue_code == *"" {
        log::warn!("Invalid continue code, aborting search");
        return;
    }

    // get more search results from wikipedia and find the search_results_view
    let wiki: &wiki::WikiApi = siv.user_data().unwrap();
    let search_response = match wiki.continue_search(&search_query, continue_code) {
        Ok(response) => response,
        Err(error) => {
            // log an error_message
            log::warn!("{:?}", error);

            // display an error_message
            siv.add_layer(
                Dialog::info(
                    "A Problem occurred while continuing the search.\nCheck the logs for further information",
                )
                .title("Error")
                .title_position(HAlign::Center),
            );
            return;
        }
    };

    let mut search_results_views = siv
        .find_name::<SelectView<ui::models::ArticleResultPreview>>("search_results_view")
        .unwrap();

    // add every new search result to the search results view
    for search_result in search_response.query.search.clone() {
        let search_result = ui::models::ArticleResultPreview::from(search_result);
        search_results_views.add_item(search_result.title.clone(), search_result);
    }

    // change the continue code of the search continue button for the next search
    let mut search_continue_button = siv.find_name::<Button>("search_continue_button").unwrap();
    search_continue_button.set_callback(move |s| {
        continue_search(s, search_query.clone(), &search_response.continue_code);
    });

    log::info!("Finished the search, displaying the article now");

    // focus the search results view
    let result = siv
        .focus_name("search_results_view")
        .context("Failed to focus the search results view");

    match result {
        Ok(_) => {
            log::info!("Successfully focussed the search results view")
        }
        Err(error) => log::warn!("{:?}", error),
    }
}
