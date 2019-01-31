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
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_weap.h"]
pub mod be_ai_weap_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct weaponinfo_s {
        pub valid: libc::c_int,
        pub number: libc::c_int,
        pub name: [libc::c_char; 80],
        pub model: [libc::c_char; 80],
        pub level: libc::c_int,
        pub weaponindex: libc::c_int,
        pub flags: libc::c_int,
        pub projectile: [libc::c_char; 80],
        pub numprojectiles: libc::c_int,
        pub hspread: libc::c_float,
        pub vspread: libc::c_float,
        pub speed: libc::c_float,
        pub acceleration: libc::c_float,
        pub recoil: vec3_t,
        pub offset: vec3_t,
        pub angleoffset: vec3_t,
        pub extrazvelocity: libc::c_float,
        pub ammoamount: libc::c_int,
        pub ammoindex: libc::c_int,
        pub activate: libc::c_float,
        pub reload: libc::c_float,
        pub spinup: libc::c_float,
        pub spindown: libc::c_float,
        pub proj: projectileinfo_t,
    }
    pub type projectileinfo_t = projectileinfo_s;
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
 * name:		be_ai_weap.h
 *
 * desc:		weapon AI
 *
 * $Archive: /source/code/botlib/be_ai_weap.h $
 *
 *****************************************************************************/
    //projectile flags
    //projectile damages through window
    //set when projectile returns to owner
    //weapon flags
    //set when projectile is fired with key-up event
    //damage types
    //damage on impact
    //radial damage
    //damage to all entities visible to the projectile
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct projectileinfo_s {
        pub name: [libc::c_char; 80],
        pub model: [libc::c_char; 80],
        pub flags: libc::c_int,
        pub gravity: libc::c_float,
        pub damage: libc::c_int,
        pub radius: libc::c_float,
        pub visdamage: libc::c_int,
        pub damagetype: libc::c_int,
        pub healthinc: libc::c_int,
        pub push: libc::c_float,
        pub detonation: libc::c_float,
        pub bounce: libc::c_float,
        pub bouncefric: libc::c_float,
        pub bouncestop: libc::c_float,
    }
    pub type weaponinfo_t = weaponinfo_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t};
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
        //returns the fuzzy weight for the given inventory and weight
        #[no_mangle]
        pub fn FuzzyWeight(inventory: *mut libc::c_int,
                           wc: *mut weightconfig_t, weightnum: libc::c_int)
         -> libc::c_float;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_weap.c"]
pub mod be_ai_weap_c {
    pub type weaponconfig_t = weaponconfig_s;
    //weapon configuration: set of weapons with projectiles
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct weaponconfig_s {
        pub numweapons: libc::c_int,
        pub numprojectiles: libc::c_int,
        pub projectileinfo: *mut projectileinfo_t,
        pub weaponinfo: *mut weaponinfo_t,
    }
    pub type bot_weaponstate_t = bot_weaponstate_s;
    //the bot weapon state
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_weaponstate_s {
        pub weaponweightconfig: *mut weightconfig_s,
        pub weaponweightindex: *mut libc::c_int,
    }
    use super::{libc};
    use super::be_ai_weap_h::{projectileinfo_t, weaponinfo_t};
    use super::be_ai_weight_h::{weightconfig_s, weightconfig_t};
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
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
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
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
use self::stddef_h::{size_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, Q_strncpyz};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t};
use self::l_precomp_h::{define_s, define_t, indent_s, indent_t, source_s,
                        source_t, PC_ReadToken, PC_SetBaseFolder,
                        LoadSourceFile, FreeSource};
use self::l_struct_h::{fielddef_s, structdef_s, fielddef_t, structdef_t,
                       ReadStructure};
use self::be_ai_weap_h::{weaponinfo_s, projectileinfo_t, projectileinfo_s,
                         weaponinfo_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_ai_weight_h::{fuzzyseperator_s, weight_s, weight_t,
                           weightconfig_s, weightconfig_t, ReadWeightConfig,
                           FreeWeightConfig, FindFuzzyWeight, FuzzyWeight};
use self::be_ai_weap_c::{weaponconfig_t, weaponconfig_s, bot_weaponstate_t,
                         bot_weaponstate_s};
use self::string_h::{memcpy, memset, strcmp};
use self::l_libvar_h::{LibVarValue, LibVarString, LibVarSet};
use self::l_memory_h::{GetClearedMemory, GetClearedHunkMemory, FreeMemory};
use self::be_interface_h::{botimport};
//setup the weapon AI
#[no_mangle]
pub unsafe extern "C" fn BotSetupWeaponAI() -> libc::c_int {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    file =
        LibVarString(b"weaponconfig\x00" as *const u8 as *const libc::c_char,
                     b"weapons.c\x00" as *const u8 as *const libc::c_char);
    weaponconfig = LoadWeaponConfig(file);
    if weaponconfig.is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"couldn\'t load the weapon config\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return 12i32
    }
    return 0i32;
}
static mut weaponconfig: *mut weaponconfig_t =
    0 as *const weaponconfig_t as *mut weaponconfig_t;
//end of the function BotWeaponStateFromHandle
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
//DEBUG_AI_WEAP
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn LoadWeaponConfig(mut filename: *mut libc::c_char)
 -> *mut weaponconfig_t {
    let mut max_weaponinfo: libc::c_int = 0;
    let mut max_projectileinfo: libc::c_int = 0;
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
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut wc: *mut weaponconfig_t = 0 as *mut weaponconfig_t;
    let mut weaponinfo: weaponinfo_t =
        weaponinfo_s{valid: 0,
                     number: 0,
                     name: [0; 80],
                     model: [0; 80],
                     level: 0,
                     weaponindex: 0,
                     flags: 0,
                     projectile: [0; 80],
                     numprojectiles: 0,
                     hspread: 0.,
                     vspread: 0.,
                     speed: 0.,
                     acceleration: 0.,
                     recoil: [0.; 3],
                     offset: [0.; 3],
                     angleoffset: [0.; 3],
                     extrazvelocity: 0.,
                     ammoamount: 0,
                     ammoindex: 0,
                     activate: 0.,
                     reload: 0.,
                     spinup: 0.,
                     spindown: 0.,
                     proj:
                         projectileinfo_s{name: [0; 80],
                                          model: [0; 80],
                                          flags: 0,
                                          gravity: 0.,
                                          damage: 0,
                                          radius: 0.,
                                          visdamage: 0,
                                          damagetype: 0,
                                          healthinc: 0,
                                          push: 0.,
                                          detonation: 0.,
                                          bounce: 0.,
                                          bouncefric: 0.,
                                          bouncestop: 0.,},};
    max_weaponinfo =
        LibVarValue(b"max_weaponinfo\x00" as *const u8 as *const libc::c_char,
                    b"32\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    if max_weaponinfo < 0i32 {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"max_weaponinfo = %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            max_weaponinfo);
        max_weaponinfo = 32i32;
        LibVarSet(b"max_weaponinfo\x00" as *const u8 as *const libc::c_char,
                  b"32\x00" as *const u8 as *const libc::c_char);
    }
    max_projectileinfo =
        LibVarValue(b"max_projectileinfo\x00" as *const u8 as
                        *const libc::c_char,
                    b"32\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    if max_projectileinfo < 0i32 {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"max_projectileinfo = %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            max_projectileinfo);
        max_projectileinfo = 32i32;
        LibVarSet(b"max_projectileinfo\x00" as *const u8 as
                      *const libc::c_char,
                  b"32\x00" as *const u8 as *const libc::c_char);
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
        return 0 as *mut weaponconfig_t
    }
    wc =
        GetClearedHunkMemory((::std::mem::size_of::<weaponconfig_t>() as
                                  libc::c_ulong).wrapping_add((max_weaponinfo
                                                                   as
                                                                   libc::c_ulong).wrapping_mul(::std::mem::size_of::<weaponinfo_t>()
                                                                                                   as
                                                                                                   libc::c_ulong)).wrapping_add((max_projectileinfo
                                                                                                                                     as
                                                                                                                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<projectileinfo_t>()
                                                                                                                                                                     as
                                                                                                                                                                     libc::c_ulong)))
            as *mut weaponconfig_t;
    (*wc).weaponinfo =
        (wc as
             *mut libc::c_char).offset(::std::mem::size_of::<weaponconfig_t>()
                                           as libc::c_ulong as isize) as
            *mut weaponinfo_t;
    (*wc).projectileinfo =
        ((*wc).weaponinfo as
             *mut libc::c_char).offset((max_weaponinfo as
                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<weaponinfo_t>()
                                                                            as
                                                                            libc::c_ulong)
                                           as isize) as *mut projectileinfo_t;
    (*wc).numweapons = max_weaponinfo;
    (*wc).numprojectiles = 0i32;
    while 0 != PC_ReadToken(source, &mut token) {
        if 0 ==
               strcmp(token.string.as_mut_ptr(),
                      b"weaponinfo\x00" as *const u8 as *const libc::c_char) {
            memset(&mut weaponinfo as *mut weaponinfo_t as *mut libc::c_void,
                   0i32,
                   ::std::mem::size_of::<weaponinfo_t>() as libc::c_ulong);
            if 0 ==
                   ReadStructure(source, &mut weaponinfo_struct,
                                 &mut weaponinfo as *mut weaponinfo_t as
                                     *mut libc::c_char) {
                FreeMemory(wc as *mut libc::c_void);
                FreeSource(source);
                return 0 as *mut weaponconfig_t
            }
            if weaponinfo.number < 0i32 || weaponinfo.number >= max_weaponinfo
               {
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"weapon info number %d out of range in %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    weaponinfo.number,
                                                                    path.as_mut_ptr());
                FreeMemory(wc as *mut libc::c_void);
                FreeSource(source);
                return 0 as *mut weaponconfig_t
            }
            memcpy(&mut *(*wc).weaponinfo.offset(weaponinfo.number as isize)
                       as *mut weaponinfo_t as *mut libc::c_void,
                   &mut weaponinfo as *mut weaponinfo_t as
                       *const libc::c_void,
                   ::std::mem::size_of::<weaponinfo_t>() as libc::c_ulong);
            (*(*wc).weaponinfo.offset(weaponinfo.number as isize)).valid =
                qtrue as libc::c_int
        } else if 0 ==
                      strcmp(token.string.as_mut_ptr(),
                             b"projectileinfo\x00" as *const u8 as
                                 *const libc::c_char) {
            if (*wc).numprojectiles >= max_projectileinfo {
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"more than %d projectiles defined in %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    max_projectileinfo,
                                                                    path.as_mut_ptr());
                FreeMemory(wc as *mut libc::c_void);
                FreeSource(source);
                return 0 as *mut weaponconfig_t
            }
            memset(&mut *(*wc).projectileinfo.offset((*wc).numprojectiles as
                                                         isize) as
                       *mut projectileinfo_t as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<projectileinfo_t>() as
                       libc::c_ulong);
            if 0 ==
                   ReadStructure(source, &mut projectileinfo_struct,
                                 &mut *(*wc).projectileinfo.offset((*wc).numprojectiles
                                                                       as
                                                                       isize)
                                     as *mut projectileinfo_t as
                                     *mut libc::c_char) {
                FreeMemory(wc as *mut libc::c_void);
                FreeSource(source);
                return 0 as *mut weaponconfig_t
            }
            (*wc).numprojectiles += 1
        } else {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"unknown definition %s in %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                token.string.as_mut_ptr(),
                                                                path.as_mut_ptr());
            FreeMemory(wc as *mut libc::c_void);
            FreeSource(source);
            return 0 as *mut weaponconfig_t
        }
    }
    FreeSource(source);
    i = 0i32;
    while i < (*wc).numweapons {
        if !(0 == (*(*wc).weaponinfo.offset(i as isize)).valid) {
            if 0 == (*(*wc).weaponinfo.offset(i as isize)).name[0usize] {
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"weapon %d has no name in %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    i,
                                                                    path.as_mut_ptr());
                FreeMemory(wc as *mut libc::c_void);
                return 0 as *mut weaponconfig_t
            }
            if 0 == (*(*wc).weaponinfo.offset(i as isize)).projectile[0usize]
               {
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"weapon %s has no projectile in %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    (*(*wc).weaponinfo.offset(i
                                                                                                  as
                                                                                                  isize)).name.as_mut_ptr(),
                                                                    path.as_mut_ptr());
                FreeMemory(wc as *mut libc::c_void);
                return 0 as *mut weaponconfig_t
            }
            j = 0i32;
            while j < (*wc).numprojectiles {
                if 0 ==
                       strcmp((*(*wc).projectileinfo.offset(j as
                                                                isize)).name.as_mut_ptr(),
                              (*(*wc).weaponinfo.offset(i as
                                                            isize)).projectile.as_mut_ptr())
                   {
                    memcpy(&mut (*(*wc).weaponinfo.offset(i as isize)).proj as
                               *mut projectileinfo_t as *mut libc::c_void,
                           &mut *(*wc).projectileinfo.offset(j as isize) as
                               *mut projectileinfo_t as *const libc::c_void,
                           ::std::mem::size_of::<projectileinfo_t>() as
                               libc::c_ulong);
                    break ;
                } else { j += 1 }
            }
            if j == (*wc).numprojectiles {
                botimport.Print.expect("non-null function pointer")(3i32,
                                                                    b"weapon %s uses undefined projectile in %s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    (*(*wc).weaponinfo.offset(i
                                                                                                  as
                                                                                                  isize)).name.as_mut_ptr(),
                                                                    path.as_mut_ptr());
                FreeMemory(wc as *mut libc::c_void);
                return 0 as *mut weaponconfig_t
            }
        }
        i += 1
    }
    if 0 == (*wc).numweapons {
        botimport.Print.expect("non-null function pointer")(2i32,
                                                            b"no weapon info loaded\n\x00"
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
    return wc;
}
static mut projectileinfo_struct: structdef_t =
    unsafe {
        structdef_s{size:
                        ::std::mem::size_of::<projectileinfo_t>() as
                            libc::c_ulong as libc::c_int,
                    fields: projectileinfo_fields.as_ptr() as *mut _,}
    };
// Initialized in run_static_initializers
static mut projectileinfo_fields: [fielddef_t; 15] =
    [fielddef_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                offset: 0,
                type_0: 0,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *const structdef_s as *mut structdef_s,}; 15];
//name of the projectile
//model of the projectile
//special flags
//amount of gravity applied to the projectile [0,1]
//damage of the projectile
//radius of damage
//damage of the projectile to visible entities
//type of damage (combination of the DAMAGETYPE_? flags)
//health increase the owner gets
//amount a player is pushed away from the projectile impact
//time before projectile explodes after fire pressed
//amount the projectile bounces
//amount the bounce decreases per bounce
//minimum bounce value before bouncing stops
//recurive projectile definition??
static mut weaponinfo_struct: structdef_t =
    unsafe {
        structdef_s{size:
                        ::std::mem::size_of::<weaponinfo_t>() as libc::c_ulong
                            as libc::c_int,
                    fields: weaponinfo_fields.as_ptr() as *mut _,}
    };
// Initialized in run_static_initializers
static mut weaponinfo_fields: [fielddef_t; 23] =
    [fielddef_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                offset: 0,
                type_0: 0,
                maxarray: 0,
                floatmin: 0.,
                floatmax: 0.,
                substruct: 0 as *const structdef_s as *mut structdef_s,}; 23];
//shut down the weapon AI
#[no_mangle]
pub unsafe extern "C" fn BotShutdownWeaponAI() {
    let mut i: libc::c_int = 0;
    if !weaponconfig.is_null() {
        FreeMemory(weaponconfig as *mut libc::c_void);
    }
    weaponconfig = 0 as *mut weaponconfig_t;
    i = 1i32;
    while i <= 64i32 {
        if !botweaponstates[i as usize].is_null() { BotFreeWeaponState(i); }
        i += 1
    };
}
//frees the weapon state
#[no_mangle]
pub unsafe extern "C" fn BotFreeWeaponState(mut handle: libc::c_int) {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"weapon state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    if botweaponstates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid weapon state %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    BotFreeWeaponWeights(handle);
    FreeMemory(botweaponstates[handle as usize] as *mut libc::c_void);
    botweaponstates[handle as usize] = 0 as *mut bot_weaponstate_t;
}
static mut botweaponstates: [*mut bot_weaponstate_t; 65] =
    [0 as *const bot_weaponstate_t as *mut bot_weaponstate_t; 65];
//end of the function WeaponWeightIndex
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeWeaponWeights(mut weaponstate: libc::c_int) {
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() { return }
    if !(*ws).weaponweightconfig.is_null() {
        FreeWeightConfig((*ws).weaponweightconfig);
    }
    if !(*ws).weaponweightindex.is_null() {
        FreeMemory((*ws).weaponweightindex as *mut libc::c_void);
    };
}
//end of the function BotValidWeaponNumber
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotWeaponStateFromHandle(mut handle: libc::c_int)
 -> *mut bot_weaponstate_t {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"weapon state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_weaponstate_t
    }
    if botweaponstates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid weapon state %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_weaponstate_t
    }
    return botweaponstates[handle as usize];
}
//returns the best weapon to fight with
#[no_mangle]
pub unsafe extern "C" fn BotChooseBestFightWeapon(mut weaponstate:
                                                      libc::c_int,
                                                  mut inventory:
                                                      *mut libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut bestweapon: libc::c_int = 0;
    let mut weight: libc::c_float = 0.;
    let mut bestweight: libc::c_float = 0.;
    let mut wc: *mut weaponconfig_t = 0 as *mut weaponconfig_t;
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() { return 0i32 }
    wc = weaponconfig;
    if weaponconfig.is_null() { return 0i32 }
    if (*ws).weaponweightconfig.is_null() { return 0i32 }
    bestweight = 0i32 as libc::c_float;
    bestweapon = 0i32;
    i = 0i32;
    while i < (*wc).numweapons {
        if !(0 == (*(*wc).weaponinfo.offset(i as isize)).valid) {
            index = *(*ws).weaponweightindex.offset(i as isize);
            if !(index < 0i32) {
                weight =
                    FuzzyWeight(inventory, (*ws).weaponweightconfig, index);
                if weight > bestweight { bestweight = weight; bestweapon = i }
            }
        }
        i += 1
    }
    return bestweapon;
}
//returns the information of the current weapon
#[no_mangle]
pub unsafe extern "C" fn BotGetWeaponInfo(mut weaponstate: libc::c_int,
                                          mut weapon: libc::c_int,
                                          mut weaponinfo: *mut weaponinfo_t) {
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    if 0 == BotValidWeaponNumber(weapon) { return }
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() { return }
    if weaponconfig.is_null() { return }
    memcpy(weaponinfo as *mut libc::c_void,
           &mut *(*weaponconfig).weaponinfo.offset(weapon as isize) as
               *mut weaponinfo_t as *const libc::c_void,
           ::std::mem::size_of::<weaponinfo_t>() as libc::c_ulong);
}
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotValidWeaponNumber(mut weaponnum: libc::c_int)
 -> libc::c_int {
    if weaponnum <= 0i32 || weaponnum > (*weaponconfig).numweapons {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"weapon number out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//loads the weapon weights
#[no_mangle]
pub unsafe extern "C" fn BotLoadWeaponWeights(mut weaponstate: libc::c_int,
                                              mut filename: *mut libc::c_char)
 -> libc::c_int {
    let mut ws: *mut bot_weaponstate_t = 0 as *mut bot_weaponstate_t;
    ws = BotWeaponStateFromHandle(weaponstate);
    if ws.is_null() { return 11i32 }
    BotFreeWeaponWeights(weaponstate);
    (*ws).weaponweightconfig = ReadWeightConfig(filename);
    if (*ws).weaponweightconfig.is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"couldn\'t load weapon config %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            filename);
        return 11i32
    }
    if weaponconfig.is_null() { return 12i32 }
    (*ws).weaponweightindex =
        WeaponWeightIndex((*ws).weaponweightconfig, weaponconfig);
    return 0i32;
}
//end of the function LoadWeaponConfig
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn WeaponWeightIndex(mut wwc: *mut weightconfig_t,
                                           mut wc: *mut weaponconfig_t)
 -> *mut libc::c_int {
    let mut index: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    index =
        GetClearedMemory((::std::mem::size_of::<libc::c_int>() as
                              libc::c_ulong).wrapping_mul((*wc).numweapons as
                                                              libc::c_ulong))
            as *mut libc::c_int;
    i = 0i32;
    while i < (*wc).numweapons {
        *index.offset(i as isize) =
            FindFuzzyWeight(wwc,
                            (*(*wc).weaponinfo.offset(i as
                                                          isize)).name.as_mut_ptr());
        i += 1
    }
    return index;
}
//returns a handle to a newly allocated weapon state
#[no_mangle]
pub unsafe extern "C" fn BotAllocWeaponState() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i <= 64i32 {
        if botweaponstates[i as usize].is_null() {
            botweaponstates[i as usize] =
                GetClearedMemory(::std::mem::size_of::<bot_weaponstate_t>() as
                                     libc::c_ulong) as *mut bot_weaponstate_t;
            return i
        }
        i += 1
    }
    return 0i32;
}
//resets the whole weapon state
#[no_mangle]
pub unsafe extern "C" fn BotResetWeaponState(mut weaponstate: libc::c_int) { }
unsafe extern "C" fn run_static_initializers() {
    projectileinfo_fields =
        [fielddef_s{name:
                        b"name\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).name as
                            *const [libc::c_char; 80] as
                            *mut [libc::c_char; 80] as size_t as libc::c_int,
                    type_0: 4i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"model\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).model as
                            *const [libc::c_char; 80] as
                            *mut [libc::c_char; 80] as size_t as libc::c_int,
                    type_0: 4i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"flags\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).flags as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"gravity\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).gravity as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"damage\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).damage as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"radius\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).radius as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"visdamage\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).visdamage as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"damagetype\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).damagetype as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"healthinc\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).healthinc as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"push\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).push as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"detonation\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).detonation as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"bounce\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).bounce as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"bouncefric\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).bouncefric as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"bouncestop\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const projectileinfo_t as
                                *mut projectileinfo_t)).bouncestop as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                    offset: 0i32,
                    type_0: 0i32,
                    maxarray: 0i32,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,}];
    weaponinfo_fields =
        [fielddef_s{name:
                        b"number\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).number as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"name\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).name as
                            *const [libc::c_char; 80] as
                            *mut [libc::c_char; 80] as size_t as libc::c_int,
                    type_0: 4i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"level\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).level as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"model\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).model as
                            *const [libc::c_char; 80] as
                            *mut [libc::c_char; 80] as size_t as libc::c_int,
                    type_0: 4i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"weaponindex\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).weaponindex as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"flags\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).flags as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"projectile\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).projectile as
                            *const [libc::c_char; 80] as
                            *mut [libc::c_char; 80] as size_t as libc::c_int,
                    type_0: 4i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"numprojectiles\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).numprojectiles as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"hspread\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).hspread as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"vspread\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).vspread as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"speed\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).speed as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"acceleration\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).acceleration as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"recoil\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).recoil as *const vec3_t as
                            *mut vec3_t as size_t as libc::c_int,
                    type_0: 3i32 | 0x100i32,
                    maxarray: 3i32,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"offset\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).offset as *const vec3_t as
                            *mut vec3_t as size_t as libc::c_int,
                    type_0: 3i32 | 0x100i32,
                    maxarray: 3i32,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"angleoffset\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).angleoffset as
                            *const vec3_t as *mut vec3_t as size_t as
                            libc::c_int,
                    type_0: 3i32 | 0x100i32,
                    maxarray: 3i32,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"extrazvelocity\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).extrazvelocity as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"ammoamount\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).ammoamount as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"ammoindex\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).ammoindex as
                            *const libc::c_int as *mut libc::c_int as size_t
                            as libc::c_int,
                    type_0: 2i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"activate\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).activate as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"reload\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).reload as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"spinup\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).spinup as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name:
                        b"spindown\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                    offset:
                        &(*(0 as *const weaponinfo_t as
                                *mut weaponinfo_t)).spindown as
                            *const libc::c_float as *mut libc::c_float as
                            size_t as libc::c_int,
                    type_0: 3i32,
                    maxarray: 0,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,},
         fielddef_s{name: 0 as *const libc::c_char as *mut libc::c_char,
                    offset: 0i32,
                    type_0: 0i32,
                    maxarray: 0i32,
                    floatmin: 0.,
                    floatmax: 0.,
                    substruct: 0 as *const structdef_s as *mut structdef_s,}]
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];