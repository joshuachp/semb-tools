use std::path::{Path, PathBuf};
use std::{fs, io};

use eyre::{Context, ContextCompat};
use serde::Deserialize;
use tracing::instrument;

use crate::cli::kind::KindConfig;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    pub(crate) kind: KindConfig,
}

impl Config {
    pub(crate) fn path() -> eyre::Result<PathBuf> {
        dirs::config_dir()
            .map(|mut path| {
                path.push(env!("CARGO_CRATE_NAME"));

                path
            })
            .wrap_err("couldn't get config dir")
    }

    #[instrument]
    pub(crate) fn read(config_dir: &Path, custom: Option<&Path>) -> eyre::Result<Self> {
        if let Some(custom) = custom {
            let config = fs::read(custom)?;

            toml::from_slice(&config).wrap_err("couldn't read config")
        } else {
            Config::with_config_dir(config_dir)
        }
    }

    #[instrument]
    fn with_config_dir(config_dir: &Path) -> eyre::Result<Self> {
        let path = config_dir.join("config.toml");

        match fs::read(path) {
            Ok(config) => toml::from_slice(&config).wrap_err("couldn't read config"),
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                Ok(Config::with_default(config_dir))
            }
            Err(err) => Err(err).wrap_err("couldn't read config file"),
        }
    }

    fn with_default(config_dir: &Path) -> Self {
        let kind = KindConfig::with_default(config_dir);

        Self { kind }
    }
}
