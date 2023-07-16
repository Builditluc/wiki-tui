use crate::config::CONFIG;
use crate::config::{self, Config};
use crate::ui::panel::WithPanel;
use crate::ui::root::RootLayout;
use crate::ui::search::bar_popup::open_search_bar;
use crate::ui::toc::display_toc;
use crate::ui::utils::{display_dialog, display_error};
use crate::ui::views::StatusBar;
use crate::wiki::article::{Article, ArticleBuilder, NoPage, NoPageID, NoUrl, Property};

use anyhow::{Context, Result};
use cursive::view::{Nameable, Resizable};
use cursive::views::{LastSizeView, LinearLayout, OnEventView, TextView};
use cursive::Cursive;

mod content;
mod lines;
mod view;
pub type ArticleView = view::ArticleView;

pub fn builder() -> ArticleBuilder<NoPageID, NoPage, NoUrl> {
    Article::builder().properties(vec![
        Property::Text,
        Property::Sections,
        Property::LangLinks,
    ])
}

/// Fetches an article from a given id and displays it. It's the on_submit callback for
/// the search results view
pub fn on_article_submit(siv: &mut Cursive, pageid: usize) {
    let config = Config::from_siv(siv);

    let article = match builder()
        .pageid(pageid)
        .url(
            config.borrow().api_config.language.clone(),
            &config.borrow().api_config.pre_language,
            &config.borrow().api_config.post_language,
        )
        .fetch()
    {
        Ok(article) => article,
        Err(error) => {
            warn!("{:?}", error);
            display_error(siv, error);
            return;
        }
    };
    if let Err(error) = display_article(siv, article) {
        warn!("{:?}", error);
        display_error(siv, error)
    }
}

/// Displays a confirmation dialog on whether to open the link. It's the on_submit callback for the
/// article view
pub fn on_link_submit(siv: &mut Cursive, target: String) {
    // convert the target into a human-friendly format
    let target = target.strip_prefix("/wiki/").unwrap_or(&target).to_string();
    let target_human = target.replace('_', " ");

    info!("requesting confirmation to open the link '{}'", target);

    let title = String::new();
    let body = format!("Do you want to open the article '{}'?", target_human);

    display_dialog(siv, &title, &body, move |siv| {
        info!("opening the link '{}'", target);
        // open the link
        open_link(siv, target.clone())
    });
}

/// Helper function for fetching and displaying an article from a given link
fn open_link(siv: &mut Cursive, target: String) {
    let config = Config::from_siv(siv);

    // fetch the article
    let article = match builder()
        .page(target)
        .url(
            config.borrow().api_config.language.clone(),
            &config.borrow().api_config.pre_language,
            &config.borrow().api_config.post_language,
        )
        .fetch()
        .context("failed fetching the article")
    {
        Ok(article) => article,
        Err(error) => {
            warn!("{:?}", error);

            // display an error message
            display_error(siv, error);
            return;
        }
    };
    debug!("fetched the article");

    // display the article
    if let Err(error) = display_article(siv, article) {
        warn!("{:?}", error);

        // display an error message
        // TODO: use builtin helper function for error message here
        display_error(siv, error);
        return;
    }
    debug!("displayed the article");
}

/// Helper function for displaying an article on the screen. This includes creating an article view
/// and any errors it encountred are returned
pub fn display_article(siv: &mut Cursive, article: Article) -> Result<()> {
    // if the search layer still exists, then remove it
    if siv
        .find_name::<TextView>("search_results_preview")
        .is_some()
    {
        siv.pop_layer();
        debug!("removed the last layer")
    }

    // get the amount of layers, this is used as an id for the new article layout so we don't have
    // multiple layouts with the same name

    let layer_len = siv.screen_mut().len() + 1;

    let article_layout_name = format!("article_layout-{}", layer_len);
    let article_view_name = format!("article_view-{}", layer_len);
    let status_bar_name = format!("status_bar-{}", layer_len);

    debug!("article_layout name '{}'", article_layout_name);
    debug!("article_view name '{}'", article_view_name);
    debug!("status_bar name '{}'", status_bar_name);

    let mut article_layout = RootLayout::horizontal(CONFIG.keybindings.clone());
    debug!("created the article layout");

    // display the toc if there is one
    if let Some(sections) = article.sections() {
        if let Err(error) = display_toc(siv, &mut article_layout, sections)
            .context("failed displaying the table of contents")
        {
            warn!("{:?}", error);
            display_error(siv, error);
        } else {
            debug!("displayed the table of contents");
        }
    }

    let status_bar = StatusBar::new()
        .article_title(article.title())
        .language(article.language())
        .available_languages(article.available_languages().unwrap_or_default())
        .with_name(&status_bar_name)
        .fixed_height(1)
        .full_width();

    // create the article view
    let article_view = LastSizeView::new(
        ArticleView::new(article)
            .with_name(&article_view_name)
            .with_panel()
            .title("wiki-tui"),
    );

    match config::CONFIG.settings.toc.position {
        config::TocPosition::Left => article_layout.add_child(article_view),
        config::TocPosition::Right => article_layout.insert_child(0, article_view),
    }
    debug!("created the article view");

    siv.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(
                OnEventView::new(article_layout.with_name(&article_layout_name))
                    .on_event('S', open_search_bar)
                    .fixed_height(siv.screen_size().y.saturating_sub(1)),
            )
            .child(status_bar)
            .full_screen(),
    );
    debug!("created a new fullscreen layer and added the article layout to it");

    // focus the article view
    siv.focus_name(&article_view_name)
        .context("failed focussing the article view")?;

    debug!("focussed the article view");
    Ok(())
}
