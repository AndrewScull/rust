use prelude::v1::*;

use old_io::IoResult;
use sys_common;

#[inline(never)] // if we know this is a function call, we can skip it when
                 // tracing
pub fn write(w: &mut Writer) -> IoResult<()> {
    // STUB:
    let _ = w;
    Err(sys_common::unimpl())
}

/// Unwind library interface used for backtraces
///
/// Note that dead code is allowed as here are just bindings
/// iOS doesn't use all of them it but adding more
/// platform-specific configs pollutes the code too much
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod uw {
    pub use self::_Unwind_Reason_Code::*;

    use libc;

    #[repr(C)]
    pub enum _Unwind_Reason_Code {
        _URC_NO_REASON = 0,
        _URC_FOREIGN_EXCEPTION_CAUGHT = 1,
        _URC_FATAL_PHASE2_ERROR = 2,
        _URC_FATAL_PHASE1_ERROR = 3,
        _URC_NORMAL_STOP = 4,
        _URC_END_OF_STACK = 5,
        _URC_HANDLER_FOUND = 6,
        _URC_INSTALL_CONTEXT = 7,
        _URC_CONTINUE_UNWIND = 8,
        _URC_FAILURE = 9, // used only by ARM EABI
    }

    pub enum _Unwind_Context {}

    pub type _Unwind_Trace_Fn =
            extern fn(ctx: *mut _Unwind_Context,
                      arg: *mut libc::c_void) -> _Unwind_Reason_Code;

    extern {
        pub fn _Unwind_Backtrace(trace: _Unwind_Trace_Fn,
                                 trace_argument: *mut libc::c_void)
                    -> _Unwind_Reason_Code;

        pub fn _Unwind_GetIP(ctx: *mut _Unwind_Context) -> libc::uintptr_t;

        pub fn _Unwind_FindEnclosingFunction(pc: *mut libc::c_void)
            -> *mut libc::c_void;
    }
}
