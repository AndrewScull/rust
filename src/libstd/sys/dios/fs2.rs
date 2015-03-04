#![allow(unused_variables)] // TODO: STUB

use core::prelude::*;
use io::prelude::*;

use io::{self, SeekFrom};
use path::{Path, PathBuf};
use sys::unimpl;
use sys_common::FromInner;

pub struct File;

pub struct FileAttr;

pub struct ReadDir;

pub struct DirEntry;

#[derive(Clone)]
pub struct OpenOptions;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FilePermissions;

impl FileAttr {
    // STUB
    pub fn is_dir(&self) -> bool { false }
    pub fn is_file(&self) -> bool { false }
    pub fn size(&self) -> u64 { 0 }
    pub fn perm(&self) -> FilePermissions { FilePermissions }

    pub fn accessed(&self) -> u64 { 0 }
    pub fn modified(&self) -> u64 { 0 }

    // times are in milliseconds (currently)
    fn mktime(&self, secs: u64, nsecs: u64) -> u64 { 0 }
}

impl FilePermissions {
    // STUB
    pub fn readonly(&self) -> bool { true }
    pub fn set_readonly(&mut self, readonly: bool) { }
    pub fn mode(&self) -> i32 { 0 }
}

impl FromInner<i32> for FilePermissions {
    // STUB
    fn from_inner(mode: i32) -> FilePermissions { FilePermissions }
}

impl Iterator for ReadDir {
    // STUB
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> { None }
}

impl DirEntry {
    // STUB
    pub fn path(&self) -> PathBuf { PathBuf::new("") }
}

impl OpenOptions {
    // STUB
    pub fn new() -> OpenOptions { OpenOptions }
    pub fn read(&mut self, read: bool) { }
    pub fn write(&mut self, write: bool) { }
    pub fn append(&mut self, append: bool) { }
    pub fn truncate(&mut self, truncate: bool) { }
    pub fn create(&mut self, create: bool) { }
    pub fn mode(&mut self, mode: i32) { }
}

impl File {
    // STUB
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> { Err(unimpl()) }
    pub fn file_attr(&self) -> io::Result<FileAttr> { Err(unimpl()) }
    pub fn fsync(&self) -> io::Result<()> { Err(unimpl()) }
    pub fn datasync(&self) -> io::Result<()> { Err(unimpl()) }
    pub fn truncate(&self, size: u64) -> io::Result<()> { Err(unimpl()) }
    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> { Err(unimpl()) }
    pub fn write(&self, buf: &[u8]) -> io::Result<usize> { Err(unimpl()) }
    pub fn flush(&self) -> io::Result<()> { Err(unimpl()) }
    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> { Err(unimpl()) }
}

// STUB
pub fn mkdir(p: &Path) -> io::Result<()> { Err(unimpl()) }
pub fn readdir(p: &Path) -> io::Result<ReadDir> { Err(unimpl()) }
pub fn unlink(p: &Path) -> io::Result<()> { Err(unimpl()) }
pub fn rename(old: &Path, new: &Path) -> io::Result<()> { Err(unimpl()) }
pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> { Err(unimpl()) }
pub fn rmdir(p: &Path) -> io::Result<()> { Err(unimpl()) }
pub fn chown(p: &Path, uid: isize, gid: isize) -> io::Result<()> { Err(unimpl()) }
pub fn readlink(p: &Path) -> io::Result<PathBuf> { Err(unimpl()) }
pub fn symlink(src: &Path, dst: &Path) -> io::Result<()> { Err(unimpl()) }
pub fn link(src: &Path, dst: &Path) -> io::Result<()> { Err(unimpl()) }
pub fn stat(p: &Path) -> io::Result<FileAttr> { Err(unimpl()) }
pub fn lstat(p: &Path) -> io::Result<FileAttr> { Err(unimpl()) }
pub fn utimes(p: &Path, atime: u64, mtime: u64) -> io::Result<()> { Err(unimpl()) }
