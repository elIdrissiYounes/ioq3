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
    gitem_s, gitem_t, itemType_t, team_t, unnamed_0, unnamed_1, unnamed_2, unnamed_3, unnamed_4,
    ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER,
    ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, GT_1FCTF,
    GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM,
    GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP,
    IT_POWERUP, IT_TEAM, IT_WEAPON, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER,
    PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS,
    PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT,
    PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION, PM_NOCLIP, PM_NORMAL, PM_SPECTATOR,
    PM_SPINTERMISSION, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH,
    STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS,
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
    spectatorState_t, trap_Cvar_Register, trap_Cvar_Set, trap_Cvar_Update,
    trap_Cvar_VariableIntegerValue, trap_Cvar_VariableStringBuffer, trap_FS_FCloseFile,
    trap_FS_FOpenFile, trap_GetServerinfo, trap_LocateGameData, trap_SendConsoleCommand,
    trap_SendServerCommand, trap_SetConfigstring, trap_UnlinkEntity, CON_CONNECTED, CON_CONNECTING,
    CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1, MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW,
    SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
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
use g_public_h::{
    entityShared_t, unnamed_5, BOTAI_START_FRAME, GAME_CLIENT_BEGIN, GAME_CLIENT_COMMAND,
    GAME_CLIENT_CONNECT, GAME_CLIENT_DISCONNECT, GAME_CLIENT_THINK, GAME_CLIENT_USERINFO_CHANGED,
    GAME_CONSOLE_COMMAND, GAME_INIT, GAME_RUN_FRAME, GAME_SHUTDOWN,
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
    CheckTeamStatus, OnSameTeam, SP_team_CTF_blueplayer, SP_team_CTF_bluespawn,
    SP_team_CTF_redplayer, SP_team_CTF_redspawn, Team_CheckDroppedItem,
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
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t, fsMode_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    unnamed, usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, Q_stricmp, Q_strncmp, EXEC_APPEND,
    EXEC_INSERT, EXEC_NOW, FS_APPEND, FS_APPEND_SYNC, FS_READ, FS_WRITE, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stddef_h::size_t;
use stdlib::{__compar_fn_t, atoi, intptr_t, memset, qsort, srand, strcmp};

#[no_mangle]
pub unsafe extern "C" fn BeginIntermission() {
    let mut i: libc::c_int = 0;
    let mut client: *mut gentity_t = 0 as *mut gentity_t;
    if 0 != level.intermissiontime {
        return;
    }
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int {
        AdjustTournamentScores();
    }
    level.intermissiontime = level.time;
    i = 0i32;
    while i < level.maxclients {
        client = g_entities.as_mut_ptr().offset(i as isize);
        if !(0 == (*client).inuse as u64) {
            if (*client).health <= 0i32 {
                ClientRespawn(client);
            }
            MoveClientToIntermission(client);
        }
        i += 1
    }
    if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int {
        UpdateTournamentInfo();
        SpawnModelsOnVictoryPads();
    }
    SendScoreboardMessageToAllClients();
}
#[no_mangle]
pub unsafe extern "C" fn SendScoreboardMessageToAllClients() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).pers.connected as libc::c_uint
            == CON_CONNECTED as libc::c_int as libc::c_uint
        {
            DeathmatchScoreboardMessage(g_entities.as_mut_ptr().offset(i as isize));
        }
        i += 1
    }
}
#[no_mangle]
pub static mut g_entities: [gentity_t; 1024] = [gentity_s {
    s: entityState_s {
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
    },
    r: entityShared_t {
        unused: entityState_s {
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
        },
        linked: qfalse,
        linkcount: 0,
        svFlags: 0,
        singleClient: 0,
        bmodel: qfalse,
        mins: [0.; 3],
        maxs: [0.; 3],
        contents: 0,
        absmin: [0.; 3],
        absmax: [0.; 3],
        currentOrigin: [0.; 3],
        currentAngles: [0.; 3],
        ownerNum: 0,
    },
    client: 0 as *const gclient_s as *mut gclient_s,
    inuse: qfalse,
    classname: 0 as *const libc::c_char as *mut libc::c_char,
    spawnflags: 0,
    neverFree: qfalse,
    flags: 0,
    model: 0 as *const libc::c_char as *mut libc::c_char,
    model2: 0 as *const libc::c_char as *mut libc::c_char,
    freetime: 0,
    eventTime: 0,
    freeAfterEvent: qfalse,
    unlinkAfterEvent: qfalse,
    physicsObject: qfalse,
    physicsBounce: 0.,
    clipmask: 0,
    moverState: MOVER_POS1,
    soundPos1: 0,
    sound1to2: 0,
    sound2to1: 0,
    soundPos2: 0,
    soundLoop: 0,
    parent: 0 as *const gentity_t as *mut gentity_t,
    nextTrain: 0 as *const gentity_t as *mut gentity_t,
    prevTrain: 0 as *const gentity_t as *mut gentity_t,
    pos1: [0.; 3],
    pos2: [0.; 3],
    message: 0 as *const libc::c_char as *mut libc::c_char,
    timestamp: 0,
    target: 0 as *const libc::c_char as *mut libc::c_char,
    targetname: 0 as *const libc::c_char as *mut libc::c_char,
    team: 0 as *const libc::c_char as *mut libc::c_char,
    targetShaderName: 0 as *const libc::c_char as *mut libc::c_char,
    targetShaderNewName: 0 as *const libc::c_char as *mut libc::c_char,
    target_ent: 0 as *const gentity_t as *mut gentity_t,
    speed: 0.,
    movedir: [0.; 3],
    nextthink: 0,
    think: None,
    reached: None,
    blocked: None,
    touch: None,
    use_0: None,
    pain: None,
    die: None,
    pain_debounce_time: 0,
    fly_sound_debounce_time: 0,
    last_move_time: 0,
    health: 0,
    takedamage: qfalse,
    damage: 0,
    splashDamage: 0,
    splashRadius: 0,
    methodOfDeath: 0,
    splashMethodOfDeath: 0,
    count: 0,
    chain: 0 as *const gentity_t as *mut gentity_t,
    enemy: 0 as *const gentity_t as *mut gentity_t,
    activator: 0 as *const gentity_t as *mut gentity_t,
    teamchain: 0 as *const gentity_t as *mut gentity_t,
    teammaster: 0 as *const gentity_t as *mut gentity_t,
    watertype: 0,
    waterlevel: 0,
    noise_index: 0,
    wait: 0.,
    random: 0.,
    item: 0 as *const gitem_t as *mut gitem_t,
}; 1024];
#[no_mangle]
pub static mut level: level_locals_t = level_locals_t {
    clients: 0 as *const gclient_s as *mut gclient_s,
    gentities: 0 as *const gentity_s as *mut gentity_s,
    gentitySize: 0,
    num_entities: 0,
    warmupTime: 0,
    logFile: 0,
    maxclients: 0,
    framenum: 0,
    time: 0,
    previousTime: 0,
    startTime: 0,
    teamScores: [0; 4],
    lastTeamLocationTime: 0,
    newSession: qfalse,
    restarted: qfalse,
    numConnectedClients: 0,
    numNonSpectatorClients: 0,
    numPlayingClients: 0,
    sortedClients: [0; 64],
    follow1: 0,
    follow2: 0,
    snd_fry: 0,
    warmupModificationCount: 0,
    voteString: [0; 1024],
    voteDisplayString: [0; 1024],
    voteTime: 0,
    voteExecuteTime: 0,
    voteYes: 0,
    voteNo: 0,
    numVotingClients: 0,
    teamVoteString: [[0; 1024]; 2],
    teamVoteTime: [0; 2],
    teamVoteYes: [0; 2],
    teamVoteNo: [0; 2],
    numteamVotingClients: [0; 2],
    spawning: qfalse,
    numSpawnVars: 0,
    spawnVars: [[0 as *const libc::c_char as *mut libc::c_char; 2]; 64],
    numSpawnVarChars: 0,
    spawnVarChars: [0; 4096],
    intermissionQueued: 0,
    intermissiontime: 0,
    changemap: 0 as *const libc::c_char as *mut libc::c_char,
    readyToExit: qfalse,
    exitTime: 0,
    intermission_origin: [0.; 3],
    intermission_angle: [0.; 3],
    locationLinked: qfalse,
    locationHead: 0 as *const gentity_t as *mut gentity_t,
    bodyQueIndex: 0,
    bodyQue: [0 as *const gentity_t as *mut gentity_t; 8],
};
#[no_mangle]
pub static mut g_gametype: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//
// g_main.c
//
#[no_mangle]
pub unsafe extern "C" fn MoveClientToIntermission(mut ent: *mut gentity_t) {
    if (*(*ent).client).sess.spectatorState as libc::c_uint
        == SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
    {
        StopFollowing(ent);
    }
    FindIntermissionPoint();
    (*ent).s.origin[0usize] = level.intermission_origin[0usize];
    (*ent).s.origin[1usize] = level.intermission_origin[1usize];
    (*ent).s.origin[2usize] = level.intermission_origin[2usize];
    (*(*ent).client).ps.origin[0usize] = level.intermission_origin[0usize];
    (*(*ent).client).ps.origin[1usize] = level.intermission_origin[1usize];
    (*(*ent).client).ps.origin[2usize] = level.intermission_origin[2usize];
    (*(*ent).client).ps.viewangles[0usize] = level.intermission_angle[0usize];
    (*(*ent).client).ps.viewangles[1usize] = level.intermission_angle[1usize];
    (*(*ent).client).ps.viewangles[2usize] = level.intermission_angle[2usize];
    (*(*ent).client).ps.pm_type = PM_INTERMISSION as libc::c_int;
    memset(
        (*(*ent).client).ps.powerups.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    (*(*ent).client).ps.eFlags = 0i32;
    (*ent).s.eFlags = 0i32;
    (*ent).s.eType = ET_GENERAL as libc::c_int;
    (*ent).s.modelindex = 0i32;
    (*ent).s.loopSound = 0i32;
    (*ent).s.event = 0i32;
    (*ent).r.contents = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn FindIntermissionPoint() {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut target: *mut gentity_t = 0 as *mut gentity_t;
    let mut dir: vec3_t = [0.; 3];
    ent = G_Find(
        0 as *mut gentity_t,
        &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t as libc::c_int,
        b"info_player_intermission\x00" as *const u8 as *const libc::c_char,
    );
    if ent.is_null() {
        SelectSpawnPoint(
            vec3_origin.as_mut_ptr(),
            level.intermission_origin.as_mut_ptr(),
            level.intermission_angle.as_mut_ptr(),
            qfalse,
        );
    } else {
        level.intermission_origin[0usize] = (*ent).s.origin[0usize];
        level.intermission_origin[1usize] = (*ent).s.origin[1usize];
        level.intermission_origin[2usize] = (*ent).s.origin[2usize];
        level.intermission_angle[0usize] = (*ent).s.angles[0usize];
        level.intermission_angle[1usize] = (*ent).s.angles[1usize];
        level.intermission_angle[2usize] = (*ent).s.angles[2usize];
        if !(*ent).target.is_null() {
            target = G_PickTarget((*ent).target);
            if !target.is_null() {
                dir[0usize] = (*target).s.origin[0usize] - level.intermission_origin[0usize];
                dir[1usize] = (*target).s.origin[1usize] - level.intermission_origin[1usize];
                dir[2usize] = (*target).s.origin[2usize] - level.intermission_origin[2usize];
                vectoangles(
                    dir.as_mut_ptr() as *const vec_t,
                    level.intermission_angle.as_mut_ptr(),
                );
            }
        }
    };
}
/*
=======================
AdjustTournamentScores
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn AdjustTournamentScores() {
    let mut clientNum: libc::c_int = 0;
    clientNum = level.sortedClients[0usize];
    if (*level.clients.offset(clientNum as isize)).pers.connected as libc::c_uint
        == CON_CONNECTED as libc::c_int as libc::c_uint
    {
        let ref mut fresh0 = (*level.clients.offset(clientNum as isize)).sess.wins;
        *fresh0 += 1;
        ClientUserinfoChanged(clientNum);
    }
    clientNum = level.sortedClients[1usize];
    if (*level.clients.offset(clientNum as isize)).pers.connected as libc::c_uint
        == CON_CONNECTED as libc::c_int as libc::c_uint
    {
        let ref mut fresh1 = (*level.clients.offset(clientNum as isize)).sess.losses;
        *fresh1 += 1;
        ClientUserinfoChanged(clientNum);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CalculateRanks() {
    let mut i: libc::c_int = 0;
    let mut rank: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut newScore: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    level.follow1 = -1i32;
    level.follow2 = -1i32;
    level.numConnectedClients = 0i32;
    level.numNonSpectatorClients = 0i32;
    level.numPlayingClients = 0i32;
    level.numVotingClients = 0i32;
    i = 0i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        level.numteamVotingClients[i as usize] = 0i32;
        i += 1
    }
    i = 0i32;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).pers.connected as libc::c_uint
            != CON_DISCONNECTED as libc::c_int as libc::c_uint
        {
            level.sortedClients[level.numConnectedClients as usize] = i;
            level.numConnectedClients += 1;
            if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                != TEAM_SPECTATOR as libc::c_int as libc::c_uint
            {
                level.numNonSpectatorClients += 1;
                if (*level.clients.offset(i as isize)).pers.connected as libc::c_uint
                    == CON_CONNECTED as libc::c_int as libc::c_uint
                {
                    level.numPlayingClients += 1;
                    if 0 == g_entities[i as usize].r.svFlags & 0x8i32 {
                        level.numVotingClients += 1;
                        if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                            == TEAM_RED as libc::c_int as libc::c_uint
                        {
                            level.numteamVotingClients[0usize] += 1
                        } else if (*level.clients.offset(i as isize)).sess.sessionTeam
                            as libc::c_uint
                            == TEAM_BLUE as libc::c_int as libc::c_uint
                        {
                            level.numteamVotingClients[1usize] += 1
                        }
                    }
                    if level.follow1 == -1i32 {
                        level.follow1 = i
                    } else if level.follow2 == -1i32 {
                        level.follow2 = i
                    }
                }
            }
        }
        i += 1
    }
    qsort(
        level.sortedClients.as_mut_ptr() as *mut libc::c_void,
        level.numConnectedClients as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(SortRanks),
    );
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        i = 0i32;
        while i < level.numConnectedClients {
            cl = &mut *level
                .clients
                .offset(level.sortedClients[i as usize] as isize)
                as *mut gclient_s;
            if level.teamScores[TEAM_RED as libc::c_int as usize]
                == level.teamScores[TEAM_BLUE as libc::c_int as usize]
            {
                (*cl).ps.persistant[PERS_RANK as libc::c_int as usize] = 2i32
            } else if level.teamScores[TEAM_RED as libc::c_int as usize]
                > level.teamScores[TEAM_BLUE as libc::c_int as usize]
            {
                (*cl).ps.persistant[PERS_RANK as libc::c_int as usize] = 0i32
            } else {
                (*cl).ps.persistant[PERS_RANK as libc::c_int as usize] = 1i32
            }
            i += 1
        }
    } else {
        rank = -1i32;
        score = 0i32;
        i = 0i32;
        while i < level.numPlayingClients {
            cl = &mut *level
                .clients
                .offset(level.sortedClients[i as usize] as isize)
                as *mut gclient_s;
            newScore = (*cl).ps.persistant[PERS_SCORE as libc::c_int as usize];
            if i == 0i32 || newScore != score {
                rank = i;
                (*level
                    .clients
                    .offset(level.sortedClients[i as usize] as isize))
                .ps
                .persistant[PERS_RANK as libc::c_int as usize] = rank
            } else {
                (*level
                    .clients
                    .offset(level.sortedClients[(i - 1i32) as usize] as isize))
                .ps
                .persistant[PERS_RANK as libc::c_int as usize] = rank | 0x4000i32;
                (*level
                    .clients
                    .offset(level.sortedClients[i as usize] as isize))
                .ps
                .persistant[PERS_RANK as libc::c_int as usize] = rank | 0x4000i32
            }
            score = newScore;
            if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int
                && level.numPlayingClients == 1i32
            {
                (*level
                    .clients
                    .offset(level.sortedClients[i as usize] as isize))
                .ps
                .persistant[PERS_RANK as libc::c_int as usize] = rank | 0x4000i32
            }
            i += 1
        }
    }
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        trap_SetConfigstring(
            6i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.teamScores[TEAM_RED as libc::c_int as usize],
            ),
        );
        trap_SetConfigstring(
            7i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.teamScores[TEAM_BLUE as libc::c_int as usize],
            ),
        );
    } else if level.numConnectedClients == 0i32 {
        trap_SetConfigstring(
            6i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                -9999i32,
            ),
        );
        trap_SetConfigstring(
            7i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                -9999i32,
            ),
        );
    } else if level.numConnectedClients == 1i32 {
        trap_SetConfigstring(
            6i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*level.clients.offset(level.sortedClients[0usize] as isize))
                    .ps
                    .persistant[PERS_SCORE as libc::c_int as usize],
            ),
        );
        trap_SetConfigstring(
            7i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                -9999i32,
            ),
        );
    } else {
        trap_SetConfigstring(
            6i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*level.clients.offset(level.sortedClients[0usize] as isize))
                    .ps
                    .persistant[PERS_SCORE as libc::c_int as usize],
            ),
        );
        trap_SetConfigstring(
            7i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*level.clients.offset(level.sortedClients[1usize] as isize))
                    .ps
                    .persistant[PERS_SCORE as libc::c_int as usize],
            ),
        );
    }
    CheckExitRules();
    if 0 != level.intermissiontime {
        SendScoreboardMessageToAllClients();
    };
}
#[no_mangle]
pub unsafe extern "C" fn CheckExitRules() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    if 0 != level.intermissiontime {
        CheckIntermissionExit();
        return;
    }
    if 0 != level.intermissionQueued {
        if level.time - level.intermissionQueued >= 1000i32 {
            level.intermissionQueued = 0i32;
            BeginIntermission();
        }
        return;
    }
    if 0 != ScoreIsTied() as u64 {
        return;
    }
    if g_timelimit.integer < 0i32 || g_timelimit.integer > 2147483647i32 / 60000i32 {
        G_Printf(
            b"timelimit %i is out of range, defaulting to 0\n\x00" as *const u8
                as *const libc::c_char,
            g_timelimit.integer,
        );
        trap_Cvar_Set(
            b"timelimit\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut g_timelimit);
    }
    if 0 != g_timelimit.integer && 0 == level.warmupTime {
        if level.time - level.startTime >= g_timelimit.integer * 60000i32 {
            trap_SendServerCommand(
                -1i32,
                b"print \"Timelimit hit.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            LogExit(b"Timelimit hit.\x00" as *const u8 as *const libc::c_char);
            return;
        }
    }
    if g_fraglimit.integer < 0i32 {
        G_Printf(
            b"fraglimit %i is out of range, defaulting to 0\n\x00" as *const u8
                as *const libc::c_char,
            g_fraglimit.integer,
        );
        trap_Cvar_Set(
            b"fraglimit\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut g_fraglimit);
    }
    if g_gametype.integer < GT_CTF as libc::c_int && 0 != g_fraglimit.integer {
        if level.teamScores[TEAM_RED as libc::c_int as usize] >= g_fraglimit.integer {
            trap_SendServerCommand(
                -1i32,
                b"print \"Red hit the fraglimit.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            LogExit(b"Fraglimit hit.\x00" as *const u8 as *const libc::c_char);
            return;
        }
        if level.teamScores[TEAM_BLUE as libc::c_int as usize] >= g_fraglimit.integer {
            trap_SendServerCommand(
                -1i32,
                b"print \"Blue hit the fraglimit.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            LogExit(b"Fraglimit hit.\x00" as *const u8 as *const libc::c_char);
            return;
        }
        i = 0i32;
        while i < g_maxclients.integer {
            cl = level.clients.offset(i as isize);
            if !((*cl).pers.connected as libc::c_uint
                != CON_CONNECTED as libc::c_int as libc::c_uint)
            {
                if !((*cl).sess.sessionTeam as libc::c_uint
                    != TEAM_FREE as libc::c_int as libc::c_uint)
                {
                    if (*cl).ps.persistant[PERS_SCORE as libc::c_int as usize]
                        >= g_fraglimit.integer
                    {
                        LogExit(b"Fraglimit hit.\x00" as *const u8 as *const libc::c_char);
                        trap_SendServerCommand(
                            -1i32,
                            va(
                                b"print \"%s^7 hit the fraglimit.\n\"\x00" as *const u8
                                    as *const libc::c_char
                                    as *mut libc::c_char,
                                (*cl).pers.netname.as_mut_ptr(),
                            ),
                        );
                        return;
                    }
                }
            }
            i += 1
        }
    }
    if g_capturelimit.integer < 0i32 {
        G_Printf(
            b"capturelimit %i is out of range, defaulting to 0\n\x00" as *const u8
                as *const libc::c_char,
            g_capturelimit.integer,
        );
        trap_Cvar_Set(
            b"capturelimit\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut g_capturelimit);
    }
    if g_gametype.integer >= GT_CTF as libc::c_int && 0 != g_capturelimit.integer {
        if level.teamScores[TEAM_RED as libc::c_int as usize] >= g_capturelimit.integer {
            trap_SendServerCommand(
                -1i32,
                b"print \"Red hit the capturelimit.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            LogExit(b"Capturelimit hit.\x00" as *const u8 as *const libc::c_char);
            return;
        }
        if level.teamScores[TEAM_BLUE as libc::c_int as usize] >= g_capturelimit.integer {
            trap_SendServerCommand(
                -1i32,
                b"print \"Blue hit the capturelimit.\n\"\x00" as *const u8 as *const libc::c_char,
            );
            LogExit(b"Capturelimit hit.\x00" as *const u8 as *const libc::c_char);
            return;
        }
    };
}
/*
================
LogExit

Append information about this game to the log file
================
*/
#[no_mangle]
pub unsafe extern "C" fn LogExit(mut string: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut numSorted: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    G_LogPrintf(
        b"Exit: %s\n\x00" as *const u8 as *const libc::c_char,
        string,
    );
    level.intermissionQueued = level.time;
    trap_SetConfigstring(22i32, b"1\x00" as *const u8 as *const libc::c_char);
    numSorted = level.numConnectedClients;
    if numSorted > 32i32 {
        numSorted = 32i32
    }
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        G_LogPrintf(
            b"red:%i  blue:%i\n\x00" as *const u8 as *const libc::c_char,
            level.teamScores[TEAM_RED as libc::c_int as usize],
            level.teamScores[TEAM_BLUE as libc::c_int as usize],
        );
    }
    i = 0i32;
    while i < numSorted {
        let mut ping: libc::c_int = 0;
        cl = &mut *level
            .clients
            .offset(level.sortedClients[i as usize] as isize) as *mut gclient_s;
        if !((*cl).sess.sessionTeam as libc::c_uint
            == TEAM_SPECTATOR as libc::c_int as libc::c_uint)
        {
            if !((*cl).pers.connected as libc::c_uint
                == CON_CONNECTING as libc::c_int as libc::c_uint)
            {
                ping = if (*cl).ps.ping < 999i32 {
                    (*cl).ps.ping
                } else {
                    999i32
                };
                G_LogPrintf(
                    b"score: %i  ping: %i  client: %i %s\n\x00" as *const u8 as *const libc::c_char,
                    (*cl).ps.persistant[PERS_SCORE as libc::c_int as usize],
                    ping,
                    level.sortedClients[i as usize],
                    (*cl).pers.netname.as_mut_ptr(),
                );
            }
        }
        i += 1
    }
}
#[no_mangle]
pub static mut g_capturelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_fraglimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
// allow this many total, including spectators
#[no_mangle]
pub static mut g_maxclients: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
/*
=============
ScoreIsTied
=============
*/
#[no_mangle]
pub unsafe extern "C" fn ScoreIsTied() -> qboolean {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if level.numPlayingClients < 2i32 {
        return qfalse;
    }
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        return (level.teamScores[TEAM_RED as libc::c_int as usize]
            == level.teamScores[TEAM_BLUE as libc::c_int as usize]) as libc::c_int
            as qboolean;
    }
    a = (*level.clients.offset(level.sortedClients[0usize] as isize))
        .ps
        .persistant[PERS_SCORE as libc::c_int as usize];
    b = (*level.clients.offset(level.sortedClients[1usize] as isize))
        .ps
        .persistant[PERS_SCORE as libc::c_int as usize];
    return (a == b) as libc::c_int as qboolean;
}
/*
=================
CheckIntermissionExit

The level will stay at the intermission for a minimum of 5 seconds
If all players wish to continue, the level will then exit.
If one or more players have not acknowledged the continue, the game will
wait 10 seconds before going on.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CheckIntermissionExit() {
    let mut ready: libc::c_int = 0;
    let mut notReady: libc::c_int = 0;
    let mut playerCount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut readyMask: libc::c_int = 0;
    if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int {
        return;
    }
    ready = 0i32;
    notReady = 0i32;
    readyMask = 0i32;
    playerCount = 0i32;
    i = 0i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as libc::c_uint != CON_CONNECTED as libc::c_int as libc::c_uint) {
            if !(0 != g_entities[i as usize].r.svFlags & 0x8i32) {
                playerCount += 1;
                if 0 != (*cl).readyToExit as u64 {
                    ready += 1;
                    if i < 16i32 {
                        readyMask |= 1i32 << i
                    }
                } else {
                    notReady += 1
                }
            }
        }
        i += 1
    }
    i = 0i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as libc::c_uint != CON_CONNECTED as libc::c_int as libc::c_uint) {
            (*cl).ps.stats[STAT_CLIENTS_READY as libc::c_int as usize] = readyMask
        }
        i += 1
    }
    if level.time < level.intermissiontime + 5000i32 {
        return;
    }
    if playerCount > 0i32 {
        if 0 == ready {
            level.readyToExit = qfalse;
            return;
        }
        if 0 == notReady {
            ExitLevel();
            return;
        }
    }
    if 0 == level.readyToExit as u64 {
        level.readyToExit = qtrue;
        level.exitTime = level.time
    }
    if level.time < level.exitTime + 10000i32 {
        return;
    }
    ExitLevel();
}
/*
=============
ExitLevel

When the intermission has been exited, the server is either killed
or moved to a new level based on the "nextmap" cvar

=============
*/
#[no_mangle]
pub unsafe extern "C" fn ExitLevel() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut nextmap: [libc::c_char; 1024] = [0; 1024];
    let mut d1: [libc::c_char; 1024] = [0; 1024];
    BotInterbreedEndMatch();
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int {
        if 0 == level.restarted as u64 {
            RemoveTournamentLoser();
            trap_SendConsoleCommand(
                EXEC_APPEND as libc::c_int,
                b"map_restart 0\n\x00" as *const u8 as *const libc::c_char,
            );
            level.restarted = qtrue;
            level.changemap = 0 as *mut libc::c_char;
            level.intermissiontime = 0i32
        }
        return;
    }
    trap_Cvar_VariableStringBuffer(
        b"nextmap\x00" as *const u8 as *const libc::c_char,
        nextmap.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    trap_Cvar_VariableStringBuffer(
        b"d1\x00" as *const u8 as *const libc::c_char,
        d1.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == Q_stricmp(
        nextmap.as_mut_ptr(),
        b"map_restart 0\x00" as *const u8 as *const libc::c_char,
    ) && 0 != Q_stricmp(d1.as_mut_ptr(), b"\x00" as *const u8 as *const libc::c_char)
    {
        trap_Cvar_Set(
            b"nextmap\x00" as *const u8 as *const libc::c_char,
            b"vstr d2\x00" as *const u8 as *const libc::c_char,
        );
        trap_SendConsoleCommand(
            EXEC_APPEND as libc::c_int,
            b"vstr d1\n\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        trap_SendConsoleCommand(
            EXEC_APPEND as libc::c_int,
            b"vstr nextmap\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    level.changemap = 0 as *mut libc::c_char;
    level.intermissiontime = 0i32;
    level.teamScores[TEAM_RED as libc::c_int as usize] = 0i32;
    level.teamScores[TEAM_BLUE as libc::c_int as usize] = 0i32;
    i = 0i32;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected as libc::c_uint != CON_CONNECTED as libc::c_int as libc::c_uint) {
            (*cl).ps.persistant[PERS_SCORE as libc::c_int as usize] = 0i32
        }
        i += 1
    }
    G_WriteSessionData();
    i = 0i32;
    while i < g_maxclients.integer {
        if (*level.clients.offset(i as isize)).pers.connected as libc::c_uint
            == CON_CONNECTED as libc::c_int as libc::c_uint
        {
            (*level.clients.offset(i as isize)).pers.connected = CON_CONNECTING
        }
        i += 1
    }
}
/*
=======================
RemoveTournamentLoser

Make the loser a spectator at the back of the line
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn RemoveTournamentLoser() {
    let mut clientNum: libc::c_int = 0;
    if level.numPlayingClients != 2i32 {
        return;
    }
    clientNum = level.sortedClients[1usize];
    if (*level.clients.offset(clientNum as isize)).pers.connected as libc::c_uint
        != CON_CONNECTED as libc::c_int as libc::c_uint
    {
        return;
    }
    SetTeam(
        &mut g_entities[clientNum as usize],
        b"s\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=============
SortRanks

=============
*/
#[no_mangle]
pub unsafe extern "C" fn SortRanks(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ca: *mut gclient_t = 0 as *mut gclient_t;
    let mut cb: *mut gclient_t = 0 as *mut gclient_t;
    ca = &mut *level.clients.offset(*(a as *mut libc::c_int) as isize) as *mut gclient_s;
    cb = &mut *level.clients.offset(*(b as *mut libc::c_int) as isize) as *mut gclient_s;
    if (*ca).sess.spectatorState as libc::c_uint
        == SPECTATOR_SCOREBOARD as libc::c_int as libc::c_uint
        || (*ca).sess.spectatorClient < 0i32
    {
        return 1i32;
    }
    if (*cb).sess.spectatorState as libc::c_uint
        == SPECTATOR_SCOREBOARD as libc::c_int as libc::c_uint
        || (*cb).sess.spectatorClient < 0i32
    {
        return -1i32;
    }
    if (*ca).pers.connected as libc::c_uint == CON_CONNECTING as libc::c_int as libc::c_uint {
        return 1i32;
    }
    if (*cb).pers.connected as libc::c_uint == CON_CONNECTING as libc::c_int as libc::c_uint {
        return -1i32;
    }
    if (*ca).sess.sessionTeam as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint
        && (*cb).sess.sessionTeam as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        if (*ca).sess.spectatorNum > (*cb).sess.spectatorNum {
            return -1i32;
        }
        if (*ca).sess.spectatorNum < (*cb).sess.spectatorNum {
            return 1i32;
        }
        return 0i32;
    }
    if (*ca).sess.sessionTeam as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint {
        return 1i32;
    }
    if (*cb).sess.sessionTeam as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint {
        return -1i32;
    }
    if (*ca).ps.persistant[PERS_SCORE as libc::c_int as usize]
        > (*cb).ps.persistant[PERS_SCORE as libc::c_int as usize]
    {
        return -1i32;
    }
    if (*ca).ps.persistant[PERS_SCORE as libc::c_int as usize]
        < (*cb).ps.persistant[PERS_SCORE as libc::c_int as usize]
    {
        return 1i32;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn SetLeader(mut team: libc::c_int, mut client: libc::c_int) {
    let mut i: libc::c_int = 0;
    if (*level.clients.offset(client as isize)).pers.connected as libc::c_uint
        == CON_DISCONNECTED as libc::c_int as libc::c_uint
    {
        PrintTeam(
            team,
            va(
                b"print \"%s is not connected\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*level.clients.offset(client as isize))
                    .pers
                    .netname
                    .as_mut_ptr(),
            ),
        );
        return;
    }
    if (*level.clients.offset(client as isize)).sess.sessionTeam as libc::c_uint
        != team as libc::c_uint
    {
        PrintTeam(
            team,
            va(
                b"print \"%s is not on the team anymore\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*level.clients.offset(client as isize))
                    .pers
                    .netname
                    .as_mut_ptr(),
            ),
        );
        return;
    }
    i = 0i32;
    while i < level.maxclients {
        if !((*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
            != team as libc::c_uint)
        {
            if 0 != (*level.clients.offset(i as isize)).sess.teamLeader as u64 {
                (*level.clients.offset(i as isize)).sess.teamLeader = qfalse;
                ClientUserinfoChanged(i);
            }
        }
        i += 1
    }
    (*level.clients.offset(client as isize)).sess.teamLeader = qtrue;
    ClientUserinfoChanged(client);
    PrintTeam(
        team,
        va(
            b"print \"%s is the new team leader\n\"\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*level.clients.offset(client as isize))
                .pers
                .netname
                .as_mut_ptr(),
        ),
    );
}
/*
==================
PrintTeam
==================
*/
#[no_mangle]
pub unsafe extern "C" fn PrintTeam(mut team: libc::c_int, mut message: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < level.maxclients {
        if !((*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
            != team as libc::c_uint)
        {
            trap_SendServerCommand(i, message);
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn CheckTeamLeader(mut team: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < level.maxclients {
        if !((*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
            != team as libc::c_uint)
        {
            if 0 != (*level.clients.offset(i as isize)).sess.teamLeader as u64 {
                break;
            }
        }
        i += 1
    }
    if i >= level.maxclients {
        i = 0i32;
        while i < level.maxclients {
            if !((*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                != team as libc::c_uint)
            {
                if 0 == g_entities[i as usize].r.svFlags & 0x8i32 {
                    (*level.clients.offset(i as isize)).sess.teamLeader = qtrue;
                    break;
                }
            }
            i += 1
        }
        if i >= level.maxclients {
            i = 0i32;
            while i < level.maxclients {
                if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                    != team as libc::c_uint
                {
                    i += 1
                } else {
                    (*level.clients.offset(i as isize)).sess.teamLeader = qtrue;
                    break;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn G_RunThink(mut ent: *mut gentity_t) {
    let mut thinktime: libc::c_int = 0;
    thinktime = (*ent).nextthink;
    if thinktime <= 0i32 {
        return;
    }
    if thinktime > level.time {
        return;
    }
    (*ent).nextthink = 0i32;
    if (*ent).think.is_none() {
        G_Error(b"NULL ent->think\x00" as *const u8 as *const libc::c_char);
    }
    (*ent).think.expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn AddTournamentQueue(mut client: *mut gclient_t) {
    let mut index: libc::c_int = 0;
    let mut curclient: *mut gclient_t = 0 as *mut gclient_t;
    index = 0i32;
    while index < level.maxclients {
        curclient = &mut *level.clients.offset(index as isize) as *mut gclient_s;
        if (*curclient).pers.connected as libc::c_uint
            != CON_DISCONNECTED as libc::c_int as libc::c_uint
        {
            if curclient == client {
                (*curclient).sess.spectatorNum = 0i32
            } else if (*curclient).sess.sessionTeam as libc::c_uint
                == TEAM_SPECTATOR as libc::c_int as libc::c_uint
            {
                (*curclient).sess.spectatorNum += 1
            }
        }
        index += 1
    }
}
#[no_mangle]
pub static mut g_dedicated: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_cheats: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
// allow this many active
#[no_mangle]
pub static mut g_maxGameClients: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_restarted: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_dmflags: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_friendlyFire: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_password: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_needpass: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_gravity: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_speed: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_knockback: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_quadfactor: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_forcerespawn: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_inactivity: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_debugMove: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_debugAlloc: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_debugDamage: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_weaponRespawn: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_weaponTeamRespawn: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_synchronousClients: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_motd: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_warmup: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_doWarmup: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_blood: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_allowVote: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_teamAutoJoin: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_teamForceBalance: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_banIPs: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_filterBan: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_smoothClients: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut pmove_fixed: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut pmove_msec: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_rankings: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_localTeamPref: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_clients: [gclient_t; 64] = [gclient_s {
    ps: playerState_s {
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
    },
    pers: clientPersistant_t {
        connected: CON_DISCONNECTED,
        cmd: usercmd_s {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        },
        localClient: qfalse,
        initialSpawn: qfalse,
        predictItemPickup: qfalse,
        pmoveFixed: qfalse,
        netname: [0; 36],
        maxHealth: 0,
        enterTime: 0,
        teamState: playerTeamState_t {
            state: TEAM_BEGIN,
            location: 0,
            captures: 0,
            basedefense: 0,
            carrierdefense: 0,
            flagrecovery: 0,
            fragcarrier: 0,
            assists: 0,
            lasthurtcarrier: 0.,
            lastreturnedflag: 0.,
            flagsince: 0.,
            lastfraggedcarrier: 0.,
        },
        voteCount: 0,
        teamVoteCount: 0,
        teamInfo: qfalse,
    },
    sess: clientSession_t {
        sessionTeam: TEAM_FREE,
        spectatorNum: 0,
        spectatorState: SPECTATOR_NOT,
        spectatorClient: 0,
        wins: 0,
        losses: 0,
        teamLeader: qfalse,
    },
    readyToExit: qfalse,
    noclip: qfalse,
    lastCmdTime: 0,
    buttons: 0,
    oldbuttons: 0,
    latched_buttons: 0,
    oldOrigin: [0.; 3],
    damage_armor: 0,
    damage_blood: 0,
    damage_knockback: 0,
    damage_from: [0.; 3],
    damage_fromWorld: qfalse,
    accurateCount: 0,
    accuracy_shots: 0,
    accuracy_hits: 0,
    lastkilled_client: 0,
    lasthurt_client: 0,
    lasthurt_mod: 0,
    respawnTime: 0,
    inactivityTime: 0,
    inactivityWarning: qfalse,
    rewardTime: 0,
    airOutTime: 0,
    lastKillTime: 0,
    fireHeld: qfalse,
    hook: 0 as *const gentity_t as *mut gentity_t,
    switchTeamTime: 0,
    timeResidual: 0,
    areabits: 0 as *const libc::c_char as *mut libc::c_char,
}; 64];
#[no_mangle]
pub static mut g_logfile: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_logfileSync: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_podiumDist: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_podiumDrop: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut g_listEntity: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
static mut gameCvarTable: [cvarTable_t; 46] = unsafe {
    [
        cvarTable_t {
            vmCvar: &g_cheats as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"sv_cheats\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: 0 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"gamename\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"baseq3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x40i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: 0 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"gamedate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"Feb 13 2019\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cvarFlags: 0x40i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_restarted as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_restarted\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x40i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_gametype as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_gametype\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x2i32 | 0x20i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_maxclients as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"sv_maxclients\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x20i32 | 0x1i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_maxGameClients as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_maxGameClients\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x20i32 | 0x1i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_dmflags as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"dmflags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x1i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_fraglimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"fraglimit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x1i32 | 0x400i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_timelimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"timelimit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x1i32 | 0x400i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_capturelimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"capturelimit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x1i32 | 0x400i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_synchronousClients as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_synchronousClients\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x8i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_friendlyFire as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_friendlyFire\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_teamAutoJoin as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_teamAutoJoin\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_teamForceBalance as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_teamForceBalance\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_warmup as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_warmup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_doWarmup as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_doWarmup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_logfile as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_log\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"games.log\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_logfileSync as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_logsync\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_password as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_password\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x2i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_banIPs as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_banIPs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_filterBan as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_filterBan\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_needpass as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_needpass\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x4i32 | 0x40i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_dedicated as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"dedicated\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_speed as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"320\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_gravity as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_gravity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"800\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_knockback as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_knockback\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1000\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_quadfactor as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_quadfactor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_weaponRespawn as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_weaponrespawn\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_weaponTeamRespawn as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_weaponTeamRespawn\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"30\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_forcerespawn as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_forcerespawn\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_inactivity as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_inactivity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qtrue,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_debugMove as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_debugMove\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_debugDamage as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_debugDamage\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_debugAlloc as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_debugAlloc\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_motd as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_motd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_blood as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"com_blood\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_podiumDist as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_podiumDist\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"80\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_podiumDrop as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_podiumDrop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"70\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_allowVote as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_allowVote\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_listEntity as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_listEntity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_smoothClients as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_smoothClients\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &pmove_fixed as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"pmove_fixed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x8i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &pmove_msec as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"pmove_msec\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x8i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_rankings as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_rankings\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
        cvarTable_t {
            vmCvar: &g_localTeamPref as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_localTeamPref\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
            modificationCount: 0i32,
            trackChange: qfalse,
            teamShader: qfalse,
        },
    ]
};
// Initialized in run_static_initializers
static mut gameCvarTableSize: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn G_InitGame(
    mut levelTime: libc::c_int,
    mut randomSeed: libc::c_int,
    mut restart: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    G_Printf(b"------- Game Initialization -------\n\x00" as *const u8 as *const libc::c_char);
    G_Printf(
        b"gamename: %s\n\x00" as *const u8 as *const libc::c_char,
        b"baseq3\x00" as *const u8 as *const libc::c_char,
    );
    G_Printf(
        b"gamedate: %s\n\x00" as *const u8 as *const libc::c_char,
        b"Feb 13 2019\x00" as *const u8 as *const libc::c_char,
    );
    srand(randomSeed as libc::c_uint);
    G_RegisterCvars();
    G_ProcessIPBans();
    G_InitMemory();
    memset(
        &mut level as *mut level_locals_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<level_locals_t>() as libc::c_ulong,
    );
    level.time = levelTime;
    level.startTime = levelTime;
    level.snd_fry = G_SoundIndex(
        b"sound/player/fry.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if g_gametype.integer != GT_SINGLE_PLAYER as libc::c_int
        && 0 != g_logfile.string[0usize] as libc::c_int
    {
        if 0 != g_logfileSync.integer {
            trap_FS_FOpenFile(
                g_logfile.string.as_mut_ptr(),
                &mut level.logFile,
                FS_APPEND_SYNC,
            );
        } else {
            trap_FS_FOpenFile(g_logfile.string.as_mut_ptr(), &mut level.logFile, FS_APPEND);
        }
        if 0 == level.logFile {
            G_Printf(
                b"WARNING: Couldn\'t open logfile: %s\n\x00" as *const u8 as *const libc::c_char,
                g_logfile.string.as_mut_ptr(),
            );
        } else {
            let mut serverinfo: [libc::c_char; 1024] = [0; 1024];
            trap_GetServerinfo(
                serverinfo.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            G_LogPrintf(
                b"------------------------------------------------------------\n\x00" as *const u8
                    as *const libc::c_char,
            );
            G_LogPrintf(
                b"InitGame: %s\n\x00" as *const u8 as *const libc::c_char,
                serverinfo.as_mut_ptr(),
            );
        }
    } else {
        G_Printf(b"Not logging to disk.\n\x00" as *const u8 as *const libc::c_char);
    }
    G_InitWorldSession();
    memset(
        g_entities.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ((1i32 << 10i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<gentity_t>() as libc::c_ulong),
    );
    level.gentities = g_entities.as_mut_ptr();
    level.maxclients = g_maxclients.integer;
    memset(
        g_clients.as_mut_ptr() as *mut libc::c_void,
        0i32,
        (64i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<gclient_t>() as libc::c_ulong),
    );
    level.clients = g_clients.as_mut_ptr();
    i = 0i32;
    while i < level.maxclients {
        g_entities[i as usize].client = level.clients.offset(i as isize);
        i += 1
    }
    level.num_entities = 64i32;
    i = 0i32;
    while i < 64i32 {
        g_entities[i as usize].classname =
            b"clientslot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        i += 1
    }
    trap_LocateGameData(
        level.gentities,
        level.num_entities,
        ::std::mem::size_of::<gentity_t>() as libc::c_ulong as libc::c_int,
        &mut (*level.clients.offset(0isize)).ps,
        ::std::mem::size_of::<gclient_s>() as libc::c_ulong as libc::c_int,
    );
    InitBodyQue();
    ClearRegisteredItems();
    G_SpawnEntitiesFromString();
    G_FindTeams();
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        G_CheckTeamItems();
    }
    SaveRegisteredItems();
    G_Printf(b"-----------------------------------\n\x00" as *const u8 as *const libc::c_char);
    if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int
        || 0 != trap_Cvar_VariableIntegerValue(
            b"com_buildScript\x00" as *const u8 as *const libc::c_char,
        )
    {
        G_ModelIndex(
            b"models/mapobjects/podium/podium4.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if 0 != trap_Cvar_VariableIntegerValue(b"bot_enable\x00" as *const u8 as *const libc::c_char) {
        BotAISetup(restart);
        BotAILoadMap(restart);
        G_InitBots(restart as qboolean);
    }
    G_RemapTeamShaders();
    trap_SetConfigstring(22i32, b"\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn G_RemapTeamShaders() {}
/*
================
G_FindTeams

Chain together all entities with a matching team field.
Entity teams are used for item groups and multi-entity mover groups.

All but the first will have the FL_TEAMSLAVE flag set and teammaster field set
All but the last will have the teamchain field set to the next one
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_FindTeams() {
    let mut e: *mut gentity_t = 0 as *mut gentity_t;
    let mut e2: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    c = 0i32;
    c2 = 0i32;
    i = 64i32;
    e = g_entities.as_mut_ptr().offset(i as isize);
    while i < level.num_entities {
        if !(0 == (*e).inuse as u64) {
            if !(*e).team.is_null() {
                if !(0 != (*e).flags & 0x400i32) {
                    (*e).teammaster = e;
                    c += 1;
                    c2 += 1;
                    j = i + 1i32;
                    e2 = e.offset(1isize);
                    while j < level.num_entities {
                        if !(0 == (*e2).inuse as u64) {
                            if !(*e2).team.is_null() {
                                if !(0 != (*e2).flags & 0x400i32) {
                                    if 0 == strcmp((*e).team, (*e2).team) {
                                        c2 += 1;
                                        (*e2).teamchain = (*e).teamchain;
                                        (*e).teamchain = e2;
                                        (*e2).teammaster = e;
                                        (*e2).flags |= 0x400i32;
                                        if !(*e2).targetname.is_null() {
                                            (*e).targetname = (*e2).targetname;
                                            (*e2).targetname = 0 as *mut libc::c_char
                                        }
                                    }
                                }
                            }
                        }
                        j += 1;
                        e2 = e2.offset(1isize)
                    }
                }
            }
        }
        i += 1;
        e = e.offset(1isize)
    }
    G_Printf(
        b"%i teams with %i entities\n\x00" as *const u8 as *const libc::c_char,
        c,
        c2,
    );
}
/*
=================
G_RegisterCvars
=================
*/
#[no_mangle]
pub unsafe extern "C" fn G_RegisterCvars() {
    let mut i: libc::c_int = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    let mut remapped: qboolean = qfalse;
    i = 0i32;
    cv = gameCvarTable.as_mut_ptr();
    while i < gameCvarTableSize {
        trap_Cvar_Register(
            (*cv).vmCvar,
            (*cv).cvarName,
            (*cv).defaultString,
            (*cv).cvarFlags,
        );
        if !(*cv).vmCvar.is_null() {
            (*cv).modificationCount = (*(*cv).vmCvar).modificationCount
        }
        if 0 != (*cv).teamShader as u64 {
            remapped = qtrue
        }
        i += 1;
        cv = cv.offset(1isize)
    }
    if 0 != remapped as u64 {
        G_RemapTeamShaders();
    }
    if g_gametype.integer < 0i32 || g_gametype.integer >= GT_MAX_GAME_TYPE as libc::c_int {
        G_Printf(
            b"g_gametype %i is out of range, defaulting to 0\n\x00" as *const u8
                as *const libc::c_char,
            g_gametype.integer,
        );
        trap_Cvar_Set(
            b"g_gametype\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut g_gametype);
    }
    level.warmupModificationCount = g_warmup.modificationCount;
}
#[no_mangle]
pub unsafe extern "C" fn G_RunFrame(mut levelTime: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    if 0 != level.restarted as u64 {
        return;
    }
    level.framenum += 1;
    level.previousTime = level.time;
    level.time = levelTime;
    G_UpdateCvars();
    ent = &mut g_entities[0usize] as *mut gentity_t;
    let mut current_block_24: u64;
    i = 0i32;
    while i < level.num_entities {
        if !(0 == (*ent).inuse as u64) {
            // clear events that are too old
            if level.time - (*ent).eventTime > 300i32 {
                if 0 != (*ent).s.event {
                    (*ent).s.event = 0i32;
                    if !(*ent).client.is_null() {
                        (*(*ent).client).ps.externalEvent = 0i32
                    }
                }
                // predicted events should never be set to zero
                //ent->client->ps.events[0] = 0;
                //ent->client->ps.events[1] = 0;
                if 0 != (*ent).freeAfterEvent as u64 {
                    G_FreeEntity(ent);
                    current_block_24 = 17216689946888361452;
                } else {
                    if 0 != (*ent).unlinkAfterEvent as u64 {
                        (*ent).unlinkAfterEvent = qfalse;
                        trap_UnlinkEntity(ent);
                    }
                    current_block_24 = 17478428563724192186;
                }
            } else {
                current_block_24 = 17478428563724192186;
            }
            match current_block_24 {
                17216689946888361452 => {}
                _ => {
                    // temporary entities don't think
                    if !(0 != (*ent).freeAfterEvent as u64) {
                        if !(0 == (*ent).r.linked as u64 && 0 != (*ent).neverFree as libc::c_uint) {
                            if (*ent).s.eType == ET_MISSILE as libc::c_int {
                                G_RunMissile(ent);
                            } else if (*ent).s.eType == ET_ITEM as libc::c_int
                                || 0 != (*ent).physicsObject as libc::c_uint
                            {
                                G_RunItem(ent);
                            } else if (*ent).s.eType == ET_MOVER as libc::c_int {
                                G_RunMover(ent);
                            } else if i < 64i32 {
                                G_RunClient(ent);
                            } else {
                                G_RunThink(ent);
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        ent = ent.offset(1isize)
    }
    ent = &mut g_entities[0usize] as *mut gentity_t;
    i = 0i32;
    while i < level.maxclients {
        if 0 != (*ent).inuse as u64 {
            ClientEndFrame(ent);
        }
        i += 1;
        ent = ent.offset(1isize)
    }
    CheckTournament();
    CheckExitRules();
    CheckTeamStatus();
    CheckVote();
    CheckTeamVote(TEAM_RED as libc::c_int);
    CheckTeamVote(TEAM_BLUE as libc::c_int);
    CheckCvars();
    if 0 != g_listEntity.integer {
        i = 0i32;
        while i < 1i32 << 10i32 {
            G_Printf(
                b"%4i: %s\n\x00" as *const u8 as *const libc::c_char,
                i,
                g_entities[i as usize].classname,
            );
            i += 1
        }
        trap_Cvar_Set(
            b"g_listEntity\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    };
}
/*
==================
CheckCvars
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CheckCvars() {
    static mut lastMod: libc::c_int = -1i32;
    if g_password.modificationCount != lastMod {
        lastMod = g_password.modificationCount;
        if 0 != *g_password.string.as_mut_ptr() as libc::c_int
            && 0 != Q_stricmp(
                g_password.string.as_mut_ptr(),
                b"none\x00" as *const u8 as *const libc::c_char,
            )
        {
            trap_Cvar_Set(
                b"g_needpass\x00" as *const u8 as *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            trap_Cvar_Set(
                b"g_needpass\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
/*
==================
CheckTeamVote
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CheckTeamVote(mut team: libc::c_int) {
    let mut cs_offset: libc::c_int = 0;
    if team == TEAM_RED as libc::c_int {
        cs_offset = 0i32
    } else if team == TEAM_BLUE as libc::c_int {
        cs_offset = 1i32
    } else {
        return;
    }
    if 0 == level.teamVoteTime[cs_offset as usize] {
        return;
    }
    if level.time - level.teamVoteTime[cs_offset as usize] >= 30000i32 {
        trap_SendServerCommand(
            -1i32,
            b"print \"Team vote failed.\n\"\x00" as *const u8 as *const libc::c_char,
        );
    } else if level.teamVoteYes[cs_offset as usize]
        > level.numteamVotingClients[cs_offset as usize] / 2i32
    {
        trap_SendServerCommand(
            -1i32,
            b"print \"Team vote passed.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        if 0 == Q_strncmp(
            b"leader\x00" as *const u8 as *const libc::c_char,
            level.teamVoteString[cs_offset as usize].as_mut_ptr(),
            6i32,
        ) {
            SetLeader(
                team,
                atoi(
                    level.teamVoteString[cs_offset as usize]
                        .as_mut_ptr()
                        .offset(7isize),
                ),
            );
        } else {
            trap_SendConsoleCommand(
                EXEC_APPEND as libc::c_int,
                va(
                    b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    level.teamVoteString[cs_offset as usize].as_mut_ptr(),
                ),
            );
        }
    } else if level.teamVoteNo[cs_offset as usize]
        >= level.numteamVotingClients[cs_offset as usize] / 2i32
    {
        trap_SendServerCommand(
            -1i32,
            b"print \"Team vote failed.\n\"\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        return;
    }
    level.teamVoteTime[cs_offset as usize] = 0i32;
    trap_SetConfigstring(
        12i32 + cs_offset,
        b"\x00" as *const u8 as *const libc::c_char,
    );
}
/*
==================
CheckVote
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CheckVote() {
    if 0 != level.voteExecuteTime && level.voteExecuteTime < level.time {
        level.voteExecuteTime = 0i32;
        trap_SendConsoleCommand(
            EXEC_APPEND as libc::c_int,
            va(
                b"%s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.voteString.as_mut_ptr(),
            ),
        );
    }
    if 0 == level.voteTime {
        return;
    }
    if level.time - level.voteTime >= 30000i32 {
        trap_SendServerCommand(
            -1i32,
            b"print \"Vote failed.\n\"\x00" as *const u8 as *const libc::c_char,
        );
    } else if level.voteYes > level.numVotingClients / 2i32 {
        trap_SendServerCommand(
            -1i32,
            b"print \"Vote passed.\n\"\x00" as *const u8 as *const libc::c_char,
        );
        level.voteExecuteTime = level.time + 3000i32
    } else if level.voteNo >= level.numVotingClients / 2i32 {
        trap_SendServerCommand(
            -1i32,
            b"print \"Vote failed.\n\"\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        return;
    }
    level.voteTime = 0i32;
    trap_SetConfigstring(8i32, b"\x00" as *const u8 as *const libc::c_char);
}
/*
========================================================================

FUNCTIONS CALLED EVERY FRAME

========================================================================
*/
/*
=============
CheckTournament

Once a frame, check for changes in tournement player state
=============
*/
#[no_mangle]
pub unsafe extern "C" fn CheckTournament() {
    if level.numPlayingClients == 0i32 {
        return;
    }
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int {
        if level.numPlayingClients < 2i32 {
            AddTournamentPlayer();
        }
        if level.numPlayingClients != 2i32 {
            if level.warmupTime != -1i32 {
                level.warmupTime = -1i32;
                trap_SetConfigstring(
                    5i32,
                    va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        level.warmupTime,
                    ),
                );
                G_LogPrintf(b"Warmup:\n\x00" as *const u8 as *const libc::c_char);
            }
            return;
        }
        if level.warmupTime == 0i32 {
            return;
        }
        if g_warmup.modificationCount != level.warmupModificationCount {
            level.warmupModificationCount = g_warmup.modificationCount;
            level.warmupTime = -1i32
        }
        if level.warmupTime < 0i32 {
            if level.numPlayingClients == 2i32 {
                if g_warmup.integer > 1i32 {
                    level.warmupTime = level.time + (g_warmup.integer - 1i32) * 1000i32
                } else {
                    level.warmupTime = 0i32
                }
                trap_SetConfigstring(
                    5i32,
                    va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        level.warmupTime,
                    ),
                );
            }
            return;
        }
        if level.time > level.warmupTime {
            level.warmupTime += 10000i32;
            trap_Cvar_Set(
                b"g_restarted\x00" as *const u8 as *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char,
            );
            trap_SendConsoleCommand(
                EXEC_APPEND as libc::c_int,
                b"map_restart 0\n\x00" as *const u8 as *const libc::c_char,
            );
            level.restarted = qtrue;
            return;
        }
    } else if g_gametype.integer != GT_SINGLE_PLAYER as libc::c_int && level.warmupTime != 0i32 {
        let mut counts: [libc::c_int; 4] = [0; 4];
        let mut notEnough: qboolean = qfalse;
        if g_gametype.integer >= GT_TEAM as libc::c_int {
            counts[TEAM_BLUE as libc::c_int as usize] = TeamCount(-1i32, TEAM_BLUE);
            counts[TEAM_RED as libc::c_int as usize] = TeamCount(-1i32, TEAM_RED);
            if counts[TEAM_RED as libc::c_int as usize] < 1i32
                || counts[TEAM_BLUE as libc::c_int as usize] < 1i32
            {
                notEnough = qtrue
            }
        } else if level.numPlayingClients < 2i32 {
            notEnough = qtrue
        }
        if 0 != notEnough as u64 {
            if level.warmupTime != -1i32 {
                level.warmupTime = -1i32;
                trap_SetConfigstring(
                    5i32,
                    va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        level.warmupTime,
                    ),
                );
                G_LogPrintf(b"Warmup:\n\x00" as *const u8 as *const libc::c_char);
            }
            return;
        }
        if level.warmupTime == 0i32 {
            return;
        }
        if g_warmup.modificationCount != level.warmupModificationCount {
            level.warmupModificationCount = g_warmup.modificationCount;
            level.warmupTime = -1i32
        }
        if level.warmupTime < 0i32 {
            if g_warmup.integer > 1i32 {
                level.warmupTime = level.time + (g_warmup.integer - 1i32) * 1000i32
            } else {
                level.warmupTime = 0i32
            }
            trap_SetConfigstring(
                5i32,
                va(
                    b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    level.warmupTime,
                ),
            );
            return;
        }
        if level.time > level.warmupTime {
            level.warmupTime += 10000i32;
            trap_Cvar_Set(
                b"g_restarted\x00" as *const u8 as *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char,
            );
            trap_SendConsoleCommand(
                EXEC_APPEND as libc::c_int,
                b"map_restart 0\n\x00" as *const u8 as *const libc::c_char,
            );
            level.restarted = qtrue;
            return;
        }
    };
}
//===================================================================
/*
========================================================================

PLAYER COUNTING / SCORE SORTING

========================================================================
*/
/*
=============
AddTournamentPlayer

If there are less than two tournament players, put a
spectator in the game and restart
=============
*/
#[no_mangle]
pub unsafe extern "C" fn AddTournamentPlayer() {
    let mut i: libc::c_int = 0;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut nextInLine: *mut gclient_t = 0 as *mut gclient_t;
    if level.numPlayingClients >= 2i32 {
        return;
    }
    if 0 != level.intermissiontime {
        return;
    }
    nextInLine = 0 as *mut gclient_t;
    i = 0i32;
    while i < level.maxclients {
        client = &mut *level.clients.offset(i as isize) as *mut gclient_s;
        if !((*client).pers.connected as libc::c_uint
            != CON_CONNECTED as libc::c_int as libc::c_uint)
        {
            if !((*client).sess.sessionTeam as libc::c_uint
                != TEAM_SPECTATOR as libc::c_int as libc::c_uint)
            {
                // never select the dedicated follow or scoreboard clients
                if !((*client).sess.spectatorState as libc::c_uint
                    == SPECTATOR_SCOREBOARD as libc::c_int as libc::c_uint
                    || (*client).sess.spectatorClient < 0i32)
                {
                    if nextInLine.is_null()
                        || (*client).sess.spectatorNum > (*nextInLine).sess.spectatorNum
                    {
                        nextInLine = client
                    }
                }
            }
        }
        i += 1
    }
    if nextInLine.is_null() {
        return;
    }
    level.warmupTime = -1i32;
    SetTeam(
        &mut g_entities[nextInLine.wrapping_offset_from(level.clients) as libc::c_long as usize],
        b"f\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
G_UpdateCvars
=================
*/
#[no_mangle]
pub unsafe extern "C" fn G_UpdateCvars() {
    let mut i: libc::c_int = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    let mut remapped: qboolean = qfalse;
    i = 0i32;
    cv = gameCvarTable.as_mut_ptr();
    while i < gameCvarTableSize {
        if !(*cv).vmCvar.is_null() {
            trap_Cvar_Update((*cv).vmCvar);
            if (*cv).modificationCount != (*(*cv).vmCvar).modificationCount {
                (*cv).modificationCount = (*(*cv).vmCvar).modificationCount;
                if 0 != (*cv).trackChange as u64 {
                    trap_SendServerCommand(
                        -1i32,
                        va(
                            b"print \"Server: %s changed to %s\n\"\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            (*cv).cvarName,
                            (*(*cv).vmCvar).string.as_mut_ptr(),
                        ),
                    );
                }
                if 0 != (*cv).teamShader as u64 {
                    remapped = qtrue
                }
            }
        }
        i += 1;
        cv = cv.offset(1isize)
    }
    if 0 != remapped as u64 {
        G_RemapTeamShaders();
    };
}
#[no_mangle]
pub unsafe extern "C" fn G_ShutdownGame(mut restart: libc::c_int) {
    G_Printf(b"==== ShutdownGame ====\n\x00" as *const u8 as *const libc::c_char);
    if 0 != level.logFile {
        G_LogPrintf(b"ShutdownGame:\n\x00" as *const u8 as *const libc::c_char);
        G_LogPrintf(
            b"------------------------------------------------------------\n\x00" as *const u8
                as *const libc::c_char,
        );
        trap_FS_FCloseFile(level.logFile);
        level.logFile = 0i32
    }
    G_WriteSessionData();
    if 0 != trap_Cvar_VariableIntegerValue(b"bot_enable\x00" as *const u8 as *const libc::c_char) {
        BotAIShutdown(restart);
    };
}
/*
================
vmMain

This is the only way control passes into the module.
This must be the very first function compiled into the .q3vm file
================
*/
#[no_mangle]
pub unsafe extern "C" fn vmMain(
    mut command: libc::c_int,
    mut arg0: libc::c_int,
    mut arg1: libc::c_int,
    mut arg2: libc::c_int,
    mut arg3: libc::c_int,
    mut arg4: libc::c_int,
    mut arg5: libc::c_int,
    mut arg6: libc::c_int,
    mut arg7: libc::c_int,
    mut arg8: libc::c_int,
    mut arg9: libc::c_int,
    mut arg10: libc::c_int,
    mut arg11: libc::c_int,
) -> intptr_t {
    match command {
        0 => {
            G_InitGame(arg0, arg1, arg2);
            return 0i32 as intptr_t;
        }
        1 => {
            G_ShutdownGame(arg0);
            return 0i32 as intptr_t;
        }
        2 => return ClientConnect(arg0, arg1 as qboolean, arg2 as qboolean) as intptr_t,
        7 => {
            ClientThink(arg0);
            return 0i32 as intptr_t;
        }
        4 => {
            ClientUserinfoChanged(arg0);
            return 0i32 as intptr_t;
        }
        5 => {
            ClientDisconnect(arg0);
            return 0i32 as intptr_t;
        }
        3 => {
            ClientBegin(arg0);
            return 0i32 as intptr_t;
        }
        6 => {
            ClientCommand(arg0);
            return 0i32 as intptr_t;
        }
        8 => {
            G_RunFrame(arg0);
            return 0i32 as intptr_t;
        }
        9 => return ConsoleCommand() as intptr_t,
        10 => return BotAIStartFrame(arg0) as intptr_t,
        _ => {}
    }
    return -1i32 as intptr_t;
}
/*
=======================
RemoveTournamentWinner
=======================
*/
#[no_mangle]
pub unsafe extern "C" fn RemoveTournamentWinner() {
    let mut clientNum: libc::c_int = 0;
    if level.numPlayingClients != 2i32 {
        return;
    }
    clientNum = level.sortedClients[0usize];
    if (*level.clients.offset(clientNum as isize)).pers.connected as libc::c_uint
        != CON_CONNECTED as libc::c_int as libc::c_uint
    {
        return;
    }
    SetTeam(
        &mut g_entities[clientNum as usize],
        b"s\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn run_static_initializers() {
    gameCvarTableSize = (::std::mem::size_of::<[cvarTable_t; 46]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cvarTable_t>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvarTable_t {
    pub vmCvar: *mut vmCvar_t,
    pub cvarName: *mut libc::c_char,
    pub defaultString: *mut libc::c_char,
    pub cvarFlags: libc::c_int,
    pub modificationCount: libc::c_int,
    pub trackChange: qboolean,
    pub teamShader: qboolean,
}
