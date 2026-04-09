use clap::{Args, CommandFactory, ValueEnum};
use tracing::instrument;

use crate::cli::Cli;

/// Generate shell completions.
#[derive(Debug, Clone, Args)]
pub(crate) struct Completion {
    /// Shell to generate the completions for.
    shell: Shell,
}

impl Completion {
    #[instrument(skip_all)]
    pub(crate) fn run(self) -> eyre::Result<()> {
        let shell = clap_complete::Shell::from(self.shell);

        let mut stdout = std::io::stdout().lock();

        clap_complete::generate(
            shell,
            &mut Cli::command(),
            env!("CARGO_CRATE_NAME"),
            &mut stdout,
        );

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Shell {
    Bash,
    Fish,
    Zsh,
}

impl From<Shell> for clap_complete::Shell {
    fn from(value: Shell) -> Self {
        match value {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::Zsh => clap_complete::Shell::Zsh,
        }
    }
}
