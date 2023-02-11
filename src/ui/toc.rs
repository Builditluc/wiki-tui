use crate::config;
use crate::ui::panel::WithPanel;
use crate::ui::utils::display_error;
use crate::ui::{self, article::ArticleView, views::RootLayout};
use crate::view_with_theme;
use crate::wiki::article::Section;
use anyhow::Result;

use cursive::event::{Event, Key};
use cursive::traits::Scrollable;
use cursive::view::{Nameable, Resizable};
use cursive::views::SelectView;
use cursive::Cursive;

/// Adds a table of contents to a given layout
pub fn display_toc<'a>(
    siv: &mut Cursive,
    layout: &mut RootLayout,
    sections: impl Iterator<Item = &'a Section>,
) -> Result<()> {
    let layer_len = siv.screen_mut().len();

    let toc_view_name = format!("toc_view-{}", layer_len);
    debug!("toc_view name '{}'", toc_view_name);

    let mut toc_view = SelectView::<usize>::new().on_submit(|siv, id| {
        debug!("jumping to '{}'", id);

        let layer_len = siv.screen_mut().len();
        let article_view_name = format!("article_view-{}", layer_len);

        // select the header in the article view
        if let Some(mut view) = siv.find_name::<ArticleView>(&article_view_name) {
            view.select_section(*id);
            debug!("selected the header in the article view");
        }

        // focus the article view
        if let Err(error) = siv.focus_name(&article_view_name) {
            let err = anyhow!(error).context(format!("couldn't find '{}'", article_view_name));
            display_error(siv, err);
            return;
        }
        debug!("focussed the article view");

        // update the article view
        siv.on_event(Event::Key(Key::Down));
        siv.on_event(Event::Key(Key::Up));

        debug!("send the callback to update the article view");
    });
    debug!("created the toc view");

    // add the items to the table of contents
    for section in sections {
        // add the item to the select_view
        let label = format!("{} {}", section.number(), section.text());
        debug!("added the item: '{}' to the toc_view", label);
        toc_view.add_item(label, section.index());
    }
    debug!("added the items to the table of contents");

    // add the toc to the layout
    layout.add_child(
        view_with_theme!(
            config::CONFIG.theme.toc_view,
            toc_view
                .with_name(toc_view_name)
                .scrollable()
                .scroll_x(config::CONFIG.settings.toc.scroll_x)
                .scroll_y(config::CONFIG.settings.toc.scroll_y)
                .full_height()
                .with_panel()
                .title("Content")
        )
        .min_width(config::CONFIG.settings.toc.min_width)
        .max_width(config::CONFIG.settings.toc.max_width),
    );
    debug!("added the toc_view to the article_layout");

    Ok(())
}
