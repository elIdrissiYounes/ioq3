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
        pub fn Hunk_AllocDebug(size: libc::c_int, preference: ha_pref,
                               label: *mut libc::c_char,
                               file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub static mut vec3_origin: vec3_t;
        #[no_mangle]
        pub fn ClearBounds(mins: *mut vec_t, maxs: *mut vec_t);
        #[no_mangle]
        pub fn AddPointToBounds(v: *const vec_t, mins: *mut vec_t,
                                maxs: *mut vec_t);
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_polylib.h"]
pub mod cm_polylib_h {
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
    // this is only used for visualization tools in cm_ debug functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct winding_t {
        pub numpoints: libc::c_int,
        pub p: [vec3_t; 4],
    }
    use super::{libc};
    use super::q_shared_h::{vec3_t, vec_t};
    extern "C" {
        #[no_mangle]
        pub fn FreeWinding(w: *mut winding_t);
        #[no_mangle]
        pub fn ChopWindingInPlace(w: *mut *mut winding_t, normal: *mut vec_t,
                                  dist: vec_t, epsilon: vec_t);
        #[no_mangle]
        pub fn BaseWindingForPlane(normal: *mut vec_t, dist: vec_t)
         -> *mut winding_t;
        #[no_mangle]
        pub fn CopyWinding(w: *mut winding_t) -> *mut winding_t;
        #[no_mangle]
        pub fn WindingBounds(w: *mut winding_t, mins: *mut vec_t,
                             maxs: *mut vec_t);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_patch.h"]
pub mod cm_patch_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct facet_t {
        pub surfacePlane: libc::c_int,
        pub numBorders: libc::c_int,
        pub borderPlanes: [libc::c_int; 26],
        pub borderInward: [libc::c_int; 26],
        pub borderNoAdjust: [qboolean; 26],
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
    //#define	CULL_BBOX
    /*

This file does not reference any globals, and has these entry points:

void CM_ClearLevelPatches( void );
struct patchCollide_s	*CM_GeneratePatchCollide( int width, int height, const vec3_t *points );
void CM_TraceThroughPatchCollide( traceWork_t *tw, const struct patchCollide_s *pc );
qboolean CM_PositionTestInPatchCollide( traceWork_t *tw, const struct patchCollide_s *pc );
void CM_DrawDebugSurface( void (*drawPoly)(int color, int numPoints, flaot *points) );


Issues for collision against curved surfaces:

Surface edges need to be handled differently than surface planes

Plane expansion causes raw surfaces to expand past expanded bounding box

Position test of a volume against a surface is tricky.

Position test of a point against a surface is not well defined, because the surface has no volume.


Tracing leading edge points instead of volumes?
Position test by tracing corner to corner? (8*7 traces -- ouch)

coplanar edges
triangulated patches
degenerate patches

  endcaps
  degenerate

WARNING: this may misbehave with meshes that have rows or columns that only
degenerate a few triangles.  Completely degenerate rows and columns are handled
properly.
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct patchPlane_t {
        pub plane: [libc::c_float; 4],
        pub signbits: libc::c_int,
    }
    pub type patchCollide_t = patchCollide_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct patchCollide_s {
        pub bounds: [vec3_t; 2],
        pub numPlanes: libc::c_int,
        pub planes: *mut patchPlane_t,
        pub numFacets: libc::c_int,
        pub facets: *mut facet_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cGrid_t {
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub wrapWidth: qboolean,
        pub wrapHeight: qboolean,
        pub points: [[vec3_t; 129]; 129],
    }
    use super::{libc};
    use super::q_shared_h::{qboolean, vec3_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_local.h"]
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
    use super::q_shared_h::{qboolean, vec3_t, trace_t, cvar_t, vec_t};
    use super::{libc};
    use super::cm_patch_h::{patchCollide_s};
    extern "C" {
        #[no_mangle]
        pub static mut cm_playerCurveClip: *mut cvar_t;
        #[no_mangle]
        pub fn CM_BoundsIntersect(mins: *const vec_t, maxs: *const vec_t,
                                  mins2: *const vec_t, maxs2: *const vec_t)
         -> qboolean;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_patch.c"]
pub mod cm_patch_c {
    pub const EN_TOP: unnamed_0 = 0;
    pub const EN_LEFT: unnamed_0 = 3;
    pub const EN_BOTTOM: unnamed_0 = 2;
    pub const EN_RIGHT: unnamed_0 = 1;
    //BSPC
    pub type unnamed_0 = libc::c_uint;
    use super::{libc};
    use super::cm_patch_h::{facet_t, patchPlane_t, patchCollide_s};
    use super::q_shared_h::{vec_t};
    use super::cm_local_h::{traceWork_t};
    extern "C" {
        /*
=======================================================================

DEBUGGING

=======================================================================
*/
        /*
==================
CM_DrawDebugSurface

Called from the renderer
==================
*/
        #[no_mangle]
        pub fn BotDrawDebugPolygons(drawPoly:
                                        Option<unsafe extern "C" fn(_:
                                                                        libc::c_int,
                                                                    _:
                                                                        libc::c_int,
                                                                    _:
                                                                        *mut libc::c_float)
                                                   -> ()>,
                                    value: libc::c_int);
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
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
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::q_shared_h::{cvar_t};
    use super::{libc};
    extern "C" {
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
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, ha_pref, h_dontcare, h_low, h_high, vec_t,
                       vec3_t, cplane_s, cvar_s, cvar_t, cplane_t, trace_t,
                       Hunk_AllocDebug, vec3_origin, ClearBounds,
                       AddPointToBounds, VectorNormalize, Com_Error,
                       Com_Printf};
use self::cm_polylib_h::{winding_t, FreeWinding, ChopWindingInPlace,
                         BaseWindingForPlane, CopyWinding, WindingBounds};
use self::cm_patch_h::{facet_t, patchPlane_t, patchCollide_t, patchCollide_s,
                       cGrid_t};
use self::cm_local_h::{sphere_t, traceWork_t, cm_playerCurveClip,
                       CM_BoundsIntersect};
use self::cm_patch_c::{EN_TOP, EN_LEFT, EN_BOTTOM, EN_RIGHT, unnamed_0,
                       BotDrawDebugPolygons};
use self::mathcalls_h::{sqrt, fabs};
use self::string_h::{memcpy, memset};
use self::qcommon_h::{Cvar_Get, Com_DPrintf};
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt((*v.offset(0isize) * *v.offset(0isize) +
                     *v.offset(1isize) * *v.offset(1isize) +
                     *v.offset(2isize) * *v.offset(2isize)) as libc::c_double)
               as vec_t;
}
unsafe extern "C" fn CrossProduct(mut v1: *const vec_t, mut v2: *const vec_t,
                                  mut cross: *mut vec_t) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) -
            *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) -
            *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) -
            *v1.offset(1isize) * *v2.offset(0isize);
}
// cm_patch.c
#[no_mangle]
pub unsafe extern "C" fn CM_DrawDebugSurface(mut drawPoly:
                                                 Option<unsafe extern "C" fn(_:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 libc::c_int,
                                                                             _:
                                                                                 *mut libc::c_float)
                                                            -> ()>) {
    static mut cv: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    static mut cv2: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    let mut pc: *const patchCollide_t = 0 as *const patchCollide_t;
    let mut facet: *mut facet_t = 0 as *mut facet_t;
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut curplanenum: libc::c_int = 0;
    let mut planenum: libc::c_int = 0;
    let mut curinward: libc::c_int = 0;
    let mut inward: libc::c_int = 0;
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut mins: vec3_t =
        [-15i32 as vec_t, -15i32 as vec_t, -28i32 as vec_t];
    let mut maxs: vec3_t = [15i32 as vec_t, 15i32 as vec_t, 28i32 as vec_t];
    //vec3_t mins = {0, 0, 0}, maxs = {0, 0, 0};
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    if cv2.is_null() {
        cv2 =
            Cvar_Get(b"r_debugSurface\x00" as *const u8 as
                         *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char, 0i32)
    }
    if (*cv2).integer != 1i32 {
        BotDrawDebugPolygons(drawPoly, (*cv2).integer);
        return
    }
    if debugPatchCollide.is_null() { return }
    if cv.is_null() {
        cv =
            Cvar_Get(b"cm_debugSize\x00" as *const u8 as *const libc::c_char,
                     b"2\x00" as *const u8 as *const libc::c_char, 0i32)
    }
    pc = debugPatchCollide;
    i = 0i32;
    facet = (*pc).facets;
    while i < (*pc).numFacets {
        k = 0i32;
        while k < (*facet).numBorders + 1i32 {
            if k < (*facet).numBorders {
                planenum = (*facet).borderPlanes[k as usize];
                inward = (*facet).borderInward[k as usize]
            } else {
                planenum = (*facet).surfacePlane;
                inward = qfalse as libc::c_int
            }
            plane[0usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[0usize];
            plane[1usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[1usize];
            plane[2usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[2usize];
            plane[3usize] =
                (*(*pc).planes.offset(planenum as isize)).plane[3usize];
            if 0 != inward {
                plane[0usize] = vec3_origin[0usize] - plane[0usize];
                plane[1usize] = vec3_origin[1usize] - plane[1usize];
                plane[2usize] = vec3_origin[2usize] - plane[2usize];
                plane[3usize] = -plane[3usize]
            }
            plane[3usize] += (*cv).value;
            n = 0i32;
            while n < 3i32 {
                if plane[n as usize] > 0i32 as libc::c_float {
                    v1[n as usize] = maxs[n as usize]
                } else { v1[n as usize] = mins[n as usize] }
                n += 1
            }
            v2[0usize] = -plane[0usize];
            v2[1usize] = -plane[1usize];
            v2[2usize] = -plane[2usize];
            plane[3usize] =
                (plane[3usize] as libc::c_double +
                     fabs((v1[0usize] * v2[0usize] + v1[1usize] * v2[1usize] +
                               v1[2usize] * v2[2usize]) as libc::c_double)) as
                    libc::c_float;
            w = BaseWindingForPlane(plane.as_mut_ptr(), plane[3usize]);
            j = 0i32;
            while j < (*facet).numBorders + 1i32 && !w.is_null() {
                if j < (*facet).numBorders {
                    curplanenum = (*facet).borderPlanes[j as usize];
                    curinward = (*facet).borderInward[j as usize]
                } else {
                    curplanenum = (*facet).surfacePlane;
                    curinward = qfalse as libc::c_int
                }
                //continue;
                //
                if !(curplanenum == planenum) {
                    plane[0usize] =
                        (*(*pc).planes.offset(curplanenum as
                                                  isize)).plane[0usize];
                    plane[1usize] =
                        (*(*pc).planes.offset(curplanenum as
                                                  isize)).plane[1usize];
                    plane[2usize] =
                        (*(*pc).planes.offset(curplanenum as
                                                  isize)).plane[2usize];
                    plane[3usize] =
                        (*(*pc).planes.offset(curplanenum as
                                                  isize)).plane[3usize];
                    if 0 == curinward {
                        plane[0usize] = vec3_origin[0usize] - plane[0usize];
                        plane[1usize] = vec3_origin[1usize] - plane[1usize];
                        plane[2usize] = vec3_origin[2usize] - plane[2usize];
                        plane[3usize] = -plane[3usize]
                    }
                    plane[3usize] -= (*cv).value;
                    n = 0i32;
                    while n < 3i32 {
                        if plane[n as usize] > 0i32 as libc::c_float {
                            v1[n as usize] = maxs[n as usize]
                        } else { v1[n as usize] = mins[n as usize] }
                        n += 1
                    }
                    v2[0usize] = -plane[0usize];
                    v2[1usize] = -plane[1usize];
                    v2[2usize] = -plane[2usize];
                    plane[3usize] =
                        (plane[3usize] as libc::c_double -
                             fabs((v1[0usize] * v2[0usize] +
                                       v1[1usize] * v2[1usize] +
                                       v1[2usize] * v2[2usize]) as
                                      libc::c_double)) as libc::c_float;
                    ChopWindingInPlace(&mut w, plane.as_mut_ptr(),
                                       plane[3usize], 0.1f32);
                }
                j += 1
            }
            if !w.is_null() {
                if facet == debugFacet as *mut facet_t {
                    drawPoly.expect("non-null function pointer")(4i32,
                                                                 (*w).numpoints,
                                                                 (*w).p[0usize].as_mut_ptr());
                } else {
                    drawPoly.expect("non-null function pointer")(1i32,
                                                                 (*w).numpoints,
                                                                 (*w).p[0usize].as_mut_ptr());
                }
                FreeWinding(w);
            } else {
                Com_Printf(b"winding chopped away by border planes\n\x00" as
                               *const u8 as *const libc::c_char);
            }
            k += 1
        }
        i += 1;
        facet = facet.offset(1isize)
    }
    let mut v: [vec3_t; 3] = [[0.; 3]; 3];
    v[0usize][0usize] = debugBlockPoints[0usize][0usize];
    v[0usize][1usize] = debugBlockPoints[0usize][1usize];
    v[0usize][2usize] = debugBlockPoints[0usize][2usize];
    v[1usize][0usize] = debugBlockPoints[1usize][0usize];
    v[1usize][1usize] = debugBlockPoints[1usize][1usize];
    v[1usize][2usize] = debugBlockPoints[1usize][2usize];
    v[2usize][0usize] = debugBlockPoints[2usize][0usize];
    v[2usize][1usize] = debugBlockPoints[2usize][1usize];
    v[2usize][2usize] = debugBlockPoints[2usize][2usize];
    drawPoly.expect("non-null function pointer")(2i32, 3i32,
                                                 v[0usize].as_mut_ptr());
    v[0usize][0usize] = debugBlockPoints[2usize][0usize];
    v[0usize][1usize] = debugBlockPoints[2usize][1usize];
    v[0usize][2usize] = debugBlockPoints[2usize][2usize];
    v[1usize][0usize] = debugBlockPoints[3usize][0usize];
    v[1usize][1usize] = debugBlockPoints[3usize][1usize];
    v[1usize][2usize] = debugBlockPoints[3usize][2usize];
    v[2usize][0usize] = debugBlockPoints[0usize][0usize];
    v[2usize][1usize] = debugBlockPoints[0usize][1usize];
    v[2usize][2usize] = debugBlockPoints[0usize][2usize];
    drawPoly.expect("non-null function pointer")(2i32, 3i32,
                                                 v[0usize].as_mut_ptr());
}
static mut debugBlockPoints: [vec3_t; 4] = [[0.; 3]; 4];
static mut debugFacet: *const facet_t = 0 as *const facet_t;
static mut debugPatchCollide: *const patchCollide_t =
    0 as *const patchCollide_t;
// cm_patch.c
#[no_mangle]
pub unsafe extern "C" fn CM_GeneratePatchCollide(mut width: libc::c_int,
                                                 mut height: libc::c_int,
                                                 mut points: *mut vec3_t)
 -> *mut patchCollide_s {
    let mut pf: *mut patchCollide_t = 0 as *mut patchCollide_t;
    let mut grid: cGrid_t =
        cGrid_t{width: 0,
                height: 0,
                wrapWidth: qfalse,
                wrapHeight: qfalse,
                points: [[[0.; 3]; 129]; 129],};
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if width <= 2i32 || height <= 2i32 || points.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_GeneratePatchFacets: bad parameters: (%i, %i, %p)\x00"
                      as *const u8 as *const libc::c_char, width, height,
                  points as *mut libc::c_void);
    }
    if 0 == width & 1i32 || 0 == height & 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_GeneratePatchFacets: even sizes are invalid for quadratic meshes\x00"
                      as *const u8 as *const libc::c_char);
    }
    if width > 129i32 || height > 129i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CM_GeneratePatchFacets: source is > MAX_GRID_SIZE\x00" as
                      *const u8 as *const libc::c_char);
    }
    grid.width = width;
    grid.height = height;
    grid.wrapWidth = qfalse;
    grid.wrapHeight = qfalse;
    i = 0i32;
    while i < width {
        j = 0i32;
        while j < height {
            grid.points[i as usize][j as usize][0usize] =
                (*points.offset((j * width + i) as isize))[0usize];
            grid.points[i as usize][j as usize][1usize] =
                (*points.offset((j * width + i) as isize))[1usize];
            grid.points[i as usize][j as usize][2usize] =
                (*points.offset((j * width + i) as isize))[2usize];
            j += 1
        }
        i += 1
    }
    CM_SetGridWrapWidth(&mut grid);
    CM_SubdivideGridColumns(&mut grid);
    CM_RemoveDegenerateColumns(&mut grid);
    CM_TransposeGrid(&mut grid);
    CM_SetGridWrapWidth(&mut grid);
    CM_SubdivideGridColumns(&mut grid);
    CM_RemoveDegenerateColumns(&mut grid);
    pf =
        Hunk_AllocDebug(::std::mem::size_of::<patchCollide_t>() as
                            libc::c_ulong as libc::c_int, h_high,
                        b"sizeof( *pf )\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_patch.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 1219i32)
            as *mut patchCollide_t;
    ClearBounds((*pf).bounds[0usize].as_mut_ptr(),
                (*pf).bounds[1usize].as_mut_ptr());
    i = 0i32;
    while i < grid.width {
        j = 0i32;
        while j < grid.height {
            AddPointToBounds(grid.points[i as usize][j as usize].as_mut_ptr()
                                 as *const vec_t,
                             (*pf).bounds[0usize].as_mut_ptr(),
                             (*pf).bounds[1usize].as_mut_ptr());
            j += 1
        }
        i += 1
    }
    c_totalPatchBlocks += (grid.width - 1i32) * (grid.height - 1i32);
    CM_PatchCollideFromGrid(&mut grid, pf);
    (*pf).bounds[0usize][0usize] -= 1i32 as libc::c_float;
    (*pf).bounds[0usize][1usize] -= 1i32 as libc::c_float;
    (*pf).bounds[0usize][2usize] -= 1i32 as libc::c_float;
    (*pf).bounds[1usize][0usize] += 1i32 as libc::c_float;
    (*pf).bounds[1usize][1usize] += 1i32 as libc::c_float;
    (*pf).bounds[1usize][2usize] += 1i32 as libc::c_float;
    return pf;
}
/*
==================
CM_PatchCollideFromGrid
==================
*/
unsafe extern "C" fn CM_PatchCollideFromGrid(mut grid: *mut cGrid_t,
                                             mut pf: *mut patchCollide_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p3: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut gridPlanes: [[[libc::c_int; 2]; 129]; 129] = [[[0; 2]; 129]; 129];
    let mut facet: *mut facet_t = 0 as *mut facet_t;
    let mut borders: [libc::c_int; 4] = [0; 4];
    let mut noAdjust: [libc::c_int; 4] = [0; 4];
    numPlanes = 0i32;
    numFacets = 0i32;
    i = 0i32;
    while i < (*grid).width - 1i32 {
        j = 0i32;
        while j < (*grid).height - 1i32 {
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            p3 =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            gridPlanes[i as usize][j as usize][0usize] =
                CM_FindPlane(p1, p2, p3);
            p1 =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            p2 = (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            p3 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            gridPlanes[i as usize][j as usize][1usize] =
                CM_FindPlane(p1, p2, p3);
            j += 1
        }
        i += 1
    }
    i = 0i32;
    while i < (*grid).width - 1i32 {
        j = 0i32;
        while j < (*grid).height - 1i32 {
            borders[EN_TOP as libc::c_int as usize] = -1i32;
            if j > 0i32 {
                borders[EN_TOP as libc::c_int as usize] =
                    gridPlanes[i as usize][(j - 1i32) as usize][1usize]
            } else if 0 != (*grid).wrapHeight as u64 {
                borders[EN_TOP as libc::c_int as usize] =
                    gridPlanes[i as
                                   usize][((*grid).height - 2i32) as
                                              usize][1usize]
            }
            noAdjust[EN_TOP as libc::c_int as usize] =
                (borders[EN_TOP as libc::c_int as usize] ==
                     gridPlanes[i as usize][j as usize][0usize]) as
                    libc::c_int;
            if borders[EN_TOP as libc::c_int as usize] == -1i32 ||
                   0 != noAdjust[EN_TOP as libc::c_int as usize] {
                borders[EN_TOP as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 0i32)
            }
            borders[EN_BOTTOM as libc::c_int as usize] = -1i32;
            if j < (*grid).height - 2i32 {
                borders[EN_BOTTOM as libc::c_int as usize] =
                    gridPlanes[i as usize][(j + 1i32) as usize][0usize]
            } else if 0 != (*grid).wrapHeight as u64 {
                borders[EN_BOTTOM as libc::c_int as usize] =
                    gridPlanes[i as usize][0usize][0usize]
            }
            noAdjust[EN_BOTTOM as libc::c_int as usize] =
                (borders[EN_BOTTOM as libc::c_int as usize] ==
                     gridPlanes[i as usize][j as usize][1usize]) as
                    libc::c_int;
            if borders[EN_BOTTOM as libc::c_int as usize] == -1i32 ||
                   0 != noAdjust[EN_BOTTOM as libc::c_int as usize] {
                borders[EN_BOTTOM as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 2i32)
            }
            borders[EN_LEFT as libc::c_int as usize] = -1i32;
            if i > 0i32 {
                borders[EN_LEFT as libc::c_int as usize] =
                    gridPlanes[(i - 1i32) as usize][j as usize][0usize]
            } else if 0 != (*grid).wrapWidth as u64 {
                borders[EN_LEFT as libc::c_int as usize] =
                    gridPlanes[((*grid).width - 2i32) as
                                   usize][j as usize][0usize]
            }
            noAdjust[EN_LEFT as libc::c_int as usize] =
                (borders[EN_LEFT as libc::c_int as usize] ==
                     gridPlanes[i as usize][j as usize][1usize]) as
                    libc::c_int;
            if borders[EN_LEFT as libc::c_int as usize] == -1i32 ||
                   0 != noAdjust[EN_LEFT as libc::c_int as usize] {
                borders[EN_LEFT as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 3i32)
            }
            borders[EN_RIGHT as libc::c_int as usize] = -1i32;
            if i < (*grid).width - 2i32 {
                borders[EN_RIGHT as libc::c_int as usize] =
                    gridPlanes[(i + 1i32) as usize][j as usize][1usize]
            } else if 0 != (*grid).wrapWidth as u64 {
                borders[EN_RIGHT as libc::c_int as usize] =
                    gridPlanes[0usize][j as usize][1usize]
            }
            noAdjust[EN_RIGHT as libc::c_int as usize] =
                (borders[EN_RIGHT as libc::c_int as usize] ==
                     gridPlanes[i as usize][j as usize][0usize]) as
                    libc::c_int;
            if borders[EN_RIGHT as libc::c_int as usize] == -1i32 ||
                   0 != noAdjust[EN_RIGHT as libc::c_int as usize] {
                borders[EN_RIGHT as libc::c_int as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 1i32)
            }
            if numFacets == 1024i32 {
                Com_Error(ERR_DROP as libc::c_int,
                          b"MAX_FACETS\x00" as *const u8 as
                              *const libc::c_char);
            }
            facet = &mut facets[numFacets as usize] as *mut facet_t;
            memset(facet as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<facet_t>() as libc::c_ulong);
            if gridPlanes[i as usize][j as usize][0usize] ==
                   gridPlanes[i as usize][j as usize][1usize] {
                if !(gridPlanes[i as usize][j as usize][0usize] == -1i32) {
                    // degenrate
                    (*facet).surfacePlane =
                        gridPlanes[i as usize][j as usize][0usize];
                    (*facet).numBorders = 4i32;
                    (*facet).borderPlanes[0usize] =
                        borders[EN_TOP as libc::c_int as usize];
                    (*facet).borderNoAdjust[0usize] =
                        noAdjust[EN_TOP as libc::c_int as usize] as qboolean;
                    (*facet).borderPlanes[1usize] =
                        borders[EN_RIGHT as libc::c_int as usize];
                    (*facet).borderNoAdjust[1usize] =
                        noAdjust[EN_RIGHT as libc::c_int as usize] as
                            qboolean;
                    (*facet).borderPlanes[2usize] =
                        borders[EN_BOTTOM as libc::c_int as usize];
                    (*facet).borderNoAdjust[2usize] =
                        noAdjust[EN_BOTTOM as libc::c_int as usize] as
                            qboolean;
                    (*facet).borderPlanes[3usize] =
                        borders[EN_LEFT as libc::c_int as usize];
                    (*facet).borderNoAdjust[3usize] =
                        noAdjust[EN_LEFT as libc::c_int as usize] as qboolean;
                    CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(),
                                       i, j, -1i32);
                    if 0 != CM_ValidateFacet(facet) as u64 {
                        CM_AddFacetBevels(facet);
                        numFacets += 1
                    }
                }
            } else {
                (*facet).surfacePlane =
                    gridPlanes[i as usize][j as usize][0usize];
                (*facet).numBorders = 3i32;
                (*facet).borderPlanes[0usize] =
                    borders[EN_TOP as libc::c_int as usize];
                (*facet).borderNoAdjust[0usize] =
                    noAdjust[EN_TOP as libc::c_int as usize] as qboolean;
                (*facet).borderPlanes[1usize] =
                    borders[EN_RIGHT as libc::c_int as usize];
                (*facet).borderNoAdjust[1usize] =
                    noAdjust[EN_RIGHT as libc::c_int as usize] as qboolean;
                (*facet).borderPlanes[2usize] =
                    gridPlanes[i as usize][j as usize][1usize];
                if (*facet).borderPlanes[2usize] == -1i32 {
                    (*facet).borderPlanes[2usize] =
                        borders[EN_BOTTOM as libc::c_int as usize];
                    if (*facet).borderPlanes[2usize] == -1i32 {
                        (*facet).borderPlanes[2usize] =
                            CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i,
                                            j, 4i32)
                    }
                }
                CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(), i, j,
                                   0i32);
                if 0 != CM_ValidateFacet(facet) as u64 {
                    CM_AddFacetBevels(facet);
                    numFacets += 1
                }
                if numFacets == 1024i32 {
                    Com_Error(ERR_DROP as libc::c_int,
                              b"MAX_FACETS\x00" as *const u8 as
                                  *const libc::c_char);
                }
                facet = &mut facets[numFacets as usize] as *mut facet_t;
                memset(facet as *mut libc::c_void, 0i32,
                       ::std::mem::size_of::<facet_t>() as libc::c_ulong);
                (*facet).surfacePlane =
                    gridPlanes[i as usize][j as usize][1usize];
                (*facet).numBorders = 3i32;
                (*facet).borderPlanes[0usize] =
                    borders[EN_BOTTOM as libc::c_int as usize];
                (*facet).borderNoAdjust[0usize] =
                    noAdjust[EN_BOTTOM as libc::c_int as usize] as qboolean;
                (*facet).borderPlanes[1usize] =
                    borders[EN_LEFT as libc::c_int as usize];
                (*facet).borderNoAdjust[1usize] =
                    noAdjust[EN_LEFT as libc::c_int as usize] as qboolean;
                (*facet).borderPlanes[2usize] =
                    gridPlanes[i as usize][j as usize][0usize];
                if (*facet).borderPlanes[2usize] == -1i32 {
                    (*facet).borderPlanes[2usize] =
                        borders[EN_TOP as libc::c_int as usize];
                    if (*facet).borderPlanes[2usize] == -1i32 {
                        (*facet).borderPlanes[2usize] =
                            CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i,
                                            j, 5i32)
                    }
                }
                CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(), i, j,
                                   1i32);
                if 0 != CM_ValidateFacet(facet) as u64 {
                    CM_AddFacetBevels(facet);
                    numFacets += 1
                }
            }
            j += 1
        }
        i += 1
    }
    (*pf).numPlanes = numPlanes;
    (*pf).numFacets = numFacets;
    (*pf).facets =
        Hunk_AllocDebug((numFacets as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<facet_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"numFacets * sizeof( *pf->facets )\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_patch.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 1159i32)
            as *mut facet_t;
    memcpy((*pf).facets as *mut libc::c_void,
           facets.as_mut_ptr() as *const libc::c_void,
           (numFacets as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<facet_t>()
                                                as libc::c_ulong));
    (*pf).planes =
        Hunk_AllocDebug((numPlanes as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<patchPlane_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"numPlanes * sizeof( *pf->planes )\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/cm_patch.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 1161i32)
            as *mut patchPlane_t;
    memcpy((*pf).planes as *mut libc::c_void,
           planes.as_mut_ptr() as *const libc::c_void,
           (numPlanes as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<patchPlane_t>()
                                                as libc::c_ulong));
}
/*
================================================================================

PATCH COLLIDE GENERATION

================================================================================
*/
static mut numPlanes: libc::c_int = 0;
static mut planes: [patchPlane_t; 2048] =
    [patchPlane_t{plane: [0.; 4], signbits: 0,}; 2048];
static mut numFacets: libc::c_int = 0;
static mut facets: [facet_t; 1024] =
    [facet_t{surfacePlane: 0,
             numBorders: 0,
             borderPlanes: [0; 26],
             borderInward: [0; 26],
             borderNoAdjust: [qfalse; 26],}; 1024];
/*
==================
CM_AddFacetBevels
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_AddFacetBevels(mut facet: *mut facet_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut axis: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut flipped: libc::c_int = 0;
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut d: libc::c_float = 0.;
    let mut newplane: [libc::c_float; 4] = [0.; 4];
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut w2: *mut winding_t = 0 as *mut winding_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut vec2: vec3_t = [0.; 3];
    plane[0usize] = planes[(*facet).surfacePlane as usize].plane[0usize];
    plane[1usize] = planes[(*facet).surfacePlane as usize].plane[1usize];
    plane[2usize] = planes[(*facet).surfacePlane as usize].plane[2usize];
    plane[3usize] = planes[(*facet).surfacePlane as usize].plane[3usize];
    w = BaseWindingForPlane(plane.as_mut_ptr(), plane[3usize]);
    j = 0i32;
    while j < (*facet).numBorders && !w.is_null() {
        if !((*facet).borderPlanes[j as usize] == (*facet).surfacePlane) {
            plane[0usize] =
                planes[(*facet).borderPlanes[j as usize] as
                           usize].plane[0usize];
            plane[1usize] =
                planes[(*facet).borderPlanes[j as usize] as
                           usize].plane[1usize];
            plane[2usize] =
                planes[(*facet).borderPlanes[j as usize] as
                           usize].plane[2usize];
            plane[3usize] =
                planes[(*facet).borderPlanes[j as usize] as
                           usize].plane[3usize];
            if 0 == (*facet).borderInward[j as usize] {
                plane[0usize] = vec3_origin[0usize] - plane[0usize];
                plane[1usize] = vec3_origin[1usize] - plane[1usize];
                plane[2usize] = vec3_origin[2usize] - plane[2usize];
                plane[3usize] = -plane[3usize]
            }
            ChopWindingInPlace(&mut w, plane.as_mut_ptr(), plane[3usize],
                               0.1f32);
        }
        j += 1
    }
    if w.is_null() { return }
    WindingBounds(w, mins.as_mut_ptr(), maxs.as_mut_ptr());
    axis = 0i32;
    while axis < 3i32 {
        dir = -1i32;
        while dir <= 1i32 {
            plane[2usize] = 0i32 as libc::c_float;
            plane[1usize] = plane[2usize];
            plane[0usize] = plane[1usize];
            plane[axis as usize] = dir as libc::c_float;
            if dir == 1i32 {
                plane[3usize] = maxs[axis as usize]
            } else { plane[3usize] = -mins[axis as usize] }
            //if it's the surface plane
            if !(0 !=
                     CM_PlaneEqual(&mut planes[(*facet).surfacePlane as
                                                   usize], plane.as_mut_ptr(),
                                   &mut flipped)) {
                i = 0i32;
                while i < (*facet).numBorders {
                    if 0 !=
                           CM_PlaneEqual(&mut planes[(*facet).borderPlanes[i
                                                                               as
                                                                               usize]
                                                         as usize],
                                         plane.as_mut_ptr(), &mut flipped) {
                        break ;
                    }
                    i += 1
                }
                if i == (*facet).numBorders {
                    if (*facet).numBorders >= 4i32 + 6i32 + 16i32 {
                        Com_Printf(b"ERROR: too many bevels\n\x00" as
                                       *const u8 as *const libc::c_char);
                    } else {
                        (*facet).borderPlanes[(*facet).numBorders as usize] =
                            CM_FindPlane2(plane.as_mut_ptr(), &mut flipped);
                        (*facet).borderNoAdjust[(*facet).numBorders as usize]
                            = qfalse;
                        (*facet).borderInward[(*facet).numBorders as usize] =
                            flipped;
                        (*facet).numBorders += 1
                    }
                }
            }
            dir += 2i32
        }
        axis += 1
    }
    j = 0i32;
    while j < (*w).numpoints {
        k = (j + 1i32) % (*w).numpoints;
        vec[0usize] = (*w).p[j as usize][0usize] - (*w).p[k as usize][0usize];
        vec[1usize] = (*w).p[j as usize][1usize] - (*w).p[k as usize][1usize];
        vec[2usize] = (*w).p[j as usize][2usize] - (*w).p[k as usize][2usize];
        //if it's a degenerate edge
        if !((VectorNormalize(vec.as_mut_ptr()) as libc::c_double) < 0.5f64) {
            CM_SnapVector(vec.as_mut_ptr());
            k = 0i32;
            while k < 3i32 {
                if vec[k as usize] == -1i32 as libc::c_float ||
                       vec[k as usize] == 1i32 as libc::c_float {
                    // axial
                    break ;
                } else { k += 1 }
            }
            if !(k < 3i32) {
                // only test non-axial edges
                axis = 0i32;
                while axis < 3i32 {
                    dir = -1i32;
                    while dir <= 1i32 {
                        vec2[2usize] = 0i32 as vec_t;
                        vec2[1usize] = vec2[2usize];
                        vec2[0usize] = vec2[1usize];
                        vec2[axis as usize] = dir as vec_t;
                        CrossProduct(vec.as_mut_ptr() as *const vec_t,
                                     vec2.as_mut_ptr() as *const vec_t,
                                     plane.as_mut_ptr());
                        if !((VectorNormalize(plane.as_mut_ptr()) as
                                  libc::c_double) < 0.5f64) {
                            plane[3usize] =
                                (*w).p[j as usize][0usize] * plane[0usize] +
                                    (*w).p[j as usize][1usize] * plane[1usize]
                                    +
                                    (*w).p[j as usize][2usize] *
                                        plane[2usize];
                            l = 0i32;
                            while l < (*w).numpoints {
                                d =
                                    (*w).p[l as usize][0usize] * plane[0usize]
                                        +
                                        (*w).p[l as usize][1usize] *
                                            plane[1usize] +
                                        (*w).p[l as usize][2usize] *
                                            plane[2usize] - plane[3usize];
                                if d as libc::c_double > 0.1f64 {
                                    // point in front
                                    break ;
                                } else { l += 1 }
                            }
                            if !(l < (*w).numpoints) {
                                //if it's the surface plane
                                if !(0 !=
                                         CM_PlaneEqual(&mut planes[(*facet).surfacePlane
                                                                       as
                                                                       usize],
                                                       plane.as_mut_ptr(),
                                                       &mut flipped)) {
                                    i = 0i32;
                                    while i < (*facet).numBorders {
                                        if 0 !=
                                               CM_PlaneEqual(&mut planes[(*facet).borderPlanes[i
                                                                                                   as
                                                                                                   usize]
                                                                             as
                                                                             usize],
                                                             plane.as_mut_ptr(),
                                                             &mut flipped) {
                                            break ;
                                        }
                                        i += 1
                                    }
                                    if i == (*facet).numBorders {
                                        if (*facet).numBorders >=
                                               4i32 + 6i32 + 16i32 {
                                            Com_Printf(b"ERROR: too many bevels\n\x00"
                                                           as *const u8 as
                                                           *const libc::c_char);
                                        } else {
                                            (*facet).borderPlanes[(*facet).numBorders
                                                                      as
                                                                      usize] =
                                                CM_FindPlane2(plane.as_mut_ptr(),
                                                              &mut flipped);
                                            k = 0i32;
                                            while k < (*facet).numBorders {
                                                if (*facet).borderPlanes[(*facet).numBorders
                                                                             as
                                                                             usize]
                                                       ==
                                                       (*facet).borderPlanes[k
                                                                                 as
                                                                                 usize]
                                                   {
                                                    Com_Printf(b"WARNING: bevel plane already used\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
                                                }
                                                k += 1
                                            }
                                            (*facet).borderNoAdjust[(*facet).numBorders
                                                                        as
                                                                        usize]
                                                = qfalse;
                                            (*facet).borderInward[(*facet).numBorders
                                                                      as
                                                                      usize] =
                                                flipped;
                                            w2 = CopyWinding(w);
                                            newplane[0usize] =
                                                planes[(*facet).borderPlanes[(*facet).numBorders
                                                                                 as
                                                                                 usize]
                                                           as
                                                           usize].plane[0usize];
                                            newplane[1usize] =
                                                planes[(*facet).borderPlanes[(*facet).numBorders
                                                                                 as
                                                                                 usize]
                                                           as
                                                           usize].plane[1usize];
                                            newplane[2usize] =
                                                planes[(*facet).borderPlanes[(*facet).numBorders
                                                                                 as
                                                                                 usize]
                                                           as
                                                           usize].plane[2usize];
                                            newplane[3usize] =
                                                planes[(*facet).borderPlanes[(*facet).numBorders
                                                                                 as
                                                                                 usize]
                                                           as
                                                           usize].plane[3usize];
                                            if 0 ==
                                                   (*facet).borderInward[(*facet).numBorders
                                                                             as
                                                                             usize]
                                               {
                                                newplane[0usize] =
                                                    -newplane[0usize];
                                                newplane[1usize] =
                                                    -newplane[1usize];
                                                newplane[2usize] =
                                                    -newplane[2usize];
                                                newplane[3usize] =
                                                    -newplane[3usize]
                                            }
                                            ChopWindingInPlace(&mut w2,
                                                               newplane.as_mut_ptr(),
                                                               newplane[3usize],
                                                               0.1f32);
                                            if w2.is_null() {
                                                Com_DPrintf(b"WARNING: CM_AddFacetBevels... invalid bevel\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char);
                                            } else {
                                                FreeWinding(w2);
                                                (*facet).numBorders += 1
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        dir += 2i32
                    }
                    axis += 1
                }
            }
        }
        j += 1
    }
    FreeWinding(w);
    if (*facet).numBorders >= 4i32 + 6i32 + 16i32 {
        Com_Printf(b"ERROR: too many bevels\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    (*facet).borderPlanes[(*facet).numBorders as usize] =
        (*facet).surfacePlane;
    (*facet).borderNoAdjust[(*facet).numBorders as usize] = qfalse;
    (*facet).borderInward[(*facet).numBorders as usize] =
        qtrue as libc::c_int;
    (*facet).numBorders += 1;
}
/*
==================
CM_FindPlane2
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_FindPlane2(mut plane: *mut libc::c_float,
                                       mut flipped: *mut libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < numPlanes {
        if 0 != CM_PlaneEqual(&mut planes[i as usize], plane, flipped) {
            return i
        }
        i += 1
    }
    if numPlanes == 2048i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MAX_PATCH_PLANES\x00" as *const u8 as
                      *const libc::c_char);
    }
    planes[numPlanes as usize].plane[0usize] = *plane.offset(0isize);
    planes[numPlanes as usize].plane[1usize] = *plane.offset(1isize);
    planes[numPlanes as usize].plane[2usize] = *plane.offset(2isize);
    planes[numPlanes as usize].plane[3usize] = *plane.offset(3isize);
    planes[numPlanes as usize].signbits = CM_SignbitsForNormal(plane);
    numPlanes += 1;
    *flipped = qfalse as libc::c_int;
    return numPlanes - 1i32;
}
/*
=================
CM_SignbitsForNormal
=================
*/
unsafe extern "C" fn CM_SignbitsForNormal(mut normal: *mut vec_t)
 -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    bits = 0i32;
    j = 0i32;
    while j < 3i32 {
        if *normal.offset(j as isize) < 0i32 as libc::c_float {
            bits |= 1i32 << j
        }
        j += 1
    }
    return bits;
}
/*
==================
CM_PlaneEqual
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_PlaneEqual(mut p: *mut patchPlane_t,
                                       mut plane: *mut libc::c_float,
                                       mut flipped: *mut libc::c_int)
 -> libc::c_int {
    let mut invplane: [libc::c_float; 4] = [0.; 4];
    if fabs(((*p).plane[0usize] - *plane.offset(0isize)) as libc::c_double) <
           0.0001f64 &&
           fabs(((*p).plane[1usize] - *plane.offset(1isize)) as
                    libc::c_double) < 0.0001f64 &&
           fabs(((*p).plane[2usize] - *plane.offset(2isize)) as
                    libc::c_double) < 0.0001f64 &&
           fabs(((*p).plane[3usize] - *plane.offset(3isize)) as
                    libc::c_double) < 0.02f64 {
        *flipped = qfalse as libc::c_int;
        return qtrue as libc::c_int
    }
    invplane[0usize] = -*plane.offset(0isize);
    invplane[1usize] = -*plane.offset(1isize);
    invplane[2usize] = -*plane.offset(2isize);
    invplane[3usize] = -*plane.offset(3isize);
    if fabs(((*p).plane[0usize] - invplane[0usize]) as libc::c_double) <
           0.0001f64 &&
           fabs(((*p).plane[1usize] - invplane[1usize]) as libc::c_double) <
               0.0001f64 &&
           fabs(((*p).plane[2usize] - invplane[2usize]) as libc::c_double) <
               0.0001f64 &&
           fabs(((*p).plane[3usize] - invplane[3usize]) as libc::c_double) <
               0.02f64 {
        *flipped = qtrue as libc::c_int;
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
/*
==================
CM_SnapVector
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_SnapVector(mut normal: *mut vec_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        if fabs((*normal.offset(i as isize) - 1i32 as libc::c_float) as
                    libc::c_double) < 0.0001f64 {
            let ref mut fresh1 = *normal.offset(1isize);
            let ref mut fresh0 = *normal.offset(2isize);
            *fresh0 = 0i32 as vec_t;
            *fresh1 = *fresh0;
            *normal.offset(0isize) = *fresh1;
            *normal.offset(i as isize) = 1i32 as vec_t;
            break ;
        } else if fabs((*normal.offset(i as isize) - -1i32 as libc::c_float)
                           as libc::c_double) < 0.0001f64 {
            let ref mut fresh3 = *normal.offset(1isize);
            let ref mut fresh2 = *normal.offset(2isize);
            *fresh2 = 0i32 as vec_t;
            *fresh3 = *fresh2;
            *normal.offset(0isize) = *fresh3;
            *normal.offset(i as isize) = -1i32 as vec_t;
            break ;
        } else { i += 1 }
    };
}
/*
==================
CM_ValidateFacet

If the facet isn't bounded by its borders, we screwed up.
==================
*/
unsafe extern "C" fn CM_ValidateFacet(mut facet: *mut facet_t) -> qboolean {
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut j: libc::c_int = 0;
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut bounds: [vec3_t; 2] = [[0.; 3]; 2];
    if (*facet).surfacePlane == -1i32 { return qfalse }
    plane[0usize] = planes[(*facet).surfacePlane as usize].plane[0usize];
    plane[1usize] = planes[(*facet).surfacePlane as usize].plane[1usize];
    plane[2usize] = planes[(*facet).surfacePlane as usize].plane[2usize];
    plane[3usize] = planes[(*facet).surfacePlane as usize].plane[3usize];
    w = BaseWindingForPlane(plane.as_mut_ptr(), plane[3usize]);
    j = 0i32;
    while j < (*facet).numBorders && !w.is_null() {
        if (*facet).borderPlanes[j as usize] == -1i32 {
            FreeWinding(w);
            return qfalse
        }
        plane[0usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[0usize];
        plane[1usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[1usize];
        plane[2usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[2usize];
        plane[3usize] =
            planes[(*facet).borderPlanes[j as usize] as usize].plane[3usize];
        if 0 == (*facet).borderInward[j as usize] {
            plane[0usize] = vec3_origin[0usize] - plane[0usize];
            plane[1usize] = vec3_origin[1usize] - plane[1usize];
            plane[2usize] = vec3_origin[2usize] - plane[2usize];
            plane[3usize] = -plane[3usize]
        }
        ChopWindingInPlace(&mut w, plane.as_mut_ptr(), plane[3usize], 0.1f32);
        j += 1
    }
    if w.is_null() { return qfalse }
    WindingBounds(w, bounds[0usize].as_mut_ptr(),
                  bounds[1usize].as_mut_ptr());
    FreeWinding(w);
    j = 0i32;
    while j < 3i32 {
        if bounds[1usize][j as usize] - bounds[0usize][j as usize] >
               65535i32 as libc::c_float {
            return qfalse
        }
        if bounds[0usize][j as usize] >= 65535i32 as libc::c_float {
            return qfalse
        }
        if bounds[1usize][j as usize] <= -65535i32 as libc::c_float {
            return qfalse
        }
        j += 1
    }
    return qtrue;
}
/*
===================
CM_SetBorderInward
===================
*/
unsafe extern "C" fn CM_SetBorderInward(mut facet: *mut facet_t,
                                        mut grid: *mut cGrid_t,
                                        mut gridPlanes:
                                            *mut [[libc::c_int; 2]; 129],
                                        mut i: libc::c_int,
                                        mut j: libc::c_int,
                                        mut which: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut points: [*mut libc::c_float; 4] = [0 as *mut libc::c_float; 4];
    let mut numPoints: libc::c_int = 0;
    match which {
        -1 => {
            points[0usize] =
                (*grid).points[i as usize][j as usize].as_mut_ptr();
            points[1usize] =
                (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            points[2usize] =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            points[3usize] =
                (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            numPoints = 4i32
        }
        0 => {
            points[0usize] =
                (*grid).points[i as usize][j as usize].as_mut_ptr();
            points[1usize] =
                (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            points[2usize] =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            numPoints = 3i32
        }
        1 => {
            points[0usize] =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            points[1usize] =
                (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            points[2usize] =
                (*grid).points[i as usize][j as usize].as_mut_ptr();
            numPoints = 3i32
        }
        _ => {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"CM_SetBorderInward: bad parameter\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    k = 0i32;
    while k < (*facet).numBorders {
        let mut front: libc::c_int = 0;
        let mut back: libc::c_int = 0;
        front = 0i32;
        back = 0i32;
        l = 0i32;
        while l < numPoints {
            let mut side: libc::c_int = 0;
            side =
                CM_PointOnPlaneSide(points[l as usize],
                                    (*facet).borderPlanes[k as usize]);
            if side == 0i32 { front += 1 }
            if side == 1i32 { back += 1 }
            l += 1
        }
        if 0 != front && 0 == back {
            (*facet).borderInward[k as usize] = qtrue as libc::c_int
        } else if 0 != back && 0 == front {
            (*facet).borderInward[k as usize] = qfalse as libc::c_int
        } else if 0 == front && 0 == back {
            (*facet).borderPlanes[k as usize] = -1i32
        } else {
            Com_DPrintf(b"WARNING: CM_SetBorderInward: mixed plane sides\n\x00"
                            as *const u8 as *const libc::c_char);
            (*facet).borderInward[k as usize] = qfalse as libc::c_int;
            if 0 == debugBlock as u64 {
                debugBlock = qtrue;
                debugBlockPoints[0usize][0usize] =
                    (*grid).points[i as usize][j as usize][0usize];
                debugBlockPoints[0usize][1usize] =
                    (*grid).points[i as usize][j as usize][1usize];
                debugBlockPoints[0usize][2usize] =
                    (*grid).points[i as usize][j as usize][2usize];
                debugBlockPoints[1usize][0usize] =
                    (*grid).points[(i + 1i32) as usize][j as usize][0usize];
                debugBlockPoints[1usize][1usize] =
                    (*grid).points[(i + 1i32) as usize][j as usize][1usize];
                debugBlockPoints[1usize][2usize] =
                    (*grid).points[(i + 1i32) as usize][j as usize][2usize];
                debugBlockPoints[2usize][0usize] =
                    (*grid).points[(i + 1i32) as
                                       usize][(j + 1i32) as usize][0usize];
                debugBlockPoints[2usize][1usize] =
                    (*grid).points[(i + 1i32) as
                                       usize][(j + 1i32) as usize][1usize];
                debugBlockPoints[2usize][2usize] =
                    (*grid).points[(i + 1i32) as
                                       usize][(j + 1i32) as usize][2usize];
                debugBlockPoints[3usize][0usize] =
                    (*grid).points[i as usize][(j + 1i32) as usize][0usize];
                debugBlockPoints[3usize][1usize] =
                    (*grid).points[i as usize][(j + 1i32) as usize][1usize];
                debugBlockPoints[3usize][2usize] =
                    (*grid).points[i as usize][(j + 1i32) as usize][2usize]
            }
        }
        k += 1
    };
}
static mut debugBlock: qboolean = qfalse;
/*
==================
CM_PointOnPlaneSide
==================
*/
unsafe extern "C" fn CM_PointOnPlaneSide(mut p: *mut libc::c_float,
                                         mut planeNum: libc::c_int)
 -> libc::c_int {
    let mut plane: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut d: libc::c_float = 0.;
    if planeNum == -1i32 { return 2i32 }
    plane = planes[planeNum as usize].plane.as_mut_ptr();
    d =
        *p.offset(0isize) * *plane.offset(0isize) +
            *p.offset(1isize) * *plane.offset(1isize) +
            *p.offset(2isize) * *plane.offset(2isize) - *plane.offset(3isize);
    if d as libc::c_double > 0.1f64 { return 0i32 }
    if (d as libc::c_double) < -0.1f64 { return 1i32 }
    return 2i32;
}
/*
==================
CM_EdgePlaneNum
==================
*/
unsafe extern "C" fn CM_EdgePlaneNum(mut grid: *mut cGrid_t,
                                     mut gridPlanes:
                                         *mut [[libc::c_int; 2]; 129],
                                     mut i: libc::c_int, mut j: libc::c_int,
                                     mut k: libc::c_int) -> libc::c_int {
    let mut p1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p2: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut up: vec3_t = [0.; 3];
    let mut p: libc::c_int = 0;
    match k {
        0 => {
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0i32);
            if p == -1i32 { return -1i32 }
            up[0usize] =
                *p1.offset(0isize) +
                    planes[p as usize].plane[0usize] * 4i32 as libc::c_float;
            up[1usize] =
                *p1.offset(1isize) +
                    planes[p as usize].plane[1usize] * 4i32 as libc::c_float;
            up[2usize] =
                *p1.offset(2isize) +
                    planes[p as usize].plane[2usize] * 4i32 as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr())
        }
        2 => {
            p1 = (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            p2 =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1i32);
            if p == -1i32 { return -1i32 }
            up[0usize] =
                *p1.offset(0isize) +
                    planes[p as usize].plane[0usize] * 4i32 as libc::c_float;
            up[1usize] =
                *p1.offset(1isize) +
                    planes[p as usize].plane[1usize] * 4i32 as libc::c_float;
            up[2usize] =
                *p1.offset(2isize) +
                    planes[p as usize].plane[2usize] * 4i32 as libc::c_float;
            return CM_FindPlane(p2, p1, up.as_mut_ptr())
        }
        3 => {
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1i32);
            if p == -1i32 { return -1i32 }
            up[0usize] =
                *p1.offset(0isize) +
                    planes[p as usize].plane[0usize] * 4i32 as libc::c_float;
            up[1usize] =
                *p1.offset(1isize) +
                    planes[p as usize].plane[1usize] * 4i32 as libc::c_float;
            up[2usize] =
                *p1.offset(2isize) +
                    planes[p as usize].plane[2usize] * 4i32 as libc::c_float;
            return CM_FindPlane(p2, p1, up.as_mut_ptr())
        }
        1 => {
            p1 = (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            p2 =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0i32);
            if p == -1i32 { return -1i32 }
            up[0usize] =
                *p1.offset(0isize) +
                    planes[p as usize].plane[0usize] * 4i32 as libc::c_float;
            up[1usize] =
                *p1.offset(1isize) +
                    planes[p as usize].plane[1usize] * 4i32 as libc::c_float;
            up[2usize] =
                *p1.offset(2isize) +
                    planes[p as usize].plane[2usize] * 4i32 as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr())
        }
        4 => {
            p1 =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            p2 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0i32);
            if p == -1i32 { return -1i32 }
            up[0usize] =
                *p1.offset(0isize) +
                    planes[p as usize].plane[0usize] * 4i32 as libc::c_float;
            up[1usize] =
                *p1.offset(1isize) +
                    planes[p as usize].plane[1usize] * 4i32 as libc::c_float;
            up[2usize] =
                *p1.offset(2isize) +
                    planes[p as usize].plane[2usize] * 4i32 as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr())
        }
        5 => {
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 =
                (*grid).points[(i + 1i32) as
                                   usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1i32);
            if p == -1i32 { return -1i32 }
            up[0usize] =
                *p1.offset(0isize) +
                    planes[p as usize].plane[0usize] * 4i32 as libc::c_float;
            up[1usize] =
                *p1.offset(1isize) +
                    planes[p as usize].plane[1usize] * 4i32 as libc::c_float;
            up[2usize] =
                *p1.offset(2isize) +
                    planes[p as usize].plane[2usize] * 4i32 as libc::c_float;
            return CM_FindPlane(p1, p2, up.as_mut_ptr())
        }
        _ => { }
    }
    Com_Error(ERR_DROP as libc::c_int,
              b"CM_EdgePlaneNum: bad k\x00" as *const u8 as
                  *const libc::c_char);
}
/*
==================
CM_FindPlane
==================
*/
unsafe extern "C" fn CM_FindPlane(mut p1: *mut libc::c_float,
                                  mut p2: *mut libc::c_float,
                                  mut p3: *mut libc::c_float) -> libc::c_int {
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut i: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    if 0 == CM_PlaneFromPoints(plane.as_mut_ptr(), p1, p2, p3) as u64 {
        return -1i32
    }
    i = 0i32;
    while i < numPlanes {
        if !(plane[0usize] * planes[i as usize].plane[0usize] +
                 plane[1usize] * planes[i as usize].plane[1usize] +
                 plane[2usize] * planes[i as usize].plane[2usize] <
                 0i32 as libc::c_float) {
            // allow backwards planes?
            d =
                *p1.offset(0isize) * planes[i as usize].plane[0usize] +
                    *p1.offset(1isize) * planes[i as usize].plane[1usize] +
                    *p1.offset(2isize) * planes[i as usize].plane[2usize] -
                    planes[i as usize].plane[3usize];
            if !((d as libc::c_double) < -0.1f64 ||
                     d as libc::c_double > 0.1f64) {
                d =
                    *p2.offset(0isize) * planes[i as usize].plane[0usize] +
                        *p2.offset(1isize) * planes[i as usize].plane[1usize]
                        +
                        *p2.offset(2isize) * planes[i as usize].plane[2usize]
                        - planes[i as usize].plane[3usize];
                if !((d as libc::c_double) < -0.1f64 ||
                         d as libc::c_double > 0.1f64) {
                    d =
                        *p3.offset(0isize) * planes[i as usize].plane[0usize]
                            +
                            *p3.offset(1isize) *
                                planes[i as usize].plane[1usize] +
                            *p3.offset(2isize) *
                                planes[i as usize].plane[2usize] -
                            planes[i as usize].plane[3usize];
                    if !((d as libc::c_double) < -0.1f64 ||
                             d as libc::c_double > 0.1f64) {
                        return i
                    }
                }
            }
        }
        i += 1
    }
    if numPlanes == 2048i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"MAX_PATCH_PLANES\x00" as *const u8 as
                      *const libc::c_char);
    }
    planes[numPlanes as usize].plane[0usize] = plane[0usize];
    planes[numPlanes as usize].plane[1usize] = plane[1usize];
    planes[numPlanes as usize].plane[2usize] = plane[2usize];
    planes[numPlanes as usize].plane[3usize] = plane[3usize];
    planes[numPlanes as usize].signbits =
        CM_SignbitsForNormal(plane.as_mut_ptr());
    numPlanes += 1;
    return numPlanes - 1i32;
}
/*
=====================
CM_PlaneFromPoints

Returns false if the triangle is degenrate.
The normal will point out of the clock for clockwise ordered points
=====================
*/
unsafe extern "C" fn CM_PlaneFromPoints(mut plane: *mut vec_t,
                                        mut a: *mut vec_t, mut b: *mut vec_t,
                                        mut c: *mut vec_t) -> qboolean {
    let mut d1: vec3_t = [0.; 3];
    let mut d2: vec3_t = [0.; 3];
    d1[0usize] = *b.offset(0isize) - *a.offset(0isize);
    d1[1usize] = *b.offset(1isize) - *a.offset(1isize);
    d1[2usize] = *b.offset(2isize) - *a.offset(2isize);
    d2[0usize] = *c.offset(0isize) - *a.offset(0isize);
    d2[1usize] = *c.offset(1isize) - *a.offset(1isize);
    d2[2usize] = *c.offset(2isize) - *a.offset(2isize);
    CrossProduct(d2.as_mut_ptr() as *const vec_t,
                 d1.as_mut_ptr() as *const vec_t, plane);
    if VectorNormalize(plane) == 0i32 as libc::c_float { return qfalse }
    *plane.offset(3isize) =
        *a.offset(0isize) * *plane.offset(0isize) +
            *a.offset(1isize) * *plane.offset(1isize) +
            *a.offset(2isize) * *plane.offset(2isize);
    return qtrue;
}
/*
==================
CM_GridPlane
==================
*/
unsafe extern "C" fn CM_GridPlane(mut gridPlanes:
                                      *mut [[libc::c_int; 2]; 129],
                                  mut i: libc::c_int, mut j: libc::c_int,
                                  mut tri: libc::c_int) -> libc::c_int {
    let mut p: libc::c_int = 0;
    p = (*gridPlanes.offset(i as isize))[j as usize][tri as usize];
    if p != -1i32 { return p }
    p =
        (*gridPlanes.offset(i as
                                isize))[j as
                                            usize][(0 == tri) as libc::c_int
                                                       as usize];
    if p != -1i32 { return p }
    Com_Printf(b"WARNING: CM_GridPlane unresolvable\n\x00" as *const u8 as
                   *const libc::c_char);
    return -1i32;
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

This file does not reference any globals, and has these entry points:

void CM_ClearLevelPatches( void );
struct patchCollide_s	*CM_GeneratePatchCollide( int width, int height, const vec3_t *points );
void CM_TraceThroughPatchCollide( traceWork_t *tw, const struct patchCollide_s *pc );
qboolean CM_PositionTestInPatchCollide( traceWork_t *tw, const struct patchCollide_s *pc );
void CM_DrawDebugSurface( void (*drawPoly)(int color, int numPoints, flaot *points) );


WARNING: this may misbehave with meshes that have rows or columns that only
degenerate a few triangles.  Completely degenerate rows and columns are handled
properly.
*/
/*
#define	MAX_FACETS			1024
#define	MAX_PATCH_PLANES	2048

typedef struct {
	float	plane[4];
	int		signbits;		// signx + (signy<<1) + (signz<<2), used as lookup during collision
} patchPlane_t;

typedef struct {
	int			surfacePlane;
	int			numBorders;		// 3 or four + 6 axial bevels + 4 or 3 * 4 edge bevels
	int			borderPlanes[4+6+16];
	int			borderInward[4+6+16];
	qboolean	borderNoAdjust[4+6+16];
} facet_t;

typedef struct patchCollide_s {
	vec3_t	bounds[2];
	int		numPlanes;			// surface planes plus edge planes
	patchPlane_t	*planes;
	int		numFacets;
	facet_t	*facets;
} patchCollide_t;


#define	MAX_GRID_SIZE	129

typedef struct {
	int			width;
	int			height;
	qboolean	wrapWidth;
	qboolean	wrapHeight;
	vec3_t	points[MAX_GRID_SIZE][MAX_GRID_SIZE];	// [width][height]
} cGrid_t;

#define	SUBDIVIDE_DISTANCE	16	//4	// never more than this units away from curve
#define	PLANE_TRI_EPSILON	0.1
#define	WRAP_POINT_EPSILON	0.1
*/
#[no_mangle]
pub static mut c_totalPatchBlocks: libc::c_int = 0;
/*
=================
CM_RemoveDegenerateColumns

If there are any identical columns, remove them
=================
*/
unsafe extern "C" fn CM_RemoveDegenerateColumns(mut grid: *mut cGrid_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0i32;
    while i < (*grid).width - 1i32 {
        j = 0i32;
        while j < (*grid).height {
            if 0 ==
                   CM_ComparePoints((*grid).points[i as
                                                       usize][j as
                                                                  usize].as_mut_ptr(),
                                    (*grid).points[(i + 1i32) as
                                                       usize][j as
                                                                  usize].as_mut_ptr())
                       as u64 {
                break ;
            }
            j += 1
        }
        if !(j != (*grid).height) {
            // not degenerate
            j = 0i32;
            while j < (*grid).height {
                k = i + 2i32;
                while k < (*grid).width {
                    (*grid).points[(k - 1i32) as usize][j as usize][0usize] =
                        (*grid).points[k as usize][j as usize][0usize];
                    (*grid).points[(k - 1i32) as usize][j as usize][1usize] =
                        (*grid).points[k as usize][j as usize][1usize];
                    (*grid).points[(k - 1i32) as usize][j as usize][2usize] =
                        (*grid).points[k as usize][j as usize][2usize];
                    k += 1
                }
                j += 1
            }
            (*grid).width -= 1;
            i -= 1
        }
        i += 1
    };
}
// the new aproximating point at i+1 may need to be removed
		// or subdivided farther, so don't advance i
/*
======================
CM_ComparePoints
======================
*/
unsafe extern "C" fn CM_ComparePoints(mut a: *mut libc::c_float,
                                      mut b: *mut libc::c_float) -> qboolean {
    let mut d: libc::c_float = 0.;
    d = *a.offset(0isize) - *b.offset(0isize);
    if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64 {
        return qfalse
    }
    d = *a.offset(1isize) - *b.offset(1isize);
    if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64 {
        return qfalse
    }
    d = *a.offset(2isize) - *b.offset(2isize);
    if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64 {
        return qfalse
    }
    return qtrue;
}
/*
=================
CM_SubdivideGridColumns

Adds columns as necessary to the grid until
all the aproximating points are within SUBDIVIDE_DISTANCE
from the true curve
=================
*/
unsafe extern "C" fn CM_SubdivideGridColumns(mut grid: *mut cGrid_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    i = 0i32;
    while i < (*grid).width - 2i32 {
        j = 0i32;
        while j < (*grid).height {
            if 0 !=
                   CM_NeedsSubdivision((*grid).points[i as
                                                          usize][j as
                                                                     usize].as_mut_ptr(),
                                       (*grid).points[(i + 1i32) as
                                                          usize][j as
                                                                     usize].as_mut_ptr(),
                                       (*grid).points[(i + 2i32) as
                                                          usize][j as
                                                                     usize].as_mut_ptr())
                       as u64 {
                break ;
            }
            j += 1
        }
        if j == (*grid).height {
            j = 0i32;
            while j < (*grid).height {
                k = i + 2i32;
                while k < (*grid).width {
                    (*grid).points[(k - 1i32) as usize][j as usize][0usize] =
                        (*grid).points[k as usize][j as usize][0usize];
                    (*grid).points[(k - 1i32) as usize][j as usize][1usize] =
                        (*grid).points[k as usize][j as usize][1usize];
                    (*grid).points[(k - 1i32) as usize][j as usize][2usize] =
                        (*grid).points[k as usize][j as usize][2usize];
                    k += 1
                }
                j += 1
            }
            (*grid).width -= 1;
            i += 1
        } else {
            j = 0i32;
            while j < (*grid).height {
                let mut prev: vec3_t = [0.; 3];
                let mut mid: vec3_t = [0.; 3];
                let mut next: vec3_t = [0.; 3];
                prev[0usize] = (*grid).points[i as usize][j as usize][0usize];
                prev[1usize] = (*grid).points[i as usize][j as usize][1usize];
                prev[2usize] = (*grid).points[i as usize][j as usize][2usize];
                mid[0usize] =
                    (*grid).points[(i + 1i32) as usize][j as usize][0usize];
                mid[1usize] =
                    (*grid).points[(i + 1i32) as usize][j as usize][1usize];
                mid[2usize] =
                    (*grid).points[(i + 1i32) as usize][j as usize][2usize];
                next[0usize] =
                    (*grid).points[(i + 2i32) as usize][j as usize][0usize];
                next[1usize] =
                    (*grid).points[(i + 2i32) as usize][j as usize][1usize];
                next[2usize] =
                    (*grid).points[(i + 2i32) as usize][j as usize][2usize];
                k = (*grid).width - 1i32;
                while k > i + 1i32 {
                    (*grid).points[(k + 2i32) as usize][j as usize][0usize] =
                        (*grid).points[k as usize][j as usize][0usize];
                    (*grid).points[(k + 2i32) as usize][j as usize][1usize] =
                        (*grid).points[k as usize][j as usize][1usize];
                    (*grid).points[(k + 2i32) as usize][j as usize][2usize] =
                        (*grid).points[k as usize][j as usize][2usize];
                    k -= 1
                }
                CM_Subdivide(prev.as_mut_ptr(), mid.as_mut_ptr(),
                             next.as_mut_ptr(),
                             (*grid).points[(i + 1i32) as
                                                usize][j as
                                                           usize].as_mut_ptr(),
                             (*grid).points[(i + 2i32) as
                                                usize][j as
                                                           usize].as_mut_ptr(),
                             (*grid).points[(i + 3i32) as
                                                usize][j as
                                                           usize].as_mut_ptr());
                j += 1
            }
            (*grid).width += 2i32
        }
    };
}
/*
===============
CM_Subdivide

a, b, and c are control points.
the subdivided sequence will be: a, out1, out2, out3, c
===============
*/
unsafe extern "C" fn CM_Subdivide(mut a: *mut vec_t, mut b: *mut vec_t,
                                  mut c: *mut vec_t, mut out1: *mut vec_t,
                                  mut out2: *mut vec_t,
                                  mut out3: *mut vec_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        *out1.offset(i as isize) =
            (0.5f64 *
                 (*a.offset(i as isize) + *b.offset(i as isize)) as
                     libc::c_double) as vec_t;
        *out3.offset(i as isize) =
            (0.5f64 *
                 (*b.offset(i as isize) + *c.offset(i as isize)) as
                     libc::c_double) as vec_t;
        *out2.offset(i as isize) =
            (0.5f64 *
                 (*out1.offset(i as isize) + *out3.offset(i as isize)) as
                     libc::c_double) as vec_t;
        i += 1
    };
}
/*
================================================================================

GRID SUBDIVISION

================================================================================
*/
/*
=================
CM_NeedsSubdivision

Returns true if the given quadratic curve is not flat enough for our
collision detection purposes
=================
*/
unsafe extern "C" fn CM_NeedsSubdivision(mut a: *mut vec_t, mut b: *mut vec_t,
                                         mut c: *mut vec_t) -> qboolean {
    let mut cmid: vec3_t = [0.; 3];
    let mut lmid: vec3_t = [0.; 3];
    let mut delta: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        lmid[i as usize] =
            (0.5f64 *
                 (*a.offset(i as isize) + *c.offset(i as isize)) as
                     libc::c_double) as vec_t;
        i += 1
    }
    i = 0i32;
    while i < 3i32 {
        cmid[i as usize] =
            (0.5f64 *
                 (0.5f64 *
                      (*a.offset(i as isize) + *b.offset(i as isize)) as
                          libc::c_double +
                      0.5f64 *
                          (*b.offset(i as isize) + *c.offset(i as isize)) as
                              libc::c_double)) as vec_t;
        i += 1
    }
    delta[0usize] = cmid[0usize] - lmid[0usize];
    delta[1usize] = cmid[1usize] - lmid[1usize];
    delta[2usize] = cmid[2usize] - lmid[2usize];
    dist = VectorLength(delta.as_mut_ptr() as *const vec_t);
    return (dist >= 16i32 as libc::c_float) as libc::c_int as qboolean;
}
/*
===================
CM_SetGridWrapWidth

If the left and right columns are exactly equal, set grid->wrapWidth qtrue
===================
*/
unsafe extern "C" fn CM_SetGridWrapWidth(mut grid: *mut cGrid_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    i = 0i32;
    while i < (*grid).height {
        j = 0i32;
        while j < 3i32 {
            d =
                (*grid).points[0usize][i as usize][j as usize] -
                    (*grid).points[((*grid).width - 1i32) as
                                       usize][i as usize][j as usize];
            if (d as libc::c_double) < -0.1f64 || d as libc::c_double > 0.1f64
               {
                break ;
            }
            j += 1
        }
        if j != 3i32 { break ; }
        i += 1
    }
    if i == (*grid).height {
        (*grid).wrapWidth = qtrue
    } else { (*grid).wrapWidth = qfalse };
}
/*
=================
CM_TransposeGrid

Swaps the rows and columns in place
=================
*/
unsafe extern "C" fn CM_TransposeGrid(mut grid: *mut cGrid_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut temp: vec3_t = [0.; 3];
    let mut tempWrap: qboolean = qfalse;
    if (*grid).width > (*grid).height {
        i = 0i32;
        while i < (*grid).height {
            j = i + 1i32;
            while j < (*grid).width {
                if j < (*grid).height {
                    temp[0usize] =
                        (*grid).points[i as usize][j as usize][0usize];
                    temp[1usize] =
                        (*grid).points[i as usize][j as usize][1usize];
                    temp[2usize] =
                        (*grid).points[i as usize][j as usize][2usize];
                    (*grid).points[i as usize][j as usize][0usize] =
                        (*grid).points[j as usize][i as usize][0usize];
                    (*grid).points[i as usize][j as usize][1usize] =
                        (*grid).points[j as usize][i as usize][1usize];
                    (*grid).points[i as usize][j as usize][2usize] =
                        (*grid).points[j as usize][i as usize][2usize];
                    (*grid).points[j as usize][i as usize][0usize] =
                        temp[0usize];
                    (*grid).points[j as usize][i as usize][1usize] =
                        temp[1usize];
                    (*grid).points[j as usize][i as usize][2usize] =
                        temp[2usize]
                } else {
                    (*grid).points[i as usize][j as usize][0usize] =
                        (*grid).points[j as usize][i as usize][0usize];
                    (*grid).points[i as usize][j as usize][1usize] =
                        (*grid).points[j as usize][i as usize][1usize];
                    (*grid).points[i as usize][j as usize][2usize] =
                        (*grid).points[j as usize][i as usize][2usize]
                }
                j += 1
            }
            i += 1
        }
    } else {
        i = 0i32;
        while i < (*grid).width {
            j = i + 1i32;
            while j < (*grid).height {
                if j < (*grid).width {
                    temp[0usize] =
                        (*grid).points[j as usize][i as usize][0usize];
                    temp[1usize] =
                        (*grid).points[j as usize][i as usize][1usize];
                    temp[2usize] =
                        (*grid).points[j as usize][i as usize][2usize];
                    (*grid).points[j as usize][i as usize][0usize] =
                        (*grid).points[i as usize][j as usize][0usize];
                    (*grid).points[j as usize][i as usize][1usize] =
                        (*grid).points[i as usize][j as usize][1usize];
                    (*grid).points[j as usize][i as usize][2usize] =
                        (*grid).points[i as usize][j as usize][2usize];
                    (*grid).points[i as usize][j as usize][0usize] =
                        temp[0usize];
                    (*grid).points[i as usize][j as usize][1usize] =
                        temp[1usize];
                    (*grid).points[i as usize][j as usize][2usize] =
                        temp[2usize]
                } else {
                    (*grid).points[j as usize][i as usize][0usize] =
                        (*grid).points[i as usize][j as usize][0usize];
                    (*grid).points[j as usize][i as usize][1usize] =
                        (*grid).points[i as usize][j as usize][1usize];
                    (*grid).points[j as usize][i as usize][2usize] =
                        (*grid).points[i as usize][j as usize][2usize]
                }
                j += 1
            }
            i += 1
        }
    }
    l = (*grid).width;
    (*grid).width = (*grid).height;
    (*grid).height = l;
    tempWrap = (*grid).wrapWidth;
    (*grid).wrapWidth = (*grid).wrapHeight;
    (*grid).wrapHeight = tempWrap;
}
#[no_mangle]
pub unsafe extern "C" fn CM_TraceThroughPatchCollide(mut tw: *mut traceWork_t,
                                                     mut pc:
                                                         *const patchCollide_s) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut hit: libc::c_int = 0;
    let mut hitnum: libc::c_int = 0;
    let mut offset: libc::c_float = 0.;
    let mut enterFrac: libc::c_float = 0.;
    let mut leaveFrac: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut planes_0: *mut patchPlane_t = 0 as *mut patchPlane_t;
    let mut facet: *mut facet_t = 0 as *mut facet_t;
    let mut plane: [libc::c_float; 4] =
        [0i32 as libc::c_float, 0i32 as libc::c_float, 0i32 as libc::c_float,
         0i32 as libc::c_float];
    let mut bestplane: [libc::c_float; 4] =
        [0i32 as libc::c_float, 0i32 as libc::c_float, 0i32 as libc::c_float,
         0i32 as libc::c_float];
    let mut startp: vec3_t = [0.; 3];
    let mut endp: vec3_t = [0.; 3];
    static mut cv: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    if 0 ==
           CM_BoundsIntersect((*tw).bounds[0usize].as_mut_ptr() as
                                  *const vec_t,
                              (*tw).bounds[1usize].as_mut_ptr() as
                                  *const vec_t, (*pc).bounds[0usize].as_ptr(),
                              (*pc).bounds[1usize].as_ptr()) as u64 {
        return
    }
    if 0 != (*tw).isPoint as u64 {
        CM_TracePointThroughPatchCollide(tw, pc);
        return
    }
    facet = (*pc).facets;
    i = 0i32;
    while i < (*pc).numFacets {
        enterFrac = -1.0f64 as libc::c_float;
        leaveFrac = 1.0f64 as libc::c_float;
        hitnum = -1i32;
        planes_0 =
            &mut *(*pc).planes.offset((*facet).surfacePlane as isize) as
                *mut patchPlane_t;
        plane[0usize] = (*planes_0).plane[0usize];
        plane[1usize] = (*planes_0).plane[1usize];
        plane[2usize] = (*planes_0).plane[2usize];
        plane[3usize] = (*planes_0).plane[3usize];
        if 0 != (*tw).sphere.use_0 as u64 {
            plane[3usize] += (*tw).sphere.radius;
            t =
                plane[0usize] * (*tw).sphere.offset[0usize] +
                    plane[1usize] * (*tw).sphere.offset[1usize] +
                    plane[2usize] * (*tw).sphere.offset[2usize];
            if t > 0.0f32 {
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
        } else {
            offset =
                (*tw).offsets[(*planes_0).signbits as usize][0usize] *
                    plane[0usize] +
                    (*tw).offsets[(*planes_0).signbits as usize][1usize] *
                        plane[1usize] +
                    (*tw).offsets[(*planes_0).signbits as usize][2usize] *
                        plane[2usize];
            plane[3usize] -= offset;
            startp[0usize] = (*tw).start[0usize];
            startp[1usize] = (*tw).start[1usize];
            startp[2usize] = (*tw).start[2usize];
            endp[0usize] = (*tw).end[0usize];
            endp[1usize] = (*tw).end[1usize];
            endp[2usize] = (*tw).end[2usize]
        }
        if !(0 ==
                 CM_CheckFacetPlane(plane.as_mut_ptr(), startp.as_mut_ptr(),
                                    endp.as_mut_ptr(), &mut enterFrac,
                                    &mut leaveFrac, &mut hit)) {
            if 0 != hit {
                bestplane[0usize] = plane[0usize];
                bestplane[1usize] = plane[1usize];
                bestplane[2usize] = plane[2usize];
                bestplane[3usize] = plane[3usize]
            }
            j = 0i32;
            while j < (*facet).numBorders {
                planes_0 =
                    &mut *(*pc).planes.offset((*facet).borderPlanes[j as
                                                                        usize]
                                                  as isize) as
                        *mut patchPlane_t;
                if 0 != (*facet).borderInward[j as usize] {
                    plane[0usize] = -(*planes_0).plane[0usize];
                    plane[1usize] = -(*planes_0).plane[1usize];
                    plane[2usize] = -(*planes_0).plane[2usize];
                    plane[3usize] = -(*planes_0).plane[3usize]
                } else {
                    plane[0usize] = (*planes_0).plane[0usize];
                    plane[1usize] = (*planes_0).plane[1usize];
                    plane[2usize] = (*planes_0).plane[2usize];
                    plane[3usize] = (*planes_0).plane[3usize]
                }
                if 0 != (*tw).sphere.use_0 as u64 {
                    plane[3usize] += (*tw).sphere.radius;
                    t =
                        plane[0usize] * (*tw).sphere.offset[0usize] +
                            plane[1usize] * (*tw).sphere.offset[1usize] +
                            plane[2usize] * (*tw).sphere.offset[2usize];
                    if t > 0.0f32 {
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
                        endp[2usize] =
                            (*tw).end[2usize] - (*tw).sphere.offset[2usize]
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
                        endp[2usize] =
                            (*tw).end[2usize] + (*tw).sphere.offset[2usize]
                    }
                } else {
                    offset =
                        (*tw).offsets[(*planes_0).signbits as usize][0usize] *
                            plane[0usize] +
                            (*tw).offsets[(*planes_0).signbits as
                                              usize][1usize] * plane[1usize] +
                            (*tw).offsets[(*planes_0).signbits as
                                              usize][2usize] * plane[2usize];
                    plane[3usize] =
                        (plane[3usize] as libc::c_double +
                             fabs(offset as libc::c_double)) as libc::c_float;
                    startp[0usize] = (*tw).start[0usize];
                    startp[1usize] = (*tw).start[1usize];
                    startp[2usize] = (*tw).start[2usize];
                    endp[0usize] = (*tw).end[0usize];
                    endp[1usize] = (*tw).end[1usize];
                    endp[2usize] = (*tw).end[2usize]
                }
                if 0 ==
                       CM_CheckFacetPlane(plane.as_mut_ptr(),
                                          startp.as_mut_ptr(),
                                          endp.as_mut_ptr(), &mut enterFrac,
                                          &mut leaveFrac, &mut hit) {
                    break ;
                }
                if 0 != hit {
                    hitnum = j;
                    bestplane[0usize] = plane[0usize];
                    bestplane[1usize] = plane[1usize];
                    bestplane[2usize] = plane[2usize];
                    bestplane[3usize] = plane[3usize]
                }
                j += 1
            }
            if !(j < (*facet).numBorders) {
                //never clip against the back side
                if !(hitnum == (*facet).numBorders - 1i32) {
                    if enterFrac < leaveFrac &&
                           enterFrac >= 0i32 as libc::c_float {
                        if enterFrac < (*tw).trace.fraction {
                            if enterFrac < 0i32 as libc::c_float {
                                enterFrac = 0i32 as libc::c_float
                            }
                            if cv.is_null() {
                                cv =
                                    Cvar_Get(b"r_debugSurfaceUpdate\x00" as
                                                 *const u8 as
                                                 *const libc::c_char,
                                             b"1\x00" as *const u8 as
                                                 *const libc::c_char, 0i32)
                            }
                            if !cv.is_null() && 0 != (*cv).integer {
                                debugPatchCollide = pc;
                                debugFacet = facet
                            }
                            (*tw).trace.fraction = enterFrac;
                            (*tw).trace.plane.normal[0usize] =
                                bestplane[0usize];
                            (*tw).trace.plane.normal[1usize] =
                                bestplane[1usize];
                            (*tw).trace.plane.normal[2usize] =
                                bestplane[2usize];
                            (*tw).trace.plane.dist = bestplane[3usize]
                        }
                    }
                }
            }
        }
        i += 1;
        facet = facet.offset(1isize)
    };
}
/*
====================
CM_CheckFacetPlane
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_CheckFacetPlane(mut plane: *mut libc::c_float,
                                            mut start: *mut vec_t,
                                            mut end: *mut vec_t,
                                            mut enterFrac: *mut libc::c_float,
                                            mut leaveFrac: *mut libc::c_float,
                                            mut hit: *mut libc::c_int)
 -> libc::c_int {
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    *hit = qfalse as libc::c_int;
    d1 =
        *start.offset(0isize) * *plane.offset(0isize) +
            *start.offset(1isize) * *plane.offset(1isize) +
            *start.offset(2isize) * *plane.offset(2isize) -
            *plane.offset(3isize);
    d2 =
        *end.offset(0isize) * *plane.offset(0isize) +
            *end.offset(1isize) * *plane.offset(1isize) +
            *end.offset(2isize) * *plane.offset(2isize) -
            *plane.offset(3isize);
    if d1 > 0i32 as libc::c_float &&
           (d2 as libc::c_double >= 0.125f64 || d2 >= d1) {
        return qfalse as libc::c_int
    }
    if d1 <= 0i32 as libc::c_float && d2 <= 0i32 as libc::c_float {
        return qtrue as libc::c_int
    }
    if d1 > d2 {
        f =
            ((d1 as libc::c_double - 0.125f64) / (d1 - d2) as libc::c_double)
                as libc::c_float;
        if f < 0i32 as libc::c_float { f = 0i32 as libc::c_float }
        if f > *enterFrac { *enterFrac = f; *hit = qtrue as libc::c_int }
    } else {
        f =
            ((d1 as libc::c_double + 0.125f64) / (d1 - d2) as libc::c_double)
                as libc::c_float;
        if f > 1i32 as libc::c_float { f = 1i32 as libc::c_float }
        if f < *leaveFrac { *leaveFrac = f }
    }
    return qtrue as libc::c_int;
}
/*
================================================================================

TRACE TESTING

================================================================================
*/
/*
====================
CM_TracePointThroughPatchCollide

  special case for point traces because the patch collide "brushes" have no volume
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CM_TracePointThroughPatchCollide(mut tw:
                                                              *mut traceWork_t,
                                                          mut pc:
                                                              *const patchCollide_s) {
    let mut frontFacing: [qboolean; 2048] = [qfalse; 2048];
    let mut intersection: [libc::c_float; 2048] = [0.; 2048];
    let mut intersect: libc::c_float = 0.;
    let mut planes_0: *const patchPlane_t = 0 as *const patchPlane_t;
    let mut facet: *const facet_t = 0 as *const facet_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut offset: libc::c_float = 0.;
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    static mut cv: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
    if 0 == (*cm_playerCurveClip).integer || 0 == (*tw).isPoint as u64 {
        return
    }
    planes_0 = (*pc).planes;
    i = 0i32;
    while i < (*pc).numPlanes {
        offset =
            (*tw).offsets[(*planes_0).signbits as usize][0usize] *
                (*planes_0).plane[0usize] +
                (*tw).offsets[(*planes_0).signbits as usize][1usize] *
                    (*planes_0).plane[1usize] +
                (*tw).offsets[(*planes_0).signbits as usize][2usize] *
                    (*planes_0).plane[2usize];
        d1 =
            (*tw).start[0usize] * (*planes_0).plane[0usize] +
                (*tw).start[1usize] * (*planes_0).plane[1usize] +
                (*tw).start[2usize] * (*planes_0).plane[2usize] -
                (*planes_0).plane[3usize] + offset;
        d2 =
            (*tw).end[0usize] * (*planes_0).plane[0usize] +
                (*tw).end[1usize] * (*planes_0).plane[1usize] +
                (*tw).end[2usize] * (*planes_0).plane[2usize] -
                (*planes_0).plane[3usize] + offset;
        if d1 <= 0i32 as libc::c_float {
            frontFacing[i as usize] = qfalse
        } else { frontFacing[i as usize] = qtrue }
        if d1 == d2 {
            intersection[i as usize] = 99999i32 as libc::c_float
        } else {
            intersection[i as usize] = d1 / (d1 - d2);
            if intersection[i as usize] <= 0i32 as libc::c_float {
                intersection[i as usize] = 99999i32 as libc::c_float
            }
        }
        i += 1;
        planes_0 = planes_0.offset(1isize)
    }
    facet = (*pc).facets;
    i = 0i32;
    while i < (*pc).numFacets {
        if !(0 == frontFacing[(*facet).surfacePlane as usize] as u64) {
            intersect = intersection[(*facet).surfacePlane as usize];
            if !(intersect < 0i32 as libc::c_float) {
                // surface is behind the starting point
                if !(intersect > (*tw).trace.fraction) {
                    // already hit something closer
                    j = 0i32;
                    while j < (*facet).numBorders {
                        k = (*facet).borderPlanes[j as usize];
                        if 0 !=
                               frontFacing[k as usize] as libc::c_uint ^
                                   (*facet).borderInward[j as usize] as
                                       libc::c_uint {
                            if intersection[k as usize] > intersect {
                                break ;
                            }
                        } else if intersection[k as usize] < intersect {
                            break ;
                        }
                        j += 1
                    }
                    if j == (*facet).numBorders {
                        if cv.is_null() {
                            cv =
                                Cvar_Get(b"r_debugSurfaceUpdate\x00" as
                                             *const u8 as *const libc::c_char,
                                         b"1\x00" as *const u8 as
                                             *const libc::c_char, 0i32)
                        }
                        if 0 != (*cv).integer {
                            debugPatchCollide = pc;
                            debugFacet = facet
                        }
                        planes_0 =
                            &mut *(*pc).planes.offset((*facet).surfacePlane as
                                                          isize) as
                                *mut patchPlane_t;
                        offset =
                            (*tw).offsets[(*planes_0).signbits as
                                              usize][0usize] *
                                (*planes_0).plane[0usize] +
                                (*tw).offsets[(*planes_0).signbits as
                                                  usize][1usize] *
                                    (*planes_0).plane[1usize] +
                                (*tw).offsets[(*planes_0).signbits as
                                                  usize][2usize] *
                                    (*planes_0).plane[2usize];
                        d1 =
                            (*tw).start[0usize] * (*planes_0).plane[0usize] +
                                (*tw).start[1usize] *
                                    (*planes_0).plane[1usize] +
                                (*tw).start[2usize] *
                                    (*planes_0).plane[2usize] -
                                (*planes_0).plane[3usize] + offset;
                        d2 =
                            (*tw).end[0usize] * (*planes_0).plane[0usize] +
                                (*tw).end[1usize] * (*planes_0).plane[1usize]
                                +
                                (*tw).end[2usize] * (*planes_0).plane[2usize]
                                - (*planes_0).plane[3usize] + offset;
                        (*tw).trace.fraction =
                            ((d1 as libc::c_double - 0.125f64) /
                                 (d1 - d2) as libc::c_double) as
                                libc::c_float;
                        if (*tw).trace.fraction < 0i32 as libc::c_float {
                            (*tw).trace.fraction = 0i32 as libc::c_float
                        }
                        (*tw).trace.plane.normal[0usize] =
                            (*planes_0).plane[0usize];
                        (*tw).trace.plane.normal[1usize] =
                            (*planes_0).plane[1usize];
                        (*tw).trace.plane.normal[2usize] =
                            (*planes_0).plane[2usize];
                        (*tw).trace.plane.dist = (*planes_0).plane[3usize]
                    }
                }
            }
        }
        i += 1;
        facet = facet.offset(1isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn CM_PositionTestInPatchCollide(mut tw:
                                                           *mut traceWork_t,
                                                       mut pc:
                                                           *const patchCollide_s)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut offset: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut planes_0: *mut patchPlane_t = 0 as *mut patchPlane_t;
    let mut facet: *mut facet_t = 0 as *mut facet_t;
    let mut plane: [libc::c_float; 4] = [0.; 4];
    let mut startp: vec3_t = [0.; 3];
    if 0 != (*tw).isPoint as u64 { return qfalse }
    facet = (*pc).facets;
    i = 0i32;
    while i < (*pc).numFacets {
        planes_0 =
            &mut *(*pc).planes.offset((*facet).surfacePlane as isize) as
                *mut patchPlane_t;
        plane[0usize] = (*planes_0).plane[0usize];
        plane[1usize] = (*planes_0).plane[1usize];
        plane[2usize] = (*planes_0).plane[2usize];
        plane[3usize] = (*planes_0).plane[3usize];
        if 0 != (*tw).sphere.use_0 as u64 {
            plane[3usize] += (*tw).sphere.radius;
            t =
                plane[0usize] * (*tw).sphere.offset[0usize] +
                    plane[1usize] * (*tw).sphere.offset[1usize] +
                    plane[2usize] * (*tw).sphere.offset[2usize];
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
        } else {
            offset =
                (*tw).offsets[(*planes_0).signbits as usize][0usize] *
                    plane[0usize] +
                    (*tw).offsets[(*planes_0).signbits as usize][1usize] *
                        plane[1usize] +
                    (*tw).offsets[(*planes_0).signbits as usize][2usize] *
                        plane[2usize];
            plane[3usize] -= offset;
            startp[0usize] = (*tw).start[0usize];
            startp[1usize] = (*tw).start[1usize];
            startp[2usize] = (*tw).start[2usize]
        }
        if !(plane[0usize] * startp[0usize] + plane[1usize] * startp[1usize] +
                 plane[2usize] * startp[2usize] - plane[3usize] > 0.0f32) {
            j = 0i32;
            while j < (*facet).numBorders {
                planes_0 =
                    &mut *(*pc).planes.offset((*facet).borderPlanes[j as
                                                                        usize]
                                                  as isize) as
                        *mut patchPlane_t;
                if 0 != (*facet).borderInward[j as usize] {
                    plane[0usize] = -(*planes_0).plane[0usize];
                    plane[1usize] = -(*planes_0).plane[1usize];
                    plane[2usize] = -(*planes_0).plane[2usize];
                    plane[3usize] = -(*planes_0).plane[3usize]
                } else {
                    plane[0usize] = (*planes_0).plane[0usize];
                    plane[1usize] = (*planes_0).plane[1usize];
                    plane[2usize] = (*planes_0).plane[2usize];
                    plane[3usize] = (*planes_0).plane[3usize]
                }
                if 0 != (*tw).sphere.use_0 as u64 {
                    plane[3usize] += (*tw).sphere.radius;
                    t =
                        plane[0usize] * (*tw).sphere.offset[0usize] +
                            plane[1usize] * (*tw).sphere.offset[1usize] +
                            plane[2usize] * (*tw).sphere.offset[2usize];
                    if t > 0.0f32 {
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
                } else {
                    offset =
                        (*tw).offsets[(*planes_0).signbits as usize][0usize] *
                            plane[0usize] +
                            (*tw).offsets[(*planes_0).signbits as
                                              usize][1usize] * plane[1usize] +
                            (*tw).offsets[(*planes_0).signbits as
                                              usize][2usize] * plane[2usize];
                    plane[3usize] =
                        (plane[3usize] as libc::c_double +
                             fabs(offset as libc::c_double)) as libc::c_float;
                    startp[0usize] = (*tw).start[0usize];
                    startp[1usize] = (*tw).start[1usize];
                    startp[2usize] = (*tw).start[2usize]
                }
                if plane[0usize] * startp[0usize] +
                       plane[1usize] * startp[1usize] +
                       plane[2usize] * startp[2usize] - plane[3usize] > 0.0f32
                   {
                    break ;
                }
                j += 1
            }
            if !(j < (*facet).numBorders) { return qtrue }
        }
        i += 1;
        facet = facet.offset(1isize)
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn CM_ClearLevelPatches() {
    debugPatchCollide = 0 as *const patchCollide_t;
    debugFacet = 0 as *const facet_t;
}
#[no_mangle]
pub static mut c_totalPatchSurfaces: libc::c_int = 0;
#[no_mangle]
pub static mut c_totalPatchEdges: libc::c_int = 0;