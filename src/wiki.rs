use serde::*;
use ini::{Properties, Ini};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SearchResponse;
pub struct Wiki {
    client: reqwest::blocking::Client,
    api_config: Properties
}

impl Wiki {
    pub fn new() -> Self {
        let config = Ini::load_from_file("config.ini").unwrap();
        let config = config.clone();
        Wiki {
            client: reqwest::blocking::Client::new(),
            api_config: config.section(Some("Api")).unwrap().clone()
        }
    }

    fn search(&self, title: &str) {
        let base_url = &self.api_config
            .get("BASE_URL");
        let url = format!("{}?action=query&list=searcg&srwhat=text&srsearch={}&format=json", base_url.unwrap(), title);

        let result = self.client.get(&url)
            .send()
            .unwrap();
    }
}
