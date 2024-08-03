use clap::Parser;

use crate::{
    action::{Action, ActionPacket, SearchAction},
    config::{cache_dir, config_dir, CONFIG_FILE_NAME, THEME_FILE_NAME},
};
use wiki_api::languages::Language;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Search for an article
    #[arg(value_name = "QUERY")]
    search_query: Option<String>,

    /// Override the configured search language of wikipedia. The value can be either the language
    /// code, the name of the language in english or the native language name
    #[arg(value_name = "LANGUAGE", short = 'l', long = "language")]
    language: Option<Language>,

    /// Override the configured logging level
    #[arg(value_name = "LEVEL", long = "level")]
    level: Option<tracing::level_filters::LevelFilter>,

    /// Print the path to the cache directory
    #[arg(long = "cache-dir")]
    print_cache_dir: bool,

    /// Print the path to the config file
    #[arg(long = "config-path")]
    print_config_path: bool,

    /// Print the path to the theme configuration file
    #[arg(long = "theme-config-path")]
    print_theme_config_path: bool,

    #[cfg(debug_assertions)]
    #[arg(value_name = "PATH", long = "page")]
    load_debug_page: Option<std::path::PathBuf>,
}

pub struct CliResults {
    pub actions: Option<ActionPacket>,
    pub log_level: Option<tracing::level_filters::LevelFilter>,
}

pub fn match_cli() -> CliResults {
    let cli = Cli::parse();

    let mut should_quit = false;
    let mut results = CliResults {
        actions: None,
        log_level: None,
    };

    let mut packet = ActionPacket::default();

    if let Some(language) = cli.language {
        packet.add_action(Action::Search(SearchAction::ChangeLanguage(language)));
    }

    if let Some(level) = cli.level {
        results.log_level = Some(level);
    }

    if let Some(search_query) = cli.search_query {
        packet.add_action(Action::ExitSearchBar);
        packet.add_action(Action::SwitchContextSearch);
        packet.add_action(Action::Search(SearchAction::StartSearch(search_query)));
    }

    if cli.print_config_path {
        let config_path = config_dir().map(|x| x.join(CONFIG_FILE_NAME));
        println!(
            "{}",
            config_path
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or("UNKNOWN".to_string())
        );
        should_quit = true;
    }

    if cli.print_cache_dir {
        let cache_dir = cache_dir();
        println!(
            "{}",
            cache_dir
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or("UNKNOWN".to_string())
        );
        should_quit = true;
    }

    if cli.print_theme_config_path {
        let theme_config_path = config_dir().map(|x| x.join(THEME_FILE_NAME));
        println!(
            "{}",
            theme_config_path
                .map(|x| x.to_string_lossy().to_string())
                .unwrap_or("UNKNOWN".to_string())
        );
        should_quit = true;
    }

    #[cfg(debug_assertions)]
    if let Some(ref debug_page) = cli.load_debug_page {
        if let Some(page) = wiki_api::page::Page::from_path(debug_page) {
            packet.add_action(Action::SwitchContextPage);
            packet.add_action(Action::PageViewer(
                crate::action::PageViewerAction::DisplayPage(page),
            ));
        }
    }
    results.actions = Some(packet);

    if should_quit {
        std::process::exit(libc::EXIT_SUCCESS)
    }

    results
}
