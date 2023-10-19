use clap::Parser;
use commands::{completion::completions, create::create, Command};

mod commands;
mod generator;
mod git;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Create(cc) => create(cc).await?,
        Command::Completion(cc) => completions(cc),
    }

    Ok(())
}
