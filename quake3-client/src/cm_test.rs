use libc;
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
        pub fn BoxOnPlaneSide(emins: *mut vec_t, emaxs: *mut vec_t,
                              plane: *mut cplane_s) -> libc::c_int;
        #[no_mangle]
        pub fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                            right: *mut vec_t, up: *mut vec_t);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qfiles.h"]
pub mod qfiles_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dshader_t {
        pub shader: [libc::c_char; 64],
        pub surfaceFlags: libc::c_int,
        pub contentFlags: libc::c_int,
    }
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_local.h"]
pub mod cm_local_h {
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
    pub type leafList_t = leafList_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct leafList_s {
        pub count: libc::c_int,
        pub maxcount: libc::c_int,
        pub overflowed: qboolean,
        pub list: *mut libc::c_int,
        pub bounds: [vec3_t; 2],
        pub lastLeaf: libc::c_int,
        pub storeLeafs: Option<unsafe extern "C" fn(_: *mut leafList_s,
                                                    _: libc::c_int) -> ()>,
    }
    use super::{libc};
    use super::q_shared_h::{vec3_t, cplane_t, byte, qboolean, vec_t,
                            clipHandle_t, cvar_t};
    use super::qfiles_h::{dshader_t};
    extern "C" {
        pub type patchCollide_s;
        // keep 1/8 unit away to keep the position valid before network snapping
// and to avoid various numeric issues
        #[no_mangle]
        pub static mut cm: clipMap_t;
        #[no_mangle]
        pub static mut c_pointcontents: libc::c_int;
        #[no_mangle]
        pub fn CM_ClipHandleToModel(handle: clipHandle_t) -> *mut cmodel_t;
        #[no_mangle]
        pub static mut cm_noAreas: *mut cvar_t;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::{libc};
    use super::q_shared_h::{vec_t, clipHandle_t, byte, qboolean};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_test.c"]
pub mod cm_test_c {
    use super::{libc};
    use super::q_shared_h::{vec_t};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, clipHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cplane_s, cvar_s,
                       cvar_t, cplane_t, BoxOnPlaneSide, AngleVectors,
                       Com_Error};
use self::qfiles_h::{dshader_t};
use self::cm_local_h::{cbrush_t, cbrushside_t, clipMap_t, cPatch_t, cArea_t,
                       cmodel_t, cmodel_s, cLeaf_t, cNode_t, leafList_t,
                       leafList_s, patchCollide_s, cm, c_pointcontents,
                       CM_ClipHandleToModel, cm_noAreas};
use self::string_h::{memset};
// returns an ORed contents mask
#[no_mangle]
pub unsafe extern "C" fn CM_PointContents(mut p: *const vec_t,
                                          mut model: clipHandle_t)
 -> libc::c_int {
    let mut leafnum: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut brushnum: libc::c_int = 0;
    let mut leaf: *mut cLeaf_t = 0 as *mut cLeaf_t;
    let mut b: *mut cbrush_t = 0 as *mut cbrush_t;
    let mut contents: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut clipm: *mut cmodel_t = 0 as *mut cmodel_t;
    if 0 == cm.numNodes { return 0i32 }
    if 0 != model {
        clipm = CM_ClipHandleToModel(model);
        leaf = &mut (*clipm).leaf
    } else {
        leafnum = CM_PointLeafnum_r(p, 0i32);
        leaf = &mut *cm.leafs.offset(leafnum as isize) as *mut cLeaf_t
    }
    contents = 0i32;
    k = 0i32;
    while k < (*leaf).numLeafBrushes {
        brushnum =
            *cm.leafbrushes.offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *cm.brushes.offset(brushnum as isize) as *mut cbrush_t;
        if !(0 ==
                 CM_BoundsIntersectPoint((*b).bounds[0usize].as_mut_ptr() as
                                             *const vec_t,
                                         (*b).bounds[1usize].as_mut_ptr() as
                                             *const vec_t, p) as u64) {
            i = 0i32;
            while i < (*b).numsides {
                d =
                    *p.offset(0isize) *
                        (*(*(*b).sides.offset(i as
                                                  isize)).plane).normal[0usize]
                        +
                        *p.offset(1isize) *
                            (*(*(*b).sides.offset(i as
                                                      isize)).plane).normal[1usize]
                        +
                        *p.offset(2isize) *
                            (*(*(*b).sides.offset(i as
                                                      isize)).plane).normal[2usize];
                // FIXME test for Cash
//			if ( d >= b->sides[i].plane->dist ) {
                if d > (*(*(*b).sides.offset(i as isize)).plane).dist {
                    break ;
                }
                i += 1
            }
            if i == (*b).numsides { contents |= (*b).contents }
        }
        k += 1
    }
    return contents;
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoundsIntersectPoint(mut mins: *const vec_t,
                                                 mut maxs: *const vec_t,
                                                 mut point: *const vec_t)
 -> qboolean {
    if (*maxs.offset(0isize) as libc::c_double) <
           *point.offset(0isize) as libc::c_double - 0.125f64 ||
           (*maxs.offset(1isize) as libc::c_double) <
               *point.offset(1isize) as libc::c_double - 0.125f64 ||
           (*maxs.offset(2isize) as libc::c_double) <
               *point.offset(2isize) as libc::c_double - 0.125f64 ||
           *mins.offset(0isize) as libc::c_double >
               *point.offset(0isize) as libc::c_double + 0.125f64 ||
           *mins.offset(1isize) as libc::c_double >
               *point.offset(1isize) as libc::c_double + 0.125f64 ||
           *mins.offset(2isize) as libc::c_double >
               *point.offset(2isize) as libc::c_double + 0.125f64 {
        return qfalse
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
/*
==================
CM_PointLeafnum_r

==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_PointLeafnum_r(mut p: *const vec_t,
                                           mut num: libc::c_int)
 -> libc::c_int {
    let mut d: libc::c_float = 0.;
    let mut node: *mut cNode_t = 0 as *mut cNode_t;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    while num >= 0i32 {
        node = cm.nodes.offset(num as isize);
        plane = (*node).plane;
        if ((*plane).type_0 as libc::c_int) < 3i32 {
            d = *p.offset((*plane).type_0 as isize) - (*plane).dist
        } else {
            d =
                (*plane).normal[0usize] * *p.offset(0isize) +
                    (*plane).normal[1usize] * *p.offset(1isize) +
                    (*plane).normal[2usize] * *p.offset(2isize) -
                    (*plane).dist
        }
        if d < 0i32 as libc::c_float {
            num = (*node).children[1usize]
        } else { num = (*node).children[0usize] }
    }
    c_pointcontents += 1;
    return -1i32 - num;
}
#[no_mangle]
pub unsafe extern "C" fn CM_TransformedPointContents(mut p: *const vec_t,
                                                     mut model: clipHandle_t,
                                                     mut origin: *const vec_t,
                                                     mut angles: *const vec_t)
 -> libc::c_int {
    let mut p_l: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    p_l[0usize] = *p.offset(0isize) - *origin.offset(0isize);
    p_l[1usize] = *p.offset(1isize) - *origin.offset(1isize);
    p_l[2usize] = *p.offset(2isize) - *origin.offset(2isize);
    if model != 255i32 &&
           (0. != *angles.offset(0isize) || 0. != *angles.offset(1isize) ||
                0. != *angles.offset(2isize)) {
        AngleVectors(angles, forward.as_mut_ptr(), right.as_mut_ptr(),
                     up.as_mut_ptr());
        temp[0usize] = p_l[0usize];
        temp[1usize] = p_l[1usize];
        temp[2usize] = p_l[2usize];
        p_l[0usize] =
            temp[0usize] * forward[0usize] + temp[1usize] * forward[1usize] +
                temp[2usize] * forward[2usize];
        p_l[1usize] =
            -(temp[0usize] * right[0usize] + temp[1usize] * right[1usize] +
                  temp[2usize] * right[2usize]);
        p_l[2usize] =
            temp[0usize] * up[0usize] + temp[1usize] * up[1usize] +
                temp[2usize] * up[2usize]
    }
    return CM_PointContents(p_l.as_mut_ptr() as *const vec_t, model);
}
#[no_mangle]
pub unsafe extern "C" fn CM_ClusterPVS(mut cluster: libc::c_int)
 -> *mut byte {
    if cluster < 0i32 || cluster >= cm.numClusters || 0 == cm.vised as u64 {
        return cm.visibility
    }
    return cm.visibility.offset((cluster * cm.clusterBytes) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CM_PointLeafnum(mut p: *const vec_t) -> libc::c_int {
    if 0 == cm.numNodes { return 0i32 }
    return CM_PointLeafnum_r(p, 0i32);
}
// only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
#[no_mangle]
pub unsafe extern "C" fn CM_BoxLeafnums(mut mins: *const vec_t,
                                        mut maxs: *const vec_t,
                                        mut list: *mut libc::c_int,
                                        mut listsize: libc::c_int,
                                        mut lastLeaf: *mut libc::c_int)
 -> libc::c_int {
    let mut ll: leafList_t =
        leafList_s{count: 0,
                   maxcount: 0,
                   overflowed: qfalse,
                   list: 0 as *mut libc::c_int,
                   bounds: [[0.; 3]; 2],
                   lastLeaf: 0,
                   storeLeafs: None,};
    cm.checkcount += 1;
    ll.bounds[0usize][0usize] = *mins.offset(0isize);
    ll.bounds[0usize][1usize] = *mins.offset(1isize);
    ll.bounds[0usize][2usize] = *mins.offset(2isize);
    ll.bounds[1usize][0usize] = *maxs.offset(0isize);
    ll.bounds[1usize][1usize] = *maxs.offset(1isize);
    ll.bounds[1usize][2usize] = *maxs.offset(2isize);
    ll.count = 0i32;
    ll.maxcount = listsize;
    ll.list = list;
    ll.storeLeafs = Some(CM_StoreLeafs);
    ll.lastLeaf = 0i32;
    ll.overflowed = qfalse;
    CM_BoxLeafnums_r(&mut ll, 0i32);
    *lastLeaf = ll.lastLeaf;
    return ll.count;
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoxLeafnums_r(mut ll: *mut leafList_t,
                                          mut nodenum: libc::c_int) {
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut node: *mut cNode_t = 0 as *mut cNode_t;
    let mut s: libc::c_int = 0;
    loop  {
        if nodenum < 0i32 {
            (*ll).storeLeafs.expect("non-null function pointer")(ll, nodenum);
            return
        }
        node = &mut *cm.nodes.offset(nodenum as isize) as *mut cNode_t;
        plane = (*node).plane;
        s =
            BoxOnPlaneSide((*ll).bounds[0usize].as_mut_ptr(),
                           (*ll).bounds[1usize].as_mut_ptr(), plane);
        if s == 1i32 {
            nodenum = (*node).children[0usize]
        } else if s == 2i32 {
            nodenum = (*node).children[1usize]
        } else {
            CM_BoxLeafnums_r(ll, (*node).children[0usize]);
            nodenum = (*node).children[1usize]
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CM_StoreLeafs(mut ll: *mut leafList_t,
                                       mut nodenum: libc::c_int) {
    let mut leafNum: libc::c_int = 0;
    leafNum = -1i32 - nodenum;
    if (*cm.leafs.offset(leafNum as isize)).cluster != -1i32 {
        (*ll).lastLeaf = leafNum
    }
    if (*ll).count >= (*ll).maxcount { (*ll).overflowed = qtrue; return }
    let fresh0 = (*ll).count;
    (*ll).count = (*ll).count + 1;
    *(*ll).list.offset(fresh0 as isize) = leafNum;
}
#[no_mangle]
pub unsafe extern "C" fn CM_AdjustAreaPortalState(mut area1: libc::c_int,
                                                  mut area2: libc::c_int,
                                                  mut open: qboolean) {
    if area1 < 0i32 || area2 < 0i32 { return }
    if area1 >= cm.numAreas || area2 >= cm.numAreas {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_ChangeAreaPortalState: bad area number\x00" as
                      *const u8 as *const libc::c_char);
    }
    if 0 != open as u64 {
        let ref mut fresh1 =
            *cm.areaPortals.offset((area1 * cm.numAreas + area2) as isize);
        *fresh1 += 1;
        let ref mut fresh2 =
            *cm.areaPortals.offset((area2 * cm.numAreas + area1) as isize);
        *fresh2 += 1
    } else {
        let ref mut fresh3 =
            *cm.areaPortals.offset((area1 * cm.numAreas + area2) as isize);
        *fresh3 -= 1;
        let ref mut fresh4 =
            *cm.areaPortals.offset((area2 * cm.numAreas + area1) as isize);
        *fresh4 -= 1;
        if *cm.areaPortals.offset((area2 * cm.numAreas + area1) as isize) <
               0i32 {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CM_AdjustAreaPortalState: negative reference count\x00"
                          as *const u8 as *const libc::c_char);
        }
    }
    CM_FloodAreaConnections();
}
/*
====================
CM_FloodAreaConnections

====================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_FloodAreaConnections() {
    let mut i: libc::c_int = 0;
    let mut area: *mut cArea_t = 0 as *mut cArea_t;
    let mut floodnum: libc::c_int = 0;
    cm.floodvalid += 1;
    floodnum = 0i32;
    i = 0i32;
    while i < cm.numAreas {
        area = &mut *cm.areas.offset(i as isize) as *mut cArea_t;
        if !((*area).floodvalid == cm.floodvalid) {
            // already flooded into
            floodnum += 1;
            CM_FloodArea_r(i, floodnum);
        }
        i += 1
    };
}
/*
===============================================================================

AREAPORTALS

===============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_FloodArea_r(mut areaNum: libc::c_int,
                                        mut floodnum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut area: *mut cArea_t = 0 as *mut cArea_t;
    let mut con: *mut libc::c_int = 0 as *mut libc::c_int;
    area = &mut *cm.areas.offset(areaNum as isize) as *mut cArea_t;
    if (*area).floodvalid == cm.floodvalid {
        if (*area).floodnum == floodnum { return }
        Com_Error(ERR_DROP as libc::c_int,
                  b"FloodArea_r: reflooded\x00" as *const u8 as
                      *const libc::c_char);
    }
    (*area).floodnum = floodnum;
    (*area).floodvalid = cm.floodvalid;
    con = cm.areaPortals.offset((areaNum * cm.numAreas) as isize);
    i = 0i32;
    while i < cm.numAreas {
        if *con.offset(i as isize) > 0i32 { CM_FloodArea_r(i, floodnum); }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CM_AreasConnected(mut area1: libc::c_int,
                                           mut area2: libc::c_int)
 -> qboolean {
    if 0 != (*cm_noAreas).integer { return qtrue }
    if area1 < 0i32 || area2 < 0i32 { return qfalse }
    if area1 >= cm.numAreas || area2 >= cm.numAreas {
        Com_Error(ERR_DROP as libc::c_int,
                  b"area >= cm.numAreas\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*cm.areas.offset(area1 as isize)).floodnum ==
           (*cm.areas.offset(area2 as isize)).floodnum {
        return qtrue
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn CM_WriteAreaBits(mut buffer: *mut byte,
                                          mut area: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut floodnum: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    bytes = cm.numAreas + 7i32 >> 3i32;
    if 0 != (*cm_noAreas).integer || area == -1i32 {
        memset(buffer as *mut libc::c_void, 255i32, bytes as libc::c_ulong);
    } else {
        floodnum = (*cm.areas.offset(area as isize)).floodnum;
        i = 0i32;
        while i < cm.numAreas {
            if (*cm.areas.offset(i as isize)).floodnum == floodnum ||
                   area == -1i32 {
                let ref mut fresh5 = *buffer.offset((i >> 3i32) as isize);
                *fresh5 =
                    (*fresh5 as libc::c_int | 1i32 << (i & 7i32)) as byte
            }
            i += 1
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoxBrushes(mut mins: *const vec_t,
                                       mut maxs: *const vec_t,
                                       mut list: *mut *mut cbrush_t,
                                       mut listsize: libc::c_int)
 -> libc::c_int {
    let mut ll: leafList_t =
        leafList_s{count: 0,
                   maxcount: 0,
                   overflowed: qfalse,
                   list: 0 as *mut libc::c_int,
                   bounds: [[0.; 3]; 2],
                   lastLeaf: 0,
                   storeLeafs: None,};
    cm.checkcount += 1;
    ll.bounds[0usize][0usize] = *mins.offset(0isize);
    ll.bounds[0usize][1usize] = *mins.offset(1isize);
    ll.bounds[0usize][2usize] = *mins.offset(2isize);
    ll.bounds[1usize][0usize] = *maxs.offset(0isize);
    ll.bounds[1usize][1usize] = *maxs.offset(1isize);
    ll.bounds[1usize][2usize] = *maxs.offset(2isize);
    ll.count = 0i32;
    ll.maxcount = listsize;
    ll.list = list as *mut libc::c_void as *mut libc::c_int;
    ll.storeLeafs = Some(CM_StoreBrushes);
    ll.lastLeaf = 0i32;
    ll.overflowed = qfalse;
    CM_BoxLeafnums_r(&mut ll, 0i32);
    return ll.count;
}
#[no_mangle]
pub unsafe extern "C" fn CM_StoreBrushes(mut ll: *mut leafList_t,
                                         mut nodenum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut leafnum: libc::c_int = 0;
    let mut brushnum: libc::c_int = 0;
    let mut leaf: *mut cLeaf_t = 0 as *mut cLeaf_t;
    let mut b: *mut cbrush_t = 0 as *mut cbrush_t;
    leafnum = -1i32 - nodenum;
    leaf = &mut *cm.leafs.offset(leafnum as isize) as *mut cLeaf_t;
    k = 0i32;
    while k < (*leaf).numLeafBrushes {
        brushnum =
            *cm.leafbrushes.offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *cm.brushes.offset(brushnum as isize) as *mut cbrush_t;
        if !((*b).checkcount == cm.checkcount) {
            // already checked this brush in another leaf
            (*b).checkcount = cm.checkcount;
            i = 0i32;
            while i < 3i32 {
                if (*b).bounds[0usize][i as usize] >=
                       (*ll).bounds[1usize][i as usize] ||
                       (*b).bounds[1usize][i as usize] <=
                           (*ll).bounds[0usize][i as usize] {
                    break ;
                }
                i += 1
            }
            if !(i != 3i32) {
                if (*ll).count >= (*ll).maxcount {
                    (*ll).overflowed = qtrue;
                    return
                }
                let fresh6 = (*ll).count;
                (*ll).count = (*ll).count + 1;
                let ref mut fresh7 =
                    *((*ll).list as
                          *mut *mut cbrush_t).offset(fresh6 as isize);
                *fresh7 = b
            }
        }
        k += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoundsIntersect(mut mins: *const vec_t,
                                            mut maxs: *const vec_t,
                                            mut mins2: *const vec_t,
                                            mut maxs2: *const vec_t)
 -> qboolean {
    if (*maxs.offset(0isize) as libc::c_double) <
           *mins2.offset(0isize) as libc::c_double - 0.125f64 ||
           (*maxs.offset(1isize) as libc::c_double) <
               *mins2.offset(1isize) as libc::c_double - 0.125f64 ||
           (*maxs.offset(2isize) as libc::c_double) <
               *mins2.offset(2isize) as libc::c_double - 0.125f64 ||
           *mins.offset(0isize) as libc::c_double >
               *maxs2.offset(0isize) as libc::c_double + 0.125f64 ||
           *mins.offset(1isize) as libc::c_double >
               *maxs2.offset(1isize) as libc::c_double + 0.125f64 ||
           *mins.offset(2isize) as libc::c_double >
               *maxs2.offset(2isize) as libc::c_double + 0.125f64 {
        return qfalse
    }
    return qtrue;
}