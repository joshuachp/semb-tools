use std::io::IsTerminal;

use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub(crate) mod action;
pub(crate) mod cli;
pub(crate) mod config;
pub(crate) mod tools;
pub(crate) mod utils;

fn main() -> eyre::Result<()> {
    let cli = cli::Cli::parse();

    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_ansi(std::io::stdout().is_terminal()))
        .with(tracing_error::ErrorLayer::default())
        .with(cli.level_filter())
        .try_init()?;

    cli.run()?;

    Ok(())
}
