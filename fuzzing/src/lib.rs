use std::future::Future;

pub trait Op {
    fn run(self) -> impl Future<Output = anyhow::Result<()>> + Send;
}

pub async fn run<T: Op>(op: T) -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_ansi(false)
        .compact()
        .init();

    op.run().await
}
