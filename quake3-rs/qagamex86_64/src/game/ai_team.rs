use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
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
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
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
use crate::src::game::ai_cmd::notleader;
use crate::src::game::ai_dmq3::ctf_blueflag;
use crate::src::game::ai_dmq3::ctf_redflag;
use crate::src::game::ai_dmq3::gametype;
use crate::src::game::ai_dmq3::BotPointAreaNum;
use crate::src::game::ai_dmq3::BotSameTeam;
use crate::src::game::ai_dmq3::BotSetLastOrderedTask;
use crate::src::game::ai_dmq3::BotTeam;
use crate::src::game::ai_dmq3::ClientFromName;
use crate::src::game::ai_dmq3::ClientName;
pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_BotInitialChat;
pub use crate::src::game::ai_main::BotAI_GetClientState;
pub use crate::src::game::ai_team::stdlib_h::atoi;
use crate::src::game::ai_vcmd::BotVoiceChat_Defend;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotGetChatMessage;
pub use crate::src::game::g_syscalls::trap_BotQueueConsoleMessage;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;
use crate::stdlib::memcpy;
pub use crate::stdlib::rand;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncpy;
pub use crate::stdlib::strtol;
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
 * name:		ai_team.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_team.c $
 *
 *****************************************************************************/
//
// for the voice chats
//ctf task preferences for a client

pub type bot_ctftaskpreference_t = bot_ctftaskpreference_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_ctftaskpreference_s {
    pub name: [i8; 36],
    pub preference: i32,
}
#[no_mangle]

pub static mut ctftaskpreferences: [bot_ctftaskpreference_t; 64] = [bot_ctftaskpreference_t {
    name: [0; 36],
    preference: 0,
}; 64];
/*
==================
BotValidTeamLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotValidTeamLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    if crate::stdlib::strlen((*bs).teamleader.as_mut_ptr()) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if crate::src::game::ai_dmq3::ClientFromName((*bs).teamleader.as_mut_ptr()) == -(1) {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotNumTeamMates
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotNumTeamMates(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut numplayers: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    numplayers = 0;
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 + 256 + 256 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const i8,
            )) == 0)
        {
            //skip spectators
            if !(atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const i8,
            )) == crate::bg_public_h::TEAM_SPECTATOR as i32)
            {
                //
                if crate::src::game::ai_dmq3::BotSameTeam(bs, i) != 0 {
                    numplayers += 1
                }
            }
        }
        i += 1
    }
    return numplayers;
}
/*
==================
BotClientTravelTimeToGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotClientTravelTimeToGoal(
    mut client: i32,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut ps: crate::src::qcommon::q_shared::playerState_t =
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
    let mut areanum: i32 = 0;
    if crate::src::game::ai_main::BotAI_GetClientState(client, &mut ps) != 0 {
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(ps.origin.as_mut_ptr())
    } else {
        areanum = 0
    }
    if areanum == 0 {
        return 1i32;
    }
    return crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
        areanum,
        ps.origin.as_mut_ptr(),
        (*goal).areanum,
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
}
/*
==================
BotSortTeamMatesByBaseTravelTime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSortTeamMatesByBaseTravelTime(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammates: *mut i32,
    mut maxteammates: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut numteammates: i32 = 0;
    let mut traveltime: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut traveltimes: [i32; 64] = [0; 64];
    let mut goal: *mut crate::be_ai_goal_h::bot_goal_t = 0 as *mut crate::be_ai_goal_h::bot_goal_t;
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        if crate::src::game::ai_dmq3::BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
            goal = &mut crate::src::game::ai_dmq3::ctf_redflag
        } else {
            goal = &mut crate::src::game::ai_dmq3::ctf_blueflag
        }
    }
    numteammates = 0;
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 + 256 + 256 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const i8,
            )) == 0)
        {
            //skip spectators
            if !(atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const i8,
            )) == crate::bg_public_h::TEAM_SPECTATOR as i32)
            {
                //
                if crate::src::game::ai_dmq3::BotSameTeam(bs, i) != 0 && !goal.is_null() {
                    //
                    traveltime = BotClientTravelTimeToGoal(i, goal);
                    //
                    j = 0;
                    while j < numteammates {
                        if traveltime < traveltimes[j as usize] {
                            k = numteammates;
                            while k > j {
                                traveltimes[k as usize] = traveltimes[(k - 1) as usize];
                                *teammates.offset(k as isize) = *teammates.offset((k - 1) as isize);
                                k -= 1
                            }
                            break;
                        } else {
                            j += 1
                        }
                    }
                    traveltimes[j as usize] = traveltime;
                    *teammates.offset(j as isize) = i;
                    numteammates += 1;
                    if numteammates >= maxteammates {
                        break;
                    }
                }
            }
        }
        i += 1
    }
    return numteammates;
}
/*
==================
BotSetTeamMateTaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetTeamMateTaskPreference(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammate: i32,
    mut preference: i32,
) {
    let mut teammatename: [i8; 36] = [0; 36];
    ctftaskpreferences[teammate as usize].preference = preference;
    crate::src::game::ai_dmq3::ClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    crate::stdlib::strcpy(
        ctftaskpreferences[teammate as usize].name.as_mut_ptr(),
        teammatename.as_mut_ptr(),
    );
}
/*
==================
BotGetTeamMateTaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetTeamMateTaskPreference(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammate: i32,
) -> i32 {
    let mut teammatename: [i8; 36] = [0; 36];
    if ctftaskpreferences[teammate as usize].preference == 0 {
        return 0i32;
    }
    crate::src::game::ai_dmq3::ClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(
        teammatename.as_mut_ptr(),
        ctftaskpreferences[teammate as usize].name.as_mut_ptr(),
    ) != 0
    {
        return 0i32;
    }
    return ctftaskpreferences[teammate as usize].preference;
}
/*
==================
BotSortTeamMatesByTaskPreference
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSortTeamMatesByTaskPreference(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammates: *mut i32,
    mut numteammates: i32,
) -> i32 {
    let mut defenders: [i32; 64] = [0; 64];
    let mut numdefenders: i32 = 0;
    let mut attackers: [i32; 64] = [0; 64];
    let mut numattackers: i32 = 0;
    let mut roamers: [i32; 64] = [0; 64];
    let mut numroamers: i32 = 0;
    let mut i: i32 = 0;
    let mut preference: i32 = 0;
    numroamers = 0;
    numattackers = numroamers;
    numdefenders = numattackers;
    i = 0;
    while i < numteammates {
        preference = BotGetTeamMateTaskPreference(bs, *teammates.offset(i as isize));
        if preference & 1 != 0 {
            let fresh0 = numdefenders;
            numdefenders = numdefenders + 1;
            defenders[fresh0 as usize] = *teammates.offset(i as isize)
        } else if preference & 2 != 0 {
            let fresh1 = numattackers;
            numattackers = numattackers + 1;
            attackers[fresh1 as usize] = *teammates.offset(i as isize)
        } else {
            let fresh2 = numroamers;
            numroamers = numroamers + 1;
            roamers[fresh2 as usize] = *teammates.offset(i as isize)
        }
        i += 1
    }
    numteammates = 0;
    //defenders at the front of the list
    crate::stdlib::memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut i32 as *mut libc::c_void,
        defenders.as_mut_ptr() as *const libc::c_void,
        (numdefenders as usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    numteammates += numdefenders;
    //roamers in the middle
    crate::stdlib::memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut i32 as *mut libc::c_void,
        roamers.as_mut_ptr() as *const libc::c_void,
        (numroamers as usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    numteammates += numroamers;
    //attacker in the back of the list
    crate::stdlib::memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut i32 as *mut libc::c_void,
        attackers.as_mut_ptr() as *const libc::c_void,
        (numattackers as usize).wrapping_mul(::std::mem::size_of::<i32>()),
    );
    numteammates += numattackers;
    return numteammates;
}
/*
==================
BotSayTeamOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSayTeamOrderAlways(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut toclient: i32,
) {
    let mut teamchat: [i8; 256] = [0; 256];
    let mut buf: [i8; 256] = [0; 256];
    let mut name: [i8; 36] = [0; 36];
    //if the bot is talking to itself
    if (*bs).client == toclient {
        //don't show the message just put it in the console message queue
        crate::src::game::g_syscalls::trap_BotGetChatMessage(
            (*bs).cs,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
        );
        crate::src::game::ai_dmq3::ClientName(
            (*bs).client,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 36]>() as i32,
        );
        crate::src::qcommon::q_shared::Com_sprintf(
            teamchat.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
            b"\x19(%s\x19)\x19: %s\x00" as *const u8 as *const i8,
            name.as_mut_ptr(),
            buf.as_mut_ptr(),
        );
        crate::src::game::g_syscalls::trap_BotQueueConsoleMessage(
            (*bs).cs,
            1i32,
            teamchat.as_mut_ptr(),
        );
    } else {
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, toclient, 2i32);
    };
}
/*
==================
BotSayTeamOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSayTeamOrder(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut toclient: i32,
) {
    BotSayTeamOrderAlways(bs, toclient);
}
/*
==================
BotVoiceChat
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChat(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut toclient: i32,
    mut voicechat: *mut i8,
) {
}
/*
==================
BotVoiceChatOnly
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVoiceChatOnly(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut toclient: i32,
    mut voicechat: *mut i8,
) {
}
/*
==================
BotSayVoiceTeamOrder
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSayVoiceTeamOrder(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut toclient: i32,
    mut voicechat: *mut i8,
) {
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_BothFlagsNotAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: i32 = 0;
    let mut defenders: i32 = 0;
    let mut attackers: i32 = 0;
    let mut i: i32 = 0;
    let mut other: i32 = 0;
    let mut teammates: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let mut name: [i8; 36] = [0; 36];
    let mut carriername: [i8; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[i32; 64]>() as i32,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //different orders based on the number of team mates
    match (*bs).numteammates {
        1 => {}
        2 => {
            //tell the one not carrying the flag to attack the enemy base
            if teammates[0] != (*bs).flagcarrier {
                other = teammates[0]
            } else {
                other = teammates[1]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 36]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"cmd_getflag\x00" as *const u8 as *mut i8,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(bs, other, b"getflag\x00" as *const u8 as *mut i8);
        }
        3 => {
            //tell the one closest to the base not carrying the flag to accompany the flag carrier
            if teammates[0] != (*bs).flagcarrier {
                other = teammates[0]
            } else {
                other = teammates[1]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 36]>() as i32,
            );
            if (*bs).flagcarrier != -(1) {
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                if (*bs).flagcarrier == (*bs).client {
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_accompanyme\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayVoiceTeamOrder(bs, other, b"followme\x00" as *const u8 as *mut i8);
                } else {
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_accompany\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        carriername.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        other,
                        b"followflagcarrier\x00" as *const u8 as *mut i8,
                    );
                }
            } else {
                //
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayVoiceTeamOrder(bs, other, b"getflag\x00" as *const u8 as *mut i8);
            }
            BotSayTeamOrder(bs, other);
            //tell the one furthest from the the base not carrying the flag to get the enemy flag
            if teammates[2] != (*bs).flagcarrier {
                other = teammates[2]
            } else {
                other = teammates[1]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 36]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"cmd_getflag\x00" as *const u8 as *mut i8,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(bs, other, b"returnflag\x00" as *const u8 as *mut i8);
        }
        _ => {
            defenders = (numteammates as f32 as i32 as f64 * 0.4 + 0.5) as i32;
            if defenders > 4 {
                defenders = 4
            }
            attackers = (numteammates as f32 as i32 as f64 * 0.5 + 0.5) as i32;
            if attackers > 5 {
                attackers = 5
            }
            if (*bs).flagcarrier != -(1) {
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                i = 0;
                while i < defenders {
                    //
                    if !(teammates[i as usize] == (*bs).flagcarrier) {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[i as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 36]>() as i32,
                        );
                        if (*bs).flagcarrier == (*bs).client {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompanyme\x00" as *const u8 as *mut i8,
                                name.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[i as usize],
                                b"followme\x00" as *const u8 as *mut i8,
                            );
                        } else {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompany\x00" as *const u8 as *mut i8,
                                name.as_mut_ptr(),
                                carriername.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[i as usize],
                                b"followflagcarrier\x00" as *const u8 as *mut i8,
                            );
                        }
                        BotSayTeamOrder(bs, teammates[i as usize]);
                    }
                    i += 1
                }
            } else {
                i = 0;
                while i < defenders {
                    //
                    if !(teammates[i as usize] == (*bs).flagcarrier) {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[i as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 36]>() as i32,
                        );
                        crate::src::game::ai_main::BotAI_BotInitialChat(
                            bs,
                            b"cmd_getflag\x00" as *const u8 as *mut i8,
                            name.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                        BotSayVoiceTeamOrder(
                            bs,
                            teammates[i as usize],
                            b"getflag\x00" as *const u8 as *mut i8,
                        );
                        BotSayTeamOrder(bs, teammates[i as usize]);
                    }
                    i += 1
                }
            }
            i = 0;
            while i < attackers {
                //
                if !(teammates[(numteammates - i - 1) as usize] == (*bs).flagcarrier) {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1i32) as usize],
                        b"returnflag\x00" as *const u8 as *mut i8,
                    );
                }
                i += 1
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_FlagNotAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: i32 = 0;
    let mut defenders: i32 = 0;
    let mut attackers: i32 = 0;
    let mut i: i32 = 0;
    let mut teammates: [i32; 64] = [0; 64];
    let mut name: [i8; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[i32; 64]>() as i32,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //passive strategy
    if (*bs).ctfstrategy & 1 == 0 {
        //different orders based on the number of team mates
        match (*bs).numteammates {
            1 => {}
            2 => {
                // keep one near the base for when the flag is returned
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"defend\x00" as *const u8 as *mut i8);
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            3 => {
                //keep one near the base for when the flag is returned
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"defend\x00" as *const u8 as *mut i8);
                //the other two get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(bs, teammates[1], b"getflag\x00" as *const u8 as *mut i8);
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            _ => {
                //keep some people near the base for when the flag is returned
                defenders = (numteammates as f32 as i32 as f64 * 0.3 + 0.5) as i32;
                if defenders > 3 {
                    defenders = 3
                }
                attackers = (numteammates as f32 as i32 as f64 * 0.6 + 0.5) as i32;
                if attackers > 6 {
                    attackers = 6
                }
                i = 0;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *mut i8,
                    );
                    i += 1
                }
                i = 0;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1) as usize]);
                    BotSayVoiceTeamOrder(bs, teammates[0], b"getflag\x00" as *const u8 as *mut i8);
                    i += 1
                }
            }
        }
    } else {
        //different orders based on the number of team mates
        match (*bs).numteammates {
            1 => {}
            2 => {
                //both will go for the enemy flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"getflag\x00" as *const u8 as *mut i8);
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            3 => {
                //everyone go for the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"getflag\x00" as *const u8 as *mut i8);
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(bs, teammates[1], b"getflag\x00" as *const u8 as *mut i8);
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            _ => {
                //keep some people near the base for when the flag is returned
                defenders = (numteammates as f32 as i32 as f64 * 0.2 + 0.5) as i32;
                if defenders > 2 {
                    defenders = 2
                }
                attackers = (numteammates as f32 as i32 as f64 * 0.7 + 0.5) as i32;
                if attackers > 7 {
                    attackers = 7
                }
                i = 0;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *mut i8,
                    );
                    i += 1
                }
                i = 0;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1) as usize],
                        b"getflag\x00" as *const u8 as *mut i8,
                    );
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_EnemyFlagNotAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: i32 = 0;
    let mut defenders: i32 = 0;
    let mut attackers: i32 = 0;
    let mut i: i32 = 0;
    let mut other: i32 = 0;
    let mut teammates: [i32; 64] = [0; 64];
    let mut name: [i8; 36] = [0; 36];
    let mut carriername: [i8; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[i32; 64]>() as i32,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //different orders based on the number of team mates
    match numteammates {
        1 => {}
        2 => {
            //tell the one not carrying the flag to defend the base
            if teammates[0] == (*bs).flagcarrier {
                other = teammates[1]
            } else {
                other = teammates[0]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 36]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"cmd_defendbase\x00" as *const u8 as *mut i8,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(bs, other, b"defend\x00" as *const u8 as *mut i8);
        }
        3 => {
            //tell the one closest to the base not carrying the flag to defend the base
            if teammates[0] != (*bs).flagcarrier {
                other = teammates[0]
            } else {
                other = teammates[1]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 36]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"cmd_defendbase\x00" as *const u8 as *mut i8,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(bs, other, b"defend\x00" as *const u8 as *mut i8);
            //tell the other also to defend the base
            if teammates[2] != (*bs).flagcarrier {
                other = teammates[2]
            } else {
                other = teammates[1]
            }
            crate::src::game::ai_dmq3::ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 36]>() as i32,
            );
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"cmd_defendbase\x00" as *const u8 as *mut i8,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(bs, other, b"defend\x00" as *const u8 as *mut i8);
        }
        _ => {
            //60% will defend the base
            defenders = (numteammates as f32 as i32 as f64 * 0.6 + 0.5) as i32;
            if defenders > 6 {
                defenders = 6
            }
            //30% accompanies the flag carrier
            attackers = (numteammates as f32 as i32 as f64 * 0.3 + 0.5) as i32;
            if attackers > 3 {
                attackers = 3
            }
            i = 0;
            while i < defenders {
                //
                if !(teammates[i as usize] == (*bs).flagcarrier) {
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *mut i8,
                    );
                }
                i += 1
            }
            // if we have a flag carrier
            if (*bs).flagcarrier != -(1) {
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                i = 0;
                while i < attackers {
                    //
                    if !(teammates[(numteammates - i - 1) as usize] == (*bs).flagcarrier) {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[(numteammates - i - 1) as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 36]>() as i32,
                        );
                        if (*bs).flagcarrier == (*bs).client {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompanyme\x00" as *const u8 as *mut i8,
                                name.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[(numteammates - i - 1i32) as usize],
                                b"followme\x00" as *const u8 as *mut i8,
                            );
                        } else {
                            crate::src::game::ai_main::BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompany\x00" as *const u8 as *mut i8,
                                name.as_mut_ptr(),
                                carriername.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[(numteammates - i - 1i32) as usize],
                                b"followflagcarrier\x00" as *const u8 as *mut i8,
                            );
                        }
                        BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    }
                    i += 1
                }
            } else {
                i = 0;
                while i < attackers {
                    //
                    if !(teammates[(numteammates - i - 1) as usize] == (*bs).flagcarrier) {
                        //
                        crate::src::game::ai_dmq3::ClientName(
                            teammates[(numteammates - i - 1) as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 36]>() as i32,
                        );
                        crate::src::game::ai_main::BotAI_BotInitialChat(
                            bs,
                            b"cmd_getflag\x00" as *const u8 as *mut i8,
                            name.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                        BotSayVoiceTeamOrder(
                            bs,
                            teammates[(numteammates - i - 1) as usize],
                            b"getflag\x00" as *const u8 as *mut i8,
                        );
                        BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    }
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders_BothFlagsAtBase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut numteammates: i32 = 0;
    let mut defenders: i32 = 0;
    let mut attackers: i32 = 0;
    let mut i: i32 = 0;
    let mut teammates: [i32; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let mut name: [i8; 36] = [0; 36];
    //sort team mates by travel time to base
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[i32; 64]>() as i32,
    );
    //sort team mates by CTF preference
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    //passive strategy
    if (*bs).ctfstrategy & 1 == 0 {
        //different orders based on the number of team mates
        match numteammates {
            1 => {}
            2 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"defend\x00" as *const u8 as *mut i8);
                //the other will get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            3 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"defend\x00" as *const u8 as *mut i8);
                //the second one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(bs, teammates[1], b"defend\x00" as *const u8 as *mut i8);
                //the other will get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            _ => {
                defenders = (numteammates as f32 as i32 as f64 * 0.5 + 0.5) as i32;
                if defenders > 5 {
                    defenders = 5
                }
                attackers = (numteammates as f32 as i32 as f64 * 0.4 + 0.5) as i32;
                if attackers > 4 {
                    attackers = 4
                }
                i = 0;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *mut i8,
                    );
                    i += 1
                }
                i = 0;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1) as usize],
                        b"getflag\x00" as *const u8 as *mut i8,
                    );
                    i += 1
                }
            }
        }
    } else {
        //different orders based on the number of team mates
        match numteammates {
            1 => {}
            2 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"defend\x00" as *const u8 as *mut i8);
                //the other will get the flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            3 => {
                //the one closest to the base will defend the base
                crate::src::game::ai_dmq3::ClientName(
                    teammates[0],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0]);
                BotSayVoiceTeamOrder(bs, teammates[0], b"defend\x00" as *const u8 as *mut i8);
                //the others should go for the enemy flag
                crate::src::game::ai_dmq3::ClientName(
                    teammates[1],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1]);
                BotSayVoiceTeamOrder(bs, teammates[1], b"getflag\x00" as *const u8 as *mut i8);
                //
                crate::src::game::ai_dmq3::ClientName(
                    teammates[2],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *mut i8,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *mut i8,
                );
            }
            _ => {
                defenders = (numteammates as f32 as i32 as f64 * 0.4 + 0.5) as i32;
                if defenders > 4 {
                    defenders = 4
                }
                attackers = (numteammates as f32 as i32 as f64 * 0.5 + 0.5) as i32;
                if attackers > 5 {
                    attackers = 5
                }
                i = 0;
                while i < defenders {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *mut i8,
                    );
                    i += 1
                }
                i = 0;
                while i < attackers {
                    //
                    crate::src::game::ai_dmq3::ClientName(
                        teammates[(numteammates - i - 1) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::ai_main::BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *mut i8,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1) as usize],
                        b"getflag\x00" as *const u8 as *mut i8,
                    );
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFOrders(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut flagstatus: i32 = 0;
    //
    if crate::src::game::ai_dmq3::BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
        flagstatus = (*bs).redflagstatus * 2 + (*bs).blueflagstatus
    } else {
        flagstatus = (*bs).blueflagstatus * 2 + (*bs).redflagstatus
    }
    //
    match flagstatus {
        0 => {
            BotCTFOrders_BothFlagsAtBase(bs);
        }
        1 => {
            BotCTFOrders_EnemyFlagNotAtBase(bs);
        }
        2 => {
            BotCTFOrders_FlagNotAtBase(bs);
        }
        3 => {
            BotCTFOrders_BothFlagsNotAtBase(bs);
        }
        _ => {}
    };
}
/*
==================
BotCreateGroup
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCreateGroup(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammates: *mut i32,
    mut groupsize: i32,
) {
    let mut name: [i8; 36] = [0; 36];
    let mut leadername: [i8; 36] = [0; 36];
    let mut i: i32 = 0;
    // the others in the group will follow the teammates[0]
    crate::src::game::ai_dmq3::ClientName(
        *teammates.offset(0),
        leadername.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    i = 1;
    while i < groupsize {
        crate::src::game::ai_dmq3::ClientName(
            *teammates.offset(i as isize),
            name.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 36]>() as i32,
        );
        if *teammates.offset(0) == (*bs).client {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"cmd_accompanyme\x00" as *const u8 as *mut i8,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            crate::src::game::ai_main::BotAI_BotInitialChat(
                bs,
                b"cmd_accompany\x00" as *const u8 as *mut i8,
                name.as_mut_ptr(),
                leadername.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        BotSayTeamOrderAlways(bs, *teammates.offset(i as isize));
        i += 1
    }
}
/*
==================
BotTeamOrders

  FIXME: defend key areas?
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamOrders(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut teammates: [i32; 64] = [0; 64];
    let mut numteammates: i32 = 0;
    let mut i: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    numteammates = 0;
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 + 256 + 256 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        //if no config string or no name
        if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
            || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const i8,
            )) == 0)
        {
            //skip spectators
            if !(atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const i8,
            )) == crate::bg_public_h::TEAM_SPECTATOR as i32)
            {
                //
                if crate::src::game::ai_dmq3::BotSameTeam(bs, i) != 0 {
                    teammates[numteammates as usize] = i;
                    numteammates += 1
                }
            }
        }
        i += 1
    }
    //
    match numteammates {
        1 | 2 => {}
        3 => {
            //have one follow another and one free roaming
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2i32); //a group of 2
        }
        4 => {
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2); //a group of 2
            BotCreateGroup(bs, &mut *teammates.as_mut_ptr().offset(2isize), 2i32);
            //a group of 2
        }
        5 => {
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2); //a group of 3
            BotCreateGroup(bs, &mut *teammates.as_mut_ptr().offset(2isize), 3i32);
        }
        _ => {
            if numteammates <= 10 {
                i = 0;
                while i < numteammates / 2 {
                    BotCreateGroup(bs, &mut *teammates.as_mut_ptr().offset((i * 2) as isize), 2);
                    i += 1
                    //groups of 2
                }
            }
        }
    };
}
/*
==================
FindHumanTeamLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn FindHumanTeamLeader(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut i: i32 = 0;
    i = 0;
    while i < 64 {
        if crate::src::game::g_main::g_entities[i as usize].inuse as u64 != 0 {
            // if this player is not a bot
            if crate::src::game::g_main::g_entities[i as usize].r.svFlags & 0x8 == 0 {
                // if this player is ok with being the leader
                if crate::src::game::ai_cmd::notleader[i as usize] == 0 {
                    // if this player is on the same team
                    if crate::src::game::ai_dmq3::BotSameTeam(bs, i) != 0 {
                        crate::src::game::ai_dmq3::ClientName(
                            i,
                            (*bs).teamleader.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 36]>() as i32,
                        );
                        // if not yet ordered to do anything
                        if crate::src::game::ai_dmq3::BotSetLastOrderedTask(bs) == 0 {
                            // go on defense by default
                            crate::src::game::ai_vcmd::BotVoiceChat_Defend(bs, i, 2i32);
                        }
                        return crate::src::qcommon::q_shared::qtrue as i32;
                    }
                }
            }
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
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
 * name:		ai_team.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
/*
==================
BotTeamAI
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamAI(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut numteammates: i32 = 0;
    let mut netname: [i8; 36] = [0; 36];
    //
    if crate::src::game::ai_dmq3::gametype < crate::bg_public_h::GT_TEAM as i32 {
        return;
    }
    // make sure we've got a valid team leader
    if BotValidTeamLeader(bs) == 0 {
        //
        if FindHumanTeamLeader(bs) == 0 {
            //
            if (*bs).askteamleader_time == 0. && (*bs).becometeamleader_time == 0. {
                if (*bs).entergame_time + 10f32 > crate::src::game::ai_main::floattime {
                    (*bs).askteamleader_time = crate::src::game::ai_main::floattime
                        + 5f32
                        + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 10f32
                } else {
                    (*bs).becometeamleader_time = crate::src::game::ai_main::floattime
                        + 5f32
                        + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 10f32
                }
            }
            if (*bs).askteamleader_time != 0.
                && (*bs).askteamleader_time < crate::src::game::ai_main::floattime
            {
                // if asked for a team leader and no response
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"whoisteamleader\x00" as *const u8 as *mut i8,
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0, 1);
                (*bs).askteamleader_time = 0f32;
                (*bs).becometeamleader_time = crate::src::game::ai_main::floattime
                    + 8f32
                    + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 10f32
            }
            if (*bs).becometeamleader_time != 0.
                && (*bs).becometeamleader_time < crate::src::game::ai_main::floattime
            {
                crate::src::game::ai_main::BotAI_BotInitialChat(
                    bs,
                    b"iamteamleader\x00" as *const u8 as *mut i8,
                    0 as *mut libc::c_void,
                );
                crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, 0, 1);
                BotSayVoiceTeamOrder(bs, -(1), b"startleader\x00" as *const u8 as *mut i8);
                crate::src::game::ai_dmq3::ClientName(
                    (*bs).client,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>() as i32,
                );
                crate::stdlib::strncpy(
                    (*bs).teamleader.as_mut_ptr(),
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 36]>(),
                );
                (*bs).teamleader[(::std::mem::size_of::<[i8; 36]>()).wrapping_sub(1usize)] =
                    '\u{0}' as i8;
                (*bs).becometeamleader_time = 0f32
            }
            return;
        }
    }
    (*bs).askteamleader_time = 0f32;
    (*bs).becometeamleader_time = 0f32;
    //return if this bot is NOT the team leader
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
    //
    numteammates = BotNumTeamMates(bs);
    //give orders
    match crate::src::game::ai_dmq3::gametype {
        3 => {
            if (*bs).numteammates != numteammates || (*bs).forceorders != 0 {
                (*bs).teamgiveorders_time = crate::src::game::ai_main::floattime;
                (*bs).numteammates = numteammates;
                (*bs).forceorders = crate::src::qcommon::q_shared::qfalse as i32
            }
            //if it's time to give orders
            if (*bs).teamgiveorders_time != 0.
                && (*bs).teamgiveorders_time < crate::src::game::ai_main::floattime - 5f32
            {
                BotTeamOrders(bs);
                //give orders again after 120 seconds
                (*bs).teamgiveorders_time = crate::src::game::ai_main::floattime + 120f32
            }
        }
        4 => {
            //if the number of team mates changed or the flag status changed
            //or someone wants to know what to do
            if (*bs).numteammates != numteammates
                || (*bs).flagstatuschanged != 0
                || (*bs).forceorders != 0
            {
                (*bs).teamgiveorders_time = crate::src::game::ai_main::floattime;
                (*bs).numteammates = numteammates;
                (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qfalse as i32;
                (*bs).forceorders = crate::src::qcommon::q_shared::qfalse as i32
            }
            //if there were no flag captures the last 3 minutes
            if (*bs).lastflagcapture_time < crate::src::game::ai_main::floattime - 240f32 {
                (*bs).lastflagcapture_time = crate::src::game::ai_main::floattime;
                //randomly change the CTF strategy
                if (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64) < 0.4 {
                    (*bs).ctfstrategy ^= 1;
                    (*bs).teamgiveorders_time = crate::src::game::ai_main::floattime
                }
            }
            //if it's time to give orders
            if (*bs).teamgiveorders_time != 0.
                && (*bs).teamgiveorders_time < crate::src::game::ai_main::floattime - 3f32
            {
                BotCTFOrders(bs);
                //
                (*bs).teamgiveorders_time = 0f32
            }
        }
        _ => {}
    };
}
