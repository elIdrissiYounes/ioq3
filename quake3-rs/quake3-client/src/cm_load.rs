#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           extern_types,
           libc,
           ptr_wrapping_offset_from)]
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
    pub type clipHandle_t = libc::c_int;
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
    // font rendering values used by ui and cgame
    // default
    // default
    pub type ha_pref = libc::c_uint;
    pub const h_dontcare: ha_pref = 2;
    pub const h_low: ha_pref = 1;
    pub const h_high: ha_pref = 0;
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
    /*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
					// specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
					// without proper initialization.  modified
					// will be set, even though the value hasn't
					// changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    pub type cvar_t = cvar_s;
    pub type cplane_t = cplane_s;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Hunk_AllocDebug(size: libc::c_int, preference: ha_pref,
                               label: *mut libc::c_char,
                               file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn SetPlaneSignbits(out: *mut cplane_s);
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
    }
}
#[header_src =
      "ioq3/code/qcommon/qfiles.h"]
pub mod qfiles_h {
    /*
==============================================================================

  .BSP file format

==============================================================================
*/
    // little-endian "IBSP"
    // there shouldn't be any problem with increasing these values at the
// expense of more memory allocation in the utilities
    // MAX_MAP_AREA_BYTES in q_shared must match!
    // key / value pair sizes in the entities lump
    // the editor uses these predefined yaw angles to orient entities up or down
    //=============================================================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct lump_t {
        pub fileofs: libc::c_int,
        pub filelen: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dheader_t {
        pub ident: libc::c_int,
        pub version: libc::c_int,
        pub lumps: [lump_t; 17],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dmodel_t {
        pub mins: [libc::c_float; 3],
        pub maxs: [libc::c_float; 3],
        pub firstSurface: libc::c_int,
        pub numSurfaces: libc::c_int,
        pub firstBrush: libc::c_int,
        pub numBrushes: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dshader_t {
        pub shader: [libc::c_char; 64],
        pub surfaceFlags: libc::c_int,
        pub contentFlags: libc::c_int,
    }
    // planes x^1 is allways the opposite of plane x
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dplane_t {
        pub normal: [libc::c_float; 3],
        pub dist: libc::c_float,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dnode_t {
        pub planeNum: libc::c_int,
        pub children: [libc::c_int; 2],
        pub mins: [libc::c_int; 3],
        pub maxs: [libc::c_int; 3],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dleaf_t {
        pub cluster: libc::c_int,
        pub area: libc::c_int,
        pub mins: [libc::c_int; 3],
        pub maxs: [libc::c_int; 3],
        pub firstLeafSurface: libc::c_int,
        pub numLeafSurfaces: libc::c_int,
        pub firstLeafBrush: libc::c_int,
        pub numLeafBrushes: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dbrushside_t {
        pub planeNum: libc::c_int,
        pub shaderNum: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dbrush_t {
        pub firstSide: libc::c_int,
        pub numSides: libc::c_int,
        pub shaderNum: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct drawVert_t {
        pub xyz: vec3_t,
        pub st: [libc::c_float; 2],
        pub lightmap: [libc::c_float; 2],
        pub normal: vec3_t,
        pub color: [byte; 4],
    }
    pub type unnamed_0 = libc::c_uint;
    pub const MST_FLARE: unnamed_0 = 4;
    pub const MST_TRIANGLE_SOUP: unnamed_0 = 3;
    pub const MST_PATCH: unnamed_0 = 2;
    pub const MST_PLANAR: unnamed_0 = 1;
    pub const MST_BAD: unnamed_0 = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dsurface_t {
        pub shaderNum: libc::c_int,
        pub fogNum: libc::c_int,
        pub surfaceType: libc::c_int,
        pub firstVert: libc::c_int,
        pub numVerts: libc::c_int,
        pub firstIndex: libc::c_int,
        pub numIndexes: libc::c_int,
        pub lightmapNum: libc::c_int,
        pub lightmapX: libc::c_int,
        pub lightmapY: libc::c_int,
        pub lightmapWidth: libc::c_int,
        pub lightmapHeight: libc::c_int,
        pub lightmapOrigin: vec3_t,
        pub lightmapVecs: [vec3_t; 3],
        pub patchWidth: libc::c_int,
        pub patchHeight: libc::c_int,
    }
    use super::{libc};
    use super::q_shared_h::{vec3_t, byte};
}
#[header_src =
      "ioq3/code/qcommon/cm_local.h"]
pub mod cm_local_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clipMap_t {
        pub name: [libc::c_char; 64],
        pub numShaders: libc::c_int,
        pub shaders: *mut dshader_t,
        pub numBrushSides: libc::c_int,
        pub brushsides: *mut cbrushside_t,
        pub numPlanes: libc::c_int,
        pub planes: *mut cplane_t,
        pub numNodes: libc::c_int,
        pub nodes: *mut cNode_t,
        pub numLeafs: libc::c_int,
        pub leafs: *mut cLeaf_t,
        pub numLeafBrushes: libc::c_int,
        pub leafbrushes: *mut libc::c_int,
        pub numLeafSurfaces: libc::c_int,
        pub leafsurfaces: *mut libc::c_int,
        pub numSubModels: libc::c_int,
        pub cmodels: *mut cmodel_t,
        pub numBrushes: libc::c_int,
        pub brushes: *mut cbrush_t,
        pub numClusters: libc::c_int,
        pub clusterBytes: libc::c_int,
        pub visibility: *mut byte,
        pub vised: qboolean,
        pub numEntityChars: libc::c_int,
        pub entityString: *mut libc::c_char,
        pub numAreas: libc::c_int,
        pub areas: *mut cArea_t,
        pub areaPortals: *mut libc::c_int,
        pub numSurfaces: libc::c_int,
        pub surfaces: *mut *mut cPatch_t,
        pub floodvalid: libc::c_int,
        pub checkcount: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cPatch_t {
        pub checkcount: libc::c_int,
        pub surfaceFlags: libc::c_int,
        pub contents: libc::c_int,
        pub pc: *mut patchCollide_s,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cArea_t {
        pub floodnum: libc::c_int,
        pub floodvalid: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cbrush_t {
        pub shaderNum: libc::c_int,
        pub contents: libc::c_int,
        pub bounds: [vec3_t; 2],
        pub numsides: libc::c_int,
        pub sides: *mut cbrushside_t,
        pub checkcount: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cbrushside_t {
        pub plane: *mut cplane_t,
        pub surfaceFlags: libc::c_int,
        pub shaderNum: libc::c_int,
    }
    pub type cmodel_t = cmodel_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cmodel_s {
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub leaf: cLeaf_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cLeaf_t {
        pub cluster: libc::c_int,
        pub area: libc::c_int,
        pub firstLeafBrush: libc::c_int,
        pub numLeafBrushes: libc::c_int,
        pub firstLeafSurface: libc::c_int,
        pub numLeafSurfaces: libc::c_int,
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cNode_t {
        pub plane: *mut cplane_t,
        pub children: [libc::c_int; 2],
    }
    use super::{libc};
    use super::qfiles_h::{dshader_t};
    use super::q_shared_h::{cplane_t, byte, qboolean, vec3_t, cvar_t,
                            clipHandle_t};
    extern "C" {
        pub type patchCollide_s;
        // cm_patch.c
        #[no_mangle]
        pub fn CM_GeneratePatchCollide(width: libc::c_int,
                                       height: libc::c_int,
                                       points: *mut vec3_t)
         -> *mut patchCollide_s;
        #[no_mangle]
        pub fn CM_ClearLevelPatches();
    }
}
#[header_src =
      "ioq3/code/qcommon/cm_load.c"]
pub mod cm_load_c {
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_1 {
        pub i: *mut libc::c_int,
        pub v: *mut libc::c_void,
    }
    use super::{libc};
    use super::q_shared_h::{cplane_t, byte};
    use super::cm_local_h::{cmodel_t, cbrush_t};
    use super::qfiles_h::{lump_t, dheader_t};
    extern "C" {
        #[no_mangle]
        pub fn CM_FloodAreaConnections();
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
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::{libc};
    use super::q_shared_h::{qboolean, clipHandle_t, vec_t};
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    use super::q_shared_h::{cvar_t};
    extern "C" {
        // forces flush on files we're writing to.
        #[no_mangle]
        pub fn FS_FreeFile(buffer: *mut libc::c_void);
        #[no_mangle]
        pub fn Com_BlockChecksum(buffer: *const libc::c_void,
                                 length: libc::c_int) -> libc::c_uint;
        #[no_mangle]
        pub fn FS_ReadFile(qpath: *const libc::c_char,
                           buffer: *mut *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
        /*
==============================================================

CVAR

==============================================================
*/
        /*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
        #[no_mangle]
        pub fn Cvar_Get(var_name: *const libc::c_char,
                        value: *const libc::c_char, flags: libc::c_int)
         -> *mut cvar_t;
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, clipHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, ha_pref, h_dontcare, h_low,
                       h_high, vec_t, vec3_t, cplane_s, cvar_s, cvar_t,
                       cplane_t, Hunk_AllocDebug, SetPlaneSignbits,
                       Q_strncpyz, Com_Error};
use self::qfiles_h::{lump_t, dheader_t, dmodel_t, dshader_t, dplane_t,
                     dnode_t, dleaf_t, dbrushside_t, dbrush_t, drawVert_t,
                     unnamed_0, MST_FLARE, MST_TRIANGLE_SOUP, MST_PATCH,
                     MST_PLANAR, MST_BAD, dsurface_t};
use self::cm_local_h::{clipMap_t, cPatch_t, cArea_t, cbrush_t, cbrushside_t,
                       cmodel_t, cmodel_s, cLeaf_t, cNode_t, patchCollide_s,
                       CM_GeneratePatchCollide, CM_ClearLevelPatches};
use self::cm_load_c::{unnamed_1, CM_FloodAreaConnections};
use self::string_h::{memcpy, memset, strcmp};
use self::qcommon_h::{FS_FreeFile, Com_BlockChecksum, FS_ReadFile,
                      Com_DPrintf, Cvar_Get};
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
#[no_mangle]
pub unsafe extern "C" fn CM_LoadMap(mut name: *const libc::c_char,
                                    mut clientload: qboolean,
                                    mut checksum: *mut libc::c_int) {
    let mut buf: unnamed_1 = unnamed_1{i: 0 as *mut libc::c_int,};
    let mut i: libc::c_int = 0;
    let mut header: dheader_t =
        dheader_t{ident: 0,
                  version: 0,
                  lumps: [lump_t{fileofs: 0, filelen: 0,}; 17],};
    let mut length: libc::c_int = 0;
    static mut last_checksum: libc::c_uint = 0;
    if name.is_null() || 0 == *name.offset(0isize) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_LoadMap: NULL name\x00" as *const u8 as
                      *const libc::c_char);
    }
    cm_noAreas =
        Cvar_Get(b"cm_noAreas\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    cm_noCurves =
        Cvar_Get(b"cm_noCurves\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    cm_playerCurveClip =
        Cvar_Get(b"cm_playerCurveClip\x00" as *const u8 as
                     *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x200i32);
    Com_DPrintf(b"CM_LoadMap( %s, %i )\n\x00" as *const u8 as
                    *const libc::c_char, name, clientload as libc::c_uint);
    if 0 == strcmp(cm.name.as_mut_ptr(), name) &&
           0 != clientload as libc::c_uint {
        *checksum = last_checksum as libc::c_int;
        return
    }
    memset(&mut cm as *mut clipMap_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<clipMap_t>() as libc::c_ulong);
    CM_ClearLevelPatches();
    if 0 == *name.offset(0isize) {
        cm.numLeafs = 1i32;
        cm.numClusters = 1i32;
        cm.numAreas = 1i32;
        cm.cmodels =
            Hunk_AllocDebug(::std::mem::size_of::<cmodel_t>() as libc::c_ulong
                                as libc::c_int, h_high,
                            b"sizeof( *cm.cmodels )\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            b"code/qcommon/cm_load.c\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            603i32) as *mut cmodel_t;
        *checksum = 0i32;
        return
    }
    length = FS_ReadFile(name, &mut buf.v) as libc::c_int;
    if buf.i.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Couldn\'t load %s\x00" as *const u8 as
                      *const libc::c_char, name);
    }
    last_checksum = Com_BlockChecksum(buf.i as *const libc::c_void, length);
    *checksum = last_checksum as libc::c_int;
    header = *(buf.i as *mut dheader_t);
    i = 0i32;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<dheader_t>() as
                   libc::c_ulong).wrapping_div(4i32 as libc::c_ulong) {
        *(&mut header as *mut dheader_t as
              *mut libc::c_int).offset(i as isize) =
            *(&mut header as *mut dheader_t as
                  *mut libc::c_int).offset(i as isize);
        i += 1
    }
    if header.version != 46i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_LoadMap: %s has wrong version number (%i should be %i)\x00"
                      as *const u8 as *const libc::c_char, name,
                  header.version, 46i32);
    }
    cmod_base = buf.i as *mut byte;
    CMod_LoadShaders(&mut *header.lumps.as_mut_ptr().offset(1isize));
    CMod_LoadLeafs(&mut *header.lumps.as_mut_ptr().offset(4isize));
    CMod_LoadLeafBrushes(&mut *header.lumps.as_mut_ptr().offset(6isize));
    CMod_LoadLeafSurfaces(&mut *header.lumps.as_mut_ptr().offset(5isize));
    CMod_LoadPlanes(&mut *header.lumps.as_mut_ptr().offset(2isize));
    CMod_LoadBrushSides(&mut *header.lumps.as_mut_ptr().offset(9isize));
    CMod_LoadBrushes(&mut *header.lumps.as_mut_ptr().offset(8isize));
    CMod_LoadSubmodels(&mut *header.lumps.as_mut_ptr().offset(7isize));
    CMod_LoadNodes(&mut *header.lumps.as_mut_ptr().offset(3isize));
    CMod_LoadEntityString(&mut *header.lumps.as_mut_ptr().offset(0isize));
    CMod_LoadVisibility(&mut *header.lumps.as_mut_ptr().offset(16isize));
    CMod_LoadPatches(&mut *header.lumps.as_mut_ptr().offset(13isize),
                     &mut *header.lumps.as_mut_ptr().offset(10isize));
    FS_FreeFile(buf.v);
    CM_InitBoxHull();
    CM_FloodAreaConnections();
    if 0 == clientload as u64 {
        Q_strncpyz(cm.name.as_mut_ptr(), name,
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
    };
}
// keep 1/8 unit away to keep the position valid before network snapping
// and to avoid various numeric issues
#[no_mangle]
pub static mut cm: clipMap_t =
    clipMap_t{name: [0; 64],
              numShaders: 0,
              shaders: 0 as *const dshader_t as *mut dshader_t,
              numBrushSides: 0,
              brushsides: 0 as *const cbrushside_t as *mut cbrushside_t,
              numPlanes: 0,
              planes: 0 as *const cplane_t as *mut cplane_t,
              numNodes: 0,
              nodes: 0 as *const cNode_t as *mut cNode_t,
              numLeafs: 0,
              leafs: 0 as *const cLeaf_t as *mut cLeaf_t,
              numLeafBrushes: 0,
              leafbrushes: 0 as *const libc::c_int as *mut libc::c_int,
              numLeafSurfaces: 0,
              leafsurfaces: 0 as *const libc::c_int as *mut libc::c_int,
              numSubModels: 0,
              cmodels: 0 as *const cmodel_t as *mut cmodel_t,
              numBrushes: 0,
              brushes: 0 as *const cbrush_t as *mut cbrush_t,
              numClusters: 0,
              clusterBytes: 0,
              visibility: 0 as *const byte as *mut byte,
              vised: qfalse,
              numEntityChars: 0,
              entityString: 0 as *const libc::c_char as *mut libc::c_char,
              numAreas: 0,
              areas: 0 as *const cArea_t as *mut cArea_t,
              areaPortals: 0 as *const libc::c_int as *mut libc::c_int,
              numSurfaces: 0,
              surfaces: 0 as *const *mut cPatch_t as *mut *mut cPatch_t,
              floodvalid: 0,
              checkcount: 0,};
#[no_mangle]
pub unsafe extern "C" fn CM_InitBoxHull() {
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut p: *mut cplane_t = 0 as *mut cplane_t;
    let mut s: *mut cbrushside_t = 0 as *mut cbrushside_t;
    box_planes =
        &mut *cm.planes.offset(cm.numPlanes as isize) as *mut cplane_t;
    box_brush =
        &mut *cm.brushes.offset(cm.numBrushes as isize) as *mut cbrush_t;
    (*box_brush).numsides = 6i32;
    (*box_brush).sides = cm.brushsides.offset(cm.numBrushSides as isize);
    (*box_brush).contents = 0x2000000i32;
    box_model.leaf.numLeafBrushes = 1i32;
    box_model.leaf.firstLeafBrush = cm.numLeafBrushes;
    *cm.leafbrushes.offset(cm.numLeafBrushes as isize) = cm.numBrushes;
    i = 0i32;
    while i < 6i32 {
        side = i & 1i32;
        s =
            &mut *cm.brushsides.offset((cm.numBrushSides + i) as isize) as
                *mut cbrushside_t;
        (*s).plane =
            cm.planes.offset((cm.numPlanes + i * 2i32 + side) as isize);
        (*s).surfaceFlags = 0i32;
        p = &mut *box_planes.offset((i * 2i32) as isize) as *mut cplane_t;
        (*p).type_0 = (i >> 1i32) as byte;
        (*p).signbits = 0i32 as byte;
        (*p).normal[2usize] = 0i32 as vec_t;
        (*p).normal[1usize] = (*p).normal[2usize];
        (*p).normal[0usize] = (*p).normal[1usize];
        (*p).normal[(i >> 1i32) as usize] = 1i32 as vec_t;
        p =
            &mut *box_planes.offset((i * 2i32 + 1i32) as isize) as
                *mut cplane_t;
        (*p).type_0 = (3i32 + (i >> 1i32)) as byte;
        (*p).signbits = 0i32 as byte;
        (*p).normal[2usize] = 0i32 as vec_t;
        (*p).normal[1usize] = (*p).normal[2usize];
        (*p).normal[0usize] = (*p).normal[1usize];
        (*p).normal[(i >> 1i32) as usize] = -1i32 as vec_t;
        SetPlaneSignbits(p);
        i += 1
    };
}
#[no_mangle]
pub static mut box_planes: *mut cplane_t =
    0 as *const cplane_t as *mut cplane_t;
#[no_mangle]
pub static mut box_model: cmodel_t =
    cmodel_s{mins: [0.; 3],
             maxs: [0.; 3],
             leaf:
                 cLeaf_t{cluster: 0,
                         area: 0,
                         firstLeafBrush: 0,
                         numLeafBrushes: 0,
                         firstLeafSurface: 0,
                         numLeafSurfaces: 0,},};
#[no_mangle]
pub static mut box_brush: *mut cbrush_t =
    0 as *const cbrush_t as *mut cbrush_t;
//==================================================================
/*
=================
CMod_LoadPatches
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadPatches(mut surfs: *mut lump_t,
                                          mut verts: *mut lump_t) {
    let mut dv: *mut drawVert_t = 0 as *mut drawVert_t;
    let mut dv_p: *mut drawVert_t = 0 as *mut drawVert_t;
    let mut in_0: *mut dsurface_t = 0 as *mut dsurface_t;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut patch: *mut cPatch_t = 0 as *mut cPatch_t;
    let mut points: [vec3_t; 1024] = [[0.; 3]; 1024];
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut shaderNum: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*surfs).fileofs as isize) as *mut libc::c_void as
            *mut dsurface_t;
    if 0 !=
           ((*surfs).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dsurface_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*surfs).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dsurface_t>()
                                             as libc::c_ulong) as libc::c_int;
    cm.numSurfaces = count;
    cm.surfaces =
        Hunk_AllocDebug((cm.numSurfaces as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut cPatch_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"cm.numSurfaces * sizeof( cm.surfaces[0] )\x00" as
                            *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 499i32)
            as *mut *mut cPatch_t;
    dv =
        cmod_base.offset((*verts).fileofs as isize) as *mut libc::c_void as
            *mut drawVert_t;
    if 0 !=
           ((*verts).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<drawVert_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    i = 0i32;
    while i < count {
        if !((*in_0).surfaceType != MST_PATCH as libc::c_int) {
            // ignore other surfaces
            let ref mut fresh0 = *cm.surfaces.offset(i as isize);
            patch =
                Hunk_AllocDebug(::std::mem::size_of::<cPatch_t>() as
                                    libc::c_ulong as libc::c_int, h_high,
                                b"sizeof( *patch )\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                b"code/qcommon/cm_load.c\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                513i32) as *mut cPatch_t;
            *fresh0 = patch;
            width = (*in_0).patchWidth;
            height = (*in_0).patchHeight;
            c = width * height;
            if c > 1024i32 {
                Com_Error(ERR_DROP as libc::c_int,
                          b"ParseMesh: MAX_PATCH_VERTS\x00" as *const u8 as
                              *const libc::c_char);
            }
            dv_p = dv.offset((*in_0).firstVert as isize);
            j = 0i32;
            while j < c {
                points[j as usize][0usize] = (*dv_p).xyz[0usize];
                points[j as usize][1usize] = (*dv_p).xyz[1usize];
                points[j as usize][2usize] = (*dv_p).xyz[2usize];
                j += 1;
                dv_p = dv_p.offset(1isize)
            }
            shaderNum = (*in_0).shaderNum;
            (*patch).contents =
                (*cm.shaders.offset(shaderNum as isize)).contentFlags;
            (*patch).surfaceFlags =
                (*cm.shaders.offset(shaderNum as isize)).surfaceFlags;
            (*patch).pc =
                CM_GeneratePatchCollide(width, height, points.as_mut_ptr())
        }
        i += 1;
        in_0 = in_0.offset(1isize)
    };
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
// cmodel.c -- model loading
//BSPC
// to allow boxes to be treated as brush models, we allocate
// some extra indexes along with those needed by the map
#[no_mangle]
pub static mut cmod_base: *mut byte = 0 as *const byte as *mut byte;
/*
=================
CMod_LoadVisibility
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadVisibility(mut l: *mut lump_t) {
    let mut len: libc::c_int = 0;
    let mut buf: *mut byte = 0 as *mut byte;
    len = (*l).filelen;
    if 0 == len {
        cm.clusterBytes = cm.numClusters + 31i32 & !31i32;
        cm.visibility =
            Hunk_AllocDebug(cm.clusterBytes, h_high,
                            b"cm.clusterBytes\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            b"code/qcommon/cm_load.c\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            462i32) as *mut byte;
        memset(cm.visibility as *mut libc::c_void, 255i32,
               cm.clusterBytes as libc::c_ulong);
        return
    }
    buf = cmod_base.offset((*l).fileofs as isize);
    cm.vised = qtrue;
    cm.visibility =
        Hunk_AllocDebug(len, h_high,
                        b"len\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 469i32)
            as *mut byte;
    cm.numClusters = *(buf as *mut libc::c_int).offset(0isize);
    cm.clusterBytes = *(buf as *mut libc::c_int).offset(1isize);
    memcpy(cm.visibility as *mut libc::c_void,
           buf.offset(8isize) as *const libc::c_void,
           (len - 8i32) as libc::c_ulong);
}
/*
=================
CMod_LoadEntityString
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadEntityString(mut l: *mut lump_t) {
    cm.entityString =
        Hunk_AllocDebug((*l).filelen, h_high,
                        b"l->filelen\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 444i32)
            as *mut libc::c_char;
    cm.numEntityChars = (*l).filelen;
    memcpy(cm.entityString as *mut libc::c_void,
           cmod_base.offset((*l).fileofs as isize) as *const libc::c_void,
           (*l).filelen as libc::c_ulong);
}
/*
=================
CMod_LoadNodes

=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadNodes(mut l: *mut lump_t) {
    let mut in_0: *mut dnode_t = 0 as *mut dnode_t;
    let mut child: libc::c_int = 0;
    let mut out: *mut cNode_t = 0 as *mut cNode_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut dnode_t;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dnode_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dnode_t>() as
                                             libc::c_ulong) as libc::c_int;
    if count < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Map has no nodes\x00" as *const u8 as
                      *const libc::c_char);
    }
    cm.nodes =
        Hunk_AllocDebug((count as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cNode_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"count * sizeof( *cm.nodes )\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 192i32)
            as *mut cNode_t;
    cm.numNodes = count;
    out = cm.nodes;
    i = 0i32;
    while i < count {
        (*out).plane = cm.planes.offset((*in_0).planeNum as isize);
        j = 0i32;
        while j < 2i32 {
            child = (*in_0).children[j as usize];
            (*out).children[j as usize] = child;
            j += 1
        }
        i += 1;
        out = out.offset(1isize);
        in_0 = in_0.offset(1isize)
    };
}
/*
=================
CMod_LoadSubmodels
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadSubmodels(mut l: *mut lump_t) {
    let mut in_0: *mut dmodel_t = 0 as *mut dmodel_t;
    let mut out: *mut cmodel_t = 0 as *mut cmodel_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut indexes: *mut libc::c_int = 0 as *mut libc::c_int;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut dmodel_t;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dmodel_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CMod_LoadSubmodels: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dmodel_t>() as
                                             libc::c_ulong) as libc::c_int;
    if count < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Map with no models\x00" as *const u8 as
                      *const libc::c_char);
    }
    cm.cmodels =
        Hunk_AllocDebug((count as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cmodel_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"count * sizeof( *cm.cmodels )\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 134i32)
            as *mut cmodel_t;
    cm.numSubModels = count;
    if count > 256i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MAX_SUBMODELS exceeded\x00" as *const u8 as
                      *const libc::c_char);
    }
    i = 0i32;
    while i < count {
        out = &mut *cm.cmodels.offset(i as isize) as *mut cmodel_t;
        j = 0i32;
        while j < 3i32 {
            (*out).mins[j as usize] =
                (*in_0).mins[j as usize] - 1i32 as libc::c_float;
            (*out).maxs[j as usize] =
                (*in_0).maxs[j as usize] + 1i32 as libc::c_float;
            j += 1
        }
        if !(i == 0i32) {
            // world model doesn't need other info
            (*out).leaf.numLeafBrushes = (*in_0).numBrushes;
            indexes =
                Hunk_AllocDebug((*out).leaf.numLeafBrushes * 4i32, h_high,
                                b"out->leaf.numLeafBrushes * 4\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                                b"code/qcommon/cm_load.c\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                157i32) as *mut libc::c_int;
            (*out).leaf.firstLeafBrush =
                indexes.wrapping_offset_from(cm.leafbrushes) as libc::c_long
                    as libc::c_int;
            j = 0i32;
            while j < (*out).leaf.numLeafBrushes {
                *indexes.offset(j as isize) = (*in_0).firstBrush + j;
                j += 1
            }
            (*out).leaf.numLeafSurfaces = (*in_0).numSurfaces;
            indexes =
                Hunk_AllocDebug((*out).leaf.numLeafSurfaces * 4i32, h_high,
                                b"out->leaf.numLeafSurfaces * 4\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char,
                                b"code/qcommon/cm_load.c\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                164i32) as *mut libc::c_int;
            (*out).leaf.firstLeafSurface =
                indexes.wrapping_offset_from(cm.leafsurfaces) as libc::c_long
                    as libc::c_int;
            j = 0i32;
            while j < (*out).leaf.numLeafSurfaces {
                *indexes.offset(j as isize) = (*in_0).firstSurface + j;
                j += 1
            }
        }
        i += 1;
        in_0 = in_0.offset(1isize)
    };
}
/*
=================
CMod_LoadBrushes

=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadBrushes(mut l: *mut lump_t) {
    let mut in_0: *mut dbrush_t = 0 as *mut dbrush_t;
    let mut out: *mut cbrush_t = 0 as *mut cbrush_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut dbrush_t;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dbrush_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dbrush_t>() as
                                             libc::c_ulong) as libc::c_int;
    cm.brushes =
        Hunk_AllocDebug(((1i32 + count) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cbrush_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"( BOX_BRUSHES + count ) * sizeof( *cm.brushes )\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 244i32)
            as *mut cbrush_t;
    cm.numBrushes = count;
    out = cm.brushes;
    i = 0i32;
    while i < count {
        (*out).sides = cm.brushsides.offset((*in_0).firstSide as isize);
        (*out).numsides = (*in_0).numSides;
        (*out).shaderNum = (*in_0).shaderNum;
        if (*out).shaderNum < 0i32 || (*out).shaderNum >= cm.numShaders {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CMod_LoadBrushes: bad shaderNum: %i\x00" as *const u8
                          as *const libc::c_char, (*out).shaderNum);
        }
        (*out).contents =
            (*cm.shaders.offset((*out).shaderNum as isize)).contentFlags;
        CM_BoundBrush(out);
        i += 1;
        out = out.offset(1isize);
        in_0 = in_0.offset(1isize)
    };
}
/*
=================
CM_BoundBrush

=================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_BoundBrush(mut b: *mut cbrush_t) {
    (*b).bounds[0usize][0usize] = -(*(*(*b).sides.offset(0isize)).plane).dist;
    (*b).bounds[1usize][0usize] = (*(*(*b).sides.offset(1isize)).plane).dist;
    (*b).bounds[0usize][1usize] = -(*(*(*b).sides.offset(2isize)).plane).dist;
    (*b).bounds[1usize][1usize] = (*(*(*b).sides.offset(3isize)).plane).dist;
    (*b).bounds[0usize][2usize] = -(*(*(*b).sides.offset(4isize)).plane).dist;
    (*b).bounds[1usize][2usize] = (*(*(*b).sides.offset(5isize)).plane).dist;
}
/*
=================
CMod_LoadBrushSides
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadBrushSides(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut cbrushside_t = 0 as *mut cbrushside_t;
    let mut in_0: *mut dbrushside_t = 0 as *mut dbrushside_t;
    let mut count: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut dbrushside_t;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dbrushside_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dbrushside_t>()
                                             as libc::c_ulong) as libc::c_int;
    cm.brushsides =
        Hunk_AllocDebug(((6i32 + count) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cbrushside_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"( BOX_SIDES + count ) * sizeof( *cm.brushsides )\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 421i32)
            as *mut cbrushside_t;
    cm.numBrushSides = count;
    out = cm.brushsides;
    i = 0i32;
    while i < count {
        num = (*in_0).planeNum;
        (*out).plane = &mut *cm.planes.offset(num as isize) as *mut cplane_t;
        (*out).shaderNum = (*in_0).shaderNum;
        if (*out).shaderNum < 0i32 || (*out).shaderNum >= cm.numShaders {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CMod_LoadBrushSides: bad shaderNum: %i\x00" as
                          *const u8 as *const libc::c_char, (*out).shaderNum);
        }
        (*out).surfaceFlags =
            (*cm.shaders.offset((*out).shaderNum as isize)).surfaceFlags;
        i += 1;
        in_0 = in_0.offset(1isize);
        out = out.offset(1isize)
    };
}
/*
=================
CMod_LoadPlanes
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadPlanes(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut out: *mut cplane_t = 0 as *mut cplane_t;
    let mut in_0: *mut dplane_t = 0 as *mut dplane_t;
    let mut count: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut dplane_t;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dplane_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dplane_t>() as
                                             libc::c_ulong) as libc::c_int;
    if count < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Map with no planes\x00" as *const u8 as
                      *const libc::c_char);
    }
    cm.planes =
        Hunk_AllocDebug(((12i32 + count) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cplane_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"( BOX_PLANES + count ) * sizeof( *cm.planes )\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 327i32)
            as *mut cplane_t;
    cm.numPlanes = count;
    out = cm.planes;
    i = 0i32;
    while i < count {
        bits = 0i32;
        j = 0i32;
        while j < 3i32 {
            (*out).normal[j as usize] = (*in_0).normal[j as usize];
            if (*out).normal[j as usize] < 0i32 as libc::c_float {
                bits |= 1i32 << j
            }
            j += 1
        }
        (*out).dist = (*in_0).dist;
        (*out).type_0 =
            (if (*out).normal[0usize] as libc::c_double == 1.0f64 {
                 0i32
             } else if (*out).normal[1usize] as libc::c_double == 1.0f64 {
                 1i32
             } else if (*out).normal[2usize] as libc::c_double == 1.0f64 {
                 2i32
             } else { 3i32 }) as byte;
        (*out).signbits = bits as byte;
        i += 1;
        in_0 = in_0.offset(1isize);
        out = out.offset(1isize)
    };
}
/*
=================
CMod_LoadLeafSurfaces
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadLeafSurfaces(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut in_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut count: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut libc::c_int;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                             as libc::c_ulong) as libc::c_int;
    cm.leafsurfaces =
        Hunk_AllocDebug((count as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"count * sizeof( *cm.leafsurfaces )\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 392i32)
            as *mut libc::c_int;
    cm.numLeafSurfaces = count;
    out = cm.leafsurfaces;
    i = 0i32;
    while i < count {
        *out = *in_0;
        i += 1;
        in_0 = in_0.offset(1isize);
        out = out.offset(1isize)
    };
}
/*
=================
CMod_LoadLeafBrushes
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadLeafBrushes(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut in_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut count: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut libc::c_int;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<libc::c_int>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_int>()
                                             as libc::c_ulong) as libc::c_int;
    cm.leafbrushes =
        Hunk_AllocDebug(((count + 1i32) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"(count + BOX_BRUSHES) * sizeof( *cm.leafbrushes )\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 365i32)
            as *mut libc::c_int;
    cm.numLeafBrushes = count;
    out = cm.leafbrushes;
    i = 0i32;
    while i < count {
        *out = *in_0;
        i += 1;
        in_0 = in_0.offset(1isize);
        out = out.offset(1isize)
    };
}
/*
=================
CMod_LoadLeafs
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadLeafs(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut cLeaf_t = 0 as *mut cLeaf_t;
    let mut in_0: *mut dleaf_t = 0 as *mut dleaf_t;
    let mut count: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut dleaf_t;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dleaf_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MOD_LoadBmodel: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dleaf_t>() as
                                             libc::c_ulong) as libc::c_int;
    if count < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Map with no leafs\x00" as *const u8 as
                      *const libc::c_char);
    }
    cm.leafs =
        Hunk_AllocDebug(((2i32 + count) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cLeaf_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"( BOX_LEAFS + count ) * sizeof( *cm.leafs )\x00" as
                            *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 284i32)
            as *mut cLeaf_t;
    cm.numLeafs = count;
    out = cm.leafs;
    i = 0i32;
    while i < count {
        (*out).cluster = (*in_0).cluster;
        (*out).area = (*in_0).area;
        (*out).firstLeafBrush = (*in_0).firstLeafBrush;
        (*out).numLeafBrushes = (*in_0).numLeafBrushes;
        (*out).firstLeafSurface = (*in_0).firstLeafSurface;
        (*out).numLeafSurfaces = (*in_0).numLeafSurfaces;
        if (*out).cluster >= cm.numClusters {
            cm.numClusters = (*out).cluster + 1i32
        }
        if (*out).area >= cm.numAreas { cm.numAreas = (*out).area + 1i32 }
        i += 1;
        in_0 = in_0.offset(1isize);
        out = out.offset(1isize)
    }
    cm.areas =
        Hunk_AllocDebug((cm.numAreas as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<cArea_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"cm.numAreas * sizeof( *cm.areas )\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 303i32)
            as *mut cArea_t;
    cm.areaPortals =
        Hunk_AllocDebug(((cm.numAreas * cm.numAreas) as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"cm.numAreas * cm.numAreas * sizeof( *cm.areaPortals )\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 304i32)
            as *mut libc::c_int;
}
/*
===============================================================================

					MAP LOADING

===============================================================================
*/
/*
=================
CMod_LoadShaders
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadShaders(mut l: *mut lump_t) {
    let mut in_0: *mut dshader_t = 0 as *mut dshader_t;
    let mut out: *mut dshader_t = 0 as *mut dshader_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 =
        cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as
            *mut dshader_t;
    if 0 !=
           ((*l).filelen as
                libc::c_ulong).wrapping_rem(::std::mem::size_of::<dshader_t>()
                                                as libc::c_ulong) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CMod_LoadShaders: funny lump size\x00" as *const u8 as
                      *const libc::c_char);
    }
    count =
        ((*l).filelen as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<dshader_t>() as
                                             libc::c_ulong) as libc::c_int;
    if count < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Map with no shaders\x00" as *const u8 as
                      *const libc::c_char);
    }
    cm.shaders =
        Hunk_AllocDebug((count as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<dshader_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"count * sizeof( *cm.shaders )\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_load.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 103i32)
            as *mut dshader_t;
    cm.numShaders = count;
    memcpy(cm.shaders as *mut libc::c_void, in_0 as *const libc::c_void,
           (count as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<dshader_t>()
                                                as libc::c_ulong));
    out = cm.shaders;
    i = 0i32;
    while i < count {
        (*out).contentFlags = (*out).contentFlags;
        (*out).surfaceFlags = (*out).surfaceFlags;
        i += 1;
        in_0 = in_0.offset(1isize);
        out = out.offset(1isize)
    };
}
#[no_mangle]
pub static mut cm_playerCurveClip: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cm_noCurves: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cm_noAreas: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn CM_ClearMap() {
    memset(&mut cm as *mut clipMap_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<clipMap_t>() as libc::c_ulong);
    CM_ClearLevelPatches();
}
// 0 = world, 1 + are bmodels
#[no_mangle]
pub unsafe extern "C" fn CM_InlineModel(mut index: libc::c_int)
 -> clipHandle_t {
    if index < 0i32 || index >= cm.numSubModels {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_InlineModel: bad number\x00" as *const u8 as
                      *const libc::c_char);
    }
    return index;
}
#[no_mangle]
pub unsafe extern "C" fn CM_TempBoxModel(mut mins: *const vec_t,
                                         mut maxs: *const vec_t,
                                         mut capsule: libc::c_int)
 -> clipHandle_t {
    box_model.mins[0usize] = *mins.offset(0isize);
    box_model.mins[1usize] = *mins.offset(1isize);
    box_model.mins[2usize] = *mins.offset(2isize);
    box_model.maxs[0usize] = *maxs.offset(0isize);
    box_model.maxs[1usize] = *maxs.offset(1isize);
    box_model.maxs[2usize] = *maxs.offset(2isize);
    if 0 != capsule { return 254i32 }
    (*box_planes.offset(0isize)).dist = *maxs.offset(0isize);
    (*box_planes.offset(1isize)).dist = -*maxs.offset(0isize);
    (*box_planes.offset(2isize)).dist = *mins.offset(0isize);
    (*box_planes.offset(3isize)).dist = -*mins.offset(0isize);
    (*box_planes.offset(4isize)).dist = *maxs.offset(1isize);
    (*box_planes.offset(5isize)).dist = -*maxs.offset(1isize);
    (*box_planes.offset(6isize)).dist = *mins.offset(1isize);
    (*box_planes.offset(7isize)).dist = -*mins.offset(1isize);
    (*box_planes.offset(8isize)).dist = *maxs.offset(2isize);
    (*box_planes.offset(9isize)).dist = -*maxs.offset(2isize);
    (*box_planes.offset(10isize)).dist = *mins.offset(2isize);
    (*box_planes.offset(11isize)).dist = -*mins.offset(2isize);
    (*box_brush).bounds[0usize][0usize] = *mins.offset(0isize);
    (*box_brush).bounds[0usize][1usize] = *mins.offset(1isize);
    (*box_brush).bounds[0usize][2usize] = *mins.offset(2isize);
    (*box_brush).bounds[1usize][0usize] = *maxs.offset(0isize);
    (*box_brush).bounds[1usize][1usize] = *maxs.offset(1isize);
    (*box_brush).bounds[1usize][2usize] = *maxs.offset(2isize);
    return 255i32;
}
#[no_mangle]
pub unsafe extern "C" fn CM_ModelBounds(mut model: clipHandle_t,
                                        mut mins: *mut vec_t,
                                        mut maxs: *mut vec_t) {
    let mut cmod: *mut cmodel_t = 0 as *mut cmodel_t;
    cmod = CM_ClipHandleToModel(model);
    *mins.offset(0isize) = (*cmod).mins[0usize];
    *mins.offset(1isize) = (*cmod).mins[1usize];
    *mins.offset(2isize) = (*cmod).mins[2usize];
    *maxs.offset(0isize) = (*cmod).maxs[0usize];
    *maxs.offset(1isize) = (*cmod).maxs[1usize];
    *maxs.offset(2isize) = (*cmod).maxs[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn CM_ClipHandleToModel(mut handle: clipHandle_t)
 -> *mut cmodel_t {
    if handle < 0i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_ClipHandleToModel: bad handle %i\x00" as *const u8 as
                      *const libc::c_char, handle);
    }
    if handle < cm.numSubModels {
        return &mut *cm.cmodels.offset(handle as isize) as *mut cmodel_t
    }
    if handle == 255i32 { return &mut box_model }
    if handle < 256i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_ClipHandleToModel: bad handle %i < %i < %i\x00" as
                      *const u8 as *const libc::c_char, cm.numSubModels,
                  handle, 256i32);
    }
    Com_Error(ERR_DROP as libc::c_int,
              b"CM_ClipHandleToModel: bad handle %i\x00" as *const u8 as
                  *const libc::c_char, handle + 256i32);
}
#[no_mangle]
pub unsafe extern "C" fn CM_NumClusters() -> libc::c_int {
    return cm.numClusters;
}
#[no_mangle]
pub unsafe extern "C" fn CM_NumInlineModels() -> libc::c_int {
    return cm.numSubModels;
}
#[no_mangle]
pub unsafe extern "C" fn CM_EntityString() -> *mut libc::c_char {
    return cm.entityString;
}
#[no_mangle]
pub unsafe extern "C" fn CM_LeafCluster(mut leafnum: libc::c_int)
 -> libc::c_int {
    if leafnum < 0i32 || leafnum >= cm.numLeafs {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_LeafCluster: bad number\x00" as *const u8 as
                      *const libc::c_char);
    }
    return (*cm.leafs.offset(leafnum as isize)).cluster;
}
#[no_mangle]
pub unsafe extern "C" fn CM_LeafArea(mut leafnum: libc::c_int)
 -> libc::c_int {
    if leafnum < 0i32 || leafnum >= cm.numLeafs {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_LeafArea: bad number\x00" as *const u8 as
                      *const libc::c_char);
    }
    return (*cm.leafs.offset(leafnum as isize)).area;
}
#[no_mangle]
pub static mut c_pointcontents: libc::c_int = 0;
#[no_mangle]
pub static mut c_traces: libc::c_int = 0;
#[no_mangle]
pub static mut c_brush_traces: libc::c_int = 0;
#[no_mangle]
pub static mut c_patch_traces: libc::c_int = 0;
//==================================================================
#[no_mangle]
pub unsafe extern "C" fn CM_LumpChecksum(mut lump: *mut lump_t)
 -> libc::c_uint {
    return Com_BlockChecksum(cmod_base.offset((*lump).fileofs as isize) as
                                 *const libc::c_void, (*lump).filelen);
}
#[no_mangle]
pub unsafe extern "C" fn CM_Checksum(mut header: *mut dheader_t)
 -> libc::c_uint {
    let mut checksums: [libc::c_uint; 16] = [0; 16];
    checksums[0usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(1isize));
    checksums[1usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(4isize));
    checksums[2usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(6isize));
    checksums[3usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(5isize));
    checksums[4usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(2isize));
    checksums[5usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(9isize));
    checksums[6usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(8isize));
    checksums[7usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(7isize));
    checksums[8usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(3isize));
    checksums[9usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(13isize));
    checksums[10usize] =
        CM_LumpChecksum(&mut *(*header).lumps.as_mut_ptr().offset(10isize));
    return Com_BlockChecksum(checksums.as_mut_ptr() as *const libc::c_void,
                             11i32 * 4i32);
}