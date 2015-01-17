use target::{Target, TargetOptions};
use std::default::Default;

pub fn target() -> Target {
    Target {
        data_layout: "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-\
                      f32:32:32-f64:64:64-v64:64:64-v128:128:128-a:0:64-\
                      s0:64:64-f80:128:128-n8:16:32:64-S128".to_string(),
        llvm_target: "x86_64-linux-dios".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        arch: "x86_64".to_string(),
        target_os: "dios".to_string(),
        options: TargetOptions {
            pre_link_args: vec!(
                "-Wl,--as-needed".to_string(),
                "-m64".to_string()
            ),
            executables: true,
            is_like_dios: true,
            linker_is_gnu: true,
            position_independent_executables: true,
            .. Default::default()
        }
    }
}
