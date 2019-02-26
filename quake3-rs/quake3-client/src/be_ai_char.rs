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
        //read a token from the source
        #[no_mangle]
        pub fn PC_ReadToken(source: *mut source_t, token: *mut token_t)
         -> libc::c_int;
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
        //set the base folder to load files from
        #[no_mangle]
        pub fn PC_SetBaseFolder(path: *mut libc::c_char);
        //load a source file
        #[no_mangle]
        pub fn LoadSourceFile(filename: *const libc::c_char) -> *mut source_t;
        //free the given source
        #[no_mangle]
        pub fn FreeSource(source: *mut source_t);
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
      "ioq3/code/botlib/be_ai_char.c"]
pub mod be_ai_char_c {
    pub type bot_character_t = bot_character_s;
    //a bot character
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_character_s {
        pub filename: [libc::c_char; 64],
        pub skill: libc::c_float,
        pub c: [bot_characteristic_t; 0],
    }
    pub type bot_characteristic_t = bot_characteristic_s;
    //a characteristic
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_characteristic_s {
        pub type_0: libc::c_char,
        pub value: cvalue,
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
 * name:		be_ai_char.c
 *
 * desc:		bot characters
 *
 * $Archive: /MissionPack/code/botlib/be_ai_char.c $
 *
 *****************************************************************************/
    //characteristic value
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union cvalue {
        pub integer: libc::c_int,
        pub _float: libc::c_float,
        pub string: *mut libc::c_char,
    }
    use super::{libc};
}
#[header_src = "/usr/include/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
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
      "ioq3/code/botlib/l_log.h"]
pub mod l_log_h {
    use super::{libc};
    extern "C" {
        //write to the current opened log file
        #[no_mangle]
        pub fn Log_Write(fmt: *mut libc::c_char, ...);
    }
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
#[header_src =
      "ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //gets the value of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetValue(var_name: *const libc::c_char) -> libc::c_float;
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
      "ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src =
      "ioq3/code/botlib/be_ai_char.h"]
pub mod be_ai_char_h {
    use super::{libc};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t, StripDoubleQuotes};
use self::l_precomp_h::{define_s, define_t, indent_s, indent_t, source_s,
                        source_t, PC_ReadToken, PC_ExpectTokenString,
                        PC_ExpectTokenType, PC_ExpectAnyToken,
                        PC_SetBaseFolder, LoadSourceFile, FreeSource};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_ai_char_c::{bot_character_t, bot_character_s,
                         bot_characteristic_t, bot_characteristic_s, cvalue};
use self::mathcalls_h::{fabs};
use self::string_h::{strcpy, strncpy, strcmp, strlen};
use self::l_log_h::{Log_Write};
use self::l_memory_h::{GetMemory, GetClearedMemory, FreeMemory};
use self::l_libvar_h::{LibVarGetValue};
use self::l_variadic_h::{SourceError};
use self::be_interface_h::{botimport};
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
/* ****************************************************************************
 * name:		be_ai_char.h
 *
 * desc:		bot characters
 *
 * $Archive: /source/code/botlib/be_ai_char.h $
 *
 *****************************************************************************/
//loads a bot character from a file
#[no_mangle]
pub unsafe extern "C" fn BotLoadCharacter(mut charfile: *mut libc::c_char,
                                          mut skill: libc::c_float)
 -> libc::c_int {
    let mut firstskill: libc::c_int = 0;
    let mut secondskill: libc::c_int = 0;
    let mut handle: libc::c_int = 0;
    if (skill as libc::c_double) < 1.0f64 {
        skill = 1.0f64 as libc::c_float
    } else if skill as libc::c_double > 5.0f64 {
        skill = 5.0f64 as libc::c_float
    }
    if skill as libc::c_double == 1.0f64 || skill as libc::c_double == 4.0f64
           || skill as libc::c_double == 5.0f64 {
        return BotLoadCharacterSkill(charfile, skill)
    }
    handle = BotFindCachedCharacter(charfile, skill);
    if 0 != handle {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"loaded cached skill %f from %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            skill as
                                                                libc::c_double,
                                                            charfile);
        return handle
    }
    if (skill as libc::c_double) < 4.0f64 {
        firstskill = BotLoadCharacterSkill(charfile, 1i32 as libc::c_float);
        if 0 == firstskill { return 0i32 }
        secondskill = BotLoadCharacterSkill(charfile, 4i32 as libc::c_float);
        if 0 == secondskill { return firstskill }
    } else {
        firstskill = BotLoadCharacterSkill(charfile, 4i32 as libc::c_float);
        if 0 == firstskill { return 0i32 }
        secondskill = BotLoadCharacterSkill(charfile, 5i32 as libc::c_float);
        if 0 == secondskill { return firstskill }
    }
    handle = BotInterpolateCharacters(firstskill, secondskill, skill);
    if 0 == handle { return 0i32 }
    BotDumpCharacter(botcharacters[handle as usize]);
    return handle;
}
#[no_mangle]
pub static mut botcharacters: [*mut bot_character_t; 65] =
    [0 as *const bot_character_t as *mut bot_character_t; 65];
//end of the function BotCharacterFromHandle
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotDumpCharacter(mut ch: *mut bot_character_t) {
    let mut i: libc::c_int = 0;
    Log_Write(b"%s\n\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, (*ch).filename.as_mut_ptr());
    Log_Write(b"skill %.1f\n\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char, (*ch).skill as libc::c_double);
    Log_Write(b"{\n\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char);
    i = 0i32;
    while i < 80i32 {
        match (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 as libc::c_int
            {
            1 => {
                Log_Write(b" %4d %d\n\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char, i,
                          (*(*ch).c.as_mut_ptr().offset(i as
                                                            isize)).value.integer);
            }
            2 => {
                Log_Write(b" %4d %f\n\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char, i,
                          (*(*ch).c.as_mut_ptr().offset(i as
                                                            isize)).value._float
                              as libc::c_double);
            }
            3 => {
                Log_Write(b" %4d %s\n\x00" as *const u8 as *const libc::c_char
                              as *mut libc::c_char, i,
                          (*(*ch).c.as_mut_ptr().offset(i as
                                                            isize)).value.string);
            }
            _ => { }
        }
        i += 1
    }
    Log_Write(b"}\n\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char);
}
//end of the function BotLoadCharacterSkill
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotInterpolateCharacters(mut handle1: libc::c_int,
                                                  mut handle2: libc::c_int,
                                                  mut desiredskill:
                                                      libc::c_float)
 -> libc::c_int {
    let mut ch1: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut ch2: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut out: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut i: libc::c_int = 0;
    let mut handle: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    ch1 = BotCharacterFromHandle(handle1);
    ch2 = BotCharacterFromHandle(handle2);
    if ch1.is_null() || ch2.is_null() { return 0i32 }
    handle = 1i32;
    while handle <= 64i32 {
        if botcharacters[handle as usize].is_null() { break ; }
        handle += 1
    }
    if handle > 64i32 { return 0i32 }
    out =
        GetClearedMemory((::std::mem::size_of::<bot_character_t>() as
                              libc::c_ulong).wrapping_add((80i32 as
                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<bot_characteristic_t>()
                                                                                               as
                                                                                               libc::c_ulong)))
            as *mut bot_character_t;
    (*out).skill = desiredskill;
    strcpy((*out).filename.as_mut_ptr(), (*ch1).filename.as_mut_ptr());
    botcharacters[handle as usize] = out;
    scale = (desiredskill - (*ch1).skill) / ((*ch2).skill - (*ch1).skill);
    i = 0i32;
    while i < 80i32 {
        if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as libc::c_int
               == 2i32 &&
               (*(*ch2).c.as_mut_ptr().offset(i as isize)).type_0 as
                   libc::c_int == 2i32 {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 =
                2i32 as libc::c_char;
            (*(*out).c.as_mut_ptr().offset(i as isize)).value._float =
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value._float +
                    ((*(*ch2).c.as_mut_ptr().offset(i as isize)).value._float
                         -
                         (*(*ch1).c.as_mut_ptr().offset(i as
                                                            isize)).value._float)
                        * scale
        } else if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as
                      libc::c_int == 1i32 {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 =
                1i32 as libc::c_char;
            (*(*out).c.as_mut_ptr().offset(i as isize)).value.integer =
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value.integer
        } else if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as
                      libc::c_int == 3i32 {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 =
                3i32 as libc::c_char;
            let ref mut fresh0 =
                (*(*out).c.as_mut_ptr().offset(i as isize)).value.string;
            *fresh0 =
                GetMemory(strlen((*(*ch1).c.as_mut_ptr().offset(i as
                                                                    isize)).value.string).wrapping_add(1i32
                                                                                                           as
                                                                                                           libc::c_ulong))
                    as *mut libc::c_char;
            strcpy((*(*out).c.as_mut_ptr().offset(i as isize)).value.string,
                   (*(*ch1).c.as_mut_ptr().offset(i as isize)).value.string);
        }
        i += 1
    }
    return handle;
}
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotCharacterFromHandle(mut handle: libc::c_int)
 -> *mut bot_character_t {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"character handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_character_t
    }
    if botcharacters[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid character %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_character_t
    }
    return botcharacters[handle as usize];
}
//end of the function BotLoadCachedCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadCharacterSkill(mut charfile:
                                                   *mut libc::c_char,
                                               mut skill: libc::c_float)
 -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut defaultch: libc::c_int = 0;
    defaultch =
        BotLoadCachedCharacter(b"bots/default_c.c\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               skill, qfalse as libc::c_int);
    ch =
        BotLoadCachedCharacter(charfile, skill,
                               LibVarGetValue(b"bot_reloadcharacters\x00" as
                                                  *const u8 as
                                                  *const libc::c_char) as
                                   libc::c_int);
    if 0 != defaultch && 0 != ch {
        BotDefaultCharacteristics(botcharacters[ch as usize],
                                  botcharacters[defaultch as usize]);
    }
    return ch;
}
//end of the function BotFreeCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotDefaultCharacteristics(mut ch:
                                                       *mut bot_character_t,
                                                   mut defaultch:
                                                       *mut bot_character_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 80i32 {
        if !(0 != (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0) {
            if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0 as
                   libc::c_int == 2i32 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 =
                    2i32 as libc::c_char;
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value._float =
                    (*(*defaultch).c.as_mut_ptr().offset(i as
                                                             isize)).value._float
            } else if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0
                          as libc::c_int == 1i32 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 =
                    1i32 as libc::c_char;
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value.integer =
                    (*(*defaultch).c.as_mut_ptr().offset(i as
                                                             isize)).value.integer
            } else if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0
                          as libc::c_int == 3i32 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 =
                    3i32 as libc::c_char;
                let ref mut fresh1 =
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string;
                *fresh1 =
                    GetMemory(strlen((*(*defaultch).c.as_mut_ptr().offset(i as
                                                                              isize)).value.string).wrapping_add(1i32
                                                                                                                     as
                                                                                                                     libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*(*ch).c.as_mut_ptr().offset(i as
                                                         isize)).value.string,
                       (*(*defaultch).c.as_mut_ptr().offset(i as
                                                                isize)).value.string);
            }
        }
        i += 1
    };
}
//end of the function BotFindCachedCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadCachedCharacter(mut charfile:
                                                    *mut libc::c_char,
                                                mut skill: libc::c_float,
                                                mut reload: libc::c_int)
 -> libc::c_int {
    let mut handle: libc::c_int = 0;
    let mut cachedhandle: libc::c_int = 0;
    let mut intskill: libc::c_int = 0;
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    handle = 1i32;
    while handle <= 64i32 {
        if botcharacters[handle as usize].is_null() { break ; }
        handle += 1
    }
    if handle > 64i32 { return 0i32 }
    if 0 == reload {
        cachedhandle = BotFindCachedCharacter(charfile, skill);
        if 0 != cachedhandle {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"loaded cached skill %f from %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                skill as
                                                                    libc::c_double,
                                                                charfile);
            return cachedhandle
        }
    }
    intskill = (skill as libc::c_double + 0.5f64) as libc::c_int;
    ch = BotLoadCharacterFromFile(charfile, intskill);
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"loaded skill %d from %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            intskill,
                                                            charfile);
        return handle
    }
    botimport.Print.expect("non-null function pointer")(2i32,
                                                        b"couldn\'t find skill %d in %s\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        intskill, charfile);
    if 0 == reload {
        cachedhandle =
            BotFindCachedCharacter(b"bots/default_c.c\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char, skill);
        if 0 != cachedhandle {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"loaded cached default skill %d from %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                intskill,
                                                                charfile);
            return cachedhandle
        }
    }
    ch =
        BotLoadCharacterFromFile(b"bots/default_c.c\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 intskill);
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"loaded default skill %d from %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            intskill,
                                                            charfile);
        return handle
    }
    if 0 == reload {
        cachedhandle =
            BotFindCachedCharacter(charfile, -1i32 as libc::c_float);
        if 0 != cachedhandle {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"loaded cached skill %f from %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                (*botcharacters[cachedhandle
                                                                                    as
                                                                                    usize]).skill
                                                                    as
                                                                    libc::c_double,
                                                                charfile);
            return cachedhandle
        }
    }
    ch = BotLoadCharacterFromFile(charfile, -1i32);
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"loaded skill %f from %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            (*ch).skill as
                                                                libc::c_double,
                                                            charfile);
        return handle
    }
    if 0 == reload {
        cachedhandle =
            BotFindCachedCharacter(b"bots/default_c.c\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char,
                                   -1i32 as libc::c_float);
        if 0 != cachedhandle {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"loaded cached default skill %f from %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                (*botcharacters[cachedhandle
                                                                                    as
                                                                                    usize]).skill
                                                                    as
                                                                    libc::c_double,
                                                                charfile);
            return cachedhandle
        }
    }
    ch =
        BotLoadCharacterFromFile(b"bots/default_c.c\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                                 -1i32);
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"loaded default skill %f from %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            (*ch).skill as
                                                                libc::c_double,
                                                            charfile);
        return handle
    }
    botimport.Print.expect("non-null function pointer")(2i32,
                                                        b"couldn\'t load any skill from %s\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        charfile);
    return 0i32;
}
//end else if
//end for
//end of the function BotDefaultCharacteristics
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadCharacterFromFile(mut charfile:
                                                      *mut libc::c_char,
                                                  mut skill: libc::c_int)
 -> *mut bot_character_t {
    let mut indent: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut foundcharacter: libc::c_int = 0;
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut source: *mut source_t = 0 as *mut source_t;
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
    foundcharacter = qfalse as libc::c_int;
    PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char);
    source = LoadSourceFile(charfile);
    if source.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"counldn\'t load %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            charfile);
        return 0 as *mut bot_character_t
    }
    ch =
        GetClearedMemory((::std::mem::size_of::<bot_character_t>() as
                              libc::c_ulong).wrapping_add((80i32 as
                                                               libc::c_ulong).wrapping_mul(::std::mem::size_of::<bot_characteristic_t>()
                                                                                               as
                                                                                               libc::c_ulong)))
            as *mut bot_character_t;
    strcpy((*ch).filename.as_mut_ptr(), charfile);
    while 0 != PC_ReadToken(source, &mut token) {
        if 0 ==
               strcmp(token.string.as_mut_ptr(),
                      b"skill\x00" as *const u8 as *const libc::c_char) {
            if 0 == PC_ExpectTokenType(source, 3i32, 0i32, &mut token) {
                FreeSource(source);
                BotFreeCharacterStrings(ch);
                FreeMemory(ch as *mut libc::c_void);
                return 0 as *mut bot_character_t
            }
            if 0 ==
                   PC_ExpectTokenString(source,
                                        b"{\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) {
                FreeSource(source);
                BotFreeCharacterStrings(ch);
                FreeMemory(ch as *mut libc::c_void);
                return 0 as *mut bot_character_t
            }
            //end if
            //if it's the correct skill
            if skill < 0i32 || token.intvalue == skill as libc::c_ulong {
                foundcharacter = qtrue as libc::c_int;
                (*ch).skill = token.intvalue as libc::c_float;
                while 0 != PC_ExpectAnyToken(source, &mut token) {
                    if 0 ==
                           strcmp(token.string.as_mut_ptr(),
                                  b"}\x00" as *const u8 as
                                      *const libc::c_char) {
                        break ;
                    }
                    if token.type_0 != 3i32 || 0 == token.subtype & 0x1000i32
                       {
                        SourceError(source,
                                    b"expected integer index, found %s\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char,
                                    token.string.as_mut_ptr());
                        FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t
                    }
                    index = token.intvalue as libc::c_int;
                    if index < 0i32 || index > 80i32 {
                        SourceError(source,
                                    b"characteristic index out of range [0, %d]\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, 80i32);
                        FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t
                    }
                    if 0 !=
                           (*(*ch).c.as_mut_ptr().offset(index as
                                                             isize)).type_0 {
                        SourceError(source,
                                    b"characteristic %d already initialized\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char, index);
                        FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t
                    }
                    if 0 == PC_ExpectAnyToken(source, &mut token) {
                        FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t
                    }
                    if token.type_0 == 3i32 {
                        if 0 != token.subtype & 0x800i32 {
                            (*(*ch).c.as_mut_ptr().offset(index as
                                                              isize)).value._float
                                = token.floatvalue;
                            (*(*ch).c.as_mut_ptr().offset(index as
                                                              isize)).type_0 =
                                2i32 as libc::c_char
                        } else {
                            (*(*ch).c.as_mut_ptr().offset(index as
                                                              isize)).value.integer
                                = token.intvalue as libc::c_int;
                            (*(*ch).c.as_mut_ptr().offset(index as
                                                              isize)).type_0 =
                                1i32 as libc::c_char
                        }
                    } else if token.type_0 == 1i32 {
                        StripDoubleQuotes(token.string.as_mut_ptr());
                        let ref mut fresh2 =
                            (*(*ch).c.as_mut_ptr().offset(index as
                                                              isize)).value.string;
                        *fresh2 =
                            GetMemory(strlen(token.string.as_mut_ptr()).wrapping_add(1i32
                                                                                         as
                                                                                         libc::c_ulong))
                                as *mut libc::c_char;
                        strcpy((*(*ch).c.as_mut_ptr().offset(index as
                                                                 isize)).value.string,
                               token.string.as_mut_ptr());
                        (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0
                            = 3i32 as libc::c_char
                    } else {
                        SourceError(source,
                                    b"expected integer, float or string, found %s\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char,
                                    token.string.as_mut_ptr());
                        FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t
                    }
                }
                //end else
                //end if
                break ;
            } else {
                indent = 1i32;
                while 0 != indent {
                    if 0 == PC_ExpectAnyToken(source, &mut token) {
                        FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t
                    }
                    if 0 ==
                           strcmp(token.string.as_mut_ptr(),
                                  b"{\x00" as *const u8 as
                                      *const libc::c_char) {
                        indent += 1
                    } else if 0 ==
                                  strcmp(token.string.as_mut_ptr(),
                                         b"}\x00" as *const u8 as
                                             *const libc::c_char) {
                        indent -= 1
                    }
                }
            }
        } else {
            SourceError(source,
                        b"unknown definition %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            FreeSource(source);
            BotFreeCharacterStrings(ch);
            FreeMemory(ch as *mut libc::c_void);
            return 0 as *mut bot_character_t
        }
    }
    FreeSource(source);
    if 0 == foundcharacter {
        BotFreeCharacterStrings(ch);
        FreeMemory(ch as *mut libc::c_void);
        return 0 as *mut bot_character_t
    }
    return ch;
}
//end of the function BotDumpCharacter
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeCharacterStrings(mut ch:
                                                     *mut bot_character_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 80i32 {
        if (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 as libc::c_int ==
               3i32 {
            FreeMemory((*(*ch).c.as_mut_ptr().offset(i as isize)).value.string
                           as *mut libc::c_void);
        }
        i += 1
    };
}
//end of the function BotLoadCharacterFromFile
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFindCachedCharacter(mut charfile:
                                                    *mut libc::c_char,
                                                mut skill: libc::c_float)
 -> libc::c_int {
    let mut handle: libc::c_int = 0;
    handle = 1i32;
    while handle <= 64i32 {
        if !botcharacters[handle as usize].is_null() {
            if strcmp((*botcharacters[handle as usize]).filename.as_mut_ptr(),
                      charfile) == 0i32 &&
                   (skill < 0i32 as libc::c_float ||
                        fabs(((*botcharacters[handle as usize]).skill - skill)
                                 as libc::c_double) < 0.01f64) {
                return handle
            }
        }
        handle += 1
    }
    return 0i32;
}
//frees a bot character
#[no_mangle]
pub unsafe extern "C" fn BotFreeCharacter(mut handle: libc::c_int) {
    if 0. ==
           LibVarGetValue(b"bot_reloadcharacters\x00" as *const u8 as
                              *const libc::c_char) {
        return
    }
    BotFreeCharacter2(handle);
}
//end if
//end for
//end of the function BotFreeCharacterStrings
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeCharacter2(mut handle: libc::c_int) {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"character handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    if botcharacters[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid character %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    BotFreeCharacterStrings(botcharacters[handle as usize]);
    FreeMemory(botcharacters[handle as usize] as *mut libc::c_void);
    botcharacters[handle as usize] = 0 as *mut bot_character_t;
}
//returns a float characteristic
#[no_mangle]
pub unsafe extern "C" fn Characteristic_Float(mut character: libc::c_int,
                                              mut index: libc::c_int)
 -> libc::c_float {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() { return 0i32 as libc::c_float }
    if 0 == CheckCharacteristicIndex(character, index) {
        return 0i32 as libc::c_float
    }
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as libc::c_int ==
           1i32 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer as
                   libc::c_float
    } else if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as
                  libc::c_int == 2i32 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float
    } else {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"characteristic %d is not a float\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            index);
        return 0i32 as libc::c_float
    };
}
//end of the function BotLoadCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn CheckCharacteristicIndex(mut character: libc::c_int,
                                                  mut index: libc::c_int)
 -> libc::c_int {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() { return qfalse as libc::c_int }
    if index < 0i32 || index >= 80i32 {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"characteristic %d does not exist\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            index);
        return qfalse as libc::c_int
    }
    if 0 == (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"characteristic %d is not initialized\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            index);
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//returns a bounded float characteristic
#[no_mangle]
pub unsafe extern "C" fn Characteristic_BFloat(mut character: libc::c_int,
                                               mut index: libc::c_int,
                                               mut min: libc::c_float,
                                               mut max: libc::c_float)
 -> libc::c_float {
    let mut value: libc::c_float = 0.;
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() { return 0i32 as libc::c_float }
    if min > max {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"cannot bound characteristic %d between %f and %f\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            index,
                                                            min as
                                                                libc::c_double,
                                                            max as
                                                                libc::c_double);
        return 0i32 as libc::c_float
    }
    value = Characteristic_Float(character, index);
    if value < min { return min }
    if value > max { return max }
    return value;
}
//returns an integer characteristic
#[no_mangle]
pub unsafe extern "C" fn Characteristic_Integer(mut character: libc::c_int,
                                                mut index: libc::c_int)
 -> libc::c_int {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() { return 0i32 }
    if 0 == CheckCharacteristicIndex(character, index) { return 0i32 }
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as libc::c_int ==
           1i32 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer
    } else if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as
                  libc::c_int == 2i32 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float as
                   libc::c_int
    } else {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"characteristic %d is not an integer\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            index);
        return 0i32
    };
}
//returns a bounded integer characteristic
#[no_mangle]
pub unsafe extern "C" fn Characteristic_BInteger(mut character: libc::c_int,
                                                 mut index: libc::c_int,
                                                 mut min: libc::c_int,
                                                 mut max: libc::c_int)
 -> libc::c_int {
    let mut value: libc::c_int = 0;
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() { return 0i32 }
    if min > max {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"cannot bound characteristic %d between %d and %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            index, min, max);
        return 0i32
    }
    value = Characteristic_Integer(character, index);
    if value < min { return min }
    if value > max { return max }
    return value;
}
//returns a string characteristic
#[no_mangle]
pub unsafe extern "C" fn Characteristic_String(mut character: libc::c_int,
                                               mut index: libc::c_int,
                                               mut buf: *mut libc::c_char,
                                               mut size: libc::c_int) {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() { return }
    if 0 == CheckCharacteristicIndex(character, index) { return }
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as libc::c_int ==
           3i32 {
        strncpy(buf,
                (*(*ch).c.as_mut_ptr().offset(index as isize)).value.string,
                (size - 1i32) as libc::c_ulong);
        *buf.offset((size - 1i32) as isize) = '\u{0}' as i32 as libc::c_char
    } else {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"characteristic %d is not a string\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            index);
    };
}
//free cached bot characters
#[no_mangle]
pub unsafe extern "C" fn BotShutdownCharacters() {
    let mut handle: libc::c_int = 0;
    handle = 1i32;
    while handle <= 64i32 {
        if !botcharacters[handle as usize].is_null() {
            BotFreeCharacter2(handle);
        }
        handle += 1
    };
}