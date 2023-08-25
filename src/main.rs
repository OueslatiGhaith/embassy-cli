use clap::{Parser, Subcommand};
use generator::create;

mod generator;
mod git;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// create a new Embassy project
    Create(CreateCommand),
}

#[derive(Parser)]
pub struct CreateCommand {
    /// Project name
    #[clap(short, long)]
    name: Option<String>,
    /// Vendor
    #[clap(short, long)]
    vendor: Option<String>,
    /// MCU
    #[clap(short, long)]
    mcu: Option<String>,
    /// Do not pin to the latest commit of the Embassy crate
    #[clap(long)]
    no_pin: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Create(cmd) => create(cmd).await?,
    }

    Ok(())
}
