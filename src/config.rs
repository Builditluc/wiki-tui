use crate::{cli::Cli, wiki::language::Language};

use anyhow::{bail, Context, Result};
use cursive::{
    event::{Event, Key},
    theme::{BaseColor, Color},
    Cursive,
};
use log::LevelFilter;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use std::{cell::RefCell, path::PathBuf, rc::Rc, str::FromStr};
#[cfg(not(test))]
use structopt::StructOpt;
use toml::from_str;

const CONFIG_FILE: &str = "config.toml";
const CONFIG_DIR: &str = ".config";
const APP_DIR: &str = "wiki-tui";

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

fn base_color_to_string(color: &BaseColor) -> String {
    match color {
        BaseColor::Black => "black",
        BaseColor::Red => "red",
        BaseColor::Green => "green",
        BaseColor::Yellow => "yellow",
        BaseColor::Blue => "blue",
        BaseColor::Magenta => "magenta",
        BaseColor::Cyan => "cyan",
        BaseColor::White => "white",
    }
    .to_string()
}

fn color_to_string(color: &Color) -> String {
    match color {
        Color::TerminalDefault => "default".to_string(),
        Color::Dark(color) => base_color_to_string(color),
        Color::Light(color) => format!("light {}", base_color_to_string(color)),
        Color::Rgb(r, g, b) | Color::RgbLowRes(r, g, b) => {
            format!("#{}{}{}", r, g, b)
        }
    }
}

pub struct Theme {
    pub text: Color,
    pub title: Color,
    pub highlight: Color,
    pub background: Color,
    pub search_match: Color,
    pub highlight_text: Color,
    pub highlight_inactive: Color,

    pub border: BorderStyle,

    pub search_bar: Option<ViewTheme>,
    pub search_results: Option<ViewTheme>,
    pub search_preview: Option<ViewTheme>,

    pub article_view: Option<ViewTheme>,
    pub toc_view: Option<ViewTheme>,
}

impl Serialize for Theme {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("theme", 2)?;

        macro_rules! serialize_color {
            ($field: ident) => {
                s.serialize_field(stringify!($field), &color_to_string(&self.$field))?;
            };
        }

        serialize_color!(text);
        serialize_color!(title);
        serialize_color!(highlight);
        serialize_color!(background);
        serialize_color!(search_match);
        serialize_color!(highlight_text);
        serialize_color!(highlight_inactive);

        s.end()
    }
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

#[derive(Copy, Clone)]
pub enum BorderStyle {
    Default,
    Light,
    Heavy,
    Round,
}

impl From<&String> for BorderStyle {
    fn from(s: &String) -> Self {
        match s.to_lowercase().as_str() {
            "default" => BorderStyle::Default,
            "light" => BorderStyle::Light,
            "heavy" => BorderStyle::Heavy,
            "round" => BorderStyle::Round,
            _ => BorderStyle::Default,
        }
    }
}

pub struct ViewTheme {
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

#[derive(Clone, Debug, Serialize)]
pub struct ApiConfig {
    pre_language: String,
    post_language: String,
    pub language: Language,
    pub language_changed_popup: bool,
}

impl ApiConfig {
    pub fn url(&self) -> String {
        format!(
            "{}{}{}",
            self.pre_language,
            self.language.code(),
            self.post_language
        )
    }
}

#[derive(Serialize)]
pub struct Logging {
    pub enabled: bool,
    pub log_dir: PathBuf,
    pub log_level: LevelFilter,
}

#[derive(Serialize)]
pub struct Features {
    pub links: bool,
    pub toc: bool,
}

#[derive(Clone)]
pub struct Keybindings {
    pub down: Event,
    pub up: Event,
    pub left: Event,
    pub right: Event,

    pub focus_next: Event,
    pub focus_prev: Event,

    pub toggle_language_selection: Event,
}

impl Serialize for Keybindings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("keybindings", 1)?;

        fn key_to_string(key: &Key) -> String {
            match key {
                Key::Enter => "enter",
                Key::Tab => "tab",
                Key::Backspace => "backspace",
                Key::Esc => "escape",
                Key::Left => "left",
                Key::Right => "right",
                Key::Up => "up",
                Key::Down => "down",
                Key::Ins => "insert",
                Key::Del => "delete",
                Key::Home => "home",
                Key::End => "end",
                Key::PageUp => "pageup",
                Key::PageDown => "pagedown",
                Key::PauseBreak => "pausebreak",
                Key::NumpadCenter => "numpadcenter",
                Key::F0 => "f0",
                Key::F1 => "f1",
                Key::F2 => "f2",
                Key::F3 => "f3",
                Key::F4 => "f4",
                Key::F5 => "f5",
                Key::F6 => "f6",
                Key::F7 => "f7",
                Key::F8 => "f8",
                Key::F9 => "f9",
                Key::F10 => "f10",
                Key::F11 => "f11",
                Key::F12 => "f12",
            }
            .to_string()
        }

        // this is used for serializing the nested fields mode and key
        #[derive(Serialize)]
        struct Keybinding {
            key: String,
            mode: String,
        }

        impl Keybinding {
            fn new<S: Into<String>>(key: S, mode: &str) -> Self {
                Keybinding {
                    key: key.into(),
                    mode: mode.to_string(),
                }
            }
        }

        macro_rules! serialize_event {
            ($event: ident) => {
                impl Keybinding {}

                match &self.$event {
                    Event::Char(char) => {
                        s.serialize_field(stringify!($event), &Keybinding::new(*char, "normal"))?
                    }
                    Event::CtrlChar(char) => {
                        s.serialize_field(stringify!($event), &Keybinding::new(*char, "ctrl"))?;
                    }
                    Event::Key(key) => s.serialize_field(
                        stringify!($event),
                        &Keybinding::new(&key_to_string(key), "normal"),
                    )?,
                    Event::Shift(key) => {
                        s.serialize_field(
                            stringify!($event),
                            &Keybinding::new(&key_to_string(key), "shift"),
                        )?;
                    }
                    Event::Alt(key) => {
                        s.serialize_field(
                            stringify!($event),
                            &Keybinding::new(&key_to_string(key), "alt"),
                        )?;
                    }
                    Event::AltShift(key) => {
                        s.serialize_field(
                            stringify!($event),
                            &Keybinding::new(&key_to_string(key), "altshift"),
                        )?;
                    }
                    Event::Ctrl(key) => {
                        s.serialize_field(
                            stringify!($event),
                            &Keybinding::new(&key_to_string(key), "ctrl"),
                        )?;
                    }
                    Event::CtrlShift(key) => {
                        s.serialize_field(
                            stringify!($event),
                            &Keybinding::new(&key_to_string(key), "ctrlshift"),
                        )?;
                    }
                    Event::CtrlAlt(key) => {
                        s.serialize_field(
                            stringify!($event),
                            &Keybinding::new(&key_to_string(key), "ctrlalt"),
                        )?;
                    }
                    _ => s.serialize_field(stringify!($event), "invalid, internal error")?,
                }
            };
        }

        serialize_event!(down);
        serialize_event!(up);
        serialize_event!(left);
        serialize_event!(right);

        serialize_event!(focus_next);
        serialize_event!(focus_prev);

        serialize_event!(toggle_language_selection);

        s.end()
    }
}

#[derive(Serialize)]
pub struct Settings {
    pub toc: TocSettings,
}

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub enum TocPosition {
    Left,
    Right,
}

#[derive(Clone, Serialize)]
pub enum TocTitle {
    Default,
    Custom,
    Article,
}

#[derive(Serialize)]
pub struct Config {
    #[serde(rename(serialize = "api"))]
    pub api_config: ApiConfig,
    pub theme: Theme,
    pub logging: Logging,
    pub features: Features,
    pub keybindings: Keybindings,
    pub settings: Settings,
    #[serde(skip_serializing)]
    config_path: PathBuf,
    #[serde(skip_serializing)]
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

    border: Option<String>,

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
    pre_language: Option<String>,
    post_language: Option<String>,
    language: Option<String>,
    language_changed_popup: Option<bool>,
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

    focus_next: Option<UserKeybinding>,
    focus_prev: Option<UserKeybinding>,

    toggle_language_selection: Option<UserKeybinding>,
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
                pre_language: "https://".to_string(),
                post_language: ".wikipedia.org/w/api.php".to_string(),
                language: Language::default(),
                language_changed_popup: true,
            },
            theme: Theme {
                background: Color::Dark(BaseColor::Black),
                title: Color::Dark(BaseColor::Red),
                highlight: Color::Dark(BaseColor::Red),
                highlight_inactive: Color::Dark(BaseColor::Black),
                highlight_text: Color::Dark(BaseColor::White),
                text: Color::Dark(BaseColor::White),
                search_match: Color::Dark(BaseColor::Red),

                border: BorderStyle::Default,

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
                down: Event::Key(Key::Down),
                up: Event::Key(Key::Up),
                left: Event::Key(Key::Left),
                right: Event::Key(Key::Right),

                focus_next: Event::Key(Key::Tab),
                focus_prev: Event::Shift(Key::Tab),

                toggle_language_selection: Event::Key(Key::F2),
            },
            settings: Settings {
                toc: TocSettings {
                    position: TocPosition::Right,
                    title: TocTitle::Default,
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
        if let Err(err) = config
            .load_config()
            .context("failed loading the configuration")
        {
            error!("{:?}", err);
            return config;
        }

        info!("loaded the config");

        // return the config
        config
    }

    /// Returns the configuration stored in `Cursive`. If none could be found, it
    /// creates a new `Config` and stores it into `Cursvie`.
    pub fn from_siv(siv: &mut Cursive) -> Rc<RefCell<Config>> {
        match siv.user_data::<Rc<RefCell<Config>>>() {
            Some(config) => config.clone(),
            None => {
                siv.set_user_data(Rc::new(RefCell::new(Config::new())));
                siv.user_data::<Rc<RefCell<Config>>>().unwrap().clone()
            }
        }
    }

    fn load_config(&mut self) -> Result<()> {
        // load (or create if they don't exist) the config path(s)
        // this function returns true if the config file exists and false if not
        self.load_or_create_config_paths()?;
        debug!("loaded the config paths");

        // read the config file and check if there were any errors
        let config_str =
            std::fs::read_to_string(&self.config_path).context("failed reading the config file")?;

        let user_config = from_str::<UserConfig>(&config_str).context("wrong config format")?;

        if let Some(user_api_config) = user_config.api {
            self.load_api_config(&user_api_config);
        }

        if let Some(user_theme) = user_config.theme {
            if let Err(err) = self
                .load_theme(&user_theme)
                .context("failed loading the theme configuration")
            {
                warn!("{:?}", err);
                bail!(err);
            }
            debug!("loaded the theme config")
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

        self.load_cli_arguments();

        Ok(())
    }

    fn load_cli_arguments(&mut self) {
        // override the log level
        if let Some(log_level) = self.args.level.as_ref() {
            let level = match log_level {
                0 => LevelFilter::Debug,
                1 => LevelFilter::Info,
                2 => LevelFilter::Warn,
                3 => LevelFilter::Error,
                _ => self.logging.log_level,
            };
            info!(
                "overriding the configured log level from '{}' to '{}'",
                self.logging.log_level, level
            );

            self.logging.log_level = level;
        }

        if let Some(language) = self.args.language.as_ref() {
            let language = Language::from(language.as_str());
            info!(
                "overriding the configured language from '{}' to '{}'",
                self.api_config.language.name(),
                language.name()
            );

            self.api_config.language = language;
        }
    }

    fn load_or_create_config_paths(&mut self) -> Result<bool> {
        // get the platform specific config directory
        let config_dir = match dirs::home_dir() {
            Some(config_dir) => {
                info!(
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
            info!("the app config directory doesn't exist, creating it now");
            std::fs::create_dir(app_config_dir)
                .context("couldn't create the app config directory")?;
        }

        // check, if the config file exists
        if !config_file_dir.exists() {
            info!("the config file doesn't exist");
            info!("creating the config file");

            // serialize the default configuration
            let config = toml::to_string_pretty(&self).context("couldn't serialize the config")?;
            std::fs::write(config_file_dir.clone(), config).context("couldn't write the config")?;
        }

        self.config_path = config_file_dir;
        Ok(true)
    }

    fn load_api_config(&mut self, user_api_config: &UserApiConfig) {
        info!("loading the api configuration");

        if let Some(pre_language) = &user_api_config.pre_language {
            self.api_config.pre_language = pre_language.to_string();
            debug!("loaded 'pre_language'");
        }

        if let Some(post_language) = &user_api_config.post_language {
            self.api_config.post_language = post_language.to_string();
            debug!("loaded 'post_language'");
        }

        if let Some(language) = &user_api_config.language {
            self.api_config.language = Language::from(language.as_str());
            debug!("loaded 'langugae'")
        }

        if let Some(language_changed_popup) = &user_api_config.language_changed_popup {
            self.api_config.language_changed_popup = language_changed_popup.to_owned();
            debug!("loaded 'language_changed_popup")
        }
    }

    fn load_theme(&mut self, user_theme: &UserTheme) -> Result<()> {
        info!("loading the theme configuration");

        // define the macro for loading individual color settings
        macro_rules! to_theme_color {
            ($color: ident) => {
                if user_theme.$color.is_some() {
                    match parse_color(user_theme.$color.as_ref().unwrap().to_string()) {
                        Ok(color) => {
                            self.theme.$color = color;
                            debug!("loaded '{}'", stringify!(theme.$color));
                        }
                        Err(error) => {
                            warn!("{}", error);
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

        debug!("loaded the global theme");

        // load the border
        if let Some(border) = &user_theme.border {
            self.theme.border = border.into();
        }

        // load the themes for the individual views
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
            log::debug!("loaded 'theme.search_results'");
        }

        if let Some(search_preview) = &user_theme.search_preview {
            self.theme.search_preview = Some(self.load_view_theme(search_preview));
            log::debug!("loaded 'theme.search_preview'");
        }

        if let Some(article_view) = &user_theme.article_view {
            self.theme.article_view = Some(self.load_view_theme(article_view));
            log::debug!("loaded 'theme.article_view'");
        }

        if let Some(toc_view) = &user_theme.toc_view {
            self.theme.toc_view = Some(self.load_view_theme(toc_view));
            log::debug!("loaded 'theme.toc_view'");
        }

        debug!("loaded the view themes");

        Ok(())
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
                            warn!("{}", error);
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
        info!("loading the logging configuration");

        if let Some(enabled) = user_logging.enabled {
            self.logging.enabled = enabled;
            log::debug!("loaded 'logging.enabled'");
        }

        if let Some(log_dir) = user_logging.log_dir.as_ref() {
            if let Ok(path) = PathBuf::from_str(log_dir) {
                self.logging.log_dir = path;
                log::debug!("loaded 'logging.log_dir'");
            }
        }

        if let Some(log_level) = user_logging.log_level.as_ref() {
            if let Ok(level) = LevelFilter::from_str(log_level) {
                self.logging.log_level = level;
                log::debug!("loaded 'logging.log_level'");
            }
        }
    }

    fn load_features(&mut self, user_features: &UserFeatures) {
        info!("loading the article features");

        if let Some(links) = user_features.links {
            self.features.links = links;
            log::debug!("loaded 'features.links'");
        }

        if let Some(toc) = user_features.toc {
            self.features.toc = toc;
            log::debug!("loaded 'features.toc'");
        }
    }

    fn load_keybindings(&mut self, user_keybindings: &UserKeybindings) {
        info!("loading the keybindings");

        if let Some(keybinding) = &user_keybindings.down {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.down = event_key;
                    log::debug!("loaded 'keybindings.down'");
                }
                Err(error) => {
                    warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.up {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.up = event_key;
                    log::debug!("loaded 'keybindings.up'");
                }
                Err(error) => {
                    warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.left {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.left = event_key;
                    log::debug!("loaded 'keybindings.left'");
                }
                Err(error) => {
                    warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.right {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.right = event_key;
                    log::debug!("loaded 'keybindings.right'");
                }
                Err(error) => {
                    warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.focus_next {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.focus_next = event_key;
                    log::debug!("loaded 'keybindings.focus_next'");
                }
                Err(error) => {
                    warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.focus_prev {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.focus_prev = event_key;
                    log::debug!("loaded 'keybindings.focus_prev'");
                }
                Err(error) => {
                    warn!("{:?}", error)
                }
            }
        }
        if let Some(keybinding) = &user_keybindings.toggle_language_selection {
            match parse_keybinding(
                &keybinding.key,
                keybinding.mode.as_ref().unwrap_or(&"normal".to_string()),
            ) {
                Ok(event_key) => {
                    self.keybindings.toggle_language_selection = event_key;
                    log::debug!("loaded 'keybindings.toggle_language_selection'");
                }
                Err(error) => {
                    warn!("{:?}", error)
                }
            }
        }
    }

    fn load_settings(&mut self, user_settings: &UserSettings) {
        info!("loading settings");

        if let Some(user_toc_settings) = &user_settings.toc {
            self.load_toc_settings(user_toc_settings);
        }
    }

    fn load_toc_settings(&mut self, user_toc_settings: &UserTocSettings) {
        info!("loading toc settings");

        if let Some(position) = &user_toc_settings.position {
            match position.to_lowercase().as_str() {
                "left" => self.settings.toc.position = TocPosition::Left,
                "right" => self.settings.toc.position = TocPosition::Right,
                pos => warn!("unknown toc position, got {}", pos),
            }
            log::debug!("loaded 'settings.toc.position'");
        }

        if let Some(title) = &user_toc_settings.title {
            match title.to_lowercase().as_str() {
                "default" => self.settings.toc.title = TocTitle::Default,
                "custom" => self.settings.toc.title = TocTitle::Custom,
                "article" => self.settings.toc.title = TocTitle::Article,
                _ => self.settings.toc.title = TocTitle::Default,
            }
            log::debug!("loaded 'settings.toc.title'");
        }

        if let Some(title_custom) = &user_toc_settings.title_custom {
            self.settings.toc.title_custom = Some(title_custom.to_string());
            log::debug!("loaded 'settings.toc.title_custom'");
        }

        if let Some(min_width) = &user_toc_settings.min_width {
            self.settings.toc.min_width = min_width.to_owned();
            log::debug!("loaded 'settings.toc.min_width'");
        }

        if let Some(max_width) = &user_toc_settings.max_width {
            self.settings.toc.max_width = max_width.to_owned();
            log::debug!("loaded 'settings.toc.max_width'");
        }

        if let Some(scroll_x) = &user_toc_settings.scroll_x {
            self.settings.toc.scroll_x = scroll_x.to_owned();
            log::debug!("loaded 'settings.toc.scroll_x'");
        }

        if let Some(scroll_y) = &user_toc_settings.scroll_y {
            self.settings.toc.scroll_y = scroll_y.to_owned();
            log::debug!("loaded 'settings.toc.scroll_y'");
        }

        if let Some(item_format) = &user_toc_settings.item_format {
            self.settings.toc.item_format = item_format.to_owned();
            log::debug!("loaded 'settings.toc.item_format'");
        }
    }

    pub fn get_args(&self) -> &Cli {
        &self.args
    }
}

fn parse_color(color: String) -> Result<Color> {
    Color::parse(&color.to_lowercase())
        .with_context(|| format!("failed parsing the color '{}'", color))
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
        "down" => Ok(Key::Down),
        "up" => Ok(Key::Up),
        "left" => Ok(Key::Left),
        "right" => Ok(Key::Right),
        "tab" => Ok(Key::Tab),
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
