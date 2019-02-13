use ai_chat::{BotChatTest, BotChat_ExitGame};
use ai_dmq3::{
    bot_challenge, bot_fastchat, bot_grapple, bot_nochat, bot_rocketjump, bot_testrchat, gametype,
    BotCTFCarryingFlag, BotClearActivateGoalStack, BotDeathmatchAI, BotFreeWaypoints,
    BotPointAreaNum, BotSetupDeathmatchAI, BotTeam, ClientFromName, ClientName, EasyClientName,
};
use ai_variadic_h::BotAI_Print;
use be_aas_h::{
    aas_areainfo_s, aas_areainfo_t, aas_entityinfo_s, aas_entityinfo_t, unnamed_3, SOLID_BBOX,
    SOLID_BSP, SOLID_NOT, SOLID_TRIGGER,
};
use be_ai_goal_h::{bot_goal_s, bot_goal_t};
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
    BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
    BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, team_t, unnamed_0, unnamed_1, unnamed_2, ET_BEAM, ET_EVENTS,
    ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL,
    ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO,
    IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM,
    IT_WEAPON, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, WP_BFG, WP_GAUNTLET,
    WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS,
    WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
use botlib_h::{
    bot_entitystate_s, bot_entitystate_t, bot_input_s, bot_input_t, bsp_surface_s, bsp_surface_t,
    bsp_trace_s, bsp_trace_t,
};
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
    playerTeamState_t, spectatorState_t, trap_AAS_AreaInfo, trap_AAS_EntityInfo,
    trap_AAS_Initialized, trap_AAS_Time, trap_BotAllocChatState, trap_BotAllocGoalState,
    trap_BotAllocMoveState, trap_BotAllocWeaponState, trap_BotEnterChat, trap_BotFreeCharacter,
    trap_BotFreeChatState, trap_BotFreeGoalState, trap_BotFreeMoveState, trap_BotFreeWeaponState,
    trap_BotGetServerCommand, trap_BotGetSnapshotEntity, trap_BotGetTopGoal, trap_BotGoalName,
    trap_BotInterbreedGoalFuzzyLogic, trap_BotLibLoadMap, trap_BotLibSetup, trap_BotLibShutdown,
    trap_BotLibStartFrame, trap_BotLibUpdateEntity, trap_BotLibVarSet, trap_BotLoadCharacter,
    trap_BotLoadChatFile, trap_BotLoadItemWeights, trap_BotLoadWeaponWeights,
    trap_BotMutateGoalFuzzyLogic, trap_BotQueueConsoleMessage, trap_BotResetAvoidGoals,
    trap_BotResetAvoidReach, trap_BotResetGoalState, trap_BotResetMoveState,
    trap_BotResetWeaponState, trap_BotSaveGoalFuzzyLogic, trap_BotSetChatGender,
    trap_BotUpdateEntityItems, trap_BotUserCommand, trap_Characteristic_BFloat,
    trap_Characteristic_String, trap_Cvar_Register, trap_Cvar_Set, trap_Cvar_Update,
    trap_Cvar_VariableIntegerValue, trap_Cvar_VariableStringBuffer, trap_EA_GetInput,
    trap_EA_ResetInput, trap_EA_SelectWeapon, trap_EA_View, trap_GeneticParentsAndChildSelection,
    trap_GetConfigstring, trap_SendConsoleCommand, trap_SetConfigstring, trap_Trace, CON_CONNECTED,
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
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    unnamed, usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, Com_sprintf, Info_ValueForKey,
    Q_IsColorString, Q_stricmp, EXEC_APPEND, EXEC_INSERT, EXEC_NOW, TR_GRAVITY, TR_INTERPOLATE,
    TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, fabs, memcpy, memmove, memset, sscanf, strchr, strcpy, strlen};

#[no_mangle]
pub unsafe extern "C" fn BotInterbreedEndMatch() {
    if 0 == bot_interbreed {
        return;
    }
    bot_interbreedmatchcount += 1;
    if bot_interbreedmatchcount >= bot_interbreedcycle.integer {
        bot_interbreedmatchcount = 0i32;
        trap_Cvar_Update(&mut bot_interbreedwrite);
        if 0 != strlen(bot_interbreedwrite.string.as_mut_ptr()) {
            BotWriteInterbreeded(bot_interbreedwrite.string.as_mut_ptr());
            trap_Cvar_Set(
                b"bot_interbreedwrite\x00" as *const u8 as *const libc::c_char,
                b"\x00" as *const u8 as *const libc::c_char,
            );
        }
        BotInterbreedBots();
    };
}
/*
==============
BotInterbreedBots
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotInterbreedBots() {
    let mut ranks: [libc::c_float; 64] = [0.; 64];
    let mut parent1: libc::c_int = 0;
    let mut parent2: libc::c_int = 0;
    let mut child: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 64i32 {
        if !botstates[i as usize].is_null() && 0 != (*botstates[i as usize]).inuse {
            ranks[i as usize] = ((*botstates[i as usize]).num_kills * 2i32
                - (*botstates[i as usize]).num_deaths)
                as libc::c_float
        } else {
            ranks[i as usize] = -1i32 as libc::c_float
        }
        i += 1
    }
    if 0 != trap_GeneticParentsAndChildSelection(
        64i32,
        ranks.as_mut_ptr(),
        &mut parent1,
        &mut parent2,
        &mut child,
    ) {
        trap_BotInterbreedGoalFuzzyLogic(
            (*botstates[parent1 as usize]).gs,
            (*botstates[parent2 as usize]).gs,
            (*botstates[child as usize]).gs,
        );
        trap_BotMutateGoalFuzzyLogic((*botstates[child as usize]).gs, 1i32 as libc::c_float);
    }
    i = 0i32;
    while i < 64i32 {
        if !botstates[i as usize].is_null() && 0 != (*botstates[i as usize]).inuse {
            (*botstates[i as usize]).num_kills = 0i32;
            (*botstates[i as usize]).num_deaths = 0i32
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
/* ****************************************************************************
 * name:		ai_main.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_main.c $
 *
 *****************************************************************************/
//
//
//bot states
#[no_mangle]
pub static mut botstates: [*mut bot_state_t; 64] =
    [0 as *const bot_state_t as *mut bot_state_t; 64];
#[no_mangle]
pub static mut bot_interbreedwrite: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
/*
==============
BotWriteInterbreeded
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotWriteInterbreeded(mut filename: *mut libc::c_char) {
    let mut rank: libc::c_float = 0.;
    let mut bestrank: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut bestbot: libc::c_int = 0;
    bestrank = 0i32 as libc::c_float;
    bestbot = -1i32;
    i = 0i32;
    while i < 64i32 {
        if !botstates[i as usize].is_null() && 0 != (*botstates[i as usize]).inuse {
            rank = ((*botstates[i as usize]).num_kills * 2i32 - (*botstates[i as usize]).num_deaths)
                as libc::c_float
        } else {
            rank = -1i32 as libc::c_float
        }
        if rank > bestrank {
            bestrank = rank;
            bestbot = i
        }
        i += 1
    }
    if bestbot >= 0i32 {
        trap_BotSaveGoalFuzzyLogic((*botstates[bestbot as usize]).gs, filename);
    };
}
#[no_mangle]
pub static mut bot_interbreedmatchcount: libc::c_int = 0;
#[no_mangle]
pub static mut bot_interbreedcycle: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//
#[no_mangle]
pub static mut bot_interbreed: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn BotAISetup(mut restart: libc::c_int) -> libc::c_int {
    let mut errnum: libc::c_int = 0;
    trap_Cvar_Register(
        &mut bot_thinktime,
        b"bot_thinktime\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_memorydump,
        b"bot_memorydump\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_saveroutingcache,
        b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_pause,
        b"bot_pause\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_report,
        b"bot_report\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_testsolid,
        b"bot_testsolid\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_testclusters,
        b"bot_testclusters\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_developer,
        b"bot_developer\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x200i32,
    );
    trap_Cvar_Register(
        &mut bot_interbreedchar,
        b"bot_interbreedchar\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_interbreedbots,
        b"bot_interbreedbots\x00" as *const u8 as *const libc::c_char,
        b"10\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_interbreedcycle,
        b"bot_interbreedcycle\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_interbreedwrite,
        b"bot_interbreedwrite\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    if 0 != restart {
        return qtrue as libc::c_int;
    }
    memset(
        botstates.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[*mut bot_state_t; 64]>() as libc::c_ulong,
    );
    errnum = BotInitLibrary();
    if errnum != 0i32 {
        return qfalse as libc::c_int;
    }
    return qtrue as libc::c_int;
}
/*
==============
BotInitLibrary
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotInitLibrary() -> libc::c_int {
    let mut buf: [libc::c_char; 144] = [0; 144];
    Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
        b"%d\x00" as *const u8 as *const libc::c_char,
        level.maxclients,
    );
    trap_BotLibVarSet(
        b"maxclients\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
        b"%d\x00" as *const u8 as *const libc::c_char,
        1i32 << 10i32,
    );
    trap_BotLibVarSet(
        b"maxentities\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    trap_Cvar_VariableStringBuffer(
        b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"max_aaslinks\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"max_aaslinks\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"max_levelitems\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"max_levelitems\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == strlen(buf.as_mut_ptr()) {
        strcpy(
            buf.as_mut_ptr(),
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    trap_BotLibVarSet(
        b"g_gametype\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    trap_BotLibVarSet(
        b"bot_developer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        bot_developer.string.as_mut_ptr(),
    );
    trap_Cvar_VariableStringBuffer(
        b"logfile\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    trap_BotLibVarSet(
        b"log\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    trap_Cvar_VariableStringBuffer(
        b"bot_nochat\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"nochat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"bot_visualizejumppads\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"bot_forceclustering\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"forceclustering\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"bot_forcereachability\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"forcereachability\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"bot_forcewrite\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"forcewrite\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"bot_aasoptimize\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"aasoptimize\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"saveroutingcache\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == strlen(buf.as_mut_ptr()) {
        strcpy(
            buf.as_mut_ptr(),
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    trap_BotLibVarSet(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr(),
    );
    trap_Cvar_VariableStringBuffer(
        b"fs_basepath\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"basedir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"fs_game\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"gamedir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    trap_Cvar_VariableStringBuffer(
        b"fs_homepath\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(buf.as_mut_ptr()) {
        trap_BotLibVarSet(
            b"homedir\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
        );
    }
    return trap_BotLibSetup();
}
#[no_mangle]
pub static mut bot_developer: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_interbreedbots: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_interbreedchar: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_testclusters: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_testsolid: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_report: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_pause: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_saveroutingcache: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_memorydump: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//
#[no_mangle]
pub static mut bot_thinktime: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub unsafe extern "C" fn BotAIShutdown(mut restart: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if 0 != restart {
        i = 0i32;
        while i < 64i32 {
            if !botstates[i as usize].is_null() && 0 != (*botstates[i as usize]).inuse {
                BotAIShutdownClient((*botstates[i as usize]).client, restart as qboolean);
            }
            i += 1
        }
    } else {
        trap_BotLibShutdown();
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BotAIShutdownClient(
    mut client: libc::c_int,
    mut restart: qboolean,
) -> libc::c_int {
    let mut bs: *mut bot_state_t = 0 as *mut bot_state_t;
    bs = botstates[client as usize];
    if bs.is_null() || 0 == (*bs).inuse {
        return qfalse as libc::c_int;
    }
    if 0 != restart as u64 {
        BotWriteSessionData(bs);
    }
    if 0 != BotChat_ExitGame(bs) {
        trap_BotEnterChat((*bs).cs, (*bs).client, 0i32);
    }
    trap_BotFreeMoveState((*bs).ms);
    trap_BotFreeGoalState((*bs).gs);
    trap_BotFreeChatState((*bs).cs);
    trap_BotFreeWeaponState((*bs).ws);
    trap_BotFreeCharacter((*bs).character);
    BotFreeWaypoints((*bs).checkpoints);
    BotFreeWaypoints((*bs).patrolpoints);
    BotClearActivateGoalStack(bs);
    memset(
        bs as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<bot_state_t>() as libc::c_ulong,
    );
    (*bs).inuse = qfalse as libc::c_int;
    numbots -= 1;
    return qtrue as libc::c_int;
}
//number of bots
#[no_mangle]
pub static mut numbots: libc::c_int = 0;
/*
==============
BotWriteSessionData
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotWriteSessionData(mut bs: *mut bot_state_t) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    s = va(
        b"%i %i %i %i %i %i %i %i %f %f %f %f %f %f %f %f %f %f\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        (*bs).lastgoal_decisionmaker,
        (*bs).lastgoal_ltgtype,
        (*bs).lastgoal_teammate,
        (*bs).lastgoal_teamgoal.areanum,
        (*bs).lastgoal_teamgoal.entitynum,
        (*bs).lastgoal_teamgoal.flags,
        (*bs).lastgoal_teamgoal.iteminfo,
        (*bs).lastgoal_teamgoal.number,
        (*bs).lastgoal_teamgoal.origin[0usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.origin[1usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.origin[2usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.mins[0usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.mins[1usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.mins[2usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.maxs[0usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.maxs[1usize] as libc::c_double,
        (*bs).lastgoal_teamgoal.maxs[2usize] as libc::c_double,
        (*bs).formation_dist as libc::c_double,
    );
    var = va(
        b"botsession%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*bs).client,
    );
    trap_Cvar_Set(var, s);
}
#[no_mangle]
pub unsafe extern "C" fn BotAILoadMap(mut restart: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut mapname: vmCvar_t = vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
    if 0 == restart {
        trap_Cvar_Register(
            &mut mapname,
            b"mapname\x00" as *const u8 as *const libc::c_char,
            b"\x00" as *const u8 as *const libc::c_char,
            0x4i32 | 0x40i32,
        );
        trap_BotLibLoadMap(mapname.string.as_mut_ptr());
    }
    i = 0i32;
    while i < 64i32 {
        if !botstates[i as usize].is_null() && 0 != (*botstates[i as usize]).inuse {
            BotResetState(botstates[i as usize]);
            (*botstates[i as usize]).setupcount = 4i32
        }
        i += 1
    }
    BotSetupDeathmatchAI();
    return qtrue as libc::c_int;
}
//resets the whole bot state
#[no_mangle]
pub unsafe extern "C" fn BotResetState(mut bs: *mut bot_state_t) {
    let mut client: libc::c_int = 0;
    let mut entitynum: libc::c_int = 0;
    let mut inuse: libc::c_int = 0;
    let mut movestate: libc::c_int = 0;
    let mut goalstate: libc::c_int = 0;
    let mut chatstate: libc::c_int = 0;
    let mut weaponstate: libc::c_int = 0;
    let mut settings: bot_settings_t = bot_settings_s {
        characterfile: [0; 144],
        skill: 0.,
    };
    let mut character: libc::c_int = 0;
    //current player state
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
    let mut entergame_time: libc::c_float = 0.;
    memcpy(
        &mut settings as *mut bot_settings_t as *mut libc::c_void,
        &mut (*bs).settings as *mut bot_settings_t as *const libc::c_void,
        ::std::mem::size_of::<bot_settings_t>() as libc::c_ulong,
    );
    memcpy(
        &mut ps as *mut playerState_t as *mut libc::c_void,
        &mut (*bs).cur_ps as *mut playerState_t as *const libc::c_void,
        ::std::mem::size_of::<playerState_t>() as libc::c_ulong,
    );
    inuse = (*bs).inuse;
    client = (*bs).client;
    entitynum = (*bs).entitynum;
    character = (*bs).character;
    movestate = (*bs).ms;
    goalstate = (*bs).gs;
    chatstate = (*bs).cs;
    weaponstate = (*bs).ws;
    entergame_time = (*bs).entergame_time;
    BotFreeWaypoints((*bs).checkpoints);
    BotFreeWaypoints((*bs).patrolpoints);
    memset(
        bs as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<bot_state_t>() as libc::c_ulong,
    );
    (*bs).ms = movestate;
    (*bs).gs = goalstate;
    (*bs).cs = chatstate;
    (*bs).ws = weaponstate;
    memcpy(
        &mut (*bs).cur_ps as *mut playerState_t as *mut libc::c_void,
        &mut ps as *mut playerState_t as *const libc::c_void,
        ::std::mem::size_of::<playerState_t>() as libc::c_ulong,
    );
    memcpy(
        &mut (*bs).settings as *mut bot_settings_t as *mut libc::c_void,
        &mut settings as *mut bot_settings_t as *const libc::c_void,
        ::std::mem::size_of::<bot_settings_t>() as libc::c_ulong,
    );
    (*bs).inuse = inuse;
    (*bs).client = client;
    (*bs).entitynum = entitynum;
    (*bs).character = character;
    (*bs).entergame_time = entergame_time;
    if 0 != (*bs).ms {
        trap_BotResetMoveState((*bs).ms);
    }
    if 0 != (*bs).gs {
        trap_BotResetGoalState((*bs).gs);
    }
    if 0 != (*bs).ws {
        trap_BotResetWeaponState((*bs).ws);
    }
    if 0 != (*bs).gs {
        trap_BotResetAvoidGoals((*bs).gs);
    }
    if 0 != (*bs).ms {
        trap_BotResetAvoidReach((*bs).ms);
    };
}
#[no_mangle]
pub unsafe extern "C" fn BotAISetupClient(
    mut client: libc::c_int,
    mut settings: *mut bot_settings_s,
    mut restart: qboolean,
) -> libc::c_int {
    let mut filename: [libc::c_char; 144] = [0; 144];
    let mut name: [libc::c_char; 144] = [0; 144];
    let mut gender: [libc::c_char; 144] = [0; 144];
    let mut bs: *mut bot_state_t = 0 as *mut bot_state_t;
    let mut errnum: libc::c_int = 0;
    if botstates[client as usize].is_null() {
        botstates[client as usize] =
            G_Alloc(::std::mem::size_of::<bot_state_t>() as libc::c_ulong as libc::c_int)
                as *mut bot_state_t
    }
    bs = botstates[client as usize];
    if bs.is_null() {
        return qfalse as libc::c_int;
    }
    if !bs.is_null() && 0 != (*bs).inuse {
        BotAI_Print(
            4i32,
            b"BotAISetupClient: client %d already setup\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            client,
        );
        return qfalse as libc::c_int;
    }
    if 0 == trap_AAS_Initialized() {
        BotAI_Print(
            4i32,
            b"AAS not initialized\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    (*bs).character =
        trap_BotLoadCharacter((*settings).characterfile.as_mut_ptr(), (*settings).skill);
    if 0 == (*bs).character {
        BotAI_Print(
            4i32,
            b"couldn\'t load skill %f from %s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*settings).skill as libc::c_double,
            (*settings).characterfile.as_mut_ptr(),
        );
        return qfalse as libc::c_int;
    }
    memcpy(
        &mut (*bs).settings as *mut bot_settings_t as *mut libc::c_void,
        settings as *const libc::c_void,
        ::std::mem::size_of::<bot_settings_t>() as libc::c_ulong,
    );
    (*bs).gs = trap_BotAllocGoalState(client);
    trap_Characteristic_String(
        (*bs).character,
        40i32,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    errnum = trap_BotLoadItemWeights((*bs).gs, filename.as_mut_ptr());
    if errnum != 0i32 {
        trap_BotFreeGoalState((*bs).gs);
        return qfalse as libc::c_int;
    }
    (*bs).ws = trap_BotAllocWeaponState();
    trap_Characteristic_String(
        (*bs).character,
        3i32,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    errnum = trap_BotLoadWeaponWeights((*bs).ws, filename.as_mut_ptr());
    if errnum != 0i32 {
        trap_BotFreeGoalState((*bs).gs);
        trap_BotFreeWeaponState((*bs).ws);
        return qfalse as libc::c_int;
    }
    (*bs).cs = trap_BotAllocChatState();
    trap_Characteristic_String(
        (*bs).character,
        21i32,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    trap_Characteristic_String(
        (*bs).character,
        22i32,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    errnum = trap_BotLoadChatFile((*bs).cs, filename.as_mut_ptr(), name.as_mut_ptr());
    if errnum != 0i32 {
        trap_BotFreeChatState((*bs).cs);
        trap_BotFreeGoalState((*bs).gs);
        trap_BotFreeWeaponState((*bs).ws);
        return qfalse as libc::c_int;
    }
    trap_Characteristic_String(
        (*bs).character,
        1i32,
        gender.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    if *gender.as_mut_ptr() as libc::c_int == 'f' as i32
        || *gender.as_mut_ptr() as libc::c_int == 'F' as i32
    {
        trap_BotSetChatGender((*bs).cs, 1i32);
    } else if *gender.as_mut_ptr() as libc::c_int == 'm' as i32
        || *gender.as_mut_ptr() as libc::c_int == 'M' as i32
    {
        trap_BotSetChatGender((*bs).cs, 2i32);
    } else {
        trap_BotSetChatGender((*bs).cs, 0i32);
    }
    (*bs).inuse = qtrue as libc::c_int;
    (*bs).client = client;
    (*bs).entitynum = client;
    (*bs).setupcount = 4i32;
    (*bs).entergame_time = floattime;
    (*bs).ms = trap_BotAllocMoveState();
    (*bs).walker = trap_Characteristic_BFloat(
        (*bs).character,
        48i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    numbots += 1;
    if 0 != trap_Cvar_VariableIntegerValue(b"bot_testichat\x00" as *const u8 as *const libc::c_char)
    {
        trap_BotLibVarSet(
            b"bot_testichat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        BotChatTest(bs);
    }
    BotScheduleBotThink();
    if 0 != bot_interbreed {
        trap_BotMutateGoalFuzzyLogic((*bs).gs, 1i32 as libc::c_float);
    }
    if 0 != restart as u64 {
        BotReadSessionData(bs);
    }
    return qtrue as libc::c_int;
}
/*
==============
BotReadSessionData
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotReadSessionData(mut bs: *mut bot_state_t) {
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut var: *const libc::c_char = 0 as *const libc::c_char;
    var = va(
        b"botsession%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*bs).client,
    );
    trap_Cvar_VariableStringBuffer(
        var,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    sscanf(
        s.as_mut_ptr(),
        b"%i %i %i %i %i %i %i %i %f %f %f %f %f %f %f %f %f %f\x00" as *const u8
            as *const libc::c_char,
        &mut (*bs).lastgoal_decisionmaker as *mut libc::c_int,
        &mut (*bs).lastgoal_ltgtype as *mut libc::c_int,
        &mut (*bs).lastgoal_teammate as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.areanum as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.entitynum as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.flags as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.iteminfo as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.number as *mut libc::c_int,
        &mut (*bs).lastgoal_teamgoal.origin[0usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.origin[1usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.origin[2usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.mins[0usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.mins[1usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.mins[2usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.maxs[0usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.maxs[1usize] as *mut vec_t,
        &mut (*bs).lastgoal_teamgoal.maxs[2usize] as *mut vec_t,
        &mut (*bs).formation_dist as *mut libc::c_float,
    );
}
/*
==================
BotScheduleBotThink
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotScheduleBotThink() {
    let mut i: libc::c_int = 0;
    let mut botnum: libc::c_int = 0;
    botnum = 0i32;
    i = 0i32;
    while i < 64i32 {
        if !(botstates[i as usize].is_null() || 0 == (*botstates[i as usize]).inuse) {
            (*botstates[i as usize]).botthink_residual = bot_thinktime.integer * botnum / numbots;
            botnum += 1
        }
        i += 1
    }
}
#[no_mangle]
pub static mut floattime: libc::c_float = 0.;
#[no_mangle]
pub unsafe extern "C" fn BotAIStartFrame(mut time: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut state: bot_entitystate_t = bot_entitystate_s {
        type_0: 0,
        flags: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
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
    let mut elapsed_time: libc::c_int = 0;
    let mut thinktime: libc::c_int = 0;
    static mut local_time: libc::c_int = 0;
    static mut botlib_residual: libc::c_int = 0;
    static mut lastbotthink_time: libc::c_int = 0;
    G_CheckBotSpawn();
    trap_Cvar_Update(&mut bot_rocketjump);
    trap_Cvar_Update(&mut bot_grapple);
    trap_Cvar_Update(&mut bot_fastchat);
    trap_Cvar_Update(&mut bot_nochat);
    trap_Cvar_Update(&mut bot_testrchat);
    trap_Cvar_Update(&mut bot_thinktime);
    trap_Cvar_Update(&mut bot_memorydump);
    trap_Cvar_Update(&mut bot_saveroutingcache);
    trap_Cvar_Update(&mut bot_pause);
    trap_Cvar_Update(&mut bot_report);
    if 0 != bot_report.integer {
        BotUpdateInfoConfigStrings();
    }
    if 0 != bot_pause.integer {
        i = 0i32;
        while i < 64i32 {
            if !(botstates[i as usize].is_null() || 0 == (*botstates[i as usize]).inuse) {
                if !((*g_entities[i as usize].client).pers.connected as libc::c_uint
                    != CON_CONNECTED as libc::c_int as libc::c_uint)
                {
                    (*botstates[i as usize]).lastucmd.forwardmove = 0i32 as libc::c_schar;
                    (*botstates[i as usize]).lastucmd.rightmove = 0i32 as libc::c_schar;
                    (*botstates[i as usize]).lastucmd.upmove = 0i32 as libc::c_schar;
                    (*botstates[i as usize]).lastucmd.buttons = 0i32;
                    (*botstates[i as usize]).lastucmd.serverTime = time;
                    trap_BotUserCommand(
                        (*botstates[i as usize]).client,
                        &mut (*botstates[i as usize]).lastucmd,
                    );
                }
            }
            i += 1
        }
        return qtrue as libc::c_int;
    }
    if 0 != bot_memorydump.integer {
        trap_BotLibVarSet(
            b"memorydump\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        trap_Cvar_Set(
            b"bot_memorydump\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    if 0 != bot_saveroutingcache.integer {
        trap_BotLibVarSet(
            b"saveroutingcache\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        trap_Cvar_Set(
            b"bot_saveroutingcache\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    }
    BotInterbreeding();
    if bot_thinktime.integer > 200i32 {
        trap_Cvar_Set(
            b"bot_thinktime\x00" as *const u8 as *const libc::c_char,
            b"200\x00" as *const u8 as *const libc::c_char,
        );
    }
    if bot_thinktime.integer != lastbotthink_time {
        lastbotthink_time = bot_thinktime.integer;
        BotScheduleBotThink();
    }
    elapsed_time = time - local_time;
    local_time = time;
    botlib_residual += elapsed_time;
    if elapsed_time > bot_thinktime.integer {
        thinktime = elapsed_time
    } else {
        thinktime = bot_thinktime.integer
    }
    if botlib_residual >= thinktime {
        botlib_residual -= thinktime;
        trap_BotLibStartFrame(time as libc::c_float / 1000i32 as libc::c_float);
        if 0 == trap_AAS_Initialized() {
            return qfalse as libc::c_int;
        }
        i = 0i32;
        while i < 1i32 << 10i32 {
            ent = &mut g_entities[i as usize] as *mut gentity_t;
            if 0 == (*ent).inuse as u64 {
                trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if 0 == (*ent).r.linked as u64 {
                trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if 0 != (*ent).r.svFlags & 0x1i32 {
                trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).s.eType == ET_MISSILE as libc::c_int
                && (*ent).s.weapon != WP_GRAPPLING_HOOK as libc::c_int
            {
                trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).s.eType > ET_EVENTS as libc::c_int {
                trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else {
                memset(
                    &mut state as *mut bot_entitystate_t as *mut libc::c_void,
                    0i32,
                    ::std::mem::size_of::<bot_entitystate_t>() as libc::c_ulong,
                );
                state.origin[0usize] = (*ent).r.currentOrigin[0usize];
                state.origin[1usize] = (*ent).r.currentOrigin[1usize];
                state.origin[2usize] = (*ent).r.currentOrigin[2usize];
                if i < 64i32 {
                    state.angles[0usize] = (*ent).s.apos.trBase[0usize];
                    state.angles[1usize] = (*ent).s.apos.trBase[1usize];
                    state.angles[2usize] = (*ent).s.apos.trBase[2usize]
                } else {
                    state.angles[0usize] = (*ent).r.currentAngles[0usize];
                    state.angles[1usize] = (*ent).r.currentAngles[1usize];
                    state.angles[2usize] = (*ent).r.currentAngles[2usize]
                }
                state.old_origin[0usize] = (*ent).s.origin2[0usize];
                state.old_origin[1usize] = (*ent).s.origin2[1usize];
                state.old_origin[2usize] = (*ent).s.origin2[2usize];
                state.mins[0usize] = (*ent).r.mins[0usize];
                state.mins[1usize] = (*ent).r.mins[1usize];
                state.mins[2usize] = (*ent).r.mins[2usize];
                state.maxs[0usize] = (*ent).r.maxs[0usize];
                state.maxs[1usize] = (*ent).r.maxs[1usize];
                state.maxs[2usize] = (*ent).r.maxs[2usize];
                state.type_0 = (*ent).s.eType;
                state.flags = (*ent).s.eFlags;
                if 0 != (*ent).r.bmodel as u64 {
                    state.solid = SOLID_BSP as libc::c_int
                } else {
                    state.solid = SOLID_BBOX as libc::c_int
                }
                state.groundent = (*ent).s.groundEntityNum;
                state.modelindex = (*ent).s.modelindex;
                state.modelindex2 = (*ent).s.modelindex2;
                state.frame = (*ent).s.frame;
                state.event = (*ent).s.event;
                state.eventParm = (*ent).s.eventParm;
                state.powerups = (*ent).s.powerups;
                state.legsAnim = (*ent).s.legsAnim;
                state.torsoAnim = (*ent).s.torsoAnim;
                state.weapon = (*ent).s.weapon;
                trap_BotLibUpdateEntity(
                    i,
                    &mut state as *mut bot_entitystate_t as *mut libc::c_void,
                );
            }
            i += 1
        }
        BotAIRegularUpdate();
    }
    floattime = trap_AAS_Time();
    i = 0i32;
    while i < 64i32 {
        if !(botstates[i as usize].is_null() || 0 == (*botstates[i as usize]).inuse) {
            (*botstates[i as usize]).botthink_residual += elapsed_time;
            if (*botstates[i as usize]).botthink_residual >= thinktime {
                (*botstates[i as usize]).botthink_residual -= thinktime;
                if 0 == trap_AAS_Initialized() {
                    return qfalse as libc::c_int;
                }
                if (*g_entities[i as usize].client).pers.connected as libc::c_uint
                    == CON_CONNECTED as libc::c_int as libc::c_uint
                {
                    BotAI(i, thinktime as libc::c_float / 1000i32 as libc::c_float);
                }
            }
        }
        i += 1
    }
    i = 0i32;
    while i < 64i32 {
        if !(botstates[i as usize].is_null() || 0 == (*botstates[i as usize]).inuse) {
            if !((*g_entities[i as usize].client).pers.connected as libc::c_uint
                != CON_CONNECTED as libc::c_int as libc::c_uint)
            {
                BotUpdateInput(botstates[i as usize], time, elapsed_time);
                trap_BotUserCommand(
                    (*botstates[i as usize]).client,
                    &mut (*botstates[i as usize]).lastucmd,
                );
            }
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
/*
==============
BotUpdateInput
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotUpdateInput(
    mut bs: *mut bot_state_t,
    mut time: libc::c_int,
    mut elapsed_time: libc::c_int,
) {
    let mut bi: bot_input_t = bot_input_s {
        thinktime: 0.,
        dir: [0.; 3],
        speed: 0.,
        viewangles: [0.; 3],
        actionflags: 0,
        weapon: 0,
    };
    let mut j: libc::c_int = 0;
    j = 0i32;
    while j < 3i32 {
        (*bs).viewangles[j as usize] = AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                + (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536i32 as libc::c_double)) as libc::c_float,
        );
        j += 1
    }
    BotChangeViewAngles(bs, elapsed_time as libc::c_float / 1000i32 as libc::c_float);
    trap_EA_GetInput(
        (*bs).client,
        time as libc::c_float / 1000i32 as libc::c_float,
        &mut bi as *mut bot_input_t as *mut libc::c_void,
    );
    if 0 != bi.actionflags & 0x8i32 {
        if 0 != (*bs).lastucmd.buttons & 1i32 {
            bi.actionflags &= !(0x8i32 | 0x1i32)
        }
    }
    BotInputToUserCommand(
        &mut bi,
        &mut (*bs).lastucmd,
        (*bs).cur_ps.delta_angles.as_mut_ptr(),
        time,
    );
    j = 0i32;
    while j < 3i32 {
        (*bs).viewangles[j as usize] = AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                - (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536i32 as libc::c_double)) as libc::c_float,
        );
        j += 1
    }
}
/*
==============
BotInputToUserCommand
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotInputToUserCommand(
    mut bi: *mut bot_input_t,
    mut ucmd: *mut usercmd_t,
    mut delta_angles: *mut libc::c_int,
    mut time: libc::c_int,
) {
    let mut angles: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut temp: libc::c_short = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut m: libc::c_float = 0.;
    memset(
        ucmd as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<usercmd_t>() as libc::c_ulong,
    );
    (*ucmd).serverTime = time;
    if 0 != (*bi).actionflags & 0x8000i32 {
        (*bi).actionflags |= 0x10i32;
        (*bi).actionflags &= !0x8000i32
    }
    if 0 != (*bi).actionflags & 0x8i32 {
        (*ucmd).buttons = 1i32
    }
    if 0 != (*bi).actionflags & 0x1i32 {
        (*ucmd).buttons |= 1i32
    }
    if 0 != (*bi).actionflags & 0x10000i32 {
        (*ucmd).buttons |= 2i32
    }
    if 0 != (*bi).actionflags & 0x20000i32 {
        (*ucmd).buttons |= 8i32
    }
    if 0 != (*bi).actionflags & 0x2i32 {
        (*ucmd).buttons |= 4i32
    }
    if 0 != (*bi).actionflags & 0x80000i32 {
        (*ucmd).buttons |= 16i32
    }
    if 0 != (*bi).actionflags & 0x100000i32 {
        (*ucmd).buttons |= 32i32
    }
    if 0 != (*bi).actionflags & 0x200000i32 {
        (*ucmd).buttons |= 64i32
    }
    if 0 != (*bi).actionflags & 0x800000i32 {
        (*ucmd).buttons |= 128i32
    }
    if 0 != (*bi).actionflags & 0x1000000i32 {
        (*ucmd).buttons |= 256i32
    }
    if 0 != (*bi).actionflags & 0x2000000i32 {
        (*ucmd).buttons |= 512i32
    }
    if 0 != (*bi).actionflags & 0x8000000i32 {
        (*ucmd).buttons |= 1024i32
    }
    (*ucmd).weapon = (*bi).weapon as byte;
    (*ucmd).angles[0usize] = ((*bi).viewangles[0usize] * 65536i32 as libc::c_float
        / 360i32 as libc::c_float) as libc::c_int
        & 65535i32;
    (*ucmd).angles[1usize] = ((*bi).viewangles[1usize] * 65536i32 as libc::c_float
        / 360i32 as libc::c_float) as libc::c_int
        & 65535i32;
    (*ucmd).angles[2usize] = ((*bi).viewangles[2usize] * 65536i32 as libc::c_float
        / 360i32 as libc::c_float) as libc::c_int
        & 65535i32;
    j = 0i32;
    while j < 3i32 {
        temp = ((*ucmd).angles[j as usize] - *delta_angles.offset(j as isize)) as libc::c_short;
        (*ucmd).angles[j as usize] = temp as libc::c_int;
        j += 1
    }
    if 0. != (*bi).dir[2usize] {
        angles[0usize] = (*bi).viewangles[0usize]
    } else {
        angles[0usize] = 0i32 as vec_t
    }
    angles[1usize] = (*bi).viewangles[1usize];
    angles[2usize] = 0i32 as vec_t;
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    (*bi).speed = (*bi).speed * 127i32 as libc::c_float / 400i32 as libc::c_float;
    f = forward[0usize] * (*bi).dir[0usize]
        + forward[1usize] * (*bi).dir[1usize]
        + forward[2usize] * (*bi).dir[2usize];
    r = right[0usize] * (*bi).dir[0usize]
        + right[1usize] * (*bi).dir[1usize]
        + right[2usize] * (*bi).dir[2usize];
    u = (fabs(forward[2usize] as libc::c_double) * (*bi).dir[2usize] as libc::c_double)
        as libc::c_float;
    m = fabs(f as libc::c_double) as libc::c_float;
    if fabs(r as libc::c_double) > m as libc::c_double {
        m = fabs(r as libc::c_double) as libc::c_float
    }
    if fabs(u as libc::c_double) > m as libc::c_double {
        m = fabs(u as libc::c_double) as libc::c_float
    }
    if m > 0i32 as libc::c_float {
        f *= (*bi).speed / m;
        r *= (*bi).speed / m;
        u *= (*bi).speed / m
    }
    (*ucmd).forwardmove = f as libc::c_schar;
    (*ucmd).rightmove = r as libc::c_schar;
    (*ucmd).upmove = u as libc::c_schar;
    if 0 != (*bi).actionflags & 0x200i32 {
        (*ucmd).forwardmove = 127i32 as libc::c_schar
    }
    if 0 != (*bi).actionflags & 0x800i32 {
        (*ucmd).forwardmove = -127i32 as libc::c_schar
    }
    if 0 != (*bi).actionflags & 0x1000i32 {
        (*ucmd).rightmove = -127i32 as libc::c_schar
    }
    if 0 != (*bi).actionflags & 0x2000i32 {
        (*ucmd).rightmove = 127i32 as libc::c_schar
    }
    if 0 != (*bi).actionflags & 0x10i32 {
        (*ucmd).upmove = 127i32 as libc::c_schar
    }
    if 0 != (*bi).actionflags & 0x80i32 {
        (*ucmd).upmove = -127i32 as libc::c_schar
    };
}
/*
==============
BotChangeViewAngles
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotChangeViewAngles(
    mut bs: *mut bot_state_t,
    mut thinktime: libc::c_float,
) {
    let mut diff: libc::c_float = 0.;
    let mut factor: libc::c_float = 0.;
    let mut maxchange: libc::c_float = 0.;
    let mut anglespeed: libc::c_float = 0.;
    let mut disired_speed: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if (*bs).ideal_viewangles[0usize] > 180i32 as libc::c_float {
        (*bs).ideal_viewangles[0usize] -= 360i32 as libc::c_float
    }
    if (*bs).enemy >= 0i32 {
        factor = trap_Characteristic_BFloat((*bs).character, 4i32, 0.01f32, 1i32 as libc::c_float);
        maxchange = trap_Characteristic_BFloat(
            (*bs).character,
            5i32,
            1i32 as libc::c_float,
            1800i32 as libc::c_float,
        )
    } else {
        factor = 0.05f32;
        maxchange = 360i32 as libc::c_float
    }
    if maxchange < 240i32 as libc::c_float {
        maxchange = 240i32 as libc::c_float
    }
    maxchange *= thinktime;
    i = 0i32;
    while i < 2i32 {
        if 0 != bot_challenge.integer {
            diff = fabs(AngleDifference(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
            ) as libc::c_double) as libc::c_float;
            anglespeed = diff * factor;
            if anglespeed > maxchange {
                anglespeed = maxchange
            }
            (*bs).viewangles[i as usize] = BotChangeViewAngle(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
                anglespeed,
            )
        } else {
            (*bs).viewangles[i as usize] = AngleMod((*bs).viewangles[i as usize]);
            (*bs).ideal_viewangles[i as usize] = AngleMod((*bs).ideal_viewangles[i as usize]);
            diff = AngleDifference(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
            );
            disired_speed = diff * factor;
            (*bs).viewanglespeed[i as usize] += (*bs).viewanglespeed[i as usize] - disired_speed;
            if (*bs).viewanglespeed[i as usize] > 180i32 as libc::c_float {
                (*bs).viewanglespeed[i as usize] = maxchange
            }
            if (*bs).viewanglespeed[i as usize] < -180i32 as libc::c_float {
                (*bs).viewanglespeed[i as usize] = -maxchange
            }
            anglespeed = (*bs).viewanglespeed[i as usize];
            if anglespeed > maxchange {
                anglespeed = maxchange
            }
            if anglespeed < -maxchange {
                anglespeed = -maxchange
            }
            (*bs).viewangles[i as usize] += anglespeed;
            (*bs).viewangles[i as usize] = AngleMod((*bs).viewangles[i as usize]);
            (*bs).viewanglespeed[i as usize] = ((*bs).viewanglespeed[i as usize] as libc::c_double
                * (0.45f64 * (1i32 as libc::c_float - factor) as libc::c_double))
                as vec_t
        }
        i += 1
    }
    if (*bs).viewangles[0usize] > 180i32 as libc::c_float {
        (*bs).viewangles[0usize] -= 360i32 as libc::c_float
    }
    trap_EA_View((*bs).client, (*bs).viewangles.as_mut_ptr());
}
/*
==============
AngleDifference
==============
*/
#[no_mangle]
pub unsafe extern "C" fn AngleDifference(
    mut ang1: libc::c_float,
    mut ang2: libc::c_float,
) -> libc::c_float {
    let mut diff: libc::c_float = 0.;
    diff = ang1 - ang2;
    if ang1 > ang2 {
        if diff as libc::c_double > 180.0f64 {
            diff = (diff as libc::c_double - 360.0f64) as libc::c_float
        }
    } else if (diff as libc::c_double) < -180.0f64 {
        diff = (diff as libc::c_double + 360.0f64) as libc::c_float
    }
    return diff;
}
/*
==============
BotChangeViewAngle
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotChangeViewAngle(
    mut angle: libc::c_float,
    mut ideal_angle: libc::c_float,
    mut speed: libc::c_float,
) -> libc::c_float {
    let mut move_0: libc::c_float = 0.;
    angle = AngleMod(angle);
    ideal_angle = AngleMod(ideal_angle);
    if angle == ideal_angle {
        return angle;
    }
    move_0 = ideal_angle - angle;
    if ideal_angle > angle {
        if move_0 as libc::c_double > 180.0f64 {
            move_0 = (move_0 as libc::c_double - 360.0f64) as libc::c_float
        }
    } else if (move_0 as libc::c_double) < -180.0f64 {
        move_0 = (move_0 as libc::c_double + 360.0f64) as libc::c_float
    }
    if move_0 > 0i32 as libc::c_float {
        if move_0 > speed {
            move_0 = speed
        }
    } else if move_0 < -speed {
        move_0 = -speed
    }
    return AngleMod(angle + move_0);
}
/*
==============
BotAI
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotAI(
    mut client: libc::c_int,
    mut thinktime: libc::c_float,
) -> libc::c_int {
    let mut bs: *mut bot_state_t = 0 as *mut bot_state_t;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut args: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    trap_EA_ResetInput(client);
    bs = botstates[client as usize];
    if bs.is_null() || 0 == (*bs).inuse {
        BotAI_Print(
            4i32,
            b"BotAI: client %d is not setup\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            client,
        );
        return qfalse as libc::c_int;
    }
    if 0 == BotAI_GetClientState(client, &mut (*bs).cur_ps) {
        BotAI_Print(
            4i32,
            b"BotAI: failed to get player state for player %d\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            client,
        );
        return qfalse as libc::c_int;
    }
    while 0
        != trap_BotGetServerCommand(
            client,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        )
    {
        args = strchr(buf.as_mut_ptr(), ' ' as i32);
        if args.is_null() {
            continue;
        }
        let fresh0 = args;
        args = args.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char;
        RemoveColorEscapeSequences(args);
        if !(0
            == Q_stricmp(
                buf.as_mut_ptr(),
                b"cp \x00" as *const u8 as *const libc::c_char,
            ))
        {
            if !(0
                == Q_stricmp(
                    buf.as_mut_ptr(),
                    b"cs\x00" as *const u8 as *const libc::c_char,
                ))
            {
                if 0 == Q_stricmp(
                    buf.as_mut_ptr(),
                    b"print\x00" as *const u8 as *const libc::c_char,
                ) {
                    memmove(
                        args as *mut libc::c_void,
                        args.offset(1isize) as *const libc::c_void,
                        strlen(args),
                    );
                    *args.offset(strlen(args).wrapping_sub(1i32 as libc::c_ulong) as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    trap_BotQueueConsoleMessage((*bs).cs, 0i32, args);
                } else if 0
                    == Q_stricmp(
                        buf.as_mut_ptr(),
                        b"chat\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    memmove(
                        args as *mut libc::c_void,
                        args.offset(1isize) as *const libc::c_void,
                        strlen(args),
                    );
                    *args.offset(strlen(args).wrapping_sub(1i32 as libc::c_ulong) as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    trap_BotQueueConsoleMessage((*bs).cs, 1i32, args);
                } else if 0
                    == Q_stricmp(
                        buf.as_mut_ptr(),
                        b"tchat\x00" as *const u8 as *const libc::c_char,
                    )
                {
                    memmove(
                        args as *mut libc::c_void,
                        args.offset(1isize) as *const libc::c_void,
                        strlen(args),
                    );
                    *args.offset(strlen(args).wrapping_sub(1i32 as libc::c_ulong) as isize) =
                        '\u{0}' as i32 as libc::c_char;
                    trap_BotQueueConsoleMessage((*bs).cs, 1i32, args);
                } else if !(0
                    == Q_stricmp(
                        buf.as_mut_ptr(),
                        b"scores\x00" as *const u8 as *const libc::c_char,
                    ))
                {
                    0 == Q_stricmp(
                        buf.as_mut_ptr(),
                        b"clientLevelShot\x00" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    j = 0i32;
    while j < 3i32 {
        (*bs).viewangles[j as usize] = AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                + (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536i32 as libc::c_double)) as libc::c_float,
        );
        j += 1
    }
    (*bs).ltime += thinktime;
    (*bs).thinktime = thinktime;
    (*bs).origin[0usize] = (*bs).cur_ps.origin[0usize];
    (*bs).origin[1usize] = (*bs).cur_ps.origin[1usize];
    (*bs).origin[2usize] = (*bs).cur_ps.origin[2usize];
    (*bs).eye[0usize] = (*bs).cur_ps.origin[0usize];
    (*bs).eye[1usize] = (*bs).cur_ps.origin[1usize];
    (*bs).eye[2usize] = (*bs).cur_ps.origin[2usize];
    (*bs).eye[2usize] += (*bs).cur_ps.viewheight as libc::c_float;
    (*bs).areanum = BotPointAreaNum((*bs).origin.as_mut_ptr());
    BotDeathmatchAI(bs, thinktime);
    trap_EA_SelectWeapon((*bs).client, (*bs).weaponnum);
    j = 0i32;
    while j < 3i32 {
        (*bs).viewangles[j as usize] = AngleMod(
            ((*bs).viewangles[j as usize] as libc::c_double
                - (*bs).cur_ps.delta_angles[j as usize] as libc::c_double
                    * (360.0f64 / 65536i32 as libc::c_double)) as libc::c_float,
        );
        j += 1
    }
    return qtrue as libc::c_int;
}
/*
==============
RemoveColorEscapeSequences
==============
*/
#[no_mangle]
pub unsafe extern "C" fn RemoveColorEscapeSequences(mut text: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = 0i32;
    i = 0i32;
    while 0 != *text.offset(i as isize) {
        if 0 != Q_IsColorString(&mut *text.offset(i as isize)) as u64 {
            i += 1
        } else if !(*text.offset(i as isize) as libc::c_int > 0x7ei32) {
            let fresh1 = l;
            l = l + 1;
            *text.offset(fresh1 as isize) = *text.offset(i as isize)
        }
        i += 1
    }
    *text.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn BotAI_GetClientState(
    mut clientNum: libc::c_int,
    mut state: *mut playerState_t,
) -> libc::c_int {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    ent = &mut g_entities[clientNum as usize] as *mut gentity_t;
    if 0 == (*ent).inuse as u64 {
        return qfalse as libc::c_int;
    }
    if (*ent).client.is_null() {
        return qfalse as libc::c_int;
    }
    memcpy(
        state as *mut libc::c_void,
        &mut (*(*ent).client).ps as *mut playerState_t as *const libc::c_void,
        ::std::mem::size_of::<playerState_t>() as libc::c_ulong,
    );
    return qtrue as libc::c_int;
}
/*
==============
BotAIRegularUpdate
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotAIRegularUpdate() {
    if regularupdate_time < floattime {
        trap_BotUpdateEntityItems();
        regularupdate_time = (floattime as libc::c_double + 0.3f64) as libc::c_float
    };
}
//floating point time
//time to do a regular update
#[no_mangle]
pub static mut regularupdate_time: libc::c_float = 0.;
/*
==============
BotInterbreeding
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotInterbreeding() {
    let mut i: libc::c_int = 0;
    trap_Cvar_Update(&mut bot_interbreedchar);
    if 0 == strlen(bot_interbreedchar.string.as_mut_ptr()) {
        return;
    }
    if gametype != GT_TOURNAMENT as libc::c_int {
        trap_Cvar_Set(
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
            va(
                b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                GT_TOURNAMENT as libc::c_int,
            ),
        );
        ExitLevel();
        return;
    }
    i = 0i32;
    while i < 64i32 {
        if !botstates[i as usize].is_null() && 0 != (*botstates[i as usize]).inuse {
            BotAIShutdownClient((*botstates[i as usize]).client, qfalse);
        }
        i += 1
    }
    trap_BotLibVarSet(
        b"bot_reloadcharacters\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < bot_interbreedbots.integer {
        trap_SendConsoleCommand(
            EXEC_INSERT as libc::c_int,
            va(
                b"addbot %s 4 free %i %s%d\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                bot_interbreedchar.string.as_mut_ptr(),
                i * 50i32,
                bot_interbreedchar.string.as_mut_ptr(),
                i,
            ),
        );
        i += 1
    }
    trap_Cvar_Set(
        b"bot_interbreedchar\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    bot_interbreed = qtrue as libc::c_int;
}
/*
==============
BotUpdateInfoConfigStrings
==============
*/
#[no_mangle]
pub unsafe extern "C" fn BotUpdateInfoConfigStrings() {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0i32;
    while i < level.maxclients {
        //
        if !(botstates[i as usize].is_null() || 0 == (*botstates[i as usize]).inuse) {
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
                BotSetInfoConfigString(botstates[i as usize]);
            }
        }
        i += 1
    }
}
/*
==================
BotSetInfoConfigString
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSetInfoConfigString(mut bs: *mut bot_state_t) {
    let mut goalname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut action: [libc::c_char; 256] = [0; 256];
    let mut leader: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut carrying: [libc::c_char; 32] = [0; 32];
    let mut cs: *mut libc::c_char = 0 as *mut libc::c_char;
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
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) == 0i32 {
        leader = b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        leader = b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    strcpy(
        carrying.as_mut_ptr(),
        b"  \x00" as *const u8 as *const libc::c_char,
    );
    if gametype == GT_CTF as libc::c_int {
        if 0 != BotCTFCarryingFlag(bs) {
            strcpy(
                carrying.as_mut_ptr(),
                b"F \x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    match (*bs).ltgtype {
        1 => {
            EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"helping %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        2 => {
            EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"accompanying %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        3 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"defending %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        10 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"getting item %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        11 => {
            ClientName(
                (*bs).teamgoal.entitynum,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"killing %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
        7 | 8 => {
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"camping\x00" as *const u8 as *const libc::c_char,
            );
        }
        9 => {
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"patrolling\x00" as *const u8 as *const libc::c_char,
            );
        }
        4 => {
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"capturing flag\x00" as *const u8 as *const libc::c_char,
            );
        }
        5 => {
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"rushing base\x00" as *const u8 as *const libc::c_char,
            );
        }
        6 => {
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"returning flag\x00" as *const u8 as *const libc::c_char,
            );
        }
        13 => {
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"attacking the enemy base\x00" as *const u8 as *const libc::c_char,
            );
        }
        12 => {
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"harvesting\x00" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void);
            trap_BotGoalName(
                goal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                b"roaming %s\x00" as *const u8 as *const libc::c_char,
                goalname.as_mut_ptr(),
            );
        }
    }
    cs = va(
        b"l\\%s\\c\\%s\\a\\%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        leader,
        carrying.as_mut_ptr(),
        action.as_mut_ptr(),
    );
    trap_SetConfigstring(25i32 + (*bs).client, cs);
}
#[no_mangle]
pub unsafe extern "C" fn BotTestAAS(mut origin: *mut vec_t) {
    let mut areanum: libc::c_int = 0;
    let mut info: aas_areainfo_t = aas_areainfo_s {
        contents: 0,
        flags: 0,
        presencetype: 0,
        cluster: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        center: [0.; 3],
    };
    trap_Cvar_Update(&mut bot_testsolid);
    trap_Cvar_Update(&mut bot_testclusters);
    if 0 != bot_testsolid.integer {
        if 0 == trap_AAS_Initialized() {
            return;
        }
        areanum = BotPointAreaNum(origin);
        if 0 != areanum {
            BotAI_Print(
                1i32,
                b"\rempty area\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            BotAI_Print(
                1i32,
                b"\r^1SOLID area\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    } else if 0 != bot_testclusters.integer {
        if 0 == trap_AAS_Initialized() {
            return;
        }
        areanum = BotPointAreaNum(origin);
        if 0 == areanum {
            BotAI_Print(
                1i32,
                b"\r^1Solid!                              \x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            trap_AAS_AreaInfo(
                areanum,
                &mut info as *mut aas_areainfo_t as *mut libc::c_void,
            );
            BotAI_Print(
                1i32,
                b"\rarea %d, cluster %d       \x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                areanum,
                info.cluster,
            );
        }
    };
}
//returns the number of bots in the game
#[no_mangle]
pub unsafe extern "C" fn NumBots() -> libc::c_int {
    return numbots;
}
//returns info about the entity
#[no_mangle]
pub unsafe extern "C" fn BotEntityInfo(mut entnum: libc::c_int, mut info: *mut aas_entityinfo_t) {
    trap_AAS_EntityInfo(entnum, info as *mut libc::c_void);
}
// from the game source
#[no_mangle]
pub unsafe extern "C" fn BotAI_Trace(
    mut bsptrace: *mut bsp_trace_t,
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
    mut passent: libc::c_int,
    mut contentmask: libc::c_int,
) {
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
    trap_Trace(
        &mut trace,
        start as *const vec_t,
        mins as *const vec_t,
        maxs as *const vec_t,
        end as *const vec_t,
        passent,
        contentmask,
    );
    (*bsptrace).allsolid = trace.allsolid;
    (*bsptrace).startsolid = trace.startsolid;
    (*bsptrace).fraction = trace.fraction;
    (*bsptrace).endpos[0usize] = trace.endpos[0usize];
    (*bsptrace).endpos[1usize] = trace.endpos[1usize];
    (*bsptrace).endpos[2usize] = trace.endpos[2usize];
    (*bsptrace).plane.dist = trace.plane.dist;
    (*bsptrace).plane.normal[0usize] = trace.plane.normal[0usize];
    (*bsptrace).plane.normal[1usize] = trace.plane.normal[1usize];
    (*bsptrace).plane.normal[2usize] = trace.plane.normal[2usize];
    (*bsptrace).plane.signbits = trace.plane.signbits;
    (*bsptrace).plane.type_0 = trace.plane.type_0;
    (*bsptrace).surface.value = 0i32;
    (*bsptrace).surface.flags = trace.surfaceFlags;
    (*bsptrace).ent = trace.entityNum;
    (*bsptrace).exp_dist = 0i32 as libc::c_float;
    (*bsptrace).sidenum = 0i32;
    (*bsptrace).contents = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn BotAI_GetEntityState(
    mut entityNum: libc::c_int,
    mut state: *mut entityState_t,
) -> libc::c_int {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    ent = &mut g_entities[entityNum as usize] as *mut gentity_t;
    memset(
        state as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<entityState_t>() as libc::c_ulong,
    );
    if 0 == (*ent).inuse as u64 {
        return qfalse as libc::c_int;
    }
    if 0 == (*ent).r.linked as u64 {
        return qfalse as libc::c_int;
    }
    if 0 != (*ent).r.svFlags & 0x1i32 {
        return qfalse as libc::c_int;
    }
    memcpy(
        state as *mut libc::c_void,
        &mut (*ent).s as *mut entityState_t as *const libc::c_void,
        ::std::mem::size_of::<entityState_t>() as libc::c_ulong,
    );
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BotAI_GetSnapshotEntity(
    mut clientNum: libc::c_int,
    mut sequence: libc::c_int,
    mut state: *mut entityState_t,
) -> libc::c_int {
    let mut entNum: libc::c_int = 0;
    entNum = trap_BotGetSnapshotEntity(clientNum, sequence);
    if entNum == -1i32 {
        memset(
            state as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<entityState_t>() as libc::c_ulong,
        );
        return -1i32;
    }
    BotAI_GetEntityState(entNum, state);
    return sequence + 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn BotTeamLeader(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut leader: libc::c_int = 0;
    leader = ClientFromName((*bs).teamleader.as_mut_ptr());
    if leader < 0i32 {
        return qfalse as libc::c_int;
    }
    if botstates[leader as usize].is_null() || 0 == (*botstates[leader as usize]).inuse {
        return qfalse as libc::c_int;
    }
    return qtrue as libc::c_int;
}
/*
==================
BotReportStatus
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotReportStatus(mut bs: *mut bot_state_t) {
    let mut goalname: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 256] = [0; 256];
    let mut leader: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flagstatus: [libc::c_char; 32] = [0; 32];
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) == 0i32 {
        leader = b"L\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        leader = b" \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    strcpy(
        flagstatus.as_mut_ptr(),
        b"  \x00" as *const u8 as *const libc::c_char,
    );
    if gametype == GT_CTF as libc::c_int {
        if 0 != BotCTFCarryingFlag(bs) {
            if BotTeam(bs) == TEAM_RED as libc::c_int {
                strcpy(
                    flagstatus.as_mut_ptr(),
                    b"^1F \x00" as *const u8 as *const libc::c_char,
                );
            } else {
                strcpy(
                    flagstatus.as_mut_ptr(),
                    b"^4F \x00" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    match (*bs).ltgtype {
        1 => {
            EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: helping %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        2 => {
            EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: accompanying %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        3 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: defending %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        10 => {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: getting item %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        11 => {
            ClientName(
                (*bs).teamgoal.entitynum,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: killing %s\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        7 | 8 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: camping\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        9 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: patrolling\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        4 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: capturing flag\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        5 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: rushing base\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        6 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: returning flag\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        13 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: attacking the enemy base\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        12 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: harvesting\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        _ => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: roaming\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
    };
}
/*
==================
BotTeamplayReport
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotTeamplayReport() {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    BotAI_Print(
        1i32,
        b"^1RED\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < level.maxclients {
        //
        if !(botstates[i as usize].is_null() || 0 == (*botstates[i as usize]).inuse) {
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
                if atoi(Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const libc::c_char,
                )) == TEAM_RED as libc::c_int
                {
                    BotReportStatus(botstates[i as usize]);
                }
            }
        }
        i += 1
    }
    BotAI_Print(
        1i32,
        b"^4BLUE\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0i32;
    while i < level.maxclients {
        //
        if !(botstates[i as usize].is_null() || 0 == (*botstates[i as usize]).inuse) {
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
                if atoi(Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const libc::c_char,
                )) == TEAM_BLUE as libc::c_int
                {
                    BotReportStatus(botstates[i as usize]);
                }
            }
        }
        i += 1
    }
}
pub type bot_activategoal_t = bot_activategoal_s;
pub type bot_waypoint_t = bot_waypoint_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_activategoal_s {
    pub inuse: libc::c_int,
    pub goal: bot_goal_t,
    pub time: libc::c_float,
    pub start_time: libc::c_float,
    pub justused_time: libc::c_float,
    pub shoot: libc::c_int,
    pub weapon: libc::c_int,
    pub target: vec3_t,
    pub origin: vec3_t,
    pub areas: [libc::c_int; 32],
    pub numareas: libc::c_int,
    pub areasdisabled: libc::c_int,
    pub next: *mut bot_activategoal_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_state_s {
    pub inuse: libc::c_int,
    pub botthink_residual: libc::c_int,
    pub client: libc::c_int,
    pub entitynum: libc::c_int,
    pub cur_ps: playerState_t,
    pub last_eFlags: libc::c_int,
    pub lastucmd: usercmd_t,
    pub entityeventTime: [libc::c_int; 1024],
    pub settings: bot_settings_t,
    pub ainode: Option<unsafe extern "C" fn(_: *mut bot_state_s) -> libc::c_int>,
    pub thinktime: libc::c_float,
    pub origin: vec3_t,
    pub velocity: vec3_t,
    pub presencetype: libc::c_int,
    pub eye: vec3_t,
    pub areanum: libc::c_int,
    pub inventory: [libc::c_int; 256],
    pub tfl: libc::c_int,
    pub flags: libc::c_int,
    pub respawn_wait: libc::c_int,
    pub lasthealth: libc::c_int,
    pub lastkilledplayer: libc::c_int,
    pub lastkilledby: libc::c_int,
    pub botdeathtype: libc::c_int,
    pub enemydeathtype: libc::c_int,
    pub botsuicide: libc::c_int,
    pub enemysuicide: libc::c_int,
    pub setupcount: libc::c_int,
    pub map_restart: libc::c_int,
    pub entergamechat: libc::c_int,
    pub num_deaths: libc::c_int,
    pub num_kills: libc::c_int,
    pub revenge_enemy: libc::c_int,
    pub revenge_kills: libc::c_int,
    pub lastframe_health: libc::c_int,
    pub lasthitcount: libc::c_int,
    pub chatto: libc::c_int,
    pub walker: libc::c_float,
    pub ltime: libc::c_float,
    pub entergame_time: libc::c_float,
    pub ltg_time: libc::c_float,
    pub nbg_time: libc::c_float,
    pub respawn_time: libc::c_float,
    pub respawnchat_time: libc::c_float,
    pub chase_time: libc::c_float,
    pub enemyvisible_time: libc::c_float,
    pub check_time: libc::c_float,
    pub stand_time: libc::c_float,
    pub lastchat_time: libc::c_float,
    pub kamikaze_time: libc::c_float,
    pub invulnerability_time: libc::c_float,
    pub standfindenemy_time: libc::c_float,
    pub attackstrafe_time: libc::c_float,
    pub attackcrouch_time: libc::c_float,
    pub attackchase_time: libc::c_float,
    pub attackjump_time: libc::c_float,
    pub enemysight_time: libc::c_float,
    pub enemydeath_time: libc::c_float,
    pub enemyposition_time: libc::c_float,
    pub defendaway_time: libc::c_float,
    pub defendaway_range: libc::c_float,
    pub rushbaseaway_time: libc::c_float,
    pub attackaway_time: libc::c_float,
    pub harvestaway_time: libc::c_float,
    pub ctfroam_time: libc::c_float,
    pub killedenemy_time: libc::c_float,
    pub arrive_time: libc::c_float,
    pub lastair_time: libc::c_float,
    pub teleport_time: libc::c_float,
    pub camp_time: libc::c_float,
    pub weaponchange_time: libc::c_float,
    pub firethrottlewait_time: libc::c_float,
    pub firethrottleshoot_time: libc::c_float,
    pub notblocked_time: libc::c_float,
    pub blockedbyavoidspot_time: libc::c_float,
    pub predictobstacles_time: libc::c_float,
    pub predictobstacles_goalareanum: libc::c_int,
    pub aimtarget: vec3_t,
    pub enemyvelocity: vec3_t,
    pub enemyorigin: vec3_t,
    pub kamikazebody: libc::c_int,
    pub proxmines: [libc::c_int; 64],
    pub numproxmines: libc::c_int,
    pub character: libc::c_int,
    pub ms: libc::c_int,
    pub gs: libc::c_int,
    pub cs: libc::c_int,
    pub ws: libc::c_int,
    pub enemy: libc::c_int,
    pub lastenemyareanum: libc::c_int,
    pub lastenemyorigin: vec3_t,
    pub weaponnum: libc::c_int,
    pub viewangles: vec3_t,
    pub ideal_viewangles: vec3_t,
    pub viewanglespeed: vec3_t,
    pub ltgtype: libc::c_int,
    pub teammate: libc::c_int,
    pub decisionmaker: libc::c_int,
    pub ordered: libc::c_int,
    pub order_time: libc::c_float,
    pub owndecision_time: libc::c_int,
    pub teamgoal: bot_goal_t,
    pub altroutegoal: bot_goal_t,
    pub reachedaltroutegoal_time: libc::c_float,
    pub teammessage_time: libc::c_float,
    pub teamgoal_time: libc::c_float,
    pub teammatevisible_time: libc::c_float,
    pub teamtaskpreference: libc::c_int,
    pub lastgoal_decisionmaker: libc::c_int,
    pub lastgoal_ltgtype: libc::c_int,
    pub lastgoal_teammate: libc::c_int,
    pub lastgoal_teamgoal: bot_goal_t,
    pub lead_teammate: libc::c_int,
    pub lead_teamgoal: bot_goal_t,
    pub lead_time: libc::c_float,
    pub leadvisible_time: libc::c_float,
    pub leadmessage_time: libc::c_float,
    pub leadbackup_time: libc::c_float,
    pub teamleader: [libc::c_char; 36],
    pub askteamleader_time: libc::c_float,
    pub becometeamleader_time: libc::c_float,
    pub teamgiveorders_time: libc::c_float,
    pub lastflagcapture_time: libc::c_float,
    pub numteammates: libc::c_int,
    pub redflagstatus: libc::c_int,
    pub blueflagstatus: libc::c_int,
    pub neutralflagstatus: libc::c_int,
    pub flagstatuschanged: libc::c_int,
    pub forceorders: libc::c_int,
    pub flagcarrier: libc::c_int,
    pub ctfstrategy: libc::c_int,
    pub subteam: [libc::c_char; 32],
    pub formation_dist: libc::c_float,
    pub activatestack: *mut bot_activategoal_t,
    pub activategoalheap: [bot_activategoal_t; 8],
    pub checkpoints: *mut bot_waypoint_t,
    pub patrolpoints: *mut bot_waypoint_t,
    pub curpatrolpoint: *mut bot_waypoint_t,
    pub patrolflags: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_waypoint_s {
    pub inuse: libc::c_int,
    pub name: [libc::c_char; 32],
    pub goal: bot_goal_t,
    pub next: *mut bot_waypoint_s,
    pub prev: *mut bot_waypoint_s,
}
pub type bot_state_t = bot_state_s;
