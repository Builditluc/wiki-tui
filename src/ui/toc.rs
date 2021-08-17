use crate::*;
use ui::models::table_of_contents;

pub fn add_table_of_contents(siv: &mut Cursive, toc: ui::models::table_of_contents::Table) {
    // get the article_layout and create an empty select view
    let mut article_layout = siv.find_name::<LinearLayout>("article_layout").unwrap();
    let mut toc_view = SelectView::<table_of_contents::Item>::new().on_submit(|siv, item| {
        use crate::ui::article::ArticleView;
        let item_index = match siv.find_name::<SelectView<table_of_contents::Item>>("toc_view") {
            Some(view) => {
                let mut index: usize = 0;
                for (idx, _item) in view.iter().enumerate() {
                    if _item.1.text == item.text {
                        index = idx;
                        break;
                    }
                }
                index
            }
            None => 0 as usize,
        };

        log::info!("item_index: {}", item_index);
        if let Some(mut view) = siv.find_name::<ArticleView>("article_view") {
            view.select_header(item_index)
        }
    });

    // now go through every item
    log::info!("Adding the table of content to the toc_view");
    for item in toc.items.into_iter() {
        add_item_to_toc(&mut toc_view, item);
    }

    article_layout.insert_child(
        1,
        Dialog::around(toc_view.with_name("toc_view").full_height()).title(toc.title),
    );
    article_layout.set_weight(1, 10);
    log::info!("Added the toc_view to the article_layout");
}

fn add_item_to_toc(
    toc_view: &mut SelectView<ui::models::table_of_contents::Item>,
    item: ui::models::table_of_contents::Item,
) {
    // add the item to the select_view
    let label = format!("{}{}", " ".repeat(item.number as usize), item.text);
    log::debug!("Added the item: {} to the toc_view", label);
    toc_view.add_item(label, item);
}
