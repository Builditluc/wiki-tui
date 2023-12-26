use anyhow::{bail, Result};
use directories::ProjectDirs;
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
