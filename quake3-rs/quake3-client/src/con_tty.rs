#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           extern_types,
           label_break_value,
           libc)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
    pub type __ssize_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/bits/types/struct_FILE.h"]
pub mod struct_FILE_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    pub type _IO_lock_t = ();
    use super::{libc};
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::{size_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
#[header_src = "/usr/include/bits/types/FILE.h"]
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::{_IO_FILE};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    pub type ssize_t = __ssize_t;
    use super::types_h::{__ssize_t};
    use super::FILE_h::{FILE};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut stderr: *mut FILE;
        #[no_mangle]
        pub fn fputs(__s: *const libc::c_char, __stream: *mut FILE)
         -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
    /*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
					// specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
					// without proper initialization.  modified
					// will be set, even though the value hasn't
					// changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    pub type cvar_t = cvar_s;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    /*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct field_t {
        pub cursor: libc::c_int,
        pub scroll: libc::c_int,
        pub widthInChars: libc::c_int,
        pub buffer: [libc::c_char; 256],
    }
    use super::{libc};
    use super::q_shared_h::{cvar_t};
    extern "C" {
        #[no_mangle]
        pub fn Field_Clear(edit: *mut field_t);
        #[no_mangle]
        pub static mut com_ansiColor: *mut cvar_t;
    }
}
#[header_src = "/usr/include/bits/termios.h"]
pub mod termios_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct termios {
        pub c_iflag: tcflag_t,
        pub c_oflag: tcflag_t,
        pub c_cflag: tcflag_t,
        pub c_lflag: tcflag_t,
        pub c_line: cc_t,
        pub c_cc: [cc_t; 32],
        pub c_ispeed: speed_t,
        pub c_ospeed: speed_t,
    }
    pub type speed_t = libc::c_uint;
    pub type cc_t = libc::c_uchar;
    pub type tcflag_t = libc::c_uint;
    use super::{libc};
}
#[header_src = "/usr/include/signal.h"]
pub mod signal_h {
    pub type __sighandler_t
        =
        Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn signal(__sig: libc::c_int, __handler: __sighandler_t)
         -> __sighandler_t;
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
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src =
      "ioq3/code/sys/sys_local.h"]
pub mod sys_local_h {
    use super::qcommon_h::{field_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Sys_AnsiColorPrint(msg: *const libc::c_char);
    }
}
#[header_src = "/usr/include/fcntl.h"]
pub mod fcntl_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, ...)
         -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/sys/con_tty.c"]
pub mod con_tty_c {
    use super::qcommon_h::{field_t};
    use super::q_shared_h::{qboolean};
    use super::{libc};
    extern "C" {
        /*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
        /*
=============================================================
tty console routines

NOTE: if the user is editing a line when something gets printed to the early
console then it won't look good so we provide CON_Hide and CON_Show to be
called before and after a stdout or stderr output
=============================================================
*/
        #[no_mangle]
        pub static mut stdinIsATTY: qboolean;
    }
}
#[header_src = "/usr/include/termios.h"]
pub mod include_termios_h {
    use super::{libc};
    use super::termios_h::{termios};
    extern "C" {
        #[no_mangle]
        pub fn tcsetattr(__fd: libc::c_int, __optional_actions: libc::c_int,
                         __termios_p: *const termios) -> libc::c_int;
        #[no_mangle]
        pub fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios)
         -> libc::c_int;
    }
}
#[header_src = "/usr/include/unistd.h"]
pub mod unistd_h {
    use super::stdio_h::{ssize_t};
    use super::{libc};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn write(__fd: libc::c_int, __buf: *const libc::c_void,
                     __n: size_t) -> ssize_t;
    }
}
use self::types_h::{__off_t, __off64_t, __ssize_t};
use self::stddef_h::{size_t};
use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt,
                          _IO_marker};
use self::FILE_h::{FILE};
use self::stdio_h::{ssize_t, stderr, fputs};
use self::q_shared_h::{qboolean, qtrue, qfalse, cvar_s, cvar_t, Com_Printf};
use self::qcommon_h::{field_t, Field_Clear, com_ansiColor};
use self::termios_h::{termios, speed_t, cc_t, tcflag_t};
use self::signal_h::{__sighandler_t, signal};
use self::assert_h::{__assert_fail};
use self::string_h::{strlen};
use self::sys_local_h::{Sys_AnsiColorPrint};
use self::fcntl_h::{fcntl};
use self::con_tty_c::{stdinIsATTY};
use self::include_termios_h::{tcsetattr, tcgetattr};
use self::unistd_h::{write};
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
// Require a minimum version of SDL
// Console
#[no_mangle]
pub unsafe extern "C" fn CON_Shutdown() {
    if 0 != ttycon_on as u64 {
        CON_Hide();
        tcsetattr(0i32, 1i32, &mut TTY_tc);
    }
    fcntl(0i32, 4i32, fcntl(0i32, 3i32, 0i32) & !0o4000i32);
}
static mut TTY_tc: termios =
    termios{c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,};
#[no_mangle]
pub unsafe extern "C" fn CON_Hide() {
    if 0 != ttycon_on as u64 {
        let mut i: libc::c_int = 0;
        if 0 != ttycon_hide { ttycon_hide += 1; return }
        if TTY_con.cursor > 0i32 {
            i = 0i32;
            while i < TTY_con.cursor { CON_Back(); i += 1 }
        }
        i =
            strlen(b"tty]\x00" as *const u8 as *const libc::c_char) as
                libc::c_int;
        while i > 0i32 { CON_Back(); i -= 1 }
        ttycon_hide += 1
    };
}
static mut ttycon_hide: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn CON_Back() {
    let mut key: libc::c_char = 0;
    let mut size: size_t = 0;
    key = '\u{8}' as i32 as libc::c_char;
    size =
        write(1i32, &mut key as *mut libc::c_char as *const libc::c_void,
              1i32 as size_t) as size_t;
    key = ' ' as i32 as libc::c_char;
    size =
        write(1i32, &mut key as *mut libc::c_char as *const libc::c_void,
              1i32 as size_t) as size_t;
    key = '\u{8}' as i32 as libc::c_char;
    size =
        write(1i32, &mut key as *mut libc::c_char as *const libc::c_void,
              1i32 as size_t) as size_t;
}
#[no_mangle]
pub static mut TTY_con: field_t =
    field_t{cursor: 0, scroll: 0, widthInChars: 0, buffer: [0; 256],};
// general flag to tell about tty console mode
#[no_mangle]
pub static mut ttycon_on: qboolean = qfalse;
#[no_mangle]
pub unsafe extern "C" fn CON_Init() {
    let mut tc: termios =
        termios{c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: 0,
                c_line: 0,
                c_cc: [0; 32],
                c_ispeed: 0,
                c_ospeed: 0,};
    signal(21i32,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1i32 as libc::intptr_t));
    signal(22i32,
           ::std::mem::transmute::<libc::intptr_t,
                                   __sighandler_t>(1i32 as libc::intptr_t));
    signal(18i32, Some(CON_SigCont));
    fcntl(0i32, 4i32, fcntl(0i32, 3i32, 0i32) | 0o4000i32);
    if 0 == stdinIsATTY as u64 {
        Com_Printf(b"tty console mode disabled\n\x00" as *const u8 as
                       *const libc::c_char);
        ttycon_on = qfalse;
        stdin_active = qtrue;
        return
    }
    Field_Clear(&mut TTY_con);
    tcgetattr(0i32, &mut TTY_tc);
    TTY_erase = TTY_tc.c_cc[2usize] as libc::c_int;
    TTY_eof = TTY_tc.c_cc[4usize] as libc::c_int;
    tc = TTY_tc;
    tc.c_lflag &= !(0o10i32 | 0o2i32) as libc::c_uint;
    tc.c_iflag &= !(0o40i32 | 0o20i32) as libc::c_uint;
    tc.c_cc[6usize] = 1i32 as cc_t;
    tc.c_cc[5usize] = 0i32 as cc_t;
    tcsetattr(0i32, 1i32, &mut tc);
    ttycon_on = qtrue;
    ttycon_hide = 1i32;
    CON_Show();
}
#[no_mangle]
pub unsafe extern "C" fn CON_Show() {
    if 0 != ttycon_on as u64 {
        let mut i: libc::c_int = 0;
        if ttycon_hide > 0i32 {
        } else {
            __assert_fail(b"ttycon_hide>0\x00" as *const u8 as
                              *const libc::c_char,
                          b"code/sys/con_tty.c\x00" as *const u8 as
                              *const libc::c_char, 147i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 20],
                                                    &[libc::c_char; 20]>(b"void CON_Show(void)\x00")).as_ptr());
        }
        ttycon_hide -= 1;
        if ttycon_hide == 0i32 {
            let mut size: size_t = 0;
            size =
                write(1i32,
                      b"tty]\x00" as *const u8 as *const libc::c_char as
                          *const libc::c_void,
                      strlen(b"tty]\x00" as *const u8 as *const libc::c_char))
                    as size_t;
            if 0 != TTY_con.cursor {
                i = 0i32;
                while i < TTY_con.cursor {
                    size =
                        write(1i32,
                              TTY_con.buffer.as_mut_ptr().offset(i as isize)
                                  as *const libc::c_void, 1i32 as size_t) as
                            size_t;
                    i += 1
                }
            }
        }
    };
}
static mut TTY_eof: libc::c_int = 0;
// some key codes that the terminal may be using, initialised on start up
#[no_mangle]
pub static mut TTY_erase: libc::c_int = 0;
#[no_mangle]
pub static mut stdin_active: qboolean = qfalse;
/*
==================
CON_SigCont
Reinitialize console input after receiving SIGCONT, as on Linux the terminal seems to lose all
set attributes if user did CTRL+Z and then does fg again.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CON_SigCont(mut signum: libc::c_int) { CON_Init(); }
#[no_mangle]
pub unsafe extern "C" fn Hist_Add(mut field: *mut field_t) {
    let mut i: libc::c_int = 0;
    if 0 == (*field).cursor { return }
    if hist_count <= 32i32 {
    } else {
        __assert_fail(b"hist_count <= CON_HISTORY\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 196i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void Hist_Add(field_t *)\x00")).as_ptr());
    }
    if hist_count >= 0i32 {
    } else {
        __assert_fail(b"hist_count >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 197i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void Hist_Add(field_t *)\x00")).as_ptr());
    }
    if hist_current >= -1i32 {
    } else {
        __assert_fail(b"hist_current >= -1\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 198i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void Hist_Add(field_t *)\x00")).as_ptr());
    }
    if hist_current <= hist_count {
    } else {
        __assert_fail(b"hist_current <= hist_count\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 199i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"void Hist_Add(field_t *)\x00")).as_ptr());
    }
    i = 32i32 - 1i32;
    while i > 0i32 {
        ttyEditLines[i as usize] = ttyEditLines[(i - 1i32) as usize];
        i -= 1
    }
    ttyEditLines[0usize] = *field;
    if hist_count < 32i32 { hist_count += 1 }
    hist_current = -1i32;
}
static mut hist_current: libc::c_int = -1i32;
static mut hist_count: libc::c_int = 0i32;
// This is somewhat of aduplicate of the graphical console history
// but it's safer more modular to have our own here
static mut ttyEditLines: [field_t; 32] =
    [field_t{cursor: 0, scroll: 0, widthInChars: 0, buffer: [0; 256],}; 32];
#[no_mangle]
pub unsafe extern "C" fn Hist_Prev() -> *mut field_t {
    let mut hist_prev: libc::c_int = 0;
    if hist_count <= 32i32 {
    } else {
        __assert_fail(b"hist_count <= CON_HISTORY\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 221i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Prev(void)\x00")).as_ptr());
    }
    if hist_count >= 0i32 {
    } else {
        __assert_fail(b"hist_count >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 222i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Prev(void)\x00")).as_ptr());
    }
    if hist_current >= -1i32 {
    } else {
        __assert_fail(b"hist_current >= -1\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 223i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Prev(void)\x00")).as_ptr());
    }
    if hist_current <= hist_count {
    } else {
        __assert_fail(b"hist_current <= hist_count\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 224i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Prev(void)\x00")).as_ptr());
    }
    hist_prev = hist_current + 1i32;
    if hist_prev >= hist_count { return 0 as *mut field_t }
    hist_current += 1;
    return &mut *ttyEditLines.as_mut_ptr().offset(hist_current as isize) as
               *mut field_t;
}
#[no_mangle]
pub unsafe extern "C" fn Hist_Next() -> *mut field_t {
    if hist_count <= 32i32 {
    } else {
        __assert_fail(b"hist_count <= CON_HISTORY\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 241i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Next(void)\x00")).as_ptr());
    }
    if hist_count >= 0i32 {
    } else {
        __assert_fail(b"hist_count >= 0\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 242i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Next(void)\x00")).as_ptr());
    }
    if hist_current >= -1i32 {
    } else {
        __assert_fail(b"hist_current >= -1\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 243i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Next(void)\x00")).as_ptr());
    }
    if hist_current <= hist_count {
    } else {
        __assert_fail(b"hist_current <= hist_count\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/sys/con_tty.c\x00" as *const u8 as
                          *const libc::c_char, 244i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 25],
                                                &[libc::c_char; 25]>(b"field_t *Hist_Next(void)\x00")).as_ptr());
    }
    if hist_current >= 0i32 { hist_current -= 1 }
    if hist_current == -1i32 { return 0 as *mut field_t }
    return &mut *ttyEditLines.as_mut_ptr().offset(hist_current as isize) as
               *mut field_t;
}
#[no_mangle]
pub unsafe extern "C" fn CON_Print(mut msg: *const libc::c_char) {
    if 0 == *msg.offset(0isize) { return }
    CON_Hide();
    if !com_ansiColor.is_null() && 0 != (*com_ansiColor).integer {
        Sys_AnsiColorPrint(msg);
    } else { fputs(msg, stderr); }
    if 0 == ttycon_on as u64 { return }
    if *msg.offset(strlen(msg).wrapping_sub(1i32 as libc::c_ulong) as isize)
           as libc::c_int == '\n' as i32 {
        CON_Show();
        while ttycon_show_overdue > 0i32 {
            CON_Show();
            ttycon_show_overdue -= 1
        }
    } else { ttycon_show_overdue += 1 };
}
static mut ttycon_show_overdue: libc::c_int = 0i32;