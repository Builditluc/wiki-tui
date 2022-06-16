use cursive::{views::LinearLayout, Cursive};

/// Removes a given view from a given layout. If the view or the layout couldn't be found, the
/// function fails silently
pub fn remove_view_from_layout(siv: &mut Cursive, view_name: &str, layout_name: &str) {
    let result = siv.call_on_name(layout_name, |view: &mut LinearLayout| {
        if let Some(i) = view.find_child_from_name(view_name) {
            log::debug!("removed '{}' from '{}'", view_name, layout_name);
            view.remove_child(i);
        }
    });
    if result.is_none() {
        log::warn!("couldn't find a layout with the name '{}'", layout_name);
    }
}

/// Wraps a view into a ThemedView with the given theme. If the macro is used without a theme,
/// it'll just apply the default one to the view
#[macro_export]
macro_rules! view_with_theme {
    ($theme: expr, $view: expr) => {
        if let Some(theme) = $theme.as_ref() {
            ui::ThemedView::new(theme.to_theme(), $view)
        } else {
            ui::ThemedView::new(config::CONFIG.theme.to_theme(), $view)
        }
    };
}

/// Wraps a view into a OnEventView that overrides the keybindings of the view with the ones
/// defined in the config
#[macro_export]
macro_rules! override_keybindings {
    ($view: expr) => {{
        let mut event_view = cursive::views::OnEventView::new($view);

        // go through all of the keybindings
        if let Some(event_key) = config::CONFIG.keybindings.down.clone() {
            log::debug!("registered the '{:?}' key as Down", event_key);
            event_view.set_on_pre_event(event_key, |siv: &mut cursive::Cursive| {
                siv.on_event(cursive::event::Event::Key(cursive::event::Key::Down))
            });
        }

        if let Some(event_key) = config::CONFIG.keybindings.up.clone() {
            log::debug!("registered the '{:?}' key as Up", event_key);
            event_view.set_on_pre_event(event_key, |siv: &mut cursive::Cursive| {
                siv.on_event(cursive::event::Event::Key(cursive::event::Key::Up))
            });
        }

        if let Some(event_key) = config::CONFIG.keybindings.left.clone() {
            log::debug!("registered the '{:?}' key as Left", event_key);
            event_view.set_on_pre_event(event_key, |siv: &mut cursive::Cursive| {
                siv.on_event(cursive::event::Event::Key(cursive::event::Key::Left))
            });
        }

        if let Some(event_key) = config::CONFIG.keybindings.right.clone() {
            log::debug!("registered the '{:?}' key as Right", event_key);
            event_view.set_on_pre_event(event_key, |siv: &mut cursive::Cursive| {
                siv.on_event(cursive::event::Event::Key(cursive::event::Key::Right))
            });
        }

        event_view
    }};
}
