#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(custom_attribute, libc)]
use ai_chat::{
    BotChatTime, BotChat_Death, BotChat_EndLevel, BotChat_EnemySuicide, BotChat_HitNoDeath,
    BotChat_HitNoKill, BotChat_HitTalking, BotChat_Kill, BotChat_Random, BotChat_StartLevel,
};
use ai_dmq3::{
    bot_grapple, ctf_blueflag, ctf_redflag, gametype, BotAIBlocked, BotAIPredictObstacles,
    BotAimAtEnemy, BotAlternateRoute, BotAttackMove, BotBattleUseItems, BotCTFCarryingFlag,
    BotCanAndWantsToRocketJump, BotCheckAttack, BotChooseWeapon, BotClearActivateGoalStack,
    BotEntityVisible, BotFindEnemy, BotHasPersistantPowerupAndWeapon, BotInLavaOrSlime,
    BotIntermission, BotIsDead, BotIsObserver, BotMapScripts, BotPointAreaNum,
    BotPopFromActivateGoalStack, BotRoamGoal, BotSetupForMovement, BotTeam, BotTeamGoals,
    BotUpdateBattleInventory, BotWantsToCamp, BotWantsToChase, BotWantsToRetreat, ClientName,
    EasyClientName, EntityIsDead, EntityIsInvisible, EntityIsShooting, InFieldOfVision,
};
use ai_main::{
    bot_activategoal_s, bot_activategoal_t, bot_developer, bot_state_s, bot_state_t,
    bot_waypoint_s, bot_waypoint_t, floattime, BotAILoadMap, BotAISetup, BotAISetupClient,
    BotAIShutdown, BotAIShutdownClient, BotAIStartFrame, BotAI_GetEntityState, BotAI_Trace,
    BotEntityInfo, BotInterbreedEndMatch, BotResetState, BotTestAAS,
};
use ai_team::BotVoiceChatOnly;
use ai_variadic_h::{BotAI_BotInitialChat, BotAI_Print};
use be_aas_h::{aas_entityinfo_s, aas_entityinfo_t};
use be_ai_goal_h::{bot_goal_s, bot_goal_t};
use be_ai_move_h::{bot_moveresult_s, bot_moveresult_t};
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
    BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
    BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
use bg_public_h::{
    unnamed, unnamed_0, unnamed_1, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE,
    GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR,
    PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS,
    TEAM_RED, TEAM_SPECTATOR,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
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
    bot_settings_s, bot_settings_t, trap_AAS_AreaReachability, trap_AAS_AreaTravelTimeToGoalArea,
    trap_AAS_PointContents, trap_AAS_Swimming, trap_BotChooseLTGItem, trap_BotChooseNBGItem,
    trap_BotEmptyGoalStack, trap_BotEnterChat, trap_BotGetSecondGoal, trap_BotGetTopGoal,
    trap_BotGoalName, trap_BotItemGoalInVisButNotVisible, trap_BotMoveInDirection,
    trap_BotMoveToGoal, trap_BotMovementViewTarget, trap_BotPopGoal, trap_BotPushGoal,
    trap_BotResetAvoidGoals, trap_BotResetAvoidReach, trap_BotResetGoalState,
    trap_BotResetLastAvoidReach, trap_BotResetMoveState, trap_BotSetAvoidGoalTime,
    trap_BotTouchingGoal, trap_Characteristic_BFloat, trap_EA_Action, trap_EA_Attack,
    trap_EA_Crouch, trap_EA_Gesture, trap_EA_Respawn, trap_EA_Talk, trap_PointContents,
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
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, playerState_s,
    playerState_t, qboolean, qfalse, qtrue, trType_t, trajectory_t, usercmd_s, usercmd_t, vec3_t,
    vec_t, vmCvar_t, Com_sprintf, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE,
    TR_STATIONARY,
};
use stdlib::{memcpy, memset, rand, sqrt, strcat, strcpy};
extern crate libc;

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
 * name:		ai_dmnet.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Intermission(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
    BotRecordNodeSwitch(
        bs,
        b"intermission\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    BotResetState(bs);
    if 0 != BotChat_EndLevel(bs) {
        trap_BotEnterChat((*bs).cs, 0i32, (*bs).chatto);
    }
    (*bs).ainode = Some(AINode_Intermission);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Intermission(mut bs: *mut bot_state_t) -> libc::c_int {
    if 0 == BotIntermission(bs) as u64 {
        if 0 != BotChat_StartLevel(bs) {
            (*bs).stand_time = floattime + BotChatTime(bs)
        } else {
            (*bs).stand_time = floattime + 2i32 as libc::c_float
        }
        AIEnter_Stand(
            bs,
            b"intermission: chat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Stand(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
    BotRecordNodeSwitch(
        bs,
        b"stand\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).standfindenemy_time = floattime + 1i32 as libc::c_float;
    (*bs).ainode = Some(AINode_Stand);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Stand(mut bs: *mut bot_state_t) -> libc::c_int {
    if (*bs).lastframe_health > (*bs).inventory[29usize] {
        if 0 != BotChat_HitTalking(bs) {
            (*bs).standfindenemy_time =
                ((floattime + BotChatTime(bs)) as libc::c_double + 0.1f64) as libc::c_float;
            (*bs).stand_time =
                ((floattime + BotChatTime(bs)) as libc::c_double + 0.1f64) as libc::c_float
        }
    }
    if (*bs).standfindenemy_time < floattime {
        if 0 != BotFindEnemy(bs, -1i32) {
            AIEnter_Battle_Fight(
                bs,
                b"stand: found enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
        (*bs).standfindenemy_time = floattime + 1i32 as libc::c_float
    }
    trap_EA_Talk((*bs).client);
    if (*bs).stand_time < floattime {
        trap_BotEnterChat((*bs).cs, 0i32, (*bs).chatto);
        AIEnter_Seek_LTG(
            bs,
            b"stand: time out\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Seek_LTG(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
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
    let mut buf: [libc::c_char; 144] = [0; 144];
    if 0 != trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void) {
        trap_BotGoalName(goal.number, buf.as_mut_ptr(), 144i32);
        BotRecordNodeSwitch(
            bs,
            b"seek LTG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
            s,
        );
    } else {
        BotRecordNodeSwitch(
            bs,
            b"seek LTG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"no goal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s,
        );
    }
    (*bs).ainode = Some(AINode_Seek_LTG);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Seek_LTG(mut bs: *mut bot_state_t) -> libc::c_int {
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
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
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
    let mut range: libc::c_int = 0;
    if 0 != BotIsObserver(bs) as u64 {
        AIEnter_Observer(
            bs,
            b"seek ltg: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIntermission(bs) as u64 {
        AIEnter_Intermission(
            bs,
            b"seek ltg: intermission\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIsDead(bs) as u64 {
        AIEnter_Respawn(
            bs,
            b"seek ltg: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotChat_Random(bs) {
        (*bs).stand_time = floattime + BotChatTime(bs);
        AIEnter_Stand(
            bs,
            b"seek ltg: random chat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    (*bs).tfl = 0x2i32
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
        | 0x1000000i32;
    if 0 != bot_grapple.integer {
        (*bs).tfl |= 0x4000i32
    }
    if 0 != BotInLavaOrSlime(bs) as u64 {
        (*bs).tfl |= 0x400000i32 | 0x200000i32
    }
    if 0 != BotCanAndWantsToRocketJump(bs) {
        (*bs).tfl |= 0x1000i32
    }
    BotMapScripts(bs);
    (*bs).enemy = -1i32;
    if (*bs).killedenemy_time > floattime - 2i32 as libc::c_float {
        if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
            < (*bs).thinktime * 1i32 as libc::c_float
        {
            trap_EA_Gesture((*bs).client);
        }
    }
    if 0 != BotFindEnemy(bs, -1i32) {
        if 0 != BotWantsToRetreat(bs) {
            AIEnter_Battle_Retreat(
                bs,
                b"seek ltg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        } else {
            trap_BotResetLastAvoidReach((*bs).ms);
            trap_BotEmptyGoalStack((*bs).gs);
            AIEnter_Battle_Fight(
                bs,
                b"seek ltg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    BotTeamGoals(bs, qfalse as libc::c_int);
    if 0 == BotLongTermGoal(bs, (*bs).tfl, qfalse as libc::c_int, &mut goal) {
        return qtrue as libc::c_int;
    }
    if (*bs).check_time < floattime {
        (*bs).check_time = (floattime as libc::c_double + 0.5f64) as libc::c_float;
        BotWantsToCamp(bs);
        if (*bs).ltgtype == 3i32 {
            range = 400i32
        } else {
            range = 150i32
        }
        if gametype == GT_CTF as libc::c_int {
            if 0 != BotCTFCarryingFlag(bs) {
                range = 50i32
            }
        }
        if 0 != BotNearbyGoal(bs, (*bs).tfl, &mut goal, range as libc::c_float) {
            trap_BotResetLastAvoidReach((*bs).ms);
            (*bs).nbg_time = ((floattime + 4i32 as libc::c_float) as libc::c_double
                + range as libc::c_double * 0.01f64) as libc::c_float;
            AIEnter_Seek_NBG(
                bs,
                b"ltg seek: nbg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    if 0 != BotAIPredictObstacles(bs, &mut goal) {
        return qfalse as libc::c_int;
    }
    BotSetupForMovement(bs);
    trap_BotMoveToGoal(
        &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    if 0 != moveresult.failure {
        trap_BotResetAvoidReach((*bs).ms);
        (*bs).ltg_time = 0i32 as libc::c_float
    }
    BotAIBlocked(bs, &mut moveresult, qtrue as libc::c_int);
    BotClearPath(bs, &mut moveresult);
    if 0 != moveresult.flags & (8i32 | 1i32 | 2i32) {
        (*bs).ideal_viewangles[0usize] = moveresult.ideal_viewangles[0usize];
        (*bs).ideal_viewangles[1usize] = moveresult.ideal_viewangles[1usize];
        (*bs).ideal_viewangles[2usize] = moveresult.ideal_viewangles[2usize]
    } else if 0 != moveresult.flags & 4i32 {
        if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double)
            < (*bs).thinktime as libc::c_double * 0.8f64
        {
            BotRoamGoal(bs, target.as_mut_ptr());
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2usize] =
                ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
        }
    } else if 0 == (*bs).flags & 32i32 {
        if 0 != trap_BotMovementViewTarget(
            (*bs).ms,
            &mut goal as *mut bot_goal_t as *mut libc::c_void,
            (*bs).tfl,
            300i32 as libc::c_float,
            target.as_mut_ptr(),
        ) {
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else if 0. != VectorLengthSquared(moveresult.movedir.as_mut_ptr() as *const vec_t) {
            vectoangles(
                moveresult.movedir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
            as libc::c_double)
            < (*bs).thinktime as libc::c_double * 0.8f64
        {
            BotRoamGoal(bs, target.as_mut_ptr());
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2usize] =
                ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
        }
        (*bs).ideal_viewangles[2usize] =
            ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
    }
    if 0 != moveresult.flags & 16i32 {
        (*bs).weaponnum = moveresult.weapon
    }
    return qtrue as libc::c_int;
}
/*
==================
BotClearPath

 try to deactivate obstacles like proximity mines on the bot's path
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotClearPath(
    mut bs: *mut bot_state_t,
    mut moveresult: *mut bot_moveresult_t,
) {
    let mut i: libc::c_int = 0;
    let mut bestmine: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut bestdist: libc::c_float = 0.;
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
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
    if 0 != (*bs).kamikazebody {
        if 0 == (*moveresult).flags & (1i32 | 16i32) {
            BotAI_GetEntityState((*bs).kamikazebody, &mut state);
            target[0usize] = state.pos.trBase[0usize];
            target[1usize] = state.pos.trBase[1usize];
            target[2usize] = state.pos.trBase[2usize];
            target[2usize] += 8i32 as libc::c_float;
            dir[0usize] = target[0usize] - (*bs).eye[0usize];
            dir[1usize] = target[1usize] - (*bs).eye[1usize];
            dir[2usize] = target[2usize] - (*bs).eye[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*moveresult).ideal_viewangles.as_mut_ptr(),
            );
            (*moveresult).weapon = BotSelectActivateWeapon(bs);
            if (*moveresult).weapon == -1i32 {
                (*moveresult).weapon = 0i32
            }
            if 0 != (*moveresult).weapon {
                (*moveresult).flags |= 16i32 | 1i32;
                if (*bs).cur_ps.weapon == (*moveresult).weapon {
                    if 0 != InFieldOfVision(
                        (*bs).viewangles.as_mut_ptr(),
                        20i32 as libc::c_float,
                        (*moveresult).ideal_viewangles.as_mut_ptr(),
                    ) as u64
                    {
                        BotAI_Trace(
                            &mut bsptrace,
                            (*bs).eye.as_mut_ptr(),
                            0 as *mut vec_t,
                            0 as *mut vec_t,
                            target.as_mut_ptr(),
                            (*bs).entitynum,
                            1i32 | 0x2000000i32 | 0x4000000i32,
                        );
                        if bsptrace.fraction as libc::c_double >= 1.0f64
                            || bsptrace.ent == state.number
                        {
                            trap_EA_Attack((*bs).client);
                        }
                    }
                }
            }
        }
    }
    if 0 != (*moveresult).flags & 256i32 {
        (*bs).blockedbyavoidspot_time = floattime + 5i32 as libc::c_float
    }
    if (*bs).blockedbyavoidspot_time > floattime && 0 == (*moveresult).flags & (1i32 | 16i32) {
        bestdist = 300i32 as libc::c_float;
        bestmine = -1i32;
        i = 0i32;
        while i < (*bs).numproxmines {
            BotAI_GetEntityState((*bs).proxmines[i as usize], &mut state);
            dir[0usize] = state.pos.trBase[0usize] - (*bs).origin[0usize];
            dir[1usize] = state.pos.trBase[1usize] - (*bs).origin[1usize];
            dir[2usize] = state.pos.trBase[2usize] - (*bs).origin[2usize];
            dist = VectorLength(dir.as_mut_ptr() as *const vec_t);
            if dist < bestdist {
                bestdist = dist;
                bestmine = i
            }
            i += 1
        }
        if bestmine != -1i32 {
            BotAI_GetEntityState((*bs).proxmines[bestmine as usize], &mut state);
            target[0usize] = state.pos.trBase[0usize];
            target[1usize] = state.pos.trBase[1usize];
            target[2usize] = state.pos.trBase[2usize];
            target[2usize] += 2i32 as libc::c_float;
            dir[0usize] = target[0usize] - (*bs).eye[0usize];
            dir[1usize] = target[1usize] - (*bs).eye[1usize];
            dir[2usize] = target[2usize] - (*bs).eye[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*moveresult).ideal_viewangles.as_mut_ptr(),
            );
            if (*bs).inventory[11usize] > 0i32 && (*bs).inventory[21usize] > 0i32 {
                (*moveresult).weapon = 8i32
            } else if (*bs).inventory[8usize] > 0i32 && (*bs).inventory[23usize] > 0i32 {
                (*moveresult).weapon = 5i32
            } else if (*bs).inventory[13usize] > 0i32 && (*bs).inventory[25usize] > 0i32 {
                (*moveresult).weapon = 9i32
            } else {
                (*moveresult).weapon = 0i32
            }
            if 0 != (*moveresult).weapon {
                (*moveresult).flags |= 16i32 | 1i32;
                if (*bs).cur_ps.weapon == (*moveresult).weapon {
                    if 0 != InFieldOfVision(
                        (*bs).viewangles.as_mut_ptr(),
                        20i32 as libc::c_float,
                        (*moveresult).ideal_viewangles.as_mut_ptr(),
                    ) as u64
                    {
                        BotAI_Trace(
                            &mut bsptrace,
                            (*bs).eye.as_mut_ptr(),
                            0 as *mut vec_t,
                            0 as *mut vec_t,
                            target.as_mut_ptr(),
                            (*bs).entitynum,
                            1i32 | 0x2000000i32 | 0x4000000i32,
                        );
                        if bsptrace.fraction as libc::c_double >= 1.0f64
                            || bsptrace.ent == state.number
                        {
                            trap_EA_Attack((*bs).client);
                        }
                    }
                }
            }
        }
    };
}
/*
==================
BotSelectActivateWeapon
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSelectActivateWeapon(mut bs: *mut bot_state_t) -> libc::c_int {
    if (*bs).inventory[6usize] > 0i32 && (*bs).inventory[19usize] > 0i32 {
        return 2i32;
    } else if (*bs).inventory[5usize] > 0i32 && (*bs).inventory[18usize] > 0i32 {
        return 3i32;
    } else if (*bs).inventory[11usize] > 0i32 && (*bs).inventory[21usize] > 0i32 {
        return 8i32;
    } else if (*bs).inventory[9usize] > 0i32 && (*bs).inventory[22usize] > 0i32 {
        return 6i32;
    } else if (*bs).inventory[7usize] > 0i32 && (*bs).inventory[20usize] > 0i32 {
        return 4i32;
    } else if (*bs).inventory[10usize] > 0i32 && (*bs).inventory[24usize] > 0i32 {
        return 7i32;
    } else if (*bs).inventory[8usize] > 0i32 && (*bs).inventory[23usize] > 0i32 {
        return 5i32;
    } else if (*bs).inventory[13usize] > 0i32 && (*bs).inventory[25usize] > 0i32 {
        return 9i32;
    } else {
        return -1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Seek_NBG(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
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
    let mut buf: [libc::c_char; 144] = [0; 144];
    if 0 != trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void) {
        trap_BotGoalName(goal.number, buf.as_mut_ptr(), 144i32);
        BotRecordNodeSwitch(
            bs,
            b"seek NBG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
            s,
        );
    } else {
        BotRecordNodeSwitch(
            bs,
            b"seek NBG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"no goal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s,
        );
    }
    (*bs).ainode = Some(AINode_Seek_NBG);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Seek_NBG(mut bs: *mut bot_state_t) -> libc::c_int {
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
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
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
    if 0 != BotIsObserver(bs) as u64 {
        AIEnter_Observer(
            bs,
            b"seek nbg: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIntermission(bs) as u64 {
        AIEnter_Intermission(
            bs,
            b"seek nbg: intermision\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIsDead(bs) as u64 {
        AIEnter_Respawn(
            bs,
            b"seek nbg: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    (*bs).tfl = 0x2i32
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
        | 0x1000000i32;
    if 0 != bot_grapple.integer {
        (*bs).tfl |= 0x4000i32
    }
    if 0 != BotInLavaOrSlime(bs) as u64 {
        (*bs).tfl |= 0x400000i32 | 0x200000i32
    }
    if 0 != BotCanAndWantsToRocketJump(bs) {
        (*bs).tfl |= 0x1000i32
    }
    BotMapScripts(bs);
    (*bs).enemy = -1i32;
    if 0 == trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void) {
        (*bs).nbg_time = 0i32 as libc::c_float
    } else if 0 != BotReachedGoal(bs, &mut goal) {
        BotChooseWeapon(bs);
        (*bs).nbg_time = 0i32 as libc::c_float
    }
    if (*bs).nbg_time < floattime {
        trap_BotPopGoal((*bs).gs);
        (*bs).check_time = (floattime as libc::c_double + 0.05f64) as libc::c_float;
        AIEnter_Seek_LTG(
            bs,
            b"seek nbg: time out\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotAIPredictObstacles(bs, &mut goal) {
        return qfalse as libc::c_int;
    }
    BotSetupForMovement(bs);
    trap_BotMoveToGoal(
        &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    if 0 != moveresult.failure {
        trap_BotResetAvoidReach((*bs).ms);
        (*bs).nbg_time = 0i32 as libc::c_float
    }
    BotAIBlocked(bs, &mut moveresult, qtrue as libc::c_int);
    BotClearPath(bs, &mut moveresult);
    if 0 != moveresult.flags & (8i32 | 1i32 | 2i32) {
        (*bs).ideal_viewangles[0usize] = moveresult.ideal_viewangles[0usize];
        (*bs).ideal_viewangles[1usize] = moveresult.ideal_viewangles[1usize];
        (*bs).ideal_viewangles[2usize] = moveresult.ideal_viewangles[2usize]
    } else if 0 != moveresult.flags & 4i32 {
        if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double)
            < (*bs).thinktime as libc::c_double * 0.8f64
        {
            BotRoamGoal(bs, target.as_mut_ptr());
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2usize] =
                ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
        }
    } else if 0 == (*bs).flags & 32i32 {
        if 0 == trap_BotGetSecondGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void) {
            trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void);
        }
        if 0 != trap_BotMovementViewTarget(
            (*bs).ms,
            &mut goal as *mut bot_goal_t as *mut libc::c_void,
            (*bs).tfl,
            300i32 as libc::c_float,
            target.as_mut_ptr(),
        ) {
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else {
            vectoangles(
                moveresult.movedir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        }
        (*bs).ideal_viewangles[2usize] =
            ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
    }
    if 0 != moveresult.flags & 16i32 {
        (*bs).weaponnum = moveresult.weapon
    }
    if 0 != BotFindEnemy(bs, -1i32) {
        if 0 != BotWantsToRetreat(bs) {
            AIEnter_Battle_NBG(
                bs,
                b"seek nbg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            trap_BotResetLastAvoidReach((*bs).ms);
            trap_BotEmptyGoalStack((*bs).gs);
            AIEnter_Battle_Fight(
                bs,
                b"seek nbg: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Battle_Fight(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
    BotRecordNodeSwitch(
        bs,
        b"battle fight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    trap_BotResetLastAvoidReach((*bs).ms);
    (*bs).ainode = Some(AINode_Battle_Fight);
    (*bs).flags &= !64i32;
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Battle_Fight(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut areanum: libc::c_int = 0;
    let mut target: vec3_t = [0.; 3];
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
    if 0 != BotIsObserver(bs) as u64 {
        AIEnter_Observer(
            bs,
            b"battle fight: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIntermission(bs) as u64 {
        AIEnter_Intermission(
            bs,
            b"battle fight: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIsDead(bs) as u64 {
        AIEnter_Respawn(
            bs,
            b"battle fight: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    0 != BotFindEnemy(bs, (*bs).enemy);
    if (*bs).enemy < 0i32 {
        AIEnter_Seek_LTG(
            bs,
            b"battle fight: no enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    BotEntityInfo((*bs).enemy, &mut entinfo);
    if 0. != (*bs).enemydeath_time {
        if ((*bs).enemydeath_time as libc::c_double) < floattime as libc::c_double - 1.0f64 {
            (*bs).enemydeath_time = 0i32 as libc::c_float;
            if 0 != (*bs).enemysuicide {
                BotChat_EnemySuicide(bs);
            }
            if (*bs).lastkilledplayer == (*bs).enemy && 0 != BotChat_Kill(bs) {
                (*bs).stand_time = floattime + BotChatTime(bs);
                AIEnter_Stand(
                    bs,
                    b"battle fight: enemy dead\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            } else {
                (*bs).ltg_time = 0i32 as libc::c_float;
                AIEnter_Seek_LTG(
                    bs,
                    b"battle fight: enemy dead\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            return qfalse as libc::c_int;
        }
    } else if 0 != EntityIsDead(&mut entinfo) as u64 {
        (*bs).enemydeath_time = floattime
    }
    if 0 != EntityIsInvisible(&mut entinfo) as libc::c_uint
        && 0 == EntityIsShooting(&mut entinfo) as u64
    {
        if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double)
            < 0.2f64
        {
            AIEnter_Seek_LTG(
                bs,
                b"battle fight: invisible\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    target[0usize] = entinfo.origin[0usize];
    target[1usize] = entinfo.origin[1usize];
    target[2usize] = entinfo.origin[2usize];
    (*bs).enemy >= 64i32;
    areanum = BotPointAreaNum(target.as_mut_ptr());
    if 0 != areanum && 0 != trap_AAS_AreaReachability(areanum) {
        (*bs).lastenemyorigin[0usize] = target[0usize];
        (*bs).lastenemyorigin[1usize] = target[1usize];
        (*bs).lastenemyorigin[2usize] = target[2usize];
        (*bs).lastenemyareanum = areanum
    }
    BotUpdateBattleInventory(bs, (*bs).enemy);
    if (*bs).lastframe_health > (*bs).inventory[29usize] {
        if 0 != BotChat_HitNoDeath(bs) {
            (*bs).stand_time = floattime + BotChatTime(bs);
            AIEnter_Stand(
                bs,
                b"battle fight: chat health decreased\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    if (*bs).cur_ps.persistant[PERS_HITS as libc::c_int as usize] > (*bs).lasthitcount {
        if 0 != BotChat_HitNoKill(bs) {
            (*bs).stand_time = floattime + BotChatTime(bs);
            AIEnter_Stand(
                bs,
                b"battle fight: chat hit someone\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    if 0.
        == BotEntityVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            360i32 as libc::c_float,
            (*bs).enemy,
        )
    {
        if 0 != BotWantsToChase(bs) {
            AIEnter_Battle_Chase(
                bs,
                b"battle fight: enemy out of sight\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        } else {
            AIEnter_Seek_LTG(
                bs,
                b"battle fight: enemy out of sight\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    BotBattleUseItems(bs);
    (*bs).tfl = 0x2i32
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
        | 0x1000000i32;
    if 0 != bot_grapple.integer {
        (*bs).tfl |= 0x4000i32
    }
    if 0 != BotInLavaOrSlime(bs) as u64 {
        (*bs).tfl |= 0x400000i32 | 0x200000i32
    }
    if 0 != BotCanAndWantsToRocketJump(bs) {
        (*bs).tfl |= 0x1000i32
    }
    BotChooseWeapon(bs);
    moveresult = BotAttackMove(bs, (*bs).tfl);
    if 0 != moveresult.failure {
        trap_BotResetAvoidReach((*bs).ms);
        (*bs).ltg_time = 0i32 as libc::c_float
    }
    BotAIBlocked(bs, &mut moveresult, qfalse as libc::c_int);
    BotAimAtEnemy(bs);
    BotCheckAttack(bs);
    if 0 == (*bs).flags & 64i32 {
        if 0 != BotWantsToRetreat(bs) {
            AIEnter_Battle_Retreat(
                bs,
                b"battle fight: wants to retreat\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qtrue as libc::c_int;
        }
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Battle_Retreat(
    mut bs: *mut bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"battle retreat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).ainode = Some(AINode_Battle_Retreat);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Battle_Retreat(mut bs: *mut bot_state_t) -> libc::c_int {
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
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut attack_skill: libc::c_float = 0.;
    let mut range: libc::c_float = 0.;
    let mut areanum: libc::c_int = 0;
    if 0 != BotIsObserver(bs) as u64 {
        AIEnter_Observer(
            bs,
            b"battle retreat: observer\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIntermission(bs) as u64 {
        AIEnter_Intermission(
            bs,
            b"battle retreat: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIsDead(bs) as u64 {
        AIEnter_Respawn(
            bs,
            b"battle retreat: bot dead\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if (*bs).enemy < 0i32 {
        AIEnter_Seek_LTG(
            bs,
            b"battle retreat: no enemy\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    BotEntityInfo((*bs).enemy, &mut entinfo);
    if 0 != EntityIsDead(&mut entinfo) as u64 {
        AIEnter_Seek_LTG(
            bs,
            b"battle retreat: enemy dead\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    0 != BotFindEnemy(bs, (*bs).enemy);
    (*bs).tfl = 0x2i32
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
        | 0x1000000i32;
    if 0 != bot_grapple.integer {
        (*bs).tfl |= 0x4000i32
    }
    if 0 != BotInLavaOrSlime(bs) as u64 {
        (*bs).tfl |= 0x400000i32 | 0x200000i32
    }
    BotMapScripts(bs);
    BotUpdateBattleInventory(bs, (*bs).enemy);
    if 0 != BotWantsToChase(bs) {
        trap_BotEmptyGoalStack((*bs).gs);
        AIEnter_Battle_Chase(
            bs,
            b"battle retreat: wants to chase\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0.
        != BotEntityVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            360i32 as libc::c_float,
            (*bs).enemy,
        )
    {
        (*bs).enemyvisible_time = floattime;
        target[0usize] = entinfo.origin[0usize];
        target[1usize] = entinfo.origin[1usize];
        target[2usize] = entinfo.origin[2usize];
        (*bs).enemy >= 64i32;
        areanum = BotPointAreaNum(target.as_mut_ptr());
        if 0 != areanum && 0 != trap_AAS_AreaReachability(areanum) {
            (*bs).lastenemyorigin[0usize] = target[0usize];
            (*bs).lastenemyorigin[1usize] = target[1usize];
            (*bs).lastenemyorigin[2usize] = target[2usize];
            (*bs).lastenemyareanum = areanum
        }
    }
    if (*bs).enemyvisible_time < floattime - 4i32 as libc::c_float {
        AIEnter_Seek_LTG(
            bs,
            b"battle retreat: lost enemy\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    } else {
        if (*bs).enemyvisible_time < floattime {
            if 0 != BotFindEnemy(bs, -1i32) {
                AIEnter_Battle_Fight(
                    bs,
                    b"battle retreat: another enemy\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return qfalse as libc::c_int;
            }
        }
    }
    BotTeamGoals(bs, qtrue as libc::c_int);
    BotBattleUseItems(bs);
    if 0 == BotLongTermGoal(bs, (*bs).tfl, qtrue as libc::c_int, &mut goal) {
        AIEnter_Battle_SuicidalFight(
            bs,
            b"battle retreat: no way out\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if (*bs).check_time < floattime {
        (*bs).check_time = floattime + 1i32 as libc::c_float;
        range = 150i32 as libc::c_float;
        if gametype == GT_CTF as libc::c_int {
            if 0 != BotCTFCarryingFlag(bs) {
                range = 50i32 as libc::c_float
            }
        }
        if 0 != BotNearbyGoal(bs, (*bs).tfl, &mut goal, range) {
            trap_BotResetLastAvoidReach((*bs).ms);
            (*bs).nbg_time = floattime + range / 100i32 as libc::c_float + 1i32 as libc::c_float;
            AIEnter_Battle_NBG(
                bs,
                b"battle retreat: nbg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    BotSetupForMovement(bs);
    trap_BotMoveToGoal(
        &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    if 0 != moveresult.failure {
        trap_BotResetAvoidReach((*bs).ms);
        (*bs).ltg_time = 0i32 as libc::c_float
    }
    BotAIBlocked(bs, &mut moveresult, qfalse as libc::c_int);
    BotChooseWeapon(bs);
    if 0 != moveresult.flags & (1i32 | 2i32) {
        (*bs).ideal_viewangles[0usize] = moveresult.ideal_viewangles[0usize];
        (*bs).ideal_viewangles[1usize] = moveresult.ideal_viewangles[1usize];
        (*bs).ideal_viewangles[2usize] = moveresult.ideal_viewangles[2usize]
    } else if 0 == moveresult.flags & 8i32 && 0 == (*bs).flags & 32i32 {
        attack_skill = trap_Characteristic_BFloat(
            (*bs).character,
            2i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        );
        if attack_skill as libc::c_double > 0.3f64 {
            BotAimAtEnemy(bs);
        } else {
            if 0 != trap_BotMovementViewTarget(
                (*bs).ms,
                &mut goal as *mut bot_goal_t as *mut libc::c_void,
                (*bs).tfl,
                300i32 as libc::c_float,
                target.as_mut_ptr(),
            ) {
                dir[0usize] = target[0usize] - (*bs).origin[0usize];
                dir[1usize] = target[1usize] - (*bs).origin[1usize];
                dir[2usize] = target[2usize] - (*bs).origin[2usize];
                vectoangles(
                    dir.as_mut_ptr() as *const vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            } else {
                vectoangles(
                    moveresult.movedir.as_mut_ptr() as *const vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            }
            (*bs).ideal_viewangles[2usize] =
                ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
        }
    }
    if 0 != moveresult.flags & 16i32 {
        (*bs).weaponnum = moveresult.weapon
    }
    BotCheckAttack(bs);
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Battle_NBG(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
    BotRecordNodeSwitch(
        bs,
        b"battle NBG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).ainode = Some(AINode_Battle_NBG);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Battle_NBG(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut areanum: libc::c_int = 0;
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
    let mut attack_skill: libc::c_float = 0.;
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    if 0 != BotIsObserver(bs) as u64 {
        AIEnter_Observer(
            bs,
            b"battle nbg: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIntermission(bs) as u64 {
        AIEnter_Intermission(
            bs,
            b"battle nbg: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIsDead(bs) as u64 {
        AIEnter_Respawn(
            bs,
            b"battle nbg: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if (*bs).enemy < 0i32 {
        AIEnter_Seek_NBG(
            bs,
            b"battle nbg: no enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    BotEntityInfo((*bs).enemy, &mut entinfo);
    if 0 != EntityIsDead(&mut entinfo) as u64 {
        AIEnter_Seek_NBG(
            bs,
            b"battle nbg: enemy dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    (*bs).tfl = 0x2i32
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
        | 0x1000000i32;
    if 0 != bot_grapple.integer {
        (*bs).tfl |= 0x4000i32
    }
    if 0 != BotInLavaOrSlime(bs) as u64 {
        (*bs).tfl |= 0x400000i32 | 0x200000i32
    }
    if 0 != BotCanAndWantsToRocketJump(bs) {
        (*bs).tfl |= 0x1000i32
    }
    BotMapScripts(bs);
    if 0.
        != BotEntityVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            360i32 as libc::c_float,
            (*bs).enemy,
        )
    {
        (*bs).enemyvisible_time = floattime;
        target[0usize] = entinfo.origin[0usize];
        target[1usize] = entinfo.origin[1usize];
        target[2usize] = entinfo.origin[2usize];
        (*bs).enemy >= 64i32;
        areanum = BotPointAreaNum(target.as_mut_ptr());
        if 0 != areanum && 0 != trap_AAS_AreaReachability(areanum) {
            (*bs).lastenemyorigin[0usize] = target[0usize];
            (*bs).lastenemyorigin[1usize] = target[1usize];
            (*bs).lastenemyorigin[2usize] = target[2usize];
            (*bs).lastenemyareanum = areanum
        }
    }
    if 0 == trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void) {
        (*bs).nbg_time = 0i32 as libc::c_float
    } else if 0 != BotReachedGoal(bs, &mut goal) {
        (*bs).nbg_time = 0i32 as libc::c_float
    }
    if (*bs).nbg_time < floattime {
        trap_BotPopGoal((*bs).gs);
        if 0 != trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void) {
            AIEnter_Battle_Retreat(
                bs,
                b"battle nbg: time out\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            AIEnter_Battle_Fight(
                bs,
                b"battle nbg: time out\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        return qfalse as libc::c_int;
    }
    BotSetupForMovement(bs);
    trap_BotMoveToGoal(
        &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    if 0 != moveresult.failure {
        trap_BotResetAvoidReach((*bs).ms);
        (*bs).nbg_time = 0i32 as libc::c_float
    }
    BotAIBlocked(bs, &mut moveresult, qfalse as libc::c_int);
    BotUpdateBattleInventory(bs, (*bs).enemy);
    BotChooseWeapon(bs);
    if 0 != moveresult.flags & (1i32 | 2i32) {
        (*bs).ideal_viewangles[0usize] = moveresult.ideal_viewangles[0usize];
        (*bs).ideal_viewangles[1usize] = moveresult.ideal_viewangles[1usize];
        (*bs).ideal_viewangles[2usize] = moveresult.ideal_viewangles[2usize]
    } else if 0 == moveresult.flags & 8i32 && 0 == (*bs).flags & 32i32 {
        attack_skill = trap_Characteristic_BFloat(
            (*bs).character,
            2i32,
            0i32 as libc::c_float,
            1i32 as libc::c_float,
        );
        if attack_skill as libc::c_double > 0.3f64 {
            BotAimAtEnemy(bs);
        } else {
            if 0 != trap_BotMovementViewTarget(
                (*bs).ms,
                &mut goal as *mut bot_goal_t as *mut libc::c_void,
                (*bs).tfl,
                300i32 as libc::c_float,
                target.as_mut_ptr(),
            ) {
                dir[0usize] = target[0usize] - (*bs).origin[0usize];
                dir[1usize] = target[1usize] - (*bs).origin[1usize];
                dir[2usize] = target[2usize] - (*bs).origin[2usize];
                vectoangles(
                    dir.as_mut_ptr() as *const vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            } else {
                vectoangles(
                    moveresult.movedir.as_mut_ptr() as *const vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
            }
            (*bs).ideal_viewangles[2usize] =
                ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
        }
    }
    if 0 != moveresult.flags & 16i32 {
        (*bs).weaponnum = moveresult.weapon
    }
    BotCheckAttack(bs);
    return qtrue as libc::c_int;
}
/*
==================
BotReachedGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotReachedGoal(
    mut bs: *mut bot_state_t,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
    if 0 != (*goal).flags & 1i32 {
        if 0 != trap_BotTouchingGoal((*bs).origin.as_mut_ptr(), goal as *mut libc::c_void) {
            if 0 == (*goal).flags & 4i32 {
                trap_BotSetAvoidGoalTime((*bs).gs, (*goal).number, -1i32 as libc::c_float);
            }
            return qtrue as libc::c_int;
        }
        if 0 != trap_BotItemGoalInVisButNotVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            goal as *mut libc::c_void,
        ) {
            return qtrue as libc::c_int;
        }
        if (*bs).areanum == (*goal).areanum {
            if (*bs).origin[0usize] > (*goal).origin[0usize] + (*goal).mins[0usize]
                && (*bs).origin[0usize] < (*goal).origin[0usize] + (*goal).maxs[0usize]
            {
                if (*bs).origin[1usize] > (*goal).origin[1usize] + (*goal).mins[1usize]
                    && (*bs).origin[1usize] < (*goal).origin[1usize] + (*goal).maxs[1usize]
                {
                    if 0 == trap_AAS_Swimming((*bs).origin.as_mut_ptr()) {
                        return qtrue as libc::c_int;
                    }
                }
            }
        }
    } else if 0 != (*goal).flags & 128i32 {
        if 0 != trap_BotTouchingGoal((*bs).origin.as_mut_ptr(), goal as *mut libc::c_void) {
            return qtrue as libc::c_int;
        }
        if (*bs).lastair_time > floattime - 1i32 as libc::c_float {
            return qtrue as libc::c_int;
        }
    } else if 0 != trap_BotTouchingGoal((*bs).origin.as_mut_ptr(), goal as *mut libc::c_void) {
        return qtrue as libc::c_int;
    }
    return qfalse as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Respawn(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
    BotRecordNodeSwitch(
        bs,
        b"respawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    trap_BotResetMoveState((*bs).ms);
    trap_BotResetGoalState((*bs).gs);
    trap_BotResetAvoidGoals((*bs).gs);
    trap_BotResetAvoidReach((*bs).ms);
    if 0 != BotChat_Death(bs) {
        (*bs).respawn_time = floattime + BotChatTime(bs);
        (*bs).respawnchat_time = floattime
    } else {
        (*bs).respawn_time = floattime
            + 1i32 as libc::c_float
            + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float;
        (*bs).respawnchat_time = 0i32 as libc::c_float
    }
    (*bs).respawn_wait = qfalse as libc::c_int;
    (*bs).ainode = Some(AINode_Respawn);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Respawn(mut bs: *mut bot_state_t) -> libc::c_int {
    if 0 != (*bs).respawn_wait {
        if 0 == BotIsDead(bs) as u64 {
            AIEnter_Seek_LTG(
                bs,
                b"respawn: respawned\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            trap_EA_Respawn((*bs).client);
        }
    } else if (*bs).respawn_time < floattime {
        (*bs).respawn_wait = qtrue as libc::c_int;
        trap_EA_Respawn((*bs).client);
        if 0. != (*bs).respawnchat_time {
            trap_BotEnterChat((*bs).cs, 0i32, (*bs).chatto);
            (*bs).enemy = -1i32
        }
    }
    if 0. != (*bs).respawnchat_time
        && ((*bs).respawnchat_time as libc::c_double) < floattime as libc::c_double - 0.5f64
    {
        trap_EA_Talk((*bs).client);
    }
    return qtrue as libc::c_int;
}
/*
==================
BotRecordNodeSwitch
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotRecordNodeSwitch(
    mut bs: *mut bot_state_t,
    mut node: *mut libc::c_char,
    mut str: *mut libc::c_char,
    mut s: *mut libc::c_char,
) {
    let mut netname: [libc::c_char; 36] = [0; 36];
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    Com_sprintf(
        nodeswitch[numnodeswitches as usize].as_mut_ptr(),
        144i32,
        b"%s at %2.1f entered %s: %s from %s\n\x00" as *const u8 as *const libc::c_char,
        netname.as_mut_ptr(),
        floattime as libc::c_double,
        node,
        str,
        s,
    );
    numnodeswitches += 1;
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
 * name:		ai_dmnet.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_dmnet.c $
 *
 *****************************************************************************/
//
//data file headers
// for the voice chats
//goal flag, see ../botlib/be_ai_goal.h for the other GFL_*
#[no_mangle]
pub static mut numnodeswitches: libc::c_int = 0;
#[no_mangle]
pub static mut nodeswitch: [[libc::c_char; 144]; 51] = [[0; 144]; 51];
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Observer(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
    BotRecordNodeSwitch(
        bs,
        b"observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    BotResetState(bs);
    (*bs).ainode = Some(AINode_Observer);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Observer(mut bs: *mut bot_state_t) -> libc::c_int {
    if 0 == BotIsObserver(bs) as u64 {
        AIEnter_Stand(
            bs,
            b"observer: left observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return qtrue as libc::c_int;
}
/*
==================
BotNearbyGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotNearbyGoal(
    mut bs: *mut bot_state_t,
    mut tfl: libc::c_int,
    mut ltg: *mut bot_goal_t,
    mut range: libc::c_float,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if 0 != BotGoForAir(bs, tfl, ltg, range) {
        return qtrue as libc::c_int;
    }
    if 0 != BotCTFCarryingFlag(bs) {
        if trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            (*bs).teamgoal.areanum,
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
        ) < 300i32
        {
            range = 50i32 as libc::c_float
        }
    }
    ret = trap_BotChooseNBGItem(
        (*bs).gs,
        (*bs).origin.as_mut_ptr(),
        (*bs).inventory.as_mut_ptr(),
        tfl,
        ltg as *mut libc::c_void,
        range,
    );
    return ret;
}
/*
==================
BotGoForAir
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGoForAir(
    mut bs: *mut bot_state_t,
    mut tfl: libc::c_int,
    mut ltg: *mut bot_goal_t,
    mut range: libc::c_float,
) -> libc::c_int {
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
    if (*bs).lastair_time < floattime - 6i32 as libc::c_float {
        if 0 != BotGetAirGoal(bs, &mut goal) {
            trap_BotPushGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void);
            return qtrue as libc::c_int;
        } else {
            while 0
                != trap_BotChooseNBGItem(
                    (*bs).gs,
                    (*bs).origin.as_mut_ptr(),
                    (*bs).inventory.as_mut_ptr(),
                    tfl,
                    ltg as *mut libc::c_void,
                    range,
                )
            {
                trap_BotGetTopGoal((*bs).gs, &mut goal as *mut bot_goal_t as *mut libc::c_void);
                if 0 == trap_AAS_PointContents(goal.origin.as_mut_ptr()) & (32i32 | 16i32 | 8i32) {
                    return qtrue as libc::c_int;
                }
                trap_BotPopGoal((*bs).gs);
            }
            trap_BotResetAvoidGoals((*bs).gs);
        }
    }
    return qfalse as libc::c_int;
}
/*
==================
BotGetAirGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGetAirGoal(
    mut bs: *mut bot_state_t,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
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
    let mut end: vec3_t = [0.; 3];
    let mut mins: vec3_t = [-15i32 as vec_t, -15i32 as vec_t, -2i32 as vec_t];
    let mut maxs: vec3_t = [15i32 as vec_t, 15i32 as vec_t, 2i32 as vec_t];
    let mut areanum: libc::c_int = 0;
    end[0usize] = (*bs).origin[0usize];
    end[1usize] = (*bs).origin[1usize];
    end[2usize] = (*bs).origin[2usize];
    end[2usize] += 1000i32 as libc::c_float;
    BotAI_Trace(
        &mut bsptrace,
        (*bs).origin.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*bs).entitynum,
        1i32 | 0x10000i32,
    );
    end[0usize] = bsptrace.endpos[0usize];
    end[1usize] = bsptrace.endpos[1usize];
    end[2usize] = bsptrace.endpos[2usize];
    BotAI_Trace(
        &mut bsptrace,
        end.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        (*bs).origin.as_mut_ptr(),
        (*bs).entitynum,
        32i32 | 16i32 | 8i32,
    );
    if bsptrace.fraction > 0i32 as libc::c_float {
        areanum = BotPointAreaNum(bsptrace.endpos.as_mut_ptr());
        if 0 != areanum {
            (*goal).origin[0usize] = bsptrace.endpos[0usize];
            (*goal).origin[1usize] = bsptrace.endpos[1usize];
            (*goal).origin[2usize] = bsptrace.endpos[2usize];
            (*goal).origin[2usize] -= 2i32 as libc::c_float;
            (*goal).areanum = areanum;
            (*goal).mins[0usize] = -15i32 as vec_t;
            (*goal).mins[1usize] = -15i32 as vec_t;
            (*goal).mins[2usize] = -1i32 as vec_t;
            (*goal).maxs[0usize] = 15i32 as vec_t;
            (*goal).maxs[1usize] = 15i32 as vec_t;
            (*goal).maxs[2usize] = 1i32 as vec_t;
            (*goal).flags = 128i32;
            (*goal).number = 0i32;
            (*goal).iteminfo = 0i32;
            (*goal).entitynum = 0i32;
            return qtrue as libc::c_int;
        }
    }
    return qfalse as libc::c_int;
}
/*
==================
AIEnter_Battle_SuicidalFight
==================
*/
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Battle_SuicidalFight(
    mut bs: *mut bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"battle fight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    trap_BotResetLastAvoidReach((*bs).ms);
    (*bs).ainode = Some(AINode_Battle_Fight);
    (*bs).flags |= 64i32;
}
/*
==================
BotLongTermGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotLongTermGoal(
    mut bs: *mut bot_state_t,
    mut tfl: libc::c_int,
    mut retreat: libc::c_int,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
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
    let mut teammate: [libc::c_char; 256] = [0; 256];
    let mut squaredist: libc::c_float = 0.;
    let mut areanum: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    if (*bs).lead_time > 0i32 as libc::c_float && 0 == retreat {
        if (*bs).lead_time < floattime {
            BotAI_BotInitialChat(
                bs,
                b"lead_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                EasyClientName(
                    (*bs).lead_teammate,
                    teammate.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).teammate, 2i32);
            (*bs).lead_time = 0i32 as libc::c_float;
            return BotGetLongTermGoal(bs, tfl, retreat, goal);
        }
        if (*bs).leadmessage_time < 0i32 as libc::c_float && -(*bs).leadmessage_time < floattime {
            BotAI_BotInitialChat(
                bs,
                b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                EasyClientName(
                    (*bs).lead_teammate,
                    teammate.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).teammate, 2i32);
            (*bs).leadmessage_time = floattime
        }
        BotEntityInfo((*bs).lead_teammate, &mut entinfo);
        if 0 != entinfo.valid {
            areanum = BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if 0 != areanum && 0 != trap_AAS_AreaReachability(areanum) {
                (*bs).lead_teamgoal.entitynum = (*bs).lead_teammate;
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
        if 0.
            != BotEntityVisible(
                (*bs).entitynum,
                (*bs).eye.as_mut_ptr(),
                (*bs).viewangles.as_mut_ptr(),
                360i32 as libc::c_float,
                (*bs).lead_teammate,
            )
        {
            (*bs).leadvisible_time = floattime
        }
        if (*bs).leadvisible_time < floattime - 1i32 as libc::c_float {
            (*bs).leadbackup_time = floattime + 2i32 as libc::c_float
        }
        dir[0usize] = (*bs).origin[0usize] - (*bs).lead_teamgoal.origin[0usize];
        dir[1usize] = (*bs).origin[1usize] - (*bs).lead_teamgoal.origin[1usize];
        dir[2usize] = (*bs).origin[2usize] - (*bs).lead_teamgoal.origin[2usize];
        squaredist = VectorLengthSquared(dir.as_mut_ptr() as *const vec_t);
        if (*bs).leadbackup_time > floattime {
            if (*bs).leadmessage_time < floattime - 20i32 as libc::c_float {
                BotAI_BotInitialChat(
                    bs,
                    b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    EasyClientName(
                        (*bs).lead_teammate,
                        teammate.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                            as libc::c_int,
                    ),
                    0 as *mut libc::c_void,
                );
                trap_BotEnterChat((*bs).cs, (*bs).teammate, 2i32);
                (*bs).leadmessage_time = floattime
            }
            if squaredist < (100i32 * 100i32) as libc::c_float {
                (*bs).leadbackup_time = 0i32 as libc::c_float
            }
            memcpy(
                goal as *mut libc::c_void,
                &mut (*bs).lead_teamgoal as *mut bot_goal_t as *const libc::c_void,
                ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
            );
            return qtrue as libc::c_int;
        } else {
            if squaredist > (500i32 * 500i32) as libc::c_float {
                if (*bs).leadmessage_time < floattime - 20i32 as libc::c_float {
                    BotAI_BotInitialChat(
                        bs,
                        b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        EasyClientName(
                            (*bs).lead_teammate,
                            teammate.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                                as libc::c_int,
                        ),
                        0 as *mut libc::c_void,
                    );
                    trap_BotEnterChat((*bs).cs, (*bs).teammate, 2i32);
                    (*bs).leadmessage_time = floattime
                }
                dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
                dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
                dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
                vectoangles(
                    dir.as_mut_ptr() as *const vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
                (*bs).ideal_viewangles[2usize] =
                    ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t;
                return qfalse as libc::c_int;
            }
        }
    }
    return BotGetLongTermGoal(bs, tfl, retreat, goal);
}
/*
==================
BotGetLongTermGoal

we could also create a separate AI node for every long term goal type
however this saves us a lot of code
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGetLongTermGoal(
    mut bs: *mut bot_state_t,
    mut tfl: libc::c_int,
    mut retreat: libc::c_int,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut dir2: vec3_t = [0.; 3];
    let mut netname: [libc::c_char; 36] = [0; 36];
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut areanum: libc::c_int = 0;
    let mut croucher: libc::c_float = 0.;
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
    let mut botinfo: aas_entityinfo_t = aas_entityinfo_s {
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
    let mut wp: *mut bot_waypoint_t = 0 as *mut bot_waypoint_t;
    if (*bs).ltgtype == 1i32 && 0 == retreat {
        if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
            BotAI_BotInitialChat(
                bs,
                b"help_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            BotVoiceChatOnly(
                bs,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            trap_EA_Action((*bs).client, 0x100000i32);
            (*bs).teammessage_time = 0i32 as libc::c_float
        }
        if (*bs).teamgoal_time < floattime {
            (*bs).ltgtype = 0i32
        }
        if (*bs).teammatevisible_time < floattime - 10i32 as libc::c_float {
            (*bs).ltgtype = 0i32
        }
        BotEntityInfo((*bs).teammate, &mut entinfo);
        if 0.
            != BotEntityVisible(
                (*bs).entitynum,
                (*bs).eye.as_mut_ptr(),
                (*bs).viewangles.as_mut_ptr(),
                360i32 as libc::c_float,
                (*bs).teammate,
            )
        {
            dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
            dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
            dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
            if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                < (100i32 * 100i32) as libc::c_float
            {
                trap_BotResetAvoidReach((*bs).ms);
                return qfalse as libc::c_int;
            }
        } else {
            (*bs).teammatevisible_time = floattime
        }
        if 0 != entinfo.valid {
            areanum = BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if 0 != areanum && 0 != trap_AAS_AreaReachability(areanum) {
                (*bs).teamgoal.entitynum = (*bs).teammate;
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
        memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        return qtrue as libc::c_int;
    }
    if (*bs).ltgtype == 2i32 && 0 == retreat {
        if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
            BotAI_BotInitialChat(
                bs,
                b"accompany_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            BotVoiceChatOnly(
                bs,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            trap_EA_Action((*bs).client, 0x100000i32);
            (*bs).teammessage_time = 0i32 as libc::c_float
        }
        if (*bs).teamgoal_time < floattime {
            BotAI_BotInitialChat(
                bs,
                b"accompany_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).teammate, 2i32);
            (*bs).ltgtype = 0i32
        }
        BotEntityInfo((*bs).teammate, &mut entinfo);
        if 0.
            != BotEntityVisible(
                (*bs).entitynum,
                (*bs).eye.as_mut_ptr(),
                (*bs).viewangles.as_mut_ptr(),
                360i32 as libc::c_float,
                (*bs).teammate,
            )
        {
            (*bs).teammatevisible_time = floattime;
            dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
            dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
            dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
            if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t)
                < (*bs).formation_dist * (*bs).formation_dist
            {
                BotEntityInfo((*bs).entitynum, &mut botinfo);
                if botinfo.origin[2usize] + botinfo.maxs[2usize]
                    > entinfo.origin[2usize] + entinfo.mins[2usize]
                {
                    if botinfo.origin[0usize] + botinfo.maxs[0usize]
                        > entinfo.origin[0usize] + entinfo.mins[0usize] - 4i32 as libc::c_float
                        && botinfo.origin[0usize] + botinfo.mins[0usize]
                            < entinfo.origin[0usize] + entinfo.maxs[0usize] + 4i32 as libc::c_float
                    {
                        if botinfo.origin[1usize] + botinfo.maxs[1usize]
                            > entinfo.origin[1usize] + entinfo.mins[1usize] - 4i32 as libc::c_float
                            && botinfo.origin[1usize] + botinfo.mins[1usize]
                                < entinfo.origin[1usize]
                                    + entinfo.maxs[1usize]
                                    + 4i32 as libc::c_float
                        {
                            if botinfo.origin[2usize] + botinfo.maxs[2usize]
                                > entinfo.origin[2usize] + entinfo.mins[2usize]
                                    - 4i32 as libc::c_float
                                && botinfo.origin[2usize] + botinfo.mins[2usize]
                                    < entinfo.origin[2usize]
                                        + entinfo.maxs[2usize]
                                        + 4i32 as libc::c_float
                            {
                                AngleVectors(
                                    entinfo.angles.as_mut_ptr() as *const vec_t,
                                    dir.as_mut_ptr(),
                                    0 as *mut vec_t,
                                    0 as *mut vec_t,
                                );
                                dir[2usize] = 0i32 as vec_t;
                                VectorNormalize(dir.as_mut_ptr());
                                dir2[0usize] = (*bs).origin[0usize] - entinfo.origin[0usize];
                                dir2[1usize] = (*bs).origin[1usize] - entinfo.origin[1usize];
                                dir2[2usize] = (*bs).origin[2usize] - entinfo.origin[2usize];
                                VectorNormalize(dir2.as_mut_ptr());
                                if (dir[0usize] * dir2[0usize]
                                    + dir[1usize] * dir2[1usize]
                                    + dir[2usize] * dir2[2usize])
                                    as libc::c_double
                                    > 0.7f64
                                {
                                    BotSetupForMovement(bs);
                                    trap_BotMoveInDirection(
                                        (*bs).ms,
                                        dir2.as_mut_ptr(),
                                        400i32 as libc::c_float,
                                        1i32,
                                    );
                                }
                            }
                        }
                    }
                }
                if (*bs).attackcrouch_time < floattime - 5i32 as libc::c_float {
                    croucher = trap_Characteristic_BFloat(
                        (*bs).character,
                        36i32,
                        0i32 as libc::c_float,
                        1i32 as libc::c_float,
                    );
                    if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        < (*bs).thinktime * croucher
                    {
                        (*bs).attackcrouch_time =
                            floattime + 5i32 as libc::c_float + croucher * 15i32 as libc::c_float
                    }
                }
                if 0 != trap_AAS_Swimming((*bs).origin.as_mut_ptr()) {
                    (*bs).attackcrouch_time = floattime - 1i32 as libc::c_float
                }
                if (*bs).arrive_time < floattime - 2i32 as libc::c_float {
                    if 0. == (*bs).arrive_time {
                        trap_EA_Gesture((*bs).client);
                        BotAI_BotInitialChat(
                            bs,
                            b"accompany_arrive\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            EasyClientName(
                                (*bs).teammate,
                                netname.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                    as libc::c_int,
                            ),
                            0 as *mut libc::c_void,
                        );
                        trap_BotEnterChat((*bs).cs, (*bs).teammate, 2i32);
                        (*bs).arrive_time = floattime
                    } else if (*bs).attackcrouch_time > floattime {
                        trap_EA_Crouch((*bs).client);
                    } else if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double)
                        < (*bs).thinktime as libc::c_double * 0.05f64
                    {
                        trap_EA_Gesture((*bs).client);
                    }
                }
                if (*bs).arrive_time > floattime - 2i32 as libc::c_float {
                    dir[0usize] = entinfo.origin[0usize] - (*bs).origin[0usize];
                    dir[1usize] = entinfo.origin[1usize] - (*bs).origin[1usize];
                    dir[2usize] = entinfo.origin[2usize] - (*bs).origin[2usize];
                    vectoangles(
                        dir.as_mut_ptr() as *const vec_t,
                        (*bs).ideal_viewangles.as_mut_ptr(),
                    );
                    (*bs).ideal_viewangles[2usize] =
                        ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
                } else if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double)
                    < (*bs).thinktime as libc::c_double * 0.8f64
                {
                    BotRoamGoal(bs, target.as_mut_ptr());
                    dir[0usize] = target[0usize] - (*bs).origin[0usize];
                    dir[1usize] = target[1usize] - (*bs).origin[1usize];
                    dir[2usize] = target[2usize] - (*bs).origin[2usize];
                    vectoangles(
                        dir.as_mut_ptr() as *const vec_t,
                        (*bs).ideal_viewangles.as_mut_ptr(),
                    );
                    (*bs).ideal_viewangles[2usize] =
                        ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
                }
                if 0 != BotGoForAir(bs, (*bs).tfl, &mut (*bs).teamgoal, 400i32 as libc::c_float) {
                    trap_BotResetLastAvoidReach((*bs).ms);
                    (*bs).nbg_time = floattime + 8i32 as libc::c_float;
                    AIEnter_Seek_NBG(
                        bs,
                        b"BotLongTermGoal: go for air\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return qfalse as libc::c_int;
                }
                trap_BotResetAvoidReach((*bs).ms);
                return qfalse as libc::c_int;
            }
        }
        if 0 != entinfo.valid {
            areanum = BotPointAreaNum(entinfo.origin.as_mut_ptr());
            if 0 != areanum && 0 != trap_AAS_AreaReachability(areanum) {
                (*bs).teamgoal.entitynum = (*bs).teammate;
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
        memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        if (*bs).teammatevisible_time < floattime - 60i32 as libc::c_float {
            BotAI_BotInitialChat(
                bs,
                b"accompany_cannotfind\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                EasyClientName(
                    (*bs).teammate,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                ),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).teammate, 2i32);
            (*bs).ltgtype = 0i32;
            (*bs).teammatevisible_time = floattime
        }
        return qtrue as libc::c_int;
    }
    if (*bs).ltgtype == 3i32 {
        if trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            (*bs).teamgoal.areanum,
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
        ) as libc::c_float
            > (*bs).defendaway_range
        {
            (*bs).defendaway_time = 0i32 as libc::c_float
        }
    }
    if (*bs).ltgtype == 3i32 && 0 == retreat && (*bs).defendaway_time < floattime {
        if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"defend_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, 0i32, 1i32);
            BotVoiceChatOnly(
                bs,
                -1i32,
                b"ondefense\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (*bs).teammessage_time = 0i32 as libc::c_float
        }
        memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        if (*bs).teamgoal_time < floattime {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"defend_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, 0i32, 1i32);
            (*bs).ltgtype = 0i32
        }
        dir[0usize] = (*goal).origin[0usize] - (*bs).origin[0usize];
        dir[1usize] = (*goal).origin[1usize] - (*bs).origin[1usize];
        dir[2usize] = (*goal).origin[2usize] - (*bs).origin[2usize];
        if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t) < (70i32 * 70i32) as libc::c_float
        {
            trap_BotResetAvoidReach((*bs).ms);
            (*bs).defendaway_time = floattime
                + 3i32 as libc::c_float
                + 3i32 as libc::c_float
                    * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float);
            if 0 != BotHasPersistantPowerupAndWeapon(bs) {
                (*bs).defendaway_range = 100i32 as libc::c_float
            } else {
                (*bs).defendaway_range = 350i32 as libc::c_float
            }
        }
        return qtrue as libc::c_int;
    }
    if (*bs).ltgtype == 11i32 && 0 == retreat {
        if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
            EasyClientName(
                (*bs).teamgoal.entitynum,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"kill_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            (*bs).teammessage_time = 0i32 as libc::c_float
        }
        if (*bs).killedenemy_time > (*bs).teamgoal_time - 180i32 as libc::c_float
            && (*bs).lastkilledplayer == (*bs).teamgoal.entitynum
        {
            EasyClientName(
                (*bs).teamgoal.entitynum,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"kill_done\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            (*bs).ltgtype = 0i32
        }
        if (*bs).teamgoal_time < floattime {
            (*bs).ltgtype = 0i32
        }
        return BotGetItemLongTermGoal(bs, tfl, goal);
    }
    if (*bs).ltgtype == 10i32 && 0 == retreat {
        if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"getitem_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            BotVoiceChatOnly(
                bs,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            trap_EA_Action((*bs).client, 0x100000i32);
            (*bs).teammessage_time = 0i32 as libc::c_float
        }
        memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        if (*bs).teamgoal_time < floattime {
            (*bs).ltgtype = 0i32
        }
        if 0 != trap_BotItemGoalInVisButNotVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            goal as *mut libc::c_void,
        ) {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"getitem_notthere\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            (*bs).ltgtype = 0i32
        } else if 0 != BotReachedGoal(bs, goal) {
            trap_BotGoalName(
                (*bs).teamgoal.number,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"getitem_gotit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            (*bs).ltgtype = 0i32
        }
        return qtrue as libc::c_int;
    }
    if ((*bs).ltgtype == 7i32 || (*bs).ltgtype == 8i32) && 0 == retreat {
        if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
            if (*bs).ltgtype == 8i32 {
                BotAI_BotInitialChat(
                    bs,
                    b"camp_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    EasyClientName(
                        (*bs).teammate,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    ),
                    0 as *mut libc::c_void,
                );
                trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
                BotVoiceChatOnly(
                    bs,
                    (*bs).decisionmaker,
                    b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                trap_EA_Action((*bs).client, 0x100000i32);
            }
            (*bs).teammessage_time = 0i32 as libc::c_float
        }
        memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).teamgoal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        if (*bs).teamgoal_time < floattime {
            if (*bs).ltgtype == 8i32 {
                BotAI_BotInitialChat(
                    bs,
                    b"camp_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            }
            (*bs).ltgtype = 0i32
        }
        dir[0usize] = (*goal).origin[0usize] - (*bs).origin[0usize];
        dir[1usize] = (*goal).origin[1usize] - (*bs).origin[1usize];
        dir[2usize] = (*goal).origin[2usize] - (*bs).origin[2usize];
        if VectorLengthSquared(dir.as_mut_ptr() as *const vec_t) < (60i32 * 60i32) as libc::c_float
        {
            if 0. == (*bs).arrive_time {
                if (*bs).ltgtype == 8i32 {
                    BotAI_BotInitialChat(
                        bs,
                        b"camp_arrive\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        EasyClientName(
                            (*bs).teammate,
                            netname.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        ),
                        0 as *mut libc::c_void,
                    );
                    trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
                    BotVoiceChatOnly(
                        bs,
                        (*bs).decisionmaker,
                        b"inposition\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*bs).arrive_time = floattime
            }
            if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double)
                < (*bs).thinktime as libc::c_double * 0.8f64
            {
                BotRoamGoal(bs, target.as_mut_ptr());
                dir[0usize] = target[0usize] - (*bs).origin[0usize];
                dir[1usize] = target[1usize] - (*bs).origin[1usize];
                dir[2usize] = target[2usize] - (*bs).origin[2usize];
                vectoangles(
                    dir.as_mut_ptr() as *const vec_t,
                    (*bs).ideal_viewangles.as_mut_ptr(),
                );
                (*bs).ideal_viewangles[2usize] =
                    ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
            }
            if (*bs).attackcrouch_time < floattime - 5i32 as libc::c_float {
                croucher = trap_Characteristic_BFloat(
                    (*bs).character,
                    36i32,
                    0i32 as libc::c_float,
                    1i32 as libc::c_float,
                );
                if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    < (*bs).thinktime * croucher
                {
                    (*bs).attackcrouch_time =
                        floattime + 5i32 as libc::c_float + croucher * 15i32 as libc::c_float
                }
            }
            if (*bs).attackcrouch_time > floattime {
                trap_EA_Crouch((*bs).client);
            }
            if 0 != trap_AAS_Swimming((*bs).origin.as_mut_ptr()) {
                (*bs).attackcrouch_time = floattime - 1i32 as libc::c_float
            }
            if 0 != trap_PointContents((*bs).eye.as_mut_ptr() as *const vec_t, (*bs).entitynum)
                & (32i32 | 16i32 | 8i32)
            {
                if (*bs).ltgtype == 8i32 {
                    BotAI_BotInitialChat(
                        bs,
                        b"camp_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        0 as *mut libc::c_void,
                    );
                    trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
                    if (*bs).lastgoal_ltgtype == 8i32 {
                        (*bs).lastgoal_ltgtype = 0i32
                    }
                }
                (*bs).ltgtype = 0i32
            }
            trap_BotResetAvoidReach((*bs).ms);
            return qfalse as libc::c_int;
        }
        return qtrue as libc::c_int;
    }
    if (*bs).ltgtype == 9i32 && 0 == retreat {
        if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
            strcpy(
                buf.as_mut_ptr(),
                b"\x00" as *const u8 as *const libc::c_char,
            );
            wp = (*bs).patrolpoints;
            while !wp.is_null() {
                strcat(buf.as_mut_ptr(), (*wp).name.as_mut_ptr());
                if !(*wp).next.is_null() {
                    strcat(
                        buf.as_mut_ptr(),
                        b" to \x00" as *const u8 as *const libc::c_char,
                    );
                }
                wp = (*wp).next
            }
            BotAI_BotInitialChat(
                bs,
                b"patrol_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                buf.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            BotVoiceChatOnly(
                bs,
                (*bs).decisionmaker,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            trap_EA_Action((*bs).client, 0x100000i32);
            (*bs).teammessage_time = 0i32 as libc::c_float
        }
        if (*bs).curpatrolpoint.is_null() {
            (*bs).ltgtype = 0i32;
            return qfalse as libc::c_int;
        }
        if 0 != trap_BotTouchingGoal(
            (*bs).origin.as_mut_ptr(),
            &mut (*(*bs).curpatrolpoint).goal as *mut bot_goal_t as *mut libc::c_void,
        ) {
            if 0 != (*bs).patrolflags & 4i32 {
                if !(*(*bs).curpatrolpoint).prev.is_null() {
                    (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).prev
                } else {
                    (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).next;
                    (*bs).patrolflags &= !4i32
                }
            } else if !(*(*bs).curpatrolpoint).next.is_null() {
                (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).next
            } else {
                (*bs).curpatrolpoint = (*(*bs).curpatrolpoint).prev;
                (*bs).patrolflags |= 4i32
            }
        }
        if (*bs).teamgoal_time < floattime {
            BotAI_BotInitialChat(
                bs,
                b"patrol_stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                0 as *mut libc::c_void,
            );
            trap_BotEnterChat((*bs).cs, (*bs).decisionmaker, 2i32);
            (*bs).ltgtype = 0i32
        }
        if (*bs).curpatrolpoint.is_null() {
            (*bs).ltgtype = 0i32;
            return qfalse as libc::c_int;
        }
        memcpy(
            goal as *mut libc::c_void,
            &mut (*(*bs).curpatrolpoint).goal as *mut bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
        );
        return qtrue as libc::c_int;
    }
    if gametype == GT_CTF as libc::c_int {
        if (*bs).ltgtype == 4i32 {
            if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
                BotAI_BotInitialChat(
                    bs,
                    b"captureflag_start\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                trap_BotEnterChat((*bs).cs, 0i32, 1i32);
                BotVoiceChatOnly(
                    bs,
                    -1i32,
                    b"ongetflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                (*bs).teammessage_time = 0i32 as libc::c_float
            }
            match BotTeam(bs) {
                1 => {
                    memcpy(
                        goal as *mut libc::c_void,
                        &mut ctf_blueflag as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                    );
                }
                2 => {
                    memcpy(
                        goal as *mut libc::c_void,
                        &mut ctf_redflag as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                    );
                }
                _ => {
                    (*bs).ltgtype = 0i32;
                    return qfalse as libc::c_int;
                }
            }
            if 0 != trap_BotTouchingGoal((*bs).origin.as_mut_ptr(), goal as *mut libc::c_void) {
                match BotTeam(bs) {
                    1 => (*bs).blueflagstatus = 1i32,
                    2 => (*bs).redflagstatus = 1i32,
                    _ => {}
                }
                (*bs).ltgtype = 0i32
            }
            if (*bs).teamgoal_time < floattime {
                (*bs).ltgtype = 0i32
            }
            BotAlternateRoute(bs, goal);
            return qtrue as libc::c_int;
        }
        if (*bs).ltgtype == 5i32 && (*bs).rushbaseaway_time < floattime {
            match BotTeam(bs) {
                1 => {
                    memcpy(
                        goal as *mut libc::c_void,
                        &mut ctf_redflag as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                    );
                }
                2 => {
                    memcpy(
                        goal as *mut libc::c_void,
                        &mut ctf_blueflag as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                    );
                }
                _ => {
                    (*bs).ltgtype = 0i32;
                    return qfalse as libc::c_int;
                }
            }
            if 0 == BotCTFCarryingFlag(bs) {
                (*bs).ltgtype = 0i32
            }
            if (*bs).teamgoal_time < floattime {
                (*bs).ltgtype = 0i32
            }
            if 0 != trap_BotTouchingGoal((*bs).origin.as_mut_ptr(), goal as *mut libc::c_void) {
                if 0 != BotCTFCarryingFlag(bs) {
                    trap_BotResetAvoidReach((*bs).ms);
                    (*bs).rushbaseaway_time = floattime
                        + 5i32 as libc::c_float
                        + 10i32 as libc::c_float
                            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                } else {
                    (*bs).ltgtype = 0i32
                }
            }
            BotAlternateRoute(bs, goal);
            return qtrue as libc::c_int;
        }
        if (*bs).ltgtype == 6i32 {
            if 0. != (*bs).teammessage_time && (*bs).teammessage_time < floattime {
                BotAI_BotInitialChat(
                    bs,
                    b"returnflag_start\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                trap_BotEnterChat((*bs).cs, 0i32, 1i32);
                BotVoiceChatOnly(
                    bs,
                    -1i32,
                    b"onreturnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                (*bs).teammessage_time = 0i32 as libc::c_float
            }
            match BotTeam(bs) {
                1 => {
                    memcpy(
                        goal as *mut libc::c_void,
                        &mut ctf_blueflag as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                    );
                }
                2 => {
                    memcpy(
                        goal as *mut libc::c_void,
                        &mut ctf_redflag as *mut bot_goal_t as *const libc::c_void,
                        ::std::mem::size_of::<bot_goal_t>() as libc::c_ulong,
                    );
                }
                _ => {
                    (*bs).ltgtype = 0i32;
                    return qfalse as libc::c_int;
                }
            }
            if 0 != trap_BotTouchingGoal((*bs).origin.as_mut_ptr(), goal as *mut libc::c_void) {
                (*bs).ltgtype = 0i32
            }
            if (*bs).teamgoal_time < floattime {
                (*bs).ltgtype = 0i32
            }
            BotAlternateRoute(bs, goal);
            return qtrue as libc::c_int;
        }
    }
    return BotGetItemLongTermGoal(bs, tfl, goal);
}
/*
==================
BotGetItemLongTermGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotGetItemLongTermGoal(
    mut bs: *mut bot_state_t,
    mut tfl: libc::c_int,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
    if 0 == trap_BotGetTopGoal((*bs).gs, goal as *mut libc::c_void) {
        (*bs).ltg_time = 0i32 as libc::c_float
    } else if 0 != BotReachedGoal(bs, goal) {
        BotChooseWeapon(bs);
        (*bs).ltg_time = 0i32 as libc::c_float
    }
    if (*bs).ltg_time < floattime {
        trap_BotPopGoal((*bs).gs);
        if 0 != trap_BotChooseLTGItem(
            (*bs).gs,
            (*bs).origin.as_mut_ptr(),
            (*bs).inventory.as_mut_ptr(),
            tfl,
        ) {
            (*bs).ltg_time = floattime + 20i32 as libc::c_float
        } else {
            trap_BotResetAvoidGoals((*bs).gs);
            trap_BotResetAvoidReach((*bs).ms);
        }
        return trap_BotGetTopGoal((*bs).gs, goal as *mut libc::c_void);
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Battle_Chase(mut bs: *mut bot_state_t, mut s: *mut libc::c_char) {
    BotRecordNodeSwitch(
        bs,
        b"battle chase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).chase_time = floattime;
    (*bs).ainode = Some(AINode_Battle_Chase);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Battle_Chase(mut bs: *mut bot_state_t) -> libc::c_int {
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
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
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
    let mut range: libc::c_float = 0.;
    if 0 != BotIsObserver(bs) as u64 {
        AIEnter_Observer(
            bs,
            b"battle chase: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIntermission(bs) as u64 {
        AIEnter_Intermission(
            bs,
            b"battle chase: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIsDead(bs) as u64 {
        AIEnter_Respawn(
            bs,
            b"battle chase: bot dead\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if (*bs).enemy < 0i32 {
        AIEnter_Seek_LTG(
            bs,
            b"battle chase: no enemy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0.
        != BotEntityVisible(
            (*bs).entitynum,
            (*bs).eye.as_mut_ptr(),
            (*bs).viewangles.as_mut_ptr(),
            360i32 as libc::c_float,
            (*bs).enemy,
        )
    {
        AIEnter_Battle_Fight(
            bs,
            b"battle chase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotFindEnemy(bs, -1i32) {
        AIEnter_Battle_Fight(
            bs,
            b"battle chase: better enemy\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 == (*bs).lastenemyareanum {
        AIEnter_Seek_LTG(
            bs,
            b"battle chase: no enemy area\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    (*bs).tfl = 0x2i32
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
        | 0x1000000i32;
    if 0 != bot_grapple.integer {
        (*bs).tfl |= 0x4000i32
    }
    if 0 != BotInLavaOrSlime(bs) as u64 {
        (*bs).tfl |= 0x400000i32 | 0x200000i32
    }
    if 0 != BotCanAndWantsToRocketJump(bs) {
        (*bs).tfl |= 0x1000i32
    }
    BotMapScripts(bs);
    goal.entitynum = (*bs).enemy;
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
    if 0 != trap_BotTouchingGoal(
        (*bs).origin.as_mut_ptr(),
        &mut goal as *mut bot_goal_t as *mut libc::c_void,
    ) {
        (*bs).chase_time = 0i32 as libc::c_float
    }
    if 0. == (*bs).chase_time || (*bs).chase_time < floattime - 10i32 as libc::c_float {
        AIEnter_Seek_LTG(
            bs,
            b"battle chase: time out\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if (*bs).check_time < floattime {
        (*bs).check_time = floattime + 1i32 as libc::c_float;
        range = 150i32 as libc::c_float;
        if 0 != BotNearbyGoal(bs, (*bs).tfl, &mut goal, range) {
            (*bs).nbg_time = (floattime as libc::c_double
                + 0.1f64 * range as libc::c_double
                + 1i32 as libc::c_double) as libc::c_float;
            trap_BotResetLastAvoidReach((*bs).ms);
            AIEnter_Battle_NBG(
                bs,
                b"battle chase: nbg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
    }
    BotUpdateBattleInventory(bs, (*bs).enemy);
    BotSetupForMovement(bs);
    trap_BotMoveToGoal(
        &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
        (*bs).ms,
        &mut goal as *mut bot_goal_t as *mut libc::c_void,
        (*bs).tfl,
    );
    if 0 != moveresult.failure {
        trap_BotResetAvoidReach((*bs).ms);
        (*bs).ltg_time = 0i32 as libc::c_float
    }
    BotAIBlocked(bs, &mut moveresult, qfalse as libc::c_int);
    if 0 != moveresult.flags & (8i32 | 1i32 | 2i32) {
        (*bs).ideal_viewangles[0usize] = moveresult.ideal_viewangles[0usize];
        (*bs).ideal_viewangles[1usize] = moveresult.ideal_viewangles[1usize];
        (*bs).ideal_viewangles[2usize] = moveresult.ideal_viewangles[2usize]
    } else if 0 == (*bs).flags & 32i32 {
        if (*bs).chase_time > floattime - 2i32 as libc::c_float {
            BotAimAtEnemy(bs);
        } else if 0
            != trap_BotMovementViewTarget(
                (*bs).ms,
                &mut goal as *mut bot_goal_t as *mut libc::c_void,
                (*bs).tfl,
                300i32 as libc::c_float,
                target.as_mut_ptr(),
            )
        {
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else {
            vectoangles(
                moveresult.movedir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        }
        (*bs).ideal_viewangles[2usize] =
            ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
    }
    if 0 != moveresult.flags & 16i32 {
        (*bs).weaponnum = moveresult.weapon
    }
    if (*bs).areanum == (*bs).lastenemyareanum {
        (*bs).chase_time = 0i32 as libc::c_float
    }
    if 0 != BotWantsToRetreat(bs) {
        AIEnter_Battle_Retreat(
            bs,
            b"battle chase: wants to retreat\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qtrue as libc::c_int;
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AIEnter_Seek_ActivateEntity(
    mut bs: *mut bot_state_t,
    mut s: *mut libc::c_char,
) {
    BotRecordNodeSwitch(
        bs,
        b"activate entity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    (*bs).ainode = Some(AINode_Seek_ActivateEntity);
}
#[no_mangle]
pub unsafe extern "C" fn AINode_Seek_ActivateEntity(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut goal: *mut bot_goal_t = 0 as *mut bot_goal_t;
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut ideal_viewangles: vec3_t = [0.; 3];
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
    let mut targetvisible: libc::c_int = 0;
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
    if 0 != BotIsObserver(bs) as u64 {
        BotClearActivateGoalStack(bs);
        AIEnter_Observer(
            bs,
            b"active entity: observer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIntermission(bs) as u64 {
        BotClearActivateGoalStack(bs);
        AIEnter_Intermission(
            bs,
            b"activate entity: intermission\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    if 0 != BotIsDead(bs) as u64 {
        BotClearActivateGoalStack(bs);
        AIEnter_Respawn(
            bs,
            b"activate entity: bot dead\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    (*bs).tfl = 0x2i32
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
        | 0x1000000i32;
    if 0 != bot_grapple.integer {
        (*bs).tfl |= 0x4000i32
    }
    if 0 != BotInLavaOrSlime(bs) as u64 {
        (*bs).tfl |= 0x400000i32 | 0x200000i32
    }
    BotMapScripts(bs);
    (*bs).enemy = -1i32;
    if (*bs).activatestack.is_null() {
        BotClearActivateGoalStack(bs);
        AIEnter_Seek_NBG(
            bs,
            b"activate entity: no goal\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return qfalse as libc::c_int;
    }
    goal = &mut (*(*bs).activatestack).goal;
    targetvisible = qfalse as libc::c_int;
    if 0 != (*(*bs).activatestack).shoot {
        BotAI_Trace(
            &mut bsptrace,
            (*bs).eye.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
            (*(*bs).activatestack).target.as_mut_ptr(),
            (*bs).entitynum,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        if bsptrace.fraction as libc::c_double >= 1.0f64 || bsptrace.ent == (*goal).entitynum {
            targetvisible = qtrue as libc::c_int;
            if (*bs).cur_ps.weapon == (*(*bs).activatestack).weapon {
                dir[0usize] = (*(*bs).activatestack).target[0usize] - (*bs).eye[0usize];
                dir[1usize] = (*(*bs).activatestack).target[1usize] - (*bs).eye[1usize];
                dir[2usize] = (*(*bs).activatestack).target[2usize] - (*bs).eye[2usize];
                vectoangles(
                    dir.as_mut_ptr() as *const vec_t,
                    ideal_viewangles.as_mut_ptr(),
                );
                if 0 != InFieldOfVision(
                    (*bs).viewangles.as_mut_ptr(),
                    20i32 as libc::c_float,
                    ideal_viewangles.as_mut_ptr(),
                ) as u64
                {
                    trap_EA_Attack((*bs).client);
                }
            }
        }
    }
    if 0 != targetvisible {
        BotEntityInfo((*goal).entitynum, &mut entinfo);
        if 0 == VectorCompare(
            (*(*bs).activatestack).origin.as_mut_ptr() as *const vec_t,
            entinfo.origin.as_mut_ptr() as *const vec_t,
        ) {
            (*(*bs).activatestack).time = 0i32 as libc::c_float
        }
        if (*(*bs).activatestack).time < floattime {
            BotPopFromActivateGoalStack(bs);
            if !(*bs).activatestack.is_null() {
                (*(*bs).activatestack).time = floattime + 10i32 as libc::c_float;
                return qfalse as libc::c_int;
            }
            AIEnter_Seek_NBG(
                bs,
                b"activate entity: time out\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
        memset(
            &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<bot_moveresult_t>() as libc::c_ulong,
        );
    } else {
        if goal.is_null() {
            (*(*bs).activatestack).time = 0i32 as libc::c_float
        } else if 0 == (*(*bs).activatestack).shoot {
            if 0 != trap_BotTouchingGoal((*bs).origin.as_mut_ptr(), goal as *mut libc::c_void) {
                (*(*bs).activatestack).time = 0i32 as libc::c_float
            }
        }
        if (*(*bs).activatestack).time < floattime {
            BotPopFromActivateGoalStack(bs);
            if !(*bs).activatestack.is_null() {
                (*(*bs).activatestack).time = floattime + 10i32 as libc::c_float;
                return qfalse as libc::c_int;
            }
            AIEnter_Seek_NBG(
                bs,
                b"activate entity: activated\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return qfalse as libc::c_int;
        }
        if 0 != BotAIPredictObstacles(bs, goal) {
            return qfalse as libc::c_int;
        }
        BotSetupForMovement(bs);
        trap_BotMoveToGoal(
            &mut moveresult as *mut bot_moveresult_t as *mut libc::c_void,
            (*bs).ms,
            goal as *mut libc::c_void,
            (*bs).tfl,
        );
        if 0 != moveresult.failure {
            trap_BotResetAvoidReach((*bs).ms);
            (*(*bs).activatestack).time = 0i32 as libc::c_float
        }
        BotAIBlocked(bs, &mut moveresult, qtrue as libc::c_int);
    }
    BotClearPath(bs, &mut moveresult);
    if 0 != (*(*bs).activatestack).shoot {
        if 0 == moveresult.flags & 1i32 {
            dir[0usize] = (*(*bs).activatestack).target[0usize] - (*bs).eye[0usize];
            dir[1usize] = (*(*bs).activatestack).target[1usize] - (*bs).eye[1usize];
            dir[2usize] = (*(*bs).activatestack).target[2usize] - (*bs).eye[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                moveresult.ideal_viewangles.as_mut_ptr(),
            );
            moveresult.flags |= 1i32
        }
        if 0 == moveresult.flags & 16i32 {
            moveresult.flags |= 16i32;
            (*(*bs).activatestack).weapon = BotSelectActivateWeapon(bs);
            if (*(*bs).activatestack).weapon == -1i32 {
                (*(*bs).activatestack).weapon = 0i32
            }
            moveresult.weapon = (*(*bs).activatestack).weapon
        }
    }
    if 0 != moveresult.flags & (8i32 | 1i32 | 2i32) {
        (*bs).ideal_viewangles[0usize] = moveresult.ideal_viewangles[0usize];
        (*bs).ideal_viewangles[1usize] = moveresult.ideal_viewangles[1usize];
        (*bs).ideal_viewangles[2usize] = moveresult.ideal_viewangles[2usize]
    } else if 0 != moveresult.flags & 4i32 {
        if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double)
            < (*bs).thinktime as libc::c_double * 0.8f64
        {
            BotRoamGoal(bs, target.as_mut_ptr());
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            (*bs).ideal_viewangles[2usize] =
                ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
        }
    } else if 0 == (*bs).flags & 32i32 {
        if 0 != trap_BotMovementViewTarget(
            (*bs).ms,
            goal as *mut libc::c_void,
            (*bs).tfl,
            300i32 as libc::c_float,
            target.as_mut_ptr(),
        ) {
            dir[0usize] = target[0usize] - (*bs).origin[0usize];
            dir[1usize] = target[1usize] - (*bs).origin[1usize];
            dir[2usize] = target[2usize] - (*bs).origin[2usize];
            vectoangles(
                dir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        } else {
            vectoangles(
                moveresult.movedir.as_mut_ptr() as *const vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
        }
        (*bs).ideal_viewangles[2usize] =
            ((*bs).ideal_viewangles[2usize] as libc::c_double * 0.5f64) as vec_t
    }
    if 0 != moveresult.flags & 16i32 {
        (*bs).weaponnum = moveresult.weapon
    }
    if 0 != BotFindEnemy(bs, -1i32) {
        if 0 != BotWantsToRetreat(bs) {
            AIEnter_Battle_NBG(
                bs,
                b"activate entity: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            trap_BotResetLastAvoidReach((*bs).ms);
            trap_BotEmptyGoalStack((*bs).gs);
            AIEnter_Battle_Fight(
                bs,
                b"activate entity: found enemy\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        BotClearActivateGoalStack(bs);
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BotResetNodeSwitches() {
    numnodeswitches = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn BotDumpNodeSwitches(mut bs: *mut bot_state_t) {
    let mut i: libc::c_int = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    BotAI_Print(
        1i32,
        b"%s at %1.1f switched more than %d AI nodes\n\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        netname.as_mut_ptr(),
        floattime as libc::c_double,
        50i32,
    );
    i = 0i32;
    while i < numnodeswitches {
        BotAI_Print(
            1i32,
            b"%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            nodeswitch[i as usize].as_mut_ptr(),
        );
        i += 1
    }
    BotAI_Print(
        4i32,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
