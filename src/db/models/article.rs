use crate::db::schema::articles;

use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Associations, Identifiable, Debug)]
#[table_name="articles"]
pub struct Article {
    pub id: i32,
    pub article_id: String,
    pub title: String,
    pub text: String,

    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="articles"]
pub struct NewArticle<'a> {
    pub id: &'a i32,
    pub article_id: &'a str,
    pub title: &'a str,
    pub text: &'a str,

    pub updated_at: &'a NaiveDateTime
}

type AllColumns = (
    articles::id,
    articles::article_id,
    articles::title,
    articles::text,
    articles::updated_at
);

const ALL_COLUMNS: AllColumns = (
    articles::id,
    articles::article_id,
    articles::title,
    articles::text,
    articles::updated_at
);

type All = diesel::dsl::Select<articles::table, AllColumns>;
type WithId<'a> = diesel::dsl::Eq<articles::article_id, &'a str>;
type ById<'a> = diesel::dsl::Filter<All, WithId<'a>>;

fn with_id(article_id: &str) -> WithId { articles::article_id.eq(article_id) }

impl Article {
    pub fn all() -> All { articles::table.select(ALL_COLUMNS) }
    pub fn by_id(id: &str) -> ById { Self::all().filter(with_id(id)) }
}