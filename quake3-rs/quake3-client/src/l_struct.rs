#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(custom_attribute, extern_types, libc)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
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
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
    }
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
    extern "C" {
        //remove any leading and trailing double quotes from the token
        #[no_mangle]
        pub fn StripDoubleQuotes(string: *mut libc::c_char);
        //remove any leading and trailing single quotes from the token
        #[no_mangle]
        pub fn StripSingleQuotes(string: *mut libc::c_char);
    }
}
#[header_src =
      "ioq3/code/botlib/l_precomp.h"]
pub mod l_precomp_h {
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
 * name:		l_precomp.h
 *
 * desc:		pre compiler
 *
 * $Archive: /source/code/botlib/l_precomp.h $
 *
 *****************************************************************************/
    //macro definitions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct define_s {
        pub name: *mut libc::c_char,
        pub flags: libc::c_int,
        pub builtin: libc::c_int,
        pub numparms: libc::c_int,
        pub parms: *mut token_t,
        pub tokens: *mut token_t,
        pub next: *mut define_s,
        pub hashnext: *mut define_s,
    }
    pub type define_t = define_s;
    //indents
//used for conditional compilation directives:
//#if, #else, #elif, #ifdef, #ifndef
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct indent_s {
        pub type_0: libc::c_int,
        pub skip: libc::c_int,
        pub script: *mut script_t,
        pub next: *mut indent_s,
    }
    pub type indent_t = indent_s;
    //source file
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct source_s {
        pub filename: [libc::c_char; 1024],
        pub includepath: [libc::c_char; 1024],
        pub punctuations: *mut punctuation_t,
        pub scriptstack: *mut script_t,
        pub tokens: *mut token_t,
        pub defines: *mut define_t,
        pub definehash: *mut *mut define_t,
        pub indentstack: *mut indent_t,
        pub skip: libc::c_int,
        pub token: token_t,
    }
    pub type source_t = source_s;
    use super::{libc};
    use super::l_script_h::{token_t, script_t, punctuation_t};
    extern "C" {
        //expect a certain token
        #[no_mangle]
        pub fn PC_ExpectTokenString(source: *mut source_t,
                                    string: *mut libc::c_char) -> libc::c_int;
        //expect a certain token type
        #[no_mangle]
        pub fn PC_ExpectTokenType(source: *mut source_t, type_0: libc::c_int,
                                  subtype: libc::c_int, token: *mut token_t)
         -> libc::c_int;
        //expect a token
        #[no_mangle]
        pub fn PC_ExpectAnyToken(source: *mut source_t, token: *mut token_t)
         -> libc::c_int;
        //returns true when the token is available
        #[no_mangle]
        pub fn PC_CheckTokenString(source: *mut source_t,
                                   string: *mut libc::c_char) -> libc::c_int;
        //unread the last token read from the script
        #[no_mangle]
        pub fn PC_UnreadLastToken(source: *mut source_t);
    }
}
#[header_src =
      "ioq3/code/botlib/l_struct.h"]
pub mod l_struct_h {
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
 * name:		l_struct.h
 *
 * desc:		structure reading/writing
 *
 * $Archive: /source/code/botlib/l_struct.h $
 *
 *****************************************************************************/
    //field types
    // char
    // int
    // float
    // char [MAX_STRINGFIELD]
    // struct (sub structure)
    //type only mask
    // only type, clear subtype
    //sub types
    // array of type
    // bounded value
    //structure field definition
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fielddef_s {
        pub name: *mut libc::c_char,
        pub offset: libc::c_int,
        pub type_0: libc::c_int,
        pub maxarray: libc::c_int,
        pub floatmin: libc::c_float,
        pub floatmax: libc::c_float,
        pub substruct: *mut structdef_s,
    }
    //structure definition
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct structdef_s {
        pub size: libc::c_int,
        pub fields: *mut fielddef_t,
    }
    pub type fielddef_t = fielddef_s;
    pub type structdef_t = structdef_s;
    use super::{libc};
    use super::l_precomp_h::{source_t};
    use super::FILE_h::{FILE};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    use super::FILE_h::{FILE};
    extern "C" {
        #[no_mangle]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, ...)
         -> libc::c_int;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src =
      "ioq3/code/botlib/l_variadic.h"]
pub mod l_variadic_h {
    use super::l_precomp_h::{source_t};
    use super::{libc};
    extern "C" {
        //print a source error
        #[no_mangle]
        pub fn SourceError(source: *mut source_t,
                           str: *mut libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/botlib/l_struct.c"]
pub mod l_struct_c {
    use super::{libc};
    use super::l_precomp_h::{source_t};
    use super::l_struct_h::{fielddef_t, structdef_t};
    use super::q_shared_h::{qboolean};
    use super::FILE_h::{FILE};
}
use self::types_h::{__off_t, __off64_t};
use self::stddef_h::{size_t};
use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt,
                          _IO_marker};
use self::FILE_h::{FILE};
use self::q_shared_h::{qboolean, qtrue, qfalse, Com_sprintf};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t, StripDoubleQuotes,
                       StripSingleQuotes};
use self::l_precomp_h::{define_s, define_t, indent_s, indent_t, source_s,
                        source_t, PC_ExpectTokenString, PC_ExpectTokenType,
                        PC_ExpectAnyToken, PC_CheckTokenString,
                        PC_UnreadLastToken};
use self::l_struct_h::{fielddef_s, structdef_s, fielddef_t, structdef_t};
use self::stdio_h::{fprintf};
use self::string_h::{strncpy, strcmp, strlen};
use self::l_variadic_h::{SourceError};
//read a structure from a script
#[no_mangle]
pub unsafe extern "C" fn ReadStructure(mut source: *mut source_t,
                                       mut def: *mut structdef_t,
                                       mut structure: *mut libc::c_char)
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
    let mut fd: *mut fielddef_t = 0 as *mut fielddef_t;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut num: libc::c_int = 0;
    if 0 ==
           PC_ExpectTokenString(source,
                                b"{\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char) {
        return 0i32
    }
    loop  {
        if 0 == PC_ExpectAnyToken(source, &mut token) {
            return qfalse as libc::c_int
        }
        //if end of structure
        if 0 ==
               strcmp(token.string.as_mut_ptr(),
                      b"}\x00" as *const u8 as *const libc::c_char) {
            break ;
        }
        fd = FindField((*def).fields, token.string.as_mut_ptr());
        if fd.is_null() {
            SourceError(source,
                        b"unknown structure field %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return qfalse as libc::c_int
        }
        if 0 != (*fd).type_0 & 0x100i32 {
            num = (*fd).maxarray;
            if 0 ==
                   PC_ExpectTokenString(source,
                                        b"{\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) {
                return qfalse as libc::c_int
            }
        } else { num = 1i32 }
        p = structure.offset((*fd).offset as isize) as *mut libc::c_void;
        loop  {
            let fresh0 = num;
            num = num - 1;
            if !(fresh0 > 0i32) { break ; }
            if 0 != (*fd).type_0 & 0x100i32 {
                if 0 !=
                       PC_CheckTokenString(source,
                                           b"}\x00" as *const u8 as
                                               *const libc::c_char as
                                               *mut libc::c_char) {
                    break ;
                }
            }
            match (*fd).type_0 & 0xffi32 {
                1 => {
                    if 0 == ReadChar(source, fd, p) as u64 {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as
                             *mut libc::c_char).offset(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong as
                                                           isize) as
                            *mut libc::c_void
                }
                2 => {
                    if 0 == ReadNumber(source, fd, p) as u64 {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as
                             *mut libc::c_char).offset(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong as
                                                           isize) as
                            *mut libc::c_void
                }
                3 => {
                    if 0 == ReadNumber(source, fd, p) as u64 {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as
                             *mut libc::c_char).offset(::std::mem::size_of::<libc::c_float>()
                                                           as libc::c_ulong as
                                                           isize) as
                            *mut libc::c_void
                }
                4 => {
                    if 0 == ReadString(source, fd, p) {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as *mut libc::c_char).offset(80isize) as
                            *mut libc::c_void
                }
                6 => {
                    if (*fd).substruct.is_null() {
                        SourceError(source,
                                    b"BUG: no sub structure defined\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
                        return qfalse as libc::c_int
                    }
                    ReadStructure(source, (*fd).substruct,
                                  p as *mut libc::c_char);
                    p =
                        (p as
                             *mut libc::c_char).offset((*(*fd).substruct).size
                                                           as isize) as
                            *mut libc::c_void
                }
                _ => { }
            }
            //end case
            //end switch
            if !(0 != (*fd).type_0 & 0x100i32) { continue ; }
            if 0 == PC_ExpectAnyToken(source, &mut token) {
                return qfalse as libc::c_int
            }
            if 0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b"}\x00" as *const u8 as *const libc::c_char) {
                break ;
            }
            if 0 !=
                   strcmp(token.string.as_mut_ptr(),
                          b",\x00" as *const u8 as *const libc::c_char) {
                SourceError(source,
                            b"expected a comma, found %s\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            token.string.as_mut_ptr());
                return qfalse as libc::c_int
            }
        }
    }
    return qtrue as libc::c_int;
}
//end of the function ReadChar
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ReadString(mut source: *mut source_t,
                                    mut fd: *mut fielddef_t,
                                    mut p: *mut libc::c_void) -> libc::c_int {
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
    if 0 == PC_ExpectTokenType(source, 1i32, 0i32, &mut token) { return 0i32 }
    StripDoubleQuotes(token.string.as_mut_ptr());
    strncpy(p as *mut libc::c_char, token.string.as_mut_ptr(),
            (80i32 - 1i32) as libc::c_ulong);
    *(p as *mut libc::c_char).offset((80i32 - 1i32) as isize) =
        '\u{0}' as i32 as libc::c_char;
    return 1i32;
}
//end of the function FindField
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ReadNumber(mut source: *mut source_t,
                                    mut fd: *mut fielddef_t,
                                    mut p: *mut libc::c_void) -> qboolean {
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
    let mut negative: libc::c_int = qfalse as libc::c_int;
    let mut intval: libc::c_long = 0;
    let mut intmin: libc::c_long = 0i32 as libc::c_long;
    let mut intmax: libc::c_long = 0i32 as libc::c_long;
    let mut floatval: libc::c_double = 0.;
    if 0 == PC_ExpectAnyToken(source, &mut token) { return qfalse }
    if token.type_0 == 5i32 {
        if 0 != (*fd).type_0 & 0x400i32 {
            SourceError(source,
                        b"expected unsigned value, found %s\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return qfalse
        }
        if 0 !=
               strcmp(token.string.as_mut_ptr(),
                      b"-\x00" as *const u8 as *const libc::c_char) {
            SourceError(source,
                        b"unexpected punctuation %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return qfalse
        }
        negative = qtrue as libc::c_int;
        if 0 == PC_ExpectAnyToken(source, &mut token) { return qfalse }
    }
    if token.type_0 != 3i32 {
        SourceError(source,
                    b"expected number, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr());
        return qfalse
    }
    if 0 != token.subtype & 0x800i32 {
        if (*fd).type_0 & 0xffi32 != 3i32 {
            SourceError(source,
                        b"unexpected float\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            return qfalse
        }
        floatval = token.floatvalue as libc::c_double;
        if 0 != negative { floatval = -floatval }
        if 0 != (*fd).type_0 & 0x200i32 {
            if floatval < (*fd).floatmin as libc::c_double ||
                   floatval > (*fd).floatmax as libc::c_double {
                SourceError(source,
                            b"float out of range [%f, %f]\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            (*fd).floatmin as libc::c_double,
                            (*fd).floatmax as libc::c_double);
                return qfalse
            }
        }
        *(p as *mut libc::c_float) = floatval as libc::c_float;
        return qtrue
    }
    intval = token.intvalue as libc::c_long;
    if 0 != negative { intval = -intval }
    if (*fd).type_0 & 0xffi32 == 1i32 {
        if 0 != (*fd).type_0 & 0x400i32 {
            intmin = 0i32 as libc::c_long;
            intmax = 255i32 as libc::c_long
        } else {
            intmin = -128i32 as libc::c_long;
            intmax = 127i32 as libc::c_long
        }
    }
    if (*fd).type_0 & 0xffi32 == 2i32 {
        if 0 != (*fd).type_0 & 0x400i32 {
            intmin = 0i32 as libc::c_long;
            intmax = 65535i32 as libc::c_long
        } else {
            intmin = -32768i32 as libc::c_long;
            intmax = 32767i32 as libc::c_long
        }
    }
    if (*fd).type_0 & 0xffi32 == 1i32 || (*fd).type_0 & 0xffi32 == 2i32 {
        if 0 != (*fd).type_0 & 0x200i32 {
            intmin =
                (if intmin as libc::c_float > (*fd).floatmin {
                     intmin as libc::c_float
                 } else { (*fd).floatmin }) as libc::c_long;
            intmax =
                (if (intmax as libc::c_float) < (*fd).floatmax {
                     intmax as libc::c_float
                 } else { (*fd).floatmax }) as libc::c_long
        }
        if intval < intmin || intval > intmax {
            SourceError(source,
                        b"value %ld out of range [%ld, %ld]\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        intval, intmin, intmax);
            return qfalse
        }
    } else if (*fd).type_0 & 0xffi32 == 3i32 {
        if 0 != (*fd).type_0 & 0x200i32 {
            if (intval as libc::c_float) < (*fd).floatmin ||
                   intval as libc::c_float > (*fd).floatmax {
                SourceError(source,
                            b"value %ld out of range [%f, %f]\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, intval,
                            (*fd).floatmin as libc::c_double,
                            (*fd).floatmax as libc::c_double);
                return qfalse
            }
        }
    }
    if (*fd).type_0 & 0xffi32 == 1i32 {
        if 0 != (*fd).type_0 & 0x400i32 {
            *(p as *mut libc::c_uchar) = intval as libc::c_uchar
        } else { *(p as *mut libc::c_char) = intval as libc::c_char }
    } else if (*fd).type_0 & 0xffi32 == 2i32 {
        if 0 != (*fd).type_0 & 0x400i32 {
            *(p as *mut libc::c_uint) = intval as libc::c_uint
        } else { *(p as *mut libc::c_int) = intval as libc::c_int }
    } else if (*fd).type_0 & 0xffi32 == 3i32 {
        *(p as *mut libc::c_float) = intval as libc::c_float
    }
    return qtrue;
}
//end of the function ReadNumber
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ReadChar(mut source: *mut source_t,
                                  mut fd: *mut fielddef_t,
                                  mut p: *mut libc::c_void) -> qboolean {
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
    if 0 == PC_ExpectAnyToken(source, &mut token) { return qfalse }
    if token.type_0 == 2i32 {
        StripSingleQuotes(token.string.as_mut_ptr());
        *(p as *mut libc::c_char) = token.string[0usize]
    } else {
        PC_UnreadLastToken(source);
        if 0 == ReadNumber(source, fd, p) as u64 { return qfalse }
    }
    return qtrue;
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
 * name:		l_struct.c
 *
 * desc:		structure reading / writing
 *
 * $Archive: /MissionPack/CODE/botlib/l_struct.c $
 *
 *****************************************************************************/
//BOTLIB
//BSPC
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn FindField(mut defs: *mut fielddef_t,
                                   mut name: *mut libc::c_char)
 -> *mut fielddef_t {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !(*defs.offset(i as isize)).name.is_null() {
        if 0 == strcmp((*defs.offset(i as isize)).name, name) {
            return &mut *defs.offset(i as isize) as *mut fielddef_t
        }
        i += 1
    }
    return 0 as *mut fielddef_t;
}
//write a structure to a file
#[no_mangle]
pub unsafe extern "C" fn WriteStructure(mut fp: *mut FILE,
                                        mut def: *mut structdef_t,
                                        mut structure: *mut libc::c_char)
 -> libc::c_int {
    return WriteStructWithIndent(fp, def, structure, 0i32);
}
//end of the function WriteFloat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn WriteStructWithIndent(mut fp: *mut FILE,
                                               mut def: *mut structdef_t,
                                               mut structure:
                                                   *mut libc::c_char,
                                               mut indent: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fd: *mut fielddef_t = 0 as *mut fielddef_t;
    if 0 == WriteIndent(fp, indent) { return qfalse as libc::c_int }
    if fprintf(fp, b"{\r\n\x00" as *const u8 as *const libc::c_char) < 0i32 {
        return qfalse as libc::c_int
    }
    indent += 1;
    i = 0i32;
    while !(*(*def).fields.offset(i as isize)).name.is_null() {
        fd = &mut *(*def).fields.offset(i as isize) as *mut fielddef_t;
        if 0 == WriteIndent(fp, indent) { return qfalse as libc::c_int }
        if fprintf(fp, b"%s\t\x00" as *const u8 as *const libc::c_char,
                   (*fd).name) < 0i32 {
            return qfalse as libc::c_int
        }
        p = structure.offset((*fd).offset as isize) as *mut libc::c_void;
        if 0 != (*fd).type_0 & 0x100i32 {
            num = (*fd).maxarray;
            if fprintf(fp, b"{\x00" as *const u8 as *const libc::c_char) <
                   0i32 {
                return qfalse as libc::c_int
            }
        } else { num = 1i32 }
        loop  {
            let fresh1 = num;
            num = num - 1;
            if !(fresh1 > 0i32) { break ; }
            match (*fd).type_0 & 0xffi32 {
                1 => {
                    if fprintf(fp,
                               b"%d\x00" as *const u8 as *const libc::c_char,
                               *(p as *mut libc::c_char) as libc::c_int) <
                           0i32 {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as
                             *mut libc::c_char).offset(::std::mem::size_of::<libc::c_char>()
                                                           as libc::c_ulong as
                                                           isize) as
                            *mut libc::c_void
                }
                2 => {
                    if fprintf(fp,
                               b"%d\x00" as *const u8 as *const libc::c_char,
                               *(p as *mut libc::c_int)) < 0i32 {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as
                             *mut libc::c_char).offset(::std::mem::size_of::<libc::c_int>()
                                                           as libc::c_ulong as
                                                           isize) as
                            *mut libc::c_void
                }
                3 => {
                    if 0 == WriteFloat(fp, *(p as *mut libc::c_float)) {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as
                             *mut libc::c_char).offset(::std::mem::size_of::<libc::c_float>()
                                                           as libc::c_ulong as
                                                           isize) as
                            *mut libc::c_void
                }
                4 => {
                    if fprintf(fp,
                               b"\"%s\"\x00" as *const u8 as
                                   *const libc::c_char,
                               p as *mut libc::c_char) < 0i32 {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as *mut libc::c_char).offset(80isize) as
                            *mut libc::c_void
                }
                6 => {
                    if 0 ==
                           WriteStructWithIndent(fp, (*fd).substruct,
                                                 structure, indent) {
                        return qfalse as libc::c_int
                    }
                    p =
                        (p as
                             *mut libc::c_char).offset((*(*fd).substruct).size
                                                           as isize) as
                            *mut libc::c_void
                }
                _ => { }
            }
            if 0 != (*fd).type_0 & 0x100i32 {
                if num > 0i32 {
                    if fprintf(fp,
                               b",\x00" as *const u8 as *const libc::c_char) <
                           0i32 {
                        return qfalse as libc::c_int
                    }
                } else if fprintf(fp,
                                  b"}\x00" as *const u8 as
                                      *const libc::c_char) < 0i32 {
                    return qfalse as libc::c_int
                }
            }
        }
        if fprintf(fp, b"\r\n\x00" as *const u8 as *const libc::c_char) < 0i32
           {
            return qfalse as libc::c_int
        }
        i += 1
    }
    indent -= 1;
    if 0 == WriteIndent(fp, indent) { return qfalse as libc::c_int }
    if fprintf(fp, b"}\r\n\x00" as *const u8 as *const libc::c_char) < 0i32 {
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//writes indents
#[no_mangle]
pub unsafe extern "C" fn WriteIndent(mut fp: *mut FILE,
                                     mut indent: libc::c_int) -> libc::c_int {
    loop  {
        let fresh2 = indent;
        indent = indent - 1;
        if !(fresh2 > 0i32) { break ; }
        if fprintf(fp, b"\t\x00" as *const u8 as *const libc::c_char) < 0i32 {
            return qfalse as libc::c_int
        }
    }
    return qtrue as libc::c_int;
}
//writes a float without traling zeros
#[no_mangle]
pub unsafe extern "C" fn WriteFloat(mut fp: *mut FILE,
                                    mut value: libc::c_float) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut l: libc::c_int = 0;
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                    as libc::c_int,
                b"%f\x00" as *const u8 as *const libc::c_char,
                value as libc::c_double);
    l = strlen(buf.as_mut_ptr()) as libc::c_int;
    loop  {
        let fresh3 = l;
        l = l - 1;
        if !(fresh3 > 1i32) { break ; }
        if buf[l as usize] as libc::c_int != '0' as i32 &&
               buf[l as usize] as libc::c_int != '.' as i32 {
            break ;
        }
        if buf[l as usize] as libc::c_int == '.' as i32 {
            buf[l as usize] = 0i32 as libc::c_char;
            break ;
        } else { buf[l as usize] = 0i32 as libc::c_char }
    }
    if fprintf(fp, b"%s\x00" as *const u8 as *const libc::c_char,
               buf.as_mut_ptr()) < 0i32 {
        return 0i32
    }
    return 1i32;
}