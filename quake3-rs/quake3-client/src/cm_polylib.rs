#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(custom_attribute, libc)]
extern crate libc;
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
}
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
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
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut vec3_origin: vec3_t;
        #[no_mangle]
        pub fn VectorNormalize2(v: *const vec_t, out: *mut vec_t) -> vec_t;
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
    }
}
#[header_src =
      "ioq3/code/qcommon/cm_polylib.h"]
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
        pub p: [vec3_t; 0],
    }
    use super::{libc};
    use super::q_shared_h::{vec3_t, vec_t};
}
#[header_src = "/usr/include/bits/mathcalls.h"]
pub mod mathcalls_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn printf(_: *const libc::c_char, ...) -> libc::c_int;
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
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/qcommon/cm_polylib.c"]
pub mod cm_polylib_c {
    use super::{libc};
}
use self::stdint_h::{intptr_t};
use self::q_shared_h::{qboolean, qtrue, qfalse, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, vec_t, vec3_t, vec3_origin,
                       VectorNormalize2, Com_Error};
use self::cm_polylib_h::{winding_t};
use self::mathcalls_h::{sqrt, fabs};
use self::stdio_h::{printf};
use self::string_h::{memcpy, memset};
use self::qcommon_h::{Z_MallocDebug, Z_Free};
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
// you can define on_epsilon in the makefile as tighter
#[no_mangle]
pub unsafe extern "C" fn AllocWinding(mut points: libc::c_int)
 -> *mut winding_t {
    let mut w: *mut winding_t = 0 as *mut winding_t;
    let mut s: libc::c_int = 0;
    c_winding_allocs += 1;
    c_winding_points += points;
    c_active_windings += 1;
    if c_active_windings > c_peak_windings {
        c_peak_windings = c_active_windings
    }
    s =
        (::std::mem::size_of::<vec_t>() as
             libc::c_ulong).wrapping_mul(3i32 as
                                             libc::c_ulong).wrapping_mul(points
                                                                             as
                                                                             libc::c_ulong).wrapping_add(::std::mem::size_of::<libc::c_int>()
                                                                                                             as
                                                                                                             libc::c_ulong)
            as libc::c_int;
    w =
        Z_MallocDebug(s,
                      b"s\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/cm_polylib.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 61i32) as
            *mut winding_t;
    memset(w as *mut libc::c_void, 0i32, s as libc::c_ulong);
    return w;
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
// counters are only bumped when running single threaded,
// because they are an awful coherence problem
#[no_mangle]
pub static mut c_active_windings: libc::c_int = 0;
#[no_mangle]
pub static mut c_peak_windings: libc::c_int = 0;
#[no_mangle]
pub static mut c_winding_points: libc::c_int = 0;
#[no_mangle]
pub static mut c_winding_allocs: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn WindingArea(mut w: *mut winding_t) -> vec_t {
    let mut i: libc::c_int = 0;
    let mut d1: vec3_t = [0.; 3];
    let mut d2: vec3_t = [0.; 3];
    let mut cross: vec3_t = [0.; 3];
    let mut total: vec_t = 0.;
    total = 0i32 as vec_t;
    i = 2i32;
    while i < (*w).numpoints {
        d1[0usize] =
            (*(*w).p.as_mut_ptr().offset((i - 1i32) as isize))[0usize] -
                (*(*w).p.as_mut_ptr().offset(0isize))[0usize];
        d1[1usize] =
            (*(*w).p.as_mut_ptr().offset((i - 1i32) as isize))[1usize] -
                (*(*w).p.as_mut_ptr().offset(0isize))[1usize];
        d1[2usize] =
            (*(*w).p.as_mut_ptr().offset((i - 1i32) as isize))[2usize] -
                (*(*w).p.as_mut_ptr().offset(0isize))[2usize];
        d2[0usize] =
            (*(*w).p.as_mut_ptr().offset(i as isize))[0usize] -
                (*(*w).p.as_mut_ptr().offset(0isize))[0usize];
        d2[1usize] =
            (*(*w).p.as_mut_ptr().offset(i as isize))[1usize] -
                (*(*w).p.as_mut_ptr().offset(0isize))[1usize];
        d2[2usize] =
            (*(*w).p.as_mut_ptr().offset(i as isize))[2usize] -
                (*(*w).p.as_mut_ptr().offset(0isize))[2usize];
        CrossProduct(d1.as_mut_ptr() as *const vec_t,
                     d2.as_mut_ptr() as *const vec_t, cross.as_mut_ptr());
        total =
            (total as libc::c_double +
                 0.5f64 *
                     VectorLength(cross.as_mut_ptr() as *const vec_t) as
                         libc::c_double) as vec_t;
        i += 1
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn WindingCenter(mut w: *mut winding_t,
                                       mut center: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    *center.offset(0isize) = vec3_origin[0usize];
    *center.offset(1isize) = vec3_origin[1usize];
    *center.offset(2isize) = vec3_origin[2usize];
    i = 0i32;
    while i < (*w).numpoints {
        *center.offset(0isize) =
            (*(*w).p.as_mut_ptr().offset(i as isize))[0usize] +
                *center.offset(0isize);
        *center.offset(1isize) =
            (*(*w).p.as_mut_ptr().offset(i as isize))[1usize] +
                *center.offset(1isize);
        *center.offset(2isize) =
            (*(*w).p.as_mut_ptr().offset(i as isize))[2usize] +
                *center.offset(2isize);
        i += 1
    }
    scale = (1.0f64 / (*w).numpoints as libc::c_double) as libc::c_float;
    *center.offset(0isize) = *center.offset(0isize) * scale;
    *center.offset(1isize) = *center.offset(1isize) * scale;
    *center.offset(2isize) = *center.offset(2isize) * scale;
}
#[no_mangle]
pub unsafe extern "C" fn ClipWindingEpsilon(mut in_0: *mut winding_t,
                                            mut normal: *mut vec_t,
                                            mut dist: vec_t,
                                            mut epsilon: vec_t,
                                            mut front: *mut *mut winding_t,
                                            mut back: *mut *mut winding_t) {
    let mut dists: [vec_t; 68] =
        [0i32 as vec_t, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0.];
    let mut sides: [libc::c_int; 68] =
        [0i32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut counts: [libc::c_int; 3] = [0; 3];
    // VC 4.2 optimizer bug if not static
    static mut dot: vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut vec_t = 0 as *mut vec_t;
    let mut p2: *mut vec_t = 0 as *mut vec_t;
    let mut mid: vec3_t = [0.; 3];
    let mut f: *mut winding_t = 0 as *mut winding_t;
    let mut b: *mut winding_t = 0 as *mut winding_t;
    let mut maxpts: libc::c_int = 0;
    counts[2usize] = 0i32;
    counts[1usize] = counts[2usize];
    counts[0usize] = counts[1usize];
    i = 0i32;
    while i < (*in_0).numpoints {
        dot =
            (*(*in_0).p.as_mut_ptr().offset(i as isize))[0usize] *
                *normal.offset(0isize) +
                (*(*in_0).p.as_mut_ptr().offset(i as isize))[1usize] *
                    *normal.offset(1isize) +
                (*(*in_0).p.as_mut_ptr().offset(i as isize))[2usize] *
                    *normal.offset(2isize);
        dot -= dist;
        dists[i as usize] = dot;
        if dot > epsilon {
            sides[i as usize] = 0i32
        } else if dot < -epsilon {
            sides[i as usize] = 1i32
        } else { sides[i as usize] = 2i32 }
        counts[sides[i as usize] as usize] += 1;
        i += 1
    }
    sides[i as usize] = sides[0usize];
    dists[i as usize] = dists[0usize];
    *back = 0 as *mut winding_t;
    *front = *back;
    if 0 == counts[0usize] { *back = CopyWinding(in_0); return }
    if 0 == counts[1usize] { *front = CopyWinding(in_0); return }
    maxpts = (*in_0).numpoints + 4i32;
    f = AllocWinding(maxpts);
    *front = f;
    b = AllocWinding(maxpts);
    *back = b;
    i = 0i32;
    while i < (*in_0).numpoints {
        p1 = (*(*in_0).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        if sides[i as usize] == 2i32 {
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0usize] =
                *p1.offset(0isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1usize] =
                *p1.offset(1isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2usize] =
                *p1.offset(2isize);
            (*f).numpoints += 1;
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0usize] =
                *p1.offset(0isize);
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1usize] =
                *p1.offset(1isize);
            (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2usize] =
                *p1.offset(2isize);
            (*b).numpoints += 1
        } else {
            if sides[i as usize] == 0i32 {
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0usize]
                    = *p1.offset(0isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1usize]
                    = *p1.offset(1isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2usize]
                    = *p1.offset(2isize);
                (*f).numpoints += 1
            }
            if sides[i as usize] == 1i32 {
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0usize]
                    = *p1.offset(0isize);
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1usize]
                    = *p1.offset(1isize);
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2usize]
                    = *p1.offset(2isize);
                (*b).numpoints += 1
            }
            if !(sides[(i + 1i32) as usize] == 2i32 ||
                     sides[(i + 1i32) as usize] == sides[i as usize]) {
                p2 =
                    (*(*in_0).p.as_mut_ptr().offset(((i + 1i32) %
                                                         (*in_0).numpoints) as
                                                        isize)).as_mut_ptr();
                dot =
                    dists[i as usize] /
                        (dists[i as usize] - dists[(i + 1i32) as usize]);
                j = 0i32;
                while j < 3i32 {
                    if *normal.offset(j as isize) == 1i32 as libc::c_float {
                        mid[j as usize] = dist
                    } else if *normal.offset(j as isize) ==
                                  -1i32 as libc::c_float {
                        mid[j as usize] = -dist
                    } else {
                        mid[j as usize] =
                            *p1.offset(j as isize) +
                                dot *
                                    (*p2.offset(j as isize) -
                                         *p1.offset(j as isize))
                    }
                    j += 1
                }
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0usize]
                    = mid[0usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1usize]
                    = mid[1usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2usize]
                    = mid[2usize];
                (*f).numpoints += 1;
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[0usize]
                    = mid[0usize];
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[1usize]
                    = mid[1usize];
                (*(*b).p.as_mut_ptr().offset((*b).numpoints as isize))[2usize]
                    = mid[2usize];
                (*b).numpoints += 1
            }
        }
        i += 1
    }
    if (*f).numpoints > maxpts || (*b).numpoints > maxpts {
        Com_Error(ERR_DROP as libc::c_int,
                  b"ClipWinding: points exceeded estimate\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*f).numpoints > 64i32 || (*b).numpoints > 64i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"ClipWinding: MAX_POINTS_ON_WINDING\x00" as *const u8 as
                      *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CopyWinding(mut w: *mut winding_t)
 -> *mut winding_t {
    let mut size: intptr_t = 0;
    let mut c: *mut winding_t = 0 as *mut winding_t;
    c = AllocWinding((*w).numpoints);
    size =
        &mut *(*w).p.as_mut_ptr().offset((*w).numpoints as isize) as
            *mut vec3_t as intptr_t - w as intptr_t;
    memcpy(c as *mut libc::c_void, w as *const libc::c_void,
           size as libc::c_ulong);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn ChopWinding(mut in_0: *mut winding_t,
                                     mut normal: *mut vec_t, mut dist: vec_t)
 -> *mut winding_t {
    let mut f: *mut winding_t = 0 as *mut winding_t;
    let mut b: *mut winding_t = 0 as *mut winding_t;
    ClipWindingEpsilon(in_0, normal, dist, 0.1f32, &mut f, &mut b);
    FreeWinding(in_0);
    if !b.is_null() { FreeWinding(b); }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn FreeWinding(mut w: *mut winding_t) {
    if *(w as *mut libc::c_uint) == 0xdeaddeadu32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"FreeWinding: freed a freed winding\x00" as *const u8 as
                      *const libc::c_char);
    }
    *(w as *mut libc::c_uint) = 0xdeaddeadu32;
    c_active_windings -= 1;
    Z_Free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ReverseWinding(mut w: *mut winding_t)
 -> *mut winding_t {
    let mut i: libc::c_int = 0;
    let mut c: *mut winding_t = 0 as *mut winding_t;
    c = AllocWinding((*w).numpoints);
    i = 0i32;
    while i < (*w).numpoints {
        (*(*c).p.as_mut_ptr().offset(i as isize))[0usize] =
            (*(*w).p.as_mut_ptr().offset(((*w).numpoints - 1i32 - i) as
                                             isize))[0usize];
        (*(*c).p.as_mut_ptr().offset(i as isize))[1usize] =
            (*(*w).p.as_mut_ptr().offset(((*w).numpoints - 1i32 - i) as
                                             isize))[1usize];
        (*(*c).p.as_mut_ptr().offset(i as isize))[2usize] =
            (*(*w).p.as_mut_ptr().offset(((*w).numpoints - 1i32 - i) as
                                             isize))[2usize];
        i += 1
    }
    (*c).numpoints = (*w).numpoints;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn BaseWindingForPlane(mut normal: *mut vec_t,
                                             mut dist: vec_t)
 -> *mut winding_t {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut max: vec_t = 0.;
    let mut v: vec_t = 0.;
    let mut org: vec3_t = [0.; 3];
    let mut vright: vec3_t = [0.; 3];
    let mut vup: vec3_t = [0.; 3];
    let mut w: *mut winding_t = 0 as *mut winding_t;
    max = -65535i32 as vec_t;
    x = -1i32;
    i = 0i32;
    while i < 3i32 {
        v = fabs(*normal.offset(i as isize) as libc::c_double) as vec_t;
        if v > max { x = i; max = v }
        i += 1
    }
    if x == -1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"BaseWindingForPlane: no axis found\x00" as *const u8 as
                      *const libc::c_char);
    }
    vup[0usize] = vec3_origin[0usize];
    vup[1usize] = vec3_origin[1usize];
    vup[2usize] = vec3_origin[2usize];
    match x {
        0 | 1 => { vup[2usize] = 1i32 as vec_t }
        2 => { vup[0usize] = 1i32 as vec_t }
        _ => { }
    }
    v =
        vup[0usize] * *normal.offset(0isize) +
            vup[1usize] * *normal.offset(1isize) +
            vup[2usize] * *normal.offset(2isize);
    vup[0usize] = vup[0usize] + *normal.offset(0isize) * -v;
    vup[1usize] = vup[1usize] + *normal.offset(1isize) * -v;
    vup[2usize] = vup[2usize] + *normal.offset(2isize) * -v;
    VectorNormalize2(vup.as_mut_ptr() as *const vec_t, vup.as_mut_ptr());
    org[0usize] = *normal.offset(0isize) * dist;
    org[1usize] = *normal.offset(1isize) * dist;
    org[2usize] = *normal.offset(2isize) * dist;
    CrossProduct(vup.as_mut_ptr() as *const vec_t, normal as *const vec_t,
                 vright.as_mut_ptr());
    vup[0usize] = vup[0usize] * 65535i32 as libc::c_float;
    vup[1usize] = vup[1usize] * 65535i32 as libc::c_float;
    vup[2usize] = vup[2usize] * 65535i32 as libc::c_float;
    vright[0usize] = vright[0usize] * 65535i32 as libc::c_float;
    vright[1usize] = vright[1usize] * 65535i32 as libc::c_float;
    vright[2usize] = vright[2usize] * 65535i32 as libc::c_float;
    w = AllocWinding(4i32);
    (*(*w).p.as_mut_ptr().offset(0isize))[0usize] =
        org[0usize] - vright[0usize];
    (*(*w).p.as_mut_ptr().offset(0isize))[1usize] =
        org[1usize] - vright[1usize];
    (*(*w).p.as_mut_ptr().offset(0isize))[2usize] =
        org[2usize] - vright[2usize];
    (*(*w).p.as_mut_ptr().offset(0isize))[0usize] =
        (*(*w).p.as_mut_ptr().offset(0isize))[0usize] + vup[0usize];
    (*(*w).p.as_mut_ptr().offset(0isize))[1usize] =
        (*(*w).p.as_mut_ptr().offset(0isize))[1usize] + vup[1usize];
    (*(*w).p.as_mut_ptr().offset(0isize))[2usize] =
        (*(*w).p.as_mut_ptr().offset(0isize))[2usize] + vup[2usize];
    (*(*w).p.as_mut_ptr().offset(1isize))[0usize] =
        org[0usize] + vright[0usize];
    (*(*w).p.as_mut_ptr().offset(1isize))[1usize] =
        org[1usize] + vright[1usize];
    (*(*w).p.as_mut_ptr().offset(1isize))[2usize] =
        org[2usize] + vright[2usize];
    (*(*w).p.as_mut_ptr().offset(1isize))[0usize] =
        (*(*w).p.as_mut_ptr().offset(1isize))[0usize] + vup[0usize];
    (*(*w).p.as_mut_ptr().offset(1isize))[1usize] =
        (*(*w).p.as_mut_ptr().offset(1isize))[1usize] + vup[1usize];
    (*(*w).p.as_mut_ptr().offset(1isize))[2usize] =
        (*(*w).p.as_mut_ptr().offset(1isize))[2usize] + vup[2usize];
    (*(*w).p.as_mut_ptr().offset(2isize))[0usize] =
        org[0usize] + vright[0usize];
    (*(*w).p.as_mut_ptr().offset(2isize))[1usize] =
        org[1usize] + vright[1usize];
    (*(*w).p.as_mut_ptr().offset(2isize))[2usize] =
        org[2usize] + vright[2usize];
    (*(*w).p.as_mut_ptr().offset(2isize))[0usize] =
        (*(*w).p.as_mut_ptr().offset(2isize))[0usize] - vup[0usize];
    (*(*w).p.as_mut_ptr().offset(2isize))[1usize] =
        (*(*w).p.as_mut_ptr().offset(2isize))[1usize] - vup[1usize];
    (*(*w).p.as_mut_ptr().offset(2isize))[2usize] =
        (*(*w).p.as_mut_ptr().offset(2isize))[2usize] - vup[2usize];
    (*(*w).p.as_mut_ptr().offset(3isize))[0usize] =
        org[0usize] - vright[0usize];
    (*(*w).p.as_mut_ptr().offset(3isize))[1usize] =
        org[1usize] - vright[1usize];
    (*(*w).p.as_mut_ptr().offset(3isize))[2usize] =
        org[2usize] - vright[2usize];
    (*(*w).p.as_mut_ptr().offset(3isize))[0usize] =
        (*(*w).p.as_mut_ptr().offset(3isize))[0usize] - vup[0usize];
    (*(*w).p.as_mut_ptr().offset(3isize))[1usize] =
        (*(*w).p.as_mut_ptr().offset(3isize))[1usize] - vup[1usize];
    (*(*w).p.as_mut_ptr().offset(3isize))[2usize] =
        (*(*w).p.as_mut_ptr().offset(3isize))[2usize] - vup[2usize];
    (*w).numpoints = 4i32;
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn CheckWinding(mut w: *mut winding_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut vec_t = 0 as *mut vec_t;
    let mut p2: *mut vec_t = 0 as *mut vec_t;
    let mut d: vec_t = 0.;
    let mut edgedist: vec_t = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut edgenormal: vec3_t = [0.; 3];
    let mut facenormal: vec3_t = [0.; 3];
    let mut area: vec_t = 0.;
    let mut facedist: vec_t = 0.;
    if (*w).numpoints < 3i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CheckWinding: %i points\x00" as *const u8 as
                      *const libc::c_char, (*w).numpoints);
    }
    area = WindingArea(w);
    if area < 1i32 as libc::c_float {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CheckWinding: %f area\x00" as *const u8 as
                      *const libc::c_char, area as libc::c_double);
    }
    WindingPlane(w, facenormal.as_mut_ptr(), &mut facedist);
    i = 0i32;
    while i < (*w).numpoints {
        p1 = (*(*w).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        j = 0i32;
        while j < 3i32 {
            if *p1.offset(j as isize) > 65535i32 as libc::c_float ||
                   *p1.offset(j as isize) < -65535i32 as libc::c_float {
                Com_Error(ERR_DROP as libc::c_int,
                          b"CheckFace: BUGUS_RANGE: %f\x00" as *const u8 as
                              *const libc::c_char,
                          *p1.offset(j as isize) as libc::c_double);
            }
            j += 1
        }
        j = if i + 1i32 == (*w).numpoints { 0i32 } else { i + 1i32 };
        d =
            *p1.offset(0isize) * facenormal[0usize] +
                *p1.offset(1isize) * facenormal[1usize] +
                *p1.offset(2isize) * facenormal[2usize] - facedist;
        if d < -0.1f32 || d > 0.1f32 {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CheckWinding: point off plane\x00" as *const u8 as
                          *const libc::c_char);
        }
        p2 = (*(*w).p.as_mut_ptr().offset(j as isize)).as_mut_ptr();
        dir[0usize] = *p2.offset(0isize) - *p1.offset(0isize);
        dir[1usize] = *p2.offset(1isize) - *p1.offset(1isize);
        dir[2usize] = *p2.offset(2isize) - *p1.offset(2isize);
        if VectorLength(dir.as_mut_ptr() as *const vec_t) < 0.1f32 {
            Com_Error(ERR_DROP as libc::c_int,
                      b"CheckWinding: degenerate edge\x00" as *const u8 as
                          *const libc::c_char);
        }
        CrossProduct(facenormal.as_mut_ptr() as *const vec_t,
                     dir.as_mut_ptr() as *const vec_t,
                     edgenormal.as_mut_ptr());
        VectorNormalize2(edgenormal.as_mut_ptr() as *const vec_t,
                         edgenormal.as_mut_ptr());
        edgedist =
            *p1.offset(0isize) * edgenormal[0usize] +
                *p1.offset(1isize) * edgenormal[1usize] +
                *p1.offset(2isize) * edgenormal[2usize];
        edgedist += 0.1f32;
        j = 0i32;
        while j < (*w).numpoints {
            if !(j == i) {
                d =
                    (*(*w).p.as_mut_ptr().offset(j as isize))[0usize] *
                        edgenormal[0usize] +
                        (*(*w).p.as_mut_ptr().offset(j as isize))[1usize] *
                            edgenormal[1usize] +
                        (*(*w).p.as_mut_ptr().offset(j as isize))[2usize] *
                            edgenormal[2usize];
                if d > edgedist {
                    Com_Error(ERR_DROP as libc::c_int,
                              b"CheckWinding: non-convex\x00" as *const u8 as
                                  *const libc::c_char);
                }
            }
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn WindingPlane(mut w: *mut winding_t,
                                      mut normal: *mut vec_t,
                                      mut dist: *mut vec_t) {
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    v1[0usize] =
        (*(*w).p.as_mut_ptr().offset(1isize))[0usize] -
            (*(*w).p.as_mut_ptr().offset(0isize))[0usize];
    v1[1usize] =
        (*(*w).p.as_mut_ptr().offset(1isize))[1usize] -
            (*(*w).p.as_mut_ptr().offset(0isize))[1usize];
    v1[2usize] =
        (*(*w).p.as_mut_ptr().offset(1isize))[2usize] -
            (*(*w).p.as_mut_ptr().offset(0isize))[2usize];
    v2[0usize] =
        (*(*w).p.as_mut_ptr().offset(2isize))[0usize] -
            (*(*w).p.as_mut_ptr().offset(0isize))[0usize];
    v2[1usize] =
        (*(*w).p.as_mut_ptr().offset(2isize))[1usize] -
            (*(*w).p.as_mut_ptr().offset(0isize))[1usize];
    v2[2usize] =
        (*(*w).p.as_mut_ptr().offset(2isize))[2usize] -
            (*(*w).p.as_mut_ptr().offset(0isize))[2usize];
    CrossProduct(v2.as_mut_ptr() as *const vec_t,
                 v1.as_mut_ptr() as *const vec_t, normal);
    VectorNormalize2(normal as *const vec_t, normal);
    *dist =
        (*(*w).p.as_mut_ptr().offset(0isize))[0usize] * *normal.offset(0isize)
            +
            (*(*w).p.as_mut_ptr().offset(0isize))[1usize] *
                *normal.offset(1isize) +
            (*(*w).p.as_mut_ptr().offset(0isize))[2usize] *
                *normal.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn RemoveColinearPoints(mut w: *mut winding_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut nump: libc::c_int = 0;
    let mut p: [vec3_t; 64] = [[0.; 3]; 64];
    nump = 0i32;
    i = 0i32;
    while i < (*w).numpoints {
        j = (i + 1i32) % (*w).numpoints;
        k = (i + (*w).numpoints - 1i32) % (*w).numpoints;
        v1[0usize] =
            (*(*w).p.as_mut_ptr().offset(j as isize))[0usize] -
                (*(*w).p.as_mut_ptr().offset(i as isize))[0usize];
        v1[1usize] =
            (*(*w).p.as_mut_ptr().offset(j as isize))[1usize] -
                (*(*w).p.as_mut_ptr().offset(i as isize))[1usize];
        v1[2usize] =
            (*(*w).p.as_mut_ptr().offset(j as isize))[2usize] -
                (*(*w).p.as_mut_ptr().offset(i as isize))[2usize];
        v2[0usize] =
            (*(*w).p.as_mut_ptr().offset(i as isize))[0usize] -
                (*(*w).p.as_mut_ptr().offset(k as isize))[0usize];
        v2[1usize] =
            (*(*w).p.as_mut_ptr().offset(i as isize))[1usize] -
                (*(*w).p.as_mut_ptr().offset(k as isize))[1usize];
        v2[2usize] =
            (*(*w).p.as_mut_ptr().offset(i as isize))[2usize] -
                (*(*w).p.as_mut_ptr().offset(k as isize))[2usize];
        VectorNormalize2(v1.as_mut_ptr() as *const vec_t, v1.as_mut_ptr());
        VectorNormalize2(v2.as_mut_ptr() as *const vec_t, v2.as_mut_ptr());
        if ((v1[0usize] * v2[0usize] + v1[1usize] * v2[1usize] +
                 v1[2usize] * v2[2usize]) as libc::c_double) < 0.999f64 {
            p[nump as usize][0usize] =
                (*(*w).p.as_mut_ptr().offset(i as isize))[0usize];
            p[nump as usize][1usize] =
                (*(*w).p.as_mut_ptr().offset(i as isize))[1usize];
            p[nump as usize][2usize] =
                (*(*w).p.as_mut_ptr().offset(i as isize))[2usize];
            nump += 1
        }
        i += 1
    }
    if nump == (*w).numpoints { return }
    c_removed += (*w).numpoints - nump;
    (*w).numpoints = nump;
    memcpy((*w).p.as_mut_ptr() as *mut libc::c_void,
           p.as_mut_ptr() as *const libc::c_void,
           (nump as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec3_t>() as
                                                libc::c_ulong));
}
/*
============
RemoveColinearPoints
============
*/
#[no_mangle]
pub static mut c_removed: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn WindingOnPlaneSide(mut w: *mut winding_t,
                                            mut normal: *mut vec_t,
                                            mut dist: vec_t) -> libc::c_int {
    let mut front: qboolean = qfalse;
    let mut back: qboolean = qfalse;
    let mut i: libc::c_int = 0;
    let mut d: vec_t = 0.;
    front = qfalse;
    back = qfalse;
    i = 0i32;
    while i < (*w).numpoints {
        d =
            (*(*w).p.as_mut_ptr().offset(i as isize))[0usize] *
                *normal.offset(0isize) +
                (*(*w).p.as_mut_ptr().offset(i as isize))[1usize] *
                    *normal.offset(1isize) +
                (*(*w).p.as_mut_ptr().offset(i as isize))[2usize] *
                    *normal.offset(2isize) - dist;
        if d < -0.1f32 {
            if 0 != front as u64 { return 3i32 }
            back = qtrue
        } else if d > 0.1f32 {
            if 0 != back as u64 { return 3i32 }
            front = qtrue
        }
        i += 1
    }
    if 0 != back as u64 { return 1i32 }
    if 0 != front as u64 { return 0i32 }
    return 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn WindingBounds(mut w: *mut winding_t,
                                       mut mins: *mut vec_t,
                                       mut maxs: *mut vec_t) {
    let mut v: vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let ref mut fresh1 = *mins.offset(1isize);
    let ref mut fresh0 = *mins.offset(2isize);
    *fresh0 = 65535i32 as vec_t;
    *fresh1 = *fresh0;
    *mins.offset(0isize) = *fresh1;
    let ref mut fresh3 = *maxs.offset(1isize);
    let ref mut fresh2 = *maxs.offset(2isize);
    *fresh2 = -65535i32 as vec_t;
    *fresh3 = *fresh2;
    *maxs.offset(0isize) = *fresh3;
    i = 0i32;
    while i < (*w).numpoints {
        j = 0i32;
        while j < 3i32 {
            v = (*(*w).p.as_mut_ptr().offset(i as isize))[j as usize];
            if v < *mins.offset(j as isize) { *mins.offset(j as isize) = v }
            if v > *maxs.offset(j as isize) { *maxs.offset(j as isize) = v }
            j += 1
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn AddWindingToConvexHull(mut w: *mut winding_t,
                                                mut hull: *mut *mut winding_t,
                                                mut normal: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut copy: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dir: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut numHullPoints: libc::c_int = 0;
    let mut numNew: libc::c_int = 0;
    let mut hullPoints: [vec3_t; 128] = [[0.; 3]; 128];
    let mut newHullPoints: [vec3_t; 128] = [[0.; 3]; 128];
    let mut hullDirs: [vec3_t; 128] = [[0.; 3]; 128];
    let mut hullSide: [qboolean; 128] = [qfalse; 128];
    let mut outside: qboolean = qfalse;
    if (*hull).is_null() { *hull = CopyWinding(w); return }
    numHullPoints = (**hull).numpoints;
    memcpy(hullPoints.as_mut_ptr() as *mut libc::c_void,
           (**hull).p.as_mut_ptr() as *const libc::c_void,
           (numHullPoints as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec3_t>() as
                                                libc::c_ulong));
    i = 0i32;
    while i < (*w).numpoints {
        p = (*(*w).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        j = 0i32;
        while j < numHullPoints {
            k = (j + 1i32) % numHullPoints;
            dir[0usize] =
                hullPoints[k as usize][0usize] -
                    hullPoints[j as usize][0usize];
            dir[1usize] =
                hullPoints[k as usize][1usize] -
                    hullPoints[j as usize][1usize];
            dir[2usize] =
                hullPoints[k as usize][2usize] -
                    hullPoints[j as usize][2usize];
            VectorNormalize2(dir.as_mut_ptr() as *const vec_t,
                             dir.as_mut_ptr());
            CrossProduct(normal as *const vec_t,
                         dir.as_mut_ptr() as *const vec_t,
                         hullDirs[j as usize].as_mut_ptr());
            j += 1
        }
        outside = qfalse;
        j = 0i32;
        while j < numHullPoints {
            dir[0usize] = *p.offset(0isize) - hullPoints[j as usize][0usize];
            dir[1usize] = *p.offset(1isize) - hullPoints[j as usize][1usize];
            dir[2usize] = *p.offset(2isize) - hullPoints[j as usize][2usize];
            d =
                dir[0usize] * hullDirs[j as usize][0usize] +
                    dir[1usize] * hullDirs[j as usize][1usize] +
                    dir[2usize] * hullDirs[j as usize][2usize];
            if d >= 0.1f32 { outside = qtrue }
            if d >= -0.1f32 {
                hullSide[j as usize] = qtrue
            } else { hullSide[j as usize] = qfalse }
            j += 1
        }
        // if the point is effectively inside, do nothing
        if !(0 == outside as u64) {
            j = 0i32;
            while j < numHullPoints {
                if 0 == hullSide[(j % numHullPoints) as usize] as u64 &&
                       0 !=
                           hullSide[((j + 1i32) % numHullPoints) as usize] as
                               libc::c_uint {
                    break ;
                }
                j += 1
            }
            if !(j == numHullPoints) {
                newHullPoints[0usize][0usize] = *p.offset(0isize);
                newHullPoints[0usize][1usize] = *p.offset(1isize);
                newHullPoints[0usize][2usize] = *p.offset(2isize);
                numNew = 1i32;
                j = (j + 1i32) % numHullPoints;
                k = 0i32;
                while k < numHullPoints {
                    if !(0 !=
                             hullSide[((j + k) % numHullPoints) as usize] as
                                 libc::c_uint &&
                             0 !=
                                 hullSide[((j + k + 1i32) % numHullPoints) as
                                              usize] as libc::c_uint) {
                        copy =
                            hullPoints[((j + k + 1i32) % numHullPoints) as
                                           usize].as_mut_ptr();
                        newHullPoints[numNew as usize][0usize] =
                            *copy.offset(0isize);
                        newHullPoints[numNew as usize][1usize] =
                            *copy.offset(1isize);
                        newHullPoints[numNew as usize][2usize] =
                            *copy.offset(2isize);
                        numNew += 1
                    }
                    k += 1
                }
                numHullPoints = numNew;
                memcpy(hullPoints.as_mut_ptr() as *mut libc::c_void,
                       newHullPoints.as_mut_ptr() as *const libc::c_void,
                       (numHullPoints as
                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec3_t>()
                                                            as
                                                            libc::c_ulong));
            }
        }
        i += 1
    }
    FreeWinding(*hull);
    w = AllocWinding(numHullPoints);
    (*w).numpoints = numHullPoints;
    *hull = w;
    memcpy((*w).p.as_mut_ptr() as *mut libc::c_void,
           hullPoints.as_mut_ptr() as *const libc::c_void,
           (numHullPoints as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<vec3_t>() as
                                                libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn ChopWindingInPlace(mut inout: *mut *mut winding_t,
                                            mut normal: *mut vec_t,
                                            mut dist: vec_t,
                                            mut epsilon: vec_t) {
    let mut in_0: *mut winding_t = 0 as *mut winding_t;
    let mut dists: [vec_t; 68] =
        [0i32 as vec_t, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
         0., 0., 0.];
    let mut sides: [libc::c_int; 68] =
        [0i32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
         0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut counts: [libc::c_int; 3] = [0; 3];
    // VC 4.2 optimizer bug if not static
    static mut dot: vec_t = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p1: *mut vec_t = 0 as *mut vec_t;
    let mut p2: *mut vec_t = 0 as *mut vec_t;
    let mut mid: vec3_t = [0.; 3];
    let mut f: *mut winding_t = 0 as *mut winding_t;
    let mut maxpts: libc::c_int = 0;
    in_0 = *inout;
    counts[2usize] = 0i32;
    counts[1usize] = counts[2usize];
    counts[0usize] = counts[1usize];
    i = 0i32;
    while i < (*in_0).numpoints {
        dot =
            (*(*in_0).p.as_mut_ptr().offset(i as isize))[0usize] *
                *normal.offset(0isize) +
                (*(*in_0).p.as_mut_ptr().offset(i as isize))[1usize] *
                    *normal.offset(1isize) +
                (*(*in_0).p.as_mut_ptr().offset(i as isize))[2usize] *
                    *normal.offset(2isize);
        dot -= dist;
        dists[i as usize] = dot;
        if dot > epsilon {
            sides[i as usize] = 0i32
        } else if dot < -epsilon {
            sides[i as usize] = 1i32
        } else { sides[i as usize] = 2i32 }
        counts[sides[i as usize] as usize] += 1;
        i += 1
    }
    sides[i as usize] = sides[0usize];
    dists[i as usize] = dists[0usize];
    if 0 == counts[0usize] {
        FreeWinding(in_0);
        *inout = 0 as *mut winding_t;
        return
    }
    if 0 == counts[1usize] { return }
    maxpts = (*in_0).numpoints + 4i32;
    f = AllocWinding(maxpts);
    i = 0i32;
    while i < (*in_0).numpoints {
        p1 = (*(*in_0).p.as_mut_ptr().offset(i as isize)).as_mut_ptr();
        if sides[i as usize] == 2i32 {
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0usize] =
                *p1.offset(0isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1usize] =
                *p1.offset(1isize);
            (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2usize] =
                *p1.offset(2isize);
            (*f).numpoints += 1
        } else {
            if sides[i as usize] == 0i32 {
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0usize]
                    = *p1.offset(0isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1usize]
                    = *p1.offset(1isize);
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2usize]
                    = *p1.offset(2isize);
                (*f).numpoints += 1
            }
            if !(sides[(i + 1i32) as usize] == 2i32 ||
                     sides[(i + 1i32) as usize] == sides[i as usize]) {
                p2 =
                    (*(*in_0).p.as_mut_ptr().offset(((i + 1i32) %
                                                         (*in_0).numpoints) as
                                                        isize)).as_mut_ptr();
                dot =
                    dists[i as usize] /
                        (dists[i as usize] - dists[(i + 1i32) as usize]);
                j = 0i32;
                while j < 3i32 {
                    if *normal.offset(j as isize) == 1i32 as libc::c_float {
                        mid[j as usize] = dist
                    } else if *normal.offset(j as isize) ==
                                  -1i32 as libc::c_float {
                        mid[j as usize] = -dist
                    } else {
                        mid[j as usize] =
                            *p1.offset(j as isize) +
                                dot *
                                    (*p2.offset(j as isize) -
                                         *p1.offset(j as isize))
                    }
                    j += 1
                }
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[0usize]
                    = mid[0usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[1usize]
                    = mid[1usize];
                (*(*f).p.as_mut_ptr().offset((*f).numpoints as isize))[2usize]
                    = mid[2usize];
                (*f).numpoints += 1
            }
        }
        i += 1
    }
    if (*f).numpoints > maxpts {
        Com_Error(ERR_DROP as libc::c_int,
                  b"ClipWinding: points exceeded estimate\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*f).numpoints > 64i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"ClipWinding: MAX_POINTS_ON_WINDING\x00" as *const u8 as
                      *const libc::c_char);
    }
    FreeWinding(in_0);
    *inout = f;
}
// frees the original if clipped
#[no_mangle]
pub unsafe extern "C" fn pw(mut w: *mut winding_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*w).numpoints {
        printf(b"(%5.1f, %5.1f, %5.1f)\n\x00" as *const u8 as
                   *const libc::c_char,
               (*(*w).p.as_mut_ptr().offset(i as isize))[0usize] as
                   libc::c_double,
               (*(*w).p.as_mut_ptr().offset(i as isize))[1usize] as
                   libc::c_double,
               (*(*w).p.as_mut_ptr().offset(i as isize))[2usize] as
                   libc::c_double);
        i += 1
    };
}