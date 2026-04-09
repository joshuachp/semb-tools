use clap::Args;
use tracing::instrument;

#[derive(Debug, Clone, Args)]
pub(crate) struct Kind {}

impl Kind {
    #[instrument(skip_all)]
    pub(crate) fn run(self) -> eyre::Result<()> {
        todo!()
    }
}
