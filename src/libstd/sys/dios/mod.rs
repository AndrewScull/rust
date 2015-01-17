#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]

use prelude::v1::*;

use io::{IoResult, IoError};
use libc;
use num::{Int, SignedInt};
use sys_common;

pub mod backtrace;
pub mod c;
pub mod condvar;
pub mod fs;
pub mod helper_signal;
pub mod mutex;
pub mod os;
pub mod pipe;
pub mod process;
pub mod rwlock;
pub mod stack_overflow;
//pub mod tcp;
pub mod thread;
pub mod thread_local;
pub mod time;
pub mod timer;
pub mod tty;
//pub mod udp;

//pub mod addrinfo {
//    pub use sys_common::net::get_host_addresses;
//    pub use sys_common::net::get_address_name;
//}

// FIXME: move these to c module
pub type sock_t = u32;
pub type wrlen = libc::size_t;
pub type msglen_t = libc::size_t;
pub unsafe fn close_sock(sock: sock_t) {
    // STUB:
    let _ = sock;
}

pub fn last_error() -> IoError {
    // STUB:
    sys_common::unimpl()
}

pub fn last_net_error() -> IoError {
    // STUB:
    sys_common::unimpl()
}

pub fn last_gai_error(s: libc::c_int) -> IoError {
    // STUB:
    let _ = s;
    sys_common::unimpl()
}

pub fn decode_error(errno: i32) -> IoError {
    // STUB:
    let _ = errno;
    sys_common::unimpl()
}

pub fn decode_error_detailed(errno: i32) -> IoError {
    // STUB:
    let _ = errno;
    sys_common::unimpl()
}

#[allow(unused_mut)]
pub fn retry<T, F> (mut f: F) -> T where
    T: SignedInt,
    F: FnMut() -> T,
{
    // STUB:
    let _ = f;
    let one: T = Int::one();
    -one
}

pub fn ms_to_timeval(ms: u64) -> libc::timeval {
    libc::timeval {
        tv_sec: (ms / 1000) as libc::time_t,
        tv_usec: ((ms % 1000) * 1000) as libc::suseconds_t,
    }
}

pub fn wouldblock() -> bool {
    // STUB:
    false
}

pub fn set_nonblocking() -> IoResult<()> {
    // STUB:
    Err(sys_common::unimpl())
}

pub fn init_net() {}
