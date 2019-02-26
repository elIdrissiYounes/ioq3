#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
extern crate libc;
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
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
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type fileHandle_t = libc::c_int;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    /*
==============================================================

VoIP

==============================================================
*/
    // if you change the count of flags be sure to also change VOIP_FLAGNUM
    // spatialized voip message
    // non-spatialized voip message
    // number of flags voip knows. You will have to bump protocol version number if you
// change this.
    /*
==============================================================

COLLISION DETECTION

==============================================================
*/
    // plane types are used to speed some tests
// 0-2 are axial planes
    /*
=================
PlaneTypeForNormal
=================
*/
    // plane_t structure
// !!! if this is changed, it must be changed in asm code too !!!
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cplane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: byte,
        pub signbits: byte,
        pub pad: [byte; 2],
    }
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
    pub type cplane_t = cplane_s;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
    }
}
#[header_src =
      "ioq3/code/botlib/botlib.h"]
pub mod botlib_h {
    //bsp_trace_t hit surface
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_int,
        pub value: libc::c_int,
    }
    pub type bsp_surface_t = bsp_surface_s;
    //remove the bsp_trace_s structure definition l8r on
//a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_trace_s {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub exp_dist: libc::c_float,
        pub sidenum: libc::c_int,
        pub surface: bsp_surface_t,
        pub contents: libc::c_int,
        pub ent: libc::c_int,
    }
    pub type bsp_trace_t = bsp_trace_s;
    //bot AI library exported functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_import_s {
        pub Print: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_char, ...)
                              -> ()>,
        pub Trace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: libc::c_int, _: libc::c_int)
                              -> ()>,
        pub EntityTrace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
        pub PointContents: Option<unsafe extern "C" fn(_: *mut vec_t)
                                      -> libc::c_int>,
        pub inPVS: Option<unsafe extern "C" fn(_: *mut vec_t, _: *mut vec_t)
                              -> libc::c_int>,
        pub BSPEntityData: Option<unsafe extern "C" fn()
                                      -> *mut libc::c_char>,
        pub BSPModelMinsMaxsOrigin: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t)
                                               -> ()>,
        pub BotClientCommand: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_char)
                                         -> ()>,
        pub GetMemory: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FreeMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>,
        pub AvailableMemory: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub HunkAlloc: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FS_FOpenFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut fileHandle_t,
                                                      _: fsMode_t)
                                     -> libc::c_int>,
        pub FS_Read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: libc::c_int,
                                                 _: fileHandle_t)
                                -> libc::c_int>,
        pub FS_Write: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                                  _: libc::c_int,
                                                  _: fileHandle_t)
                                 -> libc::c_int>,
        pub FS_FCloseFile: Option<unsafe extern "C" fn(_: fileHandle_t)
                                      -> ()>,
        pub FS_Seek: Option<unsafe extern "C" fn(_: fileHandle_t,
                                                 _: libc::c_long,
                                                 _: libc::c_int)
                                -> libc::c_int>,
        pub DebugLineCreate: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub DebugLineDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
        pub DebugLineShow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: libc::c_int) -> ()>,
        pub DebugPolygonCreate: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: *mut vec3_t)
                                           -> libc::c_int>,
        pub DebugPolygonDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    }
    pub type botlib_import_t = botlib_import_s;
    use super::{libc};
    use super::q_shared_h::{qboolean, vec3_t, cplane_t, vec_t, fileHandle_t,
                            fsMode_t};
}
#[header_src =
      "ioq3/code/botlib/l_script.h"]
pub mod l_script_h {
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
    /* ****************************************************************************
 * name:		l_script.h
 *
 * desc:		lexicographical parser
 *
 * $Archive: /source/code/botlib/l_script.h $
 *
 *****************************************************************************/
    //undef if binary numbers of the form 0b... or 0B... are not allowed
    //undef if not using the token.intvalue and token.floatvalue
    //use dollar sign also as punctuation
    //maximum token length
    //script flags
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    //string sub type
//---------------
//		the length of the string
//literal sub type
//----------------
//		the ASCII code of the literal
//number sub type
//---------------
    // decimal number
    // hexadecimal number
    // octal number
    // binary number
    //BINARYNUMBERS
    // floating point number
    // integer number
    // long number
    // unsigned number
    //punctuation sub type
//--------------------
    //name sub type
//-------------
//		the length of the name
    //punctuation
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct punctuation_s {
        pub p: *mut libc::c_char,
        pub n: libc::c_int,
        pub next: *mut punctuation_s,
    }
    pub type punctuation_t = punctuation_s;
    //token
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct token_s {
        pub string: [libc::c_char; 1024],
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub intvalue: libc::c_ulong,
        pub floatvalue: libc::c_float,
        pub whitespace_p: *mut libc::c_char,
        pub endwhitespace_p: *mut libc::c_char,
        pub line: libc::c_int,
        pub linescrossed: libc::c_int,
        pub next: *mut token_s,
    }
    pub type token_t = token_s;
    //script file
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct script_s {
        pub filename: [libc::c_char; 1024],
        pub buffer: *mut libc::c_char,
        pub script_p: *mut libc::c_char,
        pub end_p: *mut libc::c_char,
        pub lastscript_p: *mut libc::c_char,
        pub whitespace_p: *mut libc::c_char,
        pub endwhitespace_p: *mut libc::c_char,
        pub length: libc::c_int,
        pub line: libc::c_int,
        pub lastline: libc::c_int,
        pub tokenavailable: libc::c_int,
        pub flags: libc::c_int,
        pub punctuations: *mut punctuation_t,
        pub punctuationtable: *mut *mut punctuation_t,
        pub token: token_t,
        pub next: *mut script_s,
    }
    pub type script_t = script_s;
    use super::{libc};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strncmp(_: *const libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src =
      "ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src =
      "ioq3/code/botlib/l_variadic.h"]
pub mod l_variadic_h {
    use super::l_script_h::{script_t};
    use super::{libc};
    extern "C" {
        //print a script error with filename and line number
        #[no_mangle]
        pub fn ScriptError(script: *mut script_t,
                           str: *mut libc::c_char, ...);
        //print a script warning with filename and line number
        #[no_mangle]
        pub fn ScriptWarning(script: *mut script_t,
                             str: *mut libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/botlib/l_script.c"]
pub mod l_script_c {
    use super::{libc};
    use super::l_script_h::{script_t, token_t, punctuation_t};
}
#[header_src =
      "ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
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
        /* ****************************************************************************
 * name:		l_memory.h
 *
 * desc:		memory management
 *
 * $Archive: /source/code/botlib/l_memory.h $
 *
 *****************************************************************************/
        //#define MEMDEBUG
        //allocate a memory block of the given size
        #[no_mangle]
        pub fn GetMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, Com_sprintf, Q_strncpyz};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t};
use self::string_h::{memcpy, memmove, memset, strcpy, strcat, strcmp, strncmp,
                     strlen};
use self::be_interface_h::{botimport};
use self::l_variadic_h::{ScriptError, ScriptWarning};
use self::l_memory_h::{GetMemory, GetClearedMemory, FreeMemory};
//read a token from the script
#[no_mangle]
pub unsafe extern "C" fn PS_ReadToken(mut script: *mut script_t,
                                      mut token: *mut token_t)
 -> libc::c_int {
    if 0 != (*script).tokenavailable {
        (*script).tokenavailable = 0i32;
        memcpy(token as *mut libc::c_void,
               &mut (*script).token as *mut token_t as *const libc::c_void,
               ::std::mem::size_of::<token_t>() as libc::c_ulong);
        return 1i32
    }
    (*script).lastscript_p = (*script).script_p;
    (*script).lastline = (*script).line;
    memset(token as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<token_t>() as libc::c_ulong);
    (*script).whitespace_p = (*script).script_p;
    (*token).whitespace_p = (*script).script_p;
    if 0 == PS_ReadWhiteSpace(script) { return 0i32 }
    (*script).endwhitespace_p = (*script).script_p;
    (*token).endwhitespace_p = (*script).script_p;
    (*token).line = (*script).line;
    (*token).linescrossed = (*script).line - (*script).lastline;
    if *(*script).script_p as libc::c_int == '\"' as i32 {
        if 0 == PS_ReadString(script, token, '\"' as i32) { return 0i32 }
    } else if *(*script).script_p as libc::c_int == '\'' as i32 {
        if 0 == PS_ReadString(script, token, '\'' as i32) { return 0i32 }
    } else if *(*script).script_p as libc::c_int >= '0' as i32 &&
                  *(*script).script_p as libc::c_int <= '9' as i32 ||
                  *(*script).script_p as libc::c_int == '.' as i32 &&
                      (*(*script).script_p.offset(1isize) as libc::c_int >=
                           '0' as i32 &&
                           *(*script).script_p.offset(1isize) as libc::c_int
                               <= '9' as i32) {
        if 0 == PS_ReadNumber(script, token) { return 0i32 }
    } else if 0 != (*script).flags & 0x10i32 {
        return PS_ReadPrimitive(script, token)
    } else {
        if *(*script).script_p as libc::c_int >= 'a' as i32 &&
               *(*script).script_p as libc::c_int <= 'z' as i32 ||
               *(*script).script_p as libc::c_int >= 'A' as i32 &&
                   *(*script).script_p as libc::c_int <= 'Z' as i32 ||
               *(*script).script_p as libc::c_int == '_' as i32 {
            if 0 == PS_ReadName(script, token) { return 0i32 }
        } else if 0 == PS_ReadPunctuation(script, token) {
            ScriptError(script,
                        b"can\'t read token\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            return 0i32
        }
    }
    memcpy(&mut (*script).token as *mut token_t as *mut libc::c_void,
           token as *const libc::c_void,
           ::std::mem::size_of::<token_t>() as libc::c_ulong);
    return 1i32;
}
//end of the function PS_ReadLiteral
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadPunctuation(mut script: *mut script_t,
                                            mut token: *mut token_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut punc: *mut punctuation_t = 0 as *mut punctuation_t;
    punc =
        *(*script).punctuationtable.offset(*(*script).script_p as libc::c_uint
                                               as isize);
    while !punc.is_null() {
        p = (*punc).p;
        len = strlen(p) as libc::c_int;
        if (*script).script_p.offset(len as isize) <= (*script).end_p {
            if 0 == strncmp((*script).script_p, p, len as libc::c_ulong) {
                Q_strncpyz((*token).string.as_mut_ptr(), p, 1024i32);
                (*script).script_p = (*script).script_p.offset(len as isize);
                (*token).type_0 = 5i32;
                (*token).subtype = (*punc).n;
                return 1i32
            }
        }
        punc = (*punc).next
    }
    return 0i32;
}
//end of the function PS_ReadString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadName(mut script: *mut script_t,
                                     mut token: *mut token_t) -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut c: libc::c_char = 0;
    (*token).type_0 = 4i32;
    loop  {
        let fresh1 = len;
        len = len + 1;
        let fresh0 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[fresh1 as usize] = *fresh0;
        if len >= 1024i32 {
            ScriptError(script,
                        b"name longer than MAX_TOKEN = %d\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        1024i32);
            return 0i32
        }
        c = *(*script).script_p;
        if !(c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
                 ||
                 c as libc::c_int >= 'A' as i32 &&
                     c as libc::c_int <= 'Z' as i32 ||
                 c as libc::c_int >= '0' as i32 &&
                     c as libc::c_int <= '9' as i32 ||
                 c as libc::c_int == '_' as i32) {
            break ;
        }
    }
    (*token).string[len as usize] = '\u{0}' as i32 as libc::c_char;
    (*token).subtype = len;
    return 1i32;
}
//end of the function PS_ReadPunctuation
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadPrimitive(mut script: *mut script_t,
                                          mut token: *mut token_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    len = 0i32;
    while *(*script).script_p as libc::c_int > ' ' as i32 &&
              *(*script).script_p as libc::c_int != ';' as i32 {
        if len >= 1024i32 - 1i32 {
            ScriptError(script,
                        b"primitive token longer than MAX_TOKEN = %d\x00" as
                            *const u8 as *const libc::c_char as
                            *mut libc::c_char, 1024i32);
            return 0i32
        }
        let fresh3 = len;
        len = len + 1;
        let fresh2 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[fresh3 as usize] = *fresh2
    }
    (*token).string[len as usize] = 0i32 as libc::c_char;
    memcpy(&mut (*script).token as *mut token_t as *mut libc::c_void,
           token as *const libc::c_void,
           ::std::mem::size_of::<token_t>() as libc::c_ulong);
    return 1i32;
}
//end else if
//end of the function NumberValue
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadNumber(mut script: *mut script_t,
                                       mut token: *mut token_t)
 -> libc::c_int {
    let mut len: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    let mut octal: libc::c_int = 0;
    let mut dot: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    (*token).type_0 = 3i32;
    if *(*script).script_p as libc::c_int == '0' as i32 &&
           (*(*script).script_p.offset(1isize) as libc::c_int == 'x' as i32 ||
                *(*script).script_p.offset(1isize) as libc::c_int ==
                    'X' as i32) {
        let fresh5 = len;
        len = len + 1;
        let fresh4 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[fresh5 as usize] = *fresh4;
        let fresh7 = len;
        len = len + 1;
        let fresh6 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[fresh7 as usize] = *fresh6;
        c = *(*script).script_p;
        while c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
                  ||
                  c as libc::c_int >= 'a' as i32 &&
                      c as libc::c_int <= 'f' as i32 ||
                  c as libc::c_int >= 'A' as i32 &&
                      c as libc::c_int <= 'A' as i32 {
            let fresh9 = len;
            len = len + 1;
            let fresh8 = (*script).script_p;
            (*script).script_p = (*script).script_p.offset(1);
            (*token).string[fresh9 as usize] = *fresh8;
            if len >= 1024i32 {
                ScriptError(script,
                            b"hexadecimal number longer than MAX_TOKEN = %d\x00"
                                as *const u8 as *const libc::c_char as
                                *mut libc::c_char, 1024i32);
                return 0i32
            }
            c = *(*script).script_p
        }
        (*token).subtype |= 0x100i32
    } else if *(*script).script_p as libc::c_int == '0' as i32 &&
                  (*(*script).script_p.offset(1isize) as libc::c_int ==
                       'b' as i32 ||
                       *(*script).script_p.offset(1isize) as libc::c_int ==
                           'B' as i32) {
        let fresh11 = len;
        len = len + 1;
        let fresh10 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[fresh11 as usize] = *fresh10;
        let fresh13 = len;
        len = len + 1;
        let fresh12 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[fresh13 as usize] = *fresh12;
        c = *(*script).script_p;
        while c as libc::c_int == '0' as i32 || c as libc::c_int == '1' as i32
              {
            let fresh15 = len;
            len = len + 1;
            let fresh14 = (*script).script_p;
            (*script).script_p = (*script).script_p.offset(1);
            (*token).string[fresh15 as usize] = *fresh14;
            if len >= 1024i32 {
                ScriptError(script,
                            b"binary number longer than MAX_TOKEN = %d\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, 1024i32);
                return 0i32
            }
            c = *(*script).script_p
        }
        (*token).subtype |= 0x400i32
    } else {
        octal = qfalse as libc::c_int;
        dot = qfalse as libc::c_int;
        if *(*script).script_p as libc::c_int == '0' as i32 {
            octal = qtrue as libc::c_int
        }
        loop  {
            c = *(*script).script_p;
            if c as libc::c_int == '.' as i32 {
                dot = qtrue as libc::c_int
            } else if c as libc::c_int == '8' as i32 ||
                          c as libc::c_int == '9' as i32 {
                octal = qfalse as libc::c_int
            } else if (c as libc::c_int) < '0' as i32 ||
                          c as libc::c_int > '9' as i32 {
                break ;
            }
            let fresh17 = len;
            len = len + 1;
            let fresh16 = (*script).script_p;
            (*script).script_p = (*script).script_p.offset(1);
            (*token).string[fresh17 as usize] = *fresh16;
            if len >= 1024i32 - 1i32 {
                ScriptError(script,
                            b"number longer than MAX_TOKEN = %d\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, 1024i32);
                return 0i32
            }
        }
        if 0 != octal {
            (*token).subtype |= 0x200i32
        } else { (*token).subtype |= 0x8i32 }
        if 0 != dot { (*token).subtype |= 0x800i32 }
    }
    i = 0i32;
    while i < 2i32 {
        c = *(*script).script_p;
        if (c as libc::c_int == 'l' as i32 || c as libc::c_int == 'L' as i32)
               && 0 == (*token).subtype & 0x2000i32 {
            (*script).script_p = (*script).script_p.offset(1isize);
            (*token).subtype |= 0x2000i32
        } else if (c as libc::c_int == 'u' as i32 ||
                       c as libc::c_int == 'U' as i32) &&
                      0 == (*token).subtype & (0x4000i32 | 0x800i32) {
            (*script).script_p = (*script).script_p.offset(1isize);
            (*token).subtype |= 0x4000i32
        }
        i += 1
    }
    (*token).string[len as usize] = '\u{0}' as i32 as libc::c_char;
    NumberValue((*token).string.as_mut_ptr(), (*token).subtype,
                &mut (*token).intvalue, &mut (*token).floatvalue);
    if 0 == (*token).subtype & 0x800i32 { (*token).subtype |= 0x1000i32 }
    return 1i32;
}
//end of the function PS_ReadName
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn NumberValue(mut string: *mut libc::c_char,
                                     mut subtype: libc::c_int,
                                     mut intvalue: *mut libc::c_ulong,
                                     mut floatvalue: *mut libc::c_float) {
    let mut dotfound: libc::c_ulong = 0i32 as libc::c_ulong;
    *intvalue = 0i32 as libc::c_ulong;
    *floatvalue = 0i32 as libc::c_float;
    if 0 != subtype & 0x800i32 {
        while 0 != *string {
            if *string as libc::c_int == '.' as i32 {
                if 0 != dotfound { return }
                dotfound = 10i32 as libc::c_ulong;
                string = string.offset(1isize)
            }
            if 0 != dotfound {
                *floatvalue =
                    *floatvalue +
                        (*string as libc::c_int - '0' as i32) as libc::c_float
                            / dotfound as libc::c_float;
                dotfound = dotfound.wrapping_mul(10i32 as libc::c_ulong)
            } else {
                *floatvalue =
                    (*floatvalue as libc::c_double * 10.0f64 +
                         (*string as libc::c_int - '0' as i32) as
                             libc::c_float as libc::c_double) as libc::c_float
            }
            string = string.offset(1isize)
        }
        *intvalue = *floatvalue as libc::c_ulong
    } else if 0 != subtype & 0x8i32 {
        while 0 != *string {
            let fresh18 = string;
            string = string.offset(1);
            *intvalue =
                (*intvalue).wrapping_mul(10i32 as
                                             libc::c_ulong).wrapping_add((*fresh18
                                                                              as
                                                                              libc::c_int
                                                                              -
                                                                              '0'
                                                                                  as
                                                                                  i32)
                                                                             as
                                                                             libc::c_ulong)
        }
        *floatvalue = *intvalue as libc::c_float
    } else if 0 != subtype & 0x100i32 {
        string = string.offset(2isize);
        while 0 != *string {
            *intvalue <<= 4i32;
            if *string as libc::c_int >= 'a' as i32 &&
                   *string as libc::c_int <= 'f' as i32 {
                *intvalue =
                    (*intvalue).wrapping_add((*string as libc::c_int -
                                                  'a' as i32 + 10i32) as
                                                 libc::c_ulong)
            } else if *string as libc::c_int >= 'A' as i32 &&
                          *string as libc::c_int <= 'F' as i32 {
                *intvalue =
                    (*intvalue).wrapping_add((*string as libc::c_int -
                                                  'A' as i32 + 10i32) as
                                                 libc::c_ulong)
            } else {
                *intvalue =
                    (*intvalue).wrapping_add((*string as libc::c_int -
                                                  '0' as i32) as
                                                 libc::c_ulong)
            }
            string = string.offset(1isize)
        }
        *floatvalue = *intvalue as libc::c_float
    } else if 0 != subtype & 0x200i32 {
        string = string.offset(1isize);
        while 0 != *string {
            let fresh19 = string;
            string = string.offset(1);
            *intvalue =
                (*intvalue <<
                     3i32).wrapping_add((*fresh19 as libc::c_int - '0' as i32)
                                            as libc::c_ulong)
        }
        *floatvalue = *intvalue as libc::c_float
    } else if 0 != subtype & 0x400i32 {
        string = string.offset(2isize);
        while 0 != *string {
            let fresh20 = string;
            string = string.offset(1);
            *intvalue =
                (*intvalue <<
                     1i32).wrapping_add((*fresh20 as libc::c_int - '0' as i32)
                                            as libc::c_ulong)
        }
        *floatvalue = *intvalue as libc::c_float
    };
}
//end of the function PS_ReadEscapeCharacter
//============================================================================
// Reads C-like string. Escape characters are interpretted.
// Quotes are included with the string.
// Reads two strings with a white space between them as one string.
//
// Parameter:				script		: script to read from
//								token			: buffer to store the string
// Returns:					qtrue when a string was read successfully
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadString(mut script: *mut script_t,
                                       mut token: *mut token_t,
                                       mut quote: libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut tmpline: libc::c_int = 0;
    let mut tmpscript_p: *mut libc::c_char = 0 as *mut libc::c_char;
    if quote == '\"' as i32 {
        (*token).type_0 = 1i32
    } else { (*token).type_0 = 2i32 }
    len = 0i32;
    let fresh22 = len;
    len = len + 1;
    let fresh21 = (*script).script_p;
    (*script).script_p = (*script).script_p.offset(1);
    (*token).string[fresh22 as usize] = *fresh21;
    loop  {
        if len >= 1024i32 - 2i32 {
            ScriptError(script,
                        b"string longer than MAX_TOKEN = %d\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        1024i32);
            return 0i32
        }
        //end if
        //if there is an escape character and
		//if escape characters inside a string are allowed
        if *(*script).script_p as libc::c_int == '\\' as i32 &&
               0 == (*script).flags & 0x8i32 {
            if 0 ==
                   PS_ReadEscapeCharacter(script,
                                          &mut *(*token).string.as_mut_ptr().offset(len
                                                                                        as
                                                                                        isize))
               {
                (*token).string[len as usize] = 0i32 as libc::c_char;
                return 0i32
            }
            len += 1
        } else if *(*script).script_p as libc::c_int == quote {
            (*script).script_p = (*script).script_p.offset(1isize);
            //if white spaces in a string are not allowed
            if 0 != (*script).flags & 0x4i32 { break ; }
            tmpscript_p = (*script).script_p;
            tmpline = (*script).line;
            //read unusefull stuff between possible two following strings
            if 0 == PS_ReadWhiteSpace(script) {
                (*script).script_p = tmpscript_p;
                (*script).line = tmpline;
                break ;
            } else if *(*script).script_p as libc::c_int != quote {
                (*script).script_p = tmpscript_p;
                (*script).line = tmpline;
                break ;
            } else { (*script).script_p = (*script).script_p.offset(1isize) }
        } else {
            if *(*script).script_p as libc::c_int == '\u{0}' as i32 {
                (*token).string[len as usize] = 0i32 as libc::c_char;
                ScriptError(script,
                            b"missing trailing quote\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
                return 0i32
            }
            if *(*script).script_p as libc::c_int == '\n' as i32 {
                (*token).string[len as usize] = 0i32 as libc::c_char;
                ScriptError(script,
                            b"newline inside string %s\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            (*token).string.as_mut_ptr());
                return 0i32
            }
            let fresh24 = len;
            len = len + 1;
            let fresh23 = (*script).script_p;
            (*script).script_p = (*script).script_p.offset(1);
            (*token).string[fresh24 as usize] = *fresh23
        }
    }
    let fresh25 = len;
    len = len + 1;
    (*token).string[fresh25 as usize] = quote as libc::c_char;
    (*token).string[len as usize] = '\u{0}' as i32 as libc::c_char;
    (*token).subtype = len;
    return 1i32;
}
//end of the function SetScriptPunctuations
//============================================================================
// Reads spaces, tabs, C-like comments etc.
// When a newline character is found the scripts line counter is increased.
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadWhiteSpace(mut script: *mut script_t)
 -> libc::c_int {
    loop  {
        while *(*script).script_p as libc::c_int <= ' ' as i32 {
            if 0 == *(*script).script_p { return 0i32 }
            if *(*script).script_p as libc::c_int == '\n' as i32 {
                (*script).line += 1
            }
            (*script).script_p = (*script).script_p.offset(1isize)
        }
        //end while
        //skip comments
        if *(*script).script_p as libc::c_int == '/' as i32 {
            //comments //
            if *(*script).script_p.offset(1isize) as libc::c_int == '/' as i32
               {
                (*script).script_p = (*script).script_p.offset(1isize);
                loop  {
                    (*script).script_p = (*script).script_p.offset(1isize);
                    if 0 == *(*script).script_p { return 0i32 }
                    if !(*(*script).script_p as libc::c_int != '\n' as i32) {
                        break ;
                    }
                }
                (*script).line += 1;
                (*script).script_p = (*script).script_p.offset(1isize);
                if 0 == *(*script).script_p { return 0i32 }
                continue ;
            } else if *(*script).script_p.offset(1isize) as libc::c_int ==
                          '*' as i32 {
                (*script).script_p = (*script).script_p.offset(1isize);
                loop  {
                    (*script).script_p = (*script).script_p.offset(1isize);
                    if 0 == *(*script).script_p { return 0i32 }
                    if *(*script).script_p as libc::c_int == '\n' as i32 {
                        (*script).line += 1
                    }
                    if *(*script).script_p as libc::c_int == '*' as i32 &&
                           *(*script).script_p.offset(1isize) as libc::c_int
                               == '/' as i32 {
                        break ;
                    }
                }
                (*script).script_p = (*script).script_p.offset(1isize);
                if 0 == *(*script).script_p { return 0i32 }
                (*script).script_p = (*script).script_p.offset(1isize);
                if 0 == *(*script).script_p { return 0i32 }
                continue ;
            }
        }
        //end if
        //end if
        break ;
    }
    return 1i32;
}
//end of the function PS_ReadWhiteSpace
//============================================================================
// Reads an escape character.
//
// Parameter:				script		: script to read from
//								ch				: place to store the read escape character
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadEscapeCharacter(mut script: *mut script_t,
                                                mut ch: *mut libc::c_char)
 -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    (*script).script_p = (*script).script_p.offset(1isize);
    match *(*script).script_p as libc::c_int {
        92 => { c = '\\' as i32 }
        110 => { c = '\n' as i32 }
        114 => { c = '\r' as i32 }
        116 => { c = '\t' as i32 }
        118 => { c = '\u{b}' as i32 }
        98 => { c = '\u{8}' as i32 }
        102 => { c = '\u{c}' as i32 }
        97 => { c = '\u{7}' as i32 }
        39 => { c = '\'' as i32 }
        34 => { c = '\"' as i32 }
        63 => { c = '?' as i32 }
        120 => {
            (*script).script_p = (*script).script_p.offset(1isize);
            i = 0i32;
            val = 0i32;
            loop  {
                c = *(*script).script_p as libc::c_int;
                if c >= '0' as i32 && c <= '9' as i32 {
                    c = c - '0' as i32
                } else if c >= 'A' as i32 && c <= 'Z' as i32 {
                    c = c - 'A' as i32 + 10i32
                } else {
                    if !(c >= 'a' as i32 && c <= 'z' as i32) { break ; }
                    c = c - 'a' as i32 + 10i32
                }
                val = (val << 4i32) + c;
                i += 1;
                (*script).script_p = (*script).script_p.offset(1isize)
            }
            (*script).script_p = (*script).script_p.offset(-1isize);
            if val > 0xffi32 {
                ScriptWarning(script,
                              b"too large value in escape character\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
                val = 0xffi32
            }
            c = val
        }
        _ => {
            if (*(*script).script_p as libc::c_int) < '0' as i32 ||
                   *(*script).script_p as libc::c_int > '9' as i32 {
                ScriptError(script,
                            b"unknown escape char\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
            }
            i = 0i32;
            val = 0i32;
            loop  {
                c = *(*script).script_p as libc::c_int;
                if !(c >= '0' as i32 && c <= '9' as i32) { break ; }
                c = c - '0' as i32;
                val = val * 10i32 + c;
                i += 1;
                (*script).script_p = (*script).script_p.offset(1isize)
            }
            (*script).script_p = (*script).script_p.offset(-1isize);
            if val > 0xffi32 {
                ScriptWarning(script,
                              b"too large value in escape character\x00" as
                                  *const u8 as *const libc::c_char as
                                  *mut libc::c_char);
                val = 0xffi32
            }
            c = val
        }
    }
    (*script).script_p = (*script).script_p.offset(1isize);
    *ch = c as libc::c_char;
    return 1i32;
}
//expect a certain token
#[no_mangle]
pub unsafe extern "C" fn PS_ExpectTokenString(mut script: *mut script_t,
                                              mut string: *mut libc::c_char)
 -> libc::c_int {
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    if 0 == PS_ReadToken(script, &mut token) {
        ScriptError(script,
                    b"couldn\'t find expected %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char, string);
        return 0i32
    }
    if 0 != strcmp(token.string.as_mut_ptr(), string) {
        ScriptError(script,
                    b"expected %s, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char, string,
                    token.string.as_mut_ptr());
        return 0i32
    }
    return 1i32;
}
//expect a certain token type
#[no_mangle]
pub unsafe extern "C" fn PS_ExpectTokenType(mut script: *mut script_t,
                                            mut type_0: libc::c_int,
                                            mut subtype: libc::c_int,
                                            mut token: *mut token_t)
 -> libc::c_int {
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if 0 == PS_ReadToken(script, token) {
        ScriptError(script,
                    b"couldn\'t read expected token\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0i32
    }
    if (*token).type_0 != type_0 {
        strcpy(str.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char);
        if type_0 == 1i32 {
            strcpy(str.as_mut_ptr(),
                   b"string\x00" as *const u8 as *const libc::c_char);
        }
        if type_0 == 2i32 {
            strcpy(str.as_mut_ptr(),
                   b"literal\x00" as *const u8 as *const libc::c_char);
        }
        if type_0 == 3i32 {
            strcpy(str.as_mut_ptr(),
                   b"number\x00" as *const u8 as *const libc::c_char);
        }
        if type_0 == 4i32 {
            strcpy(str.as_mut_ptr(),
                   b"name\x00" as *const u8 as *const libc::c_char);
        }
        if type_0 == 5i32 {
            strcpy(str.as_mut_ptr(),
                   b"punctuation\x00" as *const u8 as *const libc::c_char);
        }
        ScriptError(script,
                    b"expected a %s, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    str.as_mut_ptr(), (*token).string.as_mut_ptr());
        return 0i32
    }
    if (*token).type_0 == 3i32 {
        if (*token).subtype & subtype != subtype {
            strcpy(str.as_mut_ptr(),
                   b"\x00" as *const u8 as *const libc::c_char);
            if 0 != subtype & 0x8i32 {
                strcpy(str.as_mut_ptr(),
                       b"decimal\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != subtype & 0x100i32 {
                strcpy(str.as_mut_ptr(),
                       b"hex\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != subtype & 0x200i32 {
                strcpy(str.as_mut_ptr(),
                       b"octal\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != subtype & 0x400i32 {
                strcpy(str.as_mut_ptr(),
                       b"binary\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != subtype & 0x2000i32 {
                strcat(str.as_mut_ptr(),
                       b" long\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != subtype & 0x4000i32 {
                strcat(str.as_mut_ptr(),
                       b" unsigned\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != subtype & 0x800i32 {
                strcat(str.as_mut_ptr(),
                       b" float\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != subtype & 0x1000i32 {
                strcat(str.as_mut_ptr(),
                       b" integer\x00" as *const u8 as *const libc::c_char);
            }
            ScriptError(script,
                        b"expected %s, found %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        str.as_mut_ptr(), (*token).string.as_mut_ptr());
            return 0i32
        }
    } else if (*token).type_0 == 5i32 {
        if subtype < 0i32 {
            ScriptError(script,
                        b"BUG: wrong punctuation subtype\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            return 0i32
        }
        if (*token).subtype != subtype {
            ScriptError(script,
                        b"expected %s, found %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        (*(*script).punctuations.offset(subtype as isize)).p,
                        (*token).string.as_mut_ptr());
            return 0i32
        }
    }
    return 1i32;
}
//expect a token
#[no_mangle]
pub unsafe extern "C" fn PS_ExpectAnyToken(mut script: *mut script_t,
                                           mut token: *mut token_t)
 -> libc::c_int {
    if 0 == PS_ReadToken(script, token) {
        ScriptError(script,
                    b"couldn\'t read expected token\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0i32
    } else { return 1i32 };
}
//returns true when the token is available
#[no_mangle]
pub unsafe extern "C" fn PS_CheckTokenString(mut script: *mut script_t,
                                             mut string: *mut libc::c_char)
 -> libc::c_int {
    let mut tok: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    if 0 == PS_ReadToken(script, &mut tok) { return 0i32 }
    if 0 == strcmp(tok.string.as_mut_ptr(), string) { return 1i32 }
    (*script).script_p = (*script).lastscript_p;
    return 0i32;
}
//returns true and reads the token when a token with the given type is available
#[no_mangle]
pub unsafe extern "C" fn PS_CheckTokenType(mut script: *mut script_t,
                                           mut type_0: libc::c_int,
                                           mut subtype: libc::c_int,
                                           mut token: *mut token_t)
 -> libc::c_int {
    let mut tok: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    if 0 == PS_ReadToken(script, &mut tok) { return 0i32 }
    if tok.type_0 == type_0 && tok.subtype & subtype == subtype {
        memcpy(token as *mut libc::c_void,
               &mut tok as *mut token_t as *const libc::c_void,
               ::std::mem::size_of::<token_t>() as libc::c_ulong);
        return 1i32
    }
    (*script).script_p = (*script).lastscript_p;
    return 0i32;
}
//skip tokens until the given token string is read
#[no_mangle]
pub unsafe extern "C" fn PS_SkipUntilString(mut script: *mut script_t,
                                            mut string: *mut libc::c_char)
 -> libc::c_int {
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    while 0 != PS_ReadToken(script, &mut token) {
        if 0 == strcmp(token.string.as_mut_ptr(), string) { return 1i32 }
    }
    return 0i32;
}
//unread the last token read from the script
#[no_mangle]
pub unsafe extern "C" fn PS_UnreadLastToken(mut script: *mut script_t) {
    (*script).tokenavailable = 1i32;
}
//unread the given token
#[no_mangle]
pub unsafe extern "C" fn PS_UnreadToken(mut script: *mut script_t,
                                        mut token: *mut token_t) {
    memcpy(&mut (*script).token as *mut token_t as *mut libc::c_void,
           token as *const libc::c_void,
           ::std::mem::size_of::<token_t>() as libc::c_ulong);
    (*script).tokenavailable = 1i32;
}
//returns the next character of the read white space, returns NULL if none
#[no_mangle]
pub unsafe extern "C" fn PS_NextWhiteSpaceChar(mut script: *mut script_t)
 -> libc::c_char {
    if (*script).whitespace_p != (*script).endwhitespace_p {
        let fresh26 = (*script).whitespace_p;
        (*script).whitespace_p = (*script).whitespace_p.offset(1);
        return *fresh26
    } else { return 0i32 as libc::c_char };
}
//remove any leading and trailing double quotes from the token
#[no_mangle]
pub unsafe extern "C" fn StripDoubleQuotes(mut string: *mut libc::c_char) {
    if *string as libc::c_int == '\"' as i32 {
        memmove(string as *mut libc::c_void,
                string.offset(1isize) as *const libc::c_void, strlen(string));
    }
    if *string.offset(strlen(string).wrapping_sub(1i32 as libc::c_ulong) as
                          isize) as libc::c_int == '\"' as i32 {
        *string.offset(strlen(string).wrapping_sub(1i32 as libc::c_ulong) as
                           isize) = '\u{0}' as i32 as libc::c_char
    };
}
//remove any leading and trailing single quotes from the token
#[no_mangle]
pub unsafe extern "C" fn StripSingleQuotes(mut string: *mut libc::c_char) {
    if *string as libc::c_int == '\'' as i32 {
        memmove(string as *mut libc::c_void,
                string.offset(1isize) as *const libc::c_void, strlen(string));
    }
    if *string.offset(strlen(string).wrapping_sub(1i32 as libc::c_ulong) as
                          isize) as libc::c_int == '\'' as i32 {
        *string.offset(strlen(string).wrapping_sub(1i32 as libc::c_ulong) as
                           isize) = '\u{0}' as i32 as libc::c_char
    };
}
//read a possible signed integer
#[no_mangle]
pub unsafe extern "C" fn ReadSignedInt(mut script: *mut script_t)
 -> libc::c_long {
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut sign: libc::c_long = 1i32 as libc::c_long;
    PS_ExpectAnyToken(script, &mut token);
    if 0 ==
           strcmp(token.string.as_mut_ptr(),
                  b"-\x00" as *const u8 as *const libc::c_char) {
        if 0 == PS_ExpectAnyToken(script, &mut token) {
            ScriptError(script,
                        b"Missing integer value\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            return 0i32 as libc::c_long
        }
        sign = -1i32 as libc::c_long
    }
    if token.type_0 != 3i32 || token.subtype == 0x800i32 {
        ScriptError(script,
                    b"expected integer value, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr());
        return 0i32 as libc::c_long
    }
    return (sign as libc::c_ulong).wrapping_mul(token.intvalue) as
               libc::c_long;
}
//read a possible signed floating point number
#[no_mangle]
pub unsafe extern "C" fn ReadSignedFloat(mut script: *mut script_t)
 -> libc::c_float {
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut sign: libc::c_float = 1.0f64 as libc::c_float;
    PS_ExpectAnyToken(script, &mut token);
    if 0 ==
           strcmp(token.string.as_mut_ptr(),
                  b"-\x00" as *const u8 as *const libc::c_char) {
        if 0 == PS_ExpectAnyToken(script, &mut token) {
            ScriptError(script,
                        b"Missing float value\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            return 0i32 as libc::c_float
        }
        sign = -1.0f64 as libc::c_float
    }
    if token.type_0 != 3i32 {
        ScriptError(script,
                    b"expected float value, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr());
        return 0i32 as libc::c_float
    }
    return sign * token.floatvalue;
}
//set an array with punctuations, NULL restores default C/C++ set
#[no_mangle]
pub unsafe extern "C" fn SetScriptPunctuations(mut script: *mut script_t,
                                               mut p: *mut punctuation_t) {
    if !p.is_null() {
        PS_CreatePunctuationTable(script, p);
    } else {
        PS_CreatePunctuationTable(script, default_punctuations.as_mut_ptr());
    }
    if !p.is_null() {
        (*script).punctuations = p
    } else { (*script).punctuations = default_punctuations.as_mut_ptr() };
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
/* ****************************************************************************
 * name:		l_script.c
 *
 * desc:		lexicographical parser
 *
 * $Archive: /MissionPack/code/botlib/l_script.c $
 *
 *****************************************************************************/
//#define SCREWUP
//#define BOTLIB
//#define MEQCC
//#define BSPC
//SCREWUP
//include files for usage in the bot library
//BOTLIB
//MEQCC
//BSPC
//longer punctuations first
#[no_mangle]
pub static mut default_punctuations: [punctuation_t; 53] =
    [punctuation_s{p:
                       b">>=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 1i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"<<=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 2i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"...\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 3i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"##\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 4i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"&&\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 5i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"||\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 6i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b">=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 7i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"<=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 8i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"==\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 9i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"!=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 10i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"*=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 11i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"/=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 12i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"%=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 13i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"+=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 14i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"-=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 15i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"++\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 16i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"--\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 17i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"&=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 18i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"|=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 19i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"^=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 20i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b">>\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 21i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"<<\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 22i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"->\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 23i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"::\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 24i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b".*\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 25i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"*\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 26i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"/\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 27i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"%\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 28i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"+\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 29i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"-\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 30i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"=\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 31i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"&\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 32i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"|\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 33i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"^\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 34i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"~\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 35i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"!\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 36i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b">\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 37i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"<\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 38i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b".\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 39i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b",\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 40i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b";\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 41i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b":\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 42i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"?\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 43i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"(\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 44i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b")\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 45i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"{\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 46i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"}\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 47i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"[\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 48i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"]\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 49i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"\\\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 50i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"#\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 51i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p:
                       b"$\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char,
                   n: 52i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,},
     punctuation_s{p: 0 as *const libc::c_char as *mut libc::c_char,
                   n: 0i32,
                   next: 0 as *const punctuation_s as *mut punctuation_s,}];
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_CreatePunctuationTable(mut script: *mut script_t,
                                                   mut punctuations:
                                                       *mut punctuation_t) {
    let mut i: libc::c_int = 0;
    let mut p: *mut punctuation_t = 0 as *mut punctuation_t;
    let mut lastp: *mut punctuation_t = 0 as *mut punctuation_t;
    let mut newp: *mut punctuation_t = 0 as *mut punctuation_t;
    if (*script).punctuationtable.is_null() {
        (*script).punctuationtable =
            GetMemory((256i32 as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut punctuation_t>()
                                                           as libc::c_ulong))
                as *mut *mut punctuation_t
    }
    memset((*script).punctuationtable as *mut libc::c_void, 0i32,
           (256i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut punctuation_t>()
                                                as libc::c_ulong));
    i = 0i32;
    while !(*punctuations.offset(i as isize)).p.is_null() {
        newp = &mut *punctuations.offset(i as isize) as *mut punctuation_t;
        lastp = 0 as *mut punctuation_t;
        p =
            *(*script).punctuationtable.offset(*(*newp).p.offset(0isize) as
                                                   libc::c_uint as isize);
        while !p.is_null() {
            if strlen((*p).p) < strlen((*newp).p) {
                (*newp).next = p;
                if !lastp.is_null() {
                    (*lastp).next = newp
                } else {
                    let ref mut fresh27 =
                        *(*script).punctuationtable.offset(*(*newp).p.offset(0isize)
                                                               as libc::c_uint
                                                               as isize);
                    *fresh27 = newp
                }
                break ;
            } else { lastp = p; p = (*p).next }
        }
        if p.is_null() {
            (*newp).next = 0 as *mut punctuation_s;
            if !lastp.is_null() {
                (*lastp).next = newp
            } else {
                let ref mut fresh28 =
                    *(*script).punctuationtable.offset(*(*newp).p.offset(0isize)
                                                           as libc::c_uint as
                                                           isize);
                *fresh28 = newp
            }
        }
        i += 1
    };
}
//set script flags
#[no_mangle]
pub unsafe extern "C" fn SetScriptFlags(mut script: *mut script_t,
                                        mut flags: libc::c_int) {
    (*script).flags = flags;
}
//get script flags
#[no_mangle]
pub unsafe extern "C" fn GetScriptFlags(mut script: *mut script_t)
 -> libc::c_int {
    return (*script).flags;
}
//reset a script
#[no_mangle]
pub unsafe extern "C" fn ResetScript(mut script: *mut script_t) {
    (*script).script_p = (*script).buffer;
    (*script).lastscript_p = (*script).buffer;
    (*script).whitespace_p = 0 as *mut libc::c_char;
    (*script).endwhitespace_p = 0 as *mut libc::c_char;
    (*script).tokenavailable = 0i32;
    (*script).line = 1i32;
    (*script).lastline = 1i32;
    memset(&mut (*script).token as *mut token_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<token_t>() as libc::c_ulong);
}
//returns true if at the end of the script
#[no_mangle]
pub unsafe extern "C" fn EndOfScript(mut script: *mut script_t)
 -> libc::c_int {
    return ((*script).script_p >= (*script).end_p) as libc::c_int;
}
//returns a pointer to the punctuation with the given number
#[no_mangle]
pub unsafe extern "C" fn PunctuationFromNum(mut script: *mut script_t,
                                            mut num: libc::c_int)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !(*(*script).punctuations.offset(i as isize)).p.is_null() {
        if (*(*script).punctuations.offset(i as isize)).n == num {
            return (*(*script).punctuations.offset(i as isize)).p
        }
        i += 1
    }
    return b"unknown punctuation\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char;
}
//load a script from the given file at the given offset with the given length
#[no_mangle]
pub unsafe extern "C" fn LoadScriptFile(mut filename: *const libc::c_char)
 -> *mut script_t {
    let mut fp: fileHandle_t = 0;
    let mut pathname: [libc::c_char; 64] = [0; 64];
    let mut length: libc::c_int = 0;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut script: *mut script_t = 0 as *mut script_t;
    if 0 != strlen(basefolder.as_mut_ptr()) {
        Com_sprintf(pathname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as
                        libc::c_ulong as libc::c_int,
                    b"%s/%s\x00" as *const u8 as *const libc::c_char,
                    basefolder.as_mut_ptr(), filename);
    } else {
        Com_sprintf(pathname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as
                        libc::c_ulong as libc::c_int,
                    b"%s\x00" as *const u8 as *const libc::c_char, filename);
    }
    length =
        botimport.FS_FOpenFile.expect("non-null function pointer")(pathname.as_mut_ptr(),
                                                                   &mut fp,
                                                                   FS_READ);
    if 0 == fp { return 0 as *mut script_t }
    buffer =
        GetClearedMemory((::std::mem::size_of::<script_t>() as
                              libc::c_ulong).wrapping_add(length as
                                                              libc::c_ulong).wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_ulong));
    script = buffer as *mut script_t;
    memset(script as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<script_t>() as libc::c_ulong);
    Q_strncpyz((*script).filename.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    (*script).buffer =
        (buffer as
             *mut libc::c_char).offset(::std::mem::size_of::<script_t>() as
                                           libc::c_ulong as isize);
    *(*script).buffer.offset(length as isize) = 0i32 as libc::c_char;
    (*script).length = length;
    (*script).script_p = (*script).buffer;
    (*script).lastscript_p = (*script).buffer;
    (*script).end_p =
        &mut *(*script).buffer.offset(length as isize) as *mut libc::c_char;
    (*script).tokenavailable = 0i32;
    (*script).line = 1i32;
    (*script).lastline = 1i32;
    SetScriptPunctuations(script, 0 as *mut punctuation_t);
    botimport.FS_Read.expect("non-null function pointer")((*script).buffer as
                                                              *mut libc::c_void,
                                                          length, fp);
    botimport.FS_FCloseFile.expect("non-null function pointer")(fp);
    return script;
}
//binary operators
//
//define merge operator
//logic operators
//arithmatic operators
//binary operators
//reference operators
//C++
//arithmatic operators
//binary operators
//logic operators
//reference operator
//seperators
//label indication
//if statement
//embracements
//
//precompiler operator
//DOLLAR
#[no_mangle]
pub static mut basefolder: [libc::c_char; 64] = [0; 64];
//load a script from the given memory with the given length
#[no_mangle]
pub unsafe extern "C" fn LoadScriptMemory(mut ptr: *mut libc::c_char,
                                          mut length: libc::c_int,
                                          mut name: *mut libc::c_char)
 -> *mut script_t {
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut script: *mut script_t = 0 as *mut script_t;
    buffer =
        GetClearedMemory((::std::mem::size_of::<script_t>() as
                              libc::c_ulong).wrapping_add(length as
                                                              libc::c_ulong).wrapping_add(1i32
                                                                                              as
                                                                                              libc::c_ulong));
    script = buffer as *mut script_t;
    memset(script as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<script_t>() as libc::c_ulong);
    Q_strncpyz((*script).filename.as_mut_ptr(), name,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    (*script).buffer =
        (buffer as
             *mut libc::c_char).offset(::std::mem::size_of::<script_t>() as
                                           libc::c_ulong as isize);
    *(*script).buffer.offset(length as isize) = 0i32 as libc::c_char;
    (*script).length = length;
    (*script).script_p = (*script).buffer;
    (*script).lastscript_p = (*script).buffer;
    (*script).end_p =
        &mut *(*script).buffer.offset(length as isize) as *mut libc::c_char;
    (*script).tokenavailable = 0i32;
    (*script).line = 1i32;
    (*script).lastline = 1i32;
    SetScriptPunctuations(script, 0 as *mut punctuation_t);
    memcpy((*script).buffer as *mut libc::c_void, ptr as *const libc::c_void,
           length as libc::c_ulong);
    return script;
}
//free a script
#[no_mangle]
pub unsafe extern "C" fn FreeScript(mut script: *mut script_t) {
    if !(*script).punctuationtable.is_null() {
        FreeMemory((*script).punctuationtable as *mut libc::c_void);
    }
    FreeMemory(script as *mut libc::c_void);
}
//set the base folder to load files from
#[no_mangle]
pub unsafe extern "C" fn PS_SetBaseFolder(mut path: *mut libc::c_char) {
    Com_sprintf(basefolder.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"%s\x00" as *const u8 as *const libc::c_char, path);
}
//end of the function PS_ReadNumber
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PS_ReadLiteral(mut script: *mut script_t,
                                        mut token: *mut token_t)
 -> libc::c_int {
    (*token).type_0 = 2i32;
    let fresh29 = (*script).script_p;
    (*script).script_p = (*script).script_p.offset(1);
    (*token).string[0usize] = *fresh29;
    if 0 == *(*script).script_p {
        ScriptError(script,
                    b"end of file before trailing \'\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return 0i32
    }
    if *(*script).script_p as libc::c_int == '\\' as i32 {
        if 0 ==
               PS_ReadEscapeCharacter(script,
                                      &mut *(*token).string.as_mut_ptr().offset(1isize))
           {
            return 0i32
        }
    } else {
        let fresh30 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[1usize] = *fresh30
    }
    if *(*script).script_p as libc::c_int != '\'' as i32 {
        ScriptWarning(script,
                      b"too many characters in literal, ignored\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char);
        while 0 != *(*script).script_p as libc::c_int &&
                  *(*script).script_p as libc::c_int != '\'' as i32 &&
                  *(*script).script_p as libc::c_int != '\n' as i32 {
            (*script).script_p = (*script).script_p.offset(1isize)
        }
        if *(*script).script_p as libc::c_int == '\'' as i32 {
            (*script).script_p = (*script).script_p.offset(1isize)
        }
    }
    let fresh31 = (*script).script_p;
    (*script).script_p = (*script).script_p.offset(1);
    (*token).string[2usize] = *fresh31;
    (*token).string[3usize] = '\u{0}' as i32 as libc::c_char;
    (*token).subtype = (*token).string[1usize] as libc::c_int;
    return 1i32;
}
//end of the function EndOfScript
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn NumLinesCrossed(mut script: *mut script_t)
 -> libc::c_int {
    return (*script).line - (*script).lastline;
}
//end of the function NumLinesCrossed
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn ScriptSkipTo(mut script: *mut script_t,
                                      mut value: *mut libc::c_char)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut firstchar: libc::c_char = 0;
    firstchar = *value;
    len = strlen(value) as libc::c_int;
    loop  {
        if 0 == PS_ReadWhiteSpace(script) { return 0i32 }
        if *(*script).script_p as libc::c_int == firstchar as libc::c_int {
            if 0 == strncmp((*script).script_p, value, len as libc::c_ulong) {
                return 1i32
            }
        }
        (*script).script_p = (*script).script_p.offset(1isize)
    };
}