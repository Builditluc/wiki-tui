use anyhow::*;
use dirs;
use ini::Ini;
use lazy_static::*;
use reqwest;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.ini";
const APP_CONFIG_DIR: &str = "wiki-tui";

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub base_url: String,
}

pub struct Config {
    pub api_config: ApiConfig,
    config_path: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        // initialize the struct with the defaults
        let mut config = Config {
            api_config: ApiConfig {
                base_url: "https://en.wikipedia.org/w/api.php".to_string(),
            },
            config_path: PathBuf::new(),
        };

        // do the loading stuff here
        config.load_config();

        // return the config
        config
    }

    fn load_config(&mut self) {
        // load (or create if they not exist) the config path(s)
        // this function returns true if the config file exists and false if not
        let config_exists = self.load_or_create_config_paths();

        // check, if any errors occured during loading
        if config_exists.is_err() {
            // Abort the loading
            return;
        }

        // read the config file and check if there were any errors
        let config = match Ini::load_from_file(&self.config_path) {
            Ok(config) => config,
            Err(_) => return,
        };

        // if the config file exists, then load it
        if config_exists.is_ok() && config_exists.unwrap() {
            self.load_api_config(&config);
        }
    }

    fn load_or_create_config_paths(&mut self) -> Result<bool> {
        // get the platform specific config directory
        let config_dir = match dirs::config_dir() {
            Some(config_dir) => config_dir,
            None => panic!("Couldn't find your config directory"),
        };

        // build the app config path and the config file path
        let app_config_dir = config_dir.join(APP_CONFIG_DIR);
        let config_file_dir = app_config_dir.join(CONFIG_FILE_NAME);

        // create the app config folder if it doesn't exist
        if !app_config_dir.exists() {
            fs::create_dir(app_config_dir);
        }

        // check, if the config file exists
        if !config_file_dir.exists() {
            return Ok(false);
        }

        // if the config file exists,
        // return true and store the path to it
        self.config_path = config_file_dir;
        Ok(true)
    }

    fn load_api_config(&mut self, config: &Ini) {
        // get the api_config section
        let api_config = match config.section(Some("Api")) {
            Some(api_config) => api_config,
            None => return,
        };

        // now load the settings
        if api_config.get("BASE_URL").is_some() {
            self.api_config.base_url = api_config.get("BASE_URL").unwrap().to_string();
        }
    }
}
