#![allow(dead_code)]

use prelude::v1::*;

use libc;
use io::IoResult;
use sys_common;

pub struct TTY;

impl TTY {
    pub fn new(fd: libc::c_int) -> IoResult<TTY> {
        // STUB:
        let _ = fd;
        Err(sys_common::unimpl())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        // STUB:
        let _ = buf;
        Err(sys_common::unimpl())
    }
    pub fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        // STUB:
        let _ = buf;
        Err(sys_common::unimpl())
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
    pub fn isatty(&self) -> bool { false }
}
