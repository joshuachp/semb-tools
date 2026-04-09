use crate::utils::command_ok;

use super::Tool;

#[derive(Debug, Default)]
pub(crate) struct HelmCli {}

impl Tool for HelmCli {
    const NAME: &str = "helm";

    const INSTALL_LINK: &str = "https://helm.sh/docs/intro/install";

    fn is_installed(&self) -> eyre::Result<bool> {
        let mut cmd = self.cmd();
        cmd.arg("version");

        Ok(command_ok(cmd))
    }
}
