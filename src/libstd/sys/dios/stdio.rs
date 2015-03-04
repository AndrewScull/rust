use prelude::v1::*;

use io::{self, ErrorKind};
use libc;
use mem;
use ptr;
use sys::unimpl;

pub struct Stdin;
pub struct Stdout {
    console_ref: Option<*mut libc::dios_ref_t>
}
pub struct Stderr {
    console_ref: Option<*mut libc::dios_ref_t>
}

// TODO: it's not really Send...
unsafe impl Send for Stdout {}
unsafe impl Send for Stderr {}

fn get_console() -> Option<*mut libc::dios_ref_t> {
        unsafe {
            let mut console_ref: *mut libc::dios_ref_t = mem::uninitialized();
            let mut ref_count: libc::uint64_t = 1;

            // Get the stdout reference
            // TODO: 0 here is local lookup, put that in liblibc
            if libc::dios_lookup(0,
                                 &libc::STDOUT_NAME,
                                 &mut console_ref,
                                 &mut ref_count) == 0 && ref_count >= 1 {
                Some(console_ref)
            } else {
                None
            }
        }
}

fn console_write(console_ref: *mut libc::dios_ref_t, buf: &[u8]) -> io::Result<usize> {
    unsafe {
        // Open for writing
        let mut iov: *mut libc::dios_iovec_t = mem::uninitialized();
        if libc::dios_begin_write(0, console_ref, buf.len() as u64, &mut iov) != 0 {
            return Err(io::Error::new(
                ErrorKind::ResourceUnavailable,
                "Failed to begin write",
                None,
            ));
        }

        // Copy in data
        ptr::copy_nonoverlapping((*iov).buf, buf.as_ptr() as *const libc::c_void, buf.len());

        // Commit the write
        // TODO: now I have a buffer which hasn't been returned?
        if libc::dios_end_write(0, console_ref, buf.len() as u64, iov) != 0 {
            return Err(io::Error::new(
                ErrorKind::ResourceUnavailable,
                "Failed to end write",
                None,
            ));
        }
    }

    return Ok(buf.len());
}

impl Stdin {
    pub fn new() -> Stdin { Stdin }
    pub fn read(&self, data: &mut [u8]) -> io::Result<usize> {
        // STUB: there is not stdin for DIOS yet
        let _ = data;
        Err(unimpl())
    }
}

impl Stdout {
    pub fn new() -> Stdout {
        Stdout { console_ref: get_console() }
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        match self.console_ref {
            Some(cref) => console_write(cref, data),
            None => Err(io::Error::new(
                ErrorKind::ResourceUnavailable,
                "Console no open for writing",
                None,
            ))
        }
    }
}

impl Stderr {
    pub fn new() -> Stderr {
        Stderr { console_ref: get_console() }
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        match self.console_ref {
            Some(cref) => console_write(cref, data),
            None => Err(io::Error::new(
                ErrorKind::ResourceUnavailable,
                "Console no open for writing",
                None,
            ))
        }
    }
}
