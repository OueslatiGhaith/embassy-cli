use clap::Subcommand;

use self::{completion::CompletionCommand, create::CreateCommand};

pub mod completion;
pub mod create;

#[derive(Subcommand)]
pub enum Command {
    /// Create a new Embassy project
    Create(CreateCommand),
    /// Generate shell completions
    Completion(CompletionCommand),
}
