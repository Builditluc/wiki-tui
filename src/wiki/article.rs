use cursive::utils::*;
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

#[derive(Clone)]
pub struct Article {
    pub title: String,
    pub content: markup::StyledString,
}

