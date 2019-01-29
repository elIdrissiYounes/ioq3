use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, BG_EvaluateTrajectory,
    BG_EvaluateTrajectoryDelta, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF,
    GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
    TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
};
use cg_consolecmds::{CG_ConsoleCommand, CG_InitConsoleCommands};
use cg_draw::{
    drawTeamOverlayModificationCount, numSortedTeamPlayers, sortedTeamPlayers,
    CG_AddLagometerFrameInfo, CG_AddLagometerSnapshotInfo, CG_CenterPrint, CG_DrawActive,
    CG_DrawFlagModel, CG_DrawHead, CG_DrawTeamBackground,
};
use cg_drawtools::{
    CG_AdjustFrom640, CG_ColorForHealth, CG_DrawBigString, CG_DrawBigStringColor, CG_DrawPic,
    CG_DrawSmallString, CG_DrawSmallStringColor, CG_DrawStringExt, CG_DrawStrlen, CG_FadeColor,
    CG_FillRect, CG_GetColorForHealth, CG_TileClear, UI_DrawProportionalString,
};
use cg_effects::{
    CG_Bleed, CG_BubbleTrail, CG_GibPlayer, CG_MakeExplosion, CG_ScorePlum, CG_SmokePuff,
    CG_SpawnEffect,
};
use cg_ents::{
    CG_AddPacketEntities, CG_AdjustPositionForMover, CG_Beam, CG_PositionEntityOnTag,
    CG_PositionRotatedEntityOnTag, CG_SetEntitySoundPosition,
};
use cg_event::{CG_CheckEvents, CG_EntityEvent, CG_PainEvent, CG_PlaceString};
use cg_info::{CG_DrawInformation, CG_LoadingClient, CG_LoadingItem, CG_LoadingString};
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, leBounceSoundType_t,
    leMarkType_t, leType_t, lerpFrame_t, localEntity_s, localEntity_t, playerEntity_t, score_t,
    trap_R_AddLightToScene, trap_R_AddRefEntityToScene, trap_S_StartSound, unnamed_0,
    FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL,
    FOOTSTEP_SPLASH, FOOTSTEP_TOTAL, LEBS_BLOOD, LEBS_BRASS, LEBS_NONE, LEF_PUFF_DONT_SCALE,
    LEF_SOUND1, LEF_SOUND2, LEF_TUMBLE, LEMT_BLOOD, LEMT_BURN, LEMT_NONE, LE_EXPLOSION,
    LE_FADE_RGB, LE_FALL_SCALE_FADE, LE_FRAGMENT, LE_MARK, LE_MOVE_SCALE_FADE, LE_SCALE_FADE,
    LE_SCOREPLUM, LE_SPRITE_EXPLOSION,
};
use cg_main::{
    cg, cg_addMarks, cg_animSpeed, cg_autoswitch, cg_blood, cg_bobpitch, cg_bobroll, cg_bobup,
    cg_brassTime, cg_buildScript, cg_cameraMode, cg_cameraOrbit, cg_cameraOrbitDelay,
    cg_centertime, cg_crosshairHealth, cg_crosshairSize, cg_crosshairX, cg_crosshairY,
    cg_debugAnim, cg_debugEvents, cg_debugPosition, cg_deferPlayers, cg_draw2D, cg_draw3dIcons,
    cg_drawAmmoWarning, cg_drawAttacker, cg_drawCrosshair, cg_drawCrosshairNames, cg_drawFPS,
    cg_drawFriend, cg_drawGun, cg_drawIcons, cg_drawRewards, cg_drawSnapshot, cg_drawStatus,
    cg_drawTeamOverlay, cg_drawTimer, cg_entities, cg_errorDecay, cg_footsteps, cg_forceModel,
    cg_fov, cg_gibs, cg_gun_frame, cg_gun_x, cg_gun_y, cg_gun_z, cg_items, cg_lagometer,
    cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail, cg_oldRocket,
    cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll, cg_scorePlum,
    cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats, cg_swingSpeed,
    cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly, cg_thirdPerson,
    cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale, cg_timescaleFadeEnd,
    cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength, cg_tracerWidth, cg_trueLightning,
    cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed, pmove_msec, CG_Argv,
    CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer, CG_LastAttacker, CG_StartMusic,
    CG_UpdateCvars,
};
use cg_marks::{CG_AddMarks, CG_ImpactMark, CG_InitMarkPolys};
use cg_particles::{CG_AddParticles, CG_ClearParticles, CG_ParticleExplosion};
use cg_players::{
    CG_AddRefEntityWithPowerups, CG_CustomSound, CG_LoadDeferredPlayers, CG_NewClientInfo,
    CG_Player, CG_ResetPlayerEntity,
};
use cg_playerstate::{CG_Respawn, CG_TransitionPlayerState};
use cg_predict::{CG_BuildSolidList, CG_PointContents, CG_PredictPlayerState, CG_Trace};
use cg_public_h::snapshot_t;
use cg_scoreboard::{CG_DrawOldScoreboard, CG_DrawTourneyScoreboard};
use cg_servercmds::{
    CG_ExecuteNewServerCommands, CG_ParseServerinfo, CG_SetConfigValues, CG_ShaderStateChanged,
};
use cg_snapshot::CG_ProcessSnapshots;
use cg_variadic_h::CG_Error;
use cg_view::{
    CG_AddBufferedSound, CG_DrawActiveFrame, CG_TestGun_f, CG_TestModelNextFrame_f,
    CG_TestModelNextSkin_f, CG_TestModelPrevFrame_f, CG_TestModelPrevSkin_f, CG_TestModel_f,
    CG_ZoomDown_f, CG_ZoomUp_f,
};
use cg_weapons::{
    CG_AddPlayerWeapon, CG_AddViewWeapon, CG_Bullet, CG_DrawWeaponSelect, CG_FireWeapon,
    CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f, CG_OutOfAmmoChange,
    CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire, CG_Weapon_f,
};
use libc;
use q_shared_h::{
    byte, cplane_s, cplane_t, entityState_s, entityState_t, gameState_t, playerState_s,
    playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trace_t,
    trajectory_t, unnamed, vec3_origin, vec3_t, vec_t, AnglesToAxis, VectorNormalize,
    CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE,
    CHAN_WEAPON, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{memset, rand, sin, sqrt};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
unsafe extern "C" fn CrossProduct(
    mut v1: *const vec_t,
    mut v2: *const vec_t,
    mut cross: *mut vec_t,
) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) - *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) - *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) - *v1.offset(1isize) * *v2.offset(0isize);
}
//
// cg_localents.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_InitLocalEntities() {
    let mut i: libc::c_int = 0;
    memset(
        cg_localEntities.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[localEntity_t; 512]>() as libc::c_ulong,
    );
    cg_activeLocalEntities.next = &mut cg_activeLocalEntities;
    cg_activeLocalEntities.prev = &mut cg_activeLocalEntities;
    cg_freeLocalEntities = cg_localEntities.as_mut_ptr();
    i = 0i32;
    while i < 512i32 - 1i32 {
        cg_localEntities[i as usize].next =
            &mut cg_localEntities[(i + 1i32) as usize] as *mut localEntity_t;
        i += 1
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
//
// cg_localents.c -- every frame, generate renderer commands for locally
// processed entities, like smoke puffs, gibs, shells, etc.
#[no_mangle]
pub static mut cg_localEntities: [localEntity_t; 512] = [localEntity_s {
    prev: 0 as *const localEntity_s as *mut localEntity_s,
    next: 0 as *const localEntity_s as *mut localEntity_s,
    leType: LE_MARK,
    leFlags: 0,
    startTime: 0,
    endTime: 0,
    fadeInTime: 0,
    lifeRate: 0.,
    pos: trajectory_t {
        trType: TR_STATIONARY,
        trTime: 0,
        trDuration: 0,
        trBase: [0.; 3],
        trDelta: [0.; 3],
    },
    angles: trajectory_t {
        trType: TR_STATIONARY,
        trTime: 0,
        trDuration: 0,
        trBase: [0.; 3],
        trDelta: [0.; 3],
    },
    bounceFactor: 0.,
    color: [0.; 4],
    radius: 0.,
    light: 0.,
    lightColor: [0.; 3],
    leMarkType: LEMT_NONE,
    leBounceSoundType: LEBS_NONE,
    refEntity: refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    },
}; 512];
// single linked list
#[no_mangle]
pub static mut cg_freeLocalEntities: *mut localEntity_t =
    0 as *const localEntity_t as *mut localEntity_t;
// double linked list
#[no_mangle]
pub static mut cg_activeLocalEntities: localEntity_t = localEntity_s {
    prev: 0 as *const localEntity_s as *mut localEntity_s,
    next: 0 as *const localEntity_s as *mut localEntity_s,
    leType: LE_MARK,
    leFlags: 0,
    startTime: 0,
    endTime: 0,
    fadeInTime: 0,
    lifeRate: 0.,
    pos: trajectory_t {
        trType: TR_STATIONARY,
        trTime: 0,
        trDuration: 0,
        trBase: [0.; 3],
        trDelta: [0.; 3],
    },
    angles: trajectory_t {
        trType: TR_STATIONARY,
        trTime: 0,
        trDuration: 0,
        trBase: [0.; 3],
        trDelta: [0.; 3],
    },
    bounceFactor: 0.,
    color: [0.; 4],
    radius: 0.,
    light: 0.,
    lightColor: [0.; 3],
    leMarkType: LEMT_NONE,
    leBounceSoundType: LEBS_NONE,
    refEntity: refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    },
};
#[no_mangle]
pub unsafe extern "C" fn CG_AllocLocalEntity() -> *mut localEntity_t {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    if cg_freeLocalEntities.is_null() {
        CG_FreeLocalEntity(cg_activeLocalEntities.prev);
    }
    le = cg_freeLocalEntities;
    cg_freeLocalEntities = (*cg_freeLocalEntities).next;
    memset(
        le as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<localEntity_t>() as libc::c_ulong,
    );
    (*le).next = cg_activeLocalEntities.next;
    (*le).prev = &mut cg_activeLocalEntities;
    (*cg_activeLocalEntities.next).prev = le;
    cg_activeLocalEntities.next = le;
    return le;
}
/*
==================
CG_FreeLocalEntity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_FreeLocalEntity(mut le: *mut localEntity_t) {
    if (*le).prev.is_null() {
        CG_Error(b"CG_FreeLocalEntity: not active\x00" as *const u8 as *const libc::c_char);
    }
    (*(*le).prev).next = (*le).next;
    (*(*le).next).prev = (*le).prev;
    (*le).next = cg_freeLocalEntities;
    cg_freeLocalEntities = le;
}
#[no_mangle]
pub unsafe extern "C" fn CG_AddLocalEntities() {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut next: *mut localEntity_t = 0 as *mut localEntity_t;
    le = cg_activeLocalEntities.prev;
    while le != &mut cg_activeLocalEntities as *mut localEntity_t {
        next = (*le).prev;
        if cg.time >= (*le).endTime {
            CG_FreeLocalEntity(le);
        } else {
            match (*le).leType as libc::c_uint {
                0 => {}
                2 => {
                    CG_AddSpriteExplosion(le);
                }
                1 => {
                    CG_AddExplosion(le);
                }
                3 => {
                    CG_AddFragment(le);
                }
                4 => {
                    CG_AddMoveScaleFade(le);
                }
                6 => {
                    CG_AddFadeRGB(le);
                }
                5 => {
                    CG_AddFallScaleFade(le);
                }
                7 => {
                    CG_AddScaleFade(le);
                }
                8 => {
                    CG_AddScorePlum(le);
                }
                _ => {
                    CG_Error(
                        b"Bad leType: %i\x00" as *const u8 as *const libc::c_char,
                        (*le).leType as libc::c_uint,
                    );
                }
            }
        }
        le = next
    }
}
/*
===================
CG_AddScorePlum
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_AddScorePlum(mut le: *mut localEntity_t) {
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut origin: vec3_t = [0.; 3];
    let mut delta: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut c: libc::c_float = 0.;
    let mut len: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut digits: [libc::c_int; 10] = [0; 10];
    let mut numdigits: libc::c_int = 0;
    let mut negative: libc::c_int = 0;
    re = &mut (*le).refEntity;
    c = ((*le).endTime - cg.time) as libc::c_float * (*le).lifeRate;
    score = (*le).radius as libc::c_int;
    if score < 0i32 {
        (*re).shaderRGBA[0usize] = 0xffi32 as byte;
        (*re).shaderRGBA[1usize] = 0x11i32 as byte;
        (*re).shaderRGBA[2usize] = 0x11i32 as byte
    } else {
        (*re).shaderRGBA[0usize] = 0xffi32 as byte;
        (*re).shaderRGBA[1usize] = 0xffi32 as byte;
        (*re).shaderRGBA[2usize] = 0xffi32 as byte;
        if score >= 50i32 {
            (*re).shaderRGBA[1usize] = 0i32 as byte
        } else if score >= 20i32 {
            (*re).shaderRGBA[1usize] = 0i32 as byte;
            (*re).shaderRGBA[0usize] = (*re).shaderRGBA[1usize]
        } else if score >= 10i32 {
            (*re).shaderRGBA[2usize] = 0i32 as byte
        } else if score >= 2i32 {
            (*re).shaderRGBA[2usize] = 0i32 as byte;
            (*re).shaderRGBA[0usize] = (*re).shaderRGBA[2usize]
        }
    }
    if (c as libc::c_double) < 0.25f64 {
        (*re).shaderRGBA[3usize] = ((0xffi32 * 4i32) as libc::c_float * c) as byte
    } else {
        (*re).shaderRGBA[3usize] = 0xffi32 as byte
    }
    (*re).radius = (8i32 / 2i32) as libc::c_float;
    origin[0usize] = (*le).pos.trBase[0usize];
    origin[1usize] = (*le).pos.trBase[1usize];
    origin[2usize] = (*le).pos.trBase[2usize];
    origin[2usize] += 110i32 as libc::c_float - c * 100i32 as libc::c_float;
    dir[0usize] = cg.refdef.vieworg[0usize] - origin[0usize];
    dir[1usize] = cg.refdef.vieworg[1usize] - origin[1usize];
    dir[2usize] = cg.refdef.vieworg[2usize] - origin[2usize];
    CrossProduct(
        dir.as_mut_ptr() as *const vec_t,
        up.as_mut_ptr() as *const vec_t,
        vec.as_mut_ptr(),
    );
    VectorNormalize(vec.as_mut_ptr());
    origin[0usize] = (origin[0usize] as libc::c_double
        + vec[0usize] as libc::c_double
            * (-10i32 as libc::c_double
                + 20i32 as libc::c_double
                    * sin(
                        (c * 2i32 as libc::c_float) as libc::c_double * 3.14159265358979323846f64
                    ))) as vec_t;
    origin[1usize] = (origin[1usize] as libc::c_double
        + vec[1usize] as libc::c_double
            * (-10i32 as libc::c_double
                + 20i32 as libc::c_double
                    * sin(
                        (c * 2i32 as libc::c_float) as libc::c_double * 3.14159265358979323846f64
                    ))) as vec_t;
    origin[2usize] = (origin[2usize] as libc::c_double
        + vec[2usize] as libc::c_double
            * (-10i32 as libc::c_double
                + 20i32 as libc::c_double
                    * sin(
                        (c * 2i32 as libc::c_float) as libc::c_double * 3.14159265358979323846f64
                    ))) as vec_t;
    delta[0usize] = origin[0usize] - cg.refdef.vieworg[0usize];
    delta[1usize] = origin[1usize] - cg.refdef.vieworg[1usize];
    delta[2usize] = origin[2usize] - cg.refdef.vieworg[2usize];
    len = VectorLength(delta.as_mut_ptr() as *const vec_t);
    if len < 20i32 as libc::c_float {
        CG_FreeLocalEntity(le);
        return;
    }
    negative = qfalse as libc::c_int;
    if score < 0i32 {
        negative = qtrue as libc::c_int;
        score = -score
    }
    numdigits = 0i32;
    while !(0 != numdigits && 0 == score) {
        digits[numdigits as usize] = score % 10i32;
        score = score / 10i32;
        numdigits += 1
    }
    if 0 != negative {
        digits[numdigits as usize] = 10i32;
        numdigits += 1
    }
    i = 0i32;
    while i < numdigits {
        (*re).origin[0usize] = origin[0usize]
            + vec[0usize]
                * ((numdigits as libc::c_float / 2i32 as libc::c_float - i as libc::c_float)
                    * 8i32 as libc::c_float);
        (*re).origin[1usize] = origin[1usize]
            + vec[1usize]
                * ((numdigits as libc::c_float / 2i32 as libc::c_float - i as libc::c_float)
                    * 8i32 as libc::c_float);
        (*re).origin[2usize] = origin[2usize]
            + vec[2usize]
                * ((numdigits as libc::c_float / 2i32 as libc::c_float - i as libc::c_float)
                    * 8i32 as libc::c_float);
        (*re).customShader =
            cgs.media.numberShaders[digits[(numdigits - 1i32 - i) as usize] as usize];
        trap_R_AddRefEntityToScene(re);
        i += 1
    }
}
/*
===================
CG_AddScaleFade

For rocket smokes that hang in place, fade out, and are
removed if the view passes through them.
There are often many of these, so it needs to be simple.
===================
*/
unsafe extern "C" fn CG_AddScaleFade(mut le: *mut localEntity_t) {
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut c: libc::c_float = 0.;
    let mut delta: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    re = &mut (*le).refEntity;
    c = ((*le).endTime - cg.time) as libc::c_float * (*le).lifeRate;
    (*re).shaderRGBA[3usize] = (0xffi32 as libc::c_float * c * (*le).color[3usize]) as byte;
    (*re).radius = ((*le).radius as libc::c_double * (1.0f64 - c as libc::c_double)
        + 8i32 as libc::c_double) as libc::c_float;
    delta[0usize] = (*re).origin[0usize] - cg.refdef.vieworg[0usize];
    delta[1usize] = (*re).origin[1usize] - cg.refdef.vieworg[1usize];
    delta[2usize] = (*re).origin[2usize] - cg.refdef.vieworg[2usize];
    len = VectorLength(delta.as_mut_ptr() as *const vec_t);
    if len < (*le).radius {
        CG_FreeLocalEntity(le);
        return;
    }
    trap_R_AddRefEntityToScene(re);
}
/*
=================
CG_AddFallScaleFade

This is just an optimized CG_AddMoveScaleFade
For blood mists that drift down, fade out, and are
removed if the view passes through them.
There are often 100+ of these, so it needs to be simple.
=================
*/
unsafe extern "C" fn CG_AddFallScaleFade(mut le: *mut localEntity_t) {
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut c: libc::c_float = 0.;
    let mut delta: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    re = &mut (*le).refEntity;
    c = ((*le).endTime - cg.time) as libc::c_float * (*le).lifeRate;
    (*re).shaderRGBA[3usize] = (0xffi32 as libc::c_float * c * (*le).color[3usize]) as byte;
    (*re).origin[2usize] = ((*le).pos.trBase[2usize] as libc::c_double
        - (1.0f64 - c as libc::c_double) * (*le).pos.trDelta[2usize] as libc::c_double)
        as libc::c_float;
    (*re).radius = ((*le).radius as libc::c_double * (1.0f64 - c as libc::c_double)
        + 16i32 as libc::c_double) as libc::c_float;
    delta[0usize] = (*re).origin[0usize] - cg.refdef.vieworg[0usize];
    delta[1usize] = (*re).origin[1usize] - cg.refdef.vieworg[1usize];
    delta[2usize] = (*re).origin[2usize] - cg.refdef.vieworg[2usize];
    len = VectorLength(delta.as_mut_ptr() as *const vec_t);
    if len < (*le).radius {
        CG_FreeLocalEntity(le);
        return;
    }
    trap_R_AddRefEntityToScene(re);
}
/*
=====================================================================

TRIVIAL LOCAL ENTITIES

These only do simple scaling or modulation before passing to the renderer
=====================================================================
*/
/*
====================
CG_AddFadeRGB
====================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_AddFadeRGB(mut le: *mut localEntity_t) {
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut c: libc::c_float = 0.;
    re = &mut (*le).refEntity;
    c = ((*le).endTime - cg.time) as libc::c_float * (*le).lifeRate;
    c *= 0xffi32 as libc::c_float;
    (*re).shaderRGBA[0usize] = ((*le).color[0usize] * c) as byte;
    (*re).shaderRGBA[1usize] = ((*le).color[1usize] * c) as byte;
    (*re).shaderRGBA[2usize] = ((*le).color[2usize] * c) as byte;
    (*re).shaderRGBA[3usize] = ((*le).color[3usize] * c) as byte;
    trap_R_AddRefEntityToScene(re);
}
/*
==================
CG_AddMoveScaleFade
==================
*/
unsafe extern "C" fn CG_AddMoveScaleFade(mut le: *mut localEntity_t) {
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut c: libc::c_float = 0.;
    let mut delta: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    re = &mut (*le).refEntity;
    if (*le).fadeInTime > (*le).startTime && cg.time < (*le).fadeInTime {
        c = (1.0f64
            - (((*le).fadeInTime - cg.time) as libc::c_float
                / ((*le).fadeInTime - (*le).startTime) as libc::c_float)
                as libc::c_double) as libc::c_float
    } else {
        c = ((*le).endTime - cg.time) as libc::c_float * (*le).lifeRate
    }
    (*re).shaderRGBA[3usize] = (0xffi32 as libc::c_float * c * (*le).color[3usize]) as byte;
    if 0 == (*le).leFlags & LEF_PUFF_DONT_SCALE as libc::c_int {
        (*re).radius = ((*le).radius as libc::c_double * (1.0f64 - c as libc::c_double)
            + 8i32 as libc::c_double) as libc::c_float
    }
    BG_EvaluateTrajectory(&mut (*le).pos, cg.time, (*re).origin.as_mut_ptr());
    delta[0usize] = (*re).origin[0usize] - cg.refdef.vieworg[0usize];
    delta[1usize] = (*re).origin[1usize] - cg.refdef.vieworg[1usize];
    delta[2usize] = (*re).origin[2usize] - cg.refdef.vieworg[2usize];
    len = VectorLength(delta.as_mut_ptr() as *const vec_t);
    if len < (*le).radius {
        CG_FreeLocalEntity(le);
        return;
    }
    trap_R_AddRefEntityToScene(re);
}
/*
================
CG_AddFragment
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_AddFragment(mut le: *mut localEntity_t) {
    let mut newOrigin: vec3_t = [0.; 3];
    let mut trace: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_s {
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
    if (*le).pos.trType as libc::c_uint == TR_STATIONARY as libc::c_int as libc::c_uint {
        let mut t: libc::c_int = 0;
        let mut oldZ: libc::c_float = 0.;
        t = (*le).endTime - cg.time;
        if t < 1000i32 {
            (*le).refEntity.lightingOrigin[0usize] = (*le).refEntity.origin[0usize];
            (*le).refEntity.lightingOrigin[1usize] = (*le).refEntity.origin[1usize];
            (*le).refEntity.lightingOrigin[2usize] = (*le).refEntity.origin[2usize];
            (*le).refEntity.renderfx |= 0x80i32;
            oldZ = (*le).refEntity.origin[2usize];
            (*le).refEntity.origin[2usize] = ((*le).refEntity.origin[2usize] as libc::c_double
                - 16i32 as libc::c_double
                    * (1.0f64 - (t as libc::c_float / 1000i32 as libc::c_float) as libc::c_double))
                as libc::c_float;
            trap_R_AddRefEntityToScene(&mut (*le).refEntity);
            (*le).refEntity.origin[2usize] = oldZ
        } else {
            trap_R_AddRefEntityToScene(&mut (*le).refEntity);
        }
        return;
    }
    BG_EvaluateTrajectory(&mut (*le).pos, cg.time, newOrigin.as_mut_ptr());
    CG_Trace(
        &mut trace,
        (*le).refEntity.origin.as_mut_ptr() as *const vec_t,
        0 as *const vec_t,
        0 as *const vec_t,
        newOrigin.as_mut_ptr() as *const vec_t,
        -1i32,
        1i32,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        (*le).refEntity.origin[0usize] = newOrigin[0usize];
        (*le).refEntity.origin[1usize] = newOrigin[1usize];
        (*le).refEntity.origin[2usize] = newOrigin[2usize];
        if 0 != (*le).leFlags & LEF_TUMBLE as libc::c_int {
            let mut angles: vec3_t = [0.; 3];
            BG_EvaluateTrajectory(&mut (*le).angles, cg.time, angles.as_mut_ptr());
            AnglesToAxis(
                angles.as_mut_ptr() as *const vec_t,
                (*le).refEntity.axis.as_mut_ptr(),
            );
        }
        trap_R_AddRefEntityToScene(&mut (*le).refEntity);
        if (*le).leBounceSoundType as libc::c_uint == LEBS_BLOOD as libc::c_int as libc::c_uint {
            CG_BloodTrail(le);
        }
        return;
    }
    if 0 != CG_PointContents(trace.endpos.as_mut_ptr() as *const vec_t, 0i32) as libc::c_uint
        & 0x80000000u32
    {
        CG_FreeLocalEntity(le);
        return;
    }
    CG_FragmentBounceMark(le, &mut trace);
    CG_FragmentBounceSound(le, &mut trace);
    CG_ReflectVelocity(le, &mut trace);
    trap_R_AddRefEntityToScene(&mut (*le).refEntity);
}
/*
================
CG_ReflectVelocity
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_ReflectVelocity(mut le: *mut localEntity_t, mut trace: *mut trace_t) {
    let mut velocity: vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    let mut hitTime: libc::c_int = 0;
    hitTime = ((cg.time - cg.frametime) as libc::c_float
        + cg.frametime as libc::c_float * (*trace).fraction) as libc::c_int;
    BG_EvaluateTrajectoryDelta(&mut (*le).pos, hitTime, velocity.as_mut_ptr());
    dot = velocity[0usize] * (*trace).plane.normal[0usize]
        + velocity[1usize] * (*trace).plane.normal[1usize]
        + velocity[2usize] * (*trace).plane.normal[2usize];
    (*le).pos.trDelta[0usize] =
        velocity[0usize] + (*trace).plane.normal[0usize] * (-2i32 as libc::c_float * dot);
    (*le).pos.trDelta[1usize] =
        velocity[1usize] + (*trace).plane.normal[1usize] * (-2i32 as libc::c_float * dot);
    (*le).pos.trDelta[2usize] =
        velocity[2usize] + (*trace).plane.normal[2usize] * (-2i32 as libc::c_float * dot);
    (*le).pos.trDelta[0usize] = (*le).pos.trDelta[0usize] * (*le).bounceFactor;
    (*le).pos.trDelta[1usize] = (*le).pos.trDelta[1usize] * (*le).bounceFactor;
    (*le).pos.trDelta[2usize] = (*le).pos.trDelta[2usize] * (*le).bounceFactor;
    (*le).pos.trBase[0usize] = (*trace).endpos[0usize];
    (*le).pos.trBase[1usize] = (*trace).endpos[1usize];
    (*le).pos.trBase[2usize] = (*trace).endpos[2usize];
    (*le).pos.trTime = cg.time;
    if 0 != (*trace).allsolid as libc::c_uint
        || (*trace).plane.normal[2usize] > 0i32 as libc::c_float
            && ((*le).pos.trDelta[2usize] < 40i32 as libc::c_float
                || (*le).pos.trDelta[2usize]
                    < -cg.frametime as libc::c_float * (*le).pos.trDelta[2usize])
    {
        (*le).pos.trType = TR_STATIONARY
    };
}
/*
================
CG_FragmentBounceSound
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_FragmentBounceSound(
    mut le: *mut localEntity_t,
    mut trace: *mut trace_t,
) {
    if (*le).leBounceSoundType as libc::c_uint == LEBS_BLOOD as libc::c_int as libc::c_uint {
        if 0 != rand() & 1i32 {
            let mut r: libc::c_int = rand() & 3i32;
            let mut s: sfxHandle_t = 0;
            if r == 0i32 {
                s = cgs.media.gibBounce1Sound
            } else if r == 1i32 {
                s = cgs.media.gibBounce2Sound
            } else {
                s = cgs.media.gibBounce3Sound
            }
            trap_S_StartSound(
                (*trace).endpos.as_mut_ptr(),
                (1i32 << 10i32) - 2i32,
                CHAN_AUTO as libc::c_int,
                s,
            );
        }
    } else {
        (*le).leBounceSoundType as libc::c_uint == LEBS_BRASS as libc::c_int as libc::c_uint;
    }
    (*le).leBounceSoundType = LEBS_NONE;
}
/*
================
CG_FragmentBounceMark
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_FragmentBounceMark(
    mut le: *mut localEntity_t,
    mut trace: *mut trace_t,
) {
    let mut radius: libc::c_int = 0;
    if (*le).leMarkType as libc::c_uint == LEMT_BLOOD as libc::c_int as libc::c_uint {
        radius = 16i32 + (rand() & 31i32);
        CG_ImpactMark(
            cgs.media.bloodMarkShader,
            (*trace).endpos.as_mut_ptr() as *const vec_t,
            (*trace).plane.normal.as_mut_ptr() as *const vec_t,
            (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                * 360i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            qtrue,
            radius as libc::c_float,
            qfalse,
        );
    } else if (*le).leMarkType as libc::c_uint == LEMT_BURN as libc::c_int as libc::c_uint {
        radius = 8i32 + (rand() & 15i32);
        CG_ImpactMark(
            cgs.media.burnMarkShader,
            (*trace).endpos.as_mut_ptr() as *const vec_t,
            (*trace).plane.normal.as_mut_ptr() as *const vec_t,
            (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                * 360i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            qtrue,
            radius as libc::c_float,
            qfalse,
        );
    }
    (*le).leMarkType = LEMT_NONE;
}
/*
====================================================================================

FRAGMENT PROCESSING

A fragment localentity interacts with the environment in some way (hitting walls),
or generates more localentities along a trail.

====================================================================================
*/
/*
================
CG_BloodTrail

Leave expanding blood puffs behind gibs
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_BloodTrail(mut le: *mut localEntity_t) {
    let mut t: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut newOrigin: vec3_t = [0.; 3];
    let mut blood: *mut localEntity_t = 0 as *mut localEntity_t;
    step = 150i32;
    t = step * ((cg.time - cg.frametime + step) / step);
    t2 = step * (cg.time / step);
    while t <= t2 {
        BG_EvaluateTrajectory(&mut (*le).pos, t, newOrigin.as_mut_ptr());
        blood = CG_SmokePuff(
            newOrigin.as_mut_ptr() as *const vec_t,
            vec3_origin.as_mut_ptr() as *const vec_t,
            20i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            2000i32 as libc::c_float,
            t,
            0i32,
            0i32,
            cgs.media.bloodTrailShader,
        );
        (*blood).leType = LE_FALL_SCALE_FADE;
        (*blood).pos.trDelta[2usize] = 40i32 as vec_t;
        t += step
    }
}
/*
================
CG_AddExplosion
================
*/
unsafe extern "C" fn CG_AddExplosion(mut ex: *mut localEntity_t) {
    let mut ent: *mut refEntity_t = 0 as *mut refEntity_t;
    ent = &mut (*ex).refEntity;
    trap_R_AddRefEntityToScene(ent);
    if 0. != (*ex).light {
        let mut light: libc::c_float = 0.;
        light = (cg.time - (*ex).startTime) as libc::c_float
            / ((*ex).endTime - (*ex).startTime) as libc::c_float;
        if (light as libc::c_double) < 0.5f64 {
            light = 1.0f64 as libc::c_float
        } else {
            light = (1.0f64 - (light as libc::c_double - 0.5f64) * 2i32 as libc::c_double)
                as libc::c_float
        }
        light = (*ex).light * light;
        trap_R_AddLightToScene(
            (*ent).origin.as_mut_ptr() as *const vec_t,
            light,
            (*ex).lightColor[0usize],
            (*ex).lightColor[1usize],
            (*ex).lightColor[2usize],
        );
    };
}
/*
================
CG_AddSpriteExplosion
================
*/
unsafe extern "C" fn CG_AddSpriteExplosion(mut le: *mut localEntity_t) {
    let mut re: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut c: libc::c_float = 0.;
    re = (*le).refEntity;
    c = ((*le).endTime - cg.time) as libc::c_float
        / ((*le).endTime - (*le).startTime) as libc::c_float;
    if c > 1i32 as libc::c_float {
        c = 1.0f64 as libc::c_float
    }
    re.shaderRGBA[0usize] = 0xffi32 as byte;
    re.shaderRGBA[1usize] = 0xffi32 as byte;
    re.shaderRGBA[2usize] = 0xffi32 as byte;
    re.shaderRGBA[3usize] = ((0xffi32 as libc::c_float * c) as libc::c_double * 0.33f64) as byte;
    re.reType = RT_SPRITE;
    re.radius = (42i32 as libc::c_double * (1.0f64 - c as libc::c_double) + 30i32 as libc::c_double)
        as libc::c_float;
    trap_R_AddRefEntityToScene(&mut re);
    if 0. != (*le).light {
        let mut light: libc::c_float = 0.;
        light = (cg.time - (*le).startTime) as libc::c_float
            / ((*le).endTime - (*le).startTime) as libc::c_float;
        if (light as libc::c_double) < 0.5f64 {
            light = 1.0f64 as libc::c_float
        } else {
            light = (1.0f64 - (light as libc::c_double - 0.5f64) * 2i32 as libc::c_double)
                as libc::c_float
        }
        light = (*le).light * light;
        trap_R_AddLightToScene(
            re.origin.as_mut_ptr() as *const vec_t,
            light,
            (*le).lightColor[0usize],
            (*le).lightColor[1usize],
            (*le).lightColor[2usize],
        );
    };
}
