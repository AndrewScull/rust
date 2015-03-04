use prelude::v1::*;

use io;
use libc::{self, c_int, size_t};
use net::SocketAddr;
use sys_common::{self, AsInner};

pub type wrlen_t = size_t;

pub struct Socket;

pub fn init() {}

pub fn cvt_gai(err: c_int) -> io::Result<()> { Err(unimpl()) }

impl Socket {
    pub fn new(addr: &SocketAddr, ty: c_int) -> io::Result<Socket> { Err(unimpl()) }
    pub fn accept(&self, storage: *mut libc::sockaddr,
                  len: *mut libc::socklen_t) -> io::Result<Socket> { Err(unimpl()) }
    pub fn duplicate(&self) -> io::Result<Socket> { Err(unimpl()) }
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> { Err(unimpl()) }
}

impl AsInner<c_int> for Socket {
    fn as_inner(&self) -> &c_int { &0 }
}
