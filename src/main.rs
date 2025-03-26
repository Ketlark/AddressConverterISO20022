use std::io::Error;

use addressconverteriso20022::app::commands::create::CreateCommand;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "addressconverteriso20022")]
#[command(about = "A CLI tool to manage postal addresses", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new postal address interactively
    Create,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Commands::Create => CreateCommand::execute().await,
    }
}
