use anyhow::*;
use cursive::theme::BaseColor;
use cursive::theme::Color;
use lazy_static::*;
use log::LevelFilter;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use toml::from_str;

const CONFIG_FILE: &str = "config.toml";
const CONFIG_DIR: &str = ".config";
const APP_DIR: &str = "wiki-tui";

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

    pub search_bar: Option<ViewTheme>,
    pub search_results: Option<ViewTheme>,
    pub search_preview: Option<ViewTheme>,

    pub article_view: Option<ViewTheme>,
    pub toc_view: Option<ViewTheme>,
}

impl Theme {
    pub fn to_theme(&self) -> cursive::theme::Theme {
        cursive::theme::Theme {
            palette: {
                let mut custom_palette = cursive::theme::Palette::default();

                custom_palette.set_color("View", self.background);
                custom_palette.set_color("Background", self.background);
                custom_palette.set_color("Primary", self.text);
                custom_palette.set_color("TitlePrimary", self.title);
                custom_palette.set_color("Highlight", self.highlight);
                custom_palette.set_color("HighlightInactive", self.highlight_inactive);
                custom_palette.set_color("HighlightText", self.highlight_text);

                custom_palette
            },
            ..Default::default()
        }
    }
}

pub struct ViewTheme {
    // TODO: Add borders
    pub background: Color,
    pub text: Color,
    pub title: Color,
    pub secondary: Color,
    pub highlight: Color,
    pub highlight_text: Color,
    pub highlight_inactive: Color,
}

impl ViewTheme {
    pub fn to_theme(&self) -> cursive::theme::Theme {
        cursive::theme::Theme {
            palette: {
                let mut custom_palette = cursive::theme::Palette::default();

                custom_palette.set_color("View", self.background);
                custom_palette.set_color("Background", self.background);
                custom_palette.set_color("Primary", self.text);
                custom_palette.set_color("TitlePrimary", self.title);
                custom_palette.set_color("Highlight", self.highlight);
                custom_palette.set_color("HighlightInactive", self.highlight_inactive);
                custom_palette.set_color("HighlightText", self.highlight_text);

                custom_palette
            },
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct ApiConfig {
    pub base_url: String,
}

pub struct Logging {
    pub enabled: bool,
    pub log_dir: PathBuf,
    pub log_level: LevelFilter,
}

pub struct Config {
    pub api_config: ApiConfig,
    pub theme: Theme,
    pub logging: Logging,
    config_path: PathBuf,
}

#[derive(Deserialize, Debug)]
struct UserConfig {
    api: Option<UserApiConfig>,
    theme: Option<UserTheme>,
    logging: Option<UserLogging>,
}

#[derive(Deserialize, Debug)]
struct UserTheme {
    text: Option<String>,
    title: Option<String>,
    highlight: Option<String>,
    background: Option<String>,
    search_match: Option<String>,
    highlight_text: Option<String>,
    highlight_inactive: Option<String>,

    search_bar: Option<UserViewTheme>,
    search_results: Option<UserViewTheme>,
    search_preview: Option<UserViewTheme>,

    article_view: Option<UserViewTheme>,
    toc_view: Option<UserViewTheme>,
}

#[derive(Deserialize, Debug)]
struct UserViewTheme {
    text: Option<String>,
    title: Option<String>,
    highlight: Option<String>,
    background: Option<String>,
    highlight_text: Option<String>,
    highlight_inactive: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UserApiConfig {
    base_url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct UserLogging {
    enabled: Option<bool>,
    log_dir: Option<String>,
    log_level: Option<String>,
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

                search_bar: None,
                search_results: None,
                search_preview: None,

                article_view: None,
                toc_view: None,
            },
            logging: Logging {
                enabled: true,
                log_dir: PathBuf::from("wiki_tui.log"),
                log_level: LevelFilter::Info,
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
        let config_str = match fs::read_to_string(&self.config_path).context(format!(
            "Failed loading the config file at the location: {}",
            &self.config_path.to_str().unwrap_or("NONE")
        )) {
            Ok(config) => {
                log::info!("Successfully read the config file");
                config
            }
            Err(error) => {
                log::warn!("{:?}", error);
                return;
            }
        };

        let user_config = match from_str::<UserConfig>(&config_str)
            .context("Failed deserializing the loaded config file")
        {
            Ok(config) => {
                log::info!("Successfully deserialized config");
                config
            }
            Err(error) => {
                log::warn!("{:?}", error);
                return;
            }
        };

        if let Some(user_theme) = user_config.theme {
            self.load_theme(&user_theme);
        }

        if let Some(user_api_config) = user_config.api {
            self.load_api_config(&user_api_config);
        }

        if let Some(user_logging) = user_config.logging {
            self.load_logging(&user_logging);
        }
    }

    fn load_or_create_config_paths(&mut self) -> Result<bool> {
        // get the platform specific config directory
        let config_dir = match dirs::home_dir() {
            Some(config_dir) => {
                log::info!(
                    "The config directory is {}",
                    config_dir.join(CONFIG_DIR).to_str().unwrap()
                );
                config_dir.join(CONFIG_DIR)
            }
            None => {
                log::error!("Couldn't find the home directory");
                panic!()
            }
        };

        // build the app config path and the config file path
        let app_config_dir = config_dir.join(APP_DIR);
        let config_file_dir = app_config_dir.join(CONFIG_FILE);

        // create the app config folder if it doesn't exist
        if !app_config_dir.exists() {
            log::info!("The app config directory doesn't exist, creating it now");
            match fs::create_dir(app_config_dir).context("Couldn't create the app config directory")
            {
                Ok(_) => {
                    log::info!("Successfully created the app config directory");
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

    fn load_api_config(&mut self, user_api_config: &UserApiConfig) {
        // define the macro for loading individual api settings
        macro_rules! to_api_setting {
            ($setting: ident) => {
                log::debug!("Trying to load the setting '{}'", stringify!($setting));
                if user_api_config.$setting.is_some() {
                    self.api_config.$setting =
                        user_api_config.$setting.as_ref().unwrap().to_string();
                }
            };
        }

        to_api_setting!(base_url);
    }

    fn load_theme(&mut self, user_theme: &UserTheme) {
        // define the macro for loading individual color settings
        macro_rules! to_theme_color {
            ($color: ident) => {
                log::debug!("Trying to load the color '{}'", stringify!($color));
                if user_theme.$color.is_some() {
                    match parse_color(user_theme.$color.as_ref().unwrap().to_string()) {
                        Ok(color) => {
                            self.theme.$color = color;
                            log::debug!("Loaded the color '{}'", stringify!($color));
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

        if let Some(search_bar) = &user_theme.search_bar {
            let background_changed: bool = search_bar.background.is_some();

            let mut search_bar_theme = self.load_view_theme(search_bar);
            if background_changed {
                search_bar_theme.secondary = search_bar_theme.background;
            }

            self.theme.search_bar = Some(search_bar_theme);
        }

        if let Some(search_results) = &user_theme.search_results {
            self.theme.search_results = Some(self.load_view_theme(search_results));
        }

        if let Some(search_preview) = &user_theme.search_preview {
            self.theme.search_preview = Some(self.load_view_theme(search_preview));
        }

        if let Some(article_view) = &user_theme.article_view {
            self.theme.article_view = Some(self.load_view_theme(article_view));
        }

        if let Some(toc_view) = &user_theme.toc_view {
            self.theme.toc_view = Some(self.load_view_theme(toc_view));
        }
    }

    fn load_view_theme(&self, user_view_theme: &UserViewTheme) -> ViewTheme {
        let mut view_theme = self.create_view_theme();

        macro_rules! to_view_theme {
            ($color: ident) => {
                log::debug!("Trying to load the color '{}'", stringify!($color));
                if user_view_theme.$color.is_some() {
                    match parse_color(user_view_theme.$color.as_ref().unwrap().to_string()) {
                        Ok(color) => {
                            view_theme.$color = color;
                            log::debug!("Loaded the color '{}'", stringify!($color));
                        }
                        Err(error) => {
                            log::warn!("{}", error);
                        }
                    };
                }
            };
        }

        to_view_theme!(text);
        to_view_theme!(title);
        to_view_theme!(highlight);
        to_view_theme!(background);
        to_view_theme!(highlight_text);
        to_view_theme!(highlight_inactive);

        view_theme
    }

    fn create_view_theme(&self) -> ViewTheme {
        ViewTheme {
            background: self.theme.background,
            text: self.theme.text,
            title: self.theme.title,
            secondary: Color::parse("blue").unwrap(),
            highlight: self.theme.highlight,
            highlight_text: self.theme.highlight_text,
            highlight_inactive: self.theme.highlight_inactive,
        }
    }

    fn load_logging(&mut self, user_logging: &UserLogging) {
        // now load the settings
        log::debug!("Trying to load the enabled setting");
        if let Some(enabled) = user_logging.enabled {
            self.logging.enabled = enabled;
        }

        log::debug!("Trying to load the logging dir setting");
        if let Some(log_dir) = user_logging.log_dir.as_ref() {
            if let Ok(path) = PathBuf::from_str(log_dir) {
                self.logging.log_dir = path;
            }
        }

        log::debug!("Trying to load the log level");
        if let Some(log_level) = user_logging.log_level.as_ref() {
            if let Ok(level) = LevelFilter::from_str(log_level) {
                self.logging.log_level = level;
            }
        }
    }
}

fn parse_color(color: String) -> Result<Color> {
    Color::parse(&color.to_lowercase()).context("Failed loading the color")
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
