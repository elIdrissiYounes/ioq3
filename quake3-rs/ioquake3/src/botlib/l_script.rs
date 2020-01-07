// =============== BEGIN l_script_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct punctuation_s {
    pub p: *mut i8,
    pub n: i32,
    pub next: *mut crate::src::botlib::l_script::punctuation_s,
}
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

pub type punctuation_t = crate::src::botlib::l_script::punctuation_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct token_s {
    pub string: [i8; 1024],
    pub type_0: i32,
    pub subtype: i32,
    pub intvalue: usize,
    pub floatvalue: f32,
    pub whitespace_p: *mut i8,
    pub endwhitespace_p: *mut i8,
    pub line: i32,
    pub linescrossed: i32,
    pub next: *mut crate::src::botlib::l_script::token_s,
}
//punctuation character(s)
//punctuation indication
//next punctuation
//token

pub type token_t = crate::src::botlib::l_script::token_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct script_s {
    pub filename: [i8; 1024],
    pub buffer: *mut i8,
    pub script_p: *mut i8,
    pub end_p: *mut i8,
    pub lastscript_p: *mut i8,
    pub whitespace_p: *mut i8,
    pub endwhitespace_p: *mut i8,
    pub length: i32,
    pub line: i32,
    pub lastline: i32,
    pub tokenavailable: i32,
    pub flags: i32,
    pub punctuations: *mut crate::src::botlib::l_script::punctuation_t,
    pub punctuationtable: *mut *mut crate::src::botlib::l_script::punctuation_t,
    pub token: crate::src::botlib::l_script::token_t,
    pub next: *mut crate::src::botlib::l_script::script_s,
}
//available token
//last read token type
//last read token sub type
//integer value
//floating point value
//NUMBERVALUE
//start of white space before token
//start of white space before token
//line the token was on
//lines crossed in white space
//next token in chain
//script file

pub type script_t = crate::src::botlib::l_script::script_s;
use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
use crate::src::botlib::be_interface::botimport;
use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedMemory;
use crate::src::botlib::l_memory::GetMemory;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::strcat;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncmp;
use crate::stdlib::vsnprintf;
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

pub static mut default_punctuations: [crate::src::botlib::l_script::punctuation_t; 53] = [
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b">>=\x00" as *const u8 as *mut i8,
            n: 1,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"<<=\x00" as *const u8 as *mut i8,
            n: 2,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"...\x00" as *const u8 as *mut i8,
            n: 3,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"##\x00" as *const u8 as *mut i8,
            n: 4,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"&&\x00" as *const u8 as *mut i8,
            n: 5,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"||\x00" as *const u8 as *mut i8,
            n: 6,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b">=\x00" as *const u8 as *mut i8,
            n: 7,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"<=\x00" as *const u8 as *mut i8,
            n: 8,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"==\x00" as *const u8 as *mut i8,
            n: 9,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"!=\x00" as *const u8 as *mut i8,
            n: 10,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"*=\x00" as *const u8 as *mut i8,
            n: 11,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"/=\x00" as *const u8 as *mut i8,
            n: 12,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"%=\x00" as *const u8 as *mut i8,
            n: 13,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"+=\x00" as *const u8 as *mut i8,
            n: 14,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"-=\x00" as *const u8 as *mut i8,
            n: 15,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"++\x00" as *const u8 as *mut i8,
            n: 16,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"--\x00" as *const u8 as *mut i8,
            n: 17,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"&=\x00" as *const u8 as *mut i8,
            n: 18,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"|=\x00" as *const u8 as *mut i8,
            n: 19,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"^=\x00" as *const u8 as *mut i8,
            n: 20,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b">>\x00" as *const u8 as *mut i8,
            n: 21,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"<<\x00" as *const u8 as *mut i8,
            n: 22,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"->\x00" as *const u8 as *mut i8,
            n: 23,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"::\x00" as *const u8 as *mut i8,
            n: 24,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b".*\x00" as *const u8 as *mut i8,
            n: 25,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"*\x00" as *const u8 as *mut i8,
            n: 26,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"/\x00" as *const u8 as *mut i8,
            n: 27,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"%\x00" as *const u8 as *mut i8,
            n: 28,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"+\x00" as *const u8 as *mut i8,
            n: 29,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"-\x00" as *const u8 as *mut i8,
            n: 30,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"=\x00" as *const u8 as *mut i8,
            n: 31,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"&\x00" as *const u8 as *mut i8,
            n: 32,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"|\x00" as *const u8 as *mut i8,
            n: 33,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"^\x00" as *const u8 as *mut i8,
            n: 34,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"~\x00" as *const u8 as *mut i8,
            n: 35,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"!\x00" as *const u8 as *mut i8,
            n: 36,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b">\x00" as *const u8 as *mut i8,
            n: 37,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"<\x00" as *const u8 as *mut i8,
            n: 38,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b".\x00" as *const u8 as *mut i8,
            n: 39,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b",\x00" as *const u8 as *mut i8,
            n: 40,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b";\x00" as *const u8 as *mut i8,
            n: 41,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b":\x00" as *const u8 as *mut i8,
            n: 42,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"?\x00" as *const u8 as *mut i8,
            n: 43,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"(\x00" as *const u8 as *mut i8,
            n: 44,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b")\x00" as *const u8 as *mut i8,
            n: 45,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"{\x00" as *const u8 as *mut i8,
            n: 46,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"}\x00" as *const u8 as *mut i8,
            n: 47,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"[\x00" as *const u8 as *mut i8,
            n: 48,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"]\x00" as *const u8 as *mut i8,
            n: 49,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"\\\x00" as *const u8 as *mut i8,
            n: 50,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"#\x00" as *const u8 as *mut i8,
            n: 51,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: b"$\x00" as *const u8 as *mut i8,
            n: 52,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
    {
        let mut init = crate::src::botlib::l_script::punctuation_s {
            p: 0 as *mut i8,
            n: 0,
            next: 0 as *mut crate::src::botlib::l_script::punctuation_s,
        };
        init
    },
];
#[no_mangle]

pub static mut basefolder: [i8; 64] = [0; 64];
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_CreatePunctuationTable(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut punctuations: *mut crate::src::botlib::l_script::punctuation_t,
) {
    let mut i: i32 = 0;
    let mut p: *mut crate::src::botlib::l_script::punctuation_t =
        0 as *mut crate::src::botlib::l_script::punctuation_t;
    let mut lastp: *mut crate::src::botlib::l_script::punctuation_t =
        0 as *mut crate::src::botlib::l_script::punctuation_t;
    let mut newp: *mut crate::src::botlib::l_script::punctuation_t =
        0 as *mut crate::src::botlib::l_script::punctuation_t;
    //get memory for the table
    if (*script).punctuationtable.is_null() {
        (*script).punctuationtable =
            crate::src::botlib::l_memory::GetMemory((256usize).wrapping_mul(::std::mem::size_of::<
                *mut crate::src::botlib::l_script::punctuation_t,
            >())) as *mut *mut crate::src::botlib::l_script::punctuation_t
    }
    crate::stdlib::memset(
        (*script).punctuationtable as *mut libc::c_void,
        0,
        (256usize).wrapping_mul(::std::mem::size_of::<
            *mut crate::src::botlib::l_script::punctuation_t,
        >()),
    );
    //add the punctuations in the list to the punctuation table
    i = 0;
    while !(*punctuations.offset(i as isize)).p.is_null() {
        newp = &mut *punctuations.offset(i as isize)
            as *mut crate::src::botlib::l_script::punctuation_t;
        lastp = 0 as *mut crate::src::botlib::l_script::punctuation_t;
        //end if
        p = *(*script)
            .punctuationtable
            .offset(*(*newp).p.offset(0) as u32 as isize);
        while !p.is_null() {
            //sort the punctuations in this table entry on length (longer punctuations first)
            //end for
            if crate::stdlib::strlen((*p).p) < crate::stdlib::strlen((*newp).p) {
                (*newp).next = p; //end if
                if !lastp.is_null() {
                    (*lastp).next = newp
                } else {
                    let ref mut fresh0 = *(*script)
                        .punctuationtable
                        .offset(*(*newp).p.offset(0) as u32 as isize);
                    *fresh0 = newp
                }
                break;
            } else {
                lastp = p;
                p = (*p).next
            }
        }
        if p.is_null() {
            (*newp).next = 0 as *mut crate::src::botlib::l_script::punctuation_s;
            if !lastp.is_null() {
                (*lastp).next = newp
            } else {
                let ref mut fresh1 = *(*script)
                    .punctuationtable
                    .offset(*(*newp).p.offset(0) as u32 as isize);
                *fresh1 = newp
            }
        }
        i += 1
    }
    //end for
}
//returns a pointer to the punctuation with the given number
//end of the function PS_CreatePunctuationTable
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn PunctuationFromNum(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut num: i32,
) -> *mut i8 {
    let mut i: i32 = 0; //end for
    i = 0;
    while !(*(*script).punctuations.offset(i as isize)).p.is_null() {
        if (*(*script).punctuations.offset(i as isize)).n == num {
            return (*(*script).punctuations.offset(i as isize)).p;
        }
        i += 1
    }
    return b"unknown punctuation\x00" as *const u8 as *mut i8;
}
//print a script error with filename and line number
//end of the function PunctuationFromNum
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ScriptError(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut str: *mut i8,
    mut args: ...
) {
    let mut text: [i8; 1024] = [0; 1024];
    let mut ap: ::std::ffi::VaListImpl;
    if (*script).flags & 0x1 != 0 {
        return;
    }
    ap = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        str,
        ap.as_va_list(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        3,
        b"file %s, line %d: %s\n\x00" as *const u8 as *mut i8,
        (*script).filename.as_mut_ptr(),
        (*script).line,
        text.as_mut_ptr(),
    );
    //BOTLIB
    //MEQCC
    //BSPC
}
//print a script warning with filename and line number
//end of the function ScriptError
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ScriptWarning(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut str: *mut i8,
    mut args: ...
) {
    let mut text: [i8; 1024] = [0; 1024];
    let mut ap: ::std::ffi::VaListImpl;
    if (*script).flags & 0x2 != 0 {
        return;
    }
    ap = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        str,
        ap.as_va_list(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        2,
        b"file %s, line %d: %s\n\x00" as *const u8 as *mut i8,
        (*script).filename.as_mut_ptr(),
        (*script).line,
        text.as_mut_ptr(),
    );
    //BOTLIB
    //MEQCC
    //BSPC
}
//set an array with punctuations, NULL restores default C/C++ set
//end of the function ScriptWarning
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn SetScriptPunctuations(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut p: *mut crate::src::botlib::l_script::punctuation_t,
) {
    if !p.is_null() {
        PS_CreatePunctuationTable(script, p);
    } else {
        PS_CreatePunctuationTable(script, default_punctuations.as_mut_ptr());
    }
    //PUNCTABLE
    if !p.is_null() {
        (*script).punctuations = p
    } else {
        (*script).punctuations = default_punctuations.as_mut_ptr()
    };
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

pub unsafe extern "C" fn PS_ReadWhiteSpace(
    mut script: *mut crate::src::botlib::l_script::script_t,
) -> i32 {
    loop
    //end if
    {
        while *(*script).script_p as i32 <= ' ' as i32 {
            if *(*script).script_p == 0 {
                return 0i32;
            } //end while
            if *(*script).script_p as i32 == '\n' as i32 {
                (*script).line += 1
            }
            (*script).script_p = (*script).script_p.offset(1)
        }
        //skip white space
        //end while
        //skip comments
        if !(*(*script).script_p as i32 == '/' as i32) {
            break; //end if
        }
        if *(*script).script_p.offset(1) as i32 == '/' as i32 {
            //comments //
            (*script).script_p = (*script).script_p.offset(1); //end if
            loop {
                (*script).script_p = (*script).script_p.offset(1); //end do
                if *(*script).script_p == 0 {
                    return 0i32;
                }
                if !(*(*script).script_p as i32 != '\n' as i32) {
                    break;
                }
            }
            (*script).line += 1;
            (*script).script_p = (*script).script_p.offset(1);
            if *(*script).script_p == 0 {
                return 0i32;
            }
        } else {
            //comments /* */
            if !(*(*script).script_p.offset(1) as i32 == '*' as i32) {
                break; //end do
            }
            (*script).script_p = (*script).script_p.offset(1);
            loop {
                (*script).script_p = (*script).script_p.offset(1);
                if *(*script).script_p == 0 {
                    return 0i32;
                }
                if *(*script).script_p as i32 == '\n' as i32 {
                    (*script).line += 1
                }
                if *(*script).script_p as i32 == '*' as i32
                    && *(*script).script_p.offset(1) as i32 == '/' as i32
                {
                    break;
                }
            }
            (*script).script_p = (*script).script_p.offset(1);
            if *(*script).script_p == 0 {
                return 0i32;
            }
            (*script).script_p = (*script).script_p.offset(1);
            if *(*script).script_p == 0 {
                return 0i32;
            }
        }
    }
    return 1;
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

pub unsafe extern "C" fn PS_ReadEscapeCharacter(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut ch: *mut i8,
) -> i32 {
    let mut c: i32 = 0;
    let mut val: i32 = 0;
    let mut i: i32 = 0;
    //step over the leading '\\'
    (*script).script_p = (*script).script_p.offset(1);
    //determine the escape character
    match *(*script).script_p as i32 {
        92 => {
            c = '\\' as i32 //end switch
                            //end default
        }
        110 => c = '\n' as i32,
        114 => c = '\r' as i32,
        116 => c = '\t' as i32,
        118 => c = '\u{b}' as i32,
        98 => c = '\u{8}' as i32,
        102 => c = '\u{c}' as i32,
        97 => c = '\u{7}' as i32,
        39 => c = '\'' as i32,
        34 => c = '\"' as i32,
        63 => c = '?' as i32,
        120 => {
            (*script).script_p = (*script).script_p.offset(1); //end case
            i = 0; //end for
            val = 0; //end if
            loop {
                c = *(*script).script_p as i32;
                if c >= '0' as i32 && c <= '9' as i32 {
                    c = c - '0' as i32
                } else if c >= 'A' as i32 && c <= 'Z' as i32 {
                    c = c - 'A' as i32 + 10
                } else {
                    if !(c >= 'a' as i32 && c <= 'z' as i32) {
                        break;
                    }
                    c = c - 'a' as i32 + 10
                }
                val = (val << 4) + c;
                i += 1;
                (*script).script_p = (*script).script_p.offset(1)
            }
            (*script).script_p = (*script).script_p.offset(-1);
            if val > 0xff {
                ScriptWarning(
                    script,
                    b"too large value in escape character\x00" as *const u8 as *mut i8,
                );
                val = 0xff
            }
            c = val
        }
        _ => {
            //NOTE: decimal ASCII code, NOT octal
            if (*(*script).script_p as i32) < '0' as i32 || *(*script).script_p as i32 > '9' as i32
            {
                ScriptError(script, b"unknown escape char\x00" as *const u8 as *mut i8);
                //end for
            } //end if
            i = 0;
            val = 0;
            loop {
                c = *(*script).script_p as i32;
                if !(c >= '0' as i32 && c <= '9' as i32) {
                    break;
                }
                c = c - '0' as i32;
                val = val * 10 + c;
                i += 1;
                (*script).script_p = (*script).script_p.offset(1)
            }
            (*script).script_p = (*script).script_p.offset(-1);
            if val > 0xff {
                ScriptWarning(
                    script,
                    b"too large value in escape character\x00" as *const u8 as *mut i8,
                );
                val = 0xff
            }
            c = val
        }
    }
    //step over the escape character or the last digit of the number
    (*script).script_p = (*script).script_p.offset(1);
    //store the escape character
    *ch = c as i8;
    //successfully read escape character
    return 1;
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

pub unsafe extern "C" fn PS_ReadString(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
    mut quote: i32,
) -> i32 {
    let mut len: i32 = 0;
    let mut tmpline: i32 = 0;
    let mut tmpscript_p: *mut i8 = 0 as *mut i8;
    if quote == '\"' as i32 {
        (*token).type_0 = 1
    } else {
        (*token).type_0 = 2
    }
    len = 0;
    //leading quote
    let fresh2 = (*script).script_p;
    (*script).script_p = (*script).script_p.offset(1);
    let fresh3 = len;
    len = len + 1;
    (*token).string[fresh3 as usize] = *fresh2;
    loop
    //
    {
        if len >= 1024 - 2 {
            ScriptError(
                script,
                b"string longer than MAX_TOKEN = %d\x00" as *const u8 as *mut i8,
                1024i32,
            ); //end while
            return 0i32;
        }
        //minus 2 because trailing double quote and zero have to be appended
        //end if
        //if there is an escape character and
        //if escape characters inside a string are allowed
        if *(*script).script_p as i32 == '\\' as i32 && (*script).flags & 0x8 == 0 {
            if PS_ReadEscapeCharacter(
                script,
                &mut *(*token).string.as_mut_ptr().offset(len as isize),
            ) == 0
            {
                (*token).string[len as usize] = 0i8; //end if
                return 0i32;
            } //end if
            len += 1
        } else if *(*script).script_p as i32 == quote {
            //if a trailing quote
            (*script).script_p = (*script).script_p.offset(1); //end if
                                                               //step over the double quote
                                                               //if white spaces in a string are not allowed
            if (*script).flags & 0x4 != 0 {
                break;
            }
            //
            tmpscript_p = (*script).script_p;
            tmpline = (*script).line;
            //read unusefull stuff between possible two following strings
            if PS_ReadWhiteSpace(script) == 0 {
                (*script).script_p = tmpscript_p; //end if
                (*script).line = tmpline;
                break;
            } else if *(*script).script_p as i32 != quote {
                (*script).script_p = tmpscript_p;
                (*script).line = tmpline;
                break;
            } else {
                //if there's no leading double qoute
                //end if
                //step over the new leading double quote
                (*script).script_p = (*script).script_p.offset(1)
            }
        } else {
            if *(*script).script_p as i32 == '\u{0}' as i32 {
                (*token).string[len as usize] = 0i8; //end if
                ScriptError(
                    script,
                    b"missing trailing quote\x00" as *const u8 as *mut i8,
                ); //end if
                return 0i32;
            }
            if *(*script).script_p as i32 == '\n' as i32 {
                (*token).string[len as usize] = 0i8;
                ScriptError(
                    script,
                    b"newline inside string %s\x00" as *const u8 as *mut i8,
                    (*token).string.as_mut_ptr(),
                );
                return 0i32;
            }
            let fresh4 = (*script).script_p;
            (*script).script_p = (*script).script_p.offset(1);
            let fresh5 = len;
            len = len + 1;
            (*token).string[fresh5 as usize] = *fresh4
        }
    }
    //trailing quote
    let fresh6 = len;
    len = len + 1;
    (*token).string[fresh6 as usize] = quote as i8;
    //end string with a zero
    (*token).string[len as usize] = '\u{0}' as i8;
    //the sub type is the length of the string
    (*token).subtype = len;
    return 1;
}
//end of the function PS_ReadString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ReadName(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut len: i32 = 0; //end if
    let mut c: i8 = 0;
    (*token).type_0 = 4;
    loop {
        let fresh7 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        let fresh8 = len;
        len = len + 1;
        (*token).string[fresh8 as usize] = *fresh7;
        if len >= 1024 {
            ScriptError(
                script,
                b"name longer than MAX_TOKEN = %d\x00" as *const u8 as *mut i8,
                1024i32,
            );
            return 0i32;
        }
        c = *(*script).script_p;
        if !(c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32
            || c as i32 >= 'A' as i32 && c as i32 <= 'Z' as i32
            || c as i32 >= '0' as i32 && c as i32 <= '9' as i32
            || c as i32 == '_' as i32)
        {
            break;
        }
    }
    (*token).string[len as usize] = '\u{0}' as i8;
    //the sub type is the length of the name
    (*token).subtype = len;
    return 1;
}
//end of the function PS_ReadName
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn NumberValue(
    mut string: *mut i8,
    mut subtype: i32,
    mut intvalue: *mut usize,
    mut floatvalue: *mut f32,
) {
    let mut dotfound: usize = 0;
    *intvalue = 0usize;
    *floatvalue = 0f32;
    //floating point number
    if subtype & 0x800 != 0 {
        while *string != 0 {
            //end if
            //end while
            if *string as i32 == '.' as i32 {
                if dotfound != 0 {
                    return;
                } //end if
                dotfound = 10; //end else
                string = string.offset(1)
            } //end if
            if dotfound != 0 {
                *floatvalue = *floatvalue + (*string as i32 - '0' as i32) as f32 / dotfound as f32; //end else if
                dotfound = dotfound.wrapping_mul(10usize)
            } else {
                *floatvalue =
                    (*floatvalue as f64 * 10.0 + (*string as i32 - '0' as i32) as f32 as f64) as f32
            } //end else if
            string = string.offset(1)
        }
        *intvalue = *floatvalue as usize
    } else if subtype & 0x8 != 0 {
        while *string != 0 {
            let fresh9 = string;
            string = string.offset(1);
            *intvalue = (*intvalue)
                .wrapping_mul(10usize)
                .wrapping_add((*fresh9 as i32 - '0' as i32) as usize)
        }
        *floatvalue = *intvalue as f32
    } else if subtype & 0x100 != 0 {
        //step over the leading 0x or 0X
        string = string.offset(2); //end while
        while *string != 0 {
            *intvalue <<= 4; //end else if
            if *string as i32 >= 'a' as i32 && *string as i32 <= 'f' as i32 {
                *intvalue = (*intvalue).wrapping_add((*string as i32 - 'a' as i32 + 10) as usize)
            } else if *string as i32 >= 'A' as i32 && *string as i32 <= 'F' as i32 {
                *intvalue = (*intvalue).wrapping_add((*string as i32 - 'A' as i32 + 10) as usize)
            } else {
                *intvalue = (*intvalue).wrapping_add((*string as i32 - '0' as i32) as usize)
            }
            string = string.offset(1)
        }
        *floatvalue = *intvalue as f32
    } else if subtype & 0x200 != 0 {
        //step over the first zero
        string = string.offset(1);
        while *string != 0 {
            let fresh10 = string;
            string = string.offset(1);
            *intvalue = (*intvalue << 3).wrapping_add((*fresh10 as i32 - '0' as i32) as usize)
        }
        *floatvalue = *intvalue as f32
    } else if subtype & 0x400 != 0 {
        //step over the leading 0b or 0B
        string = string.offset(2);
        while *string != 0 {
            let fresh11 = string;
            string = string.offset(1);
            *intvalue = (*intvalue << 1).wrapping_add((*fresh11 as i32 - '0' as i32) as usize)
        }
        *floatvalue = *intvalue as f32
    };
    //end else if
}
//end of the function NumberValue
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ReadNumber(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut octal: i32 = 0;
    let mut dot: i32 = 0;
    let mut c: i8 = 0;
    //	unsigned long int intvalue = 0;
    //	double floatvalue = 0;
    (*token).type_0 = 3;
    //check for a hexadecimal number
    if *(*script).script_p as i32 == '0' as i32
        && (*(*script).script_p.offset(1) as i32 == 'x' as i32
            || *(*script).script_p.offset(1) as i32 == 'X' as i32)
    {
        //end else
        let fresh12 = (*script).script_p; //end if
        (*script).script_p = (*script).script_p.offset(1);
        let fresh13 = len;
        len = len + 1;
        (*token).string[fresh13 as usize] = *fresh12;
        let fresh14 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        let fresh15 = len;
        len = len + 1;
        (*token).string[fresh15 as usize] = *fresh14;
        c = *(*script).script_p;
        //hexadecimal
        while c as i32 >= '0' as i32 && c as i32 <= '9' as i32
            || c as i32 >= 'a' as i32 && c as i32 <= 'f' as i32
            || c as i32 >= 'A' as i32 && c as i32 <= 'A' as i32
        {
            let fresh16 = (*script).script_p; //end while
            (*script).script_p = (*script).script_p.offset(1); //end if
            let fresh17 = len;
            len = len + 1;
            (*token).string[fresh17 as usize] = *fresh16;
            if len >= 1024 {
                ScriptError(
                    script,
                    b"hexadecimal number longer than MAX_TOKEN = %d\x00" as *const u8 as *mut i8,
                    1024i32,
                );
                return 0i32;
            }
            c = *(*script).script_p
        }
        (*token).subtype |= 0x100
    } else if *(*script).script_p as i32 == '0' as i32
        && (*(*script).script_p.offset(1) as i32 == 'b' as i32
            || *(*script).script_p.offset(1) as i32 == 'B' as i32)
    {
        //check for a binary number
        let fresh18 = (*script).script_p; //end if
        (*script).script_p = (*script).script_p.offset(1);
        let fresh19 = len;
        len = len + 1;
        (*token).string[fresh19 as usize] = *fresh18;
        let fresh20 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        let fresh21 = len;
        len = len + 1;
        (*token).string[fresh21 as usize] = *fresh20;
        c = *(*script).script_p;
        //binary
        while c as i32 == '0' as i32 || c as i32 == '1' as i32 {
            let fresh22 = (*script).script_p; //end while
            (*script).script_p = (*script).script_p.offset(1); //end if
            let fresh23 = len;
            len = len + 1;
            (*token).string[fresh23 as usize] = *fresh22;
            if len >= 1024 {
                ScriptError(
                    script,
                    b"binary number longer than MAX_TOKEN = %d\x00" as *const u8 as *mut i8,
                    1024i32,
                );
                return 0i32;
            }
            c = *(*script).script_p
        }
        (*token).subtype |= 0x400
    } else {
        //BINARYNUMBERS
        //decimal or octal integer or floating point number
        octal = crate::src::qcommon::q_shared::qfalse as i32; //end while
        dot = crate::src::qcommon::q_shared::qfalse as i32; //end for
        if *(*script).script_p as i32 == '0' as i32 {
            octal = crate::src::qcommon::q_shared::qtrue as i32
        }
        loop {
            c = *(*script).script_p;
            if c as i32 == '.' as i32 {
                dot = crate::src::qcommon::q_shared::qtrue as i32
            } else if c as i32 == '8' as i32 || c as i32 == '9' as i32 {
                octal = crate::src::qcommon::q_shared::qfalse as i32
            } else if (c as i32) < '0' as i32 || c as i32 > '9' as i32 {
                break;
            }
            let fresh24 = (*script).script_p;
            (*script).script_p = (*script).script_p.offset(1);
            let fresh25 = len;
            len = len + 1;
            (*token).string[fresh25 as usize] = *fresh24;
            if len >= 1024 - 1 {
                ScriptError(
                    script,
                    b"number longer than MAX_TOKEN = %d\x00" as *const u8 as *mut i8,
                    1024i32,
                );
                return 0i32;
            }
        }
        if octal != 0 {
            (*token).subtype |= 0x200
        } else {
            (*token).subtype |= 0x8
        }
        if dot != 0 {
            (*token).subtype |= 0x800
        }
    }
    i = 0;
    while i < 2 {
        c = *(*script).script_p;
        //end if
        if (c as i32 == 'l' as i32 || c as i32 == 'L' as i32) && (*token).subtype & 0x2000 == 0 {
            //check for a LONG number
            (*script).script_p = (*script).script_p.offset(1); //end if
            (*token).subtype |= 0x2000
        } else if (c as i32 == 'u' as i32 || c as i32 == 'U' as i32)
            && (*token).subtype & (0x4000 | 0x800) == 0
        {
            (*script).script_p = (*script).script_p.offset(1);
            (*token).subtype |= 0x4000
        }
        i += 1
    }
    (*token).string[len as usize] = '\u{0}' as i8;
    NumberValue(
        (*token).string.as_mut_ptr(),
        (*token).subtype,
        &mut (*token).intvalue,
        &mut (*token).floatvalue,
    );
    //check for an UNSIGNED number
    //NUMBERVALUE
    if (*token).subtype & 0x800 == 0 {
        (*token).subtype |= 0x1000
    }
    return 1;
}
//end of the function PS_ReadNumber
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ReadLiteral(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    (*token).type_0 = 2;
    //first quote
    let fresh26 = (*script).script_p;
    (*script).script_p = (*script).script_p.offset(1);
    (*token).string[0] = *fresh26;
    //check for end of file
    if *(*script).script_p == 0 {
        ScriptError(
            script,
            b"end of file before trailing \'\x00" as *const u8 as *mut i8,
        ); //end if
        return 0i32;
    }
    //if it is an escape character
    if *(*script).script_p as i32 == '\\' as i32 {
        //end else
        if PS_ReadEscapeCharacter(script, &mut *(*token).string.as_mut_ptr().offset(1)) == 0 {
            return 0i32;
        }
    } else {
        let fresh27 = (*script).script_p; //end if
        (*script).script_p = (*script).script_p.offset(1);
        (*token).string[1] = *fresh27
    }
    //check for trailing quote
    if *(*script).script_p as i32 != '\'' as i32 {
        ScriptWarning(
            script,
            b"too many characters in literal, ignored\x00" as *const u8 as *mut i8,
        ); //end if
        while *(*script).script_p as i32 != 0
            && *(*script).script_p as i32 != '\'' as i32
            && *(*script).script_p as i32 != '\n' as i32
        {
            (*script).script_p = (*script).script_p.offset(1)
        } //end while
        if *(*script).script_p as i32 == '\'' as i32 {
            (*script).script_p = (*script).script_p.offset(1)
        }
    }
    //store the trailing quote
    let fresh28 = (*script).script_p;
    (*script).script_p = (*script).script_p.offset(1);
    (*token).string[2] = *fresh28;
    //store trailing zero to end the string
    (*token).string[3] = '\u{0}' as i8;
    //the sub type is the integer literal value
    (*token).subtype = (*token).string[1] as i32;
    //
    return 1;
}
//end of the function PS_ReadLiteral
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ReadPunctuation(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut len: i32 = 0; //end for
    let mut p: *mut i8 = 0 as *mut i8;
    let mut punc: *mut crate::src::botlib::l_script::punctuation_t =
        0 as *mut crate::src::botlib::l_script::punctuation_t;
    punc = *(*script)
        .punctuationtable
        .offset(*(*script).script_p as u32 as isize);
    while !punc.is_null() {
        //PUNCTABLE
        p = (*punc).p;
        len = crate::stdlib::strlen(p) as i32;
        //end if
        if (*script).script_p.offset(len as isize) <= (*script).end_p {
            //if the script contains at least as much characters as the punctuation
            //if the script contains the punctuation
            if crate::stdlib::strncmp((*script).script_p, p, len as usize) == 0 {
                crate::src::qcommon::q_shared::Q_strncpyz((*token).string.as_mut_ptr(), p, 1024);
                (*script).script_p = (*script).script_p.offset(len as isize);
                (*token).type_0 = 5;
                //sub type is the number of the punctuation
                (*token).subtype = (*punc).n;
                return 1i32;
            }
            //end if
        }
        punc = (*punc).next
    }
    return 0;
}
//end of the function PS_ReadPunctuation
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ReadPrimitive(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut len: i32 = 0; //end while
    len = 0; //end if
    while *(*script).script_p as i32 > ' ' as i32 && *(*script).script_p as i32 != ';' as i32 {
        if len >= 1024 - 1 {
            ScriptError(
                script,
                b"primitive token longer than MAX_TOKEN = %d\x00" as *const u8 as *mut i8,
                1024i32,
            );
            return 0i32;
        }
        let fresh29 = (*script).script_p;
        (*script).script_p = (*script).script_p.offset(1);
        let fresh30 = len;
        len = len + 1;
        (*token).string[fresh30 as usize] = *fresh29
    }
    (*token).string[len as usize] = 0i8;
    //copy the token into the script structure
    crate::stdlib::memcpy(
        &mut (*script).token as *mut crate::src::botlib::l_script::token_t as *mut libc::c_void,
        token as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
    );
    //primitive reading successful
    return 1;
}
//read a token from the script
//end of the function PS_ReadPrimitive
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ReadToken(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    //if there is a token available (from UnreadToken)
    if (*script).tokenavailable != 0 {
        (*script).tokenavailable = 0; //end if
        crate::stdlib::memcpy(
            token as *mut libc::c_void,
            &mut (*script).token as *mut crate::src::botlib::l_script::token_t
                as *const libc::c_void,
            ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
        );
        return 1i32;
    }
    //save script pointer
    (*script).lastscript_p = (*script).script_p;
    //save line counter
    (*script).lastline = (*script).line;
    //clear the token stuff
    crate::stdlib::memset(
        token as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
    );
    //start of the white space
    (*script).whitespace_p = (*script).script_p;
    (*token).whitespace_p = (*script).script_p;
    //read unusefull stuff
    if PS_ReadWhiteSpace(script) == 0 {
        return 0i32;
    }
    //end of the white space
    (*script).endwhitespace_p = (*script).script_p;
    (*token).endwhitespace_p = (*script).script_p;
    //line the token is on
    (*token).line = (*script).line;
    //number of lines crossed before token
    (*token).linescrossed = (*script).line - (*script).lastline;
    //if there is a leading double quote
    if *(*script).script_p as i32 == '\"' as i32 {
        //end if
        if PS_ReadString(script, token, '\"' as i32) == 0 {
            return 0i32;
        }
    } else if *(*script).script_p as i32 == '\'' as i32 {
        //end if
        //if a literal
        if PS_ReadString(script, token, '\'' as i32) == 0 {
            return 0i32;
        }
    } else if *(*script).script_p as i32 >= '0' as i32 && *(*script).script_p as i32 <= '9' as i32
        || *(*script).script_p as i32 == '.' as i32
            && (*(*script).script_p.offset(1) as i32 >= '0' as i32
                && *(*script).script_p.offset(1) as i32 <= '9' as i32)
    {
        //end if
        //if (!PS_ReadLiteral(script, token)) return 0;
        //if there is a number
        if PS_ReadNumber(script, token) == 0 {
            return 0i32;
        }
    } else if (*script).flags & 0x10 != 0 {
        //end if
        //if this is a primitive script
        return PS_ReadPrimitive(script, token);
    } else {
        //end else if
        //if there is a name
        if *(*script).script_p as i32 >= 'a' as i32 && *(*script).script_p as i32 <= 'z' as i32
            || *(*script).script_p as i32 >= 'A' as i32 && *(*script).script_p as i32 <= 'Z' as i32
            || *(*script).script_p as i32 == '_' as i32
        {
            if PS_ReadName(script, token) == 0 {
                return 0i32;
            }
        } else if PS_ReadPunctuation(script, token) == 0 {
            ScriptError(script, b"can\'t read token\x00" as *const u8 as *mut i8); //end if
            return 0i32;
        }
    }
    //check for punctuations
    //copy the token into the script structure
    crate::stdlib::memcpy(
        &mut (*script).token as *mut crate::src::botlib::l_script::token_t as *mut libc::c_void,
        token as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
    );
    //successfully read a token
    return 1;
}
//expect a certain token
//end of the function PS_ReadToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ExpectTokenString(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut string: *mut i8,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end if
    if PS_ReadToken(script, &mut token) == 0 {
        ScriptError(
            script,
            b"couldn\'t find expected %s\x00" as *const u8 as *mut i8,
            string,
        ); //end if
        return 0i32;
    }
    if crate::stdlib::strcmp(token.string.as_mut_ptr(), string) != 0 {
        ScriptError(
            script,
            b"expected %s, found %s\x00" as *const u8 as *mut i8,
            string,
            token.string.as_mut_ptr(),
        );
        return 0i32;
    }
    return 1;
}
//expect a certain token type
//end of the function PS_ExpectToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ExpectTokenType(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut type_0: i32,
    mut subtype: i32,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut str: [i8; 1024] = [0; 1024]; //end if
    if PS_ReadToken(script, token) == 0 {
        ScriptError(
            script,
            b"couldn\'t read expected token\x00" as *const u8 as *mut i8,
        ); //end if
        return 0i32;
    } //end else if
    if (*token).type_0 != type_0 {
        crate::stdlib::strcpy(str.as_mut_ptr(), b"\x00" as *const u8 as *const i8);
        if type_0 == 1 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"string\x00" as *const u8 as *const i8);
        }
        if type_0 == 2 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"literal\x00" as *const u8 as *const i8);
        }
        if type_0 == 3 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"number\x00" as *const u8 as *const i8);
        }
        if type_0 == 4 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"name\x00" as *const u8 as *const i8);
        }
        if type_0 == 5 {
            crate::stdlib::strcpy(
                str.as_mut_ptr(),
                b"punctuation\x00" as *const u8 as *const i8,
            );
        }
        ScriptError(
            script,
            b"expected a %s, found %s\x00" as *const u8 as *mut i8,
            str.as_mut_ptr(),
            (*token).string.as_mut_ptr(),
        );
        return 0i32;
    }
    if (*token).type_0 == 3 {
        if (*token).subtype & subtype != subtype {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"\x00" as *const u8 as *const i8);
            if subtype & 0x8 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"decimal\x00" as *const u8 as *const i8);
            }
            if subtype & 0x100 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"hex\x00" as *const u8 as *const i8);
            }
            if subtype & 0x200 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"octal\x00" as *const u8 as *const i8);
            }
            if subtype & 0x400 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"binary\x00" as *const u8 as *const i8);
            }
            if subtype & 0x2000 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" long\x00" as *const u8 as *const i8);
            }
            if subtype & 0x4000 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" unsigned\x00" as *const u8 as *const i8);
            }
            if subtype & 0x800 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" float\x00" as *const u8 as *const i8);
            }
            if subtype & 0x1000 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" integer\x00" as *const u8 as *const i8);
            }
            ScriptError(
                script,
                b"expected %s, found %s\x00" as *const u8 as *mut i8,
                str.as_mut_ptr(),
                (*token).string.as_mut_ptr(),
            );
            return 0i32;
        }
    //end if
    } else if (*token).type_0 == 5 {
        if subtype < 0 {
            ScriptError(
                script,
                b"BUG: wrong punctuation subtype\x00" as *const u8 as *mut i8,
            ); //end if
            return 0i32;
        }
        if (*token).subtype != subtype {
            ScriptError(
                script,
                b"expected %s, found %s\x00" as *const u8 as *mut i8,
                (*(*script).punctuations.offset(subtype as isize)).p,
                (*token).string.as_mut_ptr(),
            );
            return 0i32;
        }
        //end if
    }
    return 1;
}
//expect a token
//end of the function PS_ExpectTokenType
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_ExpectAnyToken(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    if PS_ReadToken(script, token) == 0 {
        ScriptError(
            script,
            b"couldn\'t read expected token\x00" as *const u8 as *mut i8,
        ); //end if
        return 0i32;
    } else {
        return 1i32;
    };
    //end else
}
//returns true when the token is available
//end of the function PS_ExpectAnyToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_CheckTokenString(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut string: *mut i8,
) -> i32 {
    let mut tok: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PS_ReadToken(script, &mut tok) == 0 {
        return 0i32;
    }
    //if the token is available
    if crate::stdlib::strcmp(tok.string.as_mut_ptr(), string) == 0 {
        return 1i32;
    }
    //token not available
    (*script).script_p = (*script).lastscript_p;
    return 0;
}
//returns true and reads the token when a token with the given type is available
//end of the function PS_CheckTokenString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_CheckTokenType(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut type_0: i32,
    mut subtype: i32,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut tok: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PS_ReadToken(script, &mut tok) == 0 {
        return 0i32;
    }
    //if the type matches
    if tok.type_0 == type_0 && tok.subtype & subtype == subtype {
        crate::stdlib::memcpy(
            token as *mut libc::c_void,
            &mut tok as *mut crate::src::botlib::l_script::token_t as *const libc::c_void,
            ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
        ); //end if
        return 1i32;
    }
    //token is not available
    (*script).script_p = (*script).lastscript_p;
    return 0;
}
//skip tokens until the given token string is read
//end of the function PS_CheckTokenType
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_SkipUntilString(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut string: *mut i8,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end while
    while PS_ReadToken(script, &mut token) != 0 {
        if crate::stdlib::strcmp(token.string.as_mut_ptr(), string) == 0 {
            return 1i32;
        }
    }
    return 0;
}
//unread the last token read from the script
//end of the function PS_SkipUntilString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_UnreadLastToken(
    mut script: *mut crate::src::botlib::l_script::script_t,
) {
    (*script).tokenavailable = 1;
}
//unread the given token
//end of the function UnreadLastToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_UnreadToken(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) {
    crate::stdlib::memcpy(
        &mut (*script).token as *mut crate::src::botlib::l_script::token_t as *mut libc::c_void,
        token as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
    );
    (*script).tokenavailable = 1;
}
//returns the next character of the read white space, returns NULL if none
//end of the function UnreadToken
//============================================================================
// returns the next character of the read white space, returns NULL if none
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_NextWhiteSpaceChar(
    mut script: *mut crate::src::botlib::l_script::script_t,
) -> i8 {
    if (*script).whitespace_p != (*script).endwhitespace_p {
        let fresh31 = (*script).whitespace_p; //end if
        (*script).whitespace_p = (*script).whitespace_p.offset(1);
        return *fresh31;
    } else {
        return 0i8;
    };
    //end else
}
//remove any leading and trailing double quotes from the token
//end of the function PS_NextWhiteSpaceChar
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn StripDoubleQuotes(mut string: *mut i8) {
    if *string as i32 == '\"' as i32 {
        crate::stdlib::memmove(
            string as *mut libc::c_void,
            string.offset(1isize) as *const libc::c_void,
            crate::stdlib::strlen(string),
        ); //end if
    }
    if *string.offset(crate::stdlib::strlen(string).wrapping_sub(1usize) as isize) as i32
        == '\"' as i32
    {
        *string.offset(crate::stdlib::strlen(string).wrapping_sub(1usize) as isize) = '\u{0}' as i8
    };
    //end if
}
//remove any leading and trailing single quotes from the token
//end of the function StripDoubleQuotes
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn StripSingleQuotes(mut string: *mut i8) {
    if *string as i32 == '\'' as i32 {
        crate::stdlib::memmove(
            string as *mut libc::c_void,
            string.offset(1isize) as *const libc::c_void,
            crate::stdlib::strlen(string),
        ); //end if
    }
    if *string.offset(crate::stdlib::strlen(string).wrapping_sub(1usize) as isize) as i32
        == '\'' as i32
    {
        *string.offset(crate::stdlib::strlen(string).wrapping_sub(1usize) as isize) = '\u{0}' as i8
    };
    //end if
}
//read a possible signed floating point number
//end of the function StripSingleQuotes
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadSignedFloat(
    mut script: *mut crate::src::botlib::l_script::script_t,
) -> f32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut sign: f32 = 1f32;
    PS_ExpectAnyToken(script, &mut token);
    if crate::stdlib::strcmp(
        token.string.as_mut_ptr(),
        b"-\x00" as *const u8 as *const i8,
    ) == 0
    {
        if PS_ExpectAnyToken(script, &mut token) == 0 {
            ScriptError(script, b"Missing float value\x00" as *const u8 as *mut i8);
            return 0f32;
        }
        sign = -1f32
    }
    if token.type_0 != 3 {
        ScriptError(
            script,
            b"expected float value, found %s\x00" as *const u8 as *mut i8,
            token.string.as_mut_ptr(),
        );
        return 0f32;
    }
    return sign * token.floatvalue;
}
//read a possible signed integer
//end of the function ReadSignedFloat
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadSignedInt(
    mut script: *mut crate::src::botlib::l_script::script_t,
) -> isize {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut sign: isize = 1;
    PS_ExpectAnyToken(script, &mut token);
    if crate::stdlib::strcmp(
        token.string.as_mut_ptr(),
        b"-\x00" as *const u8 as *const i8,
    ) == 0
    {
        if PS_ExpectAnyToken(script, &mut token) == 0 {
            ScriptError(script, b"Missing integer value\x00" as *const u8 as *mut i8);
            return 0isize;
        }
        sign = -1
    }
    if token.type_0 != 3 || token.subtype == 0x800 {
        ScriptError(
            script,
            b"expected integer value, found %s\x00" as *const u8 as *mut i8,
            token.string.as_mut_ptr(),
        );
        return 0isize;
    }
    return (sign as usize).wrapping_mul(token.intvalue) as isize;
}
//set script flags
//end of the function ReadSignedInt
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn SetScriptFlags(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut flags: i32,
) {
    (*script).flags = flags;
}
//get script flags
//end of the function SetScriptFlags
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn GetScriptFlags(
    mut script: *mut crate::src::botlib::l_script::script_t,
) -> i32 {
    return (*script).flags;
}
//reset a script
//end of the function GetScriptFlags
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn ResetScript(mut script: *mut crate::src::botlib::l_script::script_t) {
    //pointer in script buffer
    (*script).script_p = (*script).buffer;
    //pointer in script buffer before reading token
    (*script).lastscript_p = (*script).buffer;
    //begin of white space
    (*script).whitespace_p = 0 as *mut i8;
    //end of white space
    (*script).endwhitespace_p = 0 as *mut i8;
    //set if there's a token available in script->token
    (*script).tokenavailable = 0;
    //
    (*script).line = 1;
    (*script).lastline = 1;
    //clear the saved token
    crate::stdlib::memset(
        &mut (*script).token as *mut crate::src::botlib::l_script::token_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
    );
}
//returns true if at the end of the script
//end of the function ResetScript
//============================================================================
// returns true if at the end of the script
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn EndOfScript(
    mut script: *mut crate::src::botlib::l_script::script_t,
) -> i32 {
    return ((*script).script_p >= (*script).end_p) as i32;
}
//end of the function EndOfScript
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn NumLinesCrossed(
    mut script: *mut crate::src::botlib::l_script::script_t,
) -> i32 {
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

pub unsafe extern "C" fn ScriptSkipTo(
    mut script: *mut crate::src::botlib::l_script::script_t,
    mut value: *mut i8,
) -> i32 {
    let mut len: i32 = 0; //end if
    let mut firstchar: i8 = 0;
    firstchar = *value;
    len = crate::stdlib::strlen(value) as i32;
    loop {
        if PS_ReadWhiteSpace(script) == 0 {
            return 0i32;
        }
        if *(*script).script_p as i32 == firstchar as i32 {
            if crate::stdlib::strncmp((*script).script_p, value, len as usize) == 0 {
                return 1i32;
            }
            //end if
        }
        (*script).script_p = (*script).script_p.offset(1)
    }
}
//load a script from the given file at the given offset with the given length
//end of the function ScriptSkipTo
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadScriptFile(
    mut filename: *const i8,
) -> *mut crate::src::botlib::l_script::script_t {
    let mut fp: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut pathname: [i8; 64] = [0; 64];
    let mut length: i32 = 0;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    if crate::stdlib::strlen(basefolder.as_mut_ptr()) != 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            pathname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"%s/%s\x00" as *const u8 as *const i8,
            basefolder.as_mut_ptr(),
            filename,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            pathname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"%s\x00" as *const u8 as *const i8,
            filename,
        );
    }
    length = crate::src::botlib::be_interface::botimport
        .FS_FOpenFile
        .expect("non-null function pointer")(
        pathname.as_mut_ptr(),
        &mut fp,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if fp == 0 {
        return 0 as *mut crate::src::botlib::l_script::script_t;
    }
    buffer = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<crate::src::botlib::l_script::script_t>())
            .wrapping_add(length as usize)
            .wrapping_add(1usize),
    );
    script = buffer as *mut crate::src::botlib::l_script::script_t;
    crate::stdlib::memset(
        script as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_script::script_t>(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*script).filename.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    (*script).buffer = (buffer as *mut i8)
        .offset(::std::mem::size_of::<crate::src::botlib::l_script::script_t>() as isize);
    *(*script).buffer.offset(length as isize) = 0i8;
    (*script).length = length;
    //pointer in script buffer
    (*script).script_p = (*script).buffer;
    //pointer in script buffer before reading token
    (*script).lastscript_p = (*script).buffer;
    //pointer to end of script buffer
    (*script).end_p = &mut *(*script).buffer.offset(length as isize) as *mut i8;
    //set if there's a token available in script->token
    (*script).tokenavailable = 0;
    //
    (*script).line = 1;
    (*script).lastline = 1;
    //
    SetScriptPunctuations(
        script,
        0 as *mut crate::src::botlib::l_script::punctuation_t,
    );
    //
    crate::src::botlib::be_interface::botimport
        .FS_Read
        .expect("non-null function pointer")((*script).buffer as *mut libc::c_void, length, fp);
    crate::src::botlib::be_interface::botimport
        .FS_FCloseFile
        .expect("non-null function pointer")(fp);
    return script;
}
//load a script from the given memory with the given length
//end of the function LoadScriptFile
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadScriptMemory(
    mut ptr: *mut i8,
    mut length: i32,
    mut name: *mut i8,
) -> *mut crate::src::botlib::l_script::script_t {
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    buffer = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<crate::src::botlib::l_script::script_t>())
            .wrapping_add(length as usize)
            .wrapping_add(1usize),
    );
    script = buffer as *mut crate::src::botlib::l_script::script_t;
    crate::stdlib::memset(
        script as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_script::script_t>(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*script).filename.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    (*script).buffer = (buffer as *mut i8)
        .offset(::std::mem::size_of::<crate::src::botlib::l_script::script_t>() as isize);
    *(*script).buffer.offset(length as isize) = 0i8;
    (*script).length = length;
    //pointer in script buffer
    (*script).script_p = (*script).buffer;
    //pointer in script buffer before reading token
    (*script).lastscript_p = (*script).buffer;
    //pointer to end of script buffer
    (*script).end_p = &mut *(*script).buffer.offset(length as isize) as *mut i8;
    //set if there's a token available in script->token
    (*script).tokenavailable = 0;
    //
    (*script).line = 1;
    (*script).lastline = 1;
    //
    SetScriptPunctuations(
        script,
        0 as *mut crate::src::botlib::l_script::punctuation_t,
    );
    //
    crate::stdlib::memcpy(
        (*script).buffer as *mut libc::c_void,
        ptr as *const libc::c_void,
        length as usize,
    );
    //
    return script;
}
//free a script
//end of the function LoadScriptMemory
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeScript(mut script: *mut crate::src::botlib::l_script::script_t) {
    if !(*script).punctuationtable.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*script).punctuationtable as *mut libc::c_void);
    }
    //PUNCTABLE
    crate::src::botlib::l_memory::FreeMemory(script as *mut libc::c_void);
}
//set the base folder to load files from
//end of the function FreeScript
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PS_SetBaseFolder(mut path: *mut i8) {
    crate::src::qcommon::q_shared::Com_sprintf(
        basefolder.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"%s\x00" as *const u8 as *const i8,
        path,
    );
}
//end of the function PS_SetBaseFolder
