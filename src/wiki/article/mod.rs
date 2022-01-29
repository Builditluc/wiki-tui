pub mod compiled_article;
pub mod element;
pub mod toc;

// Temporary
use crate::ui;
use serde::Deserialize;

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

#[derive(Clone)]
pub struct Article {
    pub elements: Vec<ArticleElement>,
}

#[derive(Clone)]
pub struct ArticleElement {
    pub content: String,
    pub element_type: ArticleElementType,
    pub link_target: Option<String>,
}

#[derive(Clone)]
pub enum ArticleElementType {
    Link,
    Text,
    Header,
    Bold,
    Italic,
}

#[derive(Clone)]
pub struct ParsedArticle {
    pub article: Article,
    pub toc: Option<ui::models::table_of_contents::Table>,
}
