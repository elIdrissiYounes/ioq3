// =============== BEGIN cm_polylib_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct winding_t {
    pub numpoints: i32,
    pub p: [crate::src::qcommon::q_shared::vec3_t; 0],
}
use ::libc;

pub mod q_shared_h {

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

pub use crate::stdlib::intptr_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::src::qcommon::cm_polylib::q_shared_h::CrossProduct;
pub use crate::src::qcommon::cm_polylib::q_shared_h::VectorLength;
pub use crate::src::qcommon::common::Com_Error;

pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;

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
// counters are only bumped when running single threaded,
// because they are an awful coherence problem
#[no_mangle]

pub static mut c_active_windings: i32 = 0;
#[no_mangle]

pub static mut c_peak_windings: i32 = 0;
#[no_mangle]

pub static mut c_winding_allocs: i32 = 0;
#[no_mangle]

pub static mut c_winding_points: i32 = 0;
// frees the original if clipped
#[no_mangle]

pub unsafe extern "C" fn pw(mut w: *mut crate::src::qcommon::cm_polylib::winding_t) {
    let mut i: i32 = 0;
    i = 0;
    while i < (*w).numpoints {
        crate::stdlib::printf(
            b"(%5.1f, %5.1f, %5.1f)\n\x00" as *const u8 as *const i8,
            (*(*w).p.as_mut_ptr().offset(i as isize))[0usize] as f64,
            (*(*w).p.as_mut_ptr().offset(i as isize))[1usize] as f64,
            (*(*w).p.as_mut_ptr().offset(i as isize))[2usize] as f64,
        );
        i += 1
    }
}
/*
=============
AllocWinding
=============
*/
#[no_mangle]

pub unsafe extern "C" fn AllocWinding(
    mut points: i32,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut s: i32 = 0;
    c_winding_allocs += 1;
    c_winding_points += points;
    c_active_windings += 1;
    if c_active_windings > c_peak_windings {
        c_peak_windings = c_active_windings
    }
    s = (::std::mem::size_of::<crate::src::qcommon::q_shared::vec_t>())
        .wrapping_mul(3usize)
        .wrapping_mul(points as usize)
        .wrapping_add(::std::mem::size_of::<i32>()) as i32;
    w = crate::src::qcommon::common::Z_Malloc(s) as *mut crate::src::qcommon::cm_polylib::winding_t;
    crate::stdlib::memset(w as *mut libc::c_void, 0, s as usize);
    return w;
}
#[no_mangle]

pub unsafe extern "C" fn FreeWinding(mut w: *mut crate::src::qcommon::cm_polylib::winding_t) {
    if *(w as *mut u32) == 0xdeaddead {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"FreeWinding: freed a freed winding\x00" as *const u8 as *const i8,
        );
    }
    *(w as *mut u32) = 0xdeaddead;
    c_active_windings -= 1;
    crate::src::qcommon::common::Z_Free(w as *mut libc::c_void);
}
/*
============
RemoveColinearPoints
============
*/
#[no_mangle]

pub static mut c_removed: i32 = 0;
#[no_mangle]

pub unsafe extern "C" fn RemoveColinearPoints(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) {
    let mut _i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut nump: i32 = 0;
    let mut p: [crate::src::qcommon::q_shared::vec3_t; 64] = [[0.; 3]; 64];
    nump = 0;

    for i in 0..(*w).numpoints {
        j = (i + 1) % (*w).numpoints;

        k = (i + (*w).numpoints - 1) % (*w).numpoints;

        v1[0] = (*(*w).p.as_mut_ptr().offset(j as isize))[0]
            - (*(*w).p.as_mut_ptr().offset(i as isize))[0];

        v1[1] = (*(*w).p.as_mut_ptr().offset(j as isize))[1]
            - (*(*w).p.as_mut_ptr().offset(i as isize))[1];

        v1[2] = (*(*w).p.as_mut_ptr().offset(j as isize))[2]
            - (*(*w).p.as_mut_ptr().offset(i as isize))[2];

        v2[0] = (*(*w).p.as_mut_ptr().offset(i as isize))[0]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[0];

        v2[1] = (*(*w).p.as_mut_ptr().offset(i as isize))[1]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[1];

        v2[2] = (*(*w).p.as_mut_ptr().offset(i as isize))[2]
            - (*(*w).p.as_mut_ptr().offset(k as isize))[2];

        crate::src::qcommon::q_math::VectorNormalize2(
            v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            v1.as_mut_ptr(),
        );

        crate::src::qcommon::q_math::VectorNormalize2(
            v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            v2.as_mut_ptr(),
        );

        if ((v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]) as f64) < 0.999 {
            p[nump as usize][0] = (*(*w).p.as_mut_ptr().offset(i as isize))[0];
            p[nump as usize][1] = (*(*w).p.as_mut_ptr().offset(i as isize))[1];
            p[nump as usize][2] = (*(*w).p.as_mut_ptr().offset(i as isize))[2];
            nump += 1
        }
    }
    if nump == (*w).numpoints {
        return;
    }
    c_removed += (*w).numpoints - nump;
    (*w).numpoints = nump;
    crate::stdlib::memcpy(
        (*w).p.as_mut_ptr() as *mut libc::c_void,
        p.as_mut_ptr() as *const libc::c_void,
        (nump as usize)
            .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::vec3_t>()),
    );
}
/*
============
WindingPlane
============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingPlane(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut v1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    v1[0] = (*(*w).p.as_mut_ptr().offset(1))[0] - (*(*w).p.as_mut_ptr().offset(0))[0];
    v1[1] = (*(*w).p.as_mut_ptr().offset(1))[1] - (*(*w).p.as_mut_ptr().offset(0))[1];
    v1[2] = (*(*w).p.as_mut_ptr().offset(1))[2] - (*(*w).p.as_mut_ptr().offset(0))[2];
    v2[0] = (*(*w).p.as_mut_ptr().offset(2))[0] - (*(*w).p.as_mut_ptr().offset(0))[0];
    v2[1] = (*(*w).p.as_mut_ptr().offset(2))[1] - (*(*w).p.as_mut_ptr().offset(0))[1];
    v2[2] = (*(*w).p.as_mut_ptr().offset(2))[2] - (*(*w).p.as_mut_ptr().offset(0))[2];
    CrossProduct(
        v2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        v1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        normal,
    );
    crate::src::qcommon::q_math::VectorNormalize2(
        normal as *const crate::src::qcommon::q_shared::vec_t,
        normal,
    );
    *dist = (*(*w).p.as_mut_ptr().offset(0))[0] * *normal.offset(0)
        + (*(*w).p.as_mut_ptr().offset(0))[1] * *normal.offset(1)
        + (*(*w).p.as_mut_ptr().offset(0))[2] * *normal.offset(2);
}
/*
=============
WindingArea
=============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingArea(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) -> crate::src::qcommon::q_shared::vec_t {
    let mut _i: i32 = 0;
    let mut d1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cross: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut total: crate::src::qcommon::q_shared::vec_t = 0.;
    total = 0f32;

    for i in 2..(*w).numpoints {
        d1[0] = (*(*w).p.as_mut_ptr().offset((i - 1) as isize))[0]
            - (*(*w).p.as_mut_ptr().offset(0))[0];

        d1[1] = (*(*w).p.as_mut_ptr().offset((i - 1) as isize))[1]
            - (*(*w).p.as_mut_ptr().offset(0))[1];

        d1[2] = (*(*w).p.as_mut_ptr().offset((i - 1) as isize))[2]
            - (*(*w).p.as_mut_ptr().offset(0))[2];

        d2[0] = (*(*w).p.as_mut_ptr().offset(i as isize))[0] - (*(*w).p.as_mut_ptr().offset(0))[0];

        d2[1] = (*(*w).p.as_mut_ptr().offset(i as isize))[1] - (*(*w).p.as_mut_ptr().offset(0))[1];

        d2[2] = (*(*w).p.as_mut_ptr().offset(i as isize))[2] - (*(*w).p.as_mut_ptr().offset(0))[2];

        CrossProduct(
            d1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            d2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            cross.as_mut_ptr(),
        );

        total = (total as f64
            + 0.5
                * VectorLength(cross.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                    as f64) as crate::src::qcommon::q_shared::vec_t;
    }
    return total;
}
/*
=============
WindingBounds
=============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingBounds(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut v: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let ref mut fresh0 = *mins.offset(2);
    *fresh0 = 65535f32;
    let ref mut fresh1 = *mins.offset(1);
    *fresh1 = *fresh0;
    *mins.offset(0) = *fresh1;
    let ref mut fresh2 = *maxs.offset(2);
    *fresh2 = -65535f32;
    let ref mut fresh3 = *maxs.offset(1);
    *fresh3 = *fresh2;
    *maxs.offset(0) = *fresh3;
    i = 0;
    while i < (*w).numpoints {
        for j in 0..3 {
            v = (*(*w).p.as_mut_ptr().offset(i as isize))[j as usize];

            if v < *mins.offset(j as isize) {
                *mins.offset(j as isize) = v
            }

            if v > *maxs.offset(j as isize) {
                *maxs.offset(j as isize) = v
            }
        }
        i += 1
    }
}
/*
=============
WindingCenter
=============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingCenter(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut center: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut _i: i32 = 0;
    let mut scale: f32 = 0.;
    *center.offset(0) = crate::src::qcommon::q_math::vec3_origin[0];
    *center.offset(1) = crate::src::qcommon::q_math::vec3_origin[1];
    *center.offset(2) = crate::src::qcommon::q_math::vec3_origin[2];

    for i in 0..(*w).numpoints {
        *center.offset(0) = (*(*w).p.as_mut_ptr().offset(i as isize))[0] + *center.offset(0);

        *center.offset(1) = (*(*w).p.as_mut_ptr().offset(i as isize))[1] + *center.offset(1);

        *center.offset(2) = (*(*w).p.as_mut_ptr().offset(i as isize))[2] + *center.offset(2);
    }
    scale = (1.0 / (*w).numpoints as f64) as f32;
    *center.offset(0) = *center.offset(0) * scale;
    *center.offset(1) = *center.offset(1) * scale;
    *center.offset(2) = *center.offset(2) * scale;
}
/*
=================
BaseWindingForPlane
=================
*/
#[no_mangle]

pub unsafe extern "C" fn BaseWindingForPlane(
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut _i: i32 = 0;
    let mut x: i32 = 0;
    let mut max: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut v: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vright: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut w: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    // find the major axis
    max = -65535f32;
    x = -(1);

    for i in 0..3 {
        v = crate::stdlib::fabs(*normal.offset(i as isize) as f64)
            as crate::src::qcommon::q_shared::vec_t;

        if v > max {
            x = i;
            max = v
        }
    }
    if x == -(1) {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"BaseWindingForPlane: no axis found\x00" as *const u8 as *const i8,
        );
    }
    vup[0] = crate::src::qcommon::q_math::vec3_origin[0];
    vup[1] = crate::src::qcommon::q_math::vec3_origin[1];
    vup[2] = crate::src::qcommon::q_math::vec3_origin[2];
    match x {
        0 | 1 => vup[2] = 1f32,
        2 => vup[0] = 1f32,
        _ => {}
    }
    v = vup[0] * *normal.offset(0) + vup[1] * *normal.offset(1) + vup[2] * *normal.offset(2);
    vup[0] = vup[0] + *normal.offset(0) * -v;
    vup[1] = vup[1] + *normal.offset(1) * -v;
    vup[2] = vup[2] + *normal.offset(2) * -v;
    crate::src::qcommon::q_math::VectorNormalize2(
        vup.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vup.as_mut_ptr(),
    );
    org[0] = *normal.offset(0) * dist;
    org[1] = *normal.offset(1) * dist;
    org[2] = *normal.offset(2) * dist;
    CrossProduct(
        vup.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        normal as *const crate::src::qcommon::q_shared::vec_t,
        vright.as_mut_ptr(),
    );
    vup[0] = vup[0] * 65535f32;
    vup[1] = vup[1] * 65535f32;
    vup[2] = vup[2] * 65535f32;
    vright[0] = vright[0] * 65535f32;
    vright[1] = vright[1] * 65535f32;
    vright[2] = vright[2] * 65535f32;
    // project a really big	axis aligned box onto the plane
    w = AllocWinding(4);
    (*(*w).p.as_mut_ptr().offset(0))[0] = org[0] - vright[0];
    (*(*w).p.as_mut_ptr().offset(0))[1] = org[1] - vright[1];
    (*(*w).p.as_mut_ptr().offset(0))[2] = org[2] - vright[2];
    (*(*w).p.as_mut_ptr().offset(0))[0] = (*(*w).p.as_mut_ptr().offset(0))[0] + vup[0];
    (*(*w).p.as_mut_ptr().offset(0))[1] = (*(*w).p.as_mut_ptr().offset(0))[1] + vup[1];
    (*(*w).p.as_mut_ptr().offset(0))[2] = (*(*w).p.as_mut_ptr().offset(0))[2] + vup[2];
    (*(*w).p.as_mut_ptr().offset(1))[0] = org[0] + vright[0];
    (*(*w).p.as_mut_ptr().offset(1))[1] = org[1] + vright[1];
    (*(*w).p.as_mut_ptr().offset(1))[2] = org[2] + vright[2];
    (*(*w).p.as_mut_ptr().offset(1))[0] = (*(*w).p.as_mut_ptr().offset(1))[0] + vup[0];
    (*(*w).p.as_mut_ptr().offset(1))[1] = (*(*w).p.as_mut_ptr().offset(1))[1] + vup[1];
    (*(*w).p.as_mut_ptr().offset(1))[2] = (*(*w).p.as_mut_ptr().offset(1))[2] + vup[2];
    (*(*w).p.as_mut_ptr().offset(2))[0] = org[0] + vright[0];
    (*(*w).p.as_mut_ptr().offset(2))[1] = org[1] + vright[1];
    (*(*w).p.as_mut_ptr().offset(2))[2] = org[2] + vright[2];
    (*(*w).p.as_mut_ptr().offset(2))[0] = (*(*w).p.as_mut_ptr().offset(2))[0] - vup[0];
    (*(*w).p.as_mut_ptr().offset(2))[1] = (*(*w).p.as_mut_ptr().offset(2))[1] - vup[1];
    (*(*w).p.as_mut_ptr().offset(2))[2] = (*(*w).p.as_mut_ptr().offset(2))[2] - vup[2];
    (*(*w).p.as_mut_ptr().offset(3))[0] = org[0] - vright[0];
    (*(*w).p.as_mut_ptr().offset(3))[1] = org[1] - vright[1];
    (*(*w).p.as_mut_ptr().offset(3))[2] = org[2] - vright[2];
    (*(*w).p.as_mut_ptr().offset(3))[0] = (*(*w).p.as_mut_ptr().offset(3))[0] - vup[0];
    (*(*w).p.as_mut_ptr().offset(3))[1] = (*(*w).p.as_mut_ptr().offset(3))[1] - vup[1];
    (*(*w).p.as_mut_ptr().offset(3))[2] = (*(*w).p.as_mut_ptr().offset(3))[2] - vup[2];
    (*w).numpoints = 4;
    return w;
}
/*
==================
CopyWinding
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CopyWinding(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut size: crate::stdlib::intptr_t = 0;
    let mut c: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    c = AllocWinding((*w).numpoints);
    size = &mut *(*w).p.as_mut_ptr().offset((*w).numpoints as isize)
        as *mut crate::src::qcommon::q_shared::vec3_t as crate::stdlib::intptr_t
        - w as crate::stdlib::intptr_t;
    crate::stdlib::memcpy(
        c as *mut libc::c_void,
        w as *const libc::c_void,
        size as usize,
    );
    return c;
}
/*
==================
ReverseWinding
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ReverseWinding(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut _i: i32 = 0;
    let mut c: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    c = AllocWinding((*w).numpoints);

    for i in 0..(*w).numpoints {
        (*(*c).p.as_mut_ptr().offset(i as isize))[0] = (*(*w)
            .p
            .as_mut_ptr()
            .offset(((*w).numpoints - 1 - i) as isize))[0];

        (*(*c).p.as_mut_ptr().offset(i as isize))[1] = (*(*w)
            .p
            .as_mut_ptr()
            .offset(((*w).numpoints - 1 - i) as isize))[1];

        (*(*c).p.as_mut_ptr().offset(i as isize))[2] = (*(*w)
            .p
            .as_mut_ptr()
            .offset(((*w).numpoints - 1 - i) as isize))[2];
    }
    (*c).numpoints = (*w).numpoints;
    return c;
}
/*
=============
ClipWindingEpsilon
=============
*/
#[no_mangle]

pub unsafe extern "C" fn ClipWindingEpsilon(
    mut in_0: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
    mut epsilon: crate::src::qcommon::q_shared::vec_t,
    mut front: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
    mut back: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
) {
    let mut dists: [crate::src::qcommon::q_shared::vec_t; 68] = [
        0f32, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ]; // VC 4.2 optimizer bug if not static
    let mut sides: [i32; 68] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut counts: [i32; 3] = [0; 3];
    static mut dot: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let mut p1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut p2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut b: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut maxpts: i32 = 0;
    counts[2] = 0;
    counts[1] = counts[2];
    counts[0] = counts[1];
    // determine sides for each point
    i = 0; // can't use counts[0]+2 because
    while i < (*in_0).numpoints {
        dot = (*(*in_0).p.as_mut_ptr().offset(i as isize))[0] * *normal.offset(0)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[1] * *normal.offset(1)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[2] * *normal.offset(2);
        dot -= dist;
        dists[i as usize] = dot;
        if dot > epsilon {
            sides[i as usize] = 0
        } else if dot < -epsilon {
            sides[i as usize] = 1
        } else {
            sides[i as usize] = 2
        }
        counts[sides[i as usize] as usize] += 1;
        i += 1
    }
    sides[i as usize] = sides[0];
    dists[i as usize] = dists[0];
    *back = 0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    *front = *back;
    if counts[0] == 0 {
        *back = CopyWinding(in_0);
        return;
    }
    if counts[1] == 0 {
        *front = CopyWinding(in_0);
        return;
    }
    maxpts = (*in_0).numpoints + 4;
    // of fp grouping errors
    f = AllocWinding(maxpts);
    *front = f;
    b = AllocWinding(maxpts);
    *back = b;
    i = 0;
    while i < (*in_0).numpoints {
        p1 = (*(*in_0).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        if sides[i as usize] == 2 {
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0] = *p1.offset(0);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1] = *p1.offset(1);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2] = *p1.offset(2);
            (*f).numpoints += 1;
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0] = *p1.offset(0);
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1] = *p1.offset(1);
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2] = *p1.offset(2);
            (*b).numpoints += 1
        } else {
            if sides[i as usize] == 0 {
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0] = *p1.offset(0);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1] = *p1.offset(1);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2] = *p1.offset(2);
                (*f).numpoints += 1
            }
            if sides[i as usize] == 1 {
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0] = *p1.offset(0);
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1] = *p1.offset(1);
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2] = *p1.offset(2);
                (*b).numpoints += 1
            }
            if !(sides[(i + 1) as usize] == 2 || sides[(i + 1) as usize] == sides[i as usize]) {
                // generate a split point
                p2 = (*(*in_0)
                    .p
                    .as_mut_ptr()
                    .offset(((i + 1) % (*in_0).numpoints) as isize))
                .as_mut_ptr();
                dot = dists[i as usize] / (dists[i as usize] - dists[(i + 1) as usize]);

                for j in 0..3 {
                    if *normal.offset(j as isize) == 1f32 {
                        mid[j as usize] = dist
                    } else if *normal.offset(j as isize) == -1f32 {
                        mid[j as usize] = -dist
                    } else {
                        mid[j as usize] = *p1.offset(j as isize)
                            + dot * (*p2.offset(j as isize) - *p1.offset(j as isize))
                    }
                }
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0] = mid[0];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1] = mid[1];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2] = mid[2];
                (*f).numpoints += 1;
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0] = mid[0];
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1] = mid[1];
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2] = mid[2];
                (*b).numpoints += 1
            }
        }
        i += 1
    }
    if (*f).numpoints > maxpts || (*b).numpoints > maxpts {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ClipWinding: points exceeded estimate\x00" as *const u8 as *const i8,
        );
    }
    if (*f).numpoints > 64 || (*b).numpoints > 64 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ClipWinding: MAX_POINTS_ON_WINDING\x00" as *const u8 as *const i8,
        );
    };
}
/*
=============
ChopWindingInPlace
=============
*/
#[no_mangle]

pub unsafe extern "C" fn ChopWindingInPlace(
    mut inout: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
    mut epsilon: crate::src::qcommon::q_shared::vec_t,
) {
    let mut in_0: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t; // VC 4.2 optimizer bug if not static
    let mut dists: [crate::src::qcommon::q_shared::vec_t; 68] = [
        0f32, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ];
    let mut sides: [i32; 68] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut counts: [i32; 3] = [0; 3];
    static mut dot: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let mut p1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut p2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut mid: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut maxpts: i32 = 0;
    in_0 = *inout;
    counts[2] = 0;
    counts[1] = counts[2];
    counts[0] = counts[1];
    // determine sides for each point
    i = 0; // inout stays the same
    while i < (*in_0).numpoints {
        dot = (*(*in_0).p.as_mut_ptr().offset(i as isize))[0] * *normal.offset(0)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[1] * *normal.offset(1)
            + (*(*in_0).p.as_mut_ptr().offset(i as isize))[2] * *normal.offset(2); // can't use counts[0]+2 because
        dot -= dist;
        dists[i as usize] = dot;
        if dot > epsilon {
            sides[i as usize] = 0
        } else if dot < -epsilon {
            sides[i as usize] = 1
        } else {
            sides[i as usize] = 2
        }
        counts[sides[i as usize] as usize] += 1;
        i += 1
    }
    sides[i as usize] = sides[0];
    dists[i as usize] = dists[0];
    if counts[0] == 0 {
        FreeWinding(in_0);
        *inout = 0 as *mut crate::src::qcommon::cm_polylib::winding_t;
        return;
    }
    if counts[1] == 0 {
        return;
    }
    maxpts = (*in_0).numpoints + 4;
    // of fp grouping errors
    f = AllocWinding(maxpts);
    i = 0;
    while i < (*in_0).numpoints {
        p1 = (*(*in_0).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        if sides[i as usize] == 2 {
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0] = *p1.offset(0);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1] = *p1.offset(1);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2] = *p1.offset(2);
            (*f).numpoints += 1
        } else {
            if sides[i as usize] == 0 {
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0] = *p1.offset(0);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1] = *p1.offset(1);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2] = *p1.offset(2);
                (*f).numpoints += 1
            }
            if !(sides[(i + 1) as usize] == 2 || sides[(i + 1) as usize] == sides[i as usize]) {
                // generate a split point
                p2 = (*(*in_0)
                    .p
                    .as_mut_ptr()
                    .offset(((i + 1) % (*in_0).numpoints) as isize))
                .as_mut_ptr();
                dot = dists[i as usize] / (dists[i as usize] - dists[(i + 1) as usize]);

                for j in 0..3 {
                    if *normal.offset(j as isize) == 1f32 {
                        mid[j as usize] = dist
                    } else if *normal.offset(j as isize) == -1f32 {
                        mid[j as usize] = -dist
                    } else {
                        mid[j as usize] = *p1.offset(j as isize)
                            + dot * (*p2.offset(j as isize) - *p1.offset(j as isize))
                    }
                }
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0] = mid[0];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1] = mid[1];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2] = mid[2];
                (*f).numpoints += 1
            }
        }
        i += 1
    }
    if (*f).numpoints > maxpts {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ClipWinding: points exceeded estimate\x00" as *const u8 as *const i8,
        );
    }
    if (*f).numpoints > 64 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"ClipWinding: MAX_POINTS_ON_WINDING\x00" as *const u8 as *const i8,
        );
    }
    FreeWinding(in_0);
    *inout = f;
}
/*
=================
ChopWinding

Returns the fragment of in that is on the front side
of the cliping plane.  The original is freed.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ChopWinding(
    mut in_0: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
) -> *mut crate::src::qcommon::cm_polylib::winding_t {
    let mut f: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    let mut b: *mut crate::src::qcommon::cm_polylib::winding_t =
        0 as *mut crate::src::qcommon::cm_polylib::winding_t;
    ClipWindingEpsilon(in_0, normal, dist, 0.1, &mut f, &mut b);
    FreeWinding(in_0);
    if !b.is_null() {
        FreeWinding(b);
    }
    return f;
}
/*
=================
CheckWinding

=================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckWinding(mut w: *mut crate::src::qcommon::cm_polylib::winding_t) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p1: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut p2: *mut crate::src::qcommon::q_shared::vec_t =
        0 as *mut crate::src::qcommon::q_shared::vec_t;
    let mut d: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut edgedist: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut edgenormal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut facenormal: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut area: crate::src::qcommon::q_shared::vec_t = 0.;
    let mut facedist: crate::src::qcommon::q_shared::vec_t = 0.;
    if (*w).numpoints < 3 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CheckWinding: %i points\x00" as *const u8 as *const i8,
            (*w).numpoints,
        );
    }
    area = WindingArea(w);
    if area < 1f32 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"CheckWinding: %f area\x00" as *const u8 as *const i8,
            area as f64,
        );
    }
    WindingPlane(w, facenormal.as_mut_ptr(), &mut facedist);
    i = 0;
    while i < (*w).numpoints {
        p1 = (*(*w).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        j = 0;
        while j < 3 {
            if *p1.offset(j as isize) > 65535f32 || *p1.offset(j as isize) < -65535f32 {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"CheckFace: BUGUS_RANGE: %f\x00" as *const u8 as *const i8,
                    *p1.offset(j as isize) as f64,
                );
            }
            j += 1
        }
        j = if i + 1 == (*w).numpoints { 0 } else { (i) + 1 };
        // check the point is on the face plane
        d = *p1.offset(0) * facenormal[0]
            + *p1.offset(1) * facenormal[1]
            + *p1.offset(2) * facenormal[2]
            - facedist;
        if d < -0.1 || d > 0.1 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"CheckWinding: point off plane\x00" as *const u8 as *const i8,
            );
        }
        // check the edge isn't degenerate
        p2 = (*(*w).p.as_mut_ptr().offset(j as isize)).as_mut_ptr();
        dir[0] = *p2.offset(0) - *p1.offset(0);
        dir[1] = *p2.offset(1) - *p1.offset(1);
        dir[2] = *p2.offset(2) - *p1.offset(2);
        if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) < 0.1 {
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"CheckWinding: degenerate edge\x00" as *const u8 as *const i8,
            );
        }
        CrossProduct(
            facenormal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            edgenormal.as_mut_ptr(),
        );
        crate::src::qcommon::q_math::VectorNormalize2(
            edgenormal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            edgenormal.as_mut_ptr(),
        );
        edgedist = *p1.offset(0) * edgenormal[0]
            + *p1.offset(1) * edgenormal[1]
            + *p1.offset(2) * edgenormal[2];
        edgedist += 0.1;
        // all other points must be on front side
        j = 0;
        while j < (*w).numpoints {
            if !(j == i) {
                d = (*(*w).p.as_mut_ptr().offset(j as isize))[0] * edgenormal[0]
                    + (*(*w).p.as_mut_ptr().offset(j as isize))[1] * edgenormal[1]
                    + (*(*w).p.as_mut_ptr().offset(j as isize))[2] * edgenormal[2];
                if d > edgedist {
                    crate::src::qcommon::common::Com_Error(
                        crate::src::qcommon::q_shared::ERR_DROP as i32,
                        b"CheckWinding: non-convex\x00" as *const u8 as *const i8,
                    );
                }
            }
            j += 1
        }
        i += 1
    }
}
/*
============
WindingOnPlaneSide
============
*/
#[no_mangle]

pub unsafe extern "C" fn WindingOnPlaneSide(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut dist: crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut front: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut back: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut _i: i32 = 0;
    let mut d: crate::src::qcommon::q_shared::vec_t = 0.;
    front = crate::src::qcommon::q_shared::qfalse;
    back = crate::src::qcommon::q_shared::qfalse;

    for i in 0..(*w).numpoints {
        d = (*(*w).p.as_mut_ptr().offset(i as isize))[0] * *normal.offset(0)
            + (*(*w).p.as_mut_ptr().offset(i as isize))[1] * *normal.offset(1)
            + (*(*w).p.as_mut_ptr().offset(i as isize))[2] * *normal.offset(2)
            - dist;

        if d < -0.1 {
            if front as u64 != 0 {
                return 3i32;
            }
            back = crate::src::qcommon::q_shared::qtrue
        } else if d > 0.1 {
            if back as u64 != 0 {
                return 3i32;
            }
            front = crate::src::qcommon::q_shared::qtrue
        }
    }
    if back as u64 != 0 {
        return 1i32;
    }
    if front as u64 != 0 {
        return 0i32;
    }
    return 2;
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
// this is only used for visualization tools in cm_ debug functions
// variable sized
// you can define on_epsilon in the makefile as tighter
/*
=================
AddWindingToConvexHull

Both w and *hull are on the same plane
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AddWindingToConvexHull(
    mut w: *mut crate::src::qcommon::cm_polylib::winding_t,
    mut hull: *mut *mut crate::src::qcommon::cm_polylib::winding_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut _i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut p: *mut f32 = 0 as *mut f32;
    let mut copy: *mut f32 = 0 as *mut f32;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d: f32 = 0.;
    let mut numHullPoints: i32 = 0;
    let mut numNew: i32 = 0;
    let mut hullPoints: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut newHullPoints: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut hullDirs: [crate::src::qcommon::q_shared::vec3_t; 128] = [[0.; 3]; 128];
    let mut hullSide: [crate::src::qcommon::q_shared::qboolean; 128] =
        [crate::src::qcommon::q_shared::qfalse; 128];
    let mut outside: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    if (*hull).is_null() {
        *hull = CopyWinding(w);
        return;
    }
    numHullPoints = (**hull).numpoints;
    crate::stdlib::memcpy(
        hullPoints.as_mut_ptr() as *mut libc::c_void,
        (**hull).p.as_mut_ptr() as *const libc::c_void,
        (numHullPoints as usize)
            .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::vec3_t>()),
    );

    for i in 0..(*w).numpoints {
        p = (*(*w).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();

        j = 0;

        while j < numHullPoints {
            k = (j + 1) % numHullPoints;
            dir[0] = hullPoints[k as usize][0] - hullPoints[j as usize][0];
            dir[1] = hullPoints[k as usize][1] - hullPoints[j as usize][1];
            dir[2] = hullPoints[k as usize][2] - hullPoints[j as usize][2];
            crate::src::qcommon::q_math::VectorNormalize2(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                dir.as_mut_ptr(),
            );
            CrossProduct(
                normal as *const crate::src::qcommon::q_shared::vec_t,
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                hullDirs[j as usize].as_mut_ptr(),
            );
            j += 1
        }

        outside = crate::src::qcommon::q_shared::qfalse;

        j = 0;

        while j < numHullPoints {
            dir[0] = *p.offset(0) - hullPoints[j as usize][0];
            dir[1] = *p.offset(1) - hullPoints[j as usize][1];
            dir[2] = *p.offset(2) - hullPoints[j as usize][2];
            d = dir[0] * hullDirs[j as usize][0]
                + dir[1] * hullDirs[j as usize][1]
                + dir[2] * hullDirs[j as usize][2];
            if d >= 0.1 {
                outside = crate::src::qcommon::q_shared::qtrue
            }
            if d >= -0.1 {
                hullSide[j as usize] = crate::src::qcommon::q_shared::qtrue
            } else {
                hullSide[j as usize] = crate::src::qcommon::q_shared::qfalse
            }
            j += 1
        }

        if !(outside as u64 == 0) {
            // find the back side to front side transition
            j = 0;
            while j < numHullPoints {
                if hullSide[(j % numHullPoints) as usize] as u64 == 0
                    && hullSide[((j + 1) % numHullPoints) as usize] != 0
                {
                    break;
                }
                j += 1
            }
            if !(j == numHullPoints) {
                // insert the point here
                newHullPoints[0][0] = *p.offset(0);
                newHullPoints[0][1] = *p.offset(1);
                newHullPoints[0][2] = *p.offset(2);
                numNew = 1;
                // copy over all points that aren't double fronts
                j = (j + 1) % numHullPoints;
                k = 0;
                while k < numHullPoints {
                    if !(hullSide[((j + k) % numHullPoints) as usize] != 0
                        && hullSide[((j + k + 1) % numHullPoints) as usize] != 0)
                    {
                        copy = hullPoints[((j + k + 1) % numHullPoints) as usize].as_mut_ptr();
                        newHullPoints[numNew as usize][0] = *copy.offset(0);
                        newHullPoints[numNew as usize][1] = *copy.offset(1);
                        newHullPoints[numNew as usize][2] = *copy.offset(2);
                        numNew += 1
                    }
                    k += 1
                }
                numHullPoints = numNew;
                crate::stdlib::memcpy(
                    hullPoints.as_mut_ptr() as *mut libc::c_void,
                    newHullPoints.as_mut_ptr() as *const libc::c_void,
                    (numHullPoints as usize).wrapping_mul(::std::mem::size_of::<
                        crate::src::qcommon::q_shared::vec3_t,
                    >()),
                );
            }
        }
    }
    FreeWinding(*hull);
    w = AllocWinding(numHullPoints);
    (*w).numpoints = numHullPoints;
    *hull = w;
    crate::stdlib::memcpy(
        (*w).p.as_mut_ptr() as *mut libc::c_void,
        hullPoints.as_mut_ptr() as *const libc::c_void,
        (numHullPoints as usize)
            .wrapping_mul(::std::mem::size_of::<crate::src::qcommon::q_shared::vec3_t>()),
    );
}
