#![cfg(not(dios))]

use prelude::v1::*;

use old_io::net::ip;
use old_io::IoResult;

use sys_common;
pub use sys_common::net::TcpStream;

////////////////////////////////////////////////////////////////////////////////
// TCP listeners
////////////////////////////////////////////////////////////////////////////////

pub struct TcpListener;

unsafe impl Sync for TcpListener {}

impl TcpListener {
    pub fn bind(addr: ip::SocketAddr) -> IoResult<TcpListener> {
        // STUB:
        Err(unimpl())
    }

    pub fn listen(self, backlog: int) -> IoResult<TcpAcceptor> {
        // STUB:
        Err(unimpl())
    }

    pub fn socket_name(&mut self) -> IoResult<ip::SocketAddr> {
        // STUB:
        Err(unimpl())
    }
}

pub struct TcpAcceptor;

impl TcpAcceptor {
    pub fn accept(&mut self) -> IoResult<TcpStream> {
        // STUB:
        Err(unimpl())
    }

    pub fn socket_name(&mut self) -> IoResult<ip::SocketAddr> {
        // STUB:
        Err(unimpl())
    }

    pub fn set_timeout(&mut self, timeout: Option<u64>) {
        // STUB:
    }

    pub fn close_accept(&mut self) -> IoResult<()> {
        // STUB:
        Err(unimpl())
    }
}
