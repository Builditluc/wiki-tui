use clap::{Args, Parser, Subcommand};

use crate::action::{Action, ActionPacket, SearchAction};

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
}

pub fn match_cli() -> Option<ActionPacket> {
    let cli = Cli::parse();

    let mut packet = ActionPacket::default();

    if let Some(search_query) = cli.search_query {
        packet.add_action(Action::ExitSearchBar);
        packet.add_action(Action::SwitchContextSearch);
        packet.add_action(Action::Search(SearchAction::StartSearch(search_query)));
    }

    match &cli.commands {
        Some(Commands::Debug(command)) => command_debug(command),
        None => {}
    }

    Some(packet)
}

fn command_debug(command: &DebugCommand) {
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

    std::process::exit(libc::EXIT_SUCCESS)
}
