use anyhow::{bail, Context, Result};
use directories::ProjectDirs;
use ratatui::{
    style::{Color, Style},
    widgets::BorderType,
};
use serde::Deserialize;
use std::path::PathBuf;

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

    Ok(directory)
}

pub fn load_theme() -> Result<Theme> {
    macro_rules! override_color {
        ($default: expr, $user: expr, $color: ident) => {
            if let Some(color) = $user.$color {
                $default.$color = color;
            }
        };
    }

    let mut default_theme = Theme::default();
    let user_theme = load_user_theme().context("failed loading the user theme")?;

    override_color!(default_theme, user_theme, bg);
    override_color!(default_theme, user_theme, fg);

    override_color!(default_theme, user_theme, title);

    override_color!(default_theme, user_theme, selected_bg);
    override_color!(default_theme, user_theme, selected_fg);

    override_color!(default_theme, user_theme, inactive_fg);
    override_color!(default_theme, user_theme, highlight_fg);

    override_color!(default_theme, user_theme, border_fg);
    override_color!(default_theme, user_theme, border_bg);
    override_color!(default_theme, user_theme, border_type);

    override_color!(default_theme, user_theme, border_highlight_fg);
    override_color!(default_theme, user_theme, border_highlight_bg);

    override_color!(default_theme, user_theme, scrollbar_track_fg);
    override_color!(default_theme, user_theme, scrollbar_thumb_fg);

    override_color!(default_theme, user_theme, search_title_fg);

    override_color!(default_theme, user_theme, status_bar_fg);
    override_color!(default_theme, user_theme, status_bar_bg);

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
