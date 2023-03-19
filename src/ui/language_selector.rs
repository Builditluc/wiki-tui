use cursive::Cursive;

use crate::{
    config::{Config, CONFIG},
    wiki::language::{Language, LANGUAGES},
};

use super::{
    panel::WithPanel,
    views::{RootLayout, SelectView},
};

/// Displays a popup that lets the user chosse a new language
pub fn language_selection_popup(siv: &mut Cursive) {
    let mut language_selection: SelectView<&Language> =
        SelectView::new().on_submit(|s, item: &Language| {
            s.pop_layer();
            if let Some(mut config) = s.take_user_data::<Config>() {
                config.api_config.language = item.to_owned();
                return s.set_user_data::<Config>(config);
            }
            warn!("failed updating the language: configuration not found")
        });
    language_selection.add_all(LANGUAGES.iter().map(|l| (l.name(), l)));
    siv.add_layer(
        RootLayout::vertical(CONFIG.keybindings.clone())
            .child(language_selection)
            .input(true)
            .with_panel()
            .title("Change Language"),
    );
}
