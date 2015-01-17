#![allow(dead_code)]

use libc;
use thunk::Thunk;
use sys_common;

pub type rust_thread = u32;
pub type rust_thread_return = *mut u8;
pub type StartFn = extern "C" fn(*mut libc::c_void) -> rust_thread_return;

#[no_stack_check]
pub extern fn thread_start(main: *mut libc::c_void) -> rust_thread_return {
    return sys_common::thread::start_thread(main);
}

// This is the guard page above the stack, I think.
pub mod guard {
    pub unsafe fn current() -> uint {
        // STUB:
        0
    }

    pub unsafe fn main() -> uint {
        // STUB:
        0
    }

    pub unsafe fn init() {
        // STUB:
    }
}

pub unsafe fn create(stack: uint, p: Thunk) -> rust_thread {
    let _ = stack;
    let _ = p;
    panic!("STUB");
}

pub unsafe fn join(native: rust_thread) {
    // STUB:
    let _ = native;
}

pub unsafe fn detach(native: rust_thread) {
    // STUB:
    let _ = native;
}

pub unsafe fn yield_now() {
    // STUB:
}
