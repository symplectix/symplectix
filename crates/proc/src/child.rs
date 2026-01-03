use std::io;
use std::process::{
    Command,
    ExitStatus,
};
use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwap;
use tokio::process::{
    Child as TokioChild,
    ChildStderr,
    Command as TokioCommand,
};
use tokio::time;
use tracing::trace;

use crate::Flags;

#[derive(Debug)]
pub(crate) struct Child {
    inner: TokioChild,

    pub(crate) pid:   u32,
    pub(crate) flags: Arc<ArcSwap<Flags>>,
}

pub(crate) fn spawn(target: Command, flags: Arc<ArcSwap<Flags>>) -> io::Result<Child> {
    let inner = TokioCommand::from(target).kill_on_drop(false).spawn()?;
    let pid = inner.id().expect("fetching the process id before polling should not fail");
    Ok(Child { inner, pid, flags })
}

impl Child {
    pub(crate) fn stderr(&mut self) -> Option<ChildStderr> {
        self.inner.stderr.take()
    }

    /// Waits until the process exits or times out.
    /// For the case of timeout, Ok(None) will be returned.
    pub(crate) async fn wait(&mut self) -> io::Result<Option<ExitStatus>> {
        let opts = self.flags.load();
        match opts.timeout.kill_after {
            // Always some because no timeout given.
            None => self.inner.wait().await.map(Some),
            Some(dur) => match time::timeout(dur, self.inner.wait()).await {
                Err(_elapsed) => Ok(None),
                Ok(status) => status.map(Some),
            },
        }
    }

    #[tracing::instrument(skip(self))]
    pub(crate) async fn kill(&mut self, signal: Option<libc::c_int>) {
        let gracefully = true;
        let pid = self.pid as libc::pid_t;

        // Notify the spawned process to be terminated.
        if gracefully {
            let signal = signal.unwrap_or(libc::SIGTERM);
            if let Err(err) = kill(pid, signal) {
                trace!("kill {}: {}", signal, err);
            }
            time::sleep(Duration::from_millis(1000)).await;
        }

        if let Err(err) = kill(pid, libc::SIGKILL) {
            trace!("kill {}: {}", libc::SIGKILL, err);
        }
    }

    pub(crate) fn killpg(&mut self) {
        // Reap all descendant processes here,
        // to ensure there are no children left behind.
        if let Err(err) = killpg(self.pid as libc::c_int, libc::SIGKILL) {
            trace!("killpg: {}", err);
        }
    }
}

pub(crate) fn kill(pid: libc::pid_t, sig: libc::c_int) -> io::Result<()> {
    unsafe { if libc::kill(pid, sig) == 0 { Ok(()) } else { Err(io::Error::last_os_error()) } }
}

fn killpg(grp: libc::pid_t, sig: libc::c_int) -> io::Result<()> {
    unsafe { if libc::killpg(grp, sig) == 0 { Ok(()) } else { Err(io::Error::last_os_error()) } }
}
