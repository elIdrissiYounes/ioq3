use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, unnamed, BG_PlayerStateToEntityState,
    ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER,
    ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, GENDER_FEMALE,
    GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE,
    GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS,
    TEAM_RED, TEAM_SPECTATOR,
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
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, lerpFrame_t,
    playerEntity_t, score_t, trap_GetCurrentSnapshotNumber, trap_GetSnapshot, FOOTSTEP_BOOT,
    FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL,
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
use cg_variadic_h::{CG_Error, CG_Printf};
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
    byte, cvarHandle_t, entityState_s, entityState_t, gameState_t, playerState_s, playerState_t,
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, vec3_t, vec_t,
    vmCvar_t, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::memcpy;
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

//
// cg_snapshot.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_ProcessSnapshots() {
    let mut snap: *mut snapshot_t = 0 as *mut snapshot_t;
    let mut n: libc::c_int = 0;
    trap_GetCurrentSnapshotNumber(&mut n, &mut cg.latestSnapshotTime);
    if n != cg.latestSnapshotNum {
        if n < cg.latestSnapshotNum {
            CG_Error(
                b"CG_ProcessSnapshots: n < cg.latestSnapshotNum\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        cg.latestSnapshotNum = n
    }
    while cg.snap.is_null() {
        snap = CG_ReadNextSnapshot();
        if snap.is_null() {
            return;
        }
        if 0 == (*snap).snapFlags & 2i32 {
            CG_SetInitialSnapshot(snap);
        }
    }
    loop {
        // if we don't have a nextframe, try and read a new one in
        if cg.nextSnap.is_null() {
            snap = CG_ReadNextSnapshot();
            // if we still don't have a nextframe, we will just have to
            // extrapolate
            if snap.is_null() {
                break;
            }
            CG_SetNextSnap(snap);
            if (*cg.nextSnap).serverTime < (*cg.snap).serverTime {
                CG_Error(
                    b"CG_ProcessSnapshots: Server time went backwards\x00" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        // if our time is < nextFrame's, we have a nice interpolating state
        if cg.time >= (*cg.snap).serverTime && cg.time < (*cg.nextSnap).serverTime {
            break;
        }
        CG_TransitionSnapshot();
    }
    if cg.snap.is_null() {
        CG_Error(b"CG_ProcessSnapshots: cg.snap == NULL\x00" as *const u8 as *const libc::c_char);
    }
    if cg.time < (*cg.snap).serverTime {
        cg.time = (*cg.snap).serverTime
    }
    if !cg.nextSnap.is_null() && (*cg.nextSnap).serverTime <= cg.time {
        CG_Error(
            b"CG_ProcessSnapshots: cg.nextSnap->serverTime <= cg.time\x00" as *const u8
                as *const libc::c_char,
        );
    };
}
/*
===================
CG_TransitionSnapshot

The transition point from snap to nextSnap has passed
===================
*/
unsafe extern "C" fn CG_TransitionSnapshot() {
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut oldFrame: *mut snapshot_t = 0 as *mut snapshot_t;
    let mut i: libc::c_int = 0;
    if cg.snap.is_null() {
        CG_Error(b"CG_TransitionSnapshot: NULL cg.snap\x00" as *const u8 as *const libc::c_char);
    }
    if cg.nextSnap.is_null() {
        CG_Error(
            b"CG_TransitionSnapshot: NULL cg.nextSnap\x00" as *const u8 as *const libc::c_char,
        );
    }
    CG_ExecuteNewServerCommands((*cg.nextSnap).serverCommandSequence);
    0 != cg.mapRestart as u64;
    i = 0i32;
    while i < (*cg.snap).numEntities {
        cent = &mut cg_entities[(*cg.snap).entities[i as usize].number as usize] as *mut centity_t;
        (*cent).currentValid = qfalse;
        i += 1
    }
    oldFrame = cg.snap;
    cg.snap = cg.nextSnap;
    BG_PlayerStateToEntityState(
        &mut (*cg.snap).ps,
        &mut cg_entities[(*cg.snap).ps.clientNum as usize].currentState,
        qfalse,
    );
    cg_entities[(*cg.snap).ps.clientNum as usize].interpolate = qfalse;
    i = 0i32;
    while i < (*cg.snap).numEntities {
        cent = &mut cg_entities[(*cg.snap).entities[i as usize].number as usize] as *mut centity_t;
        CG_TransitionEntity(cent);
        (*cent).snapShotTime = (*cg.snap).serverTime;
        i += 1
    }
    cg.nextSnap = 0 as *mut snapshot_t;
    if !oldFrame.is_null() {
        let mut ops: *mut playerState_t = 0 as *mut playerState_t;
        let mut ps: *mut playerState_t = 0 as *mut playerState_t;
        ops = &mut (*oldFrame).ps;
        ps = &mut (*cg.snap).ps;
        if 0 != ((*ps).eFlags ^ (*ops).eFlags) & 0x4i32 {
            cg.thisFrameTeleport = qtrue
        }
        if 0 != cg.demoPlayback as libc::c_uint
            || 0 != (*cg.snap).ps.pm_flags & 4096i32
            || 0 != cg_nopredict.integer
            || 0 != cg_synchronousClients.integer
        {
            CG_TransitionPlayerState(ps, ops);
        }
    };
}
/*
===============
CG_TransitionEntity

cent->nextState is moved to cent->currentState and events are fired
===============
*/
unsafe extern "C" fn CG_TransitionEntity(mut cent: *mut centity_t) {
    (*cent).currentState = (*cent).nextState;
    (*cent).currentValid = qtrue;
    if 0 == (*cent).interpolate as u64 {
        CG_ResetEntity(cent);
    }
    (*cent).interpolate = qfalse;
    CG_CheckEvents(cent);
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
// cg_snapshot.c -- things that happen on snapshot transition,
// not necessarily every single rendered frame
/*
==================
CG_ResetEntity
==================
*/
unsafe extern "C" fn CG_ResetEntity(mut cent: *mut centity_t) {
    if (*cent).snapShotTime < cg.time - 300i32 {
        (*cent).previousEvent = 0i32
    }
    (*cent).trailTime = (*cg.snap).serverTime;
    (*cent).lerpOrigin[0usize] = (*cent).currentState.origin[0usize];
    (*cent).lerpOrigin[1usize] = (*cent).currentState.origin[1usize];
    (*cent).lerpOrigin[2usize] = (*cent).currentState.origin[2usize];
    (*cent).lerpAngles[0usize] = (*cent).currentState.angles[0usize];
    (*cent).lerpAngles[1usize] = (*cent).currentState.angles[1usize];
    (*cent).lerpAngles[2usize] = (*cent).currentState.angles[2usize];
    if (*cent).currentState.eType == ET_PLAYER as libc::c_int {
        CG_ResetPlayerEntity(cent);
    };
}
/*
===================
CG_SetNextSnap

A new snapshot has just been read in from the client system.
===================
*/
unsafe extern "C" fn CG_SetNextSnap(mut snap: *mut snapshot_t) {
    let mut num: libc::c_int = 0;
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    cg.nextSnap = snap;
    BG_PlayerStateToEntityState(
        &mut (*snap).ps,
        &mut cg_entities[(*snap).ps.clientNum as usize].nextState,
        qfalse,
    );
    cg_entities[(*cg.snap).ps.clientNum as usize].interpolate = qtrue;
    num = 0i32;
    while num < (*snap).numEntities {
        es = &mut (*snap).entities[num as usize] as *mut entityState_t;
        cent = &mut cg_entities[(*es).number as usize] as *mut centity_t;
        memcpy(
            &mut (*cent).nextState as *mut entityState_t as *mut libc::c_void,
            es as *const libc::c_void,
            ::std::mem::size_of::<entityState_t>() as libc::c_ulong,
        );
        if 0 == (*cent).currentValid as u64
            || 0 != ((*cent).currentState.eFlags ^ (*es).eFlags) & 0x4i32
        {
            (*cent).interpolate = qfalse
        } else {
            (*cent).interpolate = qtrue
        }
        num += 1
    }
    if !cg.snap.is_null() && 0 != ((*snap).ps.eFlags ^ (*cg.snap).ps.eFlags) & 0x4i32 {
        cg.nextFrameTeleport = qtrue
    } else {
        cg.nextFrameTeleport = qfalse
    }
    if (*cg.nextSnap).ps.clientNum != (*cg.snap).ps.clientNum {
        cg.nextFrameTeleport = qtrue
    }
    if 0 != ((*cg.nextSnap).snapFlags ^ (*cg.snap).snapFlags) & 4i32 {
        cg.nextFrameTeleport = qtrue
    }
    CG_BuildSolidList();
}
/*
========================
CG_ReadNextSnapshot

This is the only place new snapshots are requested
This may increment cgs.processedSnapshotNum multiple
times if the client system fails to return a
valid snapshot.
========================
*/
unsafe extern "C" fn CG_ReadNextSnapshot() -> *mut snapshot_t {
    let mut r: qboolean = qfalse;
    let mut dest: *mut snapshot_t = 0 as *mut snapshot_t;
    if cg.latestSnapshotNum > cgs.processedSnapshotNum + 1000i32 {
        CG_Printf(
            b"WARNING: CG_ReadNextSnapshot: way out of range, %i > %i\n\x00" as *const u8
                as *const libc::c_char,
            cg.latestSnapshotNum,
            cgs.processedSnapshotNum,
        );
    }
    while cgs.processedSnapshotNum < cg.latestSnapshotNum {
        if cg.snap == &mut cg.activeSnapshots[0usize] as *mut snapshot_t {
            dest = &mut cg.activeSnapshots[1usize] as *mut snapshot_t
        } else {
            dest = &mut cg.activeSnapshots[0usize] as *mut snapshot_t
        }
        cgs.processedSnapshotNum += 1;
        r = trap_GetSnapshot(cgs.processedSnapshotNum, dest);
        !cg.snap.is_null() && 0 != r as libc::c_uint && (*dest).serverTime == (*cg.snap).serverTime;
        if 0 != r as u64 {
            CG_AddLagometerSnapshotInfo(dest);
            return dest;
        }
        CG_AddLagometerSnapshotInfo(0 as *mut snapshot_t);
    }
    return 0 as *mut snapshot_t;
}
/*
==================
CG_SetInitialSnapshot

This will only happen on the very first snapshot.
All other times will use CG_TransitionSnapshot instead.
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_SetInitialSnapshot(mut snap: *mut snapshot_t) {
    let mut i: libc::c_int = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut state: *mut entityState_t = 0 as *mut entityState_t;
    cg.snap = snap;
    BG_PlayerStateToEntityState(
        &mut (*snap).ps,
        &mut cg_entities[(*snap).ps.clientNum as usize].currentState,
        qfalse,
    );
    CG_BuildSolidList();
    CG_ExecuteNewServerCommands((*snap).serverCommandSequence);
    CG_Respawn();
    i = 0i32;
    while i < (*cg.snap).numEntities {
        state = &mut (*cg.snap).entities[i as usize] as *mut entityState_t;
        cent = &mut cg_entities[(*state).number as usize] as *mut centity_t;
        memcpy(
            &mut (*cent).currentState as *mut entityState_t as *mut libc::c_void,
            state as *const libc::c_void,
            ::std::mem::size_of::<entityState_t>() as libc::c_ulong,
        );
        (*cent).interpolate = qfalse;
        (*cent).currentValid = qtrue;
        CG_ResetEntity(cent);
        CG_CheckEvents(cent);
        i += 1
    }
}
