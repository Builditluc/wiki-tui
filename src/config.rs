use dirs;
use ini::Ini;
use std::fs;
use reqwest;

const CONFIG_FILE_NAME: &str = "config.ini";
const APP_CONFIG_DIR: &str = "wiki-tui";

pub struct Config {
    pub config_path: Option<std::path::PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        let mut config: Config = Default::default();
        
        let config_file_exists = config.get_config_file();
        if !config_file_exists {
            config.create_config_file();
        }

        let config_file_valid = config.is_config_valid();
        if !config_file_valid {
            config.create_config_file();
        }

        config.load();
        config
    }

    fn get_config_file(&mut self) -> bool {
        match dirs::config_dir() {
            Some(config_path) => {
                let app_dir_path = config_path.join(APP_CONFIG_DIR);
                let config_file_path = app_dir_path.join(CONFIG_FILE_NAME);

                self.config_path = Some(config_file_path.clone());

                if !app_dir_path.exists() {
                    fs::create_dir(app_dir_path);
                    return false
                }

                if !config_file_path.exists() {
                    return false
                }

                true
            }

            None => {
                panic!("Couldn't find your config directory")
            }
        }
    }

    fn create_config_file(&mut self) {
        let file_url = "https://raw.githubusercontent.com/Builditluc/wiki-tui/stable/config.ini";
        let file_content = reqwest::blocking::get(file_url).unwrap().text().unwrap();

        fs::write(&self.config_path.clone().unwrap(), file_content);
    }

    fn is_config_valid(&mut self) -> bool {
        let config = Ini::load_from_file(&self.config_path.clone().unwrap());

        true 
    }

    fn load(&mut self) {
        let config = Ini::load_from_file(&self.config_path.clone().unwrap());

        // ...
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_path: None
        }
    }
}
