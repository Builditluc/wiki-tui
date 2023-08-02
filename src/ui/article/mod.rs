use crate::config::{Config, TocPosition, CONFIG};
use crate::ui::panel::WithPanel;
use crate::ui::search::bar_popup::open_search_bar;
use crate::ui::toc::display_toc;
use crate::ui::utils::{display_dialog, display_error, display_message};
use crate::ui::views::{RootLayout, StatusBar};
use crate::wiki::article::link_data::InternalData;
use crate::wiki::article::{Article, Link, Property};
use crate::wiki::search::Namespace;

use anyhow::{Context, Result};
use cursive::view::{Nameable, Resizable};
use cursive::views::{LastSizeView, LinearLayout, OnEventView, TextView};
use cursive::Cursive;

mod content;
mod lines;
mod view;
pub type ArticleView = view::ArticleView;

pub const ARTICLE_PROPERTIES: [Property; 3] =
    [Property::Text, Property::Sections, Property::LangLinks];
const SUPPORTED_NAMESPACES: [Namespace; 1] = [Namespace::Main];

/// Fetches an article from a given id and displays it. It's the on_submit callback for
/// the search results view
pub fn on_article_submit(siv: &mut Cursive, pageid: usize) {
    let config = Config::from_siv(siv);

    let article = match Article::builder()
        .pageid(pageid)
        .from_url(config.borrow().api_config.url())
        .properties(ARTICLE_PROPERTIES.to_vec())
        .language(config.borrow().api_config.language.clone())
        .fetch()
        .context("failed fetching the article")
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

/// Checks that the link is supported (supported Namespace, supported link type) and opens it
pub fn open_link(siv: &mut Cursive, link: Link) {
    let message = match link {
        Link::Internal(data) => {
            return display_dialog(
                siv,
                "Information",
                &format!("Do you want to open the link '{}'?", data.title),
                move |siv| open_internal_link(siv, data.clone()),
            )
        }
        Link::Anchor(_) => "Anchor links are not supported",
        Link::RedLink(data) => {
            warn!("tried opening a red link '{}'", data.url);
            return display_message(
                siv,
                "Information",
                &format!(
                    "The page '{}' doesn't exist and therefore cannot be opened",
                    data.title
                ),
            );
        }
        Link::External(data) => {
            warn!("tried opening an external link '{}'", data.url);
            return display_message(siv, "Information", &format!("This link doesn't point to another article. \nInstead, it leads to the following external webpage and therefore, cannot be opened: \n\n> {}", data.url.as_str()));
        }
        Link::ExternalToInternal(_) => "External to Internal links are not supported",
    };

    warn!("{}", message);
    display_message(siv, "Warning", message)
}

/// Helper function for fetching and displaying an article from a given link
fn open_internal_link(siv: &mut Cursive, data: InternalData) {
    if !SUPPORTED_NAMESPACES.contains(&data.namespace) {
        display_message(
            siv,
            "Information",
            &format!(
                "The link leads to an article in the '{}' namespace which is supported",
                data.namespace
            ),
        );
        return;
    }

    // fetch the article
    let article = match Article::builder()
        .page(data.page)
        .endpoint(data.endpoint)
        .properties(ARTICLE_PROPERTIES.to_vec())
        .language(data.language)
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

    // display the article
    if let Err(error) = display_article(siv, article).context("failed displaying the article") {
        warn!("{:?}", error);

        // display an error message
        display_error(siv, error);
        return;
    }
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
        .language(&article.language)
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

    match CONFIG.settings.toc.position {
        TocPosition::Left => article_layout.add_child(article_view),
        TocPosition::Right => article_layout.insert_child(0, article_view),
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
