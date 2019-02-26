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
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
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
        //returns true when the token is available
        #[no_mangle]
        pub fn PC_CheckTokenString(source: *mut source_t,
                                   string: *mut libc::c_char) -> libc::c_int;
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
      "ioq3/code/botlib/be_ai_weight.h"]
pub mod be_ai_weight_h {
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
 * name:		be_ai_weight.h
 *
 * desc:		fuzzy weights
 *
 * $Archive: /source/code/botlib/be_ai_weight.h $
 *
 *****************************************************************************/
    //fuzzy seperator
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fuzzyseperator_s {
        pub index: libc::c_int,
        pub value: libc::c_int,
        pub type_0: libc::c_int,
        pub weight: libc::c_float,
        pub minweight: libc::c_float,
        pub maxweight: libc::c_float,
        pub child: *mut fuzzyseperator_s,
        pub next: *mut fuzzyseperator_s,
    }
    pub type fuzzyseperator_t = fuzzyseperator_s;
    //fuzzy weight
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct weight_s {
        pub name: *mut libc::c_char,
        pub firstseperator: *mut fuzzyseperator_s,
    }
    pub type weight_t = weight_s;
    //weight configuration
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct weightconfig_s {
        pub numweights: libc::c_int,
        pub weights: [weight_t; 128],
        pub filename: [libc::c_char; 64],
    }
    pub type weightconfig_t = weightconfig_s;
    use super::{libc};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn rand() -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
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
        //print a source warning
        #[no_mangle]
        pub fn SourceWarning(source: *mut source_t,
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
      "ioq3/code/botlib/be_ai_weight.c"]
pub mod be_ai_weight_c {
    use super::be_ai_weight_h::{weightconfig_t, fuzzyseperator_t};
    use super::{libc};
    use super::l_precomp_h::{source_t};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, Q_strncpyz};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t, StripDoubleQuotes};
use self::l_precomp_h::{define_s, define_t, indent_s, indent_t, source_s,
                        source_t, PC_ReadToken, PC_ExpectTokenString,
                        PC_ExpectTokenType, PC_ExpectAnyToken,
                        PC_CheckTokenString, PC_SetBaseFolder, LoadSourceFile,
                        FreeSource};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_ai_weight_h::{fuzzyseperator_s, fuzzyseperator_t, weight_s,
                           weight_t, weightconfig_s, weightconfig_t};
use self::string_h::{strcpy, strcmp, strlen};
use self::stdlib_h::{rand};
use self::l_memory_h::{GetClearedMemory, FreeMemory};
use self::l_libvar_h::{LibVarGetValue};
use self::l_variadic_h::{SourceError, SourceWarning};
use self::be_interface_h::{botimport};
//reads a weight configuration
#[no_mangle]
pub unsafe extern "C" fn ReadWeightConfig(mut filename: *mut libc::c_char)
 -> *mut weightconfig_t {
    let mut newindent: libc::c_int = 0;
    let mut avail: libc::c_int = 0i32;
    let mut n: libc::c_int = 0;
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
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut fs: *mut fuzzyseperator_t = 0 as *mut fuzzyseperator_t;
    let mut config: *mut weightconfig_t = 0 as *mut weightconfig_t;
    if 0. ==
           LibVarGetValue(b"bot_reloadcharacters\x00" as *const u8 as
                              *const libc::c_char) {
        avail = -1i32;
        n = 0i32;
        while n < 128i32 {
            config = weightFileList[n as usize];
            if config.is_null() {
                if avail == -1i32 { avail = n }
            } else if strcmp(filename, (*config).filename.as_mut_ptr()) ==
                          0i32 {
                return config
            }
            n += 1
        }
        if avail == -1i32 {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"weightFileList was full trying to load %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                filename);
            return 0 as *mut weightconfig_t
        }
    }
    PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char);
    source = LoadSourceFile(filename);
    if source.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"counldn\'t load %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            filename);
        return 0 as *mut weightconfig_t
    }
    config =
        GetClearedMemory(::std::mem::size_of::<weightconfig_t>() as
                             libc::c_ulong) as *mut weightconfig_t;
    (*config).numweights = 0i32;
    Q_strncpyz((*config).filename.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    while 0 != PC_ReadToken(source, &mut token) {
        if 0 ==
               strcmp(token.string.as_mut_ptr(),
                      b"weight\x00" as *const u8 as *const libc::c_char) {
            if (*config).numweights >= 128i32 {
                SourceWarning(source,
                              b"too many fuzzy weights\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
                break ;
            } else {
                if 0 == PC_ExpectTokenType(source, 1i32, 0i32, &mut token) {
                    FreeWeightConfig(config);
                    FreeSource(source);
                    return 0 as *mut weightconfig_t
                }
                StripDoubleQuotes(token.string.as_mut_ptr());
                (*config).weights[(*config).numweights as usize].name =
                    GetClearedMemory(strlen(token.string.as_mut_ptr()).wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*config).weights[(*config).numweights as usize].name,
                       token.string.as_mut_ptr());
                if 0 == PC_ExpectAnyToken(source, &mut token) {
                    FreeWeightConfig(config);
                    FreeSource(source);
                    return 0 as *mut weightconfig_t
                }
                newindent = qfalse as libc::c_int;
                if 0 ==
                       strcmp(token.string.as_mut_ptr(),
                              b"{\x00" as *const u8 as *const libc::c_char) {
                    newindent = qtrue as libc::c_int;
                    if 0 == PC_ExpectAnyToken(source, &mut token) {
                        FreeWeightConfig(config);
                        FreeSource(source);
                        return 0 as *mut weightconfig_t
                    }
                }
                if 0 ==
                       strcmp(token.string.as_mut_ptr(),
                              b"switch\x00" as *const u8 as
                                  *const libc::c_char) {
                    fs = ReadFuzzySeperators_r(source);
                    if fs.is_null() {
                        FreeWeightConfig(config);
                        FreeSource(source);
                        return 0 as *mut weightconfig_t
                    }
                    (*config).weights[(*config).numweights as
                                          usize].firstseperator = fs
                } else if 0 ==
                              strcmp(token.string.as_mut_ptr(),
                                     b"return\x00" as *const u8 as
                                         *const libc::c_char) {
                    fs =
                        GetClearedMemory(::std::mem::size_of::<fuzzyseperator_t>()
                                             as libc::c_ulong) as
                            *mut fuzzyseperator_t;
                    (*fs).index = 0i32;
                    (*fs).value = 999999i32;
                    (*fs).next = 0 as *mut fuzzyseperator_s;
                    (*fs).child = 0 as *mut fuzzyseperator_s;
                    if 0 == ReadFuzzyWeight(source, fs) {
                        FreeMemory(fs as *mut libc::c_void);
                        FreeWeightConfig(config);
                        FreeSource(source);
                        return 0 as *mut weightconfig_t
                    }
                    (*config).weights[(*config).numweights as
                                          usize].firstseperator = fs
                } else {
                    SourceError(source,
                                b"invalid name %s\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                token.string.as_mut_ptr());
                    FreeWeightConfig(config);
                    FreeSource(source);
                    return 0 as *mut weightconfig_t
                }
                if 0 != newindent {
                    if 0 ==
                           PC_ExpectTokenString(source,
                                                b"}\x00" as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char) {
                        FreeWeightConfig(config);
                        FreeSource(source);
                        return 0 as *mut weightconfig_t
                    }
                }
                (*config).numweights += 1
            }
        } else {
            SourceError(source,
                        b"invalid name %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            FreeWeightConfig(config);
            FreeSource(source);
            return 0 as *mut weightconfig_t
        }
    }
    FreeSource(source);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        filename);
    if 0. ==
           LibVarGetValue(b"bot_reloadcharacters\x00" as *const u8 as
                              *const libc::c_char) {
        weightFileList[avail as usize] = config
    }
    return config;
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
 * name:		be_ai_weight.c
 *
 * desc:		fuzzy logic
 *
 * $Archive: /MissionPack/code/botlib/be_ai_weight.c $
 *
 *****************************************************************************/
#[no_mangle]
pub static mut weightFileList: [*mut weightconfig_t; 128] =
    [0 as *const weightconfig_t as *mut weightconfig_t; 128];
//free a weight configuration
#[no_mangle]
pub unsafe extern "C" fn FreeWeightConfig(mut config: *mut weightconfig_t) {
    if 0. ==
           LibVarGetValue(b"bot_reloadcharacters\x00" as *const u8 as
                              *const libc::c_char) {
        return
    }
    FreeWeightConfig2(config);
}
//end of the function FreeFuzzySeperators
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn FreeWeightConfig2(mut config: *mut weightconfig_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*config).numweights {
        FreeFuzzySeperators_r((*config).weights[i as usize].firstseperator);
        if !(*config).weights[i as usize].name.is_null() {
            FreeMemory((*config).weights[i as usize].name as
                           *mut libc::c_void);
        }
        i += 1
    }
    FreeMemory(config as *mut libc::c_void);
}
//end of the function ReadFuzzyWeight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn FreeFuzzySeperators_r(mut fs:
                                                   *mut fuzzyseperator_t) {
    if fs.is_null() { return }
    if !(*fs).child.is_null() { FreeFuzzySeperators_r((*fs).child); }
    if !(*fs).next.is_null() { FreeFuzzySeperators_r((*fs).next); }
    FreeMemory(fs as *mut libc::c_void);
}
//end of the function ReadValue
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ReadFuzzyWeight(mut source: *mut source_t,
                                         mut fs: *mut fuzzyseperator_t)
 -> libc::c_int {
    if 0 !=
           PC_CheckTokenString(source,
                               b"balance\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char) {
        (*fs).type_0 = 1i32;
        if 0 ==
               PC_ExpectTokenString(source,
                                    b"(\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            return qfalse as libc::c_int
        }
        if 0 == ReadValue(source, &mut (*fs).weight) {
            return qfalse as libc::c_int
        }
        if 0 ==
               PC_ExpectTokenString(source,
                                    b",\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            return qfalse as libc::c_int
        }
        if 0 == ReadValue(source, &mut (*fs).minweight) {
            return qfalse as libc::c_int
        }
        if 0 ==
               PC_ExpectTokenString(source,
                                    b",\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            return qfalse as libc::c_int
        }
        if 0 == ReadValue(source, &mut (*fs).maxweight) {
            return qfalse as libc::c_int
        }
        if 0 ==
               PC_ExpectTokenString(source,
                                    b")\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            return qfalse as libc::c_int
        }
    } else {
        (*fs).type_0 = 0i32;
        if 0 == ReadValue(source, &mut (*fs).weight) {
            return qfalse as libc::c_int
        }
        (*fs).minweight = (*fs).weight;
        (*fs).maxweight = (*fs).weight
    }
    if 0 ==
           PC_ExpectTokenString(source,
                                b";\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char) {
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ReadValue(mut source: *mut source_t,
                                   mut value: *mut libc::c_float)
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
    if 0 == PC_ExpectAnyToken(source, &mut token) {
        return qfalse as libc::c_int
    }
    if 0 ==
           strcmp(token.string.as_mut_ptr(),
                  b"-\x00" as *const u8 as *const libc::c_char) {
        SourceWarning(source,
                      b"negative value set to zero\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
        if 0 == PC_ExpectAnyToken(source, &mut token) {
            SourceError(source,
                        b"Missing return value\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            return qfalse as libc::c_int
        }
    }
    if token.type_0 != 3i32 {
        SourceError(source,
                    b"invalid return value %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr());
        return qfalse as libc::c_int
    }
    *value = token.floatvalue;
    return qtrue as libc::c_int;
}
//end of the function FreeWeightConfig
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ReadFuzzySeperators_r(mut source: *mut source_t)
 -> *mut fuzzyseperator_t {
    let mut newindent: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut def: libc::c_int = 0;
    let mut founddefault: libc::c_int = 0;
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
    let mut fs: *mut fuzzyseperator_t = 0 as *mut fuzzyseperator_t;
    let mut lastfs: *mut fuzzyseperator_t = 0 as *mut fuzzyseperator_t;
    let mut firstfs: *mut fuzzyseperator_t = 0 as *mut fuzzyseperator_t;
    founddefault = qfalse as libc::c_int;
    firstfs = 0 as *mut fuzzyseperator_t;
    lastfs = 0 as *mut fuzzyseperator_t;
    if 0 ==
           PC_ExpectTokenString(source,
                                b"(\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char) {
        return 0 as *mut fuzzyseperator_t
    }
    if 0 == PC_ExpectTokenType(source, 3i32, 0x1000i32, &mut token) {
        return 0 as *mut fuzzyseperator_t
    }
    index = token.intvalue as libc::c_int;
    if 0 ==
           PC_ExpectTokenString(source,
                                b")\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char) {
        return 0 as *mut fuzzyseperator_t
    }
    if 0 ==
           PC_ExpectTokenString(source,
                                b"{\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char) {
        return 0 as *mut fuzzyseperator_t
    }
    if 0 == PC_ExpectAnyToken(source, &mut token) {
        return 0 as *mut fuzzyseperator_t
    }
    loop  {
        def =
            (0 ==
                 strcmp(token.string.as_mut_ptr(),
                        b"default\x00" as *const u8 as *const libc::c_char))
                as libc::c_int;
        if 0 != def ||
               0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b"case\x00" as *const u8 as *const libc::c_char) {
            fs =
                GetClearedMemory(::std::mem::size_of::<fuzzyseperator_t>() as
                                     libc::c_ulong) as *mut fuzzyseperator_t;
            (*fs).index = index;
            if !lastfs.is_null() { (*lastfs).next = fs } else { firstfs = fs }
            lastfs = fs;
            if 0 != def {
                if 0 != founddefault {
                    SourceError(source,
                                b"switch already has a default\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut fuzzyseperator_t
                }
                (*fs).value = 999999i32;
                founddefault = qtrue as libc::c_int
            } else {
                if 0 ==
                       PC_ExpectTokenType(source, 3i32, 0x1000i32, &mut token)
                   {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut fuzzyseperator_t
                }
                (*fs).value = token.intvalue as libc::c_int
            }
            if 0 ==
                   PC_ExpectTokenString(source,
                                        b":\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) ||
                   0 == PC_ExpectAnyToken(source, &mut token) {
                FreeFuzzySeperators_r(firstfs);
                return 0 as *mut fuzzyseperator_t
            }
            newindent = qfalse as libc::c_int;
            if 0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b"{\x00" as *const u8 as *const libc::c_char) {
                newindent = qtrue as libc::c_int;
                if 0 == PC_ExpectAnyToken(source, &mut token) {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut fuzzyseperator_t
                }
            }
            if 0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b"return\x00" as *const u8 as *const libc::c_char) {
                if 0 == ReadFuzzyWeight(source, fs) {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut fuzzyseperator_t
                }
            } else if 0 ==
                          strcmp(token.string.as_mut_ptr(),
                                 b"switch\x00" as *const u8 as
                                     *const libc::c_char) {
                (*fs).child = ReadFuzzySeperators_r(source);
                if (*fs).child.is_null() {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut fuzzyseperator_t
                }
            } else {
                SourceError(source,
                            b"invalid name %s\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            token.string.as_mut_ptr());
                return 0 as *mut fuzzyseperator_t
            }
            if 0 != newindent {
                if 0 ==
                       PC_ExpectTokenString(source,
                                            b"}\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut fuzzyseperator_t
                }
            }
        } else {
            FreeFuzzySeperators_r(firstfs);
            SourceError(source,
                        b"invalid name %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return 0 as *mut fuzzyseperator_t
        }
        if 0 == PC_ExpectAnyToken(source, &mut token) {
            FreeFuzzySeperators_r(firstfs);
            return 0 as *mut fuzzyseperator_t
        }
        if !(0 !=
                 strcmp(token.string.as_mut_ptr(),
                        b"}\x00" as *const u8 as *const libc::c_char)) {
            break ;
        }
    }
    if 0 == founddefault {
        SourceWarning(source,
                      b"switch without default\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
        fs =
            GetClearedMemory(::std::mem::size_of::<fuzzyseperator_t>() as
                                 libc::c_ulong) as *mut fuzzyseperator_t;
        (*fs).index = index;
        (*fs).value = 999999i32;
        (*fs).weight = 0i32 as libc::c_float;
        (*fs).next = 0 as *mut fuzzyseperator_s;
        (*fs).child = 0 as *mut fuzzyseperator_s;
        if !lastfs.is_null() { (*lastfs).next = fs } else { firstfs = fs }
    }
    return firstfs;
}
//find the fuzzy weight with the given name
#[no_mangle]
pub unsafe extern "C" fn FindFuzzyWeight(mut wc: *mut weightconfig_t,
                                         mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*wc).numweights {
        if 0 == strcmp((*wc).weights[i as usize].name, name) { return i }
        i += 1
    }
    return -1i32;
}
//returns the fuzzy weight for the given inventory and weight
#[no_mangle]
pub unsafe extern "C" fn FuzzyWeight(mut inventory: *mut libc::c_int,
                                     mut wc: *mut weightconfig_t,
                                     mut weightnum: libc::c_int)
 -> libc::c_float {
    return FuzzyWeight_r(inventory,
                         (*wc).weights[weightnum as usize].firstseperator);
}
//end of the function FindFuzzyWeight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn FuzzyWeight_r(mut inventory: *mut libc::c_int,
                                       mut fs: *mut fuzzyseperator_t)
 -> libc::c_float {
    let mut scale: libc::c_float = 0.;
    let mut w1: libc::c_float = 0.;
    let mut w2: libc::c_float = 0.;
    if *inventory.offset((*fs).index as isize) < (*fs).value {
        if !(*fs).child.is_null() {
            return FuzzyWeight_r(inventory, (*fs).child)
        } else { return (*fs).weight }
    } else {
        if !(*fs).next.is_null() {
            if *inventory.offset((*fs).index as isize) < (*(*fs).next).value {
                if !(*fs).child.is_null() {
                    w1 = FuzzyWeight_r(inventory, (*fs).child)
                } else { w1 = (*fs).weight }
                if !(*(*fs).next).child.is_null() {
                    w2 = FuzzyWeight_r(inventory, (*(*fs).next).child)
                } else { w2 = (*(*fs).next).weight }
                if (*(*fs).next).value == 999999i32 {
                    return w2
                } else {
                    scale =
                        (*inventory.offset((*fs).index as isize) -
                             (*fs).value) as libc::c_float /
                            ((*(*fs).next).value - (*fs).value) as
                                libc::c_float
                }
                return (1i32 as libc::c_float - scale) * w1 + scale * w2
            }
            return FuzzyWeight_r(inventory, (*fs).next)
        }
    }
    return (*fs).weight;
}
#[no_mangle]
pub unsafe extern "C" fn FuzzyWeightUndecided(mut inventory: *mut libc::c_int,
                                              mut wc: *mut weightconfig_t,
                                              mut weightnum: libc::c_int)
 -> libc::c_float {
    return FuzzyWeightUndecided_r(inventory,
                                  (*wc).weights[weightnum as
                                                    usize].firstseperator);
}
//end of the function FuzzyWeight_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn FuzzyWeightUndecided_r(mut inventory:
                                                    *mut libc::c_int,
                                                mut fs: *mut fuzzyseperator_t)
 -> libc::c_float {
    let mut scale: libc::c_float = 0.;
    let mut w1: libc::c_float = 0.;
    let mut w2: libc::c_float = 0.;
    if *inventory.offset((*fs).index as isize) < (*fs).value {
        if !(*fs).child.is_null() {
            return FuzzyWeightUndecided_r(inventory, (*fs).child)
        } else {
            return (*fs).minweight +
                       (rand() & 0x7fffi32) as libc::c_float /
                           0x7fffi32 as libc::c_float *
                           ((*fs).maxweight - (*fs).minweight)
        }
    } else {
        if !(*fs).next.is_null() {
            if *inventory.offset((*fs).index as isize) < (*(*fs).next).value {
                if !(*fs).child.is_null() {
                    w1 = FuzzyWeightUndecided_r(inventory, (*fs).child)
                } else {
                    w1 =
                        (*fs).minweight +
                            (rand() & 0x7fffi32) as libc::c_float /
                                0x7fffi32 as libc::c_float *
                                ((*fs).maxweight - (*fs).minweight)
                }
                if !(*(*fs).next).child.is_null() {
                    w2 = FuzzyWeight_r(inventory, (*(*fs).next).child)
                } else {
                    w2 =
                        (*(*fs).next).minweight +
                            (rand() & 0x7fffi32) as libc::c_float /
                                0x7fffi32 as libc::c_float *
                                ((*(*fs).next).maxweight -
                                     (*(*fs).next).minweight)
                }
                if (*(*fs).next).value == 999999i32 {
                    return w2
                } else {
                    scale =
                        (*inventory.offset((*fs).index as isize) -
                             (*fs).value) as libc::c_float /
                            ((*(*fs).next).value - (*fs).value) as
                                libc::c_float
                }
                return (1i32 as libc::c_float - scale) * w1 + scale * w2
            }
            return FuzzyWeightUndecided_r(inventory, (*fs).next)
        }
    }
    return (*fs).weight;
}
//scales the weight with the given name
#[no_mangle]
pub unsafe extern "C" fn ScaleWeight(mut config: *mut weightconfig_t,
                                     mut name: *mut libc::c_char,
                                     mut scale: libc::c_float) {
    let mut i: libc::c_int = 0;
    if scale < 0i32 as libc::c_float {
        scale = 0i32 as libc::c_float
    } else if scale > 1i32 as libc::c_float { scale = 1i32 as libc::c_float }
    i = 0i32;
    while i < (*config).numweights {
        if 0 == strcmp(name, (*config).weights[i as usize].name) {
            ScaleFuzzySeperator_r((*config).weights[i as
                                                        usize].firstseperator,
                                  scale);
            break ;
        } else { i += 1 }
    };
}
//end for
//end of the function EvolveWeightConfig
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ScaleFuzzySeperator_r(mut fs: *mut fuzzyseperator_t,
                                               mut scale: libc::c_float) {
    if !(*fs).child.is_null() {
        ScaleFuzzySeperator_r((*fs).child, scale);
    } else if (*fs).type_0 == 1i32 {
        (*fs).weight = ((*fs).maxweight + (*fs).minweight) * scale;
        if (*fs).weight < (*fs).minweight {
            (*fs).weight = (*fs).minweight
        } else if (*fs).weight > (*fs).maxweight {
            (*fs).weight = (*fs).maxweight
        }
    }
    if !(*fs).next.is_null() { ScaleFuzzySeperator_r((*fs).next, scale); };
}
//evolves the weight configuration
#[no_mangle]
pub unsafe extern "C" fn EvolveWeightConfig(mut config: *mut weightconfig_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*config).numweights {
        EvolveFuzzySeperator_r((*config).weights[i as usize].firstseperator);
        i += 1
    };
}
//end of the function FuzzyWeightUndecided
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn EvolveFuzzySeperator_r(mut fs:
                                                    *mut fuzzyseperator_t) {
    if !(*fs).child.is_null() {
        EvolveFuzzySeperator_r((*fs).child);
    } else if (*fs).type_0 == 1i32 {
        if (((rand() & 0x7fffi32) as libc::c_float /
                 0x7fffi32 as libc::c_float) as libc::c_double) < 0.01f64 {
            (*fs).weight =
                ((*fs).weight as libc::c_double +
                     2.0f64 *
                         (((rand() & 0x7fffi32) as libc::c_float /
                               0x7fffi32 as libc::c_float) as libc::c_double -
                              0.5f64) *
                         ((*fs).maxweight - (*fs).minweight) as
                             libc::c_double) as libc::c_float
        } else {
            (*fs).weight =
                ((*fs).weight as libc::c_double +
                     2.0f64 *
                         (((rand() & 0x7fffi32) as libc::c_float /
                               0x7fffi32 as libc::c_float) as libc::c_double -
                              0.5f64) *
                         ((*fs).maxweight - (*fs).minweight) as libc::c_double
                         * 0.5f64) as libc::c_float
        }
        if (*fs).weight < (*fs).minweight {
            (*fs).minweight = (*fs).weight
        } else if (*fs).weight > (*fs).maxweight {
            (*fs).maxweight = (*fs).weight
        }
    }
    if !(*fs).next.is_null() { EvolveFuzzySeperator_r((*fs).next); };
}
//interbreed the weight configurations and stores the interbreeded one in configout
#[no_mangle]
pub unsafe extern "C" fn InterbreedWeightConfigs(mut config1:
                                                     *mut weightconfig_t,
                                                 mut config2:
                                                     *mut weightconfig_t,
                                                 mut configout:
                                                     *mut weightconfig_t) {
    let mut i: libc::c_int = 0;
    if (*config1).numweights != (*config2).numweights ||
           (*config1).numweights != (*configout).numweights {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"cannot interbreed weight configs, unequal numweights\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return
    }
    i = 0i32;
    while i < (*config1).numweights {
        InterbreedFuzzySeperator_r((*config1).weights[i as
                                                          usize].firstseperator,
                                   (*config2).weights[i as
                                                          usize].firstseperator,
                                   (*configout).weights[i as
                                                            usize].firstseperator);
        i += 1
    };
}
//end for
//end of the function ScaleFuzzyBalanceRange
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn InterbreedFuzzySeperator_r(mut fs1:
                                                        *mut fuzzyseperator_t,
                                                    mut fs2:
                                                        *mut fuzzyseperator_t,
                                                    mut fsout:
                                                        *mut fuzzyseperator_t)
 -> libc::c_int {
    if !(*fs1).child.is_null() {
        if (*fs2).child.is_null() || (*fsout).child.is_null() {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"cannot interbreed weight configs, unequal child\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
            return qfalse as libc::c_int
        }
        if 0 ==
               InterbreedFuzzySeperator_r((*fs2).child, (*fs2).child,
                                          (*fsout).child) {
            return qfalse as libc::c_int
        }
    } else if (*fs1).type_0 == 1i32 {
        if (*fs2).type_0 != 1i32 || (*fsout).type_0 != 1i32 {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"cannot interbreed weight configs, unequal balance\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
            return qfalse as libc::c_int
        }
        (*fsout).weight =
            ((*fs1).weight + (*fs2).weight) / 2i32 as libc::c_float;
        if (*fsout).weight > (*fsout).maxweight {
            (*fsout).maxweight = (*fsout).weight
        }
        if (*fsout).weight > (*fsout).minweight {
            (*fsout).minweight = (*fsout).weight
        }
    }
    if !(*fs1).next.is_null() {
        if (*fs2).next.is_null() || (*fsout).next.is_null() {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"cannot interbreed weight configs, unequal next\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
            return qfalse as libc::c_int
        }
        if 0 ==
               InterbreedFuzzySeperator_r((*fs1).next, (*fs2).next,
                                          (*fsout).next) {
            return qfalse as libc::c_int
        }
    }
    return qtrue as libc::c_int;
}
//frees cached weight configurations
#[no_mangle]
pub unsafe extern "C" fn BotShutdownWeights() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 128i32 {
        if !weightFileList[i as usize].is_null() {
            FreeWeightConfig2(weightFileList[i as usize]);
            weightFileList[i as usize] = 0 as *mut weightconfig_t
        }
        i += 1
    };
}
//end if
//end for
//end of the function ScaleWeight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ScaleFuzzySeperatorBalanceRange_r(mut fs:
                                                               *mut fuzzyseperator_t,
                                                           mut scale:
                                                               libc::c_float) {
    if !(*fs).child.is_null() {
        ScaleFuzzySeperatorBalanceRange_r((*fs).child, scale);
    } else if (*fs).type_0 == 1i32 {
        let mut mid: libc::c_float =
            (((*fs).minweight + (*fs).maxweight) as libc::c_double * 0.5f64)
                as libc::c_float;
        (*fs).maxweight = mid + ((*fs).maxweight - mid) * scale;
        (*fs).minweight = mid + ((*fs).minweight - mid) * scale;
        if (*fs).maxweight < (*fs).minweight {
            (*fs).maxweight = (*fs).minweight
        }
    }
    if !(*fs).next.is_null() {
        ScaleFuzzySeperatorBalanceRange_r((*fs).next, scale);
    };
}
//end of the function ScaleFuzzySeperatorBalanceRange_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ScaleFuzzyBalanceRange(mut config:
                                                    *mut weightconfig_t,
                                                mut scale: libc::c_float) {
    let mut i: libc::c_int = 0;
    if scale < 0i32 as libc::c_float {
        scale = 0i32 as libc::c_float
    } else if scale > 100i32 as libc::c_float {
        scale = 100i32 as libc::c_float
    }
    i = 0i32;
    while i < (*config).numweights {
        ScaleFuzzySeperatorBalanceRange_r((*config).weights[i as
                                                                usize].firstseperator,
                                          scale);
        i += 1
    };
}