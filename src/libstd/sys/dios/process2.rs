#![allow(unused_variables)] // TODO: STUB

use prelude::v1::*;

use ffi::OsStr;
use fmt;
use io;
use sys::unimpl;
use sys::pipe2::AnonPipe;

////////////////////////////////////////////////////////////////////////////////
// Command
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Command {
    pub program: u32,
    pub args: Vec<u32>,
}

impl Command {
    pub fn new(program: &OsStr) -> Command {
        Command {
            program: 0,
            args: Vec::new()
        }
    }

    pub fn arg(&mut self, arg: &OsStr) { }
    pub fn args<'a, I: Iterator<Item = &'a OsStr>>(&mut self, args: I) { }
    fn init_env_map(&mut self) { }
    pub fn env(&mut self, key: &OsStr, val: &OsStr) { }
    pub fn env_remove(&mut self, key: &OsStr) { }
    pub fn env_clear(&mut self) { }
    pub fn cwd(&mut self, dir: &OsStr) { }
}

////////////////////////////////////////////////////////////////////////////////
// Processes
////////////////////////////////////////////////////////////////////////////////

/// Unix exit statuses
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ExitStatus {
    /// Normal termination with an exit code.
    Code(i32),

    /// Termination by signal, with the signal number.
    ///
    /// Never generated on Windows.
    Signal(i32),
}

impl ExitStatus {
    pub fn success(&self) -> bool {
        *self == ExitStatus::Code(0)
    }
    pub fn code(&self) -> Option<i32> {
        match *self {
            ExitStatus::Code(c) => Some(c),
            _ => None
        }
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExitStatus::Code(code) =>  write!(f, "exit code: {}", code),
            ExitStatus::Signal(code) =>  write!(f, "signal: {}", code),
        }
    }
}

/// The unique id of the process (this should never be negative).
pub struct Process;

impl Process {
    pub unsafe fn kill(&self) -> io::Result<()> { Err(unimpl()) }
    pub fn spawn(cfg: &Command,
                 in_fd: Option<AnonPipe>, out_fd: Option<AnonPipe>, err_fd: Option<AnonPipe>)
                 -> io::Result<Process>
    { Err(unimpl()) }

    pub fn wait(&self) -> io::Result<ExitStatus> { Err(unimpl()) }
    pub fn try_wait(&self) -> Option<ExitStatus> { None }
}
