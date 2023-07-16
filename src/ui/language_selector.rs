use std::{cell::RefCell, rc::Rc};

use anyhow::Context;
use cursive::{
    view::{Nameable, Resizable},
    views::{DummyView, EditView},
    Cursive,
};

use crate::{
    config::{Config, CONFIG},
    ui::utils::display_error,
    wiki::{
        article::LanguageLink,
        language::{Language, LANGUAGES},
    },
};

use super::{
    article::{self, ArticleView},
    panel::WithPanel,
    scroll_view::Scrollable,
    utils::{display_message, percentage},
    views::{RootLayout, SelectView},
};

const POPUP_NAME: &str = "language_selection_popup";
const LANGUAGE_SELECTION_NAME: &str = "language_selection";

const SELECTION_WIDTH_PERCENTAGE: f32 = 0.2;
const SELECTION_HEIGHT_PERCENTAGE: f32 = 0.5;

fn change_lang(siv: &mut Cursive, lang: impl Into<Language>) {
    let language: Language = lang.into();
    let success_msg = format!("Changed the language to {}", language.name());

    let user_data_changed = siv
        .with_user_data(|c: &mut Rc<RefCell<Config>>| {
            info!("changing the language to '{:?}'", language);
            c.borrow_mut().api_config.language = language;
            true
        })
        .is_some();

    if user_data_changed && CONFIG.api_config.language_changed_popup {
        display_message(siv, "Information", &success_msg);
    }
}

/// Displays a popup that lets the user chosse a new language
pub fn language_selection_popup(siv: &mut Cursive) {
    if siv.find_name::<RootLayout>(POPUP_NAME).is_some() {
        siv.pop_layer();
        return;
    }

    info!(
        "displaying '{}' languages for selection",
        LANGUAGES.iter().count()
    );

    let language_search = EditView::new()
        .on_edit(|siv, text, _| {
            let sorted_languages = LANGUAGES
                .iter()
                .filter(|lang| {
                    let lang = lang.name().to_lowercase();
                    let query = text.to_lowercase();
                    lang.contains(&query)
                })
                .map(|lang| lang.to_owned())
                .collect::<Vec<Language>>();

            siv.call_on_name(
                LANGUAGE_SELECTION_NAME,
                |language_selection: &mut SelectView<String>| {
                    language_selection.clear();
                    language_selection.add_all(
                        sorted_languages
                            .iter()
                            .map(|l| (l.name(), l.code().to_owned())),
                    );
                },
            );
        })
        .on_submit(|siv, _| {
            if let Some(selected_language) = siv.call_on_name(
                LANGUAGE_SELECTION_NAME,
                |language_selection: &mut SelectView<String>| {
                    language_selection.selection().unwrap_or_default()
                },
            ) {
                siv.pop_layer();
                change_lang(siv, selected_language.as_str());
            }
        });

    let mut language_selection: SelectView<String> =
        SelectView::new().on_submit(|s, item: &String| {
            s.pop_layer();
            change_lang(s, item.as_str());
        });

    language_selection.add_all(LANGUAGES.iter().map(|l| (l.name(), l.code().to_owned())));

    let screen_size = siv.screen_size();

    let selection_width = percentage(screen_size.x, SELECTION_WIDTH_PERCENTAGE);
    let selection_height = percentage(screen_size.y, SELECTION_HEIGHT_PERCENTAGE);

    siv.add_layer(
        RootLayout::vertical(CONFIG.keybindings.clone())
            .child(language_search)
            .child(DummyView {})
            .child(
                language_selection
                    .with_name(LANGUAGE_SELECTION_NAME)
                    .scrollable(),
            )
            .input(true)
            .with_name(POPUP_NAME)
            .with_panel()
            .title("Change Language")
            .fixed_size((selection_width, selection_height)),
    );
}

const ARTICLE_POPUP_NAME: &str = "article_language_selection_popup";
const ARTICLE_LANGUAGE_SELECTION_NAME: &str = "article_language_selection";

const ARTICLE_SELECTION_WIDTH_PERCENTAGE: f32 = 0.4;
const ARTICLE_SELECTION_HEIGHT_PERCNETAGE: f32 = 0.5;

fn change_article_language(siv: &mut Cursive, language_link: Rc<LanguageLink>) {
    let config = Config::from_siv(siv);
    let success_msg = format!(
        "Changed the language of your current article to {}",
        language_link.language.name()
    );

    let article = match article::builder()
        .page(&language_link.title)
        .url(
            language_link.language.to_owned(),
            &config.borrow().api_config.pre_language,
            &config.borrow().api_config.post_language,
        )
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

    siv.pop_layer();
    if let Err(err) = article::display_article(siv, article) {
        display_error(siv, err);
    }

    if CONFIG.api_config.article_language_changed_popup {
        display_message(siv, "Information", &success_msg);
    }
}

pub fn article_language_selection_popup(siv: &mut Cursive, languages: Vec<LanguageLink>) {
    if siv.find_name::<RootLayout>(ARTICLE_POPUP_NAME).is_some() {
        siv.pop_layer();
        return;
    }

    info!(
        "displaying '{}' article languages for selection",
        languages.len()
    );

    let mut language_selection: SelectView<LanguageLink> =
        SelectView::new().on_submit(|s, item: &LanguageLink| {
            s.pop_layer();
            change_article_language(s, item.to_owned().into());
        });

    language_selection.add_all(
        languages
            .iter()
            .map(|lang_link| (lang_link.language.name(), lang_link.to_owned())),
    );

    let language_search = EditView::new()
        .on_edit(move |siv, text, _| {
            let sorted_languages = languages
                .iter()
                .filter(|lang_link| {
                    let lang = lang_link.language.name().to_lowercase();
                    let query = text.to_lowercase();
                    lang.contains(&query)
                })
                .map(|lang_link| lang_link.to_owned())
                .collect::<Vec<LanguageLink>>();

            siv.call_on_name(
                ARTICLE_LANGUAGE_SELECTION_NAME,
                move |language_selection: &mut SelectView<LanguageLink>| {
                    language_selection.clear();
                    language_selection.add_all(
                        sorted_languages
                            .iter()
                            .map(|lang_link| (lang_link.language.name(), lang_link.to_owned())),
                    )
                },
            );
        })
        .on_submit(|siv, _| {
            if let Some(Some(selected_language)) = siv.call_on_name(
                ARTICLE_LANGUAGE_SELECTION_NAME,
                |language_selection: &mut SelectView<LanguageLink>| language_selection.selection(),
            ) {
                siv.pop_layer();
                change_article_language(siv, selected_language);
            }
        });

    let screen_size = siv.screen_size();

    let selection_width = percentage(screen_size.x, ARTICLE_SELECTION_WIDTH_PERCENTAGE);
    let selection_height = percentage(screen_size.y, ARTICLE_SELECTION_HEIGHT_PERCNETAGE);

    siv.add_layer(
        RootLayout::vertical(CONFIG.keybindings.clone())
            .child(language_search)
            .child(DummyView {})
            .child(
                language_selection
                    .with_name(ARTICLE_LANGUAGE_SELECTION_NAME)
                    .scrollable(),
            )
            .input(true)
            .with_name(POPUP_NAME)
            .with_panel()
            .title("Switch Article Language")
            .fixed_size((selection_width, selection_height)),
    )
}
