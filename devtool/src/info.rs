use super::{
    Context,
    DevTool,
};

#[derive(Debug, Clone, clap::Parser)]
pub(crate) struct Info {}

impl DevTool for Info {
    fn run(self, ctx: Context) -> anyhow::Result<()> {
        println!("{}", ctx.workspace_status.version());
        Ok(())
    }
}
