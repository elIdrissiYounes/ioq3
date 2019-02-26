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
        //read a token from the script
        #[no_mangle]
        pub fn PS_ReadToken(script: *mut script_t, token: *mut token_t)
         -> libc::c_int;
        //expect a certain token type
        #[no_mangle]
        pub fn PS_ExpectTokenType(script: *mut script_t, type_0: libc::c_int,
                                  subtype: libc::c_int, token: *mut token_t)
         -> libc::c_int;
        //remove any leading and trailing double quotes from the token
        #[no_mangle]
        pub fn StripDoubleQuotes(string: *mut libc::c_char);
        //set script flags
        #[no_mangle]
        pub fn SetScriptFlags(script: *mut script_t, flags: libc::c_int);
        //load a script from the given memory with the given length
        #[no_mangle]
        pub fn LoadScriptMemory(ptr: *mut libc::c_char, length: libc::c_int,
                                name: *mut libc::c_char) -> *mut script_t;
        //free a script
        #[no_mangle]
        pub fn FreeScript(script: *mut script_t);
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
      "ioq3/code/botlib/be_aas_bspq3.c"]
pub mod be_aas_bspq3_c {
    pub type bsp_t = bsp_s;
    //id Software BSP data
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_s {
        pub loaded: libc::c_int,
        pub entdatasize: libc::c_int,
        pub dentdata: *mut libc::c_char,
        pub numentities: libc::c_int,
        pub entities: [bsp_entity_t; 2048],
    }
    pub type bsp_entity_t = bsp_entity_s;
    //bsp data entity
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_entity_s {
        pub epairs: *mut bsp_epair_t,
    }
    pub type bsp_epair_t = bsp_epair_s;
    //bsp entity epair
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_epair_s {
        pub key: *mut libc::c_char,
        pub value: *mut libc::c_char,
        pub next: *mut bsp_epair_s,
    }
    use super::{libc};
    use super::botlib_h::{botlib_import_t};
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
 * name:		be_aas_bspq3.c
 *
 * desc:		BSP, Environment Sampling
 *
 * $Archive: /MissionPack/code/botlib/be_aas_bspq3.c $
 *
 *****************************************************************************/
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_def.h"]
pub mod be_aas_def_h {
    //structure to link entities to leaves and leaves to entities
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_link_s {
        pub entnum: libc::c_int,
        pub leafnum: libc::c_int,
        pub next_ent: *mut bsp_link_s,
        pub prev_ent: *mut bsp_link_s,
        pub next_leaf: *mut bsp_link_s,
        pub prev_leaf: *mut bsp_link_s,
    }
    pub type bsp_link_t = bsp_link_s;
    use super::{libc};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...)
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
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
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
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
        //
        //allocate a memory block of the given size
        #[no_mangle]
        pub fn GetHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
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
    }
}
#[header_src =
      "ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::botlib_h::{bsp_trace_t};
    use super::q_shared_h::{vec_t, qboolean};
    use super::{libc};
    use super::be_aas_def_h::{bsp_link_t};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t, PS_ReadToken, PS_ExpectTokenType,
                       StripDoubleQuotes, SetScriptFlags, LoadScriptMemory,
                       FreeScript};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_aas_bspq3_c::{bsp_t, bsp_s, bsp_entity_t, bsp_entity_s,
                           bsp_epair_t, bsp_epair_s, botimport};
use self::be_aas_def_h::{bsp_link_s, bsp_link_t};
use self::stdio_h::{sscanf};
use self::string_h::{memcpy, memset, strcpy, strncpy, strcmp, strlen};
use self::stdlib_h::{atof, atoi};
use self::l_memory_h::{GetHunkMemory, GetClearedHunkMemory, FreeMemory};
use self::l_variadic_h::{ScriptError};
//
//AASINTERN
//trace through the world
#[no_mangle]
pub unsafe extern "C" fn AAS_Trace(mut start: *mut vec_t,
                                   mut mins: *mut vec_t, mut maxs: *mut vec_t,
                                   mut end: *mut vec_t,
                                   mut passent: libc::c_int,
                                   mut contentmask: libc::c_int)
 -> bsp_trace_t {
    let mut bsptrace: bsp_trace_t =
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
    botimport.Trace.expect("non-null function pointer")(&mut bsptrace, start,
                                                        mins, maxs, end,
                                                        passent, contentmask);
    return bsptrace;
}
//returns the contents at the given point
#[no_mangle]
pub unsafe extern "C" fn AAS_PointContents(mut point: *mut vec_t)
 -> libc::c_int {
    return botimport.PointContents.expect("non-null function pointer")(point);
}
//returns true when p2 is in the PVS of p1
#[no_mangle]
pub unsafe extern "C" fn AAS_inPVS(mut p1: *mut vec_t, mut p2: *mut vec_t)
 -> qboolean {
    return botimport.inPVS.expect("non-null function pointer")(p1, p2) as
               qboolean;
}
//returns true when p2 is in the PHS of p1
#[no_mangle]
pub unsafe extern "C" fn AAS_inPHS(mut p1: *mut vec_t, mut p2: *mut vec_t)
 -> qboolean {
    return qtrue;
}
//creates a list with entities totally or partly within the given box
#[no_mangle]
pub unsafe extern "C" fn AAS_BoxEntities(mut absmins: *mut vec_t,
                                         mut absmaxs: *mut vec_t,
                                         mut list: *mut libc::c_int,
                                         mut maxcount: libc::c_int)
 -> libc::c_int {
    return 0i32;
}
//gets the mins, maxs and origin of a BSP model
#[no_mangle]
pub unsafe extern "C" fn AAS_BSPModelMinsMaxsOrigin(mut modelnum: libc::c_int,
                                                    mut angles: *mut vec_t,
                                                    mut mins: *mut vec_t,
                                                    mut maxs: *mut vec_t,
                                                    mut origin: *mut vec_t) {
    botimport.BSPModelMinsMaxsOrigin.expect("non-null function pointer")(modelnum,
                                                                         angles,
                                                                         mins,
                                                                         maxs,
                                                                         origin);
}
//handle to the next bsp entity
#[no_mangle]
pub unsafe extern "C" fn AAS_NextBSPEntity(mut ent: libc::c_int)
 -> libc::c_int {
    ent += 1;
    if ent >= 1i32 && ent < bspworld.numentities { return ent }
    return 0i32;
}
//global bsp
#[no_mangle]
pub static mut bspworld: bsp_t =
    bsp_s{loaded: 0,
          entdatasize: 0,
          dentdata: 0 as *const libc::c_char as *mut libc::c_char,
          numentities: 0,
          entities:
              [bsp_entity_s{epairs:
                                0 as *const bsp_epair_t as *mut bsp_epair_t,};
                  2048],};
//return the value of the BSP epair key
#[no_mangle]
pub unsafe extern "C" fn AAS_ValueForBSPEpairKey(mut ent: libc::c_int,
                                                 mut key: *mut libc::c_char,
                                                 mut value: *mut libc::c_char,
                                                 mut size: libc::c_int)
 -> libc::c_int {
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    *value.offset(0isize) = '\u{0}' as i32 as libc::c_char;
    if 0 == AAS_BSPEntityInRange(ent) { return qfalse as libc::c_int }
    epair = bspworld.entities[ent as usize].epairs;
    while !epair.is_null() {
        if 0 == strcmp((*epair).key, key) {
            strncpy(value, (*epair).value, (size - 1i32) as libc::c_ulong);
            *value.offset((size - 1i32) as isize) =
                '\u{0}' as i32 as libc::c_char;
            return qtrue as libc::c_int
        }
        epair = (*epair).next
    }
    return qfalse as libc::c_int;
}
//end of the function AAS_NextBSPEntity
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_BSPEntityInRange(mut ent: libc::c_int)
 -> libc::c_int {
    if ent <= 0i32 || ent >= bspworld.numentities {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"bsp entity out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//get a vector for the BSP epair key
#[no_mangle]
pub unsafe extern "C" fn AAS_VectorForBSPEpairKey(mut ent: libc::c_int,
                                                  mut key: *mut libc::c_char,
                                                  mut v: *mut vec_t)
 -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut v1: libc::c_double = 0.;
    let mut v2: libc::c_double = 0.;
    let mut v3: libc::c_double = 0.;
    let ref mut fresh1 = *v.offset(1isize);
    let ref mut fresh0 = *v.offset(2isize);
    *fresh0 = 0i32 as vec_t;
    *fresh1 = *fresh0;
    *v.offset(0isize) = *fresh1;
    if 0 == AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128i32) {
        return qfalse as libc::c_int
    }
    v3 = 0i32 as libc::c_double;
    v2 = v3;
    v1 = v2;
    sscanf(buf.as_mut_ptr(),
           b"%lf %lf %lf\x00" as *const u8 as *const libc::c_char,
           &mut v1 as *mut libc::c_double, &mut v2 as *mut libc::c_double,
           &mut v3 as *mut libc::c_double);
    *v.offset(0isize) = v1 as vec_t;
    *v.offset(1isize) = v2 as vec_t;
    *v.offset(2isize) = v3 as vec_t;
    return qtrue as libc::c_int;
}
//get a float for the BSP epair key
#[no_mangle]
pub unsafe extern "C" fn AAS_FloatForBSPEpairKey(mut ent: libc::c_int,
                                                 mut key: *mut libc::c_char,
                                                 mut value:
                                                     *mut libc::c_float)
 -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    *value = 0i32 as libc::c_float;
    if 0 == AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128i32) {
        return qfalse as libc::c_int
    }
    *value = atof(buf.as_mut_ptr()) as libc::c_float;
    return qtrue as libc::c_int;
}
//get an integer for the BSP epair key
#[no_mangle]
pub unsafe extern "C" fn AAS_IntForBSPEpairKey(mut ent: libc::c_int,
                                               mut key: *mut libc::c_char,
                                               mut value: *mut libc::c_int)
 -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    *value = 0i32;
    if 0 == AAS_ValueForBSPEpairKey(ent, key, buf.as_mut_ptr(), 128i32) {
        return qfalse as libc::c_int
    }
    *value = atoi(buf.as_mut_ptr());
    return qtrue as libc::c_int;
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
 * name:		be_aas_bsp.h
 *
 * desc:		AAS
 *
 * $Archive: /source/code/botlib/be_aas_bsp.h $
 *
 *****************************************************************************/
//loads the given BSP file
#[no_mangle]
pub unsafe extern "C" fn AAS_LoadBSPFile() -> libc::c_int {
    AAS_DumpBSPData();
    bspworld.entdatasize =
        strlen(botimport.BSPEntityData.expect("non-null function pointer")()).wrapping_add(1i32
                                                                                               as
                                                                                               libc::c_ulong)
            as libc::c_int;
    bspworld.dentdata =
        GetClearedHunkMemory(bspworld.entdatasize as libc::c_ulong) as
            *mut libc::c_char;
    memcpy(bspworld.dentdata as *mut libc::c_void,
           botimport.BSPEntityData.expect("non-null function pointer")() as
               *const libc::c_void, bspworld.entdatasize as libc::c_ulong);
    AAS_ParseBSPEntities();
    bspworld.loaded = qtrue as libc::c_int;
    return 0i32;
}
//end of the function AAS_FreeBSPEntities
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_ParseBSPEntities() {
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
    let mut ent: *mut bsp_entity_t = 0 as *mut bsp_entity_t;
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    script =
        LoadScriptMemory(bspworld.dentdata, bspworld.entdatasize,
                         b"entdata\x00" as *const u8 as *const libc::c_char as
                             *mut libc::c_char);
    SetScriptFlags(script, 0x4i32 | 0x8i32);
    bspworld.numentities = 1i32;
    while 0 != PS_ReadToken(script, &mut token) {
        if 0 !=
               strcmp(token.string.as_mut_ptr(),
                      b"{\x00" as *const u8 as *const libc::c_char) {
            ScriptError(script,
                        b"invalid %s\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char, token.string.as_mut_ptr());
            AAS_FreeBSPEntities();
            FreeScript(script);
            return
        }
        //end if
        if bspworld.numentities >= 2048i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"too many entities in BSP file\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
            break ;
        } else {
            ent =
                &mut *bspworld.entities.as_mut_ptr().offset(bspworld.numentities
                                                                as isize) as
                    *mut bsp_entity_t;
            bspworld.numentities += 1;
            (*ent).epairs = 0 as *mut bsp_epair_t;
            while 0 != PS_ReadToken(script, &mut token) {
                if 0 ==
                       strcmp(token.string.as_mut_ptr(),
                              b"}\x00" as *const u8 as *const libc::c_char) {
                    break ;
                }
                epair =
                    GetClearedHunkMemory(::std::mem::size_of::<bsp_epair_t>()
                                             as libc::c_ulong) as
                        *mut bsp_epair_t;
                (*epair).next = (*ent).epairs;
                (*ent).epairs = epair;
                if token.type_0 != 1i32 {
                    ScriptError(script,
                                b"invalid %s\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                token.string.as_mut_ptr());
                    AAS_FreeBSPEntities();
                    FreeScript(script);
                    return
                }
                StripDoubleQuotes(token.string.as_mut_ptr());
                (*epair).key =
                    GetHunkMemory(strlen(token.string.as_mut_ptr()).wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*epair).key, token.string.as_mut_ptr());
                if 0 == PS_ExpectTokenType(script, 1i32, 0i32, &mut token) {
                    AAS_FreeBSPEntities();
                    FreeScript(script);
                    return
                }
                StripDoubleQuotes(token.string.as_mut_ptr());
                (*epair).value =
                    GetHunkMemory(strlen(token.string.as_mut_ptr()).wrapping_add(1i32
                                                                                     as
                                                                                     libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*epair).value, token.string.as_mut_ptr());
            }
            if 0 !=
                   strcmp(token.string.as_mut_ptr(),
                          b"}\x00" as *const u8 as *const libc::c_char) {
                ScriptError(script,
                            b"missing }\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
                AAS_FreeBSPEntities();
                FreeScript(script);
                return
            }
        }
    }
    FreeScript(script);
}
//end of the function AAS_IntForBSPEpairKey
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_FreeBSPEntities() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut bsp_entity_t = 0 as *mut bsp_entity_t;
    let mut epair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    let mut nextepair: *mut bsp_epair_t = 0 as *mut bsp_epair_t;
    i = 1i32;
    while i < bspworld.numentities {
        ent =
            &mut *bspworld.entities.as_mut_ptr().offset(i as isize) as
                *mut bsp_entity_t;
        epair = (*ent).epairs;
        while !epair.is_null() {
            nextepair = (*epair).next;
            if !(*epair).key.is_null() {
                FreeMemory((*epair).key as *mut libc::c_void);
            }
            if !(*epair).value.is_null() {
                FreeMemory((*epair).value as *mut libc::c_void);
            }
            FreeMemory(epair as *mut libc::c_void);
            epair = nextepair
        }
        i += 1
    }
    bspworld.numentities = 0i32;
}
//dump the loaded BSP data
#[no_mangle]
pub unsafe extern "C" fn AAS_DumpBSPData() {
    AAS_FreeBSPEntities();
    if !bspworld.dentdata.is_null() {
        FreeMemory(bspworld.dentdata as *mut libc::c_void);
    }
    bspworld.dentdata = 0 as *mut libc::c_char;
    bspworld.entdatasize = 0i32;
    bspworld.loaded = qfalse as libc::c_int;
    memset(&mut bspworld as *mut bsp_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<bsp_t>() as libc::c_ulong);
}
//unlink the given entity from the bsp tree leaves
#[no_mangle]
pub unsafe extern "C" fn AAS_UnlinkFromBSPLeaves(mut leaves:
                                                     *mut bsp_link_t) {
}
//link the given entity to the bsp tree leaves of the given model
#[no_mangle]
pub unsafe extern "C" fn AAS_BSPLinkEntity(mut absmins: *mut vec_t,
                                           mut absmaxs: *mut vec_t,
                                           mut entnum: libc::c_int,
                                           mut modelnum: libc::c_int)
 -> *mut bsp_link_t {
    return 0 as *mut bsp_link_t;
}
//calculates collision with given entity
#[no_mangle]
pub unsafe extern "C" fn AAS_EntityCollision(mut entnum: libc::c_int,
                                             mut start: *mut vec_t,
                                             mut boxmins: *mut vec_t,
                                             mut boxmaxs: *mut vec_t,
                                             mut end: *mut vec_t,
                                             mut contentmask: libc::c_int,
                                             mut trace: *mut bsp_trace_t)
 -> qboolean {
    let mut enttrace: bsp_trace_t =
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
    botimport.EntityTrace.expect("non-null function pointer")(&mut enttrace,
                                                              start, boxmins,
                                                              boxmaxs, end,
                                                              entnum,
                                                              contentmask);
    if enttrace.fraction < (*trace).fraction {
        memcpy(trace as *mut libc::c_void,
               &mut enttrace as *mut bsp_trace_t as *const libc::c_void,
               ::std::mem::size_of::<bsp_trace_t>() as libc::c_ulong);
        return qtrue
    }
    return qfalse;
}
//end of the function AAS_ParseBSPEntities
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AAS_BSPTraceLight(mut start: *mut vec_t,
                                           mut end: *mut vec_t,
                                           mut endpos: *mut vec_t,
                                           mut red: *mut libc::c_int,
                                           mut green: *mut libc::c_int,
                                           mut blue: *mut libc::c_int)
 -> libc::c_int {
    return 0i32;
}