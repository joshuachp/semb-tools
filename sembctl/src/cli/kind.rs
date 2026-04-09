use std::path::{Path, PathBuf};

use clap::{Args, Subcommand};
use serde::Deserialize;
use tracing::{info, instrument};

use crate::config::Config;
use crate::tools::helm::HelmCli;
use crate::tools::kind::KindCli;
use crate::tools::kubectl::KubectlCli;
use crate::tools::{Deps, Tool, ToolCheck};
use crate::utils::spawn_cmd;

const KIND_CONFIG: &str = include_str!("../../assets/kind-config.yaml");

#[derive(Debug, Default)]
struct KindDeps {
    kind: KindCli,
    kubectl: KubectlCli,
    helm: HelmCli,
}

impl Deps for KindDeps {
    fn deps(&self) -> eyre::Result<impl AsRef<[ToolCheck]>> {
        let Self {
            kind,
            kubectl,
            helm,
        } = self;

        Ok([kind.check()?, kubectl.check()?, helm.check()?])
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KindConfig {
    cluster: KindClusterConfig,
}
impl KindConfig {
    pub(crate) fn with_default(config_dir: &Path) -> Self {
        let cluster = KindClusterConfig::with_default(&config_dir);

        Self { cluster }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct KindClusterConfig {
    kubeconfig: PathBuf,
}

impl KindClusterConfig {
    pub(crate) fn with_default(config_dir: &Path) -> Self {
        let mut kubeconfig = config_dir.join("kind");
        kubeconfig.push("kubeconfig.yaml");

        Self { kubeconfig }
    }
}

/// Manages a local kind cluster
#[derive(Debug, Clone, Args)]
pub(crate) struct Kind {
    #[clap(subcommand)]
    command: Command,
}

/// Kind cluster commands
#[derive(Debug, Clone, Subcommand)]
pub(crate) enum Command {
    /// Create a clean cluster
    Create,
}

impl Kind {
    #[instrument(skip_all)]
    pub(crate) fn run(self, config: &Config) -> eyre::Result<()> {
        match self.command {
            Command::Create => self.create(config),
        }
    }

    #[instrument(skip_all)]
    fn create(self, config: &Config) -> eyre::Result<()> {
        KindDeps::default().check()?;

        info!("Deleting existing cluster");

        let mut cmd = std::process::Command::new("kind");
        cmd.args(["delete", "cluster"]);

        spawn_cmd(cmd)?;

        info!("Creating new cluster");

        let mut cmd = std::process::Command::new("kind");
        cmd.args(["create", "cluster", "--kubeconfig"])
            .arg(&config.kind.cluster.kubeconfig);

        spawn_cmd(cmd)?;

        Ok(())
    }
}
