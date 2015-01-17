use prelude::v1::*;

use ptr;

// This isn't thread safy, but we don't have threads yet
static mut STORE: [*mut u8; 64] = [0 as *mut u8; 64];
static mut KEYCNT: Key = 0;

pub type Key = usize;

#[inline]
pub unsafe fn create(dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
    // STUB:
    let _ = dtor;
    KEYCNT += 1;
    assert!(KEYCNT < 64);
    KEYCNT
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    // STUB:
    STORE[key] = value;
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    // STUB:
    STORE[key]
}

#[inline]
pub unsafe fn destroy(key: Key) {
    // STUB:
    STORE[key] = ptr::null_mut();
}
