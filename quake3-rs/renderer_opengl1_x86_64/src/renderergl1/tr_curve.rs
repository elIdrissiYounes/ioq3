use ::libc;

pub mod q_shared_h {

    /*
    ==============================================================

    MATHLIB

    ==============================================================
    */

    // server browser sources
    // TTimo: AS_MPLAYER is no longer used
    // cinematic states

    // all other conditions, i.e. stop/EOF/abort

    // play

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

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0) * *v.offset(0)
            + *v.offset(1) * *v.offset(1)
            + *v.offset(2) * *v.offset(2);
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
    use crate::stdlib::sqrt;

    // __Q_SHARED_H
}

pub use crate::qfiles_h::drawVert_t;
pub use crate::src::qcommon::q_math::AddPointToBounds;
pub use crate::src::qcommon::q_math::ClearBounds;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::e_status;
pub use crate::src::qcommon::q_shared::h_dontcare;
pub use crate::src::qcommon::q_shared::h_high;
pub use crate::src::qcommon::q_shared::h_low;
pub use crate::src::qcommon::q_shared::ha_pref;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FMV_EOF;
pub use crate::src::qcommon::q_shared::FMV_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_BLT;
pub use crate::src::qcommon::q_shared::FMV_ID_IDLE;
pub use crate::src::qcommon::q_shared::FMV_ID_WAIT;
pub use crate::src::qcommon::q_shared::FMV_LOOPED;
pub use crate::src::qcommon::q_shared::FMV_PLAY;
pub use crate::src::renderergl1::tr_curve::q_shared_h::CrossProduct;
pub use crate::src::renderergl1::tr_curve::q_shared_h::VectorLength;
pub use crate::src::renderergl1::tr_curve::q_shared_h::VectorLengthSquared;
pub use crate::tr_public_h::refimport_t;

pub use crate::src::renderergl1::tr_init::r_subdivisions;
use crate::src::renderergl1::tr_main::ri;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::sqrt;
pub use crate::tr_local_h::srfGridMesh_s;
pub use crate::tr_local_h::srfGridMesh_t;
pub use crate::tr_local_h::surfaceType_t;
pub use crate::tr_local_h::SF_BAD;
pub use crate::tr_local_h::SF_ENTITY;
pub use crate::tr_local_h::SF_FACE;
pub use crate::tr_local_h::SF_FLARE;
pub use crate::tr_local_h::SF_GRID;
pub use crate::tr_local_h::SF_IQM;
pub use crate::tr_local_h::SF_MAX;
pub use crate::tr_local_h::SF_MD3;
pub use crate::tr_local_h::SF_MDR;
pub use crate::tr_local_h::SF_NUM_SURFACE_TYPES;
pub use crate::tr_local_h::SF_POLY;
pub use crate::tr_local_h::SF_SKIP;
pub use crate::tr_local_h::SF_TRIANGLES;
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

This file does all of the processing necessary to turn a raw grid of points
read from the map file into a srfGridMesh_t ready for rendering.

The level of detail solution is direction independent, based only on subdivided
distance from the true curve.

Only a single entry point:

srfGridMesh_t *R_SubdividePatchToGrid( int width, int height,
                                drawVert_t points[MAX_PATCH_SIZE*MAX_PATCH_SIZE] ) {

*/
/*
============
LerpDrawVert
============
*/

unsafe extern "C" fn LerpDrawVert(
    mut a: *mut crate::qfiles_h::drawVert_t,
    mut b: *mut crate::qfiles_h::drawVert_t,
    mut out: *mut crate::qfiles_h::drawVert_t,
) {
    (*out).xyz[0] = 0.5f32 * ((*a).xyz[0] + (*b).xyz[0]);
    (*out).xyz[1] = 0.5f32 * ((*a).xyz[1] + (*b).xyz[1]);
    (*out).xyz[2] = 0.5f32 * ((*a).xyz[2] + (*b).xyz[2]);
    (*out).st[0] = 0.5f32 * ((*a).st[0] + (*b).st[0]);
    (*out).st[1] = 0.5f32 * ((*a).st[1] + (*b).st[1]);
    (*out).lightmap[0] = 0.5f32 * ((*a).lightmap[0] + (*b).lightmap[0]);
    (*out).lightmap[1] = 0.5f32 * ((*a).lightmap[1] + (*b).lightmap[1]);
    (*out).color[0] =
        ((*a).color[0] as i32 + (*b).color[0] as i32 >> 1) as crate::src::qcommon::q_shared::byte;
    (*out).color[1] =
        ((*a).color[1] as i32 + (*b).color[1] as i32 >> 1) as crate::src::qcommon::q_shared::byte;
    (*out).color[2] =
        ((*a).color[2] as i32 + (*b).color[2] as i32 >> 1) as crate::src::qcommon::q_shared::byte;
    (*out).color[3] =
        ((*a).color[3] as i32 + (*b).color[3] as i32 >> 1) as crate::src::qcommon::q_shared::byte;
}
/*
============
Transpose
============
*/

unsafe extern "C" fn Transpose(
    mut width: i32,
    mut height: i32,
    mut ctrl: *mut [crate::qfiles_h::drawVert_t; 65],
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut temp: crate::qfiles_h::drawVert_t = crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    };
    if width > height {
        i = 0;
        while i < height {
            j = i + 1;
            while j < width {
                if j < height {
                    // swap the value
                    temp = (*ctrl.offset(j as isize))[i as usize];
                    (*ctrl.offset(j as isize))[i as usize] = (*ctrl.offset(i as isize))[j as usize];
                    (*ctrl.offset(i as isize))[j as usize] = temp
                } else {
                    // just copy
                    (*ctrl.offset(j as isize))[i as usize] = (*ctrl.offset(i as isize))[j as usize]
                }
                j += 1
            }
            i += 1
        }
    } else {
        i = 0;
        while i < width {
            j = i + 1;
            while j < height {
                if j < width {
                    // swap the value
                    temp = (*ctrl.offset(i as isize))[j as usize];
                    (*ctrl.offset(i as isize))[j as usize] = (*ctrl.offset(j as isize))[i as usize];
                    (*ctrl.offset(j as isize))[i as usize] = temp
                } else {
                    // just copy
                    (*ctrl.offset(i as isize))[j as usize] = (*ctrl.offset(j as isize))[i as usize]
                }
                j += 1
            }
            i += 1
        }
    };
}
/*
=================
MakeMeshNormals

Handles all the complicated wrapping and degenerate cases
=================
*/

unsafe extern "C" fn MakeMeshNormals(
    mut width: i32,
    mut height: i32,
    mut ctrl: *mut [crate::qfiles_h::drawVert_t; 65],
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut dist: i32 = 0;
    let mut normal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut sum: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut count: i32 = 0;
    let mut base: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dv: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut around: [crate::src::qcommon::q_shared::vec3_t; 8] = [[0.; 3]; 8];
    let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut good: [crate::src::qcommon::q_shared::qboolean; 8] =
        [crate::src::qcommon::q_shared::qfalse; 8];
    let mut wrapWidth: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut wrapHeight: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut len: f32 = 0.;
    static mut neighbors: [[i32; 2]; 8] = [
        [0, 1],
        [1, 1],
        [1, 0],
        [1, -(1)],
        [0, -(1)],
        [-(1), -(1)],
        [-(1), 0],
        [-(1), 1],
    ];
    wrapWidth = crate::src::qcommon::q_shared::qfalse;
    i = 0;
    while i < height {
        delta[0] = (*ctrl.offset(i as isize))[0].xyz[0]
            - (*ctrl.offset(i as isize))[(width - 1i32) as usize].xyz[0];
        delta[1] = (*ctrl.offset(i as isize))[0].xyz[1]
            - (*ctrl.offset(i as isize))[(width - 1i32) as usize].xyz[1];
        delta[2] = (*ctrl.offset(i as isize))[0].xyz[2]
            - (*ctrl.offset(i as isize))[(width - 1i32) as usize].xyz[2];
        len =
            VectorLengthSquared(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if len as f64 > 1.0 {
            break;
        }
        i += 1
    }
    if i == height {
        wrapWidth = crate::src::qcommon::q_shared::qtrue
    }
    wrapHeight = crate::src::qcommon::q_shared::qfalse;
    i = 0;
    while i < width {
        delta[0] = (*ctrl.offset(0))[i as usize].xyz[0]
            - (*ctrl.offset((height - 1i32) as isize))[i as usize].xyz[0];
        delta[1] = (*ctrl.offset(0))[i as usize].xyz[1]
            - (*ctrl.offset((height - 1i32) as isize))[i as usize].xyz[1];
        delta[2] = (*ctrl.offset(0))[i as usize].xyz[2]
            - (*ctrl.offset((height - 1i32) as isize))[i as usize].xyz[2];
        len =
            VectorLengthSquared(delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if len as f64 > 1.0 {
            break;
        }
        i += 1
    }
    if i == width {
        wrapHeight = crate::src::qcommon::q_shared::qtrue
    }
    i = 0;
    while i < width {
        for j in 0..height {
            count = 0;

            dv = &mut *(*ctrl.offset(j as isize)).as_mut_ptr().offset(i as isize)
                as *mut crate::qfiles_h::drawVert_t;

            base[0] = (*dv).xyz[0];

            base[1] = (*dv).xyz[1];

            base[2] = (*dv).xyz[2];

            k = 0;

            while k < 8 {
                around[k as usize][2] = 0f32;
                around[k as usize][1] = around[k as usize][2];
                around[k as usize][0] = around[k as usize][1];
                good[k as usize] = crate::src::qcommon::q_shared::qfalse;
                dist = 1;
                while dist <= 3 {
                    x = i + neighbors[k as usize][0] * dist;
                    y = j + neighbors[k as usize][1] * dist;
                    if wrapWidth as u64 != 0 {
                        if x < 0 {
                            x = width - 1 + x
                        } else if x >= width {
                            x = 1 + x - width
                        }
                    }
                    if wrapHeight as u64 != 0 {
                        if y < 0 {
                            y = height - 1 + y
                        } else if y >= height {
                            y = 1 + y - height
                        }
                    }
                    if x < 0 || x >= width || y < 0 || y >= height {
                        break;
                    }
                    temp[0] = (*ctrl.offset(y as isize))[x as usize].xyz[0] - base[0];
                    temp[1] = (*ctrl.offset(y as isize))[x as usize].xyz[1] - base[1];
                    temp[2] = (*ctrl.offset(y as isize))[x as usize].xyz[2] - base[2];
                    if crate::src::qcommon::q_math::VectorNormalize2(
                        temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        temp.as_mut_ptr(),
                    ) == 0f32
                    {
                        dist += 1
                    // degenerate edge, get more dist
                    } else {
                        good[k as usize] = crate::src::qcommon::q_shared::qtrue;
                        around[k as usize][0] = temp[0];
                        around[k as usize][1] = temp[1];
                        around[k as usize][2] = temp[2];
                        break;
                        // good edge
                    }
                }
                k += 1
            }

            sum[2] = 0f32;

            sum[1] = sum[2];

            sum[0] = sum[1];

            k = 0;

            while k < 8 {
                if !(good[k as usize] as u64 == 0 || good[(k + 1 & 7) as usize] as u64 == 0) {
                    CrossProduct(
                        around[(k + 1 & 7) as usize].as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                        around[k as usize].as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                        normal.as_mut_ptr(),
                    );
                    if !(crate::src::qcommon::q_math::VectorNormalize2(
                        normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        normal.as_mut_ptr(),
                    ) == 0f32)
                    {
                        sum[0] = normal[0] + sum[0];
                        sum[1] = normal[1] + sum[1];
                        sum[2] = normal[2] + sum[2];
                        count += 1
                    }
                }
                k += 1
                // didn't get two points
            }

            crate::src::qcommon::q_math::VectorNormalize2(
                sum.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*dv).normal.as_mut_ptr(),
            );
        }
        i += 1
    }
}
/*
============
InvertCtrl
============
*/

unsafe extern "C" fn InvertCtrl(
    mut width: i32,
    mut height: i32,
    mut ctrl: *mut [crate::qfiles_h::drawVert_t; 65],
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut temp: crate::qfiles_h::drawVert_t = crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    };
    i = 0;
    while i < height {
        for j in 0..width / 2 {
            temp = (*ctrl.offset(i as isize))[j as usize];

            (*ctrl.offset(i as isize))[j as usize] =
                (*ctrl.offset(i as isize))[(width - 1 - j) as usize];

            (*ctrl.offset(i as isize))[(width - 1 - j) as usize] = temp;
        }
        i += 1
    }
}
/*
=================
InvertErrorTable
=================
*/

unsafe extern "C" fn InvertErrorTable(
    mut errorTable: *mut [f32; 65],
    mut width: i32,
    mut height: i32,
) {
    let mut i: i32 = 0;
    let mut copy: [[f32; 65]; 2] = [[0.; 65]; 2];
    crate::stdlib::memcpy(
        copy.as_mut_ptr() as *mut libc::c_void,
        errorTable as *const libc::c_void,
        ::std::mem::size_of::<[[f32; 65]; 2]>(),
    );
    i = 0;
    while i < width {
        (*errorTable.offset(1))[i as usize] = copy[0][i as usize];
        i += 1
        //[width-1-i];
    }
    i = 0;
    while i < height {
        (*errorTable.offset(0))[i as usize] = copy[1][(height - 1 - i) as usize];
        i += 1
    }
}
/*
==================
PutPointsOnCurve
==================
*/

unsafe extern "C" fn PutPointsOnCurve(
    mut ctrl: *mut [crate::qfiles_h::drawVert_t; 65],
    mut width: i32,
    mut height: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut prev: crate::qfiles_h::drawVert_t = crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    };
    let mut next: crate::qfiles_h::drawVert_t = crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    };
    i = 0;
    while i < width {
        j = 1;
        while j < height {
            LerpDrawVert(
                &mut *(*ctrl.offset(j as isize)).as_mut_ptr().offset(i as isize),
                &mut *(*ctrl.offset((j + 1) as isize))
                    .as_mut_ptr()
                    .offset(i as isize),
                &mut prev,
            );
            LerpDrawVert(
                &mut *(*ctrl.offset(j as isize)).as_mut_ptr().offset(i as isize),
                &mut *(*ctrl.offset((j - 1) as isize))
                    .as_mut_ptr()
                    .offset(i as isize),
                &mut next,
            );
            LerpDrawVert(
                &mut prev,
                &mut next,
                &mut *(*ctrl.offset(j as isize)).as_mut_ptr().offset(i as isize),
            );
            j += 2
        }
        i += 1
    }
    j = 0;
    while j < height {
        i = 1;
        while i < width {
            LerpDrawVert(
                &mut *(*ctrl.offset(j as isize)).as_mut_ptr().offset(i as isize),
                &mut *(*ctrl.offset(j as isize))
                    .as_mut_ptr()
                    .offset((i + 1) as isize),
                &mut prev,
            );
            LerpDrawVert(
                &mut *(*ctrl.offset(j as isize)).as_mut_ptr().offset(i as isize),
                &mut *(*ctrl.offset(j as isize))
                    .as_mut_ptr()
                    .offset((i - 1) as isize),
                &mut next,
            );
            LerpDrawVert(
                &mut prev,
                &mut next,
                &mut *(*ctrl.offset(j as isize)).as_mut_ptr().offset(i as isize),
            );
            i += 2
        }
        j += 1
    }
}
/*
=================
R_CreateSurfaceGridMesh
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_CreateSurfaceGridMesh(
    mut width: i32,
    mut height: i32,
    mut ctrl: *mut [crate::qfiles_h::drawVert_t; 65],
    mut errorTable: *mut [f32; 65],
) -> *mut crate::tr_local_h::srfGridMesh_t {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut size: i32 = 0;
    let mut vert: *mut crate::qfiles_h::drawVert_t = 0 as *mut crate::qfiles_h::drawVert_t;
    let mut tmpVec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut grid: *mut crate::tr_local_h::srfGridMesh_t =
        0 as *mut crate::tr_local_h::srfGridMesh_t;
    // copy the results out to a grid
    size = ((width * height - 1i32) as usize)
        .wrapping_mul(::std::mem::size_of::<crate::qfiles_h::drawVert_t>())
        .wrapping_add(::std::mem::size_of::<crate::tr_local_h::srfGridMesh_t>()) as i32;
    grid = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(size)
        as *mut crate::tr_local_h::srfGridMesh_t;
    crate::stdlib::memset(grid as *mut libc::c_void, 0, size as usize);
    (*grid).widthLodError = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(width * 4) as *mut f32;
    crate::stdlib::memcpy(
        (*grid).widthLodError as *mut libc::c_void,
        (*errorTable.offset(0)).as_mut_ptr() as *const libc::c_void,
        (width * 4i32) as usize,
    );
    (*grid).heightLodError = crate::src::renderergl1::tr_main::ri
        .Malloc
        .expect("non-null function pointer")(height * 4) as *mut f32;
    crate::stdlib::memcpy(
        (*grid).heightLodError as *mut libc::c_void,
        (*errorTable.offset(1)).as_mut_ptr() as *const libc::c_void,
        (height * 4i32) as usize,
    );
    (*grid).width = width;
    (*grid).height = height;
    (*grid).surfaceType = crate::tr_local_h::SF_GRID;
    crate::src::qcommon::q_math::ClearBounds(
        (*grid).meshBounds[0].as_mut_ptr(),
        (*grid).meshBounds[1].as_mut_ptr(),
    );

    for i in 0..width {
        for j in 0..height {
            vert = &mut *(*grid).verts.as_mut_ptr().offset((j * width + i) as isize)
                as *mut crate::qfiles_h::drawVert_t;

            *vert = (*ctrl.offset(j as isize))[i as usize];

            crate::src::qcommon::q_math::AddPointToBounds(
                (*vert).xyz.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*grid).meshBounds[0].as_mut_ptr(),
                (*grid).meshBounds[1].as_mut_ptr(),
            );
        }
    }
    // compute local origin and bounds
    (*grid).localOrigin[0] = (*grid).meshBounds[0][0] + (*grid).meshBounds[1][0];
    (*grid).localOrigin[1] = (*grid).meshBounds[0][1] + (*grid).meshBounds[1][1];
    (*grid).localOrigin[2] = (*grid).meshBounds[0][2] + (*grid).meshBounds[1][2];
    (*grid).localOrigin[0] = (*grid).localOrigin[0] * 0.5;
    (*grid).localOrigin[1] = (*grid).localOrigin[1] * 0.5;
    (*grid).localOrigin[2] = (*grid).localOrigin[2] * 0.5;
    tmpVec[0] = (*grid).meshBounds[0][0] - (*grid).localOrigin[0];
    tmpVec[1] = (*grid).meshBounds[0][1] - (*grid).localOrigin[1];
    tmpVec[2] = (*grid).meshBounds[0][2] - (*grid).localOrigin[2];
    (*grid).meshRadius =
        VectorLength(tmpVec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    (*grid).lodOrigin[0] = (*grid).localOrigin[0];
    (*grid).lodOrigin[1] = (*grid).localOrigin[1];
    (*grid).lodOrigin[2] = (*grid).localOrigin[2];
    (*grid).lodRadius = (*grid).meshRadius;
    //
    return grid;
}
/*
=================
R_FreeSurfaceGridMesh
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_FreeSurfaceGridMesh(mut grid: *mut crate::tr_local_h::srfGridMesh_t) {
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")((*grid).widthLodError as *mut libc::c_void);
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")((*grid).heightLodError as *mut libc::c_void);
    crate::src::renderergl1::tr_main::ri
        .Free
        .expect("non-null function pointer")(grid as *mut libc::c_void);
}
/*
=================
R_SubdividePatchToGrid
=================
*/
#[no_mangle]

pub unsafe extern "C" fn R_SubdividePatchToGrid(
    mut width: i32,
    mut height: i32,
    mut points: *mut crate::qfiles_h::drawVert_t,
) -> *mut crate::tr_local_h::srfGridMesh_t {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let mut prev: crate::qfiles_h::drawVert_t = {
        let mut init = crate::qfiles_h::drawVert_t {
            xyz: [0f32, 0f32, 0f32],
            st: [0f32, 0f32],
            lightmap: [0f32, 0f32],
            normal: [0f32, 0f32, 0f32],
            color: [0u8, 0, 0, 0],
        };
        init
    };
    let mut next: crate::qfiles_h::drawVert_t = {
        let mut init = crate::qfiles_h::drawVert_t {
            xyz: [0f32, 0f32, 0f32],
            st: [0f32, 0f32],
            lightmap: [0f32, 0f32],
            normal: [0f32, 0f32, 0f32],
            color: [0u8, 0, 0, 0],
        };
        init
    };
    let mut mid: crate::qfiles_h::drawVert_t = {
        let mut init = crate::qfiles_h::drawVert_t {
            xyz: [0f32, 0f32, 0f32],
            st: [0f32, 0f32],
            lightmap: [0f32, 0f32],
            normal: [0f32, 0f32, 0f32],
            color: [0u8, 0, 0, 0],
        };
        init
    };
    let mut len: f32 = 0.;
    let mut maxLen: f32 = 0.;
    let mut dir: i32 = 0;
    let mut t: i32 = 0;
    let mut ctrl: [[crate::qfiles_h::drawVert_t; 65]; 65] = [[crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    }; 65]; 65];
    let mut errorTable: [[f32; 65]; 2] = [[0.; 65]; 2];
    i = 0;
    while i < width {
        j = 0;
        while j < height {
            ctrl[j as usize][i as usize] = *points.offset((j * width + i) as isize);
            j += 1
        }
        i += 1
    }

    for dir in 0..2 {
        j = 0;

        while j < 65 {
            errorTable[dir as usize][j as usize] = 0f32;
            j += 1
        }

        j = 0;

        while (j + 2) < width {
            // check subdivided midpoints against control points
            // FIXME: also check midpoints of adjacent patches against the control points
            // this would basically stitch all patches in the same LOD group together.
            maxLen = 0f32;
            i = 0;
            while i < height {
                let mut midxyz: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut midxyz2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut dir_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut projected: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                let mut d: f32 = 0.;
                // calculate the point on the curve

                for l in 0..3 {
                    midxyz[l as usize] = (ctrl[i as usize][j as usize].xyz[l as usize]
                        + ctrl[i as usize][(j + 1) as usize].xyz[l as usize] * 2f32
                        + ctrl[i as usize][(j + 2) as usize].xyz[l as usize])
                        * 0.25;
                }
                // see how far off the line it is
                // using dist-from-line will not account for internal
                // texture warping, but it gives a lot less polygons than
                // dist-from-midpoint
                midxyz[0] = midxyz[0] - ctrl[i as usize][j as usize].xyz[0]; // we will do the sqrt later
                midxyz[1] = midxyz[1] - ctrl[i as usize][j as usize].xyz[1];
                midxyz[2] = midxyz[2] - ctrl[i as usize][j as usize].xyz[2];
                dir_0[0] =
                    ctrl[i as usize][(j + 2) as usize].xyz[0] - ctrl[i as usize][j as usize].xyz[0];
                dir_0[1] =
                    ctrl[i as usize][(j + 2) as usize].xyz[1] - ctrl[i as usize][j as usize].xyz[1];
                dir_0[2] =
                    ctrl[i as usize][(j + 2) as usize].xyz[2] - ctrl[i as usize][j as usize].xyz[2];
                crate::src::qcommon::q_math::VectorNormalize(dir_0.as_mut_ptr());
                d = midxyz[0] * dir_0[0] + midxyz[1] * dir_0[1] + midxyz[2] * dir_0[2];
                projected[0] = dir_0[0] * d;
                projected[1] = dir_0[1] * d;
                projected[2] = dir_0[2] * d;
                midxyz2[0] = midxyz[0] - projected[0];
                midxyz2[1] = midxyz[1] - projected[1];
                midxyz2[2] = midxyz[2] - projected[2];
                len = VectorLengthSquared(
                    midxyz2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                );
                if len > maxLen {
                    maxLen = len
                }
                i += 1
            }
            maxLen = crate::stdlib::sqrt(maxLen as f64) as f32;
            // if all the points are on the lines, remove the entire columns
            if maxLen < 0.1 {
                errorTable[dir as usize][(j + 1) as usize] = 999f32
            } else if width + 2 > 65 {
                errorTable[dir as usize][(j + 1) as usize] = 1.0 / maxLen
            // see if we want to insert subdivided columns
            // can't subdivide any more
            } else if maxLen <= (*crate::src::renderergl1::tr_init::r_subdivisions).value {
                errorTable[dir as usize][(j + 1) as usize] = 1.0 / maxLen
            // didn't need subdivision
            } else {
                errorTable[dir as usize][(j + 2) as usize] = 1.0 / maxLen;
                // insert two columns and replace the peak
                width += 2;
                i = 0;
                while i < height {
                    LerpDrawVert(
                        &mut *(*ctrl.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset(j as isize),
                        &mut *(*ctrl.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset((j + 1) as isize),
                        &mut prev,
                    );
                    LerpDrawVert(
                        &mut *(*ctrl.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset((j + 1) as isize),
                        &mut *(*ctrl.as_mut_ptr().offset(i as isize))
                            .as_mut_ptr()
                            .offset((j + 2) as isize),
                        &mut next,
                    );
                    LerpDrawVert(&mut prev, &mut next, &mut mid);
                    k = width - 1;
                    while k > j + 3 {
                        ctrl[i as usize][k as usize] = ctrl[i as usize][(k - 2) as usize];
                        k -= 1
                    }
                    ctrl[i as usize][(j + 1) as usize] = prev;
                    ctrl[i as usize][(j + 2) as usize] = mid;
                    ctrl[i as usize][(j + 3) as usize] = next;
                    i += 1
                }
                // back up and recheck this set again, it may need more subdivision
                j -= 2
            }
            j += 2
        }

        Transpose(width, height, ctrl.as_mut_ptr());

        t = width;

        width = height;

        height = t;
    }
    // put all the aproximating points on the curve
    PutPointsOnCurve(ctrl.as_mut_ptr(), width, height);
    // cull out any rows or columns that are colinear
    i = 1;
    while i < width - 1 {
        if !(errorTable[0][i as usize] != 999f32) {
            j = i + 1;
            while j < width {
                k = 0;
                while k < height {
                    ctrl[k as usize][(j - 1) as usize] = ctrl[k as usize][j as usize];
                    k += 1
                }
                errorTable[0][(j - 1) as usize] = errorTable[0][j as usize];
                j += 1
            }
            width -= 1
        }
        i += 1
    }
    i = 1;
    while i < height - 1 {
        if !(errorTable[1][i as usize] != 999f32) {
            j = i + 1;
            while j < height {
                k = 0;
                while k < width {
                    ctrl[(j - 1) as usize][k as usize] = ctrl[j as usize][k as usize];
                    k += 1
                }
                errorTable[1][(j - 1) as usize] = errorTable[1][j as usize];
                j += 1
            }
            height -= 1
        }
        i += 1
    }
    // flip for longest tristrips as an optimization
    // the results should be visually identical with or
    // without this step
    if height > width {
        Transpose(width, height, ctrl.as_mut_ptr());
        InvertErrorTable(errorTable.as_mut_ptr(), width, height);
        t = width;
        width = height;
        height = t;
        InvertCtrl(width, height, ctrl.as_mut_ptr());
    }
    // calculate normals
    MakeMeshNormals(width, height, ctrl.as_mut_ptr());
    return R_CreateSurfaceGridMesh(width, height, ctrl.as_mut_ptr(), errorTable.as_mut_ptr());
}
/*
===============
R_GridInsertColumn
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_GridInsertColumn(
    mut grid: *mut crate::tr_local_h::srfGridMesh_t,
    mut column: i32,
    mut row: i32,
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut loderror: f32,
) -> *mut crate::tr_local_h::srfGridMesh_t {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut oldwidth: i32 = 0;
    let mut ctrl: [[crate::qfiles_h::drawVert_t; 65]; 65] = [[crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    }; 65]; 65];
    let mut errorTable: [[f32; 65]; 2] = [[0.; 65]; 2];
    let mut lodRadius: f32 = 0.;
    let mut lodOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    oldwidth = 0;
    width = (*grid).width + 1;
    if width > 65 {
        return 0 as *mut crate::tr_local_h::srfGridMesh_t;
    }
    height = (*grid).height;

    for i in 0..width {
        if i == column {
            //insert new column
            j = 0;
            while j < (*grid).height {
                LerpDrawVert(
                    &mut *(*grid)
                        .verts
                        .as_mut_ptr()
                        .offset((j * (*grid).width + i - 1) as isize),
                    &mut *(*grid)
                        .verts
                        .as_mut_ptr()
                        .offset((j * (*grid).width + i) as isize),
                    &mut *(*ctrl.as_mut_ptr().offset(j as isize))
                        .as_mut_ptr()
                        .offset(i as isize),
                );
                if j == row {
                    ctrl[j as usize][i as usize].xyz[0] = *point.offset(0);
                    ctrl[j as usize][i as usize].xyz[1] = *point.offset(1);
                    ctrl[j as usize][i as usize].xyz[2] = *point.offset(2)
                }
                j += 1
            }
            errorTable[0][i as usize] = loderror
        } else {
            errorTable[0][i as usize] = *(*grid).widthLodError.offset(oldwidth as isize);
            j = 0;
            while j < (*grid).height {
                ctrl[j as usize][i as usize] = *(*grid)
                    .verts
                    .as_mut_ptr()
                    .offset((j * (*grid).width + oldwidth) as isize);
                j += 1
            }
            oldwidth += 1
        }
    }
    j = 0;
    while j < (*grid).height {
        errorTable[1][j as usize] = *(*grid).heightLodError.offset(j as isize);
        j += 1
    }
    // put all the aproximating points on the curve
    //PutPointsOnCurve( ctrl, width, height );
    // calculate normals
    MakeMeshNormals(width, height, ctrl.as_mut_ptr());
    lodOrigin[0] = (*grid).lodOrigin[0];
    lodOrigin[1] = (*grid).lodOrigin[1];
    lodOrigin[2] = (*grid).lodOrigin[2];
    lodRadius = (*grid).lodRadius;
    // free the old grid
    R_FreeSurfaceGridMesh(grid);
    // create a new grid
    grid = R_CreateSurfaceGridMesh(width, height, ctrl.as_mut_ptr(), errorTable.as_mut_ptr());
    (*grid).lodRadius = lodRadius;
    (*grid).lodOrigin[0] = lodOrigin[0];
    (*grid).lodOrigin[1] = lodOrigin[1];
    (*grid).lodOrigin[2] = lodOrigin[2];
    return grid;
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
// 14 bits
// can't be increased without changing bit packing for drawsurfs
// see QSORT_SHADERNUM_SHIFT
// range from 0.0 to 1.0, should be color normalized
// origin in local coordinate system
// texture detail is lost tho when the lightmap is dark
// a trRefEntity_t has all the information passed in by
// the client game, as well as some locally derived info
// compensate for non-normalized axis
// true for bmodels that touch a dlight
// normalized direction towards light
// color normalized to 0-255
// 32 bit rgba packed
// in world coordinates
// orientation in world
// viewParms->or.origin in local coordinates
//===============================================================================
// mirrors, portals, viewscreens
// sky box
// opaque
// scorch marks, etc.
// ladders, grates, grills that may have small blended edges
// in addition to alpha test
// for items that should be drawn in front of the water plane
// regular transparency and filters
// generally only used for additive type effects
// gun smoke puffs
// blood blobs
// tr.identityLight
// always (1,1,1,1)
// grabbed from entity's modulate field
// grabbed from 1 - entity.modulate
// tess.vertexColors
// tess.vertexColors * tr.identityLight
// programmatically generated
// standard fog
// fixed color
// clear to 0,0
// S and T from world coordinates
// vertex coordinate modification type
// used for TMOD_TURBULENT and TMOD_STRETCH
// used for TMOD_TRANSFORM
// s' = s * m[0][0] + t * m[1][0] + trans[0]
// t' = s * m[0][1] + t * m[0][1] + trans[1]
// used for TMOD_SCALE
// s *= scale[0]
// t *= scale[1]
// used for TMOD_SCROLL
// s' = s + scroll[0] * time
// t' = t + scroll[1] * time
// + = clockwise
// - = counterclockwise
// for CGEN_CONST and AGEN_CONST
// GLS_xxxx mask
// surface is translucent and will just be adjusted properly
// surface is opaque but possibly alpha tested
// surface is trnaslucent, but still needs a fog pass (fog surface)
// game path, including extension
// for a shader to match, both name and lightmapIndex must match
// this shader == tr.shaders[index]
// this shader == tr.sortedShaders[sortedIndex]
// lower numbered shaders draw before higher numbered
// we want to return index 0 if the shader failed to
// load for some reason, but R_FindShader should
// still keep a name allocated for it, so if
// something calls RE_RegisterShader again with
// the same name, we don't try looking for it again
// found in a .shader file
// if explicitlyDefined, this will have SURF_* flags
// merge across entites optimizable (smoke, blood)
// distance to fog out at
// 0, GL_MODULATE, GL_ADD (FIXME: put in stage)
// CT_FRONT_SIDED, CT_BACK_SIDED, or CT_TWO_SIDED
// set for decals and other items that must be offset
// for console fonts, 2D elements, etc.
// for images that must always be full resolution
// draw a blended pass, possibly with depth test equals
// not all shaders will need all data to be gathered
// time this shader is clamped to
// current time offset for this shader
// current shader this one is remapped too
// trRefdef_t holds everything that comes in refdef_t,
// as well as the locally generated scene information
// transformation matrix
// time in milliseconds for shader effects and other time dependent rendering issues
// RDF_NOWORLDMODEL, etc
// 1 bits will prevent the associated area from rendering at all
// qtrue if areamask changed since last scene
// tr.refdef.time / 1000.0
// text messages for deform text shaders
//=================================================================================
// max surfaces per-skin
// This is an arbitry limit. Vanilla Q3 only supported 32 surfaces in skins but failed to
// enforce the maximum limit when reading skin files. It was possile to use more than 32
// surfaces which accessed out of bounds memory past end of skin->surfaces hunk block.
// skins allow models to be retextured without modifying the model file
// game path, including extension
// dynamically allocated array of surfaces
// in packed byte format
// texture coordinate vector scales
// for clipping distance in fog when outside
// may be different than or.origin for portals
// true if this view is through a portal
// the portal is a mirror, invert the face culling
// copied from tr.frameSceneNum
// copied from tr.frameCount
// clip anything behind this if mirroring
/*
==============================================================================

SURFACES

==============================================================================
*/
// any changes in surfaceType must be mirrored in rb_surfaceTable[]
// ignore
// beams, rails, lightning, etc that can be determined by entity
// ensures that sizeof( surfaceType_t ) == sizeof( int )
// bit combination for fast compares
// any of surface*_t
// max dimensions of a patch mesh in map file
// max dimensions of a grid mesh in memory
// when cgame directly specifies a polygon, it becomes a srfPoly_t
// as soon as it is called
// dynamic lighting information
// culling information
// lod information, which may be different
// than the culling information to allow for
// groups of curves that LOD as a unit
// vertexes
// variable sized
// dynamic lighting information
// triangle definitions (no normals at points)
// variable sized
// there is a variable length list of indices here also
// misc_models in maps are turned into direct geometry by q3map
// dynamic lighting information
// culling information (FIXME: use this!)
// triangle definitions
// inter-quake-model
// vertex arrays
// [num_vertexes] indexes into influenceBlendVertexes
// unique list of vertex blend indexes/weights for faster CPU vertex skinning
// [num_influences]
// [num_influences]
// depending upon the exporter, blend indices and weights might be int/float
// as opposed to the recommended byte/byte, for example Noesis exports
// int/float whereas the official IQM tool exports byte/byte
// IQM_UBYTE or IQM_FLOAT
// inter-quake-model surface
/*
==============================================================================

BRUSH MODELS

==============================================================================
*/
//
// in memory representation
//
// if == tr.viewCount, already added
// any of srf*_t
// common with leaf and node
// -1 for nodes, to differentiate from leafs
// node needs to be traversed if current
// for bounding box culling
// node specific
// leaf specific
// for culling
// ie: maps/tim_dm2.bsp
// ie: tim_dm2
// includes leafs
// may be passed in by CM_LoadMap to save space
// clusterBytes of 0xff
//======================================================================
// model = tr.models[model->index]
// just for listing purposes
// only if type == MOD_BRUSH
// only if type == MOD_MESH
// only if type == (MOD_MDR | MOD_IQM)
//====================================================
/*

the drawsurf sort data is packed into a single 32 bit value so it can be
compared quickly during the qsorting process

the bits are allocated as follows:

0 - 1	: dlightmap index
//2		: used to be clipped flag REMOVED - 03.21.00 rad
2 - 6	: fog index
11 - 20	: entity index
21 - 31	: sorted shader index

    TTimo - 1.32
0-1   : dlightmap index
2-6   : fog index
7-16  : entity index
17-30 : sorted shader index
*/
/*
** performanceCounters_t
*/
// the renderer front end should never modify glstate_t
// total msec for backend run
// all state modified by the back end is separated
// from the front end state
// flag for drawing sun
// if qtrue, drawstretchpic doesn't need to change modes
// shader needs to be finished
// currentEntity will point at this when doing 2D rendering
/*
** trGlobals_t
**
** Most renderer globals are defined here.
** backend functions should never modify any of these fields,
** but may read fields that aren't dynamically modified
** by the frontend.
*/
// cleared at shutdown, set at beginRegistration
// incremented every time a new vis cluster is entered
// incremented every frame
// incremented every scene
// incremented every view (twice a scene if portaled)
// and every R_MarkFragments call
// zeroed at RE_BeginFrame
// from RE_SetWorldVisData, shared with CM_Load
// inverse-quare highlight for projective adding
// full of 0xff
// full of tr.identityLightByte
// point currentEntity at this when rendering world
// currentEntityNum << QSORT_REFENTITYNUM_SHIFT
// 1.0 / ( 1 << overbrightBits )
// identityLight * 255
// r_overbrightBits->integer, but set to 0 if no hw gamma
// for current entity
// from the sky shader for this level
// not in pc due to clearing issue
//
// put large tables at the end, so most elements will be
// within the +/32K indexed range on risc processors
//
// shader indexes from other modules will be looked up in tr.shaders[]
// shader indexes from drawsurfs will be looked up in sortedShaders[]
// lower indexed sortedShaders must be rendered first (opaque surfaces before translucent)
// outside of TR since it shouldn't be cleared during ref re-init
//
// cvars
//
// coefficient for the flare intensity falloff function.
// used for debugging anything
// used for verbose debug spew
// allows us to ignore our Tess fast paths
// near Z clip plane
// z distance of projection plane
// separation of cameras for stereo rendering
// enables stencil buffer overdraw measurement
// push/pull LOD transitions
// "0" = based on compiled vertex array existence
// "1" = glDrawElemet tristrips
// "2" = glDrawElements triangles
// "-1" = no drawing
// controls whether in game video should be draw
// controls whether sky should be cleared or drawn
// controls drawing of sun quad
// dynamic lights enabled/disabled
// dlight non-facing surfaces for continuity
// bypasses the ref rendering
// disable/enable entity rendering
// disable/enable world rendering
// various levels of information display
// enables/disables detail texturing stages
// disable/enable usage of PVS
// enables culling of planar surfaces with back side test
// optional display refresh option
// turns off binding to appropriate textures
// make most world faces use default shader
// development aid to see texture mip usage
// controls picmip values
// avoid lightmap pass
// render lightmaps only
// vertex lighting mode for better performance
// ui is running fullscreen
// number of frames to emit GL logs
// enables wireframe rendering of the world
// forces sky in front of all surfaces
// draws wireframe normals
// force screen clear every frame
// controls shadows: 0 = none, 1 = blur, 2 = stencil, 3 = black planar projection
// light flares
//====================================================================
// completely unclipped
// clipped by one or more planes
// completely outside the clipping planes
/*
** GL wrapper/helper functions
*/
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=516
//
// tr_shader.c
//
/*
====================================================================

TESSELATOR/SHADER DECLARATIONS

====================================================================
*/
// or together of all vertexDlightBits
// info extracted from current shader
/*
============================================================

WORLD MAP

============================================================
*/
/*
============================================================

FLARES

============================================================
*/
/*
============================================================

LIGHTS

============================================================
*/
/*
============================================================

SHADOWS

============================================================
*/
/*
============================================================

SKIES

============================================================
*/
/*
============================================================

CURVE TESSELATION

============================================================
*/
/*
===============
R_GridInsertRow
===============
*/
#[no_mangle]

pub unsafe extern "C" fn R_GridInsertRow(
    mut grid: *mut crate::tr_local_h::srfGridMesh_t,
    mut row: i32,
    mut column: i32,
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut loderror: f32,
) -> *mut crate::tr_local_h::srfGridMesh_t {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut oldheight: i32 = 0;
    let mut ctrl: [[crate::qfiles_h::drawVert_t; 65]; 65] = [[crate::qfiles_h::drawVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        lightmap: [0.; 2],
        normal: [0.; 3],
        color: [0; 4],
    }; 65]; 65];
    let mut errorTable: [[f32; 65]; 2] = [[0.; 65]; 2];
    let mut lodRadius: f32 = 0.;
    let mut lodOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    oldheight = 0;
    width = (*grid).width;
    height = (*grid).height + 1;
    if height > 65 {
        return 0 as *mut crate::tr_local_h::srfGridMesh_t;
    }

    for i in 0..height {
        if i == row {
            //insert new row
            j = 0;
            while j < (*grid).width {
                LerpDrawVert(
                    &mut *(*grid)
                        .verts
                        .as_mut_ptr()
                        .offset(((i - 1) * (*grid).width + j) as isize),
                    &mut *(*grid)
                        .verts
                        .as_mut_ptr()
                        .offset((i * (*grid).width + j) as isize),
                    &mut *(*ctrl.as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(j as isize),
                );
                if j == column {
                    ctrl[i as usize][j as usize].xyz[0] = *point.offset(0);
                    ctrl[i as usize][j as usize].xyz[1] = *point.offset(1);
                    ctrl[i as usize][j as usize].xyz[2] = *point.offset(2)
                }
                j += 1
            }
            errorTable[1][i as usize] = loderror
        } else {
            errorTable[1][i as usize] = *(*grid).heightLodError.offset(oldheight as isize);
            j = 0;
            while j < (*grid).width {
                ctrl[i as usize][j as usize] = *(*grid)
                    .verts
                    .as_mut_ptr()
                    .offset((oldheight * (*grid).width + j) as isize);
                j += 1
            }
            oldheight += 1
        }
    }
    j = 0;
    while j < (*grid).width {
        errorTable[0][j as usize] = *(*grid).widthLodError.offset(j as isize);
        j += 1
    }
    // put all the aproximating points on the curve
    //PutPointsOnCurve( ctrl, width, height );
    // calculate normals
    MakeMeshNormals(width, height, ctrl.as_mut_ptr());
    lodOrigin[0] = (*grid).lodOrigin[0];
    lodOrigin[1] = (*grid).lodOrigin[1];
    lodOrigin[2] = (*grid).lodOrigin[2];
    lodRadius = (*grid).lodRadius;
    // free the old grid
    R_FreeSurfaceGridMesh(grid);
    // create a new grid
    grid = R_CreateSurfaceGridMesh(width, height, ctrl.as_mut_ptr(), errorTable.as_mut_ptr());
    (*grid).lodRadius = lodRadius;
    (*grid).lodOrigin[0] = lodOrigin[0];
    (*grid).lodOrigin[1] = lodOrigin[1];
    (*grid).lodOrigin[2] = lodOrigin[2];
    return grid;
}
