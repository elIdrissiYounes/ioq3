#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, gitem_s, gitem_t, itemType_t, pmove_t, team_t,
    unnamed, unnamed_0, unnamed_1, unnamed_2, unnamed_3, unnamed_4, ET_BEAM, ET_EVENTS, ET_GENERAL,
    ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER,
    ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL,
    EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM,
    EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE,
    EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND,
    EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP, EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED,
    EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT, EV_MISSILE_HIT, EV_MISSILE_MISS,
    EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE, EV_OBELISKPAIN, EV_OBITUARY,
    EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT, EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD,
    EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK, EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL,
    EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16, EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND,
    EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME, EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO,
    EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0, EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11,
    EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14, EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3,
    EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6, EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9,
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, GENDER_FEMALE, GENDER_MALE,
    GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR,
    PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT,
    PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY,
    PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
    playerEntity_t, score_t, trap_CM_BoxTrace, trap_CM_InlineModel, trap_CM_PointContents,
    trap_CM_TempBoxModel, trap_CM_TransformedBoxTrace, trap_CM_TransformedPointContents,
    trap_Cvar_Set, trap_Cvar_Update, trap_GetCurrentCmdNumber, trap_GetUserCmd, FOOTSTEP_BOOT,
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
use q_math::{
    axisDefault, colorWhite, g_color_table, vec3_origin, vectoangles, AngleMod, AngleNormalize180,
    AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis, AxisClear, AxisCopy, ByteToDir,
    LerpAngle, MatrixMultiply, PerpendicularVector, Q_crandom, Q_random, RotateAroundDirection,
    RotatePointAroundVector, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, clipHandle_t, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t,
    gameState_t, playerState_s, playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t,
    trType_t, trace_t, trajectory_t, usercmd_s, usercmd_t, vec3_t, vec_t, vmCvar_t, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::sqrt;
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};
extern crate libc;

unsafe extern "C" fn VectorCompare(mut v1: *const vec_t, mut v2: *const vec_t) -> libc::c_int {
    if *v1.offset(0isize) != *v2.offset(0isize)
        || *v1.offset(1isize) != *v2.offset(1isize)
        || *v1.offset(2isize) != *v2.offset(2isize)
    {
        return 0i32;
    }
    return 1i32;
}
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
//
// cg_predict.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_BuildSolidList() {
    let mut i: libc::c_int = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut snap: *mut snapshot_t = 0 as *mut snapshot_t;
    let mut ent: *mut entityState_t = 0 as *mut entityState_t;
    cg_numSolidEntities = 0i32;
    cg_numTriggerEntities = 0i32;
    if !cg.nextSnap.is_null()
        && 0 == cg.nextFrameTeleport as u64
        && 0 == cg.thisFrameTeleport as u64
    {
        snap = cg.nextSnap
    } else {
        snap = cg.snap
    }
    i = 0i32;
    while i < (*snap).numEntities {
        cent = &mut *cg_entities
            .as_mut_ptr()
            .offset((*(*snap).entities.as_mut_ptr().offset(i as isize)).number as isize)
            as *mut centity_t;
        ent = &mut (*cent).currentState;
        if (*ent).eType == ET_ITEM as libc::c_int
            || (*ent).eType == ET_PUSH_TRIGGER as libc::c_int
            || (*ent).eType == ET_TELEPORT_TRIGGER as libc::c_int
        {
            cg_triggerEntities[cg_numTriggerEntities as usize] = cent;
            cg_numTriggerEntities += 1
        } else if 0 != (*cent).nextState.solid {
            cg_solidEntities[cg_numSolidEntities as usize] = cent;
            cg_numSolidEntities += 1
        }
        i += 1
    }
}
static mut cg_numSolidEntities: libc::c_int = 0;
static mut cg_solidEntities: [*mut centity_t; 256] = [0 as *const centity_t as *mut centity_t; 256];
static mut cg_numTriggerEntities: libc::c_int = 0;
static mut cg_triggerEntities: [*mut centity_t; 256] =
    [0 as *const centity_t as *mut centity_t; 256];
#[no_mangle]
pub unsafe extern "C" fn CG_PointContents(
    mut point: *const vec_t,
    mut passEntityNum: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ent: *mut entityState_t = 0 as *mut entityState_t;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut cmodel: clipHandle_t = 0;
    let mut contents: libc::c_int = 0;
    contents = trap_CM_PointContents(point, 0i32);
    i = 0i32;
    while i < cg_numSolidEntities {
        cent = cg_solidEntities[i as usize];
        ent = &mut (*cent).currentState;
        if !((*ent).number == passEntityNum) {
            if !((*ent).solid != 0xffffffi32) {
                // special value for bmodel
                cmodel = trap_CM_InlineModel((*ent).modelindex);
                if !(0 == cmodel) {
                    contents |= trap_CM_TransformedPointContents(
                        point,
                        cmodel,
                        (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
                        (*cent).lerpAngles.as_mut_ptr() as *const vec_t,
                    )
                }
            }
        }
        i += 1
    }
    return contents;
}
#[no_mangle]
pub unsafe extern "C" fn CG_Trace(
    mut result: *mut trace_t,
    mut start: *const vec_t,
    mut mins: *const vec_t,
    mut maxs: *const vec_t,
    mut end: *const vec_t,
    mut skipNumber: libc::c_int,
    mut mask: libc::c_int,
) {
    let mut t: trace_t = trace_t {
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
    trap_CM_BoxTrace(&mut t, start, end, mins, maxs, 0i32, mask);
    t.entityNum = if t.fraction as libc::c_double != 1.0f64 {
        (1i32 << 10i32) - 2i32
    } else {
        (1i32 << 10i32) - 1i32
    };
    CG_ClipMoveToEntities(start, mins, maxs, end, skipNumber, mask, &mut t);
    *result = t;
}
/*
====================
CG_ClipMoveToEntities

====================
*/
unsafe extern "C" fn CG_ClipMoveToEntities(
    mut start: *const vec_t,
    mut mins: *const vec_t,
    mut maxs: *const vec_t,
    mut end: *const vec_t,
    mut skipNumber: libc::c_int,
    mut mask: libc::c_int,
    mut tr: *mut trace_t,
) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut zd: libc::c_int = 0;
    let mut zu: libc::c_int = 0;
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
    let mut ent: *mut entityState_t = 0 as *mut entityState_t;
    let mut cmodel: clipHandle_t = 0;
    let mut bmins: vec3_t = [0.; 3];
    let mut bmaxs: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    i = 0i32;
    while i < cg_numSolidEntities {
        cent = cg_solidEntities[i as usize];
        ent = &mut (*cent).currentState;
        if !((*ent).number == skipNumber) {
            if (*ent).solid == 0xffffffi32 {
                cmodel = trap_CM_InlineModel((*ent).modelindex);
                angles[0usize] = (*cent).lerpAngles[0usize];
                angles[1usize] = (*cent).lerpAngles[1usize];
                angles[2usize] = (*cent).lerpAngles[2usize];
                BG_EvaluateTrajectory(
                    &mut (*cent).currentState.pos,
                    cg.physicsTime,
                    origin.as_mut_ptr(),
                );
            } else {
                x = (*ent).solid & 255i32;
                zd = (*ent).solid >> 8i32 & 255i32;
                zu = ((*ent).solid >> 16i32 & 255i32) - 32i32;
                bmins[1usize] = -x as vec_t;
                bmins[0usize] = bmins[1usize];
                bmaxs[1usize] = x as vec_t;
                bmaxs[0usize] = bmaxs[1usize];
                bmins[2usize] = -zd as vec_t;
                bmaxs[2usize] = zu as vec_t;
                cmodel = trap_CM_TempBoxModel(
                    bmins.as_mut_ptr() as *const vec_t,
                    bmaxs.as_mut_ptr() as *const vec_t,
                );
                angles[0usize] = vec3_origin[0usize];
                angles[1usize] = vec3_origin[1usize];
                angles[2usize] = vec3_origin[2usize];
                origin[0usize] = (*cent).lerpOrigin[0usize];
                origin[1usize] = (*cent).lerpOrigin[1usize];
                origin[2usize] = (*cent).lerpOrigin[2usize]
            }
            trap_CM_TransformedBoxTrace(
                &mut trace,
                start,
                end,
                mins,
                maxs,
                cmodel,
                mask,
                origin.as_mut_ptr() as *const vec_t,
                angles.as_mut_ptr() as *const vec_t,
            );
            if 0 != trace.allsolid as libc::c_uint || trace.fraction < (*tr).fraction {
                trace.entityNum = (*ent).number;
                *tr = trace
            } else if 0 != trace.startsolid as u64 {
                (*tr).startsolid = qtrue
            }
            if 0 != (*tr).allsolid as u64 {
                return;
            }
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn CG_PredictPlayerState() {
    let mut cmdNum: libc::c_int = 0;
    let mut current: libc::c_int = 0;
    let mut oldPlayerState: playerState_t = playerState_s {
        commandTime: 0,
        pm_type: 0,
        bobCycle: 0,
        pm_flags: 0,
        pm_time: 0,
        origin: [0.; 3],
        velocity: [0.; 3],
        weaponTime: 0,
        gravity: 0,
        speed: 0,
        delta_angles: [0; 3],
        groundEntityNum: 0,
        legsTimer: 0,
        legsAnim: 0,
        torsoTimer: 0,
        torsoAnim: 0,
        movementDir: 0,
        grapplePoint: [0.; 3],
        eFlags: 0,
        eventSequence: 0,
        events: [0; 2],
        eventParms: [0; 2],
        externalEvent: 0,
        externalEventParm: 0,
        externalEventTime: 0,
        clientNum: 0,
        weapon: 0,
        weaponstate: 0,
        viewangles: [0.; 3],
        viewheight: 0,
        damageEvent: 0,
        damageYaw: 0,
        damagePitch: 0,
        damageCount: 0,
        stats: [0; 16],
        persistant: [0; 16],
        powerups: [0; 16],
        ammo: [0; 16],
        generic1: 0,
        loopSound: 0,
        jumppad_ent: 0,
        ping: 0,
        pmove_framecount: 0,
        jumppad_frame: 0,
        entityEventSequence: 0,
    };
    let mut moved: qboolean = qfalse;
    let mut oldestCmd: usercmd_t = usercmd_s {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    };
    let mut latestCmd: usercmd_t = usercmd_s {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    };
    cg.hyperspace = qfalse;
    if 0 == cg.validPPS as u64 {
        cg.validPPS = qtrue;
        cg.predictedPlayerState = (*cg.snap).ps
    }
    if 0 != cg.demoPlayback as libc::c_uint || 0 != (*cg.snap).ps.pm_flags & 4096i32 {
        CG_InterpolatePlayerState(qfalse);
        return;
    }
    if 0 != cg_nopredict.integer || 0 != cg_synchronousClients.integer {
        CG_InterpolatePlayerState(qtrue);
        return;
    }
    cg_pmove.ps = &mut cg.predictedPlayerState;
    cg_pmove.trace = Some(CG_Trace);
    cg_pmove.pointcontents = Some(CG_PointContents);
    if (*cg_pmove.ps).pm_type == PM_DEAD as libc::c_int {
        cg_pmove.tracemask = (1i32 | 0x10000i32 | 0x2000000i32) & !0x2000000i32
    } else {
        cg_pmove.tracemask = 1i32 | 0x10000i32 | 0x2000000i32
    }
    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_SPECTATOR as libc::c_int
    {
        cg_pmove.tracemask &= !0x2000000i32
    }
    cg_pmove.noFootsteps = (cgs.dmflags & 32i32 > 0i32) as libc::c_int as qboolean;
    oldPlayerState = cg.predictedPlayerState;
    current = trap_GetCurrentCmdNumber();
    cmdNum = current - 64i32 + 1i32;
    trap_GetUserCmd(cmdNum, &mut oldestCmd);
    if oldestCmd.serverTime > (*cg.snap).ps.commandTime && oldestCmd.serverTime < cg.time {
        if 0 != cg_showmiss.integer {
            CG_Printf(
                b"exceeded PACKET_BACKUP on commands\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    trap_GetUserCmd(current, &mut latestCmd);
    if !cg.nextSnap.is_null()
        && 0 == cg.nextFrameTeleport as u64
        && 0 == cg.thisFrameTeleport as u64
    {
        cg.predictedPlayerState = (*cg.nextSnap).ps;
        cg.physicsTime = (*cg.nextSnap).serverTime
    } else {
        cg.predictedPlayerState = (*cg.snap).ps;
        cg.physicsTime = (*cg.snap).serverTime
    }
    if pmove_msec.integer < 8i32 {
        trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"8\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut pmove_msec);
    } else if pmove_msec.integer > 33i32 {
        trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"33\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut pmove_msec);
    }
    cg_pmove.pmove_fixed = pmove_fixed.integer;
    cg_pmove.pmove_msec = pmove_msec.integer;
    moved = qfalse;
    cmdNum = current - 64i32 + 1i32;
    while cmdNum <= current {
        trap_GetUserCmd(cmdNum, &mut cg_pmove.cmd);
        if 0 != cg_pmove.pmove_fixed {
            PM_UpdateViewAngles(cg_pmove.ps, &mut cg_pmove.cmd);
        }
        // don't do anything if the time is before the snapshot player time
        if !(cg_pmove.cmd.serverTime <= cg.predictedPlayerState.commandTime) {
            // don't do anything if the command was from a previous map_restart
            if !(cg_pmove.cmd.serverTime > latestCmd.serverTime) {
                if cg.predictedPlayerState.commandTime == oldPlayerState.commandTime {
                    let mut delta: vec3_t = [0.; 3];
                    let mut len: libc::c_float = 0.;
                    if 0 != cg.thisFrameTeleport as u64 {
                        cg.predictedError[2usize] = 0i32 as vec_t;
                        cg.predictedError[1usize] = cg.predictedError[2usize];
                        cg.predictedError[0usize] = cg.predictedError[1usize];
                        if 0 != cg_showmiss.integer {
                            CG_Printf(
                                b"PredictionTeleport\n\x00" as *const u8 as *const libc::c_char,
                            );
                        }
                        cg.thisFrameTeleport = qfalse
                    } else {
                        let mut adjusted: vec3_t = [0.; 3];
                        let mut new_angles: vec3_t = [0.; 3];
                        CG_AdjustPositionForMover(
                            cg.predictedPlayerState.origin.as_mut_ptr() as *const vec_t,
                            cg.predictedPlayerState.groundEntityNum,
                            cg.physicsTime,
                            cg.oldTime,
                            adjusted.as_mut_ptr(),
                            cg.predictedPlayerState.viewangles.as_mut_ptr(),
                            new_angles.as_mut_ptr(),
                        );
                        if 0 != cg_showmiss.integer {
                            if 0 == VectorCompare(
                                oldPlayerState.origin.as_mut_ptr() as *const vec_t,
                                adjusted.as_mut_ptr() as *const vec_t,
                            ) {
                                CG_Printf(
                                    b"prediction error\n\x00" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        delta[0usize] = oldPlayerState.origin[0usize] - adjusted[0usize];
                        delta[1usize] = oldPlayerState.origin[1usize] - adjusted[1usize];
                        delta[2usize] = oldPlayerState.origin[2usize] - adjusted[2usize];
                        len = VectorLength(delta.as_mut_ptr() as *const vec_t);
                        if len as libc::c_double > 0.1f64 {
                            if 0 != cg_showmiss.integer {
                                CG_Printf(
                                    b"Prediction miss: %f\n\x00" as *const u8
                                        as *const libc::c_char,
                                    len as libc::c_double,
                                );
                            }
                            if 0 != cg_errorDecay.integer {
                                let mut t: libc::c_int = 0;
                                let mut f: libc::c_float = 0.;
                                t = cg.time - cg.predictedErrorTime;
                                f = (cg_errorDecay.value - t as libc::c_float)
                                    / cg_errorDecay.value;
                                if f < 0i32 as libc::c_float {
                                    f = 0i32 as libc::c_float
                                }
                                if f > 0i32 as libc::c_float && 0 != cg_showmiss.integer {
                                    CG_Printf(
                                        b"Double prediction decay: %f\n\x00" as *const u8
                                            as *const libc::c_char,
                                        f as libc::c_double,
                                    );
                                }
                                cg.predictedError[0usize] = cg.predictedError[0usize] * f;
                                cg.predictedError[1usize] = cg.predictedError[1usize] * f;
                                cg.predictedError[2usize] = cg.predictedError[2usize] * f
                            } else {
                                cg.predictedError[2usize] = 0i32 as vec_t;
                                cg.predictedError[1usize] = cg.predictedError[2usize];
                                cg.predictedError[0usize] = cg.predictedError[1usize]
                            }
                            cg.predictedError[0usize] = delta[0usize] + cg.predictedError[0usize];
                            cg.predictedError[1usize] = delta[1usize] + cg.predictedError[1usize];
                            cg.predictedError[2usize] = delta[2usize] + cg.predictedError[2usize];
                            cg.predictedErrorTime = cg.oldTime
                        }
                    }
                }
                cg_pmove.gauntletHit = qfalse;
                if 0 != cg_pmove.pmove_fixed {
                    cg_pmove.cmd.serverTime = (cg_pmove.cmd.serverTime + pmove_msec.integer - 1i32)
                        / pmove_msec.integer
                        * pmove_msec.integer
                }
                Pmove(&mut cg_pmove);
                moved = qtrue;
                CG_TouchTriggerPrediction();
            }
        }
        cmdNum += 1
    }
    if cg_showmiss.integer > 1i32 {
        CG_Printf(
            b"[%i : %i] \x00" as *const u8 as *const libc::c_char,
            cg_pmove.cmd.serverTime,
            cg.time,
        );
    }
    if 0 == moved as u64 {
        if 0 != cg_showmiss.integer {
            CG_Printf(b"not moved\n\x00" as *const u8 as *const libc::c_char);
        }
        return;
    }
    CG_AdjustPositionForMover(
        cg.predictedPlayerState.origin.as_mut_ptr() as *const vec_t,
        cg.predictedPlayerState.groundEntityNum,
        cg.physicsTime,
        cg.time,
        cg.predictedPlayerState.origin.as_mut_ptr(),
        cg.predictedPlayerState.viewangles.as_mut_ptr(),
        cg.predictedPlayerState.viewangles.as_mut_ptr(),
    );
    if 0 != cg_showmiss.integer {
        if cg.predictedPlayerState.eventSequence > oldPlayerState.eventSequence + 2i32 {
            CG_Printf(b"WARNING: dropped event\n\x00" as *const u8 as *const libc::c_char);
        }
    }
    CG_TransitionPlayerState(&mut cg.predictedPlayerState, &mut oldPlayerState);
    if 0 != cg_showmiss.integer {
        if cg.eventSequence > cg.predictedPlayerState.eventSequence {
            CG_Printf(b"WARNING: double event\n\x00" as *const u8 as *const libc::c_char);
            cg.eventSequence = cg.predictedPlayerState.eventSequence
        }
    };
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
// cg_predict.c -- this file generates cg.predictedPlayerState by either
// interpolating between snapshots from the server or locally predicting
// ahead the client's movement.
// It also handles local physics interaction, like fragments bouncing off walls
static mut cg_pmove: pmove_t = pmove_t {
    ps: 0 as *const playerState_t as *mut playerState_t,
    cmd: usercmd_s {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    },
    tracemask: 0,
    debugLevel: 0,
    noFootsteps: qfalse,
    gauntletHit: qfalse,
    framecount: 0,
    numtouch: 0,
    touchents: [0; 32],
    mins: [0.; 3],
    maxs: [0.; 3],
    watertype: 0,
    waterlevel: 0,
    xyspeed: 0.,
    pmove_fixed: 0,
    pmove_msec: 0,
    trace: None,
    pointcontents: None,
};
/*
=========================
CG_TouchTriggerPrediction

Predict push triggers and items
=========================
*/
unsafe extern "C" fn CG_TouchTriggerPrediction() {
    let mut i: libc::c_int = 0;
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
    let mut ent: *mut entityState_t = 0 as *mut entityState_t;
    let mut cmodel: clipHandle_t = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut spectator: qboolean = qfalse;
    if cg.predictedPlayerState.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        return;
    }
    spectator =
        (cg.predictedPlayerState.pm_type == PM_SPECTATOR as libc::c_int) as libc::c_int as qboolean;
    if cg.predictedPlayerState.pm_type != PM_NORMAL as libc::c_int && 0 == spectator as u64 {
        return;
    }
    i = 0i32;
    while i < cg_numTriggerEntities {
        cent = cg_triggerEntities[i as usize];
        ent = &mut (*cent).currentState;
        if (*ent).eType == ET_ITEM as libc::c_int && 0 == spectator as u64 {
            CG_TouchItem(cent);
        } else if !((*ent).solid != 0xffffffi32) {
            cmodel = trap_CM_InlineModel((*ent).modelindex);
            if !(0 == cmodel) {
                trap_CM_BoxTrace(
                    &mut trace,
                    cg.predictedPlayerState.origin.as_mut_ptr() as *const vec_t,
                    cg.predictedPlayerState.origin.as_mut_ptr() as *const vec_t,
                    cg_pmove.mins.as_mut_ptr() as *const vec_t,
                    cg_pmove.maxs.as_mut_ptr() as *const vec_t,
                    cmodel,
                    -1i32,
                );
                if !(0 == trace.startsolid as u64) {
                    if (*ent).eType == ET_TELEPORT_TRIGGER as libc::c_int {
                        cg.hyperspace = qtrue
                    } else if (*ent).eType == ET_PUSH_TRIGGER as libc::c_int {
                        BG_TouchJumpPad(&mut cg.predictedPlayerState, ent);
                    }
                }
            }
        }
        i += 1
    }
    if cg.predictedPlayerState.jumppad_frame != cg.predictedPlayerState.pmove_framecount {
        cg.predictedPlayerState.jumppad_frame = 0i32;
        cg.predictedPlayerState.jumppad_ent = 0i32
    };
}
/*
===================
CG_TouchItem
===================
*/
unsafe extern "C" fn CG_TouchItem(mut cent: *mut centity_t) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    if 0 == cg_predictItems.integer {
        return;
    }
    if 0 == BG_PlayerTouchesItem(
        &mut cg.predictedPlayerState,
        &mut (*cent).currentState,
        cg.time,
    ) as u64
    {
        return;
    }
    if (*cent).miscTime == cg.time {
        return;
    }
    if 0 == BG_CanItemBeGrabbed(
        cgs.gametype as libc::c_int,
        &mut (*cent).currentState,
        &mut cg.predictedPlayerState,
    ) as u64
    {
        return;
    }
    item = &mut *bg_itemlist
        .as_mut_ptr()
        .offset((*cent).currentState.modelindex as isize) as *mut gitem_t;
    if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint {
        if cg.predictedPlayerState.persistant[PERS_TEAM as libc::c_int as usize]
            == TEAM_RED as libc::c_int
            && (*item).giType as libc::c_uint == IT_TEAM as libc::c_int as libc::c_uint
            && (*item).giTag == PW_REDFLAG as libc::c_int
        {
            return;
        }
        if cg.predictedPlayerState.persistant[PERS_TEAM as libc::c_int as usize]
            == TEAM_BLUE as libc::c_int
            && (*item).giType as libc::c_uint == IT_TEAM as libc::c_int as libc::c_uint
            && (*item).giTag == PW_BLUEFLAG as libc::c_int
        {
            return;
        }
    }
    BG_AddPredictableEventToPlayerstate(
        EV_ITEM_PICKUP as libc::c_int,
        (*cent).currentState.modelindex,
        &mut cg.predictedPlayerState,
    );
    (*cent).currentState.eFlags |= 0x80i32;
    (*cent).miscTime = cg.time;
    if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint {
        cg.predictedPlayerState.stats[STAT_WEAPONS as libc::c_int as usize] |=
            1i32 << (*item).giTag;
        if 0 == cg.predictedPlayerState.ammo[(*item).giTag as usize] {
            cg.predictedPlayerState.ammo[(*item).giTag as usize] = 1i32
        }
    };
}
/*
========================
CG_InterpolatePlayerState

Generates cg.predictedPlayerState by interpolating between
cg.snap->player_state and cg.nextFrame->player_state
========================
*/
unsafe extern "C" fn CG_InterpolatePlayerState(mut grabAngles: qboolean) {
    let mut f: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut out: *mut playerState_t = 0 as *mut playerState_t;
    let mut prev: *mut snapshot_t = 0 as *mut snapshot_t;
    let mut next: *mut snapshot_t = 0 as *mut snapshot_t;
    out = &mut cg.predictedPlayerState;
    prev = cg.snap;
    next = cg.nextSnap;
    *out = (*cg.snap).ps;
    if 0 != grabAngles as u64 {
        let mut cmd: usercmd_t = usercmd_s {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
        let mut cmdNum: libc::c_int = 0;
        cmdNum = trap_GetCurrentCmdNumber();
        trap_GetUserCmd(cmdNum, &mut cmd);
        PM_UpdateViewAngles(out, &mut cmd);
    }
    if 0 != cg.nextFrameTeleport as u64 {
        return;
    }
    if next.is_null() || (*next).serverTime <= (*prev).serverTime {
        return;
    }
    f = (cg.time - (*prev).serverTime) as libc::c_float
        / ((*next).serverTime - (*prev).serverTime) as libc::c_float;
    i = (*next).ps.bobCycle;
    if i < (*prev).ps.bobCycle {
        i += 256i32
    }
    (*out).bobCycle = ((*prev).ps.bobCycle as libc::c_float
        + f * (i - (*prev).ps.bobCycle) as libc::c_float) as libc::c_int;
    i = 0i32;
    while i < 3i32 {
        (*out).origin[i as usize] = (*prev).ps.origin[i as usize]
            + f * ((*next).ps.origin[i as usize] - (*prev).ps.origin[i as usize]);
        if 0 == grabAngles as u64 {
            (*out).viewangles[i as usize] = LerpAngle(
                (*prev).ps.viewangles[i as usize],
                (*next).ps.viewangles[i as usize],
                f,
            )
        }
        (*out).velocity[i as usize] = (*prev).ps.velocity[i as usize]
            + f * ((*next).ps.velocity[i as usize] - (*prev).ps.velocity[i as usize]);
        i += 1
    }
}
