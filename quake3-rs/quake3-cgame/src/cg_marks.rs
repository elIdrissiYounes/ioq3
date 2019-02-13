use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, GENDER_FEMALE, GENDER_MALE,
    GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED,
    TEAM_SPECTATOR,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
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
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, lerpFrame_t,
    markPoly_s, markPoly_t, playerEntity_t, score_t, trap_CM_MarkFragments, trap_R_AddPolyToScene,
    FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL,
    FOOTSTEP_SPLASH, FOOTSTEP_TOTAL,
};
use cg_localents::{CG_AddLocalEntities, CG_AllocLocalEntity, CG_InitLocalEntities};
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
use q_math::{
    axisDefault, colorWhite, g_color_table, vec3_origin, vectoangles, AngleMod, AngleNormalize180,
    AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis, AxisClear, AxisCopy, ByteToDir,
    LerpAngle, MatrixMultiply, PerpendicularVector, Q_crandom, Q_random, RotateAroundDirection,
    RotatePointAroundVector, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cvarHandle_t, entityState_s, entityState_t, gameState_t, markFragment_t, playerState_s,
    playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, vec3_t,
    vec_t, vmCvar_t, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{memcpy, memset};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, polyVert_t, poly_s, poly_t, refEntityType_t,
    refEntity_t, refdef_t, textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO,
    GLHW_3DFX_2D3D, GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM,
    RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE, RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE,
    RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};

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
#[no_mangle]
pub static mut cg_markPolys: [markPoly_t; 256] = [markPoly_s {
    prevMark: 0 as *const markPoly_s as *mut markPoly_s,
    nextMark: 0 as *const markPoly_s as *mut markPoly_s,
    time: 0,
    markShader: 0,
    alphaFade: qfalse,
    color: [0.; 4],
    poly: poly_s {
        hShader: 0,
        numVerts: 0,
        verts: 0 as *const polyVert_t as *mut polyVert_t,
    },
    verts: [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 10],
}; 256];
//
// cg_marks.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_InitMarkPolys() {
    let mut i: libc::c_int = 0;
    memset(
        cg_markPolys.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[markPoly_t; 256]>() as libc::c_ulong,
    );
    cg_activeMarkPolys.nextMark = &mut cg_activeMarkPolys;
    cg_activeMarkPolys.prevMark = &mut cg_activeMarkPolys;
    cg_freeMarkPolys = cg_markPolys.as_mut_ptr();
    i = 0i32;
    while i < 256i32 - 1i32 {
        cg_markPolys[i as usize].nextMark =
            &mut cg_markPolys[(i + 1i32) as usize] as *mut markPoly_t;
        i += 1
    }
}
// single linked list
#[no_mangle]
pub static mut cg_freeMarkPolys: *mut markPoly_t = 0 as *const markPoly_t as *mut markPoly_t;
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
// cg_marks.c -- wall marks
/*
===================================================================

MARK POLYS

===================================================================
*/
// double linked list
#[no_mangle]
pub static mut cg_activeMarkPolys: markPoly_t = markPoly_s {
    prevMark: 0 as *const markPoly_s as *mut markPoly_s,
    nextMark: 0 as *const markPoly_s as *mut markPoly_s,
    time: 0,
    markShader: 0,
    alphaFade: qfalse,
    color: [0.; 4],
    poly: poly_s {
        hShader: 0,
        numVerts: 0,
        verts: 0 as *const polyVert_t as *mut polyVert_t,
    },
    verts: [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 10],
};
#[no_mangle]
pub unsafe extern "C" fn CG_AddMarks() {
    let mut j: libc::c_int = 0;
    let mut mp: *mut markPoly_t = 0 as *mut markPoly_t;
    let mut next: *mut markPoly_t = 0 as *mut markPoly_t;
    let mut t: libc::c_int = 0;
    let mut fade: libc::c_int = 0;
    if 0 == cg_addMarks.integer {
        return;
    }
    mp = cg_activeMarkPolys.nextMark;
    while mp != &mut cg_activeMarkPolys as *mut markPoly_t {
        next = (*mp).nextMark;
        // see if it is time to completely remove it
        if cg.time > (*mp).time + 10000i32 {
            CG_FreeMarkPoly(mp);
        } else {
            if (*mp).markShader == cgs.media.energyMarkShader {
                fade = (450i32 as libc::c_double
                    - 450i32 as libc::c_double
                        * ((cg.time - (*mp).time) as libc::c_double / 3000.0f64))
                    as libc::c_int;
                if fade < 255i32 {
                    if fade < 0i32 {
                        fade = 0i32
                    }
                    if (*mp).verts[0usize].modulate[0usize] as libc::c_int != 0i32 {
                        j = 0i32;
                        while j < (*mp).poly.numVerts {
                            (*mp).verts[j as usize].modulate[0usize] =
                                ((*mp).color[0usize] * fade as libc::c_float) as byte;
                            (*mp).verts[j as usize].modulate[1usize] =
                                ((*mp).color[1usize] * fade as libc::c_float) as byte;
                            (*mp).verts[j as usize].modulate[2usize] =
                                ((*mp).color[2usize] * fade as libc::c_float) as byte;
                            j += 1
                        }
                    }
                }
            }
            t = (*mp).time + 10000i32 - cg.time;
            if t < 1000i32 {
                fade = 255i32 * t / 1000i32;
                if 0 != (*mp).alphaFade as u64 {
                    j = 0i32;
                    while j < (*mp).poly.numVerts {
                        (*mp).verts[j as usize].modulate[3usize] = fade as byte;
                        j += 1
                    }
                } else {
                    j = 0i32;
                    while j < (*mp).poly.numVerts {
                        (*mp).verts[j as usize].modulate[0usize] =
                            ((*mp).color[0usize] * fade as libc::c_float) as byte;
                        (*mp).verts[j as usize].modulate[1usize] =
                            ((*mp).color[1usize] * fade as libc::c_float) as byte;
                        (*mp).verts[j as usize].modulate[2usize] =
                            ((*mp).color[2usize] * fade as libc::c_float) as byte;
                        j += 1
                    }
                }
            }
            trap_R_AddPolyToScene(
                (*mp).markShader,
                (*mp).poly.numVerts,
                (*mp).verts.as_mut_ptr(),
            );
        }
        mp = next
    }
}
/*
==================
CG_FreeMarkPoly
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_FreeMarkPoly(mut le: *mut markPoly_t) {
    if (*le).prevMark.is_null() || (*le).nextMark.is_null() {
        CG_Error(b"CG_FreeLocalEntity: not active\x00" as *const u8 as *const libc::c_char);
    }
    (*(*le).prevMark).nextMark = (*le).nextMark;
    (*(*le).nextMark).prevMark = (*le).prevMark;
    (*le).nextMark = cg_freeMarkPolys;
    cg_freeMarkPolys = le;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ImpactMark(
    mut markShader: qhandle_t,
    mut origin: *const vec_t,
    mut dir: *const vec_t,
    mut orientation: libc::c_float,
    mut red: libc::c_float,
    mut green: libc::c_float,
    mut blue: libc::c_float,
    mut alpha: libc::c_float,
    mut alphaFade: qboolean,
    mut radius: libc::c_float,
    mut temporary: qboolean,
) {
    let mut axis: [vec3_t; 3] = [[0.; 3]; 3];
    let mut texCoordScale: libc::c_float = 0.;
    let mut originalPoints: [vec3_t; 4] = [[0.; 3]; 4];
    let mut colors: [byte; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numFragments: libc::c_int = 0;
    let mut markFragments: [markFragment_t; 128] = [markFragment_t {
        firstPoint: 0,
        numPoints: 0,
    }; 128];
    let mut mf: *mut markFragment_t = 0 as *mut markFragment_t;
    let mut markPoints: [vec3_t; 384] = [[0.; 3]; 384];
    let mut projection: vec3_t = [0.; 3];
    if 0 == cg_addMarks.integer {
        return;
    }
    if radius <= 0i32 as libc::c_float {
        CG_Error(b"CG_ImpactMark called with <= 0 radius\x00" as *const u8 as *const libc::c_char);
    }
    VectorNormalize2(dir, axis[0usize].as_mut_ptr());
    PerpendicularVector(
        axis[1usize].as_mut_ptr(),
        axis[0usize].as_mut_ptr() as *const vec_t,
    );
    RotatePointAroundVector(
        axis[2usize].as_mut_ptr(),
        axis[0usize].as_mut_ptr() as *const vec_t,
        axis[1usize].as_mut_ptr() as *const vec_t,
        orientation,
    );
    CrossProduct(
        axis[0usize].as_mut_ptr() as *const vec_t,
        axis[2usize].as_mut_ptr() as *const vec_t,
        axis[1usize].as_mut_ptr(),
    );
    texCoordScale = (0.5f64 * 1.0f64 / radius as libc::c_double) as libc::c_float;
    i = 0i32;
    while i < 3i32 {
        originalPoints[0usize][i as usize] = *origin.offset(i as isize)
            - radius * axis[1usize][i as usize]
            - radius * axis[2usize][i as usize];
        originalPoints[1usize][i as usize] = *origin.offset(i as isize)
            + radius * axis[1usize][i as usize]
            - radius * axis[2usize][i as usize];
        originalPoints[2usize][i as usize] = *origin.offset(i as isize)
            + radius * axis[1usize][i as usize]
            + radius * axis[2usize][i as usize];
        originalPoints[3usize][i as usize] = *origin.offset(i as isize)
            - radius * axis[1usize][i as usize]
            + radius * axis[2usize][i as usize];
        i += 1
    }
    projection[0usize] = *dir.offset(0isize) * -20i32 as libc::c_float;
    projection[1usize] = *dir.offset(1isize) * -20i32 as libc::c_float;
    projection[2usize] = *dir.offset(2isize) * -20i32 as libc::c_float;
    numFragments = trap_CM_MarkFragments(
        4i32,
        originalPoints.as_mut_ptr() as *mut libc::c_void as *const vec3_t,
        projection.as_mut_ptr() as *const vec_t,
        384i32,
        markPoints[0usize].as_mut_ptr(),
        128i32,
        markFragments.as_mut_ptr(),
    );
    colors[0usize] = (red * 255i32 as libc::c_float) as byte;
    colors[1usize] = (green * 255i32 as libc::c_float) as byte;
    colors[2usize] = (blue * 255i32 as libc::c_float) as byte;
    colors[3usize] = (alpha * 255i32 as libc::c_float) as byte;
    i = 0i32;
    mf = markFragments.as_mut_ptr();
    while i < numFragments {
        let mut v: *mut polyVert_t = 0 as *mut polyVert_t;
        let mut verts: [polyVert_t; 10] = [polyVert_t {
            xyz: [0.; 3],
            st: [0.; 2],
            modulate: [0; 4],
        }; 10];
        let mut mark: *mut markPoly_t = 0 as *mut markPoly_t;
        if (*mf).numPoints > 10i32 {
            (*mf).numPoints = 10i32
        }
        j = 0i32;
        v = verts.as_mut_ptr();
        while j < (*mf).numPoints {
            let mut delta: vec3_t = [0.; 3];
            (*v).xyz[0usize] = markPoints[((*mf).firstPoint + j) as usize][0usize];
            (*v).xyz[1usize] = markPoints[((*mf).firstPoint + j) as usize][1usize];
            (*v).xyz[2usize] = markPoints[((*mf).firstPoint + j) as usize][2usize];
            delta[0usize] = (*v).xyz[0usize] - *origin.offset(0isize);
            delta[1usize] = (*v).xyz[1usize] - *origin.offset(1isize);
            delta[2usize] = (*v).xyz[2usize] - *origin.offset(2isize);
            (*v).st[0usize] = (0.5f64
                + ((delta[0usize] * axis[1usize][0usize]
                    + delta[1usize] * axis[1usize][1usize]
                    + delta[2usize] * axis[1usize][2usize])
                    * texCoordScale) as libc::c_double)
                as libc::c_float;
            (*v).st[1usize] = (0.5f64
                + ((delta[0usize] * axis[2usize][0usize]
                    + delta[1usize] * axis[2usize][1usize]
                    + delta[2usize] * axis[2usize][2usize])
                    * texCoordScale) as libc::c_double)
                as libc::c_float;
            *((*v).modulate.as_mut_ptr() as *mut libc::c_int) =
                *(colors.as_mut_ptr() as *mut libc::c_int);
            j += 1;
            v = v.offset(1isize)
        }
        // if it is a temporary (shadow) mark, add it immediately and forget about it
        if 0 != temporary as u64 {
            trap_R_AddPolyToScene(markShader, (*mf).numPoints, verts.as_mut_ptr());
        } else {
            mark = CG_AllocMark();
            (*mark).time = cg.time;
            (*mark).alphaFade = alphaFade;
            (*mark).markShader = markShader;
            (*mark).poly.numVerts = (*mf).numPoints;
            (*mark).color[0usize] = red;
            (*mark).color[1usize] = green;
            (*mark).color[2usize] = blue;
            (*mark).color[3usize] = alpha;
            memcpy(
                (*mark).verts.as_mut_ptr() as *mut libc::c_void,
                verts.as_mut_ptr() as *const libc::c_void,
                ((*mf).numPoints as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<polyVert_t>() as libc::c_ulong),
            );
            markTotal += 1
        }
        i += 1;
        mf = mf.offset(1isize)
    }
}
static mut markTotal: libc::c_int = 0;
/*
===================
CG_AllocMark

Will allways succeed, even if it requires freeing an old active mark
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_AllocMark() -> *mut markPoly_t {
    let mut le: *mut markPoly_t = 0 as *mut markPoly_t;
    let mut time: libc::c_int = 0;
    if cg_freeMarkPolys.is_null() {
        time = (*cg_activeMarkPolys.prevMark).time;
        while !cg_activeMarkPolys.prevMark.is_null() && time == (*cg_activeMarkPolys.prevMark).time
        {
            CG_FreeMarkPoly(cg_activeMarkPolys.prevMark);
        }
    }
    le = cg_freeMarkPolys;
    cg_freeMarkPolys = (*cg_freeMarkPolys).nextMark;
    memset(
        le as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<markPoly_t>() as libc::c_ulong,
    );
    (*le).nextMark = cg_activeMarkPolys.nextMark;
    (*le).prevMark = &mut cg_activeMarkPolys;
    (*cg_activeMarkPolys.nextMark).prevMark = le;
    cg_activeMarkPolys.nextMark = le;
    return le;
}
