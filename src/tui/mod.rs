use crate::traits::wiki::Wikipedia;
use crate::db::wiki::WikiSql;

pub struct WikipediaUi {
    wiki: Box<dyn Wikipedia>
}

impl WikipediaUi {
    pub fn new() -> Self {
        WikipediaUi {
            wiki: WikiSql::new_boxed()
        }
    }

    pub fn run() {
        let mut siv = cursive::default();
    }
}