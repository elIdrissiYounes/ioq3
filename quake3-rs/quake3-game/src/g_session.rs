#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           libc,
           ptr_wrapping_offset_from)]
use ai_main::{
    bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown, BotAIShutdownClient,
    BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
};
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
    BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
    BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, team_t, unnamed, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR,
    IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON,
    TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
use g_local_h::{
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gclient_t, gentity_s,
    gentity_t, level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t,
    spectatorState_t, trap_Cvar_Set, trap_Cvar_VariableStringBuffer, CON_CONNECTED, CON_CONNECTING,
    CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1, MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW,
    SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
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
use g_variadic_h::G_Printf;
use g_weapon::{
    CheckGauntletAttack, FireWeapon, LogAccuracyHit, SnapVectorTowards, Weapon_HookFree,
    Weapon_HookThink,
};
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, Info_ValueForKey, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, sscanf};
extern crate libc;

//
// g_session.c
//
#[no_mangle]
pub unsafe extern "C" fn G_ReadSessionData(mut client: *mut gclient_t) {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    let mut teamLeader: libc::c_int = 0;
    let mut spectatorState: libc::c_int = 0;
    let mut sessionTeam: libc::c_int = 0;
    var = va(
        b"session%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int,
    );
    trap_Cvar_VariableStringBuffer(
        var,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    sscanf(
        s.as_mut_ptr(),
        b"%i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char,
        &mut sessionTeam as *mut libc::c_int,
        &mut (*client).sess.spectatorNum as *mut libc::c_int,
        &mut spectatorState as *mut libc::c_int,
        &mut (*client).sess.spectatorClient as *mut libc::c_int,
        &mut (*client).sess.wins as *mut libc::c_int,
        &mut (*client).sess.losses as *mut libc::c_int,
        &mut teamLeader as *mut libc::c_int,
    );
    (*client).sess.sessionTeam = sessionTeam as team_t;
    (*client).sess.spectatorState = spectatorState as spectatorState_t;
    (*client).sess.teamLeader = teamLeader as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn G_InitSessionData(
    mut client: *mut gclient_t,
    mut userinfo: *mut libc::c_char,
) {
    let mut sess: *mut clientSession_t = 0 as *mut clientSession_t;
    let mut value: *const libc::c_char = 0 as *const libc::c_char;
    sess = &mut (*client).sess;
    value = Info_ValueForKey(
        userinfo,
        b"teampref\x00" as *const u8 as *const libc::c_char,
    );
    if 0 == *value.offset(0isize)
        && 0 != g_localTeamPref.string[0usize] as libc::c_int
        && 0 != (*client).pers.localClient as libc::c_uint
    {
        value = g_localTeamPref.string.as_mut_ptr();
        trap_Cvar_Set(
            b"g_localTeamPref\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
        );
    }
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        (*sess).sessionTeam = TEAM_SPECTATOR;
        (*sess).spectatorState = SPECTATOR_FREE;
        if 0 != *value.offset(0isize) as libc::c_int || 0 != g_teamAutoJoin.integer {
            SetTeam(
                &mut *g_entities
                    .as_mut_ptr()
                    .offset(client.wrapping_offset_from(level.clients) as libc::c_long as isize),
                value,
            );
        }
    } else {
        if *value.offset(0isize) as libc::c_int == 's' as i32 {
            (*sess).sessionTeam = TEAM_SPECTATOR
        } else {
            match g_gametype.integer {
                1 => {
                    if level.numNonSpectatorClients >= 2i32 {
                        (*sess).sessionTeam = TEAM_SPECTATOR
                    } else {
                        (*sess).sessionTeam = TEAM_FREE
                    }
                }
                0 | 2 | _ => {
                    if g_maxGameClients.integer > 0i32
                        && level.numNonSpectatorClients >= g_maxGameClients.integer
                    {
                        (*sess).sessionTeam = TEAM_SPECTATOR
                    } else {
                        (*sess).sessionTeam = TEAM_FREE
                    }
                }
            }
        }
        (*sess).spectatorState = SPECTATOR_FREE
    }
    AddTournamentQueue(client);
    G_WriteClientSessionData(client);
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
/*
=======================================================================

  SESSION DATA

Session data is the only data that stays persistant across level loads
and tournament restarts.
=======================================================================
*/
/*
================
G_WriteClientSessionData

Called on game shutdown
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_WriteClientSessionData(mut client: *mut gclient_t) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    s = va(
        b"%i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*client).sess.sessionTeam as libc::c_uint,
        (*client).sess.spectatorNum,
        (*client).sess.spectatorState as libc::c_uint,
        (*client).sess.spectatorClient,
        (*client).sess.wins,
        (*client).sess.losses,
        (*client).sess.teamLeader as libc::c_uint,
    );
    var = va(
        b"session%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int,
    );
    trap_Cvar_Set(var, s);
}
#[no_mangle]
pub unsafe extern "C" fn G_InitWorldSession() {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut gt: libc::c_int = 0;
    trap_Cvar_VariableStringBuffer(
        b"session\x00" as *const u8 as *const libc::c_char,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    gt = atoi(s.as_mut_ptr());
    if g_gametype.integer != gt {
        level.newSession = qtrue;
        G_Printf(
            b"Gametype changed, clearing session data.\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn G_WriteSessionData() {
    let mut i: libc::c_int = 0;
    trap_Cvar_Set(
        b"session\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            g_gametype.integer,
        ),
    );
    i = 0i32;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).pers.connected as libc::c_uint
            == CON_CONNECTED as libc::c_int as libc::c_uint
        {
            G_WriteClientSessionData(&mut *level.clients.offset(i as isize));
        }
        i += 1
    }
}
