#![allow(unused_variables)] // TODO: STUB

use prelude::v1::*;

use io;
use libc;
use sys::unimpl;

////////////////////////////////////////////////////////////////////////////////
// Anonymous pipes
////////////////////////////////////////////////////////////////////////////////

pub struct AnonPipe;

pub unsafe fn anon_pipe() -> io::Result<(AnonPipe, AnonPipe)> { Err(unimpl()) }

impl AnonPipe {
    pub fn from_fd(fd: libc::c_int) -> AnonPipe { AnonPipe }
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> { Err(unimpl()) }
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> { Err(unimpl()) }
    pub fn raw(&self) -> libc::c_int { 0 }
}
