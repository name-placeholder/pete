//! Custom types for tracing errors.
use std::io;

use crate::ptracer::{Pid, Restart};

/// Alias for `Result<T, self::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// A tracing error.
///
/// Tracees are controlled by the tracer, but still impacted by their environment. In
/// particular, they may die unexpectedly while in ptrace-stop. Some errors can be due to
/// this, and will not necessarily be followed by a `wait()` status reporting the tracee's
/// death. This should be observed as an error with a `source` of
/// `nix::Error::Sys(Errno::ESRCH)`.
///
/// For this reason, tracing programs may need to handle an `ESRCH` error on any
/// `ptrace()` operation.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not attach to tracee = {pid}")]
    Attach {
        pid: Pid,
        source: nix::Error,
    },

    #[error("Could not restart tracee = {pid} with mode = {mode:?}")]
    Restart { pid: Pid, mode: Restart, source: nix::Error },

    #[error("Input/output error")]
    IO(#[from] io::Error),

    #[error("OS error")]
    OS(#[from] nix::Error),

    #[error("Internal error: {0}. Please open an issue at https://github.com/ranweiler/pete/issues")]
    Internal(String),
}

macro_rules! internal_error {
    ($ctx: expr) => {
        return Err($crate::error::Error::Internal($ctx.into()));
    }
}
