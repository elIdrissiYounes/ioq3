#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(custom_attribute, libc)]
extern crate libc;
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src =
      "ioq3/code/sys/sys_local.h"]
pub mod sys_local_h {
    use super::{libc};
}
#[header_src =
      "ioq3/code/sys/con_log.c"]
pub mod con_log_c { }
use self::string_h::{memcpy, strlen};
#[no_mangle]
pub unsafe extern "C" fn CON_LogSize() -> libc::c_uint {
    if readPos <= writePos {
        return writePos.wrapping_sub(readPos)
    } else {
        return writePos.wrapping_add(32768i32 as
                                         libc::c_uint).wrapping_sub(readPos)
    };
}
static mut readPos: libc::c_uint = 0i32 as libc::c_uint;
static mut writePos: libc::c_uint = 0i32 as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn CON_LogWrite(mut in_0: *const libc::c_char)
 -> libc::c_uint {
    let mut length: libc::c_uint = strlen(in_0) as libc::c_uint;
    let mut firstChunk: libc::c_uint = 0;
    let mut secondChunk: libc::c_uint = 0;
    while CON_LogFree() < length && CON_LogSize() > 0i32 as libc::c_uint {
        while consoleLog[readPos as usize] as libc::c_int != '\n' as i32 &&
                  CON_LogSize() > 1i32 as libc::c_uint {
            readPos =
                readPos.wrapping_add(1i32 as
                                         libc::c_uint).wrapping_rem(32768i32
                                                                        as
                                                                        libc::c_uint)
        }
        readPos =
            readPos.wrapping_add(1i32 as
                                     libc::c_uint).wrapping_rem(32768i32 as
                                                                    libc::c_uint)
    }
    if CON_LogFree() < length { return 0i32 as libc::c_uint }
    if writePos.wrapping_add(length) > 32768i32 as libc::c_uint {
        firstChunk = (32768i32 as libc::c_uint).wrapping_sub(writePos);
        secondChunk = length.wrapping_sub(firstChunk)
    } else { firstChunk = length; secondChunk = 0i32 as libc::c_uint }
    memcpy(consoleLog.as_mut_ptr().offset(writePos as isize) as
               *mut libc::c_void, in_0 as *const libc::c_void,
           firstChunk as libc::c_ulong);
    memcpy(consoleLog.as_mut_ptr() as *mut libc::c_void,
           in_0.offset(firstChunk as isize) as *const libc::c_void,
           secondChunk as libc::c_ulong);
    writePos =
        writePos.wrapping_add(length).wrapping_rem(32768i32 as libc::c_uint);
    return length;
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
static mut consoleLog: [libc::c_char; 32768] = [0; 32768];
/*
==================
CON_LogFree
==================
*/
unsafe extern "C" fn CON_LogFree() -> libc::c_uint {
    return (32768i32 as
                libc::c_uint).wrapping_sub(CON_LogSize()).wrapping_sub(1i32 as
                                                                           libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn CON_LogRead(mut out: *mut libc::c_char,
                                     mut outSize: libc::c_uint)
 -> libc::c_uint {
    let mut firstChunk: libc::c_uint = 0;
    let mut secondChunk: libc::c_uint = 0;
    if CON_LogSize() < outSize { outSize = CON_LogSize() }
    if readPos.wrapping_add(outSize) > 32768i32 as libc::c_uint {
        firstChunk = (32768i32 as libc::c_uint).wrapping_sub(readPos);
        secondChunk = outSize.wrapping_sub(firstChunk)
    } else { firstChunk = outSize; secondChunk = 0i32 as libc::c_uint }
    memcpy(out as *mut libc::c_void,
           consoleLog.as_mut_ptr().offset(readPos as isize) as
               *const libc::c_void, firstChunk as libc::c_ulong);
    memcpy(out.offset(firstChunk as isize) as *mut libc::c_void,
           consoleLog.as_mut_ptr() as *const libc::c_void,
           secondChunk as libc::c_ulong);
    readPos =
        readPos.wrapping_add(outSize).wrapping_rem(32768i32 as libc::c_uint);
    return outSize;
}