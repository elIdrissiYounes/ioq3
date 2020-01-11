use ::libc;

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::STAT_ARMOR;
pub use crate::bg_public_h::STAT_CLIENTS_READY;
pub use crate::bg_public_h::STAT_DEAD_YAW;
pub use crate::bg_public_h::STAT_HEALTH;
pub use crate::bg_public_h::STAT_HOLDABLE_ITEM;
pub use crate::bg_public_h::STAT_MAX_HEALTH;
pub use crate::bg_public_h::STAT_WEAPONS;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic;
pub use crate::src::cgame::cg_syscalls::trap_R_SetColor;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
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
// cg_drawtools.c -- helper functions called by cg_draw, cg_scoreboard, cg_info, etc
/*
================
CG_AdjustFrom640

Adjusted for resolution and screen aspect ratio
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AdjustFrom640(
    mut x: *mut f32,
    mut y: *mut f32,
    mut w: *mut f32,
    mut h: *mut f32,
) {
    // scale for screen sizes
    *x *= crate::src::cgame::cg_main::cgs.screenXScale;
    *y *= crate::src::cgame::cg_main::cgs.screenYScale;
    *w *= crate::src::cgame::cg_main::cgs.screenXScale;
    *h *= crate::src::cgame::cg_main::cgs.screenYScale;
}
/*
================
CG_FillRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FillRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut color: *const f32,
) {
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color);
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0f32,
        0f32,
        0f32,
        0f32,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
================
CG_DrawSides

Coords are virtual 640x480
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSides(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut size: f32,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= crate::src::cgame::cg_main::cgs.screenXScale;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        size,
        h,
        0f32,
        0f32,
        0f32,
        0f32,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x + w - size,
        y,
        size,
        h,
        0f32,
        0f32,
        0f32,
        0f32,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawTopBottom(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut size: f32,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= crate::src::cgame::cg_main::cgs.screenYScale;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        w,
        size,
        0f32,
        0f32,
        0f32,
        0f32,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x,
        y + h - size,
        w,
        size,
        0f32,
        0f32,
        0f32,
        0f32,
        crate::src::cgame::cg_main::cgs.media.whiteShader,
    );
}
/*
================
UI_DrawRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut size: f32,
    mut color: *const f32,
) {
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color);
    CG_DrawTopBottom(x, y, width, height, size);
    CG_DrawSides(x, y + size, width, height - size * 2f32, size);
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
================
CG_DrawPic

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawPic(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x, y, width, height, 0f32, 0f32, 1f32, 1f32, hShader,
    );
}
/*
===============
CG_DrawChar

Coordinates and size in 640*480 virtual screen size
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawChar(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
    mut ch: i32,
) {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut size: f32 = 0.;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    ch &= 255;
    if ch == ' ' as i32 {
        return;
    }
    ax = x as f32;
    ay = y as f32;
    aw = width as f32;
    ah = height as f32;
    CG_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    row = ch >> 4;
    col = ch & 15;
    frow = (row as f64 * 0.0625) as f32;
    fcol = (col as f64 * 0.0625) as f32;
    size = 0.0625;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        ax,
        ay,
        aw,
        ah,
        fcol,
        frow,
        fcol + size,
        frow + size,
        crate::src::cgame::cg_main::cgs.media.charsetShader,
    );
}
/*
==================
CG_DrawStringExt

Draws a multi-colored string with a drop shadow, optionally forcing
to a fixed color.

Coordinates are at 640 by 480 virtual resolution
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawStringExt(
    mut x: i32,
    mut y: i32,
    mut string: *const i8,
    mut setColor: *const f32,
    mut forceColor: crate::src::qcommon::q_shared::qboolean,
    mut shadow: crate::src::qcommon::q_shared::qboolean,
    mut charWidth: i32,
    mut charHeight: i32,
    mut maxChars: i32,
) {
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4]; // do them all!
    let mut s: *const i8 = 0 as *const i8;
    let mut xx: i32 = 0;
    let mut cnt: i32 = 0;
    if maxChars <= 0 {
        maxChars = 32767
    }
    // draw the drop shadow
    if shadow as u64 != 0 {
        color[2] = 0f32;
        color[1] = color[2];
        color[0] = color[1];
        color[3] = *setColor.offset(3);
        crate::src::cgame::cg_syscalls::trap_R_SetColor(color.as_mut_ptr());
        s = string;
        xx = x;
        cnt = 0;
        while *s as i32 != 0 && cnt < maxChars {
            if crate::src::qcommon::q_shared::Q_IsColorString(s) as u64 != 0 {
                s = s.offset(2)
            } else {
                CG_DrawChar(xx + 2, y + 2, charWidth, charHeight, *s as i32);
                cnt += 1;
                xx += charWidth;
                s = s.offset(1)
            }
        }
    }
    // draw the colored text
    s = string;
    xx = x;
    cnt = 0;
    crate::src::cgame::cg_syscalls::trap_R_SetColor(setColor);
    while *s as i32 != 0 && cnt < maxChars {
        if crate::src::qcommon::q_shared::Q_IsColorString(s) as u64 != 0 {
            if forceColor as u64 == 0 {
                crate::stdlib::memcpy(
                    color.as_mut_ptr() as *mut libc::c_void,
                    crate::src::qcommon::q_math::g_color_table
                        [(*s.offset(1) as i32 - '0' as i32 & 0x7) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<crate::src::qcommon::q_shared::vec4_t>(),
                );
                color[3] = *setColor.offset(3);
                crate::src::cgame::cg_syscalls::trap_R_SetColor(color.as_mut_ptr());
            }
            s = s.offset(2)
        } else {
            CG_DrawChar(xx, y, charWidth, charHeight, *s as i32);
            xx += charWidth;
            cnt += 1;
            s = s.offset(1)
        }
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawBigString(
    mut x: i32,
    mut y: i32,
    mut s: *const i8,
    mut alpha: f32,
) {
    let mut color: [f32; 4] = [0.; 4];
    color[2] = 1f32;
    color[1] = color[2];
    color[0] = color[1];
    color[3] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        crate::src::qcommon::q_shared::qfalse,
        crate::src::qcommon::q_shared::qtrue,
        16,
        16,
        0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawBigStringColor(
    mut x: i32,
    mut y: i32,
    mut s: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const f32,
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
        16,
        16,
        0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSmallString(
    mut x: i32,
    mut y: i32,
    mut s: *const i8,
    mut alpha: f32,
) {
    let mut color: [f32; 4] = [0.; 4];
    color[2] = 1f32;
    color[1] = color[2];
    color[0] = color[1];
    color[3] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        crate::src::qcommon::q_shared::qfalse,
        crate::src::qcommon::q_shared::qfalse,
        8,
        16,
        0,
    );
}
#[no_mangle]

pub unsafe extern "C" fn CG_DrawSmallStringColor(
    mut x: i32,
    mut y: i32,
    mut s: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const f32,
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qfalse,
        8,
        16,
        0,
    );
}
/*
=================
CG_DrawStrlen

Returns character count, skiping color escape codes
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawStrlen(mut str: *const i8) -> i32 {
    let mut s: *const i8 = str;
    let mut count: i32 = 0;
    while *s != 0 {
        if crate::src::qcommon::q_shared::Q_IsColorString(s) as u64 != 0 {
            s = s.offset(2)
        } else {
            count += 1;
            s = s.offset(1)
        }
    }
    return count;
}
/*
=============
CG_TileClearBox

This repeats a 64*64 tile graphic to fill the screen around a sized down
refresh window.
=============
*/

unsafe extern "C" fn CG_TileClearBox(
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut s1: f32 = 0.;
    let mut t1: f32 = 0.;
    let mut s2: f32 = 0.;
    let mut t2: f32 = 0.;
    s1 = (x as f64 / 64.0) as f32;
    t1 = (y as f64 / 64.0) as f32;
    s2 = ((x + w) as f64 / 64.0) as f32;
    t2 = ((y + h) as f64 / 64.0) as f32;
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        x as f32, y as f32, w as f32, h as f32, s1, t1, s2, t2, hShader,
    );
}
/*
==============
CG_TileClear

Clear around a sized down screen
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TileClear() {
    let mut top: i32 = 0;
    let mut bottom: i32 = 0;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    w = crate::src::cgame::cg_main::cgs.glconfig.vidWidth;
    h = crate::src::cgame::cg_main::cgs.glconfig.vidHeight;
    if crate::src::cgame::cg_main::cg.refdef.x == 0
        && crate::src::cgame::cg_main::cg.refdef.y == 0
        && crate::src::cgame::cg_main::cg.refdef.width == w
        && crate::src::cgame::cg_main::cg.refdef.height == h
    {
        return;
        // full screen rendering
    }
    top = crate::src::cgame::cg_main::cg.refdef.y;
    bottom = top + crate::src::cgame::cg_main::cg.refdef.height - 1;
    left = crate::src::cgame::cg_main::cg.refdef.x;
    right = left + crate::src::cgame::cg_main::cg.refdef.width - 1;
    // clear above view screen
    CG_TileClearBox(
        0,
        0,
        w,
        top,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
    // clear below view screen
    CG_TileClearBox(
        0,
        bottom,
        w,
        h - bottom,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
    // clear left of view screen
    CG_TileClearBox(
        0,
        top,
        left,
        bottom - top + 1,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
    // clear right of view screen
    CG_TileClearBox(
        right,
        top,
        w - right,
        bottom - top + 1,
        crate::src::cgame::cg_main::cgs.media.backTileShader,
    );
}
/*
================
CG_FadeColor
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FadeColor(mut startMsec: i32, mut totalMsec: i32) -> *mut f32 {
    static mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut t: i32 = 0;
    if startMsec == 0 {
        return 0 as *mut f32;
    }
    t = crate::src::cgame::cg_main::cg.time - startMsec;
    if t >= totalMsec {
        return 0 as *mut f32;
    }
    // fade out
    if totalMsec - t < 200 {
        color[3] = ((totalMsec - t) as f64 * 1.0 / 200f64) as crate::src::qcommon::q_shared::vec_t
    } else {
        color[3] = 1f32
    }
    color[2] = 1f32;
    color[1] = color[2];
    color[0] = color[1];
    return color.as_mut_ptr();
}
/*
================
CG_TeamColor
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TeamColor(mut team: i32) -> *mut f32 {
    static mut red: crate::src::qcommon::q_shared::vec4_t = [1f32, 0.2, 0.2, 1f32];
    static mut blue: crate::src::qcommon::q_shared::vec4_t = [0.2, 0.2, 1f32, 1f32];
    static mut other: crate::src::qcommon::q_shared::vec4_t = [1f32, 1f32, 1f32, 1f32];
    static mut spectator: crate::src::qcommon::q_shared::vec4_t = [0.7, 0.7, 0.7, 1f32];
    match team {
        1 => return red.as_mut_ptr(),
        2 => return blue.as_mut_ptr(),
        3 => return spectator.as_mut_ptr(),
        _ => return other.as_mut_ptr(),
    };
}
/*
=================
CG_GetColorForHealth
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_GetColorForHealth(
    mut health: i32,
    mut armor: i32,
    mut hcolor: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut count: i32 = 0;
    let mut max: i32 = 0;
    // calculate the total points of damage that can
    // be sustained at the current health / armor level
    if health <= 0 {
        let ref mut fresh0 = *hcolor.offset(2); // black
        *fresh0 = 0f32;
        let ref mut fresh1 = *hcolor.offset(1);
        *fresh1 = *fresh0;
        *hcolor.offset(0) = *fresh1;
        *hcolor.offset(3) = 1f32;
        return;
    }
    count = armor;
    max = (health as f64 * 0.66 / (1.0 - 0.66)) as i32;
    if max < count {
        count = max
    }
    health += count;
    // set the color based on health
    *hcolor.offset(0) = 1f32;
    *hcolor.offset(3) = 1f32;
    if health >= 100 {
        *hcolor.offset(2) = 1f32
    } else if health < 66 {
        *hcolor.offset(2) = 0f32
    } else {
        *hcolor.offset(2) = ((health - 66i32) as f64 / 33.0) as crate::src::qcommon::q_shared::vec_t
    }
    if health > 60 {
        *hcolor.offset(1) = 1f32
    } else if health < 30 {
        *hcolor.offset(1) = 0f32
    } else {
        *hcolor.offset(1) = ((health - 30i32) as f64 / 30.0) as crate::src::qcommon::q_shared::vec_t
    };
}
/*
=================
CG_ColorForHealth
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ColorForHealth(mut hcolor: *mut crate::src::qcommon::q_shared::vec_t) {
    CG_GetColorForHealth(
        (*crate::src::cgame::cg_main::cg.snap).ps.stats[crate::bg_public_h::STAT_HEALTH as usize],
        (*crate::src::cgame::cg_main::cg.snap).ps.stats[crate::bg_public_h::STAT_ARMOR as usize],
        hcolor,
    );
}
/*
=================
UI_DrawProportionalString2
=================
*/

static mut propMap: [[i32; 3]; 128] = [
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, 8],
    [11, 122, 7],
    [154, 181, 14],
    [55, 122, 17],
    [79, 122, 18],
    [101, 122, 23],
    [153, 122, 18],
    [9, 93, 7],
    [207, 122, 8],
    [230, 122, 9],
    [177, 122, 18],
    [30, 152, 18],
    [85, 181, 7],
    [34, 93, 11],
    [110, 181, 6],
    [130, 152, 14],
    [22, 64, 17],
    [41, 64, 12],
    [58, 64, 17],
    [78, 64, 18],
    [98, 64, 19],
    [120, 64, 18],
    [141, 64, 18],
    [204, 64, 16],
    [162, 64, 17],
    [182, 64, 18],
    [59, 181, 7],
    [35, 181, 7],
    [203, 152, 14],
    [56, 93, 14],
    [228, 152, 14],
    [177, 181, 18],
    [28, 122, 22],
    [5, 4, 18],
    [27, 4, 18],
    [48, 4, 18],
    [69, 4, 17],
    [90, 4, 13],
    [106, 4, 13],
    [121, 4, 18],
    [143, 4, 17],
    [164, 4, 8],
    [175, 4, 16],
    [195, 4, 18],
    [216, 4, 12],
    [230, 4, 23],
    [6, 34, 18],
    [27, 34, 18],
    [48, 34, 18],
    [68, 34, 18],
    [90, 34, 17],
    [110, 34, 18],
    [130, 34, 14],
    [146, 34, 18],
    [166, 34, 19],
    [185, 34, 29],
    [215, 34, 18],
    [234, 34, 18],
    [5, 64, 14],
    [60, 152, 7],
    [106, 151, 13],
    [83, 152, 7],
    [128, 122, 17],
    [4, 152, 21],
    [134, 181, 5],
    [5, 4, 18],
    [27, 4, 18],
    [48, 4, 18],
    [69, 4, 17],
    [90, 4, 13],
    [106, 4, 13],
    [121, 4, 18],
    [143, 4, 17],
    [164, 4, 8],
    [175, 4, 16],
    [195, 4, 18],
    [216, 4, 12],
    [230, 4, 23],
    [6, 34, 18],
    [27, 34, 18],
    [48, 34, 18],
    [68, 34, 18],
    [90, 34, 17],
    [110, 34, 18],
    [130, 34, 14],
    [146, 34, 18],
    [166, 34, 19],
    [185, 34, 29],
    [215, 34, 18],
    [234, 34, 18],
    [5, 64, 14],
    [153, 152, 13],
    [11, 181, 5],
    [180, 152, 13],
    [79, 93, 17],
    [0, 0, -(1)],
];

static mut propMapB: [[i32; 3]; 26] = [
    [11, 12, 33],
    [49, 12, 31],
    [85, 12, 31],
    [120, 12, 30],
    [156, 12, 21],
    [183, 12, 21],
    [207, 12, 32],
    [13, 55, 30],
    [49, 55, 13],
    [66, 55, 29],
    [101, 55, 31],
    [135, 55, 21],
    [158, 55, 40],
    [204, 55, 32],
    [12, 97, 31],
    [48, 97, 31],
    [82, 97, 30],
    [118, 97, 30],
    [153, 97, 30],
    [185, 97, 25],
    [213, 97, 30],
    [11, 139, 32],
    [42, 139, 51],
    [93, 139, 32],
    [126, 139, 31],
    [158, 139, 25],
];
/*
=================
UI_DrawBannerString
=================
*/

unsafe extern "C" fn UI_DrawBannerString2(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color as *const f32);
    ax = x as f32 * crate::src::cgame::cg_main::cgs.screenXScale
        + crate::src::cgame::cg_main::cgs.screenXBias;
    ay = y as f32 * crate::src::cgame::cg_main::cgs.screenYScale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127) as u8;
        if ch as i32 == ' ' as i32 {
            ax += (12f32 + 4f32) * crate::src::cgame::cg_main::cgs.screenXScale
        } else if ch as i32 >= 'A' as i32 && ch as i32 <= 'Z' as i32 {
            ch = (ch as i32 - 'A' as i32) as u8;
            fcol = propMapB[ch as usize][0] as f32 / 256.0;
            frow = propMapB[ch as usize][1] as f32 / 256.0;
            fwidth = propMapB[ch as usize][2] as f32 / 256.0;
            fheight = 36f32 / 256.0;
            aw = propMapB[ch as usize][2] as f32 * crate::src::cgame::cg_main::cgs.screenXScale;
            ah = 36f32 * crate::src::cgame::cg_main::cgs.screenYScale;
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                crate::src::cgame::cg_main::cgs.media.charsetPropB,
            );
            ax += aw + 4f32 * crate::src::cgame::cg_main::cgs.screenXScale
        }
        s = s.offset(1)
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawBannerString(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut style: i32,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: i32 = 0;
    let mut width: i32 = 0;
    let mut drawcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    // find the width of the drawn text
    s = str;
    width = 0;
    while *s != 0 {
        ch = *s as i32;
        if ch == ' ' as i32 {
            width += 12
        } else if ch >= 'A' as i32 && ch <= 'Z' as i32 {
            width += propMapB[(ch - 'A' as i32) as usize][2] + 4
        }
        s = s.offset(1)
    }
    width -= 4;
    match style & 0x7 {
        1 => x -= width / 2,
        2 => x -= width,
        0 | _ => {}
    }
    if style & 0x800 != 0 {
        drawcolor[2] = 0f32;
        drawcolor[1] = drawcolor[2];
        drawcolor[0] = drawcolor[1];
        drawcolor[3] = *color.offset(3);
        UI_DrawBannerString2(x + 2i32, y + 2i32, str, drawcolor.as_mut_ptr());
    }
    UI_DrawBannerString2(x, y, str, color);
}
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalStringWidth(mut str: *const i8) -> i32 {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: i32 = 0;
    let mut charWidth: i32 = 0;
    let mut width: i32 = 0;
    s = str;
    width = 0;
    while *s != 0 {
        ch = *s as i32 & 127;
        charWidth = propMap[ch as usize][2];
        if charWidth != -(1) {
            width += charWidth;
            width += 3
        }
        s = s.offset(1)
    }
    width -= 3;
    return width;
}

unsafe extern "C" fn UI_DrawProportionalString2(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
    mut sizeScale: f32,
    mut charset: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color as *const f32);
    ax = x as f32 * crate::src::cgame::cg_main::cgs.screenXScale
        + crate::src::cgame::cg_main::cgs.screenXBias;
    ay = y as f32 * crate::src::cgame::cg_main::cgs.screenYScale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127) as u8;
        if ch as i32 == ' ' as i32 {
            aw = 8f32 * crate::src::cgame::cg_main::cgs.screenXScale * sizeScale
        } else if propMap[ch as usize][2] != -(1) {
            fcol = propMap[ch as usize][0] as f32 / 256.0;
            frow = propMap[ch as usize][1] as f32 / 256.0;
            fwidth = propMap[ch as usize][2] as f32 / 256.0;
            fheight = 27f32 / 256.0;
            aw = propMap[ch as usize][2] as f32
                * crate::src::cgame::cg_main::cgs.screenXScale
                * sizeScale;
            ah = 27f32 * crate::src::cgame::cg_main::cgs.screenYScale * sizeScale;
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                charset,
            );
        } else {
            aw = 0f32
        }
        ax += aw + 3f32 * crate::src::cgame::cg_main::cgs.screenXScale * sizeScale;
        s = s.offset(1)
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
=================
UI_ProportionalSizeScale
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalSizeScale(mut style: i32) -> f32 {
    if style & 0x10 != 0 {
        return 0.75f32;
    }
    return 1f32;
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
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
/*
=================
UI_DrawProportionalString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawProportionalString(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut style: i32,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut drawcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut width: i32 = 0;
    let mut sizeScale: f32 = 0.;
    sizeScale = UI_ProportionalSizeScale(style);
    match style & 0x7 {
        1 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width / 2
        }
        2 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width
        }
        0 | _ => {}
    }
    if style & 0x800 != 0 {
        drawcolor[2] = 0f32;
        drawcolor[1] = drawcolor[2];
        drawcolor[0] = drawcolor[1];
        drawcolor[3] = *color.offset(3);
        UI_DrawProportionalString2(
            x + 2i32,
            y + 2i32,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetProp,
        );
    }
    if style & 0x2000 != 0 {
        drawcolor[0] = (*color.offset(0) as f64 * 0.8) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1] = (*color.offset(1) as f64 * 0.8) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[2] = (*color.offset(2) as f64 * 0.8) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[3] = *color.offset(3);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetProp,
        );
        return;
    }
    if style & 0x4000 != 0 {
        drawcolor[0] = (*color.offset(0) as f64 * 0.8) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1] = (*color.offset(1) as f64 * 0.8) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[2] = (*color.offset(2) as f64 * 0.8) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[3] = *color.offset(3);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            color,
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetProp,
        );
        drawcolor[0] = *color.offset(0);
        drawcolor[1] = *color.offset(1);
        drawcolor[2] = *color.offset(2);
        drawcolor[3] = (0.5
            + 0.5 * crate::stdlib::sin((crate::src::cgame::cg_main::cg.time / 75) as f64))
            as crate::src::qcommon::q_shared::vec_t;
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            crate::src::cgame::cg_main::cgs.media.charsetPropGlow,
        );
        return;
    }
    UI_DrawProportionalString2(
        x,
        y,
        str,
        color,
        sizeScale,
        crate::src::cgame::cg_main::cgs.media.charsetProp,
    );
}
