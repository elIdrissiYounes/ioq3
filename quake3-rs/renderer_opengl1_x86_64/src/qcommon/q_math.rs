use ::libc;

pub mod q_shared_h {

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

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0) * *v.offset(0)
                + *v.offset(1) * *v.offset(1)
                + *v.offset(2) * *v.offset(2))
                as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0) = *v1.offset(1)
            * *v2.offset(2)
            - *v1.offset(2) * *v2.offset(1);
        *cross.offset(1) = *v1.offset(2)
            * *v2.offset(0)
            - *v1.offset(0) * *v2.offset(2);
        *cross.offset(2) = *v1.offset(0)
            * *v2.offset(1)
            - *v1.offset(1) * *v2.offset(0);
    }
    use crate::stdlib::sqrt;
    // __Q_SHARED_H
}

pub use crate::src::qcommon::q_math::q_shared_h::CrossProduct;
pub use crate::src::qcommon::q_math::q_shared_h::VectorLength;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::floatint_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
use crate::stdlib::acos;
use crate::stdlib::atan2;
use crate::stdlib::cos;
use crate::stdlib::fabs;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::sin;
use crate::stdlib::sqrt;
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
// q_math.c -- stateless support routines that are included in each code module
// Some of the vector functions are static inline in q_shared.h. q3asm
// doesn't understand static functions though, so we only want them in
// one file. That's what this is about.
#[no_mangle]

pub static mut vec3_origin: crate::src::qcommon::q_shared::vec3_t = [
    0f32,
    0f32,
    0f32,
];
#[no_mangle]

pub static mut axisDefault: [crate::src::qcommon::q_shared::vec3_t; 3] = [
    [
        1f32,
        0f32,
        0f32,
    ],
    [
        0f32,
        1f32,
        0f32,
    ],
    [
        0f32,
        0f32,
        1f32,
    ],
];
#[no_mangle]

pub static mut colorBlack: crate::src::qcommon::q_shared::vec4_t = [
    0f32,
    0f32,
    0f32,
    1f32,
];
#[no_mangle]

pub static mut colorRed: crate::src::qcommon::q_shared::vec4_t = [
    1f32,
    0f32,
    0f32,
    1f32,
];
#[no_mangle]

pub static mut colorGreen: crate::src::qcommon::q_shared::vec4_t = [
    0f32,
    1f32,
    0f32,
    1f32,
];
#[no_mangle]

pub static mut colorBlue: crate::src::qcommon::q_shared::vec4_t = [
    0f32,
    0f32,
    1f32,
    1f32,
];
#[no_mangle]

pub static mut colorYellow: crate::src::qcommon::q_shared::vec4_t = [
    1f32,
    1f32,
    0f32,
    1f32,
];
#[no_mangle]

pub static mut colorMagenta: crate::src::qcommon::q_shared::vec4_t = [
    1f32,
    0f32,
    1f32,
    1f32,
];
#[no_mangle]

pub static mut colorCyan: crate::src::qcommon::q_shared::vec4_t = [
    0f32,
    1f32,
    1f32,
    1f32,
];
#[no_mangle]

pub static mut colorWhite: crate::src::qcommon::q_shared::vec4_t = [
    1f32,
    1f32,
    1f32,
    1f32,
];
#[no_mangle]

pub static mut colorLtGrey: crate::src::qcommon::q_shared::vec4_t = [
    0.75,
    0.75,
    0.75,
    1f32,
];
#[no_mangle]

pub static mut colorMdGrey: crate::src::qcommon::q_shared::vec4_t = [
    0.5,
    0.5,
    0.5,
    1f32,
];
#[no_mangle]

pub static mut colorDkGrey: crate::src::qcommon::q_shared::vec4_t = [
    0.25,
    0.25,
    0.25,
    1f32,
];
#[no_mangle]

pub static mut g_color_table: [crate::src::qcommon::q_shared::vec4_t; 8] = [
    [
        0f32,
        0f32,
        0f32,
        1f32,
    ],
    [
        1f32,
        0f32,
        0f32,
        1f32,
    ],
    [
        0f32,
        1f32,
        0f32,
        1f32,
    ],
    [
        1f32,
        1f32,
        0f32,
        1f32,
    ],
    [
        0f32,
        0f32,
        1f32,
        1f32,
    ],
    [
        0f32,
        1f32,
        1f32,
        1f32,
    ],
    [
        1f32,
        0f32,
        1f32,
        1f32,
    ],
    [
        1f32,
        1f32,
        1f32,
        1f32,
    ],
];
#[no_mangle]

pub static mut bytedirs: [crate::src::qcommon::q_shared::vec3_t; 162] = [
    [-0.525731, 0.000000, 0.850651],
    [-0.442863, 0.238856, 0.864188],
    [-0.295242, 0.000000, 0.955423],
    [-0.309017, 0.500000, 0.809017],
    [-0.162460, 0.262866, 0.951056],
    [0.000000, 0.000000, 1.000000],
    [0.000000, 0.850651, 0.525731],
    [-0.147621, 0.716567, 0.681718],
    [0.147621, 0.716567, 0.681718],
    [0.000000, 0.525731, 0.850651],
    [0.309017, 0.500000, 0.809017],
    [0.525731, 0.000000, 0.850651],
    [0.295242, 0.000000, 0.955423],
    [0.442863, 0.238856, 0.864188],
    [0.162460, 0.262866, 0.951056],
    [-0.681718, 0.147621, 0.716567],
    [-0.809017, 0.309017, 0.500000],
    [-0.587785, 0.425325, 0.688191],
    [-0.850651, 0.525731, 0.000000],
    [-0.864188, 0.442863, 0.238856],
    [-0.716567, 0.681718, 0.147621],
    [-0.688191, 0.587785, 0.425325],
    [-0.500000, 0.809017, 0.309017],
    [-0.238856, 0.864188, 0.442863],
    [-0.425325, 0.688191, 0.587785],
    [-0.716567, 0.681718, -0.147621],
    [-0.500000, 0.809017, -0.309017],
    [-0.525731, 0.850651, 0.000000],
    [0.000000, 0.850651, -0.525731],
    [-0.238856, 0.864188, -0.442863],
    [0.000000, 0.955423, -0.295242],
    [-0.262866, 0.951056, -0.162460],
    [0.000000, 1.000000, 0.000000],
    [0.000000, 0.955423, 0.295242],
    [-0.262866, 0.951056, 0.162460],
    [0.238856, 0.864188, 0.442863],
    [0.262866, 0.951056, 0.162460],
    [0.500000, 0.809017, 0.309017],
    [0.238856, 0.864188, -0.442863],
    [0.262866, 0.951056, -0.162460],
    [0.500000, 0.809017, -0.309017],
    [0.850651, 0.525731, 0.000000],
    [0.716567, 0.681718, 0.147621],
    [0.716567, 0.681718, -0.147621],
    [0.525731, 0.850651, 0.000000],
    [0.425325, 0.688191, 0.587785],
    [0.864188, 0.442863, 0.238856],
    [0.688191, 0.587785, 0.425325],
    [0.809017, 0.309017, 0.500000],
    [0.681718, 0.147621, 0.716567],
    [0.587785, 0.425325, 0.688191],
    [0.955423, 0.295242, 0.000000],
    [1.000000, 0.000000, 0.000000],
    [0.951056, 0.162460, 0.262866],
    [0.850651, -0.525731, 0.000000],
    [0.955423, -0.295242, 0.000000],
    [0.864188, -0.442863, 0.238856],
    [0.951056, -0.162460, 0.262866],
    [0.809017, -0.309017, 0.500000],
    [0.681718, -0.147621, 0.716567],
    [0.850651, 0.000000, 0.525731],
    [0.864188, 0.442863, -0.238856],
    [0.809017, 0.309017, -0.500000],
    [0.951056, 0.162460, -0.262866],
    [0.525731, 0.000000, -0.850651],
    [0.681718, 0.147621, -0.716567],
    [0.681718, -0.147621, -0.716567],
    [0.850651, 0.000000, -0.525731],
    [0.809017, -0.309017, -0.500000],
    [0.864188, -0.442863, -0.238856],
    [0.951056, -0.162460, -0.262866],
    [0.147621, 0.716567, -0.681718],
    [0.309017, 0.500000, -0.809017],
    [0.425325, 0.688191, -0.587785],
    [0.442863, 0.238856, -0.864188],
    [0.587785, 0.425325, -0.688191],
    [0.688191, 0.587785, -0.425325],
    [-0.147621, 0.716567, -0.681718],
    [-0.309017, 0.500000, -0.809017],
    [0.000000, 0.525731, -0.850651],
    [-0.525731, 0.000000, -0.850651],
    [-0.442863, 0.238856, -0.864188],
    [-0.295242, 0.000000, -0.955423],
    [-0.162460, 0.262866, -0.951056],
    [0.000000, 0.000000, -1.000000],
    [0.295242, 0.000000, -0.955423],
    [0.162460, 0.262866, -0.951056],
    [-0.442863, -0.238856, -0.864188],
    [-0.309017, -0.500000, -0.809017],
    [-0.162460, -0.262866, -0.951056],
    [0.000000, -0.850651, -0.525731],
    [-0.147621, -0.716567, -0.681718],
    [0.147621, -0.716567, -0.681718],
    [0.000000, -0.525731, -0.850651],
    [0.309017, -0.500000, -0.809017],
    [0.442863, -0.238856, -0.864188],
    [0.162460, -0.262866, -0.951056],
    [0.238856, -0.864188, -0.442863],
    [0.500000, -0.809017, -0.309017],
    [0.425325, -0.688191, -0.587785],
    [0.716567, -0.681718, -0.147621],
    [0.688191, -0.587785, -0.425325],
    [0.587785, -0.425325, -0.688191],
    [0.000000, -0.955423, -0.295242],
    [0.000000, -1.000000, 0.000000],
    [0.262866, -0.951056, -0.162460],
    [0.000000, -0.850651, 0.525731],
    [0.000000, -0.955423, 0.295242],
    [0.238856, -0.864188, 0.442863],
    [0.262866, -0.951056, 0.162460],
    [0.500000, -0.809017, 0.309017],
    [0.716567, -0.681718, 0.147621],
    [0.525731, -0.850651, 0.000000],
    [-0.238856, -0.864188, -0.442863],
    [-0.500000, -0.809017, -0.309017],
    [-0.262866, -0.951056, -0.162460],
    [-0.850651, -0.525731, 0.000000],
    [-0.716567, -0.681718, -0.147621],
    [-0.716567, -0.681718, 0.147621],
    [-0.525731, -0.850651, 0.000000],
    [-0.500000, -0.809017, 0.309017],
    [-0.238856, -0.864188, 0.442863],
    [-0.262866, -0.951056, 0.162460],
    [-0.864188, -0.442863, 0.238856],
    [-0.809017, -0.309017, 0.500000],
    [-0.688191, -0.587785, 0.425325],
    [-0.681718, -0.147621, 0.716567],
    [-0.442863, -0.238856, 0.864188],
    [-0.587785, -0.425325, 0.688191],
    [-0.309017, -0.500000, 0.809017],
    [-0.147621, -0.716567, 0.681718],
    [-0.425325, -0.688191, 0.587785],
    [-0.162460, -0.262866, 0.951056],
    [0.442863, -0.238856, 0.864188],
    [0.162460, -0.262866, 0.951056],
    [0.309017, -0.500000, 0.809017],
    [0.147621, -0.716567, 0.681718],
    [0.000000, -0.525731, 0.850651],
    [0.425325, -0.688191, 0.587785],
    [0.587785, -0.425325, 0.688191],
    [0.688191, -0.587785, 0.425325],
    [-0.955423, 0.295242, 0.000000],
    [-0.951056, 0.162460, 0.262866],
    [-1.000000, 0.000000, 0.000000],
    [-0.850651, 0.000000, 0.525731],
    [-0.955423, -0.295242, 0.000000],
    [-0.951056, -0.162460, 0.262866],
    [-0.864188, 0.442863, -0.238856],
    [-0.951056, 0.162460, -0.262866],
    [-0.809017, 0.309017, -0.500000],
    [-0.864188, -0.442863, -0.238856],
    [-0.951056, -0.162460, -0.262866],
    [-0.809017, -0.309017, -0.500000],
    [-0.681718, 0.147621, -0.716567],
    [-0.681718, -0.147621, -0.716567],
    [-0.850651, 0.000000, -0.525731],
    [-0.688191, 0.587785, -0.425325],
    [-0.587785, 0.425325, -0.688191],
    [-0.425325, 0.688191, -0.587785],
    [-0.425325, -0.688191, -0.587785],
    [-0.587785, -0.425325, -0.688191],
    [-0.688191, -0.587785, -0.425325],
];
//==============================================================
#[no_mangle]

pub unsafe extern "C" fn Q_rand(mut seed: *mut i32) -> i32 {
    *seed = (69069u32)
        .wrapping_mul(*seed as u32)
        .wrapping_add(1u32) as i32;
    return *seed;
}
#[no_mangle]

pub unsafe extern "C" fn Q_random(mut seed: *mut i32) -> f32 {
    return (Q_rand(seed) & 0xffff) as f32
        / 65536f32;
}
#[no_mangle]

pub unsafe extern "C" fn Q_crandom(mut seed: *mut i32) -> f32 {
    return (2.0 * (Q_random(seed) as f64 - 0.5)) as f32;
}
//=======================================================
#[no_mangle]

pub unsafe extern "C" fn ClampChar(mut i: i32) -> i8 {
    if i < -(128) {
        return -(128i32) as i8;
    }
    if i > 127 {
        return 127i8;
    }
    return i as i8;
}
#[no_mangle]

pub unsafe extern "C" fn ClampShort(mut i: i32) -> i16 {
    if i < -(32768) {
        return -(32768i32) as i16;
    }
    if i > 0x7fff {
        return 0x7fffi16;
    }
    return i as i16;
}
// this isn't a real cheap function to call!
#[no_mangle]

pub unsafe extern "C" fn DirToByte(
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut best: i32 = 0;
    let mut d: f32 = 0.;
    let mut bestd: f32 = 0.;
    if dir.is_null() {
        return 0i32;
    }
    bestd = 0f32;
    best = 0;
    i = 0;
    while i < 162 {
        d = *dir.offset(0)
            * bytedirs[i as usize][0]
            + *dir.offset(1)
                * bytedirs[i as usize][1]
            + *dir.offset(2)
                * bytedirs[i as usize][2];
        if d > bestd {
            bestd = d;
            best = i
        }
        i += 1
    }
    return best;
}
#[no_mangle]

pub unsafe extern "C" fn ByteToDir(
    mut b: i32,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    if b < 0 || b >= 162 {
        *dir.offset(0) = vec3_origin[0];
        *dir.offset(1) = vec3_origin[1];
        *dir.offset(2) = vec3_origin[2];
        return;
    }
    *dir.offset(0) = bytedirs[b as usize][0];
    *dir.offset(1) = bytedirs[b as usize][1];
    *dir.offset(2) = bytedirs[b as usize][2];
}
#[no_mangle]

pub unsafe extern "C" fn ColorBytes3(
    mut r: f32,
    mut g: f32,
    mut b: f32,
) -> u32 {
    let mut i: u32 = 0;
    *(&mut i as *mut u32 as *mut crate::src::qcommon::q_shared::byte)
        .offset(0) =
        (r * 255f32) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut u32 as *mut crate::src::qcommon::q_shared::byte)
        .offset(1) =
        (g * 255f32) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut u32 as *mut crate::src::qcommon::q_shared::byte)
        .offset(2) =
        (b * 255f32) as crate::src::qcommon::q_shared::byte;
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn ColorBytes4(
    mut r: f32,
    mut g: f32,
    mut b: f32,
    mut a: f32,
) -> u32 {
    let mut i: u32 = 0;
    *(&mut i as *mut u32 as *mut crate::src::qcommon::q_shared::byte)
        .offset(0) =
        (r * 255f32) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut u32 as *mut crate::src::qcommon::q_shared::byte)
        .offset(1) =
        (g * 255f32) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut u32 as *mut crate::src::qcommon::q_shared::byte)
        .offset(2) =
        (b * 255f32) as crate::src::qcommon::q_shared::byte;
    *(&mut i as *mut u32 as *mut crate::src::qcommon::q_shared::byte)
        .offset(3) =
        (a * 255f32) as crate::src::qcommon::q_shared::byte;
    return i;
}
#[no_mangle]

pub unsafe extern "C" fn NormalizeColor(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    let mut max: f32 = 0.;
    max = *in_0.offset(0);
    if *in_0.offset(1) > max {
        max = *in_0.offset(1)
    }
    if *in_0.offset(2) > max {
        max = *in_0.offset(2)
    }
    if max == 0. {
        let ref mut fresh0 = *out.offset(2);
        *fresh0 = 0f32;
        let ref mut fresh1 = *out.offset(1);
        *fresh1 = *fresh0;
        *out.offset(0) = *fresh1
    } else {
        *out.offset(0) = *in_0.offset(0) / max;
        *out.offset(1) = *in_0.offset(1) / max;
        *out.offset(2) = *in_0.offset(2) / max
    }
    return max;
}
/*
=====================
PlaneFromPoints

Returns false if the triangle is degenrate.
The normal will point out of the clock for clockwise ordered points
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn PlaneFromPoints(
    mut plane: *mut crate::src::qcommon::q_shared::vec_t,
    mut a: *const crate::src::qcommon::q_shared::vec_t,
    mut b: *const crate::src::qcommon::q_shared::vec_t,
    mut c: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut d1: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut d2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    d1[0] =
        *b.offset(0) - *a.offset(0);
    d1[1] =
        *b.offset(1) - *a.offset(1);
    d1[2] =
        *b.offset(2) - *a.offset(2);
    d2[0] =
        *c.offset(0) - *a.offset(0);
    d2[1] =
        *c.offset(1) - *a.offset(1);
    d2[2] =
        *c.offset(2) - *a.offset(2);
    CrossProduct(
        d2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        d1.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        plane,
    );
    if VectorNormalize(plane) == 0f32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    *plane.offset(3) = *a.offset(0)
        * *plane.offset(0)
        + *a.offset(1) * *plane.offset(1)
        + *a.offset(2) * *plane.offset(2);
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
RotatePointAroundVector

This is not implemented very well...
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RotatePointAroundVector(
    mut dst: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *const crate::src::qcommon::q_shared::vec_t,
    mut point: *const crate::src::qcommon::q_shared::vec_t,
    mut degrees: f32,
) {
    let mut m: [[f32; 3]; 3] = [[0.; 3]; 3];
    let mut im: [[f32; 3]; 3] = [[0.; 3]; 3];
    let mut zrot: [[f32; 3]; 3] = [[0.; 3]; 3];
    let mut tmpmat: [[f32; 3]; 3] = [[0.; 3]; 3];
    let mut rot: [[f32; 3]; 3] = [[0.; 3]; 3];
    let mut i: i32 = 0;
    let mut vr: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vup: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vf: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut rad: f32 = 0.;
    vf[0] = *dir.offset(0);
    vf[1] = *dir.offset(1);
    vf[2] = *dir.offset(2);
    PerpendicularVector(vr.as_mut_ptr(), dir);
    CrossProduct(
        vr.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vf.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vup.as_mut_ptr(),
    );
    m[0][0] = vr[0];
    m[1][0] = vr[1];
    m[2][0] = vr[2];
    m[0][1] = vup[0];
    m[1][1] = vup[1];
    m[2][1] = vup[2];
    m[0][2] = vf[0];
    m[1][2] = vf[1];
    m[2][2] = vf[2];
    crate::stdlib::memcpy(
        im.as_mut_ptr() as *mut libc::c_void,
        m.as_mut_ptr() as *const libc::c_void,
        
        ::std::mem::size_of::<[[f32; 3]; 3]>(),
    );
    im[0][1] =
        m[1][0];
    im[0][2] =
        m[2][0];
    im[1][0] =
        m[0][1];
    im[1][2] =
        m[2][1];
    im[2][0] =
        m[0][2];
    im[2][1] =
        m[1][2];
    crate::stdlib::memset(
        zrot.as_mut_ptr() as *mut libc::c_void,
        0,
        
        ::std::mem::size_of::<[[f32; 3]; 3]>(),
    );
    zrot[2][2] = 1.0;
    zrot[1][1] =
        zrot[2][2];
    zrot[0][0] =
        zrot[1][1];
    rad = (degrees as f64 * 3.14159265358979323846 / 180f64)
        as f32;
    zrot[0][0] =
        crate::stdlib::cos(rad as f64) as f32;
    zrot[0][1] =
        crate::stdlib::sin(rad as f64) as f32;
    zrot[1][0] =
        -crate::stdlib::sin(rad as f64) as f32;
    zrot[1][1] =
        crate::stdlib::cos(rad as f64) as f32;
    MatrixMultiply(m.as_mut_ptr(), zrot.as_mut_ptr(), tmpmat.as_mut_ptr());
    MatrixMultiply(tmpmat.as_mut_ptr(), im.as_mut_ptr(), rot.as_mut_ptr());
    i = 0;
    while i < 3 {
        *dst.offset(i as isize) = rot[i as usize][0]
            * *point.offset(0)
            + rot[i as usize][1] * *point.offset(1)
            + rot[i as usize][2] * *point.offset(2);
        i += 1
    }
}
/*
===============
RotateAroundDirection
===============
*/
#[no_mangle]

pub unsafe extern "C" fn RotateAroundDirection(
    mut axis: *mut crate::src::qcommon::q_shared::vec3_t,
    mut yaw: f32,
) {
    // create an arbitrary axis[1]
    PerpendicularVector(
        (*axis.offset(1)).as_mut_ptr(),
        (*axis.offset(0)).as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
    );
    // rotate it around axis[0] by yaw
    if yaw != 0. {
        let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        temp[0] =
            (*axis.offset(1))[0];
        temp[1] =
            (*axis.offset(1))[1];
        temp[2] =
            (*axis.offset(1))[2];
        RotatePointAroundVector(
            (*axis.offset(1isize)).as_mut_ptr(),
            (*axis.offset(0isize)).as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            yaw,
        );
    }
    // cross to get axis[2]
    CrossProduct(
        (*axis.offset(0)).as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*axis.offset(1)).as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        (*axis.offset(2)).as_mut_ptr(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn vectoangles(
    mut value1: *const crate::src::qcommon::q_shared::vec_t,
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut forward: f32 = 0.;
    let mut yaw: f32 = 0.;
    let mut pitch: f32 = 0.;
    if *value1.offset(1) == 0f32
        && *value1.offset(0) == 0f32
    {
        yaw = 0f32;
        if *value1.offset(2) > 0f32 {
            pitch = 90f32
        } else {
            pitch = 270f32
        }
    } else {
        if *value1.offset(0) != 0. {
            yaw = (crate::stdlib::atan2(
                *value1.offset(1) as f64,
                *value1.offset(0) as f64,
            ) * 180f64
                / 3.14159265358979323846) as f32
        } else if *value1.offset(1) > 0f32 {
            yaw = 90f32
        } else {
            yaw = 270f32
        }
        if yaw < 0f32 {
            yaw += 360f32
        }
        forward = crate::stdlib::sqrt(
            (*value1.offset(0) * *value1.offset(0)
                + *value1.offset(1)
                    * *value1.offset(1)) as f64,
        ) as f32;
        pitch = (crate::stdlib::atan2(
            *value1.offset(2) as f64,
            forward as f64,
        ) * 180f64
            / 3.14159265358979323846) as f32;
        if pitch < 0f32 {
            pitch += 360f32
        }
    }
    *angles.offset(0) = -pitch;
    *angles.offset(1) = yaw;
    *angles.offset(2) =
        0f32;
}
/*
=================
AnglesToAxis
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AnglesToAxis(
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
    mut axis: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // angle vectors returns "right" instead of "y axis"
    AngleVectors(
        angles,
        (*axis.offset(0)).as_mut_ptr(),
        right.as_mut_ptr(),
        (*axis.offset(2)).as_mut_ptr(),
    );
    (*axis.offset(1))[0] =
        vec3_origin[0] - right[0];
    (*axis.offset(1))[1] =
        vec3_origin[1] - right[1];
    (*axis.offset(1))[2] =
        vec3_origin[2] - right[2];
}
#[no_mangle]

pub unsafe extern "C" fn AxisClear(mut axis: *mut crate::src::qcommon::q_shared::vec3_t) {
    (*axis.offset(0))[0] =
        1f32;
    (*axis.offset(0))[1] =
        0f32;
    (*axis.offset(0))[2] =
        0f32;
    (*axis.offset(1))[0] =
        0f32;
    (*axis.offset(1))[1] =
        1f32;
    (*axis.offset(1))[2] =
        0f32;
    (*axis.offset(2))[0] =
        0f32;
    (*axis.offset(2))[1] =
        0f32;
    (*axis.offset(2))[2] =
        1f32;
}
#[no_mangle]

pub unsafe extern "C" fn AxisCopy(
    mut in_0: *mut crate::src::qcommon::q_shared::vec3_t,
    mut out: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    (*out.offset(0))[0] =
        (*in_0.offset(0))[0];
    (*out.offset(0))[1] =
        (*in_0.offset(0))[1];
    (*out.offset(0))[2] =
        (*in_0.offset(0))[2];
    (*out.offset(1))[0] =
        (*in_0.offset(1))[0];
    (*out.offset(1))[1] =
        (*in_0.offset(1))[1];
    (*out.offset(1))[2] =
        (*in_0.offset(1))[2];
    (*out.offset(2))[0] =
        (*in_0.offset(2))[0];
    (*out.offset(2))[1] =
        (*in_0.offset(2))[1];
    (*out.offset(2))[2] =
        (*in_0.offset(2))[2];
}
#[no_mangle]

pub unsafe extern "C" fn ProjectPointOnPlane(
    mut dst: *mut crate::src::qcommon::q_shared::vec_t,
    mut p: *const crate::src::qcommon::q_shared::vec_t,
    mut normal: *const crate::src::qcommon::q_shared::vec_t,
) {
    let mut d: f32 = 0.;
    let mut n: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut inv_denom: f32 = 0.;
    inv_denom = *normal.offset(0)
        * *normal.offset(0)
        + *normal.offset(1) * *normal.offset(1)
        + *normal.offset(2) * *normal.offset(2);
    // zero vectors get here
    inv_denom = 1.0 / inv_denom;
    d = (*normal.offset(0) * *p.offset(0)
        + *normal.offset(1) * *p.offset(1)
        + *normal.offset(2) * *p.offset(2))
        * inv_denom;
    n[0] = *normal.offset(0) * inv_denom;
    n[1] = *normal.offset(1) * inv_denom;
    n[2] = *normal.offset(2) * inv_denom;
    *dst.offset(0) =
        *p.offset(0) - d * n[0];
    *dst.offset(1) =
        *p.offset(1) - d * n[1];
    *dst.offset(2) =
        *p.offset(2) - d * n[2];
}
/*
================
MakeNormalVectors

Given a normalized forward vector, create two
other perpendicular vectors
================
*/
#[no_mangle]

pub unsafe extern "C" fn MakeNormalVectors(
    mut forward: *const crate::src::qcommon::q_shared::vec_t,
    mut right: *mut crate::src::qcommon::q_shared::vec_t,
    mut up: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut d: f32 = 0.;
    // this rotate and negate guarantees a vector
    // not colinear with the original
    *right.offset(1) = -*forward.offset(0);
    *right.offset(2) = *forward.offset(1);
    *right.offset(0) = *forward.offset(2);
    d = *right.offset(0) * *forward.offset(0)
        + *right.offset(1) * *forward.offset(1)
        + *right.offset(2) * *forward.offset(2);
    *right.offset(0) =
        *right.offset(0) + *forward.offset(0) * -d;
    *right.offset(1) =
        *right.offset(1) + *forward.offset(1) * -d;
    *right.offset(2) =
        *right.offset(2) + *forward.offset(2) * -d;
    VectorNormalize(right);
    CrossProduct(
        right as *const crate::src::qcommon::q_shared::vec_t,
        forward,
        up,
    );
}
#[no_mangle]

pub unsafe extern "C" fn VectorRotate(
    mut in_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0) = *in_0.offset(0)
        * (*matrix.offset(0))[0]
        + *in_0.offset(1)
            * (*matrix.offset(0))[1]
        + *in_0.offset(2)
            * (*matrix.offset(0))[2];
    *out.offset(1) = *in_0.offset(0)
        * (*matrix.offset(1))[0]
        + *in_0.offset(1)
            * (*matrix.offset(1))[1]
        + *in_0.offset(2)
            * (*matrix.offset(1))[2];
    *out.offset(2) = *in_0.offset(0)
        * (*matrix.offset(2))[0]
        + *in_0.offset(1)
            * (*matrix.offset(2))[1]
        + *in_0.offset(2)
            * (*matrix.offset(2))[2];
}
//============================================================================
/*
** float q_rsqrt( float number )
*/
#[no_mangle]

pub unsafe extern "C" fn Q_rsqrt(mut number: f32) -> f32 {
    let mut t: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. }; // what the fuck?
    let mut x2: f32 = 0.; // 1st iteration
    let mut y: f32 = 0.;
    let threehalfs: f32 = 1.5;
    x2 = number * 0.5;
    t.f = number;
    t.i = 0x5f3759df - (t.i >> 1);
    y = t.f;
    y = y * (threehalfs - x2 * y * y);
    //	y  = y * ( threehalfs - ( x2 * y * y ) );   // 2nd iteration, this can be removed
    return y;
}
#[no_mangle]

pub unsafe extern "C" fn Q_fabs(mut f: f32) -> f32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = f;
    fi.i &= 0x7fffffff;
    return fi.f;
}
//============================================================
/*
===============
LerpAngle

===============
*/
#[no_mangle]

pub unsafe extern "C" fn LerpAngle(
    mut from: f32,
    mut to: f32,
    mut frac: f32,
) -> f32 {
    let mut a: f32 = 0.;
    if to - from > 180f32 {
        to -= 360f32
    }
    if to - from < -180f32 {
        to += 360f32
    }
    a = from + frac * (to - from);
    return a;
}
/*
=================
AngleSubtract

Always returns a value from -180 to 180
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleSubtract(
    mut a1: f32,
    mut a2: f32,
) -> f32 {
    let mut a: f32 = 0.;
    a = a1 - a2;
    while a > 180f32 {
        a -= 360f32
    }
    while a < -180f32 {
        a += 360f32
    }
    return a;
}
#[no_mangle]

pub unsafe extern "C" fn AnglesSubtract(
    mut v1: *mut crate::src::qcommon::q_shared::vec_t,
    mut v2: *mut crate::src::qcommon::q_shared::vec_t,
    mut v3: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *v3.offset(0) = AngleSubtract(
        *v1.offset(0),
        *v2.offset(0),
    );
    *v3.offset(1) = AngleSubtract(
        *v1.offset(1),
        *v2.offset(1),
    );
    *v3.offset(2) = AngleSubtract(
        *v1.offset(2),
        *v2.offset(2),
    );
}
#[no_mangle]

pub unsafe extern "C" fn AngleMod(mut a: f32) -> f32 {
    a = (360.0 / 65536f64
        * ((a as f64 * (65536f64 / 360.0))
            as i32
            & 65535) as f64) as f32;
    return a;
}
/*
=================
AngleNormalize360

returns angle normalized to the range [0 <= angle < 360]
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleNormalize360(mut angle: f32) -> f32 {
    return (360.0 / 65536f64
        * ((angle as f64 * (65536f64 / 360.0))
            as i32
            & 65535) as f64) as f32;
}
/*
=================
AngleNormalize180

returns angle normalized to the range [-180 < angle <= 180]
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleNormalize180(mut angle: f32) -> f32 {
    angle = AngleNormalize360(angle);
    if angle as f64 > 180.0 {
        angle = (angle as f64 - 360.0) as f32
    }
    return angle;
}
/*
=================
AngleDelta

returns the normalized delta from angle1 to angle2
=================
*/
#[no_mangle]

pub unsafe extern "C" fn AngleDelta(
    mut angle1: f32,
    mut angle2: f32,
) -> f32 {
    return AngleNormalize180(angle1 - angle2);
}
//============================================================
/*
=================
SetPlaneSignbits
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SetPlaneSignbits(mut out: *mut crate::src::qcommon::q_shared::cplane_t) {
    let mut bits: i32 = 0;
    let mut j: i32 = 0;
    // for fast box on planeside test
    bits = 0;
    j = 0;
    while j < 3 {
        if (*out).normal[j as usize] < 0f32 {
            bits |= (1) << j
        }
        j += 1
    }
    (*out).signbits = bits as crate::src::qcommon::q_shared::byte;
}
/*
==================
BoxOnPlaneSide

Returns 1, 2, or 1 + 2
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BoxOnPlaneSide(
    mut emins: *mut crate::src::qcommon::q_shared::vec_t,
    mut emaxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut p: *mut crate::src::qcommon::q_shared::cplane_s,
) -> i32 {
    let mut dist: [f32; 2] = [0.; 2];
    let mut sides: i32 = 0;
    let mut b: i32 = 0;
    let mut i: i32 = 0;
    // fast axial cases
    if ((*p).type_0 as i32) < 3 {
        if (*p).dist <= *emins.offset((*p).type_0 as isize) {
            return 1i32;
        }
        if (*p).dist >= *emaxs.offset((*p).type_0 as isize) {
            return 2i32;
        }
        return 3i32;
    }
    // general case
    dist[1] = 0f32;
    dist[0] = dist[1];
    if ((*p).signbits as i32) < 8 {
        // >= 8: default case is original code (dist[0]=dist[1]=0)
        i = 0;
        while i < 3 {
            b = (*p).signbits as i32 >> i & 1;
            dist[b as usize] += (*p).normal[i as usize] * *emaxs.offset(i as isize);
            dist[(b == 0) as i32 as usize] +=
                (*p).normal[i as usize] * *emins.offset(i as isize);
            i += 1
        }
    }
    sides = 0;
    if dist[0] >= (*p).dist {
        sides = 1
    }
    if dist[1] < (*p).dist {
        sides |= 2
    }
    return sides;
}
/*
=================
RadiusFromBounds
=================
*/
#[no_mangle]

pub unsafe extern "C" fn RadiusFromBounds(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
) -> f32 {
    let mut i: i32 = 0;
    let mut corner: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut a: f32 = 0.;
    let mut b: f32 = 0.;
    i = 0;
    while i < 3 {
        a = crate::stdlib::fabs(*mins.offset(i as isize) as f64) as f32;
        b = crate::stdlib::fabs(*maxs.offset(i as isize) as f64) as f32;
        corner[i as usize] = if a > b { a } else { b };
        i += 1
    }
    return VectorLength(corner.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
}
#[no_mangle]

pub unsafe extern "C" fn ClearBounds(
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let ref mut fresh2 = *mins.offset(2);
    *fresh2 = 99999f32;
    let ref mut fresh3 = *mins.offset(1);
    *fresh3 = *fresh2;
    *mins.offset(0) = *fresh3;
    let ref mut fresh4 = *maxs.offset(2);
    *fresh4 = -99999f32;
    let ref mut fresh5 = *maxs.offset(1);
    *fresh5 = *fresh4;
    *maxs.offset(0) = *fresh5;
}
#[no_mangle]

pub unsafe extern "C" fn AddPointToBounds(
    mut v: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) {
    if *v.offset(0) < *mins.offset(0) {
        *mins.offset(0) = *v.offset(0)
    }
    if *v.offset(0) > *maxs.offset(0) {
        *maxs.offset(0) = *v.offset(0)
    }
    if *v.offset(1) < *mins.offset(1) {
        *mins.offset(1) = *v.offset(1)
    }
    if *v.offset(1) > *maxs.offset(1) {
        *maxs.offset(1) = *v.offset(1)
    }
    if *v.offset(2) < *mins.offset(2) {
        *mins.offset(2) = *v.offset(2)
    }
    if *v.offset(2) > *maxs.offset(2) {
        *maxs.offset(2) = *v.offset(2)
    };
}
#[no_mangle]

pub unsafe extern "C" fn BoundsIntersect(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut mins2: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if *maxs.offset(0) < *mins2.offset(0)
        || *maxs.offset(1) < *mins2.offset(1)
        || *maxs.offset(2) < *mins2.offset(2)
        || *mins.offset(0) > *maxs2.offset(0)
        || *mins.offset(1) > *maxs2.offset(1)
        || *mins.offset(2) > *maxs2.offset(2)
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn BoundsIntersectSphere(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
    mut radius: crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if *origin.offset(0) - radius > *maxs.offset(0)
        || *origin.offset(0) + radius
            < *mins.offset(0)
        || *origin.offset(1) - radius
            > *maxs.offset(1)
        || *origin.offset(1) + radius
            < *mins.offset(1)
        || *origin.offset(2) - radius
            > *maxs.offset(2)
        || *origin.offset(2) + radius
            < *mins.offset(2)
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn BoundsIntersectPoint(
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut origin: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if *origin.offset(0) > *maxs.offset(0)
        || *origin.offset(0) < *mins.offset(0)
        || *origin.offset(1) > *maxs.offset(1)
        || *origin.offset(1) < *mins.offset(1)
        || *origin.offset(2) > *maxs.offset(2)
        || *origin.offset(2) < *mins.offset(2)
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn VectorNormalize(
    mut v: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::vec_t {
    // NOTE: TTimo - Apple G4 altivec source uses double?
    let mut length: f32 = 0.;
    let mut ilength: f32 = 0.;
    length = *v.offset(0) * *v.offset(0)
        + *v.offset(1) * *v.offset(1)
        + *v.offset(2) * *v.offset(2);
    if length != 0. {
        /* writing it this way allows gcc to recognize that rsqrt can be used */
        ilength = 1f32
            / crate::stdlib::sqrt(length as f64) as f32;
        /* sqrt(length) = length * (1 / sqrt(length)) */
        length *= ilength;
        let ref mut fresh6 = *v.offset(0);
        *fresh6 *= ilength;
        let ref mut fresh7 = *v.offset(1);
        *fresh7 *= ilength;
        let ref mut fresh8 = *v.offset(2);
        *fresh8 *= ilength
    }
    return length;
}
#[no_mangle]

pub unsafe extern "C" fn VectorNormalize2(
    mut v: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::vec_t {
    let mut length: f32 = 0.;
    let mut ilength: f32 = 0.;
    length = *v.offset(0) * *v.offset(0)
        + *v.offset(1) * *v.offset(1)
        + *v.offset(2) * *v.offset(2);
    if length != 0. {
        /* writing it this way allows gcc to recognize that rsqrt can be used */
        ilength = 1f32
            / crate::stdlib::sqrt(length as f64) as f32;
        /* sqrt(length) = length * (1 / sqrt(length)) */
        length *= ilength;
        *out.offset(0) = *v.offset(0) * ilength;
        *out.offset(1) = *v.offset(1) * ilength;
        *out.offset(2) = *v.offset(2) * ilength
    } else {
        let ref mut fresh9 = *out.offset(2);
        *fresh9 = 0f32;
        let ref mut fresh10 = *out.offset(1);
        *fresh10 = *fresh9;
        *out.offset(0) = *fresh10
    }
    return length;
}
#[no_mangle]

pub unsafe extern "C" fn _VectorMA(
    mut veca: *const crate::src::qcommon::q_shared::vec_t,
    mut scale: f32,
    mut vecb: *const crate::src::qcommon::q_shared::vec_t,
    mut vecc: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *vecc.offset(0) =
        *veca.offset(0) + scale * *vecb.offset(0);
    *vecc.offset(1) =
        *veca.offset(1) + scale * *vecb.offset(1);
    *vecc.offset(2) =
        *veca.offset(2) + scale * *vecb.offset(2);
}
#[no_mangle]

pub unsafe extern "C" fn _DotProduct(
    mut v1: *const crate::src::qcommon::q_shared::vec_t,
    mut v2: *const crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::vec_t {
    return *v1.offset(0) * *v2.offset(0)
        + *v1.offset(1) * *v2.offset(1)
        + *v1.offset(2) * *v2.offset(2);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorSubtract(
    mut veca: *const crate::src::qcommon::q_shared::vec_t,
    mut vecb: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0) =
        *veca.offset(0) - *vecb.offset(0);
    *out.offset(1) =
        *veca.offset(1) - *vecb.offset(1);
    *out.offset(2) =
        *veca.offset(2) - *vecb.offset(2);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorAdd(
    mut veca: *const crate::src::qcommon::q_shared::vec_t,
    mut vecb: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0) =
        *veca.offset(0) + *vecb.offset(0);
    *out.offset(1) =
        *veca.offset(1) + *vecb.offset(1);
    *out.offset(2) =
        *veca.offset(2) + *vecb.offset(2);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorCopy(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0) = *in_0.offset(0);
    *out.offset(1) = *in_0.offset(1);
    *out.offset(2) = *in_0.offset(2);
}
#[no_mangle]

pub unsafe extern "C" fn _VectorScale(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut scale: crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0) = *in_0.offset(0) * scale;
    *out.offset(1) = *in_0.offset(1) * scale;
    *out.offset(2) = *in_0.offset(2) * scale;
}
#[no_mangle]

pub unsafe extern "C" fn Vector4Scale(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut scale: crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    *out.offset(0) = *in_0.offset(0) * scale;
    *out.offset(1) = *in_0.offset(1) * scale;
    *out.offset(2) = *in_0.offset(2) * scale;
    *out.offset(3) = *in_0.offset(3) * scale;
}
#[no_mangle]

pub unsafe extern "C" fn Q_log2(mut val: i32) -> i32 {
    let mut answer: i32 = 0;
    answer = 0;
    loop {
        val >>= 1;
        if !(val != 0) {
            break;
        }
        answer += 1
    }
    return answer;
}
/*
=================
PlaneTypeForNormal
=================
*/
/*
int	PlaneTypeForNormal (vec3_t normal) {
    if ( normal[0] == 1.0 )
        return PLANE_X;
    if ( normal[1] == 1.0 )
        return PLANE_Y;
    if ( normal[2] == 1.0 )
        return PLANE_Z;

    return PLANE_NON_AXIAL;
}
*/
/*
================
MatrixMultiply
================
*/
#[no_mangle]

pub unsafe extern "C" fn MatrixMultiply(
    mut in1: *mut [f32; 3],
    mut in2: *mut [f32; 3],
    mut out: *mut [f32; 3],
) {
    (*out.offset(0))[0] = (*in1
        .offset(0))[0]
        * (*in2.offset(0))[0]
        + (*in1.offset(0))[1]
            * (*in2.offset(1))[0]
        + (*in1.offset(0))[2]
            * (*in2.offset(2))[0];
    (*out.offset(0))[1] = (*in1
        .offset(0))[0]
        * (*in2.offset(0))[1]
        + (*in1.offset(0))[1]
            * (*in2.offset(1))[1]
        + (*in1.offset(0))[2]
            * (*in2.offset(2))[1];
    (*out.offset(0))[2] = (*in1
        .offset(0))[0]
        * (*in2.offset(0))[2]
        + (*in1.offset(0))[1]
            * (*in2.offset(1))[2]
        + (*in1.offset(0))[2]
            * (*in2.offset(2))[2];
    (*out.offset(1))[0] = (*in1
        .offset(1))[0]
        * (*in2.offset(0))[0]
        + (*in1.offset(1))[1]
            * (*in2.offset(1))[0]
        + (*in1.offset(1))[2]
            * (*in2.offset(2))[0];
    (*out.offset(1))[1] = (*in1
        .offset(1))[0]
        * (*in2.offset(0))[1]
        + (*in1.offset(1))[1]
            * (*in2.offset(1))[1]
        + (*in1.offset(1))[2]
            * (*in2.offset(2))[1];
    (*out.offset(1))[2] = (*in1
        .offset(1))[0]
        * (*in2.offset(0))[2]
        + (*in1.offset(1))[1]
            * (*in2.offset(1))[2]
        + (*in1.offset(1))[2]
            * (*in2.offset(2))[2];
    (*out.offset(2))[0] = (*in1
        .offset(2))[0]
        * (*in2.offset(0))[0]
        + (*in1.offset(2))[1]
            * (*in2.offset(1))[0]
        + (*in1.offset(2))[2]
            * (*in2.offset(2))[0];
    (*out.offset(2))[1] = (*in1
        .offset(2))[0]
        * (*in2.offset(0))[1]
        + (*in1.offset(2))[1]
            * (*in2.offset(1))[1]
        + (*in1.offset(2))[2]
            * (*in2.offset(2))[1];
    (*out.offset(2))[2] = (*in1
        .offset(2))[0]
        * (*in2.offset(0))[2]
        + (*in1.offset(2))[1]
            * (*in2.offset(1))[2]
        + (*in1.offset(2))[2]
            * (*in2.offset(2))[2];
}
#[no_mangle]

pub unsafe extern "C" fn AngleVectors(
    mut angles: *const crate::src::qcommon::q_shared::vec_t,
    mut forward: *mut crate::src::qcommon::q_shared::vec_t,
    mut right: *mut crate::src::qcommon::q_shared::vec_t,
    mut up: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut angle: f32 = 0.;
    static mut sr: f32 = 0.;
    static mut sp: f32 = 0.;
    static mut sy: f32 = 0.;
    static mut cr: f32 = 0.;
    static mut cp: f32 = 0.;
    static mut cy: f32 = 0.;
    // static to help MS compiler fp bugs
    angle = (*angles.offset(1) as f64
        * (3.14159265358979323846 * 2f64
            / 360f64)) as f32;
    sy = crate::stdlib::sin(angle as f64) as f32;
    cy = crate::stdlib::cos(angle as f64) as f32;
    angle = (*angles.offset(0) as f64
        * (3.14159265358979323846 * 2f64
            / 360f64)) as f32;
    sp = crate::stdlib::sin(angle as f64) as f32;
    cp = crate::stdlib::cos(angle as f64) as f32;
    angle = (*angles.offset(2) as f64
        * (3.14159265358979323846 * 2f64
            / 360f64)) as f32;
    sr = crate::stdlib::sin(angle as f64) as f32;
    cr = crate::stdlib::cos(angle as f64) as f32;
    if !forward.is_null() {
        *forward.offset(0) = cp * cy;
        *forward.offset(1) = cp * sy;
        *forward.offset(2) = -sp
    }
    if !right.is_null() {
        *right.offset(0) =
            -1f32 * sr * sp * cy
                + -1f32 * cr * -sy;
        *right.offset(1) =
            -1f32 * sr * sp * sy
                + -1f32 * cr * cy;
        *right.offset(2) = -1f32 * sr * cp
    }
    if !up.is_null() {
        *up.offset(0) = cr * sp * cy + -sr * -sy;
        *up.offset(1) = cr * sp * sy + -sr * cy;
        *up.offset(2) = cr * cp
    };
}
// perpendicular vector could be replaced by this
//int	PlaneTypeForNormal (vec3_t normal);
/*
** assumes "src" is normalized
*/
#[no_mangle]

pub unsafe extern "C" fn PerpendicularVector(
    mut dst: *mut crate::src::qcommon::q_shared::vec_t,
    mut src: *const crate::src::qcommon::q_shared::vec_t,
) {
    let mut pos: i32 = 0;
    let mut i: i32 = 0;
    let mut minelem: f32 = 1.0;
    let mut tempvec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    /*
    	** find the smallest magnitude axially aligned vector
    	*/
    pos = 0;
    i = 0;
    while i < 3 {
        if crate::stdlib::fabs(*src.offset(i as isize) as f64)
            < minelem as f64
        {
            pos = i;
            minelem =
                crate::stdlib::fabs(*src.offset(i as isize) as f64) as f32
        }
        i += 1
    }
    tempvec[2] = 0.0;
    tempvec[1] = tempvec[2];
    tempvec[0] = tempvec[1];
    tempvec[pos as usize] = 1.0;
    /*
    	** project the point onto the plane defined by src
    	*/
    ProjectPointOnPlane(
        dst,
        tempvec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        src,
    );
    /*
    	** normalize the result
    	*/
    VectorNormalize(dst);
}
/*
================
Q_isnan

Don't pass doubles to this
================
*/
#[no_mangle]

pub unsafe extern "C" fn Q_isnan(mut x: f32) -> i32 {
    let mut fi: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    fi.f = x;
    fi.ui &= 0x7fffffff;
    fi.ui = (0x7f800000u32).wrapping_sub(fi.ui);
    return (fi.ui >> 31) as i32;
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
// expand constants before stringifying them
// angle indexes
// up / down
// left / right
// fall over
// the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
// max length of a string passed to Cmd_TokenizeString
// max tokens resulting from Cmd_TokenizeString
// max length of an individual token
// used for system info key only
// max length of a quake game pathname
// max length of a client name
// parameters for command buffer stuffing
// don't return until completed, a VM should NEVER use this,
// because some commands might cause the VM to be unloaded...
// insert at current position, but don't run yet
// add to end of the command buffer (normal case)
//
// these aren't needed by any of the VMs.  put in another header?
//
// bit vector of area visibility
// print levels from renderer (FIXME: set up for game / cgame?)
// only print when "developer 1"
// parameters to the main Error routine
// exit the entire game with a popup window
// print to console and disconnect from game
// don't kill server
// client disconnected from the server
// pop up the need-cd dialog
// font rendering values used by ui and cgame
// default
// default
/*
==============================================================

MATHLIB

==============================================================
*/
// all drawing is done to a 640*480 virtual screen size
// and will be automatically scaled to the real resolution
// ^[0-9a-zA-Z]
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
//------------------------------------------------------------------------
/*
=====================
Q_acos

the msvc acos doesn't always return a value between -PI and PI:

int i;
i = 1065353246;
acos(*(float*) &i) == -1.#IND0

=====================
*/
#[no_mangle]

pub unsafe extern "C" fn Q_acos(mut c: f32) -> f32 {
    let mut angle: f32 = 0.;
    angle = crate::stdlib::acos(c as f64) as f32;
    if angle as f64 > 3.14159265358979323846 {
        return 3.141592653589793f32;
    }
    if (angle as f64) < -3.14159265358979323846 {
        return 3.141592653589793f32;
    }
    return angle;
}
