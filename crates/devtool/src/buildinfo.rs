include!(concat!(env!("OUT_DIR"), "/buildinfo.rs"));

use super::{
    Context,
    DevTool,
};

#[derive(Debug, Clone, clap::Parser)]
pub(crate) struct BuildInfo {}

impl DevTool for BuildInfo {
    fn run(self, _ctx: Context) -> anyhow::Result<()> {
        buildinfo();
        Ok(())
    }
}
