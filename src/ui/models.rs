use crate::wiki::search::SearchResult;

#[derive(Clone)]
pub struct ArticleResultPreview {
    pub page_id: i32,
    pub snippet: String,
    pub title: String,
}

pub struct Article {
    pub page_id: i32,
    pub title: String,
    pub content: String,
}

impl From<SearchResult> for ArticleResultPreview {
    fn from(search_result: SearchResult) -> Self {
        ArticleResultPreview {
            page_id: search_result.page_id,
            snippet: search_result.snippet,
            title: search_result.title,
        }
    }
}
