use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __time_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h"]
pub mod time_t_h {
    pub type time_t = __time_t;
    use super::types_h::{__time_t};
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
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type fileHandle_t = libc::c_int;
    // parameters to the main Error routine
    pub type unnamed = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed = 0;
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
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct pc_token_s {
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub intvalue: libc::c_int,
        pub floatvalue: libc::c_float,
        pub string: [libc::c_char; 1024],
    }
    pub type pc_token_t = pc_token_s;
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
    pub type cplane_t = cplane_s;
    use super::{libc};
    extern "C" {
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int,
                        src: *const libc::c_char);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/botlib.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_script.h"]
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
        //read a token from the script
        #[no_mangle]
        pub fn PS_ReadToken(script: *mut script_t, token: *mut token_t)
         -> libc::c_int;
        //remove any leading and trailing double quotes from the token
        #[no_mangle]
        pub fn StripDoubleQuotes(string: *mut libc::c_char);
        //returns true if at the end of the script
        #[no_mangle]
        pub fn EndOfScript(script: *mut script_t) -> libc::c_int;
        //load a script from the given file at the given offset with the given length
        #[no_mangle]
        pub fn LoadScriptFile(filename: *const libc::c_char) -> *mut script_t;
        //load a script from the given memory with the given length
        #[no_mangle]
        pub fn LoadScriptMemory(ptr: *mut libc::c_char, length: libc::c_int,
                                name: *mut libc::c_char) -> *mut script_t;
        //free a script
        #[no_mangle]
        pub fn FreeScript(script: *mut script_t);
        //set the base folder to load files from
        #[no_mangle]
        pub fn PS_SetBaseFolder(path: *mut libc::c_char);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_precomp.h"]
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
    use super::q_shared_h::{pc_token_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_precomp.c"]
pub mod l_precomp_c {
    pub type directive_t = directive_s;
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
 * name:		l_precomp.c
 *
 * desc:		pre compiler
 *
 * $Archive: /MissionPack/code/botlib/l_precomp.c $
 *
 *****************************************************************************/
    //Notes:			fix: PC_StringizeTokens
    //#define SCREWUP
//#define BOTLIB
//#define QUAKE
//#define QUAKEC
//#define MEQCC
    //SCREWUP
    //BOTLIB
    //MEQCC
    //BSPC
    //QUAKE
    //#define DEBUG_EVAL
    //directive name with parse function
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct directive_s {
        pub name: *mut libc::c_char,
        pub func: Option<unsafe extern "C" fn(_: *mut source_t)
                             -> libc::c_int>,
    }
    pub type value_t = value_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct value_s {
        pub intvalue: libc::c_long,
        pub floatvalue: libc::c_float,
        pub parentheses: libc::c_int,
        pub prev: *mut value_s,
        pub next: *mut value_s,
    }
    pub type operator_t = operator_s;
    //end of the function PC_Directive_endif
    //============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct operator_s {
        pub operator: libc::c_int,
        pub priority: libc::c_int,
        pub parentheses: libc::c_int,
        pub prev: *mut operator_s,
        pub next: *mut operator_s,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct builtin {
        pub string: *mut libc::c_char,
        pub builtin: libc::c_int,
    }
    use super::{libc};
    use super::l_precomp_h::{source_t, define_t};
    use super::l_script_h::{token_t, script_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...)
         -> libc::c_int;
    }
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
        pub fn strncat(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
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
        pub fn free(__ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn labs(_: libc::c_long) -> libc::c_long;
    }
}
#[header_src = "/usr/include/time.h"]
pub mod time_h {
    use super::time_t_h::{time_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn time(__timer: *mut time_t) -> time_t;
        #[no_mangle]
        pub fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_variadic.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_log.h"]
pub mod l_log_h {
    use super::{libc};
    extern "C" {
        //write to the current opened log file
        #[no_mangle]
        pub fn Log_Write(fmt: *mut libc::c_char, ...);
    }
}
use self::types_h::{__time_t};
use self::stddef_h::{size_t};
use self::time_t_h::{time_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cplane_s,
                       pc_token_s, pc_token_t, fsMode_t, FS_APPEND_SYNC,
                       FS_APPEND, FS_WRITE, FS_READ, cplane_t, Q_stricmp,
                       Q_strncpyz, Q_strcat, Com_Error};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t, PS_ReadToken, StripDoubleQuotes,
                       EndOfScript, LoadScriptFile, LoadScriptMemory,
                       FreeScript, PS_SetBaseFolder};
use self::l_precomp_h::{define_s, define_t, indent_s, indent_t, source_s,
                        source_t};
use self::l_precomp_c::{directive_t, directive_s, value_t, value_s,
                        operator_t, operator_s, builtin};
use self::mathcalls_h::{fabs};
use self::stdio_h::{sprintf};
use self::string_h::{memcpy, memmove, memset, strcpy, strcat, strncat, strcmp,
                     strlen};
use self::stdlib_h::{free, labs};
use self::time_h::{time, ctime};
use self::be_interface_h::{botimport};
use self::l_memory_h::{GetMemory, GetClearedMemory, FreeMemory};
use self::l_variadic_h::{SourceError, SourceWarning};
use self::l_log_h::{Log_Write};
//read a token from the source
#[no_mangle]
pub unsafe extern "C" fn PC_ReadToken(mut source: *mut source_t,
                                      mut token: *mut token_t)
 -> libc::c_int {
    let mut define: *mut define_t = 0 as *mut define_t;
    loop  {
        if 0 == PC_ReadSourceToken(source, token) {
            return qfalse as libc::c_int
        }
        //check for precompiler directives
        if (*token).type_0 == 5i32 &&
               *(*token).string.as_mut_ptr() as libc::c_int == '#' as i32 {
            //QUAKC
            if 0 == PC_ReadDirective(source) { return qfalse as libc::c_int }
        } else if (*token).type_0 == 5i32 &&
                      *(*token).string.as_mut_ptr() as libc::c_int ==
                          '$' as i32 {
            //QUAKEC
            if 0 == PC_ReadDollarDirective(source) {
                return qfalse as libc::c_int
            }
        } else {
            if (*token).type_0 == 1i32 {
                let mut newtoken: token_t =
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
                if 0 != PC_ReadToken(source, &mut newtoken) {
                    if newtoken.type_0 == 1i32 {
                        (*token).string[strlen((*token).string.as_mut_ptr()).wrapping_sub(1i32
                                                                                              as
                                                                                              libc::c_ulong)
                                            as usize] =
                            '\u{0}' as i32 as libc::c_char;
                        if strlen((*token).string.as_mut_ptr()).wrapping_add(strlen(newtoken.string.as_mut_ptr().offset(1isize))).wrapping_add(1i32
                                                                                                                                                   as
                                                                                                                                                   libc::c_ulong)
                               >= 1024i32 as libc::c_ulong {
                            SourceError(source,
                                        b"string longer than MAX_TOKEN %d\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char, 1024i32);
                            return qfalse as libc::c_int
                        }
                        strcat((*token).string.as_mut_ptr(),
                               newtoken.string.as_mut_ptr().offset(1isize));
                    } else { PC_UnreadToken(source, &mut newtoken); }
                }
            }
            //end if
            //if skipping source because of conditional compilation
            if 0 != (*source).skip { continue ; }
            //if the token is a name
            if (*token).type_0 == 4i32 {
                define =
                    PC_FindHashedDefine((*source).definehash,
                                        (*token).string.as_mut_ptr());
                //DEFINEHASHING
                //if it is a define macro
                if !define.is_null() {
                    if 0 == PC_ExpandDefineIntoSource(source, token, define) {
                        return qfalse as libc::c_int
                    }
                    continue ;
                }
            }
            memcpy(&mut (*source).token as *mut token_t as *mut libc::c_void,
                   token as *const libc::c_void,
                   ::std::mem::size_of::<token_t>() as libc::c_ulong);
            return qtrue as libc::c_int
        }
    };
}
//end of the function PC_ExpandDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_ExpandDefineIntoSource(mut source: *mut source_t,
                                                   mut deftoken: *mut token_t,
                                                   mut define: *mut define_t)
 -> libc::c_int {
    let mut firsttoken: *mut token_t = 0 as *mut token_t;
    let mut lasttoken: *mut token_t = 0 as *mut token_t;
    if 0 ==
           PC_ExpandDefine(source, deftoken, define, &mut firsttoken,
                           &mut lasttoken) {
        return qfalse as libc::c_int
    }
    if !firsttoken.is_null() && !lasttoken.is_null() {
        (*lasttoken).next = (*source).tokens;
        (*source).tokens = firsttoken;
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//end of the function PC_ExpandBuiltinDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_ExpandDefine(mut source: *mut source_t,
                                         mut deftoken: *mut token_t,
                                         mut define: *mut define_t,
                                         mut firsttoken: *mut *mut token_t,
                                         mut lasttoken: *mut *mut token_t)
 -> libc::c_int {
    let mut parms: [*mut token_t; 128] =
        [0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t, 0 as *mut token_t,
         0 as *mut token_t, 0 as *mut token_t];
    let mut dt: *mut token_t = 0 as *mut token_t;
    let mut pt: *mut token_t = 0 as *mut token_t;
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut t1: *mut token_t = 0 as *mut token_t;
    let mut t2: *mut token_t = 0 as *mut token_t;
    let mut first: *mut token_t = 0 as *mut token_t;
    let mut last: *mut token_t = 0 as *mut token_t;
    let mut nextpt: *mut token_t = 0 as *mut token_t;
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
    let mut parmnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if 0 != (*define).builtin {
        return PC_ExpandBuiltinDefine(source, deftoken, define, firsttoken,
                                      lasttoken)
    }
    if 0 != (*define).numparms {
        if 0 == PC_ReadDefineParms(source, define, parms.as_mut_ptr(), 128i32)
           {
            return qfalse as libc::c_int
        }
    }
    first = 0 as *mut token_t;
    last = 0 as *mut token_t;
    let mut current_block_41: u64;
    dt = (*define).tokens;
    while !dt.is_null() {
        parmnum = -1i32;
        if (*dt).type_0 == 4i32 {
            parmnum = PC_FindDefineParm(define, (*dt).string.as_mut_ptr())
        }
        //end if
        //if it is a define parameter
        if parmnum >= 0i32 {
            pt = parms[parmnum as usize];
            while !pt.is_null() {
                t = PC_CopyToken(pt);
                (*t).next = 0 as *mut token_s;
                if !last.is_null() { (*last).next = t } else { first = t }
                last = t;
                pt = (*pt).next
            }
        } else {
            //end for
            //end if
            //if stringizing operator
            if (*dt).string[0usize] as libc::c_int == '#' as i32 &&
                   (*dt).string[1usize] as libc::c_int == '\u{0}' as i32 {
                if !(*dt).next.is_null() {
                    parmnum =
                        PC_FindDefineParm(define,
                                          (*(*dt).next).string.as_mut_ptr())
                } else { parmnum = -1i32 }
                //
                if parmnum >= 0i32 {
                    dt = (*dt).next;
                    if 0 ==
                           PC_StringizeTokens(parms[parmnum as usize],
                                              &mut token) {
                        SourceError(source,
                                    b"can\'t stringize tokens\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
                        return qfalse as libc::c_int
                    }
                    t = PC_CopyToken(&mut token);
                    current_block_41 = 13131896068329595644;
                } else {
                    //end if
                    SourceWarning(source,
                                  b"stringizing operator without define parameter\x00"
                                      as *const u8 as *const libc::c_char as
                                      *mut libc::c_char);
                    current_block_41 = 1917311967535052937;
                }
            } else {
                t = PC_CopyToken(dt);
                current_block_41 = 13131896068329595644;
            }
            match current_block_41 {
                1917311967535052937 => { }
                _ => {
                    (*t).next = 0 as *mut token_s;
                    if !last.is_null() { (*last).next = t } else { first = t }
                    last = t
                }
            }
        }
        dt = (*dt).next
    }
    t = first;
    while !t.is_null() {
        if !(*t).next.is_null() {
            //if the merging operator
            if (*(*t).next).string[0usize] as libc::c_int == '#' as i32 &&
                   (*(*t).next).string[1usize] as libc::c_int == '#' as i32 {
                t1 = t;
                t2 = (*(*t).next).next;
                if !t2.is_null() {
                    if 0 == PC_MergeTokens(t1, t2) {
                        SourceError(source,
                                    b"can\'t merge %s with %s\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char,
                                    (*t1).string.as_mut_ptr(),
                                    (*t2).string.as_mut_ptr());
                        return qfalse as libc::c_int
                    }
                    PC_FreeToken((*t1).next);
                    (*t1).next = (*t2).next;
                    if t2 == last { last = t1 }
                    PC_FreeToken(t2);
                    continue ;
                }
            }
        }
        t = (*t).next
    }
    *firsttoken = first;
    *lasttoken = last;
    i = 0i32;
    while i < (*define).numparms {
        pt = parms[i as usize];
        while !pt.is_null() {
            nextpt = (*pt).next;
            PC_FreeToken(pt);
            pt = nextpt
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
//end of the function PC_CopyToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_FreeToken(mut token: *mut token_t) {
    FreeMemory(token as *mut libc::c_void);
    numtokens -= 1;
}
#[no_mangle]
pub static mut numtokens: libc::c_int = 0;
//end of the function PC_StringizeTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_MergeTokens(mut t1: *mut token_t,
                                        mut t2: *mut token_t) -> libc::c_int {
    if (*t1).type_0 == 4i32 && ((*t2).type_0 == 4i32 || (*t2).type_0 == 3i32)
       {
        strcat((*t1).string.as_mut_ptr(), (*t2).string.as_mut_ptr());
        return qtrue as libc::c_int
    }
    if (*t1).type_0 == 1i32 && (*t2).type_0 == 1i32 {
        (*t1).string[strlen((*t1).string.as_mut_ptr()).wrapping_sub(1i32 as
                                                                        libc::c_ulong)
                         as usize] = '\u{0}' as i32 as libc::c_char;
        strcat((*t1).string.as_mut_ptr(), &mut (*t2).string[1usize]);
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
/*
	int i;

	if (tokenheapinitialized) return;
	freetokens = NULL;
	for (i = 0; i < TOKEN_HEAP_SIZE; i++)
	{
		token_heap[i].next = freetokens;
		freetokens = &token_heap[i];
	} //end for
	tokenheapinitialized = qtrue;
	*/
//end of the function PC_InitTokenHeap
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_CopyToken(mut token: *mut token_t)
 -> *mut token_t {
    let mut t: *mut token_t = 0 as *mut token_t;
    t =
        GetMemory(::std::mem::size_of::<token_t>() as libc::c_ulong) as
            *mut token_t;
    if t.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"out of token space\x00" as *const u8 as
                      *const libc::c_char);
    }
    memcpy(t as *mut libc::c_void, token as *const libc::c_void,
           ::std::mem::size_of::<token_t>() as libc::c_ulong);
    (*t).next = 0 as *mut token_s;
    numtokens += 1;
    return t;
}
//end of the function PC_ReadDefineParms
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_StringizeTokens(mut tokens: *mut token_t,
                                            mut token: *mut token_t)
 -> libc::c_int {
    let mut t: *mut token_t = 0 as *mut token_t;
    (*token).type_0 = 1i32;
    (*token).whitespace_p = 0 as *mut libc::c_char;
    (*token).endwhitespace_p = 0 as *mut libc::c_char;
    (*token).string[0usize] = '\u{0}' as i32 as libc::c_char;
    strcat((*token).string.as_mut_ptr(),
           b"\"\x00" as *const u8 as *const libc::c_char);
    t = tokens;
    while !t.is_null() {
        strncat((*token).string.as_mut_ptr(), (*t).string.as_mut_ptr(),
                (1024i32 as
                     libc::c_ulong).wrapping_sub(strlen((*token).string.as_mut_ptr())).wrapping_sub(1i32
                                                                                                        as
                                                                                                        libc::c_ulong));
        t = (*t).next
    }
    strncat((*token).string.as_mut_ptr(),
            b"\"\x00" as *const u8 as *const libc::c_char,
            (1024i32 as
                 libc::c_ulong).wrapping_sub(strlen((*token).string.as_mut_ptr())).wrapping_sub(1i32
                                                                                                    as
                                                                                                    libc::c_ulong));
    return qtrue as libc::c_int;
}
//end of the function PC_FindDefine
//============================================================================
//
// Parameter:				-
// Returns:					number of the parm
//								if no parm found with the given name -1 is returned
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_FindDefineParm(mut define: *mut define_t,
                                           mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut p: *mut token_t = 0 as *mut token_t;
    let mut i: libc::c_int = 0;
    i = 0i32;
    p = (*define).parms;
    while !p.is_null() {
        if 0 == strcmp((*p).string.as_mut_ptr(), name) { return i }
        i += 1;
        p = (*p).next
    }
    return -1i32;
}
//end of the function PC_UnreadSourceToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_ReadDefineParms(mut source: *mut source_t,
                                            mut define: *mut define_t,
                                            mut parms: *mut *mut token_t,
                                            mut maxparms: libc::c_int)
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
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut last: *mut token_t = 0 as *mut token_t;
    let mut i: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    let mut lastcomma: libc::c_int = 0;
    let mut numparms: libc::c_int = 0;
    let mut indent: libc::c_int = 0;
    if 0 == PC_ReadSourceToken(source, &mut token) {
        SourceError(source,
                    b"define %s missing parms\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    (*define).name);
        return qfalse as libc::c_int
    }
    if (*define).numparms > maxparms {
        SourceError(source,
                    b"define with more than %d parameters\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char, maxparms);
        return qfalse as libc::c_int
    }
    i = 0i32;
    while i < (*define).numparms {
        let ref mut fresh0 = *parms.offset(i as isize);
        *fresh0 = 0 as *mut token_t;
        i += 1
    }
    if 0 !=
           strcmp(token.string.as_mut_ptr(),
                  b"(\x00" as *const u8 as *const libc::c_char) {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(source,
                    b"define %s missing parms\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    (*define).name);
        return qfalse as libc::c_int
    }
    done = 0i32;
    numparms = 0i32;
    indent = 0i32;
    while 0 == done {
        if numparms >= maxparms {
            SourceError(source,
                        b"define %s with too many parms\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        (*define).name);
            return qfalse as libc::c_int
        }
        if numparms >= (*define).numparms {
            SourceWarning(source,
                          b"define %s has too many parms\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          (*define).name);
            return qfalse as libc::c_int
        }
        let ref mut fresh1 = *parms.offset(numparms as isize);
        *fresh1 = 0 as *mut token_t;
        lastcomma = 1i32;
        last = 0 as *mut token_t;
        while 0 == done {
            if 0 == PC_ReadSourceToken(source, &mut token) {
                SourceError(source,
                            b"define %s incomplete\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            (*define).name);
                return qfalse as libc::c_int
            }
            //end if
            //
            if 0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b",\x00" as *const u8 as *const libc::c_char) {
                if indent <= 0i32 {
                    if 0 != lastcomma {
                        SourceWarning(source,
                                      b"too many comma\'s\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char);
                    }
                    break ;
                }
            }
            lastcomma = 0i32;
            //
            if 0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b"(\x00" as *const u8 as *const libc::c_char) {
                indent += 1
            } else {
                //end if
                if 0 ==
                       strcmp(token.string.as_mut_ptr(),
                              b")\x00" as *const u8 as *const libc::c_char) {
                    indent -= 1;
                    if indent <= 0i32 {
                        if (*parms.offset(((*define).numparms - 1i32) as
                                              isize)).is_null() {
                            SourceWarning(source,
                                          b"too few define parms\x00" as
                                              *const u8 as *const libc::c_char
                                              as *mut libc::c_char);
                        }
                        done = 1i32;
                        break ;
                    }
                }
                if numparms < (*define).numparms {
                    t = PC_CopyToken(&mut token);
                    (*t).next = 0 as *mut token_s;
                    if !last.is_null() {
                        (*last).next = t
                    } else {
                        let ref mut fresh2 = *parms.offset(numparms as isize);
                        *fresh2 = t
                    }
                    last = t
                }
            }
        }
        numparms += 1
    }
    return qtrue as libc::c_int;
}
//end of the function PC_FreeToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_ReadSourceToken(mut source: *mut source_t,
                                            mut token: *mut token_t)
 -> libc::c_int {
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut script: *mut script_t = 0 as *mut script_t;
    let mut type_0: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    while (*source).tokens.is_null() {
        if 0 != PS_ReadToken((*source).scriptstack, token) {
            return qtrue as libc::c_int
        }
        if 0 != EndOfScript((*source).scriptstack) {
            while !(*source).indentstack.is_null() &&
                      (*(*source).indentstack).script == (*source).scriptstack
                  {
                SourceWarning(source,
                              b"missing #endif\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char);
                PC_PopIndent(source, &mut type_0, &mut skip);
            }
        }
        if (*(*source).scriptstack).next.is_null() {
            return qfalse as libc::c_int
        }
        script = (*source).scriptstack;
        (*source).scriptstack = (*(*source).scriptstack).next;
        FreeScript(script);
    }
    memcpy(token as *mut libc::c_void,
           (*source).tokens as *const libc::c_void,
           ::std::mem::size_of::<token_t>() as libc::c_ulong);
    t = (*source).tokens;
    (*source).tokens = (*(*source).tokens).next;
    PC_FreeToken(t);
    return qtrue as libc::c_int;
}
//end of the function PC_PushIndent
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_PopIndent(mut source: *mut source_t,
                                      mut type_0: *mut libc::c_int,
                                      mut skip: *mut libc::c_int) {
    let mut indent: *mut indent_t = 0 as *mut indent_t;
    *type_0 = 0i32;
    *skip = 0i32;
    indent = (*source).indentstack;
    if indent.is_null() { return }
    if (*(*source).indentstack).script != (*source).scriptstack { return }
    *type_0 = (*indent).type_0;
    *skip = (*indent).skip;
    (*source).indentstack = (*(*source).indentstack).next;
    (*source).skip -= (*indent).skip;
    FreeMemory(indent as *mut libc::c_void);
}
//end of the function PC_ReadSourceToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_UnreadSourceToken(mut source: *mut source_t,
                                              mut token: *mut token_t)
 -> libc::c_int {
    let mut t: *mut token_t = 0 as *mut token_t;
    t = PC_CopyToken(token);
    (*t).next = (*source).tokens;
    (*source).tokens = t;
    return qtrue as libc::c_int;
}
//DEFINEHASHING
//end for
//end of the function PC_AddBuiltinDefines
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_ExpandBuiltinDefine(mut source: *mut source_t,
                                                mut deftoken: *mut token_t,
                                                mut define: *mut define_t,
                                                mut firsttoken:
                                                    *mut *mut token_t,
                                                mut lasttoken:
                                                    *mut *mut token_t)
 -> libc::c_int {
    let mut token: *mut token_t = 0 as *mut token_t;
    let mut t: time_t = 0;
    let mut curtime: *mut libc::c_char = 0 as *mut libc::c_char;
    token = PC_CopyToken(deftoken);
    match (*define).builtin {
        1 => {
            sprintf((*token).string.as_mut_ptr(),
                    b"%d\x00" as *const u8 as *const libc::c_char,
                    (*deftoken).line);
            (*token).intvalue = (*deftoken).line as libc::c_ulong;
            (*token).floatvalue = (*deftoken).line as libc::c_float;
            (*token).type_0 = 3i32;
            (*token).subtype = 0x8i32 | 0x1000i32;
            *firsttoken = token;
            *lasttoken = token
        }
        2 => {
            strcpy((*token).string.as_mut_ptr(),
                   (*(*source).scriptstack).filename.as_mut_ptr());
            (*token).type_0 = 4i32;
            (*token).subtype =
                strlen((*token).string.as_mut_ptr()) as libc::c_int;
            *firsttoken = token;
            *lasttoken = token
        }
        3 => {
            t = time(0 as *mut time_t);
            curtime = ctime(&mut t);
            strcpy((*token).string.as_mut_ptr(),
                   b"\"\x00" as *const u8 as *const libc::c_char);
            strncat((*token).string.as_mut_ptr(), curtime.offset(4isize),
                    7i32 as libc::c_ulong);
            strncat((*token).string.as_mut_ptr().offset(7isize),
                    curtime.offset(20isize), 4i32 as libc::c_ulong);
            strcat((*token).string.as_mut_ptr(),
                   b"\"\x00" as *const u8 as *const libc::c_char);
            free(curtime as *mut libc::c_void);
            (*token).type_0 = 4i32;
            (*token).subtype =
                strlen((*token).string.as_mut_ptr()) as libc::c_int;
            *firsttoken = token;
            *lasttoken = token
        }
        4 => {
            t = time(0 as *mut time_t);
            curtime = ctime(&mut t);
            strcpy((*token).string.as_mut_ptr(),
                   b"\"\x00" as *const u8 as *const libc::c_char);
            strncat((*token).string.as_mut_ptr(), curtime.offset(11isize),
                    8i32 as libc::c_ulong);
            strcat((*token).string.as_mut_ptr(),
                   b"\"\x00" as *const u8 as *const libc::c_char);
            free(curtime as *mut libc::c_void);
            (*token).type_0 = 4i32;
            (*token).subtype =
                strlen((*token).string.as_mut_ptr()) as libc::c_int;
            *firsttoken = token;
            *lasttoken = token
        }
        5 | _ => {
            *firsttoken = 0 as *mut token_t;
            *lasttoken = 0 as *mut token_t
        }
    }
    return qtrue as libc::c_int;
}
//end of the function PC_AddDefineToHash
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_FindHashedDefine(mut definehash:
                                                 *mut *mut define_t,
                                             mut name: *mut libc::c_char)
 -> *mut define_t {
    let mut d: *mut define_t = 0 as *mut define_t;
    let mut hash: libc::c_int = 0;
    hash = PC_NameHash(name);
    d = *definehash.offset(hash as isize);
    while !d.is_null() {
        if 0 == strcmp((*d).name, name) { return d }
        d = (*d).hashnext
    }
    return 0 as *mut define_t;
}
//end for
//end of the function PC_PrintDefineHashTable
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
//char primes[16] = {1, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 41, 43, 47};
#[no_mangle]
pub unsafe extern "C" fn PC_NameHash(mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut hash: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    hash = 0i32;
    i = 0i32;
    while *name.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        hash += *name.offset(i as isize) as libc::c_int * (119i32 + i);
        i += 1
    }
    hash = (hash ^ hash >> 10i32 ^ hash >> 20i32) & 1024i32 - 1i32;
    return hash;
}
//unread the given token
#[no_mangle]
pub unsafe extern "C" fn PC_UnreadToken(mut source: *mut source_t,
                                        mut token: *mut token_t) {
    PC_UnreadSourceToken(source, token);
}
#[no_mangle]
pub unsafe extern "C" fn PC_ReadDollarDirective(mut source: *mut source_t)
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
    let mut i: libc::c_int = 0;
    if 0 == PC_ReadSourceToken(source, &mut token) {
        SourceError(source,
                    b"found $ without name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.linescrossed > 0i32 {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(source,
                    b"found $ at end of line\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.type_0 == 4i32 {
        i = 0i32;
        while !dollardirectives[i as usize].name.is_null() {
            if 0 ==
                   strcmp(dollardirectives[i as usize].name,
                          token.string.as_mut_ptr()) {
                return dollardirectives[i as
                                            usize].func.expect("non-null function pointer")(source)
            }
            i += 1
        }
    }
    PC_UnreadSourceToken(source, &mut token);
    SourceError(source,
                b"unknown precompiler directive %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr());
    return qfalse as libc::c_int;
}
//end of the function PC_DollarDirective_evalfloat
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub static mut dollardirectives: [directive_t; 20] =
    [directive_s{name:
                     b"evalint\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_DollarDirective_evalint),},
     directive_s{name:
                     b"evalfloat\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_DollarDirective_evalfloat),},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,}];
//end of the function PC_DollarDirective_evalint
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_DollarDirective_evalfloat(mut source:
                                                          *mut source_t)
 -> libc::c_int {
    let mut value: libc::c_float = 0.;
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
    if 0 ==
           PC_DollarEvaluate(source, 0 as *mut libc::c_long, &mut value,
                             qfalse as libc::c_int) {
        return qfalse as libc::c_int
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0i32;
    sprintf(token.string.as_mut_ptr(),
            b"%1.2f\x00" as *const u8 as *const libc::c_char,
            fabs(value as libc::c_double));
    token.type_0 = 3i32;
    token.subtype = 0x800i32 | 0x2000i32 | 0x8i32;
    token.floatvalue = fabs(value as libc::c_double) as libc::c_float;
    token.intvalue = token.floatvalue as libc::c_ulong;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0i32 as libc::c_float { UnreadSignToken(source); }
    return qtrue as libc::c_int;
}
//end of the function PC_Directive_pragma
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn UnreadSignToken(mut source: *mut source_t) {
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
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0i32;
    strcpy(token.string.as_mut_ptr(),
           b"-\x00" as *const u8 as *const libc::c_char);
    token.type_0 = 5i32;
    token.subtype = 30i32;
    PC_UnreadSourceToken(source, &mut token);
}
//end of the function PC_Evaluate
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_DollarEvaluate(mut source: *mut source_t,
                                           mut intvalue: *mut libc::c_long,
                                           mut floatvalue: *mut libc::c_float,
                                           mut integer: libc::c_int)
 -> libc::c_int {
    let mut indent: libc::c_int = 0;
    let mut defined: libc::c_int = qfalse as libc::c_int;
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
    let mut firsttoken: *mut token_t = 0 as *mut token_t;
    let mut lasttoken: *mut token_t = 0 as *mut token_t;
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut nexttoken: *mut token_t = 0 as *mut token_t;
    let mut define: *mut define_t = 0 as *mut define_t;
    if !intvalue.is_null() { *intvalue = 0i32 as libc::c_long }
    if !floatvalue.is_null() { *floatvalue = 0i32 as libc::c_float }
    if 0 == PC_ReadSourceToken(source, &mut token) {
        SourceError(source,
                    b"no leading ( after $evalint/$evalfloat\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if 0 == PC_ReadSourceToken(source, &mut token) {
        SourceError(source,
                    b"nothing to evaluate\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    indent = 1i32;
    firsttoken = 0 as *mut token_t;
    lasttoken = 0 as *mut token_t;
    loop  {
        //if the token is a name
        if token.type_0 == 4i32 {
            if 0 != defined {
                defined = qfalse as libc::c_int;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else { firsttoken = t }
                lasttoken = t
            } else if 0 ==
                          strcmp(token.string.as_mut_ptr(),
                                 b"defined\x00" as *const u8 as
                                     *const libc::c_char) {
                defined = qtrue as libc::c_int;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else { firsttoken = t }
                lasttoken = t
            } else {
                define =
                    PC_FindHashedDefine((*source).definehash,
                                        token.string.as_mut_ptr());
                if define.is_null() {
                    SourceError(source,
                                b"can\'t evaluate %s, not defined\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                                token.string.as_mut_ptr());
                    return qfalse as libc::c_int
                }
                if 0 == PC_ExpandDefineIntoSource(source, &mut token, define)
                   {
                    return qfalse as libc::c_int
                }
            }
        } else if token.type_0 == 3i32 || token.type_0 == 5i32 {
            if *token.string.as_mut_ptr() as libc::c_int == '(' as i32 {
                indent += 1
            } else if *token.string.as_mut_ptr() as libc::c_int == ')' as i32
             {
                indent -= 1
            }
            if indent <= 0i32 { break ; }
            t = PC_CopyToken(&mut token);
            (*t).next = 0 as *mut token_s;
            if !lasttoken.is_null() {
                (*lasttoken).next = t
            } else { firsttoken = t }
            lasttoken = t
        } else {
            SourceError(source,
                        b"can\'t evaluate %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return qfalse as libc::c_int
        }
        if !(0 != PC_ReadSourceToken(source, &mut token)) { break ; }
    }
    if 0 ==
           PC_EvaluateTokens(source, firsttoken, intvalue, floatvalue,
                             integer) {
        return qfalse as libc::c_int
    }
    t = firsttoken;
    while !t.is_null() {
        nexttoken = (*t).next;
        PC_FreeToken(t);
        t = nexttoken
    }
    return qtrue as libc::c_int;
}
//end of the function PC_OperatorPriority
//#define AllocValue()			GetClearedMemory(sizeof(value_t));
//#define FreeValue(val)		FreeMemory(val)
//#define AllocOperator(op)		op = (operator_t *) GetClearedMemory(sizeof(operator_t));
//#define FreeOperator(op)		FreeMemory(op);
//
#[no_mangle]
pub unsafe extern "C" fn PC_EvaluateTokens(mut source: *mut source_t,
                                           mut tokens: *mut token_t,
                                           mut intvalue: *mut libc::c_long,
                                           mut floatvalue: *mut libc::c_float,
                                           mut integer: libc::c_int)
 -> libc::c_int {
    let mut o: *mut operator_t = 0 as *mut operator_t;
    let mut firstoperator: *mut operator_t = 0 as *mut operator_t;
    let mut lastoperator: *mut operator_t = 0 as *mut operator_t;
    let mut v: *mut value_t = 0 as *mut value_t;
    let mut firstvalue: *mut value_t = 0 as *mut value_t;
    let mut lastvalue: *mut value_t = 0 as *mut value_t;
    let mut v1: *mut value_t = 0 as *mut value_t;
    let mut v2: *mut value_t = 0 as *mut value_t;
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut brace: libc::c_int = 0i32;
    let mut parentheses: libc::c_int = 0i32;
    let mut error: libc::c_int = 0i32;
    let mut lastwasvalue: libc::c_int = 0i32;
    let mut negativevalue: libc::c_int = 0i32;
    let mut questmarkintvalue: libc::c_int = 0i32;
    let mut questmarkfloatvalue: libc::c_float = 0i32 as libc::c_float;
    let mut gotquestmarkvalue: libc::c_int = qfalse as libc::c_int;
    //
    let mut operator_heap: [operator_t; 64] =
        [operator_s{operator: 0,
                    priority: 0,
                    parentheses: 0,
                    prev: 0 as *mut operator_s,
                    next: 0 as *mut operator_s,}; 64];
    let mut numoperators: libc::c_int = 0i32;
    let mut value_heap: [value_t; 64] =
        [value_s{intvalue: 0,
                 floatvalue: 0.,
                 parentheses: 0,
                 prev: 0 as *mut value_s,
                 next: 0 as *mut value_s,}; 64];
    let mut numvalues: libc::c_int = 0i32;
    lastoperator = 0 as *mut operator_t;
    firstoperator = lastoperator;
    lastvalue = 0 as *mut value_t;
    firstvalue = lastvalue;
    if !intvalue.is_null() { *intvalue = 0i32 as libc::c_long }
    if !floatvalue.is_null() { *floatvalue = 0i32 as libc::c_float }
    t = tokens;
    while !t.is_null() {
        let mut current_block_97: u64;
        match (*t).type_0 {
            4 => {
                if 0 != lastwasvalue || 0 != negativevalue {
                    SourceError(source,
                                b"syntax error in #if/#elif\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char);
                    error = 1i32
                } else if 0 !=
                              strcmp((*t).string.as_mut_ptr(),
                                     b"defined\x00" as *const u8 as
                                         *const libc::c_char) {
                    SourceError(source,
                                b"undefined name %s in #if/#elif\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                                (*t).string.as_mut_ptr());
                    error = 1i32
                } else {
                    t = (*t).next;
                    if 0 ==
                           strcmp((*t).string.as_mut_ptr(),
                                  b"(\x00" as *const u8 as
                                      *const libc::c_char) {
                        brace = qtrue as libc::c_int;
                        t = (*t).next
                    }
                    //end if
                    if t.is_null() || (*t).type_0 != 4i32 {
                        SourceError(source,
                                    b"defined without name in #if/#elif\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
                        error = 1i32
                    } else if numvalues >= 64i32 {
                        SourceError(source,
                                    b"out of value space\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char);
                        error = 1i32
                    } else {
                        let fresh3 = numvalues;
                        numvalues = numvalues + 1;
                        v = &mut value_heap[fresh3 as usize] as *mut value_t;
                        if !PC_FindHashedDefine((*source).definehash,
                                                (*t).string.as_mut_ptr()).is_null()
                           {
                            (*v).intvalue = 1i32 as libc::c_long;
                            (*v).floatvalue = 1i32 as libc::c_float
                        } else {
                            (*v).intvalue = 0i32 as libc::c_long;
                            (*v).floatvalue = 0i32 as libc::c_float
                        }
                        (*v).parentheses = parentheses;
                        (*v).next = 0 as *mut value_s;
                        (*v).prev = lastvalue;
                        if !lastvalue.is_null() {
                            (*lastvalue).next = v
                        } else { firstvalue = v }
                        lastvalue = v;
                        if 0 != brace {
                            t = (*t).next;
                            if t.is_null() ||
                                   0 !=
                                       strcmp((*t).string.as_mut_ptr(),
                                              b")\x00" as *const u8 as
                                                  *const libc::c_char) {
                                SourceError(source,
                                            b"defined without ) in #if/#elif\x00"
                                                as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char);
                                error = 1i32;
                                current_block_97 = 2473505634946569239;
                            } else { current_block_97 = 1854459640724737493; }
                        } else { current_block_97 = 1854459640724737493; }
                        match current_block_97 {
                            2473505634946569239 => { }
                            _ => {
                                brace = qfalse as libc::c_int;
                                lastwasvalue = 1i32
                            }
                        }
                    }
                }
            }
            3 => {
                if 0 != lastwasvalue {
                    SourceError(source,
                                b"syntax error in #if/#elif\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char);
                    error = 1i32
                } else if numvalues >= 64i32 {
                    SourceError(source,
                                b"out of value space\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
                    error = 1i32
                } else {
                    let fresh4 = numvalues;
                    numvalues = numvalues + 1;
                    v = &mut value_heap[fresh4 as usize] as *mut value_t;
                    if 0 != negativevalue {
                        (*v).intvalue =
                            -((*t).intvalue as libc::c_int) as libc::c_long;
                        (*v).floatvalue = -(*t).floatvalue
                    } else {
                        (*v).intvalue = (*t).intvalue as libc::c_long;
                        (*v).floatvalue = (*t).floatvalue
                    }
                    (*v).parentheses = parentheses;
                    (*v).next = 0 as *mut value_s;
                    (*v).prev = lastvalue;
                    if !lastvalue.is_null() {
                        (*lastvalue).next = v
                    } else { firstvalue = v }
                    lastvalue = v;
                    lastwasvalue = 1i32;
                    negativevalue = 0i32
                }
            }
            5 => {
                if 0 != negativevalue {
                    SourceError(source,
                                b"misplaced minus sign in #if/#elif\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                    error = 1i32
                } else if (*t).subtype == 44i32 {
                    parentheses += 1
                } else if (*t).subtype == 45i32 {
                    parentheses -= 1;
                    if parentheses < 0i32 {
                        SourceError(source,
                                    b"too many ) in #if/#elsif\x00" as
                                        *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
                        error = 1i32
                    }
                } else {
                    //end if
                    //end else if
                    //check for invalid operators on floating point values
                    if 0 == integer {
                        if (*t).subtype == 35i32 || (*t).subtype == 28i32 ||
                               (*t).subtype == 21i32 || (*t).subtype == 22i32
                               || (*t).subtype == 32i32 ||
                               (*t).subtype == 33i32 || (*t).subtype == 34i32
                           {
                            SourceError(source,
                                        b"illigal operator %s on floating point operands\x00"
                                            as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        (*t).string.as_mut_ptr());
                            error = 1i32;
                            current_block_97 = 2473505634946569239;
                        } else { current_block_97 = 6712462580143783635; }
                    } else { current_block_97 = 6712462580143783635; }
                    match current_block_97 {
                        2473505634946569239 => { }
                        _ => {
                            let mut current_block_80: u64;
                            match (*t).subtype {
                                36 | 35 => {
                                    if 0 != lastwasvalue {
                                        SourceError(source,
                                                    b"! or ~ after value in #if/#elif\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char);
                                        error = 1i32
                                    }
                                    //end if
                                    current_block_80 = 10248984122780841972;
                                }
                                16 | 17 => {
                                    SourceError(source,
                                                b"++ or -- used in #if/#elif\x00"
                                                    as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char);
                                    current_block_80 = 10248984122780841972;
                                }
                                30 => {
                                    if 0 == lastwasvalue {
                                        negativevalue = 1i32;
                                        current_block_80 =
                                            10248984122780841972;
                                    } else {
                                        //end if
                                        //end case
                                        current_block_80 =
                                            5706227035632243100;
                                    }
                                }
                                26 | 27 | 28 | 29 | 5 | 6 | 7 | 8 | 9 | 10 |
                                37 | 38 | 21 | 22 | 32 | 33 | 34 | 42 | 43 =>
                                {
                                    current_block_80 = 5706227035632243100;
                                }
                                _ => {
                                    SourceError(source,
                                                b"invalid operator %s in #if/#elif\x00"
                                                    as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char,
                                                (*t).string.as_mut_ptr());
                                    error = 1i32;
                                    current_block_80 = 10248984122780841972;
                                }
                            }
                            match current_block_80 {
                                5706227035632243100 => {
                                    if 0 == lastwasvalue {
                                        SourceError(source,
                                                    b"operator %s after operator in #if/#elif\x00"
                                                        as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char,
                                                    (*t).string.as_mut_ptr());
                                        error = 1i32
                                    }
                                }
                                _ => { }
                            }
                            //end default
                            //end switch
                            if 0 == error && 0 == negativevalue {
                                //o = (operator_t *) GetClearedMemory(sizeof(operator_t));
                                if numoperators >= 64i32 {
                                    SourceError(source,
                                                b"out of operator space\x00"
                                                    as *const u8 as
                                                    *const libc::c_char as
                                                    *mut libc::c_char);
                                    error = 1i32;
                                    current_block_97 = 2473505634946569239;
                                } else {
                                    let fresh5 = numoperators;
                                    numoperators = numoperators + 1;
                                    o =
                                        &mut operator_heap[fresh5 as usize] as
                                            *mut operator_t;
                                    (*o).operator = (*t).subtype;
                                    (*o).priority =
                                        PC_OperatorPriority((*t).subtype);
                                    (*o).parentheses = parentheses;
                                    (*o).next = 0 as *mut operator_s;
                                    (*o).prev = lastoperator;
                                    if !lastoperator.is_null() {
                                        (*lastoperator).next = o
                                    } else { firstoperator = o }
                                    lastoperator = o;
                                    lastwasvalue = 0i32;
                                    current_block_97 = 11235674318412060590;
                                }
                            } else {
                                current_block_97 = 11235674318412060590;
                            }
                            match current_block_97 {
                                2473505634946569239 => { }
                                _ => { }
                            }
                        }
                    }
                }
            }
            _ => {
                //end if
                SourceError(source,
                            b"unknown %s in #if/#elif\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            (*t).string.as_mut_ptr());
                error = 1i32
            }
        }
        //end default
        //end switch
        if 0 != error { break ; }
        t = (*t).next
    }
    if 0 == error {
        if 0 == lastwasvalue {
            SourceError(source,
                        b"trailing operator in #if/#elif\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            error = 1i32
        } else if 0 != parentheses {
            SourceError(source,
                        b"too many ( in #if/#elif\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            error = 1i32
        }
    }
    gotquestmarkvalue = qfalse as libc::c_int;
    questmarkintvalue = 0i32;
    questmarkfloatvalue = 0i32 as libc::c_float;
    while 0 == error && !firstoperator.is_null() {
        v = firstvalue;
        o = firstoperator;
        while !(*o).next.is_null() {
            //if the current operator is nested deeper in parentheses
			//than the next operator
            if (*o).parentheses > (*(*o).next).parentheses { break ; }
            //if the current and next operator are nested equally deep in parentheses
            if (*o).parentheses == (*(*o).next).parentheses {
                //if the priority of the current operator is equal or higher
				//than the priority of the next operator
                if (*o).priority >= (*(*o).next).priority { break ; }
            }
            if (*o).operator != 36i32 && (*o).operator != 35i32 {
                v = (*v).next
            }
            //if there's no value or no next value
            if v.is_null() {
                SourceError(source,
                            b"mising values in #if/#elif\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
                error = 1i32;
                break ;
            } else { o = (*o).next }
        }
        //end if
        //end for
        if 0 != error { break ; }
        v1 = v;
        v2 = (*v).next;
        match (*o).operator {
            36 => {
                (*v1).intvalue =
                    (0 == (*v1).intvalue) as libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    (0. == (*v1).floatvalue) as libc::c_int as libc::c_float
            }
            35 => { (*v1).intvalue = !(*v1).intvalue }
            26 => {
                (*v1).intvalue *= (*v2).intvalue;
                (*v1).floatvalue *= (*v2).floatvalue
            }
            27 => {
                if 0 == (*v2).intvalue || 0. == (*v2).floatvalue {
                    SourceError(source,
                                b"divide by zero in #if/#elif\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                    error = 1i32
                } else {
                    (*v1).intvalue /= (*v2).intvalue;
                    (*v1).floatvalue /= (*v2).floatvalue
                }
            }
            28 => {
                if 0 == (*v2).intvalue {
                    SourceError(source,
                                b"divide by zero in #if/#elif\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                    error = 1i32
                } else { (*v1).intvalue %= (*v2).intvalue }
            }
            29 => {
                (*v1).intvalue += (*v2).intvalue;
                (*v1).floatvalue += (*v2).floatvalue
            }
            30 => {
                (*v1).intvalue -= (*v2).intvalue;
                (*v1).floatvalue -= (*v2).floatvalue
            }
            5 => {
                (*v1).intvalue =
                    (0 != (*v1).intvalue && 0 != (*v2).intvalue) as
                        libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    (0. != (*v1).floatvalue && 0. != (*v2).floatvalue) as
                        libc::c_int as libc::c_float
            }
            6 => {
                (*v1).intvalue =
                    (0 != (*v1).intvalue || 0 != (*v2).intvalue) as
                        libc::c_int as libc::c_long;
                (*v1).floatvalue =
                    (0. != (*v1).floatvalue || 0. != (*v2).floatvalue) as
                        libc::c_int as libc::c_float
            }
            7 => {
                (*v1).intvalue =
                    ((*v1).intvalue >= (*v2).intvalue) as libc::c_int as
                        libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue >= (*v2).floatvalue) as libc::c_int as
                        libc::c_float
            }
            8 => {
                (*v1).intvalue =
                    ((*v1).intvalue <= (*v2).intvalue) as libc::c_int as
                        libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue <= (*v2).floatvalue) as libc::c_int as
                        libc::c_float
            }
            9 => {
                (*v1).intvalue =
                    ((*v1).intvalue == (*v2).intvalue) as libc::c_int as
                        libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue == (*v2).floatvalue) as libc::c_int as
                        libc::c_float
            }
            10 => {
                (*v1).intvalue =
                    ((*v1).intvalue != (*v2).intvalue) as libc::c_int as
                        libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue != (*v2).floatvalue) as libc::c_int as
                        libc::c_float
            }
            37 => {
                (*v1).intvalue =
                    ((*v1).intvalue > (*v2).intvalue) as libc::c_int as
                        libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue > (*v2).floatvalue) as libc::c_int as
                        libc::c_float
            }
            38 => {
                (*v1).intvalue =
                    ((*v1).intvalue < (*v2).intvalue) as libc::c_int as
                        libc::c_long;
                (*v1).floatvalue =
                    ((*v1).floatvalue < (*v2).floatvalue) as libc::c_int as
                        libc::c_float
            }
            21 => { (*v1).intvalue >>= (*v2).intvalue }
            22 => { (*v1).intvalue <<= (*v2).intvalue }
            32 => { (*v1).intvalue &= (*v2).intvalue }
            33 => { (*v1).intvalue |= (*v2).intvalue }
            34 => { (*v1).intvalue ^= (*v2).intvalue }
            42 => {
                if 0 == gotquestmarkvalue {
                    SourceError(source,
                                b": without ? in #if/#elif\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char);
                    error = 1i32
                } else {
                    if 0 != integer {
                        if 0 == questmarkintvalue {
                            (*v1).intvalue = (*v2).intvalue
                        }
                    } else if 0. == questmarkfloatvalue {
                        (*v1).floatvalue = (*v2).floatvalue
                    }
                    gotquestmarkvalue = qfalse as libc::c_int
                }
            }
            43 => {
                if 0 != gotquestmarkvalue {
                    SourceError(source,
                                b"? after ? in #if/#elif\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
                    error = 1i32
                } else {
                    questmarkintvalue = (*v1).intvalue as libc::c_int;
                    questmarkfloatvalue = (*v1).floatvalue;
                    gotquestmarkvalue = qtrue as libc::c_int
                }
            }
            _ => { }
        }
        //end if
        //end switch
        //DEBUG_EVAL
        if 0 != error { break ; }
        if (*o).operator != 36i32 && (*o).operator != 35i32 {
            if (*o).operator != 43i32 { v = (*v).next }
            if !v.is_null() {
                if !(*v).prev.is_null() {
                    (*(*v).prev).next = (*v).next
                } else { firstvalue = (*v).next }
                if !(*v).next.is_null() { (*(*v).next).prev = (*v).prev }
            }
        }
        if !(*o).prev.is_null() {
            (*(*o).prev).next = (*o).next
        } else { firstoperator = (*o).next }
        if !(*o).next.is_null() { (*(*o).next).prev = (*o).prev }
    }
    if !firstvalue.is_null() {
        if !intvalue.is_null() { *intvalue = (*firstvalue).intvalue }
        if !floatvalue.is_null() { *floatvalue = (*firstvalue).floatvalue }
    }
    o = firstoperator;
    while !o.is_null() { lastoperator = (*o).next; o = lastoperator }
    v = firstvalue;
    while !v.is_null() { lastvalue = (*v).next; v = lastvalue }
    if 0 == error { return qtrue as libc::c_int }
    if !intvalue.is_null() { *intvalue = 0i32 as libc::c_long }
    if !floatvalue.is_null() { *floatvalue = 0i32 as libc::c_float }
    return qfalse as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PC_OperatorPriority(mut op: libc::c_int)
 -> libc::c_int {
    match op {
        26 => { return 15i32 }
        27 => { return 15i32 }
        28 => { return 15i32 }
        29 => { return 14i32 }
        30 => { return 14i32 }
        5 => { return 7i32 }
        6 => { return 6i32 }
        7 => { return 12i32 }
        8 => { return 12i32 }
        9 => { return 11i32 }
        10 => { return 11i32 }
        36 => { return 16i32 }
        37 => { return 12i32 }
        38 => { return 12i32 }
        21 => { return 13i32 }
        22 => { return 13i32 }
        32 => { return 10i32 }
        33 => { return 8i32 }
        34 => { return 9i32 }
        35 => { return 16i32 }
        42 => { return 5i32 }
        43 => { return 5i32 }
        _ => { }
    }
    return qfalse as libc::c_int;
}
//end of the function PC_ReadDirective
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_DollarDirective_evalint(mut source: *mut source_t)
 -> libc::c_int {
    let mut value: libc::c_long = 0;
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
    if 0 ==
           PC_DollarEvaluate(source, &mut value, 0 as *mut libc::c_float,
                             qtrue as libc::c_int) {
        return qfalse as libc::c_int
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0i32;
    sprintf(token.string.as_mut_ptr(),
            b"%ld\x00" as *const u8 as *const libc::c_char, labs(value));
    token.type_0 = 3i32;
    token.subtype = 0x1000i32 | 0x2000i32 | 0x8i32;
    token.intvalue = labs(value) as libc::c_ulong;
    token.floatvalue = token.intvalue as libc::c_float;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0i32 as libc::c_long { UnreadSignToken(source); }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PC_ReadDirective(mut source: *mut source_t)
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
    let mut i: libc::c_int = 0;
    if 0 == PC_ReadSourceToken(source, &mut token) {
        SourceError(source,
                    b"found # without name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.linescrossed > 0i32 {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(source,
                    b"found # at end of line\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.type_0 == 4i32 {
        i = 0i32;
        while !directives[i as usize].name.is_null() {
            if 0 ==
                   strcmp(directives[i as usize].name,
                          token.string.as_mut_ptr()) {
                return directives[i as
                                      usize].func.expect("non-null function pointer")(source)
            }
            i += 1
        }
    }
    SourceError(source,
                b"unknown precompiler directive %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr());
    return qfalse as libc::c_int;
}
//end of the function PC_Directive_evalfloat
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub static mut directives: [directive_t; 20] =
    [directive_s{name:
                     b"if\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_if),},
     directive_s{name:
                     b"ifdef\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_ifdef),},
     directive_s{name:
                     b"ifndef\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_ifndef),},
     directive_s{name:
                     b"elif\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_elif),},
     directive_s{name:
                     b"else\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_else),},
     directive_s{name:
                     b"endif\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_endif),},
     directive_s{name:
                     b"include\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_include),},
     directive_s{name:
                     b"define\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_define),},
     directive_s{name:
                     b"undef\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_undef),},
     directive_s{name:
                     b"line\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_line),},
     directive_s{name:
                     b"error\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_error),},
     directive_s{name:
                     b"pragma\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_pragma),},
     directive_s{name:
                     b"eval\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_eval),},
     directive_s{name:
                     b"evalfloat\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 func: Some(PC_Directive_evalfloat),},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,},
     directive_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                 func: None,}];
//end of the function PC_Directive_eval
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_evalfloat(mut source: *mut source_t)
 -> libc::c_int {
    let mut value: libc::c_float = 0.;
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
    if 0 ==
           PC_Evaluate(source, 0 as *mut libc::c_long, &mut value,
                       qfalse as libc::c_int) {
        return qfalse as libc::c_int
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0i32;
    sprintf(token.string.as_mut_ptr(),
            b"%1.2f\x00" as *const u8 as *const libc::c_char,
            fabs(value as libc::c_double));
    token.type_0 = 3i32;
    token.subtype = 0x800i32 | 0x2000i32 | 0x8i32;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0i32 as libc::c_float { UnreadSignToken(source); }
    return qtrue as libc::c_int;
}
//end of the function PC_EvaluateTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Evaluate(mut source: *mut source_t,
                                     mut intvalue: *mut libc::c_long,
                                     mut floatvalue: *mut libc::c_float,
                                     mut integer: libc::c_int)
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
    let mut firsttoken: *mut token_t = 0 as *mut token_t;
    let mut lasttoken: *mut token_t = 0 as *mut token_t;
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut nexttoken: *mut token_t = 0 as *mut token_t;
    let mut define: *mut define_t = 0 as *mut define_t;
    let mut defined: libc::c_int = qfalse as libc::c_int;
    if !intvalue.is_null() { *intvalue = 0i32 as libc::c_long }
    if !floatvalue.is_null() { *floatvalue = 0i32 as libc::c_float }
    if 0 == PC_ReadLine(source, &mut token) {
        SourceError(source,
                    b"no value after #if/#elif\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    firsttoken = 0 as *mut token_t;
    lasttoken = 0 as *mut token_t;
    loop  {
        if token.type_0 == 4i32 {
            if 0 != defined {
                defined = qfalse as libc::c_int;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else { firsttoken = t }
                lasttoken = t
            } else if 0 ==
                          strcmp(token.string.as_mut_ptr(),
                                 b"defined\x00" as *const u8 as
                                     *const libc::c_char) {
                defined = qtrue as libc::c_int;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else { firsttoken = t }
                lasttoken = t
            } else {
                define =
                    PC_FindHashedDefine((*source).definehash,
                                        token.string.as_mut_ptr());
                if define.is_null() {
                    SourceError(source,
                                b"can\'t evaluate %s, not defined\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                                token.string.as_mut_ptr());
                    return qfalse as libc::c_int
                }
                if 0 == PC_ExpandDefineIntoSource(source, &mut token, define)
                   {
                    return qfalse as libc::c_int
                }
            }
        } else if token.type_0 == 3i32 || token.type_0 == 5i32 {
            t = PC_CopyToken(&mut token);
            (*t).next = 0 as *mut token_s;
            if !lasttoken.is_null() {
                (*lasttoken).next = t
            } else { firsttoken = t }
            lasttoken = t
        } else {
            SourceError(source,
                        b"can\'t evaluate %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return qfalse as libc::c_int
        }
        if !(0 != PC_ReadLine(source, &mut token)) { break ; }
    }
    if 0 ==
           PC_EvaluateTokens(source, firsttoken, intvalue, floatvalue,
                             integer) {
        return qfalse as libc::c_int
    }
    t = firsttoken;
    while !t.is_null() {
        nexttoken = (*t).next;
        PC_FreeToken(t);
        t = nexttoken
    }
    return qtrue as libc::c_int;
}
//read a token only if on the same line, lines are concatenated with a slash
#[no_mangle]
pub unsafe extern "C" fn PC_ReadLine(mut source: *mut source_t,
                                     mut token: *mut token_t) -> libc::c_int {
    let mut crossline: libc::c_int = 0;
    crossline = 0i32;
    loop  {
        if 0 == PC_ReadSourceToken(source, token) {
            return qfalse as libc::c_int
        }
        if (*token).linescrossed > crossline {
            PC_UnreadSourceToken(source, token);
            return qfalse as libc::c_int
        }
        crossline = 1i32;
        if !(0 ==
                 strcmp((*token).string.as_mut_ptr(),
                        b"\\\x00" as *const u8 as *const libc::c_char)) {
            break ;
        }
    }
    return qtrue as libc::c_int;
}
//end of the function UnreadSignToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_eval(mut source: *mut source_t)
 -> libc::c_int {
    let mut value: libc::c_long = 0;
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
    if 0 ==
           PC_Evaluate(source, &mut value, 0 as *mut libc::c_float,
                       qtrue as libc::c_int) {
        return qfalse as libc::c_int
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0i32;
    sprintf(token.string.as_mut_ptr(),
            b"%ld\x00" as *const u8 as *const libc::c_char, labs(value));
    token.type_0 = 3i32;
    token.subtype = 0x1000i32 | 0x2000i32 | 0x8i32;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0i32 as libc::c_long { UnreadSignToken(source); }
    return qtrue as libc::c_int;
}
//end of the function PC_Directive_error
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_pragma(mut source: *mut source_t)
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
    SourceWarning(source,
                  b"#pragma directive not supported\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char);
    while 0 != PC_ReadLine(source, &mut token) { }
    return qtrue as libc::c_int;
}
//end of the function PC_Directive_line
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_error(mut source: *mut source_t)
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
    strcpy(token.string.as_mut_ptr(),
           b"\x00" as *const u8 as *const libc::c_char);
    PC_ReadSourceToken(source, &mut token);
    SourceError(source,
                b"#error directive: %s\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char,
                token.string.as_mut_ptr());
    return qfalse as libc::c_int;
}
//end of the function PC_Directive
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_line(mut source: *mut source_t)
 -> libc::c_int {
    SourceError(source,
                b"#line directive not supported\x00" as *const u8 as
                    *const libc::c_char as *mut libc::c_char);
    return qfalse as libc::c_int;
}
//end of the function PC_ClearTokenWhiteSpace
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_undef(mut source: *mut source_t)
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
    let mut define: *mut define_t = 0 as *mut define_t;
    let mut lastdefine: *mut define_t = 0 as *mut define_t;
    let mut hash: libc::c_int = 0;
    if (*source).skip > 0i32 { return qtrue as libc::c_int }
    if 0 == PC_ReadLine(source, &mut token) {
        SourceError(source,
                    b"undef without name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.type_0 != 4i32 {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(source,
                    b"expected name, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr());
        return qfalse as libc::c_int
    }
    hash = PC_NameHash(token.string.as_mut_ptr());
    lastdefine = 0 as *mut define_t;
    define = *(*source).definehash.offset(hash as isize);
    while !define.is_null() {
        if 0 == strcmp((*define).name, token.string.as_mut_ptr()) {
            if 0 != (*define).flags & 0x1i32 {
                SourceWarning(source,
                              b"can\'t undef %s\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              token.string.as_mut_ptr());
            } else {
                if !lastdefine.is_null() {
                    (*lastdefine).hashnext = (*define).hashnext
                } else {
                    let ref mut fresh6 =
                        *(*source).definehash.offset(hash as isize);
                    *fresh6 = (*define).hashnext
                }
                PC_FreeDefine(define);
            }
            //end else
            break ;
        } else { lastdefine = define; define = (*define).hashnext }
    }
    return qtrue as libc::c_int;
}
//end of the function PC_FindDefineParm
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_FreeDefine(mut define: *mut define_t) {
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut next: *mut token_t = 0 as *mut token_t;
    t = (*define).parms;
    while !t.is_null() { next = (*t).next; PC_FreeToken(t); t = next }
    t = (*define).tokens;
    while !t.is_null() { next = (*t).next; PC_FreeToken(t); t = next }
    FreeMemory((*define).name as *mut libc::c_void);
    FreeMemory(define as *mut libc::c_void);
}
//end of the function PC_Directive_undef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_define(mut source: *mut source_t)
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
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut last: *mut token_t = 0 as *mut token_t;
    let mut define: *mut define_t = 0 as *mut define_t;
    if (*source).skip > 0i32 { return qtrue as libc::c_int }
    if 0 == PC_ReadLine(source, &mut token) {
        SourceError(source,
                    b"#define without name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.type_0 != 4i32 {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(source,
                    b"expected name after #define, found %s\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr());
        return qfalse as libc::c_int
    }
    define =
        PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
    if !define.is_null() {
        if 0 != (*define).flags & 0x1i32 {
            SourceError(source,
                        b"can\'t redefine %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return qfalse as libc::c_int
        }
        SourceWarning(source,
                      b"redefinition of %s\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      token.string.as_mut_ptr());
        PC_UnreadSourceToken(source, &mut token);
        if 0 == PC_Directive_undef(source) { return qfalse as libc::c_int }
    }
    define =
        GetMemory(::std::mem::size_of::<define_t>() as libc::c_ulong) as
            *mut define_t;
    memset(define as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<define_t>() as libc::c_ulong);
    (*define).name =
        GetMemory(strlen(token.string.as_mut_ptr()).wrapping_add(1i32 as
                                                                     libc::c_ulong))
            as *mut libc::c_char;
    strcpy((*define).name, token.string.as_mut_ptr());
    PC_AddDefineToHash(define, (*source).definehash);
    if 0 == PC_ReadLine(source, &mut token) { return qtrue as libc::c_int }
    if 0 == PC_WhiteSpaceBeforeToken(&mut token) &&
           0 ==
               strcmp(token.string.as_mut_ptr(),
                      b"(\x00" as *const u8 as *const libc::c_char) {
        last = 0 as *mut token_t;
        if 0 ==
               PC_CheckTokenString(source,
                                   b")\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) {
            loop  {
                if 0 == PC_ReadLine(source, &mut token) {
                    SourceError(source,
                                b"expected define parameter\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char);
                    return qfalse as libc::c_int
                }
                if token.type_0 != 4i32 {
                    SourceError(source,
                                b"invalid define parameter\x00" as *const u8
                                    as *const libc::c_char as
                                    *mut libc::c_char);
                    return qfalse as libc::c_int
                }
                if PC_FindDefineParm(define, token.string.as_mut_ptr()) >=
                       0i32 {
                    SourceError(source,
                                b"two the same define parameters\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                    return qfalse as libc::c_int
                }
                t = PC_CopyToken(&mut token);
                PC_ClearTokenWhiteSpace(t);
                (*t).next = 0 as *mut token_s;
                if !last.is_null() {
                    (*last).next = t
                } else { (*define).parms = t }
                last = t;
                (*define).numparms += 1;
                if 0 == PC_ReadLine(source, &mut token) {
                    SourceError(source,
                                b"define parameters not terminated\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                    return qfalse as libc::c_int
                }
                //end if
                //
                if 0 ==
                       strcmp(token.string.as_mut_ptr(),
                              b")\x00" as *const u8 as *const libc::c_char) {
                    break ;
                }
                if 0 !=
                       strcmp(token.string.as_mut_ptr(),
                              b",\x00" as *const u8 as *const libc::c_char) {
                    SourceError(source,
                                b"define not terminated\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char);
                    return qfalse as libc::c_int
                }
            }
        }
        if 0 == PC_ReadLine(source, &mut token) {
            return qtrue as libc::c_int
        }
    }
    last = 0 as *mut token_t;
    loop  {
        t = PC_CopyToken(&mut token);
        if (*t).type_0 == 4i32 &&
               0 == strcmp((*t).string.as_mut_ptr(), (*define).name) {
            SourceError(source,
                        b"recursive define (removed recursion)\x00" as
                            *const u8 as *const libc::c_char as
                            *mut libc::c_char);
        } else {
            PC_ClearTokenWhiteSpace(t);
            (*t).next = 0 as *mut token_s;
            if !last.is_null() {
                (*last).next = t
            } else { (*define).tokens = t }
            last = t
        }
        if !(0 != PC_ReadLine(source, &mut token)) { break ; }
    }
    if !last.is_null() {
        if 0 ==
               strcmp((*(*define).tokens).string.as_mut_ptr(),
                      b"##\x00" as *const u8 as *const libc::c_char) ||
               0 ==
                   strcmp((*last).string.as_mut_ptr(),
                          b"##\x00" as *const u8 as *const libc::c_char) {
            SourceError(source,
                        b"define with misplaced ##\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            return qfalse as libc::c_int
        }
    }
    return qtrue as libc::c_int;
}
//end of the function PC_WhiteSpaceBeforeToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_ClearTokenWhiteSpace(mut token: *mut token_t) {
    (*token).whitespace_p = 0 as *mut libc::c_char;
    (*token).endwhitespace_p = 0 as *mut libc::c_char;
    (*token).linescrossed = 0i32;
}
//returns true when the token is available
#[no_mangle]
pub unsafe extern "C" fn PC_CheckTokenString(mut source: *mut source_t,
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
    if 0 == PC_ReadToken(source, &mut tok) { return qfalse as libc::c_int }
    if 0 == strcmp(tok.string.as_mut_ptr(), string) {
        return qtrue as libc::c_int
    }
    PC_UnreadSourceToken(source, &mut tok);
    return qfalse as libc::c_int;
}
//returns true if there was a white space in front of the token
#[no_mangle]
pub unsafe extern "C" fn PC_WhiteSpaceBeforeToken(mut token: *mut token_t)
 -> libc::c_int {
    return ((*token).endwhitespace_p.wrapping_offset_from((*token).whitespace_p)
                as libc::c_long > 0i32 as libc::c_long) as libc::c_int;
}
//end of the function PC_NameHash
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_AddDefineToHash(mut define: *mut define_t,
                                            mut definehash:
                                                *mut *mut define_t) {
    let mut hash: libc::c_int = 0;
    hash = PC_NameHash((*define).name);
    (*define).hashnext = *definehash.offset(hash as isize);
    let ref mut fresh7 = *definehash.offset(hash as isize);
    *fresh7 = define;
}
//end while
//end of the function PC_ConvertPath
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_include(mut source: *mut source_t)
 -> libc::c_int {
    let mut script: *mut script_t = 0 as *mut script_t;
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
    let mut path: [libc::c_char; 64] = [0; 64];
    if (*source).skip > 0i32 { return qtrue as libc::c_int }
    if 0 == PC_ReadSourceToken(source, &mut token) {
        SourceError(source,
                    b"#include without file name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.linescrossed > 0i32 {
        SourceError(source,
                    b"#include without file name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.type_0 == 1i32 {
        StripDoubleQuotes(token.string.as_mut_ptr());
        PC_ConvertPath(token.string.as_mut_ptr());
        script = LoadScriptFile(token.string.as_mut_ptr());
        if script.is_null() {
            Q_strncpyz(path.as_mut_ptr(), (*source).includepath.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 64]>() as
                           libc::c_ulong as libc::c_int);
            Q_strcat(path.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 64]>() as
                         libc::c_ulong as libc::c_int,
                     token.string.as_mut_ptr());
            script = LoadScriptFile(path.as_mut_ptr())
        }
    } else if token.type_0 == 5i32 &&
                  *token.string.as_mut_ptr() as libc::c_int == '<' as i32 {
        Q_strncpyz(path.as_mut_ptr(), (*source).includepath.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
        while 0 != PC_ReadSourceToken(source, &mut token) {
            if token.linescrossed > 0i32 {
                PC_UnreadSourceToken(source, &mut token);
                break ;
            } else {
                //end if
                if token.type_0 == 5i32 &&
                       *token.string.as_mut_ptr() as libc::c_int == '>' as i32
                   {
                    break ;
                }
                Q_strcat(path.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 64]>() as
                             libc::c_ulong as libc::c_int,
                         token.string.as_mut_ptr());
            }
        }
        if *token.string.as_mut_ptr() as libc::c_int != '>' as i32 {
            SourceWarning(source,
                          b"#include missing trailing >\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char);
        }
        if 0 == strlen(path.as_mut_ptr()) {
            SourceError(source,
                        b"#include without file name between < >\x00" as
                            *const u8 as *const libc::c_char as
                            *mut libc::c_char);
            return qfalse as libc::c_int
        }
        PC_ConvertPath(path.as_mut_ptr());
        script = LoadScriptFile(path.as_mut_ptr())
    } else {
        SourceError(source,
                    b"#include without file name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if script.is_null() {
        SourceError(source,
                    b"file %s not found\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    path.as_mut_ptr());
        return qfalse as libc::c_int
    }
    PC_PushScript(source, script);
    return qtrue as libc::c_int;
}
//end of the function PC_PopIndent
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_PushScript(mut source: *mut source_t,
                                       mut script: *mut script_t) {
    let mut s: *mut script_t = 0 as *mut script_t;
    s = (*source).scriptstack;
    while !s.is_null() {
        if 0 ==
               Q_stricmp((*s).filename.as_mut_ptr(),
                         (*script).filename.as_mut_ptr()) {
            SourceError(source,
                        b"%s recursively included\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        (*script).filename.as_mut_ptr());
            return
        }
        s = (*s).next
    }
    (*script).next = (*source).scriptstack;
    (*source).scriptstack = script;
}
//end of the function PC_ExpandDefineIntoSource
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_ConvertPath(mut path: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = path;
    while 0 != *ptr {
        if (*ptr as libc::c_int == '\\' as i32 ||
                *ptr as libc::c_int == '/' as i32) &&
               (*ptr.offset(1isize) as libc::c_int == '\\' as i32 ||
                    *ptr.offset(1isize) as libc::c_int == '/' as i32) {
            memmove(ptr as *mut libc::c_void,
                    ptr.offset(1isize) as *const libc::c_void, strlen(ptr));
        } else { ptr = ptr.offset(1isize) }
    }
    ptr = path;
    while 0 != *ptr {
        if *ptr as libc::c_int == '/' as i32 ||
               *ptr as libc::c_int == '\\' as i32 {
            *ptr = '/' as i32 as libc::c_char
        }
        ptr = ptr.offset(1isize)
    };
}
//end of the function PC_Directive_else
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_endif(mut source: *mut source_t)
 -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    PC_PopIndent(source, &mut type_0, &mut skip);
    if 0 == type_0 {
        SourceError(source,
                    b"misplaced #endif\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//end of the function PC_Directive_ifndef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_else(mut source: *mut source_t)
 -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    PC_PopIndent(source, &mut type_0, &mut skip);
    if 0 == type_0 {
        SourceError(source,
                    b"misplaced #else\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if type_0 == 0x2i32 {
        SourceError(source,
                    b"#else after #else\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    PC_PushIndent(source, 0x2i32, (0 == skip) as libc::c_int);
    return qtrue as libc::c_int;
}
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_PushIndent(mut source: *mut source_t,
                                       mut type_0: libc::c_int,
                                       mut skip: libc::c_int) {
    let mut indent: *mut indent_t = 0 as *mut indent_t;
    indent =
        GetMemory(::std::mem::size_of::<indent_t>() as libc::c_ulong) as
            *mut indent_t;
    (*indent).type_0 = type_0;
    (*indent).script = (*source).scriptstack;
    (*indent).skip = (skip != 0i32) as libc::c_int;
    (*source).skip += (*indent).skip;
    (*indent).next = (*source).indentstack;
    (*source).indentstack = indent;
}
//end of the function PC_DollarEvaluate
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_elif(mut source: *mut source_t)
 -> libc::c_int {
    let mut value: libc::c_long = 0;
    let mut type_0: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    PC_PopIndent(source, &mut type_0, &mut skip);
    if 0 == type_0 || type_0 == 0x2i32 {
        SourceError(source,
                    b"misplaced #elif\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if 0 ==
           PC_Evaluate(source, &mut value, 0 as *mut libc::c_float,
                       qtrue as libc::c_int) {
        return qfalse as libc::c_int
    }
    skip = (value == 0i32 as libc::c_long) as libc::c_int;
    PC_PushIndent(source, 0x4i32, skip);
    return qtrue as libc::c_int;
}
//end of the function PC_Directive_ifdef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_ifndef(mut source: *mut source_t)
 -> libc::c_int {
    return PC_Directive_if_def(source, 0x10i32);
}
//DEFINEHASHING
//DEFINEHASHING
//end for
//end of the function PC_AddGlobalDefinesToSource
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_if_def(mut source: *mut source_t,
                                             mut type_0: libc::c_int)
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
    let mut d: *mut define_t = 0 as *mut define_t;
    let mut skip: libc::c_int = 0;
    if 0 == PC_ReadLine(source, &mut token) {
        SourceError(source,
                    b"#ifdef without name\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    }
    if token.type_0 != 4i32 {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(source,
                    b"expected name after #ifdef, found %s\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    token.string.as_mut_ptr());
        return qfalse as libc::c_int
    }
    d = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
    skip =
        ((type_0 == 0x8i32) as libc::c_int ==
             (d == 0 as *mut libc::c_void as *mut define_t) as libc::c_int) as
            libc::c_int;
    PC_PushIndent(source, type_0, skip);
    return qtrue as libc::c_int;
}
//end of the function PC_Directiveif_def
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_ifdef(mut source: *mut source_t)
 -> libc::c_int {
    return PC_Directive_if_def(source, 0x8i32);
}
//end of the function PC_Directive_elif
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_Directive_if(mut source: *mut source_t)
 -> libc::c_int {
    let mut value: libc::c_long = 0;
    let mut skip: libc::c_int = 0;
    if 0 ==
           PC_Evaluate(source, &mut value, 0 as *mut libc::c_float,
                       qtrue as libc::c_int) {
        return qfalse as libc::c_int
    }
    skip = (value == 0i32 as libc::c_long) as libc::c_int;
    PC_PushIndent(source, 0x1i32, skip);
    return qtrue as libc::c_int;
}
//expect a certain token
#[no_mangle]
pub unsafe extern "C" fn PC_ExpectTokenString(mut source: *mut source_t,
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
    if 0 == PC_ReadToken(source, &mut token) {
        SourceError(source,
                    b"couldn\'t find expected %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char, string);
        return qfalse as libc::c_int
    }
    if 0 != strcmp(token.string.as_mut_ptr(), string) {
        SourceError(source,
                    b"expected %s, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char, string,
                    token.string.as_mut_ptr());
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//expect a certain token type
#[no_mangle]
pub unsafe extern "C" fn PC_ExpectTokenType(mut source: *mut source_t,
                                            mut type_0: libc::c_int,
                                            mut subtype: libc::c_int,
                                            mut token: *mut token_t)
 -> libc::c_int {
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if 0 == PC_ReadToken(source, token) {
        SourceError(source,
                    b"couldn\'t read expected token\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
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
        SourceError(source,
                    b"expected a %s, found %s\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char,
                    str.as_mut_ptr(), (*token).string.as_mut_ptr());
        return qfalse as libc::c_int
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
            SourceError(source,
                        b"expected %s, found %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        str.as_mut_ptr(), (*token).string.as_mut_ptr());
            return qfalse as libc::c_int
        }
    } else if (*token).type_0 == 5i32 {
        if (*token).subtype != subtype {
            SourceError(source,
                        b"found %s\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char, (*token).string.as_mut_ptr());
            return qfalse as libc::c_int
        }
    }
    return qtrue as libc::c_int;
}
//expect a token
#[no_mangle]
pub unsafe extern "C" fn PC_ExpectAnyToken(mut source: *mut source_t,
                                           mut token: *mut token_t)
 -> libc::c_int {
    if 0 == PC_ReadToken(source, token) {
        SourceError(source,
                    b"couldn\'t read expected token\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        return qfalse as libc::c_int
    } else { return qtrue as libc::c_int };
}
//returns true and reads the token when a token with the given type is available
#[no_mangle]
pub unsafe extern "C" fn PC_CheckTokenType(mut source: *mut source_t,
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
    if 0 == PC_ReadToken(source, &mut tok) { return qfalse as libc::c_int }
    if tok.type_0 == type_0 && tok.subtype & subtype == subtype {
        memcpy(token as *mut libc::c_void,
               &mut tok as *mut token_t as *const libc::c_void,
               ::std::mem::size_of::<token_t>() as libc::c_ulong);
        return qtrue as libc::c_int
    }
    PC_UnreadSourceToken(source, &mut tok);
    return qfalse as libc::c_int;
}
//skip tokens until the given token string is read
#[no_mangle]
pub unsafe extern "C" fn PC_SkipUntilString(mut source: *mut source_t,
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
    while 0 != PC_ReadToken(source, &mut token) {
        if 0 == strcmp(token.string.as_mut_ptr(), string) {
            return qtrue as libc::c_int
        }
    }
    return qfalse as libc::c_int;
}
//unread the last token read from the script
#[no_mangle]
pub unsafe extern "C" fn PC_UnreadLastToken(mut source: *mut source_t) {
    PC_UnreadSourceToken(source, &mut (*source).token);
}
//add a define to the source
#[no_mangle]
pub unsafe extern "C" fn PC_AddDefine(mut source: *mut source_t,
                                      mut string: *mut libc::c_char)
 -> libc::c_int {
    let mut define: *mut define_t = 0 as *mut define_t;
    define = PC_DefineFromString(string);
    if define.is_null() { return qfalse as libc::c_int }
    PC_AddDefineToHash(define, (*source).definehash);
    return qtrue as libc::c_int;
}
//end of the function PC_Directive_define
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_DefineFromString(mut string: *mut libc::c_char)
 -> *mut define_t {
    let mut script: *mut script_t = 0 as *mut script_t;
    let mut src: source_t =
        source_s{filename: [0; 1024],
                 includepath: [0; 1024],
                 punctuations: 0 as *mut punctuation_t,
                 scriptstack: 0 as *mut script_t,
                 tokens: 0 as *mut token_t,
                 defines: 0 as *mut define_t,
                 definehash: 0 as *mut *mut define_t,
                 indentstack: 0 as *mut indent_t,
                 skip: 0,
                 token:
                     token_s{string: [0; 1024],
                             type_0: 0,
                             subtype: 0,
                             intvalue: 0,
                             floatvalue: 0.,
                             whitespace_p: 0 as *mut libc::c_char,
                             endwhitespace_p: 0 as *mut libc::c_char,
                             line: 0,
                             linescrossed: 0,
                             next: 0 as *mut token_s,},};
    let mut t: *mut token_t = 0 as *mut token_t;
    let mut res: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut def: *mut define_t = 0 as *mut define_t;
    PC_InitTokenHeap();
    script =
        LoadScriptMemory(string, strlen(string) as libc::c_int,
                         b"*extern\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
    memset(&mut src as *mut source_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<source_t>() as libc::c_ulong);
    Q_strncpyz(src.filename.as_mut_ptr(),
               b"*extern\x00" as *const u8 as *const libc::c_char,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    src.scriptstack = script;
    src.definehash =
        GetClearedMemory((1024i32 as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut define_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut *mut define_t;
    res = PC_Directive_define(&mut src);
    t = src.tokens;
    while !t.is_null() {
        src.tokens = (*src.tokens).next;
        PC_FreeToken(t);
        t = src.tokens
    }
    def = 0 as *mut define_t;
    i = 0i32;
    while i < 1024i32 {
        if !(*src.definehash.offset(i as isize)).is_null() {
            def = *src.definehash.offset(i as isize);
            break ;
        } else { i += 1 }
    }
    FreeMemory(src.definehash as *mut libc::c_void);
    FreeScript(script);
    if res > 0i32 { return def }
    if !src.defines.is_null() { PC_FreeDefine(def); }
    return 0 as *mut define_t;
}
//end of the function PC_PushScript
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_InitTokenHeap() { }
//add a globals define that will be added to all opened sources
#[no_mangle]
pub unsafe extern "C" fn PC_AddGlobalDefine(mut string: *mut libc::c_char)
 -> libc::c_int {
    let mut define: *mut define_t = 0 as *mut define_t;
    define = PC_DefineFromString(string);
    if define.is_null() { return qfalse as libc::c_int }
    (*define).next = globaldefines;
    globaldefines = define;
    return qtrue as libc::c_int;
}
/*
int tokenheapinitialized;				//true when the token heap is initialized
token_t token_heap[TOKEN_HEAP_SIZE];	//heap with tokens
token_t *freetokens;					//free tokens from the heap
*/
//list with global defines added to every source loaded
#[no_mangle]
pub static mut globaldefines: *mut define_t =
    0 as *const define_t as *mut define_t;
//remove the given global define
#[no_mangle]
pub unsafe extern "C" fn PC_RemoveGlobalDefine(mut name: *mut libc::c_char)
 -> libc::c_int {
    let mut define: *mut define_t = 0 as *mut define_t;
    define = PC_FindDefine(globaldefines, name);
    if !define.is_null() {
        PC_FreeDefine(define);
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//end of the function PC_FindHashedDefine
//DEFINEHASHING
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_FindDefine(mut defines: *mut define_t,
                                       mut name: *mut libc::c_char)
 -> *mut define_t {
    let mut d: *mut define_t = 0 as *mut define_t;
    d = defines;
    while !d.is_null() {
        if 0 == strcmp((*d).name, name) { return d }
        d = (*d).next
    }
    return 0 as *mut define_t;
}
//remove all globals defines
#[no_mangle]
pub unsafe extern "C" fn PC_RemoveAllGlobalDefines() {
    let mut define: *mut define_t = 0 as *mut define_t;
    define = globaldefines;
    while !define.is_null() {
        globaldefines = (*globaldefines).next;
        PC_FreeDefine(define);
        define = globaldefines
    };
}
//add builtin defines
#[no_mangle]
pub unsafe extern "C" fn PC_AddBuiltinDefines(mut source: *mut source_t) {
    let mut i: libc::c_int = 0;
    let mut define: *mut define_t = 0 as *mut define_t;
    let mut builtin_0: [builtin; 5] =
        [builtin{string:
                     b"__LINE__\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 builtin: 1i32,},
         builtin{string:
                     b"__FILE__\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 builtin: 2i32,},
         builtin{string:
                     b"__DATE__\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 builtin: 3i32,},
         builtin{string:
                     b"__TIME__\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                 builtin: 4i32,},
         builtin{string: 0 as *mut libc::c_char, builtin: 0i32,}];
    i = 0i32;
    while !builtin_0[i as usize].string.is_null() {
        define =
            GetMemory(::std::mem::size_of::<define_t>() as libc::c_ulong) as
                *mut define_t;
        memset(define as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<define_t>() as libc::c_ulong);
        (*define).name =
            GetMemory(strlen(builtin_0[i as
                                           usize].string).wrapping_add(1i32 as
                                                                           libc::c_ulong))
                as *mut libc::c_char;
        strcpy((*define).name, builtin_0[i as usize].string);
        (*define).flags |= 0x1i32;
        (*define).builtin = builtin_0[i as usize].builtin;
        PC_AddDefineToHash(define, (*source).definehash);
        i += 1
    };
}
//set the source include path
#[no_mangle]
pub unsafe extern "C" fn PC_SetIncludePath(mut source: *mut source_t,
                                           mut path: *mut libc::c_char) {
    let mut len: size_t = 0;
    Q_strncpyz((*source).includepath.as_mut_ptr(), path,
               (::std::mem::size_of::<[libc::c_char; 1024]>() as
                    libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                   libc::c_int);
    len = strlen((*source).includepath.as_mut_ptr());
    if len > 0i32 as libc::c_ulong &&
           (*source).includepath[len.wrapping_sub(1i32 as libc::c_ulong) as
                                     usize] as libc::c_int != '\\' as i32 &&
           (*source).includepath[len.wrapping_sub(1i32 as libc::c_ulong) as
                                     usize] as libc::c_int != '/' as i32 {
        strcat((*source).includepath.as_mut_ptr(),
               b"/\x00" as *const u8 as *const libc::c_char);
    };
}
//set the punction set
#[no_mangle]
pub unsafe extern "C" fn PC_SetPunctuations(mut source: *mut source_t,
                                            mut p: *mut punctuation_t) {
    (*source).punctuations = p;
}
//set the base folder to load files from
#[no_mangle]
pub unsafe extern "C" fn PC_SetBaseFolder(mut path: *mut libc::c_char) {
    PS_SetBaseFolder(path);
}
//load a source file
#[no_mangle]
pub unsafe extern "C" fn LoadSourceFile(mut filename: *const libc::c_char)
 -> *mut source_t {
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut script: *mut script_t = 0 as *mut script_t;
    PC_InitTokenHeap();
    script = LoadScriptFile(filename);
    if script.is_null() { return 0 as *mut source_t }
    (*script).next = 0 as *mut script_s;
    source =
        GetMemory(::std::mem::size_of::<source_t>() as libc::c_ulong) as
            *mut source_t;
    memset(source as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<source_t>() as libc::c_ulong);
    Q_strncpyz((*source).filename.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    (*source).scriptstack = script;
    (*source).tokens = 0 as *mut token_t;
    (*source).defines = 0 as *mut define_t;
    (*source).indentstack = 0 as *mut indent_t;
    (*source).skip = 0i32;
    (*source).definehash =
        GetClearedMemory((1024i32 as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut define_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut *mut define_t;
    PC_AddGlobalDefinesToSource(source);
    return source;
}
//end of the function PC_CopyDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_AddGlobalDefinesToSource(mut source:
                                                         *mut source_t) {
    let mut define: *mut define_t = 0 as *mut define_t;
    let mut newdefine: *mut define_t = 0 as *mut define_t;
    define = globaldefines;
    while !define.is_null() {
        newdefine = PC_CopyDefine(source, define);
        PC_AddDefineToHash(newdefine, (*source).definehash);
        define = (*define).next
    };
}
//end for
//end of the function PC_RemoveAllGlobalDefines
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_CopyDefine(mut source: *mut source_t,
                                       mut define: *mut define_t)
 -> *mut define_t {
    let mut newdefine: *mut define_t = 0 as *mut define_t;
    let mut token: *mut token_t = 0 as *mut token_t;
    let mut newtoken: *mut token_t = 0 as *mut token_t;
    let mut lasttoken: *mut token_t = 0 as *mut token_t;
    newdefine =
        GetMemory(::std::mem::size_of::<define_t>() as libc::c_ulong) as
            *mut define_t;
    (*newdefine).name =
        GetMemory(strlen((*define).name).wrapping_add(1i32 as libc::c_ulong))
            as *mut libc::c_char;
    strcpy((*newdefine).name, (*define).name);
    (*newdefine).flags = (*define).flags;
    (*newdefine).builtin = (*define).builtin;
    (*newdefine).numparms = (*define).numparms;
    (*newdefine).next = 0 as *mut define_s;
    (*newdefine).hashnext = 0 as *mut define_s;
    (*newdefine).tokens = 0 as *mut token_t;
    lasttoken = 0 as *mut token_t;
    token = (*define).tokens;
    while !token.is_null() {
        newtoken = PC_CopyToken(token);
        (*newtoken).next = 0 as *mut token_s;
        if !lasttoken.is_null() {
            (*lasttoken).next = newtoken
        } else { (*newdefine).tokens = newtoken }
        lasttoken = newtoken;
        token = (*token).next
    }
    (*newdefine).parms = 0 as *mut token_t;
    lasttoken = 0 as *mut token_t;
    token = (*define).parms;
    while !token.is_null() {
        newtoken = PC_CopyToken(token);
        (*newtoken).next = 0 as *mut token_s;
        if !lasttoken.is_null() {
            (*lasttoken).next = newtoken
        } else { (*newdefine).parms = newtoken }
        lasttoken = newtoken;
        token = (*token).next
    }
    return newdefine;
}
//load a source from memory
#[no_mangle]
pub unsafe extern "C" fn LoadSourceMemory(mut ptr: *mut libc::c_char,
                                          mut length: libc::c_int,
                                          mut name: *mut libc::c_char)
 -> *mut source_t {
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut script: *mut script_t = 0 as *mut script_t;
    PC_InitTokenHeap();
    script = LoadScriptMemory(ptr, length, name);
    if script.is_null() { return 0 as *mut source_t }
    (*script).next = 0 as *mut script_s;
    source =
        GetMemory(::std::mem::size_of::<source_t>() as libc::c_ulong) as
            *mut source_t;
    memset(source as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<source_t>() as libc::c_ulong);
    Q_strncpyz((*source).filename.as_mut_ptr(), name,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    (*source).scriptstack = script;
    (*source).tokens = 0 as *mut token_t;
    (*source).defines = 0 as *mut define_t;
    (*source).indentstack = 0 as *mut indent_t;
    (*source).skip = 0i32;
    (*source).definehash =
        GetClearedMemory((1024i32 as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut define_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut *mut define_t;
    PC_AddGlobalDefinesToSource(source);
    return source;
}
//free the given source
#[no_mangle]
pub unsafe extern "C" fn FreeSource(mut source: *mut source_t) {
    let mut script: *mut script_t = 0 as *mut script_t;
    let mut token: *mut token_t = 0 as *mut token_t;
    let mut define: *mut define_t = 0 as *mut define_t;
    let mut indent: *mut indent_t = 0 as *mut indent_t;
    let mut i: libc::c_int = 0;
    while !(*source).scriptstack.is_null() {
        script = (*source).scriptstack;
        (*source).scriptstack = (*(*source).scriptstack).next;
        FreeScript(script);
    }
    while !(*source).tokens.is_null() {
        token = (*source).tokens;
        (*source).tokens = (*(*source).tokens).next;
        PC_FreeToken(token);
    }
    i = 0i32;
    while i < 1024i32 {
        while !(*(*source).definehash.offset(i as isize)).is_null() {
            define = *(*source).definehash.offset(i as isize);
            let ref mut fresh8 = *(*source).definehash.offset(i as isize);
            *fresh8 = (**(*source).definehash.offset(i as isize)).hashnext;
            PC_FreeDefine(define);
        }
        i += 1
    }
    while !(*source).indentstack.is_null() {
        indent = (*source).indentstack;
        (*source).indentstack = (*(*source).indentstack).next;
        FreeMemory(indent as *mut libc::c_void);
    }
    if !(*source).definehash.is_null() {
        FreeMemory((*source).definehash as *mut libc::c_void);
    }
    FreeMemory(source as *mut libc::c_void);
}
//BSPC
//
#[no_mangle]
pub unsafe extern "C" fn PC_LoadSourceHandle(mut filename:
                                                 *const libc::c_char)
 -> libc::c_int {
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < 64i32 {
        if sourceFiles[i as usize].is_null() { break ; }
        i += 1
    }
    if i >= 64i32 { return 0i32 }
    PS_SetBaseFolder(b"\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char);
    source = LoadSourceFile(filename);
    if source.is_null() { return 0i32 }
    sourceFiles[i as usize] = source;
    return i;
}
//end of the function FreeSource
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub static mut sourceFiles: [*mut source_t; 64] =
    [0 as *const source_t as *mut source_t; 64];
#[no_mangle]
pub unsafe extern "C" fn PC_FreeSourceHandle(mut handle: libc::c_int)
 -> libc::c_int {
    if handle < 1i32 || handle >= 64i32 { return qfalse as libc::c_int }
    if sourceFiles[handle as usize].is_null() { return qfalse as libc::c_int }
    FreeSource(sourceFiles[handle as usize]);
    sourceFiles[handle as usize] = 0 as *mut source_t;
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PC_ReadTokenHandle(mut handle: libc::c_int,
                                            mut pc_token: *mut pc_token_t)
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
    let mut ret: libc::c_int = 0;
    if handle < 1i32 || handle >= 64i32 { return 0i32 }
    if sourceFiles[handle as usize].is_null() { return 0i32 }
    ret = PC_ReadToken(sourceFiles[handle as usize], &mut token);
    strcpy((*pc_token).string.as_mut_ptr(), token.string.as_mut_ptr());
    (*pc_token).type_0 = token.type_0;
    (*pc_token).subtype = token.subtype;
    (*pc_token).intvalue = token.intvalue as libc::c_int;
    (*pc_token).floatvalue = token.floatvalue;
    if (*pc_token).type_0 == 1i32 {
        StripDoubleQuotes((*pc_token).string.as_mut_ptr());
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn PC_SourceFileAndLine(mut handle: libc::c_int,
                                              mut filename: *mut libc::c_char,
                                              mut line: *mut libc::c_int)
 -> libc::c_int {
    if handle < 1i32 || handle >= 64i32 { return qfalse as libc::c_int }
    if sourceFiles[handle as usize].is_null() { return qfalse as libc::c_int }
    strcpy(filename, (*sourceFiles[handle as usize]).filename.as_mut_ptr());
    if !(*sourceFiles[handle as usize]).scriptstack.is_null() {
        *line = (*(*sourceFiles[handle as usize]).scriptstack).line
    } else { *line = 0i32 }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PC_CheckOpenSourceHandles() {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < 64i32 {
        if !sourceFiles[i as usize].is_null() {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"file %s still open in precompiler\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                (*(*sourceFiles[i
                                                                                    as
                                                                                    usize]).scriptstack).filename.as_mut_ptr());
        }
        i += 1
    };
}
//end of the function PC_MergeTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
/*
void PC_PrintDefine(define_t *define)
{
	printf("define->name = %s\n", define->name);
	printf("define->flags = %d\n", define->flags);
	printf("define->builtin = %d\n", define->builtin);
	printf("define->numparms = %d\n", define->numparms);
//	token_t *parms;					//define parameters
//	token_t *tokens;					//macro tokens (possibly containing parm tokens)
//	struct define_s *next;			//next defined macro in a list
} //end of the function PC_PrintDefine*/
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]
pub unsafe extern "C" fn PC_PrintDefineHashTable(mut definehash:
                                                     *mut *mut define_t) {
    let mut i: libc::c_int = 0;
    let mut d: *mut define_t = 0 as *mut define_t;
    i = 0i32;
    while i < 1024i32 {
        Log_Write(b"%4d:\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, i);
        d = *definehash.offset(i as isize);
        while !d.is_null() {
            Log_Write(b" %s\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, (*d).name);
            d = (*d).hashnext
        }
        Log_Write(b"\n\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
        i += 1
    };
}