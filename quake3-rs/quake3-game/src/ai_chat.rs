use ai_dmq3::{
    bot_fastchat, bot_nochat, gametype, BotEntityVisible, BotIsDead, BotIsObserver, BotSameTeam,
    ClientName, EasyClientName, EntityIsDead, EntityIsInvisible, EntityIsShooting, TeamPlayIsOn,
};
use ai_main::{
    bot_activategoal_s, bot_activategoal_t, bot_developer, bot_state_s, bot_state_t,
    bot_waypoint_s, bot_waypoint_t, floattime, BotAILoadMap, BotAISetup, BotAISetupClient,
    BotAIShutdown, BotAIShutdownClient, BotAIStartFrame, BotAI_GetClientState, BotAI_Trace,
    BotEntityInfo, BotInterbreedEndMatch, BotTestAAS,
};
use ai_variadic_h::BotAI_BotInitialChat;
use be_aas_h::{aas_entityinfo_s, aas_entityinfo_t};
use be_ai_goal_h::{bot_goal_s, bot_goal_t};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, unnamed_1, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO,
    IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM,
    IT_WEAPON, MOD_BFG, MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE,
    MOD_GRENADE, MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA,
    MOD_PLASMA_SPLASH, MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME,
    MOD_SUICIDE, MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER,
    PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT,
    PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED,
    PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, TEAM_BLUE, TEAM_FREE,
    TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
    playerTeamState_t, spectatorState_t, trap_AAS_PresenceTypeBoundingBox, trap_BotEnterChat,
    trap_BotNumInitialChats, trap_Characteristic_BFloat, trap_GetConfigstring, trap_GetServerinfo,
    trap_PointContents, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1,
    MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD,
    TEAM_ACTIVE, TEAM_BEGIN,
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
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    usercmd_s, usercmd_t, vec3_t, vec_t, vmCvar_t, Info_ValueForKey, TR_GRAVITY, TR_INTERPOLATE,
    TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, rand, strcpy, strlen, strncpy};

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
 * name:		ai_chat.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_EnterGame(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        27i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    if 0 == BotValidChatPosition(bs) {
        return qfalse as libc::c_int;
    }
    BotAI_BotInitialChat(
        bs,
        b"game_enter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
        BotRandomOpponentName(bs),
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        BotMapTitle(),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
/*
==================
BotMapTitle
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotMapTitle() -> *mut libc::c_char {
    let mut info: [libc::c_char; 1024] = [0; 1024];
    static mut mapname: [libc::c_char; 128] = [0; 128];
    trap_GetServerinfo(
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    strncpy(
        mapname.as_mut_ptr(),
        Info_ValueForKey(
            info.as_mut_ptr(),
            b"mapname\x00" as *const u8 as *const libc::c_char,
        ),
        (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    mapname[(::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
    return mapname.as_mut_ptr();
}
/*
==================
BotRandomOpponentName
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotRandomOpponentName(mut bs: *mut bot_state_t) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut opponents: [libc::c_int; 64] = [0; 64];
    let mut numopponents: libc::c_int = 0;
    static mut name: [libc::c_char; 32] = [0; 32];
    numopponents = 0i32;
    opponents[0usize] = 0i32;
    i = 0i32;
    while i < level.maxclients {
        if !(i == (*bs).client) {
            trap_GetConfigstring(
                32i32 + 256i32 + 256i32 + i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            //if no config string or no name
            if !(0 == strlen(buf.as_mut_ptr())
                || 0 == strlen(Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"n\x00" as *const u8 as *const libc::c_char,
                )))
            {
                //skip spectators
                if !(atoi(Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const libc::c_char,
                )) == TEAM_SPECTATOR as libc::c_int)
                {
                    //skip team mates
                    if !(0 != BotSameTeam(bs, i)) {
                        opponents[numopponents as usize] = i;
                        numopponents += 1
                    }
                }
            }
        }
        i += 1
    }
    count = ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
        * numopponents as libc::c_float) as libc::c_int;
    i = 0i32;
    while i < numopponents {
        count -= 1;
        if count <= 0i32 {
            EasyClientName(
                opponents[i as usize],
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            );
            return name.as_mut_ptr();
        }
        i += 1
    }
    EasyClientName(
        opponents[0usize],
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    return name.as_mut_ptr();
}
// returns true if the bot can chat at the current position
#[no_mangle]
pub unsafe extern "C" fn BotValidChatPosition(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut point: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
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
    if 0 != BotIsDead(bs) as u64 {
        return qtrue as libc::c_int;
    }
    if 0 != (*bs).inventory[35usize]
        || 0 != (*bs).inventory[36usize]
        || 0 != (*bs).inventory[37usize]
        || 0 != (*bs).inventory[38usize]
        || 0 != (*bs).inventory[39usize]
        || 0 != (*bs).inventory[40usize]
    {
        return qfalse as libc::c_int;
    }
    point[0usize] = (*bs).origin[0usize];
    point[1usize] = (*bs).origin[1usize];
    point[2usize] = (*bs).origin[2usize];
    point[2usize] -= 24i32 as libc::c_float;
    if 0 != trap_PointContents(point.as_mut_ptr() as *const vec_t, (*bs).entitynum) & (8i32 | 16i32)
    {
        return qfalse as libc::c_int;
    }
    point[0usize] = (*bs).origin[0usize];
    point[1usize] = (*bs).origin[1usize];
    point[2usize] = (*bs).origin[2usize];
    point[2usize] += 32i32 as libc::c_float;
    if 0 != trap_PointContents(point.as_mut_ptr() as *const vec_t, (*bs).entitynum)
        & (32i32 | 8i32 | 16i32)
    {
        return qfalse as libc::c_int;
    }
    start[0usize] = (*bs).origin[0usize];
    start[1usize] = (*bs).origin[1usize];
    start[2usize] = (*bs).origin[2usize];
    end[0usize] = (*bs).origin[0usize];
    end[1usize] = (*bs).origin[1usize];
    end[2usize] = (*bs).origin[2usize];
    start[2usize] += 1i32 as libc::c_float;
    end[2usize] -= 10i32 as libc::c_float;
    trap_AAS_PresenceTypeBoundingBox(4i32, mins.as_mut_ptr(), maxs.as_mut_ptr());
    BotAI_Trace(
        &mut trace,
        start.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*bs).client,
        1i32,
    );
    if trace.ent != (1i32 << 10i32) - 2i32 {
        return qfalse as libc::c_int;
    }
    return qtrue as libc::c_int;
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
 * name:		ai_chat.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_chat.c $
 *
 *****************************************************************************/
//
//
// for the voice chats
/*
==================
BotNumActivePlayers
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotNumActivePlayers() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    num = 0i32;
    i = 0i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32i32 + 256i32 + 256i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(0 == strlen(buf.as_mut_ptr())
            || 0 == strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )))
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as libc::c_int)
            {
                num += 1
            }
        }
        i += 1
    }
    return num;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_ExitGame(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        27i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    BotAI_BotInitialChat(
        bs,
        b"game_exit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
        BotRandomOpponentName(bs),
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        b"[invalid var]\x00" as *const u8 as *const libc::c_char,
        BotMapTitle(),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_StartLevel(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if 0 != BotIsObserver(bs) as u64 {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        26i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    BotAI_BotInitialChat(
        bs,
        b"level_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_EndLevel(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if 0 != BotIsObserver(bs) as u64 {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if 0 != TeamPlayIsOn() {
        return qtrue as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        26i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    if 0 != BotIsFirstInRankings(bs) {
        BotAI_BotInitialChat(
            bs,
            b"level_end_victory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
    } else if 0 != BotIsLastInRankings(bs) {
        BotAI_BotInitialChat(
            bs,
            b"level_end_lose\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
    } else {
        BotAI_BotInitialChat(
            bs,
            b"level_end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
    }
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
/*
==================
BotLastClientInRankings
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotLastClientInRankings() -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut worstscore: libc::c_int = 0;
    let mut bestclient: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    static mut name: [libc::c_char; 32] = [0; 32];
    let mut ps: playerState_t = playerState_s {
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
    worstscore = 999999i32;
    bestclient = 0i32;
    i = 0i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32i32 + 256i32 + 256i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(0 == strlen(buf.as_mut_ptr())
            || 0 == strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )))
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as libc::c_int)
            {
                if 0 != BotAI_GetClientState(i, &mut ps)
                    && ps.persistant[PERS_SCORE as libc::c_int as usize] < worstscore
                {
                    worstscore = ps.persistant[PERS_SCORE as libc::c_int as usize];
                    bestclient = i
                }
            }
        }
        i += 1
    }
    EasyClientName(bestclient, name.as_mut_ptr(), 32i32);
    return name.as_mut_ptr();
}
/*
==================
BotFirstClientInRankings
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotFirstClientInRankings() -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut bestscore: libc::c_int = 0;
    let mut bestclient: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    static mut name: [libc::c_char; 32] = [0; 32];
    let mut ps: playerState_t = playerState_s {
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
    bestscore = -999999i32;
    bestclient = 0i32;
    i = 0i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32i32 + 256i32 + 256i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(0 == strlen(buf.as_mut_ptr())
            || 0 == strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )))
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as libc::c_int)
            {
                if 0 != BotAI_GetClientState(i, &mut ps)
                    && ps.persistant[PERS_SCORE as libc::c_int as usize] > bestscore
                {
                    bestscore = ps.persistant[PERS_SCORE as libc::c_int as usize];
                    bestclient = i
                }
            }
        }
        i += 1
    }
    EasyClientName(bestclient, name.as_mut_ptr(), 32i32);
    return name.as_mut_ptr();
}
/*
==================
BotIsLastInRankings
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotIsLastInRankings(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ps: playerState_t = playerState_s {
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
    score = (*bs).cur_ps.persistant[PERS_SCORE as libc::c_int as usize];
    i = 0i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32i32 + 256i32 + 256i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(0 == strlen(buf.as_mut_ptr())
            || 0 == strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )))
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as libc::c_int)
            {
                if 0 != BotAI_GetClientState(i, &mut ps)
                    && score > ps.persistant[PERS_SCORE as libc::c_int as usize]
                {
                    return qfalse as libc::c_int;
                }
            }
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
/*
==================
BotIsFirstInRankings
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotIsFirstInRankings(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ps: playerState_t = playerState_s {
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
    score = (*bs).cur_ps.persistant[PERS_SCORE as libc::c_int as usize];
    i = 0i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32i32 + 256i32 + 256i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        //if no config string or no name
        if !(0 == strlen(buf.as_mut_ptr())
            || 0 == strlen(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            )))
        {
            //skip spectators
            if !(atoi(Info_ValueForKey(
                buf.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )) == TEAM_SPECTATOR as libc::c_int)
            {
                if 0 != BotAI_GetClientState(i, &mut ps)
                    && score < ps.persistant[PERS_SCORE as libc::c_int as usize]
                {
                    return qfalse as libc::c_int;
                }
            }
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_HitTalking(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lasthurt_client: libc::c_int = 0;
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    lasthurt_client = (*g_entities[(*bs).client as usize].client).lasthurt_client;
    if 0 == lasthurt_client {
        return qfalse as libc::c_int;
    }
    if lasthurt_client == (*bs).client {
        return qfalse as libc::c_int;
    }
    if lasthurt_client < 0i32 || lasthurt_client >= 64i32 {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        31i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    if 0 == bot_fastchat.integer {
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            > rnd as libc::c_double * 0.5f64
        {
            return qfalse as libc::c_int;
        }
    }
    if 0 == BotValidChatPosition(bs) {
        return qfalse as libc::c_int;
    }
    ClientName(
        (*g_entities[(*bs).client as usize].client).lasthurt_client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    weap = BotWeaponNameForMeansOfDeath((*g_entities[(*bs).client as usize].client).lasthurt_mod);
    BotAI_BotInitialChat(
        bs,
        b"hit_talking\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        weap,
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
/*
==================
BotWeaponNameForMeansOfDeath
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotWeaponNameForMeansOfDeath(mut mod_0: libc::c_int) -> *mut libc::c_char {
    match mod_0 {
        1 => return b"Shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"Gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => return b"Machinegun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 | 5 => {
            return b"Grenade Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        6 | 7 => {
            return b"Rocket Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        8 | 9 => return b"Plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        10 => return b"Railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        11 => return b"Lightning Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        12 | 13 => return b"BFG10K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        23 => return b"Grapple\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => {
            return b"[unknown weapon]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    };
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_HitNoDeath(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rnd: libc::c_float = 0.;
    let mut lasthurt_client: libc::c_int = 0;
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
    lasthurt_client = (*g_entities[(*bs).client as usize].client).lasthurt_client;
    if 0 == lasthurt_client {
        return qfalse as libc::c_int;
    }
    if lasthurt_client == (*bs).client {
        return qfalse as libc::c_int;
    }
    if lasthurt_client < 0i32 || lasthurt_client >= 64i32 {
        return qfalse as libc::c_int;
    }
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        32i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    if 0 == bot_fastchat.integer {
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            > rnd as libc::c_double * 0.5f64
        {
            return qfalse as libc::c_int;
        }
    }
    if 0 == BotValidChatPosition(bs) {
        return qfalse as libc::c_int;
    }
    if 0 != BotVisibleEnemies(bs) {
        return qfalse as libc::c_int;
    }
    BotEntityInfo((*bs).enemy, &mut entinfo);
    if 0 != EntityIsShooting(&mut entinfo) as u64 {
        return qfalse as libc::c_int;
    }
    ClientName(
        lasthurt_client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    weap = BotWeaponNameForMeansOfDeath((*g_entities[(*bs).client as usize].client).lasthurt_mod);
    BotAI_BotInitialChat(
        bs,
        b"hit_nodeath\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        weap,
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
/*
==================
BotVisibleEnemies
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotVisibleEnemies(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut vis: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
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
    i = 0i32;
    while i < 64i32 {
        if !(i == (*bs).client) {
            BotEntityInfo(i, &mut entinfo);
            //
            if !(0 == entinfo.valid) {
                //if the enemy isn't dead and the enemy isn't the bot self
                if !(0 != EntityIsDead(&mut entinfo) as libc::c_uint
                    || entinfo.number == (*bs).entitynum)
                {
                    //if the enemy is invisible and not shooting
                    if !(0 != EntityIsInvisible(&mut entinfo) as libc::c_uint
                        && 0 == EntityIsShooting(&mut entinfo) as u64)
                    {
                        //if on the same team
                        if !(0 != BotSameTeam(bs, i)) {
                            vis = BotEntityVisible(
                                (*bs).entitynum,
                                (*bs).eye.as_mut_ptr(),
                                (*bs).viewangles.as_mut_ptr(),
                                360i32 as libc::c_float,
                                i,
                            );
                            if vis > 0i32 as libc::c_float {
                                return qtrue as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_HitNoKill(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rnd: libc::c_float = 0.;
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
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        33i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    if 0 == bot_fastchat.integer {
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            > rnd as libc::c_double * 0.5f64
        {
            return qfalse as libc::c_int;
        }
    }
    if 0 == BotValidChatPosition(bs) {
        return qfalse as libc::c_int;
    }
    if 0 != BotVisibleEnemies(bs) {
        return qfalse as libc::c_int;
    }
    BotEntityInfo((*bs).enemy, &mut entinfo);
    if 0 != EntityIsShooting(&mut entinfo) as u64 {
        return qfalse as libc::c_int;
    }
    ClientName(
        (*bs).enemy,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    weap = BotWeaponNameForMeansOfDeath((*g_entities[(*bs).enemy as usize].client).lasthurt_mod);
    BotAI_BotInitialChat(
        bs,
        b"hit_nokill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        weap,
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_Death(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        29i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    if (*bs).lastkilledby >= 0i32 && (*bs).lastkilledby < 64i32 {
        EasyClientName((*bs).lastkilledby, name.as_mut_ptr(), 32i32);
    } else {
        strcpy(
            name.as_mut_ptr(),
            b"[world]\x00" as *const u8 as *const libc::c_char,
        );
    }
    if 0 != TeamPlayIsOn() && 0 != BotSameTeam(bs, (*bs).lastkilledby) {
        if (*bs).lastkilledby == (*bs).client {
            return qfalse as libc::c_int;
        }
        BotAI_BotInitialChat(
            bs,
            b"death_teammate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        (*bs).chatto = 1i32
    } else {
        if 0 != TeamPlayIsOn() {
            return qtrue as libc::c_int;
        }
        if (*bs).botdeathtype == MOD_WATER as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"death_drown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_SLIME as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"death_slime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_LAVA as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"death_lava\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_FALLING as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"death_cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if 0 != (*bs).botsuicide
            || (*bs).botdeathtype == MOD_CRUSH as libc::c_int
            || (*bs).botdeathtype == MOD_SUICIDE as libc::c_int
            || (*bs).botdeathtype == MOD_TARGET_LASER as libc::c_int
            || (*bs).botdeathtype == MOD_TRIGGER_HURT as libc::c_int
            || (*bs).botdeathtype == MOD_UNKNOWN as libc::c_int
        {
            BotAI_BotInitialChat(
                bs,
                b"death_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                BotRandomOpponentName(bs),
                0 as *mut libc::c_void,
            );
        } else if (*bs).botdeathtype == MOD_TELEFRAG as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"death_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if ((*bs).botdeathtype == MOD_GAUNTLET as libc::c_int
            || (*bs).botdeathtype == MOD_RAILGUN as libc::c_int
            || (*bs).botdeathtype == MOD_BFG as libc::c_int
            || (*bs).botdeathtype == MOD_BFG_SPLASH as libc::c_int)
            && (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double)
                < 0.5f64
        {
            if (*bs).botdeathtype == MOD_GAUNTLET as libc::c_int {
                BotAI_BotInitialChat(
                    bs,
                    b"death_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                    0 as *mut libc::c_void,
                );
            } else if (*bs).botdeathtype == MOD_RAILGUN as libc::c_int {
                BotAI_BotInitialChat(
                    bs,
                    b"death_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                    0 as *mut libc::c_void,
                );
            } else {
                BotAI_BotInitialChat(
                    bs,
                    b"death_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                    0 as *mut libc::c_void,
                );
            }
        } else if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
            < trap_Characteristic_BFloat(
                (*bs).character,
                24i32,
                0i32 as libc::c_float,
                1i32 as libc::c_float,
            )
        {
            BotAI_BotInitialChat(
                bs,
                b"death_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs,
                b"death_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
                0 as *mut libc::c_void,
            );
        }
        (*bs).chatto = 0i32
    }
    (*bs).lastchat_time = floattime;
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_Kill(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        28i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
    }
    if (*bs).lastkilledplayer == (*bs).client {
        return qfalse as libc::c_int;
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    if 0 == BotValidChatPosition(bs) {
        return qfalse as libc::c_int;
    }
    if 0 != BotVisibleEnemies(bs) {
        return qfalse as libc::c_int;
    }
    EasyClientName((*bs).lastkilledplayer, name.as_mut_ptr(), 32i32);
    (*bs).chatto = 0i32;
    if 0 != TeamPlayIsOn() && 0 != BotSameTeam(bs, (*bs).lastkilledplayer) {
        BotAI_BotInitialChat(
            bs,
            b"kill_teammate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        (*bs).chatto = 1i32
    } else {
        if 0 != TeamPlayIsOn() {
            return qfalse as libc::c_int;
        }
        if (*bs).enemydeathtype == MOD_GAUNTLET as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"kill_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if (*bs).enemydeathtype == MOD_RAILGUN as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"kill_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if (*bs).enemydeathtype == MOD_TELEFRAG as libc::c_int {
            BotAI_BotInitialChat(
                bs,
                b"kill_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
            < trap_Characteristic_BFloat(
                (*bs).character,
                24i32,
                0i32 as libc::c_float,
                1i32 as libc::c_float,
            )
        {
            BotAI_BotInitialChat(
                bs,
                b"kill_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs,
                b"kill_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
    }
    (*bs).lastchat_time = floattime;
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_EnemySuicide(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut rnd: libc::c_float = 0.;
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        30i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
    }
    if 0 == BotValidChatPosition(bs) {
        return qfalse as libc::c_int;
    }
    if 0 != BotVisibleEnemies(bs) {
        return qfalse as libc::c_int;
    }
    if (*bs).enemy >= 0i32 {
        EasyClientName((*bs).enemy, name.as_mut_ptr(), 32i32);
    } else {
        strcpy(
            name.as_mut_ptr(),
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    BotAI_BotInitialChat(
        bs,
        b"enemy_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotChat_Random(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut rnd: libc::c_float = 0.;
    let mut name: [libc::c_char; 32] = [0; 32];
    if 0 != bot_nochat.integer {
        return qfalse as libc::c_int;
    }
    if 0 != BotIsObserver(bs) as u64 {
        return qfalse as libc::c_int;
    }
    if (*bs).lastchat_time > floattime - 25i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if gametype == GT_TOURNAMENT as libc::c_int {
        return qfalse as libc::c_int;
    }
    if (*bs).ltgtype == 1i32 || (*bs).ltgtype == 2i32 || (*bs).ltgtype == 5i32 {
        return qfalse as libc::c_int;
    }
    rnd = trap_Characteristic_BFloat(
        (*bs).character,
        34i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
        > (*bs).thinktime as libc::c_double * 0.1f64
    {
        return qfalse as libc::c_int;
    }
    if 0 == bot_fastchat.integer {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > rnd {
            return qfalse as libc::c_int;
        }
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            > 0.25f64
        {
            return qfalse as libc::c_int;
        }
    }
    if BotNumActivePlayers() <= 1i32 {
        return qfalse as libc::c_int;
    }
    if 0 == BotValidChatPosition(bs) {
        return qfalse as libc::c_int;
    }
    if 0 != BotVisibleEnemies(bs) {
        return qfalse as libc::c_int;
    }
    if (*bs).lastkilledplayer == (*bs).client {
        strcpy(name.as_mut_ptr(), BotRandomOpponentName(bs));
    } else {
        EasyClientName(
            (*bs).lastkilledplayer,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
    }
    if 0 != TeamPlayIsOn() {
        return qfalse as libc::c_int;
    }
    if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
        < trap_Characteristic_BFloat(
            (*bs).character,
            25i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    {
        BotAI_BotInitialChat(
            bs,
            b"random_misc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
    } else {
        BotAI_BotInitialChat(
            bs,
            b"random_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
    }
    (*bs).lastchat_time = floattime;
    (*bs).chatto = 0i32;
    return qtrue as libc::c_int;
}
/*
==================
BotRandomWeaponName
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotRandomWeaponName() -> *mut libc::c_char {
    let mut rnd: libc::c_int = 0;
    rnd = (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
        * 8.9f64) as libc::c_int;
    match rnd {
        0 => return b"Gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 => return b"Shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"Machinegun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => {
            return b"Grenade Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        4 => return b"Rocket Launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        5 => return b"Plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        6 => return b"Railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        7 => return b"Lightning Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => return b"BFG10K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    };
}
// time the selected chat takes to type in
#[no_mangle]
pub unsafe extern "C" fn BotChatTime(mut bs: *mut bot_state_t) -> libc::c_float {
    return 2.0f64 as libc::c_float;
}
// test the initial bot chats
#[no_mangle]
pub unsafe extern "C" fn BotChatTest(mut bs: *mut bot_state_t) {
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"game_enter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"game_enter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"game_exit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"game_exit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"level_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_end_victory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"level_end_victory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_end_lose\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"level_end_lose\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"level_end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"level_end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            EasyClientName((*bs).client, name.as_mut_ptr(), 32i32),
            BotRandomOpponentName(bs),
            BotFirstClientInRankings(),
            BotLastClientInRankings(),
            BotMapTitle(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    EasyClientName(
        (*bs).lastkilledby,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_drown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_drown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_slime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_slime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_lava\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_lava\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"death_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"death_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            BotWeaponNameForMeansOfDeath((*bs).botdeathtype),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    EasyClientName((*bs).lastkilledplayer, name.as_mut_ptr(), 32i32);
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"kill_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"kill_rail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"kill_telefrag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"kill_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"kill_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"kill_praise\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"enemy_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"enemy_suicide\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    ClientName(
        (*g_entities[(*bs).client as usize].client).lasthurt_client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    weap =
        BotWeaponNameForMeansOfDeath((*g_entities[(*bs).client as usize].client).lasthurt_client);
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"hit_talking\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"hit_talking\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            weap,
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"hit_nodeath\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"hit_nodeath\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            weap,
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"hit_nokill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"hit_nokill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            weap,
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    if (*bs).lastkilledplayer == (*bs).client {
        strcpy(name.as_mut_ptr(), BotRandomOpponentName(bs));
    } else {
        EasyClientName(
            (*bs).lastkilledplayer,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"random_misc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"random_misc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
    num = trap_BotNumInitialChats(
        (*bs).cs,
        b"random_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < num {
        BotAI_BotInitialChat(
            bs,
            b"random_insult\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            BotRandomOpponentName(bs),
            name.as_mut_ptr(),
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            b"[invalid var]\x00" as *const u8 as *const libc::c_char,
            BotMapTitle(),
            BotRandomWeaponName(),
            0 as *mut libc::c_void,
        );
        trap_BotEnterChat((*bs).cs, 0i32, 0i32);
        i += 1
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_matchvariable_s {
    pub offset: libc::c_char,
    pub length: libc::c_int,
}
pub type bot_consolemessage_t = bot_consolemessage_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_match_s {
    pub string: [libc::c_char; 256],
    pub type_0: libc::c_int,
    pub subtype: libc::c_int,
    pub variables: [bot_matchvariable_t; 8],
}
pub type bot_match_t = bot_match_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_consolemessage_s {
    pub handle: libc::c_int,
    pub time: libc::c_float,
    pub type_0: libc::c_int,
    pub message: [libc::c_char; 256],
    pub prev: *mut bot_consolemessage_s,
    pub next: *mut bot_consolemessage_s,
}
pub type bot_matchvariable_t = bot_matchvariable_s;
