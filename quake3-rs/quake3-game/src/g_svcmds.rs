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
    gitem_s, gitem_t, itemType_t, team_t, unnamed_0, ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE,
    ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER,
    ET_TEAM, ET_TELEPORT_TRIGGER, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS,
    TEAM_RED, TEAM_SPECTATOR,
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
    spectatorState_t, trap_Argc, trap_Argv, trap_Cvar_Set, trap_SendConsoleCommand,
    trap_SendServerCommand, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2,
    MOVER_2TO1, MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT,
    SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
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
    unnamed, usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, Com_Printf, Q_strcat, Q_stricmp,
    Q_strncpyz, EXEC_APPEND, EXEC_INSERT, EXEC_NOW, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, strchr, strlen};
extern crate libc;

//
// g_svcmds.c
//
#[no_mangle]
pub unsafe extern "C" fn ConsoleCommand() -> qboolean {
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    trap_Argv(
        0i32,
        cmd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"entitylist\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_EntityList_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"forceteam\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_ForceTeam_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"game_memory\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_GameMem_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"addbot\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_AddBot_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"botlist\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_BotList_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"abort_podium\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_AbortPodium_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"addip\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_AddIP_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"removeip\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Svcmd_RemoveIP_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"listip\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        trap_SendConsoleCommand(
            EXEC_NOW as libc::c_int,
            b"g_banIPs\n\x00" as *const u8 as *const libc::c_char,
        );
        return qtrue;
    }
    if 0 != g_dedicated.integer {
        if Q_stricmp(
            cmd.as_mut_ptr(),
            b"say\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            trap_SendServerCommand(
                -1i32,
                va(
                    b"print \"server: %s\n\"\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ConcatArgs(1i32),
                ),
            );
            return qtrue;
        }
        trap_SendServerCommand(
            -1i32,
            va(
                b"print \"server: %s\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ConcatArgs(0i32),
            ),
        );
        return qtrue;
    }
    return qfalse;
}
/*
=================
Svcmd_RemoveIP_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Svcmd_RemoveIP_f() {
    let mut f: ipFilter_t = ipFilter_s {
        mask: 0,
        compare: 0,
    };
    let mut i: libc::c_int = 0;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() < 2i32 {
        G_Printf(b"Usage: removeip <ip-mask>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    trap_Argv(
        1i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == StringToFilter(str.as_mut_ptr(), &mut f) as u64 {
        return;
    }
    i = 0i32;
    while i < numIPFilters {
        if ipFilters[i as usize].mask == f.mask && ipFilters[i as usize].compare == f.compare {
            ipFilters[i as usize].compare = 0xffffffffu32;
            G_Printf(b"Removed.\n\x00" as *const u8 as *const libc::c_char);
            UpdateIPBans();
            return;
        }
        i += 1
    }
    G_Printf(
        b"Didn\'t find %s.\n\x00" as *const u8 as *const libc::c_char,
        str.as_mut_ptr(),
    );
}
/*
=================
UpdateIPBans
=================
*/
unsafe extern "C" fn UpdateIPBans() {
    let mut b: [byte; 4] = [0i32 as byte, 0, 0, 0];
    let mut m: [byte; 4] = [0i32 as byte, 0, 0, 0];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut iplist_final: [libc::c_char; 256] = [
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
        0,
    ];
    let mut ip: [libc::c_char; 64] = [
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
    ];
    *iplist_final.as_mut_ptr() = 0i32 as libc::c_char;
    i = 0i32;
    while i < numIPFilters {
        if !(ipFilters[i as usize].compare == 0xffffffffu32) {
            *(b.as_mut_ptr() as *mut libc::c_uint) = ipFilters[i as usize].compare;
            *(m.as_mut_ptr() as *mut libc::c_uint) = ipFilters[i as usize].mask;
            *ip.as_mut_ptr() = 0i32 as libc::c_char;
            j = 0i32;
            while j < 4i32 {
                if m[j as usize] as libc::c_int != 255i32 {
                    Q_strcat(
                        ip.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        b"*\x00" as *const u8 as *const libc::c_char,
                    );
                } else {
                    Q_strcat(
                        ip.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        va(
                            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            b[j as usize] as libc::c_int,
                        ),
                    );
                }
                Q_strcat(
                    ip.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    if j < 3i32 {
                        b".\x00" as *const u8 as *const libc::c_char
                    } else {
                        b" \x00" as *const u8 as *const libc::c_char
                    },
                );
                j += 1
            }
            if strlen(iplist_final.as_mut_ptr()).wrapping_add(strlen(ip.as_mut_ptr()))
                < 256i32 as libc::c_ulong
            {
                Q_strcat(
                    iplist_final.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
                    ip.as_mut_ptr(),
                );
            } else {
                Com_Printf(
                    b"g_banIPs overflowed at MAX_CVAR_VALUE_STRING\n\x00" as *const u8
                        as *const libc::c_char,
                );
                break;
            }
        }
        i += 1
    }
    trap_Cvar_Set(
        b"g_banIPs\x00" as *const u8 as *const libc::c_char,
        iplist_final.as_mut_ptr(),
    );
}
static mut ipFilters: [ipFilter_t; 1024] = [ipFilter_s {
    mask: 0,
    compare: 0,
}; 1024];
static mut numIPFilters: libc::c_int = 0;
/*
=================
StringToFilter
=================
*/
unsafe extern "C" fn StringToFilter(mut s: *mut libc::c_char, mut f: *mut ipFilter_t) -> qboolean {
    let mut num: [libc::c_char; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: [byte; 4] = [0; 4];
    let mut m: [byte; 4] = [0; 4];
    i = 0i32;
    while i < 4i32 {
        b[i as usize] = 0i32 as byte;
        m[i as usize] = 0i32 as byte;
        i += 1
    }
    i = 0i32;
    while i < 4i32 {
        if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
            // 'match any'
            if *s as libc::c_int == '*' as i32 {
                s = s.offset(1isize);
                if 0 == *s {
                    break;
                }
                s = s.offset(1isize)
            } else {
                G_Printf(
                    b"Bad filter address: %s\n\x00" as *const u8 as *const libc::c_char,
                    s,
                );
                return qfalse;
            }
        } else {
            j = 0i32;
            while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
                let fresh1 = j;
                j = j + 1;
                let fresh0 = s;
                s = s.offset(1);
                num[fresh1 as usize] = *fresh0
            }
            num[j as usize] = 0i32 as libc::c_char;
            b[i as usize] = atoi(num.as_mut_ptr()) as byte;
            m[i as usize] = 255i32 as byte;
            if 0 == *s {
                break;
            }
            s = s.offset(1isize)
        }
        i += 1
    }
    (*f).mask = *(m.as_mut_ptr() as *mut libc::c_uint);
    (*f).compare = *(b.as_mut_ptr() as *mut libc::c_uint);
    return qtrue;
}
/*
=================
Svcmd_AddIP_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Svcmd_AddIP_f() {
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() < 2i32 {
        G_Printf(b"Usage: addip <ip-mask>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    trap_Argv(
        1i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    AddIP(str.as_mut_ptr());
}
/*
=================
AddIP
=================
*/
unsafe extern "C" fn AddIP(mut str: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < numIPFilters {
        if ipFilters[i as usize].compare == 0xffffffffu32 {
            // free spot
            break;
        } else {
            i += 1
        }
    }
    if i == numIPFilters {
        if numIPFilters == 1024i32 {
            G_Printf(b"IP filter list is full\n\x00" as *const u8 as *const libc::c_char);
            return;
        }
        numIPFilters += 1
    }
    if 0 == StringToFilter(str, &mut *ipFilters.as_mut_ptr().offset(i as isize)) as u64 {
        ipFilters[i as usize].compare = 0xffffffffu32
    }
    UpdateIPBans();
}
/*
===================
Svcmd_ForceTeam_f

forceteam <player> <team>
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Svcmd_ForceTeam_f() {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() < 3i32 {
        G_Printf(b"Usage: forceteam <player> <team>\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    trap_Argv(
        1i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    cl = ClientForString(str.as_mut_ptr());
    if cl.is_null() {
        return;
    }
    trap_Argv(
        2i32,
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    SetTeam(
        &mut *g_entities
            .as_mut_ptr()
            .offset(cl.wrapping_offset_from(level.clients) as libc::c_long as isize),
        str.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ClientForString(mut s: *const libc::c_char) -> *mut gclient_t {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: libc::c_int = 0;
    let mut idnum: libc::c_int = 0;
    if *s.offset(0isize) as libc::c_int >= '0' as i32
        && *s.offset(0isize) as libc::c_int <= '9' as i32
    {
        idnum = atoi(s);
        if idnum < 0i32 || idnum >= level.maxclients {
            Com_Printf(
                b"Bad client slot: %i\n\x00" as *const u8 as *const libc::c_char,
                idnum,
            );
            return 0 as *mut gclient_t;
        }
        cl = &mut *level.clients.offset(idnum as isize) as *mut gclient_s;
        if (*cl).pers.connected as libc::c_uint == CON_DISCONNECTED as libc::c_int as libc::c_uint {
            G_Printf(
                b"Client %i is not connected\n\x00" as *const u8 as *const libc::c_char,
                idnum,
            );
            return 0 as *mut gclient_t;
        }
        return cl;
    }
    i = 0i32;
    while i < level.maxclients {
        cl = &mut *level.clients.offset(i as isize) as *mut gclient_s;
        if !((*cl).pers.connected as libc::c_uint
            == CON_DISCONNECTED as libc::c_int as libc::c_uint)
        {
            if 0 == Q_stricmp((*cl).pers.netname.as_mut_ptr(), s) {
                return cl;
            }
        }
        i += 1
    }
    G_Printf(
        b"User %s is not on the server\n\x00" as *const u8 as *const libc::c_char,
        s,
    );
    return 0 as *mut gclient_t;
}
/*
===================
Svcmd_EntityList_f
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Svcmd_EntityList_f() {
    let mut e: libc::c_int = 0;
    let mut check: *mut gentity_t = 0 as *mut gentity_t;
    check = g_entities.as_mut_ptr();
    e = 0i32;
    while e < level.num_entities {
        if !(0 == (*check).inuse as u64) {
            G_Printf(b"%3i:\x00" as *const u8 as *const libc::c_char, e);
            match (*check).s.eType {
                0 => {
                    G_Printf(b"ET_GENERAL          \x00" as *const u8 as *const libc::c_char);
                }
                1 => {
                    G_Printf(b"ET_PLAYER           \x00" as *const u8 as *const libc::c_char);
                }
                2 => {
                    G_Printf(b"ET_ITEM             \x00" as *const u8 as *const libc::c_char);
                }
                3 => {
                    G_Printf(b"ET_MISSILE          \x00" as *const u8 as *const libc::c_char);
                }
                4 => {
                    G_Printf(b"ET_MOVER            \x00" as *const u8 as *const libc::c_char);
                }
                5 => {
                    G_Printf(b"ET_BEAM             \x00" as *const u8 as *const libc::c_char);
                }
                6 => {
                    G_Printf(b"ET_PORTAL           \x00" as *const u8 as *const libc::c_char);
                }
                7 => {
                    G_Printf(b"ET_SPEAKER          \x00" as *const u8 as *const libc::c_char);
                }
                8 => {
                    G_Printf(b"ET_PUSH_TRIGGER     \x00" as *const u8 as *const libc::c_char);
                }
                9 => {
                    G_Printf(b"ET_TELEPORT_TRIGGER \x00" as *const u8 as *const libc::c_char);
                }
                10 => {
                    G_Printf(b"ET_INVISIBLE        \x00" as *const u8 as *const libc::c_char);
                }
                11 => {
                    G_Printf(b"ET_GRAPPLE          \x00" as *const u8 as *const libc::c_char);
                }
                _ => {
                    G_Printf(
                        b"%3i                 \x00" as *const u8 as *const libc::c_char,
                        (*check).s.eType,
                    );
                }
            }
            if !(*check).classname.is_null() {
                G_Printf(
                    b"%s\x00" as *const u8 as *const libc::c_char,
                    (*check).classname,
                );
            }
            G_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        e += 1;
        check = check.offset(1isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_ProcessIPBans() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: [libc::c_char; 256] = [0; 256];
    Q_strncpyz(
        str.as_mut_ptr(),
        g_banIPs.string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    s = g_banIPs.string.as_mut_ptr();
    t = s;
    while 0 != *t {
        /* */
        s = strchr(s, ' ' as i32);
        if s.is_null() {
            break;
        }
        while *s as libc::c_int == ' ' as i32 {
            let fresh2 = s;
            s = s.offset(1);
            *fresh2 = 0i32 as libc::c_char
        }
        if 0 != *t {
            AddIP(t);
        }
        t = s
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_FilterPacket(mut from: *mut libc::c_char) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut in_0: libc::c_uint = 0;
    let mut m: [byte; 4] = [0i32 as byte, 0, 0, 0];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0i32;
    p = from;
    while 0 != *p as libc::c_int && i < 4i32 {
        m[i as usize] = 0i32 as byte;
        while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            m[i as usize] =
                (m[i as usize] as libc::c_int * 10i32 + (*p as libc::c_int - '0' as i32)) as byte;
            p = p.offset(1isize)
        }
        if 0 == *p || *p as libc::c_int == ':' as i32 {
            break;
        }
        i += 1;
        p = p.offset(1isize)
    }
    in_0 = *(m.as_mut_ptr() as *mut libc::c_uint);
    i = 0i32;
    while i < numIPFilters {
        if in_0 & ipFilters[i as usize].mask == ipFilters[i as usize].compare {
            return (g_filterBan.integer != 0i32) as libc::c_int as qboolean;
        }
        i += 1
    }
    return (g_filterBan.integer == 0i32) as libc::c_int as qboolean;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipFilter_s {
    pub mask: libc::c_uint,
    pub compare: libc::c_uint,
}
pub type ipFilter_t = ipFilter_s;
