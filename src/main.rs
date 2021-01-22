#[macro_use] extern crate log;

pub mod db;
pub mod tui;
pub mod tests;
pub mod logging;

use crate::db::api::{AllPagesRes, ArticlesResultCallback};
struct TestCallback;

impl TestCallback {
    pub fn new() -> Self {
        Self {}
    }
}

impl db::api::ArticlesResultCallback for TestCallback {
    fn on_req_start(&self, req_no: i32) {
        println!("Started request {}", req_no);
    }

    fn on_req_finish(&self, res: AllPagesRes) {
        println!("Finished the request");
    }

    fn on_all_finished(&self, req_no: i32) {
        println!("Finished all {} requests", req_no);
    }
}

fn main() {
    logging::Logger::new();

    let api = db::api::Api::new();
    api.fetch_all_articles(Box::new(TestCallback::new()));

}