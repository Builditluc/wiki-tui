use anyhow::*;
use cursive::theme::BaseColor;
use cursive::theme::Color;
use dirs;
use ini::Ini;
use lazy_static::*;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.ini";
const APP_CONFIG_DIR: &str = "wiki-tui";

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

pub struct Theme {
    pub text: Color,
    pub title: Color,
    pub highlight: Color,
    pub background: Color,
    pub search_match: Color,
    pub highlight_text: Color,
    pub highlight_inactive: Color,
}

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub base_url: String,
}

pub struct Config {
    pub api_config: ApiConfig,
    pub theme: Theme,
    config_path: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        // initialize the struct with the defaults
        let mut config = Config {
            api_config: ApiConfig {
                base_url: "https://en.wikipedia.org/".to_string(),
            },
            theme: Theme {
                background: Color::Dark(BaseColor::White),
                title: Color::Dark(BaseColor::Red),
                highlight: Color::Dark(BaseColor::Red),
                highlight_inactive: Color::Dark(BaseColor::Blue),
                highlight_text: Color::Dark(BaseColor::White),
                text: Color::Dark(BaseColor::Black),
                search_match: Color::Dark(BaseColor::Red),
            },
            config_path: PathBuf::new(),
        };

        // do the loading stuff here
        log::info!("Loading the config");
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
            log::warn!("Failed loading the config paths, {:?}", config_exists.err());
            return;
        }

        // read the config file and check if there were any errors
        let config = match Ini::load_from_file(&self.config_path).context(format!(
            "Failed loading the config file at the location: {}",
            &self.config_path.to_str().unwrap_or("NONE")
        )) {
            Ok(config) => {
                log::info!("Successfully loaded the config file");
                config
            }
            Err(error) => {
                log::warn!("{:?}", error);
                return;
            }
        };

        // if the config file exists, then load it
        if config_exists.unwrap() {
            log::debug!("Loading the Config");
            self.load_api_config(&config);
            self.load_theme(&config);
        }
    }

    fn load_or_create_config_paths(&mut self) -> Result<bool> {
        // get the platform specific config directory
        let config_dir = match dirs::config_dir() {
            Some(config_dir) => {
                log::info!("The config directory is {}", config_dir.to_str().unwrap());
                config_dir
            }
            None => {
                log::error!("Couldn't find the config directory");
                panic!()
            }
        };

        // build the app config path and the config file path
        let app_config_dir = config_dir.join(APP_CONFIG_DIR);
        let config_file_dir = app_config_dir.join(CONFIG_FILE_NAME);

        // create the app config folder if it doesn't exist
        if !app_config_dir.exists() {
            log::debug!("The app config directory doesn't exist, creating it now");
            match fs::create_dir(app_config_dir).context("Couldn't create the app config directory")
            {
                Ok(_) => {
                    log::debug!("Successfully created the app config directory");
                }
                Err(error) => return Err(error),
            };
        }

        // check, if the config file exists
        if !config_file_dir.exists() {
            log::info!("The config file doesn't exist");
            return Ok(false);
        }

        // if the config file exists,
        // return true and store the path to it
        log::debug!("The config file exists");
        self.config_path = config_file_dir;
        Ok(true)
    }

    fn load_api_config(&mut self, config: &Ini) {
        // get the api_config section
        let api_config = match config.section(Some("Api")) {
            Some(api_config) => {
                log::debug!("Found the Api Config");
                api_config
            }
            None => {
                log::debug!("Api Config not found");
                return;
            }
        };

        // now load the settings
        log::debug!("Trying to load the BASE_URL");
        if api_config.get("BASE_URL").is_some() {
            self.api_config.base_url = api_config.get("BASE_URL").unwrap().to_string();
            log::debug!("Loaded the BASE_URL");
        }
    }

    fn load_theme(&mut self, config: &Ini) {
        // get the theme section
        let theme = match config.section(Some("Theme")) {
            Some(theme) => {
                log::debug!("Found the Theme Config");
                theme
            }
            None => {
                log::debug!("Theme Config not found");
                return;
            }
        };

        // define the macro for loading individual color settings
        macro_rules! to_theme_color {
            ($color: ident) => {
                log::debug!("Trying to load the setting '{}'", stringify!($color));
                if theme.get(stringify!($color)).is_some() {
                    match parse_color(theme.get(stringify!($color)).unwrap().to_string()) {
                        Ok(color) => {
                            self.theme.$color = color;
                            log::debug!("Loaded the setting '{}'", stringify!($color));
                        }
                        Err(error) => {
                            log::warn!("{}", error);
                        }
                    };
                }
            };
        }

        // now load the settings
        to_theme_color!(text);
        to_theme_color!(title);
        to_theme_color!(highlight);
        to_theme_color!(background);
        to_theme_color!(search_match);
        to_theme_color!(highlight_text);
        to_theme_color!(highlight_inactive);
    }
}

fn parse_color(color: String) -> Result<Color> {
    Color::parse(&color.to_lowercase()).context("Failed loading the color")
}
