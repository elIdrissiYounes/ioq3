#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           extern_types,
           label_break_value,
           libc)]
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
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
    pub type clipHandle_t = libc::c_int;
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
    // a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct trace_t {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub surfaceFlags: libc::c_int,
        pub contents: libc::c_int,
        pub entityNum: libc::c_int,
    }
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut vec3_origin: vec3_t;
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
        #[no_mangle]
        pub fn AngleVectors(angles: *const vec_t, forward: *mut vec_t,
                            right: *mut vec_t, up: *mut vec_t);
    }
}
#[header_src =
      "ioq3/code/qcommon/qfiles.h"]
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
#[header_src =
      "ioq3/code/qcommon/cm_local.h"]
pub mod cm_local_h {
    // cm_test.c
    // Used for oriented capsule collision detection
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sphere_t {
        pub use_0: qboolean,
        pub radius: libc::c_float,
        pub halfheight: libc::c_float,
        pub offset: vec3_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct traceWork_t {
        pub start: vec3_t,
        pub end: vec3_t,
        pub size: [vec3_t; 2],
        pub offsets: [vec3_t; 8],
        pub maxOffset: libc::c_float,
        pub extents: vec3_t,
        pub bounds: [vec3_t; 2],
        pub modelOrigin: vec3_t,
        pub contents: libc::c_int,
        pub isPoint: qboolean,
        pub trace: trace_t,
        pub sphere: sphere_t,
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
    use super::q_shared_h::{qboolean, vec3_t, trace_t, cplane_t, byte, cvar_t,
                            vec_t, clipHandle_t};
    use super::{libc};
    use super::qfiles_h::{dshader_t};
    extern "C" {
        pub type patchCollide_s;
        // keep 1/8 unit away to keep the position valid before network snapping
// and to avoid various numeric issues
        #[no_mangle]
        pub static mut cm: clipMap_t;
        #[no_mangle]
        pub fn CM_TraceThroughPatchCollide(tw: *mut traceWork_t,
                                           pc: *const patchCollide_s);
        #[no_mangle]
        pub static mut c_patch_traces: libc::c_int;
        #[no_mangle]
        pub static mut cm_noCurves: *mut cvar_t;
        #[no_mangle]
        pub static mut c_brush_traces: libc::c_int;
        #[no_mangle]
        pub fn CM_BoundsIntersect(mins: *const vec_t, maxs: *const vec_t,
                                  mins2: *const vec_t, maxs2: *const vec_t)
         -> qboolean;
        #[no_mangle]
        pub fn CM_ClipHandleToModel(handle: clipHandle_t) -> *mut cmodel_t;
        #[no_mangle]
        pub fn CM_PositionTestInPatchCollide(tw: *mut traceWork_t,
                                             pc: *const patchCollide_s)
         -> qboolean;
        #[no_mangle]
        pub fn CM_BoxLeafnums_r(ll: *mut leafList_t, nodenum: libc::c_int);
        #[no_mangle]
        pub fn CM_StoreLeafs(ll: *mut leafList_t, nodenum: libc::c_int);
        #[no_mangle]
        pub static mut c_traces: libc::c_int;
    }
}
#[header_src = "/usr/include/assert.h"]
pub mod assert_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
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
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::q_shared_h::{clipHandle_t, vec_t, trace_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn CM_TempBoxModel(mins: *const vec_t, maxs: *const vec_t,
                               capsule: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn CM_ModelBounds(model: clipHandle_t, mins: *mut vec_t,
                              maxs: *mut vec_t);
    }
}
#[header_src =
      "ioq3/code/qcommon/cm_trace.c"]
pub mod cm_trace_c {
    use super::q_shared_h::{trace_t, vec_t, clipHandle_t, vec3_t};
    use super::{libc};
    use super::cm_local_h::{sphere_t, traceWork_t, cLeaf_t, cPatch_t,
                            cbrush_t};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t,
                       clipHandle_t, vec_t, vec3_t, cplane_s, cvar_s, cvar_t,
                       cplane_t, trace_t, vec3_origin, VectorNormalize,
                       AngleVectors};
use self::qfiles_h::{dshader_t};
use self::cm_local_h::{sphere_t, traceWork_t, cNode_t, clipMap_t, cPatch_t,
                       cArea_t, cbrush_t, cbrushside_t, cmodel_t, cmodel_s,
                       cLeaf_t, leafList_t, leafList_s, patchCollide_s, cm,
                       CM_TraceThroughPatchCollide, c_patch_traces,
                       cm_noCurves, c_brush_traces, CM_BoundsIntersect,
                       CM_ClipHandleToModel, CM_PositionTestInPatchCollide,
                       CM_BoxLeafnums_r, CM_StoreLeafs, c_traces};
use self::assert_h::{__assert_fail};
use self::mathcalls_h::{fabs};
use self::string_h::{memset};
use self::cm_public_h::{CM_TempBoxModel, CM_ModelBounds};
unsafe extern "C" fn VectorLengthSquared(mut v: *const vec_t) -> vec_t {
    return *v.offset(0isize) * *v.offset(0isize) +
               *v.offset(1isize) * *v.offset(1isize) +
               *v.offset(2isize) * *v.offset(2isize);
}
unsafe extern "C" fn VectorInverse(mut v: *mut vec_t) {
    *v.offset(0isize) = -*v.offset(0isize);
    *v.offset(1isize) = -*v.offset(1isize);
    *v.offset(2isize) = -*v.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoxTrace(mut results: *mut trace_t,
                                     mut start: *const vec_t,
                                     mut end: *const vec_t,
                                     mut mins: *mut vec_t,
                                     mut maxs: *mut vec_t,
                                     mut model: clipHandle_t,
                                     mut brushmask: libc::c_int,
                                     mut capsule: libc::c_int) {
    CM_Trace(results, start, end, mins, maxs, model,
             vec3_origin.as_mut_ptr() as *const vec_t, brushmask, capsule,
             0 as *mut sphere_t);
}
//======================================================================
/*
==================
CM_Trace
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_Trace(mut results: *mut trace_t,
                                  mut start: *const vec_t,
                                  mut end: *const vec_t, mut mins: *mut vec_t,
                                  mut maxs: *mut vec_t,
                                  mut model: clipHandle_t,
                                  mut origin: *const vec_t,
                                  mut brushmask: libc::c_int,
                                  mut capsule: libc::c_int,
                                  mut sphere: *mut sphere_t) {
    let mut i: libc::c_int = 0;
    let mut tw: traceWork_t =
        traceWork_t{start: [0.; 3],
                    end: [0.; 3],
                    size: [[0.; 3]; 2],
                    offsets: [[0.; 3]; 8],
                    maxOffset: 0.,
                    extents: [0.; 3],
                    bounds: [[0.; 3]; 2],
                    modelOrigin: [0.; 3],
                    contents: 0,
                    isPoint: qfalse,
                    trace:
                        trace_t{allsolid: qfalse,
                                startsolid: qfalse,
                                fraction: 0.,
                                endpos: [0.; 3],
                                plane:
                                    cplane_s{normal: [0.; 3],
                                             dist: 0.,
                                             type_0: 0,
                                             signbits: 0,
                                             pad: [0; 2],},
                                surfaceFlags: 0,
                                contents: 0,
                                entityNum: 0,},
                    sphere:
                        sphere_t{use_0: qfalse,
                                 radius: 0.,
                                 halfheight: 0.,
                                 offset: [0.; 3],},};
    let mut offset: vec3_t = [0.; 3];
    let mut cmod: *mut cmodel_t = 0 as *mut cmodel_t;
    cmod = CM_ClipHandleToModel(model);
    cm.checkcount += 1;
    c_traces += 1;
    memset(&mut tw as *mut traceWork_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<traceWork_t>() as libc::c_ulong);
    tw.trace.fraction = 1i32 as libc::c_float;
    tw.modelOrigin[0usize] = *origin.offset(0isize);
    tw.modelOrigin[1usize] = *origin.offset(1isize);
    tw.modelOrigin[2usize] = *origin.offset(2isize);
    if 0 == cm.numNodes { *results = tw.trace; return }
    if mins.is_null() { mins = vec3_origin.as_mut_ptr() }
    if maxs.is_null() { maxs = vec3_origin.as_mut_ptr() }
    tw.contents = brushmask;
    i = 0i32;
    while i < 3i32 {
        offset[i as usize] =
            ((*mins.offset(i as isize) + *maxs.offset(i as isize)) as
                 libc::c_double * 0.5f64) as vec_t;
        tw.size[0usize][i as usize] =
            *mins.offset(i as isize) - offset[i as usize];
        tw.size[1usize][i as usize] =
            *maxs.offset(i as isize) - offset[i as usize];
        tw.start[i as usize] = *start.offset(i as isize) + offset[i as usize];
        tw.end[i as usize] = *end.offset(i as isize) + offset[i as usize];
        i += 1
    }
    if !sphere.is_null() {
        tw.sphere = *sphere
    } else {
        tw.sphere.use_0 = capsule as qboolean;
        tw.sphere.radius =
            if tw.size[1usize][0usize] > tw.size[1usize][2usize] {
                tw.size[1usize][2usize]
            } else { tw.size[1usize][0usize] };
        tw.sphere.halfheight = tw.size[1usize][2usize];
        tw.sphere.offset[0usize] = 0i32 as vec_t;
        tw.sphere.offset[1usize] = 0i32 as vec_t;
        tw.sphere.offset[2usize] = tw.size[1usize][2usize] - tw.sphere.radius
    }
    tw.maxOffset =
        tw.size[1usize][0usize] + tw.size[1usize][1usize] +
            tw.size[1usize][2usize];
    tw.offsets[0usize][0usize] = tw.size[0usize][0usize];
    tw.offsets[0usize][1usize] = tw.size[0usize][1usize];
    tw.offsets[0usize][2usize] = tw.size[0usize][2usize];
    tw.offsets[1usize][0usize] = tw.size[1usize][0usize];
    tw.offsets[1usize][1usize] = tw.size[0usize][1usize];
    tw.offsets[1usize][2usize] = tw.size[0usize][2usize];
    tw.offsets[2usize][0usize] = tw.size[0usize][0usize];
    tw.offsets[2usize][1usize] = tw.size[1usize][1usize];
    tw.offsets[2usize][2usize] = tw.size[0usize][2usize];
    tw.offsets[3usize][0usize] = tw.size[1usize][0usize];
    tw.offsets[3usize][1usize] = tw.size[1usize][1usize];
    tw.offsets[3usize][2usize] = tw.size[0usize][2usize];
    tw.offsets[4usize][0usize] = tw.size[0usize][0usize];
    tw.offsets[4usize][1usize] = tw.size[0usize][1usize];
    tw.offsets[4usize][2usize] = tw.size[1usize][2usize];
    tw.offsets[5usize][0usize] = tw.size[1usize][0usize];
    tw.offsets[5usize][1usize] = tw.size[0usize][1usize];
    tw.offsets[5usize][2usize] = tw.size[1usize][2usize];
    tw.offsets[6usize][0usize] = tw.size[0usize][0usize];
    tw.offsets[6usize][1usize] = tw.size[1usize][1usize];
    tw.offsets[6usize][2usize] = tw.size[1usize][2usize];
    tw.offsets[7usize][0usize] = tw.size[1usize][0usize];
    tw.offsets[7usize][1usize] = tw.size[1usize][1usize];
    tw.offsets[7usize][2usize] = tw.size[1usize][2usize];
    if 0 != tw.sphere.use_0 as u64 {
        i = 0i32;
        while i < 3i32 {
            if tw.start[i as usize] < tw.end[i as usize] {
                tw.bounds[0usize][i as usize] =
                    (tw.start[i as usize] as libc::c_double -
                         fabs(tw.sphere.offset[i as usize] as libc::c_double)
                         - tw.sphere.radius as libc::c_double) as vec_t;
                tw.bounds[1usize][i as usize] =
                    (tw.end[i as usize] as libc::c_double +
                         fabs(tw.sphere.offset[i as usize] as libc::c_double)
                         + tw.sphere.radius as libc::c_double) as vec_t
            } else {
                tw.bounds[0usize][i as usize] =
                    (tw.end[i as usize] as libc::c_double -
                         fabs(tw.sphere.offset[i as usize] as libc::c_double)
                         - tw.sphere.radius as libc::c_double) as vec_t;
                tw.bounds[1usize][i as usize] =
                    (tw.start[i as usize] as libc::c_double +
                         fabs(tw.sphere.offset[i as usize] as libc::c_double)
                         + tw.sphere.radius as libc::c_double) as vec_t
            }
            i += 1
        }
    } else {
        i = 0i32;
        while i < 3i32 {
            if tw.start[i as usize] < tw.end[i as usize] {
                tw.bounds[0usize][i as usize] =
                    tw.start[i as usize] + tw.size[0usize][i as usize];
                tw.bounds[1usize][i as usize] =
                    tw.end[i as usize] + tw.size[1usize][i as usize]
            } else {
                tw.bounds[0usize][i as usize] =
                    tw.end[i as usize] + tw.size[0usize][i as usize];
                tw.bounds[1usize][i as usize] =
                    tw.start[i as usize] + tw.size[1usize][i as usize]
            }
            i += 1
        }
    }
    if *start.offset(0isize) == *end.offset(0isize) &&
           *start.offset(1isize) == *end.offset(1isize) &&
           *start.offset(2isize) == *end.offset(2isize) {
        if 0 != model {
            if model == 254i32 {
                if 0 != tw.sphere.use_0 as u64 {
                    CM_TestCapsuleInCapsule(&mut tw, model);
                } else { CM_TestBoundingBoxInCapsule(&mut tw, model); }
            } else { CM_TestInLeaf(&mut tw, &mut (*cmod).leaf); }
        } else { CM_PositionTest(&mut tw); }
    } else {
        if tw.size[0usize][0usize] == 0i32 as libc::c_float &&
               tw.size[0usize][1usize] == 0i32 as libc::c_float &&
               tw.size[0usize][2usize] == 0i32 as libc::c_float {
            tw.isPoint = qtrue;
            tw.extents[2usize] = 0i32 as vec_t;
            tw.extents[1usize] = tw.extents[2usize];
            tw.extents[0usize] = tw.extents[1usize]
        } else {
            tw.isPoint = qfalse;
            tw.extents[0usize] = tw.size[1usize][0usize];
            tw.extents[1usize] = tw.size[1usize][1usize];
            tw.extents[2usize] = tw.size[1usize][2usize]
        }
        if 0 != model {
            if model == 254i32 {
                if 0 != tw.sphere.use_0 as u64 {
                    CM_TraceCapsuleThroughCapsule(&mut tw, model);
                } else { CM_TraceBoundingBoxThroughCapsule(&mut tw, model); }
            } else { CM_TraceThroughLeaf(&mut tw, &mut (*cmod).leaf); }
        } else {
            CM_TraceThroughTree(&mut tw, 0i32, 0i32 as libc::c_float,
                                1i32 as libc::c_float, tw.start.as_mut_ptr(),
                                tw.end.as_mut_ptr());
        }
    }
    if tw.trace.fraction == 1i32 as libc::c_float {
        tw.trace.endpos[0usize] = *end.offset(0isize);
        tw.trace.endpos[1usize] = *end.offset(1isize);
        tw.trace.endpos[2usize] = *end.offset(2isize)
    } else {
        i = 0i32;
        while i < 3i32 {
            tw.trace.endpos[i as usize] =
                *start.offset(i as isize) +
                    tw.trace.fraction *
                        (*end.offset(i as isize) - *start.offset(i as isize));
            i += 1
        }
    }
    if 0 != tw.trace.allsolid as libc::c_uint ||
           tw.trace.fraction as libc::c_double == 1.0f64 ||
           VectorLengthSquared(tw.trace.plane.normal.as_mut_ptr() as
                                   *const vec_t) as libc::c_double > 0.9999f64
       {
    } else {
        __assert_fail(b"tw.trace.allsolid || tw.trace.fraction == 1.0 || VectorLengthSquared(tw.trace.plane.normal) > 0.9999\x00"
                          as *const u8 as *const libc::c_char,
                      b"code/qcommon/cm_trace.c\x00" as *const u8 as
                          *const libc::c_char, 1357i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 124],
                                                &[libc::c_char; 124]>(b"void CM_Trace(trace_t *, const vec_t *, const vec_t *, vec_t *, vec_t *, clipHandle_t, const vec_t *, int, int, sphere_t *)\x00")).as_ptr());
    }
    *results = tw.trace;
}
//=========================================================================================
/*
==================
CM_TraceThroughTree

Traverse all the contacted leafs from the start to the end position.
If the trace is a point, they will be exactly in order, but for larger
trace volumes it is possible to hit something in a later leaf with
a smaller intercept fraction.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceThroughTree(mut tw: *mut traceWork_t,
                                             mut num: libc::c_int,
                                             mut p1f: libc::c_float,
                                             mut p2f: libc::c_float,
                                             mut p1: *mut vec_t,
                                             mut p2: *mut vec_t) {
    let mut node: *mut cNode_t = 0 as *mut cNode_t;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut offset: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut frac2: libc::c_float = 0.;
    let mut idist: libc::c_float = 0.;
    let mut mid: vec3_t = [0.; 3];
    let mut side: libc::c_int = 0;
    let mut midf: libc::c_float = 0.;
    if (*tw).trace.fraction <= p1f { return }
    if num < 0i32 {
        CM_TraceThroughLeaf(tw,
                            &mut *cm.leafs.offset((-1i32 - num) as isize));
        return
    }
    node = cm.nodes.offset(num as isize);
    plane = (*node).plane;
    if ((*plane).type_0 as libc::c_int) < 3i32 {
        t1 = *p1.offset((*plane).type_0 as isize) - (*plane).dist;
        t2 = *p2.offset((*plane).type_0 as isize) - (*plane).dist;
        offset = (*tw).extents[(*plane).type_0 as usize]
    } else {
        t1 =
            (*plane).normal[0usize] * *p1.offset(0isize) +
                (*plane).normal[1usize] * *p1.offset(1isize) +
                (*plane).normal[2usize] * *p1.offset(2isize) - (*plane).dist;
        t2 =
            (*plane).normal[0usize] * *p2.offset(0isize) +
                (*plane).normal[1usize] * *p2.offset(1isize) +
                (*plane).normal[2usize] * *p2.offset(2isize) - (*plane).dist;
        if 0 != (*tw).isPoint as u64 {
            offset = 0i32 as libc::c_float
        } else { offset = 2048i32 as libc::c_float }
    }
    if t1 >= offset + 1i32 as libc::c_float &&
           t2 >= offset + 1i32 as libc::c_float {
        CM_TraceThroughTree(tw, (*node).children[0usize], p1f, p2f, p1, p2);
        return
    }
    if t1 < -offset - 1i32 as libc::c_float &&
           t2 < -offset - 1i32 as libc::c_float {
        CM_TraceThroughTree(tw, (*node).children[1usize], p1f, p2f, p1, p2);
        return
    }
    if t1 < t2 {
        idist = (1.0f64 / (t1 - t2) as libc::c_double) as libc::c_float;
        side = 1i32;
        frac2 =
            (((t1 + offset) as libc::c_double + 0.125f64) *
                 idist as libc::c_double) as libc::c_float;
        frac =
            (((t1 - offset) as libc::c_double + 0.125f64) *
                 idist as libc::c_double) as libc::c_float
    } else if t1 > t2 {
        idist = (1.0f64 / (t1 - t2) as libc::c_double) as libc::c_float;
        side = 0i32;
        frac2 =
            (((t1 - offset) as libc::c_double - 0.125f64) *
                 idist as libc::c_double) as libc::c_float;
        frac =
            (((t1 + offset) as libc::c_double + 0.125f64) *
                 idist as libc::c_double) as libc::c_float
    } else {
        side = 0i32;
        frac = 1i32 as libc::c_float;
        frac2 = 0i32 as libc::c_float
    }
    if frac < 0i32 as libc::c_float { frac = 0i32 as libc::c_float }
    if frac > 1i32 as libc::c_float { frac = 1i32 as libc::c_float }
    midf = p1f + (p2f - p1f) * frac;
    mid[0usize] =
        *p1.offset(0isize) + frac * (*p2.offset(0isize) - *p1.offset(0isize));
    mid[1usize] =
        *p1.offset(1isize) + frac * (*p2.offset(1isize) - *p1.offset(1isize));
    mid[2usize] =
        *p1.offset(2isize) + frac * (*p2.offset(2isize) - *p1.offset(2isize));
    CM_TraceThroughTree(tw, (*node).children[side as usize], p1f, midf, p1,
                        mid.as_mut_ptr());
    if frac2 < 0i32 as libc::c_float { frac2 = 0i32 as libc::c_float }
    if frac2 > 1i32 as libc::c_float { frac2 = 1i32 as libc::c_float }
    midf = p1f + (p2f - p1f) * frac2;
    mid[0usize] =
        *p1.offset(0isize) +
            frac2 * (*p2.offset(0isize) - *p1.offset(0isize));
    mid[1usize] =
        *p1.offset(1isize) +
            frac2 * (*p2.offset(1isize) - *p1.offset(1isize));
    mid[2usize] =
        *p1.offset(2isize) +
            frac2 * (*p2.offset(2isize) - *p1.offset(2isize));
    CM_TraceThroughTree(tw, (*node).children[(side ^ 1i32) as usize], midf,
                        p2f, mid.as_mut_ptr(), p2);
}
/*
================
CM_TraceThroughLeaf
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceThroughLeaf(mut tw: *mut traceWork_t,
                                             mut leaf: *mut cLeaf_t) {
    let mut k: libc::c_int = 0;
    let mut brushnum: libc::c_int = 0;
    let mut b: *mut cbrush_t = 0 as *mut cbrush_t;
    let mut patch: *mut cPatch_t = 0 as *mut cPatch_t;
    k = 0i32;
    while k < (*leaf).numLeafBrushes {
        brushnum =
            *cm.leafbrushes.offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *cm.brushes.offset(brushnum as isize) as *mut cbrush_t;
        if !((*b).checkcount == cm.checkcount) {
            // already checked this brush in another leaf
            (*b).checkcount = cm.checkcount;
            if !(0 == (*b).contents & (*tw).contents) {
                if !(0 ==
                         CM_BoundsIntersect((*tw).bounds[0usize].as_mut_ptr()
                                                as *const vec_t,
                                            (*tw).bounds[1usize].as_mut_ptr()
                                                as *const vec_t,
                                            (*b).bounds[0usize].as_mut_ptr()
                                                as *const vec_t,
                                            (*b).bounds[1usize].as_mut_ptr()
                                                as *const vec_t) as u64) {
                    CM_TraceThroughBrush(tw, b);
                    if 0. == (*tw).trace.fraction { return }
                }
            }
        }
        k += 1
    }
    if 0 == (*cm_noCurves).integer {
        k = 0i32;
        while k < (*leaf).numLeafSurfaces {
            patch =
                *cm.surfaces.offset(*cm.leafsurfaces.offset(((*leaf).firstLeafSurface
                                                                 + k) as
                                                                isize) as
                                        isize);
            if !patch.is_null() {
                if !((*patch).checkcount == cm.checkcount) {
                    // already checked this patch in another leaf
                    (*patch).checkcount = cm.checkcount;
                    if !(0 == (*patch).contents & (*tw).contents) {
                        CM_TraceThroughPatch(tw, patch);
                        if 0. == (*tw).trace.fraction { return }
                    }
                }
            }
            k += 1
        }
    };
}
/*
===============================================================================

TRACING

===============================================================================
*/
/*
================
CM_TraceThroughPatch
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceThroughPatch(mut tw: *mut traceWork_t,
                                              mut patch: *mut cPatch_t) {
    let mut oldFrac: libc::c_float = 0.;
    c_patch_traces += 1;
    oldFrac = (*tw).trace.fraction;
    CM_TraceThroughPatchCollide(tw, (*patch).pc);
    if (*tw).trace.fraction < oldFrac {
        (*tw).trace.surfaceFlags = (*patch).surfaceFlags;
        (*tw).trace.contents = (*patch).contents
    };
}
/*
================
CM_TraceThroughBrush
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceThroughBrush(mut tw: *mut traceWork_t,
                                              mut brush: *mut cbrush_t) {
    let mut i: libc::c_int = 0;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut clipplane: *mut cplane_t = 0 as *mut cplane_t;
    let mut dist: libc::c_float = 0.;
    let mut enterFrac: libc::c_float = 0.;
    let mut leaveFrac: libc::c_float = 0.;
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    let mut getout: qboolean = qfalse;
    let mut startout: qboolean = qfalse;
    let mut f: libc::c_float = 0.;
    let mut side: *mut cbrushside_t = 0 as *mut cbrushside_t;
    let mut leadside: *mut cbrushside_t = 0 as *mut cbrushside_t;
    let mut t: libc::c_float = 0.;
    let mut startp: vec3_t = [0.; 3];
    let mut endp: vec3_t = [0.; 3];
    enterFrac = -1.0f64 as libc::c_float;
    leaveFrac = 1.0f64 as libc::c_float;
    clipplane = 0 as *mut cplane_t;
    if 0 == (*brush).numsides { return }
    c_brush_traces += 1;
    getout = qfalse;
    startout = qfalse;
    leadside = 0 as *mut cbrushside_t;
    if 0 != (*tw).sphere.use_0 as u64 {
        i = 0i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            dist = (*plane).dist + (*tw).sphere.radius;
            t =
                (*plane).normal[0usize] * (*tw).sphere.offset[0usize] +
                    (*plane).normal[1usize] * (*tw).sphere.offset[1usize] +
                    (*plane).normal[2usize] * (*tw).sphere.offset[2usize];
            if t > 0i32 as libc::c_float {
                startp[0usize] =
                    (*tw).start[0usize] - (*tw).sphere.offset[0usize];
                startp[1usize] =
                    (*tw).start[1usize] - (*tw).sphere.offset[1usize];
                startp[2usize] =
                    (*tw).start[2usize] - (*tw).sphere.offset[2usize];
                endp[0usize] =
                    (*tw).end[0usize] - (*tw).sphere.offset[0usize];
                endp[1usize] =
                    (*tw).end[1usize] - (*tw).sphere.offset[1usize];
                endp[2usize] = (*tw).end[2usize] - (*tw).sphere.offset[2usize]
            } else {
                startp[0usize] =
                    (*tw).start[0usize] + (*tw).sphere.offset[0usize];
                startp[1usize] =
                    (*tw).start[1usize] + (*tw).sphere.offset[1usize];
                startp[2usize] =
                    (*tw).start[2usize] + (*tw).sphere.offset[2usize];
                endp[0usize] =
                    (*tw).end[0usize] + (*tw).sphere.offset[0usize];
                endp[1usize] =
                    (*tw).end[1usize] + (*tw).sphere.offset[1usize];
                endp[2usize] = (*tw).end[2usize] + (*tw).sphere.offset[2usize]
            }
            d1 =
                startp[0usize] * (*plane).normal[0usize] +
                    startp[1usize] * (*plane).normal[1usize] +
                    startp[2usize] * (*plane).normal[2usize] - dist;
            d2 =
                endp[0usize] * (*plane).normal[0usize] +
                    endp[1usize] * (*plane).normal[1usize] +
                    endp[2usize] * (*plane).normal[2usize] - dist;
            if d2 > 0i32 as libc::c_float { getout = qtrue }
            if d1 > 0i32 as libc::c_float { startout = qtrue }
            if d1 > 0i32 as libc::c_float &&
                   (d2 as libc::c_double >= 0.125f64 || d2 >= d1) {
                return
            }
            // if it doesn't cross the plane, the plane isn't relevant
            if !(d1 <= 0i32 as libc::c_float && d2 <= 0i32 as libc::c_float) {
                if d1 > d2 {
                    f =
                        ((d1 as libc::c_double - 0.125f64) /
                             (d1 - d2) as libc::c_double) as libc::c_float;
                    if f < 0i32 as libc::c_float { f = 0i32 as libc::c_float }
                    if f > enterFrac {
                        enterFrac = f;
                        clipplane = plane;
                        leadside = side
                    }
                } else {
                    f =
                        ((d1 as libc::c_double + 0.125f64) /
                             (d1 - d2) as libc::c_double) as libc::c_float;
                    if f > 1i32 as libc::c_float { f = 1i32 as libc::c_float }
                    if f < leaveFrac { leaveFrac = f }
                }
            }
            i += 1
        }
    } else {
        i = 0i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            dist =
                (*plane).dist -
                    ((*tw).offsets[(*plane).signbits as usize][0usize] *
                         (*plane).normal[0usize] +
                         (*tw).offsets[(*plane).signbits as usize][1usize] *
                             (*plane).normal[1usize] +
                         (*tw).offsets[(*plane).signbits as usize][2usize] *
                             (*plane).normal[2usize]);
            d1 =
                (*tw).start[0usize] * (*plane).normal[0usize] +
                    (*tw).start[1usize] * (*plane).normal[1usize] +
                    (*tw).start[2usize] * (*plane).normal[2usize] - dist;
            d2 =
                (*tw).end[0usize] * (*plane).normal[0usize] +
                    (*tw).end[1usize] * (*plane).normal[1usize] +
                    (*tw).end[2usize] * (*plane).normal[2usize] - dist;
            if d2 > 0i32 as libc::c_float { getout = qtrue }
            if d1 > 0i32 as libc::c_float { startout = qtrue }
            if d1 > 0i32 as libc::c_float &&
                   (d2 as libc::c_double >= 0.125f64 || d2 >= d1) {
                return
            }
            // if it doesn't cross the plane, the plane isn't relevant
            if !(d1 <= 0i32 as libc::c_float && d2 <= 0i32 as libc::c_float) {
                if d1 > d2 {
                    f =
                        ((d1 as libc::c_double - 0.125f64) /
                             (d1 - d2) as libc::c_double) as libc::c_float;
                    if f < 0i32 as libc::c_float { f = 0i32 as libc::c_float }
                    if f > enterFrac {
                        enterFrac = f;
                        clipplane = plane;
                        leadside = side
                    }
                } else {
                    f =
                        ((d1 as libc::c_double + 0.125f64) /
                             (d1 - d2) as libc::c_double) as libc::c_float;
                    if f > 1i32 as libc::c_float { f = 1i32 as libc::c_float }
                    if f < leaveFrac { leaveFrac = f }
                }
            }
            i += 1
        }
    }
    if 0 == startout as u64 {
        (*tw).trace.startsolid = qtrue;
        if 0 == getout as u64 {
            (*tw).trace.allsolid = qtrue;
            (*tw).trace.fraction = 0i32 as libc::c_float;
            (*tw).trace.contents = (*brush).contents
        }
        return
    }
    if enterFrac < leaveFrac {
        if enterFrac > -1i32 as libc::c_float &&
               enterFrac < (*tw).trace.fraction {
            if enterFrac < 0i32 as libc::c_float {
                enterFrac = 0i32 as libc::c_float
            }
            (*tw).trace.fraction = enterFrac;
            if !clipplane.is_null() { (*tw).trace.plane = *clipplane }
            if !leadside.is_null() {
                (*tw).trace.surfaceFlags = (*leadside).surfaceFlags
            }
            (*tw).trace.contents = (*brush).contents
        }
    };
}
/*
================
CM_TraceBoundingBoxThroughCapsule

bounding box vs. capsule collision
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceBoundingBoxThroughCapsule(mut tw:
                                                               *mut traceWork_t,
                                                           mut model:
                                                               clipHandle_t) {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut size: [vec3_t; 2] = [[0.; 3]; 2];
    let mut h: clipHandle_t = 0;
    let mut cmod: *mut cmodel_t = 0 as *mut cmodel_t;
    let mut i: libc::c_int = 0;
    CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    i = 0i32;
    while i < 3i32 {
        offset[i as usize] =
            ((mins[i as usize] + maxs[i as usize]) as libc::c_double * 0.5f64)
                as vec_t;
        size[0usize][i as usize] = mins[i as usize] - offset[i as usize];
        size[1usize][i as usize] = maxs[i as usize] - offset[i as usize];
        (*tw).start[i as usize] -= offset[i as usize];
        (*tw).end[i as usize] -= offset[i as usize];
        i += 1
    }
    (*tw).sphere.use_0 = qtrue;
    (*tw).sphere.radius =
        if size[1usize][0usize] > size[1usize][2usize] {
            size[1usize][2usize]
        } else { size[1usize][0usize] };
    (*tw).sphere.halfheight = size[1usize][2usize];
    (*tw).sphere.offset[0usize] = 0i32 as vec_t;
    (*tw).sphere.offset[1usize] = 0i32 as vec_t;
    (*tw).sphere.offset[2usize] = size[1usize][2usize] - (*tw).sphere.radius;
    h =
        CM_TempBoxModel((*tw).size[0usize].as_mut_ptr() as *const vec_t,
                        (*tw).size[1usize].as_mut_ptr() as *const vec_t,
                        qfalse as libc::c_int);
    cmod = CM_ClipHandleToModel(h);
    CM_TraceThroughLeaf(tw, &mut (*cmod).leaf);
}
//t[0] = (- b ) / 2 * a;
		// slide along the cylinder
// no intersection at all
/*
================
CM_TraceCapsuleThroughCapsule

capsule vs. capsule collision (not rotated)
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceCapsuleThroughCapsule(mut tw:
                                                           *mut traceWork_t,
                                                       mut model:
                                                           clipHandle_t) {
    let mut i: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut top: vec3_t = [0.; 3];
    let mut bottom: vec3_t = [0.; 3];
    let mut starttop: vec3_t = [0.; 3];
    let mut startbottom: vec3_t = [0.; 3];
    let mut endtop: vec3_t = [0.; 3];
    let mut endbottom: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut symetricSize: [vec3_t; 2] = [[0.; 3]; 2];
    let mut radius: libc::c_float = 0.;
    let mut halfwidth: libc::c_float = 0.;
    let mut halfheight: libc::c_float = 0.;
    let mut offs: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    if (*tw).bounds[0usize][0usize] > maxs[0usize] + 1.0f32 ||
           (*tw).bounds[0usize][1usize] > maxs[1usize] + 1.0f32 ||
           (*tw).bounds[0usize][2usize] > maxs[2usize] + 1.0f32 ||
           (*tw).bounds[1usize][0usize] < mins[0usize] - 1.0f32 ||
           (*tw).bounds[1usize][1usize] < mins[1usize] - 1.0f32 ||
           (*tw).bounds[1usize][2usize] < mins[2usize] - 1.0f32 {
        return
    }
    starttop[0usize] = (*tw).start[0usize] + (*tw).sphere.offset[0usize];
    starttop[1usize] = (*tw).start[1usize] + (*tw).sphere.offset[1usize];
    starttop[2usize] = (*tw).start[2usize] + (*tw).sphere.offset[2usize];
    startbottom[0usize] = (*tw).start[0usize] - (*tw).sphere.offset[0usize];
    startbottom[1usize] = (*tw).start[1usize] - (*tw).sphere.offset[1usize];
    startbottom[2usize] = (*tw).start[2usize] - (*tw).sphere.offset[2usize];
    endtop[0usize] = (*tw).end[0usize] + (*tw).sphere.offset[0usize];
    endtop[1usize] = (*tw).end[1usize] + (*tw).sphere.offset[1usize];
    endtop[2usize] = (*tw).end[2usize] + (*tw).sphere.offset[2usize];
    endbottom[0usize] = (*tw).end[0usize] - (*tw).sphere.offset[0usize];
    endbottom[1usize] = (*tw).end[1usize] - (*tw).sphere.offset[1usize];
    endbottom[2usize] = (*tw).end[2usize] - (*tw).sphere.offset[2usize];
    i = 0i32;
    while i < 3i32 {
        offset[i as usize] =
            ((mins[i as usize] + maxs[i as usize]) as libc::c_double * 0.5f64)
                as vec_t;
        symetricSize[0usize][i as usize] =
            mins[i as usize] - offset[i as usize];
        symetricSize[1usize][i as usize] =
            maxs[i as usize] - offset[i as usize];
        i += 1
    }
    halfwidth = symetricSize[1usize][0usize];
    halfheight = symetricSize[1usize][2usize];
    radius = if halfwidth > halfheight { halfheight } else { halfwidth };
    offs = halfheight - radius;
    top[0usize] = offset[0usize];
    top[1usize] = offset[1usize];
    top[2usize] = offset[2usize];
    top[2usize] += offs;
    bottom[0usize] = offset[0usize];
    bottom[1usize] = offset[1usize];
    bottom[2usize] = offset[2usize];
    bottom[2usize] -= offs;
    radius += (*tw).sphere.radius;
    if (*tw).start[0usize] != (*tw).end[0usize] ||
           (*tw).start[1usize] != (*tw).end[1usize] {
        h = halfheight + (*tw).sphere.halfheight - radius;
        if h > 0i32 as libc::c_float {
            CM_TraceThroughVerticalCylinder(tw, offset.as_mut_ptr(), radius,
                                            h, (*tw).start.as_mut_ptr(),
                                            (*tw).end.as_mut_ptr());
        }
    }
    CM_TraceThroughSphere(tw, top.as_mut_ptr(), radius,
                          startbottom.as_mut_ptr(), endbottom.as_mut_ptr());
    CM_TraceThroughSphere(tw, bottom.as_mut_ptr(), radius,
                          starttop.as_mut_ptr(), endtop.as_mut_ptr());
}
/*
================
CM_TraceThroughSphere

get the first intersection of the ray with the sphere
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceThroughSphere(mut tw: *mut traceWork_t,
                                               mut origin: *mut vec_t,
                                               mut radius: libc::c_float,
                                               mut start: *mut vec_t,
                                               mut end: *mut vec_t) {
    let mut l1: libc::c_float = 0.;
    let mut l2: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut fraction: libc::c_float = 0.;
    //float a;
    let mut b: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut d: libc::c_float = 0.;
    let mut sqrtd: libc::c_float = 0.;
    let mut v1: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut intersection: vec3_t = [0.; 3];
    dir[0usize] = *start.offset(0isize) - *origin.offset(0isize);
    dir[1usize] = *start.offset(1isize) - *origin.offset(1isize);
    dir[2usize] = *start.offset(2isize) - *origin.offset(2isize);
    l1 = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
    if l1 < radius * radius {
        (*tw).trace.fraction = 0i32 as libc::c_float;
        (*tw).trace.startsolid = qtrue;
        dir[0usize] = *end.offset(0isize) - *origin.offset(0isize);
        dir[1usize] = *end.offset(1isize) - *origin.offset(1isize);
        dir[2usize] = *end.offset(2isize) - *origin.offset(2isize);
        l1 = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
        if l1 < radius * radius { (*tw).trace.allsolid = qtrue }
        return
    }
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    length = VectorNormalize(dir.as_mut_ptr());
    l1 = CM_DistanceFromLineSquared(origin, start, end, dir.as_mut_ptr());
    v1[0usize] = *end.offset(0isize) - *origin.offset(0isize);
    v1[1usize] = *end.offset(1isize) - *origin.offset(1isize);
    v1[2usize] = *end.offset(2isize) - *origin.offset(2isize);
    l2 = VectorLengthSquared(v1.as_mut_ptr() as *const vec_t);
    if l1 >= radius * radius &&
           l2 as libc::c_double >
               (radius as libc::c_double + 0.125f64) *
                   (radius as libc::c_double + 0.125f64) {
        return
    }
    v1[0usize] = *start.offset(0isize) - *origin.offset(0isize);
    v1[1usize] = *start.offset(1isize) - *origin.offset(1isize);
    v1[2usize] = *start.offset(2isize) - *origin.offset(2isize);
    b =
        2.0f32 *
            (dir[0usize] * v1[0usize] + dir[1usize] * v1[1usize] +
                 dir[2usize] * v1[2usize]);
    c =
        v1[0usize] * v1[0usize] + v1[1usize] * v1[1usize] +
            v1[2usize] * v1[2usize] - (radius + 1.0f32) * (radius + 1.0f32);
    d = b * b - 4.0f32 * c;
    if d > 0i32 as libc::c_float {
        sqrtd = SquareRootFloat(d);
        fraction = (-b - sqrtd) * 0.5f32;
        if fraction < 0i32 as libc::c_float {
            fraction = 0i32 as libc::c_float
        } else { fraction /= length }
        if fraction < (*tw).trace.fraction {
            (*tw).trace.fraction = fraction;
            dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
            dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
            dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
            intersection[0usize] =
                *start.offset(0isize) + dir[0usize] * fraction;
            intersection[1usize] =
                *start.offset(1isize) + dir[1usize] * fraction;
            intersection[2usize] =
                *start.offset(2isize) + dir[2usize] * fraction;
            dir[0usize] = intersection[0usize] - *origin.offset(0isize);
            dir[1usize] = intersection[1usize] - *origin.offset(1isize);
            dir[2usize] = intersection[2usize] - *origin.offset(2isize);
            scale = 1i32 as libc::c_float / (radius + 1.0f32);
            dir[0usize] = dir[0usize] * scale;
            dir[1usize] = dir[1usize] * scale;
            dir[2usize] = dir[2usize] * scale;
            (*tw).trace.plane.normal[0usize] = dir[0usize];
            (*tw).trace.plane.normal[1usize] = dir[1usize];
            (*tw).trace.plane.normal[2usize] = dir[2usize];
            intersection[0usize] =
                (*tw).modelOrigin[0usize] + intersection[0usize];
            intersection[1usize] =
                (*tw).modelOrigin[1usize] + intersection[1usize];
            intersection[2usize] =
                (*tw).modelOrigin[2usize] + intersection[2usize];
            (*tw).trace.plane.dist =
                (*tw).trace.plane.normal[0usize] * intersection[0usize] +
                    (*tw).trace.plane.normal[1usize] * intersection[1usize] +
                    (*tw).trace.plane.normal[2usize] * intersection[2usize];
            (*tw).trace.contents = 0x2000000i32
        }
    } else { d == 0i32 as libc::c_float; };
}
/*
================
SquareRootFloat
================
*/
#[no_mangle]
pub unsafe extern "C" fn SquareRootFloat(mut number: libc::c_float)
 -> libc::c_float {
    let mut t: floatint_t = floatint_t{f: 0.,};
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let f: libc::c_float = 1.5f32;
    x = number * 0.5f32;
    t.f = number;
    t.i = 0x5f3759dfi32 - (t.i >> 1i32);
    y = t.f;
    y = y * (f - x * y * y);
    y = y * (f - x * y * y);
    return number * y;
}
/*
================
CM_DistanceFromLineSquared
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_DistanceFromLineSquared(mut p: *mut vec_t,
                                                    mut lp1: *mut vec_t,
                                                    mut lp2: *mut vec_t,
                                                    mut dir: *mut vec_t)
 -> libc::c_float {
    let mut proj: vec3_t = [0.; 3];
    let mut t: vec3_t = [0.; 3];
    let mut j: libc::c_int = 0;
    CM_ProjectPointOntoVector(p, lp1, dir, proj.as_mut_ptr());
    j = 0i32;
    while j < 3i32 {
        if proj[j as usize] > *lp1.offset(j as isize) &&
               proj[j as usize] > *lp2.offset(j as isize) ||
               proj[j as usize] < *lp1.offset(j as isize) &&
                   proj[j as usize] < *lp2.offset(j as isize) {
            break ;
        }
        j += 1
    }
    if j < 3i32 {
        if fabs((proj[j as usize] - *lp1.offset(j as isize)) as
                    libc::c_double) <
               fabs((proj[j as usize] - *lp2.offset(j as isize)) as
                        libc::c_double) {
            t[0usize] = *p.offset(0isize) - *lp1.offset(0isize);
            t[1usize] = *p.offset(1isize) - *lp1.offset(1isize);
            t[2usize] = *p.offset(2isize) - *lp1.offset(2isize)
        } else {
            t[0usize] = *p.offset(0isize) - *lp2.offset(0isize);
            t[1usize] = *p.offset(1isize) - *lp2.offset(1isize);
            t[2usize] = *p.offset(2isize) - *lp2.offset(2isize)
        }
        return VectorLengthSquared(t.as_mut_ptr() as *const vec_t)
    }
    t[0usize] = *p.offset(0isize) - proj[0usize];
    t[1usize] = *p.offset(1isize) - proj[1usize];
    t[2usize] = *p.offset(2isize) - proj[2usize];
    return VectorLengthSquared(t.as_mut_ptr() as *const vec_t);
}
/*
================
CM_ProjectPointOntoVector
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_ProjectPointOntoVector(mut point: *mut vec_t,
                                                   mut vStart: *mut vec_t,
                                                   mut vDir: *mut vec_t,
                                                   mut vProj: *mut vec_t) {
    let mut pVec: vec3_t = [0.; 3];
    pVec[0usize] = *point.offset(0isize) - *vStart.offset(0isize);
    pVec[1usize] = *point.offset(1isize) - *vStart.offset(1isize);
    pVec[2usize] = *point.offset(2isize) - *vStart.offset(2isize);
    *vProj.offset(0isize) =
        *vStart.offset(0isize) +
            *vDir.offset(0isize) *
                (pVec[0usize] * *vDir.offset(0isize) +
                     pVec[1usize] * *vDir.offset(1isize) +
                     pVec[2usize] * *vDir.offset(2isize));
    *vProj.offset(1isize) =
        *vStart.offset(1isize) +
            *vDir.offset(1isize) *
                (pVec[0usize] * *vDir.offset(0isize) +
                     pVec[1usize] * *vDir.offset(1isize) +
                     pVec[2usize] * *vDir.offset(2isize));
    *vProj.offset(2isize) =
        *vStart.offset(2isize) +
            *vDir.offset(2isize) *
                (pVec[0usize] * *vDir.offset(0isize) +
                     pVec[1usize] * *vDir.offset(1isize) +
                     pVec[2usize] * *vDir.offset(2isize));
}
//t1 = (- b ) / 2;
		// slide along the sphere
// no intersection at all
/*
================
CM_TraceThroughVerticalCylinder

get the first intersection of the ray with the cylinder
the cylinder extends halfheight above and below the origin
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TraceThroughVerticalCylinder(mut tw:
                                                             *mut traceWork_t,
                                                         mut origin:
                                                             *mut vec_t,
                                                         mut radius:
                                                             libc::c_float,
                                                         mut halfheight:
                                                             libc::c_float,
                                                         mut start:
                                                             *mut vec_t,
                                                         mut end:
                                                             *mut vec_t) {
    let mut length: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut fraction: libc::c_float = 0.;
    let mut l1: libc::c_float = 0.;
    let mut l2: libc::c_float = 0.;
    //float a;
    let mut b: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut d: libc::c_float = 0.;
    let mut sqrtd: libc::c_float = 0.;
    let mut v1: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut start2d: vec3_t = [0.; 3];
    let mut end2d: vec3_t = [0.; 3];
    let mut org2d: vec3_t = [0.; 3];
    let mut intersection: vec3_t = [0.; 3];
    start2d[0usize] = *start.offset(0isize);
    start2d[1usize] = *start.offset(1isize);
    start2d[2usize] = 0i32 as vec_t;
    end2d[0usize] = *end.offset(0isize);
    end2d[1usize] = *end.offset(1isize);
    end2d[2usize] = 0i32 as vec_t;
    org2d[0usize] = *origin.offset(0isize);
    org2d[1usize] = *origin.offset(1isize);
    org2d[2usize] = 0i32 as vec_t;
    if *start.offset(2isize) <= *origin.offset(2isize) + halfheight &&
           *start.offset(2isize) >= *origin.offset(2isize) - halfheight {
        dir[0usize] = start2d[0usize] - org2d[0usize];
        dir[1usize] = start2d[1usize] - org2d[1usize];
        dir[2usize] = start2d[2usize] - org2d[2usize];
        l1 = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
        if l1 < radius * radius {
            (*tw).trace.fraction = 0i32 as libc::c_float;
            (*tw).trace.startsolid = qtrue;
            dir[0usize] = end2d[0usize] - org2d[0usize];
            dir[1usize] = end2d[1usize] - org2d[1usize];
            dir[2usize] = end2d[2usize] - org2d[2usize];
            l1 = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
            if l1 < radius * radius { (*tw).trace.allsolid = qtrue }
            return
        }
    }
    dir[0usize] = end2d[0usize] - start2d[0usize];
    dir[1usize] = end2d[1usize] - start2d[1usize];
    dir[2usize] = end2d[2usize] - start2d[2usize];
    length = VectorNormalize(dir.as_mut_ptr());
    l1 =
        CM_DistanceFromLineSquared(org2d.as_mut_ptr(), start2d.as_mut_ptr(),
                                   end2d.as_mut_ptr(), dir.as_mut_ptr());
    v1[0usize] = end2d[0usize] - org2d[0usize];
    v1[1usize] = end2d[1usize] - org2d[1usize];
    v1[2usize] = end2d[2usize] - org2d[2usize];
    l2 = VectorLengthSquared(v1.as_mut_ptr() as *const vec_t);
    if l1 >= radius * radius &&
           l2 as libc::c_double >
               (radius as libc::c_double + 0.125f64) *
                   (radius as libc::c_double + 0.125f64) {
        return
    }
    v1[0usize] = *start.offset(0isize) - *origin.offset(0isize);
    v1[1usize] = *start.offset(1isize) - *origin.offset(1isize);
    v1[2usize] = *start.offset(2isize) - *origin.offset(2isize);
    b = 2.0f32 * (v1[0usize] * dir[0usize] + v1[1usize] * dir[1usize]);
    c =
        v1[0usize] * v1[0usize] + v1[1usize] * v1[1usize] -
            (radius + 1.0f32) * (radius + 1.0f32);
    d = b * b - 4.0f32 * c;
    if d > 0i32 as libc::c_float {
        sqrtd = SquareRootFloat(d);
        fraction = (-b - sqrtd) * 0.5f32;
        if fraction < 0i32 as libc::c_float {
            fraction = 0i32 as libc::c_float
        } else { fraction /= length }
        if fraction < (*tw).trace.fraction {
            dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
            dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
            dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
            intersection[0usize] =
                *start.offset(0isize) + dir[0usize] * fraction;
            intersection[1usize] =
                *start.offset(1isize) + dir[1usize] * fraction;
            intersection[2usize] =
                *start.offset(2isize) + dir[2usize] * fraction;
            if intersection[2usize] <= *origin.offset(2isize) + halfheight &&
                   intersection[2usize] >= *origin.offset(2isize) - halfheight
               {
                (*tw).trace.fraction = fraction;
                dir[0usize] = intersection[0usize] - *origin.offset(0isize);
                dir[1usize] = intersection[1usize] - *origin.offset(1isize);
                dir[2usize] = intersection[2usize] - *origin.offset(2isize);
                dir[2usize] = 0i32 as vec_t;
                scale = 1i32 as libc::c_float / (radius + 1.0f32);
                dir[0usize] = dir[0usize] * scale;
                dir[1usize] = dir[1usize] * scale;
                dir[2usize] = dir[2usize] * scale;
                (*tw).trace.plane.normal[0usize] = dir[0usize];
                (*tw).trace.plane.normal[1usize] = dir[1usize];
                (*tw).trace.plane.normal[2usize] = dir[2usize];
                intersection[0usize] =
                    (*tw).modelOrigin[0usize] + intersection[0usize];
                intersection[1usize] =
                    (*tw).modelOrigin[1usize] + intersection[1usize];
                intersection[2usize] =
                    (*tw).modelOrigin[2usize] + intersection[2usize];
                (*tw).trace.plane.dist =
                    (*tw).trace.plane.normal[0usize] * intersection[0usize] +
                        (*tw).trace.plane.normal[1usize] *
                            intersection[1usize] +
                        (*tw).trace.plane.normal[2usize] *
                            intersection[2usize];
                (*tw).trace.contents = 0x2000000i32
            }
        }
    } else { d == 0i32 as libc::c_float; };
}
/*
==================
CM_PositionTest
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_PositionTest(mut tw: *mut traceWork_t) {
    let mut leafs: [libc::c_int; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut ll: leafList_t =
        leafList_s{count: 0,
                   maxcount: 0,
                   overflowed: qfalse,
                   list: 0 as *mut libc::c_int,
                   bounds: [[0.; 3]; 2],
                   lastLeaf: 0,
                   storeLeafs: None,};
    ll.bounds[0usize][0usize] =
        (*tw).start[0usize] + (*tw).size[0usize][0usize];
    ll.bounds[0usize][1usize] =
        (*tw).start[1usize] + (*tw).size[0usize][1usize];
    ll.bounds[0usize][2usize] =
        (*tw).start[2usize] + (*tw).size[0usize][2usize];
    ll.bounds[1usize][0usize] =
        (*tw).start[0usize] + (*tw).size[1usize][0usize];
    ll.bounds[1usize][1usize] =
        (*tw).start[1usize] + (*tw).size[1usize][1usize];
    ll.bounds[1usize][2usize] =
        (*tw).start[2usize] + (*tw).size[1usize][2usize];
    i = 0i32;
    while i < 3i32 {
        ll.bounds[0usize][i as usize] -= 1i32 as libc::c_float;
        ll.bounds[1usize][i as usize] += 1i32 as libc::c_float;
        i += 1
    }
    ll.count = 0i32;
    ll.maxcount = 1024i32;
    ll.list = leafs.as_mut_ptr();
    ll.storeLeafs = Some(CM_StoreLeafs);
    ll.lastLeaf = 0i32;
    ll.overflowed = qfalse;
    cm.checkcount += 1;
    CM_BoxLeafnums_r(&mut ll, 0i32);
    cm.checkcount += 1;
    i = 0i32;
    while i < ll.count {
        CM_TestInLeaf(tw,
                      &mut *cm.leafs.offset(*leafs.as_mut_ptr().offset(i as
                                                                           isize)
                                                as isize));
        if 0 != (*tw).trace.allsolid as u64 { break ; }
        i += 1
    };
}
/*
================
CM_TestInLeaf
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TestInLeaf(mut tw: *mut traceWork_t,
                                       mut leaf: *mut cLeaf_t) {
    let mut k: libc::c_int = 0;
    let mut brushnum: libc::c_int = 0;
    let mut b: *mut cbrush_t = 0 as *mut cbrush_t;
    let mut patch: *mut cPatch_t = 0 as *mut cPatch_t;
    k = 0i32;
    while k < (*leaf).numLeafBrushes {
        brushnum =
            *cm.leafbrushes.offset(((*leaf).firstLeafBrush + k) as isize);
        b = &mut *cm.brushes.offset(brushnum as isize) as *mut cbrush_t;
        if !((*b).checkcount == cm.checkcount) {
            // already checked this brush in another leaf
            (*b).checkcount = cm.checkcount;
            if !(0 == (*b).contents & (*tw).contents) {
                CM_TestBoxInBrush(tw, b);
                if 0 != (*tw).trace.allsolid as u64 { return }
            }
        }
        k += 1
    }
    if 0 == (*cm_noCurves).integer {
        k = 0i32;
        while k < (*leaf).numLeafSurfaces {
            patch =
                *cm.surfaces.offset(*cm.leafsurfaces.offset(((*leaf).firstLeafSurface
                                                                 + k) as
                                                                isize) as
                                        isize);
            if !patch.is_null() {
                if !((*patch).checkcount == cm.checkcount) {
                    // already checked this brush in another leaf
                    (*patch).checkcount = cm.checkcount;
                    if !(0 == (*patch).contents & (*tw).contents) {
                        if 0 !=
                               CM_PositionTestInPatchCollide(tw, (*patch).pc)
                                   as u64 {
                            (*tw).trace.allsolid = qtrue;
                            (*tw).trace.startsolid = (*tw).trace.allsolid;
                            (*tw).trace.fraction = 0i32 as libc::c_float;
                            (*tw).trace.contents = (*patch).contents;
                            return
                        }
                    }
                }
            }
            k += 1
        }
    };
}
/*
===============================================================================

POSITION TESTING

===============================================================================
*/
/*
================
CM_TestBoxInBrush
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TestBoxInBrush(mut tw: *mut traceWork_t,
                                           mut brush: *mut cbrush_t) {
    let mut i: libc::c_int = 0;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut dist: libc::c_float = 0.;
    let mut d1: libc::c_float = 0.;
    let mut side: *mut cbrushside_t = 0 as *mut cbrushside_t;
    let mut t: libc::c_float = 0.;
    let mut startp: vec3_t = [0.; 3];
    if 0 == (*brush).numsides { return }
    if (*tw).bounds[0usize][0usize] > (*brush).bounds[1usize][0usize] ||
           (*tw).bounds[0usize][1usize] > (*brush).bounds[1usize][1usize] ||
           (*tw).bounds[0usize][2usize] > (*brush).bounds[1usize][2usize] ||
           (*tw).bounds[1usize][0usize] < (*brush).bounds[0usize][0usize] ||
           (*tw).bounds[1usize][1usize] < (*brush).bounds[0usize][1usize] ||
           (*tw).bounds[1usize][2usize] < (*brush).bounds[0usize][2usize] {
        return
    }
    if 0 != (*tw).sphere.use_0 as u64 {
        i = 6i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            dist = (*plane).dist + (*tw).sphere.radius;
            t =
                (*plane).normal[0usize] * (*tw).sphere.offset[0usize] +
                    (*plane).normal[1usize] * (*tw).sphere.offset[1usize] +
                    (*plane).normal[2usize] * (*tw).sphere.offset[2usize];
            if t > 0i32 as libc::c_float {
                startp[0usize] =
                    (*tw).start[0usize] - (*tw).sphere.offset[0usize];
                startp[1usize] =
                    (*tw).start[1usize] - (*tw).sphere.offset[1usize];
                startp[2usize] =
                    (*tw).start[2usize] - (*tw).sphere.offset[2usize]
            } else {
                startp[0usize] =
                    (*tw).start[0usize] + (*tw).sphere.offset[0usize];
                startp[1usize] =
                    (*tw).start[1usize] + (*tw).sphere.offset[1usize];
                startp[2usize] =
                    (*tw).start[2usize] + (*tw).sphere.offset[2usize]
            }
            d1 =
                startp[0usize] * (*plane).normal[0usize] +
                    startp[1usize] * (*plane).normal[1usize] +
                    startp[2usize] * (*plane).normal[2usize] - dist;
            if d1 > 0i32 as libc::c_float { return }
            i += 1
        }
    } else {
        i = 6i32;
        while i < (*brush).numsides {
            side = (*brush).sides.offset(i as isize);
            plane = (*side).plane;
            dist =
                (*plane).dist -
                    ((*tw).offsets[(*plane).signbits as usize][0usize] *
                         (*plane).normal[0usize] +
                         (*tw).offsets[(*plane).signbits as usize][1usize] *
                             (*plane).normal[1usize] +
                         (*tw).offsets[(*plane).signbits as usize][2usize] *
                             (*plane).normal[2usize]);
            d1 =
                (*tw).start[0usize] * (*plane).normal[0usize] +
                    (*tw).start[1usize] * (*plane).normal[1usize] +
                    (*tw).start[2usize] * (*plane).normal[2usize] - dist;
            if d1 > 0i32 as libc::c_float { return }
            i += 1
        }
    }
    (*tw).trace.allsolid = qtrue;
    (*tw).trace.startsolid = (*tw).trace.allsolid;
    (*tw).trace.fraction = 0i32 as libc::c_float;
    (*tw).trace.contents = (*brush).contents;
}
/*
==================
CM_TestBoundingBoxInCapsule

bounding box inside capsule check
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TestBoundingBoxInCapsule(mut tw: *mut traceWork_t,
                                                     mut model:
                                                         clipHandle_t) {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut size: [vec3_t; 2] = [[0.; 3]; 2];
    let mut h: clipHandle_t = 0;
    let mut cmod: *mut cmodel_t = 0 as *mut cmodel_t;
    let mut i: libc::c_int = 0;
    CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    i = 0i32;
    while i < 3i32 {
        offset[i as usize] =
            ((mins[i as usize] + maxs[i as usize]) as libc::c_double * 0.5f64)
                as vec_t;
        size[0usize][i as usize] = mins[i as usize] - offset[i as usize];
        size[1usize][i as usize] = maxs[i as usize] - offset[i as usize];
        (*tw).start[i as usize] -= offset[i as usize];
        (*tw).end[i as usize] -= offset[i as usize];
        i += 1
    }
    (*tw).sphere.use_0 = qtrue;
    (*tw).sphere.radius =
        if size[1usize][0usize] > size[1usize][2usize] {
            size[1usize][2usize]
        } else { size[1usize][0usize] };
    (*tw).sphere.halfheight = size[1usize][2usize];
    (*tw).sphere.offset[0usize] = 0i32 as vec_t;
    (*tw).sphere.offset[1usize] = 0i32 as vec_t;
    (*tw).sphere.offset[2usize] = size[1usize][2usize] - (*tw).sphere.radius;
    h =
        CM_TempBoxModel((*tw).size[0usize].as_mut_ptr() as *const vec_t,
                        (*tw).size[1usize].as_mut_ptr() as *const vec_t,
                        qfalse as libc::c_int);
    cmod = CM_ClipHandleToModel(h);
    CM_TestInLeaf(tw, &mut (*cmod).leaf);
}
/*
==================
CM_TestCapsuleInCapsule

capsule inside capsule check
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TestCapsuleInCapsule(mut tw: *mut traceWork_t,
                                                 mut model: clipHandle_t) {
    let mut i: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut top: vec3_t = [0.; 3];
    let mut bottom: vec3_t = [0.; 3];
    let mut p1: vec3_t = [0.; 3];
    let mut p2: vec3_t = [0.; 3];
    let mut tmp: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut symetricSize: [vec3_t; 2] = [[0.; 3]; 2];
    let mut radius: libc::c_float = 0.;
    let mut halfwidth: libc::c_float = 0.;
    let mut halfheight: libc::c_float = 0.;
    let mut offs: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    CM_ModelBounds(model, mins.as_mut_ptr(), maxs.as_mut_ptr());
    top[0usize] = (*tw).start[0usize] + (*tw).sphere.offset[0usize];
    top[1usize] = (*tw).start[1usize] + (*tw).sphere.offset[1usize];
    top[2usize] = (*tw).start[2usize] + (*tw).sphere.offset[2usize];
    bottom[0usize] = (*tw).start[0usize] - (*tw).sphere.offset[0usize];
    bottom[1usize] = (*tw).start[1usize] - (*tw).sphere.offset[1usize];
    bottom[2usize] = (*tw).start[2usize] - (*tw).sphere.offset[2usize];
    i = 0i32;
    while i < 3i32 {
        offset[i as usize] =
            ((mins[i as usize] + maxs[i as usize]) as libc::c_double * 0.5f64)
                as vec_t;
        symetricSize[0usize][i as usize] =
            mins[i as usize] - offset[i as usize];
        symetricSize[1usize][i as usize] =
            maxs[i as usize] - offset[i as usize];
        i += 1
    }
    halfwidth = symetricSize[1usize][0usize];
    halfheight = symetricSize[1usize][2usize];
    radius = if halfwidth > halfheight { halfheight } else { halfwidth };
    offs = halfheight - radius;
    r = ((*tw).sphere.radius + radius) * ((*tw).sphere.radius + radius);
    p1[0usize] = offset[0usize];
    p1[1usize] = offset[1usize];
    p1[2usize] = offset[2usize];
    p1[2usize] += offs;
    tmp[0usize] = p1[0usize] - top[0usize];
    tmp[1usize] = p1[1usize] - top[1usize];
    tmp[2usize] = p1[2usize] - top[2usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const vec_t) < r {
        (*tw).trace.allsolid = qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0i32 as libc::c_float
    }
    tmp[0usize] = p1[0usize] - bottom[0usize];
    tmp[1usize] = p1[1usize] - bottom[1usize];
    tmp[2usize] = p1[2usize] - bottom[2usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const vec_t) < r {
        (*tw).trace.allsolid = qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0i32 as libc::c_float
    }
    p2[0usize] = offset[0usize];
    p2[1usize] = offset[1usize];
    p2[2usize] = offset[2usize];
    p2[2usize] -= offs;
    tmp[0usize] = p2[0usize] - top[0usize];
    tmp[1usize] = p2[1usize] - top[1usize];
    tmp[2usize] = p2[2usize] - top[2usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const vec_t) < r {
        (*tw).trace.allsolid = qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0i32 as libc::c_float
    }
    tmp[0usize] = p2[0usize] - bottom[0usize];
    tmp[1usize] = p2[1usize] - bottom[1usize];
    tmp[2usize] = p2[2usize] - bottom[2usize];
    if VectorLengthSquared(tmp.as_mut_ptr() as *const vec_t) < r {
        (*tw).trace.allsolid = qtrue;
        (*tw).trace.startsolid = (*tw).trace.allsolid;
        (*tw).trace.fraction = 0i32 as libc::c_float
    }
    if top[2usize] >= p1[2usize] && top[2usize] <= p2[2usize] ||
           bottom[2usize] >= p1[2usize] && bottom[2usize] <= p2[2usize] {
        p1[2usize] = 0i32 as vec_t;
        top[2usize] = p1[2usize];
        tmp[0usize] = top[0usize] - p1[0usize];
        tmp[1usize] = top[1usize] - p1[1usize];
        tmp[2usize] = top[2usize] - p1[2usize];
        if VectorLengthSquared(tmp.as_mut_ptr() as *const vec_t) < r {
            (*tw).trace.allsolid = qtrue;
            (*tw).trace.startsolid = (*tw).trace.allsolid;
            (*tw).trace.fraction = 0i32 as libc::c_float
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CM_TransformedBoxTrace(mut results: *mut trace_t,
                                                mut start: *const vec_t,
                                                mut end: *const vec_t,
                                                mut mins: *mut vec_t,
                                                mut maxs: *mut vec_t,
                                                mut model: clipHandle_t,
                                                mut brushmask: libc::c_int,
                                                mut origin: *const vec_t,
                                                mut angles: *const vec_t,
                                                mut capsule: libc::c_int) {
    let mut trace: trace_t =
        trace_t{allsolid: qfalse,
                startsolid: qfalse,
                fraction: 0.,
                endpos: [0.; 3],
                plane:
                    cplane_s{normal: [0.; 3],
                             dist: 0.,
                             type_0: 0,
                             signbits: 0,
                             pad: [0; 2],},
                surfaceFlags: 0,
                contents: 0,
                entityNum: 0,};
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut rotated: qboolean = qfalse;
    let mut offset: vec3_t = [0.; 3];
    let mut symetricSize: [vec3_t; 2] = [[0.; 3]; 2];
    let mut matrix: [vec3_t; 3] = [[0.; 3]; 3];
    let mut transpose: [vec3_t; 3] = [[0.; 3]; 3];
    let mut i: libc::c_int = 0;
    let mut halfwidth: libc::c_float = 0.;
    let mut halfheight: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut sphere: sphere_t =
        sphere_t{use_0: qfalse, radius: 0., halfheight: 0., offset: [0.; 3],};
    if mins.is_null() { mins = vec3_origin.as_mut_ptr() }
    if maxs.is_null() { maxs = vec3_origin.as_mut_ptr() }
    i = 0i32;
    while i < 3i32 {
        offset[i as usize] =
            ((*mins.offset(i as isize) + *maxs.offset(i as isize)) as
                 libc::c_double * 0.5f64) as vec_t;
        symetricSize[0usize][i as usize] =
            *mins.offset(i as isize) - offset[i as usize];
        symetricSize[1usize][i as usize] =
            *maxs.offset(i as isize) - offset[i as usize];
        start_l[i as usize] = *start.offset(i as isize) + offset[i as usize];
        end_l[i as usize] = *end.offset(i as isize) + offset[i as usize];
        i += 1
    }
    start_l[0usize] = start_l[0usize] - *origin.offset(0isize);
    start_l[1usize] = start_l[1usize] - *origin.offset(1isize);
    start_l[2usize] = start_l[2usize] - *origin.offset(2isize);
    end_l[0usize] = end_l[0usize] - *origin.offset(0isize);
    end_l[1usize] = end_l[1usize] - *origin.offset(1isize);
    end_l[2usize] = end_l[2usize] - *origin.offset(2isize);
    if model != 255i32 &&
           (0. != *angles.offset(0isize) || 0. != *angles.offset(1isize) ||
                0. != *angles.offset(2isize)) {
        rotated = qtrue
    } else { rotated = qfalse }
    halfwidth = symetricSize[1usize][0usize];
    halfheight = symetricSize[1usize][2usize];
    sphere.use_0 = capsule as qboolean;
    sphere.radius =
        if halfwidth > halfheight { halfheight } else { halfwidth };
    sphere.halfheight = halfheight;
    t = halfheight - sphere.radius;
    if 0 != rotated as u64 {
        CreateRotationMatrix(angles, matrix.as_mut_ptr());
        RotatePoint(start_l.as_mut_ptr(), matrix.as_mut_ptr());
        RotatePoint(end_l.as_mut_ptr(), matrix.as_mut_ptr());
        sphere.offset[0usize] = matrix[0usize][2usize] * t;
        sphere.offset[1usize] = -matrix[1usize][2usize] * t;
        sphere.offset[2usize] = matrix[2usize][2usize] * t
    } else {
        sphere.offset[0usize] = 0i32 as vec_t;
        sphere.offset[1usize] = 0i32 as vec_t;
        sphere.offset[2usize] = t
    }
    CM_Trace(&mut trace, start_l.as_mut_ptr() as *const vec_t,
             end_l.as_mut_ptr() as *const vec_t,
             symetricSize[0usize].as_mut_ptr(),
             symetricSize[1usize].as_mut_ptr(), model, origin, brushmask,
             capsule, &mut sphere);
    if 0 != rotated as libc::c_uint &&
           trace.fraction as libc::c_double != 1.0f64 {
        TransposeMatrix(matrix.as_mut_ptr(), transpose.as_mut_ptr());
        RotatePoint(trace.plane.normal.as_mut_ptr(), transpose.as_mut_ptr());
    }
    trace.endpos[0usize] =
        *start.offset(0isize) +
            trace.fraction * (*end.offset(0isize) - *start.offset(0isize));
    trace.endpos[1usize] =
        *start.offset(1isize) +
            trace.fraction * (*end.offset(1isize) - *start.offset(1isize));
    trace.endpos[2usize] =
        *start.offset(2isize) +
            trace.fraction * (*end.offset(2isize) - *start.offset(2isize));
    *results = trace;
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
// always use bbox vs. bbox collision and never capsule vs. bbox or vice versa
//#define ALWAYS_BBOX_VS_BBOX
// always use capsule vs. capsule collision and never capsule vs. bbox or vice versa
//#define ALWAYS_CAPSULE_VS_CAPSULE
//#define CAPSULE_DEBUG
/*
===============================================================================

BASIC MATH

===============================================================================
*/
/*
================
RotatePoint
================
*/
#[no_mangle]
pub unsafe extern "C" fn RotatePoint(mut point: *mut vec_t,
                                     mut matrix: *mut vec3_t) {
    let mut tvec: vec3_t = [0.; 3];
    tvec[0usize] = *point.offset(0isize);
    tvec[1usize] = *point.offset(1isize);
    tvec[2usize] = *point.offset(2isize);
    *point.offset(0isize) =
        (*matrix.offset(0isize))[0usize] * tvec[0usize] +
            (*matrix.offset(0isize))[1usize] * tvec[1usize] +
            (*matrix.offset(0isize))[2usize] * tvec[2usize];
    *point.offset(1isize) =
        (*matrix.offset(1isize))[0usize] * tvec[0usize] +
            (*matrix.offset(1isize))[1usize] * tvec[1usize] +
            (*matrix.offset(1isize))[2usize] * tvec[2usize];
    *point.offset(2isize) =
        (*matrix.offset(2isize))[0usize] * tvec[0usize] +
            (*matrix.offset(2isize))[1usize] * tvec[1usize] +
            (*matrix.offset(2isize))[2usize] * tvec[2usize];
}
/*
================
TransposeMatrix
================
*/
#[no_mangle]
pub unsafe extern "C" fn TransposeMatrix(mut matrix: *mut vec3_t,
                                         mut transpose: *mut vec3_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        j = 0i32;
        while j < 3i32 {
            (*transpose.offset(i as isize))[j as usize] =
                (*matrix.offset(j as isize))[i as usize];
            j += 1
        }
        i += 1
    };
}
/*
================
CreateRotationMatrix
================
*/
#[no_mangle]
pub unsafe extern "C" fn CreateRotationMatrix(mut angles: *const vec_t,
                                              mut matrix: *mut vec3_t) {
    AngleVectors(angles, (*matrix.offset(0isize)).as_mut_ptr(),
                 (*matrix.offset(1isize)).as_mut_ptr(),
                 (*matrix.offset(2isize)).as_mut_ptr());
    VectorInverse((*matrix.offset(1isize)).as_mut_ptr());
}
/*
================
CM_VectorDistanceSquared
================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_VectorDistanceSquared(mut p1: *mut vec_t,
                                                  mut p2: *mut vec_t)
 -> libc::c_float {
    let mut dir: vec3_t = [0.; 3];
    dir[0usize] = *p2.offset(0isize) - *p1.offset(0isize);
    dir[1usize] = *p2.offset(1isize) - *p1.offset(1isize);
    dir[2usize] = *p2.offset(2isize) - *p1.offset(2isize);
    return VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
}