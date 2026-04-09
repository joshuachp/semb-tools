use crate::utils::command_ok;

use super::Tool;

#[derive(Debug, Default)]
pub(crate) struct KindCli {}

impl Tool for KindCli {
    const NAME: &str = "kind";

    const INSTALL_LINK: &str = "https://kind.sigs.k8s.io/docs/user/quick-start/#installation";

    fn is_installed(&self) -> eyre::Result<bool> {
        let mut cmd = self.cmd();
        cmd.arg("--version");

        Ok(command_ok(cmd))
    }
}
