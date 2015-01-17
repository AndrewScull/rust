#![allow(dead_code)]

pub struct RWLock;

pub const RWLOCK_INIT: RWLock = RWLock;

impl RWLock {
    #[inline]
    pub fn new() -> RWLock {
        // Might be moved and address is changing it is better to avoid
        // initialization of potentially opaque OS data before it landed
        RWLOCK_INIT
    }
    #[inline]
    pub fn read(&self) {
        // STUB:
    }
    #[inline]
    pub fn try_read(&self) -> bool {
        // STUB:
        true
    }
    #[inline]
    pub fn write(&self) {
        // STUB:
    }
    #[inline]
    pub fn try_write(&self) -> bool {
        // STUB:
        true
    }
    #[inline]
    pub fn read_unlock(&self) {
        // STUB:
    }
    #[inline]
    pub fn write_unlock(&self) {
        // STUB:
    }
    #[inline]
    pub fn destroy(&self) {
        // STUB:
    }
}
