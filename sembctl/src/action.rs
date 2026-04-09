use crate::tools::Deps;

pub(crate) trait Run {
    type Deps: Deps;

    fn deps(&self) -> eyre::Result<Self::Deps>;

    fn actions(&self) -> eyre::Result<Self::Deps>;

    fn run(&mut self) -> eyre::Result<()> {
        self.deps()?.check()?;

        Ok(())
    }
}
