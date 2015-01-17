#![allow(dead_code)]

use libc;

pub type signal = i32;

pub fn new() -> (signal, signal) {
    // STUB:
    (0, 0)
}

pub fn signal(fd: libc::c_int) {
    // STUB:
    let _ = fd;
}

pub fn close(fd: libc::c_int) {
    // STUB:
    let _ = fd;
}
