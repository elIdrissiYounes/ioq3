use libc;
#[header_src = "vararg"]
pub mod vararg {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stdarg.h"]
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
    use super::vararg::{__builtin_va_list};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    //
// these aren't needed by any of the VMs.  put in another header?
//
    // bit vector of area visibility
    // print levels from renderer (FIXME: set up for game / cgame?)
    pub type unnamed = libc::c_uint;
    pub const PRINT_ERROR: unnamed = 3;
    pub const PRINT_WARNING: unnamed = 2;
    // only print when "developer 1"
    pub const PRINT_DEVELOPER: unnamed = 1;
    pub const PRINT_ALL: unnamed = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    use super::vararg::{__va_list_tag};
    extern "C" {
        #[no_mangle]
        pub fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                         _: *const libc::c_char, _: *mut __va_list_tag)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/client/cl_variadic.h"]
pub mod cl_variadic_h {
    use super::{libc};
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::stdarg_h::{va_list};
use self::q_shared_h::{unnamed, PRINT_ERROR, PRINT_WARNING, PRINT_DEVELOPER,
                       PRINT_ALL, Com_Printf};
use self::stdio_h::{vsnprintf};
use self::qcommon_h::{Com_DPrintf};
#[no_mangle]
pub unsafe extern "C" fn CL_RefPrintf(mut print_level: libc::c_int,
                                      mut fmt: *const libc::c_char, ...) {
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    vsnprintf(msg.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
              fmt, argptr);
    if print_level == PRINT_ALL as libc::c_int {
        Com_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                   msg.as_mut_ptr());
    } else if print_level == PRINT_WARNING as libc::c_int {
        Com_Printf(b"^3%s\x00" as *const u8 as *const libc::c_char,
                   msg.as_mut_ptr());
    } else if print_level == PRINT_DEVELOPER as libc::c_int {
        Com_DPrintf(b"^1%s\x00" as *const u8 as *const libc::c_char,
                    msg.as_mut_ptr());
    };
}