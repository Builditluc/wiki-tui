use std::{cell::RefCell, rc::Rc};

use cursive::{
    view::Nameable,
    views::{DummyView, EditView},
    Cursive,
};

use crate::{
    config::{Config, CONFIG},
    wiki::language::{Language, LANGUAGES},
};

use super::{
    panel::WithPanel,
    scroll_view::Scrollable,
    views::{RootLayout, SelectView},
};

const POPUP_NAME: &str = "language_selection_popup";
const LANGUAGE_SELECTION_NAME: &str = "language_selection";

fn change_lang(siv: &mut Cursive, lang: impl Into<Language>) {
    siv.with_user_data(|c: &mut Rc<RefCell<Config>>| {
        let language = lang.into();
        info!("changing the language to '{:?}'", language);
        c.borrow_mut().api_config.language = language;
    });
}

/// Displays a popup that lets the user chosse a new language
pub fn language_selection_popup(siv: &mut Cursive) {
    if siv.find_name::<RootLayout>(POPUP_NAME).is_some() {
        siv.pop_layer();
        return;
    }

    let language_search = EditView::new().on_edit(|siv, text, _| {
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
    });

    let mut language_selection: SelectView<String> =
        SelectView::new().on_submit(|s, item: &String| {
            s.pop_layer();
            change_lang(s, item.as_str());
        });

    language_selection.add_all(LANGUAGES.iter().map(|l| (l.name(), l.code().to_owned())));

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
            .title("Change Language"),
    );
}
