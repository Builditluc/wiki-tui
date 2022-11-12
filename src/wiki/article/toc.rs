/// A struct representing the table of contents of an article
#[derive(PartialEq, Debug, Clone)]
pub struct TableOfContents {
    /// The title of the table of contents
    title: String,
    /// The items of the table of contents
    items: Vec<TableOfContentsItem>,
}

impl TableOfContents {
    /// Creates a new table of contents from a given title and array of items
    pub fn new(title: String, items: Vec<TableOfContentsItem>) -> Self {
        Self { title, items }
    }

    /// The title of the table of contents
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Iterate over all of the items in the table of contents
    pub fn items(&self) -> impl Iterator<Item = &TableOfContentsItem> {
        self.items.iter()
    }
}

/// An item of from a table of contents
#[derive(PartialEq, Debug, Clone)]
pub struct TableOfContentsItem {
    /// The number (level) of the item in the table of contents
    number: i32,
    /// The title of the item
    text: String,
    /// The sub items of this item, if there are any
    sub_items: Option<Vec<TableOfContentsItem>>,
}

#[allow(dead_code)]
impl TableOfContentsItem {
    /// Create a new item from a given number, text and sub items
    pub fn new(number: i32, text: String, sub_items: Option<Vec<TableOfContentsItem>>) -> Self {
        Self {
            number,
            text,
            sub_items,
        }
    }

    /// The number(level) of the item in the table of contents
    pub fn number(&self) -> &i32 {
        &self.number
    }

    /// The title of the item
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Iterate over the sub items of this item if there are any
    pub fn sub_items(&self) -> Option<impl Iterator<Item = &TableOfContentsItem>> {
        self.sub_items.as_ref().map(|sub_items| sub_items.iter())
    }
}
