use crate::config;
use crate::ui::{self, article::ArticleView, RootLayout};
use crate::wiki::article::TableOfContents;
use crate::wiki::article::TableOfContentsItem;
use crate::{unwrap, view_with_theme};

use cursive::event::{Event, Key};
use cursive::traits::Scrollable;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, SelectView};
use cursive::Cursive;

pub fn add_table_of_contents(siv: &mut Cursive, toc: &TableOfContents) {
    // get the article_layout and create an empty select view

    let mut article_layout = unwrap!(
        siv.find_name::<RootLayout>("article_layout"),
        "couldn't find the article layout"
    );
    let mut toc_view = SelectView::<TableOfContentsItem>::new().on_submit(|siv, item| {
        log::info!("jumping to '{}'", item.text());
        let item_index = match siv.find_name::<SelectView<TableOfContentsItem>>("toc_view") {
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
            None => 0_usize,
        };

        log::trace!("item_index: {}", item_index);

        if let Some(mut view) = siv.find_name::<ArticleView>("article_view") {
            view.select_header(item_index)
        }

        if let Err(error) = siv.focus_name("article_view") {
            log::warn!("failed selecting the article view: {}", error);
            return;
        }

        if let Err(error) = siv.cb_sink().send(Box::new(move |siv: &mut Cursive| {
            siv.on_event(Event::Key(Key::Down));
            siv.on_event(Event::Key(Key::Up));
        })) {
            log::warn!(
                "failed sending the callback to update the article view: {}",
                error
            );
        };
    });

    // now go through every item
    log::debug!("adding the table of content to the toc_view");
    for item in toc.items() {
        add_item_to_toc(&mut toc_view, item);
    }

    article_layout.add_child(
        view_with_theme!(
            config::CONFIG.theme.toc_view,
            Dialog::around(
                toc_view
                    .with_name("toc_view")
                    .scrollable()
                    .scroll_x(config::CONFIG.settings.toc.scroll_x)
                    .scroll_y(config::CONFIG.settings.toc.scroll_y)
                    .full_height()
            )
            .title(toc.title())
        )
        .min_width(config::CONFIG.settings.toc.min_width)
        .max_width(config::CONFIG.settings.toc.max_width),
    );

    log::debug!("added the toc_view to the article_layout");
}

fn add_item_to_toc(toc_view: &mut SelectView<TableOfContentsItem>, item: &TableOfContentsItem) {
    // add the item to the select_view
    let label = format!("{}{}", " ".repeat(*item.number() as usize), item.text());
    log::debug!("added the item: {} to the toc_view", label);
    toc_view.add_item(label, item.clone());
}
