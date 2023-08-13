use clap::{Parser, Subcommand};
use generator::create;

mod generator;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// create a new Embassy project
    Create {
        /// Project name
        #[clap(short, long)]
        name: Option<String>,
        /// Vendor
        #[clap(short, long)]
        vendor: Option<String>,
        /// MCU
        #[clap(short, long)]
        mcu: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Create { name, vendor, mcu } => create(name, vendor, mcu)?,
    }

    Ok(())
}
