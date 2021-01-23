use crate::db::api;
use crate::traits::wiki::*;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::db::models::article_index::ArticleIndex;
use uuid::Uuid;

//TODO: Make WikiSQL implement the Wiki traits
pub struct WikiSql {
    connection: SqliteConnection,
    api: api::Api,
}

impl WikiSql {
    pub fn new() -> Self {
        let new = Self { connection: Self::establish_connection(), api: api::Api::new() };
        debug!("Successfully created a new instance of db::wiki::wiki");
        return new;
    }

    fn establish_connection() -> SqliteConnection {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        info!("Successfully connected to {}", database_url);

        return connection;
    }
}

impl Fetchable for WikiSql {
    //TODO: Implement get_all_articles for WikiSql
    fn get_all_articles(&self) -> Vec<ArticleIndex> {
        unimplemented!()
    }

    //TODO: Implement get_article_by_id for WikiSql
    fn get_article_by_id(&self, article_id: &Uuid) -> ArticleIndex {
        unimplemented!()
    }

    //TODO: Implement get_article_by_page_id for WikiSql
    fn get_article_by_page_id(&self, page_id: &i32) -> ArticleIndex {
        unimplemented!()
    }

    //TODO: Implement get_article_by_title for WikiSql
    fn get_article_by_title(&self, title: &String) -> ArticleIndex {
        unimplemented!()
    }

    //TODO: Implement get_article_with_title for WikiSql
    fn get_article_with_title(&self, title: &String) -> Vec<ArticleIndex> {
        unimplemented!()
    }

    //TODO: Implement fetch_article for WikiSql
    fn fetch_article(&self, index: &ArticleIndex) {
        unimplemented!()
    }
}