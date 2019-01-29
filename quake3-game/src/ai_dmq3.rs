use ai_chat::{
    bot_consolemessage_s, bot_consolemessage_t, bot_match_s, bot_match_t, bot_matchvariable_s,
    bot_matchvariable_t, BotChatTime, BotChat_EnterGame, BotValidChatPosition,
};
use ai_cmd::BotMatchMessage;
use ai_dmnet::{
    AIEnter_Seek_ActivateEntity, AIEnter_Seek_LTG, AIEnter_Stand, AINode_Seek_LTG, AINode_Seek_NBG,
    AINode_Stand, BotDumpNodeSwitches, BotResetNodeSwitches,
};
use ai_main::{
    bot_activategoal_s, bot_activategoal_t, bot_developer, bot_state_s, bot_state_t,
    bot_waypoint_s, bot_waypoint_t, floattime, BotAILoadMap, BotAISetup, BotAISetupClient,
    BotAIShutdown, BotAIShutdownClient, BotAIStartFrame, BotAI_GetClientState,
    BotAI_GetEntityState, BotAI_GetSnapshotEntity, BotAI_Trace, BotEntityInfo,
    BotInterbreedEndMatch, BotTeamLeader, BotTestAAS, NumBots,
};
use ai_team::{BotTeamAI, BotVoiceChat};
use ai_variadic_h::BotAI_Print;
use be_aas_h::{
    aas_altroutegoal_s, aas_altroutegoal_t, aas_areainfo_s, aas_areainfo_t, aas_clientmove_s,
    aas_clientmove_t, aas_entityinfo_s, aas_entityinfo_t, aas_predictroute_s, aas_predictroute_t,
    aas_trace_s, aas_trace_t,
};
use be_ai_goal_h::{bot_goal_s, bot_goal_t};
use be_ai_move_h::{bot_initmove_s, bot_initmove_t, bot_moveresult_s, bot_moveresult_t};
use be_ai_weap_h::{projectileinfo_s, projectileinfo_t, weaponinfo_s, weaponinfo_t};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, unnamed_1, unnamed_2, unnamed_3,
    unnamed_4, unnamed_5, unnamed_6, unnamed_7, unnamed_8, unnamed_9, ET_BEAM, ET_EVENTS,
    ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL,
    ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH,
    EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE,
    EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP,
    EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP,
    EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP,
    EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED, EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT,
    EV_MISSILE_HIT, EV_MISSILE_MISS, EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE,
    EV_OBELISKPAIN, EV_OBITUARY, EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT,
    EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD, EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK,
    EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL, EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16,
    EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND, EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME,
    EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO, EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0,
    EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11, EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14,
    EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3, EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6,
    EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9, EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH,
    EV_WATER_UNDER, GTS_BLUEOBELISK_ATTACKED, GTS_BLUETEAM_SCORED, GTS_BLUETEAM_TOOK_LEAD,
    GTS_BLUE_CAPTURE, GTS_BLUE_RETURN, GTS_BLUE_TAKEN, GTS_KAMIKAZE, GTS_REDOBELISK_ATTACKED,
    GTS_REDTEAM_SCORED, GTS_REDTEAM_TOOK_LEAD, GTS_RED_CAPTURE, GTS_RED_RETURN, GTS_RED_TAKEN,
    GTS_TEAMS_ARE_TIED, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR,
    PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT,
    PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY,
    PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAMTASK_CAMP, TEAMTASK_DEFENSE, TEAMTASK_ESCORT, TEAMTASK_FOLLOW, TEAMTASK_NONE,
    TEAMTASK_OFFENSE, TEAMTASK_PATROL, TEAMTASK_RETRIEVE, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS,
    TEAM_RED, TEAM_SPECTATOR, WEAPON_DROPPING, WEAPON_FIRING, WEAPON_RAISING, WEAPON_READY, WP_BFG,
    WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE,
    WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
    playerTeamState_t, spectatorState_t, trap_AAS_AreaInfo, trap_AAS_AreaReachability,
    trap_AAS_AreaTravelTimeToGoalArea, trap_AAS_BBoxAreas, trap_AAS_EnableRoutingArea,
    trap_AAS_FloatForBSPEpairKey, trap_AAS_IntForBSPEpairKey, trap_AAS_NextBSPEntity,
    trap_AAS_PointAreaNum, trap_AAS_PointContents, trap_AAS_PredictClientMovement,
    trap_AAS_PredictRoute, trap_AAS_PresenceTypeBoundingBox, trap_AAS_TraceAreas,
    trap_AAS_ValueForBSPEpairKey, trap_AAS_VectorForBSPEpairKey, trap_BotAddAvoidSpot,
    trap_BotChooseBestFightWeapon, trap_BotDumpAvoidGoals, trap_BotDumpGoalStack,
    trap_BotFindMatch, trap_BotGetLevelItemGoal, trap_BotGetNextCampSpotGoal,
    trap_BotGetWeaponInfo, trap_BotInitMoveState, trap_BotLibVarSet, trap_BotMatchVariable,
    trap_BotMoveInDirection, trap_BotMoveToGoal, trap_BotNextConsoleMessage,
    trap_BotNumConsoleMessages, trap_BotPredictVisiblePosition, trap_BotRemoveConsoleMessage,
    trap_BotRemoveFromAvoidGoals, trap_BotReplaceSynonyms, trap_BotReplyChat,
    trap_BotSetChatGender, trap_BotSetChatName, trap_Characteristic_BFloat,
    trap_Characteristic_String, trap_Cvar_Register, trap_Cvar_Update,
    trap_Cvar_VariableIntegerValue, trap_EA_Action, trap_EA_Attack, trap_EA_Say,
    trap_EA_SelectWeapon, trap_EA_Use, trap_EA_View, trap_GetConfigstring, trap_GetServerinfo,
    trap_GetUserinfo, trap_PointContents, trap_SetUserinfo, trap_UnifyWhiteSpaces, CON_CONNECTED,
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
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    usercmd_s, usercmd_t, va, vec3_origin, vec3_t, vec_t, vectoangles, vmCvar_t, AngleMod,
    AngleVectors, Com_sprintf, Info_SetValueForKey, Info_ValueForKey, Q_CleanStr, Q_stricmp,
    Q_strncpyz, VectorNormalize, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE,
    TR_STATIONARY,
};
use stdlib::{
    atoi, fabs, memcpy, memmove, memset, rand, sqrt, strcmp, strlen, strncpy, strstr, toupper,
};

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
unsafe extern "C" fn VectorLengthSquared(mut v: *const vec_t) -> vec_t {
    return *v.offset(0isize) * *v.offset(0isize)
        + *v.offset(1isize) * *v.offset(1isize)
        + *v.offset(2isize) * *v.offset(2isize);
}
unsafe extern "C" fn CrossProduct(
    mut v1: *const vec_t,
    mut v2: *const vec_t,
    mut cross: *mut vec_t,
) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) - *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) - *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) - *v1.offset(1isize) * *v2.offset(0isize);
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
 * name:		ai_dmq3.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
//setup the deathmatch AI
#[no_mangle]
pub unsafe extern "C" fn BotSetupDeathmatchAI() {
    let mut ent: libc::c_int = 0;
    let mut modelnum: libc::c_int = 0;
    let mut model: [libc::c_char; 128] = [0; 128];
    gametype =
        trap_Cvar_VariableIntegerValue(b"g_gametype\x00" as *const u8 as *const libc::c_char);
    trap_Cvar_Register(
        &mut bot_rocketjump,
        b"bot_rocketjump\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_grapple,
        b"bot_grapple\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_fastchat,
        b"bot_fastchat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_nochat,
        b"bot_nochat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_testrchat,
        b"bot_testrchat\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_challenge,
        b"bot_challenge\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut bot_predictobstacles,
        b"bot_predictobstacles\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    trap_Cvar_Register(
        &mut g_spSkill,
        b"g_spSkill\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
    if gametype == GT_CTF as libc::c_int {
        if trap_BotGetLevelItemGoal(
            -1i32,
            b"Red Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut ctf_redflag as *mut bot_goal_t as *mut libc::c_void,
        ) < 0i32
        {
            BotAI_Print(
                2i32,
                b"CTF without Red Flag\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if trap_BotGetLevelItemGoal(
            -1i32,
            b"Blue Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut ctf_blueflag as *mut bot_goal_t as *mut libc::c_void,
        ) < 0i32
        {
            BotAI_Print(
                2i32,
                b"CTF without Blue Flag\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    max_bspmodelindex = 0i32;
    ent = trap_AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0
            == trap_AAS_ValueForBSPEpairKey(
                ent,
                b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                model.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            ))
        {
            if model[0usize] as libc::c_int == '*' as i32 {
                modelnum = atoi(model.as_mut_ptr().offset(1isize));
                if modelnum > max_bspmodelindex {
                    max_bspmodelindex = modelnum
                }
            }
        }
        ent = trap_AAS_NextBSPEntity(ent)
    }
    BotInitWaypoints();
}
/*
==================
BotInitWaypoints
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotInitWaypoints() {
    let mut i: libc::c_int = 0;
    botai_freewaypoints = 0 as *mut bot_waypoint_t;
    i = 0i32;
    while i < 128i32 {
        botai_waypoints[i as usize].next = botai_freewaypoints;
        botai_freewaypoints = &mut botai_waypoints[i as usize] as *mut bot_waypoint_t;
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
 * name:		ai_dmq3.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_dmq3.c $
 *
 *****************************************************************************/
//
//
// for the voice chats
// from aasfile.h
//
#[no_mangle]
pub static mut botai_waypoints: [bot_waypoint_t; 128] = [bot_waypoint_s {
    inuse: 0,
    name: [0; 32],
    goal: bot_goal_s {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    },
    next: 0 as *const bot_waypoint_s as *mut bot_waypoint_s,
    prev: 0 as *const bot_waypoint_s as *mut bot_waypoint_s,
}; 128];
#[no_mangle]
pub static mut botai_freewaypoints: *mut bot_waypoint_t =
    0 as *const bot_waypoint_t as *mut bot_waypoint_t;
//maximum BSP model index
#[no_mangle]
pub static mut max_bspmodelindex: libc::c_int = 0;
#[no_mangle]
pub static mut ctf_blueflag: bot_goal_t = bot_goal_s {
    origin: [0.; 3],
    areanum: 0,
    mins: [0.; 3],
    maxs: [0.; 3],
    entitynum: 0,
    number: 0,
    flags: 0,
    iteminfo: 0,
};
#[no_mangle]
pub static mut ctf_redflag: bot_goal_t = bot_goal_s {
    origin: [0.; 3],
    areanum: 0,
    mins: [0.; 3],
    maxs: [0.; 3],
    entitynum: 0,
    number: 0,
    flags: 0,
    iteminfo: 0,
};
//ctf flags
//CTF skins
//game type
#[no_mangle]
pub static mut gametype: libc::c_int = 0;
#[no_mangle]
pub static mut g_spSkill: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//NOTE: not using a cvars which can be updated because the game should be reloaded anyway
//game type
#[no_mangle]
pub static mut bot_predictobstacles: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_challenge: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_testrchat: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_nochat: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_fastchat: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_grapple: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut bot_rocketjump: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//shutdown the deathmatch AI
#[no_mangle]
pub unsafe extern "C" fn BotShutdownDeathmatchAI() {
    altroutegoals_setup = qfalse as libc::c_int;
}
//CTF flag goals
#[no_mangle]
pub static mut altroutegoals_setup: libc::c_int = 0;
//let the bot live within its deathmatch AI net
#[no_mangle]
pub unsafe extern "C" fn BotDeathmatchAI(mut bs: *mut bot_state_t, mut thinktime: libc::c_float) {
    let mut gender: [libc::c_char; 144] = [0; 144];
    let mut name: [libc::c_char; 144] = [0; 144];
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    if (*bs).setupcount > 0i32 {
        (*bs).setupcount -= 1;
        if (*bs).setupcount > 0i32 {
            return;
        }
        trap_Characteristic_String(
            (*bs).character,
            1i32,
            gender.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
        );
        trap_GetUserinfo(
            (*bs).client,
            userinfo.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"sex\x00" as *const u8 as *const libc::c_char,
            gender.as_mut_ptr(),
        );
        trap_SetUserinfo((*bs).client, userinfo.as_mut_ptr());
        if gender[0usize] as libc::c_int == 'm' as i32 {
            trap_BotSetChatGender((*bs).cs, 2i32);
        } else if gender[0usize] as libc::c_int == 'f' as i32 {
            trap_BotSetChatGender((*bs).cs, 1i32);
        } else {
            trap_BotSetChatGender((*bs).cs, 0i32);
        }
        ClientName(
            (*bs).client,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
        );
        trap_BotSetChatName((*bs).cs, name.as_mut_ptr(), (*bs).client);
        (*bs).lastframe_health = (*bs).inventory[29usize];
        (*bs).lasthitcount = (*bs).cur_ps.persistant[PERS_HITS as libc::c_int as usize];
        (*bs).setupcount = 0i32;
        BotSetupAlternativeRouteGoals();
    }
    (*bs).flags &= !32i32;
    if 0 == BotIntermission(bs) as u64 {
        BotSetTeleportTime(bs);
        BotUpdateInventory(bs);
        BotCheckSnapshot(bs);
        BotCheckAir(bs);
    }
    BotCheckConsoleMessages(bs);
    if 0 == BotIntermission(bs) as u64 && 0 == BotIsObserver(bs) as u64 {
        BotTeamAI(bs);
    }
    if (*bs).ainode.is_none() {
        AIEnter_Seek_LTG(
            bs,
            b"BotDeathmatchAI: no ai node\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if 0 == (*bs).entergamechat && (*bs).entergame_time > floattime - 8i32 as libc::c_float {
        if 0 != BotChat_EnterGame(bs) {
            (*bs).stand_time = floattime + BotChatTime(bs);
            AIEnter_Stand(
                bs,
                b"BotDeathmatchAI: chat enter game\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        (*bs).entergamechat = qtrue as libc::c_int
    }
    BotResetNodeSwitches();
    i = 0i32;
    while i < 50i32 {
        if 0 != (*bs).ainode.expect("non-null function pointer")(bs) {
            break;
        }
        i += 1
    }
    if 0 == (*bs).inuse {
        return;
    }
    if i >= 50i32 {
        trap_BotDumpGoalStack((*bs).gs);
        trap_BotDumpAvoidGoals((*bs).gs);
        BotDumpNodeSwitches(bs);
        ClientName(
            (*bs).client,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
        );
        BotAI_Print(
            3i32,
            b"%s at %1.1f switched more than %d AI nodes\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
            floattime as libc::c_double,
            50i32,
        );
    }
    (*bs).lastframe_health = (*bs).inventory[29usize];
    (*bs).lasthitcount = (*bs).cur_ps.persistant[PERS_HITS as libc::c_int as usize];
}
//returns the name of the client
#[no_mangle]
pub unsafe extern "C" fn ClientName(
    mut client: libc::c_int,
    mut name: *mut libc::c_char,
    mut size: libc::c_int,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if client < 0i32 || client >= 64i32 {
        BotAI_Print(
            3i32,
            b"ClientName: client out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return b"[client out of range]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    trap_GetConfigstring(
        32i32 + 256i32 + 256i32 + client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    strncpy(
        name,
        Info_ValueForKey(
            buf.as_mut_ptr(),
            b"n\x00" as *const u8 as *const libc::c_char,
        ),
        (size - 1i32) as libc::c_ulong,
    );
    *name.offset((size - 1i32) as isize) = '\u{0}' as i32 as libc::c_char;
    Q_CleanStr(name);
    return name;
}
//returns true if the bot is in observer mode
#[no_mangle]
pub unsafe extern "C" fn BotIsObserver(mut bs: *mut bot_state_t) -> qboolean {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if (*bs).cur_ps.pm_type == PM_SPECTATOR as libc::c_int {
        return qtrue;
    }
    trap_GetConfigstring(
        32i32 + 256i32 + 256i32 + (*bs).client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if atoi(Info_ValueForKey(
        buf.as_mut_ptr(),
        b"t\x00" as *const u8 as *const libc::c_char,
    )) == TEAM_SPECTATOR as libc::c_int
    {
        return qtrue;
    }
    return qfalse;
}
//returns true if the bot is in the intermission
#[no_mangle]
pub unsafe extern "C" fn BotIntermission(mut bs: *mut bot_state_t) -> qboolean {
    if 0 != level.intermissiontime {
        return qtrue;
    }
    return ((*bs).cur_ps.pm_type == PM_FREEZE as libc::c_int
        || (*bs).cur_ps.pm_type == PM_INTERMISSION as libc::c_int) as libc::c_int
        as qboolean;
}
/*
==================
BotCheckConsoleMessages
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCheckConsoleMessages(mut bs: *mut bot_state_t) {
    let mut botname: [libc::c_char; 36] = [0; 36];
    let mut message: [libc::c_char; 256] = [0; 256];
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chat_reply: libc::c_float = 0.;
    let mut context: libc::c_int = 0;
    let mut handle: libc::c_int = 0;
    let mut m: bot_consolemessage_t = bot_consolemessage_s {
        handle: 0,
        time: 0.,
        type_0: 0,
        message: [0; 256],
        prev: 0 as *mut bot_consolemessage_s,
        next: 0 as *mut bot_consolemessage_s,
    };
    let mut match_0: bot_match_t = bot_match_s {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [bot_matchvariable_s {
            offset: 0,
            length: 0,
        }; 8],
    };
    ClientName(
        (*bs).client,
        botname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    loop {
        handle = trap_BotNextConsoleMessage(
            (*bs).cs,
            &mut m as *mut bot_consolemessage_t as *mut libc::c_void,
        );
        if !(handle != 0i32) {
            break;
        }
        //if the chat state is flooded with messages the bot will read them quickly
        if trap_BotNumConsoleMessages((*bs).cs) < 10i32 {
            //if it is a chat message the bot needs some time to read it
            if m.type_0 == 1i32
                && m.time
                    > floattime
                        - (1i32 as libc::c_float
                            + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
            {
                break;
            }
        }
        ptr = m.message.as_mut_ptr();
        if m.type_0 == 1i32 {
            if 0 != trap_BotFindMatch(
                m.message.as_mut_ptr(),
                &mut match_0 as *mut bot_match_t as *mut libc::c_void,
                128i32 as libc::c_ulong,
            ) {
                ptr = m
                    .message
                    .as_mut_ptr()
                    .offset(match_0.variables[2usize].offset as libc::c_int as isize)
            }
        }
        trap_UnifyWhiteSpaces(ptr);
        context = BotSynonymContext(bs);
        trap_BotReplaceSynonyms(ptr, context as libc::c_ulong);
        //if there's no match
        if 0 == BotMatchMessage(bs, m.message.as_mut_ptr()) {
            //if it is a chat message
            if m.type_0 == 1i32 && 0 == bot_nochat.integer {
                //
                if 0 == trap_BotFindMatch(
                    m.message.as_mut_ptr(),
                    &mut match_0 as *mut bot_match_t as *mut libc::c_void,
                    128i32 as libc::c_ulong,
                ) {
                    trap_BotRemoveConsoleMessage((*bs).cs, handle);
                    continue;
                } else if 0 != match_0.subtype & 32768i32 {
                    trap_BotRemoveConsoleMessage((*bs).cs, handle);
                    continue;
                } else {
                    trap_BotMatchVariable(
                        &mut match_0 as *mut bot_match_t as *mut libc::c_void,
                        0i32,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    trap_BotMatchVariable(
                        &mut match_0 as *mut bot_match_t as *mut libc::c_void,
                        2i32,
                        message.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                            as libc::c_int,
                    );
                    //if this is a message from the bot self
                    if (*bs).client == ClientFromName(netname.as_mut_ptr()) {
                        trap_BotRemoveConsoleMessage((*bs).cs, handle);
                        continue;
                    } else {
                        trap_UnifyWhiteSpaces(message.as_mut_ptr());
                        trap_Cvar_Update(&mut bot_testrchat);
                        if 0 != bot_testrchat.integer {
                            trap_BotLibVarSet(
                                b"bot_testrchat\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                            if 0 != trap_BotReplyChat(
                                (*bs).cs,
                                message.as_mut_ptr(),
                                context,
                                16i32,
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                                0 as *mut libc::c_char,
                                botname.as_mut_ptr(),
                                netname.as_mut_ptr(),
                            ) {
                                BotAI_Print(
                                    1i32,
                                    b"------------------------\n\x00" as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            } else {
                                BotAI_Print(
                                    1i32,
                                    b"**** no valid reply ****\n\x00" as *const u8
                                        as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                            }
                        } else if (*bs).ainode != Some(AINode_Stand)
                            && 0 != BotValidChatPosition(bs)
                            && 0 == TeamPlayIsOn()
                        {
                            chat_reply = trap_Characteristic_BFloat(
                                (*bs).character,
                                35i32,
                                0i32 as libc::c_float,
                                1i32 as libc::c_float,
                            );
                            if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                                as libc::c_double)
                                < 1.5f64 / (NumBots() + 1i32) as libc::c_double
                                && ((rand() & 0x7fffi32) as libc::c_float
                                    / 0x7fffi32 as libc::c_float)
                                    < chat_reply
                            {
                                //if bot replies with a chat message
                                if 0 != trap_BotReplyChat(
                                    (*bs).cs,
                                    message.as_mut_ptr(),
                                    context,
                                    16i32,
                                    0 as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                    botname.as_mut_ptr(),
                                    netname.as_mut_ptr(),
                                ) {
                                    trap_BotRemoveConsoleMessage((*bs).cs, handle);
                                    (*bs).stand_time = floattime + BotChatTime(bs);
                                    AIEnter_Stand(
                                        bs,
                                        b"BotCheckConsoleMessages: reply chat\x00" as *const u8
                                            as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    //EA_Say(bs->client, bs->cs.chatmessage);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        trap_BotRemoveConsoleMessage((*bs).cs, handle);
    }
}
//returns true if teamplay is on
#[no_mangle]
pub unsafe extern "C" fn TeamPlayIsOn() -> libc::c_int {
    return (gametype >= GT_TEAM as libc::c_int) as libc::c_int;
}
//returns the number of the client with the given name
#[no_mangle]
pub unsafe extern "C" fn ClientFromName(mut name: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0i32;
    while i < level.maxclients {
        trap_GetConfigstring(
            32i32 + 256i32 + 256i32 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        Q_CleanStr(buf.as_mut_ptr());
        if 0 == Q_stricmp(
            Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            ),
            name,
        ) {
            return i;
        }
        i += 1
    }
    return -1i32;
}
// returns the appropriate synonym context for the current game type and situation
#[no_mangle]
pub unsafe extern "C" fn BotSynonymContext(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut context: libc::c_int = 0;
    context = 1i32 | 2i32 | 1024i32;
    if gametype == GT_CTF as libc::c_int {
        if BotTeam(bs) == TEAM_RED as libc::c_int {
            context |= 4i32
        } else {
            context |= 8i32
        }
    }
    return context;
}
//returns the team the bot is in
#[no_mangle]
pub unsafe extern "C" fn BotTeam(mut bs: *mut bot_state_t) -> libc::c_int {
    if (*bs).client < 0i32 || (*bs).client >= 64i32 {
        return qfalse as libc::c_int;
    }
    if (*level.clients.offset((*bs).client as isize))
        .sess
        .sessionTeam as libc::c_uint
        == TEAM_RED as libc::c_int as libc::c_uint
    {
        return TEAM_RED as libc::c_int;
    } else {
        if (*level.clients.offset((*bs).client as isize))
            .sess
            .sessionTeam as libc::c_uint
            == TEAM_BLUE as libc::c_int as libc::c_uint
        {
            return TEAM_BLUE as libc::c_int;
        }
    }
    return TEAM_FREE as libc::c_int;
}
/*
==================
BotCheckAir
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCheckAir(mut bs: *mut bot_state_t) {
    if (*bs).inventory[36usize] <= 0i32 {
        if 0 != trap_AAS_PointContents((*bs).eye.as_mut_ptr()) & (32i32 | 16i32 | 8i32) {
            return;
        }
    }
    (*bs).lastair_time = floattime;
}
/*
==================
BotCheckSnapshot
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCheckSnapshot(mut bs: *mut bot_state_t) {
    let mut ent: libc::c_int = 0;
    let mut state: entityState_t = entityState_s {
        number: 0,
        eType: 0,
        eFlags: 0,
        pos: trajectory_t {
            trType: TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        apos: trajectory_t {
            trType: TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        time: 0,
        time2: 0,
        origin: [0.; 3],
        origin2: [0.; 3],
        angles: [0.; 3],
        angles2: [0.; 3],
        otherEntityNum: 0,
        otherEntityNum2: 0,
        groundEntityNum: 0,
        constantLight: 0,
        loopSound: 0,
        modelindex: 0,
        modelindex2: 0,
        clientNum: 0,
        frame: 0,
        solid: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
        generic1: 0,
    };
    trap_BotAddAvoidSpot(
        (*bs).ms,
        vec3_origin.as_mut_ptr(),
        0i32 as libc::c_float,
        0i32,
    );
    (*bs).kamikazebody = 0i32;
    (*bs).numproxmines = 0i32;
    ent = 0i32;
    loop {
        ent = BotAI_GetSnapshotEntity((*bs).client, ent, &mut state);
        if !(ent != -1i32) {
            break;
        }
        BotCheckEvents(bs, &mut state);
        BotCheckForGrenades(bs, &mut state);
    }
    BotAI_GetEntityState((*bs).client, &mut state);
    state.event = (*bs).cur_ps.externalEvent;
    state.eventParm = (*bs).cur_ps.externalEventParm;
    BotCheckEvents(bs, &mut state);
}
/*
==================
BotCheckEvents
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCheckEvents(mut bs: *mut bot_state_t, mut state: *mut entityState_t) {
    let mut event: libc::c_int = 0;
    let mut buf: [libc::c_char; 128] = [0; 128];
    if (*bs).entityeventTime[(*state).number as usize]
        == g_entities[(*state).number as usize].eventTime
    {
        return;
    }
    (*bs).entityeventTime[(*state).number as usize] =
        g_entities[(*state).number as usize].eventTime;
    if (*state).eType > ET_EVENTS as libc::c_int {
        event = (*state).eType - ET_EVENTS as libc::c_int & !(0x100i32 | 0x200i32)
    } else {
        event = (*state).event & !(0x100i32 | 0x200i32)
    }
    match event {
        60 => {
            let mut target: libc::c_int = 0;
            let mut attacker: libc::c_int = 0;
            let mut mod_0: libc::c_int = 0;
            target = (*state).otherEntityNum;
            attacker = (*state).otherEntityNum2;
            mod_0 = (*state).eventParm;
            if target == (*bs).client {
                (*bs).botdeathtype = mod_0;
                (*bs).lastkilledby = attacker;
                if target == attacker
                    || target == (1i32 << 10i32) - 1i32
                    || target == (1i32 << 10i32) - 2i32
                {
                    (*bs).botsuicide = qtrue as libc::c_int
                } else {
                    (*bs).botsuicide = qfalse as libc::c_int
                }
                (*bs).num_deaths += 1
            } else if attacker == (*bs).client {
                (*bs).enemydeathtype = mod_0;
                (*bs).lastkilledplayer = target;
                (*bs).killedenemy_time = floattime;
                (*bs).num_kills += 1
            } else if attacker == (*bs).enemy && target == attacker {
                (*bs).enemysuicide = qtrue as libc::c_int
            }
        }
        46 => {
            //
            if (*state).eventParm < 0i32 || (*state).eventParm >= 256i32 {
                BotAI_Print(
                    3i32,
                    b"EV_GLOBAL_SOUND: eventParm (%d) out of range\n\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*state).eventParm,
                );
            } else {
                trap_GetConfigstring(
                    32i32 + 256i32 + (*state).eventParm,
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
                );
                if 0 == strcmp(
                    buf.as_mut_ptr(),
                    b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const libc::c_char,
                ) {
                    BotGoForPowerups(bs);
                }
            }
        }
        47 => {
            if gametype == GT_CTF as libc::c_int {
                match (*state).eventParm {
                    0 => {
                        (*bs).blueflagstatus = 0i32;
                        (*bs).redflagstatus = 0i32;
                        (*bs).flagstatuschanged = qtrue as libc::c_int
                    }
                    1 => {
                        //see BotMatch_CTF
                        (*bs).blueflagstatus = 0i32;
                        (*bs).redflagstatus = 0i32;
                        (*bs).flagstatuschanged = qtrue as libc::c_int
                    }
                    2 => {
                        //see BotMatch_CTF
                        (*bs).blueflagstatus = 0i32;
                        (*bs).flagstatuschanged = qtrue as libc::c_int
                    }
                    3 => {
                        (*bs).redflagstatus = 0i32;
                        (*bs).flagstatuschanged = qtrue as libc::c_int
                    }
                    4 => {
                        (*bs).blueflagstatus = 1i32;
                        (*bs).flagstatuschanged = qtrue as libc::c_int
                    }
                    5 => {
                        //see BotMatch_CTF
                        (*bs).redflagstatus = 1i32;
                        (*bs).flagstatuschanged = qtrue as libc::c_int
                    }
                    _ => {}
                }
            }
        }
        42 => {
            lastteleport_origin[0usize] = (*state).origin[0usize];
            lastteleport_origin[1usize] = (*state).origin[1usize];
            lastteleport_origin[2usize] = (*state).origin[2usize];
            lastteleport_time = floattime
        }
        45 => {
            //if this sound is played on the bot
            if (*state).number == (*bs).client {
                if (*state).eventParm < 0i32 || (*state).eventParm >= 256i32 {
                    BotAI_Print(
                        3i32,
                        b"EV_GENERAL_SOUND: eventParm (%d) out of range\n\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*state).eventParm,
                    );
                } else {
                    trap_GetConfigstring(
                        32i32 + 256i32 + (*state).eventParm,
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                            as libc::c_int,
                    );
                    if 0 == strcmp(
                        buf.as_mut_ptr(),
                        b"*falling1.wav\x00" as *const u8 as *const libc::c_char,
                    ) {
                        if (*bs).inventory[30usize] > 0i32 {
                            trap_EA_Use((*bs).client);
                        }
                    }
                }
            }
        }
        1 | 2 | 3 | 4 | 5 | 10 | 11 | 12 | 6 | 7 | 8 | 9 | 13 | 14 | 76 | 15 | 16 | 17 | 18
        | 19 | 20 | 21 | 22 | 23 => {}
        24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | _ => {}
    };
}
//last teleport event time
#[no_mangle]
pub static mut lastteleport_time: libc::c_float = 0.;
//last teleport event origin
#[no_mangle]
pub static mut lastteleport_origin: vec3_t = [0.; 3];
/*
==================
BotGoForPowerups
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGoForPowerups(mut bs: *mut bot_state_t) {
    BotDontAvoid(
        bs,
        b"Quad Damage\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    BotDontAvoid(
        bs,
        b"Regeneration\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    BotDontAvoid(
        bs,
        b"Battle Suit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    BotDontAvoid(
        bs,
        b"Speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    BotDontAvoid(
        bs,
        b"Invisibility\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*bs).ltg_time = 0i32 as libc::c_float;
}
/*
==================
BotDontAvoid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotDontAvoid(mut bs: *mut bot_state_t, mut itemname: *mut libc::c_char) {
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
    let mut num: libc::c_int = 0;
    num = trap_BotGetLevelItemGoal(
        -1i32,
        itemname,
        &mut goal as *mut bot_goal_t as *mut libc::c_void,
    );
    while num >= 0i32 {
        trap_BotRemoveFromAvoidGoals((*bs).gs, goal.number);
        num = trap_BotGetLevelItemGoal(
            num,
            itemname,
            &mut goal as *mut bot_goal_t as *mut libc::c_void,
        )
    }
}
/*
==================
BotCheckEvents
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCheckForGrenades(
    mut bs: *mut bot_state_t,
    mut state: *mut entityState_t,
) {
    if (*state).eType != ET_MISSILE as libc::c_int
        || (*state).weapon != WP_GRENADE_LAUNCHER as libc::c_int
    {
        return;
    }
    trap_BotAddAvoidSpot(
        (*bs).ms,
        (*state).pos.trBase.as_mut_ptr(),
        160i32 as libc::c_float,
        1i32,
    );
}
//update the inventory
#[no_mangle]
pub unsafe extern "C" fn BotUpdateInventory(mut bs: *mut bot_state_t) {
    let mut oldinventory: [libc::c_int; 256] = [0; 256];
    memcpy(
        oldinventory.as_mut_ptr() as *mut libc::c_void,
        (*bs).inventory.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong,
    );
    (*bs).inventory[1usize] = (*bs).cur_ps.stats[STAT_ARMOR as libc::c_int as usize];
    (*bs).inventory[4usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_GAUNTLET as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[5usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_SHOTGUN as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[6usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_MACHINEGUN as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[7usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_GRENADE_LAUNCHER as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[8usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_ROCKET_LAUNCHER as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[9usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_LIGHTNING as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[10usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_RAILGUN as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[11usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_PLASMAGUN as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[13usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_BFG as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[14usize] = ((*bs).cur_ps.stats[STAT_WEAPONS as libc::c_int as usize]
        & 1i32 << WP_GRAPPLING_HOOK as libc::c_int
        != 0i32) as libc::c_int;
    (*bs).inventory[18usize] = (*bs).cur_ps.ammo[WP_SHOTGUN as libc::c_int as usize];
    (*bs).inventory[19usize] = (*bs).cur_ps.ammo[WP_MACHINEGUN as libc::c_int as usize];
    (*bs).inventory[20usize] = (*bs).cur_ps.ammo[WP_GRENADE_LAUNCHER as libc::c_int as usize];
    (*bs).inventory[21usize] = (*bs).cur_ps.ammo[WP_PLASMAGUN as libc::c_int as usize];
    (*bs).inventory[22usize] = (*bs).cur_ps.ammo[WP_LIGHTNING as libc::c_int as usize];
    (*bs).inventory[23usize] = (*bs).cur_ps.ammo[WP_ROCKET_LAUNCHER as libc::c_int as usize];
    (*bs).inventory[24usize] = (*bs).cur_ps.ammo[WP_RAILGUN as libc::c_int as usize];
    (*bs).inventory[25usize] = (*bs).cur_ps.ammo[WP_BFG as libc::c_int as usize];
    (*bs).inventory[29usize] = (*bs).cur_ps.stats[STAT_HEALTH as libc::c_int as usize];
    (*bs).inventory[30usize] =
        ((*bs).cur_ps.stats[STAT_HOLDABLE_ITEM as libc::c_int as usize] == 26i32) as libc::c_int;
    (*bs).inventory[31usize] =
        ((*bs).cur_ps.stats[STAT_HOLDABLE_ITEM as libc::c_int as usize] == 27i32) as libc::c_int;
    (*bs).inventory[35usize] =
        ((*bs).cur_ps.powerups[PW_QUAD as libc::c_int as usize] != 0i32) as libc::c_int;
    (*bs).inventory[36usize] =
        ((*bs).cur_ps.powerups[PW_BATTLESUIT as libc::c_int as usize] != 0i32) as libc::c_int;
    (*bs).inventory[37usize] =
        ((*bs).cur_ps.powerups[PW_HASTE as libc::c_int as usize] != 0i32) as libc::c_int;
    (*bs).inventory[38usize] =
        ((*bs).cur_ps.powerups[PW_INVIS as libc::c_int as usize] != 0i32) as libc::c_int;
    (*bs).inventory[39usize] =
        ((*bs).cur_ps.powerups[PW_REGEN as libc::c_int as usize] != 0i32) as libc::c_int;
    (*bs).inventory[40usize] =
        ((*bs).cur_ps.powerups[PW_FLIGHT as libc::c_int as usize] != 0i32) as libc::c_int;
    (*bs).inventory[45usize] =
        ((*bs).cur_ps.powerups[PW_REDFLAG as libc::c_int as usize] != 0i32) as libc::c_int;
    (*bs).inventory[46usize] =
        ((*bs).cur_ps.powerups[PW_BLUEFLAG as libc::c_int as usize] != 0i32) as libc::c_int;
    BotCheckItemPickup(bs, oldinventory.as_mut_ptr());
}
/*
==================
BotCheckItemPickup
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCheckItemPickup(
    mut bs: *mut bot_state_t,
    mut oldinventory: *mut libc::c_int,
) {
}
/*
==================
BotSetTeleportTime
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSetTeleportTime(mut bs: *mut bot_state_t) {
    if 0 != ((*bs).cur_ps.eFlags ^ (*bs).last_eFlags) & 0x4i32 {
        (*bs).teleport_time = floattime
    }
    (*bs).last_eFlags = (*bs).cur_ps.eFlags;
}
/*
==================
BotSetupAlternateRouteGoals
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSetupAlternativeRouteGoals() {
    if 0 != altroutegoals_setup {
        return;
    }
    altroutegoals_setup = qtrue as libc::c_int;
}
//free waypoints
#[no_mangle]
pub unsafe extern "C" fn BotFreeWaypoints(mut wp: *mut bot_waypoint_t) {
    let mut nextwp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    while !wp.is_null() {
        nextwp = (*wp).next;
        (*wp).next = botai_freewaypoints;
        botai_freewaypoints = wp;
        wp = nextwp
    }
}
//choose a weapon
#[no_mangle]
pub unsafe extern "C" fn BotChooseWeapon(mut bs: *mut bot_state_t) {
    let mut newweaponnum: libc::c_int = 0;
    if (*bs).cur_ps.weaponstate == WEAPON_RAISING as libc::c_int
        || (*bs).cur_ps.weaponstate == WEAPON_DROPPING as libc::c_int
    {
        trap_EA_SelectWeapon((*bs).client, (*bs).weaponnum);
    } else {
        newweaponnum = trap_BotChooseBestFightWeapon((*bs).ws, (*bs).inventory.as_mut_ptr());
        if (*bs).weaponnum != newweaponnum {
            (*bs).weaponchange_time = floattime
        }
        (*bs).weaponnum = newweaponnum;
        trap_EA_SelectWeapon((*bs).client, (*bs).weaponnum);
    };
}
//setup movement stuff
#[no_mangle]
pub unsafe extern "C" fn BotSetupForMovement(mut bs: *mut bot_state_t) {
    let mut initmove: bot_initmove_t = bot_initmove_s {
        origin: [0.; 3],
        velocity: [0.; 3],
        viewoffset: [0.; 3],
        entitynum: 0,
        client: 0,
        thinktime: 0.,
        presencetype: 0,
        viewangles: [0.; 3],
        or_moveflags: 0,
    };
    memset(
        &mut initmove as *mut bot_initmove_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<bot_initmove_t>() as libc::c_ulong,
    );
    initmove.origin[0usize] = (*bs).cur_ps.origin[0usize];
    initmove.origin[1usize] = (*bs).cur_ps.origin[1usize];
    initmove.origin[2usize] = (*bs).cur_ps.origin[2usize];
    initmove.velocity[0usize] = (*bs).cur_ps.velocity[0usize];
    initmove.velocity[1usize] = (*bs).cur_ps.velocity[1usize];
    initmove.velocity[2usize] = (*bs).cur_ps.velocity[2usize];
    initmove.viewoffset[2usize] = 0i32 as vec_t;
    initmove.viewoffset[1usize] = initmove.viewoffset[2usize];
    initmove.viewoffset[0usize] = initmove.viewoffset[1usize];
    initmove.viewoffset[2usize] += (*bs).cur_ps.viewheight as libc::c_float;
    initmove.entitynum = (*bs).entitynum;
    initmove.client = (*bs).client;
    initmove.thinktime = (*bs).thinktime;
    if (*bs).cur_ps.groundEntityNum != (1i32 << 10i32) - 1i32 {
        initmove.or_moveflags |= 2i32
    }
    if 0 != (*bs).cur_ps.pm_flags & 64i32 && (*bs).cur_ps.pm_time > 0i32 {
        initmove.or_moveflags |= 32i32
    }
    if 0 != (*bs).cur_ps.pm_flags & 256i32 && (*bs).cur_ps.pm_time > 0i32 {
        initmove.or_moveflags |= 16i32
    }
    if 0 != (*bs).cur_ps.pm_flags & 1i32 {
        initmove.presencetype = 4i32
    } else {
        initmove.presencetype = 2i32
    }
    if (*bs).walker as libc::c_double > 0.5f64 {
        initmove.or_moveflags |= 512i32
    }
    initmove.viewangles[0usize] = (*bs).viewangles[0usize];
    initmove.viewangles[1usize] = (*bs).viewangles[1usize];
    initmove.viewangles[2usize] = (*bs).viewangles[2usize];
    trap_BotInitMoveState(
        (*bs).ms,
        &mut initmove as *mut bot_initmove_t as *mut libc::c_void,
    );
}
//update the inventory during battle
#[no_mangle]
pub unsafe extern "C" fn BotUpdateBattleInventory(
    mut bs: *mut bot_state_t,
    mut enemy: libc::c_int,
) {
    let mut dir: vec3_t = [0.; 3];
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
    BotEntityInfo(enemy, &mut entinfo);
    dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
    dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
    dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
    (*bs).inventory[201usize] = dir[2usize] as libc::c_int;
    dir[2usize] = 0i32 as vec_t;
    (*bs).inventory[200usize] = VectorLength(dir.as_mut_ptr() as *const vec_t) as libc::c_int;
}
//use holdable items during battle
#[no_mangle]
pub unsafe extern "C" fn BotBattleUseItems(mut bs: *mut bot_state_t) {
    if (*bs).inventory[29usize] < 40i32 {
        if (*bs).inventory[30usize] > 0i32 {
            if 0 == BotCTFCarryingFlag(bs) {
                trap_EA_Use((*bs).client);
            }
        }
    }
    if (*bs).inventory[29usize] < 60i32 {
        if (*bs).inventory[31usize] > 0i32 {
            trap_EA_Use((*bs).client);
        }
    };
}
//returns the flag the bot is carrying (CTFFLAG_?)
#[no_mangle]
pub unsafe extern "C" fn BotCTFCarryingFlag(mut bs: *mut bot_state_t) -> libc::c_int {
    if gametype != GT_CTF as libc::c_int {
        return 0i32;
    }
    if (*bs).inventory[45usize] > 0i32 {
        return 1i32;
    } else {
        if (*bs).inventory[46usize] > 0i32 {
            return 2i32;
        }
    }
    return 0i32;
}
//return true if the bot is dead
#[no_mangle]
pub unsafe extern "C" fn BotIsDead(mut bs: *mut bot_state_t) -> qboolean {
    return ((*bs).cur_ps.pm_type == PM_DEAD as libc::c_int) as libc::c_int as qboolean;
}
//returns true if the bot is in lava or slime
#[no_mangle]
pub unsafe extern "C" fn BotInLavaOrSlime(mut bs: *mut bot_state_t) -> qboolean {
    let mut feet: vec3_t = [0.; 3];
    feet[0usize] = (*bs).origin[0usize];
    feet[1usize] = (*bs).origin[1usize];
    feet[2usize] = (*bs).origin[2usize];
    feet[2usize] -= 23i32 as libc::c_float;
    return (trap_AAS_PointContents(feet.as_mut_ptr()) & (8i32 | 16i32)) as qboolean;
}
//returns true if the entity is dead
#[no_mangle]
pub unsafe extern "C" fn EntityIsDead(mut entinfo: *mut aas_entityinfo_t) -> qboolean {
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
    if (*entinfo).number >= 0i32 && (*entinfo).number < 64i32 {
        if 0 == BotAI_GetClientState((*entinfo).number, &mut ps) {
            return qfalse;
        }
        if ps.pm_type != PM_NORMAL as libc::c_int {
            return qtrue;
        }
    }
    return qfalse;
}
//returns true if the entity is invisible
#[no_mangle]
pub unsafe extern "C" fn EntityIsInvisible(mut entinfo: *mut aas_entityinfo_t) -> qboolean {
    if 0 != EntityCarriesFlag(entinfo) as u64 {
        return qfalse;
    }
    if 0 != (*entinfo).powerups & 1i32 << PW_INVIS as libc::c_int {
        return qtrue;
    }
    return qfalse;
}
/*
==================
EntityCarriesFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn EntityCarriesFlag(mut entinfo: *mut aas_entityinfo_t) -> qboolean {
    if 0 != (*entinfo).powerups & 1i32 << PW_REDFLAG as libc::c_int {
        return qtrue;
    }
    if 0 != (*entinfo).powerups & 1i32 << PW_BLUEFLAG as libc::c_int {
        return qtrue;
    }
    return qfalse;
}
//returns true if the entity is shooting
#[no_mangle]
pub unsafe extern "C" fn EntityIsShooting(mut entinfo: *mut aas_entityinfo_t) -> qboolean {
    if 0 != (*entinfo).flags & 0x100i32 {
        return qtrue;
    }
    return qfalse;
}
// set a user info key/value pair
#[no_mangle]
pub unsafe extern "C" fn BotSetUserInfo(
    mut bs: *mut bot_state_t,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    trap_GetUserinfo(
        (*bs).client,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, value);
    trap_SetUserinfo((*bs).client, userinfo.as_mut_ptr());
    ClientUserinfoChanged((*bs).client);
}
// set the team status (offense, defense etc.)
#[no_mangle]
pub unsafe extern "C" fn BotSetTeamStatus(mut bs: *mut bot_state_t) {}
//returns a simplified client name
#[no_mangle]
pub unsafe extern "C" fn EasyClientName(
    mut client: libc::c_int,
    mut buf: *mut libc::c_char,
    mut size: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut str1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut name: [libc::c_char; 128] = [
        0i32 as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    ClientName(
        client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    i = 0i32;
    while 0 != name[i as usize] {
        name[i as usize] = (name[i as usize] as libc::c_int & 127i32) as libc::c_char;
        i += 1
    }
    ptr = strstr(
        name.as_mut_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    while !ptr.is_null() {
        memmove(
            ptr as *mut libc::c_void,
            ptr.offset(1isize) as *const libc::c_void,
            strlen(ptr.offset(1isize)).wrapping_add(1i32 as libc::c_ulong),
        );
        ptr = strstr(
            name.as_mut_ptr(),
            b" \x00" as *const u8 as *const libc::c_char,
        )
    }
    str1 = strstr(
        name.as_mut_ptr(),
        b"[\x00" as *const u8 as *const libc::c_char,
    );
    str2 = strstr(
        name.as_mut_ptr(),
        b"]\x00" as *const u8 as *const libc::c_char,
    );
    if !str1.is_null() && !str2.is_null() {
        if str2 > str1 {
            memmove(
                str1 as *mut libc::c_void,
                str2.offset(1isize) as *const libc::c_void,
                strlen(str2.offset(1isize)).wrapping_add(1i32 as libc::c_ulong),
            );
        } else {
            memmove(
                str2 as *mut libc::c_void,
                str1.offset(1isize) as *const libc::c_void,
                strlen(str1.offset(1isize)).wrapping_add(1i32 as libc::c_ulong),
            );
        }
    }
    if (name[0usize] as libc::c_int == 'm' as i32 || name[0usize] as libc::c_int == 'M' as i32)
        && (name[1usize] as libc::c_int == 'r' as i32 || name[1usize] as libc::c_int == 'R' as i32)
    {
        memmove(
            name.as_mut_ptr() as *mut libc::c_void,
            name.as_mut_ptr().offset(2isize) as *const libc::c_void,
            strlen(name.as_mut_ptr().offset(2isize)).wrapping_add(1i32 as libc::c_ulong),
        );
    }
    ptr = name.as_mut_ptr();
    while 0 != *ptr {
        c = *ptr;
        if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
            || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
            || c as libc::c_int == '_' as i32
        {
            ptr = ptr.offset(1isize)
        } else if c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 {
            *ptr = (*ptr as libc::c_int + ('a' as i32 - 'A' as i32)) as libc::c_char;
            ptr = ptr.offset(1isize)
        } else {
            memmove(
                ptr as *mut libc::c_void,
                ptr.offset(1isize) as *const libc::c_void,
                strlen(ptr.offset(1isize)).wrapping_add(1i32 as libc::c_ulong),
            );
        }
    }
    strncpy(buf, name.as_mut_ptr(), (size - 1i32) as libc::c_ulong);
    *buf.offset((size - 1i32) as isize) = '\u{0}' as i32 as libc::c_char;
    return buf;
}
//returns the skin used by the client
#[no_mangle]
pub unsafe extern "C" fn ClientSkin(
    mut client: libc::c_int,
    mut skin: *mut libc::c_char,
    mut size: libc::c_int,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if client < 0i32 || client >= 64i32 {
        BotAI_Print(
            3i32,
            b"ClientSkin: client out of range\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return b"[client out of range]\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    trap_GetConfigstring(
        32i32 + 256i32 + 256i32 + client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    strncpy(
        skin,
        Info_ValueForKey(
            buf.as_mut_ptr(),
            b"model\x00" as *const u8 as *const libc::c_char,
        ),
        (size - 1i32) as libc::c_ulong,
    );
    *skin.offset((size - 1i32) as isize) = '\u{0}' as i32 as libc::c_char;
    return skin;
}
// set last ordered task
#[no_mangle]
pub unsafe extern "C" fn BotSetLastOrderedTask(mut bs: *mut bot_state_t) -> libc::c_int {
    if gametype == GT_CTF as libc::c_int {
        if (*bs).lastgoal_ltgtype == 6i32 {
            if BotTeam(bs) == TEAM_RED as libc::c_int {
                if (*bs).redflagstatus == 0i32 {
                    (*bs).lastgoal_ltgtype = 0i32
                }
            } else if (*bs).blueflagstatus == 0i32 {
                (*bs).lastgoal_ltgtype = 0i32
            }
        }
    }
    if 0 != (*bs).lastgoal_ltgtype {
        (*bs).decisionmaker = (*bs).lastgoal_decisionmaker;
        (*bs).ordered = qtrue as libc::c_int;
        (*bs).ltgtype = (*bs).lastgoal_ltgtype;
        memcpy(
            &mut (*bs).teamgoal as *mut bot_goal_t as *mut libc::c_void,
            &mut (*bs).lastgoal_teamgoal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        (*bs).teammate = (*bs).lastgoal_teammate;
        (*bs).teamgoal_time = floattime + 300i32 as libc::c_float;
        BotSetTeamStatus(bs);
        if gametype == GT_CTF as libc::c_int {
            if (*bs).ltgtype == 4i32 {
                let mut tb: *mut bot_goal_t = 0 as *mut bot_goal_t;
                let mut eb: *mut bot_goal_t = 0 as *mut bot_goal_t;
                let mut tt: libc::c_int = 0;
                let mut et: libc::c_int = 0;
                tb = BotTeamFlag(bs);
                eb = BotEnemyFlag(bs);
                tt = trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*tb).areanum,
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
                et = trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*eb).areanum,
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
                if et > tt {
                    BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
                }
            }
        }
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
}
//returns the opposite team of the bot
#[no_mangle]
pub unsafe extern "C" fn BotOppositeTeam(mut bs: *mut bot_state_t) -> libc::c_int {
    match BotTeam(bs) {
        1 => return TEAM_BLUE as libc::c_int,
        2 => return TEAM_RED as libc::c_int,
        _ => return TEAM_FREE as libc::c_int,
    };
}
//
//get a random alternate route goal towards the given base
#[no_mangle]
pub unsafe extern "C" fn BotGetAlternateRouteGoal(
    mut bs: *mut bot_state_t,
    mut base: libc::c_int,
) -> libc::c_int {
    let mut altroutegoals: *mut aas_altroutegoal_t = 0 as *mut aas_altroutegoal_t;
    let mut goal: *mut bot_goal_t = 0 as *mut bot_goal_t;
    let mut numaltroutegoals: libc::c_int = 0;
    let mut rnd: libc::c_int = 0;
    if base == TEAM_RED as libc::c_int {
        altroutegoals = red_altroutegoals.as_mut_ptr();
        numaltroutegoals = red_numaltroutegoals
    } else {
        altroutegoals = blue_altroutegoals.as_mut_ptr();
        numaltroutegoals = blue_numaltroutegoals
    }
    if 0 == numaltroutegoals {
        return qfalse as libc::c_int;
    }
    rnd = ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
        * numaltroutegoals as libc::c_float) as libc::c_int;
    if rnd >= numaltroutegoals {
        rnd = numaltroutegoals - 1i32
    }
    goal = &mut (*bs).altroutegoal;
    (*goal).areanum = (*altroutegoals.offset(rnd as isize)).areanum;
    (*goal).origin[0usize] = (*altroutegoals.offset(rnd as isize)).origin[0usize];
    (*goal).origin[1usize] = (*altroutegoals.offset(rnd as isize)).origin[1usize];
    (*goal).origin[2usize] = (*altroutegoals.offset(rnd as isize)).origin[2usize];
    (*goal).mins[0usize] = -8i32 as vec_t;
    (*goal).mins[1usize] = -8i32 as vec_t;
    (*goal).mins[2usize] = -8i32 as vec_t;
    (*goal).maxs[0usize] = 8i32 as vec_t;
    (*goal).maxs[1usize] = 8i32 as vec_t;
    (*goal).maxs[2usize] = 8i32 as vec_t;
    (*goal).entitynum = 0i32;
    (*goal).iteminfo = 0i32;
    (*goal).number = 0i32;
    (*goal).flags = 0i32;
    (*bs).reachedaltroutegoal_time = 0i32 as libc::c_float;
    return qtrue as libc::c_int;
}
#[no_mangle]
pub static mut blue_numaltroutegoals: libc::c_int = 0;
#[no_mangle]
pub static mut blue_altroutegoals: [aas_altroutegoal_t; 32] = [aas_altroutegoal_s {
    origin: [0.; 3],
    areanum: 0,
    starttraveltime: 0,
    goaltraveltime: 0,
    extratraveltime: 0,
}; 32];
#[no_mangle]
pub static mut red_numaltroutegoals: libc::c_int = 0;
#[no_mangle]
pub static mut red_altroutegoals: [aas_altroutegoal_t; 32] = [aas_altroutegoal_s {
    origin: [0.; 3],
    areanum: 0,
    starttraveltime: 0,
    goaltraveltime: 0,
    extratraveltime: 0,
}; 32];
/*
==================
BotEnemyFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotEnemyFlag(mut bs: *mut bot_state_t) -> *mut bot_goal_t {
    if BotTeam(bs) == TEAM_RED as libc::c_int {
        return &mut ctf_blueflag;
    } else {
        return &mut ctf_redflag;
    };
}
/*
==================
BotTeamFlag
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotTeamFlag(mut bs: *mut bot_state_t) -> *mut bot_goal_t {
    if BotTeam(bs) == TEAM_RED as libc::c_int {
        return &mut ctf_redflag;
    } else {
        return &mut ctf_blueflag;
    };
}
// selection of goals for teamplay
#[no_mangle]
pub unsafe extern "C" fn BotTeamGoals(mut bs: *mut bot_state_t, mut retreat: libc::c_int) {
    if 0 != retreat {
        if gametype == GT_CTF as libc::c_int {
            BotCTFRetreatGoals(bs);
        }
    } else if gametype == GT_CTF as libc::c_int {
        BotCTFSeekGoals(bs);
    }
    (*bs).order_time = 0i32 as libc::c_float;
}
//set ctf goals (defend base, get enemy flag) during seek
#[no_mangle]
pub unsafe extern "C" fn BotCTFSeekGoals(mut bs: *mut bot_state_t) {
    let mut rnd: libc::c_float = 0.;
    let mut l1: libc::c_float = 0.;
    let mut l2: libc::c_float = 0.;
    let mut flagstatus: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
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
    if 0 != BotCTFCarryingFlag(bs) {
        if (*bs).ltgtype != 5i32 {
            BotRefuseOrder(bs);
            (*bs).ltgtype = 5i32;
            (*bs).teamgoal_time = floattime + 120i32 as libc::c_float;
            (*bs).rushbaseaway_time = 0i32 as libc::c_float;
            (*bs).decisionmaker = (*bs).client;
            (*bs).ordered = qfalse as libc::c_int;
            match BotTeam(bs) {
                1 => {
                    dir[0usize] = (*bs).origin[0usize] - ctf_blueflag.origin[0usize];
                    dir[1usize] = (*bs).origin[1usize] - ctf_blueflag.origin[1usize];
                    dir[2usize] = (*bs).origin[2usize] - ctf_blueflag.origin[2usize]
                }
                2 => {
                    dir[0usize] = (*bs).origin[0usize] - ctf_redflag.origin[0usize];
                    dir[1usize] = (*bs).origin[1usize] - ctf_redflag.origin[1usize];
                    dir[2usize] = (*bs).origin[2usize] - ctf_redflag.origin[2usize]
                }
                _ => {
                    dir[0usize] = 999i32 as vec_t;
                    dir[1usize] = 999i32 as vec_t;
                    dir[2usize] = 999i32 as vec_t
                }
            }
            if VectorLength(dir.as_mut_ptr() as *const vec_t) < 128i32 as libc::c_float {
                BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
            } else {
                (*bs).altroutegoal.areanum = 0i32
            }
            BotSetUserInfo(
                bs,
                b"teamtask\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                va(
                    b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    TEAMTASK_OFFENSE as libc::c_int,
                ),
            );
            BotVoiceChat(
                bs,
                -1i32,
                b"ihaveflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if (*bs).rushbaseaway_time > floattime {
            if BotTeam(bs) == TEAM_RED as libc::c_int {
                flagstatus = (*bs).redflagstatus
            } else {
                flagstatus = (*bs).blueflagstatus
            }
            if flagstatus == 0i32 {
                (*bs).rushbaseaway_time = 0i32 as libc::c_float
            }
        }
        return;
    }
    if (*bs).ltgtype == 2i32 && 0 == (*bs).ordered {
        BotEntityInfo((*bs).teammate, &mut entinfo);
        if 0 == EntityCarriesFlag(&mut entinfo) as u64 {
            (*bs).ltgtype = 0i32
        }
    }
    if BotTeam(bs) == TEAM_RED as libc::c_int {
        flagstatus = (*bs).redflagstatus * 2i32 + (*bs).blueflagstatus
    } else {
        flagstatus = (*bs).blueflagstatus * 2i32 + (*bs).redflagstatus
    }
    if flagstatus == 1i32 {
        if ((*bs).owndecision_time as libc::c_float) < floattime {
            if !((*bs).ltgtype == 3i32
                && ((*bs).teamgoal.number == ctf_redflag.number
                    || (*bs).teamgoal.number == ctf_blueflag.number))
            {
                c = BotTeamFlagCarrierVisible(bs);
                if c >= 0i32 && ((*bs).ltgtype != 2i32 || (*bs).teammate != c) {
                    BotRefuseOrder(bs);
                    (*bs).decisionmaker = (*bs).client;
                    (*bs).ordered = qfalse as libc::c_int;
                    (*bs).teammate = c;
                    (*bs).teammatevisible_time = floattime;
                    (*bs).teammessage_time = 0i32 as libc::c_float;
                    (*bs).arrive_time = 1i32 as libc::c_float;
                    BotVoiceChat(
                        bs,
                        (*bs).teammate,
                        b"onfollow\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
                    (*bs).ltgtype = 2i32;
                    (*bs).formation_dist = (3.5f64 * 32i32 as libc::c_double) as libc::c_float;
                    BotSetTeamStatus(bs);
                    (*bs).owndecision_time = (floattime + 5i32 as libc::c_float) as libc::c_int
                }
            }
        }
        return;
    } else {
        if flagstatus == 2i32 {
            if ((*bs).owndecision_time as libc::c_float) < floattime {
                c = BotEnemyFlagCarrierVisible(bs);
                c >= 0i32;
                if (*bs).ltgtype != 4i32
                    && (*bs).ltgtype != 6i32
                    && (*bs).ltgtype != 1i32
                    && (*bs).ltgtype != 2i32
                    && (*bs).ltgtype != 8i32
                    && (*bs).ltgtype != 9i32
                    && (*bs).ltgtype != 10i32
                {
                    BotRefuseOrder(bs);
                    (*bs).decisionmaker = (*bs).client;
                    (*bs).ordered = qfalse as libc::c_int;
                    if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double)
                        < 0.5f64
                    {
                        (*bs).ltgtype = 4i32
                    } else {
                        (*bs).ltgtype = 6i32
                    }
                    (*bs).teammessage_time = 0i32 as libc::c_float;
                    (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
                    BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
                    BotSetTeamStatus(bs);
                    (*bs).owndecision_time = (floattime + 5i32 as libc::c_float) as libc::c_int
                }
            }
            return;
        } else {
            if flagstatus == 3i32 {
                if ((*bs).owndecision_time as libc::c_float) < floattime {
                    if (*bs).ltgtype != 6i32 && (*bs).ltgtype != 2i32 {
                        c = BotTeamFlagCarrierVisible(bs);
                        if c >= 0i32 {
                            BotRefuseOrder(bs);
                            (*bs).decisionmaker = (*bs).client;
                            (*bs).ordered = qfalse as libc::c_int;
                            (*bs).teammate = c;
                            (*bs).teammatevisible_time = floattime;
                            (*bs).teammessage_time = 0i32 as libc::c_float;
                            (*bs).arrive_time = 1i32 as libc::c_float;
                            BotVoiceChat(
                                bs,
                                (*bs).teammate,
                                b"onfollow\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
                            (*bs).ltgtype = 2i32;
                            (*bs).formation_dist =
                                (3.5f64 * 32i32 as libc::c_double) as libc::c_float;
                            BotSetTeamStatus(bs);
                            (*bs).owndecision_time =
                                (floattime + 5i32 as libc::c_float) as libc::c_int
                        } else {
                            BotRefuseOrder(bs);
                            (*bs).decisionmaker = (*bs).client;
                            (*bs).ordered = qfalse as libc::c_int;
                            (*bs).teammessage_time = floattime
                                + 2i32 as libc::c_float
                                    * ((rand() & 0x7fffi32) as libc::c_float
                                        / 0x7fffi32 as libc::c_float);
                            (*bs).ltgtype = 6i32;
                            (*bs).teamgoal_time = floattime + 180i32 as libc::c_float;
                            BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
                            BotSetTeamStatus(bs);
                            (*bs).owndecision_time =
                                (floattime + 5i32 as libc::c_float) as libc::c_int
                        }
                    }
                }
                return;
            }
        }
    }
    if 0 != BotTeamLeader(bs) {
        return;
    }
    if 0 != (*bs).lastgoal_ltgtype {
        (*bs).teamgoal_time += 60i32 as libc::c_float
    }
    if 0 == (*bs).ordered && 0 != (*bs).lastgoal_ltgtype {
        (*bs).ltgtype = 0i32
    }
    if (*bs).ltgtype == 1i32
        || (*bs).ltgtype == 2i32
        || (*bs).ltgtype == 3i32
        || (*bs).ltgtype == 4i32
        || (*bs).ltgtype == 5i32
        || (*bs).ltgtype == 6i32
        || (*bs).ltgtype == 8i32
        || (*bs).ltgtype == 9i32
        || (*bs).ltgtype == 10i32
        || (*bs).ltgtype == 14i32
        || (*bs).ltgtype == 15i32
    {
        return;
    }
    if 0 != BotSetLastOrderedTask(bs) {
        return;
    }
    if (*bs).owndecision_time as libc::c_float > floattime {
        return;
    }
    if (*bs).ctfroam_time > floattime {
        return;
    }
    if BotAggression(bs) < 50i32 as libc::c_float {
        return;
    }
    (*bs).teammessage_time = floattime
        + 2i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
    if 0 != (*bs).teamtaskpreference & (2i32 | 1i32) {
        if 0 != (*bs).teamtaskpreference & 2i32 {
            l1 = 0.7f32
        } else {
            l1 = 0.2f32
        }
        l2 = 0.9f32
    } else {
        l1 = 0.4f32;
        l2 = 0.7f32
    }
    rnd = (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float;
    if rnd < l1 && 0 != ctf_redflag.areanum && 0 != ctf_blueflag.areanum {
        (*bs).decisionmaker = (*bs).client;
        (*bs).ordered = qfalse as libc::c_int;
        (*bs).ltgtype = 4i32;
        (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
        BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
        BotSetTeamStatus(bs);
    } else if rnd < l2 && 0 != ctf_redflag.areanum && 0 != ctf_blueflag.areanum {
        (*bs).decisionmaker = (*bs).client;
        (*bs).ordered = qfalse as libc::c_int;
        if BotTeam(bs) == TEAM_RED as libc::c_int {
            memcpy(
                &mut (*bs).teamgoal as *mut bot_goal_t as *mut libc::c_void,
                &mut ctf_redflag as *mut bot_goal_t as *const libc::c_void,
                ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
            );
        } else {
            memcpy(
                &mut (*bs).teamgoal as *mut bot_goal_t as *mut libc::c_void,
                &mut ctf_blueflag as *mut bot_goal_t as *const libc::c_void,
                ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
            );
        }
        (*bs).ltgtype = 3i32;
        (*bs).teamgoal_time = floattime + 600i32 as libc::c_float;
        (*bs).defendaway_time = 0i32 as libc::c_float;
        BotSetTeamStatus(bs);
    } else {
        (*bs).ltgtype = 0i32;
        (*bs).ctfroam_time = floattime + 60i32 as libc::c_float;
        BotSetTeamStatus(bs);
    }
    (*bs).owndecision_time = (floattime + 5i32 as libc::c_float) as libc::c_int;
}
//returns the aggression of the bot in the range [0, 100]
#[no_mangle]
pub unsafe extern "C" fn BotAggression(mut bs: *mut bot_state_t) -> libc::c_float {
    if 0 != (*bs).inventory[35usize] {
        if (*bs).weaponnum != WP_GAUNTLET as libc::c_int || (*bs).inventory[200usize] < 80i32 {
            return 70i32 as libc::c_float;
        }
    }
    if (*bs).inventory[201usize] > 200i32 {
        return 0i32 as libc::c_float;
    }
    if (*bs).inventory[29usize] < 60i32 {
        return 0i32 as libc::c_float;
    }
    if (*bs).inventory[29usize] < 80i32 {
        if (*bs).inventory[1usize] < 40i32 {
            return 0i32 as libc::c_float;
        }
    }
    if (*bs).inventory[13usize] > 0i32 && (*bs).inventory[25usize] > 7i32 {
        return 100i32 as libc::c_float;
    }
    if (*bs).inventory[10usize] > 0i32 && (*bs).inventory[24usize] > 5i32 {
        return 95i32 as libc::c_float;
    }
    if (*bs).inventory[9usize] > 0i32 && (*bs).inventory[22usize] > 50i32 {
        return 90i32 as libc::c_float;
    }
    if (*bs).inventory[8usize] > 0i32 && (*bs).inventory[23usize] > 5i32 {
        return 90i32 as libc::c_float;
    }
    if (*bs).inventory[11usize] > 0i32 && (*bs).inventory[21usize] > 40i32 {
        return 85i32 as libc::c_float;
    }
    if (*bs).inventory[7usize] > 0i32 && (*bs).inventory[20usize] > 10i32 {
        return 80i32 as libc::c_float;
    }
    if (*bs).inventory[5usize] > 0i32 && (*bs).inventory[18usize] > 10i32 {
        return 50i32 as libc::c_float;
    }
    return 0i32 as libc::c_float;
}
/*
==================
BotRefuseOrder
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotRefuseOrder(mut bs: *mut bot_state_t) {
    if 0 == (*bs).ordered {
        return;
    }
    if 0. != (*bs).order_time && (*bs).order_time > floattime - 10i32 as libc::c_float {
        trap_EA_Action((*bs).client, 0x200000i32);
        BotVoiceChat(
            bs,
            (*bs).decisionmaker,
            b"no\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        (*bs).order_time = 0i32 as libc::c_float
    };
}
//returns visible team mate flag carrier if available
#[no_mangle]
pub unsafe extern "C" fn BotTeamFlagCarrierVisible(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut vis: libc::c_float = 0.;
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
    while i < level.maxclients {
        if !(i == (*bs).client) {
            BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(0 == entinfo.valid) {
                //if this player is carrying a flag
                if !(0 == EntityCarriesFlag(&mut entinfo) as u64) {
                    //if the flag carrier is not on the same team
                    if !(0 == BotSameTeam(bs, i)) {
                        vis = BotEntityVisible(
                            (*bs).entitynum,
                            (*bs).eye.as_mut_ptr(),
                            (*bs).viewangles.as_mut_ptr(),
                            360i32 as libc::c_float,
                            i,
                        );
                        if !(vis <= 0i32 as libc::c_float) {
                            return i;
                        }
                    }
                }
            }
        }
        i += 1
    }
    return -1i32;
}
//returns entity visibility in the range [0, 1]
#[no_mangle]
pub unsafe extern "C" fn BotEntityVisible(
    mut viewer: libc::c_int,
    mut eye: *mut vec_t,
    mut viewangles: *mut vec_t,
    mut fov: libc::c_float,
    mut ent: libc::c_int,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut contents_mask: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    let mut hitent: libc::c_int = 0;
    let mut infog: libc::c_int = 0;
    let mut inwater: libc::c_int = 0;
    let mut otherinfog: libc::c_int = 0;
    let mut pc: libc::c_int = 0;
    let mut squaredfogdist: libc::c_float = 0.;
    let mut waterfactor: libc::c_float = 0.;
    let mut vis: libc::c_float = 0.;
    let mut bestvis: libc::c_float = 0.;
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
    let mut dir: vec3_t = [0.; 3];
    let mut entangles: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut middle: vec3_t = [0.; 3];
    BotEntityInfo(ent, &mut entinfo);
    if 0 == entinfo.valid {
        return 0i32 as libc::c_float;
    }
    middle[0usize] = entinfo.mins[0usize] + entinfo.maxs[0usize];
    middle[1usize] = entinfo.mins[1usize] + entinfo.maxs[1usize];
    middle[2usize] = entinfo.mins[2usize] + entinfo.maxs[2usize];
    middle[0usize] = (middle[0usize] as libc::c_double * 0.5f64) as vec_t;
    middle[1usize] = (middle[1usize] as libc::c_double * 0.5f64) as vec_t;
    middle[2usize] = (middle[2usize] as libc::c_double * 0.5f64) as vec_t;
    middle[0usize] = entinfo.origin[0usize] + middle[0usize];
    middle[1usize] = entinfo.origin[1usize] + middle[1usize];
    middle[2usize] = entinfo.origin[2usize] + middle[2usize];
    dir[0usize] = middle[0usize] - *eye.offset(0isize);
    dir[1usize] = middle[1usize] - *eye.offset(1isize);
    dir[2usize] = middle[2usize] - *eye.offset(2isize);
    vectoangles(dir.as_mut_ptr() as *const vec_t, entangles.as_mut_ptr());
    if 0 == InFieldOfVision(viewangles, fov, entangles.as_mut_ptr()) as u64 {
        return 0i32 as libc::c_float;
    }
    pc = trap_AAS_PointContents(eye);
    infog = pc & 64i32;
    inwater = pc & (8i32 | 16i32 | 32i32);
    bestvis = 0i32 as libc::c_float;
    i = 0i32;
    while i < 3i32 {
        contents_mask = 1i32 | 0x10000i32;
        passent = viewer;
        hitent = ent;
        start[0usize] = *eye.offset(0isize);
        start[1usize] = *eye.offset(1isize);
        start[2usize] = *eye.offset(2isize);
        end[0usize] = middle[0usize];
        end[1usize] = middle[1usize];
        end[2usize] = middle[2usize];
        if 0 != trap_AAS_PointContents(middle.as_mut_ptr()) & (8i32 | 16i32 | 32i32) {
            contents_mask |= 8i32 | 16i32 | 32i32
        }
        if 0 != inwater {
            if 0 == contents_mask & (8i32 | 16i32 | 32i32) {
                passent = ent;
                hitent = viewer;
                start[0usize] = middle[0usize];
                start[1usize] = middle[1usize];
                start[2usize] = middle[2usize];
                end[0usize] = *eye.offset(0isize);
                end[1usize] = *eye.offset(1isize);
                end[2usize] = *eye.offset(2isize)
            }
            contents_mask ^= 8i32 | 16i32 | 32i32
        }
        BotAI_Trace(
            &mut trace,
            start.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
            end.as_mut_ptr(),
            passent,
            contents_mask,
        );
        waterfactor = 1.0f64 as libc::c_float;
        if 0 != trace.contents & (8i32 | 16i32 | 32i32) {
            contents_mask &= !(8i32 | 16i32 | 32i32);
            BotAI_Trace(
                &mut trace,
                trace.endpos.as_mut_ptr(),
                0 as *mut vec_t,
                0 as *mut vec_t,
                end.as_mut_ptr(),
                passent,
                contents_mask,
            );
            waterfactor = 0.5f64 as libc::c_float
        }
        if trace.fraction >= 1i32 as libc::c_float || trace.ent == hitent {
            otherinfog = trap_AAS_PointContents(middle.as_mut_ptr()) & 64i32;
            if 0 != infog && 0 != otherinfog {
                dir[0usize] = trace.endpos[0usize] - *eye.offset(0isize);
                dir[1usize] = trace.endpos[1usize] - *eye.offset(1isize);
                dir[2usize] = trace.endpos[2usize] - *eye.offset(2isize);
                squaredfogdist = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
            } else if 0 != infog {
                start[0usize] = trace.endpos[0usize];
                start[1usize] = trace.endpos[1usize];
                start[2usize] = trace.endpos[2usize];
                BotAI_Trace(
                    &mut trace,
                    start.as_mut_ptr(),
                    0 as *mut vec_t,
                    0 as *mut vec_t,
                    eye,
                    viewer,
                    64i32,
                );
                dir[0usize] = *eye.offset(0isize) - trace.endpos[0usize];
                dir[1usize] = *eye.offset(1isize) - trace.endpos[1usize];
                dir[2usize] = *eye.offset(2isize) - trace.endpos[2usize];
                squaredfogdist = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
            } else if 0 != otherinfog {
                end[0usize] = trace.endpos[0usize];
                end[1usize] = trace.endpos[1usize];
                end[2usize] = trace.endpos[2usize];
                BotAI_Trace(
                    &mut trace,
                    eye,
                    0 as *mut vec_t,
                    0 as *mut vec_t,
                    end.as_mut_ptr(),
                    viewer,
                    64i32,
                );
                dir[0usize] = end[0usize] - trace.endpos[0usize];
                dir[1usize] = end[1usize] - trace.endpos[1usize];
                dir[2usize] = end[2usize] - trace.endpos[2usize];
                squaredfogdist = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
            } else {
                squaredfogdist = 0i32 as libc::c_float
            }
            vis = (1i32 as libc::c_double
                / if squaredfogdist as libc::c_double * 0.001f64 < 1i32 as libc::c_double {
                    1i32 as libc::c_double
                } else {
                    squaredfogdist as libc::c_double * 0.001f64
                }) as libc::c_float;
            vis *= waterfactor;
            if vis > bestvis {
                bestvis = vis
            }
            if bestvis as libc::c_double >= 0.95f64 {
                return bestvis;
            }
        }
        if i == 0i32 {
            middle[2usize] += entinfo.mins[2usize]
        } else if i == 1i32 {
            middle[2usize] += entinfo.maxs[2usize] - entinfo.mins[2usize]
        }
        i += 1
    }
    return bestvis;
}
//returns true if within the field of vision for the given angles
#[no_mangle]
pub unsafe extern "C" fn InFieldOfVision(
    mut viewangles: *mut vec_t,
    mut fov: libc::c_float,
    mut angles: *mut vec_t,
) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut diff: libc::c_float = 0.;
    let mut angle: libc::c_float = 0.;
    i = 0i32;
    while i < 2i32 {
        angle = AngleMod(*viewangles.offset(i as isize));
        *angles.offset(i as isize) = AngleMod(*angles.offset(i as isize));
        diff = *angles.offset(i as isize) - angle;
        if *angles.offset(i as isize) > angle {
            if diff as libc::c_double > 180.0f64 {
                diff = (diff as libc::c_double - 360.0f64) as libc::c_float
            }
        } else if (diff as libc::c_double) < -180.0f64 {
            diff = (diff as libc::c_double + 360.0f64) as libc::c_float
        }
        if diff > 0i32 as libc::c_float {
            if diff as libc::c_double > fov as libc::c_double * 0.5f64 {
                return qfalse;
            }
        } else if (diff as libc::c_double) < -fov as libc::c_double * 0.5f64 {
            return qfalse;
        }
        i += 1
    }
    return qtrue;
}
//returns true if the bot and the entity are in the same team
#[no_mangle]
pub unsafe extern "C" fn BotSameTeam(
    mut bs: *mut bot_state_t,
    mut entnum: libc::c_int,
) -> libc::c_int {
    if (*bs).client < 0i32 || (*bs).client >= 64i32 {
        return qfalse as libc::c_int;
    }
    if entnum < 0i32 || entnum >= 64i32 {
        return qfalse as libc::c_int;
    }
    if gametype >= GT_TEAM as libc::c_int {
        if (*level.clients.offset((*bs).client as isize))
            .sess
            .sessionTeam as libc::c_uint
            == (*level.clients.offset(entnum as isize)).sess.sessionTeam as libc::c_uint
        {
            return qtrue as libc::c_int;
        }
    }
    return qfalse as libc::c_int;
}
//returns visible enemy flag carrier if available
#[no_mangle]
pub unsafe extern "C" fn BotEnemyFlagCarrierVisible(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut vis: libc::c_float = 0.;
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
    while i < level.maxclients {
        if !(i == (*bs).client) {
            BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(0 == entinfo.valid) {
                //if this player is carrying a flag
                if !(0 == EntityCarriesFlag(&mut entinfo) as u64) {
                    //if the flag carrier is on the same team
                    if !(0 != BotSameTeam(bs, i)) {
                        vis = BotEntityVisible(
                            (*bs).entitynum,
                            (*bs).eye.as_mut_ptr(),
                            (*bs).viewangles.as_mut_ptr(),
                            360i32 as libc::c_float,
                            i,
                        );
                        if !(vis <= 0i32 as libc::c_float) {
                            return i;
                        }
                    }
                }
            }
        }
        i += 1
    }
    return -1i32;
}
//set ctf goals (defend base, get enemy flag) during retreat
#[no_mangle]
pub unsafe extern "C" fn BotCTFRetreatGoals(mut bs: *mut bot_state_t) {
    if 0 != BotCTFCarryingFlag(bs) {
        if (*bs).ltgtype != 5i32 {
            BotRefuseOrder(bs);
            (*bs).ltgtype = 5i32;
            (*bs).teamgoal_time = floattime + 120i32 as libc::c_float;
            (*bs).rushbaseaway_time = 0i32 as libc::c_float;
            (*bs).decisionmaker = (*bs).client;
            (*bs).ordered = qfalse as libc::c_int;
            BotSetTeamStatus(bs);
        }
    };
}
//returns how bad the bot feels
#[no_mangle]
pub unsafe extern "C" fn BotFeelingBad(mut bs: *mut bot_state_t) -> libc::c_float {
    if (*bs).weaponnum == WP_GAUNTLET as libc::c_int {
        return 100i32 as libc::c_float;
    }
    if (*bs).inventory[29usize] < 40i32 {
        return 100i32 as libc::c_float;
    }
    if (*bs).weaponnum == WP_MACHINEGUN as libc::c_int {
        return 90i32 as libc::c_float;
    }
    if (*bs).inventory[29usize] < 60i32 {
        return 80i32 as libc::c_float;
    }
    return 0i32 as libc::c_float;
}
//returns true if the bot wants to retreat
#[no_mangle]
pub unsafe extern "C" fn BotWantsToRetreat(mut bs: *mut bot_state_t) -> libc::c_int {
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
    if gametype == GT_CTF as libc::c_int {
        if 0 != BotCTFCarryingFlag(bs) {
            return qtrue as libc::c_int;
        }
    }
    if (*bs).enemy >= 0i32 {
        BotEntityInfo((*bs).enemy, &mut entinfo);
        if 0 != EntityCarriesFlag(&mut entinfo) as u64 {
            return qfalse as libc::c_int;
        }
    }
    if (*bs).ltgtype == 4i32 {
        return qtrue as libc::c_int;
    }
    if BotAggression(bs) < 50i32 as libc::c_float {
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
}
//returns true if the bot wants to chase
#[no_mangle]
pub unsafe extern "C" fn BotWantsToChase(mut bs: *mut bot_state_t) -> libc::c_int {
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
    if gametype == GT_CTF as libc::c_int {
        if 0 != BotCTFCarryingFlag(bs) {
            return qfalse as libc::c_int;
        }
        BotEntityInfo((*bs).enemy, &mut entinfo);
        if 0 != EntityCarriesFlag(&mut entinfo) as u64 {
            return qtrue as libc::c_int;
        }
    }
    if (*bs).ltgtype == 4i32 {
        return qfalse as libc::c_int;
    }
    if BotAggression(bs) > 50i32 as libc::c_float {
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
}
//returns true if the bot wants to help
#[no_mangle]
pub unsafe extern "C" fn BotWantsToHelp(mut bs: *mut bot_state_t) -> libc::c_int {
    return qtrue as libc::c_int;
}
//returns true if the bot can and wants to rocketjump
#[no_mangle]
pub unsafe extern "C" fn BotCanAndWantsToRocketJump(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut rocketjumper: libc::c_float = 0.;
    if 0 == bot_rocketjump.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).inventory[8usize] <= 0i32 {
        return qfalse as libc::c_int;
    }
    if (*bs).inventory[23usize] < 3i32 {
        return qfalse as libc::c_int;
    }
    if 0 != (*bs).inventory[35usize] {
        return qfalse as libc::c_int;
    }
    if (*bs).inventory[29usize] < 60i32 {
        return qfalse as libc::c_int;
    }
    if (*bs).inventory[29usize] < 90i32 {
        if (*bs).inventory[1usize] < 40i32 {
            return qfalse as libc::c_int;
        }
    }
    rocketjumper = trap_Characteristic_BFloat(
        (*bs).character,
        38i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if (rocketjumper as libc::c_double) < 0.5f64 {
        return qfalse as libc::c_int;
    }
    return qtrue as libc::c_int;
}
// returns true if the bot has a persistant powerup and a weapon
#[no_mangle]
pub unsafe extern "C" fn BotHasPersistantPowerupAndWeapon(mut bs: *mut bot_state_t) -> libc::c_int {
    if (*bs).inventory[29usize] < 60i32 {
        return qfalse as libc::c_int;
    }
    if (*bs).inventory[29usize] < 80i32 {
        if (*bs).inventory[1usize] < 40i32 {
            return qfalse as libc::c_int;
        }
    }
    if (*bs).inventory[13usize] > 0i32 && (*bs).inventory[25usize] > 7i32 {
        return qtrue as libc::c_int;
    }
    if (*bs).inventory[10usize] > 0i32 && (*bs).inventory[24usize] > 5i32 {
        return qtrue as libc::c_int;
    }
    if (*bs).inventory[9usize] > 0i32 && (*bs).inventory[22usize] > 50i32 {
        return qtrue as libc::c_int;
    }
    if (*bs).inventory[8usize] > 0i32 && (*bs).inventory[23usize] > 5i32 {
        return qtrue as libc::c_int;
    }
    if (*bs).inventory[15usize] > 0i32 && (*bs).inventory[26usize] > 5i32 {
        return qtrue as libc::c_int;
    }
    if (*bs).inventory[16usize] > 0i32 && (*bs).inventory[27usize] > 5i32 {
        return qtrue as libc::c_int;
    }
    if (*bs).inventory[17usize] > 0i32 && (*bs).inventory[28usize] > 40i32 {
        return qtrue as libc::c_int;
    }
    if (*bs).inventory[11usize] > 0i32 && (*bs).inventory[21usize] > 20i32 {
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
}
//returns true if the bot wants to and goes camping
#[no_mangle]
pub unsafe extern "C" fn BotWantsToCamp(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut camper: libc::c_float = 0.;
    let mut cs: libc::c_int = 0;
    let mut traveltime: libc::c_int = 0;
    let mut besttraveltime: libc::c_int = 0;
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
    let mut bestgoal: bot_goal_t = bot_goal_s {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    camper = trap_Characteristic_BFloat(
        (*bs).character,
        44i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if (camper as libc::c_double) < 0.1f64 {
        return qfalse as libc::c_int;
    }
    if (*bs).ltgtype == 1i32
        || (*bs).ltgtype == 2i32
        || (*bs).ltgtype == 3i32
        || (*bs).ltgtype == 4i32
        || (*bs).ltgtype == 5i32
        || (*bs).ltgtype == 7i32
        || (*bs).ltgtype == 8i32
        || (*bs).ltgtype == 9i32
    {
        return qfalse as libc::c_int;
    }
    if (*bs).camp_time
        > floattime - 60i32 as libc::c_float
            + 300i32 as libc::c_float * (1i32 as libc::c_float - camper)
    {
        return qfalse as libc::c_int;
    }
    if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > camper {
        (*bs).camp_time = floattime;
        return qfalse as libc::c_int;
    }
    if BotAggression(bs) < 50i32 as libc::c_float {
        return qfalse as libc::c_int;
    }
    if ((*bs).inventory[8usize] <= 0i32 || (*bs).inventory[23usize] < 10i32)
        && ((*bs).inventory[10usize] <= 0i32 || (*bs).inventory[24usize] < 10i32)
        && ((*bs).inventory[13usize] <= 0i32 || (*bs).inventory[25usize] < 10i32)
    {
        return qfalse as libc::c_int;
    }
    besttraveltime = 99999i32;
    cs = trap_BotGetNextCampSpotGoal(0i32, &mut goal as *mut bot_goal_t as *mut libc::c_void);
    while 0 != cs {
        traveltime = trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            goal.areanum,
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
        if 0 != traveltime && traveltime < besttraveltime {
            besttraveltime = traveltime;
            memcpy(
                &mut bestgoal as *mut bot_goal_t as *mut libc::c_void,
                &mut goal as *mut bot_goal_t as *const libc::c_void,
                ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
            );
        }
        cs = trap_BotGetNextCampSpotGoal(cs, &mut goal as *mut bot_goal_t as *mut libc::c_void)
    }
    if besttraveltime > 150i32 {
        return qfalse as libc::c_int;
    }
    BotGoCamp(bs, &mut bestgoal);
    (*bs).ordered = qfalse as libc::c_int;
    return qtrue as libc::c_int;
}
/*
==================
BotGoCamp
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGoCamp(mut bs: *mut bot_state_t, mut goal: *mut bot_goal_t) {
    let mut camper: libc::c_float = 0.;
    (*bs).decisionmaker = (*bs).client;
    (*bs).teammessage_time = 0i32 as libc::c_float;
    (*bs).ltgtype = 7i32;
    memcpy(
        &mut (*bs).teamgoal as *mut bot_goal_t as *mut libc::c_void,
        goal as *const libc::c_void,
        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
    );
    camper = trap_Characteristic_BFloat(
        (*bs).character,
        44i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if camper as libc::c_double > 0.99f64 {
        (*bs).teamgoal_time = floattime + 99999i32 as libc::c_float
    } else {
        (*bs).teamgoal_time = floattime
            + 120i32 as libc::c_float
            + 180i32 as libc::c_float * camper
            + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                * 15i32 as libc::c_float
    }
    (*bs).camp_time = floattime;
    (*bs).teammate = 0i32;
    (*bs).arrive_time = 1i32 as libc::c_float;
}
//the bot will perform attack movements
#[no_mangle]
pub unsafe extern "C" fn BotAttackMove(
    mut bs: *mut bot_state_t,
    mut tfl: libc::c_int,
) -> bot_moveresult_t {
    let mut movetype: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut attackentity: libc::c_int = 0;
    let mut attack_skill: libc::c_float = 0.;
    let mut jumper: libc::c_float = 0.;
    let mut croucher: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut strafechange_time: libc::c_float = 0.;
    let mut attack_dist: libc::c_float = 0.;
    let mut attack_range: libc::c_float = 0.;
    let mut forward: vec3_t = [0.; 3];
    let mut backward: vec3_t = [0.; 3];
    let mut sideward: vec3_t = [0.; 3];
    let mut hordir: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
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
    let mut moveresult: bot_moveresult_t = bot_moveresult_s {
        failure: 0,
        type_0: 0,
        blocked: 0,
        blockentity: 0,
        traveltype: 0,
        flags: 0,
        weapon: 0,
        movedir: [0.; 3],
        ideal_viewangles: [0.; 3],
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
    attackentity = (*bs).enemy;
    if (*bs).attackchase_time > floattime {
        goal.entitynum = attackentity;
        goal.areanum = (*bs).lastenemyareanum;
        goal.origin[0usize] = (*bs).lastenemyorigin[0usize];
        goal.origin[1usize] = (*bs).lastenemyorigin[1usize];
        goal.origin[2usize] = (*bs).lastenemyorigin[2usize];
        goal.mins[0usize] = -8i32 as vec_t;
        goal.mins[1usize] = -8i32 as vec_t;
        goal.mins[2usize] = -8i32 as vec_t;
        goal.maxs[0usize] = 8i32 as vec_t;
        goal.maxs[1usize] = 8i32 as vec_t;
        goal.maxs[2usize] = 8i32 as vec_t;
        BotSetupForMovement(bs);
        trap_BotMoveToGoal(
            &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
            (*bs).ms,
            &mut goal as *mut bot_goal_t as *mut libc::c_void,
            tfl,
        );
        return moveresult;
    }
    memset(
        &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<bot_moveresult_t>() as libc::c_ulong,
    );
    attack_skill = trap_Characteristic_BFloat(
        (*bs).character,
        2i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    jumper = trap_Characteristic_BFloat(
        (*bs).character,
        37i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    croucher = trap_Characteristic_BFloat(
        (*bs).character,
        36i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if (attack_skill as libc::c_double) < 0.2f64 {
        return moveresult;
    }
    BotSetupForMovement(bs);
    BotEntityInfo(attackentity, &mut entinfo);
    forward[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
    forward[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
    forward[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
    dist = VectorNormalize(forward.as_mut_ptr());
    backward[0usize] = -forward[0usize];
    backward[1usize] = -forward[1usize];
    backward[2usize] = -forward[2usize];
    movetype = 1i32;
    if (*bs).attackcrouch_time < floattime - 1i32 as libc::c_float {
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) < jumper {
            movetype = 4i32
        } else if (*bs).attackcrouch_time < floattime - 1i32 as libc::c_float
            && ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) < croucher
        {
            (*bs).attackcrouch_time = floattime + croucher * 5i32 as libc::c_float
        }
    }
    if (*bs).attackcrouch_time > floattime {
        movetype = 2i32
    }
    if movetype == 4i32 {
        if (*bs).attackjump_time > floattime {
            movetype = 1i32
        } else {
            (*bs).attackjump_time = floattime + 1i32 as libc::c_float
        }
    }
    if (*bs).cur_ps.weapon == WP_GAUNTLET as libc::c_int {
        attack_dist = 0i32 as libc::c_float;
        attack_range = 0i32 as libc::c_float
    } else {
        attack_dist = 140i32 as libc::c_float;
        attack_range = 40i32 as libc::c_float
    }
    if attack_skill as libc::c_double <= 0.4f64 {
        if dist > attack_dist + attack_range {
            if 0 != trap_BotMoveInDirection(
                (*bs).ms,
                forward.as_mut_ptr(),
                400i32 as libc::c_float,
                movetype,
            ) {
                return moveresult;
            }
        }
        if dist < attack_dist - attack_range {
            if 0 != trap_BotMoveInDirection(
                (*bs).ms,
                backward.as_mut_ptr(),
                400i32 as libc::c_float,
                movetype,
            ) {
                return moveresult;
            }
        }
        return moveresult;
    }
    (*bs).attackstrafe_time += (*bs).thinktime;
    strafechange_time = (0.4f64 + (1i32 as libc::c_float - attack_skill) as libc::c_double * 0.2f64)
        as libc::c_float;
    if attack_skill as libc::c_double > 0.7f64 {
        strafechange_time = (strafechange_time as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 0.2f64) as libc::c_float
    }
    if (*bs).attackstrafe_time > strafechange_time {
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            > 0.935f64
        {
            (*bs).flags ^= 1i32;
            (*bs).attackstrafe_time = 0i32 as libc::c_float
        }
    }
    i = 0i32;
    while i < 2i32 {
        hordir[0usize] = forward[0usize];
        hordir[1usize] = forward[1usize];
        hordir[2usize] = 0i32 as vec_t;
        VectorNormalize(hordir.as_mut_ptr());
        CrossProduct(
            hordir.as_mut_ptr() as *const vec_t,
            up.as_mut_ptr() as *const vec_t,
            sideward.as_mut_ptr(),
        );
        if 0 != (*bs).flags & 1i32 {
            sideward[0usize] = -sideward[0usize];
            sideward[1usize] = -sideward[1usize];
            sideward[2usize] = -sideward[2usize]
        }
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            > 0.9f64
        {
            sideward[0usize] = sideward[0usize] + backward[0usize];
            sideward[1usize] = sideward[1usize] + backward[1usize];
            sideward[2usize] = sideward[2usize] + backward[2usize]
        } else if dist > attack_dist + attack_range {
            sideward[0usize] = sideward[0usize] + forward[0usize];
            sideward[1usize] = sideward[1usize] + forward[1usize];
            sideward[2usize] = sideward[2usize] + forward[2usize]
        } else if dist < attack_dist - attack_range {
            sideward[0usize] = sideward[0usize] + backward[0usize];
            sideward[1usize] = sideward[1usize] + backward[1usize];
            sideward[2usize] = sideward[2usize] + backward[2usize]
        }
        if 0 != trap_BotMoveInDirection(
            (*bs).ms,
            sideward.as_mut_ptr(),
            400i32 as libc::c_float,
            movetype,
        ) {
            return moveresult;
        }
        (*bs).flags ^= 1i32;
        (*bs).attackstrafe_time = 0i32 as libc::c_float;
        i += 1
    }
    return moveresult;
}
// returns the client number of the team mate flag carrier (-1 if none)
#[no_mangle]
pub unsafe extern "C" fn BotTeamFlagCarrier(mut bs: *mut bot_state_t) -> libc::c_int {
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
    while i < level.maxclients {
        if !(i == (*bs).client) {
            BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(0 == entinfo.valid) {
                //if this player is carrying a flag
                if !(0 == EntityCarriesFlag(&mut entinfo) as u64) {
                    //if the flag carrier is not on the same team
                    if !(0 == BotSameTeam(bs, i)) {
                        return i;
                    }
                }
            }
        }
        i += 1
    }
    return -1i32;
}
//get the number of visible teammates and enemies
#[no_mangle]
pub unsafe extern "C" fn BotVisibleTeamMatesAndEnemies(
    mut bs: *mut bot_state_t,
    mut teammates: *mut libc::c_int,
    mut enemies: *mut libc::c_int,
    mut range: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut vis: libc::c_float = 0.;
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
    let mut dir: vec3_t = [0.; 3];
    if !teammates.is_null() {
        *teammates = 0i32
    }
    if !enemies.is_null() {
        *enemies = 0i32
    }
    i = 0i32;
    while i < level.maxclients {
        if !(i == (*bs).client) {
            BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(0 == entinfo.valid) {
                //if this player is carrying a flag
                if !(0 == EntityCarriesFlag(&mut entinfo) as u64) {
                    dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
                    dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
                    dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
                    if !(VectorLengthSquared(dir.as_mut_ptr() as *const vec_t) > range * range) {
                        vis = BotEntityVisible(
                            (*bs).entitynum,
                            (*bs).eye.as_mut_ptr(),
                            (*bs).viewangles.as_mut_ptr(),
                            360i32 as libc::c_float,
                            i,
                        );
                        if !(vis <= 0i32 as libc::c_float) {
                            if 0 != BotSameTeam(bs, i) {
                                if !teammates.is_null() {
                                    *teammates += 1
                                }
                            } else if !enemies.is_null() {
                                *enemies += 1
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
}
//returns true and sets the .enemy field when an enemy is found
#[no_mangle]
pub unsafe extern "C" fn BotFindEnemy(
    mut bs: *mut bot_state_t,
    mut curenemy: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut healthdecrease: libc::c_int = 0;
    let mut f: libc::c_float = 0.;
    let mut alertness: libc::c_float = 0.;
    let mut easyfragger: libc::c_float = 0.;
    let mut vis: libc::c_float = 0.;
    let mut squaredist: libc::c_float = 0.;
    let mut cursquaredist: libc::c_float = 0.;
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
    let mut curenemyinfo: aas_entityinfo_t = aas_entityinfo_s {
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
    let mut dir: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    alertness = trap_Characteristic_BFloat(
        (*bs).character,
        46i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    easyfragger = trap_Characteristic_BFloat(
        (*bs).character,
        45i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    healthdecrease = ((*bs).lasthealth > (*bs).inventory[29usize]) as libc::c_int;
    (*bs).lasthealth = (*bs).inventory[29usize];
    if curenemy >= 0i32 {
        BotEntityInfo(curenemy, &mut curenemyinfo);
        if 0 != EntityCarriesFlag(&mut curenemyinfo) as u64 {
            return qfalse as libc::c_int;
        }
        dir[0usize] = curenemyinfo.origin[0usize] - (*bs).origin[0usize];
        dir[1usize] = curenemyinfo.origin[1usize] - (*bs).origin[1usize];
        dir[2usize] = curenemyinfo.origin[2usize] - (*bs).origin[2usize];
        cursquaredist = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
    } else {
        cursquaredist = 0i32 as libc::c_float
    }
    let mut current_block_32: u64;
    i = 0i32;
    while i < level.maxclients {
        if !(i == (*bs).client) {
            //if it's the current enemy
            if !(i == curenemy) {
                //if the enemy has targeting disabled
                if !(0 != g_entities[i as usize].flags & 0x20i32) {
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
                                //if not an easy fragger don't shoot at chatting players
                                if !((easyfragger as libc::c_double) < 0.5f64
                                    && 0 != EntityIsChatting(&mut entinfo) as libc::c_uint)
                                {
                                    //
                                    if lastteleport_time > floattime - 3i32 as libc::c_float {
                                        dir[0usize] =
                                            entinfo.origin[0usize] - lastteleport_origin[0usize];
                                        dir[1usize] =
                                            entinfo.origin[1usize] - lastteleport_origin[1usize];
                                        dir[2usize] =
                                            entinfo.origin[2usize] - lastteleport_origin[2usize];
                                        if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                                            < (70i32 * 70i32) as libc::c_float
                                        {
                                            current_block_32 = 8236137900636309791;
                                        } else {
                                            current_block_32 = 5689316957504528238;
                                        }
                                    } else {
                                        current_block_32 = 5689316957504528238;
                                    }
                                    match current_block_32 {
                                        8236137900636309791 => {}
                                        _ => {
                                            dir[0usize] =
                                                entinfo.origin[0usize] - (*bs).origin[0usize];
                                            dir[1usize] =
                                                entinfo.origin[1usize] - (*bs).origin[1usize];
                                            dir[2usize] =
                                                entinfo.origin[2usize] - (*bs).origin[2usize];
                                            squaredist = VectorLengthSquared(
                                                dir.as_mut_ptr() as *const vec_t
                                            );
                                            //if this entity is not carrying a flag
                                            if 0 == EntityCarriesFlag(&mut entinfo) as u64 {
                                                //if this enemy is further away than the current one
                                                if curenemy >= 0i32 && squaredist > cursquaredist {
                                                    current_block_32 = 8236137900636309791;
                                                } else {
                                                    current_block_32 = 18153031941552419006;
                                                }
                                            } else {
                                                current_block_32 = 18153031941552419006;
                                            }
                                            match current_block_32 {
                                                8236137900636309791 => {}
                                                _ => {
                                                    //end if
                                                    //if the bot has no
                                                    if !(squaredist as libc::c_double
                                                        > (900.0f64
                                                            + alertness as libc::c_double
                                                                * 4000.0f64)
                                                            * (900.0f64
                                                                + alertness as libc::c_double
                                                                    * 4000.0f64))
                                                    {
                                                        //if on the same team
                                                        if !(0 != BotSameTeam(bs, i)) {
                                                            if curenemy < 0i32
                                                                && (0 != healthdecrease
                                                                    || 0 != EntityIsShooting(
                                                                        &mut entinfo,
                                                                    )
                                                                        as libc::c_uint)
                                                            {
                                                                f = 360i32 as libc::c_float
                                                            } else {
                                                                f = (90i32 + 90i32) as libc::c_float
                                                                    - (90i32 as libc::c_float
                                                                        - if squaredist
                                                                            > (810i32 * 810i32)
                                                                                as libc::c_float
                                                                        {
                                                                            (810i32 * 810i32)
                                                                                as libc::c_float
                                                                        } else {
                                                                            squaredist
                                                                        } / (810i32 * 9i32)
                                                                            as libc::c_float)
                                                            }
                                                            vis = BotEntityVisible(
                                                                (*bs).entitynum,
                                                                (*bs).eye.as_mut_ptr(),
                                                                (*bs).viewangles.as_mut_ptr(),
                                                                f,
                                                                i,
                                                            );
                                                            if !(vis <= 0i32 as libc::c_float) {
                                                                //if the enemy is quite far away, not shooting and the bot is not damaged
                                                                if curenemy < 0i32
                                                                    && squaredist
                                                                        > (100i32 * 100i32)
                                                                            as libc::c_float
                                                                    && 0 == healthdecrease
                                                                    && 0 == EntityIsShooting(
                                                                        &mut entinfo,
                                                                    )
                                                                        as u64
                                                                {
                                                                    dir[0usize] = (*bs).origin
                                                                        [0usize]
                                                                        - entinfo.origin[0usize];
                                                                    dir[1usize] = (*bs).origin
                                                                        [1usize]
                                                                        - entinfo.origin[1usize];
                                                                    dir[2usize] = (*bs).origin
                                                                        [2usize]
                                                                        - entinfo.origin[2usize];
                                                                    vectoangles(
                                                                        dir.as_mut_ptr()
                                                                            as *const vec_t,
                                                                        angles.as_mut_ptr(),
                                                                    );
                                                                    //if the bot isn't in the fov of the enemy
                                                                    if 0 == InFieldOfVision(
                                                                        entinfo.angles.as_mut_ptr(),
                                                                        90i32 as libc::c_float,
                                                                        angles.as_mut_ptr(),
                                                                    )
                                                                        as u64
                                                                    {
                                                                        BotUpdateBattleInventory(
                                                                            bs, i,
                                                                        );
                                                                        //if the bot doesn't really want to fight
                                                                        if 0 != BotWantsToRetreat(
                                                                            bs,
                                                                        ) {
                                                                            current_block_32 =
                                                                                8236137900636309791;
                                                                        } else {
                                                                            current_block_32
                                                                                =
                                                                                12497913735442871383;
                                                                        }
                                                                    } else {
                                                                        current_block_32 =
                                                                            12497913735442871383;
                                                                    }
                                                                } else {
                                                                    current_block_32 =
                                                                        12497913735442871383;
                                                                }
                                                                match current_block_32 {
                                                                    8236137900636309791 => {}
                                                                    _ => {
                                                                        (*bs).enemy =
                                                                            entinfo.number;
                                                                        if curenemy >= 0i32 {
                                                                            (*bs).enemysight_time
                                                                                =
                                                                                floattime
                                                                                    -
                                                                                    2i32
                                                                                        as
                                                                                        libc::c_float
                                                                        } else {
                                                                            (*bs).enemysight_time =
                                                                                floattime
                                                                        }
                                                                        (*bs).enemysuicide =
                                                                            qfalse as libc::c_int;
                                                                        (*bs).enemydeath_time =
                                                                            0i32 as libc::c_float;
                                                                        (*bs).enemyvisible_time =
                                                                            floattime;
                                                                        return qtrue as libc::c_int;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
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
/*
==================
EntityIsChatting
==================
*/
#[no_mangle]
pub unsafe extern "C" fn EntityIsChatting(mut entinfo: *mut aas_entityinfo_t) -> qboolean {
    if 0 != (*entinfo).flags & 0x1000i32 {
        return qtrue;
    }
    return qfalse;
}
//returns a roam goal
#[no_mangle]
pub unsafe extern "C" fn BotRoamGoal(mut bs: *mut bot_state_t, mut goal: *mut vec_t) {
    let mut pc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_float = 0.;
    let mut rnd: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut bestorg: vec3_t = [0.; 3];
    let mut belowbestorg: vec3_t = [0.; 3];
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
    i = 0i32;
    while i < 10i32 {
        bestorg[0usize] = (*bs).origin[0usize];
        bestorg[1usize] = (*bs).origin[1usize];
        bestorg[2usize] = (*bs).origin[2usize];
        rnd = (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float;
        if rnd as libc::c_double > 0.25f64 {
            if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double)
                < 0.5f64
            {
                bestorg[0usize] -= 800i32 as libc::c_float
                    * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    + 100i32 as libc::c_float
            } else {
                bestorg[0usize] += 800i32 as libc::c_float
                    * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    + 100i32 as libc::c_float
            }
        }
        if (rnd as libc::c_double) < 0.75f64 {
            if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double)
                < 0.5f64
            {
                bestorg[1usize] -= 800i32 as libc::c_float
                    * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    + 100i32 as libc::c_float
            } else {
                bestorg[1usize] += 800i32 as libc::c_float
                    * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    + 100i32 as libc::c_float
            }
        }
        bestorg[2usize] = (bestorg[2usize] as libc::c_double
            + (2i32 * 48i32) as libc::c_double
                * (2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64))) as vec_t;
        BotAI_Trace(
            &mut trace,
            (*bs).origin.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
            bestorg.as_mut_ptr(),
            (*bs).entitynum,
            1i32,
        );
        dir[0usize] = trace.endpos[0usize] - (*bs).origin[0usize];
        dir[1usize] = trace.endpos[1usize] - (*bs).origin[1usize];
        dir[2usize] = trace.endpos[2usize] - (*bs).origin[2usize];
        len = VectorNormalize(dir.as_mut_ptr());
        if len > 200i32 as libc::c_float {
            dir[0usize] = dir[0usize] * (len * trace.fraction - 40i32 as libc::c_float);
            dir[1usize] = dir[1usize] * (len * trace.fraction - 40i32 as libc::c_float);
            dir[2usize] = dir[2usize] * (len * trace.fraction - 40i32 as libc::c_float);
            bestorg[0usize] = (*bs).origin[0usize] + dir[0usize];
            bestorg[1usize] = (*bs).origin[1usize] + dir[1usize];
            bestorg[2usize] = (*bs).origin[2usize] + dir[2usize];
            belowbestorg[0usize] = bestorg[0usize];
            belowbestorg[1usize] = bestorg[1usize];
            belowbestorg[2usize] = bestorg[2usize] - 800i32 as libc::c_float;
            BotAI_Trace(
                &mut trace,
                bestorg.as_mut_ptr(),
                0 as *mut vec_t,
                0 as *mut vec_t,
                belowbestorg.as_mut_ptr(),
                (*bs).entitynum,
                1i32,
            );
            if 0 == trace.startsolid as u64 {
                trace.endpos[2usize] += 1.0;
                pc = trap_PointContents(trace.endpos.as_mut_ptr() as *const vec_t, (*bs).entitynum);
                if 0 == pc & (8i32 | 16i32) {
                    *goal.offset(0isize) = bestorg[0usize];
                    *goal.offset(1isize) = bestorg[1usize];
                    *goal.offset(2isize) = bestorg[2usize];
                    return;
                }
            }
        }
        i += 1
    }
    *goal.offset(0isize) = bestorg[0usize];
    *goal.offset(1isize) = bestorg[1usize];
    *goal.offset(2isize) = bestorg[2usize];
}
//the bot will aim at the current enemy
#[no_mangle]
pub unsafe extern "C" fn BotAimAtEnemy(mut bs: *mut bot_state_t) {
    let mut i: libc::c_int = 0;
    let mut enemyvisible: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    let mut aim_skill: libc::c_float = 0.;
    let mut aim_accuracy: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut reactiontime: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut bestorigin: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut groundtarget: vec3_t = [0.; 3];
    let mut cmdmove: vec3_t = [0.; 3];
    let mut enemyvelocity: vec3_t = [0.; 3];
    let mut mins: vec3_t = [-4i32 as vec_t, -4i32 as vec_t, -4i32 as vec_t];
    let mut maxs: vec3_t = [4i32 as vec_t, 4i32 as vec_t, 4i32 as vec_t];
    let mut wi: weaponinfo_t = weaponinfo_s {
        valid: 0,
        number: 0,
        name: [0; 80],
        model: [0; 80],
        level: 0,
        weaponindex: 0,
        flags: 0,
        projectile: [0; 80],
        numprojectiles: 0,
        hspread: 0.,
        vspread: 0.,
        speed: 0.,
        acceleration: 0.,
        recoil: [0.; 3],
        offset: [0.; 3],
        angleoffset: [0.; 3],
        extrazvelocity: 0.,
        ammoamount: 0,
        ammoindex: 0,
        activate: 0.,
        reload: 0.,
        spinup: 0.,
        spindown: 0.,
        proj: projectileinfo_s {
            name: [0; 80],
            model: [0; 80],
            flags: 0,
            gravity: 0.,
            damage: 0,
            radius: 0.,
            visdamage: 0,
            damagetype: 0,
            healthinc: 0,
            push: 0.,
            detonation: 0.,
            bounce: 0.,
            bouncefric: 0.,
            bouncestop: 0.,
        },
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
    let mut target: vec3_t = [0.; 3];
    if (*bs).enemy < 0i32 {
        return;
    }
    BotEntityInfo((*bs).enemy, &mut entinfo);
    if (*bs).enemy >= 64i32 {
        target[0usize] = entinfo.origin[0usize];
        target[1usize] = entinfo.origin[1usize];
        target[2usize] = entinfo.origin[2usize];
        dir[0usize] = target[0usize] - (*bs).eye[0usize];
        dir[1usize] = target[1usize] - (*bs).eye[1usize];
        dir[2usize] = target[2usize] - (*bs).eye[2usize];
        vectoangles(
            dir.as_mut_ptr() as *const vec_t,
            (*bs).ideal_viewangles.as_mut_ptr(),
        );
        (*bs).aimtarget[0usize] = target[0usize];
        (*bs).aimtarget[1usize] = target[1usize];
        (*bs).aimtarget[2usize] = target[2usize];
        return;
    }
    aim_skill = trap_Characteristic_BFloat(
        (*bs).character,
        16i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    aim_accuracy = trap_Characteristic_BFloat(
        (*bs).character,
        7i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if aim_skill as libc::c_double > 0.95f64 {
        reactiontime = (0.5f64
            * trap_Characteristic_BFloat(
                (*bs).character,
                6i32,
                0i32 as libc::c_float,
                1i32 as libc::c_float,
            ) as libc::c_double) as libc::c_float;
        if (*bs).enemysight_time > floattime - reactiontime {
            return;
        }
        if (*bs).teleport_time > floattime - reactiontime {
            return;
        }
    }
    trap_BotGetWeaponInfo(
        (*bs).ws,
        (*bs).weaponnum,
        &mut wi as *mut weaponinfo_t as *mut libc::c_void,
    );
    if wi.number == WP_MACHINEGUN as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            8i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    } else if wi.number == WP_SHOTGUN as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            9i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    } else if wi.number == WP_GRENADE_LAUNCHER as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            11i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        );
        aim_skill = trap_Characteristic_BFloat(
            (*bs).character,
            18i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    } else if wi.number == WP_ROCKET_LAUNCHER as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            10i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        );
        aim_skill = trap_Characteristic_BFloat(
            (*bs).character,
            17i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    } else if wi.number == WP_LIGHTNING as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            12i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    } else if wi.number == WP_RAILGUN as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            14i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    } else if wi.number == WP_PLASMAGUN as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            13i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        );
        aim_skill = trap_Characteristic_BFloat(
            (*bs).character,
            19i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    } else if wi.number == WP_BFG as libc::c_int {
        aim_accuracy = trap_Characteristic_BFloat(
            (*bs).character,
            15i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        );
        aim_skill = trap_Characteristic_BFloat(
            (*bs).character,
            20i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        )
    }
    if aim_accuracy <= 0i32 as libc::c_float {
        aim_accuracy = 0.0001f32
    }
    BotEntityInfo((*bs).enemy, &mut entinfo);
    if 0 != EntityIsInvisible(&mut entinfo) as u64 {
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            > 0.1f64
        {
            aim_accuracy *= 0.4f32
        }
    }
    enemyvelocity[0usize] = entinfo.origin[0usize] - entinfo.lastvisorigin[0usize];
    enemyvelocity[1usize] = entinfo.origin[1usize] - entinfo.lastvisorigin[1usize];
    enemyvelocity[2usize] = entinfo.origin[2usize] - entinfo.lastvisorigin[2usize];
    enemyvelocity[0usize] = enemyvelocity[0usize] * (1i32 as libc::c_float / entinfo.update_time);
    enemyvelocity[1usize] = enemyvelocity[1usize] * (1i32 as libc::c_float / entinfo.update_time);
    enemyvelocity[2usize] = enemyvelocity[2usize] * (1i32 as libc::c_float / entinfo.update_time);
    if (*bs).enemyposition_time < floattime {
        (*bs).enemyposition_time = (floattime as libc::c_double + 0.5f64) as libc::c_float;
        (*bs).enemyvelocity[0usize] = enemyvelocity[0usize];
        (*bs).enemyvelocity[1usize] = enemyvelocity[1usize];
        (*bs).enemyvelocity[2usize] = enemyvelocity[2usize];
        (*bs).enemyorigin[0usize] = entinfo.origin[0usize];
        (*bs).enemyorigin[1usize] = entinfo.origin[1usize];
        (*bs).enemyorigin[2usize] = entinfo.origin[2usize]
    }
    if (aim_skill as libc::c_double) < 0.9f64 {
        dir[0usize] = entinfo.origin[0usize] - (*bs).enemyorigin[0usize];
        dir[1usize] = entinfo.origin[1usize] - (*bs).enemyorigin[1usize];
        dir[2usize] = entinfo.origin[2usize] - (*bs).enemyorigin[2usize];
        if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t) > (48i32 * 48i32) as libc::c_float
        {
            if (*bs).enemyvelocity[0usize] * enemyvelocity[0usize]
                + (*bs).enemyvelocity[1usize] * enemyvelocity[1usize]
                + (*bs).enemyvelocity[2usize] * enemyvelocity[2usize]
                < 0i32 as libc::c_float
            {
                aim_accuracy *= 0.7f32
            }
        }
    }
    enemyvisible = BotEntityVisible(
        (*bs).entitynum,
        (*bs).eye.as_mut_ptr(),
        (*bs).viewangles.as_mut_ptr(),
        360i32 as libc::c_float,
        (*bs).enemy,
    ) as libc::c_int;
    if 0 != enemyvisible {
        bestorigin[0usize] = entinfo.origin[0usize];
        bestorigin[1usize] = entinfo.origin[1usize];
        bestorigin[2usize] = entinfo.origin[2usize];
        bestorigin[2usize] += 8i32 as libc::c_float;
        start[0usize] = (*bs).origin[0usize];
        start[1usize] = (*bs).origin[1usize];
        start[2usize] = (*bs).origin[2usize];
        start[2usize] += (*bs).cur_ps.viewheight as libc::c_float;
        start[2usize] += wi.offset[2usize];
        BotAI_Trace(
            &mut trace,
            start.as_mut_ptr(),
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
            bestorigin.as_mut_ptr(),
            (*bs).entitynum,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        if trace.fraction <= 1i32 as libc::c_float && trace.ent != entinfo.number {
            bestorigin[2usize] += 16i32 as libc::c_float
        }
        if 0. != wi.speed {
            dir[0usize] = bestorigin[0usize] - (*bs).origin[0usize];
            dir[1usize] = bestorigin[1usize] - (*bs).origin[1usize];
            dir[2usize] = bestorigin[2usize] - (*bs).origin[2usize];
            dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
            dir[0usize] = entinfo.origin[0usize] - (*bs).enemyorigin[0usize];
            dir[1usize] = entinfo.origin[1usize] - (*bs).enemyorigin[1usize];
            dir[2usize] = entinfo.origin[2usize] - (*bs).enemyorigin[2usize];
            if !(dist > 100i32 as libc::c_float
                && VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                    < (32i32 * 32i32) as libc::c_float)
            {
                if aim_skill as libc::c_double > 0.8f64
                    && (*bs).cur_ps.weaponstate == WEAPON_READY as libc::c_int
                {
                    let mut move_0: aas_clientmove_t = aas_clientmove_s {
                        endpos: [0.; 3],
                        endarea: 0,
                        velocity: [0.; 3],
                        trace: aas_trace_s {
                            startsolid: qfalse,
                            fraction: 0.,
                            endpos: [0.; 3],
                            ent: 0,
                            lastarea: 0,
                            area: 0,
                            planenum: 0,
                        },
                        presencetype: 0,
                        stopevent: 0,
                        endcontents: 0,
                        time: 0.,
                        frames: 0,
                    };
                    let mut origin: vec3_t = [0.; 3];
                    dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
                    dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
                    dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
                    dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
                    dir[0usize] = entinfo.origin[0usize] - entinfo.lastvisorigin[0usize];
                    dir[1usize] = entinfo.origin[1usize] - entinfo.lastvisorigin[1usize];
                    dir[2usize] = entinfo.origin[2usize] - entinfo.lastvisorigin[2usize];
                    dir[0usize] = dir[0usize] * (1i32 as libc::c_float / entinfo.update_time);
                    dir[1usize] = dir[1usize] * (1i32 as libc::c_float / entinfo.update_time);
                    dir[2usize] = dir[2usize] * (1i32 as libc::c_float / entinfo.update_time);
                    origin[0usize] = entinfo.origin[0usize];
                    origin[1usize] = entinfo.origin[1usize];
                    origin[2usize] = entinfo.origin[2usize];
                    origin[2usize] += 1i32 as libc::c_float;
                    cmdmove[2usize] = 0i32 as vec_t;
                    cmdmove[1usize] = cmdmove[2usize];
                    cmdmove[0usize] = cmdmove[1usize];
                    trap_AAS_PredictClientMovement(
                        &mut move_0 as *mut aas_clientmove_t as *mut libc::c_void,
                        (*bs).enemy,
                        origin.as_mut_ptr(),
                        4i32,
                        qfalse as libc::c_int,
                        dir.as_mut_ptr(),
                        cmdmove.as_mut_ptr(),
                        0i32,
                        (dist * 10i32 as libc::c_float / wi.speed) as libc::c_int,
                        0.1f32,
                        0i32,
                        0i32,
                        qfalse as libc::c_int,
                    );
                    bestorigin[0usize] = move_0.endpos[0usize];
                    bestorigin[1usize] = move_0.endpos[1usize];
                    bestorigin[2usize] = move_0.endpos[2usize]
                } else if aim_skill as libc::c_double > 0.4f64 {
                    dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
                    dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
                    dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
                    dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
                    dir[0usize] = entinfo.origin[0usize] - entinfo.lastvisorigin[0usize];
                    dir[1usize] = entinfo.origin[1usize] - entinfo.lastvisorigin[1usize];
                    dir[2usize] = entinfo.origin[2usize] - entinfo.lastvisorigin[2usize];
                    dir[2usize] = 0i32 as vec_t;
                    speed = VectorNormalize(dir.as_mut_ptr()) / entinfo.update_time;
                    bestorigin[0usize] =
                        entinfo.origin[0usize] + dir[0usize] * (dist / wi.speed * speed);
                    bestorigin[1usize] =
                        entinfo.origin[1usize] + dir[1usize] * (dist / wi.speed * speed);
                    bestorigin[2usize] =
                        entinfo.origin[2usize] + dir[2usize] * (dist / wi.speed * speed)
                }
            }
        }
        if aim_skill as libc::c_double > 0.6f64 && 0 != wi.proj.damagetype & 2i32 {
            if entinfo.origin[2usize] < (*bs).origin[2usize] + 16i32 as libc::c_float {
                end[0usize] = entinfo.origin[0usize];
                end[1usize] = entinfo.origin[1usize];
                end[2usize] = entinfo.origin[2usize];
                end[2usize] -= 64i32 as libc::c_float;
                BotAI_Trace(
                    &mut trace,
                    entinfo.origin.as_mut_ptr(),
                    0 as *mut vec_t,
                    0 as *mut vec_t,
                    end.as_mut_ptr(),
                    entinfo.number,
                    1i32 | 0x2000000i32 | 0x4000000i32,
                );
                groundtarget[0usize] = bestorigin[0usize];
                groundtarget[1usize] = bestorigin[1usize];
                groundtarget[2usize] = bestorigin[2usize];
                if 0 != trace.startsolid as u64 {
                    groundtarget[2usize] = entinfo.origin[2usize] - 16i32 as libc::c_float
                } else {
                    groundtarget[2usize] = trace.endpos[2usize] - 8i32 as libc::c_float
                }
                BotAI_Trace(
                    &mut trace,
                    start.as_mut_ptr(),
                    0 as *mut vec_t,
                    0 as *mut vec_t,
                    groundtarget.as_mut_ptr(),
                    (*bs).entitynum,
                    1i32 | 0x2000000i32 | 0x4000000i32,
                );
                if fabs((trace.endpos[2usize] - groundtarget[2usize]) as libc::c_double)
                    < 50i32 as libc::c_double
                {
                    dir[0usize] = trace.endpos[0usize] - groundtarget[0usize];
                    dir[1usize] = trace.endpos[1usize] - groundtarget[1usize];
                    dir[2usize] = trace.endpos[2usize] - groundtarget[2usize];
                    if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                        < (60i32 * 60i32) as libc::c_float
                    {
                        dir[0usize] = trace.endpos[0usize] - start[0usize];
                        dir[1usize] = trace.endpos[1usize] - start[1usize];
                        dir[2usize] = trace.endpos[2usize] - start[2usize];
                        if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                            > (100i32 * 100i32) as libc::c_float
                        {
                            trace.endpos[2usize] += 1i32 as libc::c_float;
                            BotAI_Trace(
                                &mut trace,
                                trace.endpos.as_mut_ptr(),
                                0 as *mut vec_t,
                                0 as *mut vec_t,
                                entinfo.origin.as_mut_ptr(),
                                entinfo.number,
                                1i32 | 0x2000000i32 | 0x4000000i32,
                            );
                            if trace.fraction >= 1i32 as libc::c_float {
                                bestorigin[0usize] = groundtarget[0usize];
                                bestorigin[1usize] = groundtarget[1usize];
                                bestorigin[2usize] = groundtarget[2usize]
                            }
                        }
                    }
                }
            }
        }
        bestorigin[0usize] = (bestorigin[0usize] as libc::c_double
            + 20i32 as libc::c_double
                * (2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64))
                * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
            as vec_t;
        bestorigin[1usize] = (bestorigin[1usize] as libc::c_double
            + 20i32 as libc::c_double
                * (2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64))
                * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
            as vec_t;
        bestorigin[2usize] = (bestorigin[2usize] as libc::c_double
            + 10i32 as libc::c_double
                * (2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64))
                * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
            as vec_t
    } else {
        bestorigin[0usize] = (*bs).lastenemyorigin[0usize];
        bestorigin[1usize] = (*bs).lastenemyorigin[1usize];
        bestorigin[2usize] = (*bs).lastenemyorigin[2usize];
        bestorigin[2usize] += 8i32 as libc::c_float;
        if aim_skill as libc::c_double > 0.5f64 {
            if wi.number == WP_BFG as libc::c_int
                || wi.number == WP_ROCKET_LAUNCHER as libc::c_int
                || wi.number == WP_GRENADE_LAUNCHER as libc::c_int
            {
                goal.entitynum = (*bs).client;
                goal.areanum = (*bs).areanum;
                goal.origin[0usize] = (*bs).eye[0usize];
                goal.origin[1usize] = (*bs).eye[1usize];
                goal.origin[2usize] = (*bs).eye[2usize];
                goal.mins[0usize] = -8i32 as vec_t;
                goal.mins[1usize] = -8i32 as vec_t;
                goal.mins[2usize] = -8i32 as vec_t;
                goal.maxs[0usize] = 8i32 as vec_t;
                goal.maxs[1usize] = 8i32 as vec_t;
                goal.maxs[2usize] = 8i32 as vec_t;
                if 0 != trap_BotPredictVisiblePosition(
                    (*bs).lastenemyorigin.as_mut_ptr(),
                    (*bs).lastenemyareanum,
                    &mut goal as *mut bot_goal_t as *mut libc::c_void,
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
                    target.as_mut_ptr(),
                ) {
                    dir[0usize] = target[0usize] - (*bs).eye[0usize];
                    dir[1usize] = target[1usize] - (*bs).eye[1usize];
                    dir[2usize] = target[2usize] - (*bs).eye[2usize];
                    if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                        > (80i32 * 80i32) as libc::c_float
                    {
                        bestorigin[0usize] = target[0usize];
                        bestorigin[1usize] = target[1usize];
                        bestorigin[2usize] = target[2usize];
                        bestorigin[2usize] -= 20i32 as libc::c_float
                    }
                }
                aim_accuracy = 1i32 as libc::c_float
            }
        }
    }
    if 0 != enemyvisible {
        BotAI_Trace(
            &mut trace,
            (*bs).eye.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
            bestorigin.as_mut_ptr(),
            (*bs).entitynum,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        (*bs).aimtarget[0usize] = trace.endpos[0usize];
        (*bs).aimtarget[1usize] = trace.endpos[1usize];
        (*bs).aimtarget[2usize] = trace.endpos[2usize]
    } else {
        (*bs).aimtarget[0usize] = bestorigin[0usize];
        (*bs).aimtarget[1usize] = bestorigin[1usize];
        (*bs).aimtarget[2usize] = bestorigin[2usize]
    }
    dir[0usize] = bestorigin[0usize] - (*bs).eye[0usize];
    dir[1usize] = bestorigin[1usize] - (*bs).eye[1usize];
    dir[2usize] = bestorigin[2usize] - (*bs).eye[2usize];
    if wi.number == WP_MACHINEGUN as libc::c_int
        || wi.number == WP_SHOTGUN as libc::c_int
        || wi.number == WP_LIGHTNING as libc::c_int
        || wi.number == WP_RAILGUN as libc::c_int
    {
        dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
        if dist > 150i32 as libc::c_float {
            dist = 150i32 as libc::c_float
        }
        f = (0.6f64 + (dist / 150i32 as libc::c_float) as libc::c_double * 0.4f64) as libc::c_float;
        aim_accuracy *= f
    }
    if (aim_accuracy as libc::c_double) < 0.8f64 {
        VectorNormalize(dir.as_mut_ptr());
        i = 0i32;
        while i < 3i32 {
            dir[i as usize] = (dir[i as usize] as libc::c_double
                + 0.3f64
                    * (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64))
                    * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
                as vec_t;
            i += 1
        }
    }
    vectoangles(
        dir.as_mut_ptr() as *const vec_t,
        (*bs).ideal_viewangles.as_mut_ptr(),
    );
    (*bs).ideal_viewangles[0usize] = ((*bs).ideal_viewangles[0usize] as libc::c_double
        + (6i32 as libc::c_float * wi.vspread) as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))
            * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
        as vec_t;
    (*bs).ideal_viewangles[0usize] = AngleMod((*bs).ideal_viewangles[0usize]);
    (*bs).ideal_viewangles[1usize] = ((*bs).ideal_viewangles[1usize] as libc::c_double
        + (6i32 as libc::c_float * wi.hspread) as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))
            * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
        as vec_t;
    (*bs).ideal_viewangles[1usize] = AngleMod((*bs).ideal_viewangles[1usize]);
    if 0 != bot_challenge.integer {
        if aim_accuracy as libc::c_double > 0.9f64
            && (*bs).enemysight_time < floattime - 1i32 as libc::c_float
        {
            if (*bs).ideal_viewangles[0usize] > 180i32 as libc::c_float {
                (*bs).ideal_viewangles[0usize] -= 360i32 as libc::c_float
            }
            (*bs).viewangles[0usize] = (*bs).ideal_viewangles[0usize];
            (*bs).viewangles[1usize] = (*bs).ideal_viewangles[1usize];
            (*bs).viewangles[2usize] = (*bs).ideal_viewangles[2usize];
            trap_EA_View((*bs).client, (*bs).viewangles.as_mut_ptr());
        }
    };
}
//check if the bot should attack
#[no_mangle]
pub unsafe extern "C" fn BotCheckAttack(mut bs: *mut bot_state_t) {
    let mut points: libc::c_float = 0.;
    let mut reactiontime: libc::c_float = 0.;
    let mut fov: libc::c_float = 0.;
    let mut firethrottle: libc::c_float = 0.;
    let mut attackentity: libc::c_int = 0;
    let mut bsptrace: bsp_trace_t = bsp_trace_s {
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
    //float selfpreservation;
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut wi: weaponinfo_t = weaponinfo_s {
        valid: 0,
        number: 0,
        name: [0; 80],
        model: [0; 80],
        level: 0,
        weaponindex: 0,
        flags: 0,
        projectile: [0; 80],
        numprojectiles: 0,
        hspread: 0.,
        vspread: 0.,
        speed: 0.,
        acceleration: 0.,
        recoil: [0.; 3],
        offset: [0.; 3],
        angleoffset: [0.; 3],
        extrazvelocity: 0.,
        ammoamount: 0,
        ammoindex: 0,
        activate: 0.,
        reload: 0.,
        spinup: 0.,
        spindown: 0.,
        proj: projectileinfo_s {
            name: [0; 80],
            model: [0; 80],
            flags: 0,
            gravity: 0.,
            damage: 0,
            radius: 0.,
            visdamage: 0,
            damagetype: 0,
            healthinc: 0,
            push: 0.,
            detonation: 0.,
            bounce: 0.,
            bouncefric: 0.,
            bouncestop: 0.,
        },
    };
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
    let mut mins: vec3_t = [-8i32 as vec_t, -8i32 as vec_t, -8i32 as vec_t];
    let mut maxs: vec3_t = [8i32 as vec_t, 8i32 as vec_t, 8i32 as vec_t];
    attackentity = (*bs).enemy;
    BotEntityInfo(attackentity, &mut entinfo);
    attackentity >= 64i32;
    reactiontime = trap_Characteristic_BFloat(
        (*bs).character,
        6i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if (*bs).enemysight_time > floattime - reactiontime {
        return;
    }
    if (*bs).teleport_time > floattime - reactiontime {
        return;
    }
    if (*bs).weaponchange_time as libc::c_double > floattime as libc::c_double - 0.1f64 {
        return;
    }
    if (*bs).firethrottlewait_time > floattime {
        return;
    }
    firethrottle = trap_Characteristic_BFloat(
        (*bs).character,
        47i32,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
    );
    if (*bs).firethrottleshoot_time < floattime {
        if (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float > firethrottle {
            (*bs).firethrottlewait_time = floattime + firethrottle;
            (*bs).firethrottleshoot_time = 0i32 as libc::c_float
        } else {
            (*bs).firethrottleshoot_time = floattime + 1i32 as libc::c_float - firethrottle;
            (*bs).firethrottlewait_time = 0i32 as libc::c_float
        }
    }
    dir[0usize] = (*bs).aimtarget[0usize] - (*bs).eye[0usize];
    dir[1usize] = (*bs).aimtarget[1usize] - (*bs).eye[1usize];
    dir[2usize] = (*bs).aimtarget[2usize] - (*bs).eye[2usize];
    if (*bs).weaponnum == WP_GAUNTLET as libc::c_int {
        if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t) > (60i32 * 60i32) as libc::c_float
        {
            return;
        }
    }
    if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t) < (100i32 * 100i32) as libc::c_float {
        fov = 120i32 as libc::c_float
    } else {
        fov = 50i32 as libc::c_float
    }
    vectoangles(dir.as_mut_ptr() as *const vec_t, angles.as_mut_ptr());
    if 0 == InFieldOfVision((*bs).viewangles.as_mut_ptr(), fov, angles.as_mut_ptr()) as u64 {
        return;
    }
    BotAI_Trace(
        &mut bsptrace,
        (*bs).eye.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
        (*bs).aimtarget.as_mut_ptr(),
        (*bs).client,
        1i32 | 0x10000i32,
    );
    if bsptrace.fraction < 1i32 as libc::c_float && bsptrace.ent != attackentity {
        return;
    }
    trap_BotGetWeaponInfo(
        (*bs).ws,
        (*bs).weaponnum,
        &mut wi as *mut weaponinfo_t as *mut libc::c_void,
    );
    start[0usize] = (*bs).origin[0usize];
    start[1usize] = (*bs).origin[1usize];
    start[2usize] = (*bs).origin[2usize];
    start[2usize] += (*bs).cur_ps.viewheight as libc::c_float;
    AngleVectors(
        (*bs).viewangles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    start[0usize] += forward[0usize] * wi.offset[0usize] + right[0usize] * wi.offset[1usize];
    start[1usize] += forward[1usize] * wi.offset[0usize] + right[1usize] * wi.offset[1usize];
    start[2usize] +=
        forward[2usize] * wi.offset[0usize] + right[2usize] * wi.offset[1usize] + wi.offset[2usize];
    end[0usize] = start[0usize] + forward[0usize] * 1000i32 as libc::c_float;
    end[1usize] = start[1usize] + forward[1usize] * 1000i32 as libc::c_float;
    end[2usize] = start[2usize] + forward[2usize] * 1000i32 as libc::c_float;
    start[0usize] = start[0usize] + forward[0usize] * -12i32 as libc::c_float;
    start[1usize] = start[1usize] + forward[1usize] * -12i32 as libc::c_float;
    start[2usize] = start[2usize] + forward[2usize] * -12i32 as libc::c_float;
    BotAI_Trace(
        &mut trace,
        start.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*bs).entitynum,
        1i32 | 0x2000000i32 | 0x4000000i32,
    );
    if trace.ent >= 0i32 && trace.ent < 64i32 {
        if trace.ent != attackentity {
            if 0 != BotSameTeam(bs, trace.ent) {
                return;
            }
        }
    }
    if trace.ent != attackentity || attackentity >= 64i32 {
        if 0 != wi.proj.damagetype & 2i32 {
            if (trace.fraction * 1000i32 as libc::c_float) < wi.proj.radius {
                points = ((wi.proj.damage as libc::c_double
                    - 0.5f64 * trace.fraction as libc::c_double * 1000i32 as libc::c_double)
                    * 0.5f64) as libc::c_float;
                if points > 0i32 as libc::c_float {
                    return;
                }
            }
        }
    }
    if 0 != wi.flags & 1i32 {
        if 0 != (*bs).flags & 2i32 {
            trap_EA_Attack((*bs).client);
        }
    } else {
        trap_EA_Attack((*bs).client);
    }
    (*bs).flags ^= 2i32;
}
//AI when the bot is blocked
#[no_mangle]
pub unsafe extern "C" fn BotAIBlocked(
    mut bs: *mut bot_state_t,
    mut moveresult: *mut bot_moveresult_t,
    mut activate: libc::c_int,
) {
    let mut movetype: libc::c_int = 0;
    let mut bspent: libc::c_int = 0;
    let mut hordir: vec3_t = [0.; 3];
    let mut sideward: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    //vec3_t start, end, mins, maxs;
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
    let mut activategoal: bot_activategoal_t = bot_activategoal_s {
        inuse: 0,
        goal: bot_goal_s {
            origin: [0.; 3],
            areanum: 0,
            mins: [0.; 3],
            maxs: [0.; 3],
            entitynum: 0,
            number: 0,
            flags: 0,
            iteminfo: 0,
        },
        time: 0.,
        start_time: 0.,
        justused_time: 0.,
        shoot: 0,
        weapon: 0,
        target: [0.; 3],
        origin: [0.; 3],
        areas: [0; 32],
        numareas: 0,
        areasdisabled: 0,
        next: 0 as *mut bot_activategoal_s,
    };
    if 0 == (*moveresult).blocked {
        (*bs).notblocked_time = floattime;
        return;
    }
    if (*moveresult).type_0 == 8i32 {
        BotRandomMove(bs, moveresult);
        return;
    }
    BotEntityInfo((*moveresult).blockentity, &mut entinfo);
    if 0 != activate && entinfo.modelindex > 0i32 && entinfo.modelindex <= max_bspmodelindex {
        bspent = BotGetActivateGoal(bs, entinfo.number, &mut activategoal);
        if 0 != bspent {
            if !(*bs).activatestack.is_null() && 0 == (*(*bs).activatestack).inuse {
                (*bs).activatestack = 0 as *mut bot_activategoal_t
            }
            if 0 == BotIsGoingToActivateEntity(bs, activategoal.goal.entitynum) {
                BotGoForActivateGoal(bs, &mut activategoal);
            }
            if 0 == (*moveresult).flags & 32i32 && 0 != trap_AAS_AreaReachability((*bs).areanum) {
                return;
            }
        } else {
            BotEnableActivateGoalAreas(&mut activategoal, qtrue as libc::c_int);
        }
    }
    hordir[0usize] = (*moveresult).movedir[0usize];
    hordir[1usize] = (*moveresult).movedir[1usize];
    hordir[2usize] = 0i32 as vec_t;
    if (VectorNormalize(hordir.as_mut_ptr()) as libc::c_double) < 0.1f64 {
        angles[0usize] = 0i32 as vec_t;
        angles[1usize] = 360i32 as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
        angles[2usize] = 0i32 as vec_t;
        AngleVectors(
            angles.as_mut_ptr() as *const vec_t,
            hordir.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
    }
    movetype = 1i32;
    CrossProduct(
        hordir.as_mut_ptr() as *const vec_t,
        up.as_mut_ptr() as *const vec_t,
        sideward.as_mut_ptr(),
    );
    if 0 != (*bs).flags & 16i32 {
        sideward[0usize] = -sideward[0usize];
        sideward[1usize] = -sideward[1usize];
        sideward[2usize] = -sideward[2usize]
    }
    if movetype != 2i32
        || 0 == trap_BotMoveInDirection(
            (*bs).ms,
            hordir.as_mut_ptr(),
            400i32 as libc::c_float,
            movetype,
        )
    {
        if 0 == trap_BotMoveInDirection(
            (*bs).ms,
            sideward.as_mut_ptr(),
            400i32 as libc::c_float,
            movetype,
        ) {
            (*bs).flags ^= 16i32;
            sideward[0usize] = sideward[0usize] + hordir[0usize] * -1i32 as libc::c_float;
            sideward[1usize] = sideward[1usize] + hordir[1usize] * -1i32 as libc::c_float;
            sideward[2usize] = sideward[2usize] + hordir[2usize] * -1i32 as libc::c_float;
            trap_BotMoveInDirection(
                (*bs).ms,
                sideward.as_mut_ptr(),
                400i32 as libc::c_float,
                movetype,
            );
        }
    }
    if ((*bs).notblocked_time as libc::c_double) < floattime as libc::c_double - 0.4f64 {
        if (*bs).ainode == Some(AINode_Seek_NBG) {
            (*bs).nbg_time = 0i32 as libc::c_float
        } else if (*bs).ainode == Some(AINode_Seek_LTG) {
            (*bs).ltg_time = 0i32 as libc::c_float
        }
    };
}
//enable or disable the areas the blocking entity is in
#[no_mangle]
pub unsafe extern "C" fn BotEnableActivateGoalAreas(
    mut activategoal: *mut bot_activategoal_t,
    mut enable: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if (*activategoal).areasdisabled == (0 == enable) as libc::c_int {
        return;
    }
    i = 0i32;
    while i < (*activategoal).numareas {
        trap_AAS_EnableRoutingArea((*activategoal).areas[i as usize], enable);
        i += 1
    }
    (*activategoal).areasdisabled = (0 == enable) as libc::c_int;
}
/*
==================
BotGoForActivateGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGoForActivateGoal(
    mut bs: *mut bot_state_t,
    mut activategoal: *mut bot_activategoal_t,
) -> libc::c_int {
    let mut activateinfo: aas_entityinfo_t = aas_entityinfo_s {
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
    (*activategoal).inuse = qtrue as libc::c_int;
    if 0. == (*activategoal).time {
        (*activategoal).time = floattime + 10i32 as libc::c_float
    }
    (*activategoal).start_time = floattime;
    BotEntityInfo((*activategoal).goal.entitynum, &mut activateinfo);
    (*activategoal).origin[0usize] = activateinfo.origin[0usize];
    (*activategoal).origin[1usize] = activateinfo.origin[1usize];
    (*activategoal).origin[2usize] = activateinfo.origin[2usize];
    if 0 != BotPushOntoActivateGoalStack(bs, activategoal) {
        AIEnter_Seek_ActivateEntity(
            bs,
            b"BotGoForActivateGoal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qtrue as libc::c_int;
    } else {
        BotEnableActivateGoalAreas(activategoal, qtrue as libc::c_int);
        return qfalse as libc::c_int;
    };
}
/*
==================
BotPushOntoActivateGoalStack
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotPushOntoActivateGoalStack(
    mut bs: *mut bot_state_t,
    mut activategoal: *mut bot_activategoal_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut besttime: libc::c_float = 0.;
    best = -1i32;
    besttime = floattime + 9999i32 as libc::c_float;
    i = 0i32;
    while i < 8i32 {
        if 0 == (*bs).activategoalheap[i as usize].inuse {
            if (*bs).activategoalheap[i as usize].justused_time < besttime {
                besttime = (*bs).activategoalheap[i as usize].justused_time;
                best = i
            }
        }
        i += 1
    }
    if best != -1i32 {
        memcpy(
            &mut (*bs).activategoalheap[best as usize] as *mut bot_activategoal_t
                as *mut libc::c_void,
            activategoal as *const libc::c_void,
            ::std::mem::size_of::<bot_activategoal_t>() as libc::c_ulong,
        );
        (*bs).activategoalheap[best as usize].inuse = qtrue as libc::c_int;
        (*bs).activategoalheap[best as usize].next = (*bs).activatestack;
        (*bs).activatestack = &mut (*bs).activategoalheap[best as usize] as *mut bot_activategoal_t;
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
}
/*
==================
BotIsGoingToActivateEntity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotIsGoingToActivateEntity(
    mut bs: *mut bot_state_t,
    mut entitynum: libc::c_int,
) -> libc::c_int {
    let mut a: *mut bot_activategoal_t = 0 as *mut bot_activategoal_t;
    let mut i: libc::c_int = 0;
    a = (*bs).activatestack;
    while !a.is_null() {
        if !((*a).time < floattime) {
            if (*a).goal.entitynum == entitynum {
                return qtrue as libc::c_int;
            }
        }
        a = (*a).next
    }
    i = 0i32;
    while i < 8i32 {
        if !(0 != (*bs).activategoalheap[i as usize].inuse) {
            if (*bs).activategoalheap[i as usize].goal.entitynum == entitynum {
                if (*bs).activategoalheap[i as usize].justused_time
                    > floattime - 2i32 as libc::c_float
                {
                    return qtrue as libc::c_int;
                }
            }
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
/*
==================
BotGetActivateGoal

  returns the number of the bsp entity to activate
  goal->entitynum will be set to the game entity to activate
==================
*/
//#define OBSTACLEDEBUG
#[no_mangle]
pub unsafe extern "C" fn BotGetActivateGoal(
    mut bs: *mut bot_state_t,
    mut entitynum: libc::c_int,
    mut activategoal: *mut bot_activategoal_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ent: libc::c_int = 0;
    let mut cur_entities: [libc::c_int; 10] = [0; 10];
    let mut spawnflags: libc::c_int = 0;
    let mut modelindex: libc::c_int = 0;
    let mut areas: [libc::c_int; 64] = [0; 64];
    let mut numareas: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut model: [libc::c_char; 1024] = [0; 1024];
    let mut tmpmodel: [libc::c_char; 128] = [0; 128];
    let mut target: [libc::c_char; 128] = [0; 128];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut health: libc::c_float = 0.;
    let mut targetname: [[libc::c_char; 128]; 10] = [[0; 128]; 10];
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
    let mut areainfo: aas_areainfo_t = aas_areainfo_s {
        contents: 0,
        flags: 0,
        presencetype: 0,
        cluster: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        center: [0.; 3],
    };
    let mut origin: vec3_t = [0.; 3];
    let mut absmins: vec3_t = [0.; 3];
    let mut absmaxs: vec3_t = [0.; 3];
    memset(
        activategoal as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<bot_activategoal_t>() as libc::c_ulong,
    );
    BotEntityInfo(entitynum, &mut entinfo);
    Com_sprintf(
        model.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"*%d\x00" as *const u8 as *const libc::c_char,
        entinfo.modelindex,
    );
    ent = trap_AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0
            == trap_AAS_ValueForBSPEpairKey(
                ent,
                b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                tmpmodel.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            ))
        {
            if 0 == strcmp(model.as_mut_ptr(), tmpmodel.as_mut_ptr()) {
                break;
            }
        }
        ent = trap_AAS_NextBSPEntity(ent)
    }
    if 0 == ent {
        BotAI_Print(
            3i32,
            b"BotGetActivateGoal: no entity found with model %s\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            model.as_mut_ptr(),
        );
        return 0i32;
    }
    trap_AAS_ValueForBSPEpairKey(
        ent,
        b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        classname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == *classname.as_mut_ptr() {
        BotAI_Print(
            3i32,
            b"BotGetActivateGoal: entity with model %s has no classname\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            model.as_mut_ptr(),
        );
        return 0i32;
    }
    if 0 == strcmp(
        classname.as_mut_ptr(),
        b"func_door\x00" as *const u8 as *const libc::c_char,
    ) {
        if 0 != trap_AAS_FloatForBSPEpairKey(
            ent,
            b"health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut health,
        ) {
            if 0. != health {
                BotFuncDoorActivateGoal(bs, ent, activategoal);
                return ent;
            }
        }
        trap_AAS_IntForBSPEpairKey(
            ent,
            b"spawnflags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut spawnflags,
        );
        if 0 != spawnflags & 1i32 {
            return 0i32;
        }
        if 0 == trap_AAS_VectorForBSPEpairKey(
            ent,
            b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            origin.as_mut_ptr(),
        ) {
            origin[2usize] = 0i32 as vec_t;
            origin[1usize] = origin[2usize];
            origin[0usize] = origin[1usize]
        }
        if 0 == VectorCompare(
            origin.as_mut_ptr() as *const vec_t,
            entinfo.origin.as_mut_ptr() as *const vec_t,
        ) {
            return 0i32;
        }
        trap_AAS_ValueForBSPEpairKey(
            ent,
            b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            model.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 != *model.as_mut_ptr() {
            modelindex = atoi(model.as_mut_ptr().offset(1isize));
            if 0 != modelindex {
                BotModelMinsMaxs(
                    modelindex,
                    ET_MOVER as libc::c_int,
                    0i32,
                    absmins.as_mut_ptr(),
                    absmaxs.as_mut_ptr(),
                );
                numareas = trap_AAS_BBoxAreas(
                    absmins.as_mut_ptr(),
                    absmaxs.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    32i32 * 2i32,
                );
                i = 0i32;
                while i < numareas {
                    if (*activategoal).numareas >= 32i32 {
                        break;
                    }
                    if !(0 == trap_AAS_AreaReachability(areas[i as usize])) {
                        trap_AAS_AreaInfo(
                            areas[i as usize],
                            &mut areainfo as *mut aas_areainfo_t as *mut libc::c_void,
                        );
                        if 0 != areainfo.contents & 1024i32 {
                            let fresh0 = (*activategoal).numareas;
                            (*activategoal).numareas = (*activategoal).numareas + 1;
                            (*activategoal).areas[fresh0 as usize] = areas[i as usize]
                        }
                    }
                    i += 1
                }
                i = 0i32;
                while i < numareas {
                    if (*activategoal).numareas >= 32i32 {
                        break;
                    }
                    if !(0 != trap_AAS_AreaReachability(areas[i as usize])) {
                        trap_AAS_AreaInfo(
                            areas[i as usize],
                            &mut areainfo as *mut aas_areainfo_t as *mut libc::c_void,
                        );
                        if 0 != areainfo.contents & 1024i32 {
                            let fresh1 = (*activategoal).numareas;
                            (*activategoal).numareas = (*activategoal).numareas + 1;
                            (*activategoal).areas[fresh1 as usize] = areas[i as usize]
                        }
                    }
                    i += 1
                }
            }
        }
    }
    if 0 == strcmp(
        classname.as_mut_ptr(),
        b"func_button\x00" as *const u8 as *const libc::c_char,
    ) {
        return 0i32;
    }
    if 0 == trap_AAS_ValueForBSPEpairKey(
        ent,
        b"targetname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        targetname[0usize].as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    ) {
        if 0 != bot_developer.integer {
            BotAI_Print(
                3i32,
                b"BotGetActivateGoal: entity with model \"%s\" has no targetname\n\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                model.as_mut_ptr(),
            );
        }
        return 0i32;
    }
    cur_entities[0usize] = trap_AAS_NextBSPEntity(0i32);
    i = 0i32;
    while i >= 0i32 && i < 10i32 {
        ent = cur_entities[i as usize];
        while 0 != ent {
            if !(0
                == trap_AAS_ValueForBSPEpairKey(
                    ent,
                    b"target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    target.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
                ))
            {
                if 0 == strcmp(targetname[i as usize].as_mut_ptr(), target.as_mut_ptr()) {
                    cur_entities[i as usize] = trap_AAS_NextBSPEntity(ent);
                    break;
                }
            }
            ent = trap_AAS_NextBSPEntity(ent)
        }
        if 0 == ent {
            if 0 != bot_developer.integer {
                BotAI_Print(
                    3i32,
                    b"BotGetActivateGoal: no entity with target \"%s\"\n\x00" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    targetname[i as usize].as_mut_ptr(),
                );
            }
            i -= 1
        } else if 0
            == trap_AAS_ValueForBSPEpairKey(
                ent,
                b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                classname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            )
        {
            if 0 != bot_developer.integer {
                BotAI_Print(
                    3i32,
                    b"BotGetActivateGoal: entity with target \"%s\" has no classname\n\x00"
                        as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    targetname[i as usize].as_mut_ptr(),
                );
            }
        } else if 0
            == strcmp(
                classname.as_mut_ptr(),
                b"func_button\x00" as *const u8 as *const libc::c_char,
            )
        {
            //
            if 0 == BotFuncButtonActivateGoal(bs, ent, activategoal) {
                continue;
            }
            // if the bot tries to activate this button already
            if !(*bs).activatestack.is_null()
                && 0 != (*(*bs).activatestack).inuse
                && (*(*bs).activatestack).goal.entitynum == (*activategoal).goal.entitynum
                && (*(*bs).activatestack).time > floattime
                && (*(*bs).activatestack).start_time < floattime - 2i32 as libc::c_float
            {
                continue;
            }
            // if the bot is in a reachability area
            if 0 != trap_AAS_AreaReachability((*bs).areanum) {
                BotEnableActivateGoalAreas(activategoal, qfalse as libc::c_int);
                t = trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*activategoal).goal.areanum,
                    (*bs).tfl,
                );
                // if the button is not reachable
                if 0 == t {
                    continue;
                }
                (*activategoal).time = (floattime as libc::c_double
                    + t as libc::c_double * 0.01f64
                    + 5i32 as libc::c_double)
                    as libc::c_float
            }
            return ent;
        } else if 0
            == strcmp(
                classname.as_mut_ptr(),
                b"trigger_multiple\x00" as *const u8 as *const libc::c_char,
            )
        {
            //
            if 0 == BotTriggerMultipleActivateGoal(bs, ent, activategoal) {
                continue;
            }
            // if the bot tries to activate this trigger already
            if !(*bs).activatestack.is_null()
                && 0 != (*(*bs).activatestack).inuse
                && (*(*bs).activatestack).goal.entitynum == (*activategoal).goal.entitynum
                && (*(*bs).activatestack).time > floattime
                && (*(*bs).activatestack).start_time < floattime - 2i32 as libc::c_float
            {
                continue;
            }
            // if the bot is in a reachability area
            if 0 != trap_AAS_AreaReachability((*bs).areanum) {
                BotEnableActivateGoalAreas(activategoal, qfalse as libc::c_int);
                t = trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*activategoal).goal.areanum,
                    (*bs).tfl,
                );
                // if the trigger is not reachable
                if 0 == t {
                    continue;
                }
                (*activategoal).time = (floattime as libc::c_double
                    + t as libc::c_double * 0.01f64
                    + 5i32 as libc::c_double)
                    as libc::c_float
            }
            return ent;
        } else if !(0
            == strcmp(
                classname.as_mut_ptr(),
                b"func_timer\x00" as *const u8 as *const libc::c_char,
            ))
        {
            // just skip the func_timer
            if 0 == strcmp(
                classname.as_mut_ptr(),
                b"target_relay\x00" as *const u8 as *const libc::c_char,
            ) || 0
                == strcmp(
                    classname.as_mut_ptr(),
                    b"target_delay\x00" as *const u8 as *const libc::c_char,
                )
            {
                if 0 != trap_AAS_ValueForBSPEpairKey(
                    ent,
                    b"targetname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    targetname[(i + 1i32) as usize].as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
                ) {
                    i += 1;
                    cur_entities[i as usize] = trap_AAS_NextBSPEntity(0i32)
                }
            }
        }
    }
    return 0i32;
}
/*
==================
BotTriggerMultipleGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotTriggerMultipleActivateGoal(
    mut bs: *mut bot_state_t,
    mut bspent: libc::c_int,
    mut activategoal: *mut bot_activategoal_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut modelindex: libc::c_int = 0;
    let mut entitynum: libc::c_int = 0;
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut goalorigin: vec3_t = [0.; 3];
    (*activategoal).shoot = qfalse as libc::c_int;
    (*activategoal).target[2usize] = 0i32 as vec_t;
    (*activategoal).target[1usize] = (*activategoal).target[2usize];
    (*activategoal).target[0usize] = (*activategoal).target[1usize];
    trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        model.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == *model.as_mut_ptr() {
        return qfalse as libc::c_int;
    }
    modelindex = atoi(model.as_mut_ptr().offset(1isize));
    if 0 == modelindex {
        return qfalse as libc::c_int;
    }
    entitynum = BotModelMinsMaxs(
        modelindex,
        0i32,
        0x40000000i32,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    origin[0usize] = mins[0usize] + maxs[0usize];
    origin[1usize] = mins[1usize] + maxs[1usize];
    origin[2usize] = mins[2usize] + maxs[2usize];
    origin[0usize] = (origin[0usize] as libc::c_double * 0.5f64) as vec_t;
    origin[1usize] = (origin[1usize] as libc::c_double * 0.5f64) as vec_t;
    origin[2usize] = (origin[2usize] as libc::c_double * 0.5f64) as vec_t;
    goalorigin[0usize] = origin[0usize];
    goalorigin[1usize] = origin[1usize];
    goalorigin[2usize] = origin[2usize];
    start[0usize] = goalorigin[0usize];
    start[1usize] = goalorigin[1usize];
    start[2usize] = goalorigin[2usize];
    start[2usize] += 24i32 as libc::c_float;
    end[0usize] = start[0usize];
    end[1usize] = start[1usize];
    end[2usize] = start[2usize];
    end[2usize] -= 100i32 as libc::c_float;
    numareas = trap_AAS_TraceAreas(
        start.as_mut_ptr(),
        end.as_mut_ptr(),
        areas.as_mut_ptr(),
        0 as *mut vec3_t,
        10i32,
    );
    i = 0i32;
    while i < numareas {
        if 0 != trap_AAS_AreaReachability(areas[i as usize]) {
            break;
        }
        i += 1
    }
    if i < numareas {
        (*activategoal).goal.origin[0usize] = origin[0usize];
        (*activategoal).goal.origin[1usize] = origin[1usize];
        (*activategoal).goal.origin[2usize] = origin[2usize];
        (*activategoal).goal.areanum = areas[i as usize];
        (*activategoal).goal.mins[0usize] = mins[0usize] - origin[0usize];
        (*activategoal).goal.mins[1usize] = mins[1usize] - origin[1usize];
        (*activategoal).goal.mins[2usize] = mins[2usize] - origin[2usize];
        (*activategoal).goal.maxs[0usize] = maxs[0usize] - origin[0usize];
        (*activategoal).goal.maxs[1usize] = maxs[1usize] - origin[1usize];
        (*activategoal).goal.maxs[2usize] = maxs[2usize] - origin[2usize];
        (*activategoal).goal.entitynum = entitynum;
        (*activategoal).goal.number = 0i32;
        (*activategoal).goal.flags = 0i32;
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
}
/*
==================
BotModelMinsMaxs

this is ugly
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotModelMinsMaxs(
    mut modelindex: libc::c_int,
    mut eType: libc::c_int,
    mut contents: libc::c_int,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) -> libc::c_int {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    ent = &mut g_entities[0usize] as *mut gentity_t;
    i = 0i32;
    while i < level.num_entities {
        if !(0 == (*ent).inuse as u64) {
            if !(0 != eType && (*ent).s.eType != eType) {
                if !(0 != contents && (*ent).r.contents != contents) {
                    if (*ent).s.modelindex == modelindex {
                        if !mins.is_null() {
                            *mins.offset(0isize) =
                                (*ent).r.currentOrigin[0usize] + (*ent).r.mins[0usize];
                            *mins.offset(1isize) =
                                (*ent).r.currentOrigin[1usize] + (*ent).r.mins[1usize];
                            *mins.offset(2isize) =
                                (*ent).r.currentOrigin[2usize] + (*ent).r.mins[2usize]
                        }
                        if !maxs.is_null() {
                            *maxs.offset(0isize) =
                                (*ent).r.currentOrigin[0usize] + (*ent).r.maxs[0usize];
                            *maxs.offset(1isize) =
                                (*ent).r.currentOrigin[1usize] + (*ent).r.maxs[1usize];
                            *maxs.offset(2isize) =
                                (*ent).r.currentOrigin[2usize] + (*ent).r.maxs[2usize]
                        }
                        return i;
                    }
                }
            }
        }
        i += 1;
        ent = ent.offset(1isize)
    }
    if !mins.is_null() {
        let ref mut fresh3 = *mins.offset(1isize);
        let ref mut fresh2 = *mins.offset(2isize);
        *fresh2 = 0i32 as vec_t;
        *fresh3 = *fresh2;
        *mins.offset(0isize) = *fresh3
    }
    if !maxs.is_null() {
        let ref mut fresh5 = *maxs.offset(1isize);
        let ref mut fresh4 = *maxs.offset(2isize);
        *fresh4 = 0i32 as vec_t;
        *fresh5 = *fresh4;
        *maxs.offset(0isize) = *fresh5
    }
    return 0i32;
}
/*
==================
BotFuncButtonGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotFuncButtonActivateGoal(
    mut bs: *mut bot_state_t,
    mut bspent: libc::c_int,
    mut activategoal: *mut bot_activategoal_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut numareas: libc::c_int = 0;
    let mut modelindex: libc::c_int = 0;
    let mut entitynum: libc::c_int = 0;
    let mut model: [libc::c_char; 128] = [0; 128];
    let mut lip: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut health: libc::c_float = 0.;
    let mut angle: libc::c_float = 0.;
    let mut size: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut points: [vec3_t; 10] = [[0.; 3]; 10];
    let mut movedir: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut goalorigin: vec3_t = [0.; 3];
    let mut bboxmins: vec3_t = [0.; 3];
    let mut bboxmaxs: vec3_t = [0.; 3];
    let mut extramins: vec3_t = [1i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t];
    let mut extramaxs: vec3_t = [-1i32 as vec_t, -1i32 as vec_t, -1i32 as vec_t];
    let mut bsptrace: bsp_trace_t = bsp_trace_s {
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
    (*activategoal).shoot = qfalse as libc::c_int;
    (*activategoal).target[2usize] = 0i32 as vec_t;
    (*activategoal).target[1usize] = (*activategoal).target[2usize];
    (*activategoal).target[0usize] = (*activategoal).target[1usize];
    trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        model.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == *model.as_mut_ptr() {
        return qfalse as libc::c_int;
    }
    modelindex = atoi(model.as_mut_ptr().offset(1isize));
    if 0 == modelindex {
        return qfalse as libc::c_int;
    }
    entitynum = BotModelMinsMaxs(
        modelindex,
        ET_MOVER as libc::c_int,
        0i32,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    trap_AAS_FloatForBSPEpairKey(
        bspent,
        b"lip\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut lip,
    );
    if 0. == lip {
        lip = 4i32 as libc::c_float
    }
    trap_AAS_FloatForBSPEpairKey(
        bspent,
        b"angle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut angle,
    );
    angles[0usize] = 0i32 as vec_t;
    angles[1usize] = angle;
    angles[2usize] = 0i32 as vec_t;
    BotSetMovedir(angles.as_mut_ptr(), movedir.as_mut_ptr());
    size[0usize] = maxs[0usize] - mins[0usize];
    size[1usize] = maxs[1usize] - mins[1usize];
    size[2usize] = maxs[2usize] - mins[2usize];
    origin[0usize] = mins[0usize] + maxs[0usize];
    origin[1usize] = mins[1usize] + maxs[1usize];
    origin[2usize] = mins[2usize] + maxs[2usize];
    origin[0usize] = (origin[0usize] as libc::c_double * 0.5f64) as vec_t;
    origin[1usize] = (origin[1usize] as libc::c_double * 0.5f64) as vec_t;
    origin[2usize] = (origin[2usize] as libc::c_double * 0.5f64) as vec_t;
    dist = (fabs(movedir[0usize] as libc::c_double) * size[0usize] as libc::c_double
        + fabs(movedir[1usize] as libc::c_double) * size[1usize] as libc::c_double
        + fabs(movedir[2usize] as libc::c_double) * size[2usize] as libc::c_double)
        as libc::c_float;
    dist = (dist as libc::c_double * 0.5f64) as libc::c_float;
    trap_AAS_FloatForBSPEpairKey(
        bspent,
        b"health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut health,
    );
    if 0. != health {
        goalorigin[0usize] = origin[0usize] + movedir[0usize] * -dist;
        goalorigin[1usize] = origin[1usize] + movedir[1usize] * -dist;
        goalorigin[2usize] = origin[2usize] + movedir[2usize] * -dist;
        (*activategoal).target[0usize] = goalorigin[0usize];
        (*activategoal).target[1usize] = goalorigin[1usize];
        (*activategoal).target[2usize] = goalorigin[2usize];
        (*activategoal).shoot = qtrue as libc::c_int;
        BotAI_Trace(
            &mut bsptrace,
            (*bs).eye.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
            goalorigin.as_mut_ptr(),
            (*bs).entitynum,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        if bsptrace.fraction as libc::c_double >= 1.0f64 || bsptrace.ent == entitynum {
            (*activategoal).goal.entitynum = entitynum;
            (*activategoal).goal.number = 0i32;
            (*activategoal).goal.flags = 0i32;
            (*activategoal).goal.origin[0usize] = (*bs).origin[0usize];
            (*activategoal).goal.origin[1usize] = (*bs).origin[1usize];
            (*activategoal).goal.origin[2usize] = (*bs).origin[2usize];
            (*activategoal).goal.areanum = (*bs).areanum;
            (*activategoal).goal.mins[0usize] = -8i32 as vec_t;
            (*activategoal).goal.mins[1usize] = -8i32 as vec_t;
            (*activategoal).goal.mins[2usize] = -8i32 as vec_t;
            (*activategoal).goal.maxs[0usize] = 8i32 as vec_t;
            (*activategoal).goal.maxs[1usize] = 8i32 as vec_t;
            (*activategoal).goal.maxs[2usize] = 8i32 as vec_t;
            return qtrue as libc::c_int;
        } else {
            trap_AAS_PresenceTypeBoundingBox(4i32, bboxmins.as_mut_ptr(), bboxmaxs.as_mut_ptr());
            i = 0i32;
            while i < 3i32 {
                if movedir[i as usize] < 0i32 as libc::c_float {
                    dist = (dist as libc::c_double
                        + fabs(movedir[i as usize] as libc::c_double)
                            * fabs(bboxmaxs[i as usize] as libc::c_double))
                        as libc::c_float
                } else {
                    dist = (dist as libc::c_double
                        + fabs(movedir[i as usize] as libc::c_double)
                            * fabs(bboxmins[i as usize] as libc::c_double))
                        as libc::c_float
                }
                i += 1
            }
            goalorigin[0usize] = origin[0usize] + movedir[0usize] * -dist;
            goalorigin[1usize] = origin[1usize] + movedir[1usize] * -dist;
            goalorigin[2usize] = origin[2usize] + movedir[2usize] * -dist;
            start[0usize] = goalorigin[0usize];
            start[1usize] = goalorigin[1usize];
            start[2usize] = goalorigin[2usize];
            start[2usize] += 24i32 as libc::c_float;
            end[0usize] = start[0usize];
            end[1usize] = start[1usize];
            end[2usize] = start[2usize];
            end[2usize] -= 512i32 as libc::c_float;
            numareas = trap_AAS_TraceAreas(
                start.as_mut_ptr(),
                end.as_mut_ptr(),
                areas.as_mut_ptr(),
                points.as_mut_ptr(),
                10i32,
            );
            i = numareas - 1i32;
            while i >= 0i32 {
                if 0 != trap_AAS_AreaReachability(areas[i as usize]) {
                    break;
                }
                i -= 1
            }
            i < 0i32;
            if i >= 0i32 {
                (*activategoal).goal.origin[0usize] = points[i as usize][0usize];
                (*activategoal).goal.origin[1usize] = points[i as usize][1usize];
                (*activategoal).goal.origin[2usize] = points[i as usize][2usize];
                (*activategoal).goal.areanum = areas[i as usize];
                (*activategoal).goal.mins[0usize] = 8i32 as vec_t;
                (*activategoal).goal.mins[1usize] = 8i32 as vec_t;
                (*activategoal).goal.mins[2usize] = 8i32 as vec_t;
                (*activategoal).goal.maxs[0usize] = -8i32 as vec_t;
                (*activategoal).goal.maxs[1usize] = -8i32 as vec_t;
                (*activategoal).goal.maxs[2usize] = -8i32 as vec_t;
                i = 0i32;
                while i < 3i32 {
                    if movedir[i as usize] < 0i32 as libc::c_float {
                        (*activategoal).goal.maxs[i as usize] =
                            ((*activategoal).goal.maxs[i as usize] as libc::c_double
                                + fabs(movedir[i as usize] as libc::c_double)
                                    * fabs(extramaxs[i as usize] as libc::c_double))
                                as vec_t
                    } else {
                        (*activategoal).goal.mins[i as usize] =
                            ((*activategoal).goal.mins[i as usize] as libc::c_double
                                + fabs(movedir[i as usize] as libc::c_double)
                                    * fabs(extramins[i as usize] as libc::c_double))
                                as vec_t
                    }
                    i += 1
                }
                (*activategoal).goal.entitynum = entitynum;
                (*activategoal).goal.number = 0i32;
                (*activategoal).goal.flags = 0i32;
                return qtrue as libc::c_int;
            }
        }
        return qfalse as libc::c_int;
    } else {
        trap_AAS_PresenceTypeBoundingBox(4i32, bboxmins.as_mut_ptr(), bboxmaxs.as_mut_ptr());
        i = 0i32;
        while i < 3i32 {
            if movedir[i as usize] < 0i32 as libc::c_float {
                dist = (dist as libc::c_double
                    + fabs(movedir[i as usize] as libc::c_double)
                        * fabs(bboxmaxs[i as usize] as libc::c_double))
                    as libc::c_float
            } else {
                dist = (dist as libc::c_double
                    + fabs(movedir[i as usize] as libc::c_double)
                        * fabs(bboxmins[i as usize] as libc::c_double))
                    as libc::c_float
            }
            i += 1
        }
        goalorigin[0usize] = origin[0usize] + movedir[0usize] * -dist;
        goalorigin[1usize] = origin[1usize] + movedir[1usize] * -dist;
        goalorigin[2usize] = origin[2usize] + movedir[2usize] * -dist;
        start[0usize] = goalorigin[0usize];
        start[1usize] = goalorigin[1usize];
        start[2usize] = goalorigin[2usize];
        start[2usize] += 24i32 as libc::c_float;
        end[0usize] = start[0usize];
        end[1usize] = start[1usize];
        end[2usize] = start[2usize];
        end[2usize] -= 100i32 as libc::c_float;
        numareas = trap_AAS_TraceAreas(
            start.as_mut_ptr(),
            end.as_mut_ptr(),
            areas.as_mut_ptr(),
            0 as *mut vec3_t,
            10i32,
        );
        i = 0i32;
        while i < numareas {
            if 0 != trap_AAS_AreaReachability(areas[i as usize]) {
                break;
            }
            i += 1
        }
        if i < numareas {
            (*activategoal).goal.origin[0usize] = origin[0usize];
            (*activategoal).goal.origin[1usize] = origin[1usize];
            (*activategoal).goal.origin[2usize] = origin[2usize];
            (*activategoal).goal.areanum = areas[i as usize];
            (*activategoal).goal.mins[0usize] = mins[0usize] - origin[0usize];
            (*activategoal).goal.mins[1usize] = mins[1usize] - origin[1usize];
            (*activategoal).goal.mins[2usize] = mins[2usize] - origin[2usize];
            (*activategoal).goal.maxs[0usize] = maxs[0usize] - origin[0usize];
            (*activategoal).goal.maxs[1usize] = maxs[1usize] - origin[1usize];
            (*activategoal).goal.maxs[2usize] = maxs[2usize] - origin[2usize];
            i = 0i32;
            while i < 3i32 {
                if movedir[i as usize] < 0i32 as libc::c_float {
                    (*activategoal).goal.maxs[i as usize] = ((*activategoal).goal.maxs[i as usize]
                        as libc::c_double
                        + fabs(movedir[i as usize] as libc::c_double)
                            * fabs(extramaxs[i as usize] as libc::c_double))
                        as vec_t
                } else {
                    (*activategoal).goal.mins[i as usize] = ((*activategoal).goal.mins[i as usize]
                        as libc::c_double
                        + fabs(movedir[i as usize] as libc::c_double)
                            * fabs(extramins[i as usize] as libc::c_double))
                        as vec_t
                }
                i += 1
            }
            (*activategoal).goal.entitynum = entitynum;
            (*activategoal).goal.number = 0i32;
            (*activategoal).goal.flags = 0i32;
            return qtrue as libc::c_int;
        }
    }
    return qfalse as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BotSetMovedir(mut angles: *mut vec_t, mut movedir: *mut vec_t) {
    if 0 != VectorCompare(angles as *const vec_t, VEC_UP.as_mut_ptr() as *const vec_t) {
        *movedir.offset(0isize) = MOVEDIR_UP[0usize];
        *movedir.offset(1isize) = MOVEDIR_UP[1usize];
        *movedir.offset(2isize) = MOVEDIR_UP[2usize]
    } else if 0
        != VectorCompare(
            angles as *const vec_t,
            VEC_DOWN.as_mut_ptr() as *const vec_t,
        )
    {
        *movedir.offset(0isize) = MOVEDIR_DOWN[0usize];
        *movedir.offset(1isize) = MOVEDIR_DOWN[1usize];
        *movedir.offset(2isize) = MOVEDIR_DOWN[2usize]
    } else {
        AngleVectors(
            angles as *const vec_t,
            movedir,
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
    };
}
static mut MOVEDIR_DOWN: vec3_t = [0i32 as vec_t, 0i32 as vec_t, -1i32 as vec_t];
static mut VEC_DOWN: vec3_t = [0i32 as vec_t, -2i32 as vec_t, 0i32 as vec_t];
static mut MOVEDIR_UP: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
/*
==================
BotSetMovedir
==================
*/
static mut VEC_UP: vec3_t = [0i32 as vec_t, -1i32 as vec_t, 0i32 as vec_t];
/*
==================
BotFuncDoorGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotFuncDoorActivateGoal(
    mut bs: *mut bot_state_t,
    mut bspent: libc::c_int,
    mut activategoal: *mut bot_activategoal_t,
) -> libc::c_int {
    let mut modelindex: libc::c_int = 0;
    let mut entitynum: libc::c_int = 0;
    let mut model: [libc::c_char; 1024] = [0; 1024];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        model.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == *model.as_mut_ptr() {
        return qfalse as libc::c_int;
    }
    modelindex = atoi(model.as_mut_ptr().offset(1isize));
    if 0 == modelindex {
        return qfalse as libc::c_int;
    }
    entitynum = BotModelMinsMaxs(
        modelindex,
        ET_MOVER as libc::c_int,
        0i32,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    origin[0usize] = mins[0usize] + maxs[0usize];
    origin[1usize] = mins[1usize] + maxs[1usize];
    origin[2usize] = mins[2usize] + maxs[2usize];
    origin[0usize] = (origin[0usize] as libc::c_double * 0.5f64) as vec_t;
    origin[1usize] = (origin[1usize] as libc::c_double * 0.5f64) as vec_t;
    origin[2usize] = (origin[2usize] as libc::c_double * 0.5f64) as vec_t;
    (*activategoal).target[0usize] = origin[0usize];
    (*activategoal).target[1usize] = origin[1usize];
    (*activategoal).target[2usize] = origin[2usize];
    (*activategoal).shoot = qtrue as libc::c_int;
    (*activategoal).goal.entitynum = entitynum;
    (*activategoal).goal.number = 0i32;
    (*activategoal).goal.flags = 0i32;
    (*activategoal).goal.origin[0usize] = (*bs).origin[0usize];
    (*activategoal).goal.origin[1usize] = (*bs).origin[1usize];
    (*activategoal).goal.origin[2usize] = (*bs).origin[2usize];
    (*activategoal).goal.areanum = (*bs).areanum;
    (*activategoal).goal.mins[0usize] = -8i32 as vec_t;
    (*activategoal).goal.mins[1usize] = -8i32 as vec_t;
    (*activategoal).goal.mins[2usize] = -8i32 as vec_t;
    (*activategoal).goal.maxs[0usize] = 8i32 as vec_t;
    (*activategoal).goal.maxs[1usize] = 8i32 as vec_t;
    (*activategoal).goal.maxs[2usize] = 8i32 as vec_t;
    return qtrue as libc::c_int;
}
/*
==================
BotRandomMove
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotRandomMove(
    mut bs: *mut bot_state_t,
    mut moveresult: *mut bot_moveresult_t,
) {
    let mut dir: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    angles[0usize] = 0i32 as vec_t;
    angles[1usize] = (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
        * 360i32 as libc::c_float;
    angles[2usize] = 0i32 as vec_t;
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        dir.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    trap_BotMoveInDirection((*bs).ms, dir.as_mut_ptr(), 400i32 as libc::c_float, 1i32);
    (*moveresult).failure = qfalse as libc::c_int;
    (*moveresult).movedir[0usize] = dir[0usize];
    (*moveresult).movedir[1usize] = dir[1usize];
    (*moveresult).movedir[2usize] = dir[2usize];
}
//AI to predict obstacles
#[no_mangle]
pub unsafe extern "C" fn BotAIPredictObstacles(
    mut bs: *mut bot_state_t,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
    let mut modelnum: libc::c_int = 0;
    let mut entitynum: libc::c_int = 0;
    let mut bspent: libc::c_int = 0;
    let mut activategoal: bot_activategoal_t = bot_activategoal_s {
        inuse: 0,
        goal: bot_goal_s {
            origin: [0.; 3],
            areanum: 0,
            mins: [0.; 3],
            maxs: [0.; 3],
            entitynum: 0,
            number: 0,
            flags: 0,
            iteminfo: 0,
        },
        time: 0.,
        start_time: 0.,
        justused_time: 0.,
        shoot: 0,
        weapon: 0,
        target: [0.; 3],
        origin: [0.; 3],
        areas: [0; 32],
        numareas: 0,
        areasdisabled: 0,
        next: 0 as *mut bot_activategoal_s,
    };
    let mut route: aas_predictroute_t = aas_predictroute_s {
        endpos: [0.; 3],
        endarea: 0,
        stopevent: 0,
        endcontents: 0,
        endtravelflags: 0,
        numareas: 0,
        time: 0,
    };
    if 0 == bot_predictobstacles.integer {
        return qfalse as libc::c_int;
    }
    if (*bs).predictobstacles_goalareanum == (*goal).areanum
        && (*bs).predictobstacles_time > floattime - 6i32 as libc::c_float
    {
        return qfalse as libc::c_int;
    }
    (*bs).predictobstacles_goalareanum = (*goal).areanum;
    (*bs).predictobstacles_time = floattime;
    trap_AAS_PredictRoute(
        &mut route as *mut aas_predictroute_t as *mut libc::c_void,
        (*bs).areanum,
        (*bs).origin.as_mut_ptr(),
        (*goal).areanum,
        (*bs).tfl,
        100i32,
        1000i32,
        2i32 | 4i32,
        1024i32,
        0x4000000i32,
        0i32,
    );
    if 0 != route.stopevent & 4i32 {
        if 0 != route.endcontents & 1024i32 {
            modelnum = (route.endcontents & 0xffi32 << 24i32) >> 24i32;
            if 0 != modelnum {
                entitynum = BotModelMinsMaxs(
                    modelnum,
                    ET_MOVER as libc::c_int,
                    0i32,
                    0 as *mut vec_t,
                    0 as *mut vec_t,
                );
                if 0 != entitynum {
                    bspent = BotGetActivateGoal(bs, entitynum, &mut activategoal);
                    if 0 != bspent {
                        if !(*bs).activatestack.is_null() && 0 == (*(*bs).activatestack).inuse {
                            (*bs).activatestack = 0 as *mut bot_activategoal_t
                        }
                        if 0 == BotIsGoingToActivateEntity(bs, activategoal.goal.entitynum) {
                            BotGoForActivateGoal(bs, &mut activategoal);
                            return qtrue as libc::c_int;
                        } else {
                            BotEnableActivateGoalAreas(&mut activategoal, qtrue as libc::c_int);
                        }
                    }
                }
            }
        }
    } else if 0 != route.stopevent & 2i32 {
        0 != route.endtravelflags & 0x4000000i32;
    }
    return qfalse as libc::c_int;
}
//pop an activate goal from the stack
#[no_mangle]
pub unsafe extern "C" fn BotPopFromActivateGoalStack(mut bs: *mut bot_state_t) -> libc::c_int {
    if (*bs).activatestack.is_null() {
        return qfalse as libc::c_int;
    }
    BotEnableActivateGoalAreas((*bs).activatestack, qtrue as libc::c_int);
    (*(*bs).activatestack).inuse = qfalse as libc::c_int;
    (*(*bs).activatestack).justused_time = floattime;
    (*bs).activatestack = (*(*bs).activatestack).next;
    return qtrue as libc::c_int;
}
//clear the activate goal stack
#[no_mangle]
pub unsafe extern "C" fn BotClearActivateGoalStack(mut bs: *mut bot_state_t) {
    while !(*bs).activatestack.is_null() {
        BotPopFromActivateGoalStack(bs);
    }
}
//remember the last ordered task
#[no_mangle]
pub unsafe extern "C" fn BotRememberLastOrderedTask(mut bs: *mut bot_state_t) {
    if 0 == (*bs).ordered {
        return;
    }
    (*bs).lastgoal_decisionmaker = (*bs).decisionmaker;
    (*bs).lastgoal_ltgtype = (*bs).ltgtype;
    memcpy(
        &mut (*bs).lastgoal_teamgoal as *mut bot_goal_t as *mut libc::c_void,
        &mut (*bs).teamgoal as *mut bot_goal_t as *const libc::c_void,
        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
    );
    (*bs).lastgoal_teammate = (*bs).teammate;
}
//returns either the alternate route goal or the given goal
#[no_mangle]
pub unsafe extern "C" fn BotAlternateRoute(
    mut bs: *mut bot_state_t,
    mut goal: *mut bot_goal_t,
) -> *mut bot_goal_t {
    let mut t: libc::c_int = 0;
    if 0 != (*bs).altroutegoal.areanum {
        if 0. != (*bs).reachedaltroutegoal_time {
            return goal;
        }
        t = trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            (*bs).altroutegoal.areanum,
            (*bs).tfl,
        );
        if 0 != t && t < 20i32 {
            (*bs).reachedaltroutegoal_time = floattime
        }
        memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).altroutegoal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        return &mut (*bs).altroutegoal;
    }
    return goal;
}
//create a new waypoint
#[no_mangle]
pub unsafe extern "C" fn BotCreateWayPoint(
    mut name: *mut libc::c_char,
    mut origin: *mut vec_t,
    mut areanum: libc::c_int,
) -> *mut bot_waypoint_t {
    let mut wp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    let mut waypointmins: vec3_t = [-8i32 as vec_t, -8i32 as vec_t, -8i32 as vec_t];
    let mut waypointmaxs: vec3_t = [8i32 as vec_t, 8i32 as vec_t, 8i32 as vec_t];
    wp = botai_freewaypoints;
    if wp.is_null() {
        BotAI_Print(
            2i32,
            b"BotCreateWayPoint: Out of waypoints\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as *mut bot_waypoint_t;
    }
    botai_freewaypoints = (*botai_freewaypoints).next;
    Q_strncpyz(
        (*wp).name.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    (*wp).goal.origin[0usize] = *origin.offset(0isize);
    (*wp).goal.origin[1usize] = *origin.offset(1isize);
    (*wp).goal.origin[2usize] = *origin.offset(2isize);
    (*wp).goal.mins[0usize] = waypointmins[0usize];
    (*wp).goal.mins[1usize] = waypointmins[1usize];
    (*wp).goal.mins[2usize] = waypointmins[2usize];
    (*wp).goal.maxs[0usize] = waypointmaxs[0usize];
    (*wp).goal.maxs[1usize] = waypointmaxs[1usize];
    (*wp).goal.maxs[2usize] = waypointmaxs[2usize];
    (*wp).goal.areanum = areanum;
    (*wp).next = 0 as *mut bot_waypoint_s;
    (*wp).prev = 0 as *mut bot_waypoint_s;
    return wp;
}
//find a waypoint with the given name
#[no_mangle]
pub unsafe extern "C" fn BotFindWayPoint(
    mut waypoints: *mut bot_waypoint_t,
    mut name: *mut libc::c_char,
) -> *mut bot_waypoint_t {
    let mut wp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    wp = waypoints;
    while !wp.is_null() {
        if 0 == Q_stricmp((*wp).name.as_mut_ptr(), name) {
            return wp;
        }
        wp = (*wp).next
    }
    return 0 as *mut bot_waypoint_t;
}
//strstr but case insensitive
#[no_mangle]
pub unsafe extern "C" fn stristr(
    mut str: *mut libc::c_char,
    mut charset: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    while 0 != *str {
        i = 0i32;
        while 0 != *charset.offset(i as isize) as libc::c_int
            && 0 != *str.offset(i as isize) as libc::c_int
        {
            if toupper(*charset.offset(i as isize) as libc::c_int)
                != toupper(*str.offset(i as isize) as libc::c_int)
            {
                break;
            }
            i += 1
        }
        if 0 == *charset.offset(i as isize) {
            return str;
        }
        str = str.offset(1isize)
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ClientOnSameTeamFromName(
    mut bs: *mut bot_state_t,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    i = 0i32;
    while i < level.maxclients {
        if !(0 == BotSameTeam(bs, i)) {
            trap_GetConfigstring(
                32i32 + 256i32 + 256i32 + i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            Q_CleanStr(buf.as_mut_ptr());
            if 0 == Q_stricmp(
                Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"n\x00" as *const u8 as *const libc::c_char,
                ),
                name,
            ) {
                return i;
            }
        }
        i += 1
    }
    return -1i32;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotPointAreaNum(mut origin: *mut vec_t) -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    let mut end: vec3_t = [0.; 3];
    areanum = trap_AAS_PointAreaNum(origin);
    if 0 != areanum {
        return areanum;
    }
    end[0usize] = *origin.offset(0isize);
    end[1usize] = *origin.offset(1isize);
    end[2usize] = *origin.offset(2isize);
    end[2usize] += 10i32 as libc::c_float;
    numareas = trap_AAS_TraceAreas(
        origin,
        end.as_mut_ptr(),
        areas.as_mut_ptr(),
        0 as *mut vec3_t,
        10i32,
    );
    if numareas > 0i32 {
        return areas[0usize];
    }
    return 0i32;
}
//
#[no_mangle]
pub unsafe extern "C" fn BotMapScripts(mut bs: *mut bot_state_t) {
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut mapname: [libc::c_char; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut shootbutton: libc::c_int = 0;
    let mut aim_accuracy: libc::c_float = 0.;
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
    let mut dir: vec3_t = [0.; 3];
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
    if 0 == Q_stricmp(
        mapname.as_mut_ptr(),
        b"q3tourney6\x00" as *const u8 as *const libc::c_char,
    ) || 0
        == Q_stricmp(
            mapname.as_mut_ptr(),
            b"q3tourney6_ctf\x00" as *const u8 as *const libc::c_char,
        )
        || 0 == Q_stricmp(
            mapname.as_mut_ptr(),
            b"mpq3tourney6\x00" as *const u8 as *const libc::c_char,
        )
    {
        let mut mins: vec3_t = [694i32 as vec_t, 200i32 as vec_t, 480i32 as vec_t];
        let mut maxs: vec3_t = [968i32 as vec_t, 472i32 as vec_t, 680i32 as vec_t];
        let mut buttonorg: vec3_t = [304i32 as vec_t, 352i32 as vec_t, 920i32 as vec_t];
        (*bs).tfl &= !0x1000000i32;
        if 0 == Q_stricmp(
            mapname.as_mut_ptr(),
            b"mpq3tourney6\x00" as *const u8 as *const libc::c_char,
        ) {
            mins[2usize] += 64i32 as libc::c_float;
            maxs[2usize] += 64i32 as libc::c_float
        }
        if (*bs).origin[0usize] > mins[0usize] && (*bs).origin[0usize] < maxs[0usize] {
            if (*bs).origin[1usize] > mins[1usize] && (*bs).origin[1usize] < maxs[1usize] {
                if (*bs).origin[2usize] > mins[2usize] && (*bs).origin[2usize] < maxs[2usize] {
                    return;
                }
            }
        }
        shootbutton = qfalse as libc::c_int;
        i = 0i32;
        while i < level.maxclients {
            if !(i == (*bs).client) {
                BotEntityInfo(i, &mut entinfo);
                //
                if !(0 == entinfo.valid) {
                    //if the enemy isn't dead and the enemy isn't the bot self
                    if !(0 != EntityIsDead(&mut entinfo) as libc::c_uint
                        || entinfo.number == (*bs).entitynum)
                    {
                        //
                        if entinfo.origin[0usize] > mins[0usize]
                            && entinfo.origin[0usize] < maxs[0usize]
                        {
                            if entinfo.origin[1usize] > mins[1usize]
                                && entinfo.origin[1usize] < maxs[1usize]
                            {
                                if entinfo.origin[2usize] > mins[2usize]
                                    && entinfo.origin[2usize] < maxs[2usize]
                                {
                                    //if there's a team mate below the crusher
                                    if 0 != BotSameTeam(bs, i) {
                                        shootbutton = qfalse as libc::c_int;
                                        break;
                                    } else if (*bs).enemy == i {
                                        shootbutton = qtrue as libc::c_int
                                    }
                                }
                            }
                        }
                    }
                }
            }
            i += 1
        }
        if 0 != shootbutton {
            (*bs).flags |= 32i32;
            dir[0usize] = buttonorg[0usize] - (*bs).eye[0usize];
            dir[1usize] = buttonorg[1usize] - (*bs).eye[1usize];
            dir[2usize] = buttonorg[2usize] - (*bs).eye[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            aim_accuracy = trap_Characteristic_BFloat(
                (*bs).character,
                7i32,
                0i32 as libc::c_float,
                1i32 as libc::c_float,
            );
            (*bs).ideal_viewangles[0usize] = ((*bs).ideal_viewangles[0usize] as libc::c_double
                + 8i32 as libc::c_double
                    * (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64))
                    * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
                as vec_t;
            (*bs).ideal_viewangles[0usize] = AngleMod((*bs).ideal_viewangles[0usize]);
            (*bs).ideal_viewangles[1usize] = ((*bs).ideal_viewangles[1usize] as libc::c_double
                + 8i32 as libc::c_double
                    * (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64))
                    * (1i32 as libc::c_float - aim_accuracy) as libc::c_double)
                as vec_t;
            (*bs).ideal_viewangles[1usize] = AngleMod((*bs).ideal_viewangles[1usize]);
            if 0 != InFieldOfVision(
                (*bs).viewangles.as_mut_ptr(),
                20i32 as libc::c_float,
                (*bs).ideal_viewangles.as_mut_ptr(),
            ) as u64
            {
                trap_EA_Attack((*bs).client);
            }
        }
    };
}
/*
==================
EntityHasQuad
==================
*/
#[no_mangle]
pub unsafe extern "C" fn EntityHasQuad(mut entinfo: *mut aas_entityinfo_t) -> qboolean {
    if 0 != (*entinfo).powerups & 1i32 << PW_QUAD as libc::c_int {
        return qtrue;
    }
    return qfalse;
}
/*
==================
BotPrintActivateGoalInfo
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotPrintActivateGoalInfo(
    mut bs: *mut bot_state_t,
    mut activategoal: *mut bot_activategoal_t,
    mut bspent: libc::c_int,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut classname: [libc::c_char; 128] = [0; 128];
    let mut buf: [libc::c_char; 128] = [0; 128];
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        classname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != (*activategoal).shoot {
        Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"%s: I have to shoot at a %s from %1.1f %1.1f %1.1f in area %d\n\x00" as *const u8
                as *const libc::c_char,
            netname.as_mut_ptr(),
            classname.as_mut_ptr(),
            (*activategoal).goal.origin[0usize] as libc::c_double,
            (*activategoal).goal.origin[1usize] as libc::c_double,
            (*activategoal).goal.origin[2usize] as libc::c_double,
            (*activategoal).goal.areanum,
        );
    } else {
        Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"%s: I have to activate a %s at %1.1f %1.1f %1.1f in area %d\n\x00" as *const u8
                as *const libc::c_char,
            netname.as_mut_ptr(),
            classname.as_mut_ptr(),
            (*activategoal).goal.origin[0usize] as libc::c_double,
            (*activategoal).goal.origin[1usize] as libc::c_double,
            (*activategoal).goal.origin[2usize] as libc::c_double,
            (*activategoal).goal.areanum,
        );
    }
    trap_EA_Say((*bs).client, buf.as_mut_ptr());
}
/*
==================
BotSetEntityNumForGoalWithModel
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSetEntityNumForGoalWithModel(
    mut goal: *mut bot_goal_t,
    mut eType: libc::c_int,
    mut modelname: *mut libc::c_char,
) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    let mut modelindex: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    modelindex = G_ModelIndex(modelname);
    ent = &mut g_entities[0usize] as *mut gentity_t;
    i = 0i32;
    while i < level.num_entities {
        if !(0 == (*ent).inuse as u64) {
            if !(0 != eType && (*ent).s.eType != eType) {
                if !((*ent).s.modelindex != modelindex) {
                    dir[0usize] = (*goal).origin[0usize] - (*ent).s.origin[0usize];
                    dir[1usize] = (*goal).origin[1usize] - (*ent).s.origin[1usize];
                    dir[2usize] = (*goal).origin[2usize] - (*ent).s.origin[2usize];
                    if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                        < (10i32 * 10i32) as libc::c_float
                    {
                        (*goal).entitynum = i;
                        return;
                    }
                }
            }
        }
        i += 1;
        ent = ent.offset(1isize)
    }
}
/*
==================
BotSetEntityNumForGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSetEntityNumForGoal(
    mut goal: *mut bot_goal_t,
    mut classname: *mut libc::c_char,
) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    ent = &mut g_entities[0usize] as *mut gentity_t;
    i = 0i32;
    while i < level.num_entities {
        if !(0 == (*ent).inuse as u64) {
            if !(Q_stricmp((*ent).classname, classname) != 0i32) {
                dir[0usize] = (*goal).origin[0usize] - (*ent).s.origin[0usize];
                dir[1usize] = (*goal).origin[1usize] - (*ent).s.origin[1usize];
                dir[2usize] = (*goal).origin[2usize] - (*ent).s.origin[2usize];
                if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                    < (10i32 * 10i32) as libc::c_float
                {
                    (*goal).entitynum = i;
                    return;
                }
            }
        }
        i += 1;
        ent = ent.offset(1isize)
    }
}
/*
==================
BotSetEntityNumForGoalWithActivator
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSetEntityNumForGoalWithActivator(
    mut goal: *mut bot_goal_t,
    mut classname: *mut libc::c_char,
) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    ent = &mut g_entities[0usize] as *mut gentity_t;
    i = 0i32;
    while i < level.num_entities {
        if !(0 == (*ent).inuse as u64 || (*ent).activator.is_null()) {
            if !(Q_stricmp((*(*ent).activator).classname, classname) != 0i32) {
                dir[0usize] = (*goal).origin[0usize] - (*ent).s.origin[0usize];
                dir[1usize] = (*goal).origin[1usize] - (*ent).s.origin[1usize];
                dir[2usize] = (*goal).origin[2usize] - (*ent).s.origin[2usize];
                if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                    < (10i32 * 10i32) as libc::c_float
                {
                    (*goal).entitynum = i;
                    return;
                }
            }
        }
        i += 1;
        ent = ent.offset(1isize)
    }
}
/*
==================
BotGoalForBSPEntity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGoalForBSPEntity(
    mut classname: *mut libc::c_char,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
    let mut value: [libc::c_char; 1024] = [0; 1024];
    let mut origin: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut ent: libc::c_int = 0;
    let mut numareas: libc::c_int = 0;
    let mut areas: [libc::c_int; 10] = [0; 10];
    memset(
        goal as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
    );
    ent = trap_AAS_NextBSPEntity(0i32);
    while 0 != ent {
        if !(0
            == trap_AAS_ValueForBSPEpairKey(
                ent,
                b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            ))
        {
            if 0 == strcmp(value.as_mut_ptr(), classname) {
                if 0 == trap_AAS_VectorForBSPEpairKey(
                    ent,
                    b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    origin.as_mut_ptr(),
                ) {
                    return qfalse as libc::c_int;
                }
                (*goal).origin[0usize] = origin[0usize];
                (*goal).origin[1usize] = origin[1usize];
                (*goal).origin[2usize] = origin[2usize];
                start[0usize] = origin[0usize];
                start[1usize] = origin[1usize];
                start[2usize] = origin[2usize];
                start[2usize] -= 32i32 as libc::c_float;
                end[0usize] = origin[0usize];
                end[1usize] = origin[1usize];
                end[2usize] = origin[2usize];
                end[2usize] += 32i32 as libc::c_float;
                numareas = trap_AAS_TraceAreas(
                    start.as_mut_ptr(),
                    end.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    0 as *mut vec3_t,
                    10i32,
                );
                if 0 == numareas {
                    return qfalse as libc::c_int;
                }
                (*goal).areanum = areas[0usize];
                return qtrue as libc::c_int;
            }
        }
        ent = trap_AAS_NextBSPEntity(ent)
    }
    return qfalse as libc::c_int;
}
