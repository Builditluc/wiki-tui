use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
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

pub fn match_cli() {
    let cli = Cli::parse();

    match &cli.commands {
        Some(Commands::Debug(command)) => command_debug(command),
        None => {}
    }
}

fn command_debug(command: &DebugCommand) {
    if command.list {
        use crate::config::{config_dir, data_dir, CONFIG_ENV, DATA_ENV};

        println!("wiki-tui DEBUG: Debug Information");
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
}
