use cursive::{views::LinearLayout, Cursive};

pub fn remove_view_from_layout(siv: &mut Cursive, view_name: &str, layout_name: &str) {
    siv.call_on_name(layout_name, |view: &mut LinearLayout| {
        if let Some(i) = view.find_child_from_name(view_name) {
            view.remove_child(i);
        }
    });
}

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
