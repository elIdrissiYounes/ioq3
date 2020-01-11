// =============== BEGIN q_shared_h ================
pub type qhandle_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvar_s {
    pub name: *mut i8,
    pub string: *mut i8,
    pub resetString: *mut i8,
    pub latchedString: *mut i8,
    pub flags: i32,
    pub modified: crate::src::qcommon::q_shared::qboolean,
    pub modificationCount: i32,
    pub value: f32,
    pub integer: i32,
    pub validate: crate::src::qcommon::q_shared::qboolean,
    pub integral: crate::src::qcommon::q_shared::qboolean,
    pub min: f32,
    pub max: f32,
    pub description: *mut i8,
    pub next: *mut crate::src::qcommon::q_shared::cvar_t,
    pub prev: *mut crate::src::qcommon::q_shared::cvar_t,
    pub hashNext: *mut crate::src::qcommon::q_shared::cvar_t,
    pub hashPrev: *mut crate::src::qcommon::q_shared::cvar_t,
    pub hashIndex: i32,
}

pub type cvar_t = crate::src::qcommon::q_shared::cvar_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct markFragment_t {
    pub firstPoint: i32,
    pub numPoints: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct orientation_t {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct glyphInfo_t {
    pub height: i32,
    pub top: i32,
    pub bottom: i32,
    pub pitch: i32,
    pub xSkip: i32,
    pub imageWidth: i32,
    pub imageHeight: i32,
    pub s: f32,
    pub t: f32,
    pub s2: f32,
    pub t2: f32,
    pub glyph: crate::src::qcommon::q_shared::qhandle_t,
    pub shaderName: [i8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fontInfo_t {
    pub glyphs: [crate::src::qcommon::q_shared::glyphInfo_t; 256],
    pub glyphScale: f32,
    pub name: [i8; 64],
}

pub type vec_t = f32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union floatint_t {
    pub f: f32,
    pub i: i32,
    pub ui: u32,
}

pub type sfxHandle_t = i32;

pub type clipHandle_t = i32;

pub type cvarHandle_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmCvar_t {
    pub handle: crate::src::qcommon::q_shared::cvarHandle_t,
    pub modificationCount: i32,
    pub value: f32,
    pub integer: i32,
    pub string: [i8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_t {
    pub allsolid: crate::src::qcommon::q_shared::qboolean,
    pub startsolid: crate::src::qcommon::q_shared::qboolean,
    pub fraction: f32,
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub surfaceFlags: i32,
    pub contents: i32,
    pub entityNum: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gameState_t {
    pub stringOffsets: [i32; 1024],
    pub stringData: [i8; 16000],
    pub dataCount: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct playerState_s {
    pub commandTime: i32,
    pub pm_type: i32,
    pub bobCycle: i32,
    pub pm_flags: i32,
    pub pm_time: i32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub weaponTime: i32,
    pub gravity: i32,
    pub speed: i32,
    pub delta_angles: [i32; 3],
    pub groundEntityNum: i32,
    pub legsTimer: i32,
    pub legsAnim: i32,
    pub torsoTimer: i32,
    pub torsoAnim: i32,
    pub movementDir: i32,
    pub grapplePoint: crate::src::qcommon::q_shared::vec3_t,
    pub eFlags: i32,
    pub eventSequence: i32,
    pub events: [i32; 2],
    pub eventParms: [i32; 2],
    pub externalEvent: i32,
    pub externalEventParm: i32,
    pub externalEventTime: i32,
    pub clientNum: i32,
    pub weapon: i32,
    pub weaponstate: i32,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub viewheight: i32,
    pub damageEvent: i32,
    pub damageYaw: i32,
    pub damagePitch: i32,
    pub damageCount: i32,
    pub stats: [i32; 16],
    pub persistant: [i32; 16],
    pub powerups: [i32; 16],
    pub ammo: [i32; 16],
    pub generic1: i32,
    pub loopSound: i32,
    pub jumppad_ent: i32,
    pub ping: i32,
    pub pmove_framecount: i32,
    pub jumppad_frame: i32,
    pub entityEventSequence: i32,
}

pub type playerState_t = crate::src::qcommon::q_shared::playerState_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usercmd_s {
    pub serverTime: i32,
    pub angles: [i32; 3],
    pub buttons: i32,
    pub weapon: crate::src::qcommon::q_shared::byte,
    pub forwardmove: i8,
    pub rightmove: i8,
    pub upmove: i8,
}

pub type usercmd_t = crate::src::qcommon::q_shared::usercmd_s;

pub type trType_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trajectory_t {
    pub trType: crate::src::qcommon::q_shared::trType_t,
    pub trTime: i32,
    pub trDuration: i32,
    pub trBase: crate::src::qcommon::q_shared::vec3_t,
    pub trDelta: crate::src::qcommon::q_shared::vec3_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct entityState_s {
    pub number: i32,
    pub eType: i32,
    pub eFlags: i32,
    pub pos: crate::src::qcommon::q_shared::trajectory_t,
    pub apos: crate::src::qcommon::q_shared::trajectory_t,
    pub time: i32,
    pub time2: i32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub origin2: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub angles2: crate::src::qcommon::q_shared::vec3_t,
    pub otherEntityNum: i32,
    pub otherEntityNum2: i32,
    pub groundEntityNum: i32,
    pub constantLight: i32,
    pub loopSound: i32,
    pub modelindex: i32,
    pub modelindex2: i32,
    pub clientNum: i32,
    pub frame: i32,
    pub solid: i32,
    pub event: i32,
    pub eventParm: i32,
    pub powerups: i32,
    pub weapon: i32,
    pub legsAnim: i32,
    pub torsoAnim: i32,
    pub generic1: i32,
}

pub type entityState_t = crate::src::qcommon::q_shared::entityState_s;

pub type connstate_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtime_s {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
}

pub type qtime_t = crate::src::qcommon::q_shared::qtime_s;

pub type e_status = u32;

pub type vec4_t = [crate::src::qcommon::q_shared::vec_t; 4];

pub type ha_pref = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qint64 {
    pub b0: crate::src::qcommon::q_shared::byte,
    pub b1: crate::src::qcommon::q_shared::byte,
    pub b2: crate::src::qcommon::q_shared::byte,
    pub b3: crate::src::qcommon::q_shared::byte,
    pub b4: crate::src::qcommon::q_shared::byte,
    pub b5: crate::src::qcommon::q_shared::byte,
    pub b6: crate::src::qcommon::q_shared::byte,
    pub b7: crate::src::qcommon::q_shared::byte,
}

pub const TR_GRAVITY: crate::src::qcommon::q_shared::trType_t = 5;

pub const TR_SINE: crate::src::qcommon::q_shared::trType_t = 4;

pub const TR_LINEAR_STOP: crate::src::qcommon::q_shared::trType_t = 3;

pub const TR_LINEAR: crate::src::qcommon::q_shared::trType_t = 2;

pub const TR_INTERPOLATE: crate::src::qcommon::q_shared::trType_t = 1;

pub const TR_STATIONARY: crate::src::qcommon::q_shared::trType_t = 0;

pub const CA_CINEMATIC: crate::src::qcommon::q_shared::connstate_t = 9;

pub const CA_ACTIVE: crate::src::qcommon::q_shared::connstate_t = 8;

pub const CA_PRIMED: crate::src::qcommon::q_shared::connstate_t = 7;

pub const CA_LOADING: crate::src::qcommon::q_shared::connstate_t = 6;

pub const CA_CONNECTED: crate::src::qcommon::q_shared::connstate_t = 5;

pub const CA_CHALLENGING: crate::src::qcommon::q_shared::connstate_t = 4;

pub const CA_CONNECTING: crate::src::qcommon::q_shared::connstate_t = 3;

pub const CA_AUTHORIZING: crate::src::qcommon::q_shared::connstate_t = 2;

pub const CA_DISCONNECTED: crate::src::qcommon::q_shared::connstate_t = 1;

pub const CA_UNINITIALIZED: crate::src::qcommon::q_shared::connstate_t = 0;

pub const FMV_ID_WAIT: crate::src::qcommon::q_shared::e_status = 6;

pub const FMV_LOOPED: crate::src::qcommon::q_shared::e_status = 5;

pub const FMV_ID_IDLE: crate::src::qcommon::q_shared::e_status = 4;

pub const FMV_ID_BLT: crate::src::qcommon::q_shared::e_status = 3;

pub const FMV_EOF: crate::src::qcommon::q_shared::e_status = 2;

pub const FMV_PLAY: crate::src::qcommon::q_shared::e_status = 1;

pub const FMV_IDLE: crate::src::qcommon::q_shared::e_status = 0;

pub const PRINT_ERROR: crate::be_aas_h::C2RustUnnamed_0 = 3;

pub const PRINT_WARNING: crate::be_aas_h::C2RustUnnamed_0 = 2;

pub const PRINT_DEVELOPER: crate::be_aas_h::C2RustUnnamed_0 = 1;

pub const PRINT_ALL: crate::be_aas_h::C2RustUnnamed_0 = 0;

pub const h_dontcare: crate::src::qcommon::q_shared::ha_pref = 2;

pub const h_low: crate::src::qcommon::q_shared::ha_pref = 1;

pub const h_high: crate::src::qcommon::q_shared::ha_pref = 0;

pub type byte = u8;

pub type qboolean = u32;

pub type fileHandle_t = i32;

pub type vec3_t = [crate::src::qcommon::q_shared::vec_t; 3];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cplane_s {
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub dist: f32,
    pub type_0: crate::src::qcommon::q_shared::byte,
    pub signbits: crate::src::qcommon::q_shared::byte,
    pub pad: [crate::src::qcommon::q_shared::byte; 2],
}

pub type fsMode_t = u32;

pub type cplane_t = crate::src::qcommon::q_shared::cplane_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pc_token_s {
    pub type_0: i32,
    pub subtype: i32,
    pub intvalue: i32,
    pub floatvalue: f32,
    pub string: [i8; 1024],
}

pub type pc_token_t = crate::src::qcommon::q_shared::pc_token_s;

pub const qtrue: crate::src::qcommon::q_shared::qboolean = 1;

pub const qfalse: crate::src::qcommon::q_shared::qboolean = 0;

pub const FS_APPEND_SYNC: crate::src::qcommon::q_shared::fsMode_t = 3;

pub const FS_APPEND: crate::src::qcommon::q_shared::fsMode_t = 2;

pub const FS_WRITE: crate::src::qcommon::q_shared::fsMode_t = 1;

pub const FS_READ: crate::src::qcommon::q_shared::fsMode_t = 0;

pub const FS_SEEK_SET: crate::be_aas_h::C2RustUnnamed_0 = 2;

pub const FS_SEEK_END: crate::be_aas_h::C2RustUnnamed_0 = 1;

pub const FS_SEEK_CUR: crate::be_aas_h::C2RustUnnamed_0 = 0;

pub const CHAN_ANNOUNCER: crate::be_aas_h::C2RustUnnamed_0 = 7;

pub const CHAN_LOCAL_SOUND: crate::be_aas_h::C2RustUnnamed_0 = 6;

pub const CHAN_BODY: crate::be_aas_h::C2RustUnnamed_0 = 5;

pub const CHAN_ITEM: crate::be_aas_h::C2RustUnnamed_0 = 4;

pub const CHAN_VOICE: crate::be_aas_h::C2RustUnnamed_0 = 3;

pub const CHAN_WEAPON: crate::be_aas_h::C2RustUnnamed_0 = 2;

pub const CHAN_LOCAL: crate::be_aas_h::C2RustUnnamed_0 = 1;

pub const CHAN_AUTO: crate::be_aas_h::C2RustUnnamed_0 = 0;

pub const EXEC_APPEND: crate::be_aas_h::C2RustUnnamed_0 = 2;

pub const EXEC_INSERT: crate::be_aas_h::C2RustUnnamed_0 = 1;

pub const EXEC_NOW: crate::be_aas_h::C2RustUnnamed_0 = 0;

pub const ERR_NEED_CD: crate::be_aas_h::C2RustUnnamed_0 = 4;

pub const ERR_DISCONNECT: crate::be_aas_h::C2RustUnnamed_0 = 3;

pub const ERR_SERVERDISCONNECT: crate::be_aas_h::C2RustUnnamed_0 = 2;

pub const ERR_DROP: crate::be_aas_h::C2RustUnnamed_0 = 1;

pub const ERR_FATAL: crate::be_aas_h::C2RustUnnamed_0 = 0;
use ::libc;

pub mod ctype_h {

    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }
    use crate::stdlib::strtod;
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::q_shared::ctype_h::tolower;
pub use crate::src::qcommon::q_shared::ctype_h::toupper;
pub use crate::src::qcommon::q_shared::stdlib_float_h::atof;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__ctype_tolower_loc;
pub use crate::stdlib::__ctype_toupper_loc;
use crate::stdlib::memmove;
use crate::stdlib::strcat;
use crate::stdlib::strchr;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncpy;
use crate::stdlib::strrchr;
use crate::stdlib::strtod;
use crate::stdlib::vsnprintf;
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
// q_shared.c -- stateless support routines that are included in each code dll
// ^[0-9a-zA-Z]
#[no_mangle]

pub unsafe extern "C" fn Q_IsColorString(
    mut p: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    if p.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *p.offset(0) as i32 != '^' as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *p.offset(1) as i32 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // isalnum expects a signed integer in the range -1 (EOF) to 255, or it might assert on undefined behaviour
    // a dereferenced char pointer has the range -128 to 127, so we just need to rangecheck the negative part
    if (*p.offset(1) as i32) < 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *(*crate::stdlib::__ctype_b_loc()).offset(*p.offset(1) as i32 as isize) as i32
        & crate::stdlib::_ISalnum as u16 as i32
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn Com_Clamp(mut min: f32, mut max: f32, mut value: f32) -> f32 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
/*
============
COM_SkipPath
============
*/
#[no_mangle]

pub unsafe extern "C" fn COM_SkipPath(mut pathname: *mut i8) -> *mut i8 {
    let mut last: *mut i8 = 0 as *mut i8;
    last = pathname;
    while *pathname != 0 {
        if *pathname as i32 == '/' as i32 {
            last = pathname.offset(1)
        }
        pathname = pathname.offset(1)
    }
    return last;
}
/*
============
COM_GetExtension
============
*/
#[no_mangle]

pub unsafe extern "C" fn COM_GetExtension(mut name: *const i8) -> *const i8 {
    let mut dot: *const i8 = crate::stdlib::strrchr(name, '.' as i32);
    let mut slash: *const i8 = 0 as *const i8;
    if !dot.is_null() && {
        slash = crate::stdlib::strrchr(name, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        return dot.offset(1isize);
    } else {
        return b"\x00" as *const u8 as *const i8;
    };
}
/*
============
COM_StripExtension
============
*/
#[no_mangle]

pub unsafe extern "C" fn COM_StripExtension(
    mut in_0: *const i8,
    mut out: *mut i8,
    mut destsize: i32,
) {
    let mut dot: *const i8 = crate::stdlib::strrchr(in_0, '.' as i32);
    let mut slash: *const i8 = 0 as *const i8;
    if !dot.is_null() && {
        slash = crate::stdlib::strrchr(in_0, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        destsize = if (destsize as isize) < dot.wrapping_offset_from(in_0) + 1 {
            destsize as isize
        } else {
            (dot.wrapping_offset_from(in_0)) + 1
        } as i32
    }
    if in_0 == out as *const i8 && destsize > 1 {
        *out.offset((destsize - 1i32) as isize) = '\u{0}' as i8
    } else {
        Q_strncpyz(out, in_0, destsize);
    };
}
/*
============
COM_CompareExtension

string compare the end of the strings and return qtrue if strings match
============
*/
#[no_mangle]

pub unsafe extern "C" fn COM_CompareExtension(
    mut in_0: *const i8,
    mut ext: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut inlen: i32 = 0;
    let mut extlen: i32 = 0;
    inlen = crate::stdlib::strlen(in_0) as i32;
    extlen = crate::stdlib::strlen(ext) as i32;
    if extlen <= inlen {
        in_0 = in_0.offset((inlen - extlen) as isize);
        if Q_stricmp(in_0, ext) == 0 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==================
COM_DefaultExtension

if path doesn't have an extension, then append
 the specified one (which should include the .)
==================
*/
#[no_mangle]

pub unsafe extern "C" fn COM_DefaultExtension(
    mut path: *mut i8,
    mut maxSize: i32,
    mut extension: *const i8,
) {
    let mut dot: *const i8 = crate::stdlib::strrchr(path, '.' as i32);
    let mut slash: *const i8 = 0 as *const i8;
    if !dot.is_null() && {
        slash = crate::stdlib::strrchr(path, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        return;
    } else {
        Q_strcat(path, maxSize, extension);
    };
}
/*
============================================================================

                    BYTE ORDER FUNCTIONS

============================================================================
*/
/*
// can't just use function pointers, or dll linkage can
// mess up when qcommon is included in multiple places
static short	(*_BigShort) (short l);
static short	(*_LittleShort) (short l);
static int		(*_BigLong) (int l);
static int		(*_LittleLong) (int l);
static qint64	(*_BigLong64) (qint64 l);
static qint64	(*_LittleLong64) (qint64 l);
static float	(*_BigFloat) (const float *l);
static float	(*_LittleFloat) (const float *l);

short	BigShort(short l){return _BigShort(l);}
short	LittleShort(short l) {return _LittleShort(l);}
int		BigLong (int l) {return _BigLong(l);}
int		LittleLong (int l) {return _LittleLong(l);}
qint64 	BigLong64 (qint64 l) {return _BigLong64(l);}
qint64 	LittleLong64 (qint64 l) {return _LittleLong64(l);}
float	BigFloat (const float *l) {return _BigFloat(l);}
float	LittleFloat (const float *l) {return _LittleFloat(l);}
*/
#[no_mangle]

pub unsafe extern "C" fn CopyShortSwap(mut dest: *mut libc::c_void, mut src: *mut libc::c_void) {
    let mut to: *mut crate::src::qcommon::q_shared::byte =
        dest as *mut crate::src::qcommon::q_shared::byte;
    let mut from: *mut crate::src::qcommon::q_shared::byte =
        src as *mut crate::src::qcommon::q_shared::byte;
    *to.offset(0) = *from.offset(1);
    *to.offset(1) = *from.offset(0);
}
#[no_mangle]

pub unsafe extern "C" fn CopyLongSwap(mut dest: *mut libc::c_void, mut src: *mut libc::c_void) {
    let mut to: *mut crate::src::qcommon::q_shared::byte =
        dest as *mut crate::src::qcommon::q_shared::byte;
    let mut from: *mut crate::src::qcommon::q_shared::byte =
        src as *mut crate::src::qcommon::q_shared::byte;
    *to.offset(0) = *from.offset(3);
    *to.offset(1) = *from.offset(2);
    *to.offset(2) = *from.offset(1);
    *to.offset(3) = *from.offset(0);
}
#[no_mangle]

pub unsafe extern "C" fn ShortSwap(mut l: i16) -> i16 {
    let mut b1: crate::src::qcommon::q_shared::byte = 0;
    let mut b2: crate::src::qcommon::q_shared::byte = 0;
    b1 = (l as i32 & 255) as crate::src::qcommon::q_shared::byte;
    b2 = (l as i32 >> 8 & 255) as crate::src::qcommon::q_shared::byte;
    return (((b1 as i32) << 8) + b2 as i32) as i16;
}
#[no_mangle]

pub unsafe extern "C" fn ShortNoSwap(mut l: i16) -> i16 {
    return l;
}
#[no_mangle]

pub unsafe extern "C" fn LongSwap(mut l: i32) -> i32 {
    let mut b1: crate::src::qcommon::q_shared::byte = 0;
    let mut b2: crate::src::qcommon::q_shared::byte = 0;
    let mut b3: crate::src::qcommon::q_shared::byte = 0;
    let mut b4: crate::src::qcommon::q_shared::byte = 0;
    b1 = (l & 255i32) as crate::src::qcommon::q_shared::byte;
    b2 = (l >> 8 & 255i32) as crate::src::qcommon::q_shared::byte;
    b3 = (l >> 16 & 255i32) as crate::src::qcommon::q_shared::byte;
    b4 = (l >> 24 & 255i32) as crate::src::qcommon::q_shared::byte;
    return ((b1 as i32) << 24) + ((b2 as i32) << 16) + ((b3 as i32) << 8) + b4 as i32;
}
#[no_mangle]

pub unsafe extern "C" fn LongNoSwap(mut l: i32) -> i32 {
    return l;
}
#[no_mangle]

pub unsafe extern "C" fn Long64Swap(
    mut ll: crate::src::qcommon::q_shared::qint64,
) -> crate::src::qcommon::q_shared::qint64 {
    let mut result: crate::src::qcommon::q_shared::qint64 = crate::src::qcommon::q_shared::qint64 {
        b0: 0,
        b1: 0,
        b2: 0,
        b3: 0,
        b4: 0,
        b5: 0,
        b6: 0,
        b7: 0,
    };
    result.b0 = ll.b7;
    result.b1 = ll.b6;
    result.b2 = ll.b5;
    result.b3 = ll.b4;
    result.b4 = ll.b3;
    result.b5 = ll.b2;
    result.b6 = ll.b1;
    result.b7 = ll.b0;
    return result;
}
#[no_mangle]

pub unsafe extern "C" fn Long64NoSwap(
    mut ll: crate::src::qcommon::q_shared::qint64,
) -> crate::src::qcommon::q_shared::qint64 {
    return ll;
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
// this is for determining if we have an asm version of a C function
// don't include the C bits if included from qasm.h
// for windows fastcall option
//================================================================= WIN64/32 ===
//============================================================== MAC OS X ===
//================================================================= LINUX ===
//=================================================================== BSD ===
//================================================================= SUNOS ===
//================================================================== IRIX ===
//================================================================== Q3VM ===
//===========================================================================
//catch missing defines in above blocks
//endianness
#[no_mangle]

pub unsafe extern "C" fn FloatSwap(mut f: *const f32) -> f32 {
    let mut out: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    out.f = *f;
    out.ui = LongSwap(out.ui as i32) as u32;
    return out.f;
}
#[no_mangle]

pub unsafe extern "C" fn FloatNoSwap(mut f: *const f32) -> f32 {
    return *f;
}
/*
================
Swap_Init
================
*/
/*
void Swap_Init (void)
{
    byte	swaptest[2] = {1,0};

// set the byte swapping variables in a portable manner
    if ( *(short *)swaptest == 1)
    {
        _BigShort = ShortSwap;
        _LittleShort = ShortNoSwap;
        _BigLong = LongSwap;
        _LittleLong = LongNoSwap;
        _BigLong64 = Long64Swap;
        _LittleLong64 = Long64NoSwap;
        _BigFloat = FloatSwap;
        _LittleFloat = FloatNoSwap;
    }
    else
    {
        _BigShort = ShortNoSwap;
        _LittleShort = ShortSwap;
        _BigLong = LongNoSwap;
        _LittleLong = LongSwap;
        _BigLong64 = Long64NoSwap;
        _LittleLong64 = Long64Swap;
        _BigFloat = FloatNoSwap;
        _LittleFloat = FloatSwap;
    }

}
*/
/*
============================================================================

PARSING

============================================================================
*/

static mut com_token: [i8; 1024] = [0; 1024];

static mut com_parsename: [i8; 1024] = [0; 1024];

static mut com_lines: i32 = 0;

static mut com_tokenline: i32 = 0;
#[no_mangle]

pub unsafe extern "C" fn COM_BeginParseSession(mut name: *const i8) {
    com_lines = 1;
    com_tokenline = 0;
    Com_sprintf(
        com_parsename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
        b"%s\x00" as *const u8 as *const i8,
        name,
    );
}
#[no_mangle]

pub unsafe extern "C" fn COM_GetCurrentParseLine() -> i32 {
    if com_tokenline != 0 {
        return com_tokenline;
    }
    return com_lines;
}
#[no_mangle]

pub unsafe extern "C" fn COM_Parse(mut data_p: *mut *mut i8) -> *mut i8 {
    return COM_ParseExt(data_p, crate::src::qcommon::q_shared::qtrue);
}
#[no_mangle]

pub unsafe extern "C" fn COM_ParseError(mut format: *mut i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [i8; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 4096]>(),
        format,
        argptr.as_va_list(),
    );
    crate::src::qcommon::common::Com_Printf(
        b"ERROR: %s, line %d: %s\n\x00" as *const u8 as *const i8,
        com_parsename.as_mut_ptr(),
        COM_GetCurrentParseLine(),
        string.as_mut_ptr(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn COM_ParseWarning(mut format: *mut i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [i8; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 4096]>(),
        format,
        argptr.as_va_list(),
    );
    crate::src::qcommon::common::Com_Printf(
        b"WARNING: %s, line %d: %s\n\x00" as *const u8 as *const i8,
        com_parsename.as_mut_ptr(),
        COM_GetCurrentParseLine(),
        string.as_mut_ptr(),
    );
}
/*
==============
COM_Parse

Parse a token out of a string
Will never return NULL, just empty strings

If "allowLineBreaks" is qtrue then an empty
string will be returned if the next token is
a newline.
==============
*/

unsafe extern "C" fn SkipWhitespace(
    mut data: *mut i8,
    mut hasNewLines: *mut crate::src::qcommon::q_shared::qboolean,
) -> *mut i8 {
    let mut c: i32 = 0;
    loop {
        c = *data as i32;
        if !(c <= ' ' as i32) {
            break;
        }
        if c == 0 {
            return 0 as *mut i8;
        }
        if c == '\n' as i32 {
            com_lines += 1;
            *hasNewLines = crate::src::qcommon::q_shared::qtrue
        }
        data = data.offset(1)
    }
    return data;
}
#[no_mangle]

pub unsafe extern "C" fn COM_Compress(mut data_p: *mut i8) -> i32 {
    let mut in_0: *mut i8 = 0 as *mut i8;
    let mut out: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    let mut newline: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut whitespace: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    out = data_p;
    in_0 = out;
    if !in_0.is_null() {
        loop {
            c = *in_0 as i32;
            if !(c != 0) {
                break;
            }
            // skip double slash comments
            if c == '/' as i32 && *in_0.offset(1) as i32 == '/' as i32 {
                while *in_0 as i32 != 0 && *in_0 as i32 != '\n' as i32 {
                    in_0 = in_0.offset(1)
                }
            // skip /* */ comments
            } else if c == '/' as i32 && *in_0.offset(1) as i32 == '*' as i32 {
                while *in_0 as i32 != 0
                    && (*in_0 as i32 != '*' as i32 || *in_0.offset(1) as i32 != '/' as i32)
                {
                    in_0 = in_0.offset(1)
                }
                if *in_0 != 0 {
                    in_0 = in_0.offset(2)
                }
            // record when we hit a newline
            } else if c == '\n' as i32 || c == '\r' as i32 {
                newline = crate::src::qcommon::q_shared::qtrue;
                in_0 = in_0.offset(1)
            // record when we hit whitespace
            } else if c == ' ' as i32 || c == '\t' as i32 {
                whitespace = crate::src::qcommon::q_shared::qtrue;
                in_0 = in_0.offset(1)
            // an actual token
            } else {
                // if we have a pending newline, emit it (and it counts as whitespace)
                if newline as u64 != 0 {
                    let fresh0 = out;
                    out = out.offset(1);
                    *fresh0 = '\n' as i8;
                    newline = crate::src::qcommon::q_shared::qfalse;
                    whitespace = crate::src::qcommon::q_shared::qfalse
                }
                if whitespace as u64 != 0 {
                    let fresh1 = out;
                    out = out.offset(1);
                    *fresh1 = ' ' as i8;
                    whitespace = crate::src::qcommon::q_shared::qfalse
                }
                // copy quoted strings unmolested
                if c == '\"' as i32 {
                    let fresh2 = out;
                    out = out.offset(1);
                    *fresh2 = c as i8;
                    in_0 = in_0.offset(1);
                    loop {
                        c = *in_0 as i32;
                        if !(c != 0 && c != '\"' as i32) {
                            break;
                        }
                        let fresh3 = out;
                        out = out.offset(1);
                        *fresh3 = c as i8;
                        in_0 = in_0.offset(1)
                    }
                    if c == '\"' as i32 {
                        let fresh4 = out;
                        out = out.offset(1);
                        *fresh4 = c as i8;
                        in_0 = in_0.offset(1)
                    }
                } else {
                    *out = c as i8;
                    out = out.offset(1);
                    in_0 = in_0.offset(1)
                }
            }
        }
        *out = 0
    }
    return out.wrapping_offset_from(data_p) as i32;
}
#[no_mangle]

pub unsafe extern "C" fn COM_ParseExt(
    mut data_p: *mut *mut i8,
    mut allowLineBreaks: crate::src::qcommon::q_shared::qboolean,
) -> *mut i8 {
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    let mut hasNewLines: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut data: *mut i8 = 0 as *mut i8;
    data = *data_p;
    len = 0;
    com_token[0] = 0;
    com_tokenline = 0;
    // make sure incoming data is valid
    if data.is_null() {
        *data_p = 0 as *mut i8;
        return com_token.as_mut_ptr();
    }
    loop {
        // skip whitespace
        data = SkipWhitespace(data, &mut hasNewLines);
        if data.is_null() {
            *data_p = 0 as *mut i8;
            return com_token.as_mut_ptr();
        }
        if hasNewLines != 0 && allowLineBreaks as u64 == 0 {
            *data_p = data;
            return com_token.as_mut_ptr();
        }
        c = *data as i32;
        // skip double slash comments
        if c == '/' as i32 && *data.offset(1) as i32 == '/' as i32 {
            data = data.offset(2);
            while *data as i32 != 0 && *data as i32 != '\n' as i32 {
                data = data.offset(1)
            }
        } else {
            // skip /* */ comments
            if !(c == '/' as i32 && *data.offset(1) as i32 == '*' as i32) {
                break;
            }
            data = data.offset(2);
            while *data as i32 != 0
                && (*data as i32 != '*' as i32 || *data.offset(1) as i32 != '/' as i32)
            {
                if *data as i32 == '\n' as i32 {
                    com_lines += 1
                }
                data = data.offset(1)
            }
            if *data != 0 {
                data = data.offset(2)
            }
        }
    }
    // token starts on this line
    com_tokenline = com_lines;
    // handle quoted strings
    if c == '\"' as i32 {
        data = data.offset(1);
        loop {
            let fresh5 = data;
            data = data.offset(1);
            c = *fresh5 as i32;
            if c == '\"' as i32 || c == 0 {
                com_token[len as usize] = 0;
                *data_p = data;
                return com_token.as_mut_ptr();
            }
            if c == '\n' as i32 {
                com_lines += 1
            }
            if len < 1024 - 1 {
                com_token[len as usize] = c as i8;
                len += 1
            }
        }
    }
    loop
    // parse a regular word
    {
        if len < 1024 - 1 {
            com_token[len as usize] = c as i8;
            len += 1
        }
        data = data.offset(1);
        c = *data as i32;
        if !(c > 32) {
            break;
        }
    }
    com_token[len as usize] = 0;
    *data_p = data;
    return com_token.as_mut_ptr();
}
/*
==================
COM_MatchToken
==================
*/
#[no_mangle]

pub unsafe extern "C" fn COM_MatchToken(mut buf_p: *mut *mut i8, mut match_0: *mut i8) {
    let mut token: *mut i8 = 0 as *mut i8;
    token = COM_Parse(buf_p);
    if crate::stdlib::strcmp(token, match_0) != 0 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"MatchToken: %s != %s\x00" as *const u8 as *const i8,
            token,
            match_0,
        );
    };
}
/*
=================
SkipBracedSection

The next token should be an open brace or set depth to 1 if already parsed it.
Skips until a matching close brace is found.
Internal brace depths are properly skipped.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SkipBracedSection(
    mut program: *mut *mut i8,
    mut depth: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: *mut i8 = 0 as *mut i8;
    loop {
        token = COM_ParseExt(program, crate::src::qcommon::q_shared::qtrue);
        if *token.offset(1) as i32 == 0 {
            if *token.offset(0) as i32 == '{' as i32 {
                depth += 1
            } else if *token.offset(0) as i32 == '}' as i32 {
                depth -= 1
            }
        }
        if !(depth != 0 && !(*program).is_null()) {
            break;
        }
    }
    return (depth == 0) as crate::src::qcommon::q_shared::qboolean;
}
/*
=================
SkipRestOfLine
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SkipRestOfLine(mut data: *mut *mut i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    p = *data;
    if *p == 0 {
        return;
    }
    loop {
        let fresh6 = p;
        p = p.offset(1);
        c = *fresh6 as i32;
        if !(c != 0) {
            break;
        }
        if !(c == '\n' as i32) {
            continue;
        }
        com_lines += 1;
        break;
    }
    *data = p;
}
#[no_mangle]

pub unsafe extern "C" fn Parse1DMatrix(mut buf_p: *mut *mut i8, mut x: i32, mut m: *mut f32) {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    COM_MatchToken(buf_p, b"(\x00" as *const u8 as *mut i8);

    for i in 0..x {
        token = COM_Parse(buf_p);

        *m.offset(i as isize) = atof(token) as f32;
    }
    COM_MatchToken(buf_p, b")\x00" as *const u8 as *mut i8);
}
#[no_mangle]

pub unsafe extern "C" fn Parse2DMatrix(
    mut buf_p: *mut *mut i8,
    mut y: i32,
    mut x: i32,
    mut m: *mut f32,
) {
    let mut i: i32 = 0;
    COM_MatchToken(buf_p, b"(\x00" as *const u8 as *mut i8);

    for i in 0..y {
        Parse1DMatrix(buf_p, x, m.offset((i * x) as isize));
    }
    COM_MatchToken(buf_p, b")\x00" as *const u8 as *mut i8);
}
#[no_mangle]

pub unsafe extern "C" fn Parse3DMatrix(
    mut buf_p: *mut *mut i8,
    mut z: i32,
    mut y: i32,
    mut x: i32,
    mut m: *mut f32,
) {
    let mut i: i32 = 0;
    COM_MatchToken(buf_p, b"(\x00" as *const u8 as *mut i8);

    for i in 0..z {
        Parse2DMatrix(buf_p, y, x, m.offset((i * x * y) as isize));
    }
    COM_MatchToken(buf_p, b")\x00" as *const u8 as *mut i8);
}
/*
===================
Com_HexStrToInt
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_HexStrToInt(mut str: *const i8) -> i32 {
    if str.is_null() {
        return -(1i32);
    }
    // check for hex code
    if *str.offset(0) as i32 == '0' as i32
        && *str.offset(1) as i32 == 'x' as i32
        && *str.offset(2) as i32 != '\u{0}' as i32
    {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut len: i32 = crate::stdlib::strlen(str) as i32;

        for i in 2..len {
            let mut digit: i8 = 0;

            n *= 16;

            digit = ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<i8>() > 1 {
                    if 0 != 0 {
                        let mut __c: i32 = *str.offset(i as isize) as i32;
                        __res = if __c < -(128) || __c > 255 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(*str.offset(i as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc())
                        .offset(*str.offset(i as isize) as i32 as isize)
                }
                __res
            }) as i8;

            if digit as i32 >= '0' as i32 && digit as i32 <= '9' as i32 {
                digit = (digit as i32 - '0' as i32) as i8
            } else if digit as i32 >= 'a' as i32 && digit as i32 <= 'f' as i32 {
                digit = (digit as i32 - 'a' as i32 + 10) as i8
            } else {
                return -(1i32);
            }

            n += digit as i32;
        }
        return n;
    }
    return -(1);
}
/*
============================================================================

                    LIBRARY REPLACEMENT FUNCTIONS

============================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn Q_isprint(mut c: i32) -> i32 {
    if c >= 0x20 && c <= 0x7e {
        return 1i32;
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn Q_islower(mut c: i32) -> i32 {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        return 1i32;
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn Q_isupper(mut c: i32) -> i32 {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1i32;
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn Q_isalpha(mut c: i32) -> i32 {
    if c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1i32;
    }
    return 0;
}
#[no_mangle]

pub unsafe extern "C" fn Q_isanumber(mut s: *const i8) -> crate::src::qcommon::q_shared::qboolean {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut d: f64 = 0.;
    if *s as i32 == '\u{0}' as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    d = crate::stdlib::strtod(s, &mut p);
    return (*p as i32 == '\u{0}' as i32) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]

pub unsafe extern "C" fn Q_isintegral(mut f: f32) -> crate::src::qcommon::q_shared::qboolean {
    return (f as i32 as f32 == f) as crate::src::qcommon::q_shared::qboolean;
}
/*
=============
Q_strncpyz

Safe strncpy that ensures a trailing zero
=============
*/
#[no_mangle]

pub unsafe extern "C" fn Q_strncpyz(mut dest: *mut i8, mut src: *const i8, mut destsize: i32) {
    if dest.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: NULL dest\x00" as *const u8 as *const i8,
        );
    }
    if src.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: NULL src\x00" as *const u8 as *const i8,
        );
    }
    if destsize < 1 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: destsize < 1\x00" as *const u8 as *const i8,
        );
    }
    crate::stdlib::strncpy(dest, src, (destsize - 1i32) as usize);
    *dest.offset((destsize - 1i32) as isize) = 0i8;
}
#[no_mangle]

pub unsafe extern "C" fn Q_stricmpn(mut s1: *const i8, mut s2: *const i8, mut n: i32) -> i32 {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    if s1.is_null() {
        if s2.is_null() {
            return 0i32;
        } else {
            return -(1i32);
        }
    } else {
        if s2.is_null() {
            return 1i32;
        }
    }
    loop {
        let fresh7 = s1;
        s1 = s1.offset(1);
        c1 = *fresh7 as i32;
        let fresh8 = s2;
        s2 = s2.offset(1);
        c2 = *fresh8 as i32;
        let fresh9 = n;
        n = n - 1;
        if fresh9 == 0 {
            return 0i32;
            // strings are equal until end point
        }
        if c1 != c2 {
            if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
                c1 -= 'a' as i32 - 'A' as i32
            }
            if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
                c2 -= 'a' as i32 - 'A' as i32
            }
            if c1 != c2 {
                return if c1 < c2 { -(1i32) } else { 1i32 };
            }
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0;
    // strings are equal
}
#[no_mangle]

pub unsafe extern "C" fn Q_strncmp(mut s1: *const i8, mut s2: *const i8, mut n: i32) -> i32 {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    loop {
        let fresh10 = s1;
        s1 = s1.offset(1);
        c1 = *fresh10 as i32;
        let fresh11 = s2;
        s2 = s2.offset(1);
        c2 = *fresh11 as i32;
        let fresh12 = n;
        n = n - 1;
        if fresh12 == 0 {
            return 0i32;
            // strings are equal until end point
        }
        if c1 != c2 {
            return if c1 < c2 { -(1i32) } else { 1i32 };
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0;
    // strings are equal
}
#[no_mangle]

pub unsafe extern "C" fn Q_stricmp(mut s1: *const i8, mut s2: *const i8) -> i32 {
    return if !s1.is_null() && !s2.is_null() {
        Q_stricmpn(s1, s2, 99999)
    } else {
        -(1)
    };
}
#[no_mangle]

pub unsafe extern "C" fn Q_strlwr(mut s1: *mut i8) -> *mut i8 {
    let mut s: *mut i8 = 0 as *mut i8;
    s = s1;
    while *s != 0 {
        *s = ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *s as i32;
                    __res = if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*s as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(*s as i32 as isize)
            }
            __res
        }) as i8;
        s = s.offset(1)
    }
    return s1;
}
#[no_mangle]

pub unsafe extern "C" fn Q_strupr(mut s1: *mut i8) -> *mut i8 {
    let mut s: *mut i8 = 0 as *mut i8;
    s = s1;
    while *s != 0 {
        *s = ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *s as i32;
                    __res = if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                    }
                } else {
                    __res = toupper(*s as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_toupper_loc()).offset(*s as i32 as isize)
            }
            __res
        }) as i8;
        s = s.offset(1)
    }
    return s1;
}
// never goes past bounds or leaves without a terminating 0
#[no_mangle]

pub unsafe extern "C" fn Q_strcat(mut dest: *mut i8, mut size: i32, mut src: *const i8) {
    let mut l1: i32 = 0;
    l1 = crate::stdlib::strlen(dest) as i32;
    if l1 >= size {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strcat: already overflowed\x00" as *const u8 as *const i8,
        );
    }
    Q_strncpyz(dest.offset(l1 as isize), src, size - l1);
}
/*
* Find the first occurrence of find in s.
*/
#[no_mangle]

pub unsafe extern "C" fn Q_stristr(mut s: *const i8, mut find: *const i8) -> *const i8 {
    let mut c: i8 = 0;
    let mut sc: i8 = 0;
    let mut len: crate::stddef_h::size_t = 0;
    let fresh13 = find;
    find = find.offset(1);
    c = *fresh13;
    if c as i32 != 0 {
        if c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32 {
            c = (c as i32 - ('a' as i32 - 'A' as i32)) as i8
        }
        len = crate::stdlib::strlen(find);
        loop {
            loop {
                let fresh14 = s;
                s = s.offset(1);
                sc = *fresh14;
                if sc as i32 == 0 {
                    return 0 as *const i8;
                }
                if sc as i32 >= 'a' as i32 && sc as i32 <= 'z' as i32 {
                    sc = (sc as i32 - ('a' as i32 - 'A' as i32)) as i8
                }
                if !(sc as i32 != c as i32) {
                    break;
                }
            }
            if !(Q_stricmpn(s, find, len as i32) != 0) {
                break;
            }
        }
        s = s.offset(-1)
    }
    return s;
}
#[no_mangle]

pub unsafe extern "C" fn Q_PrintStrlen(mut string: *const i8) -> i32 {
    let mut len: i32 = 0;
    let mut p: *const i8 = 0 as *const i8;
    if string.is_null() {
        return 0i32;
    }
    len = 0;
    p = string;
    while *p != 0 {
        if Q_IsColorString(p) as u64 != 0 {
            p = p.offset(2)
        } else {
            p = p.offset(1);
            len += 1
        }
    }
    return len;
}
#[no_mangle]

pub unsafe extern "C" fn Q_CleanStr(mut string: *mut i8) -> *mut i8 {
    let mut d: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    s = string;
    d = string;
    loop {
        c = *s as i32;
        if !(c != 0) {
            break;
        }
        if Q_IsColorString(s) as u64 != 0 {
            s = s.offset(1)
        } else if c >= 0x20 && c <= 0x7e {
            let fresh15 = d;
            d = d.offset(1);
            *fresh15 = c as i8
        }
        s = s.offset(1)
    }
    *d = '\u{0}' as i8;
    return string;
}
#[no_mangle]

pub unsafe extern "C" fn Q_CountChar(mut string: *const i8, mut tocount: i8) -> i32 {
    let mut count: i32 = 0;
    count = 0;
    while *string != 0 {
        if *string as i32 == tocount as i32 {
            count += 1
        }
        string = string.offset(1)
    }
    return count;
}
#[no_mangle]

pub unsafe extern "C" fn Com_sprintf(
    mut dest: *mut i8,
    mut size: i32,
    mut fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut len: i32 = 0;
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    len = crate::stdlib::vsnprintf(dest, size as usize, fmt, argptr.as_va_list());
    if len >= size {
        crate::src::qcommon::common::Com_Printf(
            b"Com_sprintf: Output length %d too short, require %d bytes.\n\x00" as *const u8
                as *const i8,
            size,
            len + 1i32,
        );
    }
    return len;
}
/*
============
va

does a varargs printf into a temp buffer, so I don't need to have
varargs versions of all text functions.
============
*/
#[no_mangle]

pub unsafe extern "C" fn va(mut format: *mut i8, mut args: ...) -> *mut i8 {
    let mut argptr: ::std::ffi::VaListImpl; // in case va is called by nested functions
    static mut string: [[i8; 32000]; 2] = [[0; 32000]; 2];
    static mut index: i32 = 0;
    let mut buf: *mut i8 = 0 as *mut i8;
    buf = string[(index & 1) as usize].as_mut_ptr();
    index += 1;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        buf,
        ::std::mem::size_of::<[i8; 32000]>(),
        format,
        argptr.as_va_list(),
    );
    return buf;
}
/*
============
Com_TruncateLongString

Assumes buffer is atleast TRUNCATE_LENGTH big
============
*/
#[no_mangle]

pub unsafe extern "C" fn Com_TruncateLongString(mut buffer: *mut i8, mut s: *const i8) {
    let mut length: i32 = crate::stdlib::strlen(s) as i32;
    if length <= 64 {
        Q_strncpyz(buffer, s, 64i32);
    } else {
        Q_strncpyz(buffer, s, 64 / 2 - 3);
        Q_strcat(buffer, 64, b" ... \x00" as *const u8 as *const i8);
        Q_strcat(
            buffer,
            64i32,
            s.offset(length as isize)
                .offset(-((64i32 / 2i32) as isize))
                .offset(3isize),
        );
    };
}
/*
=====================================================================

  INFO STRINGS

=====================================================================
*/
/*
===============
Info_ValueForKey

Searches the string for the given
key and returns the associated value, or an empty string.
FIXME: overflow check?
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Info_ValueForKey(mut s: *const i8, mut key: *const i8) -> *mut i8 {
    let mut pkey: [i8; 8192] = [0; 8192]; // use two buffers so compares
    static mut value: [[i8; 8192]; 2] = [[0; 8192]; 2];
    // work without stomping on each other
    static mut valueindex: i32 = 0;
    let mut o: *mut i8 = 0 as *mut i8;
    if s.is_null() || key.is_null() {
        return b"\x00" as *const u8 as *mut i8;
    }
    if crate::stdlib::strlen(s) >= 8192 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_ValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    valueindex ^= 1;
    if *s as i32 == '\\' as i32 {
        s = s.offset(1)
    }
    loop {
        o = pkey.as_mut_ptr();
        while *s as i32 != '\\' as i32 {
            if *s == 0 {
                return b"\x00" as *const u8 as *mut i8;
            }
            let fresh16 = s;
            s = s.offset(1);
            let fresh17 = o;
            o = o.offset(1);
            *fresh17 = *fresh16
        }
        *o = 0;
        s = s.offset(1);
        o = value[valueindex as usize].as_mut_ptr();
        while *s as i32 != '\\' as i32 && *s as i32 != 0 {
            let fresh18 = s;
            s = s.offset(1);
            let fresh19 = o;
            o = o.offset(1);
            *fresh19 = *fresh18
        }
        *o = 0;
        if Q_stricmp(key, pkey.as_mut_ptr()) == 0 {
            return value[valueindex as usize].as_mut_ptr();
        }
        if *s == 0 {
            break;
        }
        s = s.offset(1)
    }
    return b"\x00" as *const u8 as *mut i8;
}
/*
===================
Info_NextPair

Used to itterate through all the key/value pairs in an info string
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Info_NextPair(
    mut head: *mut *const i8,
    mut key: *mut i8,
    mut value: *mut i8,
) {
    let mut o: *mut i8 = 0 as *mut i8;
    let mut s: *const i8 = 0 as *const i8;
    s = *head;
    if *s as i32 == '\\' as i32 {
        s = s.offset(1)
    }
    *key.offset(0) = 0i8;
    *value.offset(0) = 0i8;
    o = key;
    while *s as i32 != '\\' as i32 {
        if *s == 0 {
            *o = 0;
            *head = s;
            return;
        }
        let fresh20 = s;
        s = s.offset(1);
        let fresh21 = o;
        o = o.offset(1);
        *fresh21 = *fresh20
    }
    *o = 0;
    s = s.offset(1);
    o = value;
    while *s as i32 != '\\' as i32 && *s as i32 != 0 {
        let fresh22 = s;
        s = s.offset(1);
        let fresh23 = o;
        o = o.offset(1);
        *fresh23 = *fresh22
    }
    *o = 0;
    *head = s;
}
/*
===================
Info_RemoveKey
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Info_RemoveKey(mut s: *mut i8, mut key: *const i8) {
    let mut start: *mut i8 = 0 as *mut i8; // remove this part
    let mut pkey: [i8; 1024] = [0; 1024];
    let mut value: [i8; 1024] = [0; 1024];
    let mut o: *mut i8 = 0 as *mut i8;
    if crate::stdlib::strlen(s) >= 1024 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_RemoveKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    if !crate::stdlib::strchr(key, '\\' as i32).is_null() {
        return;
    }
    loop {
        start = s;
        if *s as i32 == '\\' as i32 {
            s = s.offset(1)
        }
        o = pkey.as_mut_ptr();
        while *s as i32 != '\\' as i32 {
            if *s == 0 {
                return;
            }
            let fresh24 = s;
            s = s.offset(1);
            let fresh25 = o;
            o = o.offset(1);
            *fresh25 = *fresh24
        }
        *o = 0;
        s = s.offset(1);
        o = value.as_mut_ptr();
        while *s as i32 != '\\' as i32 && *s as i32 != 0 {
            if *s == 0 {
                return;
            }
            let fresh26 = s;
            s = s.offset(1);
            let fresh27 = o;
            o = o.offset(1);
            *fresh27 = *fresh26
        }
        *o = 0;
        if crate::stdlib::strcmp(key, pkey.as_mut_ptr()) == 0 {
            crate::stdlib::memmove(
                start as *mut libc::c_void,
                s as *const libc::c_void,
                crate::stdlib::strlen(s).wrapping_add(1usize),
            );
            return;
        }
        if *s == 0 {
            return;
        }
    }
}
/*
===================
Info_RemoveKey_Big
===================
*/
#[no_mangle]

pub unsafe extern "C" fn Info_RemoveKey_Big(mut s: *mut i8, mut key: *const i8) {
    let mut start: *mut i8 = 0 as *mut i8; // remove this part
    let mut pkey: [i8; 8192] = [0; 8192];
    let mut value: [i8; 8192] = [0; 8192];
    let mut o: *mut i8 = 0 as *mut i8;
    if crate::stdlib::strlen(s) >= 8192 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_RemoveKey_Big: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    if !crate::stdlib::strchr(key, '\\' as i32).is_null() {
        return;
    }
    loop {
        start = s;
        if *s as i32 == '\\' as i32 {
            s = s.offset(1)
        }
        o = pkey.as_mut_ptr();
        while *s as i32 != '\\' as i32 {
            if *s == 0 {
                return;
            }
            let fresh28 = s;
            s = s.offset(1);
            let fresh29 = o;
            o = o.offset(1);
            *fresh29 = *fresh28
        }
        *o = 0;
        s = s.offset(1);
        o = value.as_mut_ptr();
        while *s as i32 != '\\' as i32 && *s as i32 != 0 {
            if *s == 0 {
                return;
            }
            let fresh30 = s;
            s = s.offset(1);
            let fresh31 = o;
            o = o.offset(1);
            *fresh31 = *fresh30
        }
        *o = 0;
        if crate::stdlib::strcmp(key, pkey.as_mut_ptr()) == 0 {
            crate::stdlib::memmove(
                start as *mut libc::c_void,
                s as *const libc::c_void,
                crate::stdlib::strlen(s).wrapping_add(1usize),
            );
            return;
        }
        if *s == 0 {
            return;
        }
    }
}
/*
==================
Info_Validate

Some characters are illegal in info strings because they
can mess up the server's parsing
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Info_Validate(
    mut s: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    if !crate::stdlib::strchr(s, '\"' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if !crate::stdlib::strchr(s, ';' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
==================
Info_SetValueForKey

Changes or adds a key/value pair
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Info_SetValueForKey(
    mut s: *mut i8,
    mut key: *const i8,
    mut value: *const i8,
) {
    let mut newi: [i8; 1024] = [0; 1024];
    let mut blacklist: *const i8 = b"\\;\"\x00" as *const u8 as *const i8;
    if crate::stdlib::strlen(s) >= 1024 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    while *blacklist != 0 {
        if !crate::stdlib::strchr(key, *blacklist as i32).is_null()
            || !crate::stdlib::strchr(value, *blacklist as i32).is_null()
        {
            crate::src::qcommon::common::Com_Printf(
                b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00" as *const u8
                    as *const i8,
                *blacklist as i32,
                key,
                value,
            );
            return;
        }
        blacklist = blacklist.offset(1)
    }
    Info_RemoveKey(s, key);
    if value.is_null() || crate::stdlib::strlen(value) == 0 {
        return;
    }
    Com_sprintf(
        newi.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
        b"\\%s\\%s\x00" as *const u8 as *const i8,
        key,
        value,
    );
    if crate::stdlib::strlen(newi.as_mut_ptr()).wrapping_add(crate::stdlib::strlen(s)) >= 1024usize
    {
        crate::src::qcommon::common::Com_Printf(
            b"Info string length exceeded\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    crate::stdlib::strcat(newi.as_mut_ptr(), s);
    crate::stdlib::strcpy(s, newi.as_mut_ptr());
}
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
/*
==================
Info_SetValueForKey_Big

Changes or adds a key/value pair
Includes and retains zero-length values
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Info_SetValueForKey_Big(
    mut s: *mut i8,
    mut key: *const i8,
    mut value: *const i8,
) {
    let mut newi: [i8; 8192] = [0; 8192];
    let mut blacklist: *const i8 = b"\\;\"\x00" as *const u8 as *const i8;
    if crate::stdlib::strlen(s) >= 8192 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    while *blacklist != 0 {
        if !crate::stdlib::strchr(key, *blacklist as i32).is_null()
            || !crate::stdlib::strchr(value, *blacklist as i32).is_null()
        {
            crate::src::qcommon::common::Com_Printf(
                b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00" as *const u8
                    as *const i8,
                *blacklist as i32,
                key,
                value,
            );
            return;
        }
        blacklist = blacklist.offset(1)
    }
    Info_RemoveKey_Big(s, key);
    if value.is_null() {
        return;
    }
    Com_sprintf(
        newi.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 8192]>() as i32,
        b"\\%s\\%s\x00" as *const u8 as *const i8,
        key,
        value,
    );
    if crate::stdlib::strlen(newi.as_mut_ptr()).wrapping_add(crate::stdlib::strlen(s)) >= 8192usize
    {
        crate::src::qcommon::common::Com_Printf(
            b"BIG Info string length exceeded\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    crate::stdlib::strcat(s, newi.as_mut_ptr());
}
//====================================================================
/*
==================
Com_CharIsOneOfCharset
==================
*/

unsafe extern "C" fn Com_CharIsOneOfCharset(
    mut c: i8,
    mut set: *mut i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    i = 0;
    while (i as usize) < crate::stdlib::strlen(set) {
        if *set.offset(i as isize) as i32 == c as i32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==================
Com_SkipCharset
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_SkipCharset(mut s: *mut i8, mut sep: *mut i8) -> *mut i8 {
    let mut p: *mut i8 = s;
    while !p.is_null() {
        if !(Com_CharIsOneOfCharset(*p, sep) as u64 != 0) {
            break;
        }
        p = p.offset(1)
    }
    return p;
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
/*
==================
Com_SkipTokens
==================
*/
#[no_mangle]

pub unsafe extern "C" fn Com_SkipTokens(
    mut s: *mut i8,
    mut numTokens: i32,
    mut sep: *mut i8,
) -> *mut i8 {
    let mut sepCount: i32 = 0;
    let mut p: *mut i8 = s;
    while sepCount < numTokens {
        let fresh32 = p;
        p = p.offset(1);
        if Com_CharIsOneOfCharset(*fresh32, sep) as u64 != 0 {
            sepCount += 1;
            while Com_CharIsOneOfCharset(*p, sep) as u64 != 0 {
                p = p.offset(1)
            }
        } else if *p as i32 == '\u{0}' as i32 {
            break;
        }
    }
    if sepCount == numTokens {
        return p;
    } else {
        return s;
    };
}
