use libc;
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
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
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
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
 * name:		l_libvar.h
 *
 * desc:		botlib vars
 *
 * $Archive: /source/code/botlib/l_libvar.h $
 *
 *****************************************************************************/
    //library variable
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct libvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub value: libc::c_float,
        pub next: *mut libvar_s,
    }
    pub type libvar_t = libvar_s;
    use super::{libc};
    use super::q_shared_h::{qboolean};
    extern "C" {
        //creates the library variable if not existing already and returns it
        #[no_mangle]
        pub fn LibVar(var_name: *const libc::c_char,
                      value: *const libc::c_char) -> *mut libvar_t;
        //creates the library variable if not existing already and returns the value
        #[no_mangle]
        pub fn LibVarValue(var_name: *const libc::c_char,
                           value: *const libc::c_char) -> libc::c_float;
        //creates the library variable if not existing already and returns the value string
        #[no_mangle]
        pub fn LibVarString(var_name: *const libc::c_char,
                            value: *const libc::c_char) -> *mut libc::c_char;
        //sets the library variable
        #[no_mangle]
        pub fn LibVarSet(var_name: *const libc::c_char,
                         value: *const libc::c_char);
    }
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
        //remove any leading and trailing double quotes from the token
        #[no_mangle]
        pub fn StripDoubleQuotes(string: *mut libc::c_char);
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
    extern "C" {
        //read a token from the source
        #[no_mangle]
        pub fn PC_ReadToken(source: *mut source_t, token: *mut token_t)
         -> libc::c_int;
        //expect a certain token type
        #[no_mangle]
        pub fn PC_ExpectTokenType(source: *mut source_t, type_0: libc::c_int,
                                  subtype: libc::c_int, token: *mut token_t)
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_struct.h"]
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
    extern "C" {
        //read a structure from a script
        #[no_mangle]
        pub fn ReadStructure(source: *mut source_t, def: *mut structdef_t,
                             structure: *mut libc::c_char) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas.h"]
pub mod be_aas_h {
    /* Defined in botlib.h

//bsp_trace_t hit surface
typedef struct bsp_surface_s
{
	char name[16];
	int flags;
	int value;
} bsp_surface_t;

//a trace is returned when a box is swept through the BSP world
typedef struct bsp_trace_s
{
	qboolean		allsolid;	// if true, plane is not valid
	qboolean		startsolid;	// if true, the initial point was in a solid area
	float			fraction;	// time completed, 1.0 = didn't hit anything
	vec3_t			endpos;		// final position
	cplane_t		plane;		// surface normal at impact
	float			exp_dist;	// expanded plane distance
	int				sidenum;	// number of the brush side hit
	bsp_surface_t	surface;	// hit surface
	int				contents;	// contents on other side of surface hit
	int				ent;		// number of entity hit
} bsp_trace_t;
//
*/
    //entity info
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_entityinfo_s {
        pub valid: libc::c_int,
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub ltime: libc::c_float,
        pub update_time: libc::c_float,
        pub number: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
        pub lastvisorigin: vec3_t,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }
    pub type aas_entityinfo_t = aas_entityinfo_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_goal.h"]
pub mod be_ai_goal_h {
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
/*****************************************************************************
 * name:		be_ai_goal.h
 *
 * desc:		goal AI
 *
 * $Archive: /source/code/botlib/be_ai_goal.h $
 *
 *****************************************************************************/
    //a bot goal
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_goal_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub entitynum: libc::c_int,
        pub number: libc::c_int,
        pub flags: libc::c_int,
        pub iteminfo: libc::c_int,
    }
    pub type bot_goal_t = bot_goal_s;
    use super::q_shared_h::{vec3_t, vec_t};
    use super::{libc};
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
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_weight.h"]
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
    extern "C" {
        //reads a weight configuration
        #[no_mangle]
        pub fn ReadWeightConfig(filename: *mut libc::c_char)
         -> *mut weightconfig_t;
        //free a weight configuration
        #[no_mangle]
        pub fn FreeWeightConfig(config: *mut weightconfig_t);
        //find the fuzzy weight with the given name
        #[no_mangle]
        pub fn FindFuzzyWeight(wc: *mut weightconfig_t,
                               name: *mut libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn FuzzyWeightUndecided(inventory: *mut libc::c_int,
                                    wc: *mut weightconfig_t,
                                    weightnum: libc::c_int) -> libc::c_float;
        //evolves the weight configuration
        #[no_mangle]
        pub fn EvolveWeightConfig(config: *mut weightconfig_t);
        //interbreed the weight configurations and stores the interbreeded one in configout
        #[no_mangle]
        pub fn InterbreedWeightConfigs(config1: *mut weightconfig_t,
                                       config2: *mut weightconfig_t,
                                       configout: *mut weightconfig_t);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_goal.c"]
pub mod be_ai_goal_c {
    pub type bot_goalstate_t = bot_goalstate_s;
    //goal state
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_goalstate_s {
        pub itemweightconfig: *mut weightconfig_s,
        pub itemweightindex: *mut libc::c_int,
        pub client: libc::c_int,
        pub lastreachabilityarea: libc::c_int,
        pub goalstack: [bot_goal_t; 8],
        pub goalstacktop: libc::c_int,
        pub avoidgoals: [libc::c_int; 256],
        pub avoidgoaltimes: [libc::c_float; 256],
    }
    pub type iteminfo_t = iteminfo_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct iteminfo_s {
        pub classname: [libc::c_char; 32],
        pub name: [libc::c_char; 80],
        pub model: [libc::c_char; 80],
        pub modelindex: libc::c_int,
        pub type_0: libc::c_int,
        pub index: libc::c_int,
        pub respawntime: libc::c_float,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub number: libc::c_int,
    }
    pub type levelitem_t = levelitem_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct levelitem_s {
        pub number: libc::c_int,
        pub iteminfo: libc::c_int,
        pub flags: libc::c_int,
        pub weight: libc::c_float,
        pub origin: vec3_t,
        pub goalareanum: libc::c_int,
        pub goalorigin: vec3_t,
        pub entitynum: libc::c_int,
        pub timeout: libc::c_float,
        pub prev: *mut levelitem_s,
        pub next: *mut levelitem_s,
    }
    pub type itemconfig_t = itemconfig_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct itemconfig_s {
        pub numiteminfo: libc::c_int,
        pub iteminfo: *mut iteminfo_t,
    }
    //-- team games go after this --
    // team deathmatch
    pub const GT_TEAM: unnamed = 3;
    // single player tournament
    pub const GT_SINGLE_PLAYER: unnamed = 2;
    pub type campspot_t = campspot_s;
    //camp spots "info_camp"
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct campspot_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub name: [libc::c_char; 128],
        pub range: libc::c_float,
        pub weight: libc::c_float,
        pub wait: libc::c_float,
        pub random: libc::c_float,
        pub next: *mut campspot_s,
    }
    pub type maplocation_t = maplocation_s;
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
 * name:		be_ai_goal.c
 *
 * desc:		goal AI
 *
 * $Archive: /MissionPack/code/botlib/be_ai_goal.c $
 *
 *****************************************************************************/
    //#define DEBUG_AI_GOAL
    //RANDOMIZE
    //minimum avoid goal time
    //default avoid goal time
    //avoid dropped goal time
    //
    //item flags
    //not in free for all
    //not in team play
    //not in single player
    //bot should never go for this
    //bot roam goal
    //location in the map "target_location"
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct maplocation_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub name: [libc::c_char; 128],
        pub next: *mut maplocation_s,
    }
    //FIXME: these are game specific
    pub type unnamed = libc::c_uint;
    pub const GT_MAX_GAME_TYPE: unnamed = 5;
    // capture the flag
    pub const GT_CTF: unnamed = 4;
    // one on one tournament
    pub const GT_TOURNAMENT: unnamed = 1;
    // free for all
    pub const GT_FFA: unnamed = 0;
    use super::be_ai_weight_h::{weightconfig_s, weightconfig_t};
    use super::{libc};
    use super::be_ai_goal_h::{bot_goal_t};
    use super::q_shared_h::{vec3_t};
    use super::l_libvar_h::{libvar_t};
    use super::l_struct_h::{structdef_t, fielddef_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
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
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_variadic.h"]
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
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_main.h"]
pub mod be_aas_main_h {
    use super::{libc};
    extern "C" {
        //returns true if the AAS file is loaded
        #[no_mangle]
        pub fn AAS_Loaded() -> libc::c_int;
        //returns the current time
        #[no_mangle]
        pub fn AAS_Time() -> libc::c_float;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_entity.h"]
pub mod be_aas_entity_h {
    use super::{libc};
    use super::be_aas_h::{aas_entityinfo_t};
    extern "C" {
        //returns the info of the given entity
        #[no_mangle]
        pub fn AAS_EntityInfo(entnum: libc::c_int,
                              info: *mut aas_entityinfo_t);
        //returns the next entity
        #[no_mangle]
        pub fn AAS_NextEntity(entnum: libc::c_int) -> libc::c_int;
        //returns the entity type
        #[no_mangle]
        pub fn AAS_EntityType(entnum: libc::c_int) -> libc::c_int;
        //returns the model index of the entity
        #[no_mangle]
        pub fn AAS_EntityModelindex(entnum: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
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
 * name:		be_aas_sample.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_sample.h $
 *
 *****************************************************************************/
        //AASINTERN
        //returns the mins and maxs of the bounding box for the given presence type
        #[no_mangle]
        pub fn AAS_PresenceTypeBoundingBox(presencetype: libc::c_int,
                                           mins: *mut vec_t,
                                           maxs: *mut vec_t);
        //returns the area the point is in
        #[no_mangle]
        pub fn AAS_PointAreaNum(point: *mut vec_t) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_reach.h"]
pub mod be_aas_reach_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
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
 * name:		be_aas_reach.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_reach.h $
 *
 *****************************************************************************/
        //AASINTERN
        //returns true if the are has reachabilities to other areas
        #[no_mangle]
        pub fn AAS_AreaReachability(areanum: libc::c_int) -> libc::c_int;
        //returns the best reachable area and goal origin for a bounding box at the given origin
        #[no_mangle]
        pub fn AAS_BestReachableArea(origin: *mut vec_t, mins: *mut vec_t,
                                     maxs: *mut vec_t, goalorigin: *mut vec_t)
         -> libc::c_int;
        //returns the best jumppad area from which the bbox at origin is reachable
        #[no_mangle]
        pub fn AAS_BestReachableFromJumpPadArea(origin: *mut vec_t,
                                                mins: *mut vec_t,
                                                maxs: *mut vec_t)
         -> libc::c_int;
        //returns true if the area is a jump pad
        #[no_mangle]
        pub fn AAS_AreaJumpPad(areanum: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_route.h"]
pub mod be_aas_route_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //returns the travel time from the area to the goal area using the given travel flags
        #[no_mangle]
        pub fn AAS_AreaTravelTimeToGoalArea(areanum: libc::c_int,
                                            origin: *mut vec_t,
                                            goalareanum: libc::c_int,
                                            travelflags: libc::c_int)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::botlib_h::{bsp_trace_t};
    use super::q_shared_h::{vec_t};
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
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
        //AASINTERN
        //trace through the world
        #[no_mangle]
        pub fn AAS_Trace(start: *mut vec_t, mins: *mut vec_t,
                         maxs: *mut vec_t, end: *mut vec_t,
                         passent: libc::c_int, contentmask: libc::c_int)
         -> bsp_trace_t;
        //returns the contents at the given point
        #[no_mangle]
        pub fn AAS_PointContents(point: *mut vec_t) -> libc::c_int;
        //handle to the next bsp entity
        #[no_mangle]
        pub fn AAS_NextBSPEntity(ent: libc::c_int) -> libc::c_int;
        //return the value of the BSP epair key
        #[no_mangle]
        pub fn AAS_ValueForBSPEpairKey(ent: libc::c_int,
                                       key: *mut libc::c_char,
                                       value: *mut libc::c_char,
                                       size: libc::c_int) -> libc::c_int;
        //get a vector for the BSP epair key
        #[no_mangle]
        pub fn AAS_VectorForBSPEpairKey(ent: libc::c_int,
                                        key: *mut libc::c_char, v: *mut vec_t)
         -> libc::c_int;
        //get a float for the BSP epair key
        #[no_mangle]
        pub fn AAS_FloatForBSPEpairKey(ent: libc::c_int,
                                       key: *mut libc::c_char,
                                       value: *mut libc::c_float)
         -> libc::c_int;
        //get an integer for the BSP epair key
        #[no_mangle]
        pub fn AAS_IntForBSPEpairKey(ent: libc::c_int, key: *mut libc::c_char,
                                     value: *mut libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_move.h"]
pub mod be_aas_move_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //
        #[no_mangle]
        pub fn AAS_DropToFloor(origin: *mut vec_t, mins: *mut vec_t,
                               maxs: *mut vec_t) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
        //true if developer is on
        #[no_mangle]
        pub static mut botDeveloper: libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_move.h"]
pub mod be_ai_move_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //returns a reachability area if the origin is in one
        #[no_mangle]
        pub fn BotReachabilityArea(origin: *mut vec_t, client: libc::c_int)
         -> libc::c_int;
    }
}
use self::stddef_h::{size_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, Q_stricmp, Q_strncpyz};
use self::l_libvar_h::{libvar_s, libvar_t, LibVar, LibVarValue, LibVarString,
                       LibVarSet};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t, StripDoubleQuotes};
use self::l_precomp_h::{define_s, define_t, indent_s, indent_t, source_s,
                        source_t, PC_ReadToken, PC_ExpectTokenType,
                        PC_SetBaseFolder, LoadSourceFile, FreeSource};
use self::l_struct_h::{fielddef_s, structdef_s, fielddef_t, structdef_t,
                       ReadStructure};
use self::be_aas_h::{aas_entityinfo_s, aas_entityinfo_t};
use self::be_ai_goal_h::{bot_goal_s, bot_goal_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_ai_weight_h::{fuzzyseperator_s, weight_s, weight_t,
                           weightconfig_s, weightconfig_t, ReadWeightConfig,
                           FreeWeightConfig, FindFuzzyWeight,
                           FuzzyWeightUndecided, EvolveWeightConfig,
                           InterbreedWeightConfigs};
use self::be_ai_goal_c::{bot_goalstate_t, bot_goalstate_s, iteminfo_t,
                         iteminfo_s, levelitem_t, levelitem_s, itemconfig_t,
                         itemconfig_s, GT_TEAM, GT_SINGLE_PLAYER, campspot_t,
                         campspot_s, maplocation_t, maplocation_s, unnamed,
                         GT_MAX_GAME_TYPE, GT_CTF, GT_TOURNAMENT, GT_FFA};
use self::mathcalls_h::{sqrt};
use self::string_h::{memcpy, memset, strcpy, strcmp};
use self::l_memory_h::{GetClearedMemory, GetClearedHunkMemory, FreeMemory};
use self::l_log_h::{Log_Write};
use self::l_variadic_h::{SourceError};
use self::be_aas_main_h::{AAS_Loaded, AAS_Time};
use self::be_aas_entity_h::{AAS_EntityInfo, AAS_NextEntity, AAS_EntityType,
                            AAS_EntityModelindex};
use self::be_aas_sample_h::{AAS_PresenceTypeBoundingBox, AAS_PointAreaNum};
use self::be_aas_reach_h::{AAS_AreaReachability, AAS_BestReachableArea,
                           AAS_BestReachableFromJumpPadArea, AAS_AreaJumpPad};
use self::be_aas_route_h::{AAS_AreaTravelTimeToGoalArea};
use self::be_aas_bsp_h::{AAS_Trace, AAS_PointContents, AAS_NextBSPEntity,
                         AAS_ValueForBSPEpairKey, AAS_VectorForBSPEpairKey,
                         AAS_FloatForBSPEpairKey, AAS_IntForBSPEpairKey};
use self::be_aas_move_h::{AAS_DropToFloor};
use self::be_interface_h::{botimport, botDeveloper};
use self::be_ai_move_h::{BotReachabilityArea};
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt((*v.offset(0isize) * *v.offset(0isize) +
                     *v.offset(1isize) * *v.offset(1isize) +
                     *v.offset(2isize) * *v.offset(2isize)) as libc::c_double)
               as vec_t;
}
//reset the whole goal state, but keep the item weights
#[no_mangle]
pub unsafe extern "C" fn BotResetGoalState(mut goalstate: libc::c_int) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    memset((*gs).goalstack.as_mut_ptr() as *mut libc::c_void, 0i32,
           (8i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<bot_goal_t>()
                                                as libc::c_ulong));
    (*gs).goalstacktop = 0i32;
    BotResetAvoidGoals(goalstate);
}
//reset avoid goals
#[no_mangle]
pub unsafe extern "C" fn BotResetAvoidGoals(mut goalstate: libc::c_int) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    memset((*gs).avoidgoals.as_mut_ptr() as *mut libc::c_void, 0i32,
           (256i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong));
    memset((*gs).avoidgoaltimes.as_mut_ptr() as *mut libc::c_void, 0i32,
           (256i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_float>()
                                                as libc::c_ulong));
}
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotGoalStateFromHandle(mut handle: libc::c_int)
 -> *mut bot_goalstate_t {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"goal state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_goalstate_t
    }
    if botgoalstates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid goal state %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_goalstate_t
    }
    return botgoalstates[handle as usize];
}
// FIXME: init?
#[no_mangle]
pub static mut botgoalstates: [*mut bot_goalstate_t; 65] =
    [0 as *const bot_goalstate_t as *mut bot_goalstate_t; 65];
//remove the goal with the given number from the avoid goals
#[no_mangle]
pub unsafe extern "C" fn BotRemoveFromAvoidGoals(mut goalstate: libc::c_int,
                                                 mut number: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    i = 0i32;
    while i < 256i32 {
        if (*gs).avoidgoals[i as usize] == number &&
               (*gs).avoidgoaltimes[i as usize] >= AAS_Time() {
            (*gs).avoidgoaltimes[i as usize] = 0i32 as libc::c_float;
            return
        }
        i += 1
    };
}
//push a goal onto the goal stack
#[no_mangle]
pub unsafe extern "C" fn BotPushGoal(mut goalstate: libc::c_int,
                                     mut goal: *mut bot_goal_t) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    if (*gs).goalstacktop >= 8i32 - 1i32 {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"goal heap overflow\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        BotDumpGoalStack(goalstate);
        return
    }
    (*gs).goalstacktop += 1;
    memcpy(&mut (*gs).goalstack[(*gs).goalstacktop as usize] as
               *mut bot_goal_t as *mut libc::c_void,
           goal as *const libc::c_void,
           ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong);
}
//dump the goal stack
#[no_mangle]
pub unsafe extern "C" fn BotDumpGoalStack(mut goalstate: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    let mut name: [libc::c_char; 32] = [0; 32];
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    i = 1i32;
    while i <= (*gs).goalstacktop {
        BotGoalName((*gs).goalstack[i as usize].number, name.as_mut_ptr(),
                    32i32);
        Log_Write(b"%d: %s\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, i, name.as_mut_ptr());
        i += 1
    };
}
//get the name name of the goal with the given number
#[no_mangle]
pub unsafe extern "C" fn BotGoalName(mut number: libc::c_int,
                                     mut name: *mut libc::c_char,
                                     mut size: libc::c_int) {
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    if itemconfig.is_null() { return }
    li = levelitems;
    while !li.is_null() {
        if (*li).number == number {
            Q_strncpyz(name,
                       (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                           isize)).name.as_mut_ptr(),
                       size);
            return
        }
        li = (*li).next
    }
    strcpy(name, b"\x00" as *const u8 as *const libc::c_char);
}
//item configuration
#[no_mangle]
pub static mut itemconfig: *mut itemconfig_t =
    0 as *const itemconfig_t as *mut itemconfig_t;
#[no_mangle]
pub static mut levelitems: *mut levelitem_t =
    0 as *const levelitem_t as *mut levelitem_t;
//pop a goal from the goal stack
#[no_mangle]
pub unsafe extern "C" fn BotPopGoal(mut goalstate: libc::c_int) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    if (*gs).goalstacktop > 0i32 { (*gs).goalstacktop -= 1 };
}
//empty the bot's goal stack
#[no_mangle]
pub unsafe extern "C" fn BotEmptyGoalStack(mut goalstate: libc::c_int) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    (*gs).goalstacktop = 0i32;
}
//dump the avoid goals
#[no_mangle]
pub unsafe extern "C" fn BotDumpAvoidGoals(mut goalstate: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    let mut name: [libc::c_char; 32] = [0; 32];
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    i = 0i32;
    while i < 256i32 {
        if (*gs).avoidgoaltimes[i as usize] >= AAS_Time() {
            BotGoalName((*gs).avoidgoals[i as usize], name.as_mut_ptr(),
                        32i32);
            Log_Write(b"avoid goal %s, number %d for %f seconds\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, name.as_mut_ptr(),
                      (*gs).avoidgoals[i as usize],
                      ((*gs).avoidgoaltimes[i as usize] - AAS_Time()) as
                          libc::c_double);
        }
        i += 1
    };
}
//get the top goal from the stack
#[no_mangle]
pub unsafe extern "C" fn BotGetTopGoal(mut goalstate: libc::c_int,
                                       mut goal: *mut bot_goal_t)
 -> libc::c_int {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return qfalse as libc::c_int }
    if 0 == (*gs).goalstacktop { return qfalse as libc::c_int }
    memcpy(goal as *mut libc::c_void,
           &mut (*gs).goalstack[(*gs).goalstacktop as usize] as
               *mut bot_goal_t as *const libc::c_void,
           ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong);
    return qtrue as libc::c_int;
}
//get the second goal on the stack
#[no_mangle]
pub unsafe extern "C" fn BotGetSecondGoal(mut goalstate: libc::c_int,
                                          mut goal: *mut bot_goal_t)
 -> libc::c_int {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return qfalse as libc::c_int }
    if (*gs).goalstacktop <= 1i32 { return qfalse as libc::c_int }
    memcpy(goal as *mut libc::c_void,
           &mut (*gs).goalstack[((*gs).goalstacktop - 1i32) as usize] as
               *mut bot_goal_t as *const libc::c_void,
           ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong);
    return qtrue as libc::c_int;
}
//choose the best long term goal item for the bot
#[no_mangle]
pub unsafe extern "C" fn BotChooseLTGItem(mut goalstate: libc::c_int,
                                          mut origin: *mut vec_t,
                                          mut inventory: *mut libc::c_int,
                                          mut travelflags: libc::c_int)
 -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut weightnum: libc::c_int = 0;
    let mut weight: libc::c_float = 0.;
    let mut bestweight: libc::c_float = 0.;
    let mut avoidtime: libc::c_float = 0.;
    let mut iteminfo: *mut iteminfo_t = 0 as *mut iteminfo_t;
    let mut ic: *mut itemconfig_t = 0 as *mut itemconfig_t;
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    let mut bestitem: *mut levelitem_t = 0 as *mut levelitem_t;
    let mut goal: bot_goal_t =
        bot_goal_s{origin: [0.; 3],
                   areanum: 0,
                   mins: [0.; 3],
                   maxs: [0.; 3],
                   entitynum: 0,
                   number: 0,
                   flags: 0,
                   iteminfo: 0,};
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return qfalse as libc::c_int }
    if (*gs).itemweightconfig.is_null() { return qfalse as libc::c_int }
    areanum = BotReachabilityArea(origin, (*gs).client);
    if 0 == areanum || 0 == AAS_AreaReachability(areanum) {
        areanum = (*gs).lastreachabilityarea
    }
    (*gs).lastreachabilityarea = areanum;
    if 0 == areanum { return qfalse as libc::c_int }
    ic = itemconfig;
    if itemconfig.is_null() { return qfalse as libc::c_int }
    bestweight = 0i32 as libc::c_float;
    bestitem = 0 as *mut levelitem_t;
    memset(&mut goal as *mut bot_goal_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong);
    let mut current_block_32: u64;
    li = levelitems;
    while !li.is_null() {
        if g_gametype == GT_SINGLE_PLAYER as libc::c_int {
            if 0 != (*li).flags & 4i32 {
                current_block_32 = 2838571290723028321;
            } else { current_block_32 = 3437258052017859086; }
        } else if g_gametype >= GT_TEAM as libc::c_int {
            if 0 != (*li).flags & 2i32 {
                current_block_32 = 2838571290723028321;
            } else { current_block_32 = 3437258052017859086; }
        } else if 0 != (*li).flags & 1i32 {
            current_block_32 = 2838571290723028321;
        } else { current_block_32 = 3437258052017859086; }
        match current_block_32 {
            3437258052017859086 => {
                if !(0 != (*li).flags & 8i32) {
                    //if the item is not in a possible goal area
                    if !(0 == (*li).goalareanum) {
                        //FIXME: is this a good thing? added this for items that never spawned into the game (f.i. CTF flags in obelisk)
                        if !(0 == (*li).entitynum && 0 == (*li).flags & 16i32)
                           {
                            iteminfo =
                                &mut *(*ic).iteminfo.offset((*li).iteminfo as
                                                                isize) as
                                    *mut iteminfo_t;
                            weightnum =
                                *(*gs).itemweightindex.offset((*iteminfo).number
                                                                  as isize);
                            if !(weightnum < 0i32) {
                                weight =
                                    FuzzyWeightUndecided(inventory,
                                                         (*gs).itemweightconfig,
                                                         weightnum);
                                if 0. != (*li).timeout {
                                    weight += (*droppedweight).value
                                }
                                if 0 != (*li).flags & 16i32 {
                                    weight *= (*li).weight
                                }
                                //
                                if weight > 0i32 as libc::c_float {
                                    t =
                                        AAS_AreaTravelTimeToGoalArea(areanum,
                                                                     origin,
                                                                     (*li).goalareanum,
                                                                     travelflags);
                                    //if the goal is reachable
                                    if t > 0i32 {
                                        avoidtime =
                                            BotAvoidGoalTime(goalstate,
                                                             (*li).number);
                                        if !(avoidtime as libc::c_double -
                                                 t as libc::c_double *
                                                     0.009f64 >
                                                 0i32 as libc::c_double) {
                                            weight =
                                                (weight as libc::c_double /
                                                     (t as libc::c_float as
                                                          libc::c_double *
                                                          0.01f64)) as
                                                    libc::c_float;
                                            if weight > bestweight {
                                                bestweight = weight;
                                                bestitem = li
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        li = (*li).next
    }
    if bestitem.is_null() { return qfalse as libc::c_int }
    iteminfo =
        &mut *(*ic).iteminfo.offset((*bestitem).iteminfo as isize) as
            *mut iteminfo_t;
    goal.origin[0usize] = (*bestitem).goalorigin[0usize];
    goal.origin[1usize] = (*bestitem).goalorigin[1usize];
    goal.origin[2usize] = (*bestitem).goalorigin[2usize];
    goal.mins[0usize] = (*iteminfo).mins[0usize];
    goal.mins[1usize] = (*iteminfo).mins[1usize];
    goal.mins[2usize] = (*iteminfo).mins[2usize];
    goal.maxs[0usize] = (*iteminfo).maxs[0usize];
    goal.maxs[1usize] = (*iteminfo).maxs[1usize];
    goal.maxs[2usize] = (*iteminfo).maxs[2usize];
    goal.areanum = (*bestitem).goalareanum;
    goal.entitynum = (*bestitem).entitynum;
    goal.number = (*bestitem).number;
    goal.flags = 1i32;
    if 0. != (*bestitem).timeout { goal.flags |= 4i32 }
    if 0 != (*bestitem).flags & 16i32 { goal.flags |= 2i32 }
    goal.iteminfo = (*bestitem).iteminfo;
    if 0. != (*bestitem).timeout {
        avoidtime = 10i32 as libc::c_float
    } else {
        avoidtime = (*iteminfo).respawntime;
        if 0. == avoidtime { avoidtime = 30i32 as libc::c_float }
        if avoidtime < 10i32 as libc::c_float {
            avoidtime = 10i32 as libc::c_float
        }
    }
    BotAddToAvoidGoals(gs, (*bestitem).number, avoidtime);
    BotPushGoal(goalstate, &mut goal);
    return qtrue as libc::c_int;
}
//end if
//end for
//end of the function BotDumpAvoidGoals
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotAddToAvoidGoals(mut gs: *mut bot_goalstate_t,
                                            mut number: libc::c_int,
                                            mut avoidtime: libc::c_float) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 256i32 {
        if (*gs).avoidgoals[i as usize] == number {
            (*gs).avoidgoals[i as usize] = number;
            (*gs).avoidgoaltimes[i as usize] = AAS_Time() + avoidtime;
            return
        }
        i += 1
    }
    i = 0i32;
    while i < 256i32 {
        if (*gs).avoidgoaltimes[i as usize] < AAS_Time() {
            (*gs).avoidgoals[i as usize] = number;
            (*gs).avoidgoaltimes[i as usize] = AAS_Time() + avoidtime;
            return
        }
        i += 1
    };
}
//returns the avoid goal time
#[no_mangle]
pub unsafe extern "C" fn BotAvoidGoalTime(mut goalstate: libc::c_int,
                                          mut number: libc::c_int)
 -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return 0i32 as libc::c_float }
    i = 0i32;
    while i < 256i32 {
        if (*gs).avoidgoals[i as usize] == number &&
               (*gs).avoidgoaltimes[i as usize] >= AAS_Time() {
            return (*gs).avoidgoaltimes[i as usize] - AAS_Time()
        }
        i += 1
    }
    return 0i32 as libc::c_float;
}
//additional dropped item weight
#[no_mangle]
pub static mut droppedweight: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//the game type
#[no_mangle]
pub static mut g_gametype: libc::c_int = 0i32;
//choose the best nearby goal item for the bot
//the item may not be further away from the current bot position than maxtime
//also the travel time from the nearby goal towards the long term goal may not
//be larger than the travel time towards the long term goal from the current bot position
#[no_mangle]
pub unsafe extern "C" fn BotChooseNBGItem(mut goalstate: libc::c_int,
                                          mut origin: *mut vec_t,
                                          mut inventory: *mut libc::c_int,
                                          mut travelflags: libc::c_int,
                                          mut ltg: *mut bot_goal_t,
                                          mut maxtime: libc::c_float)
 -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut weightnum: libc::c_int = 0;
    let mut ltg_time: libc::c_int = 0;
    let mut weight: libc::c_float = 0.;
    let mut bestweight: libc::c_float = 0.;
    let mut avoidtime: libc::c_float = 0.;
    let mut iteminfo: *mut iteminfo_t = 0 as *mut iteminfo_t;
    let mut ic: *mut itemconfig_t = 0 as *mut itemconfig_t;
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    let mut bestitem: *mut levelitem_t = 0 as *mut levelitem_t;
    let mut goal: bot_goal_t =
        bot_goal_s{origin: [0.; 3],
                   areanum: 0,
                   mins: [0.; 3],
                   maxs: [0.; 3],
                   entitynum: 0,
                   number: 0,
                   flags: 0,
                   iteminfo: 0,};
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return qfalse as libc::c_int }
    if (*gs).itemweightconfig.is_null() { return qfalse as libc::c_int }
    areanum = BotReachabilityArea(origin, (*gs).client);
    if 0 == areanum || 0 == AAS_AreaReachability(areanum) {
        areanum = (*gs).lastreachabilityarea
    }
    (*gs).lastreachabilityarea = areanum;
    if 0 == areanum { return qfalse as libc::c_int }
    if !ltg.is_null() {
        ltg_time =
            AAS_AreaTravelTimeToGoalArea(areanum, origin, (*ltg).areanum,
                                         travelflags)
    } else { ltg_time = 99999i32 }
    ic = itemconfig;
    if itemconfig.is_null() { return qfalse as libc::c_int }
    bestweight = 0i32 as libc::c_float;
    bestitem = 0 as *mut levelitem_t;
    memset(&mut goal as *mut bot_goal_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong);
    let mut current_block_41: u64;
    li = levelitems;
    while !li.is_null() {
        if g_gametype == GT_SINGLE_PLAYER as libc::c_int {
            if 0 != (*li).flags & 4i32 {
                current_block_41 = 18317007320854588510;
            } else { current_block_41 = 11636175345244025579; }
        } else if g_gametype >= GT_TEAM as libc::c_int {
            if 0 != (*li).flags & 2i32 {
                current_block_41 = 18317007320854588510;
            } else { current_block_41 = 11636175345244025579; }
        } else if 0 != (*li).flags & 1i32 {
            current_block_41 = 18317007320854588510;
        } else { current_block_41 = 11636175345244025579; }
        match current_block_41 {
            11636175345244025579 => {
                if !(0 != (*li).flags & 8i32) {
                    //if the item is in a possible goal area
                    if !(0 == (*li).goalareanum) {
                        //FIXME: is this a good thing? added this for items that never spawned into the game (f.i. CTF flags in obelisk)
                        if !(0 == (*li).entitynum && 0 == (*li).flags & 16i32)
                           {
                            iteminfo =
                                &mut *(*ic).iteminfo.offset((*li).iteminfo as
                                                                isize) as
                                    *mut iteminfo_t;
                            weightnum =
                                *(*gs).itemweightindex.offset((*iteminfo).number
                                                                  as isize);
                            if !(weightnum < 0i32) {
                                weight =
                                    FuzzyWeightUndecided(inventory,
                                                         (*gs).itemweightconfig,
                                                         weightnum);
                                if 0. != (*li).timeout {
                                    weight += (*droppedweight).value
                                }
                                if 0 != (*li).flags & 16i32 {
                                    weight *= (*li).weight
                                }
                                //
                                if weight > 0i32 as libc::c_float {
                                    t =
                                        AAS_AreaTravelTimeToGoalArea(areanum,
                                                                     origin,
                                                                     (*li).goalareanum,
                                                                     travelflags);
                                    //if the goal is reachable
                                    if t > 0i32 &&
                                           (t as libc::c_float) < maxtime {
                                        avoidtime =
                                            BotAvoidGoalTime(goalstate,
                                                             (*li).number);
                                        if !(avoidtime as libc::c_double -
                                                 t as libc::c_double *
                                                     0.009f64 >
                                                 0i32 as libc::c_double) {
                                            weight =
                                                (weight as libc::c_double /
                                                     (t as libc::c_float as
                                                          libc::c_double *
                                                          0.01f64)) as
                                                    libc::c_float;
                                            if weight > bestweight {
                                                t = 0i32;
                                                if !ltg.is_null() &&
                                                       0. == (*li).timeout {
                                                    t =
                                                        AAS_AreaTravelTimeToGoalArea((*li).goalareanum,
                                                                                     (*li).goalorigin.as_mut_ptr(),
                                                                                     (*ltg).areanum,
                                                                                     travelflags)
                                                }
                                                if t <= ltg_time {
                                                    bestweight = weight;
                                                    bestitem = li
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => { }
        }
        li = (*li).next
    }
    if bestitem.is_null() { return qfalse as libc::c_int }
    iteminfo =
        &mut *(*ic).iteminfo.offset((*bestitem).iteminfo as isize) as
            *mut iteminfo_t;
    goal.origin[0usize] = (*bestitem).goalorigin[0usize];
    goal.origin[1usize] = (*bestitem).goalorigin[1usize];
    goal.origin[2usize] = (*bestitem).goalorigin[2usize];
    goal.mins[0usize] = (*iteminfo).mins[0usize];
    goal.mins[1usize] = (*iteminfo).mins[1usize];
    goal.mins[2usize] = (*iteminfo).mins[2usize];
    goal.maxs[0usize] = (*iteminfo).maxs[0usize];
    goal.maxs[1usize] = (*iteminfo).maxs[1usize];
    goal.maxs[2usize] = (*iteminfo).maxs[2usize];
    goal.areanum = (*bestitem).goalareanum;
    goal.entitynum = (*bestitem).entitynum;
    goal.number = (*bestitem).number;
    goal.flags = 1i32;
    if 0. != (*bestitem).timeout { goal.flags |= 4i32 }
    if 0 != (*bestitem).flags & 16i32 { goal.flags |= 2i32 }
    goal.iteminfo = (*bestitem).iteminfo;
    if 0. != (*bestitem).timeout {
        avoidtime = 10i32 as libc::c_float
    } else {
        avoidtime = (*iteminfo).respawntime;
        if 0. == avoidtime { avoidtime = 30i32 as libc::c_float }
        if avoidtime < 10i32 as libc::c_float {
            avoidtime = 10i32 as libc::c_float
        }
    }
    BotAddToAvoidGoals(gs, (*bestitem).number, avoidtime);
    BotPushGoal(goalstate, &mut goal);
    return qtrue as libc::c_int;
}
//returns true if the bot touches the goal
#[no_mangle]
pub unsafe extern "C" fn BotTouchingGoal(mut origin: *mut vec_t,
                                         mut goal: *mut bot_goal_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut boxmins: vec3_t = [0.; 3];
    let mut boxmaxs: vec3_t = [0.; 3];
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    //{4, 4, 10};
    let mut safety_maxs: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    //{-4, -4, 0};
    let mut safety_mins: vec3_t =
        [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
    AAS_PresenceTypeBoundingBox(2i32, boxmins.as_mut_ptr(),
                                boxmaxs.as_mut_ptr());
    absmins[0usize] = (*goal).mins[0usize] - boxmaxs[0usize];
    absmins[1usize] = (*goal).mins[1usize] - boxmaxs[1usize];
    absmins[2usize] = (*goal).mins[2usize] - boxmaxs[2usize];
    absmaxs[0usize] = (*goal).maxs[0usize] - boxmins[0usize];
    absmaxs[1usize] = (*goal).maxs[1usize] - boxmins[1usize];
    absmaxs[2usize] = (*goal).maxs[2usize] - boxmins[2usize];
    absmins[0usize] = absmins[0usize] + (*goal).origin[0usize];
    absmins[1usize] = absmins[1usize] + (*goal).origin[1usize];
    absmins[2usize] = absmins[2usize] + (*goal).origin[2usize];
    absmaxs[0usize] = absmaxs[0usize] + (*goal).origin[0usize];
    absmaxs[1usize] = absmaxs[1usize] + (*goal).origin[1usize];
    absmaxs[2usize] = absmaxs[2usize] + (*goal).origin[2usize];
    absmaxs[0usize] = absmaxs[0usize] - safety_maxs[0usize];
    absmaxs[1usize] = absmaxs[1usize] - safety_maxs[1usize];
    absmaxs[2usize] = absmaxs[2usize] - safety_maxs[2usize];
    absmins[0usize] = absmins[0usize] - safety_mins[0usize];
    absmins[1usize] = absmins[1usize] - safety_mins[1usize];
    absmins[2usize] = absmins[2usize] - safety_mins[2usize];
    i = 0i32;
    while i < 3i32 {
        if *origin.offset(i as isize) < absmins[i as usize] ||
               *origin.offset(i as isize) > absmaxs[i as usize] {
            return qfalse as libc::c_int
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
//returns true if the goal should be visible but isn't
#[no_mangle]
pub unsafe extern "C" fn BotItemGoalInVisButNotVisible(mut viewer:
                                                           libc::c_int,
                                                       mut eye: *mut vec_t,
                                                       mut viewangles:
                                                           *mut vec_t,
                                                       mut goal:
                                                           *mut bot_goal_t)
 -> libc::c_int {
    let mut entinfo: aas_entityinfo_t =
        aas_entityinfo_s{valid: 0,
                         type_0: 0,
                         flags: 0,
                         ltime: 0.,
                         update_time: 0.,
                         number: 0,
                         origin: [0.; 3],
                         angles: [0.; 3],
                         old_origin: [0.; 3],
                         lastvisorigin: [0.; 3],
                         mins: [0.; 3],
                         maxs: [0.; 3],
                         groundent: 0,
                         solid: 0,
                         modelindex: 0,
                         modelindex2: 0,
                         frame: 0,
                         event: 0,
                         eventParm: 0,
                         powerups: 0,
                         weapon: 0,
                         legsAnim: 0,
                         torsoAnim: 0,};
    let mut trace: bsp_trace_t =
        bsp_trace_s{allsolid: qfalse,
                    startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    plane:
                        cplane_s{normal: [0.; 3],
                                 dist: 0.,
                                 type_0: 0,
                                 signbits: 0,
                                 pad: [0; 2],},
                    exp_dist: 0.,
                    sidenum: 0,
                    surface:
                        bsp_surface_s{name: [0; 16], flags: 0, value: 0,},
                    contents: 0,
                    ent: 0,};
    let mut middle: vec3_t = [0.; 3];
    if 0 == (*goal).flags & 1i32 { return qfalse as libc::c_int }
    middle[0usize] = (*goal).mins[0usize] + (*goal).mins[0usize];
    middle[1usize] = (*goal).mins[1usize] + (*goal).mins[1usize];
    middle[2usize] = (*goal).mins[2usize] + (*goal).mins[2usize];
    middle[0usize] = (middle[0usize] as libc::c_double * 0.5f64) as vec_t;
    middle[1usize] = (middle[1usize] as libc::c_double * 0.5f64) as vec_t;
    middle[2usize] = (middle[2usize] as libc::c_double * 0.5f64) as vec_t;
    middle[0usize] = (*goal).origin[0usize] + middle[0usize];
    middle[1usize] = (*goal).origin[1usize] + middle[1usize];
    middle[2usize] = (*goal).origin[2usize] + middle[2usize];
    trace =
        AAS_Trace(eye, 0 as *mut vec_t, 0 as *mut vec_t, middle.as_mut_ptr(),
                  viewer, 1i32);
    if trace.fraction >= 1i32 as libc::c_float {
        if (*goal).entitynum <= 0i32 { return qfalse as libc::c_int }
        AAS_EntityInfo((*goal).entitynum, &mut entinfo);
        if (entinfo.ltime as libc::c_double) <
               AAS_Time() as libc::c_double - 0.5f64 {
            return qtrue as libc::c_int
        }
    }
    return qfalse as libc::c_int;
}
//search for a goal for the given classname, the index can be used
//as a start point for the search when multiple goals are available with that same classname
#[no_mangle]
pub unsafe extern "C" fn BotGetLevelItemGoal(mut index: libc::c_int,
                                             mut name: *mut libc::c_char,
                                             mut goal: *mut bot_goal_t)
 -> libc::c_int {
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    if itemconfig.is_null() { return -1i32 }
    li = levelitems;
    if index >= 0i32 {
        while !li.is_null() {
            if (*li).number == index {
                li = (*li).next;
                break ;
            } else { li = (*li).next }
        }
    }
    let mut current_block_19: u64;
    while !li.is_null() {
        //
        if g_gametype == GT_SINGLE_PLAYER as libc::c_int {
            if 0 != (*li).flags & 4i32 {
                current_block_19 = 11650488183268122163;
            } else { current_block_19 = 6057473163062296781; }
        } else if g_gametype >= GT_TEAM as libc::c_int {
            if 0 != (*li).flags & 2i32 {
                current_block_19 = 11650488183268122163;
            } else { current_block_19 = 6057473163062296781; }
        } else if 0 != (*li).flags & 1i32 {
            current_block_19 = 11650488183268122163;
        } else { current_block_19 = 6057473163062296781; }
        match current_block_19 {
            6057473163062296781 => {
                if !(0 != (*li).flags & 8i32) {
                    if 0 ==
                           Q_stricmp(name,
                                     (*(*itemconfig).iteminfo.offset((*li).iteminfo
                                                                         as
                                                                         isize)).name.as_mut_ptr())
                       {
                        (*goal).areanum = (*li).goalareanum;
                        (*goal).origin[0usize] = (*li).goalorigin[0usize];
                        (*goal).origin[1usize] = (*li).goalorigin[1usize];
                        (*goal).origin[2usize] = (*li).goalorigin[2usize];
                        (*goal).entitynum = (*li).entitynum;
                        (*goal).mins[0usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                                isize)).mins[0usize];
                        (*goal).mins[1usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                                isize)).mins[1usize];
                        (*goal).mins[2usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                                isize)).mins[2usize];
                        (*goal).maxs[0usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                                isize)).maxs[0usize];
                        (*goal).maxs[1usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                                isize)).maxs[1usize];
                        (*goal).maxs[2usize] =
                            (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                                isize)).maxs[2usize];
                        (*goal).number = (*li).number;
                        (*goal).flags = 1i32;
                        if 0. != (*li).timeout { (*goal).flags |= 4i32 }
                        (*goal).iteminfo = (*li).iteminfo;
                        return (*li).number
                    }
                }
            }
            _ => { }
        }
        li = (*li).next
    }
    return -1i32;
}
//get the next camp spot in the map
#[no_mangle]
pub unsafe extern "C" fn BotGetNextCampSpotGoal(mut num: libc::c_int,
                                                mut goal: *mut bot_goal_t)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cs: *mut campspot_t = 0 as *mut campspot_t;
    let mut mins: vec3_t = [-8i32 as vec_t, -8i32 as vec_t, -8i32 as vec_t];
    let mut maxs: vec3_t = [8i32 as vec_t, 8i32 as vec_t, 8i32 as vec_t];
    if num < 0i32 { num = 0i32 }
    i = num;
    cs = campspots;
    while !cs.is_null() {
        i -= 1;
        if i < 0i32 {
            (*goal).areanum = (*cs).areanum;
            (*goal).origin[0usize] = (*cs).origin[0usize];
            (*goal).origin[1usize] = (*cs).origin[1usize];
            (*goal).origin[2usize] = (*cs).origin[2usize];
            (*goal).entitynum = 0i32;
            (*goal).mins[0usize] = mins[0usize];
            (*goal).mins[1usize] = mins[1usize];
            (*goal).mins[2usize] = mins[2usize];
            (*goal).maxs[0usize] = maxs[0usize];
            (*goal).maxs[1usize] = maxs[1usize];
            (*goal).maxs[2usize] = maxs[2usize];
            (*goal).number = 0i32;
            (*goal).flags = 0i32;
            (*goal).iteminfo = 0i32;
            return num + 1i32
        }
        cs = (*cs).next
    }
    return 0i32;
}
//camp spots
#[no_mangle]
pub static mut campspots: *mut campspot_t =
    0 as *const campspot_t as *mut campspot_t;
//get the map location with the given name
#[no_mangle]
pub unsafe extern "C" fn BotGetMapLocationGoal(mut name: *mut libc::c_char,
                                               mut goal: *mut bot_goal_t)
 -> libc::c_int {
    let mut ml: *mut maplocation_t = 0 as *mut maplocation_t;
    let mut mins: vec3_t = [-8i32 as vec_t, -8i32 as vec_t, -8i32 as vec_t];
    let mut maxs: vec3_t = [8i32 as vec_t, 8i32 as vec_t, 8i32 as vec_t];
    ml = maplocations;
    while !ml.is_null() {
        if 0 == Q_stricmp((*ml).name.as_mut_ptr(), name) {
            (*goal).areanum = (*ml).areanum;
            (*goal).origin[0usize] = (*ml).origin[0usize];
            (*goal).origin[1usize] = (*ml).origin[1usize];
            (*goal).origin[2usize] = (*ml).origin[2usize];
            (*goal).entitynum = 0i32;
            (*goal).mins[0usize] = mins[0usize];
            (*goal).mins[1usize] = mins[1usize];
            (*goal).mins[2usize] = mins[2usize];
            (*goal).maxs[0usize] = maxs[0usize];
            (*goal).maxs[1usize] = maxs[1usize];
            (*goal).maxs[2usize] = maxs[2usize];
            (*goal).number = 0i32;
            (*goal).flags = 0i32;
            (*goal).iteminfo = 0i32;
            return qtrue as libc::c_int
        }
        ml = (*ml).next
    }
    return qfalse as libc::c_int;
}
//map locations
#[no_mangle]
pub static mut maplocations: *mut maplocation_t =
    0 as *const maplocation_t as *mut maplocation_t;
//set the avoid goal time
#[no_mangle]
pub unsafe extern "C" fn BotSetAvoidGoalTime(mut goalstate: libc::c_int,
                                             mut number: libc::c_int,
                                             mut avoidtime: libc::c_float) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    if avoidtime < 0i32 as libc::c_float {
        if itemconfig.is_null() { return }
        li = levelitems;
        while !li.is_null() {
            if (*li).number == number {
                avoidtime =
                    (*(*itemconfig).iteminfo.offset((*li).iteminfo as
                                                        isize)).respawntime;
                if 0. == avoidtime { avoidtime = 30i32 as libc::c_float }
                if avoidtime < 10i32 as libc::c_float {
                    avoidtime = 10i32 as libc::c_float
                }
                BotAddToAvoidGoals(gs, number, avoidtime);
                return
            }
            li = (*li).next
        }
        return
    } else { BotAddToAvoidGoals(gs, number, avoidtime); };
}
//initializes the items in the level
#[no_mangle]
pub unsafe extern "C" fn BotInitLevelItems() {
    let mut i: libc::c_int = 0;
    let mut spawnflags: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut origin: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut ent: libc::c_int = 0;
    let mut goalareanum: libc::c_int = 0;
    let mut ic: *mut itemconfig_t = 0 as *mut itemconfig_t;
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    let mut trace: bsp_trace_t =
        bsp_trace_s{allsolid: qfalse,
                    startsolid: qfalse,
                    fraction: 0.,
                    endpos: [0.; 3],
                    plane:
                        cplane_s{normal: [0.; 3],
                                 dist: 0.,
                                 type_0: 0,
                                 signbits: 0,
                                 pad: [0; 2],},
                    exp_dist: 0.,
                    sidenum: 0,
                    surface:
                        bsp_surface_s{name: [0; 16], flags: 0, value: 0,},
                    contents: 0,
                    ent: 0,};
    BotInitInfoEntities();
    InitLevelItemHeap();
    levelitems = 0 as *mut levelitem_t;
    numlevelitems = 0i32;
    ic = itemconfig;
    if ic.is_null() { return }
    if 0 == AAS_Loaded() { return }
    i = 0i32;
    while i < (*ic).numiteminfo {
        if 0 == (*(*ic).iteminfo.offset(i as isize)).modelindex {
            Log_Write(b"item %s has modelindex 0\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      (*(*ic).iteminfo.offset(i as
                                                  isize)).classname.as_mut_ptr());
        }
        i += 1
    }
    let mut current_block_67: u64;
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            spawnflags = 0i32;
            AAS_IntForBSPEpairKey(ent,
                                  b"spawnflags\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, &mut spawnflags);
            i = 0i32;
            while i < (*ic).numiteminfo {
                if 0 ==
                       strcmp(classname.as_mut_ptr(),
                              (*(*ic).iteminfo.offset(i as
                                                          isize)).classname.as_mut_ptr())
                   {
                    break ;
                }
                i += 1
            }
            //end for
            if i >= (*ic).numiteminfo {
                Log_Write(b"entity %s unknown item\r\n\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          classname.as_mut_ptr());
            } else if 0 ==
                          AAS_VectorForBSPEpairKey(ent,
                                                   b"origin\x00" as *const u8
                                                       as *const libc::c_char
                                                       as *mut libc::c_char,
                                                   origin.as_mut_ptr()) {
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"item %s without origin\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    classname.as_mut_ptr());
            } else {
                goalareanum = 0i32;
                //if it is a floating item
                if 0 != spawnflags & 1i32 {
                    //if the item is not floating in water
                    if 0 == AAS_PointContents(origin.as_mut_ptr()) & 32i32 {
                        end[0usize] = origin[0usize];
                        end[1usize] = origin[1usize];
                        end[2usize] = origin[2usize];
                        end[2usize] -= 32i32 as libc::c_float;
                        trace =
                            AAS_Trace(origin.as_mut_ptr(),
                                      (*(*ic).iteminfo.offset(i as
                                                                  isize)).mins.as_mut_ptr(),
                                      (*(*ic).iteminfo.offset(i as
                                                                  isize)).maxs.as_mut_ptr(),
                                      end.as_mut_ptr(), -1i32,
                                      1i32 | 0x10000i32);
                        //if the item not near the ground
                        if trace.fraction >= 1i32 as libc::c_float {
                            goalareanum =
                                AAS_BestReachableFromJumpPadArea(origin.as_mut_ptr(),
                                                                 (*(*ic).iteminfo.offset(i
                                                                                             as
                                                                                             isize)).mins.as_mut_ptr(),
                                                                 (*(*ic).iteminfo.offset(i
                                                                                             as
                                                                                             isize)).maxs.as_mut_ptr());
                            Log_Write(b"item %s reachable from jumppad area %d\r\n\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                      (*(*ic).iteminfo.offset(i as
                                                                  isize)).classname.as_mut_ptr(),
                                      goalareanum);
                            //botimport.Print(PRT_MESSAGE, "item %s reachable from jumppad area %d\r\n", ic->iteminfo[i].classname, goalareanum);
                            if 0 == goalareanum {
                                current_block_67 = 5689001924483802034;
                            } else { current_block_67 = 5330834795799507926; }
                        } else { current_block_67 = 5330834795799507926; }
                    } else { current_block_67 = 5330834795799507926; }
                } else { current_block_67 = 5330834795799507926; }
                match current_block_67 {
                    5689001924483802034 => { }
                    _ => {
                        li = AllocLevelItem();
                        if li.is_null() { return }
                        numlevelitems += 1;
                        (*li).number = numlevelitems;
                        (*li).timeout = 0i32 as libc::c_float;
                        (*li).entitynum = 0i32;
                        (*li).flags = 0i32;
                        AAS_IntForBSPEpairKey(ent,
                                              b"notfree\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              &mut value);
                        if 0 != value { (*li).flags |= 1i32 }
                        AAS_IntForBSPEpairKey(ent,
                                              b"notteam\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              &mut value);
                        if 0 != value { (*li).flags |= 2i32 }
                        AAS_IntForBSPEpairKey(ent,
                                              b"notsingle\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              &mut value);
                        if 0 != value { (*li).flags |= 4i32 }
                        AAS_IntForBSPEpairKey(ent,
                                              b"notbot\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              &mut value);
                        if 0 != value { (*li).flags |= 8i32 }
                        if 0 ==
                               strcmp(classname.as_mut_ptr(),
                                      b"item_botroam\x00" as *const u8 as
                                          *const libc::c_char) {
                            (*li).flags |= 16i32;
                            AAS_FloatForBSPEpairKey(ent,
                                                    b"weight\x00" as *const u8
                                                        as *const libc::c_char
                                                        as *mut libc::c_char,
                                                    &mut (*li).weight);
                        }
                        if 0 == spawnflags & 1i32 {
                            if 0 ==
                                   AAS_DropToFloor(origin.as_mut_ptr(),
                                                   (*(*ic).iteminfo.offset(i
                                                                               as
                                                                               isize)).mins.as_mut_ptr(),
                                                   (*(*ic).iteminfo.offset(i
                                                                               as
                                                                               isize)).maxs.as_mut_ptr())
                               {
                                botimport.Print.expect("non-null function pointer")(1i32,
                                                                                    b"%s in solid at (%1.1f %1.1f %1.1f)\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char
                                                                                        as
                                                                                        *mut libc::c_char,
                                                                                    classname.as_mut_ptr(),
                                                                                    origin[0usize]
                                                                                        as
                                                                                        libc::c_double,
                                                                                    origin[1usize]
                                                                                        as
                                                                                        libc::c_double,
                                                                                    origin[2usize]
                                                                                        as
                                                                                        libc::c_double);
                            }
                        }
                        (*li).iteminfo = i;
                        (*li).origin[0usize] = origin[0usize];
                        (*li).origin[1usize] = origin[1usize];
                        (*li).origin[2usize] = origin[2usize];
                        if 0 != goalareanum {
                            (*li).goalareanum = goalareanum;
                            (*li).goalorigin[0usize] = origin[0usize];
                            (*li).goalorigin[1usize] = origin[1usize];
                            (*li).goalorigin[2usize] = origin[2usize]
                        } else {
                            (*li).goalareanum =
                                AAS_BestReachableArea(origin.as_mut_ptr(),
                                                      (*(*ic).iteminfo.offset(i
                                                                                  as
                                                                                  isize)).mins.as_mut_ptr(),
                                                      (*(*ic).iteminfo.offset(i
                                                                                  as
                                                                                  isize)).maxs.as_mut_ptr(),
                                                      (*li).goalorigin.as_mut_ptr());
                            if 0 == (*li).goalareanum {
                                botimport.Print.expect("non-null function pointer")(1i32,
                                                                                    b"%s not reachable for bots at (%1.1f %1.1f %1.1f)\n\x00"
                                                                                        as
                                                                                        *const u8
                                                                                        as
                                                                                        *const libc::c_char
                                                                                        as
                                                                                        *mut libc::c_char,
                                                                                    classname.as_mut_ptr(),
                                                                                    origin[0usize]
                                                                                        as
                                                                                        libc::c_double,
                                                                                    origin[1usize]
                                                                                        as
                                                                                        libc::c_double,
                                                                                    origin[2usize]
                                                                                        as
                                                                                        libc::c_double);
                            }
                        }
                        AddLevelItemToList(li);
                    }
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"found %d level items\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        numlevelitems);
}
#[no_mangle]
pub static mut numlevelitems: libc::c_int = 0i32;
//end of the function FreeLevelItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AddLevelItemToList(mut li: *mut levelitem_t) {
    if !levelitems.is_null() { (*levelitems).prev = li }
    (*li).prev = 0 as *mut levelitem_s;
    (*li).next = levelitems;
    levelitems = li;
}
//end of the function InitLevelItemHeap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AllocLevelItem() -> *mut levelitem_t {
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    li = freelevelitems;
    if li.is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"out of level items\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 0 as *mut levelitem_t
    }
    freelevelitems = (*freelevelitems).next;
    memset(li as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<levelitem_t>() as libc::c_ulong);
    return li;
}
#[no_mangle]
pub static mut freelevelitems: *mut levelitem_t =
    0 as *const levelitem_t as *mut levelitem_t;
//end of the function ItemWeightIndex
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn InitLevelItemHeap() {
    let mut i: libc::c_int = 0;
    let mut max_levelitems: libc::c_int = 0;
    if !levelitemheap.is_null() {
        FreeMemory(levelitemheap as *mut libc::c_void);
    }
    max_levelitems =
        LibVarValue(b"max_levelitems\x00" as *const u8 as *const libc::c_char,
                    b"256\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    levelitemheap =
        GetClearedMemory((max_levelitems as
                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<levelitem_t>()
                                                              as
                                                              libc::c_ulong))
            as *mut levelitem_t;
    i = 0i32;
    while i < max_levelitems - 1i32 {
        let ref mut fresh0 = (*levelitemheap.offset(i as isize)).next;
        *fresh0 =
            &mut *levelitemheap.offset((i + 1i32) as isize) as
                *mut levelitem_t;
        i += 1
    }
    let ref mut fresh1 =
        (*levelitemheap.offset((max_levelitems - 1i32) as isize)).next;
    *fresh1 = 0 as *mut levelitem_s;
    freelevelitems = levelitemheap;
}
//level items
#[no_mangle]
pub static mut levelitemheap: *mut levelitem_t =
    0 as *const levelitem_t as *mut levelitem_t;
//end of the function BotFreeInfoEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotInitInfoEntities() {
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut ml: *mut maplocation_t = 0 as *mut maplocation_t;
    let mut cs: *mut campspot_t = 0 as *mut campspot_t;
    let mut ent: libc::c_int = 0;
    let mut numlocations: libc::c_int = 0;
    let mut numcampspots: libc::c_int = 0;
    BotFreeInfoEntities();
    numlocations = 0i32;
    numcampspots = 0i32;
    ent = AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0 ==
                 AAS_ValueForBSPEpairKey(ent,
                                         b"classname\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         classname.as_mut_ptr(), 128i32)) {
            //map locations
            if 0 ==
                   strcmp(classname.as_mut_ptr(),
                          b"target_location\x00" as *const u8 as
                              *const libc::c_char) {
                ml =
                    GetClearedMemory(::std::mem::size_of::<maplocation_t>() as
                                         libc::c_ulong) as *mut maplocation_t;
                AAS_VectorForBSPEpairKey(ent,
                                         b"origin\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         (*ml).origin.as_mut_ptr());
                AAS_ValueForBSPEpairKey(ent,
                                        b"message\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        (*ml).name.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 128]>()
                                            as libc::c_ulong as libc::c_int);
                (*ml).areanum = AAS_PointAreaNum((*ml).origin.as_mut_ptr());
                (*ml).next = maplocations;
                maplocations = ml;
                numlocations += 1
            } else if 0 ==
                          strcmp(classname.as_mut_ptr(),
                                 b"info_camp\x00" as *const u8 as
                                     *const libc::c_char) {
                cs =
                    GetClearedMemory(::std::mem::size_of::<campspot_t>() as
                                         libc::c_ulong) as *mut campspot_t;
                AAS_VectorForBSPEpairKey(ent,
                                         b"origin\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char,
                                         (*cs).origin.as_mut_ptr());
                AAS_ValueForBSPEpairKey(ent,
                                        b"message\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        (*cs).name.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 128]>()
                                            as libc::c_ulong as libc::c_int);
                AAS_FloatForBSPEpairKey(ent,
                                        b"range\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        &mut (*cs).range);
                AAS_FloatForBSPEpairKey(ent,
                                        b"weight\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        &mut (*cs).weight);
                AAS_FloatForBSPEpairKey(ent,
                                        b"wait\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        &mut (*cs).wait);
                AAS_FloatForBSPEpairKey(ent,
                                        b"random\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char,
                                        &mut (*cs).random);
                (*cs).areanum = AAS_PointAreaNum((*cs).origin.as_mut_ptr());
                if 0 == (*cs).areanum {
                    botimport.Print.expect("non-null function pointer")(1i32,
                                                                        b"camp spot at %1.1f %1.1f %1.1f in solid\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        (*cs).origin[0usize]
                                                                            as
                                                                            libc::c_double,
                                                                        (*cs).origin[1usize]
                                                                            as
                                                                            libc::c_double,
                                                                        (*cs).origin[2usize]
                                                                            as
                                                                            libc::c_double);
                    FreeMemory(cs as *mut libc::c_void);
                } else {
                    (*cs).next = campspots;
                    campspots = cs;
                    numcampspots += 1
                }
            }
        }
        ent = AAS_NextBSPEntity(ent)
    }
    if 0 != botDeveloper {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"%d map locations\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            numlocations);
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"%d camp spots\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            numcampspots);
    };
}
//end of the function RemoveLevelItemFromList
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeInfoEntities() {
    let mut ml: *mut maplocation_t = 0 as *mut maplocation_t;
    let mut nextml: *mut maplocation_t = 0 as *mut maplocation_t;
    let mut cs: *mut campspot_t = 0 as *mut campspot_t;
    let mut nextcs: *mut campspot_t = 0 as *mut campspot_t;
    ml = maplocations;
    while !ml.is_null() {
        nextml = (*ml).next;
        FreeMemory(ml as *mut libc::c_void);
        ml = nextml
    }
    maplocations = 0 as *mut maplocation_t;
    cs = campspots;
    while !cs.is_null() {
        nextcs = (*cs).next;
        FreeMemory(cs as *mut libc::c_void);
        cs = nextcs
    }
    campspots = 0 as *mut campspot_t;
}
//regularly update dynamic entity items (dropped weapons, flags etc.)
#[no_mangle]
pub unsafe extern "C" fn BotUpdateEntityItems() {
    let mut ent: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut modelindex: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut li: *mut levelitem_t = 0 as *mut levelitem_t;
    let mut nextli: *mut levelitem_t = 0 as *mut levelitem_t;
    let mut entinfo: aas_entityinfo_t =
        aas_entityinfo_s{valid: 0,
                         type_0: 0,
                         flags: 0,
                         ltime: 0.,
                         update_time: 0.,
                         number: 0,
                         origin: [0.; 3],
                         angles: [0.; 3],
                         old_origin: [0.; 3],
                         lastvisorigin: [0.; 3],
                         mins: [0.; 3],
                         maxs: [0.; 3],
                         groundent: 0,
                         solid: 0,
                         modelindex: 0,
                         modelindex2: 0,
                         frame: 0,
                         event: 0,
                         eventParm: 0,
                         powerups: 0,
                         weapon: 0,
                         legsAnim: 0,
                         torsoAnim: 0,};
    let mut ic: *mut itemconfig_t = 0 as *mut itemconfig_t;
    li = levelitems;
    while !li.is_null() {
        nextli = (*li).next;
        if 0. != (*li).timeout {
            if (*li).timeout < AAS_Time() {
                RemoveLevelItemFromList(li);
                FreeLevelItem(li);
            }
        }
        li = nextli
    }
    ic = itemconfig;
    if itemconfig.is_null() { return }
    ent = AAS_NextEntity(0i32);
    while 0 != ent {
        if !(AAS_EntityType(ent) != 2i32) {
            modelindex = AAS_EntityModelindex(ent);
            //
            if !(0 == modelindex) {
                AAS_EntityInfo(ent, &mut entinfo);
                //FIXME: don't do this
		//skip all floating items for now
		//if (entinfo.groundent != ENTITYNUM_WORLD) continue;
		//if the entity is still moving
                if !(entinfo.origin[0usize] != entinfo.lastvisorigin[0usize]
                         ||
                         entinfo.origin[1usize] !=
                             entinfo.lastvisorigin[1usize] ||
                         entinfo.origin[2usize] !=
                             entinfo.lastvisorigin[2usize]) {
                    li = levelitems;
                    while !li.is_null() {
                        //if the level item is linked to an entity
                        if 0 != (*li).entitynum && (*li).entitynum == ent {
                            //the entity is re-used if the models are different
                            if (*(*ic).iteminfo.offset((*li).iteminfo as
                                                           isize)).modelindex
                                   != modelindex {
                                RemoveLevelItemFromList(li);
                                FreeLevelItem(li);
                                li = 0 as *mut levelitem_t;
                                break ;
                            } else {
                                //end if
                                if entinfo.origin[0usize] !=
                                       (*li).origin[0usize] ||
                                       entinfo.origin[1usize] !=
                                           (*li).origin[1usize] ||
                                       entinfo.origin[2usize] !=
                                           (*li).origin[2usize] {
                                    (*li).origin[0usize] =
                                        entinfo.origin[0usize];
                                    (*li).origin[1usize] =
                                        entinfo.origin[1usize];
                                    (*li).origin[2usize] =
                                        entinfo.origin[2usize];
                                    (*li).goalareanum =
                                        AAS_BestReachableArea((*li).origin.as_mut_ptr(),
                                                              (*(*ic).iteminfo.offset((*li).iteminfo
                                                                                          as
                                                                                          isize)).mins.as_mut_ptr(),
                                                              (*(*ic).iteminfo.offset((*li).iteminfo
                                                                                          as
                                                                                          isize)).maxs.as_mut_ptr(),
                                                              (*li).goalorigin.as_mut_ptr())
                                }
                                //end if
                                break ;
                            }
                        } else { li = (*li).next }
                    }
                    //end else
                    //end if
                    //end for
                    if li.is_null() {
                        let mut current_block_31: u64;
                        li = levelitems;
                        while !li.is_null() {
                            //if this level item is already linked
                            if !(0 != (*li).entitynum) {
                                //
                                if g_gametype ==
                                       GT_SINGLE_PLAYER as libc::c_int {
                                    if 0 != (*li).flags & 4i32 {
                                        current_block_31 =
                                            15597372965620363352;
                                    } else {
                                        current_block_31 =
                                            14775119014532381840;
                                    }
                                } else if g_gametype >= GT_TEAM as libc::c_int
                                 {
                                    if 0 != (*li).flags & 2i32 {
                                        current_block_31 =
                                            15597372965620363352;
                                    } else {
                                        current_block_31 =
                                            14775119014532381840;
                                    }
                                } else if 0 != (*li).flags & 1i32 {
                                    current_block_31 = 15597372965620363352;
                                } else {
                                    current_block_31 = 14775119014532381840;
                                }
                                match current_block_31 {
                                    15597372965620363352 => { }
                                    _ => {
                                        //if the model of the level item and the entity are the same
                                        if (*(*ic).iteminfo.offset((*li).iteminfo
                                                                       as
                                                                       isize)).modelindex
                                               == modelindex {
                                            dir[0usize] =
                                                (*li).origin[0usize] -
                                                    entinfo.origin[0usize];
                                            dir[1usize] =
                                                (*li).origin[1usize] -
                                                    entinfo.origin[1usize];
                                            dir[2usize] =
                                                (*li).origin[2usize] -
                                                    entinfo.origin[2usize];
                                            if VectorLength(dir.as_mut_ptr()
                                                                as
                                                                *const vec_t)
                                                   < 30i32 as libc::c_float {
                                                (*li).entitynum = ent;
                                                if entinfo.origin[0usize] !=
                                                       (*li).origin[0usize] ||
                                                       entinfo.origin[1usize]
                                                           !=
                                                           (*li).origin[1usize]
                                                       ||
                                                       entinfo.origin[2usize]
                                                           !=
                                                           (*li).origin[2usize]
                                                   {
                                                    (*li).origin[0usize] =
                                                        entinfo.origin[0usize];
                                                    (*li).origin[1usize] =
                                                        entinfo.origin[1usize];
                                                    (*li).origin[2usize] =
                                                        entinfo.origin[2usize];
                                                    (*li).goalareanum =
                                                        AAS_BestReachableArea((*li).origin.as_mut_ptr(),
                                                                              (*(*ic).iteminfo.offset((*li).iteminfo
                                                                                                          as
                                                                                                          isize)).mins.as_mut_ptr(),
                                                                              (*(*ic).iteminfo.offset((*li).iteminfo
                                                                                                          as
                                                                                                          isize)).maxs.as_mut_ptr(),
                                                                              (*li).goalorigin.as_mut_ptr())
                                                }
                                                //end if
                                                //DEBUG
                                                break ;
                                            }
                                        }
                                    }
                                }
                            }
                            li = (*li).next
                        }
                        //end if
                        //end else
                        //end for
                        if li.is_null() {
                            i = 0i32;
                            while i < (*ic).numiteminfo {
                                if (*(*ic).iteminfo.offset(i as
                                                               isize)).modelindex
                                       == modelindex {
                                    break ;
                                }
                                i += 1
                            }
                            //end if
                            //end for
                            //if the model is not from a known item
                            if !(i >= (*ic).numiteminfo) {
                                li = AllocLevelItem();
                                //
                                if !li.is_null() {
                                    (*li).entitynum = ent;
                                    (*li).number = numlevelitems + ent;
                                    (*li).iteminfo = i;
                                    (*li).origin[0usize] =
                                        entinfo.origin[0usize];
                                    (*li).origin[1usize] =
                                        entinfo.origin[1usize];
                                    (*li).origin[2usize] =
                                        entinfo.origin[2usize];
                                    (*li).goalareanum =
                                        AAS_BestReachableArea((*li).origin.as_mut_ptr(),
                                                              (*(*ic).iteminfo.offset(i
                                                                                          as
                                                                                          isize)).mins.as_mut_ptr(),
                                                              (*(*ic).iteminfo.offset(i
                                                                                          as
                                                                                          isize)).maxs.as_mut_ptr(),
                                                              (*li).goalorigin.as_mut_ptr());
                                    //never go for items dropped into jumppads
                                    if 0 != AAS_AreaJumpPad((*li).goalareanum)
                                       {
                                        FreeLevelItem(li);
                                    } else {
                                        (*li).timeout =
                                            AAS_Time() +
                                                30i32 as libc::c_float;
                                        AddLevelItemToList(li);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        ent = AAS_NextEntity(ent)
    };
}
//end of the function AllocLevelItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn FreeLevelItem(mut li: *mut levelitem_t) {
    (*li).next = freelevelitems;
    freelevelitems = li;
}
//end of the function AddLevelItemToList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn RemoveLevelItemFromList(mut li: *mut levelitem_t) {
    if !(*li).prev.is_null() {
        (*(*li).prev).next = (*li).next
    } else { levelitems = (*li).next }
    if !(*li).next.is_null() { (*(*li).next).prev = (*li).prev };
}
//interbreed the goal fuzzy logic
#[no_mangle]
pub unsafe extern "C" fn BotInterbreedGoalFuzzyLogic(mut parent1: libc::c_int,
                                                     mut parent2: libc::c_int,
                                                     mut child: libc::c_int) {
    let mut p1: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    let mut p2: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    let mut c: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    p1 = BotGoalStateFromHandle(parent1);
    p2 = BotGoalStateFromHandle(parent2);
    c = BotGoalStateFromHandle(child);
    if p1.is_null() || p2.is_null() || c.is_null() { return }
    InterbreedWeightConfigs((*p1).itemweightconfig, (*p2).itemweightconfig,
                            (*c).itemweightconfig);
}
//save the goal fuzzy logic to disk
#[no_mangle]
pub unsafe extern "C" fn BotSaveGoalFuzzyLogic(mut goalstate: libc::c_int,
                                               mut filename:
                                                   *mut libc::c_char) {
}
//mutate the goal fuzzy logic
#[no_mangle]
pub unsafe extern "C" fn BotMutateGoalFuzzyLogic(mut goalstate: libc::c_int,
                                                 mut range: libc::c_float) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    EvolveWeightConfig((*gs).itemweightconfig);
}
//loads item weights for the bot
#[no_mangle]
pub unsafe extern "C" fn BotLoadItemWeights(mut goalstate: libc::c_int,
                                            mut filename: *mut libc::c_char)
 -> libc::c_int {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return 9i32 }
    (*gs).itemweightconfig = ReadWeightConfig(filename);
    if (*gs).itemweightconfig.is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"couldn\'t load weights\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 9i32
    }
    if itemconfig.is_null() { return 9i32 }
    (*gs).itemweightindex =
        ItemWeightIndex((*gs).itemweightconfig, itemconfig);
    return 0i32;
}
//end of the function LoadItemConfig
//===========================================================================
// index to find the weight function of an iteminfo
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ItemWeightIndex(mut iwc: *mut weightconfig_t,
                                         mut ic: *mut itemconfig_t)
 -> *mut libc::c_int {
    let mut index: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    index =
        GetClearedMemory((::std::mem::size_of::<libc::c_int>() as
                              libc::c_ulong).wrapping_mul((*ic).numiteminfo as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    i = 0i32;
    while i < (*ic).numiteminfo {
        *index.offset(i as isize) =
            FindFuzzyWeight(iwc,
                            (*(*ic).iteminfo.offset(i as
                                                        isize)).classname.as_mut_ptr());
        if *index.offset(i as isize) < 0i32 {
            Log_Write(b"item info %d \"%s\" has no fuzzy weight\r\n\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char, i,
                      (*(*ic).iteminfo.offset(i as
                                                  isize)).classname.as_mut_ptr());
        }
        i += 1
    }
    return index;
}
//frees the item weights of the bot
#[no_mangle]
pub unsafe extern "C" fn BotFreeItemWeights(mut goalstate: libc::c_int) {
    let mut gs: *mut bot_goalstate_t = 0 as *mut bot_goalstate_t;
    gs = BotGoalStateFromHandle(goalstate);
    if gs.is_null() { return }
    if !(*gs).itemweightconfig.is_null() {
        FreeWeightConfig((*gs).itemweightconfig);
    }
    if !(*gs).itemweightindex.is_null() {
        FreeMemory((*gs).itemweightindex as *mut libc::c_void);
    };
}
//returns the handle of a newly allocated goal state
#[no_mangle]
pub unsafe extern "C" fn BotAllocGoalState(mut client: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i <= 64i32 {
        if botgoalstates[i as usize].is_null() {
            botgoalstates[i as usize] =
                GetClearedMemory(::std::mem::size_of::<bot_goalstate_t>() as
                                     libc::c_ulong) as *mut bot_goalstate_t;
            (*botgoalstates[i as usize]).client = client;
            return i
        }
        i += 1
    }
    return 0i32;
}
//free the given goal state
#[no_mangle]
pub unsafe extern "C" fn BotFreeGoalState(mut handle: libc::c_int) {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"goal state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    if botgoalstates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid goal state handle %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    BotFreeItemWeights(handle);
    FreeMemory(botgoalstates[handle as usize] as *mut libc::c_void);
    botgoalstates[handle as usize] = 0 as *mut bot_goalstate_t;
}
//setup the goal AI
#[no_mangle]
pub unsafe extern "C" fn BotSetupGoalAI() -> libc::c_int {
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    g_gametype =
        LibVarValue(b"g_gametype\x00" as *const u8 as *const libc::c_char,
                    b"0\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    filename =
        LibVarString(b"itemconfig\x00" as *const u8 as *const libc::c_char,
                     b"items.c\x00" as *const u8 as *const libc::c_char);
    itemconfig = LoadItemConfig(filename);
    if itemconfig.is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"couldn\'t load item config\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 10i32
    }
    droppedweight =
        LibVar(b"droppedweight\x00" as *const u8 as *const libc::c_char,
               b"1000\x00" as *const u8 as *const libc::c_char);
    return 0i32;
}
//end of the function BotMutateGoalFuzzyLogic
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn LoadItemConfig(mut filename: *mut libc::c_char)
 -> *mut itemconfig_t {
    let mut max_iteminfo: libc::c_int = 0;
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
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut ic: *mut itemconfig_t = 0 as *mut itemconfig_t;
    let mut ii: *mut iteminfo_t = 0 as *mut iteminfo_t;
    max_iteminfo =
        LibVarValue(b"max_iteminfo\x00" as *const u8 as *const libc::c_char,
                    b"256\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    if max_iteminfo < 0i32 {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"max_iteminfo = %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            max_iteminfo);
        max_iteminfo = 256i32;
        LibVarSet(b"max_iteminfo\x00" as *const u8 as *const libc::c_char,
                  b"256\x00" as *const u8 as *const libc::c_char);
    }
    Q_strncpyz(path.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char);
    source = LoadSourceFile(path.as_mut_ptr());
    if source.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"counldn\'t load %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            path.as_mut_ptr());
        return 0 as *mut itemconfig_t
    }
    ic =
        GetClearedHunkMemory((::std::mem::size_of::<itemconfig_t>() as
                                  libc::c_ulong).wrapping_add((max_iteminfo as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<iteminfo_t>()
                                                                                                   as
                                                                                                   libc::c_ulong)))
            as *mut itemconfig_t;
    (*ic).iteminfo =
        (ic as
             *mut libc::c_char).offset(::std::mem::size_of::<itemconfig_t>()
                                           as libc::c_ulong as isize) as
            *mut iteminfo_t;
    (*ic).numiteminfo = 0i32;
    while 0 != PC_ReadToken(source, &mut token) {
        if 0 ==
               strcmp(token.string.as_mut_ptr(),
                      b"iteminfo\x00" as *const u8 as *const libc::c_char) {
            if (*ic).numiteminfo >= max_iteminfo {
                SourceError(source,
                            b"more than %d item info defined\x00" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            max_iteminfo);
                FreeMemory(ic as *mut libc::c_void);
                FreeSource(source);
                return 0 as *mut itemconfig_t
            }
            ii =
                &mut *(*ic).iteminfo.offset((*ic).numiteminfo as isize) as
                    *mut iteminfo_t;
            memset(ii as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<iteminfo_t>() as libc::c_ulong);
            if 0 == PC_ExpectTokenType(source, 1i32, 0i32, &mut token) {
                FreeMemory(ic as *mut libc::c_void);
                FreeSource(source);
                return 0 as *mut itemconfig_t
            }
            StripDoubleQuotes(token.string.as_mut_ptr());
            Q_strncpyz((*ii).classname.as_mut_ptr(),
                       token.string.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 32]>() as
                           libc::c_ulong as libc::c_int);
            if 0 ==
                   ReadStructure(source, &mut iteminfo_struct,
                                 ii as *mut libc::c_char) {
                FreeMemory(ic as *mut libc::c_void);
                FreeSource(source);
                return 0 as *mut itemconfig_t
            }
            (*ii).number = (*ic).numiteminfo;
            (*ic).numiteminfo += 1
        } else {
            SourceError(source,
                        b"unknown definition %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            FreeMemory(ic as *mut libc::c_void);
            FreeSource(source);
            return 0 as *mut itemconfig_t
        }
    }
    FreeSource(source);
    if 0 == (*ic).numiteminfo {
        botimport.Print.expect("non-null function pointer")(2i32,
                                                            b"no item info loaded\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        path.as_mut_ptr());
    return ic;
}
#[no_mangle]
pub static mut iteminfo_struct: structdef_t =
    unsafe {
        structdef_s{size:
                        ::std::mem::size_of::<iteminfo_t>() as libc::c_ulong
                            as libc::c_int,
                    fields: iteminfo_fields.as_ptr() as *mut _,}
    };
// Initialized in run_static_initializers
#[no_mangle]
pub static mut iteminfo_fields: [fielddef_t; 9] =
    [fielddef_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                offset: 0,
                type_0: 0,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *const structdef_s as *mut structdef_s,}; 9];
//shut down the goal AI
#[no_mangle]
pub unsafe extern "C" fn BotShutdownGoalAI() {
    let mut i: libc::c_int = 0;
    if !itemconfig.is_null() { FreeMemory(itemconfig as *mut libc::c_void); }
    itemconfig = 0 as *mut itemconfig_t;
    if !levelitemheap.is_null() {
        FreeMemory(levelitemheap as *mut libc::c_void);
    }
    levelitemheap = 0 as *mut levelitem_t;
    freelevelitems = 0 as *mut levelitem_t;
    levelitems = 0 as *mut levelitem_t;
    numlevelitems = 0i32;
    BotFreeInfoEntities();
    i = 1i32;
    while i <= 64i32 {
        if !botgoalstates[i as usize].is_null() { BotFreeGoalState(i); }
        i += 1
    };
}
//end of the function BotGetNextCampSpotGoal
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFindEntityForLevelItem(mut li: *mut levelitem_t) {
    let mut ent: libc::c_int = 0;
    let mut modelindex: libc::c_int = 0;
    let mut ic: *mut itemconfig_t = 0 as *mut itemconfig_t;
    let mut entinfo: aas_entityinfo_t =
        aas_entityinfo_s{valid: 0,
                         type_0: 0,
                         flags: 0,
                         ltime: 0.,
                         update_time: 0.,
                         number: 0,
                         origin: [0.; 3],
                         angles: [0.; 3],
                         old_origin: [0.; 3],
                         lastvisorigin: [0.; 3],
                         mins: [0.; 3],
                         maxs: [0.; 3],
                         groundent: 0,
                         solid: 0,
                         modelindex: 0,
                         modelindex2: 0,
                         frame: 0,
                         event: 0,
                         eventParm: 0,
                         powerups: 0,
                         weapon: 0,
                         legsAnim: 0,
                         torsoAnim: 0,};
    let mut dir: vec3_t = [0.; 3];
    ic = itemconfig;
    if itemconfig.is_null() { return }
    ent = AAS_NextEntity(0i32);
    while 0 != ent {
        modelindex = AAS_EntityModelindex(ent);
        //
        if !(0 == modelindex) {
            AAS_EntityInfo(ent, &mut entinfo);
            //if the entity is still moving
            if !(entinfo.origin[0usize] != entinfo.lastvisorigin[0usize] ||
                     entinfo.origin[1usize] != entinfo.lastvisorigin[1usize]
                     ||
                     entinfo.origin[2usize] != entinfo.lastvisorigin[2usize])
               {
                if (*(*ic).iteminfo.offset((*li).iteminfo as
                                               isize)).modelindex ==
                       modelindex {
                    dir[0usize] =
                        (*li).origin[0usize] - entinfo.origin[0usize];
                    dir[1usize] =
                        (*li).origin[1usize] - entinfo.origin[1usize];
                    dir[2usize] =
                        (*li).origin[2usize] - entinfo.origin[2usize];
                    if VectorLength(dir.as_mut_ptr() as *const vec_t) <
                           30i32 as libc::c_float {
                        (*li).entitynum = ent
                    }
                }
            }
        }
        ent = AAS_NextEntity(ent)
    };
}
unsafe extern "C" fn run_static_initializers() {
    iteminfo_fields =
        [fielddef_s{name:
                        b"name\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).name as
                            *mut [libc::c_char; 80] as size_t as libc::c_int,
                    type_0: 4i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name:
                        b"model\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).model as
                            *mut [libc::c_char; 80] as size_t as libc::c_int,
                    type_0: 4i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name:
                        b"modelindex\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).modelindex as
                            *mut libc::c_int as size_t as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name:
                        b"type\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).type_0 as
                            *mut libc::c_int as size_t as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name:
                        b"index\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).index as
                            *mut libc::c_int as size_t as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name:
                        b"respawntime\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).respawntime as
                            *mut libc::c_float as size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name:
                        b"mins\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).mins as *mut vec3_t as
                            size_t as libc::c_int,
                    type_0: 3i32 | 0x100i32,
                    maxarray: 3i32,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name:
                        b"maxs\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &mut (*(0 as *mut iteminfo_t)).maxs as *mut vec3_t as
                            size_t as libc::c_int,
                    type_0: 3i32 | 0x100i32,
                    maxarray: 3i32,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,},
         fielddef_s{name: 0 as *mut libc::c_char,
                    offset: 0i32,
                    type_0: 0i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *mut structdef_s,}]
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];