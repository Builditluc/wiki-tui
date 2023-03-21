use std::{cell::RefCell, rc::Rc};

use cursive::{view::Nameable, Cursive};

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

/// Displays a popup that lets the user chosse a new language
pub fn language_selection_popup(siv: &mut Cursive) {
    if siv.find_name::<RootLayout>(POPUP_NAME).is_some() {
        siv.pop_layer();
        return;
    }

    let mut language_selection: SelectView<&Language> =
        SelectView::new().on_submit(|s, item: &Language| {
            s.pop_layer();
            s.with_user_data(|c: &mut Rc<RefCell<Config>>| {
                c.borrow_mut().api_config.language = item.to_owned();
            });
            warn!("failed updating the language: configuration not found")
        });
    language_selection.add_all(LANGUAGES.iter().map(|l| (l.name(), l)));
    siv.add_layer(
        RootLayout::vertical(CONFIG.keybindings.clone())
            .child(language_selection.scrollable())
            .input(true)
            .with_name(POPUP_NAME)
            .with_panel()
            .title("Change Language"),
    );
}
