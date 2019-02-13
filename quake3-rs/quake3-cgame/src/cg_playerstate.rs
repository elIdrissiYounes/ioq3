use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, unnamed_0, unnamed_1, unnamed_2,
    unnamed_3, unnamed_4, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
    PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT,
    PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED,
    PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE,
    PM_INTERMISSION, PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN,
    PW_BATTLESUIT, PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS,
    PW_INVULNERABILITY, PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN,
    PW_SCOUT, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM,
    STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN,
    WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
    playerEntity_t, score_t, trap_S_StartLocalSound, FOOTSTEP_BOOT, FOOTSTEP_ENERGY,
    FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL, FOOTSTEP_SPLASH,
    FOOTSTEP_TOTAL,
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
use cg_predict::{CG_BuildSolidList, CG_PointContents, CG_PredictPlayerState, CG_Trace};
use cg_public_h::snapshot_t;
use cg_scoreboard::{CG_DrawOldScoreboard, CG_DrawTourneyScoreboard};
use cg_servercmds::{
    CG_ExecuteNewServerCommands, CG_ParseServerinfo, CG_SetConfigValues, CG_ShaderStateChanged,
};
use cg_snapshot::CG_ProcessSnapshots;
use cg_variadic_h::CG_Printf;
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
    byte, cvarHandle_t, entityState_s, entityState_t, gameState_t, playerState_s, playerState_t,
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, unnamed, vec3_t,
    vec_t, vmCvar_t, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND,
    CHAN_VOICE, CHAN_WEAPON, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE,
    TR_STATIONARY,
};
use stdlib::sqrt;
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
//
// cg_playerstate.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_Respawn() {
    cg.thisFrameTeleport = qtrue;
    cg.weaponSelectTime = cg.time;
    cg.weaponSelect = (*cg.snap).ps.weapon;
}
#[no_mangle]
pub unsafe extern "C" fn CG_TransitionPlayerState(
    mut ps: *mut playerState_t,
    mut ops: *mut playerState_t,
) {
    if (*ps).clientNum != (*ops).clientNum {
        cg.thisFrameTeleport = qtrue;
        *ops = *ps
    }
    if (*ps).damageEvent != (*ops).damageEvent && 0 != (*ps).damageCount {
        CG_DamageFeedback((*ps).damageYaw, (*ps).damagePitch, (*ps).damageCount);
    }
    if (*ps).persistant[PERS_SPAWN_COUNT as libc::c_int as usize]
        != (*ops).persistant[PERS_SPAWN_COUNT as libc::c_int as usize]
    {
        CG_Respawn();
    }
    if 0 != cg.mapRestart as u64 {
        CG_Respawn();
        cg.mapRestart = qfalse
    }
    if (*cg.snap).ps.pm_type != PM_INTERMISSION as libc::c_int
        && (*ps).persistant[PERS_TEAM as libc::c_int as usize] != TEAM_SPECTATOR as libc::c_int
    {
        CG_CheckLocalSounds(ps, ops);
    }
    CG_CheckAmmo();
    CG_CheckPlayerstateEvents(ps, ops);
    if (*ps).viewheight != (*ops).viewheight {
        cg.duckChange = ((*ps).viewheight - (*ops).viewheight) as libc::c_float;
        cg.duckTime = cg.time
    };
}
/*
==============
CG_CheckPlayerstateEvents
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CG_CheckPlayerstateEvents(
    mut ps: *mut playerState_t,
    mut ops: *mut playerState_t,
) {
    let mut i: libc::c_int = 0;
    let mut event: libc::c_int = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    if 0 != (*ps).externalEvent && (*ps).externalEvent != (*ops).externalEvent {
        cent = &mut cg_entities[(*ps).clientNum as usize] as *mut centity_t;
        (*cent).currentState.event = (*ps).externalEvent;
        (*cent).currentState.eventParm = (*ps).externalEventParm;
        CG_EntityEvent(cent, (*cent).lerpOrigin.as_mut_ptr());
    }
    cent = &mut cg.predictedPlayerEntity;
    i = (*ps).eventSequence - 2i32;
    while i < (*ps).eventSequence {
        if i >= (*ops).eventSequence
            || i > (*ops).eventSequence - 2i32
                && (*ps).events[(i & 2i32 - 1i32) as usize]
                    != (*ops).events[(i & 2i32 - 1i32) as usize]
        {
            event = (*ps).events[(i & 2i32 - 1i32) as usize];
            (*cent).currentState.event = event;
            (*cent).currentState.eventParm = (*ps).eventParms[(i & 2i32 - 1i32) as usize];
            CG_EntityEvent(cent, (*cent).lerpOrigin.as_mut_ptr());
            cg.predictableEvents[(i & 16i32 - 1i32) as usize] = event;
            cg.eventSequence += 1
        }
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
// cg_playerstate.c -- this file acts on changes in a new playerState_t
// With normal play, this will be done after local prediction, but when
// following another player or playing back a demo, it will be checked
// when the snapshot transitions like all the other entities
/*
==============
CG_CheckAmmo

If the ammo has gone low enough to generate the warning, play a sound
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CG_CheckAmmo() {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut previous: libc::c_int = 0;
    let mut weapons: libc::c_int = 0;
    weapons = (*cg.snap).ps.stats[STAT_WEAPONS as libc::c_int as usize];
    total = 0i32;
    i = WP_MACHINEGUN as libc::c_int;
    while i < WP_NUM_WEAPONS as libc::c_int {
        if !(0 == weapons & 1i32 << i) {
            if !((*cg.snap).ps.ammo[i as usize] < 0i32) {
                match i {
                    5 | 4 | 7 | 3 => total += (*cg.snap).ps.ammo[i as usize] * 1000i32,
                    _ => total += (*cg.snap).ps.ammo[i as usize] * 200i32,
                }
                if total >= 5000i32 {
                    cg.lowAmmoWarning = 0i32;
                    return;
                }
            }
        }
        i += 1
    }
    previous = cg.lowAmmoWarning;
    if total == 0i32 {
        cg.lowAmmoWarning = 2i32
    } else {
        cg.lowAmmoWarning = 1i32
    }
    if cg.lowAmmoWarning != previous {
        trap_S_StartLocalSound(cgs.media.noAmmoSound, CHAN_LOCAL_SOUND as libc::c_int);
    };
}
/*
==================
CG_CheckLocalSounds
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_CheckLocalSounds(
    mut ps: *mut playerState_t,
    mut ops: *mut playerState_t,
) {
    let mut highScore: libc::c_int = 0;
    let mut reward: libc::c_int = 0;
    let mut sfx: sfxHandle_t = 0;
    if (*ps).persistant[PERS_TEAM as libc::c_int as usize]
        != (*ops).persistant[PERS_TEAM as libc::c_int as usize]
    {
        return;
    }
    if (*ps).persistant[PERS_HITS as libc::c_int as usize]
        > (*ops).persistant[PERS_HITS as libc::c_int as usize]
    {
        trap_S_StartLocalSound(cgs.media.hitSound, CHAN_LOCAL_SOUND as libc::c_int);
    } else if (*ps).persistant[PERS_HITS as libc::c_int as usize]
        < (*ops).persistant[PERS_HITS as libc::c_int as usize]
    {
        trap_S_StartLocalSound(cgs.media.hitTeamSound, CHAN_LOCAL_SOUND as libc::c_int);
    }
    if (*ps).stats[STAT_HEALTH as libc::c_int as usize]
        < (*ops).stats[STAT_HEALTH as libc::c_int as usize] - 1i32
    {
        if (*ps).stats[STAT_HEALTH as libc::c_int as usize] > 0i32 {
            CG_PainEvent(
                &mut cg.predictedPlayerEntity,
                (*ps).stats[STAT_HEALTH as libc::c_int as usize],
            );
        }
    }
    if 0 != cg.intermissionStarted as u64 {
        return;
    }
    reward = qfalse as libc::c_int;
    if (*ps).persistant[PERS_CAPTURES as libc::c_int as usize]
        != (*ops).persistant[PERS_CAPTURES as libc::c_int as usize]
    {
        pushReward(
            cgs.media.captureAwardSound,
            cgs.media.medalCapture,
            (*ps).persistant[PERS_CAPTURES as libc::c_int as usize],
        );
        reward = qtrue as libc::c_int
    }
    if (*ps).persistant[PERS_IMPRESSIVE_COUNT as libc::c_int as usize]
        != (*ops).persistant[PERS_IMPRESSIVE_COUNT as libc::c_int as usize]
    {
        sfx = cgs.media.impressiveSound;
        pushReward(
            sfx,
            cgs.media.medalImpressive,
            (*ps).persistant[PERS_IMPRESSIVE_COUNT as libc::c_int as usize],
        );
        reward = qtrue as libc::c_int
    }
    if (*ps).persistant[PERS_EXCELLENT_COUNT as libc::c_int as usize]
        != (*ops).persistant[PERS_EXCELLENT_COUNT as libc::c_int as usize]
    {
        sfx = cgs.media.excellentSound;
        pushReward(
            sfx,
            cgs.media.medalExcellent,
            (*ps).persistant[PERS_EXCELLENT_COUNT as libc::c_int as usize],
        );
        reward = qtrue as libc::c_int
    }
    if (*ps).persistant[PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize]
        != (*ops).persistant[PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize]
    {
        sfx = cgs.media.humiliationSound;
        pushReward(
            sfx,
            cgs.media.medalGauntlet,
            (*ps).persistant[PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize],
        );
        reward = qtrue as libc::c_int
    }
    if (*ps).persistant[PERS_DEFEND_COUNT as libc::c_int as usize]
        != (*ops).persistant[PERS_DEFEND_COUNT as libc::c_int as usize]
    {
        pushReward(
            cgs.media.defendSound,
            cgs.media.medalDefend,
            (*ps).persistant[PERS_DEFEND_COUNT as libc::c_int as usize],
        );
        reward = qtrue as libc::c_int
    }
    if (*ps).persistant[PERS_ASSIST_COUNT as libc::c_int as usize]
        != (*ops).persistant[PERS_ASSIST_COUNT as libc::c_int as usize]
    {
        pushReward(
            cgs.media.assistSound,
            cgs.media.medalAssist,
            (*ps).persistant[PERS_ASSIST_COUNT as libc::c_int as usize],
        );
        reward = qtrue as libc::c_int
    }
    if (*ps).persistant[PERS_PLAYEREVENTS as libc::c_int as usize]
        != (*ops).persistant[PERS_PLAYEREVENTS as libc::c_int as usize]
    {
        if (*ps).persistant[PERS_PLAYEREVENTS as libc::c_int as usize] & 0x1i32
            != (*ops).persistant[PERS_PLAYEREVENTS as libc::c_int as usize] & 0x1i32
        {
            trap_S_StartLocalSound(cgs.media.deniedSound, CHAN_ANNOUNCER as libc::c_int);
        } else if (*ps).persistant[PERS_PLAYEREVENTS as libc::c_int as usize] & 0x2i32
            != (*ops).persistant[PERS_PLAYEREVENTS as libc::c_int as usize] & 0x2i32
        {
            trap_S_StartLocalSound(cgs.media.humiliationSound, CHAN_ANNOUNCER as libc::c_int);
        } else if (*ps).persistant[PERS_PLAYEREVENTS as libc::c_int as usize] & 0x4i32
            != (*ops).persistant[PERS_PLAYEREVENTS as libc::c_int as usize] & 0x4i32
        {
            trap_S_StartLocalSound(cgs.media.holyShitSound, CHAN_ANNOUNCER as libc::c_int);
        }
        reward = qtrue as libc::c_int
    }
    if cgs.gametype as libc::c_uint > GT_TEAM as libc::c_int as libc::c_uint {
        if (*ps).powerups[PW_REDFLAG as libc::c_int as usize]
            != (*ops).powerups[PW_REDFLAG as libc::c_int as usize]
            && 0 != (*ps).powerups[PW_REDFLAG as libc::c_int as usize]
            || (*ps).powerups[PW_BLUEFLAG as libc::c_int as usize]
                != (*ops).powerups[PW_BLUEFLAG as libc::c_int as usize]
                && 0 != (*ps).powerups[PW_BLUEFLAG as libc::c_int as usize]
            || (*ps).powerups[PW_NEUTRALFLAG as libc::c_int as usize]
                != (*ops).powerups[PW_NEUTRALFLAG as libc::c_int as usize]
                && 0 != (*ps).powerups[PW_NEUTRALFLAG as libc::c_int as usize]
        {
            trap_S_StartLocalSound(cgs.media.youHaveFlagSound, CHAN_ANNOUNCER as libc::c_int);
        }
    }
    if 0 == reward {
        if 0 == cg.warmup {
            if (*ps).persistant[PERS_RANK as libc::c_int as usize]
                != (*ops).persistant[PERS_RANK as libc::c_int as usize]
            {
                if (cgs.gametype as libc::c_uint) < GT_TEAM as libc::c_int as libc::c_uint {
                    if (*ps).persistant[PERS_RANK as libc::c_int as usize] == 0i32 {
                        CG_AddBufferedSound(cgs.media.takenLeadSound);
                    } else if (*ps).persistant[PERS_RANK as libc::c_int as usize] == 0x4000i32 {
                        CG_AddBufferedSound(cgs.media.tiedLeadSound);
                    } else if (*ops).persistant[PERS_RANK as libc::c_int as usize] & !0x4000i32
                        == 0i32
                    {
                        CG_AddBufferedSound(cgs.media.lostLeadSound);
                    }
                }
            }
        }
    }
    if cgs.timelimit > 0i32 {
        let mut msec: libc::c_int = 0;
        msec = cg.time - cgs.levelStartTime;
        if 0 == cg.timelimitWarnings & 4i32 && msec > (cgs.timelimit * 60i32 + 2i32) * 1000i32 {
            cg.timelimitWarnings |= 1i32 | 2i32 | 4i32;
            trap_S_StartLocalSound(cgs.media.suddenDeathSound, CHAN_ANNOUNCER as libc::c_int);
        } else if 0 == cg.timelimitWarnings & 2i32
            && msec > (cgs.timelimit - 1i32) * 60i32 * 1000i32
        {
            cg.timelimitWarnings |= 1i32 | 2i32;
            trap_S_StartLocalSound(cgs.media.oneMinuteSound, CHAN_ANNOUNCER as libc::c_int);
        } else if cgs.timelimit > 5i32
            && 0 == cg.timelimitWarnings & 1i32
            && msec > (cgs.timelimit - 5i32) * 60i32 * 1000i32
        {
            cg.timelimitWarnings |= 1i32;
            trap_S_StartLocalSound(cgs.media.fiveMinuteSound, CHAN_ANNOUNCER as libc::c_int);
        }
    }
    if cgs.fraglimit > 0i32
        && (cgs.gametype as libc::c_uint) < GT_CTF as libc::c_int as libc::c_uint
    {
        highScore = cgs.scores1;
        if cgs.gametype as libc::c_uint == GT_TEAM as libc::c_int as libc::c_uint
            && cgs.scores2 > highScore
        {
            highScore = cgs.scores2
        }
        if 0 == cg.fraglimitWarnings & 4i32 && highScore == cgs.fraglimit - 1i32 {
            cg.fraglimitWarnings |= 1i32 | 2i32 | 4i32;
            CG_AddBufferedSound(cgs.media.oneFragSound);
        } else if cgs.fraglimit > 2i32
            && 0 == cg.fraglimitWarnings & 2i32
            && highScore == cgs.fraglimit - 2i32
        {
            cg.fraglimitWarnings |= 1i32 | 2i32;
            CG_AddBufferedSound(cgs.media.twoFragSound);
        } else if cgs.fraglimit > 3i32
            && 0 == cg.fraglimitWarnings & 1i32
            && highScore == cgs.fraglimit - 3i32
        {
            cg.fraglimitWarnings |= 1i32;
            CG_AddBufferedSound(cgs.media.threeFragSound);
        }
    };
}
/*
==================
pushReward
==================
*/
unsafe extern "C" fn pushReward(
    mut sfx: sfxHandle_t,
    mut shader: qhandle_t,
    mut rewardCount: libc::c_int,
) {
    if cg.rewardStack < 10i32 - 1i32 {
        cg.rewardStack += 1;
        cg.rewardSound[cg.rewardStack as usize] = sfx;
        cg.rewardShader[cg.rewardStack as usize] = shader;
        cg.rewardCount[cg.rewardStack as usize] = rewardCount
    };
}
/*
==============
CG_DamageFeedback
==============
*/
#[no_mangle]
pub unsafe extern "C" fn CG_DamageFeedback(
    mut yawByte: libc::c_int,
    mut pitchByte: libc::c_int,
    mut damage: libc::c_int,
) {
    let mut left: libc::c_float = 0.;
    let mut front: libc::c_float = 0.;
    let mut up: libc::c_float = 0.;
    let mut kick: libc::c_float = 0.;
    let mut health: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    cg.attackerTime = cg.time;
    health = (*cg.snap).ps.stats[STAT_HEALTH as libc::c_int as usize];
    if health < 40i32 {
        scale = 1i32 as libc::c_float
    } else {
        scale = (40.0f64 / health as libc::c_double) as libc::c_float
    }
    kick = damage as libc::c_float * scale;
    if kick < 5i32 as libc::c_float {
        kick = 5i32 as libc::c_float
    }
    if kick > 10i32 as libc::c_float {
        kick = 10i32 as libc::c_float
    }
    if yawByte == 255i32 && pitchByte == 255i32 {
        cg.damageX = 0i32 as libc::c_float;
        cg.damageY = 0i32 as libc::c_float;
        cg.v_dmg_roll = 0i32 as libc::c_float;
        cg.v_dmg_pitch = -kick
    } else {
        pitch =
            (pitchByte as libc::c_double / 255.0f64 * 360i32 as libc::c_double) as libc::c_float;
        yaw = (yawByte as libc::c_double / 255.0f64 * 360i32 as libc::c_double) as libc::c_float;
        angles[0usize] = pitch;
        angles[1usize] = yaw;
        angles[2usize] = 0i32 as vec_t;
        AngleVectors(
            angles.as_mut_ptr() as *const vec_t,
            dir.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        dir[0usize] = vec3_origin[0usize] - dir[0usize];
        dir[1usize] = vec3_origin[1usize] - dir[1usize];
        dir[2usize] = vec3_origin[2usize] - dir[2usize];
        front = dir[0usize] * cg.refdef.viewaxis[0usize][0usize]
            + dir[1usize] * cg.refdef.viewaxis[0usize][1usize]
            + dir[2usize] * cg.refdef.viewaxis[0usize][2usize];
        left = dir[0usize] * cg.refdef.viewaxis[1usize][0usize]
            + dir[1usize] * cg.refdef.viewaxis[1usize][1usize]
            + dir[2usize] * cg.refdef.viewaxis[1usize][2usize];
        up = dir[0usize] * cg.refdef.viewaxis[2usize][0usize]
            + dir[1usize] * cg.refdef.viewaxis[2usize][1usize]
            + dir[2usize] * cg.refdef.viewaxis[2usize][2usize];
        dir[0usize] = front;
        dir[1usize] = left;
        dir[2usize] = 0i32 as vec_t;
        dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
        if (dist as libc::c_double) < 0.1f64 {
            dist = 0.1f32
        }
        cg.v_dmg_roll = kick * left;
        cg.v_dmg_pitch = -kick * front;
        if front as libc::c_double <= 0.1f64 {
            front = 0.1f32
        }
        cg.damageX = -left / front;
        cg.damageY = up / dist
    }
    if cg.damageX as libc::c_double > 1.0f64 {
        cg.damageX = 1.0f64 as libc::c_float
    }
    if (cg.damageX as libc::c_double) < -1.0f64 {
        cg.damageX = -1.0f64 as libc::c_float
    }
    if cg.damageY as libc::c_double > 1.0f64 {
        cg.damageY = 1.0f64 as libc::c_float
    }
    if (cg.damageY as libc::c_double) < -1.0f64 {
        cg.damageY = -1.0f64 as libc::c_float
    }
    if kick > 10i32 as libc::c_float {
        kick = 10i32 as libc::c_float
    }
    cg.damageValue = kick;
    cg.v_dmg_time = (cg.time + 500i32) as libc::c_float;
    cg.damageTime = (*cg.snap).serverTime as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CG_CheckChangedPredictableEvents(mut ps: *mut playerState_t) {
    let mut i: libc::c_int = 0;
    let mut event: libc::c_int = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    cent = &mut cg.predictedPlayerEntity;
    i = (*ps).eventSequence - 2i32;
    while i < (*ps).eventSequence {
        //
        if !(i >= cg.eventSequence) {
            if i > cg.eventSequence - 16i32 {
                if (*ps).events[(i & 2i32 - 1i32) as usize]
                    != cg.predictableEvents[(i & 16i32 - 1i32) as usize]
                {
                    event = (*ps).events[(i & 2i32 - 1i32) as usize];
                    (*cent).currentState.event = event;
                    (*cent).currentState.eventParm = (*ps).eventParms[(i & 2i32 - 1i32) as usize];
                    CG_EntityEvent(cent, (*cent).lerpOrigin.as_mut_ptr());
                    cg.predictableEvents[(i & 16i32 - 1i32) as usize] = event;
                    if 0 != cg_showmiss.integer {
                        CG_Printf(
                            b"WARNING: changed predicted event\n\x00" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
            }
        }
        i += 1
    }
}
