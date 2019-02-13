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
    gitem_s, gitem_t, itemType_t, team_t, unnamed_1, unnamed_2, unnamed_3, unnamed_4, unnamed_5,
    GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER,
    GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, MOD_BFG, MOD_BFG_SPLASH, MOD_CRUSH,
    MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE, MOD_GRENADE, MOD_GRENADE_SPLASH, MOD_LAVA,
    MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA, MOD_PLASMA_SPLASH, MOD_RAILGUN, MOD_ROCKET,
    MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME, MOD_SUICIDE, MOD_TARGET_LASER, MOD_TELEFRAG,
    MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR,
    PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, STAT_ARMOR, STAT_CLIENTS_READY,
    STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE,
    TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK,
    WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN,
    WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
use g_items::{
    ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem, G_SpawnItem,
    RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
};
use g_local_h::{
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gclient_t, gentity_s,
    gentity_t, level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t,
    spectatorState_t, trap_Argc, trap_Argv, trap_Cvar_VariableStringBuffer, trap_GetUserinfo,
    trap_SendConsoleCommand, trap_SendServerCommand, trap_SetConfigstring, trap_SetUserinfo,
    CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1, MOVER_POS1,
    MOVER_POS2, SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE,
    TEAM_BEGIN,
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
    SP_team_CTF_redspawn, Team_CheckDroppedItem, Team_GetLocationMsg,
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
use g_variadic_h::{G_Error, G_LogPrintf, G_Printf};
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
    unnamed_0, usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, Com_sprintf, Info_SetValueForKey,
    Q_CleanStr, Q_stricmp, Q_stricmpn, Q_strncpyz, EXEC_APPEND, EXEC_INSERT, EXEC_NOW, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{
    _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower, _ISprint, _ISpunct,
    _ISspace, _ISupper, _ISxdigit, __ctype_b_loc, atof, atoi, memcpy, memset, strcat, strcpy,
    strlen, tolower, unnamed,
};

//
// g_cmds.c
//
#[no_mangle]
pub unsafe extern "C" fn Cmd_Score_f(mut ent: *mut gentity_t) {
    DeathmatchScoreboardMessage(ent);
}
//
// g_cmds.c
//
#[no_mangle]
pub unsafe extern "C" fn DeathmatchScoreboardMessage(mut ent: *mut gentity_t) {
    let mut entry: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 1000] = [0; 1000];
    let mut stringlength: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut numSorted: libc::c_int = 0;
    let mut scoreFlags: libc::c_int = 0;
    let mut accuracy: libc::c_int = 0;
    let mut perfect: libc::c_int = 0;
    if 0 != (*ent).r.svFlags & 0x8i32 {
        return;
    }
    string[0usize] = 0i32 as libc::c_char;
    stringlength = 0i32;
    scoreFlags = 0i32;
    numSorted = level.numConnectedClients;
    i = 0i32;
    while i < numSorted {
        let mut ping: libc::c_int = 0;
        cl = &mut *level
            .clients
            .offset(level.sortedClients[i as usize] as isize) as *mut gclient_s;
        if (*cl).pers.connected as libc::c_uint == CON_CONNECTING as libc::c_int as libc::c_uint {
            ping = -1i32
        } else {
            ping = if (*cl).ps.ping < 999i32 {
                (*cl).ps.ping
            } else {
                999i32
            }
        }
        if 0 != (*cl).accuracy_shots {
            accuracy = (*cl).accuracy_hits * 100i32 / (*cl).accuracy_shots
        } else {
            accuracy = 0i32
        }
        perfect = if (*cl).ps.persistant[PERS_RANK as libc::c_int as usize] == 0i32
            && (*cl).ps.persistant[PERS_KILLED as libc::c_int as usize] == 0i32
        {
            1i32
        } else {
            0i32
        };
        Com_sprintf(
            entry.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b" %i %i %i %i %i %i %i %i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char,
            level.sortedClients[i as usize],
            (*cl).ps.persistant[PERS_SCORE as libc::c_int as usize],
            ping,
            (level.time - (*cl).pers.enterTime) / 60000i32,
            scoreFlags,
            g_entities[level.sortedClients[i as usize] as usize]
                .s
                .powerups,
            accuracy,
            (*cl).ps.persistant[PERS_IMPRESSIVE_COUNT as libc::c_int as usize],
            (*cl).ps.persistant[PERS_EXCELLENT_COUNT as libc::c_int as usize],
            (*cl).ps.persistant[PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize],
            (*cl).ps.persistant[PERS_DEFEND_COUNT as libc::c_int as usize],
            (*cl).ps.persistant[PERS_ASSIST_COUNT as libc::c_int as usize],
            perfect,
            (*cl).ps.persistant[PERS_CAPTURES as libc::c_int as usize],
        );
        j = strlen(entry.as_mut_ptr()) as libc::c_int;
        if (stringlength + j) as libc::c_ulong
            >= ::std::mem::size_of::<[libc::c_char; 1000]>() as libc::c_ulong
        {
            break;
        }
        strcpy(
            string.as_mut_ptr().offset(stringlength as isize),
            entry.as_mut_ptr(),
        );
        stringlength += j;
        i += 1
    }
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"scores %i %i %i%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
            level.teamScores[TEAM_RED as libc::c_int as usize],
            level.teamScores[TEAM_BLUE as libc::c_int as usize],
            string.as_mut_ptr(),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn StopFollowing(mut ent: *mut gentity_t) {
    (*(*ent).client).ps.persistant[PERS_TEAM as libc::c_int as usize] =
        TEAM_SPECTATOR as libc::c_int;
    (*(*ent).client).sess.sessionTeam = TEAM_SPECTATOR;
    (*(*ent).client).sess.spectatorState = SPECTATOR_FREE;
    (*(*ent).client).ps.pm_flags &= !4096i32;
    (*ent).r.svFlags &= !0x8i32;
    (*(*ent).client).ps.clientNum =
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int;
    SetClientViewAngle(ent, (*(*ent).client).ps.viewangles.as_mut_ptr());
    if (*(*ent).client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        (*(*ent).client).ps.stats[STAT_HEALTH as libc::c_int as usize] = 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn BroadcastTeamChange(mut client: *mut gclient_t, mut oldTeam: libc::c_int) {
    if (*client).sess.sessionTeam as libc::c_uint == TEAM_RED as libc::c_int as libc::c_uint {
        trap_SendServerCommand(
            -1i32,
            va(
                b"cp \"%s^7 joined the red team.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    } else if (*client).sess.sessionTeam as libc::c_uint == TEAM_BLUE as libc::c_int as libc::c_uint
    {
        trap_SendServerCommand(
            -1i32,
            va(
                b"cp \"%s^7 joined the blue team.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    } else if (*client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
        && oldTeam != TEAM_SPECTATOR as libc::c_int
    {
        trap_SendServerCommand(
            -1i32,
            va(
                b"cp \"%s^7 joined the spectators.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    } else if (*client).sess.sessionTeam as libc::c_uint == TEAM_FREE as libc::c_int as libc::c_uint
    {
        trap_SendServerCommand(
            -1i32,
            va(
                b"cp \"%s^7 joined the battle.\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SetTeam(mut ent: *mut gentity_t, mut s: *const libc::c_char) {
    let mut team: libc::c_int = 0;
    let mut oldTeam: libc::c_int = 0;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut clientNum: libc::c_int = 0;
    let mut specState: spectatorState_t = SPECTATOR_NOT;
    let mut specClient: libc::c_int = 0;
    let mut teamLeader: libc::c_int = 0;
    client = (*ent).client;
    clientNum = client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int;
    specClient = 0i32;
    specState = SPECTATOR_NOT;
    if 0 == Q_stricmp(s, b"scoreboard\x00" as *const u8 as *const libc::c_char)
        || 0 == Q_stricmp(s, b"score\x00" as *const u8 as *const libc::c_char)
    {
        team = TEAM_SPECTATOR as libc::c_int;
        specState = SPECTATOR_SCOREBOARD
    } else if 0 == Q_stricmp(s, b"follow1\x00" as *const u8 as *const libc::c_char) {
        team = TEAM_SPECTATOR as libc::c_int;
        specState = SPECTATOR_FOLLOW;
        specClient = -1i32
    } else if 0 == Q_stricmp(s, b"follow2\x00" as *const u8 as *const libc::c_char) {
        team = TEAM_SPECTATOR as libc::c_int;
        specState = SPECTATOR_FOLLOW;
        specClient = -2i32
    } else if 0 == Q_stricmp(s, b"spectator\x00" as *const u8 as *const libc::c_char)
        || 0 == Q_stricmp(s, b"s\x00" as *const u8 as *const libc::c_char)
    {
        team = TEAM_SPECTATOR as libc::c_int;
        specState = SPECTATOR_FREE
    } else if g_gametype.integer >= GT_TEAM as libc::c_int {
        specState = SPECTATOR_NOT;
        if 0 == Q_stricmp(s, b"red\x00" as *const u8 as *const libc::c_char)
            || 0 == Q_stricmp(s, b"r\x00" as *const u8 as *const libc::c_char)
        {
            team = TEAM_RED as libc::c_int
        } else if 0 == Q_stricmp(s, b"blue\x00" as *const u8 as *const libc::c_char)
            || 0 == Q_stricmp(s, b"b\x00" as *const u8 as *const libc::c_char)
        {
            team = TEAM_BLUE as libc::c_int
        } else {
            team = PickTeam(clientNum) as libc::c_int
        }
        if 0 != g_teamForceBalance.integer
            && 0 == (*client).pers.localClient as u64
            && 0 == (*ent).r.svFlags & 0x8i32
        {
            let mut counts: [libc::c_int; 4] = [0; 4];
            counts[TEAM_BLUE as libc::c_int as usize] = TeamCount(clientNum, TEAM_BLUE);
            counts[TEAM_RED as libc::c_int as usize] = TeamCount(clientNum, TEAM_RED);
            if team == TEAM_RED as libc::c_int
                && counts[TEAM_RED as libc::c_int as usize]
                    - counts[TEAM_BLUE as libc::c_int as usize]
                    > 1i32
            {
                trap_SendServerCommand(
                    clientNum,
                    b"cp \"Red team has too many players.\n\"\x00" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            if team == TEAM_BLUE as libc::c_int
                && counts[TEAM_BLUE as libc::c_int as usize]
                    - counts[TEAM_RED as libc::c_int as usize]
                    > 1i32
            {
                trap_SendServerCommand(
                    clientNum,
                    b"cp \"Blue team has too many players.\n\"\x00" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
        }
    } else {
        team = TEAM_FREE as libc::c_int
    }
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int && level.numNonSpectatorClients >= 2i32 {
        team = TEAM_SPECTATOR as libc::c_int
    } else if g_maxGameClients.integer > 0i32
        && level.numNonSpectatorClients >= g_maxGameClients.integer
    {
        team = TEAM_SPECTATOR as libc::c_int
    }
    oldTeam = (*client).sess.sessionTeam as libc::c_int;
    if team == oldTeam && team != TEAM_SPECTATOR as libc::c_int {
        return;
    }
    if (*client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32
        && (*client).pers.connected as libc::c_uint == CON_CONNECTED as libc::c_int as libc::c_uint
    {
        CopyToBodyQue(ent);
    }
    (*client).pers.teamState.state = TEAM_BEGIN;
    if oldTeam != TEAM_SPECTATOR as libc::c_int {
        (*ent).flags &= !0x10i32;
        (*ent).health = 0i32;
        (*(*ent).client).ps.stats[STAT_HEALTH as libc::c_int as usize] = (*ent).health;
        player_die(ent, ent, ent, 100000i32, MOD_SUICIDE as libc::c_int);
    }
    if team == TEAM_SPECTATOR as libc::c_int && oldTeam != team {
        AddTournamentQueue(client);
    }
    (*client).sess.sessionTeam = team as team_t;
    (*client).sess.spectatorState = specState;
    (*client).sess.spectatorClient = specClient;
    (*client).sess.teamLeader = qfalse;
    if team == TEAM_RED as libc::c_int || team == TEAM_BLUE as libc::c_int {
        teamLeader = TeamLeader(team);
        if teamLeader == -1i32
            || 0 == g_entities[clientNum as usize].r.svFlags & 0x8i32
                && 0 != g_entities[teamLeader as usize].r.svFlags & 0x8i32
        {
            SetLeader(team, clientNum);
        }
    }
    if oldTeam == TEAM_RED as libc::c_int || oldTeam == TEAM_BLUE as libc::c_int {
        CheckTeamLeader(oldTeam);
    }
    BroadcastTeamChange(client, oldTeam);
    ClientUserinfoChanged(clientNum);
    if (*client).pers.connected as libc::c_uint != CON_CONNECTED as libc::c_int as libc::c_uint {
        return;
    }
    ClientBegin(clientNum);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_FollowCycle_f(mut ent: *mut gentity_t, mut dir: libc::c_int) {
    let mut clientnum: libc::c_int = 0;
    let mut original: libc::c_int = 0;
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int
        && (*(*ent).client).sess.sessionTeam as libc::c_uint
            == TEAM_FREE as libc::c_int as libc::c_uint
    {
        (*(*ent).client).sess.losses += 1
    }
    if (*(*ent).client).sess.spectatorState as libc::c_uint
        == SPECTATOR_NOT as libc::c_int as libc::c_uint
    {
        SetTeam(ent, b"spectator\x00" as *const u8 as *const libc::c_char);
    }
    if dir != 1i32 && dir != -1i32 {
        G_Error(
            b"Cmd_FollowCycle_f: bad dir %i\x00" as *const u8 as *const libc::c_char,
            dir,
        );
    }
    if (*(*ent).client).sess.spectatorClient < 0i32 {
        if (*(*ent).client).sess.spectatorClient == -1i32 {
            (*(*ent).client).sess.spectatorClient = -2i32
        } else if (*(*ent).client).sess.spectatorClient == -2i32 {
            (*(*ent).client).sess.spectatorClient = -1i32
        }
        return;
    }
    clientnum = (*(*ent).client).sess.spectatorClient;
    original = clientnum;
    loop {
        clientnum += dir;
        if clientnum >= level.maxclients {
            clientnum = 0i32
        }
        if clientnum < 0i32 {
            clientnum = level.maxclients - 1i32
        }
        // can only follow connected clients
        if !((*level.clients.offset(clientnum as isize)).pers.connected as libc::c_uint
            != CON_CONNECTED as libc::c_int as libc::c_uint)
        {
            // can't follow another spectator
            if !((*level.clients.offset(clientnum as isize)).sess.sessionTeam as libc::c_uint
                == TEAM_SPECTATOR as libc::c_int as libc::c_uint)
            {
                (*(*ent).client).sess.spectatorClient = clientnum;
                (*(*ent).client).sess.spectatorState = SPECTATOR_FOLLOW;
                return;
            }
        }
        if !(clientnum != original) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ClientCommand(mut clientNum: libc::c_int) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    ent = g_entities.as_mut_ptr().offset(clientNum as isize);
    if (*ent).client.is_null()
        || (*(*ent).client).pers.connected as libc::c_uint
            != CON_CONNECTED as libc::c_int as libc::c_uint
    {
        if !(*ent).client.is_null() && 0 != (*(*ent).client).pers.localClient as libc::c_uint {
            trap_Argv(
                0i32,
                cmd.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            if Q_stricmp(
                cmd.as_mut_ptr(),
                b"team\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                Cmd_Team_f(ent);
            }
        }
        return;
    }
    trap_Argv(
        0i32,
        cmd.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"say\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Say_f(ent, 0i32, qfalse);
        return;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"say_team\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Say_f(ent, 1i32, qfalse);
        return;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"tell\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Tell_f(ent);
        return;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"score\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Score_f(ent);
        return;
    }
    if 0 != level.intermissiontime {
        Cmd_Say_f(ent, qfalse as libc::c_int, qtrue);
        return;
    }
    if Q_stricmp(
        cmd.as_mut_ptr(),
        b"give\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Give_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"god\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_God_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"notarget\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Notarget_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"noclip\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Noclip_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"kill\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Kill_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"teamtask\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_TeamTask_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"levelshot\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_LevelShot_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"follow\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Follow_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"follownext\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_FollowCycle_f(ent, 1i32);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"followprev\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_FollowCycle_f(ent, -1i32);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"team\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Team_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"where\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Where_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"callvote\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_CallVote_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"vote\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Vote_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"callteamvote\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_CallTeamVote_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"teamvote\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_TeamVote_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"gc\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_GameCommand_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"setviewpos\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_SetViewpos_f(ent);
    } else if Q_stricmp(
        cmd.as_mut_ptr(),
        b"stats\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        Cmd_Stats_f(ent);
    } else {
        trap_SendServerCommand(
            clientNum,
            va(
                b"print \"unknown cmd %s\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cmd.as_mut_ptr(),
            ),
        );
    };
}
/*
=================
Cmd_Stats_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Stats_f(mut ent: *mut gentity_t) {}
// a majority will be determined in TeamCheckVote, which will also account
// for players entering or leaving
/*
=================
Cmd_SetViewpos_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_SetViewpos_f(mut ent: *mut gentity_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    if 0 == g_cheats.integer {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Cheats are not enabled on this server.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if trap_Argc() != 5i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"usage: setviewpos x y z yaw\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    angles[2usize] = 0i32 as vec_t;
    angles[1usize] = angles[2usize];
    angles[0usize] = angles[1usize];
    i = 0i32;
    while i < 3i32 {
        trap_Argv(
            i + 1i32,
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        origin[i as usize] = atof(buffer.as_mut_ptr()) as vec_t;
        i += 1
    }
    trap_Argv(
        4i32,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    angles[1usize] = atof(buffer.as_mut_ptr()) as vec_t;
    TeleportPlayer(ent, origin.as_mut_ptr(), angles.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_GameCommand_f(mut ent: *mut gentity_t) {
    let mut targetNum: libc::c_int = 0;
    let mut target: *mut gentity_t = 0 as *mut gentity_t;
    let mut order: libc::c_int = 0;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() != 3i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            va(
                b"print \"Usage: gc <player id> <order 0-%d>\n\"\x00" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                numgc_orders - 1i32,
            ),
        );
        return;
    }
    trap_Argv(
        2i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    order = atoi(arg.as_mut_ptr());
    if order < 0i32 || order >= numgc_orders {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            va(
                b"print \"Bad order: %i\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                order,
            ),
        );
        return;
    }
    trap_Argv(
        1i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    targetNum = ClientNumberFromString(ent, arg.as_mut_ptr(), qtrue, qtrue);
    if targetNum == -1i32 {
        return;
    }
    target = &mut g_entities[targetNum as usize] as *mut gentity_t;
    if 0 == (*target).inuse as u64 || (*target).client.is_null() {
        return;
    }
    G_LogPrintf(
        b"tell: %s to %s: %s\n\x00" as *const u8 as *const libc::c_char,
        (*(*ent).client).pers.netname.as_mut_ptr(),
        (*(*target).client).pers.netname.as_mut_ptr(),
        gc_orders[order as usize],
    );
    G_Say(ent, target, 2i32, gc_orders[order as usize]);
    if ent != target && 0 == (*ent).r.svFlags & 0x8i32 {
        G_Say(ent, ent, 2i32, gc_orders[order as usize]);
    };
}
static mut gc_orders: [*mut libc::c_char; 7] = [
    b"hold your position\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hold this position\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"come here\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"cover me\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"guard location\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"search and destroy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"report\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn G_Say(
    mut ent: *mut gentity_t,
    mut target: *mut gentity_t,
    mut mode: libc::c_int,
    mut chatText: *const libc::c_char,
) {
    let mut j: libc::c_int = 0;
    let mut other: *mut gentity_t = 0 as *mut gentity_t;
    let mut color: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    // don't let text be too long for malicious reasons
    let mut text: [libc::c_char; 150] = [0; 150];
    let mut location: [libc::c_char; 64] = [0; 64];
    if g_gametype.integer < GT_TEAM as libc::c_int && mode == 1i32 {
        mode = 0i32
    }
    match mode {
        1 => {
            G_LogPrintf(
                b"sayteam: %s: %s\n\x00" as *const u8 as *const libc::c_char,
                (*(*ent).client).pers.netname.as_mut_ptr(),
                chatText,
            );
            if 0 != Team_GetLocationMsg(
                ent,
                location.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            ) as u64
            {
                Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    b"\x19(%s%c%c\x19) (%s)\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                    location.as_mut_ptr(),
                );
            } else {
                Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    b"\x19(%s%c%c\x19)\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                );
            }
            color = '5' as i32
        }
        2 => {
            if !target.is_null()
                && 0 != (*target).inuse as libc::c_uint
                && !(*target).client.is_null()
                && g_gametype.integer >= GT_TEAM as libc::c_int
                && (*(*target).client).sess.sessionTeam as libc::c_uint
                    == (*(*ent).client).sess.sessionTeam as libc::c_uint
                && 0 != Team_GetLocationMsg(
                    ent,
                    location.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                ) as libc::c_uint
            {
                Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    b"\x19[%s%c%c\x19] (%s)\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                    location.as_mut_ptr(),
                );
            } else {
                Com_sprintf(
                    name.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    b"\x19[%s%c%c\x19]\x19: \x00" as *const u8 as *const libc::c_char,
                    (*(*ent).client).pers.netname.as_mut_ptr(),
                    '^' as i32,
                    '7' as i32,
                );
            }
            color = '6' as i32
        }
        0 | _ => {
            G_LogPrintf(
                b"say: %s: %s\n\x00" as *const u8 as *const libc::c_char,
                (*(*ent).client).pers.netname.as_mut_ptr(),
                chatText,
            );
            Com_sprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                b"%s%c%c\x19: \x00" as *const u8 as *const libc::c_char,
                (*(*ent).client).pers.netname.as_mut_ptr(),
                '^' as i32,
                '7' as i32,
            );
            color = '2' as i32
        }
    }
    Q_strncpyz(
        text.as_mut_ptr(),
        chatText,
        ::std::mem::size_of::<[libc::c_char; 150]>() as libc::c_ulong as libc::c_int,
    );
    if !target.is_null() {
        G_SayTo(
            ent,
            target,
            mode,
            color,
            name.as_mut_ptr(),
            text.as_mut_ptr(),
        );
        return;
    }
    if 0 != g_dedicated.integer {
        G_Printf(
            b"%s%s\n\x00" as *const u8 as *const libc::c_char,
            name.as_mut_ptr(),
            text.as_mut_ptr(),
        );
    }
    j = 0i32;
    while j < level.maxclients {
        other = &mut g_entities[j as usize] as *mut gentity_t;
        G_SayTo(
            ent,
            other,
            mode,
            color,
            name.as_mut_ptr(),
            text.as_mut_ptr(),
        );
        j += 1
    }
}
// leave it where it was
/*
==================
G_Say
==================
*/
unsafe extern "C" fn G_SayTo(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut mode: libc::c_int,
    mut color: libc::c_int,
    mut name: *const libc::c_char,
    mut message: *const libc::c_char,
) {
    if other.is_null() {
        return;
    }
    if 0 == (*other).inuse as u64 {
        return;
    }
    if (*other).client.is_null() {
        return;
    }
    if (*(*other).client).pers.connected as libc::c_uint
        != CON_CONNECTED as libc::c_int as libc::c_uint
    {
        return;
    }
    if mode == 1i32 && 0 == OnSameTeam(ent, other) as u64 {
        return;
    }
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int
        && (*(*other).client).sess.sessionTeam as libc::c_uint
            == TEAM_FREE as libc::c_int as libc::c_uint
        && (*(*ent).client).sess.sessionTeam as libc::c_uint
            != TEAM_FREE as libc::c_int as libc::c_uint
    {
        return;
    }
    trap_SendServerCommand(
        other.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"%s \"%s%c%c%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            if mode == 1i32 {
                b"tchat\x00" as *const u8 as *const libc::c_char
            } else {
                b"chat\x00" as *const u8 as *const libc::c_char
            },
            name,
            '^' as i32,
            color,
            message,
        ),
    );
}
/*
==================
ClientNumberFromString

Returns a player number for either a number or name string
Returns -1 if invalid
==================
*/
#[no_mangle]
pub unsafe extern "C" fn ClientNumberFromString(
    mut to: *mut gentity_t,
    mut s: *mut libc::c_char,
    mut checkNums: qboolean,
    mut checkNames: qboolean,
) -> libc::c_int {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut idnum: libc::c_int = 0;
    let mut cleanName: [libc::c_char; 1024] = [0; 1024];
    if 0 != checkNums as u64 {
        if 0 != StringIsInteger(s) as u64 {
            idnum = atoi(s);
            if idnum >= 0i32 && idnum < level.maxclients {
                cl = &mut *level.clients.offset(idnum as isize) as *mut gclient_s;
                if (*cl).pers.connected as libc::c_uint
                    == CON_CONNECTED as libc::c_int as libc::c_uint
                {
                    return idnum;
                }
            }
        }
    }
    if 0 != checkNames as u64 {
        idnum = 0i32;
        cl = level.clients;
        while idnum < level.maxclients {
            if !((*cl).pers.connected as libc::c_uint
                != CON_CONNECTED as libc::c_int as libc::c_uint)
            {
                Q_strncpyz(
                    cleanName.as_mut_ptr(),
                    (*cl).pers.netname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                );
                Q_CleanStr(cleanName.as_mut_ptr());
                if 0 == Q_stricmp(cleanName.as_mut_ptr(), s) {
                    return idnum;
                }
            }
            idnum += 1;
            cl = cl.offset(1isize)
        }
    }
    trap_SendServerCommand(
        to.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"print \"User %s is not on the server\n\"\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
        ),
    );
    return -1i32;
}
/*
==================
StringIsInteger
==================
*/
#[no_mangle]
pub unsafe extern "C" fn StringIsInteger(mut s: *const libc::c_char) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut foundDigit: qboolean = qfalse;
    len = strlen(s) as libc::c_int;
    foundDigit = qfalse;
    i = 0i32;
    while i < len {
        if 0 == *(*__ctype_b_loc()).offset(*s.offset(i as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        {
            return qfalse;
        }
        foundDigit = qtrue;
        i += 1
    }
    return foundDigit;
}
// Initialized in run_static_initializers
static mut numgc_orders: libc::c_int = 0;
/*
==================
Cmd_TeamVote_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_TeamVote_f(mut ent: *mut gentity_t) {
    let mut team: libc::c_int = 0;
    let mut cs_offset: libc::c_int = 0;
    let mut msg: [libc::c_char; 64] = [0; 64];
    team = (*(*ent).client).sess.sessionTeam as libc::c_int;
    if team == TEAM_RED as libc::c_int {
        cs_offset = 0i32
    } else if team == TEAM_BLUE as libc::c_int {
        cs_offset = 1i32
    } else {
        return;
    }
    if 0 == level.teamVoteTime[cs_offset as usize] {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"No team vote in progress.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if 0 != (*(*ent).client).ps.eFlags & 0x80000i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Team vote already cast.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Not allowed to vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        b"print \"Team vote cast.\n\"\x00" as *const u8 as *const libc::c_char,
    );
    (*(*ent).client).ps.eFlags |= 0x80000i32;
    trap_Argv(
        1i32,
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    if tolower(msg[0usize] as libc::c_int) == 'y' as i32 || msg[0usize] as libc::c_int == '1' as i32
    {
        level.teamVoteYes[cs_offset as usize] += 1;
        trap_SetConfigstring(
            16i32 + cs_offset,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.teamVoteYes[cs_offset as usize],
            ),
        );
    } else {
        level.teamVoteNo[cs_offset as usize] += 1;
        trap_SetConfigstring(
            18i32 + cs_offset,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.teamVoteNo[cs_offset as usize],
            ),
        );
    };
}
// a majority will be determined in CheckVote, which will also account
// for players entering or leaving
/*
==================
Cmd_CallTeamVote_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_CallTeamVote_f(mut ent: *mut gentity_t) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut team: libc::c_int = 0;
    let mut cs_offset: libc::c_int = 0;
    let mut arg1: [libc::c_char; 1024] = [0; 1024];
    let mut arg2: [libc::c_char; 1024] = [0; 1024];
    team = (*(*ent).client).sess.sessionTeam as libc::c_int;
    if team == TEAM_RED as libc::c_int {
        cs_offset = 0i32
    } else if team == TEAM_BLUE as libc::c_int {
        cs_offset = 1i32
    } else {
        return;
    }
    if 0 == g_allowVote.integer {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Voting not allowed here.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if 0 != level.teamVoteTime[cs_offset as usize] {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"A team vote is already in progress.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).pers.teamVoteCount >= 3i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"You have called the maximum number of team votes.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Not allowed to call a vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    trap_Argv(
        1i32,
        arg1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    arg2[0usize] = '\u{0}' as i32 as libc::c_char;
    i = 2i32;
    while i < trap_Argc() {
        if i > 2i32 {
            strcat(
                arg2.as_mut_ptr(),
                b" \x00" as *const u8 as *const libc::c_char,
            );
        }
        trap_Argv(
            i,
            &mut arg2[strlen(arg2.as_mut_ptr()) as usize],
            (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(strlen(arg2.as_mut_ptr())) as libc::c_int,
        );
        i += 1
    }
    c = arg2.as_mut_ptr();
    while 0 != *c {
        match *c as libc::c_int {
            10 | 13 | 59 => {
                trap_SendServerCommand(
                    ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                        as libc::c_int,
                    b"print \"Invalid vote string.\n\"\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
            _ => {}
        }
        c = c.offset(1isize)
    }
    if 0 == Q_stricmp(
        arg1.as_mut_ptr(),
        b"leader\x00" as *const u8 as *const libc::c_char,
    ) {
        let mut netname: [libc::c_char; 36] = [0; 36];
        let mut leader: [libc::c_char; 36] = [0; 36];
        if 0 == arg2[0usize] {
            i = (*(*ent).client).ps.clientNum
        } else {
            i = 0i32;
            while i < 3i32 {
                if 0 == arg2[i as usize]
                    || (arg2[i as usize] as libc::c_int) < '0' as i32
                    || arg2[i as usize] as libc::c_int > '9' as i32
                {
                    break;
                }
                i += 1
            }
            if i >= 3i32 || 0 == arg2[i as usize] {
                i = atoi(arg2.as_mut_ptr());
                if i < 0i32 || i >= level.maxclients {
                    trap_SendServerCommand(
                        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                            as libc::c_int,
                        va(
                            b"print \"Bad client slot: %i\n\"\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            i,
                        ),
                    );
                    return;
                }
                if 0 == g_entities[i as usize].inuse as u64 {
                    trap_SendServerCommand(
                        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                            as libc::c_int,
                        va(
                            b"print \"Client %i is not active\n\"\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            i,
                        ),
                    );
                    return;
                }
            } else {
                Q_strncpyz(
                    leader.as_mut_ptr(),
                    arg2.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
                );
                Q_CleanStr(leader.as_mut_ptr());
                i = 0i32;
                while i < level.maxclients {
                    if !((*level.clients.offset(i as isize)).pers.connected as libc::c_uint
                        == CON_DISCONNECTED as libc::c_int as libc::c_uint)
                    {
                        if !((*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                            != team as libc::c_uint)
                        {
                            Q_strncpyz(
                                netname.as_mut_ptr(),
                                (*level.clients.offset(i as isize))
                                    .pers
                                    .netname
                                    .as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong
                                    as libc::c_int,
                            );
                            Q_CleanStr(netname.as_mut_ptr());
                            if 0 == Q_stricmp(netname.as_mut_ptr(), leader.as_mut_ptr()) {
                                break;
                            }
                        }
                    }
                    i += 1
                }
                if i >= level.maxclients {
                    trap_SendServerCommand(
                        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                            as libc::c_int,
                        va(
                            b"print \"%s is not a valid player on your team.\n\"\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            arg2.as_mut_ptr(),
                        ),
                    );
                    return;
                }
            }
        }
        Com_sprintf(
            arg2.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%d\x00" as *const u8 as *const libc::c_char,
            i,
        );
    } else {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Invalid vote string.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Team vote commands are: leader <player>.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    Com_sprintf(
        level.teamVoteString[cs_offset as usize].as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"%s %s\x00" as *const u8 as *const libc::c_char,
        arg1.as_mut_ptr(),
        arg2.as_mut_ptr(),
    );
    i = 0i32;
    while i < level.maxclients {
        if !((*level.clients.offset(i as isize)).pers.connected as libc::c_uint
            == CON_DISCONNECTED as libc::c_int as libc::c_uint)
        {
            if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                == team as libc::c_uint
            {
                trap_SendServerCommand(
                    i,
                    va(
                        b"print \"%s called a team vote.\n\"\x00" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*(*ent).client).pers.netname.as_mut_ptr(),
                    ),
                );
            }
        }
        i += 1
    }
    level.teamVoteTime[cs_offset as usize] = level.time;
    level.teamVoteYes[cs_offset as usize] = 1i32;
    level.teamVoteNo[cs_offset as usize] = 0i32;
    i = 0i32;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
            == team as libc::c_uint
        {
            (*level.clients.offset(i as isize)).ps.eFlags &= !0x80000i32
        }
        i += 1
    }
    (*(*ent).client).ps.eFlags |= 0x80000i32;
    trap_SetConfigstring(
        12i32 + cs_offset,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.teamVoteTime[cs_offset as usize],
        ),
    );
    trap_SetConfigstring(
        14i32 + cs_offset,
        level.teamVoteString[cs_offset as usize].as_mut_ptr(),
    );
    trap_SetConfigstring(
        16i32 + cs_offset,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.teamVoteYes[cs_offset as usize],
        ),
    );
    trap_SetConfigstring(
        18i32 + cs_offset,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.teamVoteNo[cs_offset as usize],
        ),
    );
}
/*
==================
Cmd_Vote_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Vote_f(mut ent: *mut gentity_t) {
    let mut msg: [libc::c_char; 64] = [0; 64];
    if 0 == level.voteTime {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"No vote in progress.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if 0 != (*(*ent).client).ps.eFlags & 0x4000i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Vote already cast.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Not allowed to vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        b"print \"Vote cast.\n\"\x00" as *const u8 as *const libc::c_char,
    );
    (*(*ent).client).ps.eFlags |= 0x4000i32;
    trap_Argv(
        1i32,
        msg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    if tolower(msg[0usize] as libc::c_int) == 'y' as i32 || msg[0usize] as libc::c_int == '1' as i32
    {
        level.voteYes += 1;
        trap_SetConfigstring(
            10i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.voteYes,
            ),
        );
    } else {
        level.voteNo += 1;
        trap_SetConfigstring(
            11i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.voteNo,
            ),
        );
    };
}
/*
==================
Cmd_CallVote_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_CallVote_f(mut ent: *mut gentity_t) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut arg1: [libc::c_char; 1024] = [0; 1024];
    let mut arg2: [libc::c_char; 1024] = [0; 1024];
    if 0 == g_allowVote.integer {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Voting not allowed here.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if 0 != level.voteTime {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"A vote is already in progress.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).pers.voteCount >= 3i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"You have called the maximum number of votes.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Not allowed to call a vote as spectator.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    trap_Argv(
        1i32,
        arg1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    trap_Argv(
        2i32,
        arg2.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    c = arg2.as_mut_ptr();
    while 0 != *c {
        match *c as libc::c_int {
            10 | 13 | 59 => {
                trap_SendServerCommand(
                    ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                        as libc::c_int,
                    b"print \"Invalid vote string.\n\"\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
            _ => {}
        }
        c = c.offset(1isize)
    }
    if !(0
        == Q_stricmp(
            arg1.as_mut_ptr(),
            b"map_restart\x00" as *const u8 as *const libc::c_char,
        ))
    {
        if !(0
            == Q_stricmp(
                arg1.as_mut_ptr(),
                b"nextmap\x00" as *const u8 as *const libc::c_char,
            ))
        {
            if !(0
                == Q_stricmp(
                    arg1.as_mut_ptr(),
                    b"map\x00" as *const u8 as *const libc::c_char,
                ))
            {
                if !(0
                    == Q_stricmp(
                        arg1.as_mut_ptr(),
                        b"g_gametype\x00" as *const u8 as *const libc::c_char,
                    ))
                {
                    if !(0
                        == Q_stricmp(
                            arg1.as_mut_ptr(),
                            b"kick\x00" as *const u8 as *const libc::c_char,
                        ))
                    {
                        if !(0
                            == Q_stricmp(
                                arg1.as_mut_ptr(),
                                b"clientkick\x00" as *const u8 as *const libc::c_char,
                            ))
                        {
                            if !(0
                                == Q_stricmp(
                                    arg1.as_mut_ptr(),
                                    b"g_doWarmup\x00" as *const u8 as *const libc::c_char,
                                ))
                            {
                                if !(0
                                    == Q_stricmp(
                                        arg1.as_mut_ptr(),
                                        b"timelimit\x00" as *const u8 as *const libc::c_char,
                                    ))
                                {
                                    if 0 == Q_stricmp(
                                        arg1.as_mut_ptr(),
                                        b"fraglimit\x00" as *const u8 as *const libc::c_char,
                                    ) {
                                    } else {
                                        trap_SendServerCommand(
                                            ent.wrapping_offset_from(g_entities.as_mut_ptr())
                                                as libc::c_long
                                                as libc::c_int,
                                            b"print \"Invalid vote string.\n\"\x00" as *const u8
                                                as *const libc::c_char,
                                        );
                                        trap_SendServerCommand(ent.wrapping_offset_from(g_entities.as_mut_ptr())
                                                                   as
                                                                   libc::c_long
                                                                   as
                                                                   libc::c_int,
                                                               b"print \"Vote commands are: map_restart, nextmap, map <mapname>, g_gametype <n>, kick <player>, clientkick <clientnum>, g_doWarmup, timelimit <time>, fraglimit <frags>.\n\"\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char);
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if 0 != level.voteExecuteTime {
        if 0 == Q_stricmpn(
            level.voteString.as_mut_ptr(),
            b"map\x00" as *const u8 as *const libc::c_char,
            3i32,
        ) || 0
            == Q_stricmpn(
                level.voteString.as_mut_ptr(),
                b"nextmap\x00" as *const u8 as *const libc::c_char,
                7i32,
            )
        {
            trap_SendServerCommand(
                ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
                b"print \"Vote after map change.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        level.voteExecuteTime = 0i32;
        trap_SendConsoleCommand(
            EXEC_APPEND as libc::c_int,
            va(
                b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.voteString.as_mut_ptr(),
            ),
        );
    }
    if 0 == Q_stricmp(
        arg1.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    ) {
        i = atoi(arg2.as_mut_ptr());
        if i == GT_SINGLE_PLAYER as libc::c_int
            || i < GT_FFA as libc::c_int
            || i >= GT_MAX_GAME_TYPE as libc::c_int
        {
            trap_SendServerCommand(
                ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
                b"print \"Invalid gametype.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        Com_sprintf(
            level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%s %d\x00" as *const u8 as *const libc::c_char,
            arg1.as_mut_ptr(),
            i,
        );
        Com_sprintf(
            level.voteDisplayString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%s %s\x00" as *const u8 as *const libc::c_char,
            arg1.as_mut_ptr(),
            gameNames[i as usize],
        );
    } else if 0
        == Q_stricmp(
            arg1.as_mut_ptr(),
            b"map\x00" as *const u8 as *const libc::c_char,
        )
    {
        let mut s: [libc::c_char; 1024] = [0; 1024];
        trap_Cvar_VariableStringBuffer(
            b"nextmap\x00" as *const u8 as *const libc::c_char,
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 != *s.as_mut_ptr() {
            Com_sprintf(
                level.voteString.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"%s %s; set nextmap \"%s\"\x00" as *const u8 as *const libc::c_char,
                arg1.as_mut_ptr(),
                arg2.as_mut_ptr(),
                s.as_mut_ptr(),
            );
        } else {
            Com_sprintf(
                level.voteString.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"%s %s\x00" as *const u8 as *const libc::c_char,
                arg1.as_mut_ptr(),
                arg2.as_mut_ptr(),
            );
        }
        Com_sprintf(
            level.voteDisplayString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            level.voteString.as_mut_ptr(),
        );
    } else if 0
        == Q_stricmp(
            arg1.as_mut_ptr(),
            b"nextmap\x00" as *const u8 as *const libc::c_char,
        )
    {
        let mut s_0: [libc::c_char; 1024] = [0; 1024];
        trap_Cvar_VariableStringBuffer(
            b"nextmap\x00" as *const u8 as *const libc::c_char,
            s_0.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == *s_0.as_mut_ptr() {
            trap_SendServerCommand(
                ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
                b"print \"nextmap not set.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        Com_sprintf(
            level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"vstr nextmap\x00" as *const u8 as *const libc::c_char,
        );
        Com_sprintf(
            level.voteDisplayString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            level.voteString.as_mut_ptr(),
        );
    } else if 0
        == Q_stricmp(
            arg1.as_mut_ptr(),
            b"clientkick\x00" as *const u8 as *const libc::c_char,
        )
        || 0 == Q_stricmp(
            arg1.as_mut_ptr(),
            b"kick\x00" as *const u8 as *const libc::c_char,
        )
    {
        i = ClientNumberFromString(
            ent,
            arg2.as_mut_ptr(),
            (0 == Q_stricmp(
                arg1.as_mut_ptr(),
                b"clientkick\x00" as *const u8 as *const libc::c_char,
            )) as libc::c_int as qboolean,
            (0 == Q_stricmp(
                arg1.as_mut_ptr(),
                b"kick\x00" as *const u8 as *const libc::c_char,
            )) as libc::c_int as qboolean,
        );
        if i == -1i32 {
            return;
        }
        if 0 != (*level.clients.offset(i as isize)).pers.localClient as u64 {
            trap_SendServerCommand(
                ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
                b"print \"Cannot kick host player.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
        Com_sprintf(
            level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"clientkick %d\x00" as *const u8 as *const libc::c_char,
            i,
        );
        Com_sprintf(
            level.voteDisplayString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"kick %s\x00" as *const u8 as *const libc::c_char,
            (*level.clients.offset(i as isize))
                .pers
                .netname
                .as_mut_ptr(),
        );
    } else {
        Com_sprintf(
            level.voteString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%s \"%s\"\x00" as *const u8 as *const libc::c_char,
            arg1.as_mut_ptr(),
            arg2.as_mut_ptr(),
        );
        Com_sprintf(
            level.voteDisplayString.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%s\x00" as *const u8 as *const libc::c_char,
            level.voteString.as_mut_ptr(),
        );
    }
    trap_SendServerCommand(
        -1i32,
        va(
            b"print \"%s called a vote.\n\"\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*(*ent).client).pers.netname.as_mut_ptr(),
        ),
    );
    level.voteTime = level.time;
    level.voteYes = 1i32;
    level.voteNo = 0i32;
    i = 0i32;
    while i < level.maxclients {
        (*level.clients.offset(i as isize)).ps.eFlags &= !0x4000i32;
        i += 1
    }
    (*(*ent).client).ps.eFlags |= 0x4000i32;
    trap_SetConfigstring(
        8i32,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.voteTime,
        ),
    );
    trap_SetConfigstring(9i32, level.voteDisplayString.as_mut_ptr());
    trap_SetConfigstring(
        10i32,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.voteYes,
        ),
    );
    trap_SetConfigstring(
        11i32,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.voteNo,
        ),
    );
}
static mut gameNames: [*const libc::c_char; 8] = [
    b"Free For All\x00" as *const u8 as *const libc::c_char,
    b"Tournament\x00" as *const u8 as *const libc::c_char,
    b"Single Player\x00" as *const u8 as *const libc::c_char,
    b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
    b"Capture the Flag\x00" as *const u8 as *const libc::c_char,
    b"One Flag CTF\x00" as *const u8 as *const libc::c_char,
    b"Overload\x00" as *const u8 as *const libc::c_char,
    b"Harvester\x00" as *const u8 as *const libc::c_char,
];
/*
==================
Cmd_Where_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Where_f(mut ent: *mut gentity_t) {
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"print \"%s\n\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            vtos((*ent).r.currentOrigin.as_mut_ptr() as *const vec_t),
        ),
    );
}
/*
=================
Cmd_Team_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Team_f(mut ent: *mut gentity_t) {
    let mut oldTeam: libc::c_int = 0;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() != 2i32 {
        oldTeam = (*(*ent).client).sess.sessionTeam as libc::c_int;
        match oldTeam {
            2 => {
                trap_SendServerCommand(
                    ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                        as libc::c_int,
                    b"print \"Blue team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            1 => {
                trap_SendServerCommand(
                    ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                        as libc::c_int,
                    b"print \"Red team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            0 => {
                trap_SendServerCommand(
                    ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                        as libc::c_int,
                    b"print \"Free team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            3 => {
                trap_SendServerCommand(
                    ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long
                        as libc::c_int,
                    b"print \"Spectator team\n\"\x00" as *const u8 as *const libc::c_char,
                );
            }
            _ => {}
        }
        return;
    }
    if (*(*ent).client).switchTeamTime > level.time {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"May not switch teams more than once per 5 seconds.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int
        && (*(*ent).client).sess.sessionTeam as libc::c_uint
            == TEAM_FREE as libc::c_int as libc::c_uint
    {
        (*(*ent).client).sess.losses += 1
    }
    trap_Argv(
        1i32,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    SetTeam(ent, s.as_mut_ptr());
    (*(*ent).client).switchTeamTime = level.time + 5000i32;
}
/*
=================
Cmd_Follow_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Follow_f(mut ent: *mut gentity_t) {
    let mut i: libc::c_int = 0;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() != 2i32 {
        if (*(*ent).client).sess.spectatorState as libc::c_uint
            == SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
        {
            StopFollowing(ent);
        }
        return;
    }
    trap_Argv(
        1i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    i = ClientNumberFromString(ent, arg.as_mut_ptr(), qtrue, qtrue);
    if i == -1i32 {
        return;
    }
    if &mut *level.clients.offset(i as isize) as *mut gclient_s == (*ent).client {
        return;
    }
    if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        return;
    }
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int
        && (*(*ent).client).sess.sessionTeam as libc::c_uint
            == TEAM_FREE as libc::c_int as libc::c_uint
    {
        (*(*ent).client).sess.losses += 1
    }
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        != TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        SetTeam(ent, b"spectator\x00" as *const u8 as *const libc::c_char);
    }
    (*(*ent).client).sess.spectatorState = SPECTATOR_FOLLOW;
    (*(*ent).client).sess.spectatorClient = i;
}
/*
==================
Cmd_LevelShot_f

This is just to help generate the level pictures
for the menus.  It goes to the intermission immediately
and sends over a command to the client to resize the view,
hide the scoreboard, and take a special screenshot
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_LevelShot_f(mut ent: *mut gentity_t) {
    if 0 == (*(*ent).client).pers.localClient as u64 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"The levelshot command must be executed by a local client\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if 0 == CheatsOk(ent) as u64 {
        return;
    }
    if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Must not be in singleplayer mode for levelshot\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    BeginIntermission();
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        b"clientLevelShot\x00" as *const u8 as *const libc::c_char,
    );
}
/*
==================
CheatsOk
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CheatsOk(mut ent: *mut gentity_t) -> qboolean {
    if 0 == g_cheats.integer {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Cheats are not enabled on this server.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return qfalse;
    }
    if (*ent).health <= 0i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"You must be alive to use this command.\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return qfalse;
    }
    return qtrue;
}
/*
==================
Cmd_TeamTask_f
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_TeamTask_f(mut ent: *mut gentity_t) {
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    let mut task: libc::c_int = 0;
    let mut client: libc::c_int =
        (*ent).client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int;
    if trap_Argc() != 2i32 {
        return;
    }
    trap_Argv(
        1i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    task = atoi(arg.as_mut_ptr());
    trap_GetUserinfo(
        client,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teamtask\x00" as *const u8 as *const libc::c_char,
        va(
            b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            task,
        ),
    );
    trap_SetUserinfo(client, userinfo.as_mut_ptr());
    ClientUserinfoChanged(client);
}
/*
=================
Cmd_Kill_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Kill_f(mut ent: *mut gentity_t) {
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        return;
    }
    if (*ent).health <= 0i32 {
        return;
    }
    (*ent).flags &= !0x10i32;
    (*ent).health = -999i32;
    (*(*ent).client).ps.stats[STAT_HEALTH as libc::c_int as usize] = (*ent).health;
    player_die(ent, ent, ent, 100000i32, MOD_SUICIDE as libc::c_int);
}
/*
==================
Cmd_Noclip_f

argv(0) noclip
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Noclip_f(mut ent: *mut gentity_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == CheatsOk(ent) as u64 {
        return;
    }
    if 0 != (*(*ent).client).noclip as u64 {
        msg = b"noclip OFF\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        msg = b"noclip ON\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    (*(*ent).client).noclip = (0 == (*(*ent).client).noclip as u64) as libc::c_int as qboolean;
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"print \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg,
        ),
    );
}
/*
==================
Cmd_Notarget_f

Sets client to notarget

argv(0) notarget
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Notarget_f(mut ent: *mut gentity_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == CheatsOk(ent) as u64 {
        return;
    }
    (*ent).flags ^= 0x20i32;
    if 0 == (*ent).flags & 0x20i32 {
        msg = b"notarget OFF\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        msg = b"notarget ON\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"print \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg,
        ),
    );
}
/*
==================
Cmd_God_f

Sets client to godmode

argv(0) god
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_God_f(mut ent: *mut gentity_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == CheatsOk(ent) as u64 {
        return;
    }
    (*ent).flags ^= 0x10i32;
    if 0 == (*ent).flags & 0x10i32 {
        msg = b"godmode OFF\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        msg = b"godmode ON\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"print \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg,
        ),
    );
}
/*
==================
Cmd_Give_f

Give items to a client
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Give_f(mut ent: *mut gentity_t) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    let mut i: libc::c_int = 0;
    let mut give_all: qboolean = qfalse;
    let mut it_ent: *mut gentity_t = 0 as *mut gentity_t;
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
    if 0 == CheatsOk(ent) as u64 {
        return;
    }
    name = ConcatArgs(1i32);
    if Q_stricmp(name, b"all\x00" as *const u8 as *const libc::c_char) == 0i32 {
        give_all = qtrue
    } else {
        give_all = qfalse
    }
    if 0 != give_all as libc::c_uint
        || Q_stricmp(name, b"health\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        (*ent).health = (*(*ent).client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize];
        if 0 == give_all as u64 {
            return;
        }
    }
    if 0 != give_all as libc::c_uint
        || Q_stricmp(name, b"weapons\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        (*(*ent).client).ps.stats[STAT_WEAPONS as libc::c_int as usize] = (1i32
            << WP_NUM_WEAPONS as libc::c_int)
            - 1i32
            - (1i32 << WP_GRAPPLING_HOOK as libc::c_int)
            - (1i32 << WP_NONE as libc::c_int);
        if 0 == give_all as u64 {
            return;
        }
    }
    if 0 != give_all as libc::c_uint
        || Q_stricmp(name, b"ammo\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        i = 0i32;
        while i < 16i32 {
            (*(*ent).client).ps.ammo[i as usize] = 999i32;
            i += 1
        }
        if 0 == give_all as u64 {
            return;
        }
    }
    if 0 != give_all as libc::c_uint
        || Q_stricmp(name, b"armor\x00" as *const u8 as *const libc::c_char) == 0i32
    {
        (*(*ent).client).ps.stats[STAT_ARMOR as libc::c_int as usize] = 200i32;
        if 0 == give_all as u64 {
            return;
        }
    }
    if Q_stricmp(name, b"excellent\x00" as *const u8 as *const libc::c_char) == 0i32 {
        (*(*ent).client).ps.persistant[PERS_EXCELLENT_COUNT as libc::c_int as usize] += 1;
        return;
    }
    if Q_stricmp(name, b"impressive\x00" as *const u8 as *const libc::c_char) == 0i32 {
        (*(*ent).client).ps.persistant[PERS_IMPRESSIVE_COUNT as libc::c_int as usize] += 1;
        return;
    }
    if Q_stricmp(
        name,
        b"gauntletaward\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        (*(*ent).client).ps.persistant[PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize] += 1;
        return;
    }
    if Q_stricmp(name, b"defend\x00" as *const u8 as *const libc::c_char) == 0i32 {
        (*(*ent).client).ps.persistant[PERS_DEFEND_COUNT as libc::c_int as usize] += 1;
        return;
    }
    if Q_stricmp(name, b"assist\x00" as *const u8 as *const libc::c_char) == 0i32 {
        (*(*ent).client).ps.persistant[PERS_ASSIST_COUNT as libc::c_int as usize] += 1;
        return;
    }
    if 0 == give_all as u64 {
        it = BG_FindItem(name);
        if it.is_null() {
            return;
        }
        it_ent = G_Spawn();
        (*it_ent).s.origin[0usize] = (*ent).r.currentOrigin[0usize];
        (*it_ent).s.origin[1usize] = (*ent).r.currentOrigin[1usize];
        (*it_ent).s.origin[2usize] = (*ent).r.currentOrigin[2usize];
        (*it_ent).classname = (*it).classname;
        G_SpawnItem(it_ent, it);
        FinishSpawningItem(it_ent);
        memset(
            &mut trace as *mut trace_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<trace_t>() as libc::c_ulong,
        );
        Touch_Item(it_ent, ent, &mut trace);
        if 0 != (*it_ent).inuse as u64 {
            G_FreeEntity(it_ent);
        }
    };
}
/*
==================
ConcatArgs
==================
*/
#[no_mangle]
pub unsafe extern "C" fn ConcatArgs(mut start: libc::c_int) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut tlen: libc::c_int = 0;
    static mut line: [libc::c_char; 1024] = [0; 1024];
    let mut len: libc::c_int = 0;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    len = 0i32;
    c = trap_Argc();
    i = start;
    while i < c {
        trap_Argv(
            i,
            arg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        tlen = strlen(arg.as_mut_ptr()) as libc::c_int;
        if len + tlen >= 1024i32 - 1i32 {
            break;
        }
        memcpy(
            line.as_mut_ptr().offset(len as isize) as *mut libc::c_void,
            arg.as_mut_ptr() as *const libc::c_void,
            tlen as libc::c_ulong,
        );
        len += tlen;
        if i != c - 1i32 {
            line[len as usize] = ' ' as i32 as libc::c_char;
            len += 1
        }
        i += 1
    }
    line[len as usize] = 0i32 as libc::c_char;
    return line.as_mut_ptr();
}
/*
==================
Cmd_Say_f
==================
*/
unsafe extern "C" fn Cmd_Say_f(mut ent: *mut gentity_t, mut mode: libc::c_int, mut arg0: qboolean) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if trap_Argc() < 2i32 && 0 == arg0 as u64 {
        return;
    }
    if 0 != arg0 as u64 {
        p = ConcatArgs(0i32)
    } else {
        p = ConcatArgs(1i32)
    }
    SanitizeChatText(p);
    G_Say(ent, 0 as *mut gentity_t, mode, p);
}
unsafe extern "C" fn SanitizeChatText(mut text: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while 0 != *text.offset(i as isize) {
        if *text.offset(i as isize) as libc::c_int == '\n' as i32
            || *text.offset(i as isize) as libc::c_int == '\r' as i32
        {
            *text.offset(i as isize) = ' ' as i32 as libc::c_char
        }
        i += 1
    }
}
/*
==================
Cmd_Tell_f
==================
*/
unsafe extern "C" fn Cmd_Tell_f(mut ent: *mut gentity_t) {
    let mut targetNum: libc::c_int = 0;
    let mut target: *mut gentity_t = 0 as *mut gentity_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arg: [libc::c_char; 1024] = [0; 1024];
    if trap_Argc() < 3i32 {
        trap_SendServerCommand(
            ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            b"print \"Usage: tell <player id> <message>\n\"\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    trap_Argv(
        1i32,
        arg.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    targetNum = ClientNumberFromString(ent, arg.as_mut_ptr(), qtrue, qtrue);
    if targetNum == -1i32 {
        return;
    }
    target = &mut g_entities[targetNum as usize] as *mut gentity_t;
    if 0 == (*target).inuse as u64 || (*target).client.is_null() {
        return;
    }
    p = ConcatArgs(2i32);
    SanitizeChatText(p);
    G_LogPrintf(
        b"tell: %s to %s: %s\n\x00" as *const u8 as *const libc::c_char,
        (*(*ent).client).pers.netname.as_mut_ptr(),
        (*(*target).client).pers.netname.as_mut_ptr(),
        p,
    );
    G_Say(ent, target, 2i32, p);
    if ent != target && 0 == (*ent).r.svFlags & 0x8i32 {
        G_Say(ent, ent, 2i32, p);
    };
}
unsafe extern "C" fn run_static_initializers() {
    numgc_orders = (::std::mem::size_of::<[*mut libc::c_char; 7]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
