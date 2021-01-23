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
    fn get_all_articles(&self) -> Vec<ArticleIndex> {
        let result = ArticleIndex::all()
            .load::<ArticleIndex>(&self.connection);

        if result.is_ok() {
            debug!("Successfully selected every article from the ArticleIndex");
            return result.unwrap();
        }

        error!("Failed to select every article from the ArticleIndex");
        panic!("An unexpected error occurred\n Please check the logs")
    }

    fn get_article_by_id(&self, article_id: &Uuid) -> ArticleIndex {
        let result = ArticleIndex::by_id(&article_id.to_string())
            .first::<ArticleIndex>(&self.connection);

        if result.is_ok() {
            debug!("{}", format!("Successfully selected the article {} from the ArticleIndex", article_id.to_string()));
            return result.unwrap();
        }

        error!("{}", format!("Failed to select the article {} from the ArticleIndex", article_id.to_string()));
        panic!("An unexpected error occurred\n Please check the logs")
    }

    fn get_article_by_page_id(&self, page_id: &i32) -> ArticleIndex {
        let result = ArticleIndex::by_page_id(page_id)
            .first::<ArticleIndex>(&self.connection);

        if result.is_ok() {
            debug!("{}", format!("Successfully selected the article {} from the ArticleIndex", page_id));
            return result.unwrap();
        }

        error!("{}", format!("Failed to select the article {} from the ArticleIndex", page_id));
        panic!("An unexpected error occurred\n Please check the logs")
    }

    fn get_article_by_title(&self, title: &String) -> Vec<ArticleIndex> {
        let result = ArticleIndex::by_title(title)
            .load::<ArticleIndex>(&self.connection);

        if result.is_ok() {
            debug!("{}", format!("Successfully selected multiple articles with the name {} from the ArticleIndex", title));
            return result.unwrap();
        }

        error!("{}", format!("Failed to select multiple articles with the name {} from the ArticleIndex", title));
        panic!("An unexpected error occurred\n Please check the logs")
    }

    fn get_article_with_title(&self, title: &String) -> ArticleIndex {
        let result = ArticleIndex::by_title(title)
            .first::<ArticleIndex>(&self.connection);

        if result.is_ok() {
            debug!("{}", format!("Successfully selected an article with the name {} from the ArticleIndex", title));
            return result.unwrap();
        }

        error!("{}", format!("Failed to select an article with the name {} from the ArticleIndex", title));
        panic!("An unexpected error occurred \n Please check the logs")
    }

    //TODO: Implement fetch_article for WikiSql
    fn fetch_article(&self, index: &ArticleIndex) {
        unimplemented!()
    }
}

impl Updatable for WikiSql {
    //TODO: Implement update_all_articles for WikiSql
    fn update_all_articles(&self) {
        //TODO: Create a DefaultCallback Struct
        //let callback;

        debug!("Starting to fetch all articles");
        //&self.api.fetch_all_articles(Box::new(callback);
        debug!("Finished to fetch all articles");
    }

    //TODO: Implement update_article for WikiSql
    fn update_article(&self, article: ArticleIndex) {
        unimplemented!()
    }
}