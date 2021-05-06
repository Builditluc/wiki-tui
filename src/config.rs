use anyhow::*;
use dirs;
use ini::Ini;
use lazy_static::*;
use reqwest;
use std::fs;

const CONFIG_FILE_NAME: &str = "config.ini";
const APP_CONFIG_DIR: &str = "wiki-tui";

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

struct UserApiConfig {
    base_url: Option<String>,
}

pub struct ApiConfig {
    pub base_url: String,
}

pub struct Config {
    pub api_config: ApiConfig,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config {
            api_config: ApiConfig {
                base_url: "https://en.wikipedia.org/w/api.php".to_string(),
            },
        };

        // do the loading stuff here

        config
    }
}
