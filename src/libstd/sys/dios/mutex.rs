#![allow(dead_code)]

use prelude::v1::*;

pub struct Mutex;

pub const MUTEX_INIT: Mutex = Mutex;

unsafe impl Sync for Mutex {}

impl Mutex {
    #[inline]
    pub fn new() -> Mutex {
        // STUB:
        MUTEX_INIT
    }
    #[inline]
    pub fn lock(&self) {
        // STUB:
    }
    #[inline]
    pub fn unlock(&self) {
        // STUB:
    }
    #[inline]
    pub fn try_lock(&self) -> bool {
        // STUB:
        true
    }
    #[inline]
    pub fn destroy(&self) {
        // STUB:
    }
}
