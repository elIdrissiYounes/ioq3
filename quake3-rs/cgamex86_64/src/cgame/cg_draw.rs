use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }
}

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::powerup_t;
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
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::PERS_ASSIST_COUNT;
pub use crate::bg_public_h::PERS_ATTACKEE_ARMOR;
pub use crate::bg_public_h::PERS_ATTACKER;
pub use crate::bg_public_h::PERS_CAPTURES;
pub use crate::bg_public_h::PERS_DEFEND_COUNT;
pub use crate::bg_public_h::PERS_EXCELLENT_COUNT;
pub use crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT;
pub use crate::bg_public_h::PERS_HITS;
pub use crate::bg_public_h::PERS_IMPRESSIVE_COUNT;
pub use crate::bg_public_h::PERS_KILLED;
pub use crate::bg_public_h::PERS_PLAYEREVENTS;
pub use crate::bg_public_h::PERS_RANK;
pub use crate::bg_public_h::PERS_SCORE;
pub use crate::bg_public_h::PERS_SPAWN_COUNT;
pub use crate::bg_public_h::PERS_TEAM;
pub use crate::bg_public_h::PM_DEAD;
pub use crate::bg_public_h::PM_FREEZE;
pub use crate::bg_public_h::PM_INTERMISSION;
pub use crate::bg_public_h::PM_NOCLIP;
pub use crate::bg_public_h::PM_NORMAL;
pub use crate::bg_public_h::PM_SPECTATOR;
pub use crate::bg_public_h::PM_SPINTERMISSION;
pub use crate::bg_public_h::PW_AMMOREGEN;
pub use crate::bg_public_h::PW_BATTLESUIT;
pub use crate::bg_public_h::PW_BLUEFLAG;
pub use crate::bg_public_h::PW_DOUBLER;
pub use crate::bg_public_h::PW_FLIGHT;
pub use crate::bg_public_h::PW_GUARD;
pub use crate::bg_public_h::PW_HASTE;
pub use crate::bg_public_h::PW_INVIS;
pub use crate::bg_public_h::PW_INVULNERABILITY;
pub use crate::bg_public_h::PW_NEUTRALFLAG;
pub use crate::bg_public_h::PW_NONE;
pub use crate::bg_public_h::PW_NUM_POWERUPS;
pub use crate::bg_public_h::PW_QUAD;
pub use crate::bg_public_h::PW_REDFLAG;
pub use crate::bg_public_h::PW_REGEN;
pub use crate::bg_public_h::PW_SCOUT;
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
pub use crate::bg_public_h::WEAPON_DROPPING;
pub use crate::bg_public_h::WEAPON_FIRING;
pub use crate::bg_public_h::WEAPON_RAISING;
pub use crate::bg_public_h::WEAPON_READY;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::BG_FindItemForPowerup;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
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
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_PrintStrlen;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
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
pub use crate::tr_types_h::stereoFrame_t;
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
pub use crate::tr_types_h::STEREO_CENTER;
pub use crate::tr_types_h::STEREO_LEFT;
pub use crate::tr_types_h::STEREO_RIGHT;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::itemInfo_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::weaponInfo_s;
pub use crate::cg_local_h::weaponInfo_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::src::cgame::cg_draw::stdlib_float_h::atof;
pub use crate::src::cgame::cg_drawtools::CG_AdjustFrom640;
pub use crate::src::cgame::cg_drawtools::CG_ColorForHealth;
pub use crate::src::cgame::cg_drawtools::CG_DrawBigString;
pub use crate::src::cgame::cg_drawtools::CG_DrawPic;
pub use crate::src::cgame::cg_drawtools::CG_DrawSmallString;
pub use crate::src::cgame::cg_drawtools::CG_DrawStringExt;
pub use crate::src::cgame::cg_drawtools::CG_DrawStrlen;
pub use crate::src::cgame::cg_drawtools::CG_FadeColor;
pub use crate::src::cgame::cg_drawtools::CG_FillRect;
pub use crate::src::cgame::cg_drawtools::CG_GetColorForHealth;
pub use crate::src::cgame::cg_drawtools::CG_TileClear;
pub use crate::src::cgame::cg_info::CG_DrawInformation;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_centertime;
pub use crate::src::cgame::cg_main::cg_crosshairHealth;
pub use crate::src::cgame::cg_main::cg_crosshairSize;
pub use crate::src::cgame::cg_main::cg_crosshairX;
pub use crate::src::cgame::cg_main::cg_crosshairY;
pub use crate::src::cgame::cg_main::cg_draw2D;
pub use crate::src::cgame::cg_main::cg_draw3dIcons;
pub use crate::src::cgame::cg_main::cg_drawAmmoWarning;
pub use crate::src::cgame::cg_main::cg_drawAttacker;
pub use crate::src::cgame::cg_main::cg_drawCrosshair;
pub use crate::src::cgame::cg_main::cg_drawCrosshairNames;
pub use crate::src::cgame::cg_main::cg_drawFPS;
pub use crate::src::cgame::cg_main::cg_drawIcons;
pub use crate::src::cgame::cg_main::cg_drawRewards;
pub use crate::src::cgame::cg_main::cg_drawSnapshot;
pub use crate::src::cgame::cg_main::cg_drawStatus;
pub use crate::src::cgame::cg_main::cg_drawTeamOverlay;
pub use crate::src::cgame::cg_main::cg_drawTimer;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_items;
pub use crate::src::cgame::cg_main::cg_lagometer;
pub use crate::src::cgame::cg_main::cg_nopredict;
pub use crate::src::cgame::cg_main::cg_synchronousClients;
pub use crate::src::cgame::cg_main::cg_teamChatHeight;
pub use crate::src::cgame::cg_main::cg_teamChatTime;
pub use crate::src::cgame::cg_main::cg_weapons;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_predict::CG_PointContents;
pub use crate::src::cgame::cg_predict::CG_Trace;
pub use crate::src::cgame::cg_scoreboard::CG_DrawOldScoreboard;
pub use crate::src::cgame::cg_scoreboard::CG_DrawTourneyScoreboard;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::cgame::cg_syscalls::trap_GetCurrentCmdNumber;
pub use crate::src::cgame::cg_syscalls::trap_GetUserCmd;
pub use crate::src::cgame::cg_syscalls::trap_Milliseconds;
pub use crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_ClearScene;
pub use crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic;
pub use crate::src::cgame::cg_syscalls::trap_R_ModelBounds;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterShader;
pub use crate::src::cgame::cg_syscalls::trap_R_RenderScene;
pub use crate::src::cgame::cg_syscalls::trap_R_SetColor;
pub use crate::src::cgame::cg_syscalls::trap_S_StartLocalSound;
pub use crate::src::cgame::cg_weapons::CG_DrawWeaponSelect;
pub use crate::src::cgame::cg_weapons::CG_RegisterItemVisuals;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lagometer_t {
    pub frameSamples: [i32; 128],
    pub frameCount: i32,
    pub snapshotFlags: [i32; 128],
    pub snapshotSamples: [i32; 128],
    pub snapshotCount: i32,
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
// cg_draw.c -- draw all of the graphical elements during
// active (after loading) gameplay
#[no_mangle]

pub static mut drawTeamOverlayModificationCount: i32 = -(1);
#[no_mangle]

pub static mut sortedTeamPlayers: [i32; 32] = [0; 32];
#[no_mangle]

pub static mut numSortedTeamPlayers: i32 = 0;
#[no_mangle]

pub static mut systemChat: [i8; 256] = [0; 256];
#[no_mangle]

pub static mut teamChat1: [i8; 256] = [0; 256];
#[no_mangle]

pub static mut teamChat2: [i8; 256] = [0; 256];
/*
==============
CG_DrawField

Draws large numbers for status bar and powerups
==============
*/

unsafe extern "C" fn CG_DrawField(mut x: i32, mut y: i32, mut width: i32, mut value: i32) {
    let mut num: [i8; 16] = [0; 16];
    let mut ptr: *mut i8 = 0 as *mut i8;
    let mut l: i32 = 0;
    let mut frame: i32 = 0;
    if width < 1 {
        return;
    }
    // draw number string
    if width > 5 {
        width = 5
    }
    match width {
        1 => {
            value = if value > 9 { 9 } else { value };
            value = if value < 0 { 0 } else { value }
        }
        2 => {
            value = if value > 99 { 99 } else { value };
            value = if value < -(9) { -(9) } else { value }
        }
        3 => {
            value = if value > 999 { 999 } else { value };
            value = if value < -(99) { -(99) } else { value }
        }
        4 => {
            value = if value > 9999 { 9999 } else { value };
            value = if value < -(999) { -(999) } else { value }
        }
        _ => {}
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        num.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 16]>() as i32,
        b"%i\x00" as *const u8 as *const i8,
        value,
    );
    l = crate::stdlib::strlen(num.as_mut_ptr()) as i32;
    if l > width {
        l = width
    }
    x += 2 + 32 * (width - l);
    ptr = num.as_mut_ptr();
    while *ptr as i32 != 0 && l != 0 {
        if *ptr as i32 == '-' as i32 {
            frame = 10
        } else {
            frame = *ptr as i32 - '0' as i32
        }
        crate::src::cgame::cg_drawtools::CG_DrawPic(
            x as f32,
            y as f32,
            32f32,
            48f32,
            crate::src::cgame::cg_main::cgs.media.numberShaders[frame as usize],
        );
        x += 32;
        ptr = ptr.offset(1);
        l -= 1
    }
}
// MISSIONPACK
/*
================
CG_Draw3DModel

================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Draw3DModel(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut model: crate::src::qcommon::q_shared::qhandle_t,
    mut skin: crate::src::qcommon::q_shared::qhandle_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut refdef: crate::tr_types_h::refdef_t = crate::tr_types_h::refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        text: [[0; 32]; 8],
    }; // no stencil shadows
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    if crate::src::cgame::cg_main::cg_draw3dIcons.integer == 0
        || crate::src::cgame::cg_main::cg_drawIcons.integer == 0
    {
        return;
    }
    crate::src::cgame::cg_drawtools::CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    crate::stdlib::memset(
        &mut refdef as *mut crate::tr_types_h::refdef_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refdef_t>(),
    );
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    crate::src::qcommon::q_math::AnglesToAxis(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis.as_mut_ptr(),
    );
    ent.origin[0] = *origin.offset(0);
    ent.origin[1] = *origin.offset(1);
    ent.origin[2] = *origin.offset(2);
    ent.hModel = model;
    ent.customSkin = skin;
    ent.renderfx = 0x40;
    refdef.rdflags = 0x1;
    crate::src::qcommon::q_math::AxisClear(refdef.viewaxis.as_mut_ptr());
    refdef.fov_x = 30f32;
    refdef.fov_y = 30f32;
    refdef.x = x as i32;
    refdef.y = y as i32;
    refdef.width = w as i32;
    refdef.height = h as i32;
    refdef.time = crate::src::cgame::cg_main::cg.time;
    crate::src::cgame::cg_syscalls::trap_R_ClearScene();
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
    crate::src::cgame::cg_syscalls::trap_R_RenderScene(&mut refdef);
}
/*
================
CG_DrawHead

Used for both the status bar and the scoreboard
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawHead(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut clientNum: i32,
    mut headAngles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut cm: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut len: f32 = 0.;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::cg_local_h::clientInfo_t;
    if crate::src::cgame::cg_main::cg_draw3dIcons.integer != 0 {
        cm = (*ci).headModel;
        if cm == 0 {
            return;
        }
        // offset the origin y and z to center the head
        crate::src::cgame::cg_syscalls::trap_R_ModelBounds(
            cm,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        origin[2] = (-0.5 * (mins[2] + maxs[2]) as f64) as crate::src::qcommon::q_shared::vec_t;
        origin[1] = (0.5 * (mins[1] + maxs[1]) as f64) as crate::src::qcommon::q_shared::vec_t;
        // calculate distance so the head nearly fills the box
        // assume heads are taller than wide
        len = (0.7 * (maxs[2] - mins[2]) as f64) as f32; // len / tan( fov/2 )
        origin[0] = (len as f64 / 0.268) as crate::src::qcommon::q_shared::vec_t;
        // allow per-model tweaking
        origin[0] = origin[0] + (*ci).headOffset[0];
        origin[1] = origin[1] + (*ci).headOffset[1];
        origin[2] = origin[2] + (*ci).headOffset[2];
        CG_Draw3DModel(
            x,
            y,
            w,
            h,
            (*ci).headModel,
            (*ci).headSkin,
            origin.as_mut_ptr(),
            headAngles,
        );
    } else if crate::src::cgame::cg_main::cg_drawIcons.integer != 0 {
        crate::src::cgame::cg_drawtools::CG_DrawPic(x, y, w, h, (*ci).modelIcon);
    }
    // if they are deferred, draw a cross out
    if (*ci).deferred as u64 != 0 {
        crate::src::cgame::cg_drawtools::CG_DrawPic(
            x,
            y,
            w,
            h,
            crate::src::cgame::cg_main::cgs.media.deferShader,
        );
    };
}
/*
================
CG_DrawFlagModel

Used for both the status bar and the scoreboard
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawFlagModel(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut team: i32,
    mut force2D: crate::src::qcommon::q_shared::qboolean,
) {
    let mut cm: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut len: f32 = 0.;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut handle: crate::src::qcommon::q_shared::qhandle_t = 0;
    if force2D as u64 == 0 && crate::src::cgame::cg_main::cg_draw3dIcons.integer != 0 {
        angles[2] = 0f32;
        angles[1] = angles[2];
        angles[0] = angles[1];
        cm = crate::src::cgame::cg_main::cgs.media.redFlagModel;
        // offset the origin y and z to center the flag
        crate::src::cgame::cg_syscalls::trap_R_ModelBounds(
            cm,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        origin[2] = (-0.5 * (mins[2] + maxs[2]) as f64) as crate::src::qcommon::q_shared::vec_t;
        origin[1] = (0.5 * (mins[1] + maxs[1]) as f64) as crate::src::qcommon::q_shared::vec_t;
        // calculate distance so the flag nearly fills the box
        // assume heads are taller than wide
        len = (0.5 * (maxs[2] - mins[2]) as f64) as f32; // len / tan( fov/2 )
        origin[0] = (len as f64 / 0.268) as crate::src::qcommon::q_shared::vec_t;
        angles[1] = (60f64
            * crate::stdlib::sin(crate::src::cgame::cg_main::cg.time as f64 / 2000.0))
            as crate::src::qcommon::q_shared::vec_t;
        if team == crate::bg_public_h::TEAM_RED as i32 {
            handle = crate::src::cgame::cg_main::cgs.media.redFlagModel
        } else if team == crate::bg_public_h::TEAM_BLUE as i32 {
            handle = crate::src::cgame::cg_main::cgs.media.blueFlagModel
        } else if team == crate::bg_public_h::TEAM_FREE as i32 {
            handle = crate::src::cgame::cg_main::cgs.media.neutralFlagModel
        } else {
            return;
        }
        CG_Draw3DModel(
            x,
            y,
            w,
            h,
            handle,
            0i32,
            origin.as_mut_ptr(),
            angles.as_mut_ptr(),
        );
    } else if crate::src::cgame::cg_main::cg_drawIcons.integer != 0 {
        let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
        if team == crate::bg_public_h::TEAM_RED as i32 {
            item = crate::src::game::bg_misc::BG_FindItemForPowerup(crate::bg_public_h::PW_REDFLAG)
        } else if team == crate::bg_public_h::TEAM_BLUE as i32 {
            item = crate::src::game::bg_misc::BG_FindItemForPowerup(crate::bg_public_h::PW_BLUEFLAG)
        } else if team == crate::bg_public_h::TEAM_FREE as i32 {
            item =
                crate::src::game::bg_misc::BG_FindItemForPowerup(crate::bg_public_h::PW_NEUTRALFLAG)
        } else {
            return;
        }
        if !item.is_null() {
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                x,
                y,
                w,
                h,
                crate::src::cgame::cg_main::cg_items[item
                    .wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr())
                    as usize]
                    .icon,
            );
        }
    };
}
/*
================
CG_DrawStatusBarHead

================
*/

unsafe extern "C" fn CG_DrawStatusBarHead(mut x: f32) {
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut size: f32 = 0.;
    let mut stretch: f32 = 0.;
    let mut frac: f32 = 0.;
    angles[2] = 0f32;
    angles[1] = angles[2];
    angles[0] = angles[1];
    if crate::src::cgame::cg_main::cg.damageTime != 0.
        && crate::src::cgame::cg_main::cg.time as f32 - crate::src::cgame::cg_main::cg.damageTime
            < 500f32
    {
        frac = (crate::src::cgame::cg_main::cg.time as f32
            - crate::src::cgame::cg_main::cg.damageTime)
            / 500f32;
        size = (48f64 * 1.25 * (1.5 - frac as f64 * 0.5)) as f32;
        stretch = (size as f64 - 48f64 * 1.25) as f32;
        // kick in the direction of damage
        x = (x as f64
            - (stretch as f64 * 0.5
                + (crate::src::cgame::cg_main::cg.damageX * stretch) as f64 * 0.5))
            as f32;
        crate::src::cgame::cg_main::cg.headStartYaw =
            180f32 + crate::src::cgame::cg_main::cg.damageX * 45f32;
        crate::src::cgame::cg_main::cg.headEndYaw = (180f64
            + 20f64
                * crate::stdlib::cos(
                    2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                        * 3.14159265358979323846,
                )) as f32;
        crate::src::cgame::cg_main::cg.headEndPitch = (5f64
            * crate::stdlib::cos(
                2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                    * 3.14159265358979323846,
            )) as f32;
        crate::src::cgame::cg_main::cg.headStartTime = crate::src::cgame::cg_main::cg.time;
        crate::src::cgame::cg_main::cg.headEndTime =
            ((crate::src::cgame::cg_main::cg.time + 100) as f32
                + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 2000f32) as i32
    } else {
        if crate::src::cgame::cg_main::cg.time >= crate::src::cgame::cg_main::cg.headEndTime {
            // select a new head angle
            crate::src::cgame::cg_main::cg.headStartYaw = crate::src::cgame::cg_main::cg.headEndYaw;
            crate::src::cgame::cg_main::cg.headStartPitch =
                crate::src::cgame::cg_main::cg.headEndPitch;
            crate::src::cgame::cg_main::cg.headStartTime =
                crate::src::cgame::cg_main::cg.headEndTime;
            crate::src::cgame::cg_main::cg.headEndTime = ((crate::src::cgame::cg_main::cg.time
                + 100) as f32
                + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 2000f32)
                as i32;
            crate::src::cgame::cg_main::cg.headEndYaw = (180f64
                + 20f64
                    * crate::stdlib::cos(
                        2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                            * 3.14159265358979323846,
                    )) as f32;
            crate::src::cgame::cg_main::cg.headEndPitch = (5f64
                * crate::stdlib::cos(
                    2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                        * 3.14159265358979323846,
                )) as f32
        }
        size = (48f64 * 1.25) as f32
    }
    // if the server was frozen for a while we may have a bad head start time
    if crate::src::cgame::cg_main::cg.headStartTime > crate::src::cgame::cg_main::cg.time {
        crate::src::cgame::cg_main::cg.headStartTime = crate::src::cgame::cg_main::cg.time
    }
    frac = (crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.headStartTime)
        as f32
        / (crate::src::cgame::cg_main::cg.headEndTime
            - crate::src::cgame::cg_main::cg.headStartTime) as f32;
    frac = frac * frac * (3f32 - 2f32 * frac);
    angles[1] = crate::src::cgame::cg_main::cg.headStartYaw
        + (crate::src::cgame::cg_main::cg.headEndYaw - crate::src::cgame::cg_main::cg.headStartYaw)
            * frac;
    angles[0] = crate::src::cgame::cg_main::cg.headStartPitch
        + (crate::src::cgame::cg_main::cg.headEndPitch
            - crate::src::cgame::cg_main::cg.headStartPitch)
            * frac;
    CG_DrawHead(
        x,
        480f32 - size,
        size,
        size,
        (*crate::src::cgame::cg_main::cg.snap).ps.clientNum,
        angles.as_mut_ptr(),
    );
}
// MISSIONPACK
/*
================
CG_DrawStatusBarFlag

================
*/

unsafe extern "C" fn CG_DrawStatusBarFlag(mut x: f32, mut team: i32) {
    CG_DrawFlagModel(
        x,
        (480i32 - 48) as f32,
        48f32,
        48f32,
        team,
        crate::src::qcommon::q_shared::qfalse,
    );
}
// MISSIONPACK
/*
================
CG_DrawTeamBackground

================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawTeamBackground(
    mut x: i32,
    mut y: i32,
    mut w: i32,
    mut h: i32,
    mut alpha: f32,
    mut team: i32,
) {
    let mut hcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    hcolor[3] = alpha;
    if team == crate::bg_public_h::TEAM_RED as i32 {
        hcolor[0] = 1f32;
        hcolor[1] = 0f32;
        hcolor[2] = 0f32
    } else if team == crate::bg_public_h::TEAM_BLUE as i32 {
        hcolor[0] = 0f32;
        hcolor[1] = 0f32;
        hcolor[2] = 1f32
    } else {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(hcolor.as_mut_ptr());
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        x as f32,
        y as f32,
        w as f32,
        h as f32,
        crate::src::cgame::cg_main::cgs.media.teamStatusBar,
    );
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
================
CG_DrawStatusBar

================
*/

unsafe extern "C" fn CG_DrawStatusBar() {
    let mut color: i32 = 0; // health > 100
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    let mut value: i32 = 0;
    let mut hcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    static mut colors: [[f32; 4]; 4] = [
        [1.0, 0.69, 0.0, 1.0],
        [1.0, 0.2, 0.2, 1.0],
        [0.5, 0.5, 0.5, 1.0],
        [1.0, 1.0, 1.0, 1.0],
    ];
    if crate::src::cgame::cg_main::cg_drawStatus.integer == 0 {
        return;
    }
    // draw the team background
    CG_DrawTeamBackground(
        0,
        420,
        640,
        60,
        0.33,
        (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as usize],
    );
    cent = &mut *crate::src::cgame::cg_main::cg_entities
        .as_mut_ptr()
        .offset((*crate::src::cgame::cg_main::cg.snap).ps.clientNum as isize)
        as *mut crate::cg_local_h::centity_t;
    ps = &mut (*crate::src::cgame::cg_main::cg.snap).ps;
    angles[2] = 0f32;
    angles[1] = angles[2];
    angles[0] = angles[1];
    // draw any 3D icons first, so the changes back to 2D are minimized
    if (*cent).currentState.weapon != 0
        && crate::src::cgame::cg_main::cg_weapons[(*cent).currentState.weapon as usize].ammoModel
            != 0
    {
        origin[0] = 70f32;
        origin[1] = 0f32;
        origin[2] = 0f32;
        angles[1] = (90f64
            + 20f64 * crate::stdlib::sin(crate::src::cgame::cg_main::cg.time as f64 / 1000.0))
            as crate::src::qcommon::q_shared::vec_t;
        CG_Draw3DModel(
            (32i32 * 3i32 + 4i32) as f32,
            432f32,
            48f32,
            48f32,
            crate::src::cgame::cg_main::cg_weapons[(*cent).currentState.weapon as usize].ammoModel,
            0i32,
            origin.as_mut_ptr(),
            angles.as_mut_ptr(),
        );
    }
    CG_DrawStatusBarHead((185i32 + 32 * 3 + 4) as f32);
    if crate::src::cgame::cg_main::cg.predictedPlayerState.powerups
        [crate::bg_public_h::PW_REDFLAG as usize]
        != 0
    {
        CG_DrawStatusBarFlag(
            (185i32 + 32i32 * 3i32 + 4i32 + 48i32) as f32,
            crate::bg_public_h::TEAM_RED as i32,
        );
    } else if crate::src::cgame::cg_main::cg.predictedPlayerState.powerups
        [crate::bg_public_h::PW_BLUEFLAG as usize]
        != 0
    {
        CG_DrawStatusBarFlag(
            (185i32 + 32i32 * 3i32 + 4i32 + 48i32) as f32,
            crate::bg_public_h::TEAM_BLUE as i32,
        );
    } else if crate::src::cgame::cg_main::cg.predictedPlayerState.powerups
        [crate::bg_public_h::PW_NEUTRALFLAG as usize]
        != 0
    {
        CG_DrawStatusBarFlag(
            (185i32 + 32i32 * 3i32 + 4i32 + 48i32) as f32,
            crate::bg_public_h::TEAM_FREE as i32,
        );
    }
    if (*ps).stats[crate::bg_public_h::STAT_ARMOR as usize] != 0 {
        origin[0] = 90f32;
        origin[1] = 0f32;
        origin[2] = -10f32;
        angles[1] = (((crate::src::cgame::cg_main::cg.time & 2047) * 360) as f64 / 2048.0)
            as crate::src::qcommon::q_shared::vec_t;
        CG_Draw3DModel(
            (370i32 + 32i32 * 3i32 + 4i32) as f32,
            432f32,
            48f32,
            48f32,
            crate::src::cgame::cg_main::cgs.media.armorModel,
            0i32,
            origin.as_mut_ptr(),
            angles.as_mut_ptr(),
        );
    }
    //
    // ammo
    //
    if (*cent).currentState.weapon != 0 {
        value = (*ps).ammo[(*cent).currentState.weapon as usize];
        if value > -(1) {
            if crate::src::cgame::cg_main::cg
                .predictedPlayerState
                .weaponstate
                == crate::bg_public_h::WEAPON_FIRING as i32
                && crate::src::cgame::cg_main::cg
                    .predictedPlayerState
                    .weaponTime
                    > 100
            {
                // draw as dark grey when reloading
                color = 2
            // dark grey
            } else if value >= 0 {
                color = 0
            // green
            } else {
                color = 1
                // red
            }
            crate::src::cgame::cg_syscalls::trap_R_SetColor(colors[color as usize].as_mut_ptr());
            CG_DrawField(0, 432, 3, value);
            crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
            // if we didn't draw a 3D icon, draw a 2D icon for ammo
            if crate::src::cgame::cg_main::cg_draw3dIcons.integer == 0
                && crate::src::cgame::cg_main::cg_drawIcons.integer != 0
            {
                let mut icon: crate::src::qcommon::q_shared::qhandle_t = 0;
                icon = crate::src::cgame::cg_main::cg_weapons
                    [crate::src::cgame::cg_main::cg.predictedPlayerState.weapon as usize]
                    .ammoIcon;
                if icon != 0 {
                    crate::src::cgame::cg_drawtools::CG_DrawPic(
                        (32i32 * 3i32 + 4i32) as f32,
                        432f32,
                        48f32,
                        48f32,
                        icon,
                    );
                }
            }
        }
    }
    //
    // health
    //
    value = (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize];
    if value > 100 {
        crate::src::cgame::cg_syscalls::trap_R_SetColor(colors[3usize].as_mut_ptr());
    // white
    } else if value > 25 {
        crate::src::cgame::cg_syscalls::trap_R_SetColor(colors[0usize].as_mut_ptr());
    // green
    } else if value > 0 {
        color = crate::src::cgame::cg_main::cg.time >> 8 & 1; // flash
        crate::src::cgame::cg_syscalls::trap_R_SetColor(colors[color as usize].as_mut_ptr());
    } else {
        crate::src::cgame::cg_syscalls::trap_R_SetColor(colors[1usize].as_mut_ptr());
        // red
    }
    // stretch the health up when taking damage
    CG_DrawField(185, 432, 3, value);
    crate::src::cgame::cg_drawtools::CG_ColorForHealth(hcolor.as_mut_ptr());
    crate::src::cgame::cg_syscalls::trap_R_SetColor(hcolor.as_mut_ptr());
    //
    // armor
    //
    value = (*ps).stats[crate::bg_public_h::STAT_ARMOR as usize];
    if value > 0 {
        crate::src::cgame::cg_syscalls::trap_R_SetColor(colors[0].as_mut_ptr());
        CG_DrawField(370, 432, 3, value);
        crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
        // if we didn't draw a 3D icon, draw a 2D icon for armor
        if crate::src::cgame::cg_main::cg_draw3dIcons.integer == 0
            && crate::src::cgame::cg_main::cg_drawIcons.integer != 0
        {
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                (370i32 + 32i32 * 3i32 + 4i32) as f32,
                432f32,
                48f32,
                48f32,
                crate::src::cgame::cg_main::cgs.media.armorIcon,
            );
        }
    };
}
/*
===========================================================================================

  UPPER RIGHT CORNER

===========================================================================================
*/
/*
================
CG_DrawAttacker

================
*/

unsafe extern "C" fn CG_DrawAttacker(mut y: f32) -> f32 {
    let mut t: i32 = 0;
    let mut size: f32 = 0.;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut info: *const i8 = 0 as *const i8;
    let mut name: *const i8 = 0 as *const i8;
    let mut clientNum: i32 = 0;
    if crate::src::cgame::cg_main::cg.predictedPlayerState.stats
        [crate::bg_public_h::STAT_HEALTH as usize]
        <= 0
    {
        return y;
    }
    if crate::src::cgame::cg_main::cg.attackerTime == 0 {
        return y;
    }
    clientNum = crate::src::cgame::cg_main::cg
        .predictedPlayerState
        .persistant[crate::bg_public_h::PERS_ATTACKER as usize];
    if clientNum < 0
        || clientNum >= 64
        || clientNum == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum
    {
        return y;
    }
    if crate::src::cgame::cg_main::cgs.clientinfo[clientNum as usize].infoValid as u64 == 0 {
        crate::src::cgame::cg_main::cg.attackerTime = 0;
        return y;
    }
    t = crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.attackerTime;
    if t > 10000 {
        crate::src::cgame::cg_main::cg.attackerTime = 0;
        return y;
    }
    size = (48f64 * 1.25) as f32;
    angles[0] = 0f32;
    angles[1] = 180f32;
    angles[2] = 0f32;
    CG_DrawHead(640f32 - size, y, size, size, clientNum, angles.as_mut_ptr());
    info = crate::src::cgame::cg_main::CG_ConfigString(32 + 256 + 256 + clientNum);
    name =
        crate::src::qcommon::q_shared::Info_ValueForKey(info, b"n\x00" as *const u8 as *const i8);
    y += size;
    crate::src::cgame::cg_drawtools::CG_DrawBigString(
        640 - crate::src::qcommon::q_shared::Q_PrintStrlen(name) * 16,
        y as i32,
        name,
        0.5,
    );
    return y + 16f32 + 2f32;
}
/*
==================
CG_DrawSnapshot
==================
*/

unsafe extern "C" fn CG_DrawSnapshot(mut y: f32) -> f32 {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut w: i32 = 0;
    s = crate::src::qcommon::q_shared::va(
        b"time:%i snap:%i cmd:%i\x00" as *const u8 as *mut i8,
        (*crate::src::cgame::cg_main::cg.snap).serverTime,
        crate::src::cgame::cg_main::cg.latestSnapshotNum,
        crate::src::cgame::cg_main::cgs.serverCommandSequence,
    );
    w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16;
    crate::src::cgame::cg_drawtools::CG_DrawBigString(635 - w, (y + 2f32) as i32, s, 1.0);
    return y + 16f32 + 4f32;
}

unsafe extern "C" fn CG_DrawFPS(mut y: f32) -> f32 {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut w: i32 = 0;
    static mut previousTimes: [i32; 4] = [0; 4];
    static mut index: i32 = 0;
    let mut _i: i32 = 0;
    let mut total: i32 = 0;
    let mut fps: i32 = 0;
    static mut previous: i32 = 0;
    let mut t: i32 = 0;
    let mut frameTime: i32 = 0;
    // don't use serverTime, because that will be drifting to
    // correct for internet lag changes, timescales, timedemos, etc
    t = crate::src::cgame::cg_syscalls::trap_Milliseconds();
    frameTime = t - previous;
    previous = t;
    previousTimes[(index % 4) as usize] = frameTime;
    index += 1;
    if index > 4 {
        // average multiple frames together to smooth changes out a bit
        total = 0;

        for i in 0..4 {
            total += previousTimes[i as usize];
        }
        if total == 0 {
            total = 1
        }
        fps = 1000 * 4 / total;
        s = crate::src::qcommon::q_shared::va(b"%ifps\x00" as *const u8 as *mut i8, fps);
        w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16;
        crate::src::cgame::cg_drawtools::CG_DrawBigString(635i32 - w, (y + 2f32) as i32, s, 1.0f32);
    }
    return y + 16f32 + 4f32;
}
/*
=================
CG_DrawTimer
=================
*/

unsafe extern "C" fn CG_DrawTimer(mut y: f32) -> f32 {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut w: i32 = 0;
    let mut mins: i32 = 0;
    let mut seconds: i32 = 0;
    let mut tens: i32 = 0;
    let mut msec: i32 = 0;
    msec = crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cgs.levelStartTime;
    seconds = msec / 1000;
    mins = seconds / 60;
    seconds -= mins * 60;
    tens = seconds / 10;
    seconds -= tens * 10;
    s = crate::src::qcommon::q_shared::va(
        b"%i:%i%i\x00" as *const u8 as *mut i8,
        mins,
        tens,
        seconds,
    );
    w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16;
    crate::src::cgame::cg_drawtools::CG_DrawBigString(635 - w, (y + 2f32) as i32, s, 1.0);
    return y + 16f32 + 4f32;
}
/*
=================
CG_DrawTeamOverlay
=================
*/

unsafe extern "C" fn CG_DrawTeamOverlay(
    mut y: f32,
    mut right: crate::src::qcommon::q_shared::qboolean,
    mut upper: crate::src::qcommon::q_shared::qboolean,
) -> f32 {
    let mut x: i32 = 0;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    let mut xx: i32 = 0;
    let mut i: i32 = 0;
    let mut _j: i32 = 0;
    let mut len: i32 = 0;
    let mut p: *const i8 = 0 as *const i8;
    let mut hcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut pwidth: i32 = 0;
    let mut lwidth: i32 = 0;
    let mut plyrs: i32 = 0;
    let mut st: [i8; 16] = [0; 16];
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut ret_y: i32 = 0;
    let mut count: i32 = 0;
    if crate::src::cgame::cg_main::cg_drawTeamOverlay.integer == 0 {
        return y;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.persistant[crate::bg_public_h::PERS_TEAM as usize]
        != crate::bg_public_h::TEAM_RED as i32
        && (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as usize]
            != crate::bg_public_h::TEAM_BLUE as i32
    {
        return y;
        // Not on any team
    }
    plyrs = 0;
    // max player name width
    pwidth = 0;
    count = if numSortedTeamPlayers > 8 {
        8
    } else {
        numSortedTeamPlayers
    };
    i = 0;
    while i < count {
        ci = crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset(sortedTeamPlayers[i as usize] as isize);
        if (*ci).infoValid != 0
            && (*ci).team
                == (*crate::src::cgame::cg_main::cg.snap).ps.persistant
                    [crate::bg_public_h::PERS_TEAM as usize] as u32
        {
            plyrs += 1;
            len = crate::src::cgame::cg_drawtools::CG_DrawStrlen((*ci).name.as_mut_ptr());
            if len > pwidth {
                pwidth = len
            }
        }
        i += 1
    }
    if plyrs == 0 {
        return y;
    }
    if pwidth > 12 {
        pwidth = 12
    }
    // max location name width
    lwidth = 0; // if ( cg.snap->ps.persistant[PERS_TEAM] == TEAM_BLUE )
    i = 1;
    while i < 64 {
        p = crate::src::cgame::cg_main::CG_ConfigString(32 + 256 + 256 + 64 + i);
        if !p.is_null() && *p as i32 != 0 {
            len = crate::src::cgame::cg_drawtools::CG_DrawStrlen(p);
            if len > lwidth {
                lwidth = len
            }
        }
        i += 1
    }
    if lwidth > 16 {
        lwidth = 16
    }
    w = (pwidth + lwidth + 4 + 7) * 8;
    if right as u64 != 0 {
        x = 640 - w
    } else {
        x = 0
    }
    h = plyrs * (16 / 2);
    if upper as u64 != 0 {
        ret_y = (y + h as f32) as i32
    } else {
        y -= h as f32;
        ret_y = y as i32
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.persistant[crate::bg_public_h::PERS_TEAM as usize]
        == crate::bg_public_h::TEAM_RED as i32
    {
        hcolor[0] = 1.0;
        hcolor[1] = 0.0;
        hcolor[2] = 0.0;
        hcolor[3] = 0.33
    } else {
        hcolor[0] = 0.0;
        hcolor[1] = 0.0;
        hcolor[2] = 1.0;
        hcolor[3] = 0.33
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(hcolor.as_mut_ptr());
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        x as f32,
        y,
        w as f32,
        h as f32,
        crate::src::cgame::cg_main::cgs.media.teamStatusBar,
    );
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
    i = 0;
    while i < count {
        ci = crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset(sortedTeamPlayers[i as usize] as isize);
        if (*ci).infoValid != 0
            && (*ci).team
                == (*crate::src::cgame::cg_main::cg.snap).ps.persistant
                    [crate::bg_public_h::PERS_TEAM as usize] as u32
        {
            hcolor[3] = 1f32;
            hcolor[2] = hcolor[3];
            hcolor[1] = hcolor[2];
            hcolor[0] = hcolor[1];
            xx = x + 8;
            crate::src::cgame::cg_drawtools::CG_DrawStringExt(
                xx,
                y as i32,
                (*ci).name.as_mut_ptr(),
                hcolor.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse,
                crate::src::qcommon::q_shared::qfalse,
                8,
                16 / 2,
                12,
            );
            if lwidth != 0 {
                p = crate::src::cgame::cg_main::CG_ConfigString(
                    32 + 256 + 256 + 64 + (*ci).location,
                );
                if p.is_null() || *p == 0 {
                    p = b"unknown\x00" as *const u8 as *const i8
                }
                //				len = CG_DrawStrlen(p);
                //				if (len > lwidth)
                //					len = lwidth;
                //				xx = x + TINYCHAR_WIDTH * 2 + TINYCHAR_WIDTH * pwidth +
                //					((lwidth/2 - len/2) * TINYCHAR_WIDTH);
                xx = x + 8 * 2 + 8 * pwidth;
                crate::src::cgame::cg_drawtools::CG_DrawStringExt(
                    xx,
                    y as i32,
                    p,
                    hcolor.as_mut_ptr(),
                    crate::src::qcommon::q_shared::qfalse,
                    crate::src::qcommon::q_shared::qfalse,
                    8i32,
                    16i32 / 2i32,
                    16i32,
                );
            }
            crate::src::cgame::cg_drawtools::CG_GetColorForHealth(
                (*ci).health,
                (*ci).armor,
                hcolor.as_mut_ptr(),
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                st.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 16]>() as i32,
                b"%3i %3i\x00" as *const u8 as *const i8,
                (*ci).health,
                (*ci).armor,
            );
            xx = x + 8 * 3 + 8 * pwidth + 8 * lwidth;
            crate::src::cgame::cg_drawtools::CG_DrawStringExt(
                xx,
                y as i32,
                st.as_mut_ptr(),
                hcolor.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse,
                crate::src::qcommon::q_shared::qfalse,
                8,
                16 / 2,
                0,
            );
            // draw weapon icon
            xx += 8 * 3;
            if crate::src::cgame::cg_main::cg_weapons[(*ci).curWeapon as usize].weaponIcon != 0 {
                crate::src::cgame::cg_drawtools::CG_DrawPic(
                    xx as f32,
                    y,
                    8f32,
                    (16i32 / 2i32) as f32,
                    crate::src::cgame::cg_main::cg_weapons[(*ci).curWeapon as usize].weaponIcon,
                );
            } else {
                crate::src::cgame::cg_drawtools::CG_DrawPic(
                    xx as f32,
                    y,
                    8f32,
                    (16i32 / 2i32) as f32,
                    crate::src::cgame::cg_main::cgs.media.deferShader,
                );
            }
            // Draw powerup icons
            if right as u64 != 0 {
                xx = x
            } else {
                xx = x + w - 8
            }

            for j in 0..=crate::bg_public_h::PW_NUM_POWERUPS as i32 {
                if (*ci).powerups & (1) << j != 0 {
                    item = crate::src::game::bg_misc::BG_FindItemForPowerup(
                        j as crate::bg_public_h::powerup_t,
                    );
                    if !item.is_null() {
                        crate::src::cgame::cg_drawtools::CG_DrawPic(
                            xx as f32,
                            y,
                            8f32,
                            (16i32 / 2) as f32,
                            crate::src::cgame::cg_syscalls::trap_R_RegisterShader((*item).icon),
                        );
                        if right as u64 != 0 {
                            xx -= 8
                        } else {
                            xx += 8
                        }
                    }
                }
            }
            y += (16i32 / 2) as f32
        }
        i += 1
    }
    return ret_y as f32;
    //#endif
}
/*
=====================
CG_DrawUpperRight

=====================
*/

unsafe extern "C" fn CG_DrawUpperRight(mut stereoFrame: crate::tr_types_h::stereoFrame_t) {
    let mut y: f32 = 0.;
    y = 0f32;
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM
        && crate::src::cgame::cg_main::cg_drawTeamOverlay.integer == 1
    {
        y = CG_DrawTeamOverlay(
            y,
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qtrue,
        )
    }
    if crate::src::cgame::cg_main::cg_drawSnapshot.integer != 0 {
        y = CG_DrawSnapshot(y)
    }
    if crate::src::cgame::cg_main::cg_drawFPS.integer != 0
        && (stereoFrame == crate::tr_types_h::STEREO_CENTER
            || stereoFrame == crate::tr_types_h::STEREO_RIGHT)
    {
        y = CG_DrawFPS(y)
    }
    if crate::src::cgame::cg_main::cg_drawTimer.integer != 0 {
        y = CG_DrawTimer(y)
    }
    if crate::src::cgame::cg_main::cg_drawAttacker.integer != 0 {
        CG_DrawAttacker(y);
    };
}
/*
===========================================================================================

  LOWER RIGHT CORNER

===========================================================================================
*/
/*
=================
CG_DrawScores

Draw the small two score display
=================
*/

unsafe extern "C" fn CG_DrawScores(mut y: f32) -> f32 {
    let mut s: *const i8 = 0 as *const i8;
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut score: i32 = 0;
    let mut x: i32 = 0;
    let mut w: i32 = 0;
    let mut v: i32 = 0;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut y1: f32 = 0.;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    s1 = crate::src::cgame::cg_main::cgs.scores1;
    s2 = crate::src::cgame::cg_main::cgs.scores2;
    y -= (16i32 + 8) as f32;
    y1 = y;
    // draw from the right side to left
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
        x = 640;
        color[0] = 0.0;
        color[1] = 0.0;
        color[2] = 1.0;
        color[3] = 0.33;
        s = crate::src::qcommon::q_shared::va(b"%2i\x00" as *const u8 as *mut i8, s2);
        w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 + 8;
        x -= w;
        crate::src::cgame::cg_drawtools::CG_FillRect(
            x as f32,
            y - 4f32,
            w as f32,
            (16i32 + 8) as f32,
            color.as_mut_ptr(),
        );
        if (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as usize]
            == crate::bg_public_h::TEAM_BLUE as i32
        {
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                x as f32,
                y - 4f32,
                w as f32,
                (16i32 + 8i32) as f32,
                crate::src::cgame::cg_main::cgs.media.selectShader,
            );
        }
        crate::src::cgame::cg_drawtools::CG_DrawBigString(x + 4, y as i32, s, 1.0);
        if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_CTF {
            // Display flag status
            item =
                crate::src::game::bg_misc::BG_FindItemForPowerup(crate::bg_public_h::PW_BLUEFLAG);
            if !item.is_null() {
                y1 = y - 16f32 - 8f32;
                if crate::src::cgame::cg_main::cgs.blueflag >= 0
                    && crate::src::cgame::cg_main::cgs.blueflag <= 2
                {
                    crate::src::cgame::cg_drawtools::CG_DrawPic(
                        x as f32,
                        y1 - 4f32,
                        w as f32,
                        (16i32 + 8i32) as f32,
                        crate::src::cgame::cg_main::cgs.media.blueFlagShader
                            [crate::src::cgame::cg_main::cgs.blueflag as usize],
                    );
                }
            }
        }
        color[0] = 1.0;
        color[1] = 0.0;
        color[2] = 0.0;
        color[3] = 0.33;
        s = crate::src::qcommon::q_shared::va(b"%2i\x00" as *const u8 as *mut i8, s1);
        w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 + 8;
        x -= w;
        crate::src::cgame::cg_drawtools::CG_FillRect(
            x as f32,
            y - 4f32,
            w as f32,
            (16i32 + 8) as f32,
            color.as_mut_ptr(),
        );
        if (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as usize]
            == crate::bg_public_h::TEAM_RED as i32
        {
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                x as f32,
                y - 4f32,
                w as f32,
                (16i32 + 8i32) as f32,
                crate::src::cgame::cg_main::cgs.media.selectShader,
            );
        }
        crate::src::cgame::cg_drawtools::CG_DrawBigString(x + 4, y as i32, s, 1.0);
        if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_CTF {
            // Display flag status
            item = crate::src::game::bg_misc::BG_FindItemForPowerup(crate::bg_public_h::PW_REDFLAG);
            if !item.is_null() {
                y1 = y - 16f32 - 8f32;
                if crate::src::cgame::cg_main::cgs.redflag >= 0
                    && crate::src::cgame::cg_main::cgs.redflag <= 2
                {
                    crate::src::cgame::cg_drawtools::CG_DrawPic(
                        x as f32,
                        y1 - 4f32,
                        w as f32,
                        (16i32 + 8i32) as f32,
                        crate::src::cgame::cg_main::cgs.media.redFlagShader
                            [crate::src::cgame::cg_main::cgs.redflag as usize],
                    );
                }
            }
        }
        if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_CTF {
            v = crate::src::cgame::cg_main::cgs.capturelimit
        } else {
            v = crate::src::cgame::cg_main::cgs.fraglimit
        }
        if v != 0 {
            s = crate::src::qcommon::q_shared::va(b"%2i\x00" as *const u8 as *mut i8, v);
            w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 + 8;
            x -= w;
            crate::src::cgame::cg_drawtools::CG_DrawBigString(x + 4i32, y as i32, s, 1.0f32);
        }
    } else {
        let mut spectator: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        x = 640;
        score = (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_SCORE as usize];
        spectator = ((*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as usize]
            == crate::bg_public_h::TEAM_SPECTATOR as i32)
            as crate::src::qcommon::q_shared::qboolean;
        // always show your score in the second box if not in first place
        if s1 != score {
            s2 = score
        }
        if s2 != -(9999) {
            s = crate::src::qcommon::q_shared::va(b"%2i\x00" as *const u8 as *mut i8, s2);
            w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 + 8;
            x -= w;
            if spectator as u64 == 0 && score == s2 && score != s1 {
                color[0] = 1.0;
                color[1] = 0.0;
                color[2] = 0.0;
                color[3] = 0.33;
                crate::src::cgame::cg_drawtools::CG_FillRect(
                    x as f32,
                    y - 4f32,
                    w as f32,
                    (16i32 + 8) as f32,
                    color.as_mut_ptr(),
                );
                crate::src::cgame::cg_drawtools::CG_DrawPic(
                    x as f32,
                    y - 4f32,
                    w as f32,
                    (16i32 + 8i32) as f32,
                    crate::src::cgame::cg_main::cgs.media.selectShader,
                );
            } else {
                color[0] = 0.5;
                color[1] = 0.5;
                color[2] = 0.5;
                color[3] = 0.33;
                crate::src::cgame::cg_drawtools::CG_FillRect(
                    x as f32,
                    y - 4f32,
                    w as f32,
                    (16i32 + 8i32) as f32,
                    color.as_mut_ptr(),
                );
            }
            crate::src::cgame::cg_drawtools::CG_DrawBigString(x + 4i32, y as i32, s, 1.0f32);
        }
        // first place
        if s1 != -(9999) {
            s = crate::src::qcommon::q_shared::va(b"%2i\x00" as *const u8 as *mut i8, s1);
            w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 + 8;
            x -= w;
            if spectator as u64 == 0 && score == s1 {
                color[0] = 0.0;
                color[1] = 0.0;
                color[2] = 1.0;
                color[3] = 0.33;
                crate::src::cgame::cg_drawtools::CG_FillRect(
                    x as f32,
                    y - 4f32,
                    w as f32,
                    (16i32 + 8) as f32,
                    color.as_mut_ptr(),
                );
                crate::src::cgame::cg_drawtools::CG_DrawPic(
                    x as f32,
                    y - 4f32,
                    w as f32,
                    (16i32 + 8i32) as f32,
                    crate::src::cgame::cg_main::cgs.media.selectShader,
                );
            } else {
                color[0] = 0.5;
                color[1] = 0.5;
                color[2] = 0.5;
                color[3] = 0.33;
                crate::src::cgame::cg_drawtools::CG_FillRect(
                    x as f32,
                    y - 4f32,
                    w as f32,
                    (16i32 + 8i32) as f32,
                    color.as_mut_ptr(),
                );
            }
            crate::src::cgame::cg_drawtools::CG_DrawBigString(x + 4i32, y as i32, s, 1.0f32);
        }
        if crate::src::cgame::cg_main::cgs.fraglimit != 0 {
            s = crate::src::qcommon::q_shared::va(
                b"%2i\x00" as *const u8 as *mut i8,
                crate::src::cgame::cg_main::cgs.fraglimit,
            );
            w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16 + 8;
            x -= w;
            crate::src::cgame::cg_drawtools::CG_DrawBigString(x + 4i32, y as i32, s, 1.0f32);
        }
    }
    return y1 - 8f32;
}
// MISSIONPACK
/*
================
CG_DrawPowerups
================
*/

unsafe extern "C" fn CG_DrawPowerups(mut y: f32) -> f32 {
    let mut sorted: [i32; 16] = [0; 16];
    let mut sortedTime: [i32; 16] = [0; 16];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut active: i32 = 0;
    let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    let mut t: i32 = 0;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut x: i32 = 0;
    let mut color: i32 = 0;
    let mut size: f32 = 0.;
    let mut f: f32 = 0.;
    static mut colors: [[f32; 4]; 2] = [[0.2, 1.0, 0.2, 1.0], [1.0, 0.2, 0.2, 1.0]];
    ps = &mut (*crate::src::cgame::cg_main::cg.snap).ps;
    if (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= 0 {
        return y;
    }
    // sort the list by time remaining
    active = 0;
    i = 0;
    while i < 16 {
        if !((*ps).powerups[i as usize] == 0) {
            // ZOID--don't draw if the power up has unlimited time
            // This is true of the CTF flags
            if !((*ps).powerups[i as usize] == 2147483647) {
                t = (*ps).powerups[i as usize] - crate::src::cgame::cg_main::cg.time;
                if !(t <= 0) {
                    // insert into the list
                    j = 0;
                    while j < active {
                        if sortedTime[j as usize] >= t {
                            k = active - 1;
                            while k >= j {
                                sorted[(k + 1) as usize] = sorted[k as usize];
                                sortedTime[(k + 1) as usize] = sortedTime[k as usize];
                                k -= 1
                            }
                            break;
                        } else {
                            j += 1
                        }
                    }
                    sorted[j as usize] = i;
                    sortedTime[j as usize] = t;
                    active += 1
                }
            }
        }
        i += 1
    }
    // draw the icons and timers
    x = 640 - 48 - 32 * 2;
    i = 0;
    while i < active {
        item = crate::src::game::bg_misc::BG_FindItemForPowerup(
            sorted[i as usize] as crate::bg_public_h::powerup_t,
        );
        if !item.is_null() {
            color = 1;
            y -= 48f32;
            crate::src::cgame::cg_syscalls::trap_R_SetColor(colors[color as usize].as_mut_ptr());
            CG_DrawField(x, y as i32, 2, sortedTime[i as usize] / 1000);
            t = (*ps).powerups[sorted[i as usize] as usize];
            if t - crate::src::cgame::cg_main::cg.time >= 5 * 1000 {
                crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
            } else {
                let mut modulate: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
                f = (t - crate::src::cgame::cg_main::cg.time) as f32 / 1000f32;
                f -= f as i32 as f32;
                modulate[3] = f;
                modulate[2] = modulate[3];
                modulate[1] = modulate[2];
                modulate[0] = modulate[1];
                crate::src::cgame::cg_syscalls::trap_R_SetColor(modulate.as_mut_ptr());
            }
            if crate::src::cgame::cg_main::cg.powerupActive == sorted[i as usize]
                && crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.powerupTime
                    < 200
            {
                f = (1.0
                    - ((crate::src::cgame::cg_main::cg.time as f32
                        - crate::src::cgame::cg_main::cg.powerupTime as f32)
                        / 200f32) as f64) as f32;
                size = (48f64 * (1.0 + (1.5 - 1.0) * f as f64)) as f32
            } else {
                size = 48f32
            }
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                640f32 - size,
                y + (48i32 / 2i32) as f32 - size / 2f32,
                size,
                size,
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader((*item).icon),
            );
        }
        i += 1
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
    return y;
}
// MISSIONPACK
/*
=====================
CG_DrawLowerRight

=====================
*/

unsafe extern "C" fn CG_DrawLowerRight() {
    let mut y: f32 = 0.;
    y = (480i32 - 48) as f32;
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM
        && crate::src::cgame::cg_main::cg_drawTeamOverlay.integer == 2
    {
        y = CG_DrawTeamOverlay(
            y,
            crate::src::qcommon::q_shared::qtrue,
            crate::src::qcommon::q_shared::qfalse,
        )
    }
    y = CG_DrawScores(y);
    CG_DrawPowerups(y);
}
// MISSIONPACK
/*
===================
CG_DrawPickupItem
===================
*/

unsafe extern "C" fn CG_DrawPickupItem(mut y: i32) -> i32 {
    let mut value: i32 = 0;
    let mut fadeColor: *mut f32 = 0 as *mut f32;
    if (*crate::src::cgame::cg_main::cg.snap).ps.stats[crate::bg_public_h::STAT_HEALTH as usize]
        <= 0
    {
        return y;
    }
    y -= 48;
    value = crate::src::cgame::cg_main::cg.itemPickup;
    if value != 0 {
        fadeColor = crate::src::cgame::cg_drawtools::CG_FadeColor(
            crate::src::cgame::cg_main::cg.itemPickupTime,
            3000,
        );
        if !fadeColor.is_null() {
            crate::src::cgame::cg_weapons::CG_RegisterItemVisuals(value);
            crate::src::cgame::cg_syscalls::trap_R_SetColor(fadeColor);
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                8f32,
                y as f32,
                48f32,
                48f32,
                crate::src::cgame::cg_main::cg_items[value as usize].icon,
            );
            crate::src::cgame::cg_drawtools::CG_DrawBigString(
                48 + 16,
                y + (48 / 2 - 16 / 2),
                (*crate::src::game::bg_misc::bg_itemlist
                    .as_mut_ptr()
                    .offset(value as isize))
                .pickup_name,
                *fadeColor.offset(0),
            );
            crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
        }
    }
    return y;
}
// MISSIONPACK
/*
=====================
CG_DrawLowerLeft

=====================
*/

unsafe extern "C" fn CG_DrawLowerLeft() {
    let mut y: f32 = 0.;
    y = (480i32 - 48) as f32;
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM
        && crate::src::cgame::cg_main::cg_drawTeamOverlay.integer == 3
    {
        y = CG_DrawTeamOverlay(
            y,
            crate::src::qcommon::q_shared::qfalse,
            crate::src::qcommon::q_shared::qfalse,
        )
    }
    CG_DrawPickupItem(y as i32);
}
// MISSIONPACK
//===========================================================================================
/*
=================
CG_DrawTeamInfo
=================
*/

unsafe extern "C" fn CG_DrawTeamInfo() {
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut hcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut chatHeight: i32 = 0;
    // bottom end
    if crate::src::cgame::cg_main::cg_teamChatHeight.integer < 8 {
        chatHeight = crate::src::cgame::cg_main::cg_teamChatHeight.integer
    } else {
        chatHeight = 8
    } // disabled
    if chatHeight <= 0 {
        return;
    }
    if crate::src::cgame::cg_main::cgs.teamLastChatPos
        != crate::src::cgame::cg_main::cgs.teamChatPos
    {
        if crate::src::cgame::cg_main::cg.time
            - crate::src::cgame::cg_main::cgs.teamChatMsgTimes
                [(crate::src::cgame::cg_main::cgs.teamLastChatPos % chatHeight) as usize]
            > crate::src::cgame::cg_main::cg_teamChatTime.integer
        {
            crate::src::cgame::cg_main::cgs.teamLastChatPos += 1
        }
        h = (crate::src::cgame::cg_main::cgs.teamChatPos
            - crate::src::cgame::cg_main::cgs.teamLastChatPos)
            * (16 / 2);
        if crate::src::cgame::cg_main::cgs.clientinfo
            [crate::src::cgame::cg_main::cg.clientNum as usize]
            .team
            == crate::bg_public_h::TEAM_RED
        {
            hcolor[0] = 1.0;
            hcolor[1] = 0.0;
            hcolor[2] = 0.0;
            hcolor[3] = 0.33
        } else if crate::src::cgame::cg_main::cgs.clientinfo
            [crate::src::cgame::cg_main::cg.clientNum as usize]
            .team
            == crate::bg_public_h::TEAM_BLUE
        {
            hcolor[0] = 0.0;
            hcolor[1] = 0.0;
            hcolor[2] = 1.0;
            hcolor[3] = 0.33
        } else {
            hcolor[0] = 0.0;
            hcolor[1] = 1.0;
            hcolor[2] = 0.0;
            hcolor[3] = 0.33
        }
        crate::src::cgame::cg_syscalls::trap_R_SetColor(hcolor.as_mut_ptr());
        crate::src::cgame::cg_drawtools::CG_DrawPic(
            0f32,
            (420 - h) as f32,
            640f32,
            h as f32,
            crate::src::cgame::cg_main::cgs.media.teamStatusBar,
        );
        crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
        hcolor[2] = 1.0;
        hcolor[1] = hcolor[2];
        hcolor[0] = hcolor[1];
        hcolor[3] = 1.0;
        i = crate::src::cgame::cg_main::cgs.teamChatPos - 1;
        while i >= crate::src::cgame::cg_main::cgs.teamLastChatPos {
            crate::src::cgame::cg_drawtools::CG_DrawStringExt(
                0 + 8,
                420 - (crate::src::cgame::cg_main::cgs.teamChatPos - i) * (16 / 2),
                crate::src::cgame::cg_main::cgs.teamChatMsgs[(i % chatHeight) as usize]
                    .as_mut_ptr(),
                hcolor.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse,
                crate::src::qcommon::q_shared::qfalse,
                8,
                16 / 2,
                0,
            );
            i -= 1
        }
    };
}
// MISSIONPACK
/*
===================
CG_DrawHoldableItem
===================
*/

unsafe extern "C" fn CG_DrawHoldableItem() {
    let mut value: i32 = 0;
    value = (*crate::src::cgame::cg_main::cg.snap).ps.stats
        [crate::bg_public_h::STAT_HOLDABLE_ITEM as usize];
    if value != 0 {
        crate::src::cgame::cg_weapons::CG_RegisterItemVisuals(value);
        crate::src::cgame::cg_drawtools::CG_DrawPic(
            (640i32 - 48i32) as f32,
            ((480i32 - 48i32) / 2i32) as f32,
            48f32,
            48f32,
            crate::src::cgame::cg_main::cg_items[value as usize].icon,
        );
    };
}
// MISSIONPACK
// MISSIONPACK
/*
===================
CG_DrawReward
===================
*/

unsafe extern "C" fn CG_DrawReward() {
    let mut color: *mut f32 = 0 as *mut f32;
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut buf: [i8; 32] = [0; 32];
    if crate::src::cgame::cg_main::cg_drawRewards.integer == 0 {
        return;
    }
    color = crate::src::cgame::cg_drawtools::CG_FadeColor(
        crate::src::cgame::cg_main::cg.rewardTime,
        3000,
    );
    if color.is_null() {
        if crate::src::cgame::cg_main::cg.rewardStack > 0 {
            i = 0;
            while i < crate::src::cgame::cg_main::cg.rewardStack {
                crate::src::cgame::cg_main::cg.rewardSound[i as usize] =
                    crate::src::cgame::cg_main::cg.rewardSound[(i + 1) as usize];
                crate::src::cgame::cg_main::cg.rewardShader[i as usize] =
                    crate::src::cgame::cg_main::cg.rewardShader[(i + 1) as usize];
                crate::src::cgame::cg_main::cg.rewardCount[i as usize] =
                    crate::src::cgame::cg_main::cg.rewardCount[(i + 1) as usize];
                i += 1
            }
            crate::src::cgame::cg_main::cg.rewardTime = crate::src::cgame::cg_main::cg.time;
            crate::src::cgame::cg_main::cg.rewardStack -= 1;
            color = crate::src::cgame::cg_drawtools::CG_FadeColor(
                crate::src::cgame::cg_main::cg.rewardTime,
                3000,
            );
            crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                crate::src::cgame::cg_main::cg.rewardSound[0usize],
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
            );
        } else {
            return;
        }
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color);
    /*
    count = cg.rewardCount[0]/10;				// number of big rewards to draw

    if (count) {
        y = 4;
        x = 320 - count * ICON_SIZE;
        for ( i = 0 ; i < count ; i++ ) {
            CG_DrawPic( x, y, (ICON_SIZE*2)-4, (ICON_SIZE*2)-4, cg.rewardShader[0] );
            x += (ICON_SIZE*2);
        }
    }

    count = cg.rewardCount[0] - count*10;		// number of small rewards to draw
    */
    if crate::src::cgame::cg_main::cg.rewardCount[0] >= 10 {
        y = 56f32;
        x = (320i32 - 48 / 2) as f32;
        crate::src::cgame::cg_drawtools::CG_DrawPic(
            x,
            y,
            (48i32 - 4) as f32,
            (48i32 - 4) as f32,
            crate::src::cgame::cg_main::cg.rewardShader[0],
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 32]>() as i32,
            b"%d\x00" as *const u8 as *const i8,
            crate::src::cgame::cg_main::cg.rewardCount[0usize],
        );
        x = ((640 - 8 * crate::src::cgame::cg_drawtools::CG_DrawStrlen(buf.as_mut_ptr())) / 2)
            as f32;
        crate::src::cgame::cg_drawtools::CG_DrawStringExt(
            x as i32,
            (y + 48f32) as i32,
            buf.as_mut_ptr(),
            color,
            crate::src::qcommon::q_shared::qfalse,
            crate::src::qcommon::q_shared::qtrue,
            8i32,
            16i32,
            0i32,
        );
    } else {
        count = crate::src::cgame::cg_main::cg.rewardCount[0];
        y = 56f32;
        x = (320 - count * 48 / 2) as f32;
        i = 0;
        while i < count {
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                x,
                y,
                (48i32 - 4) as f32,
                (48i32 - 4) as f32,
                crate::src::cgame::cg_main::cg.rewardShader[0],
            );
            x += 48f32;
            i += 1
        }
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
#[no_mangle]

pub static mut lagometer: lagometer_t = lagometer_t {
    frameSamples: [0; 128],
    frameCount: 0,
    snapshotFlags: [0; 128],
    snapshotSamples: [0; 128],
    snapshotCount: 0,
};
/*
==============
CG_AddLagometerFrameInfo

Adds the current interpolate / extrapolate bar for this frame
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddLagometerFrameInfo() {
    let mut offset: i32 = 0;
    offset =
        crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.latestSnapshotTime;
    lagometer.frameSamples[(lagometer.frameCount & 128 - 1) as usize] = offset;
    lagometer.frameCount += 1;
}
/*
==============
CG_AddLagometerSnapshotInfo

Each time a snapshot is received, log its ping time and
the number of snapshots that were dropped before it.

Pass NULL for a dropped packet.
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddLagometerSnapshotInfo(
    mut snap: *mut crate::cg_public_h::snapshot_t,
) {
    // dropped packet
    if snap.is_null() {
        lagometer.snapshotSamples[(lagometer.snapshotCount & 128 - 1) as usize] = -(1);
        lagometer.snapshotCount += 1;
        return;
    }
    // add this snapshot's info
    lagometer.snapshotSamples[(lagometer.snapshotCount & 128 - 1) as usize] = (*snap).ping;
    lagometer.snapshotFlags[(lagometer.snapshotCount & 128 - 1) as usize] = (*snap).snapFlags;
    lagometer.snapshotCount += 1;
}
/*
==============
CG_DrawDisconnect

Should we draw something differnet for long lag vs no packets?
==============
*/

unsafe extern "C" fn CG_DrawDisconnect() {
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut cmdNum: i32 = 0;
    let mut cmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    let mut s: *const i8 = 0 as *const i8;
    let mut w: i32 = 0;
    // draw the phone jack if we are completely past our buffers
    cmdNum = crate::src::cgame::cg_syscalls::trap_GetCurrentCmdNumber() - 64 + 1;
    crate::src::cgame::cg_syscalls::trap_GetUserCmd(cmdNum, &mut cmd);
    if cmd.serverTime <= (*crate::src::cgame::cg_main::cg.snap).ps.commandTime
        || cmd.serverTime > crate::src::cgame::cg_main::cg.time
    {
        // special check for map_restart
        return;
    }
    // also add text in center of screen
    s = b"Connection Interrupted\x00" as *const u8 as *const i8;
    w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16;
    crate::src::cgame::cg_drawtools::CG_DrawBigString(320 - w / 2, 100, s, 1.0);
    // blink the icon
    if crate::src::cgame::cg_main::cg.time >> 9 & 1 != 0 {
        return;
    }
    x = (640i32 - 48) as f32;
    y = (480i32 - 48) as f32;
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        x,
        y,
        48f32,
        48f32,
        crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
            b"gfx/2d/net.tga\x00" as *const u8 as *const i8,
        ),
    );
}
/*
==============
CG_DrawLagometer
==============
*/

unsafe extern "C" fn CG_DrawLagometer() {
    let mut a: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    let mut v: f32 = 0.;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut mid: f32 = 0.;
    let mut range: f32 = 0.;
    let mut color: i32 = 0;
    let mut vscale: f32 = 0.;
    if crate::src::cgame::cg_main::cg_lagometer.integer == 0
        || crate::src::cgame::cg_main::cgs.localServer != 0
    {
        CG_DrawDisconnect();
        return;
    }
    //
    // draw the graph
    //
    x = 640 - 48;
    y = 480 - 48;
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
    crate::src::cgame::cg_drawtools::CG_DrawPic(
        x as f32,
        y as f32,
        48f32,
        48f32,
        crate::src::cgame::cg_main::cgs.media.lagometerShader,
    );
    ax = x as f32;
    ay = y as f32;
    aw = 48f32;
    ah = 48f32;
    crate::src::cgame::cg_drawtools::CG_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    color = -(1);
    range = ah / 3f32;
    mid = ay + range;
    vscale = range / 300f32;
    // draw the frame interpoalte / extrapolate graph
    a = 0;
    while (a as f32) < aw {
        i = lagometer.frameCount - 1 - a & 128 - 1;
        v = lagometer.frameSamples[i as usize] as f32;
        v *= vscale;
        if v > 0f32 {
            if color != 1 {
                color = 1;
                crate::src::cgame::cg_syscalls::trap_R_SetColor(
                    crate::src::qcommon::q_math::g_color_table
                        [('3' as i32 - '0' as i32 & 0x7i32) as usize]
                        .as_mut_ptr(),
                );
            }
            if v > range {
                v = range
            }
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax + aw - a as f32,
                mid - v,
                1f32,
                v,
                0f32,
                0f32,
                0f32,
                0f32,
                crate::src::cgame::cg_main::cgs.media.whiteShader,
            );
        } else if v < 0f32 {
            if color != 2 {
                color = 2;
                crate::src::cgame::cg_syscalls::trap_R_SetColor(
                    crate::src::qcommon::q_math::g_color_table
                        [('4' as i32 - '0' as i32 & 0x7i32) as usize]
                        .as_mut_ptr(),
                );
            }
            v = -v;
            if v > range {
                v = range
            }
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax + aw - a as f32,
                mid,
                1f32,
                v,
                0f32,
                0f32,
                0f32,
                0f32,
                crate::src::cgame::cg_main::cgs.media.whiteShader,
            );
        }
        a += 1
    }
    // draw the snapshot latency / drop graph
    range = ah / 2f32; // YELLOW for rate delay
    vscale = range / 900f32; // RED for dropped snapshots
    a = 0;
    while (a as f32) < aw {
        i = lagometer.snapshotCount - 1 - a & 128 - 1;
        v = lagometer.snapshotSamples[i as usize] as f32;
        if v > 0f32 {
            if lagometer.snapshotFlags[i as usize] & 1 != 0 {
                if color != 5 {
                    color = 5;
                    crate::src::cgame::cg_syscalls::trap_R_SetColor(
                        crate::src::qcommon::q_math::g_color_table
                            [('3' as i32 - '0' as i32 & 0x7i32) as usize]
                            .as_mut_ptr(),
                    );
                }
            } else if color != 3 {
                color = 3;
                crate::src::cgame::cg_syscalls::trap_R_SetColor(
                    crate::src::qcommon::q_math::g_color_table
                        [('2' as i32 - '0' as i32 & 0x7i32) as usize]
                        .as_mut_ptr(),
                );
            }
            v = v * vscale;
            if v > range {
                v = range
            }
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax + aw - a as f32,
                ay + ah - v,
                1f32,
                v,
                0f32,
                0f32,
                0f32,
                0f32,
                crate::src::cgame::cg_main::cgs.media.whiteShader,
            );
        } else if v < 0f32 {
            if color != 4 {
                color = 4;
                crate::src::cgame::cg_syscalls::trap_R_SetColor(
                    crate::src::qcommon::q_math::g_color_table
                        [('1' as i32 - '0' as i32 & 0x7i32) as usize]
                        .as_mut_ptr(),
                );
            }
            crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
                ax + aw - a as f32,
                ay + ah - range,
                1f32,
                range,
                0f32,
                0f32,
                0f32,
                0f32,
                crate::src::cgame::cg_main::cgs.media.whiteShader,
            );
        }
        a += 1
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
    if crate::src::cgame::cg_main::cg_nopredict.integer != 0
        || crate::src::cgame::cg_main::cg_synchronousClients.integer != 0
    {
        crate::src::cgame::cg_drawtools::CG_DrawBigString(
            x,
            y,
            b"snc\x00" as *const u8 as *const i8,
            1f32,
        );
    }
    CG_DrawDisconnect();
}
/*
===============================================================================

CENTER PRINTING

===============================================================================
*/
/*
==============
CG_CenterPrint

Called for important messages that should stay in the center of the screen
for a few moments
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CenterPrint(mut str: *const i8, mut y: i32, mut charWidth: i32) {
    let mut s: *mut i8 = 0 as *mut i8;
    crate::src::qcommon::q_shared::Q_strncpyz(
        crate::src::cgame::cg_main::cg.centerPrint.as_mut_ptr(),
        str,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::cgame::cg_main::cg.centerPrintTime = crate::src::cgame::cg_main::cg.time;
    crate::src::cgame::cg_main::cg.centerPrintY = y;
    crate::src::cgame::cg_main::cg.centerPrintCharWidth = charWidth;
    // count the number of lines for centering
    crate::src::cgame::cg_main::cg.centerPrintLines = 1;
    s = crate::src::cgame::cg_main::cg.centerPrint.as_mut_ptr();
    while *s != 0 {
        if *s as i32 == '\n' as i32 {
            crate::src::cgame::cg_main::cg.centerPrintLines += 1
        }
        s = s.offset(1)
    }
}
/*
===================
CG_DrawCenterString
===================
*/

unsafe extern "C" fn CG_DrawCenterString() {
    let mut start: *mut i8 = 0 as *mut i8;
    let mut l: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    let mut color: *mut f32 = 0 as *mut f32;
    if crate::src::cgame::cg_main::cg.centerPrintTime == 0 {
        return;
    }
    color = crate::src::cgame::cg_drawtools::CG_FadeColor(
        crate::src::cgame::cg_main::cg.centerPrintTime,
        (1000f32 * crate::src::cgame::cg_main::cg_centertime.value) as i32,
    );
    if color.is_null() {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color);
    start = crate::src::cgame::cg_main::cg.centerPrint.as_mut_ptr();
    y = crate::src::cgame::cg_main::cg.centerPrintY
        - crate::src::cgame::cg_main::cg.centerPrintLines * 16 / 2;
    loop {
        let mut linebuffer: [i8; 1024] = [0; 1024];
        l = 0;
        while l < 50 {
            if *start.offset(l as isize) == 0 || *start.offset(l as isize) as i32 == '\n' as i32 {
                break;
            }
            linebuffer[l as usize] = *start.offset(l as isize);
            l += 1
        }
        linebuffer[l as usize] = 0;
        w = crate::src::cgame::cg_main::cg.centerPrintCharWidth
            * crate::src::cgame::cg_drawtools::CG_DrawStrlen(linebuffer.as_mut_ptr());
        x = (640 - w) / 2;
        crate::src::cgame::cg_drawtools::CG_DrawStringExt(
            x,
            y,
            linebuffer.as_mut_ptr(),
            color,
            crate::src::qcommon::q_shared::qfalse,
            crate::src::qcommon::q_shared::qtrue,
            crate::src::cgame::cg_main::cg.centerPrintCharWidth,
            (crate::src::cgame::cg_main::cg.centerPrintCharWidth as f64 * 1.5) as i32,
            0,
        );
        y = (y as f64 + crate::src::cgame::cg_main::cg.centerPrintCharWidth as f64 * 1.5) as i32;
        while *start as i32 != 0 && *start as i32 != '\n' as i32 {
            start = start.offset(1)
        }
        if *start == 0 {
            break;
        }
        start = start.offset(1)
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
================================================================================

CROSSHAIR

================================================================================
*/
/*
=================
CG_DrawCrosshair
=================
*/

unsafe extern "C" fn CG_DrawCrosshair() {
    let mut w: f32 = 0.;
    let mut h: f32 = 0.;
    let mut hShader: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut f: f32 = 0.;
    let mut x: f32 = 0.;
    let mut y: f32 = 0.;
    let mut ca: i32 = 0;
    if crate::src::cgame::cg_main::cg_drawCrosshair.integer == 0 {
        return;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.persistant[crate::bg_public_h::PERS_TEAM as usize]
        == crate::bg_public_h::TEAM_SPECTATOR as i32
    {
        return;
    }
    if crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 != 0 {
        return;
    }
    // set color based on health
    if crate::src::cgame::cg_main::cg_crosshairHealth.integer != 0 {
        let mut hcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
        crate::src::cgame::cg_drawtools::CG_ColorForHealth(hcolor.as_mut_ptr());
        crate::src::cgame::cg_syscalls::trap_R_SetColor(hcolor.as_mut_ptr());
    } else {
        crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
    }
    h = crate::src::cgame::cg_main::cg_crosshairSize.value;
    w = h;
    // pulse the size of the crosshair when picking up items
    f = (crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.itemPickupBlendTime)
        as f32;
    if f > 0f32 && f < 200f32 {
        f /= 200f32;
        w *= 1f32 + f;
        h *= 1f32 + f
    }
    x = crate::src::cgame::cg_main::cg_crosshairX.integer as f32;
    y = crate::src::cgame::cg_main::cg_crosshairY.integer as f32;
    crate::src::cgame::cg_drawtools::CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    ca = crate::src::cgame::cg_main::cg_drawCrosshair.integer;
    if ca < 0 {
        ca = 0
    }
    hShader = crate::src::cgame::cg_main::cgs.media.crosshairShader[(ca % 10) as usize];
    crate::src::cgame::cg_syscalls::trap_R_DrawStretchPic(
        ((x + crate::src::cgame::cg_main::cg.refdef.x as f32) as f64
            + 0.5 * (crate::src::cgame::cg_main::cg.refdef.width as f32 - w) as f64) as f32,
        ((y + crate::src::cgame::cg_main::cg.refdef.y as f32) as f64
            + 0.5 * (crate::src::cgame::cg_main::cg.refdef.height as f32 - h) as f64)
            as f32,
        w,
        h,
        0f32,
        0f32,
        1f32,
        1f32,
        hShader,
    );
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
=================
CG_DrawCrosshair3D
=================
*/

unsafe extern "C" fn CG_DrawCrosshair3D() {
    let mut w: f32 = 0.;
    let mut hShader: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut f: f32 = 0.;
    let mut ca: i32 = 0;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
    let mut endpos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut stereoSep: f32 = 0.;
    let mut zProj: f32 = 0.;
    let mut maxdist: f32 = 0.;
    let mut xmax: f32 = 0.;
    let mut rendererinfos: [i8; 128] = [0; 128];
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    if crate::src::cgame::cg_main::cg_drawCrosshair.integer == 0 {
        return;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.persistant[crate::bg_public_h::PERS_TEAM as usize]
        == crate::bg_public_h::TEAM_SPECTATOR as i32
    {
        return;
    }
    if crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 != 0 {
        return;
    }
    w = crate::src::cgame::cg_main::cg_crosshairSize.value;
    // pulse the size of the crosshair when picking up items
    f = (crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.itemPickupBlendTime)
        as f32;
    if f > 0f32 && f < 200f32 {
        f /= 200f32;
        w *= 1f32 + f
    }
    ca = crate::src::cgame::cg_main::cg_drawCrosshair.integer;
    if ca < 0 {
        ca = 0
    }
    hShader = crate::src::cgame::cg_main::cgs.media.crosshairShader[(ca % 10) as usize];
    // Use a different method rendering the crosshair so players don't see two of them when
    // focusing their eyes at distant objects with high stereo separation
    // We are going to trace to the next shootable object and place the crosshair in front of it.
    // first get all the important renderer information
    crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer(
        b"r_zProj\x00" as *const u8 as *const i8,
        rendererinfos.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    zProj = atof(rendererinfos.as_mut_ptr()) as f32;
    crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer(
        b"r_stereoSeparation\x00" as *const u8 as *const i8,
        rendererinfos.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    stereoSep = (zProj as f64 / atof(rendererinfos.as_mut_ptr())) as f32;
    xmax = (zProj as f64
        * crate::stdlib::tan(
            crate::src::cgame::cg_main::cg.refdef.fov_x as f64 * 3.14159265358979323846 / 360f64,
        )) as f32;
    // let the trace run through until a change in stereo separation of the crosshair becomes less than one pixel.
    maxdist = crate::src::cgame::cg_main::cgs.glconfig.vidWidth as f32 * stereoSep * zProj
        / (2f32 * xmax);
    endpos[0] = crate::src::cgame::cg_main::cg.refdef.vieworg[0]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][0] * maxdist;
    endpos[1] = crate::src::cgame::cg_main::cg.refdef.vieworg[1]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][1] * maxdist;
    endpos[2] = crate::src::cgame::cg_main::cg.refdef.vieworg[2]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][2] * maxdist;
    crate::src::cgame::cg_predict::CG_Trace(
        &mut trace,
        crate::src::cgame::cg_main::cg.refdef.vieworg.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0,
        1 | 0x2000000 | 0x4000000,
    );
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.reType = crate::tr_types_h::RT_SPRITE;
    ent.renderfx = 0x8 | 0x10;
    ent.origin[0] = trace.endpos[0];
    ent.origin[1] = trace.endpos[1];
    ent.origin[2] = trace.endpos[2];
    // scale the crosshair so it appears the same size for all distances
    ent.radius = w / 640f32 * xmax * trace.fraction * maxdist / zProj;
    ent.customShader = hShader;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
=================
CG_ScanForCrosshairEntity
=================
*/

unsafe extern "C" fn CG_ScanForCrosshairEntity() {
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut content: i32 = 0;
    start[0] = crate::src::cgame::cg_main::cg.refdef.vieworg[0];
    start[1] = crate::src::cgame::cg_main::cg.refdef.vieworg[1];
    start[2] = crate::src::cgame::cg_main::cg.refdef.vieworg[2];
    end[0] = start[0] + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][0] * 131072f32;
    end[1] = start[1] + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][1] * 131072f32;
    end[2] = start[2] + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][2] * 131072f32;
    crate::src::cgame::cg_predict::CG_Trace(
        &mut trace,
        start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*crate::src::cgame::cg_main::cg.snap).ps.clientNum,
        1 | 0x2000000,
    );
    if trace.entityNum >= 64 {
        return;
    }
    // if the player is in fog, don't show it
    content = crate::src::cgame::cg_predict::CG_PointContents(
        trace.endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0,
    );
    if content & 64 != 0 {
        return;
    }
    // if the player is invisible, don't show it
    if crate::src::cgame::cg_main::cg_entities[trace.entityNum as usize]
        .currentState
        .powerups
        & (1) << crate::bg_public_h::PW_INVIS as i32
        != 0
    {
        return;
    }
    // update the fade timer
    crate::src::cgame::cg_main::cg.crosshairClientNum = trace.entityNum;
    crate::src::cgame::cg_main::cg.crosshairClientTime = crate::src::cgame::cg_main::cg.time;
}
/*
=====================
CG_DrawCrosshairNames
=====================
*/

unsafe extern "C" fn CG_DrawCrosshairNames() {
    let mut color: *mut f32 = 0 as *mut f32;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut w: f32 = 0.;
    if crate::src::cgame::cg_main::cg_drawCrosshair.integer == 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg_drawCrosshairNames.integer == 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 != 0 {
        return;
    }
    // scan the known entities to see if the crosshair is sighted on one
    CG_ScanForCrosshairEntity();
    // draw the name of the player being looked at
    color = crate::src::cgame::cg_drawtools::CG_FadeColor(
        crate::src::cgame::cg_main::cg.crosshairClientTime,
        1000,
    );
    if color.is_null() {
        crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
        return;
    }
    name = crate::src::cgame::cg_main::cgs.clientinfo
        [crate::src::cgame::cg_main::cg.crosshairClientNum as usize]
        .name
        .as_mut_ptr();
    w = (crate::src::cgame::cg_drawtools::CG_DrawStrlen(name) * 16) as f32;
    crate::src::cgame::cg_drawtools::CG_DrawBigString(
        (320f32 - w / 2f32) as i32,
        170,
        name,
        *color.offset(3) * 0.5,
    );
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
//==============================================================================
/*
=================
CG_DrawSpectator
=================
*/

unsafe extern "C" fn CG_DrawSpectator() {
    crate::src::cgame::cg_drawtools::CG_DrawBigString(
        320 - 9 * 8,
        440,
        b"SPECTATOR\x00" as *const u8 as *const i8,
        1.0,
    );
    if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_TOURNAMENT {
        crate::src::cgame::cg_drawtools::CG_DrawBigString(
            320i32 - 15i32 * 8i32,
            460i32,
            b"waiting to play\x00" as *const u8 as *const i8,
            1.0f32,
        );
    } else if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
        crate::src::cgame::cg_drawtools::CG_DrawBigString(
            320i32 - 39i32 * 8i32,
            460i32,
            b"press ESC and use the JOIN menu to play\x00" as *const u8 as *const i8,
            1.0f32,
        );
    };
}
/*
=================
CG_DrawVote
=================
*/

unsafe extern "C" fn CG_DrawVote() {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut sec: i32 = 0;
    if crate::src::cgame::cg_main::cgs.voteTime == 0 {
        return;
    }
    // play a talk beep whenever it is modified
    if crate::src::cgame::cg_main::cgs.voteModified as u64 != 0 {
        crate::src::cgame::cg_main::cgs.voteModified = crate::src::qcommon::q_shared::qfalse;
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.talkSound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
        );
    }
    sec = (30000
        - (crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cgs.voteTime))
        / 1000;
    if sec < 0 {
        sec = 0
    }
    s = crate::src::qcommon::q_shared::va(
        b"VOTE(%i):%s yes:%i no:%i\x00" as *const u8 as *mut i8,
        sec,
        crate::src::cgame::cg_main::cgs.voteString.as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.voteYes,
        crate::src::cgame::cg_main::cgs.voteNo,
    );
    crate::src::cgame::cg_drawtools::CG_DrawSmallString(0, 58, s, 1.0);
}
/*
=================
CG_DrawTeamVote
=================
*/

unsafe extern "C" fn CG_DrawTeamVote() {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut sec: i32 = 0;
    let mut cs_offset: i32 = 0;
    if crate::src::cgame::cg_main::cgs.clientinfo[crate::src::cgame::cg_main::cg.clientNum as usize]
        .team
        == crate::bg_public_h::TEAM_RED
    {
        cs_offset = 0
    } else if crate::src::cgame::cg_main::cgs.clientinfo
        [crate::src::cgame::cg_main::cg.clientNum as usize]
        .team
        == crate::bg_public_h::TEAM_BLUE
    {
        cs_offset = 1
    } else {
        return;
    }
    if crate::src::cgame::cg_main::cgs.teamVoteTime[cs_offset as usize] == 0 {
        return;
    }
    // play a talk beep whenever it is modified
    if crate::src::cgame::cg_main::cgs.teamVoteModified[cs_offset as usize] as u64 != 0 {
        crate::src::cgame::cg_main::cgs.teamVoteModified[cs_offset as usize] =
            crate::src::qcommon::q_shared::qfalse;
        crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
            crate::src::cgame::cg_main::cgs.media.talkSound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
        );
    }
    sec = (30000
        - (crate::src::cgame::cg_main::cg.time
            - crate::src::cgame::cg_main::cgs.teamVoteTime[cs_offset as usize]))
        / 1000;
    if sec < 0 {
        sec = 0
    }
    s = crate::src::qcommon::q_shared::va(
        b"TEAMVOTE(%i):%s yes:%i no:%i\x00" as *const u8 as *mut i8,
        sec,
        crate::src::cgame::cg_main::cgs.teamVoteString[cs_offset as usize].as_mut_ptr(),
        crate::src::cgame::cg_main::cgs.teamVoteYes[cs_offset as usize],
        crate::src::cgame::cg_main::cgs.teamVoteNo[cs_offset as usize],
    );
    crate::src::cgame::cg_drawtools::CG_DrawSmallString(0, 90, s, 1.0);
}

unsafe extern "C" fn CG_DrawScoreboard() -> crate::src::qcommon::q_shared::qboolean {
    return crate::src::cgame::cg_scoreboard::CG_DrawOldScoreboard();
}
/*
=================
CG_DrawIntermission
=================
*/

unsafe extern "C" fn CG_DrawIntermission() {
    //	int key;
    if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_SINGLE_PLAYER {
        CG_DrawCenterString();
        return;
    }
    crate::src::cgame::cg_main::cg.scoreFadeTime = crate::src::cgame::cg_main::cg.time;
    crate::src::cgame::cg_main::cg.scoreBoardShowing = CG_DrawScoreboard();
}
/*
=================
CG_DrawFollow
=================
*/

unsafe extern "C" fn CG_DrawFollow() -> crate::src::qcommon::q_shared::qboolean {
    let mut x: f32 = 0.;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut name: *const i8 = 0 as *const i8;
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_flags & 4096 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    color[0] = 1f32;
    color[1] = 1f32;
    color[2] = 1f32;
    color[3] = 1f32;
    crate::src::cgame::cg_drawtools::CG_DrawBigString(
        320 - 9 * 8,
        24,
        b"following\x00" as *const u8 as *const i8,
        1.0,
    );
    name = crate::src::cgame::cg_main::cgs.clientinfo
        [(*crate::src::cgame::cg_main::cg.snap).ps.clientNum as usize]
        .name
        .as_mut_ptr();
    x = (0.5 * (640 - 32 * crate::src::cgame::cg_drawtools::CG_DrawStrlen(name)) as f64) as f32;
    crate::src::cgame::cg_drawtools::CG_DrawStringExt(
        x as i32,
        40,
        name,
        color.as_mut_ptr(),
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
        32,
        48,
        0,
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=================
CG_DrawAmmoWarning
=================
*/

unsafe extern "C" fn CG_DrawAmmoWarning() {
    let mut s: *const i8 = 0 as *const i8;
    let mut w: i32 = 0;
    if crate::src::cgame::cg_main::cg_drawAmmoWarning.integer == 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg.lowAmmoWarning == 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg.lowAmmoWarning == 2 {
        s = b"OUT OF AMMO\x00" as *const u8 as *const i8
    } else {
        s = b"LOW AMMO WARNING\x00" as *const u8 as *const i8
    }
    w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16;
    crate::src::cgame::cg_drawtools::CG_DrawBigString(320 - w / 2, 64, s, 1.0);
}
/*
=================
CG_DrawWarmup
=================
*/

unsafe extern "C" fn CG_DrawWarmup() {
    let mut w: i32 = 0;
    let mut sec: i32 = 0;
    let mut _i: i32 = 0;
    let mut cw: i32 = 0;
    let mut ci1: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut ci2: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut s: *const i8 = 0 as *const i8;
    sec = crate::src::cgame::cg_main::cg.warmup;
    if sec == 0 {
        return;
    }
    if sec < 0 {
        s = b"Waiting for players\x00" as *const u8 as *const i8;
        w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s) * 16;
        crate::src::cgame::cg_drawtools::CG_DrawBigString(320 - w / 2, 24, s, 1.0);
        crate::src::cgame::cg_main::cg.warmupCount = 0;
        return;
    }
    if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_TOURNAMENT {
        // find the two active players
        ci1 = 0 as *mut crate::cg_local_h::clientInfo_t;
        ci2 = 0 as *mut crate::cg_local_h::clientInfo_t;

        for i in 0..crate::src::cgame::cg_main::cgs.maxclients {
            if crate::src::cgame::cg_main::cgs.clientinfo[i as usize].infoValid != 0
                && crate::src::cgame::cg_main::cgs.clientinfo[i as usize].team
                    == crate::bg_public_h::TEAM_FREE
            {
                if ci1.is_null() {
                    ci1 = &mut *crate::src::cgame::cg_main::cgs
                        .clientinfo
                        .as_mut_ptr()
                        .offset(i as isize)
                        as *mut crate::cg_local_h::clientInfo_t
                } else {
                    ci2 = &mut *crate::src::cgame::cg_main::cgs
                        .clientinfo
                        .as_mut_ptr()
                        .offset(i as isize)
                        as *mut crate::cg_local_h::clientInfo_t
                }
            }
        }
        if !ci1.is_null() && !ci2.is_null() {
            s = crate::src::qcommon::q_shared::va(
                b"%s vs %s\x00" as *const u8 as *mut i8,
                (*ci1).name.as_mut_ptr(),
                (*ci2).name.as_mut_ptr(),
            );
            w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s);
            if w > 640 / 32 {
                cw = 640 / w
            } else {
                cw = 32
            }
            crate::src::cgame::cg_drawtools::CG_DrawStringExt(
                320i32 - w * cw / 2i32,
                20i32,
                s,
                crate::src::qcommon::q_math::colorWhite.as_mut_ptr(),
                crate::src::qcommon::q_shared::qfalse,
                crate::src::qcommon::q_shared::qtrue,
                cw,
                (cw as f32 * 1.5f32) as i32,
                0i32,
            );
        }
    } else {
        if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_FFA {
            s = b"Free For All\x00" as *const u8 as *const i8
        } else if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_TEAM {
            s = b"Team Deathmatch\x00" as *const u8 as *const i8
        } else if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_CTF {
            s = b"Capture the Flag\x00" as *const u8 as *const i8
        } else {
            s = b"\x00" as *const u8 as *const i8
        }
        w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s);
        if w > 640 / 32 {
            cw = 640 / w
        } else {
            cw = 32
        }
        crate::src::cgame::cg_drawtools::CG_DrawStringExt(
            320i32 - w * cw / 2i32,
            25i32,
            s,
            crate::src::qcommon::q_math::colorWhite.as_mut_ptr(),
            crate::src::qcommon::q_shared::qfalse,
            crate::src::qcommon::q_shared::qtrue,
            cw,
            (cw as f32 * 1.1f32) as i32,
            0i32,
        );
    }
    sec = (sec - crate::src::cgame::cg_main::cg.time) / 1000;
    if sec < 0 {
        crate::src::cgame::cg_main::cg.warmup = 0;
        sec = 0
    }
    s = crate::src::qcommon::q_shared::va(b"Starts in: %i\x00" as *const u8 as *mut i8, sec + 1i32);
    if sec != crate::src::cgame::cg_main::cg.warmupCount {
        crate::src::cgame::cg_main::cg.warmupCount = sec;
        match sec {
            0 => {
                crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                    crate::src::cgame::cg_main::cgs.media.count1Sound,
                    crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
                );
            }
            1 => {
                crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                    crate::src::cgame::cg_main::cgs.media.count2Sound,
                    crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
                );
            }
            2 => {
                crate::src::cgame::cg_syscalls::trap_S_StartLocalSound(
                    crate::src::cgame::cg_main::cgs.media.count3Sound,
                    crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
                );
            }
            _ => {}
        }
    }
    match crate::src::cgame::cg_main::cg.warmupCount {
        0 => cw = 28,
        1 => cw = 24,
        2 => cw = 20,
        _ => cw = 16,
    }
    w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(s);
    crate::src::cgame::cg_drawtools::CG_DrawStringExt(
        320 - w * cw / 2,
        70,
        s,
        crate::src::qcommon::q_math::colorWhite.as_mut_ptr(),
        crate::src::qcommon::q_shared::qfalse,
        crate::src::qcommon::q_shared::qtrue,
        cw,
        (cw as f64 * 1.5) as i32,
        0,
    );
}
//==================================================================================
/*
=================
CG_Draw2D
=================
*/

unsafe extern "C" fn CG_Draw2D(mut stereoFrame: crate::tr_types_h::stereoFrame_t) {
    // if we are taking a levelshot for the menu, don't draw anything
    if crate::src::cgame::cg_main::cg.levelShot as u64 != 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg_draw2D.integer == 0 {
        return;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_type
        == crate::bg_public_h::PM_INTERMISSION as i32
    {
        CG_DrawIntermission();
        return;
    }
    /*
        if (cg.cameraMode) {
            return;
        }
    */
    if (*crate::src::cgame::cg_main::cg.snap).ps.persistant[crate::bg_public_h::PERS_TEAM as usize]
        == crate::bg_public_h::TEAM_SPECTATOR as i32
    {
        CG_DrawSpectator();
        if stereoFrame == crate::tr_types_h::STEREO_CENTER {
            CG_DrawCrosshair();
        }
        CG_DrawCrosshairNames();
    } else if crate::src::cgame::cg_main::cg.showScores as u64 == 0
        && (*crate::src::cgame::cg_main::cg.snap).ps.stats[crate::bg_public_h::STAT_HEALTH as usize]
            > 0
    {
        CG_DrawStatusBar();
        CG_DrawAmmoWarning();
        if stereoFrame == crate::tr_types_h::STEREO_CENTER {
            CG_DrawCrosshair();
        }
        CG_DrawCrosshairNames();
        crate::src::cgame::cg_weapons::CG_DrawWeaponSelect();
        CG_DrawHoldableItem();
        CG_DrawReward();
    }
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
        CG_DrawTeamInfo();
    }
    CG_DrawVote();
    CG_DrawTeamVote();
    CG_DrawLagometer();
    CG_DrawUpperRight(stereoFrame);
    CG_DrawLowerRight();
    CG_DrawLowerLeft();
    if CG_DrawFollow() as u64 == 0 {
        CG_DrawWarmup();
    }
    // don't draw any status if dead or the scoreboard is being explicitly shown
    // don't draw center string if scoreboard is up
    crate::src::cgame::cg_main::cg.scoreBoardShowing = CG_DrawScoreboard();
    if crate::src::cgame::cg_main::cg.scoreBoardShowing as u64 == 0 {
        CG_DrawCenterString();
    };
}
/*
=====================
CG_DrawActive

Perform all drawing needed to completely fill the screen
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawActive(mut stereoView: crate::tr_types_h::stereoFrame_t) {
    // optionally draw the info screen instead
    if crate::src::cgame::cg_main::cg.snap.is_null() {
        crate::src::cgame::cg_info::CG_DrawInformation();
        return;
    }
    // optionally draw the tournement scoreboard instead
    if (*crate::src::cgame::cg_main::cg.snap).ps.persistant[crate::bg_public_h::PERS_TEAM as usize]
        == crate::bg_public_h::TEAM_SPECTATOR as i32
        && (*crate::src::cgame::cg_main::cg.snap).ps.pm_flags & 8192 != 0
    {
        crate::src::cgame::cg_scoreboard::CG_DrawTourneyScoreboard();
        return;
    }
    // clear around the rendered view if sized down
    crate::src::cgame::cg_drawtools::CG_TileClear();
    if stereoView != crate::tr_types_h::STEREO_CENTER {
        CG_DrawCrosshair3D();
    }
    // draw 3D view
    crate::src::cgame::cg_syscalls::trap_R_RenderScene(&mut crate::src::cgame::cg_main::cg.refdef);
    // draw status bar and other floating elements
    CG_Draw2D(stereoView);
}
