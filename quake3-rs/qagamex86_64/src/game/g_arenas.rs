use ::libc;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
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
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
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
pub use crate::bg_public_h::LEGS_BACK;
pub use crate::bg_public_h::LEGS_BACKCR;
pub use crate::bg_public_h::LEGS_BACKWALK;
pub use crate::bg_public_h::LEGS_IDLE;
pub use crate::bg_public_h::LEGS_IDLECR;
pub use crate::bg_public_h::LEGS_JUMP;
pub use crate::bg_public_h::LEGS_JUMPB;
pub use crate::bg_public_h::LEGS_LAND;
pub use crate::bg_public_h::LEGS_LANDB;
pub use crate::bg_public_h::LEGS_RUN;
pub use crate::bg_public_h::LEGS_SWIM;
pub use crate::bg_public_h::LEGS_TURN;
pub use crate::bg_public_h::LEGS_WALK;
pub use crate::bg_public_h::LEGS_WALKCR;
pub use crate::bg_public_h::MAX_ANIMATIONS;
pub use crate::bg_public_h::MAX_TOTALANIMATIONS;
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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::bg_public_h::TORSO_AFFIRMATIVE;
pub use crate::bg_public_h::TORSO_ATTACK;
pub use crate::bg_public_h::TORSO_ATTACK2;
pub use crate::bg_public_h::TORSO_DROP;
pub use crate::bg_public_h::TORSO_FOLLOWME;
pub use crate::bg_public_h::TORSO_GESTURE;
pub use crate::bg_public_h::TORSO_GETFLAG;
pub use crate::bg_public_h::TORSO_GUARDBASE;
pub use crate::bg_public_h::TORSO_NEGATIVE;
pub use crate::bg_public_h::TORSO_PATROL;
pub use crate::bg_public_h::TORSO_RAISE;
pub use crate::bg_public_h::TORSO_STAND;
pub use crate::bg_public_h::TORSO_STAND2;
pub use crate::bg_public_h::WP_BFG;
pub use crate::bg_public_h::WP_GAUNTLET;
pub use crate::bg_public_h::WP_GRAPPLING_HOOK;
pub use crate::bg_public_h::WP_GRENADE_LAUNCHER;
pub use crate::bg_public_h::WP_LIGHTNING;
pub use crate::bg_public_h::WP_MACHINEGUN;
pub use crate::bg_public_h::WP_NONE;
pub use crate::bg_public_h::WP_NUM_WEAPONS;
pub use crate::bg_public_h::WP_PLASMAGUN;
pub use crate::bg_public_h::WP_RAILGUN;
pub use crate::bg_public_h::WP_ROCKET_LAUNCHER;
pub use crate::bg_public_h::WP_SHOTGUN;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
pub use crate::g_local_h::gentity_s;
pub use crate::g_local_h::gentity_t;
pub use crate::g_local_h::level_locals_t;
pub use crate::g_local_h::moverState_t;
pub use crate::g_local_h::playerTeamStateState_t;
pub use crate::g_local_h::playerTeamState_t;
pub use crate::g_local_h::spectatorState_t;
pub use crate::g_local_h::CON_CONNECTED;
pub use crate::g_local_h::CON_CONNECTING;
pub use crate::g_local_h::CON_DISCONNECTED;
pub use crate::g_local_h::MOVER_1TO2;
pub use crate::g_local_h::MOVER_2TO1;
pub use crate::g_local_h::MOVER_POS1;
pub use crate::g_local_h::MOVER_POS2;
pub use crate::g_local_h::SPECTATOR_FOLLOW;
pub use crate::g_local_h::SPECTATOR_FREE;
pub use crate::g_local_h::SPECTATOR_NOT;
pub use crate::g_local_h::SPECTATOR_SCOREBOARD;
pub use crate::g_local_h::TEAM_ACTIVE;
pub use crate::g_local_h::TEAM_BEGIN;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::CalculateRanks;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_utils::vectoyaw;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_ModelIndex;
pub use crate::src::game::g_utils::G_SetOrigin;
pub use crate::src::game::g_utils::G_Spawn;
use crate::stdlib::strlen;
use ::libc::strcat;
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
//
// g_arenas.c
//
#[no_mangle]

pub static mut podium1: *mut crate::g_local_h::gentity_t =
    0 as *const crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_t;
#[no_mangle]

pub static mut podium2: *mut crate::g_local_h::gentity_t =
    0 as *const crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_t;
#[no_mangle]

pub static mut podium3: *mut crate::g_local_h::gentity_t =
    0 as *const crate::g_local_h::gentity_t as *mut crate::g_local_h::gentity_t;
/*
==================
UpdateTournamentInfo
==================
*/
#[no_mangle]

pub unsafe extern "C" fn UpdateTournamentInfo() {
    let mut i: libc::c_int = 0;
    let mut player: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut playerClientNum: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut accuracy: libc::c_int = 0;
    let mut perfect: libc::c_int = 0;
    let mut msglen: libc::c_int = 0;
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    // find the real player
    player = 0 as *mut crate::g_local_h::gentity_t;
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.maxclients {
        player = &mut *crate::src::game::g_main::g_entities
            .as_mut_ptr()
            .offset(i as isize) as *mut crate::g_local_h::gentity_t;
        if !((*player).inuse as u64 == 0) {
            if (*player).r.svFlags & 0x8 as libc::c_int == 0 {
                break;
            }
        }
        i += 1
    }
    // this should never happen!
    if player.is_null() || i == crate::src::game::g_main::level.maxclients {
        return;
    } // could be ET_INVISIBLE
    playerClientNum = i; // clear EF_TALK, etc
    crate::src::game::g_main::CalculateRanks(); // clear powerups
    if (*crate::src::game::g_main::level
        .clients
        .offset(playerClientNum as isize))
    .sess
    .sessionTeam as libc::c_uint
        == crate::bg_public_h::TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        crate::src::qcommon::q_shared::Com_sprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"postgame %i %i 0 0 0 0 0 0\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_main::level.numNonSpectatorClients,
            playerClientNum,
        ); // clear lava burning
    } else {
        if (*(*player).client).accuracy_shots != 0 {
            accuracy = (*(*player).client).accuracy_hits * 100 as libc::c_int
                / (*(*player).client).accuracy_shots
        } else {
            accuracy = 0 as libc::c_int
        } // don't bounce
        perfect = if (*crate::src::game::g_main::level
            .clients
            .offset(playerClientNum as isize))
        .ps
        .persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize]
            == 0 as libc::c_int
            && (*(*player).client).ps.persistant
                [crate::bg_public_h::PERS_KILLED as libc::c_int as usize]
                == 0 as libc::c_int
        {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        crate::src::qcommon::q_shared::Com_sprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"postgame %i %i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char,
            crate::src::game::g_main::level.numNonSpectatorClients,
            playerClientNum,
            accuracy,
            (*(*player).client).ps.persistant
                [crate::bg_public_h::PERS_IMPRESSIVE_COUNT as libc::c_int as usize],
            (*(*player).client).ps.persistant
                [crate::bg_public_h::PERS_EXCELLENT_COUNT as libc::c_int as usize],
            (*(*player).client).ps.persistant
                [crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize],
            (*(*player).client).ps.persistant
                [crate::bg_public_h::PERS_SCORE as libc::c_int as usize],
            perfect,
        );
    }
    msglen = crate::stdlib::strlen(msg.as_mut_ptr()) as libc::c_int;
    i = 0 as libc::c_int;
    while i < crate::src::game::g_main::level.numNonSpectatorClients {
        n = crate::src::game::g_main::level.sortedClients[i as usize];
        crate::src::qcommon::q_shared::Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b" %i %i %i\x00" as *const u8 as *const libc::c_char,
            n,
            (*crate::src::game::g_main::level.clients.offset(n as isize))
                .ps
                .persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize],
            (*crate::src::game::g_main::level.clients.offset(n as isize))
                .ps
                .persistant[crate::bg_public_h::PERS_SCORE as libc::c_int as usize],
        );
        msglen = (msglen as libc::c_ulong).wrapping_add(crate::stdlib::strlen(buf.as_mut_ptr()))
            as libc::c_int as libc::c_int;
        if msglen as libc::c_ulong >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        {
            break;
        }
        ::libc::strcat(msg.as_mut_ptr(), buf.as_mut_ptr());
        i += 1
    }
    crate::src::game::g_syscalls::trap_SendConsoleCommand(
        crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
        msg.as_mut_ptr(),
    );
}

unsafe extern "C" fn SpawnModelOnVictoryPad(
    mut pad: *mut crate::g_local_h::gentity_t,
    mut offset: *mut crate::src::qcommon::q_shared::vec_t,
    mut ent: *mut crate::g_local_h::gentity_t,
    mut place: libc::c_int,
) -> *mut crate::g_local_h::gentity_t {
    let mut body: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut r: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut u: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    body = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    if body.is_null() {
        crate::src::game::g_main::G_Printf(
            b"^1ERROR: out of gentities\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut crate::g_local_h::gentity_t;
    }
    (*body).classname = (*(*ent).client).pers.netname.as_mut_ptr();
    (*body).client = (*ent).client;
    (*body).s = (*ent).s;
    (*body).s.eType = crate::bg_public_h::ET_PLAYER as libc::c_int;
    (*body).s.eFlags = 0 as libc::c_int;
    (*body).s.powerups = 0 as libc::c_int;
    (*body).s.loopSound = 0 as libc::c_int;
    (*body).s.number = body.wrapping_offset_from(crate::src::game::g_main::g_entities.as_mut_ptr())
        as libc::c_long as libc::c_int;
    (*body).timestamp = crate::src::game::g_main::level.time;
    (*body).physicsObject = crate::src::qcommon::q_shared::qtrue;
    (*body).physicsBounce = 0 as libc::c_int as libc::c_float;
    (*body).s.event = 0 as libc::c_int;
    (*body).s.pos.trType = crate::src::qcommon::q_shared::TR_STATIONARY;
    (*body).s.groundEntityNum = ((1 as libc::c_int) << 10 as libc::c_int) - 2 as libc::c_int;
    (*body).s.legsAnim = crate::bg_public_h::LEGS_IDLE as libc::c_int;
    (*body).s.torsoAnim = crate::bg_public_h::TORSO_STAND as libc::c_int;
    if (*body).s.weapon == crate::bg_public_h::WP_NONE as libc::c_int {
        (*body).s.weapon = crate::bg_public_h::WP_MACHINEGUN as libc::c_int
    }
    if (*body).s.weapon == crate::bg_public_h::WP_GAUNTLET as libc::c_int {
        (*body).s.torsoAnim = crate::bg_public_h::TORSO_STAND2 as libc::c_int
    }
    (*body).s.event = 0 as libc::c_int;
    (*body).r.svFlags = (*ent).r.svFlags;
    (*body).r.mins[0 as libc::c_int as usize] = (*ent).r.mins[0 as libc::c_int as usize];
    (*body).r.mins[1 as libc::c_int as usize] = (*ent).r.mins[1 as libc::c_int as usize];
    (*body).r.mins[2 as libc::c_int as usize] = (*ent).r.mins[2 as libc::c_int as usize];
    (*body).r.maxs[0 as libc::c_int as usize] = (*ent).r.maxs[0 as libc::c_int as usize];
    (*body).r.maxs[1 as libc::c_int as usize] = (*ent).r.maxs[1 as libc::c_int as usize];
    (*body).r.maxs[2 as libc::c_int as usize] = (*ent).r.maxs[2 as libc::c_int as usize];
    (*body).r.absmin[0 as libc::c_int as usize] = (*ent).r.absmin[0 as libc::c_int as usize];
    (*body).r.absmin[1 as libc::c_int as usize] = (*ent).r.absmin[1 as libc::c_int as usize];
    (*body).r.absmin[2 as libc::c_int as usize] = (*ent).r.absmin[2 as libc::c_int as usize];
    (*body).r.absmax[0 as libc::c_int as usize] = (*ent).r.absmax[0 as libc::c_int as usize];
    (*body).r.absmax[1 as libc::c_int as usize] = (*ent).r.absmax[1 as libc::c_int as usize];
    (*body).r.absmax[2 as libc::c_int as usize] = (*ent).r.absmax[2 as libc::c_int as usize];
    (*body).clipmask = 1 as libc::c_int | 0x10000 as libc::c_int;
    (*body).r.contents = 0x2000000 as libc::c_int;
    (*body).r.ownerNum = (*ent).r.ownerNum;
    (*body).takedamage = crate::src::qcommon::q_shared::qfalse;
    vec[0 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [0 as libc::c_int as usize]
        - (*pad).r.currentOrigin[0 as libc::c_int as usize];
    vec[1 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [1 as libc::c_int as usize]
        - (*pad).r.currentOrigin[1 as libc::c_int as usize];
    vec[2 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [2 as libc::c_int as usize]
        - (*pad).r.currentOrigin[2 as libc::c_int as usize];
    crate::src::qcommon::q_math::vectoangles(
        vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*body).s.apos.trBase.as_mut_ptr(),
    );
    (*body).s.apos.trBase[0 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    (*body).s.apos.trBase[2 as libc::c_int as usize] =
        0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AngleVectors(
        (*body).s.apos.trBase.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        f.as_mut_ptr(),
        r.as_mut_ptr(),
        u.as_mut_ptr(),
    );
    vec[0 as libc::c_int as usize] = (*pad).r.currentOrigin[0 as libc::c_int as usize]
        + f[0 as libc::c_int as usize] * *offset.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int as usize] = (*pad).r.currentOrigin[1 as libc::c_int as usize]
        + f[1 as libc::c_int as usize] * *offset.offset(0 as libc::c_int as isize);
    vec[2 as libc::c_int as usize] = (*pad).r.currentOrigin[2 as libc::c_int as usize]
        + f[2 as libc::c_int as usize] * *offset.offset(0 as libc::c_int as isize);
    vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
        + r[0 as libc::c_int as usize] * *offset.offset(1 as libc::c_int as isize);
    vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
        + r[1 as libc::c_int as usize] * *offset.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
        + r[2 as libc::c_int as usize] * *offset.offset(1 as libc::c_int as isize);
    vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
        + u[0 as libc::c_int as usize] * *offset.offset(2 as libc::c_int as isize);
    vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
        + u[1 as libc::c_int as usize] * *offset.offset(2 as libc::c_int as isize);
    vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
        + u[2 as libc::c_int as usize] * *offset.offset(2 as libc::c_int as isize);
    crate::src::game::g_utils::G_SetOrigin(
        body as *mut crate::g_local_h::gentity_s,
        vec.as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_LinkEntity(body as *mut crate::g_local_h::gentity_s);
    (*body).count = place;
    return body;
}

unsafe extern "C" fn CelebrateStop(mut player: *mut crate::g_local_h::gentity_t) {
    let mut anim: libc::c_int = 0;
    if (*player).s.weapon == crate::bg_public_h::WP_GAUNTLET as libc::c_int {
        anim = crate::bg_public_h::TORSO_STAND2 as libc::c_int
    } else {
        anim = crate::bg_public_h::TORSO_STAND as libc::c_int
    }
    (*player).s.torsoAnim = (*player).s.torsoAnim & 128 as libc::c_int ^ 128 as libc::c_int | anim;
}

unsafe extern "C" fn CelebrateStart(mut player: *mut crate::g_local_h::gentity_t) {
    (*player).s.torsoAnim = (*player).s.torsoAnim & 128 as libc::c_int ^ 128 as libc::c_int
        | crate::bg_public_h::TORSO_GESTURE as libc::c_int;
    (*player).nextthink = crate::src::game::g_main::level.time
        + (34 as libc::c_int * 66 as libc::c_int + 50 as libc::c_int);
    (*player).think =
        Some(CelebrateStop as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    /*
    player->client->ps.events[player->client->ps.eventSequence & (MAX_PS_EVENTS-1)] = EV_TAUNT;
    player->client->ps.eventParms[player->client->ps.eventSequence & (MAX_PS_EVENTS-1)] = 0;
    player->client->ps.eventSequence++;
    */
    crate::src::game::g_utils::G_AddEvent(
        player as *mut crate::g_local_h::gentity_s,
        crate::bg_public_h::EV_TAUNT as libc::c_int,
        0 as libc::c_int,
    );
}

static mut offsetFirst: crate::src::qcommon::q_shared::vec3_t = [
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    0 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    74 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];

static mut offsetSecond: crate::src::qcommon::q_shared::vec3_t = [
    -(10 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
    60 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
    54 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];

static mut offsetThird: crate::src::qcommon::q_shared::vec3_t = [
    -(19 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
    -(60 as libc::c_int) as crate::src::qcommon::q_shared::vec_t,
    45 as libc::c_int as crate::src::qcommon::q_shared::vec_t,
];

unsafe extern "C" fn PodiumPlacementThink(mut podium: *mut crate::g_local_h::gentity_t) {
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut r: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut u: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    (*podium).nextthink = crate::src::game::g_main::level.time + 100 as libc::c_int;
    crate::src::qcommon::q_math::AngleVectors(
        crate::src::game::g_main::level
            .intermission_angle
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vec.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    origin[0 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [0 as libc::c_int as usize]
        + vec[0 as libc::c_int as usize]
            * crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[1 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [1 as libc::c_int as usize]
        + vec[1 as libc::c_int as usize]
            * crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [2 as libc::c_int as usize]
        + vec[2 as libc::c_int as usize]
            * crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2 as libc::c_int as usize] -=
        crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
            b"g_podiumDrop\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_float;
    crate::src::game::g_utils::G_SetOrigin(
        podium as *mut crate::g_local_h::gentity_s,
        origin.as_mut_ptr(),
    );
    if !podium1.is_null() {
        vec[0 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [0 as libc::c_int as usize]
            - (*podium).r.currentOrigin[0 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [1 as libc::c_int as usize]
            - (*podium).r.currentOrigin[1 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [2 as libc::c_int as usize]
            - (*podium).r.currentOrigin[2 as libc::c_int as usize];
        crate::src::qcommon::q_math::vectoangles(
            vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*podium1).s.apos.trBase.as_mut_ptr(),
        );
        (*podium1).s.apos.trBase[0 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*podium1).s.apos.trBase[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::AngleVectors(
            (*podium1).s.apos.trBase.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            f.as_mut_ptr(),
            r.as_mut_ptr(),
            u.as_mut_ptr(),
        );
        vec[0 as libc::c_int as usize] = (*podium).r.currentOrigin[0 as libc::c_int as usize]
            + f[0 as libc::c_int as usize] * offsetFirst[0 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = (*podium).r.currentOrigin[1 as libc::c_int as usize]
            + f[1 as libc::c_int as usize] * offsetFirst[0 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = (*podium).r.currentOrigin[2 as libc::c_int as usize]
            + f[2 as libc::c_int as usize] * offsetFirst[0 as libc::c_int as usize];
        vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
            + r[0 as libc::c_int as usize] * offsetFirst[1 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
            + r[1 as libc::c_int as usize] * offsetFirst[1 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
            + r[2 as libc::c_int as usize] * offsetFirst[1 as libc::c_int as usize];
        vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
            + u[0 as libc::c_int as usize] * offsetFirst[2 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
            + u[1 as libc::c_int as usize] * offsetFirst[2 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
            + u[2 as libc::c_int as usize] * offsetFirst[2 as libc::c_int as usize];
        crate::src::game::g_utils::G_SetOrigin(
            podium1 as *mut crate::g_local_h::gentity_s,
            vec.as_mut_ptr(),
        );
    }
    if !podium2.is_null() {
        vec[0 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [0 as libc::c_int as usize]
            - (*podium).r.currentOrigin[0 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [1 as libc::c_int as usize]
            - (*podium).r.currentOrigin[1 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [2 as libc::c_int as usize]
            - (*podium).r.currentOrigin[2 as libc::c_int as usize];
        crate::src::qcommon::q_math::vectoangles(
            vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*podium2).s.apos.trBase.as_mut_ptr(),
        );
        (*podium2).s.apos.trBase[0 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*podium2).s.apos.trBase[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::AngleVectors(
            (*podium2).s.apos.trBase.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            f.as_mut_ptr(),
            r.as_mut_ptr(),
            u.as_mut_ptr(),
        );
        vec[0 as libc::c_int as usize] = (*podium).r.currentOrigin[0 as libc::c_int as usize]
            + f[0 as libc::c_int as usize] * offsetSecond[0 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = (*podium).r.currentOrigin[1 as libc::c_int as usize]
            + f[1 as libc::c_int as usize] * offsetSecond[0 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = (*podium).r.currentOrigin[2 as libc::c_int as usize]
            + f[2 as libc::c_int as usize] * offsetSecond[0 as libc::c_int as usize];
        vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
            + r[0 as libc::c_int as usize] * offsetSecond[1 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
            + r[1 as libc::c_int as usize] * offsetSecond[1 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
            + r[2 as libc::c_int as usize] * offsetSecond[1 as libc::c_int as usize];
        vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
            + u[0 as libc::c_int as usize] * offsetSecond[2 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
            + u[1 as libc::c_int as usize] * offsetSecond[2 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
            + u[2 as libc::c_int as usize] * offsetSecond[2 as libc::c_int as usize];
        crate::src::game::g_utils::G_SetOrigin(
            podium2 as *mut crate::g_local_h::gentity_s,
            vec.as_mut_ptr(),
        );
    }
    if !podium3.is_null() {
        vec[0 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [0 as libc::c_int as usize]
            - (*podium).r.currentOrigin[0 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [1 as libc::c_int as usize]
            - (*podium).r.currentOrigin[1 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
            [2 as libc::c_int as usize]
            - (*podium).r.currentOrigin[2 as libc::c_int as usize];
        crate::src::qcommon::q_math::vectoangles(
            vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*podium3).s.apos.trBase.as_mut_ptr(),
        );
        (*podium3).s.apos.trBase[0 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        (*podium3).s.apos.trBase[2 as libc::c_int as usize] =
            0 as libc::c_int as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::AngleVectors(
            (*podium3).s.apos.trBase.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            f.as_mut_ptr(),
            r.as_mut_ptr(),
            u.as_mut_ptr(),
        );
        vec[0 as libc::c_int as usize] = (*podium).r.currentOrigin[0 as libc::c_int as usize]
            + f[0 as libc::c_int as usize] * offsetThird[0 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = (*podium).r.currentOrigin[1 as libc::c_int as usize]
            + f[1 as libc::c_int as usize] * offsetThird[0 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = (*podium).r.currentOrigin[2 as libc::c_int as usize]
            + f[2 as libc::c_int as usize] * offsetThird[0 as libc::c_int as usize];
        vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
            + r[0 as libc::c_int as usize] * offsetThird[1 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
            + r[1 as libc::c_int as usize] * offsetThird[1 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
            + r[2 as libc::c_int as usize] * offsetThird[1 as libc::c_int as usize];
        vec[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize]
            + u[0 as libc::c_int as usize] * offsetThird[2 as libc::c_int as usize];
        vec[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize]
            + u[1 as libc::c_int as usize] * offsetThird[2 as libc::c_int as usize];
        vec[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize]
            + u[2 as libc::c_int as usize] * offsetThird[2 as libc::c_int as usize];
        crate::src::game::g_utils::G_SetOrigin(
            podium3 as *mut crate::g_local_h::gentity_s,
            vec.as_mut_ptr(),
        );
    };
}

unsafe extern "C" fn SpawnPodium() -> *mut crate::g_local_h::gentity_t {
    let mut podium: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    podium = crate::src::game::g_utils::G_Spawn() as *mut crate::g_local_h::gentity_s;
    if podium.is_null() {
        return 0 as *mut crate::g_local_h::gentity_t;
    }
    (*podium).classname = b"podium\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*podium).s.eType = crate::bg_public_h::ET_GENERAL as libc::c_int;
    (*podium).s.number = podium
        .wrapping_offset_from(crate::src::game::g_main::g_entities.as_mut_ptr())
        as libc::c_long as libc::c_int;
    (*podium).clipmask = 1 as libc::c_int;
    (*podium).r.contents = 1 as libc::c_int;
    (*podium).s.modelindex = crate::src::game::g_utils::G_ModelIndex(
        b"models/mapobjects/podium/podium4.md3\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    crate::src::qcommon::q_math::AngleVectors(
        crate::src::game::g_main::level
            .intermission_angle
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        vec.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    origin[0 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [0 as libc::c_int as usize]
        + vec[0 as libc::c_int as usize]
            * crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[1 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [1 as libc::c_int as usize]
        + vec[1 as libc::c_int as usize]
            * crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [2 as libc::c_int as usize]
        + vec[2 as libc::c_int as usize]
            * crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2 as libc::c_int as usize] -=
        crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
            b"g_podiumDrop\x00" as *const u8 as *const libc::c_char,
        ) as libc::c_float;
    crate::src::game::g_utils::G_SetOrigin(
        podium as *mut crate::g_local_h::gentity_s,
        origin.as_mut_ptr(),
    );
    vec[0 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [0 as libc::c_int as usize]
        - (*podium).r.currentOrigin[0 as libc::c_int as usize];
    vec[1 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [1 as libc::c_int as usize]
        - (*podium).r.currentOrigin[1 as libc::c_int as usize];
    vec[2 as libc::c_int as usize] = crate::src::game::g_main::level.intermission_origin
        [2 as libc::c_int as usize]
        - (*podium).r.currentOrigin[2 as libc::c_int as usize];
    (*podium).s.apos.trBase[1 as libc::c_int as usize] = crate::src::game::g_utils::vectoyaw(
        vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    crate::src::game::g_syscalls::trap_LinkEntity(podium as *mut crate::g_local_h::gentity_s);
    (*podium).think = Some(
        PodiumPlacementThink as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
    );
    (*podium).nextthink = crate::src::game::g_main::level.time + 100 as libc::c_int;
    return podium;
}
/*
==================
SpawnModelsOnVictoryPads
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SpawnModelsOnVictoryPads() {
    let mut player: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut podium: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    podium1 = 0 as *mut crate::g_local_h::gentity_t;
    podium2 = 0 as *mut crate::g_local_h::gentity_t;
    podium3 = 0 as *mut crate::g_local_h::gentity_t;
    podium = SpawnPodium();
    player = SpawnModelOnVictoryPad(
        podium,
        offsetFirst.as_mut_ptr(),
        &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(
            *crate::src::game::g_main::level
                .sortedClients
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as isize,
        ),
        (*crate::src::game::g_main::level.clients.offset(
            crate::src::game::g_main::level.sortedClients[0 as libc::c_int as usize] as isize,
        ))
        .ps
        .persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize]
            & !(0x4000 as libc::c_int),
    );
    if !player.is_null() {
        (*player).nextthink = crate::src::game::g_main::level.time + 2000 as libc::c_int;
        (*player).think =
            Some(CelebrateStart as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
        podium1 = player
    }
    player = SpawnModelOnVictoryPad(
        podium,
        offsetSecond.as_mut_ptr(),
        &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(
            *crate::src::game::g_main::level
                .sortedClients
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as isize,
        ),
        (*crate::src::game::g_main::level.clients.offset(
            crate::src::game::g_main::level.sortedClients[1 as libc::c_int as usize] as isize,
        ))
        .ps
        .persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize]
            & !(0x4000 as libc::c_int),
    );
    if !player.is_null() {
        podium2 = player
    }
    if crate::src::game::g_main::level.numNonSpectatorClients > 2 as libc::c_int {
        player = SpawnModelOnVictoryPad(
            podium,
            offsetThird.as_mut_ptr(),
            &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(
                *crate::src::game::g_main::level
                    .sortedClients
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as isize,
            ),
            (*crate::src::game::g_main::level.clients.offset(
                crate::src::game::g_main::level.sortedClients[2 as libc::c_int as usize] as isize,
            ))
            .ps
            .persistant[crate::bg_public_h::PERS_RANK as libc::c_int as usize]
                & !(0x4000 as libc::c_int),
        );
        if !player.is_null() {
            podium3 = player
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
// g_local.h -- local definitions for game module
//==================================================================
// the "gameversion" client command will print this plus compile date
// msec
// gentity->flags
// not the first on the team
// spawn point not for bot use
// spawn point just for bots
// force gesture on client
// movers are things like doors, plats, buttons, etc
//============================================================================
// communicated by server to clients
// shared by both the server system and game
// DO NOT MODIFY ANYTHING ABOVE THIS, THE SERVER
// EXPECTS THE FIELDS IN THAT ORDER!
//================================
// NULL if not a client
// set in QuakeEd
// set in QuakeEd
// if true, FreeEntity will only unlink
// bodyque uses this
// FL_* variables
// level.time when the object was freed
// events will be cleared EVENT_VALID_MSEC after set
// if true, it can be pushed by movers and fall off edges
// all game items are physicsObjects,
// 1.0 = continuous bounce, 0.0 = no bounce
// brushes with this content value will be collided against
// when moving.  items and corpses do not collide against
// players, for instance
// movers
// body queue sinking, etc
// movers call this when hitting endpoint
// wind tunnel
// quad will increase this without increasing radius
// next entity in team
// master of the team
// timing variables
// for bonus items
// Beginning a team game, spawn at base
// Now actively playing
// client data that stays across multiple levels or tournament restarts
// this is achieved by writing all the data to cvar strings at game shutdown
// time and reading them back at connection time.  Anything added here
// MUST be dealt with in G_InitSessionData() / G_ReadSessionData() / G_WriteSessionData()
// for determining next-in-line to play
// for chasecam and follow mode
// tournament stats
// true when this client is a team leader
//
// client data that stays across multiple respawns, but is cleared
// on each level change or team change at ClientBegin()
// we would lose angles if not persistant
// true if "ip" info key is "localhost"
// the first spawn should be at a cool location
// based on cg_predictItems userinfo
//
// for handicapping
// level.time the client entered the game
// status in teamplay games
// to prevent people from constantly calling votes
// to prevent people from constantly calling votes
// send team overlay updates?
// this structure is cleared on each ClientSpawn(),
// except for 'client->pers' and 'client->sess'
// ps MUST be the first element, because the server expects it
// communicated by server to clients
// the rest of the structure is private to game
// wishes to leave the intermission
// level.time of last usercmd_t, for EF_CONNECTION
// we can't just use pers.lastCommand.time, because
// of the g_sycronousclients case
// sum up damage over an entire frame, so
// shotgun blasts give a single big kick
// damage absorbed by armor
// damage taken out of health
// impact damage
// origin for vector calculation
// if true, don't use the damage_from vector
// for "impressive" reward sound
// total number of shots
// total number of hits
//
// last client that this client killed
// last client that damaged this client
// type of damage the client did
// timers
// can respawn when time > this, force after g_forcerespwan
// kick players when time > this
// qtrue if the five seoond warning has been given
// clear the EF_AWARD_IMPRESSIVE, etc when time > this
// for multiple kill rewards
// used for hook
// grapple hook if out
// time the player switched teams
// timeResidual is used to handle events that happen every second
// like health / armor countdowns and regeneration
//
// this structure is cleared as each map is entered
//
// [maxclients]
// MAX_CLIENTS <= num_entities <= ENTITYNUM_MAX_NORMAL
// restart match at this time
// store latched cvars here that we want to get at often
// in msec
// so movers can back up when blocked
// level.time the map was started
// last time of client team location update
// don't use any old session data, because
// we changed gametype
// waiting for a map_restart to fire
// includes connecting clients
// connected, non-spectators
// sorted by score
// clientNums for auto-follow spectators
// sound index for standing in lava
// for detecting if g_warmup is changed
// voting state
// level.time vote was called
// time the vote is executed
// set by CalculateRanks
// team voting state
// level.time vote was called
// set by CalculateRanks
// spawn variables
// the G_Spawn*() functions are valid
// key / value pairs
// intermission state
// intermission was qualified, but
// wait INTERMISSION_DELAY_TIME before
// actually going there so the last
// frag can be watched.  Disable future
// kills during this delay
// time the intermission was started
// at least one client wants to exit
// also used for spectator spawns
// target_locations get linked
// head of the location list
// dead bodies
//
// g_spawn.c
//
// spawn string returns a temporary reference, you must CopyString() if you want to keep it
//
// g_cmds.c
//
//
// g_items.c
//
//
// g_utils.c
//
//
// g_combat.c
//
// damage flags
// damage was indirect
// armour does not protect from this damage
// do not affect velocity, just view angles
// armor, shields, invulnerability, and godmode have no effect
//
// g_missile.c
//
//
// g_mover.c
//
//
// g_trigger.c
//
//
// g_misc.c
//
//
// g_weapon.c
//
//
// g_client.c
//
//
// g_svcmds.c
//
//
// g_weapon.c
//
//
// g_cmds.c
//
//
// g_main.c
//
//
// g_client.c
//
//
// g_active.c
//
//
// g_team.c
//
//
// g_mem.c
//
//
// g_session.c
//
//
// g_arenas.c
//
/*
===============
Svcmd_AbortPodium_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_AbortPodium_f() {
    if crate::src::game::g_main::g_gametype.integer
        != crate::bg_public_h::GT_SINGLE_PLAYER as libc::c_int
    {
        return;
    }
    if !podium1.is_null() {
        (*podium1).nextthink = crate::src::game::g_main::level.time;
        (*podium1).think =
            Some(CelebrateStop as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ())
    };
}
