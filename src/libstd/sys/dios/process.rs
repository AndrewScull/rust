#![allow(dead_code)]

use prelude::v1::*;

use collections::hash_map::Hasher;
use hash::Hash;
use io::process::ProcessExit;
use io::IoResult;
use libc;
use path::BytesContainer;
use sys::fs::FileDesc;
use sys_common::AsInner;

use sys_common;
pub use sys_common::ProcessConfig;

pub struct Process;

const CLOEXEC_MSG_FOOTER: &'static [u8] = b"NOEX";

impl Process {
    pub fn id(&self) -> libc::pid_t {
        // STUB:
        libc::dios_name_t {
            raw: [0; 32]
        }
    }

    pub unsafe fn kill(&self, signal: int) -> IoResult<()> {
        // STUB:
        let _ = signal;
        Err(sys_common::unimpl())
    }

    pub unsafe fn killpid(pid: libc::pid_t, signal: int) -> IoResult<()> {
        // STUB:
        let _ = pid;
        let _ = signal;
        Err(sys_common::unimpl())
    }

    pub fn spawn<K, V, C, P>(cfg: &C, in_fd: Option<P>,
                              out_fd: Option<P>, err_fd: Option<P>)
                              -> IoResult<Process>
        where C: ProcessConfig<K, V>, P: AsInner<FileDesc>,
              K: BytesContainer + Eq + Hash<Hasher>, V: BytesContainer
    {
        // STUB:
        let _ = cfg;
        let _ = in_fd;
        let _ = out_fd;
        let _ = err_fd;
        Err(sys_common::unimpl())
    }

    pub fn wait(&self, deadline: u64) -> IoResult<ProcessExit> {
        // STUB:
        let _ = deadline;
        Err(sys_common::unimpl())
    }

    pub fn try_wait(&self) -> Option<ProcessExit> {
        // STUB:
        None
    }
}
