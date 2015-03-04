#![allow(dead_code)]

use prelude::v1::*;

use libc;
use mem;
use ptr;
use old_io::{IoError, IoResult, IoUnavailable};
use sys_common;

pub struct TTY {
    console_ref: *mut libc::dios_ref_t
}

impl TTY {
    pub fn new(fd: libc::c_int) -> IoResult<TTY> {
        // All go to the same console
        let _ = fd;

        unsafe {
            let mut console_ref: *mut libc::dios_ref_t = mem::uninitialized();
            let mut ref_count: libc::uint64_t = 1;

            // Get the stdout reference
            // TODO: 0 here is local lookup, put that in liblibc
            if libc::dios_lookup(0,
                                 &libc::STDOUT_NAME,
                                 &mut console_ref,
                                 &mut ref_count) == 0 && ref_count >= 1 {
                Ok(TTY { console_ref: console_ref })
            } else {
                Err(IoError {
                    kind: IoUnavailable,
                    desc: "Console not found in lookup",
                    detail: None,
                })
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        // STUB:
        let _ = buf;
        Err(sys_common::unimpl())
    }

    pub fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        unsafe {
            // Open for writing
            let mut iov: *mut libc::dios_iovec_t = mem::uninitialized();
            if libc::dios_begin_write(0, self.console_ref, buf.len() as u64, &mut iov) != 0 {
                return Err(IoError {
                    kind: IoUnavailable,
                    desc: "Failed to begin write",
                    detail: None,
                });
            }

            // Copy in data
            ptr::copy_nonoverlapping((*iov).buf, buf.as_ptr() as *const libc::c_void, buf.len());

            // Commit the write
            // TODO: now I have a buffer which hasn't been returned?
            if libc::dios_end_write(0, self.console_ref, buf.len() as u64, iov) != 0 {
                return Err(IoError {
                    kind: IoUnavailable,
                    desc: "Failed to end write",
                    detail: None,
                });
            }
        }

        return Ok(());
    }

    pub fn set_raw(&mut self, _raw: bool) -> IoResult<()> {
        // STUB:
        let _ = _raw;
        Err(sys_common::unimpl())
    }

    pub fn get_winsize(&mut self) -> IoResult<(int, int)> {
        // STUB:
        Err(sys_common::unimpl())
    }

    pub fn isatty(&self) -> bool { true }
}
