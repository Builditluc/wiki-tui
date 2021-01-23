use crate::db::api;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

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