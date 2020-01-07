use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0) * *v.offset(0)
                + *v.offset(1) * *v.offset(1)
                + *v.offset(2) * *v.offset(2)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    use crate::stdlib::sqrt;

    // __Q_SHARED_H
}

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }
    use crate::stdlib::strtod;
}

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::ai_cmd::q_shared_h::VectorLength;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
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
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_ai_chat_h::bot_match_s;
pub use crate::be_ai_chat_h::bot_match_t;
pub use crate::be_ai_chat_h::bot_matchvariable_s;
pub use crate::be_ai_chat_h::bot_matchvariable_t;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::g_local_h::bot_settings_s;
pub use crate::g_local_h::bot_settings_t;
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
pub use crate::src::game::ai_cmd::stdlib_float_h::atof;
pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_BotInitialChat;
pub use crate::src::game::ai_main::BotAI_Print;
pub use crate::src::game::ai_main::BotAI_Trace;
pub use crate::src::game::ai_main::BotEntityInfo;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotFindMatch;
pub use crate::src::game::g_syscalls::trap_BotGetLevelItemGoal;
pub use crate::src::game::g_syscalls::trap_BotGoalName;
pub use crate::src::game::g_syscalls::trap_BotMatchVariable;
pub use crate::src::game::g_syscalls::trap_EA_Action;
pub use crate::src::game::g_syscalls::trap_EA_Command;
pub use crate::src::game::g_syscalls::trap_EA_SayTeam;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;
use crate::stdlib::memcpy;
use crate::stdlib::sqrt;
use crate::stdlib::sscanf;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncpy;

use crate::src::game::ai_dmq3::ctf_blueflag;
use crate::src::game::ai_dmq3::ctf_redflag;
use crate::src::game::ai_dmq3::gametype;
use crate::src::game::ai_dmq3::stristr;
use crate::src::game::ai_dmq3::BotCreateWayPoint;
use crate::src::game::ai_dmq3::BotFindWayPoint;
use crate::src::game::ai_dmq3::BotFreeWaypoints;
use crate::src::game::ai_dmq3::BotGetAlternateRouteGoal;
use crate::src::game::ai_dmq3::BotOppositeTeam;
use crate::src::game::ai_dmq3::BotPointAreaNum;
use crate::src::game::ai_dmq3::BotRememberLastOrderedTask;
use crate::src::game::ai_dmq3::BotSameTeam;
use crate::src::game::ai_dmq3::BotSetTeamStatus;
use crate::src::game::ai_dmq3::BotTeam;
use crate::src::game::ai_dmq3::ClientFromName;
use crate::src::game::ai_dmq3::ClientName;
use crate::src::game::ai_dmq3::ClientOnSameTeamFromName;
use crate::src::game::ai_dmq3::EasyClientName;
use crate::src::game::ai_dmq3::TeamPlayIsOn;
use crate::src::game::ai_team::BotGetTeamMateTaskPreference;
use crate::src::game::ai_team::BotSetTeamMateTaskPreference;
use crate::src::game::ai_team::BotVoiceChat;
use crate::src::game::ai_team::BotVoiceChatOnly;
use crate::stdlib::rand;
use crate::stdlib::strtod;
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
/* ****************************************************************************
 * name:		ai_cmd.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_cmd.c $
 *
 *****************************************************************************/
//
//
// for the voice chats
#[no_mangle]

pub static mut notleader: [i32; 64] = [0; 64];
//DEBUG
/*
==================
BotGetItemTeamGoal

FIXME: add stuff like "upper rocket launcher"
"the rl near the railgun", "lower grenade launcher" etc.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetItemTeamGoal(
    mut goalname: *mut i8,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut i: i32 = 0;
    if crate::stdlib::strlen(goalname) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    i = -(1);
    loop {
        i = crate::src::game::g_syscalls::trap_BotGetLevelItemGoal(
            i,
            goalname,
            goal as *mut libc::c_void,
        );
        if i > 0 {
            //do NOT defend dropped items
            if !((*goal).flags & 4 != 0) {
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        }
        if !(i > 0) {
            break;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotGetMessageTeamGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetMessageTeamGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut goalname: *mut i8,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut cp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    if BotGetItemTeamGoal(goalname, goal) != 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    cp = crate::src::game::ai_dmq3::BotFindWayPoint((*bs).checkpoints, goalname);
    if !cp.is_null() {
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*cp).goal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
        );
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotGetTime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetTime(mut match_0: *mut crate::be_ai_chat_h::bot_match_t) -> f32 {
    let mut timematch: crate::be_ai_chat_h::bot_match_t = crate::be_ai_chat_h::bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [crate::be_ai_chat_h::bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut timestring: [i8; 256] = [0; 256];
    let mut t: f32 = 0.;
    //if the matched string has a time
    if (*match_0).subtype & 16 != 0 {
        //get the time string
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            6,
            timestring.as_mut_ptr(),
            256,
        );
        //match it to find out if the time is in seconds or minutes
        if crate::src::game::g_syscalls::trap_BotFindMatch(
            timestring.as_mut_ptr(),
            &mut timematch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
            8,
        ) != 0
        {
            if timematch.type_0 == 107 {
                t = 99999999.0
            } else if timematch.type_0 == 109 {
                t = (10i32 * 60) as f32
            // 10 minutes
            } else if timematch.type_0 == 108 {
                t = (30i32 * 60) as f32
            // 30 minutes
            } else {
                crate::src::game::g_syscalls::trap_BotMatchVariable(
                    &mut timematch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
                    6,
                    timestring.as_mut_ptr(),
                    256,
                );
                if timematch.type_0 == 105 {
                    t = (atof(timestring.as_mut_ptr()) * 60f64) as f32
                } else if timematch.type_0 == 106 {
                    t = atof(timestring.as_mut_ptr()) as f32
                } else {
                    t = 0f32
                }
            }
            //if there's a valid time
            if t > 0f32 {
                return crate::src::game::ai_main::floattime + t;
            }
        }
    }
    return 0f32;
}
/*
==================
FindClientByName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn FindClientByName(mut name: *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::ai_dmq3::ClientName(
            i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        if crate::src::qcommon::q_shared::Q_stricmp(buf.as_mut_ptr(), name) == 0 {
            return i;
        }
        i += 1
    }
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::ai_dmq3::ClientName(
            i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        if !crate::src::game::ai_dmq3::stristr(buf.as_mut_ptr(), name).is_null() {
            return i;
        }
        i += 1
    }
    return -(1);
}
/*
==================
FindEnemyByName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn FindEnemyByName(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut name: *mut i8,
) -> i32 {
    let mut i: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        if !(crate::src::game::ai_dmq3::BotSameTeam(bs, i) != 0) {
            crate::src::game::ai_dmq3::ClientName(
                i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 1024]>() as i32,
            );
            if crate::src::qcommon::q_shared::Q_stricmp(buf.as_mut_ptr(), name) == 0 {
                return i;
            }
        }
        i += 1
    }
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        if !(crate::src::game::ai_dmq3::BotSameTeam(bs, i) != 0) {
            crate::src::game::ai_dmq3::ClientName(
                i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 1024]>() as i32,
            );
            if !crate::src::game::ai_dmq3::stristr(buf.as_mut_ptr(), name).is_null() {
                return i;
            }
        }
        i += 1
    }
    return -(1);
}
/*
==================
NumPlayersOnSameTeam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn NumPlayersOnSameTeam(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut num: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    num = 0;
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 + 256 + 256 + i,
            buf.as_mut_ptr(),
            1024,
        );
        if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
            if crate::src::game::ai_dmq3::BotSameTeam(bs, i + 1) != 0 {
                num += 1
            }
        }
        i += 1
    }
    return num;
}
/*
==================
TeamPlayIsOn
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetPatrolWaypoints(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) -> i32 {
    let mut keyarea: [i8; 256] = [0; 256];
    let mut patrolflags: i32 = 0;
    let mut wp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    let mut newwp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    let mut newpatrolpoints: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    let mut keyareamatch: crate::be_ai_chat_h::bot_match_t = crate::be_ai_chat_h::bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [crate::be_ai_chat_h::bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    newpatrolpoints = 0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    patrolflags = 0;
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5,
        keyarea.as_mut_ptr(),
        256,
    );
    loop
    //
    {
        if crate::src::game::g_syscalls::trap_BotFindMatch(
            keyarea.as_mut_ptr(),
            &mut keyareamatch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
            64,
        ) == 0
        {
            crate::src::game::g_syscalls::trap_EA_SayTeam(
                (*bs).client,
                b"what do you say?\x00" as *const u8 as *mut i8,
            );
            crate::src::game::ai_dmq3::BotFreeWaypoints(newpatrolpoints);
            (*bs).patrolpoints = 0 as *mut crate::src::game::ai_main::bot_waypoint_t;
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            &mut keyareamatch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
            5,
            keyarea.as_mut_ptr(),
            256,
        );
        if BotGetMessageTeamGoal(bs, keyarea.as_mut_ptr(), &mut goal) == 0 {
            //BotAI_BotInitialChat(bs, "cannotfind", keyarea, NULL);
            //trap_BotEnterChat(bs->cs, 0, CHAT_TEAM);
            crate::src::game::ai_dmq3::BotFreeWaypoints(newpatrolpoints);
            (*bs).patrolpoints = 0 as *mut crate::src::game::ai_main::bot_waypoint_t;
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //create a new waypoint
        newwp = crate::src::game::ai_dmq3::BotCreateWayPoint(
            keyarea.as_mut_ptr(),
            goal.origin.as_mut_ptr(),
            goal.areanum,
        );
        if newwp.is_null() {
            break;
        }
        //add the waypoint to the patrol points
        (*newwp).next = 0 as *mut crate::src::game::ai_main::bot_waypoint_s;
        wp = newpatrolpoints;
        while !wp.is_null() && !(*wp).next.is_null() {
            wp = (*wp).next
        }
        if wp.is_null() {
            newpatrolpoints = newwp;
            (*newwp).prev = 0 as *mut crate::src::game::ai_main::bot_waypoint_s
        } else {
            (*wp).next = newwp;
            (*newwp).prev = wp
        }
        //
        if keyareamatch.subtype & 512 != 0 {
            patrolflags = 1;
            break;
        } else if keyareamatch.subtype & 1024 != 0 {
            patrolflags = 2;
            break;
        } else {
            if !(keyareamatch.subtype & 256 != 0) {
                break;
            }
            crate::src::game::g_syscalls::trap_BotMatchVariable(
                &mut keyareamatch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
                6i32,
                keyarea.as_mut_ptr(),
                256i32,
            );
        }
    }
    //
    if newpatrolpoints.is_null() || (*newpatrolpoints).next.is_null() {
        crate::src::game::g_syscalls::trap_EA_SayTeam(
            (*bs).client,
            b"I need more key points to patrol\n\x00" as *const u8 as *mut i8,
        );
        crate::src::game::ai_dmq3::BotFreeWaypoints(newpatrolpoints);
        newpatrolpoints = 0 as *mut crate::src::game::ai_main::bot_waypoint_t;
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    crate::src::game::ai_dmq3::BotFreeWaypoints((*bs).patrolpoints);
    (*bs).patrolpoints = newpatrolpoints;
    //
    (*bs).curpatrolpoint = (*bs).patrolpoints;
    (*bs).patrolflags = patrolflags;
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotAddressedToBot
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAddressedToBot(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) -> i32 {
    let mut addressedto: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut name: [i8; 256] = [0; 256];
    let mut botname: [i8; 128] = [0; 128];
    let mut client: i32 = 0;
    let mut addresseematch: crate::be_ai_chat_h::bot_match_t = crate::be_ai_chat_h::bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [crate::be_ai_chat_h::bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    client = crate::src::game::ai_dmq3::ClientOnSameTeamFromName(bs, netname.as_mut_ptr());
    if client < 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the message is addressed to someone
    if (*match_0).subtype & 2 != 0 {
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            2,
            addressedto.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        //the name of this bot
        crate::src::game::ai_dmq3::ClientName((*bs).client, botname.as_mut_ptr(), 128);
        //
        while crate::src::game::g_syscalls::trap_BotFindMatch(
            addressedto.as_mut_ptr(),
            &mut addresseematch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
            32,
        ) != 0
        {
            if addresseematch.type_0 == 101 {
                return crate::src::qcommon::q_shared::qtrue as i32;
            } else if addresseematch.type_0 == 102 {
                crate::src::game::g_syscalls::trap_BotMatchVariable(
                    &mut addresseematch as *mut crate::be_ai_chat_h::bot_match_t
                        as *mut libc::c_void,
                    4,
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 256]>() as i32,
                );
                if crate::stdlib::strlen(name.as_mut_ptr()) != 0 {
                    if !crate::src::game::ai_dmq3::stristr(botname.as_mut_ptr(), name.as_mut_ptr())
                        .is_null()
                    {
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                    if !crate::src::game::ai_dmq3::stristr(
                        (*bs).subteam.as_mut_ptr(),
                        name.as_mut_ptr(),
                    )
                    .is_null()
                    {
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                }
                crate::src::game::g_syscalls::trap_BotMatchVariable(
                    &mut addresseematch as *mut crate::be_ai_chat_h::bot_match_t
                        as *mut libc::c_void,
                    6i32,
                    addressedto.as_mut_ptr(),
                    256i32,
                );
            } else {
                crate::src::game::g_syscalls::trap_BotMatchVariable(
                    &mut addresseematch as *mut crate::be_ai_chat_h::bot_match_t
                        as *mut libc::c_void,
                    4,
                    name.as_mut_ptr(),
                    256,
                );
                if crate::stdlib::strlen(name.as_mut_ptr()) != 0 {
                    if !crate::src::game::ai_dmq3::stristr(botname.as_mut_ptr(), name.as_mut_ptr())
                        .is_null()
                    {
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                    if !crate::src::game::ai_dmq3::stristr(
                        (*bs).subteam.as_mut_ptr(),
                        name.as_mut_ptr(),
                    )
                    .is_null()
                    {
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                }
                break;
            }
        }
        //Com_sprintf(buf, sizeof(buf), "not addressed to me but %s", addressedto);
        //trap_EA_Say(bs->client, buf);
        return crate::src::qcommon::q_shared::qfalse as i32;
    } else {
        let mut tellmatch: crate::be_ai_chat_h::bot_match_t = crate::be_ai_chat_h::bot_match_t {
            string: [0; 256],
            type_0: 0,
            subtype: 0,
            variables: [crate::be_ai_chat_h::bot_matchvariable_t {
                offset: 0,
                length: 0,
            }; 8],
        };
        tellmatch.type_0 = 0;
        //if this message wasn't directed solely to this bot
        if crate::src::game::g_syscalls::trap_BotFindMatch(
            (*match_0).string.as_mut_ptr(),
            &mut tellmatch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
            128,
        ) == 0
            || tellmatch.type_0 != 202
        {
            //make sure not everyone reacts to this message
            if (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32
                > 1f32 / (NumPlayersOnSameTeam(bs) - 1) as f32
            {
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        }
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotGPSToPosition
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGPSToPosition(
    mut buf: *mut i8,
    mut position: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut num: i32 = 0;
    let mut sign: i32 = 0;
    i = 0;
    while i < 3 {
        num = 0;
        while *buf.offset(j as isize) as i32 == ' ' as i32 {
            j += 1
        }
        if *buf.offset(j as isize) as i32 == '-' as i32 {
            j += 1;
            sign = -(1)
        } else {
            sign = 1
        }
        while *buf.offset(j as isize) != 0 {
            if *buf.offset(j as isize) as i32 >= '0' as i32
                && *buf.offset(j as isize) as i32 <= '9' as i32
            {
                num = num * 10 + *buf.offset(j as isize) as i32 - '0' as i32;
                j += 1
            } else {
                j += 1;
                break;
            }
        }
        crate::src::game::ai_main::BotAI_Print(1, b"%d\n\x00" as *const u8 as *mut i8, sign * num);
        *position.offset(i as isize) = sign as f32 * num as f32;
        i += 1
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotMatch_HelpAccompany
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_HelpAccompany(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut client: i32 = 0;
    let mut other: i32 = 0;
    let mut areanum: i32 = 0;
    let mut teammate: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut itemname: [i8; 256] = [0; 256];
    let mut teammatematch: crate::be_ai_chat_h::bot_match_t = crate::be_ai_chat_h::bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [crate::be_ai_chat_h::bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the team mate name
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //get the client to help
    if crate::src::game::g_syscalls::trap_BotFindMatch(
        teammate.as_mut_ptr(),
        &mut teammatematch as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
        16,
    ) != 0
        && teammatematch.type_0 == 100
    {
        //get the netname
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        other = crate::src::qcommon::q_shared::qfalse as i32
    } else {
        //asked for someone else
        client = FindClientByName(teammate.as_mut_ptr());
        //if this is the bot self
        if client == (*bs).client {
            other = crate::src::qcommon::q_shared::qfalse as i32
        } else if crate::src::game::ai_dmq3::BotSameTeam(bs, client) == 0 {
            //FIXME: say "I don't help the enemy"
            return;
        } else {
            other = crate::src::qcommon::q_shared::qtrue as i32
        }
    }
    //if the bot doesn't know who to help (FindClientByName returned -1)
    if client < 0 {
        if other != 0 {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"whois\x00" as *const u8 as *mut i8,
                teammate.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"whois\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2);
        return;
    }
    //don't help or accompany yourself
    if client == (*bs).client {
        return;
    }
    //
    (*bs).teamgoal.entitynum = -(1);
    crate::src::game::ai_main::BotEntityInfo(client, &mut entinfo);
    //if info is valid (in PVS)
    if entinfo.valid != 0 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if areanum != 0 {
            // && trap_AAS_AreaReachability(areanum)) {
            (*bs).teamgoal.entitynum = client;
            (*bs).teamgoal.areanum = areanum;
            (*bs).teamgoal.origin[0] = entinfo.origin[0];
            (*bs).teamgoal.origin[1] = entinfo.origin[1];
            (*bs).teamgoal.origin[2] = entinfo.origin[2];
            (*bs).teamgoal.mins[0] = -8f32;
            (*bs).teamgoal.mins[1] = -8f32;
            (*bs).teamgoal.mins[2] = -8f32;
            (*bs).teamgoal.maxs[0] = 8f32;
            (*bs).teamgoal.maxs[1] = 8f32;
            (*bs).teamgoal.maxs[2] = 8f32
        }
    }
    //if no teamgoal yet
    if (*bs).teamgoal.entitynum < 0 {
        //if near an item
        if (*match_0).subtype & 1 != 0 {
            //get the match variable
            crate::src::game::g_syscalls::trap_BotMatchVariable(
                match_0 as *mut libc::c_void,
                3,
                itemname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            //
            if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
                //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
                //trap_BotEnterChat(bs->cs, bs->client, CHAT_TEAM);
                return;
            }
        }
    }
    //
    if (*bs).teamgoal.entitynum < 0 {
        if other != 0 {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"whereis\x00" as *const u8 as *mut i8,
                teammate.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"whereareyou\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 1);
        return;
    }
    //the team mate
    (*bs).teammate = client;
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //the team mate who ordered
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //last time the team mate was assumed visible
    (*bs).teammatevisible_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the ltg type
    if (*match_0).type_0 == 3 {
        (*bs).ltgtype = 1; //3.5 meter
        if (*bs).teamgoal_time == 0. {
            (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 60f32
        }
    } else {
        (*bs).ltgtype = 2;
        if (*bs).teamgoal_time == 0. {
            (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32
        }
        (*bs).formation_dist = (3.5f64 * 32f64) as f32;
        (*bs).arrive_time = 0f32;
        //
        crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
        // remember last ordered task
        crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs);
    };
    //DEBUG
}
/*
==================
BotMatch_DefendKeyArea
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_DefendKeyArea(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut itemname: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the match variable
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
        //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
        //trap_BotEnterChat(bs->cs, bs->client, CHAT_TEAM);
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //the team mate who ordered
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 3;
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the team goal time
    if (*bs).teamgoal_time == 0. {
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32
    }
    //away from defending
    (*bs).defendaway_time = 0f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs);
    //DEBUG
}
/*
==================
BotMatch_GetItem
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_GetItem(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut itemname: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the match variable
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        3,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
        //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
        //trap_BotEnterChat(bs->cs, bs->client, CHAT_TEAM);
        return;
    }
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    client = crate::src::game::ai_dmq3::ClientOnSameTeamFromName(bs, netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 10;
    //set the team goal time
    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 60f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    //DEBUG
}
/*
==================
BotMatch_Camp
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Camp(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut client: i32 = 0;
    let mut areanum: i32 = 0;
    let mut netname: [i8; 256] = [0; 256];
    let mut itemname: [i8; 256] = [0; 256];
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //asked for someone else
    client = FindClientByName(netname.as_mut_ptr());
    //if there's no valid client with this name
    if client < 0 {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs,
            b"whois\x00" as *const u8 as *mut i8,
            netname.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).client, 1);
        return;
    }
    //get the match variable
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //in CTF it could be the base
    if (*match_0).subtype & 64 != 0 {
        //camp at the spot the bot is currently standing
        (*bs).teamgoal.entitynum = (*bs).entitynum;
        (*bs).teamgoal.areanum = (*bs).areanum;
        (*bs).teamgoal.origin[0] = (*bs).origin[0];
        (*bs).teamgoal.origin[1] = (*bs).origin[1];
        (*bs).teamgoal.origin[2] = (*bs).origin[2];
        (*bs).teamgoal.mins[0] = -8f32;
        (*bs).teamgoal.mins[1] = -8f32;
        (*bs).teamgoal.mins[2] = -8f32;
        (*bs).teamgoal.maxs[0] = 8f32;
        (*bs).teamgoal.maxs[1] = 8f32;
        (*bs).teamgoal.maxs[2] = 8f32
    } else if (*match_0).subtype & 32 != 0 {
        //if this is the bot self
        if client == (*bs).client {
            return;
        }
        //
        (*bs).teamgoal.entitynum = -(1);
        crate::src::game::ai_main::BotEntityInfo(client, &mut entinfo);
        //if info is valid (in PVS)
        if entinfo.valid != 0 {
            areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if areanum != 0 {
                // && trap_AAS_AreaReachability(areanum)) {
                //NOTE: just assume the bot knows where the person is
                //if (BotEntityVisible(bs->entitynum, bs->eye, bs->viewangles, 360, client)) {
                (*bs).teamgoal.entitynum = client;
                (*bs).teamgoal.areanum = areanum;
                (*bs).teamgoal.origin[0] = entinfo.origin[0];
                (*bs).teamgoal.origin[1] = entinfo.origin[1];
                (*bs).teamgoal.origin[2] = entinfo.origin[2];
                (*bs).teamgoal.mins[0] = -8f32;
                (*bs).teamgoal.mins[1] = -8f32;
                (*bs).teamgoal.mins[2] = -8f32;
                (*bs).teamgoal.maxs[0] = 8f32;
                (*bs).teamgoal.maxs[1] = 8f32;
                (*bs).teamgoal.maxs[2] = 8f32
                //}
            }
        }
        //if the other is not visible
        if (*bs).teamgoal.entitynum < 0 {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"whereareyou\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2);
            return;
        }
    } else if BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) == 0 {
        //BotAI_BotInitialChat(bs, "cannotfind", itemname, NULL);
        //client = ClientFromName(netname);
        //trap_BotEnterChat(bs->cs, client, CHAT_TELL);
        return;
    }
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 8;
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the team goal time
    if (*bs).teamgoal_time == 0. {
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32
    }
    //not arrived yet
    (*bs).arrive_time = 0f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs);
    //DEBUG
}
/*
==================
BotMatch_Patrol
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Patrol(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the patrol waypoints
    if BotGetPatrolWaypoints(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 9;
    //get the team goal time
    (*bs).teamgoal_time = BotGetTime(match_0);
    //set the team goal time if not set already
    if (*bs).teamgoal_time == 0. {
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32
    }
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs);
    //DEBUG
}
/*
==================
BotMatch_GetFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_GetFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        if crate::src::game::ai_dmq3::ctf_redflag.areanum == 0
            || crate::src::game::ai_dmq3::ctf_blueflag.areanum == 0
        {
            return;
        }
    } else {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 4;
    //set the team goal time
    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32;
    // get an alternate route in ctf
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        //get an alternative route goal towards the enemy base
        crate::src::game::ai_dmq3::BotGetAlternateRouteGoal(
            bs,
            crate::src::game::ai_dmq3::BotOppositeTeam(bs),
        );
    }
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs);
    //DEBUG
}
/*
==================
BotMatch_AttackEnemyBase
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_AttackEnemyBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        BotMatch_GetFlag(bs, match_0);
    } else {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 13;
    //set the team goal time
    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32;
    (*bs).attackaway_time = 0f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    // remember last ordered task
    crate::src::game::ai_dmq3::BotRememberLastOrderedTask(bs);
    //DEBUG
}
/*
==================
BotMatch_RushBase
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_RushBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        if crate::src::game::ai_dmq3::ctf_redflag.areanum == 0
            || crate::src::game::ai_dmq3::ctf_blueflag.areanum == 0
        {
            return;
        }
    } else {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 5;
    //set the team goal time
    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 120f32;
    (*bs).rushbaseaway_time = 0f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    //DEBUG
}
/*
==================
BotMatch_TaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_TaskPreference(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 36] = [0; 36];
    let mut teammatename: [i8; 256] = [0; 256];
    let mut teammate: i32 = 0;
    let mut preference: i32 = 0;
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        != 0
    {
        return;
    }
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    teammate = crate::src::game::ai_dmq3::ClientFromName(teammatename.as_mut_ptr());
    if teammate < 0 {
        return;
    }
    preference = crate::src::game::ai_team::BotGetTeamMateTaskPreference(bs, teammate);
    match (*match_0).subtype {
        1 => {
            preference &= !(2);
            preference |= 1
        }
        2 => {
            preference &= !(1);
            preference |= 2
        }
        4 => preference &= !(2 | 1),
        _ => {}
    }
    crate::src::game::ai_team::BotSetTeamMateTaskPreference(bs, teammate, preference);
    //
    crate::src::game::ai_dmq3::EasyClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    crate::src::game::ai_main::BotAI_BotInitialChat(
        bs,
        b"keepinmind\x00" as *const u8 as *mut i8,
        teammatename.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, teammate, 2);
    crate::src::game::ai_team::BotVoiceChatOnly(bs, teammate, b"yes\x00" as *const u8 as *mut i8);
    crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000);
}
/*
==================
BotMatch_ReturnFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_ReturnFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    //if not in CTF mode
    if crate::src::game::ai_dmq3::gametype != crate::bg_public_h::GT_CTF as i32 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = FindClientByName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).order_time = crate::src::game::ai_main::floattime;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 6;
    //set the team goal time
    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 180f32;
    (*bs).rushbaseaway_time = 0f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    //DEBUG
}
/*
==================
BotMatch_JoinSubteam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_JoinSubteam(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut teammate: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //get the sub team name
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //set the sub team name
    crate::stdlib::strncpy((*bs).subteam.as_mut_ptr(), teammate.as_mut_ptr(), 32);
    (*bs).subteam[31] = '\u{0}' as i8;
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    crate::src::game::ai_main::BotAI_BotInitialChat(
        bs,
        b"joinedteam\x00" as *const u8 as *mut i8,
        teammate.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2);
}
/*
==================
BotMatch_LeaveSubteam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_LeaveSubteam(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    if crate::stdlib::strlen((*bs).subteam.as_mut_ptr()) != 0 {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs,
            b"leftteam\x00" as *const u8 as *mut i8,
            (*bs).subteam.as_mut_ptr(),
            0 as *mut libc::c_void,
        ); //end if
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2i32);
    }
    crate::stdlib::strcpy(
        (*bs).subteam.as_mut_ptr(),
        b"\x00" as *const u8 as *const i8,
    );
}
/*
==================
BotMatch_LeaveSubteam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhichTeam(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    if crate::stdlib::strlen((*bs).subteam.as_mut_ptr()) != 0 {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs,
            b"inteam\x00" as *const u8 as *mut i8,
            (*bs).subteam.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
    } else {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs,
            b"noteam\x00" as *const u8 as *mut i8,
            0 as *mut libc::c_void,
        );
    }
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).client, 1);
}
/*
==================
BotMatch_CheckPoint
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_CheckPoint(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut areanum: i32 = 0;
    let mut client: i32 = 0;
    let mut buf: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut position: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5,
        buf.as_mut_ptr(),
        256,
    );
    position[2] = 0f32;
    position[1] = position[2];
    position[0] = position[1];
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //BotGPSToPosition(buf, position);
    crate::stdlib::sscanf(
        buf.as_mut_ptr(),
        b"%f %f %f\x00" as *const u8 as *const i8,
        &mut *position.as_mut_ptr().offset(0isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *position.as_mut_ptr().offset(1isize) as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *position.as_mut_ptr().offset(2isize) as *mut crate::src::qcommon::q_shared::vec_t,
    );
    position[2] = (position[2] as f64 + 0.5) as crate::src::qcommon::q_shared::vec_t;
    areanum = crate::src::game::ai_dmq3::BotPointAreaNum(position.as_mut_ptr());
    if areanum == 0 {
        if BotAddressedToBot(bs, match_0) != 0 {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"checkpoint_invalid\x00" as *const u8 as *mut i8,
                0 as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2i32);
        }
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        6,
        buf.as_mut_ptr(),
        256,
    );
    //check if there already exists a checkpoint with this name
    cp = crate::src::game::ai_dmq3::BotFindWayPoint((*bs).checkpoints, buf.as_mut_ptr());
    if !cp.is_null() {
        if !(*cp).next.is_null() {
            (*(*cp).next).prev = (*cp).prev
        }
        if !(*cp).prev.is_null() {
            (*(*cp).prev).next = (*cp).next
        } else {
            (*bs).checkpoints = (*cp).next
        }
        (*cp).inuse = crate::src::qcommon::q_shared::qfalse as i32
    }
    //create a new check point
    cp = crate::src::game::ai_dmq3::BotCreateWayPoint(
        buf.as_mut_ptr(),
        position.as_mut_ptr(),
        areanum,
    );
    //add the check point to the bot's known chech points
    (*cp).next = (*bs).checkpoints;
    if !(*bs).checkpoints.is_null() {
        (*(*bs).checkpoints).prev = cp
    }
    (*bs).checkpoints = cp;
    //
    if BotAddressedToBot(bs, match_0) != 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
            b"%1.0f %1.0f %1.0f\x00" as *const u8 as *const i8,
            (*cp).goal.origin[0usize] as f64,
            (*cp).goal.origin[1usize] as f64,
            (*cp).goal.origin[2usize] as f64,
        );
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs,
            b"checkpoint_confirm\x00" as *const u8 as *mut i8,
            (*cp).name.as_mut_ptr(),
            buf.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2i32);
    };
}
/*
==================
BotMatch_FormationSpace
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_FormationSpace(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut buf: [i8; 256] = [0; 256];
    let mut space: f32 = 0.;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5,
        buf.as_mut_ptr(),
        256,
    );
    //if it's the distance in feet
    if (*match_0).subtype & 8 != 0 {
        space = (0.3048 * 32f64 * atof(buf.as_mut_ptr())) as f32
    } else {
        //else it's in meters
        space = (32f64 * atof(buf.as_mut_ptr())) as f32
    }
    //check if the formation intervening space is valid
    if space < 48f32 || space > 500f32 {
        space = 100f32
    }
    (*bs).formation_dist = space;
}
/*
==================
BotMatch_Dismiss
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Dismiss(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //
    (*bs).decisionmaker = client;
    //
    (*bs).ltgtype = 0;
    (*bs).lead_time = 0f32;
    (*bs).lastgoal_ltgtype = 0;
    //
    crate::src::game::ai_main::BotAI_BotInitialChat(
        bs,
        b"dismissed\x00" as *const u8 as *mut i8,
        0 as *mut libc::c_void,
    );
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2);
}
/*
==================
BotMatch_Suicide
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Suicide(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    crate::src::game::g_syscalls::trap_EA_Command(
        (*bs).client,
        b"kill\x00" as *const u8 as *mut i8,
    );
    //
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    //
    crate::src::game::ai_team::BotVoiceChat(bs, client, b"taunt\x00" as *const u8 as *mut i8);
    crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x100000);
}
/*
==================
BotMatch_StartTeamLeaderShip
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_StartTeamLeaderShip(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut client: i32 = 0;
    let mut teammate: [i8; 256] = [0; 256];
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if chats for him or herself
    if (*match_0).subtype & 128 != 0 {
        //get the team mate that will be the team leader
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        crate::stdlib::strncpy(
            (*bs).teamleader.as_mut_ptr(),
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 36]>(),
        );
        (*bs).teamleader[(::std::mem::size_of::<[i8; 36]>()).wrapping_sub(1usize)] = '\u{0}' as i8
    } else {
        //chats for someone else
        //get the team mate that will be the team leader
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            4,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        client = FindClientByName(teammate.as_mut_ptr());
        if client >= 0 {
            crate::src::game::ai_dmq3::ClientName(
                client,
                (*bs).teamleader.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 36]>() as i32,
            );
        }
    };
}
/*
==================
BotMatch_StopTeamLeaderShip
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_StopTeamLeaderShip(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut client: i32 = 0;
    let mut teammate: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //get the team mate that stops being the team leader
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //if chats for him or herself
    if (*match_0).subtype & 128 != 0 {
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        ); //end else
        client = FindClientByName(netname.as_mut_ptr())
    } else {
        //chats for someone else
        client = FindClientByName(teammate.as_mut_ptr())
    }
    if client >= 0 {
        if crate::src::qcommon::q_shared::Q_stricmp(
            (*bs).teamleader.as_mut_ptr(),
            crate::src::game::ai_dmq3::ClientName(
                client,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            ),
        ) == 0
        {
            (*bs).teamleader[0] = '\u{0}' as i8;
            notleader[client as usize] = crate::src::qcommon::q_shared::qtrue as i32
        }
    };
}
/*
==================
BotMatch_WhoIsTeamLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhoIsTeamLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //if this bot IS the team leader
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        == 0
    {
        crate::src::game::g_syscalls::trap_EA_SayTeam(
            (*bs).client,
            b"I\'m the team leader\n\x00" as *const u8 as *mut i8,
        );
    };
}
/*
==================
BotMatch_WhatAreYouDoing
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhatAreYouDoing(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 256] = [0; 256];
    let mut goalname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //
    match (*bs).ltgtype {
        1 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"helping\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        2 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"accompanying\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        3 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"defending\x00" as *const u8 as *mut i8,
                goalname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        10 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"gettingitem\x00" as *const u8 as *mut i8,
                goalname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        11 => {
            crate::src::game::ai_dmq3::ClientName(
                (*bs).teamgoal.entitynum,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"killing\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        7 | 8 => {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"camping\x00" as *const u8 as *mut i8,
                0 as *mut libc::c_void,
            );
        }
        9 => {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"patrolling\x00" as *const u8 as *mut i8,
                0 as *mut libc::c_void,
            );
        }
        4 => {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"capturingflag\x00" as *const u8 as *mut i8,
                0 as *mut libc::c_void,
            );
        }
        5 => {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"rushingbase\x00" as *const u8 as *mut i8,
                0 as *mut libc::c_void,
            );
        }
        6 => {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"returningflag\x00" as *const u8 as *mut i8,
                0 as *mut libc::c_void,
            );
        }
        _ => {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"roaming\x00" as *const u8 as *mut i8,
                0 as *mut libc::c_void,
            );
        }
    }
    //chat what the bot is doing
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
    crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2);
}
/*
==================
BotMatch_WhatIsMyCommand
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhatIsMyCommand(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut netname: [i8; 36] = [0; 36];
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        != 0
    {
        return;
    }
    (*bs).forceorders = crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotNearestVisibleItem
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotNearestVisibleItem(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut itemname: *mut i8,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> f32 {
    let mut i: i32 = 0;
    let mut name: [i8; 64] = [0; 64];
    let mut tmpgoal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    bestdist = 999999f32;
    i = -(1);
    loop {
        i = crate::src::game::g_syscalls::trap_BotGetLevelItemGoal(
            i,
            itemname,
            &mut tmpgoal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotGoalName(
            tmpgoal.number,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        if !(crate::src::qcommon::q_shared::Q_stricmp(itemname, name.as_mut_ptr()) != 0) {
            dir[0] = tmpgoal.origin[0] - (*bs).origin[0];
            dir[1] = tmpgoal.origin[1] - (*bs).origin[1];
            dir[2] = tmpgoal.origin[2] - (*bs).origin[2];
            dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
            if dist < bestdist {
                //trace from start to end
                crate::src::game::ai_main::BotAI_Trace(
                    &mut trace,
                    (*bs).eye.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    tmpgoal.origin.as_mut_ptr(),
                    (*bs).client,
                    1 | 0x10000,
                );
                if trace.fraction as f64 >= 1.0 {
                    bestdist = dist;
                    crate::stdlib::memcpy(
                        goal as *mut libc::c_void,
                        &mut tmpgoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
                    );
                }
            }
        }
        if !(i > 0) {
            break;
        }
    }
    return bestdist;
}
/*
==================
BotMatch_WhereAreYou
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_WhereAreYou(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut dist: f32 = 0.;
    let mut bestdist: f32 = 0.;
    let mut i: i32 = 0;
    let mut bestitem: i32 = 0;
    let mut redtt: i32 = 0;
    let mut bluett: i32 = 0;
    let mut client: i32 = 0;
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut netname: [i8; 256] = [0; 256];
    let mut nearbyitems: [*mut i8; 18] = [
        b"Shotgun\x00" as *const u8 as *mut i8,
        b"Grenade Launcher\x00" as *const u8 as *mut i8,
        b"Rocket Launcher\x00" as *const u8 as *mut i8,
        b"Plasmagun\x00" as *const u8 as *mut i8,
        b"Railgun\x00" as *const u8 as *mut i8,
        b"Lightning Gun\x00" as *const u8 as *mut i8,
        b"BFG10K\x00" as *const u8 as *mut i8,
        b"Quad Damage\x00" as *const u8 as *mut i8,
        b"Regeneration\x00" as *const u8 as *mut i8,
        b"Battle Suit\x00" as *const u8 as *mut i8,
        b"Speed\x00" as *const u8 as *mut i8,
        b"Invisibility\x00" as *const u8 as *mut i8,
        b"Flight\x00" as *const u8 as *mut i8,
        b"Armor\x00" as *const u8 as *mut i8,
        b"Heavy Armor\x00" as *const u8 as *mut i8,
        b"Red Flag\x00" as *const u8 as *mut i8,
        b"Blue Flag\x00" as *const u8 as *mut i8,
        0 as *mut i8,
    ];
    //
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    bestitem = -(1);
    bestdist = 999999f32;
    i = 0;
    while !nearbyitems[i as usize].is_null() {
        dist = BotNearestVisibleItem(bs, nearbyitems[i as usize], &mut goal);
        if dist < bestdist {
            bestdist = dist;
            bestitem = i
        }
        i += 1
    }
    if bestitem != -(1) {
        if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
            redtt = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
                (*bs).areanum,
                (*bs).origin.as_mut_ptr(),
                crate::src::game::ai_dmq3::ctf_redflag.areanum,
                0x2 | 0x4
                    | 0x8
                    | 0x10
                    | 0x20
                    | 0x80
                    | 0x100
                    | 0x200
                    | 0x400
                    | 0x800
                    | 0x80000
                    | 0x100000
                    | 0x40000
                    | 0x1000000,
            );
            bluett = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
                (*bs).areanum,
                (*bs).origin.as_mut_ptr(),
                crate::src::game::ai_dmq3::ctf_blueflag.areanum,
                0x2 | 0x4
                    | 0x8
                    | 0x10
                    | 0x20
                    | 0x80
                    | 0x100
                    | 0x200
                    | 0x400
                    | 0x800
                    | 0x80000
                    | 0x100000
                    | 0x40000
                    | 0x1000000,
            );
            if (redtt as f64) < (redtt + bluett) as f64 * 0.4 {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"teamlocation\x00" as *const u8 as *mut i8,
                    nearbyitems[bestitem as usize],
                    b"red\x00" as *const u8 as *const i8,
                    0 as *mut libc::c_void,
                );
            } else if (bluett as f64) < (redtt + bluett) as f64 * 0.4 {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"teamlocation\x00" as *const u8 as *mut i8,
                    nearbyitems[bestitem as usize],
                    b"blue\x00" as *const u8 as *const i8,
                    0 as *mut libc::c_void,
                );
            } else {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"location\x00" as *const u8 as *mut i8,
                    nearbyitems[bestitem as usize],
                    0 as *mut libc::c_void,
                );
            }
        } else {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"location\x00" as *const u8 as *mut i8,
                nearbyitems[bestitem as usize],
                0 as *mut libc::c_void,
            );
        }
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2i32);
    };
}
/*
==================
BotMatch_LeadTheWay
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_LeadTheWay(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut netname: [i8; 256] = [0; 256];
    let mut teammate: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    let mut areanum: i32 = 0;
    let mut other: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    //if someone asks for someone else
    if (*match_0).subtype & 2048 != 0 {
        //get the team mate name
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            4,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        client = FindClientByName(teammate.as_mut_ptr());
        //if this is the bot self
        if client == (*bs).client {
            other = crate::src::qcommon::q_shared::qfalse as i32
        } else if crate::src::game::ai_dmq3::BotSameTeam(bs, client) == 0 {
            //FIXME: say "I don't help the enemy"
            return;
        } else {
            other = crate::src::qcommon::q_shared::qtrue as i32
        }
    } else {
        //get the netname
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        other = crate::src::qcommon::q_shared::qfalse as i32
    }
    //if the bot doesn't know who to help (FindClientByName returned -1)
    if client < 0 {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs,
            b"whois\x00" as *const u8 as *mut i8,
            netname.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).client, 1);
        return;
    }
    //
    (*bs).lead_teamgoal.entitynum = -(1);
    crate::src::game::ai_main::BotEntityInfo(client, &mut entinfo);
    //if info is valid (in PVS)
    if entinfo.valid != 0 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if areanum != 0 {
            // && trap_AAS_AreaReachability(areanum)) {
            (*bs).lead_teamgoal.entitynum = client;
            (*bs).lead_teamgoal.areanum = areanum;
            (*bs).lead_teamgoal.origin[0] = entinfo.origin[0];
            (*bs).lead_teamgoal.origin[1] = entinfo.origin[1];
            (*bs).lead_teamgoal.origin[2] = entinfo.origin[2];
            (*bs).lead_teamgoal.mins[0] = -8f32;
            (*bs).lead_teamgoal.mins[1] = -8f32;
            (*bs).lead_teamgoal.mins[2] = -8f32;
            (*bs).lead_teamgoal.maxs[0] = 8f32;
            (*bs).lead_teamgoal.maxs[1] = 8f32;
            (*bs).lead_teamgoal.maxs[2] = 8f32
        }
    }
    if (*bs).teamgoal.entitynum < 0 {
        if other != 0 {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"whereis\x00" as *const u8 as *mut i8,
                teammate.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"whereareyou\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).client, 1);
        return;
    }
    (*bs).lead_teammate = client;
    (*bs).lead_time = crate::src::game::ai_main::floattime + 600f32;
    (*bs).leadvisible_time = 0f32;
    (*bs).leadmessage_time = -(crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32));
}
/*
==================
BotMatch_Kill
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_Kill(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut enemy: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut client: i32 = 0;
    if crate::src::game::ai_dmq3::TeamPlayIsOn() == 0 {
        return;
    }
    //if not addressed to this bot
    if BotAddressedToBot(bs, match_0) == 0 {
        return;
    }
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4,
        enemy.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    //
    client = FindEnemyByName(bs, enemy.as_mut_ptr());
    if client < 0 {
        crate::src::game::ai_main::BotAI_BotInitialChat(
            bs,
            b"whois\x00" as *const u8 as *mut i8,
            enemy.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        client = crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr());
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, client, 2);
        return;
    }
    (*bs).teamgoal.entitynum = client;
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //set the ltg type
    (*bs).ltgtype = 11;
    //set the team goal time
    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 180f32;
    //
    crate::src::game::ai_dmq3::BotSetTeamStatus(bs);
    //DEBUG
}
/*
==================
BotMatch_CTF
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatch_CTF(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut flag: [i8; 128] = [0; 128];
    let mut netname: [i8; 36] = [0; 36];
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        crate::src::game::g_syscalls::trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            1,
            flag.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 128]>() as i32,
        );
        if (*match_0).subtype & 4096 != 0 {
            if crate::src::qcommon::q_shared::Q_stricmp(
                flag.as_mut_ptr(),
                b"red\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*bs).redflagstatus = 1;
                if crate::src::game::ai_dmq3::BotTeam(bs) == crate::bg_public_h::TEAM_BLUE as i32 {
                    crate::src::game::g_syscalls::trap_BotMatchVariable(
                        match_0 as *mut libc::c_void,
                        0,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    (*bs).flagcarrier =
                        crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr())
                }
            } else {
                (*bs).blueflagstatus = 1;
                if crate::src::game::ai_dmq3::BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
                    crate::src::game::g_syscalls::trap_BotMatchVariable(
                        match_0 as *mut libc::c_void,
                        0,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    (*bs).flagcarrier =
                        crate::src::game::ai_dmq3::ClientFromName(netname.as_mut_ptr())
                }
            }
            (*bs).flagstatuschanged = 1;
            (*bs).lastflagcapture_time = crate::src::game::ai_main::floattime
        } else if (*match_0).subtype & 8192 != 0 {
            (*bs).redflagstatus = 0;
            (*bs).blueflagstatus = 0;
            (*bs).flagcarrier = 0;
            (*bs).flagstatuschanged = 1
        } else if (*match_0).subtype & 16384 != 0 {
            if crate::src::qcommon::q_shared::Q_stricmp(
                flag.as_mut_ptr(),
                b"red\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*bs).redflagstatus = 0
            } else {
                (*bs).blueflagstatus = 0
            }
            (*bs).flagstatuschanged = 1
        }
    };
}
#[no_mangle]

pub unsafe extern "C" fn BotMatch_EnterGame(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut client: i32 = 0;
    let mut netname: [i8; 36] = [0; 36];
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    client = FindClientByName(netname.as_mut_ptr());
    if client >= 0 {
        notleader[client as usize] = crate::src::qcommon::q_shared::qfalse as i32
    };
    //NOTE: eliza chats will catch this
    //Com_sprintf(buf, sizeof(buf), "heya %s", netname);
    //EA_Say(bs->client, buf);
}
#[no_mangle]

pub unsafe extern "C" fn BotMatch_NewLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut match_0: *mut crate::be_ai_chat_h::bot_match_t,
) {
    let mut client: i32 = 0;
    let mut netname: [i8; 36] = [0; 36];
    crate::src::game::g_syscalls::trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    client = FindClientByName(netname.as_mut_ptr());
    if crate::src::game::ai_dmq3::BotSameTeam(bs, client) == 0 {
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*bs).teamleader.as_mut_ptr(),
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
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
/* ****************************************************************************
 * name:		ai_cmd.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
/*
==================
BotMatchMessage
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMatchMessage(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut message: *mut i8,
) -> i32 {
    let mut match_0: crate::be_ai_chat_h::bot_match_t = crate::be_ai_chat_h::bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [crate::be_ai_chat_h::bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    match_0.type_0 = 0;
    //if it is an unknown message
    if crate::src::game::g_syscalls::trap_BotFindMatch(
        message,
        &mut match_0 as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
        (2i32 | 4 | 256) as usize,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    let mut current_block_33: u64;
    //react to the found message
    match match_0.type_0 {
        3 => {
            current_block_33 = 7502529970979898288;
        }
        4 => {
            //someone calling for company
            current_block_33 = 7502529970979898288;
        }
        5 => {
            //teamplay defend a key area
            BotMatch_DefendKeyArea(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        20 => {
            //camp somewhere
            BotMatch_Camp(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        22 => {
            //patrol between several key areas
            BotMatch_Patrol(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        7 => {
            //ctf get the enemy flag
            BotMatch_GetFlag(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        6 => {
            //ctf rush to the base
            BotMatch_RushBase(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        27 => {
            //CTF & 1FCTF
            BotMatch_ReturnFlag(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        30 => {
            //CTF & 1FCTF & Obelisk & Harvester
            BotMatch_TaskPreference(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        300 => {
            //CTF & 1FCTF
            BotMatch_CTF(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        24 => {
            BotMatch_GetItem(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        13 => {
            //join a sub team
            BotMatch_JoinSubteam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        14 => {
            //leave a sub team
            BotMatch_LeaveSubteam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        29 => {
            BotMatch_WhichTeam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        21 => {
            //remember a check point
            BotMatch_CheckPoint(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        15 => {
            //start the creation of a new formation
            crate::src::game::g_syscalls::trap_EA_SayTeam(
                (*bs).client,
                b"the part of my brain to create formations has been damaged\x00" as *const u8
                    as *mut i8,
            );
            current_block_33 = 2631791190359682872;
        }
        16 => {
            //tell someone his/her position in the formation
            crate::src::game::g_syscalls::trap_EA_SayTeam(
                (*bs).client,
                b"the part of my brain to create formations has been damaged\x00" as *const u8
                    as *mut i8,
            );
            current_block_33 = 2631791190359682872;
        }
        17 => {
            //set the formation space
            BotMatch_FormationSpace(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        19 => {
            //dismiss someone
            BotMatch_Dismiss(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        8 => {
            //someone will become the team leader
            BotMatch_StartTeamLeaderShip(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        9 => {
            //someone will stop being the team leader
            BotMatch_StopTeamLeaderShip(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        10 => {
            BotMatch_WhoIsTeamLeader(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        12 => {
            //ask a bot what he/she is doing
            BotMatch_WhatAreYouDoing(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        28 => {
            BotMatch_WhatIsMyCommand(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        26 => {
            BotMatch_WhereAreYou(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        23 => {
            BotMatch_LeadTheWay(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        25 => {
            BotMatch_Kill(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        2 => {
            //someone entered the game
            BotMatch_EnterGame(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        1 => {
            BotMatch_NewLeader(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        18 | 11 => {
            current_block_33 = 2631791190359682872;
        }
        33 => {
            BotMatch_Suicide(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        _ => {
            crate::src::game::ai_main::BotAI_Print(
                1,
                b"unknown match type\n\x00" as *const u8 as *mut i8,
            );
            current_block_33 = 2631791190359682872;
        }
    }
    match current_block_33 {
        7502529970979898288 =>
        //someone calling for help
        {
            BotMatch_HelpAccompany(bs, &mut match_0);
        }
        _ => {}
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
