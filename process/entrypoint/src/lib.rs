use std::io;

use process::{Command, ExitStatus, Process};
use tokio::signal::unix::{signal, SignalKind};

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(io::Error),

    #[error("the spawned child exited unsuccessfully: {0}")]
    ExitedUnsuccessfully(ExitStatus),

    #[error("the spawned child timedout: {0}")]
    Timedout(ExitStatus),
}

#[tracing::instrument(skip(command))]
pub async fn run(command: &Command) -> Result {
    let process = command.spawn().await.map_err(Error::Io)?;
    wait(process).await
}

#[tracing::instrument(skip(process))]
pub async fn wait(mut process: Process) -> Result {
    let mut interrupt = signal(SignalKind::interrupt()).map_err(Error::Io)?;
    let mut terminate = signal(SignalKind::terminate()).map_err(Error::Io)?;

    let timedout = tokio::select! {
        biased;
        _ = interrupt.recv() => { false },
        _ = terminate.recv() => { false },
        r = process.wait() => matches!(r, Ok(None)),
    };

    let exit_status = process.stop(true).await.map_err(Error::Io)?;

    if timedout {
        Err(Error::Timedout(exit_status))
    } else if !exit_status.success() {
        Err(Error::ExitedUnsuccessfully(exit_status))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn run_process() {
        use Error::*;

        let sleep = wait(Command::new("sleep").arg("0.1").spawn().await.unwrap());
        let Ok(_) = sleep.await else {
            panic!("expected that the command 'sleep' exit successfully");
        };

        let sleep = wait(
            Command::new("sleep")
                .arg("10")
                .timeout(Duration::from_millis(10))
                .spawn()
                .await
                .unwrap(),
        );
        let Err(Timedout(_exit_status)) = sleep.await else {
            panic!("expected that the command 'sleep' times out");
        };
    }
}
