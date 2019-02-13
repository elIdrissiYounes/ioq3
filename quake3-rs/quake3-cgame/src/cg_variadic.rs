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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/cgame/cg_local.h"]
pub mod cg_local_h {
    use super::{libc};
    extern "C" {
        // abort the game
        #[no_mangle]
        pub fn trap_Error(fmt: *const libc::c_char) -> !;
        //===============================================
        //
// system traps
// These functions are how the cgame communicates with the main game system
//
        // print message on the local console
        #[no_mangle]
        pub fn trap_Print(fmt: *const libc::c_char);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/cgame/cg_variadic.h"]
pub mod cg_variadic_h {
    use super::{libc};
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::stdarg_h::{va_list};
use self::stdio_h::{vsnprintf};
use self::cg_local_h::{trap_Error, trap_Print};
// this is only here so the functions in q_shared.c and bg_*.c can link
#[no_mangle]
pub unsafe extern "C" fn Com_Error(mut level: libc::c_int,
                                   mut error: *const libc::c_char, ...) -> ! {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              error, argptr);
    trap_Error(text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Com_Printf(mut msg: *const libc::c_char, ...) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              msg, argptr_0);
    trap_Print(text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CG_Printf(mut msg: *const libc::c_char, ...) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              msg, argptr_1);
    trap_Print(text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CG_Error(mut msg: *const libc::c_char, ...) -> ! {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              msg, argptr_2);
    trap_Error(text.as_mut_ptr());
}