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