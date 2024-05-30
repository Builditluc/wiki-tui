use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::action::{Action, ActionPacket, PageViewerAction, SearchAction};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Search for an article
    #[arg(value_name = "QUERY")]
    search_query: Option<String>,

    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Debug(DebugCommand),
}

#[derive(Args)]
struct DebugCommand {
    /// Print debug information
    #[arg(short, long)]
    list: bool,
    /// Load a custom page
    #[arg(short, long)]
    page: Option<PathBuf>,
}

pub fn match_cli() -> Option<ActionPacket> {
    let cli = Cli::parse();

    let mut packet = ActionPacket::default();

    if let Some(search_query) = cli.search_query {
        packet.add_action(Action::ExitSearchBar);
        packet.add_action(Action::SwitchContextSearch);
        packet.add_action(Action::Search(SearchAction::StartSearch(search_query)));
    }

    packet = match &cli.commands {
        Some(Commands::Debug(command)) => command_debug(command, packet),
        None => packet
    };

    Some(packet)
}

fn command_debug(command: &DebugCommand, mut packet: ActionPacket) ->  ActionPacket {
    println!("wiki-tui DEBUG: Debug Information");

    if command.list {
        use crate::config::{config_dir, data_dir, CONFIG_ENV, DATA_ENV};
        println!("Config Values:");

        let data_dir = data_dir()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or("NO_PATH".to_string());
        println!("\t data_dir: '{}' ({})", data_dir, DATA_ENV);

        let config_dir = config_dir()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or("NO_PATH".to_string());
        println!("\t config_dir: '{}' ({})", config_dir, CONFIG_ENV);

        std::process::exit(libc::EXIT_SUCCESS)
    }

    if let Some(ref page_path) = command.page {
        #[cfg(debug_assertions)]
        {
            if let Some(page) = wiki_api::page::Page::from_path(page_path) {
                packet.add_action(Action::SwitchContextPage);
                packet.add_action(Action::PageViewer(PageViewerAction::DisplayPage(page)));
            }
        }
    }

    packet
}
