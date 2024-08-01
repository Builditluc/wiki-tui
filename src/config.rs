use anyhow::{bail, Result};
use directories::ProjectDirs;
use ratatui::style::{Color, Style};
use std::path::PathBuf;

pub const DATA_ENV: &str = "WIKI_TUI_DATA";
pub const CONFIG_ENV: &str = "WIKI_TUI_CONFIG";

pub fn project_dir() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "builditluc", "wiki-tui")
}

pub fn data_dir() -> Result<PathBuf> {
    let directory = if let Ok(dir) = std::env::var(DATA_ENV) {
        PathBuf::from(dir)
    } else if let Some(project_dir) = project_dir() {
        project_dir.data_local_dir().to_path_buf()
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

    pub border_highlight_fg: Color,
    pub border_highlight_bg: Color,

    pub scrollbar_track_fg: Color,
    pub scrollbar_thumb_fg: Color,

    pub search_title_fg: Color,
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

            border_highlight_fg: Color::Yellow,
            border_highlight_bg: Color::Reset,

            scrollbar_track_fg: Color::Black,
            scrollbar_thumb_fg: Color::Blue,

            search_title_fg: Color::Red,
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
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(self.border_fg).bg(self.border_bg))
            .title_style(Style::default().fg(self.title))
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::new()
    }
}
