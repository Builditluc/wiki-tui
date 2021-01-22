#![allow(dead_code)]
use dotenv;
use std::env;
use serde;
use serde::Deserialize;
use reqwest;
use reqwest::blocking::Response;

#[derive(Deserialize, Debug)]
struct AllPagesArticle {
    pageid: i32,
    ns: i32,
    title: String
}

#[derive(Deserialize, Debug)]
pub struct AllPagesContinue {
    #[serde(rename="apcontinue")]
    continue_id: String,
    #[serde(rename="continue")]
    continue_code: String
}

#[derive(Deserialize, Debug)]
struct AllPagesLimits {
    allpages: i32
}

#[derive(Deserialize, Debug)]
struct AllPagesArticles {
    allpages: Vec<AllPagesArticle>
}

#[derive(Deserialize, Debug)]
pub struct AllPagesRes {
    batchcomplete: String,
    #[serde(rename="continue")]
    continue_code: AllPagesContinue,
    limits: AllPagesLimits,
    query: AllPagesArticles,
}

impl AllPagesRes {
    pub fn is_continue(&self) -> bool {
        if self.continue_code.continue_id.is_empty() && self.continue_code.continue_code.is_empty() {
            return false
        }
        true
    }

    pub fn get_continue(&self) -> (&String, &String) {
        (&self.continue_code.continue_id, &self.continue_code.continue_code)
    }
}

pub trait ArticlesResultCallback {
    fn on_req_start (&self, req_no: i32) {
        debug!("Started the Article Request {}", req_no)
    }
    fn on_req_finish (&self, res: AllPagesRes);
    fn on_all_finished (&self, req_no: i32) {
        debug!("Finished all Article Requests with a total of {} requests", req_no)
    }
}

pub struct Api {
    client: reqwest::blocking::Client,
    base_url: String,
    ap_limit: String
}

impl Api {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        debug!("Created a new instance of Api");
        Api {
            client: reqwest::blocking::Client::new(),
            base_url: env::var("BASE_URL")
                .expect("BASE_URL must be set"),
            ap_limit: env::var("AP_LIMIT")
                .expect("AP_LIMIT must be set"),

        }
    }
}

impl Api {
    pub fn fetch_all_articles(&self, callback: Box<dyn ArticlesResultCallback>) {
        let mut n = 0;
        let mut continue_code: String = "None".to_string();
        let mut response;
        let mut is_continue;

        loop {
            callback.on_req_start(n);

            response = self.fetch_articles(Some(continue_code))
                .unwrap()
                .json::<AllPagesRes>()
                .unwrap();
            continue_code = response.get_continue().0.to_string();
            is_continue = response.is_continue();

            if !is_continue {
                break;
            }

            callback.on_req_finish(response);
            n += 1;
        }

        callback.on_all_finished(n);
    }

    fn fetch_articles(&self, continue_id: Option<String>) -> reqwest::Result<Response> {
        let mut request_url: String;
        if continue_id != Some("None".to_string()) {
            request_url = format!("{}?action=query&list=allpages&aplimit={}&apcontinue={}&format=json", &self.base_url, &self.ap_limit, continue_id.unwrap());
        } else {
            request_url = format!("{}?action=query&list=allpages&aplimit={}&format=json", &self.base_url, &self.ap_limit);
        }

        self.client.get(&request_url).send()
    }
}