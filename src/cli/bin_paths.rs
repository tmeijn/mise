use eyre::Result;

use crate::config::Config;
use crate::toolset::ToolsetBuilder;

/// List all the active runtime bin paths
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment)]
pub struct BinPaths {}

impl BinPaths {
    pub fn run(self) -> Result<()> {
        let config = Config::try_get()?;
        let ts = ToolsetBuilder::new().build(&config)?;
        ts.notify_if_versions_missing();
        for p in ts.list_paths() {
            miseprintln!("{}", p.display());
        }
        Ok(())
    }
}
