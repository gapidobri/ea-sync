mod cmd;
mod ea;

extern crate dotenv;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Timetable {
    events: Vec<TimetableEvent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TimetableEvent {
    date: String,
    from: String,
    to: String,
    title: String,
    classroom: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// eAsistent username
    #[arg(short, long, env = "EA_USERNAME")]
    username: String,

    /// eAsistent password
    #[arg(short, long, env = "EA_PASSWORD")]
    password: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Sync with Google Calendar
    Sync,

    /// Start an iCal server
    Serve(cmd::serve::Args),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Sync => {
            cmd::sync::execute(&cli).await?;
        }
        Commands::Serve(args) => {
            cmd::serve::execute(&cli, args).await?;
        }
    }

    Ok(())
}
