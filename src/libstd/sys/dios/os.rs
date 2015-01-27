use prelude::v1::*;

use old_io::IoResult;
use path::{BytesContainer};
use sys::fs::FileDesc;
use sys_common;

/// Returns the platform-specific value of errno
pub fn errno() -> int {
    // STUB:
    0
}

/// Get a detailed string description for the given error number
pub fn error_string(errno: i32) -> String {
    // STUB:
    let _ = errno;
    "STUB".to_string()
}

pub unsafe fn pipe() -> IoResult<(FileDesc, FileDesc)> {
    // STUB:
    Err(sys_common::unimpl())
}

pub fn getcwd() -> IoResult<Path> {
    // STUB:
    Err(sys_common::unimpl())
}

pub unsafe fn get_env_pairs() -> Vec<Vec<u8>> {
    // STUB:
    Vec::<Vec<u8>>::new()
}

pub fn split_paths(unparsed: &[u8]) -> Vec<Path> {
    // STUB:
    let _ = unparsed;
    Vec::<Path>::new() 
}

pub fn join_paths<T: BytesContainer>(paths: &[T]) -> Result<Vec<u8>, &'static str> {
    // STUB:
    let _ = paths;
    Err("STUB")
}

pub fn load_self() -> Option<Vec<u8>> {
    // STUB:
    None
}

pub fn chdir(p: &Path) -> IoResult<()> {
    // STUB:
    let _ = p;
    Err(sys_common::unimpl())
}

pub fn page_size() -> uint {
    // STUB:
    4096
}
