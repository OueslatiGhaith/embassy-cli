use clap::{CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};

use crate::Cli;

#[derive(Parser)]
pub struct CompletionsCommand {
    pub shell: Shell,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

pub fn completions(cmd: CompletionsCommand) {
    let mut cli = Cli::command();
    print_completions(cmd.shell, &mut cli);
}
