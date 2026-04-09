use std::io::IsTerminal;

use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod cli;
mod completions;
mod config;
mod kind;

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
