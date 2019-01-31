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
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/usr/include/assert.h"]
pub mod assert_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
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
    use super::stdint_h::{intptr_t};
    use super::q_shared_h::{qboolean};
    extern "C" {
        #[no_mangle]
        pub fn Sys_DllExtension(name: *const libc::c_char) -> qboolean;
    }
}
#[header_src = "/usr/include/SDL2/SDL_loadso.h"]
pub mod SDL_loadso_h {
    use super::{libc};
    extern "C" {
        /* *
 *  Unload a shared object from memory.
 */
        #[no_mangle]
        pub fn SDL_UnloadObject(handle: *mut libc::c_void);
        /* *
 *  Given an object handle, this function looks up the address of the
 *  named function in the shared object and returns it.  This address
 *  is no longer valid after calling SDL_UnloadObject().
 */
        #[no_mangle]
        pub fn SDL_LoadFunction(handle: *mut libc::c_void,
                                name: *const libc::c_char)
         -> *mut libc::c_void;
        /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
        /* *
 *  \file SDL_loadso.h
 *
 *  System dependent library loading routines
 *
 *  Some things to keep in mind:
 *  \li These functions only work on C function names.  Other languages may
 *      have name mangling and intrinsic language support that varies from
 *      compiler to compiler.
 *  \li Make sure you declare your function pointers with the same calling
 *      convention as the actual library function.  Your code will crash
 *      mysteriously if you do not do this.
 *  \li Avoid namespace collisions.  If you load a symbol from the library,
 *      it is not defined whether or not it goes into the global symbol
 *      namespace for the application.  If it does and it conflicts with
 *      symbols in your code or other shared libraries, you will not get
 *      the results you expect. :)
 */
        /* Set up for C function definitions, even when using C++ */
        /* *
 *  This function dynamically loads a shared object and returns a pointer
 *  to the object handle (or NULL if there was an error).
 *  The 'sofile' parameter is a system dependent name of the object file.
 */
        #[no_mangle]
        pub fn SDL_LoadObject(sofile: *const libc::c_char)
         -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/SDL2/SDL_error.h"]
pub mod SDL_error_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn SDL_GetError() -> *const libc::c_char;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/sys/sys_local.h"]
pub mod sys_local_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Sys_Exit(exitCode: libc::c_int) -> !;
        #[no_mangle]
        pub fn Sys_ErrorDialog(error: *const libc::c_char);
    }
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::stdarg_h::{va_list};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{qboolean, qtrue, qfalse, Com_Printf};
use self::assert_h::{__assert_fail};
use self::stdio_h::{vsnprintf};
use self::qcommon_h::{Sys_DllExtension};
use self::SDL_loadso_h::{SDL_UnloadObject, SDL_LoadFunction, SDL_LoadObject};
use self::SDL_error_h::{SDL_GetError};
use self::sys_local_h::{Sys_Exit, Sys_ErrorDialog};
// general development dll loading for virtual machine testing
#[no_mangle]
pub unsafe extern "C" fn Sys_LoadGameDll(mut name: *const libc::c_char,
                                         mut entryPoint:
                                             *mut Option<unsafe extern "C" fn(_:
                                                                                  libc::c_int, ...)
                                                             -> intptr_t>,
                                         mut systemcalls:
                                             Option<unsafe extern "C" fn(_:
                                                                             intptr_t, ...)
                                                        -> intptr_t>)
 -> *mut libc::c_void {
    let mut libHandle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dllEntry:
            Option<unsafe extern "C" fn(_:
                                            Option<unsafe extern "C" fn(_:
                                                                            intptr_t, ...)
                                                       -> intptr_t>) -> ()> =
        None;
    if !name.is_null() {
    } else {
        __assert_fail(b"name\x00" as *const u8 as *const libc::c_char,
                      b"code/sys/sys_variadic.c\x00" as *const u8 as
                          *const libc::c_char, 58i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 90],
                                                &[libc::c_char; 90]>(b"void *Sys_LoadGameDll(const char *, intptr_t (**)(int, ...), intptr_t (*)(intptr_t, ...))\x00")).as_ptr());
    }
    if 0 == Sys_DllExtension(name) as u64 {
        Com_Printf(b"Refusing to attempt to load library \"%s\": Extension not allowed.\n\x00"
                       as *const u8 as *const libc::c_char, name);
        return 0 as *mut libc::c_void
    }
    Com_Printf(b"Loading DLL file: %s\n\x00" as *const u8 as
                   *const libc::c_char, name);
    libHandle = SDL_LoadObject(name);
    if libHandle.is_null() {
        Com_Printf(b"Sys_LoadGameDll(%s) failed:\n\"%s\"\n\x00" as *const u8
                       as *const libc::c_char, name, SDL_GetError());
        return 0 as *mut libc::c_void
    }
    dllEntry =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                Option<unsafe extern "C" fn(_:
                                                                                                intptr_t, ...)
                                                                           ->
                                                                               intptr_t>)
                                           ->
                                               ()>>(SDL_LoadFunction(libHandle,
                                                                     b"dllEntry\x00"
                                                                         as
                                                                         *const u8
                                                                         as
                                                                         *const libc::c_char));
    *entryPoint =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                libc::c_int, ...)
                                           ->
                                               intptr_t>>(SDL_LoadFunction(libHandle,
                                                                           b"vmMain\x00"
                                                                               as
                                                                               *const u8
                                                                               as
                                                                               *const libc::c_char));
    if (*entryPoint).is_none() || dllEntry.is_none() {
        Com_Printf(b"Sys_LoadGameDll(%s) failed to find vmMain function:\n\"%s\" !\n\x00"
                       as *const u8 as *const libc::c_char, name,
                   SDL_GetError());
        SDL_UnloadObject(libHandle);
        return 0 as *mut libc::c_void
    }
    Com_Printf(b"Sys_LoadGameDll(%s) found vmMain function at %p\n\x00" as
                   *const u8 as *const libc::c_char, name, *entryPoint);
    dllEntry.expect("non-null function pointer")(systemcalls);
    return libHandle;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_Error(mut error: *const libc::c_char, ...) -> ! {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(string.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              error, argptr);
    Sys_ErrorDialog(string.as_mut_ptr());
    Sys_Exit(3i32);
}