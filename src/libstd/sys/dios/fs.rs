use prelude::v1::*;

use io::{FileAccess, FileMode};
use io::{IoResult, FileStat, SeekStyle};
use sys_common;

pub type fd_t = i32;

pub struct FileDesc;

impl FileDesc {
    pub fn new(fd: fd_t, close_on_drop: bool) -> FileDesc {
        // STUB:
        let _ = fd;
        let _ = close_on_drop;
        FileDesc
    }

    pub fn read(&self, buf: &mut [u8]) -> IoResult<uint> {
        // STUB
        let _ = buf;
        Err(sys_common::unimpl())
    }
    pub fn write(&self, buf: &[u8]) -> IoResult<()> {
        // STUB
        let _ = buf;
        Err(sys_common::unimpl())
    }

    pub fn seek(&self, pos: i64, whence: SeekStyle) -> IoResult<u64> {
        // STUB
        let _ = pos;
        let _ = whence;
        Err(sys_common::unimpl())
    }

    pub fn tell(&self) -> IoResult<u64> {
        // STUB
        Err(sys_common::unimpl())
    }

    pub fn fsync(&self) -> IoResult<()> {
        // STUB
        Err(sys_common::unimpl())
    }

    pub fn datasync(&self) -> IoResult<()> {
        // STUB
        Err(sys_common::unimpl())
    }

    pub fn truncate(&self, offset: i64) -> IoResult<()> {
        // STUB
        let _ = offset;
        Err(sys_common::unimpl())
    }

    pub fn fstat(&self) -> IoResult<FileStat> {
        // STUB
        Err(sys_common::unimpl())
    }

    /// Extract the actual filedescriptor without closing it.
    pub fn unwrap(self) -> fd_t {
        // STUB:
        0
    }
}

pub fn open(path: &Path, fm: FileMode, fa: FileAccess) -> IoResult<FileDesc> {
    // STUB
    let _ = path;
    let _ = fm;
    let _ = fa;
    Err(sys_common::unimpl())
}

pub fn mkdir(p: &Path, mode: uint) -> IoResult<()> {
    // STUB
    let _ = p;
    let _ = mode;
    Err(sys_common::unimpl())
}

pub fn readdir(p: &Path) -> IoResult<Vec<Path>> {
    // STUB
    let _ = p;
    Err(sys_common::unimpl())
}

pub fn unlink(p: &Path) -> IoResult<()> {
    // STUB
    let _ = p;
    Err(sys_common::unimpl())
}

pub fn rename(old: &Path, new: &Path) -> IoResult<()> {
    // STUB
    let _ = old;
    let _ = new;
    Err(sys_common::unimpl())
}

pub fn chmod(p: &Path, mode: uint) -> IoResult<()> {
    // STUB
    let _ = p;
    let _ = mode;
    Err(sys_common::unimpl())
}

pub fn rmdir(p: &Path) -> IoResult<()> {
    // STUB
    let _ = p;
    Err(sys_common::unimpl())
}

pub fn chown(p: &Path, uid: int, gid: int) -> IoResult<()> {
    // STUB
    let _ = p;
    let _ = uid;
    let _ = gid;
    Err(sys_common::unimpl())
}

pub fn readlink(p: &Path) -> IoResult<Path> {
    // STUB
    let _ = p;
    Err(sys_common::unimpl())
}

pub fn symlink(src: &Path, dst: &Path) -> IoResult<()> {
    // STUB
    let _ = src;
    let _ = dst;
    Err(sys_common::unimpl())
}

pub fn link(src: &Path, dst: &Path) -> IoResult<()> {
    // STUB
    let _ = src;
    let _ = dst;
    Err(sys_common::unimpl())
}

pub fn stat(p: &Path) -> IoResult<FileStat> {
    // STUB
    let _ = p;
    Err(sys_common::unimpl())
}

pub fn lstat(p: &Path) -> IoResult<FileStat> {
    // STUB
    let _ = p;
    Err(sys_common::unimpl())
}

pub fn utime(p: &Path, atime: u64, mtime: u64) -> IoResult<()> {
    // STUB
    let _ = p;
    let _ = atime;
    let _ = mtime;
    Err(sys_common::unimpl())
}
