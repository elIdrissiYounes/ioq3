use ai_cmd::notleader;
use ai_dmq3::{
    ctf_blueflag, ctf_redflag, gametype, BotPointAreaNum, BotSameTeam, BotSetLastOrderedTask,
    BotTeam, ClientFromName, ClientName,
};
use ai_main::{
    bot_activategoal_s, bot_activategoal_t, bot_developer, bot_state_s, bot_state_t,
    bot_waypoint_s, bot_waypoint_t, floattime, BotAILoadMap, BotAISetup, BotAISetupClient,
    BotAIShutdown, BotAIShutdownClient, BotAIStartFrame, BotAI_GetClientState,
    BotInterbreedEndMatch, BotTestAAS,
};
use ai_variadic_h::BotAI_BotInitialChat;
use ai_vcmd::BotVoiceChat_Defend;
use be_ai_goal_h::{bot_goal_s, bot_goal_t};
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
    bot_settings_s, bot_settings_t, clientConnected_t, clientPersistant_t, clientSession_t,
    gclient_s, gentity_s, gentity_t, level_locals_t, moverState_t, playerTeamStateState_t,
    playerTeamState_t, spectatorState_t, trap_AAS_AreaTravelTimeToGoalArea, trap_BotEnterChat,
    trap_BotGetChatMessage, trap_BotQueueConsoleMessage, trap_GetConfigstring, CON_CONNECTED,
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
    byte, cplane_s, cplane_t, entityState_s, entityState_t, fileHandle_t, playerState_s,
    playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t, usercmd_s, usercmd_t,
    vec3_t, vec_t, Com_sprintf, Info_ValueForKey, Q_stricmp, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, memcpy, rand, strcpy, strlen, strncpy};

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
#[no_mangle]
pub unsafe extern "C" fn BotTeamAI(mut bs: *mut bot_state_t) {
    let mut numteammates: libc::c_int = 0;
    let mut netname: [libc::c_char; 36] = [0; 36];
    if gametype < GT_TEAM as libc::c_int {
        return;
    }
    if 0 == BotValidTeamLeader(bs) {
        if 0 == FindHumanTeamLeader(bs) {
            if 0. == (*bs).askteamleader_time && 0. == (*bs).becometeamleader_time {
                if (*bs).entergame_time + 10i32 as libc::c_float > floattime {
                    (*bs).askteamleader_time = floattime
                        + 5i32 as libc::c_float
                        + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                            * 10i32 as libc::c_float
                } else {
                    (*bs).becometeamleader_time = floattime
                        + 5i32 as libc::c_float
                        + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                            * 10i32 as libc::c_float
                }
            }
            if 0. != (*bs).askteamleader_time && (*bs).askteamleader_time < floattime {
                BotAI_BotInitialChat(
                    bs,
                    b"whoisteamleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                trap_BotEnterChat((*bs).cs, 0i32, 1i32);
                (*bs).askteamleader_time = 0i32 as libc::c_float;
                (*bs).becometeamleader_time = floattime
                    + 8i32 as libc::c_float
                    + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                        * 10i32 as libc::c_float
            }
            if 0. != (*bs).becometeamleader_time && (*bs).becometeamleader_time < floattime {
                BotAI_BotInitialChat(
                    bs,
                    b"iamteamleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    0 as *mut libc::c_void,
                );
                trap_BotEnterChat((*bs).cs, 0i32, 1i32);
                BotSayVoiceTeamOrder(
                    bs,
                    -1i32,
                    b"startleader\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    (*bs).client,
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                strncpy(
                    (*bs).teamleader.as_mut_ptr(),
                    netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong,
                );
                (*bs).teamleader[(::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
                    as usize] = '\u{0}' as i32 as libc::c_char;
                (*bs).becometeamleader_time = 0i32 as libc::c_float
            }
            return;
        }
    }
    (*bs).askteamleader_time = 0i32 as libc::c_float;
    (*bs).becometeamleader_time = 0i32 as libc::c_float;
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr()) != 0i32 {
        return;
    }
    numteammates = BotNumTeamMates(bs);
    match gametype {
        3 => {
            if (*bs).numteammates != numteammates || 0 != (*bs).forceorders {
                (*bs).teamgiveorders_time = floattime;
                (*bs).numteammates = numteammates;
                (*bs).forceorders = qfalse as libc::c_int
            }
            if 0. != (*bs).teamgiveorders_time
                && (*bs).teamgiveorders_time < floattime - 5i32 as libc::c_float
            {
                BotTeamOrders(bs);
                (*bs).teamgiveorders_time = floattime + 120i32 as libc::c_float
            }
        }
        4 => {
            if (*bs).numteammates != numteammates
                || 0 != (*bs).flagstatuschanged
                || 0 != (*bs).forceorders
            {
                (*bs).teamgiveorders_time = floattime;
                (*bs).numteammates = numteammates;
                (*bs).flagstatuschanged = qfalse as libc::c_int;
                (*bs).forceorders = qfalse as libc::c_int
            }
            if (*bs).lastflagcapture_time < floattime - 240i32 as libc::c_float {
                (*bs).lastflagcapture_time = floattime;
                if (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double)
                    < 0.4f64
                {
                    (*bs).ctfstrategy ^= 1i32;
                    (*bs).teamgiveorders_time = floattime
                }
            }
            if 0. != (*bs).teamgiveorders_time
                && (*bs).teamgiveorders_time < floattime - 3i32 as libc::c_float
            {
                BotCTFOrders(bs);
                (*bs).teamgiveorders_time = 0i32 as libc::c_float
            }
        }
        _ => {}
    };
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCTFOrders(mut bs: *mut bot_state_t) {
    let mut flagstatus: libc::c_int = 0;
    if BotTeam(bs) == TEAM_RED as libc::c_int {
        flagstatus = (*bs).redflagstatus * 2i32 + (*bs).blueflagstatus
    } else {
        flagstatus = (*bs).blueflagstatus * 2i32 + (*bs).redflagstatus
    }
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
BotCTFOrders
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCTFOrders_BothFlagsNotAtBase(mut bs: *mut bot_state_t) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [
        0i32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];
    let mut name: [libc::c_char; 36] = [0; 36];
    let mut carriername: [libc::c_char; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    match (*bs).numteammates {
        1 => {}
        2 => {
            if teammates[0usize] != (*bs).flagcarrier {
                other = teammates[0usize]
            } else {
                other = teammates[1usize]
            }
            ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        3 => {
            if teammates[0usize] != (*bs).flagcarrier {
                other = teammates[0usize]
            } else {
                other = teammates[1usize]
            }
            ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            if (*bs).flagcarrier != -1i32 {
                ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                if (*bs).flagcarrier == (*bs).client {
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        other,
                        b"followme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_accompany\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        carriername.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayVoiceTeamOrder(
                        bs,
                        other,
                        b"followflagcarrier\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
            } else {
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayVoiceTeamOrder(
                    bs,
                    other,
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            BotSayTeamOrder(bs, other);
            if teammates[2usize] != (*bs).flagcarrier {
                other = teammates[2usize]
            } else {
                other = teammates[1usize]
            }
            ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"returnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        _ => {
            defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.4f64
                + 0.5f64) as libc::c_int;
            if defenders > 4i32 {
                defenders = 4i32
            }
            attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.5f64
                + 0.5f64) as libc::c_int;
            if attackers > 5i32 {
                attackers = 5i32
            }
            if (*bs).flagcarrier != -1i32 {
                ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                i = 0i32;
                while i < defenders {
                    //
                    if !(teammates[i as usize] == (*bs).flagcarrier) {
                        ClientName(
                            teammates[i as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        if (*bs).flagcarrier == (*bs).client {
                            BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[i as usize],
                                b"followme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        } else {
                            BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompany\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                carriername.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[i as usize],
                                b"followflagcarrier\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        BotSayTeamOrder(bs, teammates[i as usize]);
                    }
                    i += 1
                }
            } else {
                i = 0i32;
                while i < defenders {
                    //
                    if !(teammates[i as usize] == (*bs).flagcarrier) {
                        ClientName(
                            teammates[i as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        BotAI_BotInitialChat(
                            bs,
                            b"cmd_getflag\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            name.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                        BotSayVoiceTeamOrder(
                            bs,
                            teammates[i as usize],
                            b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        BotSayTeamOrder(bs, teammates[i as usize]);
                    }
                    i += 1
                }
            }
            i = 0i32;
            while i < attackers {
                //
                if !(teammates[(numteammates - i - 1i32) as usize] == (*bs).flagcarrier) {
                    ClientName(
                        teammates[(numteammates - i - 1i32) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1i32) as usize],
                        b"returnflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                i += 1
            }
        }
    };
}
/*
==================
BotSayVoiceTeamOrder
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSayVoiceTeamOrder(
    mut bs: *mut bot_state_t,
    mut toclient: libc::c_int,
    mut voicechat: *mut libc::c_char,
) {
}
/*
==================
BotSayTeamOrders
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSayTeamOrder(mut bs: *mut bot_state_t, mut toclient: libc::c_int) {
    BotSayTeamOrderAlways(bs, toclient);
}
/*
==================
BotSayTeamOrders
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSayTeamOrderAlways(
    mut bs: *mut bot_state_t,
    mut toclient: libc::c_int,
) {
    let mut teamchat: [libc::c_char; 256] = [0; 256];
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut name: [libc::c_char; 36] = [0; 36];
    if (*bs).client == toclient {
        trap_BotGetChatMessage(
            (*bs).cs,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        );
        ClientName(
            (*bs).client,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
        );
        Com_sprintf(
            teamchat.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            b"\x19(%s\x19)\x19: %s\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            buf.as_mut_ptr(),
        );
        trap_BotQueueConsoleMessage((*bs).cs, 1i32, teamchat.as_mut_ptr());
    } else {
        trap_BotEnterChat((*bs).cs, toclient, 2i32);
    };
}
/*
==================
BotSortTeamMatesByTaskPreference
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSortTeamMatesByTaskPreference(
    mut bs: *mut bot_state_t,
    mut teammates: *mut libc::c_int,
    mut numteammates: libc::c_int,
) -> libc::c_int {
    let mut defenders: [libc::c_int; 64] = [0; 64];
    let mut numdefenders: libc::c_int = 0;
    let mut attackers: [libc::c_int; 64] = [0; 64];
    let mut numattackers: libc::c_int = 0;
    let mut roamers: [libc::c_int; 64] = [0; 64];
    let mut numroamers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut preference: libc::c_int = 0;
    numroamers = 0i32;
    numattackers = numroamers;
    numdefenders = numattackers;
    i = 0i32;
    while i < numteammates {
        preference = BotGetTeamMateTaskPreference(bs, *teammates.offset(i as isize));
        if 0 != preference & 1i32 {
            let fresh0 = numdefenders;
            numdefenders = numdefenders + 1;
            defenders[fresh0 as usize] = *teammates.offset(i as isize)
        } else if 0 != preference & 2i32 {
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
    numteammates = 0i32;
    memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut libc::c_int as *mut libc::c_void,
        defenders.as_mut_ptr() as *const libc::c_void,
        (numdefenders as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    numteammates += numdefenders;
    memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut libc::c_int as *mut libc::c_void,
        roamers.as_mut_ptr() as *const libc::c_void,
        (numroamers as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    numteammates += numroamers;
    memcpy(
        &mut *teammates.offset(numteammates as isize) as *mut libc::c_int as *mut libc::c_void,
        attackers.as_mut_ptr() as *const libc::c_void,
        (numattackers as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    numteammates += numattackers;
    return numteammates;
}
#[no_mangle]
pub unsafe extern "C" fn BotGetTeamMateTaskPreference(
    mut bs: *mut bot_state_t,
    mut teammate: libc::c_int,
) -> libc::c_int {
    let mut teammatename: [libc::c_char; 36] = [0; 36];
    if 0 == ctftaskpreferences[teammate as usize].preference {
        return 0i32;
    }
    ClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != Q_stricmp(
        teammatename.as_mut_ptr(),
        ctftaskpreferences[teammate as usize].name.as_mut_ptr(),
    ) {
        return 0i32;
    }
    return ctftaskpreferences[teammate as usize].preference;
}
#[no_mangle]
pub static mut ctftaskpreferences: [bot_ctftaskpreference_t; 64] = [bot_ctftaskpreference_s {
    name: [0; 36],
    preference: 0,
}; 64];
/*
==================
BotSortTeamMatesByBaseTravelTime
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotSortTeamMatesByBaseTravelTime(
    mut bs: *mut bot_state_t,
    mut teammates: *mut libc::c_int,
    mut maxteammates: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut numteammates: libc::c_int = 0;
    let mut traveltime: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut traveltimes: [libc::c_int; 64] = [0; 64];
    let mut goal: *mut bot_goal_t = 0 as *mut bot_goal_t;
    if gametype == GT_CTF as libc::c_int {
        if BotTeam(bs) == TEAM_RED as libc::c_int {
            goal = &mut ctf_redflag
        } else {
            goal = &mut ctf_blueflag
        }
    }
    numteammates = 0i32;
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
                //
                if 0 != BotSameTeam(bs, i) && !goal.is_null() {
                    traveltime = BotClientTravelTimeToGoal(i, goal);
                    j = 0i32;
                    while j < numteammates {
                        if traveltime < traveltimes[j as usize] {
                            k = numteammates;
                            while k > j {
                                traveltimes[k as usize] = traveltimes[(k - 1i32) as usize];
                                *teammates.offset(k as isize) =
                                    *teammates.offset((k - 1i32) as isize);
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
BotClientTravelTimeToGoal
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotClientTravelTimeToGoal(
    mut client: libc::c_int,
    mut goal: *mut bot_goal_t,
) -> libc::c_int {
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
    let mut areanum: libc::c_int = 0;
    if 0 != BotAI_GetClientState(client, &mut ps) {
        areanum = BotPointAreaNum(ps.origin.as_mut_ptr())
    } else {
        areanum = 0i32
    }
    if 0 == areanum {
        return 1i32;
    }
    return trap_AAS_AreaTravelTimeToGoalArea(
        areanum,
        ps.origin.as_mut_ptr(),
        (*goal).areanum,
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
}
/*
==================
BotCTFOrders
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCTFOrders_FlagNotAtBase(mut bs: *mut bot_state_t) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [0; 64];
    let mut name: [libc::c_char; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    if 0 == (*bs).ctfstrategy & 1i32 {
        match (*bs).numteammates {
            1 => {}
            2 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[2usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.3f64
                    + 0.5f64) as libc::c_int;
                if defenders > 3i32 {
                    defenders = 3i32
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.6f64
                    + 0.5f64) as libc::c_int;
                if attackers > 6i32 {
                    attackers = 6i32
                }
                i = 0i32;
                while i < defenders {
                    ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0i32;
                while i < attackers {
                    ClientName(
                        teammates[(numteammates - i - 1i32) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[0usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
            }
        }
    } else {
        match (*bs).numteammates {
            1 => {}
            2 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[2usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.2f64
                    + 0.5f64) as libc::c_int;
                if defenders > 2i32 {
                    defenders = 2i32
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.7f64
                    + 0.5f64) as libc::c_int;
                if attackers > 7i32 {
                    attackers = 7i32
                }
                i = 0i32;
                while i < defenders {
                    ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0i32;
                while i < attackers {
                    ClientName(
                        teammates[(numteammates - i - 1i32) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1i32) as usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
pub unsafe extern "C" fn BotCTFOrders_EnemyFlagNotAtBase(mut bs: *mut bot_state_t) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [0; 64];
    let mut name: [libc::c_char; 36] = [0; 36];
    let mut carriername: [libc::c_char; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    match numteammates {
        1 => {}
        2 => {
            if teammates[0usize] == (*bs).flagcarrier {
                other = teammates[1usize]
            } else {
                other = teammates[0usize]
            }
            ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        3 => {
            if teammates[0usize] != (*bs).flagcarrier {
                other = teammates[0usize]
            } else {
                other = teammates[1usize]
            }
            ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if teammates[2usize] != (*bs).flagcarrier {
                other = teammates[2usize]
            } else {
                other = teammates[1usize]
            }
            ClientName(
                other,
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
            BotAI_BotInitialChat(
                bs,
                b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
            BotSayTeamOrder(bs, other);
            BotSayVoiceTeamOrder(
                bs,
                other,
                b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        _ => {
            defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.6f64
                + 0.5f64) as libc::c_int;
            if defenders > 6i32 {
                defenders = 6i32
            }
            attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double * 0.3f64
                + 0.5f64) as libc::c_int;
            if attackers > 3i32 {
                attackers = 3i32
            }
            i = 0i32;
            while i < defenders {
                //
                if !(teammates[i as usize] == (*bs).flagcarrier) {
                    ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                i += 1
            }
            if (*bs).flagcarrier != -1i32 {
                ClientName(
                    (*bs).flagcarrier,
                    carriername.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                i = 0i32;
                while i < attackers {
                    //
                    if !(teammates[(numteammates - i - 1i32) as usize] == (*bs).flagcarrier) {
                        ClientName(
                            teammates[(numteammates - i - 1i32) as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        if (*bs).flagcarrier == (*bs).client {
                            BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[(numteammates - i - 1i32) as usize],
                                b"followme\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        } else {
                            BotAI_BotInitialChat(
                                bs,
                                b"cmd_accompany\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                name.as_mut_ptr(),
                                carriername.as_mut_ptr(),
                                0 as *mut libc::c_void,
                            );
                            BotSayVoiceTeamOrder(
                                bs,
                                teammates[(numteammates - i - 1i32) as usize],
                                b"followflagcarrier\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                        BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    }
                    i += 1
                }
            } else {
                i = 0i32;
                while i < attackers {
                    //
                    if !(teammates[(numteammates - i - 1i32) as usize] == (*bs).flagcarrier) {
                        ClientName(
                            teammates[(numteammates - i - 1i32) as usize],
                            name.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        BotAI_BotInitialChat(
                            bs,
                            b"cmd_getflag\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            name.as_mut_ptr(),
                            0 as *mut libc::c_void,
                        );
                        BotSayVoiceTeamOrder(
                            bs,
                            teammates[(numteammates - i - 1i32) as usize],
                            b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
pub unsafe extern "C" fn BotCTFOrders_BothFlagsAtBase(mut bs: *mut bot_state_t) {
    let mut numteammates: libc::c_int = 0;
    let mut defenders: libc::c_int = 0;
    let mut attackers: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut teammates: [libc::c_int; 64] = [
        0i32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];
    let mut name: [libc::c_char; 36] = [0; 36];
    numteammates = BotSortTeamMatesByBaseTravelTime(
        bs,
        teammates.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_int; 64]>() as libc::c_ulong as libc::c_int,
    );
    BotSortTeamMatesByTaskPreference(bs, teammates.as_mut_ptr(), numteammates);
    if 0 == (*bs).ctfstrategy & 1i32 {
        match numteammates {
            1 => {}
            2 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[2usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.5f64
                    + 0.5f64) as libc::c_int;
                if defenders > 5i32 {
                    defenders = 5i32
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.4f64
                    + 0.5f64) as libc::c_int;
                if attackers > 4i32 {
                    attackers = 4i32
                }
                i = 0i32;
                while i < defenders {
                    ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0i32;
                while i < attackers {
                    ClientName(
                        teammates[(numteammates - i - 1i32) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1i32) as usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
            }
        }
    } else {
        match numteammates {
            1 => {}
            2 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            3 => {
                ClientName(
                    teammates[0usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_defendbase\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[0usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[0usize],
                    b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[1usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[1usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[1usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                ClientName(
                    teammates[2usize],
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                BotAI_BotInitialChat(
                    bs,
                    b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    name.as_mut_ptr(),
                    0 as *mut libc::c_void,
                );
                BotSayTeamOrder(bs, teammates[2usize]);
                BotSayVoiceTeamOrder(
                    bs,
                    teammates[2usize],
                    b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                defenders = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.4f64
                    + 0.5f64) as libc::c_int;
                if defenders > 4i32 {
                    defenders = 4i32
                }
                attackers = (numteammates as libc::c_float as libc::c_int as libc::c_double
                    * 0.5f64
                    + 0.5f64) as libc::c_int;
                if attackers > 5i32 {
                    attackers = 5i32
                }
                i = 0i32;
                while i < defenders {
                    ClientName(
                        teammates[i as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_defendbase\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[i as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[i as usize],
                        b"defend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
                i = 0i32;
                while i < attackers {
                    ClientName(
                        teammates[(numteammates - i - 1i32) as usize],
                        name.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                    );
                    BotAI_BotInitialChat(
                        bs,
                        b"cmd_getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name.as_mut_ptr(),
                        0 as *mut libc::c_void,
                    );
                    BotSayTeamOrder(bs, teammates[(numteammates - i - 1i32) as usize]);
                    BotSayVoiceTeamOrder(
                        bs,
                        teammates[(numteammates - i - 1i32) as usize],
                        b"getflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotTeamOrders

  FIXME: defend key areas?
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotTeamOrders(mut bs: *mut bot_state_t) {
    let mut teammates: [libc::c_int; 64] = [0; 64];
    let mut numteammates: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    numteammates = 0i32;
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
                if 0 != BotSameTeam(bs, i) {
                    teammates[numteammates as usize] = i;
                    numteammates += 1
                }
            }
        }
        i += 1
    }
    match numteammates {
        1 => {}
        2 => {}
        3 => {
            //nothing special
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2i32);
        }
        4 => {
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2i32);
            BotCreateGroup(bs, &mut teammates[2usize], 2i32);
        }
        5 => {
            BotCreateGroup(bs, teammates.as_mut_ptr(), 2i32);
            BotCreateGroup(bs, &mut teammates[2usize], 3i32);
        }
        _ => {
            if numteammates <= 10i32 {
                i = 0i32;
                while i < numteammates / 2i32 {
                    BotCreateGroup(bs, &mut teammates[(i * 2i32) as usize], 2i32);
                    i += 1
                }
            }
        }
    };
}
/*
==================
BotCreateGroup
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotCreateGroup(
    mut bs: *mut bot_state_t,
    mut teammates: *mut libc::c_int,
    mut groupsize: libc::c_int,
) {
    let mut name: [libc::c_char; 36] = [0; 36];
    let mut leadername: [libc::c_char; 36] = [0; 36];
    let mut i: libc::c_int = 0;
    ClientName(
        *teammates.offset(0isize),
        leadername.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    i = 1i32;
    while i < groupsize {
        ClientName(
            *teammates.offset(i as isize),
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
        );
        if *teammates.offset(0isize) == (*bs).client {
            BotAI_BotInitialChat(
                bs,
                b"cmd_accompanyme\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        } else {
            BotAI_BotInitialChat(
                bs,
                b"cmd_accompany\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
BotNumTeamMates
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotNumTeamMates(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut numplayers: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    numplayers = 0i32;
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
                if 0 != BotSameTeam(bs, i) {
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
FindHumanTeamLeader
==================
*/
#[no_mangle]
pub unsafe extern "C" fn FindHumanTeamLeader(mut bs: *mut bot_state_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 64i32 {
        if 0 != g_entities[i as usize].inuse as u64 {
            if 0 == g_entities[i as usize].r.svFlags & 0x8i32 {
                if 0 == notleader[i as usize] {
                    if 0 != BotSameTeam(bs, i) {
                        ClientName(
                            i,
                            (*bs).teamleader.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                as libc::c_int,
                        );
                        if 0 == BotSetLastOrderedTask(bs) {
                            BotVoiceChat_Defend(bs, i, 2i32);
                        }
                        return qtrue as libc::c_int;
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
BotValidTeamLeader
==================
*/
#[no_mangle]
pub unsafe extern "C" fn BotValidTeamLeader(mut bs: *mut bot_state_t) -> libc::c_int {
    if 0 == strlen((*bs).teamleader.as_mut_ptr()) {
        return qfalse as libc::c_int;
    }
    if ClientFromName((*bs).teamleader.as_mut_ptr()) == -1i32 {
        return qfalse as libc::c_int;
    }
    return qtrue as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn BotSetTeamMateTaskPreference(
    mut bs: *mut bot_state_t,
    mut teammate: libc::c_int,
    mut preference: libc::c_int,
) {
    let mut teammatename: [libc::c_char; 36] = [0; 36];
    ctftaskpreferences[teammate as usize].preference = preference;
    ClientName(
        teammate,
        teammatename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    strcpy(
        ctftaskpreferences[teammate as usize].name.as_mut_ptr(),
        teammatename.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChat(
    mut bs: *mut bot_state_t,
    mut toclient: libc::c_int,
    mut voicechat: *mut libc::c_char,
) {
}
#[no_mangle]
pub unsafe extern "C" fn BotVoiceChatOnly(
    mut bs: *mut bot_state_t,
    mut toclient: libc::c_int,
    mut voicechat: *mut libc::c_char,
) {
}
pub type bot_ctftaskpreference_t = bot_ctftaskpreference_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_ctftaskpreference_s {
    pub name: [libc::c_char; 36],
    pub preference: libc::c_int,
}
