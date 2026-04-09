use crate::utils::command_ok;

use super::Tool;

#[derive(Debug, Default)]
pub(crate) struct KubectlCli {}

impl Tool for KubectlCli {
    const NAME: &str = "kubectl";

    const INSTALL_LINK: &str = "https://kubernetes.io/docs/tasks/tools/#kubectl";

    fn is_installed(&self) -> eyre::Result<bool> {
        let mut cmd = self.cmd();
        cmd.arg("version");

        Ok(command_ok(cmd))
    }
}
