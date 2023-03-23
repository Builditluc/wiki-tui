use std::{cell::RefCell, rc::Rc};

use cursive::{view::Nameable, views::EditView, Cursive};

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
        c.borrow_mut().api_config.language = lang.into();
    });
}

/// Displays a popup that lets the user chosse a new language
pub fn language_selection_popup(siv: &mut Cursive) {
    if siv.find_name::<RootLayout>(POPUP_NAME).is_some() {
        siv.pop_layer();
        return;
    }

    let language_search = EditView::new();

    let mut language_selection: SelectView<String> =
        SelectView::new().on_submit(|s, item: &String| {
            s.pop_layer();
            change_lang(s, item.as_str());
        });
    language_selection.add_all(LANGUAGES.iter().map(|l| (l.name(), l.code().to_owned())));
    siv.add_layer(
        RootLayout::vertical(CONFIG.keybindings.clone())
            .child(language_search)
            .child(
                language_selection
                    .scrollable()
                    .with_name(LANGUAGE_SELECTION_NAME),
            )
            .input(true)
            .with_name(POPUP_NAME)
            .with_panel()
            .title("Change Language"),
    );
}
