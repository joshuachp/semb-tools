use std::process::Command;

use comfy_table::Row;
use eyre::eyre;

pub(crate) mod helm;
pub(crate) mod kind;
pub(crate) mod kubectl;

pub(crate) trait Tool {
    const NAME: &str;
    const INSTALL_LINK: &str;

    fn cmd(&self) -> Command {
        Command::new(Self::NAME)
    }

    fn is_installed(&self) -> eyre::Result<bool>;

    fn check(&self) -> eyre::Result<ToolCheck> {
        let present = self.is_installed()?;

        Ok(ToolCheck {
            name: Self::NAME,
            present,
            install: Self::INSTALL_LINK,
        })
    }
}

#[derive(Debug)]
pub(crate) struct ToolCheck {
    name: &'static str,
    present: bool,
    install: &'static str,
}

impl From<&ToolCheck> for Row {
    fn from(value: &ToolCheck) -> Self {
        let ToolCheck {
            name,
            present,
            install,
        } = value;

        let check = if *present { "ok" } else { "missing" };

        Row::from([name, check, install])
    }
}

pub(crate) trait Deps {
    fn deps(&self) -> eyre::Result<impl AsRef<[ToolCheck]>>;

    fn check(&self) -> eyre::Result<()> {
        let deps = self.deps()?;
        let deps = deps.as_ref();

        let is_ok = deps.iter().all(|tool| tool.present);

        if is_ok {
            self.print(&deps);

            Ok(())
        } else {
            Err(eyre!("missing dependencies"))
        }
    }

    fn print(&self, deps: &[ToolCheck]) {
        let mut table = comfy_table::Table::new();
        table
            .load_preset(comfy_table::presets::UTF8_FULL_CONDENSED)
            .set_header(["Tool", "Check", "Instruction"]);

        for tool in deps {
            table.add_row(tool);
        }

        println!("{table}");
    }
}
