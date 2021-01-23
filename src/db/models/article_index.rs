use crate::db::schema::article_index;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[table_name="article_index"]
pub struct ArticleIndex {
    pub id: i32,
    pub page_id: i32,
    pub article_id: i32,
    pub namespace: i32,
    pub title: String,

    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="article_index"]
pub struct NewArticleIndex<'a> {
    pub page_id: &'a i32,
    pub article_id: &'a i32,
    pub namespace: &'a i32,
    pub title: &'a str,

    pub updated_at: &'a NaiveDateTime
}