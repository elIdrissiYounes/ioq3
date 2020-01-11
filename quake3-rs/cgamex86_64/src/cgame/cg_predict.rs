use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorCompare(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> libc::c_int {
        if *v1.offset(0 as libc::c_int as isize) != *v2.offset(0 as libc::c_int as isize)
            || *v1.offset(1 as libc::c_int as isize) != *v2.offset(1 as libc::c_int as isize)
            || *v1.offset(2 as libc::c_int as isize) != *v2.offset(2 as libc::c_int as isize)
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
                + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
                + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize))
                as libc::c_double,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    use crate::stdlib::sqrt;

    // __Q_SHARED_H
}

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::pmove_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::ET_BEAM;
pub use crate::bg_public_h::ET_EVENTS;
pub use crate::bg_public_h::ET_GENERAL;
pub use crate::bg_public_h::ET_GRAPPLE;
pub use crate::bg_public_h::ET_INVISIBLE;
pub use crate::bg_public_h::ET_ITEM;
pub use crate::bg_public_h::ET_MISSILE;
pub use crate::bg_public_h::ET_MOVER;
pub use crate::bg_public_h::ET_PLAYER;
pub use crate::bg_public_h::ET_PORTAL;
pub use crate::bg_public_h::ET_PUSH_TRIGGER;
pub use crate::bg_public_h::ET_SPEAKER;
pub use crate::bg_public_h::ET_TEAM;
pub use crate::bg_public_h::ET_TELEPORT_TRIGGER;
pub use crate::bg_public_h::EV_BULLET;
pub use crate::bg_public_h::EV_BULLET_HIT_FLESH;
pub use crate::bg_public_h::EV_BULLET_HIT_WALL;
pub use crate::bg_public_h::EV_CHANGE_WEAPON;
pub use crate::bg_public_h::EV_DEATH1;
pub use crate::bg_public_h::EV_DEATH2;
pub use crate::bg_public_h::EV_DEATH3;
pub use crate::bg_public_h::EV_DEBUG_LINE;
pub use crate::bg_public_h::EV_FALL_FAR;
pub use crate::bg_public_h::EV_FALL_MEDIUM;
pub use crate::bg_public_h::EV_FALL_SHORT;
pub use crate::bg_public_h::EV_FIRE_WEAPON;
pub use crate::bg_public_h::EV_FOOTSPLASH;
pub use crate::bg_public_h::EV_FOOTSTEP;
pub use crate::bg_public_h::EV_FOOTSTEP_METAL;
pub use crate::bg_public_h::EV_FOOTWADE;
pub use crate::bg_public_h::EV_GENERAL_SOUND;
pub use crate::bg_public_h::EV_GIB_PLAYER;
pub use crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP;
pub use crate::bg_public_h::EV_GLOBAL_SOUND;
pub use crate::bg_public_h::EV_GLOBAL_TEAM_SOUND;
pub use crate::bg_public_h::EV_GRENADE_BOUNCE;
pub use crate::bg_public_h::EV_INVUL_IMPACT;
pub use crate::bg_public_h::EV_ITEM_PICKUP;
pub use crate::bg_public_h::EV_ITEM_POP;
pub use crate::bg_public_h::EV_ITEM_RESPAWN;
pub use crate::bg_public_h::EV_JUICED;
pub use crate::bg_public_h::EV_JUMP;
pub use crate::bg_public_h::EV_JUMP_PAD;
pub use crate::bg_public_h::EV_KAMIKAZE;
pub use crate::bg_public_h::EV_LIGHTNINGBOLT;
pub use crate::bg_public_h::EV_MISSILE_HIT;
pub use crate::bg_public_h::EV_MISSILE_MISS;
pub use crate::bg_public_h::EV_MISSILE_MISS_METAL;
pub use crate::bg_public_h::EV_NOAMMO;
pub use crate::bg_public_h::EV_NONE;
pub use crate::bg_public_h::EV_OBELISKEXPLODE;
pub use crate::bg_public_h::EV_OBELISKPAIN;
pub use crate::bg_public_h::EV_OBITUARY;
pub use crate::bg_public_h::EV_PAIN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_IN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_OUT;
pub use crate::bg_public_h::EV_POWERUP_BATTLESUIT;
pub use crate::bg_public_h::EV_POWERUP_QUAD;
pub use crate::bg_public_h::EV_POWERUP_REGEN;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_STICK;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_TRIGGER;
pub use crate::bg_public_h::EV_RAILTRAIL;
pub use crate::bg_public_h::EV_SCOREPLUM;
pub use crate::bg_public_h::EV_SHOTGUN;
pub use crate::bg_public_h::EV_STEP_12;
pub use crate::bg_public_h::EV_STEP_16;
pub use crate::bg_public_h::EV_STEP_4;
pub use crate::bg_public_h::EV_STEP_8;
pub use crate::bg_public_h::EV_STOPLOOPINGSOUND;
pub use crate::bg_public_h::EV_SWIM;
pub use crate::bg_public_h::EV_TAUNT;
pub use crate::bg_public_h::EV_TAUNT_FOLLOWME;
pub use crate::bg_public_h::EV_TAUNT_GETFLAG;
pub use crate::bg_public_h::EV_TAUNT_GUARDBASE;
pub use crate::bg_public_h::EV_TAUNT_NO;
pub use crate::bg_public_h::EV_TAUNT_PATROL;
pub use crate::bg_public_h::EV_TAUNT_YES;
pub use crate::bg_public_h::EV_USE_ITEM0;
pub use crate::bg_public_h::EV_USE_ITEM1;
pub use crate::bg_public_h::EV_USE_ITEM10;
pub use crate::bg_public_h::EV_USE_ITEM11;
pub use crate::bg_public_h::EV_USE_ITEM12;
pub use crate::bg_public_h::EV_USE_ITEM13;
pub use crate::bg_public_h::EV_USE_ITEM14;
pub use crate::bg_public_h::EV_USE_ITEM15;
pub use crate::bg_public_h::EV_USE_ITEM2;
pub use crate::bg_public_h::EV_USE_ITEM3;
pub use crate::bg_public_h::EV_USE_ITEM4;
pub use crate::bg_public_h::EV_USE_ITEM5;
pub use crate::bg_public_h::EV_USE_ITEM6;
pub use crate::bg_public_h::EV_USE_ITEM7;
pub use crate::bg_public_h::EV_USE_ITEM8;
pub use crate::bg_public_h::EV_USE_ITEM9;
pub use crate::bg_public_h::EV_WATER_CLEAR;
pub use crate::bg_public_h::EV_WATER_LEAVE;
pub use crate::bg_public_h::EV_WATER_TOUCH;
pub use crate::bg_public_h::EV_WATER_UNDER;
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
pub use crate::src::cgame::cg_ents::CG_AdjustPositionForMover;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_errorDecay;
pub use crate::src::cgame::cg_main::cg_nopredict;
pub use crate::src::cgame::cg_main::cg_predictItems;
pub use crate::src::cgame::cg_main::cg_showmiss;
pub use crate::src::cgame::cg_main::cg_synchronousClients;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::pmove_fixed;
pub use crate::src::cgame::cg_main::pmove_msec;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_playerstate::CG_TransitionPlayerState;
pub use crate::src::cgame::cg_predict::q_shared_h::VectorCompare;
pub use crate::src::cgame::cg_predict::q_shared_h::VectorLength;
pub use crate::src::cgame::cg_syscalls::trap_CM_BoxTrace;
pub use crate::src::cgame::cg_syscalls::trap_CM_InlineModel;
pub use crate::src::cgame::cg_syscalls::trap_CM_PointContents;
pub use crate::src::cgame::cg_syscalls::trap_CM_TempBoxModel;
pub use crate::src::cgame::cg_syscalls::trap_CM_TransformedBoxTrace;
pub use crate::src::cgame::cg_syscalls::trap_CM_TransformedPointContents;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_Set;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_Update;
pub use crate::src::cgame::cg_syscalls::trap_GetCurrentCmdNumber;
pub use crate::src::cgame::cg_syscalls::trap_GetUserCmd;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::BG_AddPredictableEventToPlayerstate;
pub use crate::src::game::bg_misc::BG_CanItemBeGrabbed;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::bg_misc::BG_PlayerTouchesItem;
pub use crate::src::game::bg_misc::BG_TouchJumpPad;
pub use crate::src::game::bg_pmove::PM_UpdateViewAngles;
pub use crate::src::game::bg_pmove::Pmove;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::LerpAngle;
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
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
use crate::stdlib::sqrt;
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
// cg_predict.c -- this file generates cg.predictedPlayerState by either
// interpolating between snapshots from the server or locally predicting
// ahead the client's movement.
// It also handles local physics interaction, like fragments bouncing off walls

static mut cg_pmove: crate::bg_public_h::pmove_t = crate::bg_public_h::pmove_t {
    ps: 0 as *const crate::src::qcommon::q_shared::playerState_t
        as *mut crate::src::qcommon::q_shared::playerState_t,
    cmd: crate::src::qcommon::q_shared::usercmd_t {
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
    noFootsteps: crate::src::qcommon::q_shared::qfalse,
    gauntletHit: crate::src::qcommon::q_shared::qfalse,
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

static mut cg_numSolidEntities: libc::c_int = 0;

static mut cg_solidEntities: [*mut crate::cg_local_h::centity_t; 256] =
    [0 as *const crate::cg_local_h::centity_t as *mut crate::cg_local_h::centity_t; 256];

static mut cg_numTriggerEntities: libc::c_int = 0;

static mut cg_triggerEntities: [*mut crate::cg_local_h::centity_t; 256] =
    [0 as *const crate::cg_local_h::centity_t as *mut crate::cg_local_h::centity_t; 256];
/*
====================
CG_BuildSolidList

When a new cg.snap has been set, this function builds a sublist
of the entities that are actually solid, to make for more
efficient collision detection
====================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_BuildSolidList() {
    let mut i: libc::c_int = 0;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut snap: *mut crate::cg_public_h::snapshot_t = 0 as *mut crate::cg_public_h::snapshot_t;
    let mut ent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    cg_numSolidEntities = 0 as libc::c_int;
    cg_numTriggerEntities = 0 as libc::c_int;
    if !crate::src::cgame::cg_main::cg.nextSnap.is_null()
        && crate::src::cgame::cg_main::cg.nextFrameTeleport as u64 == 0
        && crate::src::cgame::cg_main::cg.thisFrameTeleport as u64 == 0
    {
        snap = crate::src::cgame::cg_main::cg.nextSnap
    } else {
        snap = crate::src::cgame::cg_main::cg.snap
    }
    i = 0 as libc::c_int;
    while i < (*snap).numEntities {
        cent = &mut *crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*(*snap).entities.as_mut_ptr().offset(i as isize)).number as isize)
            as *mut crate::cg_local_h::centity_t;
        ent = &mut (*cent).currentState;
        if (*ent).eType == crate::bg_public_h::ET_ITEM as libc::c_int
            || (*ent).eType == crate::bg_public_h::ET_PUSH_TRIGGER as libc::c_int
            || (*ent).eType == crate::bg_public_h::ET_TELEPORT_TRIGGER as libc::c_int
        {
            cg_triggerEntities[cg_numTriggerEntities as usize] = cent;
            cg_numTriggerEntities += 1
        } else if (*cent).nextState.solid != 0 {
            cg_solidEntities[cg_numSolidEntities as usize] = cent;
            cg_numSolidEntities += 1
        }
        i += 1
    }
}
/*
====================
CG_ClipMoveToEntities

====================
*/

unsafe extern "C" fn CG_ClipMoveToEntities(
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut skipNumber: libc::c_int,
    mut mask: libc::c_int,
    mut tr: *mut crate::src::qcommon::q_shared::trace_t,
) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut zd: libc::c_int = 0;
    let mut zu: libc::c_int = 0;
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
    let mut ent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut cmodel: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut bmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    i = 0 as libc::c_int;
    while i < cg_numSolidEntities {
        cent = cg_solidEntities[i as usize];
        ent = &mut (*cent).currentState;
        if !((*ent).number == skipNumber) {
            if (*ent).solid == 0xffffff as libc::c_int {
                // special value for bmodel
                cmodel = crate::src::cgame::cg_syscalls::trap_CM_InlineModel((*ent).modelindex);
                angles[0 as libc::c_int as usize] = (*cent).lerpAngles[0 as libc::c_int as usize];
                angles[1 as libc::c_int as usize] = (*cent).lerpAngles[1 as libc::c_int as usize];
                angles[2 as libc::c_int as usize] = (*cent).lerpAngles[2 as libc::c_int as usize];
                crate::src::game::bg_misc::BG_EvaluateTrajectory(
                    &mut (*cent).currentState.pos as *mut _
                        as *const crate::src::qcommon::q_shared::trajectory_t,
                    crate::src::cgame::cg_main::cg.physicsTime,
                    origin.as_mut_ptr(),
                );
            } else {
                // encoded bbox
                x = (*ent).solid & 255 as libc::c_int;
                zd = (*ent).solid >> 8 as libc::c_int & 255 as libc::c_int;
                zu = ((*ent).solid >> 16 as libc::c_int & 255 as libc::c_int) - 32 as libc::c_int;
                bmins[1 as libc::c_int as usize] = -x as crate::src::qcommon::q_shared::vec_t;
                bmins[0 as libc::c_int as usize] = bmins[1 as libc::c_int as usize];
                bmaxs[1 as libc::c_int as usize] = x as crate::src::qcommon::q_shared::vec_t;
                bmaxs[0 as libc::c_int as usize] = bmaxs[1 as libc::c_int as usize];
                bmins[2 as libc::c_int as usize] = -zd as crate::src::qcommon::q_shared::vec_t;
                bmaxs[2 as libc::c_int as usize] = zu as crate::src::qcommon::q_shared::vec_t;
                cmodel = crate::src::cgame::cg_syscalls::trap_CM_TempBoxModel(
                    bmins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    bmaxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                );
                angles[0 as libc::c_int as usize] =
                    crate::src::qcommon::q_math::vec3_origin[0 as libc::c_int as usize];
                angles[1 as libc::c_int as usize] =
                    crate::src::qcommon::q_math::vec3_origin[1 as libc::c_int as usize];
                angles[2 as libc::c_int as usize] =
                    crate::src::qcommon::q_math::vec3_origin[2 as libc::c_int as usize];
                origin[0 as libc::c_int as usize] = (*cent).lerpOrigin[0 as libc::c_int as usize];
                origin[1 as libc::c_int as usize] = (*cent).lerpOrigin[1 as libc::c_int as usize];
                origin[2 as libc::c_int as usize] = (*cent).lerpOrigin[2 as libc::c_int as usize]
            }
            crate::src::cgame::cg_syscalls::trap_CM_TransformedBoxTrace(
                &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
                start,
                end,
                mins,
                maxs,
                cmodel,
                mask,
                origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            );
            if trace.allsolid as libc::c_uint != 0 || trace.fraction < (*tr).fraction {
                trace.entityNum = (*ent).number;
                *tr = trace
            } else if trace.startsolid as u64 != 0 {
                (*tr).startsolid = crate::src::qcommon::q_shared::qtrue
            }
            if (*tr).allsolid as u64 != 0 {
                return;
            }
        }
        i += 1
    }
}
/*
================
CG_Trace
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Trace(
    mut result: *mut crate::src::qcommon::q_shared::trace_t,
    mut start: *const crate::src::qcommon::q_shared::vec_t,
    mut mins: *const crate::src::qcommon::q_shared::vec_t,
    mut maxs: *const crate::src::qcommon::q_shared::vec_t,
    mut end: *const crate::src::qcommon::q_shared::vec_t,
    mut skipNumber: libc::c_int,
    mut mask: libc::c_int,
) {
    let mut t: crate::src::qcommon::q_shared::trace_t = crate::src::qcommon::q_shared::trace_t {
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
    crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
        &mut t as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
        start,
        end,
        mins,
        maxs,
        0 as libc::c_int,
        mask,
    );
    t.entityNum = if t.fraction as libc::c_double != 1.0f64 {
        ((1 as libc::c_int) << 10 as libc::c_int) - 2 as libc::c_int
    } else {
        ((1 as libc::c_int) << 10 as libc::c_int) - 1 as libc::c_int
    };
    // check all other solid models
    CG_ClipMoveToEntities(start, mins, maxs, end, skipNumber, mask, &mut t);
    *result = t;
}
/*
================
CG_PointContents
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PointContents(
    mut point: *const crate::src::qcommon::q_shared::vec_t,
    mut passEntityNum: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut cmodel: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut contents: libc::c_int = 0;
    contents = crate::src::cgame::cg_syscalls::trap_CM_PointContents(point, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < cg_numSolidEntities {
        cent = cg_solidEntities[i as usize];
        ent = &mut (*cent).currentState;
        if !((*ent).number == passEntityNum) {
            if !((*ent).solid != 0xffffff as libc::c_int) {
                cmodel = crate::src::cgame::cg_syscalls::trap_CM_InlineModel((*ent).modelindex);
                if !(cmodel == 0) {
                    contents |= crate::src::cgame::cg_syscalls::trap_CM_TransformedPointContents(
                        point,
                        cmodel,
                        (*cent).lerpOrigin.as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                        (*cent).lerpAngles.as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                    )
                }
            }
        }
        // special value for bmodel
        i += 1
    }
    return contents;
}
/*
========================
CG_InterpolatePlayerState

Generates cg.predictedPlayerState by interpolating between
cg.snap->player_state and cg.nextFrame->player_state
========================
*/

unsafe extern "C" fn CG_InterpolatePlayerState(
    mut grabAngles: crate::src::qcommon::q_shared::qboolean,
) {
    let mut f: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut out: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    let mut prev: *mut crate::cg_public_h::snapshot_t = 0 as *mut crate::cg_public_h::snapshot_t;
    let mut next: *mut crate::cg_public_h::snapshot_t = 0 as *mut crate::cg_public_h::snapshot_t;
    out = &mut crate::src::cgame::cg_main::cg.predictedPlayerState;
    prev = crate::src::cgame::cg_main::cg.snap;
    next = crate::src::cgame::cg_main::cg.nextSnap;
    *out = (*crate::src::cgame::cg_main::cg.snap).ps;
    // if we are still allowing local input, short circuit the view angles
    if grabAngles as u64 != 0 {
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
        let mut cmdNum: libc::c_int = 0;
        cmdNum = crate::src::cgame::cg_syscalls::trap_GetCurrentCmdNumber();
        crate::src::cgame::cg_syscalls::trap_GetUserCmd(
            cmdNum,
            &mut cmd as *mut _ as *mut crate::src::qcommon::q_shared::usercmd_s,
        );
        crate::src::game::bg_pmove::PM_UpdateViewAngles(
            out as *mut crate::src::qcommon::q_shared::playerState_s,
            &mut cmd as *mut _ as *const crate::src::qcommon::q_shared::usercmd_s,
        );
    }
    // if the next frame is a teleport, we can't lerp to it
    if crate::src::cgame::cg_main::cg.nextFrameTeleport as u64 != 0 {
        return;
    }
    if next.is_null() || (*next).serverTime <= (*prev).serverTime {
        return;
    }
    f = (crate::src::cgame::cg_main::cg.time - (*prev).serverTime) as libc::c_float
        / ((*next).serverTime - (*prev).serverTime) as libc::c_float;
    i = (*next).ps.bobCycle;
    if i < (*prev).ps.bobCycle {
        i += 256 as libc::c_int
        // handle wraparound
    }
    (*out).bobCycle = ((*prev).ps.bobCycle as libc::c_float
        + f * (i - (*prev).ps.bobCycle) as libc::c_float) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*out).origin[i as usize] = (*prev).ps.origin[i as usize]
            + f * ((*next).ps.origin[i as usize] - (*prev).ps.origin[i as usize]);
        if grabAngles as u64 == 0 {
            (*out).viewangles[i as usize] = crate::src::qcommon::q_math::LerpAngle(
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
/*
===================
CG_TouchItem
===================
*/

unsafe extern "C" fn CG_TouchItem(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    if crate::src::cgame::cg_main::cg_predictItems.integer == 0 {
        return;
    }
    if crate::src::game::bg_misc::BG_PlayerTouchesItem(
        &mut crate::src::cgame::cg_main::cg.predictedPlayerState as *mut _
            as *mut crate::src::qcommon::q_shared::playerState_s,
        &mut (*cent).currentState as *mut _ as *mut crate::src::qcommon::q_shared::entityState_s,
        crate::src::cgame::cg_main::cg.time,
    ) as u64
        == 0
    {
        return;
    }
    // never pick an item up twice in a prediction
    if (*cent).miscTime == crate::src::cgame::cg_main::cg.time {
        return;
    }
    if crate::src::game::bg_misc::BG_CanItemBeGrabbed(
        crate::src::cgame::cg_main::cgs.gametype as libc::c_int,
        &mut (*cent).currentState as *mut _ as *const crate::src::qcommon::q_shared::entityState_s,
        &mut crate::src::cgame::cg_main::cg.predictedPlayerState as *mut _
            as *const crate::src::qcommon::q_shared::playerState_s,
    ) as u64
        == 0
    {
        return;
        // can't hold it
    }
    item = &mut *crate::src::game::bg_misc::bg_itemlist
        .as_mut_ptr()
        .offset((*cent).currentState.modelindex as isize)
        as *mut crate::bg_public_h::gitem_t;
    // Special case for flags.
    // We don't predict touching our own flag
    if crate::src::cgame::cg_main::cgs.gametype as libc::c_uint
        == crate::bg_public_h::GT_CTF as libc::c_int as libc::c_uint
    {
        if crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .persistant[crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
            == crate::bg_public_h::TEAM_RED as libc::c_int
            && (*item).giType as libc::c_uint
                == crate::bg_public_h::IT_TEAM as libc::c_int as libc::c_uint
            && (*item).giTag == crate::bg_public_h::PW_REDFLAG as libc::c_int
        {
            return;
        }
        if crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .persistant[crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
            == crate::bg_public_h::TEAM_BLUE as libc::c_int
            && (*item).giType as libc::c_uint
                == crate::bg_public_h::IT_TEAM as libc::c_int as libc::c_uint
            && (*item).giTag == crate::bg_public_h::PW_BLUEFLAG as libc::c_int
        {
            return;
        }
    }
    // grab it
    crate::src::game::bg_misc::BG_AddPredictableEventToPlayerstate(
        crate::bg_public_h::EV_ITEM_PICKUP as libc::c_int,
        (*cent).currentState.modelindex,
        &mut crate::src::cgame::cg_main::cg.predictedPlayerState as *mut _
            as *mut crate::src::qcommon::q_shared::playerState_s,
    );
    // remove it from the frame so it won't be drawn
    (*cent).currentState.eFlags |= 0x80 as libc::c_int;
    // don't touch it again this prediction
    (*cent).miscTime = crate::src::cgame::cg_main::cg.time;
    // if it's a weapon, give them some predicted ammo so the autoswitch will work
    if (*item).giType as libc::c_uint
        == crate::bg_public_h::IT_WEAPON as libc::c_int as libc::c_uint
    {
        crate::src::cgame::cg_main::cg.predictedPlayerState.stats
            [crate::bg_public_h::STAT_WEAPONS as libc::c_int as usize] |=
            (1 as libc::c_int) << (*item).giTag;
        if crate::src::cgame::cg_main::cg.predictedPlayerState.ammo[(*item).giTag as usize] == 0 {
            crate::src::cgame::cg_main::cg.predictedPlayerState.ammo[(*item).giTag as usize] =
                1 as libc::c_int
        }
    };
}
/*
=========================
CG_TouchTriggerPrediction

Predict push triggers and items
=========================
*/

unsafe extern "C" fn CG_TouchTriggerPrediction() {
    let mut i: libc::c_int = 0;
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
    let mut ent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut cmodel: crate::src::qcommon::q_shared::clipHandle_t = 0;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut spectator: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    // dead clients don't activate triggers
    if crate::src::cgame::cg_main::cg.predictedPlayerState.stats
        [crate::bg_public_h::STAT_HEALTH as libc::c_int as usize]
        <= 0 as libc::c_int
    {
        return;
    }
    spectator = (crate::src::cgame::cg_main::cg.predictedPlayerState.pm_type
        == crate::bg_public_h::PM_SPECTATOR as libc::c_int) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
    if crate::src::cgame::cg_main::cg.predictedPlayerState.pm_type
        != crate::bg_public_h::PM_NORMAL as libc::c_int
        && spectator as u64 == 0
    {
        return;
    }
    i = 0 as libc::c_int;
    while i < cg_numTriggerEntities {
        cent = cg_triggerEntities[i as usize];
        ent = &mut (*cent).currentState;
        if (*ent).eType == crate::bg_public_h::ET_ITEM as libc::c_int && spectator as u64 == 0 {
            CG_TouchItem(cent);
        } else if !((*ent).solid != 0xffffff as libc::c_int) {
            cmodel = crate::src::cgame::cg_syscalls::trap_CM_InlineModel((*ent).modelindex);
            if !(cmodel == 0) {
                crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
                    &mut trace as *mut _ as *mut crate::src::qcommon::q_shared::trace_t,
                    crate::src::cgame::cg_main::cg
                        .predictedPlayerState
                        .origin
                        .as_mut_ptr()
                        as *const crate::src::qcommon::q_shared::vec_t,
                    crate::src::cgame::cg_main::cg
                        .predictedPlayerState
                        .origin
                        .as_mut_ptr()
                        as *const crate::src::qcommon::q_shared::vec_t,
                    cg_pmove.mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    cg_pmove.maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    cmodel,
                    -(1 as libc::c_int),
                );
                if !(trace.startsolid as u64 == 0) {
                    if (*ent).eType == crate::bg_public_h::ET_TELEPORT_TRIGGER as libc::c_int {
                        crate::src::cgame::cg_main::cg.hyperspace =
                            crate::src::qcommon::q_shared::qtrue
                    } else if (*ent).eType == crate::bg_public_h::ET_PUSH_TRIGGER as libc::c_int {
                        crate::src::game::bg_misc::BG_TouchJumpPad(
                            &mut crate::src::cgame::cg_main::cg.predictedPlayerState as *mut _
                                as *mut crate::src::qcommon::q_shared::playerState_s,
                            ent as *mut crate::src::qcommon::q_shared::entityState_s,
                        );
                    }
                }
            }
        }
        i += 1
    }
    // if we didn't touch a jump pad this pmove frame
    if crate::src::cgame::cg_main::cg
        .predictedPlayerState
        .jumppad_frame
        != crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .pmove_framecount
    {
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .jumppad_frame = 0 as libc::c_int;
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .jumppad_ent = 0 as libc::c_int
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
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
/*
=================
CG_PredictPlayerState

Generates cg.predictedPlayerState for the current cg.time
cg.predictedPlayerState is guaranteed to be valid after exiting.

For demo playback, this will be an interpolation between two valid
playerState_t.

For normal gameplay, it will be the result of predicted usercmd_t on
top of the most recent playerState_t received from the server.

Each new snapshot will usually have one or more new usercmd over the last,
but we simulate all unacknowledged commands each time, not just the new ones.
This means that on an internet connection, quite a few pmoves may be issued
each frame.

OPTIMIZE: don't re-simulate unless the newly arrived snapshot playerState_t
differs from the predicted one.  Would require saving all intermediate
playerState_t during prediction.

We detect prediction errors and allow them to be decayed off over several frames
to ease the jerk.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PredictPlayerState() {
    let mut cmdNum: libc::c_int = 0; // will be set if touching a trigger_teleport
    let mut current: libc::c_int = 0;
    let mut oldPlayerState: crate::src::qcommon::q_shared::playerState_t =
        crate::src::qcommon::q_shared::playerState_t {
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
    let mut moved: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut oldestCmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    let mut latestCmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    crate::src::cgame::cg_main::cg.hyperspace = crate::src::qcommon::q_shared::qfalse;
    // if this is the first frame we must guarantee
    // predictedPlayerState is valid even if there is some
    // other error condition
    if crate::src::cgame::cg_main::cg.validPPS as u64 == 0 {
        crate::src::cgame::cg_main::cg.validPPS = crate::src::qcommon::q_shared::qtrue;
        crate::src::cgame::cg_main::cg.predictedPlayerState =
            (*crate::src::cgame::cg_main::cg.snap).ps
    }
    // demo playback just copies the moves
    if crate::src::cgame::cg_main::cg.demoPlayback as libc::c_uint != 0
        || (*crate::src::cgame::cg_main::cg.snap).ps.pm_flags & 4096 as libc::c_int != 0
    {
        CG_InterpolatePlayerState(crate::src::qcommon::q_shared::qfalse);
        return;
    }
    // non-predicting local movement will grab the latest angles
    if crate::src::cgame::cg_main::cg_nopredict.integer != 0
        || crate::src::cgame::cg_main::cg_synchronousClients.integer != 0
    {
        CG_InterpolatePlayerState(crate::src::qcommon::q_shared::qtrue);
        return;
    }
    // prepare for pmove
    cg_pmove.ps = &mut crate::src::cgame::cg_main::cg.predictedPlayerState;
    cg_pmove.trace = Some(
        CG_Trace
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::trace_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
                _: libc::c_int,
            ) -> (),
    );
    cg_pmove.pointcontents = Some(
        CG_PointContents
            as unsafe extern "C" fn(
                _: *const crate::src::qcommon::q_shared::vec_t,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    if (*cg_pmove.ps).pm_type == crate::bg_public_h::PM_DEAD as libc::c_int {
        cg_pmove.tracemask = (1 as libc::c_int | 0x10000 as libc::c_int | 0x2000000 as libc::c_int)
            & !(0x2000000 as libc::c_int)
    } else {
        cg_pmove.tracemask = 1 as libc::c_int | 0x10000 as libc::c_int | 0x2000000 as libc::c_int
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.persistant
        [crate::bg_public_h::PERS_TEAM as libc::c_int as usize]
        == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int
    {
        cg_pmove.tracemask &= !(0x2000000 as libc::c_int)
        // spectators can fly through bodies
    }
    cg_pmove.noFootsteps = (crate::src::cgame::cg_main::cgs.dmflags & 32 as libc::c_int
        > 0 as libc::c_int) as libc::c_int
        as crate::src::qcommon::q_shared::qboolean;
    // save the state before the pmove so we can detect transitions
    oldPlayerState = crate::src::cgame::cg_main::cg.predictedPlayerState;
    current = crate::src::cgame::cg_syscalls::trap_GetCurrentCmdNumber();
    // if we don't have the commands right after the snapshot, we
    // can't accurately predict a current position, so just freeze at
    // the last good position we had
    cmdNum = current - 64 as libc::c_int + 1 as libc::c_int;
    crate::src::cgame::cg_syscalls::trap_GetUserCmd(
        cmdNum,
        &mut oldestCmd as *mut _ as *mut crate::src::qcommon::q_shared::usercmd_s,
    );
    if oldestCmd.serverTime > (*crate::src::cgame::cg_main::cg.snap).ps.commandTime
        && oldestCmd.serverTime < crate::src::cgame::cg_main::cg.time
    {
        // special check for map_restart
        if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
            crate::src::cgame::cg_main::CG_Printf(
                b"exceeded PACKET_BACKUP on commands\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    // get the latest command so we can know which commands are from previous map_restarts
    crate::src::cgame::cg_syscalls::trap_GetUserCmd(
        current,
        &mut latestCmd as *mut _ as *mut crate::src::qcommon::q_shared::usercmd_s,
    );
    // get the most recent information we have, even if
    // the server time is beyond our current cg.time,
    // because predicted player positions are going to
    // be ahead of everything else anyway
    if !crate::src::cgame::cg_main::cg.nextSnap.is_null()
        && crate::src::cgame::cg_main::cg.nextFrameTeleport as u64 == 0
        && crate::src::cgame::cg_main::cg.thisFrameTeleport as u64 == 0
    {
        crate::src::cgame::cg_main::cg.predictedPlayerState =
            (*crate::src::cgame::cg_main::cg.nextSnap).ps; // | cg_pmove_fixed.integer;
        crate::src::cgame::cg_main::cg.physicsTime =
            (*crate::src::cgame::cg_main::cg.nextSnap).serverTime
    } else {
        crate::src::cgame::cg_main::cg.predictedPlayerState =
            (*crate::src::cgame::cg_main::cg.snap).ps;
        crate::src::cgame::cg_main::cg.physicsTime =
            (*crate::src::cgame::cg_main::cg.snap).serverTime
    }
    if crate::src::cgame::cg_main::pmove_msec.integer < 8 as libc::c_int {
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"8\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::cgame::cg_syscalls::trap_Cvar_Update(
            &mut crate::src::cgame::cg_main::pmove_msec as *mut _
                as *mut crate::src::qcommon::q_shared::vmCvar_t,
        );
    } else if crate::src::cgame::cg_main::pmove_msec.integer > 33 as libc::c_int {
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"33\x00" as *const u8 as *const libc::c_char,
        );
        crate::src::cgame::cg_syscalls::trap_Cvar_Update(
            &mut crate::src::cgame::cg_main::pmove_msec as *mut _
                as *mut crate::src::qcommon::q_shared::vmCvar_t,
        );
    }
    cg_pmove.pmove_fixed = crate::src::cgame::cg_main::pmove_fixed.integer;
    cg_pmove.pmove_msec = crate::src::cgame::cg_main::pmove_msec.integer;
    // run cmds
    moved = crate::src::qcommon::q_shared::qfalse;
    cmdNum = current - 64 as libc::c_int + 1 as libc::c_int;
    while cmdNum <= current {
        // get the command
        crate::src::cgame::cg_syscalls::trap_GetUserCmd(
            cmdNum,
            &mut cg_pmove.cmd as *mut _ as *mut crate::src::qcommon::q_shared::usercmd_s,
        );
        if cg_pmove.pmove_fixed != 0 {
            crate::src::game::bg_pmove::PM_UpdateViewAngles(
                cg_pmove.ps as *mut crate::src::qcommon::q_shared::playerState_s,
                &mut cg_pmove.cmd as *mut _ as *const crate::src::qcommon::q_shared::usercmd_s,
            );
        }
        // check for predictable events that changed from previous predictions
        //CG_CheckChangedPredictableEvents(&cg.predictedPlayerState);
        // don't do anything if the time is before the snapshot player time
        if !(cg_pmove.cmd.serverTime
            <= crate::src::cgame::cg_main::cg
                .predictedPlayerState
                .commandTime)
        {
            // don't do anything if the command was from a previous map_restart
            if !(cg_pmove.cmd.serverTime > latestCmd.serverTime) {
                // check for a prediction error from last frame
                // on a lan, this will often be the exact value
                // from the snapshot, but on a wan we will have
                // to predict several commands to get to the point
                // we want to compare
                if crate::src::cgame::cg_main::cg
                    .predictedPlayerState
                    .commandTime
                    == oldPlayerState.commandTime
                {
                    let mut delta: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                    let mut len: libc::c_float = 0.;
                    if crate::src::cgame::cg_main::cg.thisFrameTeleport as u64 != 0 {
                        // a teleport will not cause an error decay
                        crate::src::cgame::cg_main::cg.predictedError[2 as libc::c_int as usize] =
                            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                        crate::src::cgame::cg_main::cg.predictedError[1 as libc::c_int as usize] =
                            crate::src::cgame::cg_main::cg.predictedError
                                [2 as libc::c_int as usize];
                        crate::src::cgame::cg_main::cg.predictedError[0 as libc::c_int as usize] =
                            crate::src::cgame::cg_main::cg.predictedError
                                [1 as libc::c_int as usize];
                        if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
                            crate::src::cgame::cg_main::CG_Printf(
                                b"PredictionTeleport\n\x00" as *const u8 as *const libc::c_char,
                            );
                        }
                        crate::src::cgame::cg_main::cg.thisFrameTeleport =
                            crate::src::qcommon::q_shared::qfalse
                    } else {
                        let mut adjusted: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                        let mut new_angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                        crate::src::cgame::cg_ents::CG_AdjustPositionForMover(
                            crate::src::cgame::cg_main::cg
                                .predictedPlayerState
                                .origin
                                .as_mut_ptr()
                                as *const crate::src::qcommon::q_shared::vec_t,
                            crate::src::cgame::cg_main::cg
                                .predictedPlayerState
                                .groundEntityNum,
                            crate::src::cgame::cg_main::cg.physicsTime,
                            crate::src::cgame::cg_main::cg.oldTime,
                            adjusted.as_mut_ptr(),
                            crate::src::cgame::cg_main::cg
                                .predictedPlayerState
                                .viewangles
                                .as_mut_ptr(),
                            new_angles.as_mut_ptr(),
                        );
                        if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
                            if VectorCompare(
                                oldPlayerState.origin.as_mut_ptr()
                                    as *const crate::src::qcommon::q_shared::vec_t,
                                adjusted.as_mut_ptr()
                                    as *const crate::src::qcommon::q_shared::vec_t,
                            ) == 0
                            {
                                crate::src::cgame::cg_main::CG_Printf(
                                    b"prediction error\n\x00" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        delta[0 as libc::c_int as usize] = oldPlayerState.origin
                            [0 as libc::c_int as usize]
                            - adjusted[0 as libc::c_int as usize];
                        delta[1 as libc::c_int as usize] = oldPlayerState.origin
                            [1 as libc::c_int as usize]
                            - adjusted[1 as libc::c_int as usize];
                        delta[2 as libc::c_int as usize] = oldPlayerState.origin
                            [2 as libc::c_int as usize]
                            - adjusted[2 as libc::c_int as usize];
                        len = VectorLength(
                            delta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                        );
                        if len as libc::c_double > 0.1f64 {
                            if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
                                crate::src::cgame::cg_main::CG_Printf(
                                    b"Prediction miss: %f\n\x00" as *const u8
                                        as *const libc::c_char,
                                    len as libc::c_double,
                                );
                            }
                            if crate::src::cgame::cg_main::cg_errorDecay.integer != 0 {
                                let mut t: libc::c_int = 0;
                                let mut f: libc::c_float = 0.;
                                t = crate::src::cgame::cg_main::cg.time
                                    - crate::src::cgame::cg_main::cg.predictedErrorTime;
                                f = (crate::src::cgame::cg_main::cg_errorDecay.value
                                    - t as libc::c_float)
                                    / crate::src::cgame::cg_main::cg_errorDecay.value;
                                if f < 0 as libc::c_int as libc::c_float {
                                    f = 0 as libc::c_int as libc::c_float
                                }
                                if f > 0 as libc::c_int as libc::c_float
                                    && crate::src::cgame::cg_main::cg_showmiss.integer != 0
                                {
                                    crate::src::cgame::cg_main::CG_Printf(
                                        b"Double prediction decay: %f\n\x00" as *const u8
                                            as *const libc::c_char,
                                        f as libc::c_double,
                                    );
                                }
                                crate::src::cgame::cg_main::cg.predictedError
                                    [0 as libc::c_int as usize] = crate::src::cgame::cg_main::cg
                                    .predictedError[0 as libc::c_int as usize]
                                    * f;
                                crate::src::cgame::cg_main::cg.predictedError
                                    [1 as libc::c_int as usize] = crate::src::cgame::cg_main::cg
                                    .predictedError[1 as libc::c_int as usize]
                                    * f;
                                crate::src::cgame::cg_main::cg.predictedError
                                    [2 as libc::c_int as usize] = crate::src::cgame::cg_main::cg
                                    .predictedError[2 as libc::c_int as usize]
                                    * f
                            } else {
                                crate::src::cgame::cg_main::cg.predictedError
                                    [2 as libc::c_int as usize] =
                                    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
                                crate::src::cgame::cg_main::cg.predictedError
                                    [1 as libc::c_int as usize] = crate::src::cgame::cg_main::cg
                                    .predictedError[2 as libc::c_int as usize];
                                crate::src::cgame::cg_main::cg.predictedError
                                    [0 as libc::c_int as usize] = crate::src::cgame::cg_main::cg
                                    .predictedError[1 as libc::c_int as usize]
                            }
                            crate::src::cgame::cg_main::cg.predictedError
                                [0 as libc::c_int as usize] = delta[0 as libc::c_int as usize]
                                + crate::src::cgame::cg_main::cg.predictedError
                                    [0 as libc::c_int as usize];
                            crate::src::cgame::cg_main::cg.predictedError
                                [1 as libc::c_int as usize] = delta[1 as libc::c_int as usize]
                                + crate::src::cgame::cg_main::cg.predictedError
                                    [1 as libc::c_int as usize];
                            crate::src::cgame::cg_main::cg.predictedError
                                [2 as libc::c_int as usize] = delta[2 as libc::c_int as usize]
                                + crate::src::cgame::cg_main::cg.predictedError
                                    [2 as libc::c_int as usize];
                            crate::src::cgame::cg_main::cg.predictedErrorTime =
                                crate::src::cgame::cg_main::cg.oldTime
                        }
                    }
                }
                // don't predict gauntlet firing, which is only supposed to happen
                // when it actually inflicts damage
                cg_pmove.gauntletHit = crate::src::qcommon::q_shared::qfalse;
                if cg_pmove.pmove_fixed != 0 {
                    cg_pmove.cmd.serverTime = (cg_pmove.cmd.serverTime
                        + crate::src::cgame::cg_main::pmove_msec.integer
                        - 1 as libc::c_int)
                        / crate::src::cgame::cg_main::pmove_msec.integer
                        * crate::src::cgame::cg_main::pmove_msec.integer
                }
                crate::src::game::bg_pmove::Pmove(
                    &mut cg_pmove as *mut _ as *mut crate::bg_public_h::pmove_t,
                );
                moved = crate::src::qcommon::q_shared::qtrue;
                // add push trigger movement effects
                CG_TouchTriggerPrediction();
            }
        }
        cmdNum += 1
    }
    if crate::src::cgame::cg_main::cg_showmiss.integer > 1 as libc::c_int {
        crate::src::cgame::cg_main::CG_Printf(
            b"[%i : %i] \x00" as *const u8 as *const libc::c_char,
            cg_pmove.cmd.serverTime,
            crate::src::cgame::cg_main::cg.time,
        );
    }
    if moved as u64 == 0 {
        if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
            crate::src::cgame::cg_main::CG_Printf(
                b"not moved\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        return;
    }
    // adjust for the movement of the groundentity
    crate::src::cgame::cg_ents::CG_AdjustPositionForMover(
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .origin
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .groundEntityNum,
        crate::src::cgame::cg_main::cg.physicsTime,
        crate::src::cgame::cg_main::cg.time,
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .origin
            .as_mut_ptr(),
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .viewangles
            .as_mut_ptr(),
        crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .viewangles
            .as_mut_ptr(),
    );
    if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
        if crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .eventSequence
            > oldPlayerState.eventSequence + 2 as libc::c_int
        {
            crate::src::cgame::cg_main::CG_Printf(
                b"WARNING: dropped event\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    // fire events and other transition triggered things
    crate::src::cgame::cg_playerstate::CG_TransitionPlayerState(
        &mut crate::src::cgame::cg_main::cg.predictedPlayerState as *mut _
            as *mut crate::src::qcommon::q_shared::playerState_s,
        &mut oldPlayerState as *mut _ as *mut crate::src::qcommon::q_shared::playerState_s,
    );
    if crate::src::cgame::cg_main::cg_showmiss.integer != 0 {
        if crate::src::cgame::cg_main::cg.eventSequence
            > crate::src::cgame::cg_main::cg
                .predictedPlayerState
                .eventSequence
        {
            crate::src::cgame::cg_main::CG_Printf(
                b"WARNING: double event\n\x00" as *const u8 as *const libc::c_char,
            );
            crate::src::cgame::cg_main::cg.eventSequence = crate::src::cgame::cg_main::cg
                .predictedPlayerState
                .eventSequence
        }
    };
}
