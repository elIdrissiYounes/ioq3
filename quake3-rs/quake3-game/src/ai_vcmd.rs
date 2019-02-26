#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use ai_cmd::notleader;
use ai_dmq3::{
    ctf_blueflag, ctf_redflag, gametype, BotGetAlternateRouteGoal, BotOppositeTeam,
    BotPointAreaNum, BotRememberLastOrderedTask, BotSameTeam, BotSetTeamStatus, BotTeam,
    BotTeamFlagCarrier, ClientName, EasyClientName, TeamPlayIsOn,
};
use ai_main::{
    bot_activategoal_s, bot_activategoal_t, bot_developer, bot_state_s, bot_state_t,
    bot_waypoint_s, bot_waypoint_t, floattime, BotAILoadMap, BotAISetup, BotAISetupClient,
    BotAIShutdown, BotAIShutdownClient, BotAIStartFrame, BotEntityInfo, BotInterbreedEndMatch,
    BotTestAAS,
};
use ai_team::{BotGetTeamMateTaskPreference, BotSetTeamMateTaskPreference, BotVoiceChatOnly};
use ai_variadic_h::BotAI_BotInitialChat;
use be_aas_h::{aas_entityinfo_s, aas_entityinfo_t};
use be_ai_goal_h::{bot_goal_s, bot_goal_t};
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
    BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
    BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
use bg_public_h::{
    unnamed_0, unnamed_1, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED,
    TEAM_SPECTATOR,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
use g_active::{ClientEndFrame, ClientThink, G_RunClient};
use g_arenas::{
    podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f, UpdateTournamentInfo,
};
use g_bot::{
    G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
    Svcmd_BotList_f,
};
use g_client::{
    ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
    CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch, SP_info_player_intermission,
    SP_info_player_start, SelectSpawnPoint, SetClientViewAngle, SpotWouldTelefrag, TeamCount,
    TeamLeader,
};
use g_cmds::{
    BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
    DeathmatchScoreboardMessage, SetTeam, StopFollowing,
};
use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
use g_items::{
    ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem, G_SpawnItem,
    RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
};
use g_local_h::{bot_settings_s, bot_settings_t, trap_BotEnterChat, trap_EA_Action};
use g_main::{
    g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
    g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn, g_friendlyFire,
    g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref, g_maxGameClients,
    g_maxclients, g_motd, g_password, g_quadfactor, g_restarted, g_smoothClients, g_speed,
    g_synchronousClients, g_teamAutoJoin, g_teamForceBalance, g_weaponRespawn, g_weaponTeamRespawn,
    level, pmove_fixed, pmove_msec, AddTournamentQueue, BeginIntermission, CalculateRanks,
    CheckTeamLeader, ExitLevel, FindIntermissionPoint, G_RunThink, MoveClientToIntermission,
    SetLeader,
};
use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
use g_misc::{
    SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model, SP_misc_portal_camera,
    SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade, SP_shooter_plasma,
    SP_shooter_rocket, TeleportPlayer,
};
use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
use g_mover::{
    G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
    SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
};
use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
use g_spawn::{G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector};
use g_svcmds::{ConsoleCommand, G_FilterPacket, G_ProcessIPBans};
use g_target::{
    SP_target_delay, SP_target_give, SP_target_kill, SP_target_laser, SP_target_location,
    SP_target_position, SP_target_print, SP_target_relay, SP_target_remove_powerups,
    SP_target_score, SP_target_speaker, SP_target_teleporter,
};
use g_team::{
    OnSameTeam, SP_team_CTF_blueplayer, SP_team_CTF_bluespawn, SP_team_CTF_redplayer,
    SP_team_CTF_redspawn, Team_CheckDroppedItem,
};
use g_trigger::{
    SP_func_timer, SP_target_push, SP_trigger_always, SP_trigger_hurt, SP_trigger_multiple,
    SP_trigger_push, SP_trigger_teleport,
};
use g_utils::{
    tv, vectoyaw, vtos, G_AddEvent, G_AddPredictableEvent, G_Find, G_FreeEntity, G_InitGentity,
    G_KillBox, G_ModelIndex, G_PickTarget, G_SetMovedir, G_SetOrigin, G_Sound, G_SoundIndex,
    G_Spawn, G_TeamCommand, G_TempEntity, G_UseTargets,
};
use g_weapon::{
    CheckGauntletAttack, FireWeapon, LogAccuracyHit, SnapVectorTowards, Weapon_HookFree,
    Weapon_HookThink,
};
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, playerState_s, playerState_t, qfalse, qtrue, unnamed, usercmd_s, usercmd_t, vec3_t,
    vec_t, Q_stricmp, Q_strncpyz,
};
use stdlib::{atoi, memcpy, rand};
extern crate libc;

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
 * name:		ai_vcmd.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_vcmd.c $
 *
 *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChatCommand(
    mut bs: *mut bot_state_t,
    mut mode: libc::c_int,
    mut voiceChat: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut clientNum: libc::c_int = 0;
    //int voiceOnly, color;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if mode == 0i32 {
        return qfalse as libc::c_int;
    }
    Q_strncpyz(
        buf.as_mut_ptr(),
        voiceChat,
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    cmd = buf.as_mut_ptr();
    while 0 != *cmd as libc::c_int && *cmd as libc::c_int > ' ' as i32 {
        cmd = cmd.offset(1isize)
    }
    while 0 != *cmd as libc::c_int && *cmd as libc::c_int <= ' ' as i32 {
        let fresh0 = cmd;
        cmd = cmd.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char
    }
    ptr = cmd;
    while 0 != *cmd as libc::c_int && *cmd as libc::c_int > ' ' as i32 {
        cmd = cmd.offset(1isize)
    }
    while 0 != *cmd as libc::c_int && *cmd as libc::c_int <= ' ' as i32 {
        let fresh1 = cmd;
        cmd = cmd.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char
    }
    clientNum = atoi(ptr);
    while 0 != *cmd as libc::c_int && *cmd as libc::c_int > ' ' as i32 {
        cmd = cmd.offset(1isize)
    }
    while 0 != *cmd as libc::c_int && *cmd as libc::c_int <= ' ' as i32 {
        let fresh2 = cmd;
        cmd = cmd.offset(1);
        *fresh2 = '\u{0}' as i32 as libc::c_char
    }
    if 0 == BotSameTeam(bs, clientNum) {
        return qfalse as libc::c_int;
    }
    i = 0i32;
    while !voiceCommands[i as usize].cmd.is_null() {
        if 0 == Q_stricmp(cmd, voiceCommands[i as usize].cmd) {
            voiceCommands[i as usize]
                .func
                .expect("non-null function pointer")(bs, clientNum, mode);
            return qtrue as libc::c_int;
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
#[no_mangle]
pub static mut voiceCommands: [voiceCommand_t; 15] = [
    voiceCommand_s {
        cmd: b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_GetFlag),
    },
    voiceCommand_s {
        cmd: b"offense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_Offense),
    },
    voiceCommand_s {
        cmd: b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_Defend),
    },
    voiceCommand_s {
        cmd: b"defendflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_DefendFlag),
    },
    voiceCommand_s {
        cmd: b"patrol\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_Patrol),
    },
    voiceCommand_s {
        cmd: b"camp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_Camp),
    },
    voiceCommand_s {
        cmd: b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_FollowMe),
    },
    voiceCommand_s {
        cmd: b"followflagcarrier\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_FollowFlagCarrier),
    },
    voiceCommand_s {
        cmd: b"returnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_ReturnFlag),
    },
    voiceCommand_s {
        cmd: b"startleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_StartLeader),
    },
    voiceCommand_s {
        cmd: b"stopleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_StopLeader),
    },
    voiceCommand_s {
        cmd: b"whoisleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_WhoIsLeader),
    },
    voiceCommand_s {
        cmd: b"wantondefense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_WantOnDefense),
    },
    voiceCommand_s {
        cmd: b"wantonoffense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_WantOnOffense),
    },
    voiceCommand_s {
        cmd: 0 as *const libc::c_char as *mut libc::c_char,
        func: Some(BotVoiceChat_Dummy),
    },
];
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_Dummy(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
}
/*
==================
BotVoiceChat_WantOnOffense
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_WantOnOffense(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut preference: libc::c_int = 0;
    preference = BotGetTeamMateTaskPreference(bs, client);
    preference &= !1i32;
    preference |= 2i32;
    BotSetTeamMateTaskPreference(bs, client, preference);
    EasyClientName(
        client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    BotAI_BotInitialChat(
        bs,
        b"keepinmind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        netname.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    trap_BotEnterChat((*bs).cs, client, 2i32);
    BotVoiceChatOnly(
        bs,
        client,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_EA_Action((*bs).client, 0x100000i32);
}
/*
==================
BotVoiceChat_WantOnDefense
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_WantOnDefense(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut preference: libc::c_int = 0;
    preference = BotGetTeamMateTaskPreference(bs, client);
    preference &= !2i32;
    preference |= 1i32;
    BotSetTeamMateTaskPreference(bs, client, preference);
    EasyClientName(
        client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    BotAI_BotInitialChat(
        bs,
        b"keepinmind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        netname.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    trap_BotEnterChat((*bs).cs, client, 2i32);
    BotVoiceChatOnly(
        bs,
        client,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_EA_Action((*bs).client, 0x100000i32);
}
/*
==================
BotVoiceChat_WhoIsLeader
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_WhoIsLeader(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    if 0 == TeamPlayIsOn() {
        return;
    }
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) {
        BotAI_BotInitialChat(
            bs,
            b"iamteamleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 1i32);
        BotVoiceChatOnly(
            bs,
            -1i32,
            b"startleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
/*
==================
BotVoiceChat_StopLeader
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_StopLeader(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    if 0 == Q_stricmp(
        (*bs).teamleader.as_mut_ptr(),
        ClientName(
            client,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        ),
    ) {
        (*bs).teamleader[0usize] = '\u{0}' as i32 as libc::c_char;
        notleader[client as usize] = qtrue as libc::c_int
    };
}
//DEBUG
/*
==================
BotVoiceChat_StartLeader
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_StartLeader(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    ClientName(
        client,
        (*bs).teamleader.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
}
//DEBUG
/*
==================
BotVoiceChat_ReturnFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_ReturnFlag(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    if gametype != GT_CTF as libc::c_int {
        return;
    }
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 6i32;
    (*bs).teamgoal_time = floattime + 180i32 as libc::c_float;
    (*bs).rushbaseaway_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
}
//DEBUG
/*
==================
BotVoiceChat_FollowFlagCarrier
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_FollowFlagCarrier(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    let mut carrier: libc::c_int = 0;
    carrier = BotTeamFlagCarrier(bs);
    if carrier >= 0i32 {
        BotVoiceChat_FollowMe(bs, carrier, mode);
    };
}
//DEBUG
/*
==================
BotVoiceChat_FollowMe
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_FollowMe(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    let mut areanum: libc::c_int = 0;
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_s {
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
    let mut netname: [libc::c_char; 36] = [0; 36];
    (*bs).teamgoal.entitynum = -1i32;
    BotEntityInfo(client, &mut entinfo);
    if 0 != entinfo.valid {
        areanum = BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if 0 != areanum {
            (*bs).teamgoal.entitynum = client;
            (*bs).teamgoal.areanum = areanum;
            (*bs).teamgoal.origin[0usize] = entinfo.origin[0usize];
            (*bs).teamgoal.origin[1usize] = entinfo.origin[1usize];
            (*bs).teamgoal.origin[2usize] = entinfo.origin[2usize];
            (*bs).teamgoal.mins[0usize] = -8i32 as vec_t;
            (*bs).teamgoal.mins[1usize] = -8i32 as vec_t;
            (*bs).teamgoal.mins[2usize] = -8i32 as vec_t;
            (*bs).teamgoal.maxs[0usize] = 8i32 as vec_t;
            (*bs).teamgoal.maxs[1usize] = 8i32 as vec_t;
            (*bs).teamgoal.maxs[2usize] = 8i32 as vec_t
        }
    }
    if (*bs).teamgoal.entitynum < 0i32 {
        BotAI_BotInitialChat(
            bs,
            b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName(
                client,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, client, 2i32);
        return;
    }
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammate = client;
    (*bs).teammatevisible_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
    (*bs).ltgtype = 2i32;
    (*bs).formation_dist = (3.5f64 * 32i32 as libc::c_double) as libc::c_float;
    (*bs).arrive_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
//DEBUG
/*
==================
BotVoiceChat_Camp
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_Camp(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    let mut areanum: libc::c_int = 0;
    let mut entinfo: aas_entityinfo_t = aas_entityinfo_s {
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
    let mut netname: [libc::c_char; 36] = [0; 36];
    (*bs).teamgoal.entitynum = -1i32;
    BotEntityInfo(client, &mut entinfo);
    if 0 != entinfo.valid {
        areanum = BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if 0 != areanum {
            (*bs).teamgoal.entitynum = client;
            (*bs).teamgoal.areanum = areanum;
            (*bs).teamgoal.origin[0usize] = entinfo.origin[0usize];
            (*bs).teamgoal.origin[1usize] = entinfo.origin[1usize];
            (*bs).teamgoal.origin[2usize] = entinfo.origin[2usize];
            (*bs).teamgoal.mins[0usize] = -8i32 as vec_t;
            (*bs).teamgoal.mins[1usize] = -8i32 as vec_t;
            (*bs).teamgoal.mins[2usize] = -8i32 as vec_t;
            (*bs).teamgoal.maxs[0usize] = 8i32 as vec_t;
            (*bs).teamgoal.maxs[1usize] = 8i32 as vec_t;
            (*bs).teamgoal.maxs[2usize] = 8i32 as vec_t
        }
    }
    if (*bs).teamgoal.entitynum < 0i32 {
        BotAI_BotInitialChat(
            bs,
            b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName(
                client,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            ),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, client, 2i32);
        return;
    }
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 8i32;
    (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
    (*bs).teammate = client;
    (*bs).arrive_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
/*
==================
BotVoiceChat_Patrol
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_Patrol(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    (*bs).decisionmaker = client;
    (*bs).ltgtype = 0i32;
    (*bs).lead_time = 0i32 as libc::c_float;
    (*bs).lastgoal_ltgtype = 0i32;
    BotAI_BotInitialChat(
        bs,
        b"dismissed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_void,
    );
    trap_BotEnterChat((*bs).cs, client, 2i32);
    BotVoiceChatOnly(
        bs,
        -1i32,
        b"onpatrol\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    BotSetTeamStatus(bs);
}
//DEBUG
/*
==================
BotVoiceChat_DefendFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_DefendFlag(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    BotVoiceChat_Defend(bs, client, mode);
}
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_Defend(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    if gametype == GT_CTF as libc::c_int {
        match BotTeam(bs) {
            1 => {
                memcpy(
                    &mut (*bs).teamgoal as *mut bot_goal_t as *mut libc::c_void,
                    &mut ctf_redflag as *mut bot_goal_t as *const libc::c_void,
                    ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                );
            }
            2 => {
                memcpy(
                    &mut (*bs).teamgoal as *mut bot_goal_t as *mut libc::c_void,
                    &mut ctf_blueflag as *mut bot_goal_t as *const libc::c_void,
                    ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                );
            }
            _ => return,
        }
    } else {
        return;
    }
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 3i32;
    (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
    (*bs).defendaway_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
//DEBUG
/*
==================
BotVoiceChat_Offense
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_Offense(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    if gametype == GT_CTF as libc::c_int {
        BotVoiceChat_GetFlag(bs, client, mode);
        return;
    }
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 13i32;
    (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
    (*bs).attackaway_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
/*
==================
BotVoiceChat_GetFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat_GetFlag(
    mut bs: *mut bot_state_t,
    mut client: libc::c_int,
    mut mode: libc::c_int,
) {
    if gametype == GT_CTF as libc::c_int {
        if 0 == ctf_redflag.areanum || 0 == ctf_blueflag.areanum {
            return;
        }
    } else {
        return;
    }
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 4i32;
    (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
    if gametype == GT_CTF as libc::c_int {
        BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
    }
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct voiceCommand_s {
    pub cmd: *mut libc::c_char,
    pub func:
        Option<unsafe extern "C" fn(_: *mut bot_state_t, _: libc::c_int, _: libc::c_int) -> ()>,
}
pub type voiceCommand_t = voiceCommand_s;
