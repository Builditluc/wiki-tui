use crate::config::{self, TocPosition, CONFIG};
use crate::ui::panel::WithPanel;
use crate::ui::utils::display_error;
use crate::ui::{self, article::ArticleView, views::RootLayout};
use crate::view_with_theme;
use crate::wiki::article::TableOfContents;
use crate::wiki::article::TableOfContentsItem;
use anyhow::{Context, Result};

use cursive::event::{Event, Key};
use cursive::traits::Scrollable;
use cursive::view::{Nameable, Resizable};
use cursive::views::SelectView;
use cursive::Cursive;

/// Adds a table of contents to a given layout
pub fn add_table_of_contents(siv: &mut Cursive, toc: &TableOfContents) -> Result<()> {
    let layer_len = siv.screen_mut().len();

    let article_layout_name = format!("article_layout-{}", layer_len);
    let toc_view_name = format!("toc_view-{}", layer_len);
    debug!("toc_view name '{}'", toc_view_name);

    // get the article_layout and create an empty select view
    let mut article_layout = siv
        .find_name::<RootLayout>(&article_layout_name)
        .context("couldn't find the layout")?;
    debug!("found the layout");

    let mut toc_view = SelectView::<TableOfContentsItem>::new().on_submit(|siv, item| {
        debug!("jumping to '{}'", item.text());

        let layer_len = siv.screen_mut().len();
        let article_view_name = format!("article_view-{}", layer_len);
        let toc_view_name = format!("toc_view-{}", layer_len);

        // get the index of the toc items
        let item_index = match siv.find_name::<SelectView<TableOfContentsItem>>(&toc_view_name) {
            Some(view) => {
                let mut index: usize = 0;
                for (idx, _item) in view.iter().enumerate() {
                    if _item.1.text() == item.text() {
                        index = idx;
                        break;
                    }
                }
                index
            }
            None => {
                warn!("couldn't find the toc_view");
                0_usize
            }
        };
        debug!("the index for the toc item is '{}'", item_index);

        // select the header in the article view
        if let Some(mut view) = siv.find_name::<ArticleView>(&article_view_name) {
            view.select_header(item_index);
            debug!("selected the header in the article view");
        }

        // focus the article view
        if let Err(error) = siv.focus_name(&article_view_name) {
            let err = anyhow!(error).context(format!("couldn't find '{}'", article_view_name));
            display_error(siv, err);
            return;
        }
        debug!("focussed the article view");

        // send a callback to update the article view
        if let Err(error) = siv.cb_sink().send(Box::new(move |siv: &mut Cursive| {
            siv.on_event(Event::Key(Key::Down));
            siv.on_event(Event::Key(Key::Up));
        })) {
            warn!(
                "{}",
                anyhow!(error.to_string())
                    .context("failed sending the callback to update the article view")
            );
            return;
        };
        debug!("send the callback to update the article view");
    });
    debug!("created the toc view");

    // add the items to the table of contents
    for item in toc.items() {
        // add the item to the select_view
        let label = format!("{}{}", " ".repeat(*item.number() as usize), item.text());
        debug!("added the item: '{}' to the toc_view", label);
        toc_view.add_item(label, item.clone());
    }
    debug!("added the items to the table of contents");

    // get the index of the table of contents in the layout
    let toc_layout_index = match CONFIG.settings.toc.position {
        TocPosition::Left => 0_usize,
        TocPosition::Right => 1_usize,
    };
    debug!("toc_layout_index: '{}'", toc_layout_index);

    // add the toc to the layout
    article_layout.insert_child(
        toc_layout_index,
        view_with_theme!(
            config::CONFIG.theme.toc_view,
            toc_view
                .with_name(toc_view_name)
                .scrollable()
                .scroll_x(config::CONFIG.settings.toc.scroll_x)
                .scroll_y(config::CONFIG.settings.toc.scroll_y)
                .full_height()
                .with_panel()
                .title(toc.title())
        )
        .min_width(config::CONFIG.settings.toc.min_width)
        .max_width(config::CONFIG.settings.toc.max_width),
    );
    debug!("added the toc_view to the article_layout");

    Ok(())
}
