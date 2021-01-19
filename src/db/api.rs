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
    continueCode: AllPagesContinue,
    limits: AllPagesLimits,
    query: AllPagesArticles,
}

impl AllPagesRes {
    pub fn is_continue(&self) -> bool {
        if self.continueCode.continue_id.is_empty() && self.continueCode.continue_code.is_empty() {
            return false
        }
        true
    }

    pub fn get_continue(&self) -> (&String, &String) {
        (&self.continueCode.continue_id, &self.continueCode.continue_code)
    }
}

#[allow(dead_code)]
pub struct Api {
    client: reqwest::blocking::Client,
    base_url: String,
    ap_limit: String
}

impl Api {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        Api {
            client: reqwest::blocking::Client::new(),
            base_url: env::var("BASE_URL")
                .expect("BASE_URL must be set"),
            ap_limit: env::var("AP_LIMIT")
                .expect("AP_LIMIT must be set")
        }
    }
}

#[allow(dead_code)]
impl Api {
    pub fn fetch_all_articles(&self) {
        let mut response = self.fetch_articles(None)
            .unwrap()
            .json::<AllPagesRes>()
            .unwrap();

        let mut n = 0;
        while response.is_continue() {
            response = self.fetch_articles(Some(response.get_continue().0))
                .unwrap()
                .json::<AllPagesRes>()
                .unwrap();
            for article in &response.query.allpages {
                println!("Found article {} with the title {}", article.pageid, article.title)
            }
        }
    }

    fn fetch_articles(&self, continue_id: Option<&String>) -> reqwest::Result<Response> {
        let mut request_url: String;
        if continue_id.is_some() {
            request_url = format!("{}?action=query&list=allpages&aplimit={}&apcontinue={}&format=json", &self.base_url, &self.ap_limit, continue_id.unwrap());
        } else {
            request_url = format!("{}?action=query&list=allpages&aplimit={}&format=json", &self.base_url, &self.ap_limit);
        }

        self.client.get(&request_url).send()
    }
}