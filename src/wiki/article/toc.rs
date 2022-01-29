pub struct TableOfContents {
    title: String,
    items: Vec<TableOfContentsItem>,
}

impl TableOfContents {
    pub fn new(title: String, items: Vec<TableOfContentsItem>) -> Self {
        Self { title, items }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn items(&self) -> impl Iterator<Item = &TableOfContentsItem> {
        self.items.iter()
    }
}

pub struct TableOfContentsItem {
    number: i32,
    text: String,
    sub_items: Option<Vec<TableOfContentsItem>>,
}

impl TableOfContentsItem {
    pub fn new(number: i32, text: String, sub_items: Option<Vec<TableOfContentsItem>>) -> Self {
        Self {
            number,
            text,
            sub_items,
        }
    }

    pub fn number(&self) -> &i32 {
        &self.number
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn sub_items(&self) -> Option<impl Iterator<Item = &TableOfContentsItem>> {
        self.sub_items.as_ref().map(|sub_items| sub_items.iter())
    }
}
