use clap::{CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};

use crate::Cli;

#[derive(Parser)]
pub struct CompletionCommand {
    /// Shell type to generate completions for
    pub shell: Shell,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

pub fn completions(cmd: CompletionCommand) {
    let mut cli = Cli::command();
    print_completions(cmd.shell, &mut cli);
}
