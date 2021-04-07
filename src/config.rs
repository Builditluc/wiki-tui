use dirs;
use ini::Ini;
use std::fs;
use reqwest;

const FILE_NAME: &str = "config.ini";
const APP_CONFIG_DIR: &str = "wiki-tui";

pub struct Theme;
pub struct Config {
    pub theme: Theme,
    pub config_path: Option<std::path::PathBuf>,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {}
    }
}

impl Config {
    pub fn new() -> Self {
        Config {
            theme: Default::default(),
            config_path: None
        }
    }

    fn get_or_create_config_paths(&mut self) {
        match dirs::config_dir() {
            Some(config_path) => {
                let app_config_path = config_path.join(APP_CONFIG_DIR);
                let config_file_path = app_config_path.join(FILE_NAME);

                if !app_config_path.exists() {
                    fs::create_dir(app_config_path);
                }

                if !config_file_path.exists() {
                    // download the config file and then write it
                    let file_url = "https://raw.githubusercontent.com/Builditluc/wiki-tui/stable/config.ini";
                    let file_content = reqwest::blocking::get(file_url).unwrap().text().unwrap();

                    fs::write(config_file_path.clone(), file_content);
                }

                self.config_path = Some(config_file_path);
            }
            None => {
                panic!("Couldn't find your config directory")
            }
        }
    }
    
    pub fn load(&mut self) {
        self.get_or_create_config_paths();
    } 
}
