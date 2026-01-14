use super::{
    Context,
    DevTool,
};

#[derive(Debug, Clone, clap::Parser)]
pub(crate) struct Info {}

impl DevTool for Info {
    fn run(self, ctx: Context) -> anyhow::Result<()> {
        println!("cargo: {}", ctx.cargo.to_str().unwrap());
        println!("r{}.{}", ctx.run_number, ctx.revision);
        Ok(())
    }
}
