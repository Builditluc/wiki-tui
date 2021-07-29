#[macro_use]
extern crate log;
extern crate anyhow;
extern crate ini;
extern crate lazy_static;

use anyhow::*;
use cursive::align::HAlign;
use cursive::theme::*;
use cursive::traits::*;
use cursive::utils::*;
use cursive::view::{Resizable, Scrollable};
use cursive::views::*;
use cursive::Cursive;

pub mod config;
pub mod logging;
pub mod tests;
pub mod ui;
pub mod wiki;

pub const LOGO: &str = "
  _      __   (_)   / /__   (_)         / /_  __  __   (_)
| | /| / /  / /   / //_/  / /  ______ / __/ / / / /  / /
| |/ |/ /  / /   / ,<    / /  /_____// /_  / /_/ /  / /
|__/|__/  /_/   /_/|_|  /_/          \\__/  \\__,_/  /_/ 
";

fn main() {
    // Initialize the logging module
    logging::Logger::initialize();

    // Create the wiki struct, used for interaction with the wikipedia website/api
    let wiki = wiki::WikiApi::new();

    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);
    siv.set_user_data(wiki);

    // get and apply the color theme
    let theme = Theme {
        palette: get_color_palette(),
        ..Default::default()
    };
    siv.set_theme(theme);

    // Create the views
    let search_bar = EditView::new()
        .on_submit(|s, q| on_search(s, q.to_string()))
        .with_name("search_bar")
        .full_width();

    let search_layout = Dialog::around(LinearLayout::horizontal().child(search_bar))
        .title("Search")
        .title_position(cursive::align::HAlign::Left);

    let logo_view = TextView::new(LOGO)
        .h_align(HAlign::Center)
        .with_name("logo_view")
        .full_screen();

    let article_layout = LinearLayout::horizontal()
        .child(Dialog::around(logo_view))
        .with_name("article_layout");

    // Add a fullscreen layer, containing the search bar and the article view
    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(search_layout)
                .child(article_layout),
        )
        .title("wiki-tui")
        .button("Quit", Cursive::quit)
        .full_screen(),
    );

    // Start the application
    siv.run();
}

fn on_search(siv: &mut Cursive, search_query: String) {
    log::info!("on_search was called");
    let wiki: &wiki::WikiApi = siv.user_data().unwrap();

    if search_query.is_empty() {
        log::warn!("Empty Search Query, aborting Search");
        return;
    }
    log::info!("The Search Query is \"{}\"", search_query);

    // Search wikipedia for the search query and the response
    let search_response = wiki.search(&search_query);

    // are there any results?
    let mut search_results_exist = true;
    if search_response.continue_code.continue_code == *"" {
        search_results_exist = false;
        log::warn!("No articles were found with the given query");
    }

    // clear the search bar
    siv.call_on_name("search_bar", |view: &mut EditView| {
        view.set_content("");
    });

    // Create the views
    let mut search_results_view = SelectView::<ui::models::ArticleResultPreview>::new()
        .on_select(|s, item| on_result_select(s, item))
        .on_submit(|s, a| on_article_submit(s, a));

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
        Some(search_results_view.iter().next().unwrap().1.clone())
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
        .child(Dialog::around(
            LinearLayout::vertical()
                .child(
                    search_results_view
                        .with_name("search_results_view")
                        .scrollable()
                        .min_height(10),
                )
                .child(search_continue_button),
        ))
        .child(Dialog::around(search_results_preview));

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

fn on_article_submit(siv: &mut Cursive, article_preview: &ui::models::ArticleResultPreview) {
    // remove the results layer
    siv.pop_layer();

    // remove views
    remove_view_from_article_layout(siv, "logo_view");
    remove_view_from_article_layout(siv, "article_view");
    remove_view_from_article_layout(siv, "toc_view");

    // get the article from wikipedia
    let wiki: &wiki::WikiApi = siv.user_data().unwrap();
    let parsed_article = wiki.get_article(&article_preview.page_id);

    let mut article_view =
        ui::article::ArticleView::new().on_link_submit(|s, target| on_link_submit(s, target));

    // set the contents of the article_view to the article
    log::info!("Setting the content of the article view");
    article_view.set_article(parsed_article.clone().article);

    // add the article_view to the article_layout]
    siv.call_on_name("article_layout", |view: &mut LinearLayout| {
        view.insert_child(
            0,
            Dialog::around(article_view.with_name("article_view").scrollable()),
        );
    });
    log::info!("Added the article_view to the article_layout");

    // does this article have a table of contents?
    if parsed_article.toc.is_some() {
        log::info!("The article contains a table of contents");
        add_table_of_contents(siv, parsed_article.toc.unwrap());
    } else {
        log::info!("The article doesn't contain a table of contents");
    }

    // focus the article view
    let result = siv
        .focus_name("article_view")
        .context("Failed to focus the article view");

    match result {
        Ok(_) => log::info!("Successfully focussed the article view"),
        Err(error) => log::warn!("{:?}", error),
    }
}

fn continue_search(
    siv: &mut Cursive,
    search_query: String,
    continue_code: &wiki::search::ContinueCode,
) {
    // if there is no valid continue code, abort
    if continue_code.continue_code == *"" {
        warn!("Invalid continue code, aborting search");
        return;
    }

    // get more search results from wikipedia and find the search_results_view
    let wiki: &wiki::WikiApi = siv.user_data().unwrap();
    let search_response = wiki.continue_search(&search_query, continue_code);
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

fn get_color_palette() -> Palette {
    let mut custom_palette = Palette::default();

    custom_palette.set_color("View", config::CONFIG.theme.background);
    custom_palette.set_color("Primary", config::CONFIG.theme.text);
    custom_palette.set_color("TitlePrimary", config::CONFIG.theme.title);
    custom_palette.set_color("Highlight", config::CONFIG.theme.highlight);
    custom_palette.set_color("HighlightInactive", config::CONFIG.theme.highlight_inactive);
    custom_palette.set_color("HighlightText", config::CONFIG.theme.highlight_text);

    custom_palette
}

fn on_link_submit(siv: &mut Cursive, target: &str) {
    let target = target.to_string();

    siv.add_layer(
        Dialog::around(TextView::new(format!(
            "Do you want to open the dialog {}?",
            target
        )))
        .button("Yes", move |s| show_article_from_link(s, target.clone()))
        .button("No", |s| {
            s.pop_layer();
        }),
    )
}

fn show_article_from_link(siv: &mut Cursive, target: String) {
    siv.pop_layer();

    // remove views
    remove_view_from_article_layout(siv, "logo_view");
    remove_view_from_article_layout(siv, "article_view");
    remove_view_from_article_layout(siv, "toc_view");

    // get the article from wikipedia
    let wiki: &wiki::WikiApi = siv.user_data().unwrap();
    let parsed_article = wiki.open_article(&target);

    let mut article_view =
        ui::article::ArticleView::new().on_link_submit(|s, target| on_link_submit(s, target));

    // set the contents of the article_view to the article
    log::info!("Setting the content of the article view");
    article_view.set_article(parsed_article.clone().article);

    // add the article_view to the article_layout]
    siv.call_on_name("article_layout", |view: &mut LinearLayout| {
        view.insert_child(
            0,
            Dialog::around(
                article_view
                    .with_name("article_view")
                    .full_height()
                    .scrollable(),
            ),
        );
    });
    log::info!("Added the article_view to the article_layout");

    // does this article have a table of contents?
    if parsed_article.toc.is_some() {
        log::info!("The article contains a table of contents");
        add_table_of_contents(siv, parsed_article.toc.unwrap());
    } else {
        log::info!("The article doesn't contain a table of contents");
    }

    // focus the article view
    let result = siv
        .focus_name("article_view")
        .context("Failed to focus the article view");

    match result {
        Ok(_) => log::info!("Successfully focussed the article view"),
        Err(error) => log::warn!("{:?}", error),
    }
}

fn add_table_of_contents(siv: &mut Cursive, toc: ui::models::table_of_contents::Table) {
    use ui::models::table_of_contents;

    // get the article_layout and create an empty select view
    let mut article_layout = siv.find_name::<LinearLayout>("article_layout").unwrap();
    let mut toc_view: SelectView<table_of_contents::Item> = SelectView::new();

    // now go through every item
    log::info!("Adding the table of content to the toc_view");
    for item in toc.items.into_iter() {
        add_item_to_toc(&mut toc_view, item);
    }

    article_layout.insert_child(
        1,
        Dialog::around(toc_view.with_name("toc_view").full_height()).title(toc.title),
    );
    article_layout.set_weight(1, 10);
    log::info!("Added the toc_view to the article_layout");
}

fn add_item_to_toc(
    toc_view: &mut SelectView<ui::models::table_of_contents::Item>,
    item: ui::models::table_of_contents::Item,
) {
    // add the item to the select_view
    let label = format!("{}{}", " ".repeat(item.number as usize), item.text);
    log::debug!("Added the item: {} to the toc_view", label);
    toc_view.add_item(label, item);
}

fn remove_view_from_article_layout(siv: &mut Cursive, view_name: &str) {
    siv.call_on_name("article_layout", |view: &mut LinearLayout| {
        if let Some(i) = view.find_child_from_name(view_name) {
            log::info!("Removing the {} from the article_layout", view_name);
            view.remove_child(i);
        } else {
            log::warn!("Couldn't find the {}", view_name);
        }
    });
}
