#![allow(unused_variables)] // TODO: STUB

use prelude::v1::*;

use error::Error as StdError;
use ffi::{AsOsStr, OsStr, OsString};
use fmt;
use io;
use libc::c_char;
use old_io::IoResult;
use path::{self, PathBuf};
use ptr;
use sys::fs::FileDesc;
use sys::unimpl;
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

pub fn getcwd() -> io::Result<PathBuf> {
    // STUB:
    Err(unimpl())
}

pub fn chdir(p: &path::Path) -> io::Result<()> {
    // STUB:
    let _ = p;
    Err(unimpl())
}

static ST_DUD: u32 = 0;
pub struct SplitPaths<'a> {
    dud: &'a u32
}

pub fn split_paths<'a>(unparsed: &'a OsStr) -> SplitPaths<'a> {
    SplitPaths { dud: &ST_DUD }
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> { None }
}

#[derive(Debug)]
pub struct JoinPathsError;

pub fn join_paths<I, T>(paths: I) -> Result<OsString, JoinPathsError>
    where I: Iterator<Item=T>, T: AsOsStr
{
    Err(JoinPathsError)
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        "path segment contains separator `:`".fmt(f)
    }
}

impl StdError for JoinPathsError {
    fn description(&self) -> &str { "failed to join paths" }
}

pub fn current_exe() -> io::Result<PathBuf> {
    Err(unimpl())
}

pub struct Args;

impl Iterator for Args {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> { None }
}

pub fn args() -> Args {
    Args
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize { 0 }
}

pub struct Env;

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> { None }
}

pub unsafe fn environ() -> *mut *const *const c_char {
    ptr::null_mut()
}

pub fn env() -> Env {
    Env
}

pub fn getenv(k: &OsStr) -> Option<OsString> {
    None
}

pub fn setenv(k: &OsStr, v: &OsStr) {
}

pub fn unsetenv(n: &OsStr) {
}

pub unsafe fn pipe() -> IoResult<(FileDesc, FileDesc)> {
    // STUB:
    Err(sys_common::unimpl())
}

pub fn page_size() -> uint {
    // STUB:
    4096
}

pub fn temp_dir() -> PathBuf {
    PathBuf::new("")
}

pub fn home_dir() -> Option<PathBuf> {
    None
}
