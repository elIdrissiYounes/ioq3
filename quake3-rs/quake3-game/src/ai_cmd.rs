use ai_chat::{bot_match_s, bot_match_t, bot_matchvariable_s, bot_matchvariable_t};
use ai_dmq3::{
    ctf_blueflag, ctf_redflag, gametype, stristr, BotCreateWayPoint, BotFindWayPoint,
    BotFreeWaypoints, BotGetAlternateRouteGoal, BotOppositeTeam, BotPointAreaNum,
    BotRememberLastOrderedTask, BotSameTeam, BotSetTeamStatus, BotTeam, ClientFromName, ClientName,
    ClientOnSameTeamFromName, EasyClientName, TeamPlayIsOn,
};
use ai_main::{
    bot_activategoal_s, bot_activategoal_t, bot_developer, bot_state_s, bot_state_t,
    bot_waypoint_s, bot_waypoint_t, floattime, BotAILoadMap, BotAISetup, BotAISetupClient,
    BotAIShutdown, BotAIShutdownClient, BotAIStartFrame, BotAI_Trace, BotEntityInfo,
    BotInterbreedEndMatch, BotTestAAS,
};
use ai_team::{
    BotGetTeamMateTaskPreference, BotSetTeamMateTaskPreference, BotVoiceChat, BotVoiceChatOnly,
};
use ai_variadic_h::{BotAI_BotInitialChat, BotAI_Print};
use be_aas_h::{aas_entityinfo_s, aas_entityinfo_t};
use be_ai_goal_h::{bot_goal_s, bot_goal_t};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, team_t, unnamed, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR,
    IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON,
    TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
};
use botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t};
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
use g_local_h::{
    bot_settings_s, bot_settings_t, clientConnected_t, clientPersistant_t, clientSession_t,
    gclient_s, gentity_s, gentity_t, level_locals_t, moverState_t, playerTeamStateState_t,
    playerTeamState_t, spectatorState_t, trap_AAS_AreaTravelTimeToGoalArea, trap_BotEnterChat,
    trap_BotFindMatch, trap_BotGetLevelItemGoal, trap_BotGoalName, trap_BotMatchVariable,
    trap_EA_Action, trap_EA_Command, trap_EA_SayTeam, trap_GetConfigstring, CON_CONNECTED,
    CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1, MOVER_POS1, MOVER_POS2,
    SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
};
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
use g_public_h::entityShared_t;
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
use libc;
use q_shared_h::{
    byte, cplane_s, cplane_t, entityState_s, entityState_t, fileHandle_t, playerState_s,
    playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t, usercmd_s, usercmd_t,
    vec3_t, vec_t, Com_sprintf, Q_stricmp, Q_strncpyz, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atof, memcpy, rand, sqrt, sscanf, strcpy, strlen, strncpy};

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
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
#[no_mangle]
pub static mut notleader: [libc::c_int; 64] = [0; 64];
#[no_mangle]
pub unsafe extern "C" fn BotMatchMessage(
    mut bs: *mut bot_state_t,
    mut message: *mut libc::c_char,
) -> libc::c_int {
    let mut match_0: bot_match_t = bot_match_s {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_s {
            offset: 0,
            length: 0,
        }; 8],
    };
    match_0.type_0 = 0i32;
    if 0 == trap_BotFindMatch(
        message,
        &mut match_0 as *mut bot_match_t as *mut libc::c_void,
        (2i32 | 4i32 | 256i32) as libc::c_ulong,
    ) {
        return qfalse as libc::c_int;
    }
    let mut current_block_33: u64;
    match match_0.type_0 {
        3 => {
            //someone calling for company
            current_block_33 = 7502529970979898288;
        }
        4 => {
            current_block_33 = 7502529970979898288;
        }
        5 => {
            BotMatch_DefendKeyArea(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        20 => {
            BotMatch_Camp(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        22 => {
            BotMatch_Patrol(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        7 => {
            BotMatch_GetFlag(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        6 => {
            BotMatch_RushBase(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        27 => {
            BotMatch_ReturnFlag(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        30 => {
            BotMatch_TaskPreference(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        300 => {
            BotMatch_CTF(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        24 => {
            BotMatch_GetItem(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        13 => {
            BotMatch_JoinSubteam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        14 => {
            BotMatch_LeaveSubteam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        29 => {
            BotMatch_WhichTeam(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        21 => {
            BotMatch_CheckPoint(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        15 => {
            trap_EA_SayTeam(
                (*bs).client,
                b"the part of my brain to create formations has been damaged\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            current_block_33 = 2631791190359682872;
        }
        16 => {
            trap_EA_SayTeam(
                (*bs).client,
                b"the part of my brain to create formations has been damaged\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            current_block_33 = 2631791190359682872;
        }
        17 => {
            BotMatch_FormationSpace(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        19 => {
            BotMatch_Dismiss(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        8 => {
            BotMatch_StartTeamLeaderShip(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        9 => {
            BotMatch_StopTeamLeaderShip(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        10 => {
            BotMatch_WhoIsTeamLeader(bs, &mut match_0);
            current_block_33 = 2631791190359682872;
        }
        12 => {
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
            BotAI_Print(
                1i32,
                b"unknown match type\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block_33 = 2631791190359682872;
        }
    }
    match current_block_33 {
        7502529970979898288 => {
            BotMatch_HelpAccompany(bs, &mut match_0);
        }
        _ => {}
    }
    return qtrue as libc::c_int;
}
/*
==================
BotMatch_Suicide
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_Suicide(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_EA_Command(
        (*bs).client,
        b"kill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientFromName(netname.as_mut_ptr());
    BotVoiceChat(
        bs,
        client,
        b"taunt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_EA_Action((*bs).client, 0x100000i32);
}
/*
==================
BotAddressedToBot
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotAddressedToBot(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) -> libc::c_int {
    let mut addressedto: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut botname: [libc::c_char; 128] = [0; 128];
    let mut client: libc::c_int = 0;
    let mut addresseematch: bot_match_t = bot_match_s {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_s {
            offset: 0,
            length: 0,
        }; 8],
    };
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientOnSameTeamFromName(bs, netname.as_mut_ptr());
    if client < 0i32 {
        return qfalse as libc::c_int;
    }
    if 0 != (*match_0).subtype & 2i32 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            2i32,
            addressedto.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        ClientName((*bs).client, botname.as_mut_ptr(), 128i32);
        while 0
            != trap_BotFindMatch(
                addressedto.as_mut_ptr(),
                &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
                32i32 as libc::c_ulong,
            )
        {
            if addresseematch.type_0 == 101i32 {
                return qtrue as libc::c_int;
            } else if addresseematch.type_0 == 102i32 {
                trap_BotMatchVariable(
                    &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
                    4i32,
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                );
                if 0 != strlen(name.as_mut_ptr()) {
                    if !stristr(botname.as_mut_ptr(), name.as_mut_ptr()).is_null() {
                        return qtrue as libc::c_int;
                    }
                    if !stristr((*bs).subteam.as_mut_ptr(), name.as_mut_ptr()).is_null() {
                        return qtrue as libc::c_int;
                    }
                }
                trap_BotMatchVariable(
                    &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
                    6i32,
                    addressedto.as_mut_ptr(),
                    256i32,
                );
            } else {
                trap_BotMatchVariable(
                    &mut addresseematch as *mut bot_match_t as *mut libc::c_void,
                    4i32,
                    name.as_mut_ptr(),
                    256i32,
                );
                if 0 != strlen(name.as_mut_ptr()) {
                    if !stristr(botname.as_mut_ptr(), name.as_mut_ptr()).is_null() {
                        return qtrue as libc::c_int;
                    }
                    if !stristr((*bs).subteam.as_mut_ptr(), name.as_mut_ptr()).is_null() {
                        return qtrue as libc::c_int;
                    }
                }
                break;
            }
        }
        return qfalse as libc::c_int;
    } else {
        let mut tellmatch: bot_match_t = bot_match_s {
            string: [0; 256],
            type_0: 0,
            subtype: 0,
            variables: [bot_matchvariable_s {
                offset: 0,
                length: 0,
            }; 8],
        };
        tellmatch.type_0 = 0i32;
        if 0 == trap_BotFindMatch(
            (*match_0).string.as_mut_ptr(),
            &mut tellmatch as *mut bot_match_t as *mut libc::c_void,
            128i32 as libc::c_ulong,
        ) || tellmatch.type_0 != 202i32
        {
            if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                > 1.0f64 as libc::c_float / (NumPlayersOnSameTeam(bs) - 1i32) as libc::c_float
            {
                return qfalse as libc::c_int;
            }
        }
    }
    return qtrue as libc::c_int;
}
/*
==================
NumPlayersOnSameTeam
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NumPlayersOnSameTeam(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    num = 0i32;
    i = 0i32;
    while i < level.maxclients {
        trap_GetConfigstring(32i32 + 256i32 + 256i32 + i, buf.as_mut_ptr(), 1024i32);
        if 0 != strlen(buf.as_mut_ptr()) {
            if 0 != BotSameTeam(bs, i + 1i32) {
                num += 1
            }
        }
        i += 1
    }
    return num;
}
//NOTE: eliza chats will catch this
//Com_sprintf(buf, sizeof(buf), "heya %s", netname);
//EA_Say(bs->client, buf);
#[no_mangle]
pub unsafe extern "C" fn BotMatch_NewLeader(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: libc::c_int = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
    if 0 == BotSameTeam(bs, client) {
        return;
    }
    Q_strncpyz(
        (*bs).teamleader.as_mut_ptr(),
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
}
/*
==================
FindClientByName
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FindClientByName(mut name: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0i32;
    while i < level.maxclients {
        ClientName(
            i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == Q_stricmp(buf.as_mut_ptr(), name) {
            return i;
        }
        i += 1
    }
    i = 0i32;
    while i < level.maxclients {
        ClientName(
            i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if !stristr(buf.as_mut_ptr(), name).is_null() {
            return i;
        }
        i += 1
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn BotMatch_EnterGame(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: libc::c_int = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
    if client >= 0i32 {
        notleader[client as usize] = qfalse as libc::c_int
    };
}
/*
==================
BotMatch_Kill
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_Kill(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut enemy: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4i32,
        enemy.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = FindEnemyByName(bs, enemy.as_mut_ptr());
    if client < 0i32 {
        BotAI_BotInitialChat(
            bs,
            b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            enemy.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2i32);
        return;
    }
    (*bs).teamgoal.entitynum = client;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 11i32;
    (*bs).teamgoal_time = floattime + 180i32 as libc::c_float;
    BotSetTeamStatus(bs);
}
/*
==================
FindEnemyByName
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FindEnemyByName(
    mut bs: *mut bot_state_t,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0i32;
    while i < level.maxclients {
        if !(0 != BotSameTeam(bs, i)) {
            ClientName(
                i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            if 0 == Q_stricmp(buf.as_mut_ptr(), name) {
                return i;
            }
        }
        i += 1
    }
    i = 0i32;
    while i < level.maxclients {
        if !(0 != BotSameTeam(bs, i)) {
            ClientName(
                i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            if !stristr(buf.as_mut_ptr(), name).is_null() {
                return i;
            }
        }
        i += 1
    }
    return -1i32;
}
/*
==================
BotMatch_LeadTheWay
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_LeadTheWay(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
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
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    if 0 != (*match_0).subtype & 2048i32 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            4i32,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = FindClientByName(teammate.as_mut_ptr());
        if client == (*bs).client {
            other = qfalse as libc::c_int
        } else if 0 == BotSameTeam(bs, client) {
            return;
        } else {
            other = qtrue as libc::c_int
        }
    } else {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = ClientFromName(netname.as_mut_ptr());
        other = qfalse as libc::c_int
    }
    if client < 0i32 {
        BotAI_BotInitialChat(
            bs,
            b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            netname.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, (*bs).client, 1i32);
        return;
    }
    (*bs).lead_teamgoal.entitynum = -1i32;
    BotEntityInfo(client, &mut entinfo);
    if 0 != entinfo.valid {
        areanum = BotPointAreaNum(entinfo.origin.as_mut_ptr());
        if 0 != areanum {
            (*bs).lead_teamgoal.entitynum = client;
            (*bs).lead_teamgoal.areanum = areanum;
            (*bs).lead_teamgoal.origin[0usize] = entinfo.origin[0usize];
            (*bs).lead_teamgoal.origin[1usize] = entinfo.origin[1usize];
            (*bs).lead_teamgoal.origin[2usize] = entinfo.origin[2usize];
            (*bs).lead_teamgoal.mins[0usize] = -8i32 as vec_t;
            (*bs).lead_teamgoal.mins[1usize] = -8i32 as vec_t;
            (*bs).lead_teamgoal.mins[2usize] = -8i32 as vec_t;
            (*bs).lead_teamgoal.maxs[0usize] = 8i32 as vec_t;
            (*bs).lead_teamgoal.maxs[1usize] = 8i32 as vec_t;
            (*bs).lead_teamgoal.maxs[2usize] = 8i32 as vec_t
        }
    }
    if (*bs).teamgoal.entitynum < 0i32 {
        if 0 != other {
            BotAI_BotInitialChat(
                bs,
                b"whereis\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                teammate.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs,
                b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        trap_BotEnterChat((*bs).cs, (*bs).client, 1i32);
        return;
    }
    (*bs).lead_teammate = client;
    (*bs).lead_time = floattime + 600i32 as libc::c_float;
    (*bs).leadvisible_time = 0i32 as libc::c_float;
    (*bs).leadmessage_time = -(floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float));
}
/*
==================
BotMatch_WhereAreYou
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_WhereAreYou(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut dist: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut bestitem: libc::c_int = 0;
    let mut redtt: libc::c_int = 0;
    let mut bluett: libc::c_int = 0;
    let mut client: libc::c_int = 0;
    let mut goal: bot_goal_t = bot_goal_s {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut nearbyitems: [*mut libc::c_char; 18] = [
        b"Shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Grenade Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Rocket Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Lightning Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"BFG10K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Quad Damage\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Regeneration\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Battle Suit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Invisibility\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Flight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Armor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Heavy Armor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Red Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Blue Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    bestitem = -1i32;
    bestdist = 999999i32 as libc::c_float;
    i = 0i32;
    while !nearbyitems[i as usize].is_null() {
        dist = BotNearestVisibleItem(bs, nearbyitems[i as usize], &mut goal);
        if dist < bestdist {
            bestdist = dist;
            bestitem = i
        }
        i += 1
    }
    if bestitem != -1i32 {
        if gametype == GT_CTF as libc::c_int {
            redtt = trap_AAS_AreaTravelTimeToGoalArea(
                (*bs).areanum,
                (*bs).origin.as_mut_ptr(),
                ctf_redflag.areanum,
                0x2i32
                    | 0x4i32
                    | 0x8i32
                    | 0x10i32
                    | 0x20i32
                    | 0x80i32
                    | 0x100i32
                    | 0x200i32
                    | 0x400i32
                    | 0x800i32
                    | 0x80000i32
                    | 0x100000i32
                    | 0x40000i32
                    | 0x1000000i32,
            );
            bluett = trap_AAS_AreaTravelTimeToGoalArea(
                (*bs).areanum,
                (*bs).origin.as_mut_ptr(),
                ctf_blueflag.areanum,
                0x2i32
                    | 0x4i32
                    | 0x8i32
                    | 0x10i32
                    | 0x20i32
                    | 0x80i32
                    | 0x100i32
                    | 0x200i32
                    | 0x400i32
                    | 0x800i32
                    | 0x80000i32
                    | 0x100000i32
                    | 0x40000i32
                    | 0x1000000i32,
            );
            if (redtt as libc::c_double) < (redtt + bluett) as libc::c_double * 0.4f64 {
                BotAI_BotInitialChat(
                    bs,
                    b"teamlocation\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    nearbyitems[bestitem as usize],
                    b"red\x00" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void,
                );
            } else if (bluett as libc::c_double) < (redtt + bluett) as libc::c_double * 0.4f64 {
                BotAI_BotInitialChat(
                    bs,
                    b"teamlocation\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    nearbyitems[bestitem as usize],
                    b"blue\x00" as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void,
                );
            } else {
                BotAI_BotInitialChat(
                    bs,
                    b"location\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    nearbyitems[bestitem as usize],
                    0 as *mut libc::c_void,
                );
            }
        } else {
            BotAI_BotInitialChat(
                bs,
                b"location\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                nearbyitems[bestitem as usize],
                0 as *mut libc::c_void,
            );
        }
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2i32);
    };
}
/*
==================
BotNearestVisibleItem
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotNearestVisibleItem(
    mut bs: *mut bot_state_t,
    mut itemname: *mut libc::c_char,
    mut goal: *mut bot_goal_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut tmpgoal: bot_goal_t = bot_goal_s {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    let mut dist: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut trace: bsp_trace_t = bsp_trace_s {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: bsp_surface_s {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    bestdist = 999999i32 as libc::c_float;
    i = -1i32;
    loop {
        i = trap_BotGetLevelItemGoal(
            i,
            itemname,
            &mut tmpgoal as *mut bot_goal_t as *mut libc::c_void,
        );
        trap_BotGoalName(
            tmpgoal.number,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        if !(Q_stricmp(itemname, name.as_mut_ptr()) != 0i32) {
            dir[0usize] = tmpgoal.origin[0usize] - (*bs).origin[0usize];
            dir[1usize] = tmpgoal.origin[1usize] - (*bs).origin[1usize];
            dir[2usize] = tmpgoal.origin[2usize] - (*bs).origin[2usize];
            dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
            if dist < bestdist {
                BotAI_Trace(
                    &mut trace,
                    (*bs).eye.as_mut_ptr(),
                    0 as *mut vec_t,
                    0 as *mut vec_t,
                    tmpgoal.origin.as_mut_ptr(),
                    (*bs).client,
                    1i32 | 0x10000i32,
                );
                if trace.fraction as libc::c_double >= 1.0f64 {
                    bestdist = dist;
                    memcpy(
                        goal as *mut libc::c_void,
                        &mut tmpgoal as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                    );
                }
            }
        }
        if !(i > 0i32) {
            break;
        }
    }
    return bestdist;
}
/*
==================
BotMatch_WhatIsMyCommand
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_WhatIsMyCommand(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) != 0i32 {
        return;
    }
    (*bs).forceorders = qtrue as libc::c_int;
}
/*
==================
BotMatch_WhatAreYouDoing
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_WhatAreYouDoing(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut goalname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    match (*bs).ltgtype {
        1 => {
            EasyClientName(
                (*bs).teammate,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"helping\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        2 => {
            EasyClientName(
                (*bs).teammate,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"accompanying\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        3 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"defending\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                goalname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        10 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"gettingitem\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                goalname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        11 => {
            ClientName(
                (*bs).teamgoal.entitynum,
                netname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"killing\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        7 | 8 => {
            BotAI_BotInitialChat(
                bs,
                b"camping\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        9 => {
            BotAI_BotInitialChat(
                bs,
                b"patrolling\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        4 => {
            BotAI_BotInitialChat(
                bs,
                b"capturingflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        5 => {
            BotAI_BotInitialChat(
                bs,
                b"rushingbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        6 => {
            BotAI_BotInitialChat(
                bs,
                b"returningflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
        _ => {
            BotAI_BotInitialChat(
                bs,
                b"roaming\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
        }
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientFromName(netname.as_mut_ptr());
    trap_BotEnterChat((*bs).cs, client, 2i32);
}
/*
==================
BotMatch_WhoIsTeamLeader
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_WhoIsTeamLeader(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
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
        trap_EA_SayTeam(
            (*bs).client,
            b"I\'m the team leader\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
/*
==================
BotMatch_StopTeamLeaderShip
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_StopTeamLeaderShip(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: libc::c_int = 0;
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    if 0 == TeamPlayIsOn() {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4i32,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != (*match_0).subtype & 128i32 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = FindClientByName(netname.as_mut_ptr())
    } else {
        client = FindClientByName(teammate.as_mut_ptr())
    }
    if client >= 0i32 {
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
        }
    };
}
/*
==================
BotMatch_StartTeamLeaderShip
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_StartTeamLeaderShip(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: libc::c_int = 0;
    let mut teammate: [libc::c_char; 256] = [0; 256];
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 != (*match_0).subtype & 128i32 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0i32,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        strncpy(
            (*bs).teamleader.as_mut_ptr(),
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong,
        );
        (*bs).teamleader[(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char
    } else {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            4i32,
            teammate.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = FindClientByName(teammate.as_mut_ptr());
        if client >= 0i32 {
            ClientName(
                client,
                (*bs).teamleader.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
        }
    };
}
/*
==================
BotMatch_Dismiss
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_Dismiss(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientFromName(netname.as_mut_ptr());
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
}
/*
==================
BotMatch_FormationSpace
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_FormationSpace(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut space: libc::c_float = 0.;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(match_0 as *mut libc::c_void, 5i32, buf.as_mut_ptr(), 256i32);
    if 0 != (*match_0).subtype & 8i32 {
        space = (0.3048f64 * 32i32 as libc::c_double * atof(buf.as_mut_ptr())) as libc::c_float
    } else {
        space = (32i32 as libc::c_double * atof(buf.as_mut_ptr())) as libc::c_float
    }
    if space < 48i32 as libc::c_float || space > 500i32 as libc::c_float {
        space = 100i32 as libc::c_float
    }
    (*bs).formation_dist = space;
}
/*
==================
BotMatch_CheckPoint
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_CheckPoint(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut areanum: libc::c_int = 0;
    let mut client: libc::c_int = 0;
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut position: vec3_t = [0.; 3];
    let mut cp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    if 0 == TeamPlayIsOn() {
        return;
    }
    trap_BotMatchVariable(match_0 as *mut libc::c_void, 5i32, buf.as_mut_ptr(), 256i32);
    position[2usize] = 0i32 as vec_t;
    position[1usize] = position[2usize];
    position[0usize] = position[1usize];
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientFromName(netname.as_mut_ptr());
    sscanf(
        buf.as_mut_ptr(),
        b"%f %f %f\x00" as *const u8 as *const libc::c_char,
        &mut position[0usize] as *mut vec_t,
        &mut position[1usize] as *mut vec_t,
        &mut position[2usize] as *mut vec_t,
    );
    position[2usize] = (position[2usize] as libc::c_double + 0.5f64) as vec_t;
    areanum = BotPointAreaNum(position.as_mut_ptr());
    if 0 == areanum {
        if 0 != BotAddressedToBot(bs, match_0) {
            BotAI_BotInitialChat(
                bs,
                b"checkpoint_invalid\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, client, 2i32);
        }
        return;
    }
    trap_BotMatchVariable(match_0 as *mut libc::c_void, 6i32, buf.as_mut_ptr(), 256i32);
    cp = BotFindWayPoint((*bs).checkpoints, buf.as_mut_ptr());
    if !cp.is_null() {
        if !(*cp).next.is_null() {
            (*(*cp).next).prev = (*cp).prev
        }
        if !(*cp).prev.is_null() {
            (*(*cp).prev).next = (*cp).next
        } else {
            (*bs).checkpoints = (*cp).next
        }
        (*cp).inuse = qfalse as libc::c_int
    }
    cp = BotCreateWayPoint(buf.as_mut_ptr(), position.as_mut_ptr(), areanum);
    (*cp).next = (*bs).checkpoints;
    if !(*bs).checkpoints.is_null() {
        (*(*bs).checkpoints).prev = cp
    }
    (*bs).checkpoints = cp;
    if 0 != BotAddressedToBot(bs, match_0) {
        Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            b"%1.0f %1.0f %1.0f\x00" as *const u8 as *const libc::c_char,
            (*cp).goal.origin[0usize] as libc::c_double,
            (*cp).goal.origin[1usize] as libc::c_double,
            (*cp).goal.origin[2usize] as libc::c_double,
        );
        BotAI_BotInitialChat(
            bs,
            b"checkpoint_confirm\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*cp).name.as_mut_ptr(),
            buf.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, client, 2i32);
    };
}
/*
==================
BotMatch_LeaveSubteam
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_WhichTeam(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    if 0 != strlen((*bs).subteam.as_mut_ptr()) {
        BotAI_BotInitialChat(
            bs,
            b"inteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*bs).subteam.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
    } else {
        BotAI_BotInitialChat(
            bs,
            b"noteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        );
    }
    trap_BotEnterChat((*bs).cs, (*bs).client, 1i32);
}
/*
==================
BotMatch_LeaveSubteam
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_LeaveSubteam(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    if 0 != strlen((*bs).subteam.as_mut_ptr()) {
        BotAI_BotInitialChat(
            bs,
            b"leftteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*bs).subteam.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2i32);
    }
    strcpy(
        (*bs).subteam.as_mut_ptr(),
        b"\x00" as *const u8 as *const libc::c_char,
    );
}
//DEBUG
/*
==================
BotMatch_JoinSubteam
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_JoinSubteam(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4i32,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    strncpy(
        (*bs).subteam.as_mut_ptr(),
        teammate.as_mut_ptr(),
        32i32 as libc::c_ulong,
    );
    (*bs).subteam[31usize] = '\u{0}' as i32 as libc::c_char;
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    BotAI_BotInitialChat(
        bs,
        b"joinedteam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        teammate.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    client = ClientFromName(netname.as_mut_ptr());
    trap_BotEnterChat((*bs).cs, client, 2i32);
}
//DEBUG
/*
==================
BotMatch_GetItem
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_GetItem(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut itemname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        3i32,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientOnSameTeamFromName(bs, netname.as_mut_ptr());
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 10i32;
    (*bs).teamgoal_time = floattime + 60i32 as libc::c_float;
    BotSetTeamStatus(bs);
}
/*
==================
BotGetMessageTeamGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGetMessageTeamGoal(
    mut bs: *mut bot_state_t,
    mut goalname: *mut libc::c_char,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
    let mut cp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    if 0 != BotGetItemTeamGoal(goalname, goal) {
        return qtrue as libc::c_int;
    }
    cp = BotFindWayPoint((*bs).checkpoints, goalname);
    if !cp.is_null() {
        memcpy(
            goal as *mut libc::c_void,
            &mut (*cp).goal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
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
    mut goalname: *mut libc::c_char,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if 0 == strlen(goalname) {
        return qfalse as libc::c_int;
    }
    i = -1i32;
    loop {
        i = trap_BotGetLevelItemGoal(i, goalname, goal as *mut libc::c_void);
        if i > 0i32 {
            //do NOT defend dropped items
            if !(0 != (*goal).flags & 4i32) {
                return qtrue as libc::c_int;
            }
        }
        if !(i > 0i32) {
            break;
        }
    }
    return qfalse as libc::c_int;
}
//DEBUG
/*
==================
BotMatch_CTF
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_CTF(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut flag: [libc::c_char; 128] = [0; 128];
    let mut netname: [libc::c_char; 36] = [0; 36];
    if gametype == GT_CTF as libc::c_int {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            1i32,
            flag.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        );
        if 0 != (*match_0).subtype & 4096i32 {
            if 0 == Q_stricmp(
                flag.as_mut_ptr(),
                b"red\x00" as *const u8 as *const libc::c_char,
            ) {
                (*bs).redflagstatus = 1i32;
                if BotTeam(bs) == TEAM_BLUE as libc::c_int {
                    trap_BotMatchVariable(
                        match_0 as *mut libc::c_void,
                        0i32,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    (*bs).flagcarrier = ClientFromName(netname.as_mut_ptr())
                }
            } else {
                (*bs).blueflagstatus = 1i32;
                if BotTeam(bs) == TEAM_RED as libc::c_int {
                    trap_BotMatchVariable(
                        match_0 as *mut libc::c_void,
                        0i32,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    (*bs).flagcarrier = ClientFromName(netname.as_mut_ptr())
                }
            }
            (*bs).flagstatuschanged = 1i32;
            (*bs).lastflagcapture_time = floattime
        } else if 0 != (*match_0).subtype & 8192i32 {
            (*bs).redflagstatus = 0i32;
            (*bs).blueflagstatus = 0i32;
            (*bs).flagcarrier = 0i32;
            (*bs).flagstatuschanged = 1i32
        } else if 0 != (*match_0).subtype & 16384i32 {
            if 0 == Q_stricmp(
                flag.as_mut_ptr(),
                b"red\x00" as *const u8 as *const libc::c_char,
            ) {
                (*bs).redflagstatus = 0i32
            } else {
                (*bs).blueflagstatus = 0i32
            }
            (*bs).flagstatuschanged = 1i32
        }
    };
}
//DEBUG
/*
==================
BotMatch_TaskPreference
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_TaskPreference(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut teammatename: [libc::c_char; 256] = [0; 256];
    let mut teammate: libc::c_int = 0;
    let mut preference: libc::c_int = 0;
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) != 0i32 {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    teammate = ClientFromName(teammatename.as_mut_ptr());
    if teammate < 0i32 {
        return;
    }
    preference = BotGetTeamMateTaskPreference(bs, teammate);
    match (*match_0).subtype {
        1 => {
            preference &= !2i32;
            preference |= 1i32
        }
        2 => {
            preference &= !1i32;
            preference |= 2i32
        }
        4 => preference &= !(2i32 | 1i32),
        _ => {}
    }
    BotSetTeamMateTaskPreference(bs, teammate, preference);
    EasyClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    BotAI_BotInitialChat(
        bs,
        b"keepinmind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        teammatename.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    trap_BotEnterChat((*bs).cs, teammate, 2i32);
    BotVoiceChatOnly(
        bs,
        teammate,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_EA_Action((*bs).client, 0x100000i32);
}
/*
==================
BotMatch_ReturnFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_ReturnFlag(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if gametype != GT_CTF as libc::c_int {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
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
BotMatch_RushBase
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_RushBase(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if gametype == GT_CTF as libc::c_int {
        if 0 == ctf_redflag.areanum || 0 == ctf_blueflag.areanum {
            return;
        }
    } else {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 5i32;
    (*bs).teamgoal_time = floattime + 120i32 as libc::c_float;
    (*bs).rushbaseaway_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
}
//DEBUG
/*
==================
BotMatch_GetFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_GetFlag(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if gametype == GT_CTF as libc::c_int {
        if 0 == ctf_redflag.areanum || 0 == ctf_blueflag.areanum {
            return;
        }
    } else {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
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
//DEBUG
/*
==================
BotMatch_Patrol
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_Patrol(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    if 0 == BotGetPatrolWaypoints(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 9i32;
    (*bs).teamgoal_time = BotGetTime(match_0);
    if 0. == (*bs).teamgoal_time {
        (*bs).teamgoal_time = floattime + 600i32 as libc::c_float
    }
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
/*
==================
BotGetTime
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGetTime(mut match_0: *mut bot_match_t) -> libc::c_float {
    let mut timematch: bot_match_t = bot_match_s {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_s {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut timestring: [libc::c_char; 256] = [0; 256];
    let mut t: libc::c_float = 0.;
    if 0 != (*match_0).subtype & 16i32 {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            6i32,
            timestring.as_mut_ptr(),
            256i32,
        );
        if 0 != trap_BotFindMatch(
            timestring.as_mut_ptr(),
            &mut timematch as *mut bot_match_t as *mut libc::c_void,
            8i32 as libc::c_ulong,
        ) {
            if timematch.type_0 == 107i32 {
                t = 99999999.0f32
            } else if timematch.type_0 == 109i32 {
                t = (10i32 * 60i32) as libc::c_float
            } else if timematch.type_0 == 108i32 {
                t = (30i32 * 60i32) as libc::c_float
            } else {
                trap_BotMatchVariable(
                    &mut timematch as *mut bot_match_t as *mut libc::c_void,
                    6i32,
                    timestring.as_mut_ptr(),
                    256i32,
                );
                if timematch.type_0 == 105i32 {
                    t = (atof(timestring.as_mut_ptr()) * 60i32 as libc::c_double) as libc::c_float
                } else if timematch.type_0 == 106i32 {
                    t = atof(timestring.as_mut_ptr()) as libc::c_float
                } else {
                    t = 0i32 as libc::c_float
                }
            }
            if t > 0i32 as libc::c_float {
                return floattime + t;
            }
        }
    }
    return 0i32 as libc::c_float;
}
/*
==================
TeamPlayIsOn
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGetPatrolWaypoints(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) -> libc::c_int {
    let mut keyarea: [libc::c_char; 256] = [0; 256];
    let mut patrolflags: libc::c_int = 0;
    let mut wp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    let mut newwp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    let mut newpatrolpoints: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    let mut keyareamatch: bot_match_t = bot_match_s {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_s {
            offset: 0,
            length: 0,
        }; 8],
    };
    let mut goal: bot_goal_t = bot_goal_s {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    newpatrolpoints = 0 as *mut bot_waypoint_t;
    patrolflags = 0i32;
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5i32,
        keyarea.as_mut_ptr(),
        256i32,
    );
    loop {
        if 0 == trap_BotFindMatch(
            keyarea.as_mut_ptr(),
            &mut keyareamatch as *mut bot_match_t as *mut libc::c_void,
            64i32 as libc::c_ulong,
        ) {
            trap_EA_SayTeam(
                (*bs).client,
                b"what do you say?\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            BotFreeWaypoints(newpatrolpoints);
            (*bs).patrolpoints = 0 as *mut bot_waypoint_t;
            return qfalse as libc::c_int;
        }
        trap_BotMatchVariable(
            &mut keyareamatch as *mut bot_match_t as *mut libc::c_void,
            5i32,
            keyarea.as_mut_ptr(),
            256i32,
        );
        if 0 == BotGetMessageTeamGoal(bs, keyarea.as_mut_ptr(), &mut goal) {
            BotFreeWaypoints(newpatrolpoints);
            (*bs).patrolpoints = 0 as *mut bot_waypoint_t;
            return qfalse as libc::c_int;
        }
        newwp = BotCreateWayPoint(keyarea.as_mut_ptr(), goal.origin.as_mut_ptr(), goal.areanum);
        if newwp.is_null() {
            break;
        }
        (*newwp).next = 0 as *mut bot_waypoint_s;
        wp = newpatrolpoints;
        while !wp.is_null() && !(*wp).next.is_null() {
            wp = (*wp).next
        }
        if wp.is_null() {
            newpatrolpoints = newwp;
            (*newwp).prev = 0 as *mut bot_waypoint_s
        } else {
            (*wp).next = newwp;
            (*newwp).prev = wp
        }
        //
        if 0 != keyareamatch.subtype & 512i32 {
            patrolflags = 1i32;
            break;
        } else if 0 != keyareamatch.subtype & 1024i32 {
            patrolflags = 2i32;
            break;
        } else {
            if !(0 != keyareamatch.subtype & 256i32) {
                break;
            }
            trap_BotMatchVariable(
                &mut keyareamatch as *mut bot_match_t as *mut libc::c_void,
                6i32,
                keyarea.as_mut_ptr(),
                256i32,
            );
        }
    }
    if newpatrolpoints.is_null() || (*newpatrolpoints).next.is_null() {
        trap_EA_SayTeam(
            (*bs).client,
            b"I need more key points to patrol\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        BotFreeWaypoints(newpatrolpoints);
        newpatrolpoints = 0 as *mut bot_waypoint_t;
        return qfalse as libc::c_int;
    }
    BotFreeWaypoints((*bs).patrolpoints);
    (*bs).patrolpoints = newpatrolpoints;
    (*bs).curpatrolpoint = (*bs).patrolpoints;
    (*bs).patrolflags = patrolflags;
    return qtrue as libc::c_int;
}
//DEBUG
/*
==================
BotMatch_Camp
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_Camp(mut bs: *mut bot_state_t, mut match_0: *mut bot_match_t) {
    let mut client: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut itemname: [libc::c_char; 256] = [0; 256];
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
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
    if client < 0i32 {
        BotAI_BotInitialChat(
            bs,
            b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            netname.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, (*bs).client, 1i32);
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5i32,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != (*match_0).subtype & 64i32 {
        (*bs).teamgoal.entitynum = (*bs).entitynum;
        (*bs).teamgoal.areanum = (*bs).areanum;
        (*bs).teamgoal.origin[0usize] = (*bs).origin[0usize];
        (*bs).teamgoal.origin[1usize] = (*bs).origin[1usize];
        (*bs).teamgoal.origin[2usize] = (*bs).origin[2usize];
        (*bs).teamgoal.mins[0usize] = -8i32 as vec_t;
        (*bs).teamgoal.mins[1usize] = -8i32 as vec_t;
        (*bs).teamgoal.mins[2usize] = -8i32 as vec_t;
        (*bs).teamgoal.maxs[0usize] = 8i32 as vec_t;
        (*bs).teamgoal.maxs[1usize] = 8i32 as vec_t;
        (*bs).teamgoal.maxs[2usize] = 8i32 as vec_t
    } else if 0 != (*match_0).subtype & 32i32 {
        if client == (*bs).client {
            return;
        }
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
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            client = ClientFromName(netname.as_mut_ptr());
            trap_BotEnterChat((*bs).cs, client, 2i32);
            return;
        }
    } else if 0 == BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) {
        return;
    }
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 8i32;
    (*bs).teamgoal_time = BotGetTime(match_0);
    if 0. == (*bs).teamgoal_time {
        (*bs).teamgoal_time = floattime + 600i32 as libc::c_float
    }
    (*bs).arrive_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
//DEBUG
/*
==================
BotMatch_DefendKeyArea
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_DefendKeyArea(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut itemname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        5i32,
        itemname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientFromName(netname.as_mut_ptr());
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).ltgtype = 3i32;
    (*bs).teamgoal_time = BotGetTime(match_0);
    if 0. == (*bs).teamgoal_time {
        (*bs).teamgoal_time = floattime + 600i32 as libc::c_float
    }
    (*bs).defendaway_time = 0i32 as libc::c_float;
    BotSetTeamStatus(bs);
    BotRememberLastOrderedTask(bs);
}
/*
==================
BotMatch_HelpAccompany
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_HelpAccompany(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut client: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut areanum: libc::c_int = 0;
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut itemname: [libc::c_char; 256] = [0; 256];
    let mut teammatematch: bot_match_t = bot_match_s {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_s {
            offset: 0,
            length: 0,
        }; 8],
    };
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
    if 0 == TeamPlayIsOn() {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        4i32,
        teammate.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != trap_BotFindMatch(
        teammate.as_mut_ptr(),
        &mut teammatematch as *mut bot_match_t as *mut libc::c_void,
        16i32 as libc::c_ulong,
    ) && teammatematch.type_0 == 100i32
    {
        trap_BotMatchVariable(
            match_0 as *mut libc::c_void,
            0i32,
            netname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        client = ClientFromName(netname.as_mut_ptr());
        other = qfalse as libc::c_int
    } else {
        client = FindClientByName(teammate.as_mut_ptr());
        if client == (*bs).client {
            other = qfalse as libc::c_int
        } else if 0 == BotSameTeam(bs, client) {
            return;
        } else {
            other = qtrue as libc::c_int
        }
    }
    if client < 0i32 {
        if 0 != other {
            BotAI_BotInitialChat(
                bs,
                b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                teammate.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs,
                b"whois\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        client = ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 2i32);
        return;
    }
    if client == (*bs).client {
        return;
    }
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
        if 0 != (*match_0).subtype & 1i32 {
            trap_BotMatchVariable(
                match_0 as *mut libc::c_void,
                3i32,
                itemname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            if 0 == BotGetMessageTeamGoal(bs, itemname.as_mut_ptr(), &mut (*bs).teamgoal) {
                return;
            }
        }
    }
    if (*bs).teamgoal.entitynum < 0i32 {
        if 0 != other {
            BotAI_BotInitialChat(
                bs,
                b"whereis\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                teammate.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs,
                b"whereareyou\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                netname.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        client = ClientFromName(netname.as_mut_ptr());
        trap_BotEnterChat((*bs).cs, client, 1i32);
        return;
    }
    (*bs).teammate = client;
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = ClientFromName(netname.as_mut_ptr());
    (*bs).decisionmaker = client;
    (*bs).ordered = qtrue as libc::c_int;
    (*bs).order_time = floattime;
    (*bs).teammatevisible_time = floattime;
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    (*bs).teamgoal_time = BotGetTime(match_0);
    if (*match_0).type_0 == 3i32 {
        (*bs).ltgtype = 1i32;
        if 0. == (*bs).teamgoal_time {
            (*bs).teamgoal_time = floattime + 60i32 as libc::c_float
        }
    } else {
        (*bs).ltgtype = 2i32;
        if 0. == (*bs).teamgoal_time {
            (*bs).teamgoal_time = floattime + 600i32 as libc::c_float
        }
        (*bs).formation_dist = (3.5f64 * 32i32 as libc::c_double) as libc::c_float;
        (*bs).arrive_time = 0i32 as libc::c_float;
        BotSetTeamStatus(bs);
        BotRememberLastOrderedTask(bs);
    };
}
/*
==================
BotGPSToPosition
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGPSToPosition(
    mut buf: *mut libc::c_char,
    mut position: *mut vec_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0i32;
    let mut num: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        num = 0i32;
        while *buf.offset(j as isize) as libc::c_int == ' ' as i32 {
            j += 1
        }
        if *buf.offset(j as isize) as libc::c_int == '-' as i32 {
            j += 1;
            sign = -1i32
        } else {
            sign = 1i32
        }
        while 0 != *buf.offset(j as isize) {
            if *buf.offset(j as isize) as libc::c_int >= '0' as i32
                && *buf.offset(j as isize) as libc::c_int <= '9' as i32
            {
                num = num * 10i32 + *buf.offset(j as isize) as libc::c_int - '0' as i32;
                j += 1
            } else {
                j += 1;
                break;
            }
        }
        BotAI_Print(
            1i32,
            b"%d\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            sign * num,
        );
        *position.offset(i as isize) = sign as libc::c_float * num as libc::c_float;
        i += 1
    }
    return qtrue as libc::c_int;
}
//DEBUG
/*
==================
BotMatch_AttackEnemyBase
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMatch_AttackEnemyBase(
    mut bs: *mut bot_state_t,
    mut match_0: *mut bot_match_t,
) {
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut client: libc::c_int = 0;
    if gametype == GT_CTF as libc::c_int {
        BotMatch_GetFlag(bs, match_0);
    } else {
        return;
    }
    if 0 == BotAddressedToBot(bs, match_0) {
        return;
    }
    trap_BotMatchVariable(
        match_0 as *mut libc::c_void,
        0i32,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    client = FindClientByName(netname.as_mut_ptr());
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
