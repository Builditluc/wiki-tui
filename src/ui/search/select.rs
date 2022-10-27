use anyhow::Result;
use chrono::DateTime;
use cursive::{utils::markup::StyledString, views::TextView, Cursive};

use crate::{config, wiki::search::SearchResult};

/// Generates a preview from a given search result. Any errors are returned
pub fn generate_and_display_preview(siv: &mut Cursive, item: &SearchResult) -> Result<()> {
    // check if we even have a preview snippet
    if item.snippet().is_none() {
        bail!("no preview snippet found");
    }

    let snippet = item.snippet().unwrap();
    let mut preview = StyledString::new();

    // add the title of the item to the preview
    preview.append_plain(format!("{}\n", item.title()));

    let split_snippet: Vec<&str> = snippet.split(r#"<span class="searchmatch">"#).collect();

    // go through every slice of the splitted_snippet and if it contains </span>,
    // split the slice again and make the first split red
    for slice in split_snippet {
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
    debug!("generated the preview");

    // now display the generated preview
    if siv
        .call_on_name("search_result_preview", |view: &mut TextView| {
            view.set_content(preview);
        })
        .is_none()
    {
        return Err(anyhow!("view not found").context("failed displaying the generated preview"));
    }
    debug!("displayed the generated preview");

    Ok(())
}

/// Generates an info string from a given search result
pub fn generate_and_display_info(siv: &mut Cursive, item: &SearchResult) -> Result<()> {
    let mut info = StyledString::new();

    info.append_plain(&format!("Title: {}", item.title()));
    debug!("added the title to the info");

    // add the wordcount to the info if available
    if let Some(wordcount) = item.wordcount() {
        info.append_plain(&format!("\nWord count: {} words", wordcount.to_string()));
        debug!("added the wordcount to the info");
    }

    // add the formatted timestamp to the info if available
    if let Some(timestamp) = item.timestamp() {
        match DateTime::parse_from_rfc3339(timestamp) {
            Ok(formatted_time) => info.append_plain(&format!(
                "\nLast Edited: {}",
                formatted_time.format("%H:%M:%S %d/%m/%Y ")
            )),
            Err(error) => warn!("failed formatting the found timestamp '{}'", error),
        }
        debug!("added the timestamp to the info")
    }
    debug!("generated the info text");

    // now display the generated info
    if siv
        .call_on_name("search_result_info", |view: &mut TextView| {
            view.set_content(info);
        })
        .is_none()
    {
        return Err(anyhow!("view not found").context("failed displaying the generated info"));
    }
    debug!("displayed the generated info");

    Ok(())
}
