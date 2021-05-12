use serde::*;

#[derive(Deserialize, Debug)]
pub struct ArticleResponse {
    pub query: QueryArticle,
}

#[derive(Deserialize, Debug)]
pub struct QueryArticle {
    pub pages: Vec<ArticleResult>,
}

#[derive(Deserialize, Debug)]
pub struct ArticleResult {
    #[serde(rename = "pageid")]
    pub page_id: i32,
    pub title: String,
    #[serde(rename = "extract")]
    pub content: String,
}

pub struct Article {
    pub elements: Vec<ArticleElement>,
}

pub struct ArticleElement {
    pub content: String,
    pub element_type: ArticleElementType,
    pub link_target: Option<String>,
}

pub enum ArticleElementType {
    Link,
    Text,
    Header,
}
