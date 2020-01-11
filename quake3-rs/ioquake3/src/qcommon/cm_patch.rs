// =============== BEGIN cm_patch_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct patchCollide_s {
    pub bounds: [crate::src::qcommon::q_shared::vec3_t; 2],
    pub numPlanes: i32,
    pub planes: *mut crate::src::qcommon::cm_patch::patchPlane_t,
    pub numFacets: i32,
    pub facets: *mut crate::src::qcommon::cm_patch::facet_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct facet_t {
    pub surfacePlane: i32,
    pub numBorders: i32,
    pub borderPlanes: [i32; 26],
    pub borderInward: [i32; 26],
    pub borderNoAdjust: [crate::src::qcommon::q_shared::qboolean; 26],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct patchPlane_t {
    pub plane: [f32; 4],
    pub signbits: i32,
}

pub type patchCollide_t = crate::src::qcommon::cm_patch::patchCollide_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cGrid_t {
    pub width: i32,
    pub height: i32,
    pub wrapWidth: crate::src::qcommon::q_shared::qboolean,
    pub wrapHeight: crate::src::qcommon::q_shared::qboolean,
    pub points: [[crate::src::qcommon::q_shared::vec3_t; 129]; 129],
}
use ::libc;

pub mod q_shared_h {

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

    /*
    // if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
    // or write a mail to the ioq3 mailing list.
    #else
      #define Q_ftol(v) ((long) (v))
      #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
      #define Q_SnapVector(vec) \
        do\
        {\
            vec3_t *temp = (vec);\
            \
            Q_round((*temp)[0]);\
            Q_round((*temp)[1]);\
            Q_round((*temp)[2]);\
        } while(0)
    #endif
    */
    // reciprocal square root
    // this isn't a real cheap function to call!
    // just in case you don't want to use the macros
    // fast vector normalize routine that does not check to make sure
    // that length != 0, nor does it return length, uses rsqrt approximation
    // returns vector length
    // perpendicular vector could be replaced by this
    //int	PlaneTypeForNormal (vec3_t normal);
    //=============================================
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    // data is an in/out parm, returns a parsed out token
    // mode parm for FS_FOpenFile
    //=============================================
    // portable case insensitive compare
    // buffer size safe library replacements
    // strlen that discounts Quake color sequences
    // removes color sequences from string
    // Count the number of char tocount encountered in string
    //=============================================
    // 64-bit integers for global rankings interface
    // implemented as a struct for qvm compatibility
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */
    //=============================================
    //
    // key / value info strings
    //
    // this is only here so the functions in q_shared.c and bg_*.c can link
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
    // cvar_restart will reset to this value
    // for CVAR_LATCH vars
    // set each time the cvar is changed
    // incremented each time the cvar is changed
    // atof( string )
    // atoi( string )
    // the modules that run in the virtual machine can't access the cvar_t directly,
    // so they must ask for structured updates
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

    // a trace is returned when a box is swept through the world

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0) * *v.offset(0)
                + *v.offset(1) * *v.offset(1)
                + *v.offset(2) * *v.offset(2)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0) = *v1.offset(1) * *v2.offset(2) - *v1.offset(2) * *v2.offset(1);
        *cross.offset(1) = *v1.offset(2) * *v2.offset(0) - *v1.offset(0) * *v2.offset(2);
        *cross.offset(2) = *v1.offset(0) * *v2.offset(1) - *v1.offset(1) * *v2.offset(0);
    }

    // __Q_SHARED_H
}

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::cm_local_h::sphere_t;
pub use crate::cm_local_h::traceWork_t;
pub use crate::src::qcommon::cm_load::cm_playerCurveClip;
pub use crate::src::qcommon::cm_patch::q_shared_h::CrossProduct;
pub use crate::src::qcommon::cm_patch::q_shared_h::VectorLength;
pub use crate::src::qcommon::cm_polylib::winding_t;
pub use crate::src::qcommon::cm_polylib::BaseWindingForPlane;
pub use crate::src::qcommon::cm_polylib::ChopWindingInPlace;
pub use crate::src::qcommon::cm_polylib::CopyWinding;
pub use crate::src::qcommon::cm_polylib::FreeWinding;
pub use crate::src::qcommon::cm_polylib::WindingBounds;
pub use crate::src::qcommon::cm_test::CM_BoundsIntersect;

pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Hunk_Alloc;

pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::ClearBounds;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;

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
    pub fn BotDrawDebugPolygons(
        drawPoly: Option<unsafe extern "C" fn(_: i32, _: i32, _: *mut f32) -> ()>,
        value: i32,
    );
}

pub const EN_TOP: C2RustUnnamed_113 = 0;

pub const EN_LEFT: C2RustUnnamed_113 = 3;

pub const EN_BOTTOM: C2RustUnnamed_113 = 2;

pub const EN_RIGHT: C2RustUnnamed_113 = 1;

pub type C2RustUnnamed_113 = u32;
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

pub static mut c_totalPatchBlocks: i32 = 0;
#[no_mangle]

pub static mut c_totalPatchSurfaces: i32 = 0;
#[no_mangle]

pub static mut c_totalPatchEdges: i32 = 0;

static mut debugPatchCollide: *const crate::src::qcommon::cm_patch::patchCollide_t =
    0 as *const crate::src::qcommon::cm_patch::patchCollide_t;

static mut debugFacet: *const crate::src::qcommon::cm_patch::facet_t =
    0 as *const crate::src::qcommon::cm_patch::facet_t;

static mut debugBlock: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;

static mut debugBlockPoints: [crate::src::qcommon::q_shared::vec3_t; 4] = [[0.; 3]; 4];
/*
=================
CM_ClearLevelPatches
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_ClearLevelPatches() {
    debugPatchCollide = 0 as *const crate::src::qcommon::cm_patch::patchCollide_t;
    debugFacet = 0 as *const crate::src::qcommon::cm_patch::facet_t;
}
/*
=================
CM_SignbitsForNormal
=================
*/

unsafe extern "C" fn CM_SignbitsForNormal(
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut bits: i32 = 0;
    let mut _j: i32 = 0;
    bits = 0;

    for j in 0..3 {
        if *normal.offset(j as isize) < 0f32 {
            bits |= (1) << j
        }
    }
    return bits;
}
/*
=====================
CM_PlaneFromPoints

Returns false if the triangle is degenrate.
The normal will point out of the clock for clockwise ordered points
=====================
*/

unsafe extern "C" fn CM_PlaneFromPoints(
    mut plane: *mut crate::src::qcommon::q_shared::vec_t,
    mut a: *mut crate::src::qcommon::q_shared::vec_t,
    mut b: *mut crate::src::qcommon::q_shared::vec_t,
    mut c: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut d1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    d1[0] = *b.offset(0) - *a.offset(0);
    d1[1] = *b.offset(1) - *a.offset(1);
    d1[2] = *b.offset(2) - *a.offset(2);
    d2[0] = *c.offset(0) - *a.offset(0);
    d2[1] = *c.offset(1) - *a.offset(1);
    d2[2] = *c.offset(2) - *a.offset(2);
    CrossProduct(
        d2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        d1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        plane,
    );
    if crate::src::qcommon::q_math::VectorNormalize(plane) == 0f32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    *plane.offset(3) = *a.offset(0) * *plane.offset(0)
        + *a.offset(1) * *plane.offset(1)
        + *a.offset(2) * *plane.offset(2);
    return crate::src::qcommon::q_shared::qtrue;
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

unsafe extern "C" fn CM_NeedsSubdivision(
    mut a: *mut crate::src::qcommon::q_shared::vec_t,
    mut b: *mut crate::src::qcommon::q_shared::vec_t,
    mut c: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut cmid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lmid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dist: f32 = 0.;
    let mut i: i32 = 0;
    // calculate the linear midpoint
    i = 0;
    while i < 3 {
        lmid[i as usize] = (0.5 * (*a.offset(i as isize) + *c.offset(i as isize)) as f64)
            as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    // calculate the exact curve midpoint
    i = 0;
    while i < 3 {
        cmid[i as usize] = (0.5
            * (0.5 * (*a.offset(i as isize) + *b.offset(i as isize)) as f64
                + 0.5 * (*b.offset(i as isize) + *c.offset(i as isize)) as f64))
            as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
    // see if the curve is far enough away from the linear mid
    delta[0] = cmid[0] - lmid[0];
    delta[1] = cmid[1] - lmid[1];
    delta[2] = cmid[2] - lmid[2];
    dist = VectorLength(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    return (dist >= 16f32) as crate::src::qcommon::q_shared::qboolean;
}
/*
===============
CM_Subdivide

a, b, and c are control points.
the subdivided sequence will be: a, out1, out2, out3, c
===============
*/

unsafe extern "C" fn CM_Subdivide(
    mut a: *mut crate::src::qcommon::q_shared::vec_t,
    mut b: *mut crate::src::qcommon::q_shared::vec_t,
    mut c: *mut crate::src::qcommon::q_shared::vec_t,
    mut out1: *mut crate::src::qcommon::q_shared::vec_t,
    mut out2: *mut crate::src::qcommon::q_shared::vec_t,
    mut out3: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 3 {
        *out1.offset(i as isize) = (0.5 * (*a.offset(i as isize) + *b.offset(i as isize)) as f64)
            as crate::src::qcommon::q_shared::vec_t;
        *out3.offset(i as isize) = (0.5 * (*b.offset(i as isize) + *c.offset(i as isize)) as f64)
            as crate::src::qcommon::q_shared::vec_t;
        *out2.offset(i as isize) = (0.5
            * (*out1.offset(i as isize) + *out3.offset(i as isize)) as f64)
            as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
}
/*
=================
CM_TransposeGrid

Swaps the rows and columns in place
=================
*/

unsafe extern "C" fn CM_TransposeGrid(mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut l: i32 = 0;
    let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tempWrap: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if (*grid).width > (*grid).height {
        i = 0;
        while i < (*grid).height {
            j = i + 1;
            while j < (*grid).width {
                if j < (*grid).height {
                    // swap the value
                    temp[0] = (*grid).points[i as usize][j as usize][0];
                    temp[1] = (*grid).points[i as usize][j as usize][1];
                    temp[2] = (*grid).points[i as usize][j as usize][2];
                    (*grid).points[i as usize][j as usize][0] =
                        (*grid).points[j as usize][i as usize][0];
                    (*grid).points[i as usize][j as usize][1] =
                        (*grid).points[j as usize][i as usize][1];
                    (*grid).points[i as usize][j as usize][2] =
                        (*grid).points[j as usize][i as usize][2];
                    (*grid).points[j as usize][i as usize][0] = temp[0];
                    (*grid).points[j as usize][i as usize][1] = temp[1];
                    (*grid).points[j as usize][i as usize][2] = temp[2]
                } else {
                    // just copy
                    (*grid).points[i as usize][j as usize][0] =
                        (*grid).points[j as usize][i as usize][0];
                    (*grid).points[i as usize][j as usize][1] =
                        (*grid).points[j as usize][i as usize][1];
                    (*grid).points[i as usize][j as usize][2] =
                        (*grid).points[j as usize][i as usize][2]
                }
                j += 1
            }
            i += 1
        }
    } else {
        i = 0;
        while i < (*grid).width {
            j = i + 1;
            while j < (*grid).height {
                if j < (*grid).width {
                    // swap the value
                    temp[0] = (*grid).points[j as usize][i as usize][0];
                    temp[1] = (*grid).points[j as usize][i as usize][1];
                    temp[2] = (*grid).points[j as usize][i as usize][2];
                    (*grid).points[j as usize][i as usize][0] =
                        (*grid).points[i as usize][j as usize][0];
                    (*grid).points[j as usize][i as usize][1] =
                        (*grid).points[i as usize][j as usize][1];
                    (*grid).points[j as usize][i as usize][2] =
                        (*grid).points[i as usize][j as usize][2];
                    (*grid).points[i as usize][j as usize][0] = temp[0];
                    (*grid).points[i as usize][j as usize][1] = temp[1];
                    (*grid).points[i as usize][j as usize][2] = temp[2]
                } else {
                    // just copy
                    (*grid).points[j as usize][i as usize][0] =
                        (*grid).points[i as usize][j as usize][0];
                    (*grid).points[j as usize][i as usize][1] =
                        (*grid).points[i as usize][j as usize][1];
                    (*grid).points[j as usize][i as usize][2] =
                        (*grid).points[i as usize][j as usize][2]
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
/*
===================
CM_SetGridWrapWidth

If the left and right columns are exactly equal, set grid->wrapWidth qtrue
===================
*/

unsafe extern "C" fn CM_SetGridWrapWidth(mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut d: f32 = 0.;
    i = 0;
    while i < (*grid).height {
        j = 0;
        while j < 3 {
            d = (*grid).points[0][i as usize][j as usize]
                - (*grid).points[((*grid).width - 1i32) as usize][i as usize][j as usize];
            if (d as f64) < -0.1 || d as f64 > 0.1 {
                break;
            }
            j += 1
        }
        if j != 3 {
            break;
        }
        i += 1
    }
    if i == (*grid).height {
        (*grid).wrapWidth = crate::src::qcommon::q_shared::qtrue
    } else {
        (*grid).wrapWidth = crate::src::qcommon::q_shared::qfalse
    };
}
/*
=================
CM_SubdivideGridColumns

Adds columns as necessary to the grid until
all the aproximating points are within SUBDIVIDE_DISTANCE
from the true curve
=================
*/

unsafe extern "C" fn CM_SubdivideGridColumns(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    i = 0;
    while i < (*grid).width - 2 {
        // grid->points[i][x] is an interpolating control point
        // grid->points[i+1][x] is an aproximating control point
        // grid->points[i+2][x] is an interpolating control point
        //
        // first see if we can collapse the aproximating collumn away
        //
        j = 0;
        while j < (*grid).height {
            if CM_NeedsSubdivision(
                (*grid).points[i as usize][j as usize].as_mut_ptr(),
                (*grid).points[(i + 1) as usize][j as usize].as_mut_ptr(),
                (*grid).points[(i + 2) as usize][j as usize].as_mut_ptr(),
            ) as u64
                != 0
            {
                break;
            }
            j += 1
        }
        if j == (*grid).height {
            // the new aproximating point at i+1 may need to be removed
            // or subdivided farther, so don't advance i
            // all of the points were close enough to the linear midpoints
            // that we can collapse the entire column away
            j = 0;
            while j < (*grid).height {
                // remove the column
                k = i + 2;
                while k < (*grid).width {
                    (*grid).points[(k - 1) as usize][j as usize][0] =
                        (*grid).points[k as usize][j as usize][0];
                    (*grid).points[(k - 1) as usize][j as usize][1] =
                        (*grid).points[k as usize][j as usize][1];
                    (*grid).points[(k - 1) as usize][j as usize][2] =
                        (*grid).points[k as usize][j as usize][2];
                    k += 1
                }
                j += 1
            }
            (*grid).width -= 1;
            // go to the next curve segment
            i += 1
        } else {
            //
            // we need to subdivide the curve
            //
            j = 0;
            while j < (*grid).height {
                let mut prev: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut next: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                // save the control points now
                prev[0] = (*grid).points[i as usize][j as usize][0];
                prev[1] = (*grid).points[i as usize][j as usize][1];
                prev[2] = (*grid).points[i as usize][j as usize][2];
                mid[0] = (*grid).points[(i + 1) as usize][j as usize][0];
                mid[1] = (*grid).points[(i + 1) as usize][j as usize][1];
                mid[2] = (*grid).points[(i + 1) as usize][j as usize][2];
                next[0] = (*grid).points[(i + 2) as usize][j as usize][0];
                next[1] = (*grid).points[(i + 2) as usize][j as usize][1];
                next[2] = (*grid).points[(i + 2) as usize][j as usize][2];
                // make room for two additional columns in the grid
                // columns i+1 will be replaced, column i+2 will become i+4
                // i+1, i+2, and i+3 will be generated
                k = (*grid).width - 1;
                while k > i + 1 {
                    (*grid).points[(k + 2) as usize][j as usize][0] =
                        (*grid).points[k as usize][j as usize][0];
                    (*grid).points[(k + 2) as usize][j as usize][1] =
                        (*grid).points[k as usize][j as usize][1];
                    (*grid).points[(k + 2) as usize][j as usize][2] =
                        (*grid).points[k as usize][j as usize][2];
                    k -= 1
                }
                // generate the subdivided points
                CM_Subdivide(
                    prev.as_mut_ptr(),
                    mid.as_mut_ptr(),
                    next.as_mut_ptr(),
                    (*grid).points[(i + 1) as usize][j as usize].as_mut_ptr(),
                    (*grid).points[(i + 2) as usize][j as usize].as_mut_ptr(),
                    (*grid).points[(i + 3) as usize][j as usize].as_mut_ptr(),
                );
                j += 1
            }
            (*grid).width += 2
        }
    }
}

unsafe extern "C" fn CM_ComparePoints(
    mut a: *mut f32,
    mut b: *mut f32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut d: f32 = 0.;
    d = *a.offset(0) - *b.offset(0);
    if (d as f64) < -0.1 || d as f64 > 0.1 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    d = *a.offset(1) - *b.offset(1);
    if (d as f64) < -0.1 || d as f64 > 0.1 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    d = *a.offset(2) - *b.offset(2);
    if (d as f64) < -0.1 || d as f64 > 0.1 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
CM_RemoveDegenerateColumns

If there are any identical columns, remove them
=================
*/

unsafe extern "C" fn CM_RemoveDegenerateColumns(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut _k: i32 = 0;
    i = 0;
    while i < (*grid).width - 1 {
        j = 0;
        while j < (*grid).height {
            if CM_ComparePoints(
                (*grid).points[i as usize][j as usize].as_mut_ptr(),
                (*grid).points[(i + 1) as usize][j as usize].as_mut_ptr(),
            ) as u64
                == 0
            {
                break;
            }
            j += 1
        }
        if !(j != (*grid).height) {
            j = 0;
            while j < (*grid).height {
                // remove the column

                for k in i + 2..(*grid).width {
                    (*grid).points[(k - 1) as usize][j as usize][0] =
                        (*grid).points[k as usize][j as usize][0];

                    (*grid).points[(k - 1) as usize][j as usize][1] =
                        (*grid).points[k as usize][j as usize][1];

                    (*grid).points[(k - 1) as usize][j as usize][2] =
                        (*grid).points[k as usize][j as usize][2];
                }
                j += 1
            }
            (*grid).width -= 1;
            // check against the next column
            i -= 1
        }
        i += 1
        // not degenerate
    }
}
/*
================================================================================

PATCH COLLIDE GENERATION

================================================================================
*/

static mut numPlanes: i32 = 0;

static mut planes: [crate::src::qcommon::cm_patch::patchPlane_t; 2048] =
    [crate::src::qcommon::cm_patch::patchPlane_t {
        plane: [0.; 4],
        signbits: 0,
    }; 2048];

static mut numFacets: i32 = 0;

static mut facets: [crate::src::qcommon::cm_patch::facet_t; 1024] =
    [crate::src::qcommon::cm_patch::facet_t {
        surfacePlane: 0,
        numBorders: 0,
        borderPlanes: [0; 26],
        borderInward: [0; 26],
        borderNoAdjust: [crate::src::qcommon::q_shared::qfalse; 26],
    }; 1024];
/*
==================
CM_PlaneEqual
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_PlaneEqual(
    mut p: *mut crate::src::qcommon::cm_patch::patchPlane_t,
    mut plane: *mut f32,
    mut flipped: *mut i32,
) -> i32 {
    let mut invplane: [f32; 4] = [0.; 4];
    if crate::stdlib::fabs(((*p).plane[0] - *plane.offset(0)) as f64) < 0.0001
        && crate::stdlib::fabs(((*p).plane[1] - *plane.offset(1)) as f64) < 0.0001
        && crate::stdlib::fabs(((*p).plane[2] - *plane.offset(2)) as f64) < 0.0001
        && crate::stdlib::fabs(((*p).plane[3] - *plane.offset(3)) as f64) < 0.02
    {
        *flipped = crate::src::qcommon::q_shared::qfalse as i32;
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    invplane[0] = -*plane.offset(0);
    invplane[1] = -*plane.offset(1);
    invplane[2] = -*plane.offset(2);
    invplane[3] = -*plane.offset(3);
    if crate::stdlib::fabs(((*p).plane[0] - invplane[0]) as f64) < 0.0001
        && crate::stdlib::fabs(((*p).plane[1] - invplane[1]) as f64) < 0.0001
        && crate::stdlib::fabs(((*p).plane[2] - invplane[2]) as f64) < 0.0001
        && crate::stdlib::fabs(((*p).plane[3] - invplane[3]) as f64) < 0.02
    {
        *flipped = crate::src::qcommon::q_shared::qtrue as i32;
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
CM_SnapVector
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_SnapVector(mut normal: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut i: i32 = 0;
    i = 0;
    while i < 3 {
        if crate::stdlib::fabs((*normal.offset(i as isize) - 1f32) as f64) < 0.0001 {
            let ref mut fresh0 = *normal.offset(2);
            *fresh0 = 0f32;
            let ref mut fresh1 = *normal.offset(1);
            *fresh1 = *fresh0;
            *normal.offset(0) = *fresh1;
            *normal.offset(i as isize) = 1f32;
            break;
        } else if crate::stdlib::fabs((*normal.offset(i as isize) - -1f32) as f64) < 0.0001 {
            let ref mut fresh2 = *normal.offset(2);
            *fresh2 = 0f32;
            let ref mut fresh3 = *normal.offset(1);
            *fresh3 = *fresh2;
            *normal.offset(0) = *fresh3;
            *normal.offset(i as isize) = -1f32;
            break;
        } else {
            i += 1
        }
    }
}
/*
==================
CM_FindPlane2
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_FindPlane2(mut plane: *mut f32, mut flipped: *mut i32) -> i32 {
    let mut _i: i32 = 0;
    // see if the points are close enough to an existing plane

    for i in 0..numPlanes {
        if CM_PlaneEqual(&mut *planes.as_mut_ptr().offset(i as isize), plane, flipped) != 0 {
            return i;
        }
    }
    // add a new plane
    if numPlanes == 2048 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"MAX_PATCH_PLANES\x00" as *const u8 as *const i8,
        );
    }
    planes[numPlanes as usize].plane[0] = *plane.offset(0);
    planes[numPlanes as usize].plane[1] = *plane.offset(1);
    planes[numPlanes as usize].plane[2] = *plane.offset(2);
    planes[numPlanes as usize].plane[3] = *plane.offset(3);
    planes[numPlanes as usize].signbits = CM_SignbitsForNormal(plane);
    numPlanes += 1;
    *flipped = crate::src::qcommon::q_shared::qfalse as i32;
    return numPlanes - 1;
}
/*
==================
CM_FindPlane
==================
*/

unsafe extern "C" fn CM_FindPlane(mut p1: *mut f32, mut p2: *mut f32, mut p3: *mut f32) -> i32 {
    let mut plane: [f32; 4] = [0.; 4];
    let mut _i: i32 = 0;
    let mut d: f32 = 0.;
    if CM_PlaneFromPoints(plane.as_mut_ptr(), p1, p2, p3) as u64 == 0 {
        return -(1i32);
    }
    // see if the points are close enough to an existing plane

    for i in 0..numPlanes {
        if !(plane[0] * planes[i as usize].plane[0]
            + plane[1] * planes[i as usize].plane[1]
            + plane[2] * planes[i as usize].plane[2]
            < 0f32)
        {
            d = *p1.offset(0) * planes[i as usize].plane[0]
                + *p1.offset(1) * planes[i as usize].plane[1]
                + *p1.offset(2) * planes[i as usize].plane[2]
                - planes[i as usize].plane[3];
            if !((d as f64) < -0.1 || d as f64 > 0.1) {
                d = *p2.offset(0) * planes[i as usize].plane[0]
                    + *p2.offset(1) * planes[i as usize].plane[1]
                    + *p2.offset(2) * planes[i as usize].plane[2]
                    - planes[i as usize].plane[3];
                if !((d as f64) < -0.1 || d as f64 > 0.1) {
                    d = *p3.offset(0) * planes[i as usize].plane[0]
                        + *p3.offset(1) * planes[i as usize].plane[1]
                        + *p3.offset(2) * planes[i as usize].plane[2]
                        - planes[i as usize].plane[3];
                    if !((d as f64) < -0.1 || d as f64 > 0.1) {
                        // found it
                        return i;
                    }
                }
            }
        }
    }
    // add a new plane
    if numPlanes == 2048 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"MAX_PATCH_PLANES\x00" as *const u8 as *const i8,
        );
    }
    planes[numPlanes as usize].plane[0] = plane[0];
    planes[numPlanes as usize].plane[1] = plane[1];
    planes[numPlanes as usize].plane[2] = plane[2];
    planes[numPlanes as usize].plane[3] = plane[3];
    planes[numPlanes as usize].signbits = CM_SignbitsForNormal(plane.as_mut_ptr());
    numPlanes += 1;
    return numPlanes - 1;
}
/*
==================
CM_PointOnPlaneSide
==================
*/

unsafe extern "C" fn CM_PointOnPlaneSide(mut p: *mut f32, mut planeNum: i32) -> i32 {
    let mut plane: *mut f32 = 0 as *mut f32;
    let mut d: f32 = 0.;
    if planeNum == -(1) {
        return 2i32;
    }
    plane = planes[planeNum as usize].plane.as_mut_ptr();
    d = *p.offset(0) * *plane.offset(0)
        + *p.offset(1) * *plane.offset(1)
        + *p.offset(2) * *plane.offset(2)
        - *plane.offset(3);
    if d as f64 > 0.1 {
        return 0i32;
    }
    if (d as f64) < -0.1 {
        return 1i32;
    }
    return 2;
}
/*
==================
CM_GridPlane
==================
*/

unsafe extern "C" fn CM_GridPlane(
    mut gridPlanes: *mut [[i32; 2]; 129],
    mut i: i32,
    mut j: i32,
    mut tri: i32,
) -> i32 {
    let mut p: i32 = 0;
    p = (*gridPlanes.offset(i as isize))[j as usize][tri as usize];
    if p != -(1) {
        return p;
    }
    p = (*gridPlanes.offset(i as isize))[j as usize][(tri == 0) as i32 as usize];
    if p != -(1) {
        return p;
    }
    // should never happen
    crate::src::qcommon::common::Com_Printf(
        b"WARNING: CM_GridPlane unresolvable\n\x00" as *const u8 as *const i8,
    );
    return -(1);
}
/*
==================
CM_EdgePlaneNum
==================
*/

unsafe extern "C" fn CM_EdgePlaneNum(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
    mut gridPlanes: *mut [[i32; 2]; 129],
    mut i: i32,
    mut j: i32,
    mut k: i32,
) -> i32 {
    let mut p1: *mut f32 = 0 as *mut f32;
    let mut p2: *mut f32 = 0 as *mut f32;
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: i32 = 0;
    match k {
        0 => {
            // top border
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0);
            if p == -(1) {
                return -(1i32);
            }
            up[0] = *p1.offset(0) + planes[p as usize].plane[0] * 4f32;
            up[1] = *p1.offset(1) + planes[p as usize].plane[1] * 4f32;
            up[2] = *p1.offset(2) + planes[p as usize].plane[2] * 4f32;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        2 => {
            // bottom border
            p1 = (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1);
            if p == -(1) {
                return -(1i32);
            }
            up[0] = *p1.offset(0) + planes[p as usize].plane[0] * 4f32;
            up[1] = *p1.offset(1) + planes[p as usize].plane[1] * 4f32;
            up[2] = *p1.offset(2) + planes[p as usize].plane[2] * 4f32;
            return CM_FindPlane(p2, p1, up.as_mut_ptr());
        }
        3 => {
            // left border
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1);
            if p == -(1) {
                return -(1i32);
            }
            up[0] = *p1.offset(0) + planes[p as usize].plane[0] * 4f32;
            up[1] = *p1.offset(1) + planes[p as usize].plane[1] * 4f32;
            up[2] = *p1.offset(2) + planes[p as usize].plane[2] * 4f32;
            return CM_FindPlane(p2, p1, up.as_mut_ptr());
        }
        1 => {
            // right border
            p1 = (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0);
            if p == -(1) {
                return -(1i32);
            }
            up[0] = *p1.offset(0) + planes[p as usize].plane[0] * 4f32;
            up[1] = *p1.offset(1) + planes[p as usize].plane[1] * 4f32;
            up[2] = *p1.offset(2) + planes[p as usize].plane[2] * 4f32;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        4 => {
            // diagonal out of triangle 0
            p1 = (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize].as_mut_ptr();
            p2 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 0);
            if p == -(1) {
                return -(1i32);
            }
            up[0] = *p1.offset(0) + planes[p as usize].plane[0] * 4f32;
            up[1] = *p1.offset(1) + planes[p as usize].plane[1] * 4f32;
            up[2] = *p1.offset(2) + planes[p as usize].plane[2] * 4f32;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        5 => {
            // diagonal out of triangle 1
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize].as_mut_ptr();
            p = CM_GridPlane(gridPlanes, i, j, 1);
            if p == -(1) {
                return -(1i32);
            }
            up[0] = *p1.offset(0) + planes[p as usize].plane[0] * 4f32;
            up[1] = *p1.offset(1) + planes[p as usize].plane[1] * 4f32;
            up[2] = *p1.offset(2) + planes[p as usize].plane[2] * 4f32;
            return CM_FindPlane(p1, p2, up.as_mut_ptr());
        }
        _ => {}
    }
    crate::src::qcommon::common::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"CM_EdgePlaneNum: bad k\x00" as *const u8 as *const i8,
    );
}
/*
===================
CM_SetBorderInward
===================
*/

unsafe extern "C" fn CM_SetBorderInward(
    mut facet: *mut crate::src::qcommon::cm_patch::facet_t,
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
    mut _gridPlanes: *mut [[i32; 2]; 129],
    mut i: i32,
    mut j: i32,
    mut which: i32,
) {
    let mut k: i32 = 0;
    let mut _l: i32 = 0;
    let mut points: [*mut f32; 4] = [0 as *mut f32; 4];
    let mut numPoints: i32 = 0;
    match which {
        -1 => {
            points[0] = (*grid).points[i as usize][j as usize].as_mut_ptr();
            points[1] = (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            points[2] = (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize].as_mut_ptr();
            points[3] = (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            numPoints = 4
        }
        0 => {
            points[0] = (*grid).points[i as usize][j as usize].as_mut_ptr();
            points[1] = (*grid).points[(i + 1i32) as usize][j as usize].as_mut_ptr();
            points[2] = (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize].as_mut_ptr();
            numPoints = 3
        }
        1 => {
            points[0] = (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize].as_mut_ptr();
            points[1] = (*grid).points[i as usize][(j + 1i32) as usize].as_mut_ptr();
            points[2] = (*grid).points[i as usize][j as usize].as_mut_ptr();
            numPoints = 3
        }
        _ => {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_FATAL as i32,
                b"CM_SetBorderInward: bad parameter\x00" as *const u8 as *const i8,
            );
        }
    }
    k = 0;
    while k < (*facet).numBorders {
        let mut front: i32 = 0;
        let mut back: i32 = 0;
        front = 0;
        back = 0;

        for l in 0..numPoints {
            let mut side: i32 = 0;

            side = CM_PointOnPlaneSide(points[l as usize], (*facet).borderPlanes[k as usize]);

            if side == 0 {
                front += 1
            }

            if side == 1 {
                back += 1
            }
        }
        if front != 0 && back == 0 {
            (*facet).borderInward[k as usize] = crate::src::qcommon::q_shared::qtrue as i32
        } else if back != 0 && front == 0 {
            (*facet).borderInward[k as usize] = crate::src::qcommon::q_shared::qfalse as i32
        } else if front == 0 && back == 0 {
            // flat side border
            (*facet).borderPlanes[k as usize] = -(1)
        } else {
            // bisecting side border
            crate::src::qcommon::common::Com_DPrintf(
                b"WARNING: CM_SetBorderInward: mixed plane sides\n\x00" as *const u8 as *const i8,
            );
            (*facet).borderInward[k as usize] = crate::src::qcommon::q_shared::qfalse as i32;
            if debugBlock as u64 == 0 {
                debugBlock = crate::src::qcommon::q_shared::qtrue;
                debugBlockPoints[0][0] = (*grid).points[i as usize][j as usize][0];
                debugBlockPoints[0][1] = (*grid).points[i as usize][j as usize][1];
                debugBlockPoints[0][2] = (*grid).points[i as usize][j as usize][2];
                debugBlockPoints[1][0] = (*grid).points[(i + 1i32) as usize][j as usize][0];
                debugBlockPoints[1][1] = (*grid).points[(i + 1i32) as usize][j as usize][1];
                debugBlockPoints[1][2] = (*grid).points[(i + 1i32) as usize][j as usize][2];
                debugBlockPoints[2][0] =
                    (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize][0];
                debugBlockPoints[2][1] =
                    (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize][1];
                debugBlockPoints[2][2] =
                    (*grid).points[(i + 1i32) as usize][(j + 1i32) as usize][2];
                debugBlockPoints[3][0] = (*grid).points[i as usize][(j + 1i32) as usize][0];
                debugBlockPoints[3][1] = (*grid).points[i as usize][(j + 1i32) as usize][1];
                debugBlockPoints[3][2] = (*grid).points[i as usize][(j + 1i32) as usize][2]
            }
        }
        k += 1
    }
}
/*
==================
CM_ValidateFacet

If the facet isn't bounded by its borders, we screwed up.
==================
*/

unsafe extern "C" fn CM_ValidateFacet(
    mut facet: *mut crate::src::qcommon::cm_patch::facet_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut plane: [f32; 4] = [0.; 4];
    let mut j: i32 = 0;
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut bounds: [crate::src::qcommon::q_shared::vec3_t; 2] = [[0.; 3]; 2];
    if (*facet).surfacePlane == -(1) {
        return crate::src::qcommon::q_shared::qfalse;
    }
    plane[0] = planes[(*facet).surfacePlane as usize].plane[0];
    plane[1] = planes[(*facet).surfacePlane as usize].plane[1];
    plane[2] = planes[(*facet).surfacePlane as usize].plane[2];
    plane[3] = planes[(*facet).surfacePlane as usize].plane[3];
    w = crate::src::qcommon::cm_polylib::BaseWindingForPlane(plane.as_mut_ptr(), plane[3]);
    j = 0;
    while j < (*facet).numBorders && !w.is_null() {
        if (*facet).borderPlanes[j as usize] == -(1) {
            crate::src::qcommon::cm_polylib::FreeWinding(w);
            return crate::src::qcommon::q_shared::qfalse;
        }
        plane[0] = planes[(*facet).borderPlanes[j as usize] as usize].plane[0];
        plane[1] = planes[(*facet).borderPlanes[j as usize] as usize].plane[1];
        plane[2] = planes[(*facet).borderPlanes[j as usize] as usize].plane[2];
        plane[3] = planes[(*facet).borderPlanes[j as usize] as usize].plane[3];
        if (*facet).borderInward[j as usize] == 0 {
            plane[0] = crate::src::qcommon::q_math::vec3_origin[0] - plane[0];
            plane[1] = crate::src::qcommon::q_math::vec3_origin[1] - plane[1];
            plane[2] = crate::src::qcommon::q_math::vec3_origin[2] - plane[2];
            plane[3] = -plane[3]
        }
        crate::src::qcommon::cm_polylib::ChopWindingInPlace(
            &mut w,
            plane.as_mut_ptr(),
            plane[3],
            0.1,
        );
        j += 1
    }
    if w.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
        // winding was completely chopped away
    }
    // see if the facet is unreasonably large
    crate::src::qcommon::cm_polylib::WindingBounds(
        w,
        bounds[0].as_mut_ptr(),
        bounds[1].as_mut_ptr(),
    );
    crate::src::qcommon::cm_polylib::FreeWinding(w);
    j = 0;
    while j < 3 {
        if bounds[1][j as usize] - bounds[0][j as usize] > 65535f32 {
            return crate::src::qcommon::q_shared::qfalse;
            // we must be missing a plane
        }
        if bounds[0][j as usize] >= 65535f32 {
            return crate::src::qcommon::q_shared::qfalse;
        }
        if bounds[1][j as usize] <= -65535f32 {
            return crate::src::qcommon::q_shared::qfalse;
        }
        j += 1
    }
    return crate::src::qcommon::q_shared::qtrue;
    // winding is fine
}
/*
==================
CM_AddFacetBevels
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_AddFacetBevels(mut facet: *mut crate::src::qcommon::cm_patch::facet_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut axis: i32 = 0;
    let mut dir: i32 = 0;
    let mut flipped: i32 = 0;
    let mut plane: [f32; 4] = [0.; 4];
    let mut d: f32 = 0.;
    let mut newplane: [f32; 4] = [0.; 4];
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut w2: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    plane[0] = planes[(*facet).surfacePlane as usize].plane[0];
    plane[1] = planes[(*facet).surfacePlane as usize].plane[1];
    plane[2] = planes[(*facet).surfacePlane as usize].plane[2];
    plane[3] = planes[(*facet).surfacePlane as usize].plane[3];
    w = crate::src::qcommon::cm_polylib::BaseWindingForPlane(plane.as_mut_ptr(), plane[3]);
    j = 0;
    while j < (*facet).numBorders && !w.is_null() {
        if !((*facet).borderPlanes[j as usize] == (*facet).surfacePlane) {
            plane[0] = planes[(*facet).borderPlanes[j as usize] as usize].plane[0];
            plane[1] = planes[(*facet).borderPlanes[j as usize] as usize].plane[1];
            plane[2] = planes[(*facet).borderPlanes[j as usize] as usize].plane[2];
            plane[3] = planes[(*facet).borderPlanes[j as usize] as usize].plane[3];
            if (*facet).borderInward[j as usize] == 0 {
                plane[0] = crate::src::qcommon::q_math::vec3_origin[0] - plane[0];
                plane[1] = crate::src::qcommon::q_math::vec3_origin[1] - plane[1];
                plane[2] = crate::src::qcommon::q_math::vec3_origin[2] - plane[2];
                plane[3] = -plane[3]
            }
            crate::src::qcommon::cm_polylib::ChopWindingInPlace(
                &mut w,
                plane.as_mut_ptr(),
                plane[3usize],
                0.1f32,
            );
        }
        j += 1
    }
    if w.is_null() {
        return;
    }
    crate::src::qcommon::cm_polylib::WindingBounds(w, mins.as_mut_ptr(), maxs.as_mut_ptr());
    // add the axial planes
    axis = 0;
    while axis < 3 {
        dir = -(1);
        while dir <= 1 {
            plane[2] = 0f32;
            plane[1] = plane[2];
            plane[0] = plane[1];
            plane[axis as usize] = dir as f32;
            if dir == 1 {
                plane[3] = maxs[axis as usize]
            } else {
                plane[3] = -mins[axis as usize]
            }
            //if it's the surface plane
            if !(CM_PlaneEqual(
                &mut *planes.as_mut_ptr().offset((*facet).surfacePlane as isize),
                plane.as_mut_ptr(),
                &mut flipped,
            ) != 0)
            {
                // see if the plane is already present
                i = 0;
                while i < (*facet).numBorders {
                    if CM_PlaneEqual(&mut *planes.as_mut_ptr().offset(*(*facet).borderPlanes.as_mut_ptr().offset(i
                                                                                                                     as
                                                                                                                     isize)
                                                                          as
                                                                          isize),
                                     plane.as_mut_ptr(), &mut flipped) != 0 {
                        break ;
                    }
                    i += 1
                }
                if i == (*facet).numBorders {
                    if (*facet).numBorders >= 4 + 6 + 16 {
                        crate::src::qcommon::common::Com_Printf(
                            b"ERROR: too many bevels\n\x00" as *const u8 as *const i8,
                        );
                    } else {
                        (*facet).borderPlanes[(*facet).numBorders as usize] =
                            CM_FindPlane2(plane.as_mut_ptr(), &mut flipped);
                        (*facet).borderNoAdjust[(*facet).numBorders as usize] =
                            crate::src::qcommon::q_shared::qfalse;
                        (*facet).borderInward[(*facet).numBorders as usize] = flipped;
                        (*facet).numBorders += 1
                    }
                }
            }
            dir += 2
        }
        axis += 1
    }
    //
    // add the edge bevels
    //
    // test the non-axial plane edges
    j = 0;
    while j < (*w).numpoints {
        k = (j + 1) % (*w).numpoints;
        vec[0] = (*(*w).p.as_mut_ptr().offset(j as isize))[0]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[0];
        vec[1] = (*(*w).p.as_mut_ptr().offset(j as isize))[1]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[1];
        vec[2] = (*(*w).p.as_mut_ptr().offset(j as isize))[2]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[2];
        //if it's a degenerate edge
        if !((crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr()) as f64) < 0.5) {
            CM_SnapVector(vec.as_mut_ptr()); // axial
            k = 0; // only test non-axial edges
            while k < 3 {
                if vec[k as usize] == -1f32 || vec[k as usize] == 1f32 {
                    break;
                }
                k += 1
            }
            if !(k < 3) {
                // try the six possible slanted axials from this edge
                axis = 0;
                while axis < 3 {
                    dir = -(1);
                    while dir <= 1 {
                        // construct a plane
                        vec2[2] = 0f32;
                        vec2[1] = vec2[2];
                        vec2[0] = vec2[1];
                        vec2[axis as usize] = dir as crate::src::qcommon::q_shared::vec_t;
                        CrossProduct(
                            vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            vec2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                            plane.as_mut_ptr(),
                        );
                        if !((crate::src::qcommon::q_math::VectorNormalize(plane.as_mut_ptr())
                            as f64)
                            < 0.5)
                        {
                            plane[3] = (*(*w).p.as_mut_ptr().offset(j as isize))[0] * plane[0]
                                + (*(*w).p.as_mut_ptr().offset(j as isize))[1] * plane[1]
                                + (*(*w).p.as_mut_ptr().offset(j as isize))[2] * plane[2];
                            // if all the points of the facet winding are
                            // behind this plane, it is a proper edge bevel
                            l = 0;
                            while l < (*w).numpoints {
                                d = (*(*w).p.as_mut_ptr().offset(l as isize))[0] * plane[0]
                                    + (*(*w).p.as_mut_ptr().offset(l as isize))[1] * plane[1]
                                    + (*(*w).p.as_mut_ptr().offset(l as isize))[2] * plane[2]
                                    - plane[3];
                                if d as f64 > 0.1 {
                                    break;
                                }
                                l += 1
                                // point in front
                            }
                            if !(l < (*w).numpoints) {
                                //if it's the surface plane
                                if !(CM_PlaneEqual(
                                    &mut *planes
                                        .as_mut_ptr()
                                        .offset((*facet).surfacePlane as isize),
                                    plane.as_mut_ptr(),
                                    &mut flipped,
                                ) != 0)
                                {
                                    // see if the plane is already present
                                    i = 0;
                                    while i < (*facet).numBorders {
                                        if CM_PlaneEqual(
                                            &mut *planes.as_mut_ptr().offset(
                                                *(*facet)
                                                    .borderPlanes
                                                    .as_mut_ptr()
                                                    .offset(i as isize)
                                                    as isize,
                                            ),
                                            plane.as_mut_ptr(),
                                            &mut flipped,
                                        ) != 0
                                        {
                                            break;
                                        }
                                        i += 1
                                    }
                                    if i == (*facet).numBorders {
                                        if (*facet).numBorders >= 4 + 6 + 16 {
                                            crate::src::qcommon::common::Com_Printf(
                                                b"ERROR: too many bevels\n\x00" as *const u8
                                                    as *const i8,
                                            );
                                        } else {
                                            (*facet).borderPlanes[(*facet).numBorders as usize] =
                                                CM_FindPlane2(plane.as_mut_ptr(), &mut flipped);
                                            k = 0;
                                            while k < (*facet).numBorders {
                                                if (*facet).borderPlanes
                                                    [(*facet).numBorders as usize]
                                                    == (*facet).borderPlanes[k as usize]
                                                {
                                                    crate::src::qcommon::common::Com_Printf(
                                                        b"WARNING: bevel plane already used\n\x00"
                                                            as *const u8
                                                            as *const i8,
                                                    );
                                                }
                                                k += 1
                                            }
                                            (*facet).borderNoAdjust[(*facet).numBorders as usize] =
                                                crate::src::qcommon::q_shared::qfalse;
                                            (*facet).borderInward[(*facet).numBorders as usize] =
                                                flipped;
                                            //
                                            w2 = crate::src::qcommon::cm_polylib::CopyWinding(w); //end if
                                            newplane[0] = planes[(*facet).borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane[0];
                                            newplane[1] = planes[(*facet).borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane[1];
                                            newplane[2] = planes[(*facet).borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane[2];
                                            newplane[3] = planes[(*facet).borderPlanes
                                                [(*facet).numBorders as usize]
                                                as usize]
                                                .plane[3];
                                            if (*facet).borderInward[(*facet).numBorders as usize]
                                                == 0
                                            {
                                                newplane[0] = -newplane[0];
                                                newplane[1] = -newplane[1];
                                                newplane[2] = -newplane[2];
                                                newplane[3] = -newplane[3]
                                            }
                                            crate::src::qcommon::cm_polylib::ChopWindingInPlace(
                                                &mut w2,
                                                newplane.as_mut_ptr(),
                                                newplane[3],
                                                0.1,
                                            );
                                            if w2.is_null() {
                                                crate::src::qcommon::common::Com_DPrintf(b"WARNING: CM_AddFacetBevels... invalid bevel\n\x00"
                                                                as *const u8
                                                                as
                                                                *const i8);
                                            } else {
                                                crate::src::qcommon::cm_polylib::FreeWinding(w2);
                                                //
                                                (*facet).numBorders += 1
                                            }
                                        }
                                        //already got a bevel
                                        //					break;
                                    }
                                }
                            }
                        }
                        dir += 2
                    }
                    axis += 1
                }
            }
        }
        j += 1
    }
    crate::src::qcommon::cm_polylib::FreeWinding(w);
    //add opposite plane
    if (*facet).numBorders >= 4 + 6 + 16 {
        crate::src::qcommon::common::Com_Printf(
            b"ERROR: too many bevels\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    (*facet).borderPlanes[(*facet).numBorders as usize] = (*facet).surfacePlane;
    (*facet).borderNoAdjust[(*facet).numBorders as usize] = crate::src::qcommon::q_shared::qfalse;
    (*facet).borderInward[(*facet).numBorders as usize] =
        crate::src::qcommon::q_shared::qtrue as i32;
    (*facet).numBorders += 1;
    //BSPC
}
/*
==================
CM_PatchCollideFromGrid
==================
*/

unsafe extern "C" fn CM_PatchCollideFromGrid(
    mut grid: *mut crate::src::qcommon::cm_patch::cGrid_t,
    mut pf: *mut crate::src::qcommon::cm_patch::patchCollide_t,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p1: *mut f32 = 0 as *mut f32;
    let mut p2: *mut f32 = 0 as *mut f32;
    let mut p3: *mut f32 = 0 as *mut f32;
    let mut gridPlanes: [[[i32; 2]; 129]; 129] = [[[0; 2]; 129]; 129];
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut borders: [i32; 4] = [0; 4];
    let mut noAdjust: [i32; 4] = [0; 4];
    numPlanes = 0;
    numFacets = 0;
    // find the planes for each triangle of the grid
    i = 0;
    while i < (*grid).width - 1 {
        j = 0;
        while j < (*grid).height - 1 {
            p1 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            p2 = (*grid).points[(i + 1) as usize][j as usize].as_mut_ptr();
            p3 = (*grid).points[(i + 1) as usize][(j + 1) as usize].as_mut_ptr();
            gridPlanes[i as usize][j as usize][0] = CM_FindPlane(p1, p2, p3);
            p1 = (*grid).points[(i + 1) as usize][(j + 1) as usize].as_mut_ptr();
            p2 = (*grid).points[i as usize][(j + 1) as usize].as_mut_ptr();
            p3 = (*grid).points[i as usize][j as usize].as_mut_ptr();
            gridPlanes[i as usize][j as usize][1] = CM_FindPlane(p1, p2, p3);
            j += 1
        }
        i += 1
    }
    // create the borders for each facet
    i = 0;
    while i < (*grid).width - 1 {
        j = 0;
        while j < (*grid).height - 1 {
            borders[EN_TOP as usize] = -(1);
            if j > 0 {
                borders[EN_TOP as usize] = gridPlanes[i as usize][(j - 1) as usize][1]
            } else if (*grid).wrapHeight as u64 != 0 {
                borders[EN_TOP as usize] =
                    gridPlanes[i as usize][((*grid).height - 2i32) as usize][1]
            }
            noAdjust[EN_TOP as usize] =
                (borders[EN_TOP as usize] == gridPlanes[i as usize][j as usize][0]) as i32;
            if borders[EN_TOP as usize] == -(1) || noAdjust[EN_TOP as usize] != 0 {
                borders[EN_TOP as usize] = CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 0)
            }
            borders[EN_BOTTOM as usize] = -(1);
            if j < (*grid).height - 2 {
                borders[EN_BOTTOM as usize] = gridPlanes[i as usize][(j + 1) as usize][0]
            } else if (*grid).wrapHeight as u64 != 0 {
                borders[EN_BOTTOM as usize] = gridPlanes[i as usize][0][0]
            }
            noAdjust[EN_BOTTOM as usize] =
                (borders[EN_BOTTOM as usize] == gridPlanes[i as usize][j as usize][1]) as i32;
            if borders[EN_BOTTOM as usize] == -(1) || noAdjust[EN_BOTTOM as usize] != 0 {
                borders[EN_BOTTOM as usize] =
                    CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 2)
            }
            borders[EN_LEFT as usize] = -(1);
            if i > 0 {
                borders[EN_LEFT as usize] = gridPlanes[(i - 1) as usize][j as usize][0]
            } else if (*grid).wrapWidth as u64 != 0 {
                borders[EN_LEFT as usize] =
                    gridPlanes[((*grid).width - 2i32) as usize][j as usize][0]
            }
            noAdjust[EN_LEFT as usize] =
                (borders[EN_LEFT as usize] == gridPlanes[i as usize][j as usize][1]) as i32;
            if borders[EN_LEFT as usize] == -(1) || noAdjust[EN_LEFT as usize] != 0 {
                borders[EN_LEFT as usize] = CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 3)
            }
            borders[EN_RIGHT as usize] = -(1);
            if i < (*grid).width - 2 {
                borders[EN_RIGHT as usize] = gridPlanes[(i + 1) as usize][j as usize][1]
            } else if (*grid).wrapWidth as u64 != 0 {
                borders[EN_RIGHT as usize] = gridPlanes[0][j as usize][1]
            }
            noAdjust[EN_RIGHT as usize] =
                (borders[EN_RIGHT as usize] == gridPlanes[i as usize][j as usize][0]) as i32;
            if borders[EN_RIGHT as usize] == -(1) || noAdjust[EN_RIGHT as usize] != 0 {
                borders[EN_RIGHT as usize] = CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 1)
            }
            if numFacets == 1024 {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"MAX_FACETS\x00" as *const u8 as *const i8,
                );
            }
            facet = &mut *facets.as_mut_ptr().offset(numFacets as isize)
                as *mut crate::src::qcommon::cm_patch::facet_t;
            crate::stdlib::memset(
                facet as *mut libc::c_void,
                0,
                ::std::mem::size_of::<crate::src::qcommon::cm_patch::facet_t>(),
            );
            if gridPlanes[i as usize][j as usize][0] == gridPlanes[i as usize][j as usize][1] {
                if !(gridPlanes[i as usize][j as usize][0] == -(1)) {
                    (*facet).surfacePlane = gridPlanes[i as usize][j as usize][0];
                    (*facet).numBorders = 4;
                    (*facet).borderPlanes[0] = borders[EN_TOP as usize];
                    (*facet).borderNoAdjust[0] =
                        noAdjust[EN_TOP as usize] as crate::src::qcommon::q_shared::qboolean;
                    (*facet).borderPlanes[1] = borders[EN_RIGHT as usize];
                    (*facet).borderNoAdjust[1] =
                        noAdjust[EN_RIGHT as usize] as crate::src::qcommon::q_shared::qboolean;
                    (*facet).borderPlanes[2] = borders[EN_BOTTOM as usize];
                    (*facet).borderNoAdjust[2] =
                        noAdjust[EN_BOTTOM as usize] as crate::src::qcommon::q_shared::qboolean;
                    (*facet).borderPlanes[3] = borders[EN_LEFT as usize];
                    (*facet).borderNoAdjust[3] =
                        noAdjust[EN_LEFT as usize] as crate::src::qcommon::q_shared::qboolean;
                    CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(), i, j, -(1));
                    if CM_ValidateFacet(facet) as u64 != 0 {
                        CM_AddFacetBevels(facet);
                        numFacets += 1
                    }
                }
            } else {
                // two separate triangles
                (*facet).surfacePlane = gridPlanes[i as usize][j as usize][0];
                (*facet).numBorders = 3;
                (*facet).borderPlanes[0] = borders[EN_TOP as usize];
                (*facet).borderNoAdjust[0] =
                    noAdjust[EN_TOP as usize] as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[1] = borders[EN_RIGHT as usize];
                (*facet).borderNoAdjust[1] =
                    noAdjust[EN_RIGHT as usize] as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[2] = gridPlanes[i as usize][j as usize][1];
                if (*facet).borderPlanes[2] == -(1) {
                    (*facet).borderPlanes[2] = borders[EN_BOTTOM as usize];
                    if (*facet).borderPlanes[2] == -(1) {
                        (*facet).borderPlanes[2] =
                            CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 4)
                    }
                }
                CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(), i, j, 0);
                if CM_ValidateFacet(facet) as u64 != 0 {
                    CM_AddFacetBevels(facet);
                    numFacets += 1
                }
                if numFacets == 1024 {
                    crate::src::qcommon::common::Com_Error(
                        crate::src::qcommon::q_shared::ERR_DROP as i32,
                        b"MAX_FACETS\x00" as *const u8 as *const i8,
                    );
                }
                facet = &mut *facets.as_mut_ptr().offset(numFacets as isize)
                    as *mut crate::src::qcommon::cm_patch::facet_t;
                crate::stdlib::memset(
                    facet as *mut libc::c_void,
                    0,
                    ::std::mem::size_of::<crate::src::qcommon::cm_patch::facet_t>(),
                );
                (*facet).surfacePlane = gridPlanes[i as usize][j as usize][1];
                (*facet).numBorders = 3;
                (*facet).borderPlanes[0] = borders[EN_BOTTOM as usize];
                (*facet).borderNoAdjust[0] =
                    noAdjust[EN_BOTTOM as usize] as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[1] = borders[EN_LEFT as usize];
                (*facet).borderNoAdjust[1] =
                    noAdjust[EN_LEFT as usize] as crate::src::qcommon::q_shared::qboolean;
                (*facet).borderPlanes[2] = gridPlanes[i as usize][j as usize][0];
                if (*facet).borderPlanes[2] == -(1) {
                    (*facet).borderPlanes[2] = borders[EN_TOP as usize];
                    if (*facet).borderPlanes[2] == -(1) {
                        (*facet).borderPlanes[2] =
                            CM_EdgePlaneNum(grid, gridPlanes.as_mut_ptr(), i, j, 5)
                    }
                }
                CM_SetBorderInward(facet, grid, gridPlanes.as_mut_ptr(), i, j, 1);
                if CM_ValidateFacet(facet) as u64 != 0 {
                    CM_AddFacetBevels(facet);
                    numFacets += 1
                }
            }
            j += 1
            // degenrate
        }
        i += 1
    }
    // copy the results out
    (*pf).numPlanes = numPlanes;
    (*pf).numFacets = numFacets;
    (*pf).facets = crate::src::qcommon::common::Hunk_Alloc(
        (numFacets as usize)
            .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::cm_patch::facet_t>())
            as i32,
        crate::src::qcommon::q_shared::h_high,
    ) as *mut crate::src::qcommon::cm_patch::facet_t;
    crate::stdlib::memcpy(
        (*pf).facets as *mut libc::c_void,
        facets.as_mut_ptr() as *const libc::c_void,
        (numFacets as usize)
            .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::cm_patch::facet_t>()),
    );
    (*pf).planes = crate::src::qcommon::common::Hunk_Alloc(
        (numPlanes as usize).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::cm_patch::patchPlane_t,
        >()) as i32,
        crate::src::qcommon::q_shared::h_high,
    ) as *mut crate::src::qcommon::cm_patch::patchPlane_t;
    crate::stdlib::memcpy(
        (*pf).planes as *mut libc::c_void,
        planes.as_mut_ptr() as *const libc::c_void,
        (numPlanes as usize).wrapping_mul(::std::mem::size_of::<
            crate::src::qcommon::cm_patch::patchPlane_t,
        >()),
    );
}
/*
===================
CM_GeneratePatchCollide

Creates an internal structure that will be used to perform
collision detection with a patch mesh.

Points is packed as concatenated rows.
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_GeneratePatchCollide(
    mut width: i32,
    mut height: i32,
    mut points: *mut crate::src::qcommon::q_shared::vec3_t,
) -> *mut crate::src::qcommon::cm_patch::patchCollide_s {
    let mut pf: *mut crate::src::qcommon::cm_patch::patchCollide_t =
        0 as *mut crate::src::qcommon::cm_patch::patchCollide_t;
    let mut grid: crate::src::qcommon::cm_patch::cGrid_t = crate::src::qcommon::cm_patch::cGrid_t {
        width: 0,
        height: 0,
        wrapWidth: crate::src::qcommon::q_shared::qfalse,
        wrapHeight: crate::src::qcommon::q_shared::qfalse,
        points: [[[0.; 3]; 129]; 129],
    };
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if width <= 2 || height <= 2 || points.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CM_GeneratePatchFacets: bad parameters: (%i, %i, %p)\x00" as *const u8 as *const i8,
            width,
            height,
            points as *mut libc::c_void,
        );
    }
    if width & 1 == 0 || height & 1 == 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CM_GeneratePatchFacets: even sizes are invalid for quadratic meshes\x00" as *const u8
                as *const i8,
        );
    }
    if width > 129 || height > 129 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CM_GeneratePatchFacets: source is > MAX_GRID_SIZE\x00" as *const u8 as *const i8,
        );
    }
    // build a grid
    grid.width = width;
    grid.height = height;
    grid.wrapWidth = crate::src::qcommon::q_shared::qfalse;
    grid.wrapHeight = crate::src::qcommon::q_shared::qfalse;
    i = 0;
    while i < width {
        j = 0;
        while j < height {
            grid.points[i as usize][j as usize][0] = (*points.offset((j * width + i) as isize))[0];
            grid.points[i as usize][j as usize][1] = (*points.offset((j * width + i) as isize))[1];
            grid.points[i as usize][j as usize][2] = (*points.offset((j * width + i) as isize))[2];
            j += 1
        }
        i += 1
    }
    // subdivide the grid
    CM_SetGridWrapWidth(&mut grid);
    CM_SubdivideGridColumns(&mut grid);
    CM_RemoveDegenerateColumns(&mut grid);
    CM_TransposeGrid(&mut grid);
    CM_SetGridWrapWidth(&mut grid);
    CM_SubdivideGridColumns(&mut grid);
    CM_RemoveDegenerateColumns(&mut grid);
    // we now have a grid of points exactly on the curve
    // the approximate surface defined by these points will be
    // collided against
    pf = crate::src::qcommon::common::Hunk_Alloc(
        ::std::mem::size_of::<crate::src::qcommon::cm_patch::patchCollide_t>() as i32,
        crate::src::qcommon::q_shared::h_high,
    ) as *mut crate::src::qcommon::cm_patch::patchCollide_t;
    crate::src::qcommon::q_math::ClearBounds(
        (*pf).bounds[0].as_mut_ptr(),
        (*pf).bounds[1].as_mut_ptr(),
    );
    i = 0;
    while i < grid.width {
        j = 0;
        while j < grid.height {
            crate::src::qcommon::q_math::AddPointToBounds(
                grid.points[i as usize][j as usize].as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                (*pf).bounds[0].as_mut_ptr(),
                (*pf).bounds[1].as_mut_ptr(),
            );
            j += 1
        }
        i += 1
    }
    c_totalPatchBlocks += (grid.width - 1) * (grid.height - 1);
    // generate a bsp tree for the surface
    CM_PatchCollideFromGrid(&mut grid, pf);
    // expand by one unit for epsilon purposes
    (*pf).bounds[0][0] -= 1f32;
    (*pf).bounds[0][1] -= 1f32;
    (*pf).bounds[0][2] -= 1f32;
    (*pf).bounds[1][0] += 1f32;
    (*pf).bounds[1][1] += 1f32;
    (*pf).bounds[1][2] += 1f32;
    return pf;
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

pub unsafe extern "C" fn CM_TracePointThroughPatchCollide(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut pc: *const crate::src::qcommon::cm_patch::patchCollide_s,
) {
    let mut frontFacing: [crate::src::qcommon::q_shared::qboolean; 2048] =
        [crate::src::qcommon::q_shared::qfalse; 2048];
    let mut intersection: [f32; 2048] = [0.; 2048];
    let mut intersect: f32 = 0.;
    let mut planes_0: *const crate::src::qcommon::cm_patch::patchPlane_t =
        0 as *const crate::src::qcommon::cm_patch::patchPlane_t;
    let mut facet: *const crate::src::qcommon::cm_patch::facet_t =
        0 as *const crate::src::qcommon::cm_patch::facet_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut offset: f32 = 0.;
    let mut d1: f32 = 0.;
    let mut d2: f32 = 0.;
    static mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    //BSPC
    if (*crate::src::qcommon::cm_load::cm_playerCurveClip).integer == 0 || (*tw).isPoint as u64 == 0
    {
        return;
    }
    // determine the trace's relationship to all planes
    planes_0 = (*pc).planes;
    i = 0;
    while i < (*pc).numPlanes {
        offset = (*tw).offsets[(*planes_0).signbits as usize][0] * (*planes_0).plane[0]
            + (*tw).offsets[(*planes_0).signbits as usize][1] * (*planes_0).plane[1]
            + (*tw).offsets[(*planes_0).signbits as usize][2] * (*planes_0).plane[2];
        d1 = (*tw).start[0] * (*planes_0).plane[0]
            + (*tw).start[1] * (*planes_0).plane[1]
            + (*tw).start[2] * (*planes_0).plane[2]
            - (*planes_0).plane[3]
            + offset;
        d2 = (*tw).end[0] * (*planes_0).plane[0]
            + (*tw).end[1] * (*planes_0).plane[1]
            + (*tw).end[2] * (*planes_0).plane[2]
            - (*planes_0).plane[3]
            + offset;
        if d1 <= 0f32 {
            frontFacing[i as usize] = crate::src::qcommon::q_shared::qfalse
        } else {
            frontFacing[i as usize] = crate::src::qcommon::q_shared::qtrue
        }
        if d1 == d2 {
            intersection[i as usize] = 99999f32
        } else {
            intersection[i as usize] = d1 / (d1 - d2);
            if intersection[i as usize] <= 0f32 {
                intersection[i as usize] = 99999f32
            }
        }
        i += 1;
        planes_0 = planes_0.offset(1)
    }
    // see if any of the surface planes are intersected
    facet = (*pc).facets;
    i = 0;
    while i < (*pc).numFacets {
        if !(frontFacing[(*facet).surfacePlane as usize] as u64 == 0) {
            intersect = intersection[(*facet).surfacePlane as usize];
            if !(intersect < 0f32) {
                if !(intersect > (*tw).trace.fraction) {
                    j = 0;
                    while j < (*facet).numBorders {
                        k = (*facet).borderPlanes[j as usize];
                        if frontFacing[k as usize] ^ (*facet).borderInward[j as usize] as u32 != 0 {
                            if intersection[k as usize] > intersect {
                                break;
                            }
                        } else if intersection[k as usize] < intersect {
                            break;
                        }
                        j += 1
                    }
                    if j == (*facet).numBorders {
                        // we hit this facet
                        if cv.is_null() {
                            cv = crate::src::qcommon::cvar::Cvar_Get(
                                b"r_debugSurfaceUpdate\x00" as *const u8 as *const i8,
                                b"1\x00" as *const u8 as *const i8,
                                0,
                            )
                        }
                        if (*cv).integer != 0 {
                            debugPatchCollide = pc;
                            debugFacet = facet
                        }
                        //BSPC
                        planes_0 = &mut *(*pc).planes.offset((*facet).surfacePlane as isize)
                            as *mut crate::src::qcommon::cm_patch::patchPlane_t;
                        // calculate intersection with a slight pushoff
                        offset = (*tw).offsets[(*planes_0).signbits as usize][0]
                            * (*planes_0).plane[0]
                            + (*tw).offsets[(*planes_0).signbits as usize][1]
                                * (*planes_0).plane[1]
                            + (*tw).offsets[(*planes_0).signbits as usize][2]
                                * (*planes_0).plane[2];
                        d1 = (*tw).start[0] * (*planes_0).plane[0]
                            + (*tw).start[1] * (*planes_0).plane[1]
                            + (*tw).start[2] * (*planes_0).plane[2]
                            - (*planes_0).plane[3]
                            + offset;
                        d2 = (*tw).end[0] * (*planes_0).plane[0]
                            + (*tw).end[1] * (*planes_0).plane[1]
                            + (*tw).end[2] * (*planes_0).plane[2]
                            - (*planes_0).plane[3]
                            + offset;
                        (*tw).trace.fraction = ((d1 as f64 - 0.125) / (d1 - d2) as f64) as f32;
                        if (*tw).trace.fraction < 0f32 {
                            (*tw).trace.fraction = 0f32
                        }
                        (*tw).trace.plane.normal[0] = (*planes_0).plane[0];
                        (*tw).trace.plane.normal[1] = (*planes_0).plane[1];
                        (*tw).trace.plane.normal[2] = (*planes_0).plane[2];
                        (*tw).trace.plane.dist = (*planes_0).plane[3]
                    }
                }
            }
        }
        i += 1;
        facet = facet.offset(1)
        // already hit something closer
    }
}
/*
====================
CM_CheckFacetPlane
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_CheckFacetPlane(
    mut plane: *mut f32,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut enterFrac: *mut f32,
    mut leaveFrac: *mut f32,
    mut hit: *mut i32,
) -> i32 {
    let mut d1: f32 = 0.;
    let mut d2: f32 = 0.;
    let mut f: f32 = 0.;
    *hit = crate::src::qcommon::q_shared::qfalse as i32;
    d1 = *start.offset(0) * *plane.offset(0)
        + *start.offset(1) * *plane.offset(1)
        + *start.offset(2) * *plane.offset(2)
        - *plane.offset(3);
    d2 = *end.offset(0) * *plane.offset(0)
        + *end.offset(1) * *plane.offset(1)
        + *end.offset(2) * *plane.offset(2)
        - *plane.offset(3);
    // if completely in front of face, no intersection with the entire facet
    if d1 > 0f32 && (d2 as f64 >= 0.125 || d2 >= d1) {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    // if it doesn't cross the plane, the plane isn't relevant
    if d1 <= 0f32 && d2 <= 0f32 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    // crosses face
    if d1 > d2 {
        f = ((d1 as f64 - 0.125) / (d1 - d2) as f64) as f32; // leave
        if f < 0f32 {
            f = 0f32
        }
        // enter
        //always favor previous plane hits and thus also the surface plane hit
        if f > *enterFrac {
            *enterFrac = f;
            *hit = crate::src::qcommon::q_shared::qtrue as i32
        }
    } else {
        f = ((d1 as f64 + 0.125) / (d1 - d2) as f64) as f32;
        if f > 1f32 {
            f = 1f32
        }
        if f < *leaveFrac {
            *leaveFrac = f
        }
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
====================
CM_TraceThroughPatchCollide
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_TraceThroughPatchCollide(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut pc: *const crate::src::qcommon::cm_patch::patchCollide_s,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut hit: i32 = 0;
    let mut hitnum: i32 = 0;
    let mut offset: f32 = 0.;
    let mut enterFrac: f32 = 0.;
    let mut leaveFrac: f32 = 0.;
    let mut t: f32 = 0.;
    let mut planes_0: *mut crate::src::qcommon::cm_patch::patchPlane_t =
        0 as *mut crate::src::qcommon::cm_patch::patchPlane_t;
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut plane: [f32; 4] = [0f32, 0f32, 0f32, 0f32];
    let mut bestplane: [f32; 4] = [0f32, 0f32, 0f32, 0f32];
    let mut startp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut endp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    static mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    //BSPC
    if crate::src::qcommon::cm_test::CM_BoundsIntersect(
        (*tw).bounds[0].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*tw).bounds[1].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*pc).bounds[0].as_ptr(),
        (*pc).bounds[1].as_ptr(),
    ) as u64
        == 0
    {
        return;
    }
    if (*tw).isPoint as u64 != 0 {
        CM_TracePointThroughPatchCollide(tw, pc);
        return;
    }
    facet = (*pc).facets;
    i = 0;
    while i < (*pc).numFacets {
        enterFrac = -1f32;
        leaveFrac = 1f32;
        hitnum = -(1);
        //
        planes_0 = &mut *(*pc).planes.offset((*facet).surfacePlane as isize)
            as *mut crate::src::qcommon::cm_patch::patchPlane_t;
        plane[0] = (*planes_0).plane[0];
        plane[1] = (*planes_0).plane[1];
        plane[2] = (*planes_0).plane[2];
        plane[3] = (*planes_0).plane[3];
        if (*tw).sphere.use_0 as u64 != 0 {
            // adjust the plane distance appropriately for radius
            plane[3] += (*tw).sphere.radius;
            // find the closest point on the capsule to the plane
            t = plane[0] * (*tw).sphere.offset[0]
                + plane[1] * (*tw).sphere.offset[1]
                + plane[2] * (*tw).sphere.offset[2];
            if t > 0.0 {
                startp[0] = (*tw).start[0] - (*tw).sphere.offset[0];
                startp[1] = (*tw).start[1] - (*tw).sphere.offset[1];
                startp[2] = (*tw).start[2] - (*tw).sphere.offset[2];
                endp[0] = (*tw).end[0] - (*tw).sphere.offset[0];
                endp[1] = (*tw).end[1] - (*tw).sphere.offset[1];
                endp[2] = (*tw).end[2] - (*tw).sphere.offset[2]
            } else {
                startp[0] = (*tw).start[0] + (*tw).sphere.offset[0];
                startp[1] = (*tw).start[1] + (*tw).sphere.offset[1];
                startp[2] = (*tw).start[2] + (*tw).sphere.offset[2];
                endp[0] = (*tw).end[0] + (*tw).sphere.offset[0];
                endp[1] = (*tw).end[1] + (*tw).sphere.offset[1];
                endp[2] = (*tw).end[2] + (*tw).sphere.offset[2]
            }
        } else {
            offset = (*tw).offsets[(*planes_0).signbits as usize][0] * plane[0]
                + (*tw).offsets[(*planes_0).signbits as usize][1] * plane[1]
                + (*tw).offsets[(*planes_0).signbits as usize][2] * plane[2];
            plane[3] -= offset;
            startp[0] = (*tw).start[0];
            startp[1] = (*tw).start[1];
            startp[2] = (*tw).start[2];
            endp[0] = (*tw).end[0];
            endp[1] = (*tw).end[1];
            endp[2] = (*tw).end[2]
        }
        if !(CM_CheckFacetPlane(
            plane.as_mut_ptr(),
            startp.as_mut_ptr(),
            endp.as_mut_ptr(),
            &mut enterFrac,
            &mut leaveFrac,
            &mut hit,
        ) == 0)
        {
            if hit != 0 {
                bestplane[0] = plane[0];
                bestplane[1] = plane[1];
                bestplane[2] = plane[2];
                bestplane[3] = plane[3]
            }
            j = 0;
            while j < (*facet).numBorders {
                planes_0 = &mut *(*pc)
                    .planes
                    .offset(*(*facet).borderPlanes.as_mut_ptr().offset(j as isize) as isize)
                    as *mut crate::src::qcommon::cm_patch::patchPlane_t;
                if (*facet).borderInward[j as usize] != 0 {
                    plane[0] = -(*planes_0).plane[0];
                    plane[1] = -(*planes_0).plane[1];
                    plane[2] = -(*planes_0).plane[2];
                    plane[3] = -(*planes_0).plane[3]
                } else {
                    plane[0] = (*planes_0).plane[0];
                    plane[1] = (*planes_0).plane[1];
                    plane[2] = (*planes_0).plane[2];
                    plane[3] = (*planes_0).plane[3]
                }
                if (*tw).sphere.use_0 as u64 != 0 {
                    // adjust the plane distance appropriately for radius
                    plane[3] += (*tw).sphere.radius;
                    // find the closest point on the capsule to the plane
                    t = plane[0] * (*tw).sphere.offset[0]
                        + plane[1] * (*tw).sphere.offset[1]
                        + plane[2] * (*tw).sphere.offset[2];
                    if t > 0.0 {
                        startp[0] = (*tw).start[0] - (*tw).sphere.offset[0];
                        startp[1] = (*tw).start[1] - (*tw).sphere.offset[1];
                        startp[2] = (*tw).start[2] - (*tw).sphere.offset[2];
                        endp[0] = (*tw).end[0] - (*tw).sphere.offset[0];
                        endp[1] = (*tw).end[1] - (*tw).sphere.offset[1];
                        endp[2] = (*tw).end[2] - (*tw).sphere.offset[2]
                    } else {
                        startp[0] = (*tw).start[0] + (*tw).sphere.offset[0];
                        startp[1] = (*tw).start[1] + (*tw).sphere.offset[1];
                        startp[2] = (*tw).start[2] + (*tw).sphere.offset[2];
                        endp[0] = (*tw).end[0] + (*tw).sphere.offset[0];
                        endp[1] = (*tw).end[1] + (*tw).sphere.offset[1];
                        endp[2] = (*tw).end[2] + (*tw).sphere.offset[2]
                    }
                } else {
                    // NOTE: this works even though the plane might be flipped because the bbox is centered
                    offset = (*tw).offsets[(*planes_0).signbits as usize][0] * plane[0]
                        + (*tw).offsets[(*planes_0).signbits as usize][1] * plane[1]
                        + (*tw).offsets[(*planes_0).signbits as usize][2] * plane[2];
                    plane[3] = (plane[3] as f64 + crate::stdlib::fabs(offset as f64)) as f32;
                    startp[0] = (*tw).start[0];
                    startp[1] = (*tw).start[1];
                    startp[2] = (*tw).start[2];
                    endp[0] = (*tw).end[0];
                    endp[1] = (*tw).end[1];
                    endp[2] = (*tw).end[2]
                }
                if CM_CheckFacetPlane(
                    plane.as_mut_ptr(),
                    startp.as_mut_ptr(),
                    endp.as_mut_ptr(),
                    &mut enterFrac,
                    &mut leaveFrac,
                    &mut hit,
                ) == 0
                {
                    break;
                }
                if hit != 0 {
                    hitnum = j;
                    bestplane[0] = plane[0];
                    bestplane[1] = plane[1];
                    bestplane[2] = plane[2];
                    bestplane[3] = plane[3]
                }
                j += 1
            }
            if !(j < (*facet).numBorders) {
                //never clip against the back side
                if !(hitnum == (*facet).numBorders - 1) {
                    if enterFrac < leaveFrac && enterFrac >= 0f32 {
                        if enterFrac < (*tw).trace.fraction {
                            if enterFrac < 0f32 {
                                enterFrac = 0f32
                            }
                            if cv.is_null() {
                                cv = crate::src::qcommon::cvar::Cvar_Get(
                                    b"r_debugSurfaceUpdate\x00" as *const u8 as *const i8,
                                    b"1\x00" as *const u8 as *const i8,
                                    0,
                                )
                            }
                            if !cv.is_null() && (*cv).integer != 0 {
                                debugPatchCollide = pc;
                                debugFacet = facet
                            }
                            //BSPC
                            (*tw).trace.fraction = enterFrac;
                            (*tw).trace.plane.normal[0] = bestplane[0];
                            (*tw).trace.plane.normal[1] = bestplane[1];
                            (*tw).trace.plane.normal[2] = bestplane[2];
                            (*tw).trace.plane.dist = bestplane[3]
                        }
                    }
                }
            }
        }
        i += 1;
        facet = facet.offset(1)
    }
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
// negative numbers are leafs
// submodels don't reference the main tree
// the shader that determined the contents
// to avoid repeated testings
// to avoid repeated testings
// if false, visibility is just a single cluster of ffs
// [ numAreas*numAreas ] reference counts
// non-patches will be NULL
// incremented on each trace
// keep 1/8 unit away to keep the position valid before network snapping
// and to avoid various numeric issues
// cm_test.c
// Used for oriented capsule collision detection
// size of the box being swept through the model
// [signbits][x] = either size[0][x] or size[1][x]
// longest corner length from origin
// greatest of abs(size[0]) and abs(size[1])
// enclosing box of start and end surrounding by size
// origin of the model tracing through
// ored contents of the model tracing through
// optimized case
// returned from trace call
// sphere for oriendted capsule collision
// for overflows where each leaf can't be stored individually
// cm_patch.c
/*
=======================================================================

POSITION TEST

=======================================================================
*/
/*
====================
CM_PositionTestInPatchCollide
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CM_PositionTestInPatchCollide(
    mut tw: *mut crate::cm_local_h::traceWork_t,
    mut pc: *const crate::src::qcommon::cm_patch::patchCollide_s,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut offset: f32 = 0.;
    let mut t: f32 = 0.;
    let mut planes_0: *mut crate::src::qcommon::cm_patch::patchPlane_t =
        0 as *mut crate::src::qcommon::cm_patch::patchPlane_t;
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut plane: [f32; 4] = [0.; 4];
    let mut startp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*tw).isPoint as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    //
    facet = (*pc).facets;
    i = 0;
    while i < (*pc).numFacets {
        planes_0 = &mut *(*pc).planes.offset((*facet).surfacePlane as isize)
            as *mut crate::src::qcommon::cm_patch::patchPlane_t;
        plane[0] = (*planes_0).plane[0];
        plane[1] = (*planes_0).plane[1];
        plane[2] = (*planes_0).plane[2];
        plane[3] = (*planes_0).plane[3];
        if (*tw).sphere.use_0 as u64 != 0 {
            // adjust the plane distance appropriately for radius
            plane[3] += (*tw).sphere.radius;
            // find the closest point on the capsule to the plane
            t = plane[0] * (*tw).sphere.offset[0]
                + plane[1] * (*tw).sphere.offset[1]
                + plane[2] * (*tw).sphere.offset[2];
            if t > 0f32 {
                startp[0] = (*tw).start[0] - (*tw).sphere.offset[0];
                startp[1] = (*tw).start[1] - (*tw).sphere.offset[1];
                startp[2] = (*tw).start[2] - (*tw).sphere.offset[2]
            } else {
                startp[0] = (*tw).start[0] + (*tw).sphere.offset[0];
                startp[1] = (*tw).start[1] + (*tw).sphere.offset[1];
                startp[2] = (*tw).start[2] + (*tw).sphere.offset[2]
            }
        } else {
            offset = (*tw).offsets[(*planes_0).signbits as usize][0] * plane[0]
                + (*tw).offsets[(*planes_0).signbits as usize][1] * plane[1]
                + (*tw).offsets[(*planes_0).signbits as usize][2] * plane[2];
            plane[3] -= offset;
            startp[0] = (*tw).start[0];
            startp[1] = (*tw).start[1];
            startp[2] = (*tw).start[2]
        }
        if !(plane[0] * startp[0] + plane[1] * startp[1] + plane[2] * startp[2] - plane[3] > 0.0) {
            j = 0;
            while j < (*facet).numBorders {
                planes_0 = &mut *(*pc)
                    .planes
                    .offset(*(*facet).borderPlanes.as_mut_ptr().offset(j as isize) as isize)
                    as *mut crate::src::qcommon::cm_patch::patchPlane_t;
                if (*facet).borderInward[j as usize] != 0 {
                    plane[0] = -(*planes_0).plane[0];
                    plane[1] = -(*planes_0).plane[1];
                    plane[2] = -(*planes_0).plane[2];
                    plane[3] = -(*planes_0).plane[3]
                } else {
                    plane[0] = (*planes_0).plane[0];
                    plane[1] = (*planes_0).plane[1];
                    plane[2] = (*planes_0).plane[2];
                    plane[3] = (*planes_0).plane[3]
                }
                if (*tw).sphere.use_0 as u64 != 0 {
                    // adjust the plane distance appropriately for radius
                    plane[3] += (*tw).sphere.radius;
                    // find the closest point on the capsule to the plane
                    t = plane[0] * (*tw).sphere.offset[0]
                        + plane[1] * (*tw).sphere.offset[1]
                        + plane[2] * (*tw).sphere.offset[2];
                    if t > 0.0 {
                        startp[0] = (*tw).start[0] - (*tw).sphere.offset[0];
                        startp[1] = (*tw).start[1] - (*tw).sphere.offset[1];
                        startp[2] = (*tw).start[2] - (*tw).sphere.offset[2]
                    } else {
                        startp[0] = (*tw).start[0] + (*tw).sphere.offset[0];
                        startp[1] = (*tw).start[1] + (*tw).sphere.offset[1];
                        startp[2] = (*tw).start[2] + (*tw).sphere.offset[2]
                    }
                } else {
                    // NOTE: this works even though the plane might be flipped because the bbox is centered
                    offset = (*tw).offsets[(*planes_0).signbits as usize][0] * plane[0]
                        + (*tw).offsets[(*planes_0).signbits as usize][1] * plane[1]
                        + (*tw).offsets[(*planes_0).signbits as usize][2] * plane[2];
                    plane[3] = (plane[3] as f64 + crate::stdlib::fabs(offset as f64)) as f32;
                    startp[0] = (*tw).start[0];
                    startp[1] = (*tw).start[1];
                    startp[2] = (*tw).start[2]
                }
                if plane[0] * startp[0] + plane[1] * startp[1] + plane[2] * startp[2] - plane[3]
                    > 0.0
                {
                    break;
                }
                j += 1
            }
            if !(j < (*facet).numBorders) {
                // inside this patch facet
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
        i += 1;
        facet = facet.offset(1)
    }
    return crate::src::qcommon::q_shared::qfalse;
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
// 0 = world, 1 + are bmodels
// returns an ORed contents mask
// only returns non-solid leafs
// overflow if return listsize and if *lastLeaf != list[listsize-1]
// cm_patch.c
#[no_mangle]

pub unsafe extern "C" fn CM_DrawDebugSurface(
    mut drawPoly: Option<unsafe extern "C" fn(_: i32, _: i32, _: *mut f32) -> ()>,
) {
    static mut cv: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    static mut cv2: *mut crate::src::qcommon::q_shared::cvar_t =
        0 as *mut crate::src::qcommon::q_shared::cvar_t;
    let mut pc: *const crate::src::qcommon::cm_patch::patchCollide_t =
        0 as *const crate::src::qcommon::cm_patch::patchCollide_t;
    let mut facet: *mut crate::src::qcommon::cm_patch::facet_t =
        0 as *mut crate::src::qcommon::cm_patch::facet_t;
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut _k: i32 = 0;
    let mut n: i32 = 0;
    let mut curplanenum: i32 = 0;
    let mut planenum: i32 = 0;
    let mut curinward: i32 = 0;
    let mut inward: i32 = 0;
    let mut plane: [f32; 4] = [0.; 4];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [-15f32, -15f32, -28f32];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [15f32, 15f32, 28f32];
    //vec3_t mins = {0, 0, 0}, maxs = {0, 0, 0};
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if cv2.is_null() {
        cv2 = crate::src::qcommon::cvar::Cvar_Get(
            b"r_debugSurface\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
            0,
        )
    }
    if (*cv2).integer != 1 {
        BotDrawDebugPolygons(drawPoly, (*cv2).integer);
        return;
    }
    if debugPatchCollide.is_null() {
        return;
    }
    if cv.is_null() {
        cv = crate::src::qcommon::cvar::Cvar_Get(
            b"cm_debugSize\x00" as *const u8 as *const i8,
            b"2\x00" as *const u8 as *const i8,
            0,
        )
    }
    pc = debugPatchCollide;
    i = 0;
    facet = (*pc).facets;
    while i < (*pc).numFacets {
        for k in 0..(*facet).numBorders + 1 {
            if k < (*facet).numBorders {
                planenum = (*facet).borderPlanes[k as usize];
                inward = (*facet).borderInward[k as usize]
            } else {
                planenum = (*facet).surfacePlane;
                inward = crate::src::qcommon::q_shared::qfalse as i32
                //continue;
            }

            plane[0] = (*(*pc).planes.offset(planenum as isize)).plane[0];

            plane[1] = (*(*pc).planes.offset(planenum as isize)).plane[1];

            plane[2] = (*(*pc).planes.offset(planenum as isize)).plane[2];

            plane[3] = (*(*pc).planes.offset(planenum as isize)).plane[3];

            if inward != 0 {
                plane[0] = crate::src::qcommon::q_math::vec3_origin[0] - plane[0];
                plane[1] = crate::src::qcommon::q_math::vec3_origin[1] - plane[1];
                plane[2] = crate::src::qcommon::q_math::vec3_origin[2] - plane[2];
                plane[3] = -plane[3]
            }

            plane[3] += (*cv).value;

            n = 0;

            while n < 3 {
                if plane[n as usize] > 0f32 {
                    v1[n as usize] = maxs[n as usize]
                } else {
                    v1[n as usize] = mins[n as usize]
                }
                n += 1
            }

            v2[0] = -plane[0];

            v2[1] = -plane[1];

            v2[2] = -plane[2];

            plane[3] = (plane[3] as f64
                + crate::stdlib::fabs((v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]) as f64))
                as f32;

            w = crate::src::qcommon::cm_polylib::BaseWindingForPlane(plane.as_mut_ptr(), plane[3]);

            j = 0;

            while j < (*facet).numBorders + 1 && !w.is_null() {
                //
                if j < (*facet).numBorders {
                    curplanenum = (*facet).borderPlanes[j as usize];
                    curinward = (*facet).borderInward[j as usize]
                } else {
                    curplanenum = (*facet).surfacePlane;
                    curinward = crate::src::qcommon::q_shared::qfalse as i32
                    //continue;
                }
                //
                if !(curplanenum == planenum) {
                    plane[0] = (*(*pc).planes.offset(curplanenum as isize)).plane[0];
                    plane[1] = (*(*pc).planes.offset(curplanenum as isize)).plane[1];
                    plane[2] = (*(*pc).planes.offset(curplanenum as isize)).plane[2];
                    plane[3] = (*(*pc).planes.offset(curplanenum as isize)).plane[3];
                    if curinward == 0 {
                        plane[0] = crate::src::qcommon::q_math::vec3_origin[0] - plane[0];
                        plane[1] = crate::src::qcommon::q_math::vec3_origin[1] - plane[1];
                        plane[2] = crate::src::qcommon::q_math::vec3_origin[2] - plane[2];
                        plane[3] = -plane[3]
                    }
                    //			if ( !facet->borderNoAdjust[j] ) {
                    plane[3] -= (*cv).value;
                    //			}
                    n = 0; //end for
                    while n < 3 {
                        if plane[n as usize] > 0f32 {
                            v1[n as usize] = maxs[n as usize]
                        } else {
                            v1[n as usize] = mins[n as usize]
                        }
                        n += 1
                    }
                    v2[0] = -plane[0];
                    v2[1] = -plane[1];
                    v2[2] = -plane[2];
                    plane[3] = (plane[3] as f64
                        - crate::stdlib::fabs(
                            (v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]) as f64,
                        )) as f32;
                    crate::src::qcommon::cm_polylib::ChopWindingInPlace(
                        &mut w,
                        plane.as_mut_ptr(),
                        plane[3usize],
                        0.1f32,
                    );
                }
                j += 1
            }

            if !w.is_null() {
                if facet == debugFacet as *mut crate::src::qcommon::cm_patch::facet_t {
                    drawPoly.expect("non-null function pointer")(
                        4i32,
                        (*w).numpoints,
                        (*(*w).p.as_mut_ptr().offset(0isize)).as_mut_ptr(),
                    );
                //Com_Printf("blue facet has %d border planes\n", facet->numBorders);
                } else {
                    drawPoly.expect("non-null function pointer")(
                        1i32,
                        (*w).numpoints,
                        (*(*w).p.as_mut_ptr().offset(0isize)).as_mut_ptr(),
                    );
                }
                crate::src::qcommon::cm_polylib::FreeWinding(w);
            } else {
                crate::src::qcommon::common::Com_Printf(
                    b"winding chopped away by border planes\n\x00" as *const u8 as *const i8,
                );
            }
        }
        i += 1;
        facet = facet.offset(1)
    }
    // draw the debug block
    let mut v: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    v[0][0] = debugBlockPoints[0][0];
    v[0][1] = debugBlockPoints[0][1];
    v[0][2] = debugBlockPoints[0][2];
    v[1][0] = debugBlockPoints[1][0];
    v[1][1] = debugBlockPoints[1][1];
    v[1][2] = debugBlockPoints[1][2];
    v[2][0] = debugBlockPoints[2][0];
    v[2][1] = debugBlockPoints[2][1];
    v[2][2] = debugBlockPoints[2][2];
    drawPoly.expect("non-null function pointer")(2, 3, v[0].as_mut_ptr());
    v[0][0] = debugBlockPoints[2][0];
    v[0][1] = debugBlockPoints[2][1];
    v[0][2] = debugBlockPoints[2][2];
    v[1][0] = debugBlockPoints[3][0];
    v[1][1] = debugBlockPoints[3][1];
    v[1][2] = debugBlockPoints[3][2];
    v[2][0] = debugBlockPoints[0][0];
    v[2][1] = debugBlockPoints[0][1];
    v[2][2] = debugBlockPoints[0][2];
    drawPoly.expect("non-null function pointer")(2, 3, v[0].as_mut_ptr());
}
