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
