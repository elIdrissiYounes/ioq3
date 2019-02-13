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
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __int32_t = libc::c_int;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stdarg.h"]
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
    use super::vararg::{__builtin_va_list};
}
#[header_src = "/usr/include/ctype.h"]
pub mod ctype_h {
    pub type unnamed = libc::c_uint;
    pub const _ISalnum: unnamed = 8;
    pub const _ISpunct: unnamed = 4;
    pub const _IScntrl: unnamed = 2;
    pub const _ISblank: unnamed = 1;
    pub const _ISgraph: unnamed = 32768;
    pub const _ISprint: unnamed = 16384;
    pub const _ISspace: unnamed = 8192;
    pub const _ISxdigit: unnamed = 4096;
    pub const _ISdigit: unnamed = 2048;
    pub const _ISalpha: unnamed = 1024;
    pub const _ISlower: unnamed = 512;
    pub const _ISupper: unnamed = 256;
    use super::{libc};
    use super::types_h::{__int32_t};
    extern "C" {
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
        #[no_mangle]
        pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
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
//
    // q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
  VM Considerations

  The VM can not use the standard system headers because we aren't really
  using the compiler they were meant for.  We use bg_lib.h which contains
  prototypes for the functions we define for our own use in bg_lib.c.

  When writing mods, please add needed headers HERE, do not start including
  stuff like <stdio.h> in the various .c files that make up each of the VMs
  since you will be including system headers files can will have issues.

  Remember, if you use a C library function that is not defined in bg_lib.c,
  you will have to add your own version for support in the VM.

 **********************************************************************/
    //=============================================================
    pub type byte = libc::c_uchar;
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    // parameters to the main Error routine
    pub type unnamed_0 = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed_0 = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed_0 = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed_0 = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed_0 = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed_0 = 0;
    //=============================================
    // 64-bit integers for global rankings interface
// implemented as a struct for qvm compatibility
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qint64 {
        pub b0: byte,
        pub b1: byte,
        pub b2: byte,
        pub b3: byte,
        pub b4: byte,
        pub b5: byte,
        pub b6: byte,
        pub b7: byte,
    }
    use super::{libc};
    extern "C" {
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
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
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strtod(__nptr: *const libc::c_char,
                      __endptr: *mut *mut libc::c_char) -> libc::c_double;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_platform.h"]
pub mod q_platform_h {
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.c"]
pub mod q_shared_c {
    use super::{libc};
    use super::q_shared_h::{qint64};
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::types_h::{__int32_t};
use self::stddef_h::{size_t};
use self::stdarg_h::{va_list};
use self::ctype_h::{unnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank, _ISgraph,
                    _ISprint, _ISspace, _ISxdigit, _ISdigit, _ISalpha,
                    _ISlower, _ISupper, __ctype_b_loc, __ctype_tolower_loc,
                    __ctype_toupper_loc};
use self::q_shared_h::{byte, floatint_t, qboolean, qtrue, qfalse, unnamed_0,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, qint64, Com_Error, Com_Printf};
use self::stdio_h::{vsnprintf};
use self::string_h::{memmove, strcpy, strncpy, strcat, strcmp, strchr,
                     strrchr, strlen};
use self::stdlib_h::{strtod};
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -128i32 && __c < 256i32 {
               *(*__ctype_tolower_loc()).offset(__c as isize)
           } else { __c };
}
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -128i32 && __c < 256i32 {
               *(*__ctype_toupper_loc()).offset(__c as isize)
           } else { __c };
}
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
// this is for determining if we have an asm version of a C function
// don't include the C bits if included from qasm.h
// for windows fastcall option
//================================================================= WIN64/32 ===
//============================================================== MAC OS X ===
//================================================================= LINUX ===
//=================================================================== BSD ===
//================================================================= SUNOS ===
//================================================================== IRIX ===
//================================================================== Q3VM ===
//===========================================================================
//catch missing defines in above blocks
//endianness
#[no_mangle]
pub unsafe extern "C" fn CopyShortSwap(mut dest: *mut libc::c_void,
                                       mut src: *mut libc::c_void) {
    let mut to: *mut byte = dest as *mut byte;
    let mut from: *mut byte = src as *mut byte;
    *to.offset(0isize) = *from.offset(1isize);
    *to.offset(1isize) = *from.offset(0isize);
}
#[no_mangle]
pub unsafe extern "C" fn CopyLongSwap(mut dest: *mut libc::c_void,
                                      mut src: *mut libc::c_void) {
    let mut to: *mut byte = dest as *mut byte;
    let mut from: *mut byte = src as *mut byte;
    *to.offset(0isize) = *from.offset(3isize);
    *to.offset(1isize) = *from.offset(2isize);
    *to.offset(2isize) = *from.offset(1isize);
    *to.offset(3isize) = *from.offset(0isize);
}
#[no_mangle]
pub unsafe extern "C" fn ShortSwap(mut l: libc::c_short) -> libc::c_short {
    let mut b1: byte = 0;
    let mut b2: byte = 0;
    b1 = (l as libc::c_int & 255i32) as byte;
    b2 = (l as libc::c_int >> 8i32 & 255i32) as byte;
    return (((b1 as libc::c_int) << 8i32) + b2 as libc::c_int) as
               libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn LongSwap(mut l: libc::c_int) -> libc::c_int {
    let mut b1: byte = 0;
    let mut b2: byte = 0;
    let mut b3: byte = 0;
    let mut b4: byte = 0;
    b1 = (l & 255i32) as byte;
    b2 = (l >> 8i32 & 255i32) as byte;
    b3 = (l >> 16i32 & 255i32) as byte;
    b4 = (l >> 24i32 & 255i32) as byte;
    return ((b1 as libc::c_int) << 24i32) + ((b2 as libc::c_int) << 16i32) +
               ((b3 as libc::c_int) << 8i32) + b4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FloatSwap(mut f: *const libc::c_float)
 -> libc::c_float {
    let mut out: floatint_t = floatint_t{f: 0.,};
    out.f = *f;
    out.ui = LongSwap(out.ui as libc::c_int) as libc::c_uint;
    return out.f;
}
// ^[0-9a-zA-Z]
#[no_mangle]
pub unsafe extern "C" fn Q_IsColorString(mut p: *const libc::c_char)
 -> qboolean {
    if p.is_null() { return qfalse }
    if *p.offset(0isize) as libc::c_int != '^' as i32 { return qfalse }
    if *p.offset(1isize) as libc::c_int == 0i32 { return qfalse }
    if (*p.offset(1isize) as libc::c_int) < 0i32 { return qfalse }
    if *(*__ctype_b_loc()).offset(*p.offset(1isize) as libc::c_int as isize)
           as libc::c_int &
           _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0i32 {
        return qfalse
    }
    return qtrue;
}
//=============================================
#[no_mangle]
pub unsafe extern "C" fn Com_Clamp(mut min: libc::c_float,
                                   mut max: libc::c_float,
                                   mut value: libc::c_float)
 -> libc::c_float {
    if value < min { return min }
    if value > max { return max }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn COM_SkipPath(mut pathname: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    last = pathname;
    while 0 != *pathname {
        if *pathname as libc::c_int == '/' as i32 {
            last = pathname.offset(1isize)
        }
        pathname = pathname.offset(1isize)
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn COM_GetExtension(mut name: *const libc::c_char)
 -> *const libc::c_char {
    let mut dot: *const libc::c_char = strrchr(name, '.' as i32);
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    if !dot.is_null() &&
           {
               slash = strrchr(name, '/' as i32);
               slash.is_null() || slash < dot
           } {
        return dot.offset(1isize)
    } else { return b"\x00" as *const u8 as *const libc::c_char };
}
#[no_mangle]
pub unsafe extern "C" fn COM_StripExtension(mut in_0: *const libc::c_char,
                                            mut out: *mut libc::c_char,
                                            mut destsize: libc::c_int) {
    let mut dot: *const libc::c_char = strrchr(in_0, '.' as i32);
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    if !dot.is_null() &&
           {
               slash = strrchr(in_0, '/' as i32);
               slash.is_null() || slash < dot
           } {
        destsize =
            (if (destsize as libc::c_long) <
                    dot.wrapping_offset_from(in_0) as libc::c_long +
                        1i32 as libc::c_long {
                 destsize as libc::c_long
             } else {
                 dot.wrapping_offset_from(in_0) as libc::c_long +
                     1i32 as libc::c_long
             }) as libc::c_int
    }
    if in_0 == out && destsize > 1i32 {
        *out.offset((destsize - 1i32) as isize) =
            '\u{0}' as i32 as libc::c_char
    } else { Q_strncpyz(out, in_0, destsize); };
}
// buffer size safe library replacements
#[no_mangle]
pub unsafe extern "C" fn Q_strncpyz(mut dest: *mut libc::c_char,
                                    mut src: *const libc::c_char,
                                    mut destsize: libc::c_int) {
    if dest.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Q_strncpyz: NULL dest\x00" as *const u8 as
                      *const libc::c_char);
    }
    if src.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Q_strncpyz: NULL src\x00" as *const u8 as
                      *const libc::c_char);
    }
    if destsize < 1i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Q_strncpyz: destsize < 1\x00" as *const u8 as
                      *const libc::c_char);
    }
    strncpy(dest, src, (destsize - 1i32) as libc::c_ulong);
    *dest.offset((destsize - 1i32) as isize) = 0i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn COM_CompareExtension(mut in_0: *const libc::c_char,
                                              mut ext: *const libc::c_char)
 -> qboolean {
    let mut inlen: libc::c_int = 0;
    let mut extlen: libc::c_int = 0;
    inlen = strlen(in_0) as libc::c_int;
    extlen = strlen(ext) as libc::c_int;
    if extlen <= inlen {
        in_0 = in_0.offset((inlen - extlen) as isize);
        if 0 == Q_stricmp(in_0, ext) { return qtrue }
    }
    return qfalse;
}
// portable case insensitive compare
#[no_mangle]
pub unsafe extern "C" fn Q_stricmp(mut s1: *const libc::c_char,
                                   mut s2: *const libc::c_char)
 -> libc::c_int {
    return if !s1.is_null() && !s2.is_null() {
               Q_stricmpn(s1, s2, 99999i32)
           } else { -1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn Q_stricmpn(mut s1: *const libc::c_char,
                                    mut s2: *const libc::c_char,
                                    mut n: libc::c_int) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    if s1.is_null() {
        if s2.is_null() { return 0i32 } else { return -1i32 }
    } else { if s2.is_null() { return 1i32 } }
    loop  {
        let fresh0 = s1;
        s1 = s1.offset(1);
        c1 = *fresh0 as libc::c_int;
        let fresh1 = s2;
        s2 = s2.offset(1);
        c2 = *fresh1 as libc::c_int;
        let fresh2 = n;
        n = n - 1;
        if 0 == fresh2 { return 0i32 }
        if c1 != c2 {
            if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
                c1 -= 'a' as i32 - 'A' as i32
            }
            if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
                c2 -= 'a' as i32 - 'A' as i32
            }
            if c1 != c2 { return if c1 < c2 { -1i32 } else { 1i32 } }
        }
        if !(0 != c1) { break ; }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn COM_DefaultExtension(mut path: *mut libc::c_char,
                                              mut maxSize: libc::c_int,
                                              mut extension:
                                                  *const libc::c_char) {
    let mut dot: *const libc::c_char = strrchr(path, '.' as i32);
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    if !dot.is_null() &&
           {
               slash = strrchr(path, '/' as i32);
               slash.is_null() || slash < dot
           } {
        return
    } else { Q_strcat(path, maxSize, extension); };
}
#[no_mangle]
pub unsafe extern "C" fn Q_strcat(mut dest: *mut libc::c_char,
                                  mut size: libc::c_int,
                                  mut src: *const libc::c_char) {
    let mut l1: libc::c_int = 0;
    l1 = strlen(dest) as libc::c_int;
    if l1 >= size {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Q_strcat: already overflowed\x00" as *const u8 as
                      *const libc::c_char);
    }
    Q_strncpyz(dest.offset(l1 as isize), src, size - l1);
}
#[no_mangle]
pub unsafe extern "C" fn COM_BeginParseSession(mut name:
                                                   *const libc::c_char) {
    com_lines = 1i32;
    com_tokenline = 0i32;
    Com_sprintf(com_parsename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char, name);
}
static mut com_parsename: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub unsafe extern "C" fn Com_sprintf(mut dest: *mut libc::c_char,
                                     mut size: libc::c_int,
                                     mut fmt: *const libc::c_char, ...)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    len = vsnprintf(dest, size as libc::c_ulong, fmt, argptr);
    if len >= size {
        Com_Printf(b"Com_sprintf: Output length %d too short, require %d bytes.\n\x00"
                       as *const u8 as *const libc::c_char, size, len + 1i32);
    }
    return len;
}
static mut com_tokenline: libc::c_int = 0;
static mut com_lines: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn COM_GetCurrentParseLine() -> libc::c_int {
    if 0 != com_tokenline { return com_tokenline }
    return com_lines;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Parse(mut data_p: *mut *mut libc::c_char)
 -> *mut libc::c_char {
    return COM_ParseExt(data_p, qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseExt(mut data_p: *mut *mut libc::c_char,
                                      mut allowLineBreaks: qboolean)
 -> *mut libc::c_char {
    let mut c: libc::c_int = 0i32;
    let mut len: libc::c_int = 0;
    let mut hasNewLines: qboolean = qfalse;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = *data_p;
    len = 0i32;
    com_token[0usize] = 0i32 as libc::c_char;
    com_tokenline = 0i32;
    if data.is_null() {
        *data_p = 0 as *mut libc::c_char;
        return com_token.as_mut_ptr()
    }
    loop  {
        data = SkipWhitespace(data, &mut hasNewLines);
        if data.is_null() {
            *data_p = 0 as *mut libc::c_char;
            return com_token.as_mut_ptr()
        }
        if 0 != hasNewLines as libc::c_uint && 0 == allowLineBreaks as u64 {
            *data_p = data;
            return com_token.as_mut_ptr()
        }
        c = *data as libc::c_int;
        // skip double slash comments
        if c == '/' as i32 &&
               *data.offset(1isize) as libc::c_int == '/' as i32 {
            data = data.offset(2isize);
            while 0 != *data as libc::c_int &&
                      *data as libc::c_int != '\n' as i32 {
                data = data.offset(1isize)
            }
        } else {
            // skip /* */ comments
            if !(c == '/' as i32 &&
                     *data.offset(1isize) as libc::c_int == '*' as i32) {
                break ;
            }
            data = data.offset(2isize);
            while 0 != *data as libc::c_int &&
                      (*data as libc::c_int != '*' as i32 ||
                           *data.offset(1isize) as libc::c_int != '/' as i32)
                  {
                if *data as libc::c_int == '\n' as i32 { com_lines += 1 }
                data = data.offset(1isize)
            }
            if 0 != *data { data = data.offset(2isize) }
        }
    }
    com_tokenline = com_lines;
    if c == '\"' as i32 {
        data = data.offset(1isize);
        loop  {
            let fresh3 = data;
            data = data.offset(1);
            c = *fresh3 as libc::c_int;
            if c == '\"' as i32 || 0 == c {
                com_token[len as usize] = 0i32 as libc::c_char;
                *data_p = data;
                return com_token.as_mut_ptr()
            }
            if c == '\n' as i32 { com_lines += 1 }
            if len < 1024i32 - 1i32 {
                com_token[len as usize] = c as libc::c_char;
                len += 1
            }
        }
    }
    loop  {
        if len < 1024i32 - 1i32 {
            com_token[len as usize] = c as libc::c_char;
            len += 1
        }
        data = data.offset(1isize);
        c = *data as libc::c_int;
        if !(c > 32i32) { break ; }
    }
    com_token[len as usize] = 0i32 as libc::c_char;
    *data_p = data;
    return com_token.as_mut_ptr();
}
/*
================
Swap_Init
================
*/
/*
void Swap_Init (void)
{
	byte	swaptest[2] = {1,0};

// set the byte swapping variables in a portable manner	
	if ( *(short *)swaptest == 1)
	{
		_BigShort = ShortSwap;
		_LittleShort = ShortNoSwap;
		_BigLong = LongSwap;
		_LittleLong = LongNoSwap;
		_BigLong64 = Long64Swap;
		_LittleLong64 = Long64NoSwap;
		_BigFloat = FloatSwap;
		_LittleFloat = FloatNoSwap;
	}
	else
	{
		_BigShort = ShortNoSwap;
		_LittleShort = ShortSwap;
		_BigLong = LongNoSwap;
		_LittleLong = LongSwap;
		_BigLong64 = Long64NoSwap;
		_LittleLong64 = Long64Swap;
		_BigFloat = FloatNoSwap;
		_LittleFloat = FloatSwap;
	}

}
*/
/*
============================================================================

PARSING

============================================================================
*/
static mut com_token: [libc::c_char; 1024] = [0; 1024];
/*
==============
COM_Parse

Parse a token out of a string
Will never return NULL, just empty strings

If "allowLineBreaks" is qtrue then an empty
string will be returned if the next token is
a newline.
==============
*/
unsafe extern "C" fn SkipWhitespace(mut data: *mut libc::c_char,
                                    mut hasNewLines: *mut qboolean)
 -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    loop  {
        c = *data as libc::c_int;
        if !(c <= ' ' as i32) { break ; }
        if 0 == c { return 0 as *mut libc::c_char }
        if c == '\n' as i32 { com_lines += 1; *hasNewLines = qtrue }
        data = data.offset(1isize)
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Compress(mut data_p: *mut libc::c_char)
 -> libc::c_int {
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut newline: qboolean = qfalse;
    let mut whitespace: qboolean = qfalse;
    out = data_p;
    in_0 = out;
    if !in_0.is_null() {
        loop  {
            c = *in_0 as libc::c_int;
            if !(c != 0i32) { break ; }
            if c == '/' as i32 &&
                   *in_0.offset(1isize) as libc::c_int == '/' as i32 {
                while 0 != *in_0 as libc::c_int &&
                          *in_0 as libc::c_int != '\n' as i32 {
                    in_0 = in_0.offset(1isize)
                }
            } else if c == '/' as i32 &&
                          *in_0.offset(1isize) as libc::c_int == '*' as i32 {
                while 0 != *in_0 as libc::c_int &&
                          (*in_0 as libc::c_int != '*' as i32 ||
                               *in_0.offset(1isize) as libc::c_int !=
                                   '/' as i32) {
                    in_0 = in_0.offset(1isize)
                }
                if 0 != *in_0 { in_0 = in_0.offset(2isize) }
            } else if c == '\n' as i32 || c == '\r' as i32 {
                newline = qtrue;
                in_0 = in_0.offset(1isize)
            } else if c == ' ' as i32 || c == '\t' as i32 {
                whitespace = qtrue;
                in_0 = in_0.offset(1isize)
            } else {
                if 0 != newline as u64 {
                    let fresh4 = out;
                    out = out.offset(1);
                    *fresh4 = '\n' as i32 as libc::c_char;
                    newline = qfalse;
                    whitespace = qfalse
                }
                if 0 != whitespace as u64 {
                    let fresh5 = out;
                    out = out.offset(1);
                    *fresh5 = ' ' as i32 as libc::c_char;
                    whitespace = qfalse
                }
                if c == '\"' as i32 {
                    let fresh6 = out;
                    out = out.offset(1);
                    *fresh6 = c as libc::c_char;
                    in_0 = in_0.offset(1isize);
                    loop  {
                        c = *in_0 as libc::c_int;
                        if !(0 != c && c != '\"' as i32) { break ; }
                        let fresh7 = out;
                        out = out.offset(1);
                        *fresh7 = c as libc::c_char;
                        in_0 = in_0.offset(1isize)
                    }
                    if c == '\"' as i32 {
                        let fresh8 = out;
                        out = out.offset(1);
                        *fresh8 = c as libc::c_char;
                        in_0 = in_0.offset(1isize)
                    }
                } else {
                    *out = c as libc::c_char;
                    out = out.offset(1isize);
                    in_0 = in_0.offset(1isize)
                }
            }
        }
        *out = 0i32 as libc::c_char
    }
    return out.wrapping_offset_from(data_p) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseError(mut format: *mut libc::c_char, ...) {
    static mut string: [libc::c_char; 4096] = [0; 4096];
    vsnprintf(string.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
              format, argptr_0);
    Com_Printf(b"ERROR: %s, line %d: %s\n\x00" as *const u8 as
                   *const libc::c_char, com_parsename.as_mut_ptr(),
               COM_GetCurrentParseLine(), string.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseWarning(mut format:
                                              *mut libc::c_char, ...) {
    static mut string: [libc::c_char; 4096] = [0; 4096];
    vsnprintf(string.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
              format, argptr_1);
    Com_Printf(b"WARNING: %s, line %d: %s\n\x00" as *const u8 as
                   *const libc::c_char, com_parsename.as_mut_ptr(),
               COM_GetCurrentParseLine(), string.as_mut_ptr());
}
// data is an in/out parm, returns a parsed out token
#[no_mangle]
pub unsafe extern "C" fn COM_MatchToken(mut buf_p: *mut *mut libc::c_char,
                                        mut match_0: *mut libc::c_char) {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    token = COM_Parse(buf_p);
    if 0 != strcmp(token, match_0) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MatchToken: %s != %s\x00" as *const u8 as
                      *const libc::c_char, token, match_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SkipBracedSection(mut program:
                                               *mut *mut libc::c_char,
                                           mut depth: libc::c_int)
 -> qboolean {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    loop  {
        token = COM_ParseExt(program, qtrue);
        if *token.offset(1isize) as libc::c_int == 0i32 {
            if *token.offset(0isize) as libc::c_int == '{' as i32 {
                depth += 1
            } else if *token.offset(0isize) as libc::c_int == '}' as i32 {
                depth -= 1
            }
        }
        if !(0 != depth && !(*program).is_null()) { break ; }
    }
    return (depth == 0i32) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn SkipRestOfLine(mut data: *mut *mut libc::c_char) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    p = *data;
    if 0 == *p { return }
    loop  {
        let fresh9 = p;
        p = p.offset(1);
        c = *fresh9 as libc::c_int;
        if !(c != 0i32) { break ; }
        if !(c == '\n' as i32) { continue ; }
        com_lines += 1;
        break ;
    }
    *data = p;
}
#[no_mangle]
pub unsafe extern "C" fn Parse1DMatrix(mut buf_p: *mut *mut libc::c_char,
                                       mut x: libc::c_int,
                                       mut m: *mut libc::c_float) {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    COM_MatchToken(buf_p,
                   b"(\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
    i = 0i32;
    while i < x {
        token = COM_Parse(buf_p);
        *m.offset(i as isize) = atof(token) as libc::c_float;
        i += 1
    }
    COM_MatchToken(buf_p,
                   b")\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Parse2DMatrix(mut buf_p: *mut *mut libc::c_char,
                                       mut y: libc::c_int, mut x: libc::c_int,
                                       mut m: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    COM_MatchToken(buf_p,
                   b"(\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
    i = 0i32;
    while i < y {
        Parse1DMatrix(buf_p, x, m.offset((i * x) as isize));
        i += 1
    }
    COM_MatchToken(buf_p,
                   b")\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Parse3DMatrix(mut buf_p: *mut *mut libc::c_char,
                                       mut z: libc::c_int, mut y: libc::c_int,
                                       mut x: libc::c_int,
                                       mut m: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    COM_MatchToken(buf_p,
                   b"(\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
    i = 0i32;
    while i < z {
        Parse2DMatrix(buf_p, y, x, m.offset((i * x * y) as isize));
        i += 1
    }
    COM_MatchToken(buf_p,
                   b")\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Com_HexStrToInt(mut str: *const libc::c_char)
 -> libc::c_int {
    if str.is_null() { return -1i32 }
    if *str.offset(0isize) as libc::c_int == '0' as i32 &&
           *str.offset(1isize) as libc::c_int == 'x' as i32 &&
           *str.offset(2isize) as libc::c_int != '\u{0}' as i32 {
        let mut i: libc::c_int = 0;
        let mut n: libc::c_int = 0i32;
        let mut len: libc::c_int = strlen(str) as libc::c_int;
        i = 2i32;
        while i < len {
            let mut digit: libc::c_char = 0;
            n *= 16i32;
            digit =
                {
                    let mut __res: libc::c_int = 0;
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                           > 1i32 as libc::c_ulong {
                        if 0 != 0 {
                            let mut __c: libc::c_int =
                                *str.offset(i as isize) as libc::c_int;
                            __res =
                                if __c < -128i32 || __c > 255i32 {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as
                                                                         isize)
                                }
                        } else {
                            __res =
                                tolower(*str.offset(i as isize) as
                                            libc::c_int)
                        }
                    } else {
                        __res =
                            *(*__ctype_tolower_loc()).offset(*str.offset(i as
                                                                             isize)
                                                                 as
                                                                 libc::c_int
                                                                 as isize)
                    }
                    __res
                } as libc::c_char;
            if digit as libc::c_int >= '0' as i32 &&
                   digit as libc::c_int <= '9' as i32 {
                digit = (digit as libc::c_int - '0' as i32) as libc::c_char
            } else if digit as libc::c_int >= 'a' as i32 &&
                          digit as libc::c_int <= 'f' as i32 {
                digit =
                    (digit as libc::c_int - 'a' as i32 + 10i32) as
                        libc::c_char
            } else { return -1i32 }
            n += digit as libc::c_int;
            i += 1
        }
        return n
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn Com_SkipTokens(mut s: *mut libc::c_char,
                                        mut numTokens: libc::c_int,
                                        mut sep: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut sepCount: libc::c_int = 0i32;
    let mut p: *mut libc::c_char = s;
    while sepCount < numTokens {
        let fresh10 = p;
        p = p.offset(1);
        if 0 != Com_CharIsOneOfCharset(*fresh10, sep) as u64 {
            sepCount += 1;
            while 0 != Com_CharIsOneOfCharset(*p, sep) as u64 {
                p = p.offset(1isize)
            }
        } else if *p as libc::c_int == '\u{0}' as i32 { break ; }
    }
    if sepCount == numTokens { return p } else { return s };
}
//====================================================================
/*
==================
Com_CharIsOneOfCharset
==================
*/
unsafe extern "C" fn Com_CharIsOneOfCharset(mut c: libc::c_char,
                                            mut set: *mut libc::c_char)
 -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while (i as libc::c_ulong) < strlen(set) {
        if *set.offset(i as isize) as libc::c_int == c as libc::c_int {
            return qtrue
        }
        i += 1
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn Com_SkipCharset(mut s: *mut libc::c_char,
                                         mut sep: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut p: *mut libc::c_char = s;
    while !p.is_null() {
        if !(0 != Com_CharIsOneOfCharset(*p, sep) as u64) { break ; }
        p = p.offset(1isize)
    }
    return p;
}
//=============================================
#[no_mangle]
pub unsafe extern "C" fn Q_isprint(mut c: libc::c_int) -> libc::c_int {
    if c >= 0x20i32 && c <= 0x7ei32 { return 1i32 }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Q_islower(mut c: libc::c_int) -> libc::c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 { return 1i32 }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isupper(mut c: libc::c_int) -> libc::c_int {
    if c >= 'A' as i32 && c <= 'Z' as i32 { return 1i32 }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isalpha(mut c: libc::c_int) -> libc::c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 ||
           c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1i32
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isanumber(mut s: *const libc::c_char) -> qboolean {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: libc::c_double = 0.;
    if *s as libc::c_int == '\u{0}' as i32 { return qfalse }
    d = strtod(s, &mut p);
    return (*p as libc::c_int == '\u{0}' as i32) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isintegral(mut f: libc::c_float) -> qboolean {
    return (f as libc::c_int as libc::c_float == f) as libc::c_int as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncmp(mut s1: *const libc::c_char,
                                   mut s2: *const libc::c_char,
                                   mut n: libc::c_int) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    loop  {
        let fresh11 = s1;
        s1 = s1.offset(1);
        c1 = *fresh11 as libc::c_int;
        let fresh12 = s2;
        s2 = s2.offset(1);
        c2 = *fresh12 as libc::c_int;
        let fresh13 = n;
        n = n - 1;
        if 0 == fresh13 { return 0i32 }
        if c1 != c2 { return if c1 < c2 { -1i32 } else { 1i32 } }
        if !(0 != c1) { break ; }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strlwr(mut s1: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = s1;
    while 0 != *s {
        *s =
            {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong >
                       1i32 as libc::c_ulong {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s as libc::c_int;
                        __res =
                            if __c < -128i32 || __c > 255i32 {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            }
                    } else { __res = tolower(*s as libc::c_int) }
                } else {
                    __res =
                        *(*__ctype_tolower_loc()).offset(*s as libc::c_int as
                                                             isize)
                }
                __res
            } as libc::c_char;
        s = s.offset(1isize)
    }
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strupr(mut s1: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = s1;
    while 0 != *s {
        *s =
            {
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong >
                       1i32 as libc::c_ulong {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *s as libc::c_int;
                        __res =
                            if __c < -128i32 || __c > 255i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            }
                    } else { __res = toupper(*s as libc::c_int) }
                } else {
                    __res =
                        *(*__ctype_toupper_loc()).offset(*s as libc::c_int as
                                                             isize)
                }
                __res
            } as libc::c_char;
        s = s.offset(1isize)
    }
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn Q_stristr(mut s: *const libc::c_char,
                                   mut find: *const libc::c_char)
 -> *const libc::c_char {
    let mut c: libc::c_char = 0;
    let mut sc: libc::c_char = 0;
    let mut len: size_t = 0;
    let fresh14 = find;
    find = find.offset(1);
    c = *fresh14;
    if c as libc::c_int != 0i32 {
        if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 {
            c = (c as libc::c_int - ('a' as i32 - 'A' as i32)) as libc::c_char
        }
        len = strlen(find);
        loop  {
            loop  {
                let fresh15 = s;
                s = s.offset(1);
                sc = *fresh15;
                if sc as libc::c_int == 0i32 {
                    return 0 as *const libc::c_char
                }
                if sc as libc::c_int >= 'a' as i32 &&
                       sc as libc::c_int <= 'z' as i32 {
                    sc =
                        (sc as libc::c_int - ('a' as i32 - 'A' as i32)) as
                            libc::c_char
                }
                if !(sc as libc::c_int != c as libc::c_int) { break ; }
            }
            if !(Q_stricmpn(s, find, len as libc::c_int) != 0i32) { break ; }
        }
        s = s.offset(-1isize)
    }
    return s;
}
// strlen that discounts Quake color sequences
#[no_mangle]
pub unsafe extern "C" fn Q_PrintStrlen(mut string: *const libc::c_char)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if string.is_null() { return 0i32 }
    len = 0i32;
    p = string;
    while 0 != *p {
        if 0 != Q_IsColorString(p) as u64 {
            p = p.offset(2isize)
        } else { p = p.offset(1isize); len += 1 }
    }
    return len;
}
// removes color sequences from string
#[no_mangle]
pub unsafe extern "C" fn Q_CleanStr(mut string: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    s = string;
    d = string;
    loop  {
        c = *s as libc::c_int;
        if !(c != 0i32) { break ; }
        if 0 != Q_IsColorString(s) as u64 {
            s = s.offset(1isize)
        } else if c >= 0x20i32 && c <= 0x7ei32 {
            let fresh16 = d;
            d = d.offset(1);
            *fresh16 = c as libc::c_char
        }
        s = s.offset(1isize)
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return string;
}
// Count the number of char tocount encountered in string
#[no_mangle]
pub unsafe extern "C" fn Q_CountChar(mut string: *const libc::c_char,
                                     mut tocount: libc::c_char)
 -> libc::c_int {
    let mut count: libc::c_int = 0;
    count = 0i32;
    while 0 != *string {
        if *string as libc::c_int == tocount as libc::c_int { count += 1 }
        string = string.offset(1isize)
    }
    return count;
}
//=============================================
/*
short	BigShort(short l);
short	LittleShort(short l);
int		BigLong (int l);
int		LittleLong (int l);
qint64  BigLong64 (qint64 l);
qint64  LittleLong64 (qint64 l);
float	BigFloat (const float *l);
float	LittleFloat (const float *l);

void	Swap_Init (void);
*/
#[no_mangle]
pub unsafe extern "C" fn va(mut format: *mut libc::c_char, ...)
 -> *mut libc::c_char {
    // in case va is called by nested functions
    static mut string: [[libc::c_char; 32000]; 2] = [[0; 32000]; 2];
    static mut index: libc::c_int = 0i32;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    buf = string[(index & 1i32) as usize].as_mut_ptr();
    index += 1;
    vsnprintf(buf,
              ::std::mem::size_of::<[libc::c_char; 32000]>() as libc::c_ulong,
              format, argptr_2);
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn Com_TruncateLongString(mut buffer: *mut libc::c_char,
                                                mut s: *const libc::c_char) {
    let mut length: libc::c_int = strlen(s) as libc::c_int;
    if length <= 64i32 {
        Q_strncpyz(buffer, s, 64i32);
    } else {
        Q_strncpyz(buffer, s, 64i32 / 2i32 - 3i32);
        Q_strcat(buffer, 64i32,
                 b" ... \x00" as *const u8 as *const libc::c_char);
        Q_strcat(buffer, 64i32,
                 s.offset(length as
                              isize).offset(-((64i32 / 2i32) as
                                                  isize)).offset(3isize));
    };
}
//=============================================
//
// key / value info strings
//
#[no_mangle]
pub unsafe extern "C" fn Info_ValueForKey(mut s: *const libc::c_char,
                                          mut key: *const libc::c_char)
 -> *mut libc::c_char {
    let mut pkey: [libc::c_char; 8192] = [0; 8192];
    // use two buffers so compares
    static mut value: [[libc::c_char; 8192]; 2] = [[0; 8192]; 2];
    // work without stomping on each other
    static mut valueindex: libc::c_int = 0i32;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if s.is_null() || key.is_null() {
        return b"\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if strlen(s) >= 8192i32 as libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Info_ValueForKey: oversize infostring\x00" as *const u8 as
                      *const libc::c_char);
    }
    valueindex ^= 1i32;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1isize) }
    loop  {
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if 0 == *s {
                return b"\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char
            }
            let fresh18 = o;
            o = o.offset(1);
            let fresh17 = s;
            s = s.offset(1);
            *fresh18 = *fresh17
        }
        *o = 0i32 as libc::c_char;
        s = s.offset(1isize);
        o = value[valueindex as usize].as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && 0 != *s as libc::c_int {
            let fresh20 = o;
            o = o.offset(1);
            let fresh19 = s;
            s = s.offset(1);
            *fresh20 = *fresh19
        }
        *o = 0i32 as libc::c_char;
        if 0 == Q_stricmp(key, pkey.as_mut_ptr()) {
            return value[valueindex as usize].as_mut_ptr()
        }
        if 0 == *s { break ; }
        s = s.offset(1isize)
    }
    return b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey(mut s: *mut libc::c_char,
                                        mut key: *const libc::c_char) {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pkey: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if strlen(s) >= 1024i32 as libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Info_RemoveKey: oversize infostring\x00" as *const u8 as
                      *const libc::c_char);
    }
    if !strchr(key, '\\' as i32).is_null() { return }
    loop  {
        start = s;
        if *s as libc::c_int == '\\' as i32 { s = s.offset(1isize) }
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if 0 == *s { return }
            let fresh22 = o;
            o = o.offset(1);
            let fresh21 = s;
            s = s.offset(1);
            *fresh22 = *fresh21
        }
        *o = 0i32 as libc::c_char;
        s = s.offset(1isize);
        o = value.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && 0 != *s as libc::c_int {
            if 0 == *s { return }
            let fresh24 = o;
            o = o.offset(1);
            let fresh23 = s;
            s = s.offset(1);
            *fresh24 = *fresh23
        }
        *o = 0i32 as libc::c_char;
        if 0 == strcmp(key, pkey.as_mut_ptr()) {
            memmove(start as *mut libc::c_void, s as *const libc::c_void,
                    strlen(s).wrapping_add(1i32 as libc::c_ulong));
            return
        }
        if 0 == *s { return }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey_Big(mut s: *mut libc::c_char,
                                            mut key: *const libc::c_char) {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pkey: [libc::c_char; 8192] = [0; 8192];
    let mut value: [libc::c_char; 8192] = [0; 8192];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if strlen(s) >= 8192i32 as libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Info_RemoveKey_Big: oversize infostring\x00" as *const u8
                      as *const libc::c_char);
    }
    if !strchr(key, '\\' as i32).is_null() { return }
    loop  {
        start = s;
        if *s as libc::c_int == '\\' as i32 { s = s.offset(1isize) }
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if 0 == *s { return }
            let fresh26 = o;
            o = o.offset(1);
            let fresh25 = s;
            s = s.offset(1);
            *fresh26 = *fresh25
        }
        *o = 0i32 as libc::c_char;
        s = s.offset(1isize);
        o = value.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && 0 != *s as libc::c_int {
            if 0 == *s { return }
            let fresh28 = o;
            o = o.offset(1);
            let fresh27 = s;
            s = s.offset(1);
            *fresh28 = *fresh27
        }
        *o = 0i32 as libc::c_char;
        if 0 == strcmp(key, pkey.as_mut_ptr()) {
            memmove(start as *mut libc::c_void, s as *const libc::c_void,
                    strlen(s).wrapping_add(1i32 as libc::c_ulong));
            return
        }
        if 0 == *s { return }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey(mut s: *mut libc::c_char,
                                             mut key: *const libc::c_char,
                                             mut value: *const libc::c_char) {
    let mut newi: [libc::c_char; 1024] = [0; 1024];
    let mut blacklist: *const libc::c_char =
        b"\\;\"\x00" as *const u8 as *const libc::c_char;
    if strlen(s) >= 1024i32 as libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Info_SetValueForKey: oversize infostring\x00" as *const u8
                      as *const libc::c_char);
    }
    while 0 != *blacklist {
        if !strchr(key, *blacklist as libc::c_int).is_null() ||
               !strchr(value, *blacklist as libc::c_int).is_null() {
            Com_Printf(b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00"
                           as *const u8 as *const libc::c_char,
                       *blacklist as libc::c_int, key, value);
            return
        }
        blacklist = blacklist.offset(1isize)
    }
    Info_RemoveKey(s, key);
    if value.is_null() || 0 == strlen(value) { return }
    Com_sprintf(newi.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"\\%s\\%s\x00" as *const u8 as *const libc::c_char, key,
                value);
    if strlen(newi.as_mut_ptr()).wrapping_add(strlen(s)) >=
           1024i32 as libc::c_ulong {
        Com_Printf(b"Info string length exceeded\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    strcat(newi.as_mut_ptr(), s);
    strcpy(s, newi.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey_Big(mut s: *mut libc::c_char,
                                                 mut key: *const libc::c_char,
                                                 mut value:
                                                     *const libc::c_char) {
    let mut newi: [libc::c_char; 8192] = [0; 8192];
    let mut blacklist: *const libc::c_char =
        b"\\;\"\x00" as *const u8 as *const libc::c_char;
    if strlen(s) >= 8192i32 as libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Info_SetValueForKey: oversize infostring\x00" as *const u8
                      as *const libc::c_char);
    }
    while 0 != *blacklist {
        if !strchr(key, *blacklist as libc::c_int).is_null() ||
               !strchr(value, *blacklist as libc::c_int).is_null() {
            Com_Printf(b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00"
                           as *const u8 as *const libc::c_char,
                       *blacklist as libc::c_int, key, value);
            return
        }
        blacklist = blacklist.offset(1isize)
    }
    Info_RemoveKey_Big(s, key);
    if value.is_null() { return }
    Com_sprintf(newi.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                    as libc::c_int,
                b"\\%s\\%s\x00" as *const u8 as *const libc::c_char, key,
                value);
    if strlen(newi.as_mut_ptr()).wrapping_add(strlen(s)) >=
           8192i32 as libc::c_ulong {
        Com_Printf(b"BIG Info string length exceeded\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    strcat(s, newi.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Info_Validate(mut s: *const libc::c_char)
 -> qboolean {
    if !strchr(s, '\"' as i32).is_null() { return qfalse }
    if !strchr(s, ';' as i32).is_null() { return qfalse }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Info_NextPair(mut head: *mut *const libc::c_char,
                                       mut key: *mut libc::c_char,
                                       mut value: *mut libc::c_char) {
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    s = *head;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1isize) }
    *key.offset(0isize) = 0i32 as libc::c_char;
    *value.offset(0isize) = 0i32 as libc::c_char;
    o = key;
    while *s as libc::c_int != '\\' as i32 {
        if 0 == *s { *o = 0i32 as libc::c_char; *head = s; return }
        let fresh30 = o;
        o = o.offset(1);
        let fresh29 = s;
        s = s.offset(1);
        *fresh30 = *fresh29
    }
    *o = 0i32 as libc::c_char;
    s = s.offset(1isize);
    o = value;
    while *s as libc::c_int != '\\' as i32 && 0 != *s as libc::c_int {
        let fresh32 = o;
        o = o.offset(1);
        let fresh31 = s;
        s = s.offset(1);
        *fresh32 = *fresh31
    }
    *o = 0i32 as libc::c_char;
    *head = s;
}
#[no_mangle]
pub unsafe extern "C" fn ShortNoSwap(mut l: libc::c_short) -> libc::c_short {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn LongNoSwap(mut l: libc::c_int) -> libc::c_int {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn Long64Swap(mut ll: qint64) -> qint64 {
    let mut result: qint64 =
        qint64{b0: 0, b1: 0, b2: 0, b3: 0, b4: 0, b5: 0, b6: 0, b7: 0,};
    result.b0 = ll.b7;
    result.b1 = ll.b6;
    result.b2 = ll.b5;
    result.b3 = ll.b4;
    result.b4 = ll.b3;
    result.b5 = ll.b2;
    result.b6 = ll.b1;
    result.b7 = ll.b0;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Long64NoSwap(mut ll: qint64) -> qint64 { return ll; }
#[no_mangle]
pub unsafe extern "C" fn FloatNoSwap(mut f: *const libc::c_float)
 -> libc::c_float {
    return *f;
}