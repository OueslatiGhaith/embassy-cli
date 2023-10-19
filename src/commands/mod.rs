use clap::Subcommand;

use self::{completions::CompletionsCommand, create::CreateCommand};

pub mod completions;
pub mod create;

#[derive(Subcommand)]
pub enum Command {
    /// create a new Embassy project
    Create(CreateCommand),
    Completions(CompletionsCommand),
}
