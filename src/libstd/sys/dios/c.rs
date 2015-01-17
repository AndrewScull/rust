#![allow(dead_code)]
#![allow(non_camel_case_types)]

use libc;

pub const MSG_DONTWAIT: libc::c_int = 0x40;

pub struct fd_set;

pub fn select(nfds: libc::c_int,
              readfds: *mut fd_set,
              writefds: *mut fd_set,
              errorfds: *mut fd_set,
              timeout: *mut libc::timeval) -> libc::c_int {
    // STUB:
    let _ = nfds;
    let _ = readfds;
    let _ = writefds;
    let _ = errorfds;
    let _ = timeout;
    -1
}
pub fn getsockopt(sockfd: libc::c_int,
                  level: libc::c_int,
                  optname: libc::c_int,
                  optval: *mut libc::c_void,
                  optlen: *mut libc::socklen_t) -> libc::c_int {
    // STUB:
    let _ = sockfd;
    let _ = level;
    let _ = optname;
    let _ = optval;
    let _ = optlen;
    -1
}
