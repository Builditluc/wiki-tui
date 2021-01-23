use crate::db::api::AllPagesRes;

pub trait ArticlesResultCallback {
    fn on_req_start (&self, req_no: i32) {
        debug!("Started the Article Request {}", req_no)
    }
    fn on_req_finish (&self, res: AllPagesRes);
    fn on_all_finished (&self, req_no: i32) {
        debug!("Finished all Article Requests with a total of {} requests", req_no)
    }
}

pub mod wiki {
    use uuid::Uuid;
    use crate::db::models::article_index::ArticleIndex;

    pub trait Fetchable {
        fn get_all_articles (&self) -> Vec<ArticleIndex>;
        fn get_article_by_id (&self, article_id: &Uuid) -> ArticleIndex;
        fn get_article_by_page_id (&self, page_id: &i32) -> ArticleIndex;
        fn get_article_by_title (&self, title: &String) -> ArticleIndex;
        fn get_article_with_title (&self, title: &String) -> Vec<ArticleIndex>;

        //TODO: Make this function return an Article
        fn fetch_article (&self, index: &ArticleIndex);
    }
    pub trait Updatable {
        fn update_all_articles (&self);
        fn update_article (&self, article: ArticleIndex);
    }
    pub trait Removable {
        fn delete_all_articles (&self);
        fn delete_article (&self, article: ArticleIndex);
    }
}
