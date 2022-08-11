use crate::cli::Cli;

use anyhow::{bail, Context, Result};
use cursive::{
    event::{Event, Key},
    theme::{BaseColor, Color},
};
use lazy_static::*;
use log::LevelFilter;
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
#[cfg(not(test))]
use structopt::StructOpt;
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

pub struct Features {
    pub links: bool,
    pub toc: bool,
}

pub struct Keybindings {
    pub down: Option<Event>,
    pub up: Option<Event>,
    pub left: Option<Event>,
    pub right: Option<Event>,
}

pub struct Settings {
    pub toc: TocSettings,
}

#[derive(Clone)]
pub struct TocSettings {
    pub position: TocPosition,
    pub title: TocTitle,
    pub title_custom: Option<String>,
    pub min_width: usize,
    pub max_width: usize,
    pub scroll_x: bool,
    pub scroll_y: bool,
    pub item_format: String,
}

#[derive(Clone)]
pub enum TocPosition {
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub enum TocTitle {
    DEFAULT,
    CUSTOM,
    ARTICLE,
}

pub struct Config {
    pub api_config: ApiConfig,
    pub theme: Theme,
    pub logging: Logging,
    pub features: Features,
    pub keybindings: Keybindings,
    pub settings: Settings,
    config_path: PathBuf,
    args: Cli,
}

#[derive(Deserialize, Debug)]
struct UserConfig {
    api: Option<UserApiConfig>,
    theme: Option<UserTheme>,
    logging: Option<UserLogging>,
    features: Option<UserFeatures>,
    keybindings: Option<UserKeybindings>,
    settings: Option<UserSettings>,
}

#[derive(Deserialize, Debug)]
struct UserSettings {
    toc: Option<UserTocSettings>,
}

#[derive(Deserialize, Debug)]
struct UserTocSettings {
    position: Option<String>,
    title: Option<String>,
    title_custom: Option<String>,
    min_width: Option<usize>,
    max_width: Option<usize>,
    scroll_x: Option<bool>,
    scroll_y: Option<bool>,
    item_format: Option<String>,
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

#[derive(Deserialize, Debug)]
struct UserFeatures {
    links: Option<bool>,
    toc: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct UserKeybindings {
    down: Option<UserKeybinding>,
    up: Option<UserKeybinding>,
    left: Option<UserKeybinding>,
    right: Option<UserKeybinding>,
}

#[derive(Deserialize, Debug)]
struct UserKeybinding {
    key: String,
    mode: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        // initialize the configuration with the defaults
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
            features: Features {
                links: true,
                toc: true,
            },
            keybindings: Keybindings {
                down: None,
                up: None,
                left: None,
                right: None,
            },
            settings: Settings {
                toc: TocSettings {
                    position: TocPosition::RIGHT,
                    title: TocTitle::DEFAULT,
                    title_custom: None,
                    min_width: 20,
                    max_width: 60,
                    scroll_x: true,
                    scroll_y: true,
                    item_format: "{NUMBER} {TEXT}".to_string(),
                },
            },
            config_path: PathBuf::new(),
            #[cfg(not(test))]
            args: Cli::from_args(),

            #[cfg(test)]
            args: Cli::default(),
        };

        // load the configuration from the file
        log::info!("loading the config");
        config.load_config();

        // return the config
        config
    }

    fn load_config(&mut self) {
        // load (or create if they don't exist) the config path(s)
        // this function returns true if the config file exists and false if not
        let config_exists = self.load_or_create_config_paths();

        // check, if any errors occured during loading
        if config_exists.is_err() {
            // abort the loading and return the error
            log::warn!("{:?}", config_exists);
            return;
        }

        // read the config file and check if there were any errors
        let config_str = match std::fs::read_to_string(&self.config_path)
            .context("failed reading the config file")
        {
            Ok(config) => {
                log::info!("successfully read the config file");
                config
            }
            Err(error) => {
                log::warn!("{:?}", error);
                return;
            }
        };

        let user_config = match from_str::<UserConfig>(&config_str).context("wrong format") {
            Ok(config) => {
                log::info!("successfully deserialized config");
                config
            }
            Err(error) => {
                log::warn!("deserializing the config file failed, {:?}", error);
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

        if let Some(user_features) = user_config.features {
            self.load_features(&user_features);
        }

        if let Some(user_keybindings) = user_config.keybindings {
            self.load_keybindings(&user_keybindings);
        }

        if let Some(user_settings) = user_config.settings {
            self.load_settings(&user_settings);
        }

        // override the log level
        if let Some(log_level) = self.args.level.as_ref() {
            let level = match log_level {
                0 => LevelFilter::Debug,
                1 => LevelFilter::Info,
                2 => LevelFilter::Warn,
                3 => LevelFilter::Error,
                _ => self.logging.log_level,
            };
            log::info!("overriding the configured log level to '{}'", level);
            self.logging.log_level = level;
        }
    }

    fn load_or_create_config_paths(&mut self) -> Result<bool> {
        // get the platform specific config directory
        let config_dir = match dirs::home_dir() {
            Some(config_dir) => {
                log::info!(
                    "the config directory is {}",
                    config_dir.join(CONFIG_DIR).to_str().unwrap()
                );
                config_dir.join(CONFIG_DIR)
            }
            None => {
                bail!("couldn't find the home directory")
            }
        };

        // build the app config path and the config file path
        let app_config_dir = config_dir.join(APP_DIR);
        let config_file_dir = app_config_dir.join(CONFIG_FILE);

        // create the app config folder if it doesn't exist
        if !app_config_dir.exists() {
            log::info!("the app config directory doesn't exist, creating it now");
            std::fs::create_dir(app_config_dir)
                .context("couldn't create the app config directory")?;
        }

        // check, if the config file exists
        if !config_file_dir.exists() {
            log::info!("the config file doesn't exist");
            return Ok(false);
        }

        // if the config file exists,
        // return true and store the path to it
        log::info!(
            "location of the config file: '{}'",
            // the path can be non unicode so we have to check for that
            config_file_dir.to_str().unwrap_or("UNICODE_ERROR")
        );
        self.config_path = config_file_dir;
        Ok(true)
    }

    fn load_api_config(&mut self, user_api_config: &UserApiConfig) {
        log::info!("loading the api configuration");

        // define the macro for loading individual api settings
        macro_rules! to_api_setting {
            ($setting: ident) => {
                if user_api_config.$setting.is_some() {
                    self.api_config.$setting =
                        user_api_config.$setting.as_ref().unwrap().to_string();
                }
            };
        }

        to_api_setting!(base_url);
    }

    fn load_theme(&mut self, user_theme: &UserTheme) {
        log::info!("loading the theme configuration");

        // define the macro for loading individual color settings
        macro_rules! to_theme_color {
            ($color: ident) => {
                if user_theme.$color.is_some() {
                    match parse_color(user_theme.$color.as_ref().unwrap().to_string()) {
                        Ok(color) => {
                            self.theme.$color = color;
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
                if user_view_theme.$color.is_some() {
                    match parse_color(user_view_theme.$color.as_ref().unwrap().to_string()) {
                        Ok(color) => {
                            view_theme.$color = color;
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
        log::info!("loading the logging configuration");

        if let Some(enabled) = user_logging.enabled {
            self.logging.enabled = enabled;
        }

        if let Some(log_dir) = user_logging.log_dir.as_ref() {
            if let Ok(path) = PathBuf::from_str(log_dir) {
                self.logging.log_dir = path;
            }
        }

        if let Some(log_level) = user_logging.log_level.as_ref() {
            if let Ok(level) = LevelFilter::from_str(log_level) {
                self.logging.log_level = level;
            }
        }
    }

    fn load_features(&mut self, user_features: &UserFeatures) {
        log::info!("loading the article features");

        if let Some(links) = user_features.links {
            self.features.links = links;
        }

        if let Some(toc) = user_features.toc {
            self.features.toc = toc;
        }
    }

    fn load_keybindings(&mut self, user_keybindings: &UserKeybindings) {
        log::info!("loading the keybindings");

        if let Some(keybinding) = &user_keybindings.down {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.down = Some(event_key);
                }
                Err(error) => {
                    log::warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.up {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.up = Some(event_key);
                }
                Err(error) => {
                    log::warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.left {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.left = Some(event_key);
                }
                Err(error) => {
                    log::warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.right {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.right = Some(event_key);
                }
                Err(error) => {
                    log::warn!("{:?}", error)
                }
            }
        }
    }

    fn load_settings(&mut self, user_settings: &UserSettings) {
        log::info!("loading settings");

        if let Some(user_toc_settings) = &user_settings.toc {
            self.load_toc_settings(user_toc_settings);
        }
    }

    fn load_toc_settings(&mut self, user_toc_settings: &UserTocSettings) {
        log::info!("loading toc settings");

        if let Some(position) = &user_toc_settings.position {
            match position.to_lowercase().as_str() {
                "left" => self.settings.toc.position = TocPosition::LEFT,
                "right" => self.settings.toc.position = TocPosition::RIGHT,
                pos => log::warn!("unknown toc position, got {}", pos),
            }
        }

        if let Some(title) = &user_toc_settings.title {
            match title.to_lowercase().as_str() {
                "default" => self.settings.toc.title = TocTitle::DEFAULT,
                "custom" => self.settings.toc.title = TocTitle::CUSTOM,
                "article" => self.settings.toc.title = TocTitle::ARTICLE,
                _ => self.settings.toc.title = TocTitle::DEFAULT,
            }
        }

        if let Some(title_custom) = &user_toc_settings.title_custom {
            self.settings.toc.title_custom = Some(title_custom.to_string());
        }

        if let Some(min_width) = &user_toc_settings.min_width {
            self.settings.toc.min_width = min_width.to_owned();
        }

        if let Some(max_width) = &user_toc_settings.max_width {
            self.settings.toc.max_width = max_width.to_owned();
        }

        if let Some(scroll_x) = &user_toc_settings.scroll_x {
            self.settings.toc.scroll_x = scroll_x.to_owned();
        }

        if let Some(scroll_y) = &user_toc_settings.scroll_y {
            self.settings.toc.scroll_y = scroll_y.to_owned();
        }

        if let Some(item_format) = &user_toc_settings.item_format {
            self.settings.toc.item_format = item_format.to_owned();
        }
    }

    pub fn get_args(&self) -> &Cli {
        &self.args
    }
}

fn parse_color(color: String) -> Result<Color> {
    Color::parse(&color.to_lowercase()).context("Failed loading the color")
}

fn parse_keybinding(key: &str, mode: &str) -> Result<Event> {
    // check if the key is a character
    if let Ok(character) = char::from_str(key) {
        match mode.to_lowercase().as_str() {
            "normal" => return Ok(Event::Char(character)),
            "ctrl" => return Ok(Event::CtrlChar(character)),
            _ => {
                bail!(
                    "couldn't parse the char: {} with the mode: {}",
                    character,
                    mode
                );
            }
        }
    }

    let event_key = parse_key(key)?;
    match mode.to_lowercase().as_str() {
        "normal" => Ok(Event::Key(event_key)),
        "shift" => Ok(Event::Shift(event_key)),
        "alt" => Ok(Event::Alt(event_key)),
        "altshift" => Ok(Event::AltShift(event_key)),
        "ctrl" => Ok(Event::Ctrl(event_key)),
        "ctrlshift" => Ok(Event::CtrlShift(event_key)),
        "ctrlalt" => Ok(Event::CtrlAlt(event_key)),
        _ => {
            bail!("couldn't parse the key: {} with the mode: {}", key, mode);
        }
    }
}

fn parse_key(key: &str) -> Result<Key> {
    // check if the key is a non-character key on the keyboard
    match key.to_lowercase().as_str() {
        "insert" => Ok(Key::Ins),
        "delete" => Ok(Key::Del),
        "home" => Ok(Key::Home),
        "end" => Ok(Key::End),
        "pageup" => Ok(Key::PageUp),
        "pagedown" => Ok(Key::PageDown),
        "pausebreak" => Ok(Key::PauseBreak),
        "numpadcenter" => Ok(Key::NumpadCenter),
        "f0" => Ok(Key::F0),
        "f1" => Ok(Key::F1),
        "f2" => Ok(Key::F2),
        "f3" => Ok(Key::F3),
        "f4" => Ok(Key::F4),
        "f5" => Ok(Key::F5),
        "f6" => Ok(Key::F6),
        "f7" => Ok(Key::F7),
        "f8" => Ok(Key::F8),
        "f9" => Ok(Key::F9),
        "f10" => Ok(Key::F10),
        "f11" => Ok(Key::F11),
        "f12" => Ok(Key::F12),
        _ => bail!("couldn't parse the key: {}", key),
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
