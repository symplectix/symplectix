use std::convert::Infallible;
use std::io;
use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;
use std::sync::LazyLock;

use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::SendError;
use tokio::task;
use tracing::{error, trace};

static REAPER: LazyLock<Reaper<Result<usize, Infallible>>> = LazyLock::new(Reaper::start);

struct Reaper<T> {
    tx: broadcast::Sender<(libc::c_int, ExitStatus)>,
    jh: task::JoinHandle<T>,
}

impl Reaper<Result<usize, Infallible>> {
    fn start() -> Self {
        let (tx, _rx) = broadcast::channel(16);
        let tx_cloned = tx.clone();
        let jh = task::spawn(async move {
            let mut signal = signal(SignalKind::child()).expect("bug");
            let mut reaped = 0;
            while signal.recv().await.is_some() {
                loop {
                    // Waits for any child process.
                    //
                    // The WNOHANG option is used to indicate that the call should not block
                    // if there are no processes that wish to report status.
                    let mut status: libc::c_int = 0;
                    match unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) } {
                        -1 => {
                            // If RawOsError was constructed via last_os_error,
                            // then this function always return Some.
                            match io::Error::last_os_error().raw_os_error().expect("bug") {
                                libc::ECHILD => {
                                    // We have no children that it has not yet waited for.
                                    break;
                                }
                                libc::EINTR => {
                                    // This likely can't happen since we are calling libc::waitpid with WNOHANG.
                                    trace!("got interrupted, continue reaping");
                                }
                                errno => {
                                    error!(errno, "an error is detected or a caught signal aborts the call");
                                    // break;
                                }
                            }
                        }
                        0 => {
                            // no processes wish to report status
                            break;
                        }
                        pid => {
                            match tx.send((pid, ExitStatus::from_raw(status))) {
                                Ok(subscribers) => {
                                    trace!(
                                        pid,
                                        subscribers,
                                        "reaped ok {}, {}",
                                        libc::WIFEXITED(status),
                                        libc::WEXITSTATUS(status)
                                    );
                                }
                                Err(SendError((pid, exit_status))) => {
                                    // no active receivers
                                    trace!(
                                        pid,
                                        "reaped err {}, {}, {}",
                                        exit_status,
                                        libc::WIFEXITED(status),
                                        libc::WEXITSTATUS(status)
                                    );
                                }
                            }

                            reaped += 1;
                        }
                    }
                }
            }

            unreachable!("reaped {}", reaped);
            #[allow(unreachable_code)]
            Ok(reaped)
        });

        Reaper { tx: tx_cloned, jh }
    }
}

pub struct Channel {
    rx: broadcast::Receiver<(libc::c_int, ExitStatus)>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RecvError(broadcast::error::RecvError);

impl Channel {
    pub async fn recv(&mut self) -> Result<(libc::c_int, ExitStatus), RecvError> {
        self.rx.recv().await.map_err(RecvError)
    }
}

impl RecvError {
    pub fn closed(&self) -> bool {
        matches!(self.0, broadcast::error::RecvError::Closed)
    }

    pub fn lagged(&self) -> Option<u64> {
        if let broadcast::error::RecvError::Lagged(n) = self.0 { Some(n) } else { None }
    }
}

pub fn subscribe() -> Channel {
    Channel { rx: REAPER.tx.subscribe() }
}

pub fn abort() {
    REAPER.jh.abort();
}
