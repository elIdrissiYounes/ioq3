#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
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
    gitem_s, gitem_t, itemType_t, team_t, unnamed_0, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR,
    IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON,
    TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
use g_active::{ClientEndFrame, ClientThink, G_RunClient};
use g_arenas::{
    podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f, UpdateTournamentInfo,
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
    gclient_s, gclient_t, gentity_s, gentity_t, level_locals_t, moverState_t,
    playerTeamStateState_t, playerTeamState_t, spectatorState_t, trap_Argv, trap_BotAllocateClient,
    trap_BotFreeClient, trap_Cvar_Register, trap_Cvar_Set, trap_Cvar_Update,
    trap_Cvar_VariableIntegerValue, trap_Cvar_VariableStringBuffer, trap_DropClient,
    trap_FS_FCloseFile, trap_FS_FOpenFile, trap_FS_GetFileList, trap_FS_Read, trap_GetServerinfo,
    trap_GetUserinfo, trap_Print, trap_SendConsoleCommand, trap_SendServerCommand,
    trap_SetUserinfo, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1,
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
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t, fsMode_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    unnamed, usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, COM_Parse, COM_ParseExt, Com_Clamp,
    Com_Printf, Info_SetValueForKey, Info_ValueForKey, Q_stricmp, Q_strncpyz, EXEC_APPEND,
    EXEC_INSERT, EXEC_NOW, FS_APPEND, FS_APPEND_SYNC, FS_READ, FS_WRITE, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atof, atoi, rand, strcat, strcmp, strcpy, strlen, strrchr};
extern crate libc;

//
// g_bot.c
//
#[no_mangle]
pub unsafe extern "C" fn G_InitBots(mut restart: qboolean) {
    let mut fragLimit: libc::c_int = 0;
    let mut timeLimit: libc::c_int = 0;
    let mut arenainfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut strValue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut basedelay: libc::c_int = 0;
    let mut map: [libc::c_char; 64] = [0; 64];
    let mut serverinfo: [libc::c_char; 1024] = [0; 1024];
    G_LoadBots();
    G_LoadArenas();
    trap_Cvar_Register(
        &mut bot_minplayers,
        b"bot_minplayers\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        0x4i32,
    );
    if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int {
        trap_GetServerinfo(
            serverinfo.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        Q_strncpyz(
            map.as_mut_ptr(),
            Info_ValueForKey(
                serverinfo.as_mut_ptr(),
                b"mapname\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        arenainfo = G_GetArenaInfoByMap(map.as_mut_ptr());
        if arenainfo.is_null() {
            return;
        }
        strValue = Info_ValueForKey(
            arenainfo,
            b"fraglimit\x00" as *const u8 as *const libc::c_char,
        );
        fragLimit = atoi(strValue);
        if 0 != fragLimit {
            trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const libc::c_char,
                strValue,
            );
        } else {
            trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        strValue = Info_ValueForKey(
            arenainfo,
            b"timelimit\x00" as *const u8 as *const libc::c_char,
        );
        timeLimit = atoi(strValue);
        if 0 != timeLimit {
            trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const libc::c_char,
                strValue,
            );
        } else {
            trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        if 0 == fragLimit && 0 == timeLimit {
            trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const libc::c_char,
                b"10\x00" as *const u8 as *const libc::c_char,
            );
            trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
        basedelay = 2000i32;
        strValue = Info_ValueForKey(
            arenainfo,
            b"special\x00" as *const u8 as *const libc::c_char,
        );
        if Q_stricmp(
            strValue,
            b"training\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            basedelay += 10000i32
        }
        if 0 == restart as u64 {
            G_SpawnBots(
                Info_ValueForKey(arenainfo, b"bots\x00" as *const u8 as *const libc::c_char),
                basedelay,
            );
        }
    };
}
/*
===============
G_SpawnBots
===============
*/
unsafe extern "C" fn G_SpawnBots(mut botList: *mut libc::c_char, mut baseDelay: libc::c_int) {
    let mut bot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skill: libc::c_float = 0.;
    let mut delay: libc::c_int = 0;
    let mut bots: [libc::c_char; 1024] = [0; 1024];
    podium1 = 0 as *mut gentity_t;
    podium2 = 0 as *mut gentity_t;
    podium3 = 0 as *mut gentity_t;
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char);
    if skill < 1i32 as libc::c_float {
        trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
        skill = 1i32 as libc::c_float
    } else if skill > 5i32 as libc::c_float {
        trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
            b"5\x00" as *const u8 as *const libc::c_char,
        );
        skill = 5i32 as libc::c_float
    }
    Q_strncpyz(
        bots.as_mut_ptr(),
        botList,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    p = &mut *bots.as_mut_ptr().offset(0isize) as *mut libc::c_char;
    delay = baseDelay;
    while 0 != *p {
        while 0 != *p as libc::c_int && *p as libc::c_int == ' ' as i32 {
            p = p.offset(1isize)
        }
        if 0 == *p {
            break;
        }
        bot = p;
        while 0 != *p as libc::c_int && *p as libc::c_int != ' ' as i32 {
            p = p.offset(1isize)
        }
        if 0 != *p {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = 0i32 as libc::c_char
        }
        trap_SendConsoleCommand(
            EXEC_INSERT as libc::c_int,
            va(
                b"addbot %s %f free %i\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                bot,
                skill as libc::c_double,
                delay,
            ),
        );
        delay += 1500i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_VariableValue(
    mut var_name: *const libc::c_char,
) -> libc::c_float {
    let mut buf: [libc::c_char; 128] = [0; 128];
    trap_Cvar_VariableStringBuffer(
        var_name,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    return atof(buf.as_mut_ptr()) as libc::c_float;
}
/*
===============
G_GetArenaInfoByNumber
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_GetArenaInfoByMap(mut map: *const libc::c_char) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    n = 0i32;
    while n < g_numArenas {
        if Q_stricmp(
            Info_ValueForKey(
                g_arenaInfos[n as usize],
                b"map\x00" as *const u8 as *const libc::c_char,
            ),
            map,
        ) == 0i32
        {
            return g_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
static mut g_arenaInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];
#[no_mangle]
pub static mut g_numArenas: libc::c_int = 0;
#[no_mangle]
pub static mut bot_minplayers: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
/*
===============
G_LoadArenas
===============
*/
unsafe extern "C" fn G_LoadArenas() {
    let mut numdirs: libc::c_int = 0;
    let mut arenasFile: vmCvar_t = vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 1024] = [0; 1024];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    g_numArenas = 0i32;
    trap_Cvar_Register(
        &mut arenasFile,
        b"g_arenasFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10i32 | 0x40i32,
    );
    if 0 != *arenasFile.string.as_mut_ptr() {
        G_LoadArenasFromFile(arenasFile.string.as_mut_ptr());
    } else {
        G_LoadArenasFromFile(
            b"scripts/arenas.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    numdirs = trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".arena\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        1024i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0i32;
    while i < numdirs {
        dirlen = strlen(dirptr) as libc::c_int;
        strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        strcat(filename.as_mut_ptr(), dirptr);
        G_LoadArenasFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1i32) as isize)
    }
    trap_Print(va(
        b"%i arenas parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        g_numArenas,
    ));
    n = 0i32;
    while n < g_numArenas {
        Info_SetValueForKey(
            g_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const libc::c_char,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                n,
            ),
        );
        n += 1
    }
}
/*
===============
G_LoadArenasFromFile
===============
*/
unsafe extern "C" fn G_LoadArenasFromFile(mut filename: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut f: fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if 0 == f {
        trap_Print(va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192i32 {
        trap_FS_FCloseFile(f);
        trap_Print(va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192i32,
        ));
        return;
    }
    trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    g_numArenas += G_ParseInfos(
        buf.as_mut_ptr(),
        1024i32 - g_numArenas,
        &mut *g_arenaInfos.as_mut_ptr().offset(g_numArenas as isize),
    );
}
/*
===============
G_ParseInfos
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_ParseInfos(
    mut buf: *mut libc::c_char,
    mut max: libc::c_int,
    mut infos: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    count = 0i32;
    loop {
        token = COM_Parse(&mut buf);
        if 0 == *token.offset(0isize) {
            break;
        }
        if 0 != strcmp(token, b"{\x00" as *const u8 as *const libc::c_char) {
            Com_Printf(b"Missing { in info file\n\x00" as *const u8 as *const libc::c_char);
            break;
        } else if count == max {
            Com_Printf(b"Max infos exceeded\n\x00" as *const u8 as *const libc::c_char);
            break;
        } else {
            info[0usize] = '\u{0}' as i32 as libc::c_char;
            loop {
                token = COM_ParseExt(&mut buf, qtrue);
                if 0 == *token.offset(0isize) {
                    Com_Printf(
                        b"Unexpected end of info file\n\x00" as *const u8 as *const libc::c_char,
                    );
                    break;
                } else {
                    if 0 == strcmp(token, b"}\x00" as *const u8 as *const libc::c_char) {
                        break;
                    }
                    Q_strncpyz(
                        key.as_mut_ptr(),
                        token,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                            as libc::c_int,
                    );
                    token = COM_ParseExt(&mut buf, qfalse);
                    if 0 == *token.offset(0isize) {
                        strcpy(token, b"<NULL>\x00" as *const u8 as *const libc::c_char);
                    }
                    Info_SetValueForKey(info.as_mut_ptr(), key.as_mut_ptr(), token);
                }
            }
            let ref mut fresh1 = *infos.offset(count as isize);
            *fresh1 = G_Alloc(
                strlen(info.as_mut_ptr())
                    .wrapping_add(strlen(b"\\num\\\x00" as *const u8 as *const libc::c_char))
                    .wrapping_add(strlen(va(
                        b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1024i32,
                    )))
                    .wrapping_add(1i32 as libc::c_ulong) as libc::c_int,
            ) as *mut libc::c_char;
            if !(*infos.offset(count as isize)).is_null() {
                strcpy(*infos.offset(count as isize), info.as_mut_ptr());
                count += 1
            }
        }
    }
    return count;
}
/*
===============
G_LoadBots
===============
*/
unsafe extern "C" fn G_LoadBots() {
    let mut botsFile: vmCvar_t = vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
    let mut numdirs: libc::c_int = 0;
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 1024] = [0; 1024];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    if 0 == trap_Cvar_VariableIntegerValue(b"bot_enable\x00" as *const u8 as *const libc::c_char) {
        return;
    }
    g_numBots = 0i32;
    trap_Cvar_Register(
        &mut botsFile,
        b"g_botsFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10i32 | 0x40i32,
    );
    if 0 != *botsFile.string.as_mut_ptr() {
        G_LoadBotsFromFile(botsFile.string.as_mut_ptr());
    } else {
        G_LoadBotsFromFile(
            b"scripts/bots.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    numdirs = trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".bot\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        1024i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0i32;
    while i < numdirs {
        dirlen = strlen(dirptr) as libc::c_int;
        strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        strcat(filename.as_mut_ptr(), dirptr);
        G_LoadBotsFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1i32) as isize)
    }
    trap_Print(va(
        b"%i bots parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        g_numBots,
    ));
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
// g_bot.c
static mut g_numBots: libc::c_int = 0;
/*
===============
G_LoadBotsFromFile
===============
*/
unsafe extern "C" fn G_LoadBotsFromFile(mut filename: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut f: fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if 0 == f {
        trap_Print(va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192i32 {
        trap_Print(va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192i32,
        ));
        trap_FS_FCloseFile(f);
        return;
    }
    trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    g_numBots += G_ParseInfos(
        buf.as_mut_ptr(),
        1024i32 - g_numBots,
        &mut *g_botInfos.as_mut_ptr().offset(g_numBots as isize),
    );
}
static mut g_botInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];
#[no_mangle]
pub unsafe extern "C" fn G_GetBotInfoByNumber(mut num: libc::c_int) -> *mut libc::c_char {
    if num < 0i32 || num >= g_numBots {
        trap_Print(va(
            b"^1Invalid bot number: %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            num,
        ));
        return 0 as *mut libc::c_char;
    }
    return g_botInfos[num as usize];
}
#[no_mangle]
pub unsafe extern "C" fn G_GetBotInfoByName(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    n = 0i32;
    while n < g_numBots {
        value = Info_ValueForKey(
            g_botInfos[n as usize],
            b"name\x00" as *const u8 as *const libc::c_char,
        );
        if 0 == Q_stricmp(value, name) {
            return g_botInfos[n as usize];
        }
        n += 1
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn G_CheckBotSpawn() {
    let mut n: libc::c_int = 0;
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    G_CheckMinimumPlayers();
    n = 0i32;
    while n < 16i32 {
        if !(0 == botSpawnQueue[n as usize].spawnTime) {
            if !(botSpawnQueue[n as usize].spawnTime > level.time) {
                ClientBegin(botSpawnQueue[n as usize].clientNum);
                botSpawnQueue[n as usize].spawnTime = 0i32;
                if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int {
                    trap_GetUserinfo(
                        botSpawnQueue[n as usize].clientNum,
                        userinfo.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                            as libc::c_int,
                    );
                    PlayerIntroSound(Info_ValueForKey(
                        userinfo.as_mut_ptr(),
                        b"model\x00" as *const u8 as *const libc::c_char,
                    ));
                }
            }
        }
        n += 1
    }
}
/*
=================
PlayerIntroSound
=================
*/
unsafe extern "C" fn PlayerIntroSound(mut modelAndSkin: *const libc::c_char) {
    let mut model: [libc::c_char; 64] = [0; 64];
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    skin = strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh2 = skin;
        skin = skin.offset(1);
        *fresh2 = '\u{0}' as i32 as libc::c_char
    } else {
        skin = model.as_mut_ptr()
    }
    if Q_stricmp(skin, b"default\x00" as *const u8 as *const libc::c_char) == 0i32 {
        skin = model.as_mut_ptr()
    }
    trap_SendConsoleCommand(
        EXEC_APPEND as libc::c_int,
        va(
            b"play sound/player/announce/%s.wav\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            skin,
        ),
    );
}
static mut botSpawnQueue: [botSpawnQueue_t; 16] = [botSpawnQueue_t {
    clientNum: 0,
    spawnTime: 0,
}; 16];
/*
===============
G_CheckMinimumPlayers
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_CheckMinimumPlayers() {
    let mut minplayers: libc::c_int = 0;
    let mut humanplayers: libc::c_int = 0;
    let mut botplayers: libc::c_int = 0;
    static mut checkminimumplayers_time: libc::c_int = 0;
    if 0 != level.intermissiontime {
        return;
    }
    if checkminimumplayers_time > level.time - 10000i32 {
        return;
    }
    checkminimumplayers_time = level.time;
    trap_Cvar_Update(&mut bot_minplayers);
    minplayers = bot_minplayers.integer;
    if minplayers <= 0i32 {
        return;
    }
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        if minplayers >= g_maxclients.integer / 2i32 {
            minplayers = g_maxclients.integer / 2i32 - 1i32
        }
        humanplayers = G_CountHumanPlayers(TEAM_RED as libc::c_int);
        botplayers = G_CountBotPlayers(TEAM_RED as libc::c_int);
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_RED as libc::c_int);
        } else if humanplayers + botplayers > minplayers && 0 != botplayers {
            G_RemoveRandomBot(TEAM_RED as libc::c_int);
        }
        humanplayers = G_CountHumanPlayers(TEAM_BLUE as libc::c_int);
        botplayers = G_CountBotPlayers(TEAM_BLUE as libc::c_int);
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_BLUE as libc::c_int);
        } else if humanplayers + botplayers > minplayers && 0 != botplayers {
            G_RemoveRandomBot(TEAM_BLUE as libc::c_int);
        }
    } else if g_gametype.integer == GT_TOURNAMENT as libc::c_int {
        if minplayers >= g_maxclients.integer {
            minplayers = g_maxclients.integer - 1i32
        }
        humanplayers = G_CountHumanPlayers(-1i32);
        botplayers = G_CountBotPlayers(-1i32);
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_FREE as libc::c_int);
        } else if humanplayers + botplayers > minplayers && 0 != botplayers {
            if 0 == G_RemoveRandomBot(TEAM_SPECTATOR as libc::c_int) {
                G_RemoveRandomBot(-1i32);
            }
        }
    } else if g_gametype.integer == GT_FFA as libc::c_int {
        if minplayers >= g_maxclients.integer {
            minplayers = g_maxclients.integer - 1i32
        }
        humanplayers = G_CountHumanPlayers(TEAM_FREE as libc::c_int);
        botplayers = G_CountBotPlayers(TEAM_FREE as libc::c_int);
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(TEAM_FREE as libc::c_int);
        } else if humanplayers + botplayers > minplayers && 0 != botplayers {
            G_RemoveRandomBot(TEAM_FREE as libc::c_int);
        }
    };
}
/*
===============
G_RemoveRandomBot
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_RemoveRandomBot(mut team: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    i = 0i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as libc::c_uint != CON_CONNECTED as libc::c_int as libc::c_uint) {
            if !(0 == g_entities[i as usize].r.svFlags & 0x8i32) {
                if !(team >= 0i32 && (*cl).sess.sessionTeam as libc::c_uint != team as libc::c_uint)
                {
                    trap_SendConsoleCommand(
                        EXEC_INSERT as libc::c_int,
                        va(
                            b"clientkick %d\n\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            i,
                        ),
                    );
                    return qtrue as libc::c_int;
                }
            }
        }
        i += 1
    }
    return qfalse as libc::c_int;
}
/*
===============
G_AddRandomBot
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_AddRandomBot(mut team: libc::c_int) {
    let mut teamstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skill: libc::c_float = 0.;
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char);
    if team == TEAM_RED as libc::c_int {
        teamstr = b"red\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if team == TEAM_BLUE as libc::c_int {
        teamstr = b"blue\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        teamstr = b"free\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    trap_SendConsoleCommand(
        EXEC_INSERT as libc::c_int,
        va(
            b"addbot random %f %s %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            skill as libc::c_double,
            teamstr,
            0i32,
        ),
    );
}
/*
===============
G_CountBotPlayers

Check connected and connecting (delay join) bots.
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_CountBotPlayers(mut team: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    num = 0i32;
    i = 0i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as libc::c_uint
            == CON_DISCONNECTED as libc::c_int as libc::c_uint)
        {
            if !(0 == g_entities[i as usize].r.svFlags & 0x8i32) {
                if !(team >= 0i32 && (*cl).sess.sessionTeam as libc::c_uint != team as libc::c_uint)
                {
                    num += 1
                }
            }
        }
        i += 1
    }
    return num;
}
/*
===============
G_CountHumanPlayers
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_CountHumanPlayers(mut team: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    num = 0i32;
    i = 0i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as libc::c_uint != CON_CONNECTED as libc::c_int as libc::c_uint) {
            if !(0 != g_entities[i as usize].r.svFlags & 0x8i32) {
                if !(team >= 0i32 && (*cl).sess.sessionTeam as libc::c_uint != team as libc::c_uint)
                {
                    num += 1
                }
            }
        }
        i += 1
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn G_RemoveQueuedBotBegin(mut clientNum: libc::c_int) {
    let mut n: libc::c_int = 0;
    n = 0i32;
    while n < 16i32 {
        if botSpawnQueue[n as usize].clientNum == clientNum {
            botSpawnQueue[n as usize].spawnTime = 0i32;
            return;
        }
        n += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_BotConnect(
    mut clientNum: libc::c_int,
    mut restart: qboolean,
) -> qboolean {
    let mut settings: bot_settings_t = bot_settings_s {
        characterfile: [0; 144],
        skill: 0.,
    };
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    trap_GetUserinfo(
        clientNum,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Q_strncpyz(
        settings.characterfile.as_mut_ptr(),
        Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"characterfile\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 144]>() as libc::c_ulong as libc::c_int,
    );
    settings.skill = atof(Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"skill\x00" as *const u8 as *const libc::c_char,
    )) as libc::c_float;
    if 0 == BotAISetupClient(clientNum, &mut settings, restart) {
        trap_DropClient(
            clientNum,
            b"BotAISetupClient failed\x00" as *const u8 as *const libc::c_char,
        );
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Svcmd_AddBot_f() {
    let mut skill: libc::c_float = 0.;
    let mut delay: libc::c_int = 0;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut altname: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut team: [libc::c_char; 1024] = [0; 1024];
    if 0 == trap_Cvar_VariableIntegerValue(b"bot_enable\x00" as *const u8 as *const libc::c_char) {
        return;
    }
    trap_Argv(
        1i32,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == name[0usize] {
        trap_Print(
            b"Usage: Addbot <botname> [skill 1-5] [team] [msec delay] [altname]\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    trap_Argv(
        2i32,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == string[0usize] {
        skill = 4i32 as libc::c_float
    } else {
        skill = Com_Clamp(
            1i32 as libc::c_float,
            5i32 as libc::c_float,
            atof(string.as_mut_ptr()) as libc::c_float,
        )
    }
    trap_Argv(
        3i32,
        team.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    trap_Argv(
        4i32,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == string[0usize] {
        delay = 0i32
    } else {
        delay = atoi(string.as_mut_ptr())
    }
    trap_Argv(
        5i32,
        altname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    G_AddBot(
        name.as_mut_ptr(),
        skill,
        team.as_mut_ptr(),
        delay,
        altname.as_mut_ptr(),
    );
    if level.time - level.startTime > 1000i32
        && 0 != trap_Cvar_VariableIntegerValue(
            b"cl_running\x00" as *const u8 as *const libc::c_char,
        )
    {
        trap_SendServerCommand(
            -1i32,
            b"loaddefered\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
===============
G_AddBot
===============
*/
unsafe extern "C" fn G_AddBot(
    mut name: *const libc::c_char,
    mut skill: libc::c_float,
    mut team: *const libc::c_char,
    mut delay: libc::c_int,
    mut altname: *mut libc::c_char,
) {
    let mut clientNum: libc::c_int = 0;
    let mut teamNum: libc::c_int = 0;
    let mut botinfoNum: libc::c_int = 0;
    let mut botinfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut botname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut headmodel: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    clientNum = trap_BotAllocateClient();
    if clientNum == -1i32 {
        G_Printf(
            b"^1Unable to add bot. All player slots are in use.\n\x00" as *const u8
                as *const libc::c_char,
        );
        G_Printf(b"^1Start server with more \'open\' slots (or check setting of sv_maxclients cvar).\n\x00"
                     as *const u8 as *const libc::c_char);
        return;
    }
    if team.is_null() || 0 == *team {
        if g_gametype.integer >= GT_TEAM as libc::c_int {
            if PickTeam(clientNum) as libc::c_uint == TEAM_RED as libc::c_int as libc::c_uint {
                team = b"red\x00" as *const u8 as *const libc::c_char
            } else {
                team = b"blue\x00" as *const u8 as *const libc::c_char
            }
        } else {
            team = b"free\x00" as *const u8 as *const libc::c_char
        }
    }
    if Q_stricmp(name, b"random\x00" as *const u8 as *const libc::c_char) == 0i32 {
        if Q_stricmp(team, b"red\x00" as *const u8 as *const libc::c_char) == 0i32
            || Q_stricmp(team, b"r\x00" as *const u8 as *const libc::c_char) == 0i32
        {
            teamNum = TEAM_RED as libc::c_int
        } else if Q_stricmp(team, b"blue\x00" as *const u8 as *const libc::c_char) == 0i32
            || Q_stricmp(team, b"b\x00" as *const u8 as *const libc::c_char) == 0i32
        {
            teamNum = TEAM_BLUE as libc::c_int
        } else if 0 == Q_stricmp(team, b"spectator\x00" as *const u8 as *const libc::c_char)
            || 0 == Q_stricmp(team, b"s\x00" as *const u8 as *const libc::c_char)
        {
            teamNum = TEAM_SPECTATOR as libc::c_int
        } else {
            teamNum = TEAM_FREE as libc::c_int
        }
        botinfoNum = G_SelectRandomBotInfo(teamNum);
        if botinfoNum < 0i32 {
            G_Printf(
                b"^1Error: Cannot add random bot, no bot info available.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            trap_BotFreeClient(clientNum);
            return;
        }
        botinfo = G_GetBotInfoByNumber(botinfoNum)
    } else {
        botinfo = G_GetBotInfoByName(name)
    }
    if botinfo.is_null() {
        G_Printf(
            b"^1Error: Bot \'%s\' not defined\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        trap_BotFreeClient(clientNum);
        return;
    }
    userinfo[0usize] = '\u{0}' as i32 as libc::c_char;
    botname = Info_ValueForKey(botinfo, b"funname\x00" as *const u8 as *const libc::c_char);
    if 0 == *botname.offset(0isize) {
        botname = Info_ValueForKey(botinfo, b"name\x00" as *const u8 as *const libc::c_char)
    }
    if !altname.is_null() && 0 != *altname.offset(0isize) as libc::c_int {
        botname = altname
    }
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"name\x00" as *const u8 as *const libc::c_char,
        botname,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"rate\x00" as *const u8 as *const libc::c_char,
        b"25000\x00" as *const u8 as *const libc::c_char,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"snaps\x00" as *const u8 as *const libc::c_char,
        b"20\x00" as *const u8 as *const libc::c_char,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"skill\x00" as *const u8 as *const libc::c_char,
        va(
            b"%.2f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skill as libc::c_double,
        ),
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teampref\x00" as *const u8 as *const libc::c_char,
        team,
    );
    if skill >= 1i32 as libc::c_float && skill < 2i32 as libc::c_float {
        Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const libc::c_char,
            b"50\x00" as *const u8 as *const libc::c_char,
        );
    } else if skill >= 2i32 as libc::c_float && skill < 3i32 as libc::c_float {
        Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const libc::c_char,
            b"70\x00" as *const u8 as *const libc::c_char,
        );
    } else if skill >= 3i32 as libc::c_float && skill < 4i32 as libc::c_float {
        Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const libc::c_char,
            b"90\x00" as *const u8 as *const libc::c_char,
        );
    }
    key = b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    model = Info_ValueForKey(botinfo, key);
    if 0 == *model {
        model = b"visor/default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, model);
    key = b"team_model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, model);
    key = b"headmodel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    headmodel = Info_ValueForKey(botinfo, key);
    if 0 == *headmodel {
        headmodel = model
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, headmodel);
    key = b"team_headmodel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, headmodel);
    key = b"gender\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s = Info_ValueForKey(botinfo, key);
    if 0 == *s {
        s = b"male\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"sex\x00" as *const u8 as *const libc::c_char,
        s,
    );
    key = b"color1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s = Info_ValueForKey(botinfo, key);
    if 0 == *s {
        s = b"4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, s);
    key = b"color2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s = Info_ValueForKey(botinfo, key);
    if 0 == *s {
        s = b"5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Info_SetValueForKey(userinfo.as_mut_ptr(), key, s);
    s = Info_ValueForKey(botinfo, b"aifile\x00" as *const u8 as *const libc::c_char);
    if 0 == *s {
        trap_Print(
            b"^1Error: bot has no aifile specified\n\x00" as *const u8 as *const libc::c_char,
        );
        trap_BotFreeClient(clientNum);
        return;
    }
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"characterfile\x00" as *const u8 as *const libc::c_char,
        s,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teamoverlay\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
    trap_SetUserinfo(clientNum, userinfo.as_mut_ptr());
    if !ClientConnect(clientNum, qtrue, qtrue).is_null() {
        return;
    }
    if delay == 0i32 {
        ClientBegin(clientNum);
        return;
    }
    AddBotToSpawnQueue(clientNum, delay);
}
/*
===============
AddBotToSpawnQueue
===============
*/
unsafe extern "C" fn AddBotToSpawnQueue(mut clientNum: libc::c_int, mut delay: libc::c_int) {
    let mut n: libc::c_int = 0;
    n = 0i32;
    while n < 16i32 {
        if 0 == botSpawnQueue[n as usize].spawnTime {
            botSpawnQueue[n as usize].spawnTime = level.time + delay;
            botSpawnQueue[n as usize].clientNum = clientNum;
            return;
        }
        n += 1
    }
    G_Printf(b"^3Unable to delay spawn\n\x00" as *const u8 as *const libc::c_char);
    ClientBegin(clientNum);
}
/*
===============
G_SelectRandomBotInfo

Get random least used bot info on team or whole server if team is -1.
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_SelectRandomBotInfo(mut team: libc::c_int) -> libc::c_int {
    let mut selection: [libc::c_int; 1024] = [0; 1024];
    let mut n: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut bestCount: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if team != -1i32 && G_CountBotPlayersByName(0 as *const libc::c_char, -1i32) < g_numBots {
        team = -1i32
    }
    num = 0i32;
    bestCount = 64i32;
    n = 0i32;
    while n < g_numBots {
        value = Info_ValueForKey(
            g_botInfos[n as usize],
            b"funname\x00" as *const u8 as *const libc::c_char,
        );
        if 0 == *value.offset(0isize) {
            value = Info_ValueForKey(
                g_botInfos[n as usize],
                b"name\x00" as *const u8 as *const libc::c_char,
            )
        }
        count = G_CountBotPlayersByName(value, team);
        if count < bestCount {
            bestCount = count;
            num = 0i32
        }
        if count == bestCount {
            let fresh3 = num;
            num = num + 1;
            selection[fresh3 as usize] = n;
            if num == 1024i32 {
                break;
            }
        }
        n += 1
    }
    if num > 0i32 {
        num = ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
            * (num - 1i32) as libc::c_float) as libc::c_int;
        return selection[num as usize];
    }
    return -1i32;
}
/*
===============
G_CountBotPlayersByName

Check connected and connecting (delay join) bots.

Returns number of bots with name on specified team or whole server if team is -1.
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_CountBotPlayersByName(
    mut name: *const libc::c_char,
    mut team: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    num = 0i32;
    i = 0i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as libc::c_uint
            == CON_DISCONNECTED as libc::c_int as libc::c_uint)
        {
            if !(0 == g_entities[i as usize].r.svFlags & 0x8i32) {
                if !(team >= 0i32 && (*cl).sess.sessionTeam as libc::c_uint != team as libc::c_uint)
                {
                    if !(!name.is_null() && 0 != Q_stricmp(name, (*cl).pers.netname.as_mut_ptr())) {
                        num += 1
                    }
                }
            }
        }
        i += 1
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn Svcmd_BotList_f() {
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 1024] = [0; 1024];
    let mut funname: [libc::c_char; 1024] = [0; 1024];
    let mut model: [libc::c_char; 1024] = [0; 1024];
    let mut aifile: [libc::c_char; 1024] = [0; 1024];
    trap_Print(
        b"^1name             model            aifile              funname\n\x00" as *const u8
            as *const libc::c_char,
    );
    i = 0i32;
    while i < g_numBots {
        Q_strncpyz(
            name.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"name\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == *name.as_mut_ptr() {
            strcpy(
                name.as_mut_ptr(),
                b"UnnamedPlayer\x00" as *const u8 as *const libc::c_char,
            );
        }
        Q_strncpyz(
            funname.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"funname\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == *funname.as_mut_ptr() {
            strcpy(
                funname.as_mut_ptr(),
                b"\x00" as *const u8 as *const libc::c_char,
            );
        }
        Q_strncpyz(
            model.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"model\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == *model.as_mut_ptr() {
            strcpy(
                model.as_mut_ptr(),
                b"visor/default\x00" as *const u8 as *const libc::c_char,
            );
        }
        Q_strncpyz(
            aifile.as_mut_ptr(),
            Info_ValueForKey(
                g_botInfos[i as usize],
                b"aifile\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == *aifile.as_mut_ptr() {
            strcpy(
                aifile.as_mut_ptr(),
                b"bots/default_c.c\x00" as *const u8 as *const libc::c_char,
            );
        }
        trap_Print(va(
            b"%-16s %-16s %-20s %-20s\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
            model.as_mut_ptr(),
            aifile.as_mut_ptr(),
            funname.as_mut_ptr(),
        ));
        i += 1
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct botSpawnQueue_t {
    pub clientNum: libc::c_int,
    pub spawnTime: libc::c_int,
}
