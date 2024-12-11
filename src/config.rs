use anyhow::{bail, Context, Result};
use bitflags::bitflags;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use directories::ProjectDirs;
use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{BorderType, Padding},
};
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
use tracing::level_filters::LevelFilter;
use wiki_api::{languages::Language, search, Endpoint};

pub const CACHE_ENV: &str = "WIKI_TUI_CACHE";
pub const CONFIG_ENV: &str = "WIKI_TUI_CONFIG";

pub const THEME_FILE_NAME: &str = "theme.toml";
pub const CONFIG_FILE_NAME: &str = "config.toml";

pub fn project_dir() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "builditluc", "wiki-tui")
}

pub fn cache_dir() -> Result<PathBuf> {
    let directory = if let Ok(dir) = std::env::var(CACHE_ENV) {
        PathBuf::from(dir)
    } else if let Some(project_dir) = project_dir() {
        project_dir.cache_dir().to_path_buf()
    } else {
        bail!("Unable to find data directory for wiki-tui");
    };

    Ok(directory)
}

pub fn config_dir() -> Result<PathBuf> {
    let directory = if let Ok(dir) = std::env::var(CONFIG_ENV) {
        PathBuf::from(dir)
    } else if let Some(project_dir) = project_dir() {
        project_dir.config_local_dir().to_path_buf()
    } else {
        bail!("Unable to find config directory for wiki-tui");
    };

    if !directory.exists() {
        std::fs::create_dir_all(&directory).context("Unable to create the config folder")?;
    }

    Ok(directory)
}

macro_rules! override_options {
    ($config:expr, $uconfig:ident::{$( $option:ident ),+}) => {
        $({override_options!($config, $uconfig::$option->$option)})+
    };
    ($config:expr, $uconfig:ident::{$( $uoption:ident->$option:ident ),+}) => {
        $({override_options!($config, $uconfig::$uoption->$option)})+
    };
    ($config:expr, $uconfig:ident::$uoption:ident) => {
        override_options!($config, $uconfig::$uoption->$uoption)
    };
    ($config:expr, $uconfig:ident::$uoption:ident->$option:ident) => {
        if let Some(option) = $uconfig.$uoption {
            $config.$option = option.into();
        }
    };
}

pub fn load_logging_config() -> Result<LoggingConfig> {
    let mut default_config = LoggingConfig::default();
    let user_config = toml::from_str::<UserLoggingConfig>(&get_user_config()?)
        .context("failed loading the user logging configuration")?;

    if let Some(inner) = user_config.logging {
        override_options!(default_config, inner::enabled);

        // we need to manually parse the level
        if let Some(ref level) = inner.level {
            default_config.level = LevelFilter::from_str(level)?;
        }
    }

    Ok(default_config)
}

pub struct LoggingConfig {
    pub enabled: bool,
    pub level: LevelFilter,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            enabled: true,
            level: LevelFilter::WARN,
        }
    }
}

#[derive(Deserialize)]
struct UserLoggingConfig {
    logging: Option<UserLoggingConfigInner>,
}

#[derive(Deserialize)]
struct UserLoggingConfigInner {
    enabled: Option<bool>,
    #[serde(rename = "log_level")]
    level: Option<String>,
}

pub fn load_config() -> Result<Config> {
    let mut default_config = Config::default();
    let user_config = load_user_config().context("failed loading the user config")?;

    if let Some(user_page_config) = user_config.page {
        override_page_config(&mut default_config.page, user_page_config)
    }

    if let Some(user_bindings_config) = user_config.bindings {
        override_bindings_config(&mut default_config.bindings, user_bindings_config)
    }

    if let Some(user_api_config) = user_config.api {
        override_api_config(&mut default_config.api, user_api_config)?
    }

    if let Some(user_ui_config) = user_config.ui {
        override_ui_config(&mut default_config.ui, user_ui_config)
    }

    Ok(default_config)
}

fn override_page_config(config: &mut PageConfig, user_config: UserPageConfig) {
    if let Some(user_toc) = user_config.toc {
        override_options!(config.toc, user_toc::{
            enabled,
            width_percentage,
            position,
            title,
            item_format,

            enable_scrolling
        });
    }

    override_options!(config, user_config::padding);

    if let Some(user_zen) = user_config.zen_mode {
        override_options!(config, user_zen::{
            default->default_zen,
            include->zen_mode,

            horizontal->zen_horizontal,
            vertical->zen_vertical
        });
    }
}

fn override_bindings_config(config: &mut Keybindings, user_config: UserKeybindingsConfig) {
    if let Some(user_global_bindings) = user_config.global {
        override_options!(config.global, user_global_bindings::{
            scroll_down,
            scroll_up,

            scroll_to_top,
            scroll_to_bottom,

            pop_popup,

            half_down,
            half_up,
            unselect_scroll,

            submit,
            quit,
            enter_search_bar,
            exit_search_bar,

            switch_context_search,
            switch_context_page,

            toggle_search_language_selection,
            toggle_logger
        });
    }

    if let Some(user_search_bindings) = user_config.search {
        override_options!(config.search, user_search_bindings::continue_search);
    }

    if let Some(user_page_bindings) = user_config.page {
        override_options!(config.page, user_page_bindings::{
            pop_page,
            jump_to_header,
            select_first_link,
            select_last_link,
            select_next_link,
            select_prev_link,
            open_link,
            toggle_page_language_selection,
            toggle_zen_mode,
            toggle_toc
        });
    }
}

fn override_api_config(config: &mut ApiConfig, user_config: UserApiConfig) -> Result<()> {
    // we need to manually build the endpoint
    {
        let pre_language = match user_config.pre_language {
            Some(ref language) => language.as_str(),
            None => "https://",
        };
        let language = &user_config.language.unwrap_or(config.language);
        let post_language = match user_config.post_language {
            Some(ref language) => language.as_str(),
            None => ".wikipedia.org/w/api.php",
        };

        config.endpoint = Endpoint::parse(&format!(
            "{}{}{}",
            pre_language,
            language.code(),
            post_language,
        ))
        .context("failed parsing the endpoint url")?;
    }

    override_options!(config, user_config::{
        language,

        search_limit,
        search_info,
        search_type,
        search_qiprofile,
        search_rewrites,
        search_sort_order,

        page_redirects
    });

    Ok(())
}

fn override_ui_config(config: &mut UiConfig, user_config: UserUiConfig) {
    override_options!(config, user_config::{
        popup_search_language_changed,
        popup_page_language_changed
    });
}

fn get_user_config() -> Result<String> {
    let path = config_dir()
        .context("failed retrieving the config dir")?
        .join(CONFIG_FILE_NAME);

    if !path.exists() {
        std::fs::write(&path, "").context("failed creating the config file")?;
    }

    std::fs::read_to_string(&path).context("failed reading the config file")
}

fn load_user_config() -> Result<UserConfig> {
    let user_config_str = get_user_config()?;
    toml::from_str::<UserConfig>(&user_config_str).context("failed parsing the user config")
}

pub struct Config {
    pub page: PageConfig,
    pub bindings: Keybindings,
    pub api: ApiConfig,
    pub ui: UiConfig,
}

pub struct PageConfig {
    pub toc: TocConfig,
    pub padding: Padding,

    pub default_zen: bool,
    pub zen_mode: ZenModeComponents,

    pub zen_horizontal: Constraint,
    pub zen_vertical: Constraint,
}

bitflags! {
    #[derive(Deserialize, Debug, Clone)]
    pub struct ZenModeComponents: u8 {
        const STATUS_BAR = 0b00000001;
        const TOC        = 0b00000010;
        const SEARCH_BAR = 0b00000100;
        const SCROLLBAR  = 0b00001000;
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum PaddingConfig {
    Uniform(u16),
    Horizontal { horizontal: u16 },
    Vertical { veritical: u16 },
    Proportional { proportional: u16 },
    Symmetric { symmetric: (u16, u16) },
    Custom(u16, u16, u16, u16),
}

#[allow(clippy::from_over_into)] // since we cannot implement From for an external type, we need to
                                 // ignore the warning
impl Into<Padding> for PaddingConfig {
    fn into(self) -> Padding {
        match self {
            PaddingConfig::Uniform(val) => Padding::uniform(val),
            PaddingConfig::Horizontal { horizontal } => Padding::horizontal(horizontal),
            PaddingConfig::Vertical { veritical } => Padding::vertical(veritical),
            PaddingConfig::Proportional { proportional } => Padding::proportional(proportional),
            PaddingConfig::Symmetric { symmetric } => Padding::symmetric(symmetric.0, symmetric.1),
            PaddingConfig::Custom(left, right, top, bottom) => {
                Padding::new(left, right, top, bottom)
            }
        }
    }
}

#[derive(Deserialize, PartialEq, Eq)]
pub enum TocConfigPosition {
    Left,
    Right,
}

#[derive(Deserialize)]
pub enum TocConfigTitle {
    Default,
    Article,
    Custom(String),
}

pub struct TocConfig {
    pub enabled: bool,
    pub width_percentage: u16,
    pub position: TocConfigPosition,
    pub title: TocConfigTitle,
    item_format: String,

    pub enable_scrolling: bool,
}

#[derive(Deserialize)]
struct Binding {
    code: KeyCode,
    modifiers: KeyModifiers,
}

#[derive(Deserialize)]
pub struct Keybinding {
    bindings: Vec<Binding>,
}

impl Keybinding {
    fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    fn binding(mut self, code: KeyCode, modifiers: KeyModifiers) -> Self {
        self.bindings.push(Binding { code, modifiers });
        self
    }

    pub fn matches_event(&self, event: KeyEvent) -> bool {
        return self
            .bindings
            .iter()
            .any(|x| x.code == event.code && x.modifiers == event.modifiers);
    }
}

pub struct GlobalKeybindings {
    pub scroll_down: Keybinding,
    pub scroll_up: Keybinding,

    pub scroll_to_top: Keybinding,
    pub scroll_to_bottom: Keybinding,

    pub pop_popup: Keybinding,

    pub half_down: Keybinding,
    pub half_up: Keybinding,
    pub unselect_scroll: Keybinding,

    pub submit: Keybinding,
    pub quit: Keybinding,
    pub enter_search_bar: Keybinding,
    pub exit_search_bar: Keybinding,

    pub switch_context_search: Keybinding,
    pub switch_context_page: Keybinding,

    pub toggle_search_language_selection: Keybinding,
    pub toggle_logger: Keybinding,
}

pub struct SearchKeybindings {
    pub continue_search: Keybinding,
}

pub struct PageKeybindings {
    pub pop_page: Keybinding,
    pub jump_to_header: Keybinding,

    pub select_first_link: Keybinding,
    pub select_last_link: Keybinding,

    pub select_prev_link: Keybinding,
    pub select_next_link: Keybinding,

    pub open_link: Keybinding,

    pub toggle_page_language_selection: Keybinding,
    pub toggle_zen_mode: Keybinding,
    pub toggle_toc: Keybinding,
}

pub struct Keybindings {
    pub global: GlobalKeybindings,
    pub search: SearchKeybindings,
    pub page: PageKeybindings,
}

pub struct ApiConfig {
    pub endpoint: Endpoint,
    pub language: Language,

    pub search_limit: usize,
    pub search_qiprofile: search::QiProfile,
    pub search_type: search::SearchType,
    pub search_info: search::Info,
    pub search_rewrites: bool,
    pub search_sort_order: search::SortOrder,

    pub page_redirects: bool,
}

pub struct UiConfig {
    pub popup_search_language_changed: bool,
    pub popup_page_language_changed: bool,
}

impl Config {
    pub fn new() -> Self {
        macro_rules! keybinding {
            ([$($ch:expr; $($md:ident)|*),+]) => {
                {
                    Keybinding::new()
                        $(.binding(
                            $ch,
                            KeyModifiers::NONE$(|KeyModifiers::$md)*
                        ))+
                }
            };
        }

        Self {
            page: PageConfig {
                toc: TocConfig {
                    enabled: true,
                    width_percentage: 20,
                    position: TocConfigPosition::Right,
                    title: TocConfigTitle::Default,
                    item_format: "{NUMBER} {TEXT}".to_string(),

                    enable_scrolling: true,
                },
                padding: Padding::zero(),

                default_zen: false,
                zen_mode: ZenModeComponents::empty(),

                zen_horizontal: Constraint::Percentage(80),
                zen_vertical: Constraint::Percentage(90),
            },
            bindings: Keybindings {
                global: GlobalKeybindings {
                    scroll_down: keybinding!([KeyCode::Char('j');, KeyCode::Down;]),
                    scroll_up: keybinding!([KeyCode::Char('k');, KeyCode::Up;]),

                    scroll_to_top: keybinding!([KeyCode::Char('g');, KeyCode::Home;]),
                    scroll_to_bottom: keybinding!([KeyCode::Char('G'); SHIFT, KeyCode::End;]),

                    pop_popup: keybinding!([KeyCode::Esc;]),

                    half_down: keybinding!([KeyCode::Char('d'); CONTROL, KeyCode::PageDown;]),
                    half_up: keybinding!([KeyCode::Char('u'); CONTROL, KeyCode::PageUp;]),

                    unselect_scroll: keybinding!([KeyCode::Char('h');]),

                    submit: keybinding!([KeyCode::Enter;]),
                    quit: keybinding!([KeyCode::Char('q');, KeyCode::Char('c'); CONTROL]),

                    enter_search_bar: keybinding!([KeyCode::Char('i');]),
                    exit_search_bar: keybinding!([KeyCode::Esc;]),

                    switch_context_search: keybinding!([KeyCode::Char('s');]),
                    switch_context_page: keybinding!([KeyCode::Char('p');]),

                    toggle_search_language_selection: keybinding!([KeyCode::F(2);]),
                    toggle_logger: keybinding!([KeyCode::Char('l');]),
                },
                search: SearchKeybindings {
                    continue_search: keybinding!([KeyCode::Char('c');]),
                },
                page: PageKeybindings {
                    pop_page: keybinding!([KeyCode::Esc;]),
                    jump_to_header: keybinding!([KeyCode::Enter;]),
                    select_first_link: keybinding!([KeyCode::Left; SHIFT]),
                    select_last_link: keybinding!([KeyCode::Right; SHIFT]),
                    select_prev_link: keybinding!([KeyCode::Left;]),
                    select_next_link: keybinding!([KeyCode::Right;]),
                    open_link: keybinding!([KeyCode::Enter;]),
                    toggle_page_language_selection: keybinding!([KeyCode::F(3);]),
                    toggle_zen_mode: keybinding!([KeyCode::F(4);]),
                    toggle_toc: keybinding!([KeyCode::Tab;, KeyCode::BackTab;]),
                },
            },
            api: ApiConfig {
                endpoint: Endpoint::parse("https://en.wikipedia.org/w/api.php")
                    .expect("Hardcoded links should work"),
                language: Language::English,

                search_limit: 10,
                search_qiprofile: search::QiProfile::default(),
                search_type: search::SearchType::default(),
                search_info: search::Info::default(),
                search_rewrites: false,
                search_sort_order: search::SortOrder::Relevance,

                page_redirects: false,
            },
            ui: UiConfig {
                popup_search_language_changed: true,
                popup_page_language_changed: true,
            },
        }
    }
}

impl TocConfig {
    pub fn formatted_item(&self, number: &str, text: &str) -> String {
        const NUMBER_FMT: &str = "{NUMBER}";
        const TEXT_FMT: &str = "{TEXT}";

        self.item_format
            .replace(NUMBER_FMT, number)
            .replace(TEXT_FMT, text)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new()
    }
}

#[derive(Deserialize)]
struct UserConfig {
    page: Option<UserPageConfig>,
    bindings: Option<UserKeybindingsConfig>,
    api: Option<UserApiConfig>,
    ui: Option<UserUiConfig>,
}

#[derive(Deserialize)]
struct UserPageConfig {
    toc: Option<UserTocConfig>,
    padding: Option<PaddingConfig>,

    zen_mode: Option<UserZenModeConfig>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum UserConstraint {
    Min(u16),
    Max(u16),
    Length(u16),
    Percentage(u16),
    Ratio(u32, u32),
}

#[allow(clippy::from_over_into)]
impl Into<Constraint> for UserConstraint {
    fn into(self) -> Constraint {
        match self {
            UserConstraint::Min(u) => Constraint::Min(u),
            UserConstraint::Max(u) => Constraint::Max(u),
            UserConstraint::Length(u) => Constraint::Length(u),
            UserConstraint::Percentage(u) => Constraint::Percentage(u),
            UserConstraint::Ratio(u, v) => Constraint::Ratio(u, v),
        }
    }
}

#[derive(Deserialize)]
struct UserZenModeConfig {
    default: Option<bool>,
    include: Option<ZenModeComponents>,

    horizontal: Option<UserConstraint>,
    vertical: Option<UserConstraint>,
}

#[derive(Deserialize)]
struct UserTocConfig {
    enabled: Option<bool>,
    width_percentage: Option<u16>,
    position: Option<TocConfigPosition>,
    title: Option<TocConfigTitle>,
    item_format: Option<String>,

    enable_scrolling: Option<bool>,
}
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum UserKeyCodeInner {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    Esc,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum UserKeyCode {
    Char(char),
    NonChar(UserKeyCodeInner),
}

#[allow(clippy::from_over_into)]
impl Into<KeyCode> for UserKeyCode {
    fn into(self) -> KeyCode {
        match self {
            Self::Char(char) => KeyCode::Char(char),
            Self::NonChar(inner) => match inner {
                UserKeyCodeInner::Backspace => KeyCode::Backspace,
                UserKeyCodeInner::Enter => KeyCode::Enter,
                UserKeyCodeInner::Left => KeyCode::Left,
                UserKeyCodeInner::Right => KeyCode::Right,
                UserKeyCodeInner::Up => KeyCode::Up,
                UserKeyCodeInner::Down => KeyCode::Down,
                UserKeyCodeInner::Home => KeyCode::Home,
                UserKeyCodeInner::End => KeyCode::End,
                UserKeyCodeInner::PageUp => KeyCode::PageUp,
                UserKeyCodeInner::PageDown => KeyCode::PageDown,
                UserKeyCodeInner::Tab => KeyCode::Tab,
                UserKeyCodeInner::BackTab => KeyCode::BackTab,
                UserKeyCodeInner::Delete => KeyCode::Delete,
                UserKeyCodeInner::Insert => KeyCode::Insert,
                UserKeyCodeInner::Esc => KeyCode::Esc,
                UserKeyCodeInner::F1 => KeyCode::F(1),
                UserKeyCodeInner::F2 => KeyCode::F(2),
                UserKeyCodeInner::F3 => KeyCode::F(3),
                UserKeyCodeInner::F4 => KeyCode::F(4),
                UserKeyCodeInner::F5 => KeyCode::F(5),
                UserKeyCodeInner::F6 => KeyCode::F(6),
                UserKeyCodeInner::F7 => KeyCode::F(7),
                UserKeyCodeInner::F8 => KeyCode::F(8),
                UserKeyCodeInner::F9 => KeyCode::F(9),
                UserKeyCodeInner::F10 => KeyCode::F(10),
                UserKeyCodeInner::F11 => KeyCode::F(12),
                UserKeyCodeInner::F12 => KeyCode::F(13),
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum UserBinding {
    CodeOnly(UserKeyCode),
    Binding {
        code: UserKeyCode,
        modifiers: Option<KeyModifiers>,
    },
}

#[allow(clippy::from_over_into)]
impl Into<Binding> for UserBinding {
    fn into(self) -> Binding {
        match self {
            UserBinding::CodeOnly(code) => UserBinding::Binding {
                code,
                modifiers: None,
            }
            .into(),
            UserBinding::Binding { code, modifiers } => Binding {
                code: code.into(),
                modifiers: modifiers.unwrap_or(KeyModifiers::empty()),
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum UserKeybinding {
    SingleBinding(UserBinding),
    MultipleBindings(Vec<UserBinding>),
}

#[allow(clippy::from_over_into)]
impl Into<Keybinding> for UserKeybinding {
    fn into(self) -> Keybinding {
        match self {
            UserKeybinding::SingleBinding(binding) => Keybinding {
                bindings: vec![binding.into()],
            },
            UserKeybinding::MultipleBindings(bindings) => Keybinding {
                bindings: bindings.into_iter().map(|x| x.into()).collect(),
            },
        }
    }
}

macro_rules! user_keybindings {
    ($name:ident, $($binding:ident),+) => {
        #[derive(Deserialize)]
        struct $name {
            $($binding: Option<UserKeybinding>,)+
        }
    };
}

user_keybindings!(
    UserGlobalKeybindings,
    scroll_down,
    scroll_up,
    scroll_to_top,
    scroll_to_bottom,
    pop_popup,
    half_down,
    half_up,
    unselect_scroll,
    submit,
    quit,
    enter_search_bar,
    exit_search_bar,
    switch_context_search,
    switch_context_page,
    toggle_search_language_selection,
    toggle_logger
);

user_keybindings!(UserSearchKeybindings, continue_search);

user_keybindings!(
    UserPageKeybindings,
    pop_page,
    jump_to_header,
    select_first_link,
    select_last_link,
    select_prev_link,
    select_next_link,
    open_link,
    toggle_page_language_selection,
    toggle_zen_mode,
    toggle_toc
);

#[derive(Deserialize)]
struct UserKeybindingsConfig {
    global: Option<UserGlobalKeybindings>,
    search: Option<UserSearchKeybindings>,
    page: Option<UserPageKeybindings>,
}

#[derive(Deserialize)]
struct UserApiConfig {
    pre_language: Option<String>,
    language: Option<Language>,
    post_language: Option<String>,

    search_limit: Option<usize>,
    search_qiprofile: Option<search::QiProfile>,
    search_type: Option<search::SearchType>,
    search_info: Option<search::Info>,
    search_rewrites: Option<bool>,
    search_sort_order: Option<search::SortOrder>,

    page_redirects: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct UserUiConfig {
    popup_search_language_changed: Option<bool>,
    popup_page_language_changed: Option<bool>,
}

pub fn load_theme() -> Result<Theme> {
    let mut default_theme = Theme::default();
    let user_theme = load_user_theme().context("failed loading the user theme")?;

    override_options!(default_theme, user_theme::{
        bg,
        fg,

        title,

        selected_bg,
        selected_fg,

        inactive_fg,
        highlight_fg,

        border_fg,
        border_bg,
        border_type,

        border_highlight_fg,
        border_highlight_bg,

        scrollbar_track_fg,
        scrollbar_thumb_fg,

        search_title_fg,

        status_bar_fg,
        status_bar_bg
    });

    Ok(default_theme)
}

fn load_user_theme() -> Result<UserTheme> {
    let path = config_dir()
        .context("failed retrieving the config dir")?
        .join(THEME_FILE_NAME);

    if !path.exists() {
        std::fs::write(&path, "").context("failed creating the theme config file")?;
    }

    let user_theme_str =
        std::fs::read_to_string(&path).context("failed reading the theme config file")?;

    toml::from_str::<UserTheme>(&user_theme_str).context("failed parsing the user theme")
}

#[derive(Clone)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,

    pub title: Color,

    pub selected_bg: Color,
    pub selected_fg: Color,

    pub inactive_fg: Color,
    pub highlight_fg: Color,

    pub border_fg: Color,
    pub border_bg: Color,
    pub border_type: ThemeBorderType,

    pub border_highlight_fg: Color,
    pub border_highlight_bg: Color,

    pub scrollbar_track_fg: Color,
    pub scrollbar_thumb_fg: Color,

    pub search_title_fg: Color,

    pub status_bar_fg: Color,
    pub status_bar_bg: Color,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            bg: Color::Reset,
            fg: Color::Reset,

            title: Color::White,

            selected_bg: Color::DarkGray,
            selected_fg: Color::Reset,

            inactive_fg: Color::Blue,
            highlight_fg: Color::White,

            border_fg: Color::White,
            border_bg: Color::Reset,
            border_type: ThemeBorderType::Rounded,

            border_highlight_fg: Color::Yellow,
            border_highlight_bg: Color::Reset,

            scrollbar_track_fg: Color::Black,
            scrollbar_thumb_fg: Color::Blue,

            search_title_fg: Color::Red,

            status_bar_fg: Color::Reset,
            status_bar_bg: Color::DarkGray,
        }
    }

    /// Returns a Paragraph with the background and foreground colors set
    pub fn default_paragraph<'a, T>(&self, text: T) -> ratatui::widgets::Paragraph<'a>
    where
        T: Into<ratatui::text::Text<'a>>,
    {
        ratatui::widgets::Paragraph::new(text).style(Style::default().bg(self.bg).fg(self.fg))
    }

    /// Returns a Block with Borders::ALL, BorderType::Rounded and bg and fg colors set
    pub fn default_block(&self) -> ratatui::widgets::Block {
        ratatui::widgets::Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_type(self.border_type.clone().into())
            .border_style(Style::default().fg(self.border_fg).bg(self.border_bg))
            .title_style(Style::default().fg(self.title))
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::new()
    }
}

#[derive(Deserialize, Clone)]
pub enum ThemeBorderType {
    Plain,
    Rounded,
    Double,
    Thick,
    QuadrantInside,
    QuadrantOutside,
}

impl From<ThemeBorderType> for BorderType {
    fn from(val: ThemeBorderType) -> Self {
        match val {
            ThemeBorderType::Plain => BorderType::Plain,
            ThemeBorderType::Rounded => BorderType::Rounded,
            ThemeBorderType::Double => BorderType::Double,
            ThemeBorderType::Thick => BorderType::Thick,
            ThemeBorderType::QuadrantInside => BorderType::QuadrantInside,
            ThemeBorderType::QuadrantOutside => BorderType::QuadrantOutside,
        }
    }
}

#[derive(Deserialize)]
struct UserTheme {
    bg: Option<Color>,
    fg: Option<Color>,

    title: Option<Color>,

    selected_bg: Option<Color>,
    selected_fg: Option<Color>,

    inactive_fg: Option<Color>,
    highlight_fg: Option<Color>,

    border_fg: Option<Color>,
    border_bg: Option<Color>,
    border_type: Option<ThemeBorderType>,

    border_highlight_fg: Option<Color>,
    border_highlight_bg: Option<Color>,

    scrollbar_track_fg: Option<Color>,
    scrollbar_thumb_fg: Option<Color>,

    search_title_fg: Option<Color>,

    status_bar_fg: Option<Color>,
    status_bar_bg: Option<Color>,
}
