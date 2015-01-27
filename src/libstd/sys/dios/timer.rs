use prelude::v1::*;

use old_io::IoResult;
use sys_common;

pub trait Callback {
    fn call(&mut self);
}

pub struct Timer;

// returns the current time (in milliseconds)
pub fn now() -> u64 {
    // STUB:
    0
}

impl Timer {
    pub fn new() -> IoResult<Timer> {
        // STUB:
        Err(sys_common::unimpl())
    }

    pub fn sleep(&mut self, ms: u64) {
        // STUB:
        let _ = ms;
    }

    pub fn oneshot(&mut self, msecs: u64, cb: Box<Callback + Send>) {
        // STUB:
        let _ = msecs;
        let _ = cb;
    }

    pub fn period(&mut self, msecs: u64, cb: Box<Callback + Send>) {
        // STUB:
        let _ = msecs;
        let _ = cb;
    }
}
