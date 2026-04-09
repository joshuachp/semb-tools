use clap::{Parser, Subcommand};
use tracing::level_filters::LevelFilter;

use crate::completions::Completion;
use crate::kind::Kind;

/// Cli tools
#[derive(Debug, Parser)]
#[clap(version, about)]
pub(crate) struct Cli {
    /// Log level to use
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    #[clap(subcommand)]
    command: Command,
}

impl Cli {
    pub(crate) fn run(self) -> eyre::Result<()> {
        match self.command {
            Command::Completion(completions) => completions.run(),
            Command::Kind(kind) => kind.run(),
        }
    }

    pub(crate) fn level_filter(&self) -> LevelFilter {
        match self.verbose {
            0 => LevelFilter::WARN,
            1 => LevelFilter::INFO,
            2 => LevelFilter::DEBUG,
            3.. => LevelFilter::TRACE,
        }
    }
}

/// Command to run
#[derive(Debug, Clone, Subcommand)]
pub(crate) enum Command {
    /// Generates shell completion
    Completion(Completion),
    /// Creates a Kind cluster locally
    Kind(Kind),
}
