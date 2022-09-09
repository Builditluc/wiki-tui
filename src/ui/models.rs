#[derive(Clone)]
pub struct ArticleResultPreview {
    pub page_id: i32,
    pub snippet: String,
    pub title: String,
}

impl From<i32> for ArticleResultPreview {
    fn from(page_id: i32) -> Self {
        ArticleResultPreview {
            page_id,
            snippet: String::new(),
            title: String::new(),
        }
    }
}

pub mod table_of_contents {
    #[derive(Clone, Debug)]
    pub struct Table {
        pub title: String,
        pub items: Vec<Item>,
    }

    #[derive(Clone, Debug)]
    pub struct Item {
        pub number: i32,
        pub text: String,
        pub sub_items: Option<Vec<Item>>,
    }
}
