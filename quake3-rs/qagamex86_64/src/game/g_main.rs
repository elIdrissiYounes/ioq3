use ::libc;

pub mod stdlib_h {

    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::src::game::g_main::stdlib_h::atoi;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__compar_fn_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::qsort;
pub use crate::stdlib::srand;
pub use crate::stdlib::strtol;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::ET_BEAM;
pub use crate::bg_public_h::ET_EVENTS;
pub use crate::bg_public_h::ET_GENERAL;
pub use crate::bg_public_h::ET_GRAPPLE;
pub use crate::bg_public_h::ET_INVISIBLE;
pub use crate::bg_public_h::ET_ITEM;
pub use crate::bg_public_h::ET_MISSILE;
pub use crate::bg_public_h::ET_MOVER;
pub use crate::bg_public_h::ET_PLAYER;
pub use crate::bg_public_h::ET_PORTAL;
pub use crate::bg_public_h::ET_PUSH_TRIGGER;
pub use crate::bg_public_h::ET_SPEAKER;
pub use crate::bg_public_h::ET_TEAM;
pub use crate::bg_public_h::ET_TELEPORT_TRIGGER;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::PERS_ASSIST_COUNT;
pub use crate::bg_public_h::PERS_ATTACKEE_ARMOR;
pub use crate::bg_public_h::PERS_ATTACKER;
pub use crate::bg_public_h::PERS_CAPTURES;
pub use crate::bg_public_h::PERS_DEFEND_COUNT;
pub use crate::bg_public_h::PERS_EXCELLENT_COUNT;
pub use crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT;
pub use crate::bg_public_h::PERS_HITS;
pub use crate::bg_public_h::PERS_IMPRESSIVE_COUNT;
pub use crate::bg_public_h::PERS_KILLED;
pub use crate::bg_public_h::PERS_PLAYEREVENTS;
pub use crate::bg_public_h::PERS_RANK;
pub use crate::bg_public_h::PERS_SCORE;
pub use crate::bg_public_h::PERS_SPAWN_COUNT;
pub use crate::bg_public_h::PERS_TEAM;
pub use crate::bg_public_h::PM_DEAD;
pub use crate::bg_public_h::PM_FREEZE;
pub use crate::bg_public_h::PM_INTERMISSION;
pub use crate::bg_public_h::PM_NOCLIP;
pub use crate::bg_public_h::PM_NORMAL;
pub use crate::bg_public_h::PM_SPECTATOR;
pub use crate::bg_public_h::PM_SPINTERMISSION;
pub use crate::bg_public_h::STAT_ARMOR;
pub use crate::bg_public_h::STAT_CLIENTS_READY;
pub use crate::bg_public_h::STAT_DEAD_YAW;
pub use crate::bg_public_h::STAT_HEALTH;
pub use crate::bg_public_h::STAT_HOLDABLE_ITEM;
pub use crate::bg_public_h::STAT_MAX_HEALTH;
pub use crate::bg_public_h::STAT_WEAPONS;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
pub use crate::g_local_h::gclient_t;
pub use crate::g_local_h::gentity_s;
pub use crate::g_local_h::gentity_t;
pub use crate::g_local_h::level_locals_t;
pub use crate::g_local_h::moverState_t;
pub use crate::g_local_h::playerTeamStateState_t;
pub use crate::g_local_h::playerTeamState_t;
pub use crate::g_local_h::spectatorState_t;
pub use crate::g_local_h::CON_CONNECTED;
pub use crate::g_local_h::CON_CONNECTING;
pub use crate::g_local_h::CON_DISCONNECTED;
pub use crate::g_local_h::MOVER_1TO2;
pub use crate::g_local_h::MOVER_2TO1;
pub use crate::g_local_h::MOVER_POS1;
pub use crate::g_local_h::MOVER_POS2;
pub use crate::g_local_h::SPECTATOR_FOLLOW;
pub use crate::g_local_h::SPECTATOR_FREE;
pub use crate::g_local_h::SPECTATOR_NOT;
pub use crate::g_local_h::SPECTATOR_SCOREBOARD;
pub use crate::g_local_h::TEAM_ACTIVE;
pub use crate::g_local_h::TEAM_BEGIN;
pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::BOTAI_START_FRAME;
pub use crate::g_public_h::GAME_CLIENT_BEGIN;
pub use crate::g_public_h::GAME_CLIENT_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_CONNECT;
pub use crate::g_public_h::GAME_CLIENT_DISCONNECT;
pub use crate::g_public_h::GAME_CLIENT_THINK;
pub use crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED;
pub use crate::g_public_h::GAME_CONSOLE_COMMAND;
pub use crate::g_public_h::GAME_INIT;
pub use crate::g_public_h::GAME_RUN_FRAME;
pub use crate::g_public_h::GAME_SHUTDOWN;
pub use crate::src::game::ai_main::BotAILoadMap;
pub use crate::src::game::ai_main::BotAISetup;
pub use crate::src::game::ai_main::BotAIShutdown;
pub use crate::src::game::ai_main::BotAIStartFrame;
pub use crate::src::game::ai_main::BotInterbreedEndMatch;
pub use crate::src::game::g_active::ClientEndFrame;
pub use crate::src::game::g_active::ClientThink;
pub use crate::src::game::g_active::G_RunClient;
pub use crate::src::game::g_arenas::SpawnModelsOnVictoryPads;
pub use crate::src::game::g_arenas::UpdateTournamentInfo;
pub use crate::src::game::g_bot::G_InitBots;
pub use crate::src::game::g_client::ClientBegin;
pub use crate::src::game::g_client::ClientConnect;
pub use crate::src::game::g_client::ClientDisconnect;
pub use crate::src::game::g_client::ClientRespawn;
pub use crate::src::game::g_client::ClientUserinfoChanged;
pub use crate::src::game::g_client::InitBodyQue;
pub use crate::src::game::g_client::SelectSpawnPoint;
pub use crate::src::game::g_client::TeamCount;
pub use crate::src::game::g_cmds::ClientCommand;
pub use crate::src::game::g_cmds::DeathmatchScoreboardMessage;
pub use crate::src::game::g_cmds::SetTeam;
pub use crate::src::game::g_cmds::StopFollowing;
pub use crate::src::game::g_items::ClearRegisteredItems;
pub use crate::src::game::g_items::G_CheckTeamItems;
pub use crate::src::game::g_items::G_RunItem;
pub use crate::src::game::g_items::SaveRegisteredItems;
pub use crate::src::game::g_mem::G_InitMemory;
pub use crate::src::game::g_missile::G_RunMissile;
pub use crate::src::game::g_mover::G_RunMover;
pub use crate::src::game::g_session::G_InitWorldSession;
pub use crate::src::game::g_session::G_WriteSessionData;
pub use crate::src::game::g_spawn::G_SpawnEntitiesFromString;
pub use crate::src::game::g_svcmds::ConsoleCommand;
pub use crate::src::game::g_svcmds::G_ProcessIPBans;
pub use crate::src::game::g_syscalls::trap_Cvar_Register;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_Cvar_Update;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::game::g_syscalls::trap_Error;
pub use crate::src::game::g_syscalls::trap_FS_FCloseFile;
pub use crate::src::game::g_syscalls::trap_FS_FOpenFile;
pub use crate::src::game::g_syscalls::trap_FS_Write;
pub use crate::src::game::g_syscalls::trap_GetServerinfo;
pub use crate::src::game::g_syscalls::trap_LocateGameData;
pub use crate::src::game::g_syscalls::trap_Print;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;
use crate::src::game::g_team::CheckTeamStatus;
pub use crate::src::game::g_utils::G_Find;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_ModelIndex;
pub use crate::src::game::g_utils::G_PickTarget;
pub use crate::src::game::g_utils::G_SoundIndex;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncmp;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
use crate::stdlib::memset;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
use crate::stdlib::vsnprintf;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvarTable_t {
    pub vmCvar: *mut crate::src::qcommon::q_shared::vmCvar_t,
    pub cvarName: *mut i8,
    pub defaultString: *mut i8,
    pub cvarFlags: i32,
    pub modificationCount: i32,
    pub trackChange: crate::src::qcommon::q_shared::qboolean,
    pub teamShader: crate::src::qcommon::q_shared::qboolean,
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
#[no_mangle]

pub static mut level: crate::g_local_h::level_locals_t = crate::g_local_h::level_locals_t {
    clients: 0 as *mut crate::g_local_h::gclient_s,
    gentities: 0 as *mut crate::g_local_h::gentity_s,
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
    newSession: crate::src::qcommon::q_shared::qfalse,
    restarted: crate::src::qcommon::q_shared::qfalse,
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
    spawning: crate::src::qcommon::q_shared::qfalse,
    numSpawnVars: 0,
    spawnVars: [[0 as *mut i8; 2]; 64],
    numSpawnVarChars: 0,
    spawnVarChars: [0; 4096],
    intermissionQueued: 0,
    intermissiontime: 0,
    changemap: 0 as *mut i8,
    readyToExit: crate::src::qcommon::q_shared::qfalse,
    exitTime: 0,
    intermission_origin: [0.; 3],
    intermission_angle: [0.; 3],
    locationLinked: crate::src::qcommon::q_shared::qfalse,
    locationHead: 0 as *mut crate::g_local_h::gentity_t,
    bodyQueIndex: 0,
    bodyQue: [0 as *mut crate::g_local_h::gentity_t; 8],
};
#[no_mangle]

pub static mut g_entities: [crate::g_local_h::gentity_t; 1024] = [crate::g_local_h::gentity_t {
    s: crate::src::qcommon::q_shared::entityState_t {
        number: 0,
        eType: 0,
        eFlags: 0,
        pos: crate::src::qcommon::q_shared::trajectory_t {
            trType: crate::src::qcommon::q_shared::TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        apos: crate::src::qcommon::q_shared::trajectory_t {
            trType: crate::src::qcommon::q_shared::TR_STATIONARY,
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
    r: crate::g_public_h::entityShared_t {
        unused: crate::src::qcommon::q_shared::entityState_t {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
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
        linked: crate::src::qcommon::q_shared::qfalse,
        linkcount: 0,
        svFlags: 0,
        singleClient: 0,
        bmodel: crate::src::qcommon::q_shared::qfalse,
        mins: [0.; 3],
        maxs: [0.; 3],
        contents: 0,
        absmin: [0.; 3],
        absmax: [0.; 3],
        currentOrigin: [0.; 3],
        currentAngles: [0.; 3],
        ownerNum: 0,
    },
    client: 0 as *mut crate::g_local_h::gclient_s,
    inuse: crate::src::qcommon::q_shared::qfalse,
    classname: 0 as *mut i8,
    spawnflags: 0,
    neverFree: crate::src::qcommon::q_shared::qfalse,
    flags: 0,
    model: 0 as *mut i8,
    model2: 0 as *mut i8,
    freetime: 0,
    eventTime: 0,
    freeAfterEvent: crate::src::qcommon::q_shared::qfalse,
    unlinkAfterEvent: crate::src::qcommon::q_shared::qfalse,
    physicsObject: crate::src::qcommon::q_shared::qfalse,
    physicsBounce: 0.,
    clipmask: 0,
    moverState: crate::g_local_h::MOVER_POS1,
    soundPos1: 0,
    sound1to2: 0,
    sound2to1: 0,
    soundPos2: 0,
    soundLoop: 0,
    parent: 0 as *mut crate::g_local_h::gentity_t,
    nextTrain: 0 as *mut crate::g_local_h::gentity_t,
    prevTrain: 0 as *mut crate::g_local_h::gentity_t,
    pos1: [0.; 3],
    pos2: [0.; 3],
    message: 0 as *mut i8,
    timestamp: 0,
    target: 0 as *mut i8,
    targetname: 0 as *mut i8,
    team: 0 as *mut i8,
    targetShaderName: 0 as *mut i8,
    targetShaderNewName: 0 as *mut i8,
    target_ent: 0 as *mut crate::g_local_h::gentity_t,
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
    takedamage: crate::src::qcommon::q_shared::qfalse,
    damage: 0,
    splashDamage: 0,
    splashRadius: 0,
    methodOfDeath: 0,
    splashMethodOfDeath: 0,
    count: 0,
    chain: 0 as *mut crate::g_local_h::gentity_t,
    enemy: 0 as *mut crate::g_local_h::gentity_t,
    activator: 0 as *mut crate::g_local_h::gentity_t,
    teamchain: 0 as *mut crate::g_local_h::gentity_t,
    teammaster: 0 as *mut crate::g_local_h::gentity_t,
    watertype: 0,
    waterlevel: 0,
    noise_index: 0,
    wait: 0.,
    random: 0.,
    item: 0 as *mut crate::bg_public_h::gitem_t,
}; 1024];
#[no_mangle]

pub static mut g_clients: [crate::g_local_h::gclient_t; 64] = [crate::g_local_h::gclient_t {
    ps: crate::src::qcommon::q_shared::playerState_t {
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
    pers: crate::g_local_h::clientPersistant_t {
        connected: crate::g_local_h::CON_DISCONNECTED,
        cmd: crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        },
        localClient: crate::src::qcommon::q_shared::qfalse,
        initialSpawn: crate::src::qcommon::q_shared::qfalse,
        predictItemPickup: crate::src::qcommon::q_shared::qfalse,
        pmoveFixed: crate::src::qcommon::q_shared::qfalse,
        netname: [0; 36],
        maxHealth: 0,
        enterTime: 0,
        teamState: crate::g_local_h::playerTeamState_t {
            state: crate::g_local_h::TEAM_BEGIN,
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
        teamInfo: crate::src::qcommon::q_shared::qfalse,
    },
    sess: crate::g_local_h::clientSession_t {
        sessionTeam: crate::bg_public_h::TEAM_FREE,
        spectatorNum: 0,
        spectatorState: crate::g_local_h::SPECTATOR_NOT,
        spectatorClient: 0,
        wins: 0,
        losses: 0,
        teamLeader: crate::src::qcommon::q_shared::qfalse,
    },
    readyToExit: crate::src::qcommon::q_shared::qfalse,
    noclip: crate::src::qcommon::q_shared::qfalse,
    lastCmdTime: 0,
    buttons: 0,
    oldbuttons: 0,
    latched_buttons: 0,
    oldOrigin: [0.; 3],
    damage_armor: 0,
    damage_blood: 0,
    damage_knockback: 0,
    damage_from: [0.; 3],
    damage_fromWorld: crate::src::qcommon::q_shared::qfalse,
    accurateCount: 0,
    accuracy_shots: 0,
    accuracy_hits: 0,
    lastkilled_client: 0,
    lasthurt_client: 0,
    lasthurt_mod: 0,
    respawnTime: 0,
    inactivityTime: 0,
    inactivityWarning: crate::src::qcommon::q_shared::qfalse,
    rewardTime: 0,
    airOutTime: 0,
    lastKillTime: 0,
    fireHeld: crate::src::qcommon::q_shared::qfalse,
    hook: 0 as *mut crate::g_local_h::gentity_t,
    switchTeamTime: 0,
    timeResidual: 0,
    areabits: 0 as *mut i8,
}; 64];
#[no_mangle]

pub static mut g_gametype: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_dmflags: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_fraglimit: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_timelimit: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_capturelimit: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_friendlyFire: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_password: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_needpass: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_maxclients: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_maxGameClients: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_dedicated: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_speed: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_gravity: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_cheats: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_knockback: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_quadfactor: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_forcerespawn: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_inactivity: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_debugMove: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_debugDamage: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_debugAlloc: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_weaponRespawn: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_weaponTeamRespawn: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_motd: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_synchronousClients: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_warmup: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_doWarmup: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_restarted: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_logfile: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_logfileSync: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_blood: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_podiumDist: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_podiumDrop: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_allowVote: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_teamAutoJoin: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_teamForceBalance: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_banIPs: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_filterBan: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_smoothClients: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut pmove_fixed: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut pmove_msec: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_rankings: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_listEntity: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_localTeamPref: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };

static mut gameCvarTable: [cvarTable_t; 46] = unsafe {
    [
        {
            let mut init = cvarTable_t {
                vmCvar: &g_cheats as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"sv_cheats\x00" as *const u8 as *mut i8,
                defaultString: b"\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: 0 as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"gamename\x00" as *const u8 as *mut i8,
                defaultString: b"baseq3\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x40,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: 0 as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"gamedate\x00" as *const u8 as *mut i8,
                defaultString: b"Jan  7 2020\x00" as *const u8 as *mut i8,
                cvarFlags: 0x40,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_restarted as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_restarted\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x40,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_gametype as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_gametype\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x2 | 0x20,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_maxclients as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"sv_maxclients\x00" as *const u8 as *mut i8,
                defaultString: b"8\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x20 | 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_maxGameClients as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_maxGameClients\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x20 | 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_dmflags as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"dmflags\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_fraglimit as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"fraglimit\x00" as *const u8 as *mut i8,
                defaultString: b"20\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x1 | 0x400,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_timelimit as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"timelimit\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x1 | 0x400,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_capturelimit as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"capturelimit\x00" as *const u8 as *mut i8,
                defaultString: b"8\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x1 | 0x400,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_synchronousClients as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_synchronousClients\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x8,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_friendlyFire as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_friendlyFire\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_teamAutoJoin as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_teamAutoJoin\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_teamForceBalance as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_teamForceBalance\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_warmup as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_warmup\x00" as *const u8 as *mut i8,
                defaultString: b"20\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_doWarmup as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_doWarmup\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_logfile as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_log\x00" as *const u8 as *mut i8,
                defaultString: b"games.log\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_logfileSync as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_logsync\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_password as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_password\x00" as *const u8 as *mut i8,
                defaultString: b"\x00" as *const u8 as *mut i8,
                cvarFlags: 0x2,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_banIPs as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_banIPs\x00" as *const u8 as *mut i8,
                defaultString: b"\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_filterBan as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_filterBan\x00" as *const u8 as *mut i8,
                defaultString: b"1\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_needpass as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_needpass\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x4 | 0x40,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_dedicated as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"dedicated\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_speed as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_speed\x00" as *const u8 as *mut i8,
                defaultString: b"320\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_gravity as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_gravity\x00" as *const u8 as *mut i8,
                defaultString: b"800\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_knockback as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_knockback\x00" as *const u8 as *mut i8,
                defaultString: b"1000\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_quadfactor as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_quadfactor\x00" as *const u8 as *mut i8,
                defaultString: b"3\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_weaponRespawn as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_weaponrespawn\x00" as *const u8 as *mut i8,
                defaultString: b"5\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_weaponTeamRespawn as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_weaponTeamRespawn\x00" as *const u8 as *mut i8,
                defaultString: b"30\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_forcerespawn as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_forcerespawn\x00" as *const u8 as *mut i8,
                defaultString: b"20\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_inactivity as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_inactivity\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qtrue,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_debugMove as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_debugMove\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_debugDamage as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_debugDamage\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_debugAlloc as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_debugAlloc\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_motd as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_motd\x00" as *const u8 as *mut i8,
                defaultString: b"\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_blood as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"com_blood\x00" as *const u8 as *mut i8,
                defaultString: b"1\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_podiumDist as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_podiumDist\x00" as *const u8 as *mut i8,
                defaultString: b"80\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_podiumDrop as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_podiumDrop\x00" as *const u8 as *mut i8,
                defaultString: b"70\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_allowVote as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_allowVote\x00" as *const u8 as *mut i8,
                defaultString: b"1\x00" as *const u8 as *mut i8,
                cvarFlags: 0x1,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_listEntity as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_listEntity\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_smoothClients as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_smoothClients\x00" as *const u8 as *mut i8,
                defaultString: b"1\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &pmove_fixed as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"pmove_fixed\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0x8,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &pmove_msec as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"pmove_msec\x00" as *const u8 as *mut i8,
                defaultString: b"8\x00" as *const u8 as *mut i8,
                cvarFlags: 0x8,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_rankings as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_rankings\x00" as *const u8 as *mut i8,
                defaultString: b"0\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
        {
            let mut init = cvarTable_t {
                vmCvar: &g_localTeamPref as *const crate::src::qcommon::q_shared::vmCvar_t
                    as *mut crate::src::qcommon::q_shared::vmCvar_t,
                cvarName: b"g_localTeamPref\x00" as *const u8 as *mut i8,
                defaultString: b"\x00" as *const u8 as *mut i8,
                cvarFlags: 0,
                modificationCount: 0,
                trackChange: crate::src::qcommon::q_shared::qfalse,
                teamShader: crate::src::qcommon::q_shared::qfalse,
            };
            init
        },
    ]
};
// Initialized in run_static_initializers

static mut gameCvarTableSize: i32 = 0;
/*
================
vmMain

This is the only way control passes into the module.
This must be the very first function compiled into the .q3vm file
================
*/
#[no_mangle]

pub unsafe extern "C" fn vmMain(
    mut command: i32,
    mut arg0: i32,
    mut arg1: i32,
    mut arg2: i32,
    mut arg3: i32,
    mut arg4: i32,
    mut arg5: i32,
    mut arg6: i32,
    mut arg7: i32,
    mut arg8: i32,
    mut arg9: i32,
    mut arg10: i32,
    mut arg11: i32,
) -> crate::stdlib::intptr_t {
    match command {
        0 => {
            G_InitGame(arg0, arg1, arg2);
            return 0isize;
        }
        1 => {
            G_ShutdownGame(arg0);
            return 0isize;
        }
        2 => {
            return crate::src::game::g_client::ClientConnect(
                arg0,
                arg1 as crate::src::qcommon::q_shared::qboolean,
                arg2 as crate::src::qcommon::q_shared::qboolean,
            ) as crate::stdlib::intptr_t
        }
        7 => {
            crate::src::game::g_active::ClientThink(arg0);
            return 0isize;
        }
        4 => {
            crate::src::game::g_client::ClientUserinfoChanged(arg0);
            return 0isize;
        }
        5 => {
            crate::src::game::g_client::ClientDisconnect(arg0);
            return 0isize;
        }
        3 => {
            crate::src::game::g_client::ClientBegin(arg0);
            return 0isize;
        }
        6 => {
            crate::src::game::g_cmds::ClientCommand(arg0);
            return 0isize;
        }
        8 => {
            G_RunFrame(arg0);
            return 0isize;
        }
        9 => return crate::src::game::g_svcmds::ConsoleCommand() as crate::stdlib::intptr_t,
        10 => return crate::src::game::ai_main::BotAIStartFrame(arg0) as crate::stdlib::intptr_t,
        _ => {}
    }
    return -1isize;
}
#[no_mangle]

pub unsafe extern "C" fn G_Printf(mut fmt: *const i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [i8; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        fmt,
        argptr.as_va_list(),
    );
    crate::src::game::g_syscalls::trap_Print(text.as_mut_ptr());
}
#[no_mangle]

pub unsafe extern "C" fn G_Error(mut fmt: *const i8, mut args: ...) -> ! {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [i8; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        fmt,
        argptr.as_va_list(),
    );
    crate::src::game::g_syscalls::trap_Error(text.as_mut_ptr());
}
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
    let mut e: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut e2: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut c: i32 = 0;
    let mut c2: i32 = 0;
    c = 0;
    c2 = 0;
    i = 64;
    e = g_entities.as_mut_ptr().offset(i as isize);
    while i < level.num_entities {
        if !((*e).inuse as u64 == 0) {
            if !(*e).team.is_null() {
                if !((*e).flags & 0x400 != 0) {
                    (*e).teammaster = e;
                    c += 1;
                    c2 += 1;
                    j = i + 1;
                    e2 = e.offset(1);
                    while j < level.num_entities {
                        if !((*e2).inuse as u64 == 0) {
                            if !(*e2).team.is_null() {
                                if !((*e2).flags & 0x400 != 0) {
                                    if crate::stdlib::strcmp((*e).team, (*e2).team) == 0 {
                                        c2 += 1;
                                        (*e2).teamchain = (*e).teamchain;
                                        (*e).teamchain = e2;
                                        (*e2).teammaster = e;
                                        (*e2).flags |= 0x400;
                                        // make sure that targets only point at the master
                                        if !(*e2).targetname.is_null() {
                                            (*e).targetname = (*e2).targetname;
                                            (*e2).targetname = 0 as *mut i8
                                        }
                                    }
                                }
                            }
                        }
                        j += 1;
                        e2 = e2.offset(1)
                    }
                }
            }
        }
        i += 1;
        e = e.offset(1)
    }
    G_Printf(
        b"%i teams with %i entities\n\x00" as *const u8 as *const i8,
        c,
        c2,
    );
}
#[no_mangle]

pub unsafe extern "C" fn G_RemapTeamShaders() {}
/*
=================
G_RegisterCvars
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_RegisterCvars() {
    let mut i: i32 = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    let mut remapped: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    i = 0;
    cv = gameCvarTable.as_mut_ptr();
    while i < gameCvarTableSize {
        crate::src::game::g_syscalls::trap_Cvar_Register(
            (*cv).vmCvar,
            (*cv).cvarName,
            (*cv).defaultString,
            (*cv).cvarFlags,
        );
        if !(*cv).vmCvar.is_null() {
            (*cv).modificationCount = (*(*cv).vmCvar).modificationCount
        }
        if (*cv).teamShader as u64 != 0 {
            remapped = crate::src::qcommon::q_shared::qtrue
        }
        i += 1;
        cv = cv.offset(1)
    }
    if remapped as u64 != 0 {
        G_RemapTeamShaders();
    }
    // check some things
    if g_gametype.integer < 0 || g_gametype.integer >= crate::bg_public_h::GT_MAX_GAME_TYPE as i32 {
        G_Printf(
            b"g_gametype %i is out of range, defaulting to 0\n\x00" as *const u8 as *const i8,
            g_gametype.integer,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_gametype\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_syscalls::trap_Cvar_Update(&mut g_gametype);
    }
    level.warmupModificationCount = g_warmup.modificationCount;
}
/*
=================
G_UpdateCvars
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_UpdateCvars() {
    let mut i: i32 = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    let mut remapped: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    i = 0;
    cv = gameCvarTable.as_mut_ptr();
    while i < gameCvarTableSize {
        if !(*cv).vmCvar.is_null() {
            crate::src::game::g_syscalls::trap_Cvar_Update((*cv).vmCvar);
            if (*cv).modificationCount != (*(*cv).vmCvar).modificationCount {
                (*cv).modificationCount = (*(*cv).vmCvar).modificationCount;
                if (*cv).trackChange as u64 != 0 {
                    crate::src::game::g_syscalls::trap_SendServerCommand(
                        -(1i32),
                        crate::src::qcommon::q_shared::va(
                            b"print \"Server: %s changed to %s\n\"\x00" as *const u8 as *mut i8,
                            (*cv).cvarName,
                            (*(*cv).vmCvar).string.as_mut_ptr(),
                        ),
                    );
                }
                if (*cv).teamShader as u64 != 0 {
                    remapped = crate::src::qcommon::q_shared::qtrue
                }
            }
        }
        i += 1;
        cv = cv.offset(1)
    }
    if remapped as u64 != 0 {
        G_RemapTeamShaders();
    };
}
/*
============
G_InitGame

============
*/
#[no_mangle]

pub unsafe extern "C" fn G_InitGame(mut levelTime: i32, mut randomSeed: i32, mut restart: i32) {
    let mut i: i32 = 0;
    G_Printf(b"------- Game Initialization -------\n\x00" as *const u8 as *const i8);
    G_Printf(
        b"gamename: %s\n\x00" as *const u8 as *const i8,
        b"baseq3\x00" as *const u8 as *const i8,
    );
    G_Printf(
        b"gamedate: %s\n\x00" as *const u8 as *const i8,
        b"Jan  7 2020\x00" as *const u8 as *const i8,
    );
    crate::stdlib::srand(randomSeed as u32);
    G_RegisterCvars();
    crate::src::game::g_svcmds::G_ProcessIPBans();
    crate::src::game::g_mem::G_InitMemory();
    // set some level globals
    crate::stdlib::memset(
        &mut level as *mut crate::g_local_h::level_locals_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::g_local_h::level_locals_t>(),
    ); // FIXME standing in lava / slime
    level.time = levelTime;
    level.startTime = levelTime;
    level.snd_fry = crate::src::game::g_utils::G_SoundIndex(
        b"sound/player/fry.wav\x00" as *const u8 as *mut i8,
    );
    if g_gametype.integer != crate::bg_public_h::GT_SINGLE_PLAYER as i32
        && g_logfile.string[0] as i32 != 0
    {
        if g_logfileSync.integer != 0 {
            crate::src::game::g_syscalls::trap_FS_FOpenFile(
                g_logfile.string.as_mut_ptr(),
                &mut level.logFile,
                crate::src::qcommon::q_shared::FS_APPEND_SYNC,
            );
        } else {
            crate::src::game::g_syscalls::trap_FS_FOpenFile(
                g_logfile.string.as_mut_ptr(),
                &mut level.logFile,
                crate::src::qcommon::q_shared::FS_APPEND,
            );
        }
        if level.logFile == 0 {
            G_Printf(
                b"WARNING: Couldn\'t open logfile: %s\n\x00" as *const u8 as *const i8,
                g_logfile.string.as_mut_ptr(),
            );
        } else {
            let mut serverinfo: [i8; 1024] = [0; 1024];
            crate::src::game::g_syscalls::trap_GetServerinfo(
                serverinfo.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 1024]>() as i32,
            );
            G_LogPrintf(
                b"------------------------------------------------------------\n\x00" as *const u8
                    as *const i8,
            );
            G_LogPrintf(
                b"InitGame: %s\n\x00" as *const u8 as *const i8,
                serverinfo.as_mut_ptr(),
            );
        }
    } else {
        G_Printf(b"Not logging to disk.\n\x00" as *const u8 as *const i8);
    }
    crate::src::game::g_session::G_InitWorldSession();
    // initialize all entities for this game
    crate::stdlib::memset(
        g_entities.as_mut_ptr() as *mut libc::c_void,
        0,
        (((1i32) << 10) as usize)
            .wrapping_mul(::std::mem::size_of::<crate::g_local_h::gentity_t>()),
    );
    level.gentities = g_entities.as_mut_ptr();
    // initialize all clients for this game
    level.maxclients = g_maxclients.integer;
    crate::stdlib::memset(
        g_clients.as_mut_ptr() as *mut libc::c_void,
        0,
        (64usize).wrapping_mul(::std::mem::size_of::<crate::g_local_h::gclient_t>()),
    );
    level.clients = g_clients.as_mut_ptr();
    // set client fields on player ents
    i = 0;
    while i < level.maxclients {
        g_entities[i as usize].client = level.clients.offset(i as isize);
        i += 1
    }
    // always leave room for the max number of clients,
    // even if they aren't all used, so numbers inside that
    // range are NEVER anything but clients
    level.num_entities = 64;
    i = 0;
    while i < 64 {
        g_entities[i as usize].classname = b"clientslot\x00" as *const u8 as *mut i8;
        i += 1
    }
    // let the server system know where the entites are
    crate::src::game::g_syscalls::trap_LocateGameData(
        level.gentities,
        level.num_entities,
        ::std::mem::size_of::<crate::g_local_h::gentity_t>() as i32,
        &mut (*level.clients.offset(0)).ps,
        ::std::mem::size_of::<crate::g_local_h::gclient_s>() as i32,
    );
    // reserve some spots for dead player bodies
    crate::src::game::g_client::InitBodyQue();
    crate::src::game::g_items::ClearRegisteredItems();
    // parse the key/value pairs and spawn gentities
    crate::src::game::g_spawn::G_SpawnEntitiesFromString();
    // general initialization
    G_FindTeams();
    // make sure we have flags for CTF, etc
    if g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        crate::src::game::g_items::G_CheckTeamItems();
    }
    crate::src::game::g_items::SaveRegisteredItems();
    G_Printf(b"-----------------------------------\n\x00" as *const u8 as *const i8);
    if g_gametype.integer == crate::bg_public_h::GT_SINGLE_PLAYER as i32
        || crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
            b"com_buildScript\x00" as *const u8 as *const i8,
        ) != 0
    {
        crate::src::game::g_utils::G_ModelIndex(
            b"models/mapobjects/podium/podium4.md3\x00" as *const u8 as *mut i8,
        );
    }
    if crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
        b"bot_enable\x00" as *const u8 as *const i8,
    ) != 0
    {
        crate::src::game::ai_main::BotAISetup(restart);
        crate::src::game::ai_main::BotAILoadMap(restart);
        crate::src::game::g_bot::G_InitBots(restart as crate::src::qcommon::q_shared::qboolean);
    }
    G_RemapTeamShaders();
    crate::src::game::g_syscalls::trap_SetConfigstring(22, b"\x00" as *const u8 as *const i8);
}
/*
=================
G_ShutdownGame
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_ShutdownGame(mut restart: i32) {
    G_Printf(b"==== ShutdownGame ====\n\x00" as *const u8 as *const i8);
    if level.logFile != 0 {
        G_LogPrintf(b"ShutdownGame:\n\x00" as *const u8 as *const i8);
        G_LogPrintf(
            b"------------------------------------------------------------\n\x00" as *const u8
                as *const i8,
        );
        crate::src::game::g_syscalls::trap_FS_FCloseFile(level.logFile);
        level.logFile = 0
    }
    // write all the client session data so we can get it back
    crate::src::game::g_session::G_WriteSessionData();
    if crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
        b"bot_enable\x00" as *const u8 as *const i8,
    ) != 0
    {
        crate::src::game::ai_main::BotAIShutdown(restart);
    };
}
//===================================================================
#[no_mangle]

pub unsafe extern "C" fn Com_Error(mut level_0: i32, mut error: *const i8, mut args: ...) -> ! {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [i8; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        error,
        argptr.as_va_list(),
    );
    crate::src::game::g_syscalls::trap_Error(text.as_mut_ptr());
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
// q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
// Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
// When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
// number of supported master servers
// standard demo extension
//Ignore __attribute__ on non-gcc platforms
/* *********************************************************************
 VM Considerations

 The VM can not use the standard system headers because we aren't really
 using the compiler they were meant for.  We use bg_lib.h which contains
 prototypes for the functions we define for our own use in bg_lib.c.

 When writing mods, please add needed headers HERE, do not start including
 stuff like <stdio.h> in the various .c files that make up each of the VMs
 since you will be including system headers files can will have issues.

 Remember, if you use a C library function that is not defined in bg_lib.c,
 you will have to add your own version for support in the VM.

**********************************************************************/
//=============================================================
// expand constants before stringifying them
// angle indexes
// up / down
// left / right
// fall over
// the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
// max length of a string passed to Cmd_TokenizeString
// max tokens resulting from Cmd_TokenizeString
// max length of an individual token
// used for system info key only
// max length of a quake game pathname
// max length of a client name
// parameters for command buffer stuffing
// don't return until completed, a VM should NEVER use this,
// because some commands might cause the VM to be unloaded...
// insert at current position, but don't run yet
// add to end of the command buffer (normal case)
//
// these aren't needed by any of the VMs.  put in another header?
//
// bit vector of area visibility
// print levels from renderer (FIXME: set up for game / cgame?)
// only print when "developer 1"
// parameters to the main Error routine
// exit the entire game with a popup window
// print to console and disconnect from game
// don't kill server
// client disconnected from the server
// pop up the need-cd dialog
// font rendering values used by ui and cgame
// default
// default
/*
==============================================================

MATHLIB

==============================================================
*/
// all drawing is done to a 640*480 virtual screen size
// and will be automatically scaled to the real resolution
// ^[0-9a-zA-Z]
/*
// if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
// or write a mail to the ioq3 mailing list.
#else
  #define Q_ftol(v) ((long) (v))
  #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
  #define Q_SnapVector(vec) \
    do\
    {\
        vec3_t *temp = (vec);\
        \
        Q_round((*temp)[0]);\
        Q_round((*temp)[1]);\
        Q_round((*temp)[2]);\
    } while(0)
#endif
*/
// reciprocal square root
// this isn't a real cheap function to call!
// just in case you don't want to use the macros
// fast vector normalize routine that does not check to make sure
// that length != 0, nor does it return length, uses rsqrt approximation
// returns vector length
// perpendicular vector could be replaced by this
//int	PlaneTypeForNormal (vec3_t normal);
//=============================================
//int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
//token types
// string
// literal
// number
// name
// punctuation
// data is an in/out parm, returns a parsed out token
// mode parm for FS_FOpenFile
//=============================================
// portable case insensitive compare
// buffer size safe library replacements
// strlen that discounts Quake color sequences
// removes color sequences from string
// Count the number of char tocount encountered in string
//=============================================
// 64-bit integers for global rankings interface
// implemented as a struct for qvm compatibility
//=============================================
/*
short	BigShort(short l);
short	LittleShort(short l);
int		BigLong (int l);
int		LittleLong (int l);
qint64  BigLong64 (qint64 l);
qint64  LittleLong64 (qint64 l);
float	BigFloat (const float *l);
float	LittleFloat (const float *l);

void	Swap_Init (void);
*/
//=============================================
//
// key / value info strings
//
// this is only here so the functions in q_shared.c and bg_*.c can link
#[no_mangle]

pub unsafe extern "C" fn Com_Printf(mut msg: *const i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [i8; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        msg,
        argptr.as_va_list(),
    );
    crate::src::game::g_syscalls::trap_Print(text.as_mut_ptr());
}
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
    let mut i: i32 = 0;
    let mut client: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut nextInLine: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    if level.numPlayingClients >= 2 {
        return;
    }
    // never change during intermission
    if level.intermissiontime != 0 {
        return;
    }
    nextInLine = 0 as *mut crate::g_local_h::gclient_t;

    for i in 0..level.maxclients {
        client = &mut *level.clients.offset(i as isize) as *mut crate::g_local_h::gclient_s;

        if !((*client).pers.connected != crate::g_local_h::CON_CONNECTED) {
            if !((*client).sess.sessionTeam != crate::bg_public_h::TEAM_SPECTATOR) {
                // never select the dedicated follow or scoreboard clients
                if !((*client).sess.spectatorState == crate::g_local_h::SPECTATOR_SCOREBOARD
                    || (*client).sess.spectatorClient < 0)
                {
                    if nextInLine.is_null()
                        || (*client).sess.spectatorNum > (*nextInLine).sess.spectatorNum
                    {
                        nextInLine = client
                    }
                }
            }
        }
    }
    if nextInLine.is_null() {
        return;
    }
    level.warmupTime = -(1);
    // set them to free-for-all team
    crate::src::game::g_cmds::SetTeam(
        &mut *g_entities
            .as_mut_ptr()
            .offset(nextInLine.wrapping_offset_from(level.clients)),
        b"f\x00" as *const u8 as *const i8,
    );
}
/*
=======================
AddTournamentQueue

Add client to end of tournament queue
=======================
*/
#[no_mangle]

pub unsafe extern "C" fn AddTournamentQueue(mut client: *mut crate::g_local_h::gclient_t) {
    let mut index: i32 = 0;
    let mut curclient: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    index = 0;
    while index < level.maxclients {
        curclient = &mut *level.clients.offset(index as isize) as *mut crate::g_local_h::gclient_s;
        if (*curclient).pers.connected != crate::g_local_h::CON_DISCONNECTED {
            if curclient == client {
                (*curclient).sess.spectatorNum = 0
            } else if (*curclient).sess.sessionTeam == crate::bg_public_h::TEAM_SPECTATOR {
                (*curclient).sess.spectatorNum += 1
            }
        }
        index += 1
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
    let mut clientNum: i32 = 0;
    if level.numPlayingClients != 2 {
        return;
    }
    clientNum = level.sortedClients[1];
    if (*level.clients.offset(clientNum as isize)).pers.connected != crate::g_local_h::CON_CONNECTED
    {
        return;
    }
    // make them a spectator
    crate::src::game::g_cmds::SetTeam(
        &mut *g_entities.as_mut_ptr().offset(clientNum as isize),
        b"s\x00" as *const u8 as *const i8,
    );
}
/*
=======================
RemoveTournamentWinner
=======================
*/
#[no_mangle]

pub unsafe extern "C" fn RemoveTournamentWinner() {
    let mut clientNum: i32 = 0;
    if level.numPlayingClients != 2 {
        return;
    }
    clientNum = level.sortedClients[0];
    if (*level.clients.offset(clientNum as isize)).pers.connected != crate::g_local_h::CON_CONNECTED
    {
        return;
    }
    // make them a spectator
    crate::src::game::g_cmds::SetTeam(
        &mut *g_entities.as_mut_ptr().offset(clientNum as isize),
        b"s\x00" as *const u8 as *const i8,
    );
}
/*
=======================
AdjustTournamentScores
=======================
*/
#[no_mangle]

pub unsafe extern "C" fn AdjustTournamentScores() {
    let mut clientNum: i32 = 0;
    clientNum = level.sortedClients[0];
    if (*level.clients.offset(clientNum as isize)).pers.connected == crate::g_local_h::CON_CONNECTED
    {
        let ref mut fresh0 = (*level.clients.offset(clientNum as isize)).sess.wins;
        *fresh0 += 1;
        crate::src::game::g_client::ClientUserinfoChanged(clientNum);
    }
    clientNum = level.sortedClients[1];
    if (*level.clients.offset(clientNum as isize)).pers.connected == crate::g_local_h::CON_CONNECTED
    {
        let ref mut fresh1 = (*level.clients.offset(clientNum as isize)).sess.losses;
        *fresh1 += 1;
        crate::src::game::g_client::ClientUserinfoChanged(clientNum);
    };
}
/*
=============
SortRanks

=============
*/
#[no_mangle]

pub unsafe extern "C" fn SortRanks(mut a: *const libc::c_void, mut b: *const libc::c_void) -> i32 {
    let mut ca: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut cb: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    ca = &mut *level.clients.offset(*(a as *mut i32) as isize) as *mut crate::g_local_h::gclient_s;
    cb = &mut *level.clients.offset(*(b as *mut i32) as isize) as *mut crate::g_local_h::gclient_s;
    // sort special clients last
    if (*ca).sess.spectatorState == crate::g_local_h::SPECTATOR_SCOREBOARD
        || (*ca).sess.spectatorClient < 0
    {
        return 1i32;
    }
    if (*cb).sess.spectatorState == crate::g_local_h::SPECTATOR_SCOREBOARD
        || (*cb).sess.spectatorClient < 0
    {
        return -(1i32);
    }
    // then connecting clients
    if (*ca).pers.connected == crate::g_local_h::CON_CONNECTING {
        return 1i32;
    }
    if (*cb).pers.connected == crate::g_local_h::CON_CONNECTING {
        return -(1i32);
    }
    // then spectators
    if (*ca).sess.sessionTeam == crate::bg_public_h::TEAM_SPECTATOR
        && (*cb).sess.sessionTeam == crate::bg_public_h::TEAM_SPECTATOR
    {
        if (*ca).sess.spectatorNum > (*cb).sess.spectatorNum {
            return -(1i32);
        }
        if (*ca).sess.spectatorNum < (*cb).sess.spectatorNum {
            return 1i32;
        }
        return 0i32;
    }
    if (*ca).sess.sessionTeam == crate::bg_public_h::TEAM_SPECTATOR {
        return 1i32;
    }
    if (*cb).sess.sessionTeam == crate::bg_public_h::TEAM_SPECTATOR {
        return -(1i32);
    }
    // then sort by score
    if (*ca).ps.persistant[crate::bg_public_h::PERS_SCORE as usize]
        > (*cb).ps.persistant[crate::bg_public_h::PERS_SCORE as usize]
    {
        return -(1i32);
    }
    if (*ca).ps.persistant[crate::bg_public_h::PERS_SCORE as usize]
        < (*cb).ps.persistant[crate::bg_public_h::PERS_SCORE as usize]
    {
        return 1i32;
    }
    return 0;
}
/*
============
CalculateRanks

Recalculates the score ranks of all players
This will be called on every client connect, begin, disconnect, death,
and team change.
============
*/
#[no_mangle]

pub unsafe extern "C" fn CalculateRanks() {
    let mut i: i32 = 0; // don't count bots
    let mut rank: i32 = 0;
    let mut score: i32 = 0;
    let mut newScore: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    level.follow1 = -(1);
    level.follow2 = -(1);
    level.numConnectedClients = 0;
    level.numNonSpectatorClients = 0;
    level.numPlayingClients = 0;
    level.numVotingClients = 0;
    i = 0;
    while (i as usize)
        < (::std::mem::size_of::<[i32; 2]>()).wrapping_div(::std::mem::size_of::<i32>())
    {
        level.numteamVotingClients[i as usize] = 0;
        i += 1
    }
    i = 0;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).pers.connected != crate::g_local_h::CON_DISCONNECTED
        {
            level.sortedClients[level.numConnectedClients as usize] = i;
            level.numConnectedClients += 1;
            if (*level.clients.offset(i as isize)).sess.sessionTeam
                != crate::bg_public_h::TEAM_SPECTATOR
            {
                level.numNonSpectatorClients += 1;
                // decide if this should be auto-followed
                if (*level.clients.offset(i as isize)).pers.connected
                    == crate::g_local_h::CON_CONNECTED
                {
                    level.numPlayingClients += 1;
                    if g_entities[i as usize].r.svFlags & 0x8 == 0 {
                        level.numVotingClients += 1;
                        if (*level.clients.offset(i as isize)).sess.sessionTeam
                            == crate::bg_public_h::TEAM_RED
                        {
                            level.numteamVotingClients[0] += 1
                        } else if (*level.clients.offset(i as isize)).sess.sessionTeam
                            == crate::bg_public_h::TEAM_BLUE
                        {
                            level.numteamVotingClients[1] += 1
                        }
                    }
                    if level.follow1 == -(1) {
                        level.follow1 = i
                    } else if level.follow2 == -(1) {
                        level.follow2 = i
                    }
                }
            }
        }
        i += 1
    }
    crate::stdlib::qsort(
        level.sortedClients.as_mut_ptr() as *mut libc::c_void,
        level.numConnectedClients as crate::stddef_h::size_t,
        ::std::mem::size_of::<i32>(),
        Some(
            SortRanks
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
    // set the rank value for all clients that are connected and not spectators
    if g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        // in team games, rank is just the order of the teams, 0=red, 1=blue, 2=tied
        i = 0;
        while i < level.numConnectedClients {
            cl = &mut *level
                .clients
                .offset(*level.sortedClients.as_mut_ptr().offset(i as isize) as isize)
                as *mut crate::g_local_h::gclient_s;
            if level.teamScores[crate::bg_public_h::TEAM_RED as usize]
                == level.teamScores[crate::bg_public_h::TEAM_BLUE as usize]
            {
                (*cl).ps.persistant[crate::bg_public_h::PERS_RANK as usize] = 2
            } else if level.teamScores[crate::bg_public_h::TEAM_RED as usize]
                > level.teamScores[crate::bg_public_h::TEAM_BLUE as usize]
            {
                (*cl).ps.persistant[crate::bg_public_h::PERS_RANK as usize] = 0
            } else {
                (*cl).ps.persistant[crate::bg_public_h::PERS_RANK as usize] = 1
            }
            i += 1
        }
    } else {
        rank = -(1);
        score = 0;
        i = 0;
        while i < level.numPlayingClients {
            cl = &mut *level
                .clients
                .offset(*level.sortedClients.as_mut_ptr().offset(i as isize) as isize)
                as *mut crate::g_local_h::gclient_s;
            newScore = (*cl).ps.persistant[crate::bg_public_h::PERS_SCORE as usize];
            if i == 0 || newScore != score {
                rank = i;
                // assume we aren't tied until the next client is checked
                (*level
                    .clients
                    .offset(level.sortedClients[i as usize] as isize))
                .ps
                .persistant[crate::bg_public_h::PERS_RANK as usize] = rank
            } else {
                // we are tied with the previous client
                (*level
                    .clients
                    .offset(level.sortedClients[(i - 1) as usize] as isize))
                .ps
                .persistant[crate::bg_public_h::PERS_RANK as usize] = rank | 0x4000;
                (*level
                    .clients
                    .offset(level.sortedClients[i as usize] as isize))
                .ps
                .persistant[crate::bg_public_h::PERS_RANK as usize] = rank | 0x4000
            }
            score = newScore;
            if g_gametype.integer == crate::bg_public_h::GT_SINGLE_PLAYER as i32
                && level.numPlayingClients == 1
            {
                (*level
                    .clients
                    .offset(level.sortedClients[i as usize] as isize))
                .ps
                .persistant[crate::bg_public_h::PERS_RANK as usize] = rank | 0x4000
            }
            i += 1
        }
    }
    // set the CS_SCORES1/2 configstrings, which will be visible to everyone
    if g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        crate::src::game::g_syscalls::trap_SetConfigstring(
            6,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *mut i8,
                level.teamScores[crate::bg_public_h::TEAM_RED as usize],
            ),
        );
        crate::src::game::g_syscalls::trap_SetConfigstring(
            7i32,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *mut i8,
                level.teamScores[crate::bg_public_h::TEAM_BLUE as usize],
            ),
        );
    } else if level.numConnectedClients == 0 {
        crate::src::game::g_syscalls::trap_SetConfigstring(
            6,
            crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, -(9999i32)),
        );
        crate::src::game::g_syscalls::trap_SetConfigstring(
            7i32,
            crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, -(9999i32)),
        );
    } else if level.numConnectedClients == 1 {
        crate::src::game::g_syscalls::trap_SetConfigstring(
            6,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *mut i8,
                (*level.clients.offset(level.sortedClients[0usize] as isize))
                    .ps
                    .persistant[crate::bg_public_h::PERS_SCORE as usize],
            ),
        );
        crate::src::game::g_syscalls::trap_SetConfigstring(
            7i32,
            crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, -(9999i32)),
        );
    } else {
        crate::src::game::g_syscalls::trap_SetConfigstring(
            6,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *mut i8,
                (*level.clients.offset(level.sortedClients[0usize] as isize))
                    .ps
                    .persistant[crate::bg_public_h::PERS_SCORE as usize],
            ),
        );
        crate::src::game::g_syscalls::trap_SetConfigstring(
            7i32,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *mut i8,
                (*level.clients.offset(level.sortedClients[1usize] as isize))
                    .ps
                    .persistant[crate::bg_public_h::PERS_SCORE as usize],
            ),
        );
    }
    // see if it is time to end the level
    CheckExitRules();
    // if we are at the intermission, send the new info to everyone
    if level.intermissiontime != 0 {
        SendScoreboardMessageToAllClients();
    };
}
/*
========================================================================

MAP CHANGING

========================================================================
*/
/*
========================
SendScoreboardMessageToAllClients

Do this at BeginIntermission time and whenever ranks are recalculated
due to enters/exits/forced team changes
========================
*/
#[no_mangle]

pub unsafe extern "C" fn SendScoreboardMessageToAllClients() {
    let mut i: i32 = 0;
    i = 0;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).pers.connected == crate::g_local_h::CON_CONNECTED {
            crate::src::game::g_cmds::DeathmatchScoreboardMessage(
                g_entities.as_mut_ptr().offset(i as isize),
            );
        }
        i += 1
    }
}
/*
========================
MoveClientToIntermission

When the intermission starts, this will be called for all players.
If a new client connects, this will be called after the spawn function.
========================
*/
#[no_mangle]

pub unsafe extern "C" fn MoveClientToIntermission(mut ent: *mut crate::g_local_h::gentity_t) {
    // take out of follow mode if needed
    if (*(*ent).client).sess.spectatorState == crate::g_local_h::SPECTATOR_FOLLOW {
        crate::src::game::g_cmds::StopFollowing(ent);
    }
    FindIntermissionPoint();
    // move to the spot
    (*ent).s.origin[0] = level.intermission_origin[0];
    (*ent).s.origin[1] = level.intermission_origin[1];
    (*ent).s.origin[2] = level.intermission_origin[2];
    (*(*ent).client).ps.origin[0] = level.intermission_origin[0];
    (*(*ent).client).ps.origin[1] = level.intermission_origin[1];
    (*(*ent).client).ps.origin[2] = level.intermission_origin[2];
    (*(*ent).client).ps.viewangles[0] = level.intermission_angle[0];
    (*(*ent).client).ps.viewangles[1] = level.intermission_angle[1];
    (*(*ent).client).ps.viewangles[2] = level.intermission_angle[2];
    (*(*ent).client).ps.pm_type = crate::bg_public_h::PM_INTERMISSION as i32;
    // clean up powerup info
    crate::stdlib::memset(
        (*(*ent).client).ps.powerups.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[i32; 16]>(),
    );
    (*(*ent).client).ps.eFlags = 0;
    (*ent).s.eFlags = 0;
    (*ent).s.eType = crate::bg_public_h::ET_GENERAL as i32;
    (*ent).s.modelindex = 0;
    (*ent).s.loopSound = 0;
    (*ent).s.event = 0;
    (*ent).r.contents = 0;
}
/*
==================
FindIntermissionPoint

This is also used for spectator spawns
==================
*/
#[no_mangle]

pub unsafe extern "C" fn FindIntermissionPoint() {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut target: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // find the intermission spot
    ent = crate::src::game::g_utils::G_Find(
        0 as *mut crate::g_local_h::gentity_t,
        &mut (*(0 as *mut crate::g_local_h::gentity_t)).classname as *mut *mut i8 as i32,
        b"info_player_intermission\x00" as *const u8 as *const i8,
    );
    if ent.is_null() {
        // the map creator forgot to put in an intermission point...
        crate::src::game::g_client::SelectSpawnPoint(
            crate::src::qcommon::q_math::vec3_origin.as_mut_ptr(),
            level.intermission_origin.as_mut_ptr(),
            level.intermission_angle.as_mut_ptr(),
            crate::src::qcommon::q_shared::qfalse,
        );
    } else {
        level.intermission_origin[0] = (*ent).s.origin[0];
        level.intermission_origin[1] = (*ent).s.origin[1];
        level.intermission_origin[2] = (*ent).s.origin[2];
        level.intermission_angle[0] = (*ent).s.angles[0];
        level.intermission_angle[1] = (*ent).s.angles[1];
        level.intermission_angle[2] = (*ent).s.angles[2];
        // if it has a target, look towards it
        if !(*ent).target.is_null() {
            target = crate::src::game::g_utils::G_PickTarget((*ent).target);
            if !target.is_null() {
                dir[0] = (*target).s.origin[0] - level.intermission_origin[0];
                dir[1] = (*target).s.origin[1] - level.intermission_origin[1];
                dir[2] = (*target).s.origin[2] - level.intermission_origin[2];
                crate::src::qcommon::q_math::vectoangles(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    level.intermission_angle.as_mut_ptr(),
                );
            }
        }
    };
}
/*
==================
BeginIntermission
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BeginIntermission() {
    let mut i: i32 = 0;
    let mut client: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    if level.intermissiontime != 0 {
        return;
        // already active
    }
    // if in tournement mode, change the wins / losses
    if g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32 {
        AdjustTournamentScores();
    }
    level.intermissiontime = level.time;
    // move all clients to the intermission point

    for i in 0..level.maxclients {
        client = g_entities.as_mut_ptr().offset(i as isize);

        if !((*client).inuse as u64 == 0) {
            // respawn if dead
            if (*client).health <= 0 {
                crate::src::game::g_client::ClientRespawn(client);
            }
            MoveClientToIntermission(client);
        }
    }
    // if single player game
    if g_gametype.integer == crate::bg_public_h::GT_SINGLE_PLAYER as i32 {
        crate::src::game::g_arenas::UpdateTournamentInfo();
        crate::src::game::g_arenas::SpawnModelsOnVictoryPads();
    }
    // send the current scoring to all clients
    SendScoreboardMessageToAllClients();
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
    let mut i: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut nextmap: [i8; 1024] = [0; 1024];
    let mut d1: [i8; 1024] = [0; 1024];
    //bot interbreeding
    crate::src::game::ai_main::BotInterbreedEndMatch();
    // if we are running a tournement map, kick the loser to spectator status,
    // which will automatically grab the next spectator and restart
    if g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32 {
        if level.restarted as u64 == 0 {
            RemoveTournamentLoser();
            crate::src::game::g_syscalls::trap_SendConsoleCommand(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"map_restart 0\n\x00" as *const u8 as *const i8,
            );
            level.restarted = crate::src::qcommon::q_shared::qtrue;
            level.changemap = 0 as *mut i8;
            level.intermissiontime = 0
        }
        return;
    }
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"nextmap\x00" as *const u8 as *const i8,
        nextmap.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"d1\x00" as *const u8 as *const i8,
        d1.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(
        nextmap.as_mut_ptr(),
        b"map_restart 0\x00" as *const u8 as *const i8,
    ) == 0
        && crate::src::qcommon::q_shared::Q_stricmp(
            d1.as_mut_ptr(),
            b"\x00" as *const u8 as *const i8,
        ) != 0
    {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"nextmap\x00" as *const u8 as *const i8,
            b"vstr d2\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_syscalls::trap_SendConsoleCommand(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"vstr d1\n\x00" as *const u8 as *const i8,
        );
    } else {
        crate::src::game::g_syscalls::trap_SendConsoleCommand(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"vstr nextmap\n\x00" as *const u8 as *const i8,
        );
    }
    level.changemap = 0 as *mut i8;
    level.intermissiontime = 0;
    // reset all the scores so we don't enter the intermission again
    level.teamScores[crate::bg_public_h::TEAM_RED as usize] = 0;
    level.teamScores[crate::bg_public_h::TEAM_BLUE as usize] = 0;
    i = 0;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected != crate::g_local_h::CON_CONNECTED) {
            (*cl).ps.persistant[crate::bg_public_h::PERS_SCORE as usize] = 0
        }
        i += 1
    }
    // we need to do this here before changing to CON_CONNECTING
    crate::src::game::g_session::G_WriteSessionData();
    // change all client states to connecting, so the early players into the
    // next level will know the others aren't done reconnecting
    i = 0;
    while i < g_maxclients.integer {
        if (*level.clients.offset(i as isize)).pers.connected == crate::g_local_h::CON_CONNECTED {
            (*level.clients.offset(i as isize)).pers.connected = crate::g_local_h::CON_CONNECTING
        }
        i += 1
    }
}
/*
=================
G_LogPrintf

Print to the logfile with a time stamp if it is open
=================
*/
#[no_mangle]

pub unsafe extern "C" fn G_LogPrintf(mut fmt: *const i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut string: [i8; 1024] = [0; 1024];
    let mut min: i32 = 0;
    let mut tens: i32 = 0;
    let mut sec: i32 = 0;
    sec = (level.time - level.startTime) / 1000;
    min = sec / 60;
    sec -= min * 60;
    tens = sec / 10;
    sec -= tens * 10;
    crate::src::qcommon::q_shared::Com_sprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
        b"%3i:%i%i \x00" as *const u8 as *const i8,
        min,
        tens,
        sec,
    );
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr().offset(7),
        (::std::mem::size_of::<[i8; 1024]>()).wrapping_sub(7usize),
        fmt,
        argptr.as_va_list(),
    );
    if g_dedicated.integer != 0 {
        G_Printf(
            b"%s\x00" as *const u8 as *const i8,
            string.as_mut_ptr().offset(7isize),
        );
    }
    if level.logFile == 0 {
        return;
    }
    crate::src::game::g_syscalls::trap_FS_Write(
        string.as_mut_ptr() as *const libc::c_void,
        crate::stdlib::strlen(string.as_mut_ptr()) as i32,
        level.logFile,
    );
}
/*
================
LogExit

Append information about this game to the log file
================
*/
#[no_mangle]

pub unsafe extern "C" fn LogExit(mut string: *const i8) {
    let mut i: i32 = 0;
    let mut numSorted: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    G_LogPrintf(b"Exit: %s\n\x00" as *const u8 as *const i8, string);
    level.intermissionQueued = level.time;
    // this will keep the clients from playing any voice sounds
    // that will get cut off when the queued intermission starts
    crate::src::game::g_syscalls::trap_SetConfigstring(22, b"1\x00" as *const u8 as *const i8);
    // don't send more than 32 scores (FIXME?)
    numSorted = level.numConnectedClients;
    if numSorted > 32 {
        numSorted = 32
    }
    if g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        G_LogPrintf(
            b"red:%i  blue:%i\n\x00" as *const u8 as *const i8,
            level.teamScores[crate::bg_public_h::TEAM_RED as usize],
            level.teamScores[crate::bg_public_h::TEAM_BLUE as usize],
        );
    }
    i = 0;
    while i < numSorted {
        let mut ping: i32 = 0;
        cl = &mut *level
            .clients
            .offset(*level.sortedClients.as_mut_ptr().offset(i as isize) as isize)
            as *mut crate::g_local_h::gclient_s;
        if !((*cl).sess.sessionTeam == crate::bg_public_h::TEAM_SPECTATOR) {
            if !((*cl).pers.connected == crate::g_local_h::CON_CONNECTING) {
                ping = if (*cl).ps.ping < 999 {
                    (*cl).ps.ping
                } else {
                    999
                };
                G_LogPrintf(
                    b"score: %i  ping: %i  client: %i %s\n\x00" as *const u8 as *const i8,
                    (*cl).ps.persistant[crate::bg_public_h::PERS_SCORE as usize],
                    ping,
                    level.sortedClients[i as usize],
                    (*cl).pers.netname.as_mut_ptr(),
                );
            }
        }
        i += 1
    }
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
    let mut ready: i32 = 0;
    let mut notReady: i32 = 0;
    let mut playerCount: i32 = 0;
    let mut i: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    let mut readyMask: i32 = 0;
    if g_gametype.integer == crate::bg_public_h::GT_SINGLE_PLAYER as i32 {
        return;
    }
    // see which players are ready
    ready = 0;
    notReady = 0;
    readyMask = 0;
    playerCount = 0;
    i = 0;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected != crate::g_local_h::CON_CONNECTED) {
            if !(g_entities[i as usize].r.svFlags & 0x8 != 0) {
                playerCount += 1;
                if (*cl).readyToExit as u64 != 0 {
                    ready += 1;
                    if i < 16 {
                        readyMask |= (1) << i
                    }
                } else {
                    notReady += 1
                }
            }
        }
        i += 1
    }
    // copy the readyMask to each player's stats so
    // it can be displayed on the scoreboard
    i = 0;
    while i < g_maxclients.integer {
        cl = level.clients.offset(i as isize);
        if !((*cl).pers.connected != crate::g_local_h::CON_CONNECTED) {
            (*cl).ps.stats[crate::bg_public_h::STAT_CLIENTS_READY as usize] = readyMask
        }
        i += 1
    }
    // never exit in less than five seconds
    if level.time < level.intermissiontime + 5000 {
        return;
    }
    // only test ready status when there are real players present
    if playerCount > 0 {
        // if nobody wants to go, clear timer
        if ready == 0 {
            level.readyToExit = crate::src::qcommon::q_shared::qfalse;
            return;
        }
        // if everyone wants to go, go now
        if notReady == 0 {
            ExitLevel();
            return;
        }
    }
    // the first person to ready starts the ten second timeout
    if level.readyToExit as u64 == 0 {
        level.readyToExit = crate::src::qcommon::q_shared::qtrue;
        level.exitTime = level.time
    }
    // if we have waited ten seconds since at least one player
    // wanted to exit, go ahead
    if level.time < level.exitTime + 10000 {
        return;
    }
    ExitLevel();
}
/*
=============
ScoreIsTied
=============
*/
#[no_mangle]

pub unsafe extern "C" fn ScoreIsTied() -> crate::src::qcommon::q_shared::qboolean {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    if level.numPlayingClients < 2 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        return (level.teamScores[crate::bg_public_h::TEAM_RED as usize]
            == level.teamScores[crate::bg_public_h::TEAM_BLUE as usize])
            as crate::src::qcommon::q_shared::qboolean;
    }
    a = (*level.clients.offset(level.sortedClients[0] as isize))
        .ps
        .persistant[crate::bg_public_h::PERS_SCORE as usize];
    b = (*level.clients.offset(level.sortedClients[1] as isize))
        .ps
        .persistant[crate::bg_public_h::PERS_SCORE as usize];
    return (a == b) as crate::src::qcommon::q_shared::qboolean;
}
/*
=================
CheckExitRules

There will be a delay between the time the exit is qualified for
and the time everyone is moved to the intermission spot, so you
can see the last frag.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckExitRules() {
    let mut i: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    // if at the intermission, wait for all non-bots to
    // signal ready, then go to next level
    if level.intermissiontime != 0 {
        CheckIntermissionExit();
        return;
    }
    if level.intermissionQueued != 0 {
        if level.time - level.intermissionQueued >= 1000 {
            level.intermissionQueued = 0;
            BeginIntermission();
        }
        return;
    }
    // check for sudden death
    if ScoreIsTied() as u64 != 0 {
        // always wait for sudden death
        return;
    }
    if g_timelimit.integer < 0 || g_timelimit.integer > 2147483647 / 60000 {
        G_Printf(
            b"timelimit %i is out of range, defaulting to 0\n\x00" as *const u8 as *const i8,
            g_timelimit.integer,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"timelimit\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_syscalls::trap_Cvar_Update(&mut g_timelimit);
    }
    if g_timelimit.integer != 0 && level.warmupTime == 0 {
        if level.time - level.startTime >= g_timelimit.integer * 60000 {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                -(1),
                b"print \"Timelimit hit.\n\"\x00" as *const u8 as *const i8,
            );
            LogExit(b"Timelimit hit.\x00" as *const u8 as *const i8);
            return;
        }
    }
    if g_fraglimit.integer < 0 {
        G_Printf(
            b"fraglimit %i is out of range, defaulting to 0\n\x00" as *const u8 as *const i8,
            g_fraglimit.integer,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"fraglimit\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_syscalls::trap_Cvar_Update(&mut g_fraglimit);
    }
    if g_gametype.integer < crate::bg_public_h::GT_CTF as i32 && g_fraglimit.integer != 0 {
        if level.teamScores[crate::bg_public_h::TEAM_RED as usize] >= g_fraglimit.integer {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                -(1),
                b"print \"Red hit the fraglimit.\n\"\x00" as *const u8 as *const i8,
            );
            LogExit(b"Fraglimit hit.\x00" as *const u8 as *const i8);
            return;
        }
        if level.teamScores[crate::bg_public_h::TEAM_BLUE as usize] >= g_fraglimit.integer {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                -(1),
                b"print \"Blue hit the fraglimit.\n\"\x00" as *const u8 as *const i8,
            );
            LogExit(b"Fraglimit hit.\x00" as *const u8 as *const i8);
            return;
        }
        i = 0;
        while i < g_maxclients.integer {
            cl = level.clients.offset(i as isize);
            if !((*cl).pers.connected != crate::g_local_h::CON_CONNECTED) {
                if !((*cl).sess.sessionTeam != crate::bg_public_h::TEAM_FREE) {
                    if (*cl).ps.persistant[crate::bg_public_h::PERS_SCORE as usize]
                        >= g_fraglimit.integer
                    {
                        LogExit(b"Fraglimit hit.\x00" as *const u8 as *const i8);
                        crate::src::game::g_syscalls::trap_SendServerCommand(
                            -(1),
                            crate::src::qcommon::q_shared::va(
                                b"print \"%s^7 hit the fraglimit.\n\"\x00" as *const u8 as *mut i8,
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
    if g_capturelimit.integer < 0 {
        G_Printf(
            b"capturelimit %i is out of range, defaulting to 0\n\x00" as *const u8 as *const i8,
            g_capturelimit.integer,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"capturelimit\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_syscalls::trap_Cvar_Update(&mut g_capturelimit);
    }
    if g_gametype.integer >= crate::bg_public_h::GT_CTF as i32 && g_capturelimit.integer != 0 {
        if level.teamScores[crate::bg_public_h::TEAM_RED as usize] >= g_capturelimit.integer {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                -(1),
                b"print \"Red hit the capturelimit.\n\"\x00" as *const u8 as *const i8,
            );
            LogExit(b"Capturelimit hit.\x00" as *const u8 as *const i8);
            return;
        }
        if level.teamScores[crate::bg_public_h::TEAM_BLUE as usize] >= g_capturelimit.integer {
            crate::src::game::g_syscalls::trap_SendServerCommand(
                -(1),
                b"print \"Blue hit the capturelimit.\n\"\x00" as *const u8 as *const i8,
            );
            LogExit(b"Capturelimit hit.\x00" as *const u8 as *const i8);
            return;
        }
    };
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
    // check because we run 3 game frames before calling Connect and/or ClientBegin
    // for clients on a map_restart
    if level.numPlayingClients == 0 {
        return;
    }
    if g_gametype.integer == crate::bg_public_h::GT_TOURNAMENT as i32 {
        // pull in a spectator if needed
        if level.numPlayingClients < 2 {
            AddTournamentPlayer();
        }
        // if we don't have two players, go back to "waiting for players"
        if level.numPlayingClients != 2 {
            if level.warmupTime != -(1) {
                level.warmupTime = -(1);
                crate::src::game::g_syscalls::trap_SetConfigstring(
                    5,
                    crate::src::qcommon::q_shared::va(
                        b"%i\x00" as *const u8 as *mut i8,
                        level.warmupTime,
                    ),
                );
                G_LogPrintf(b"Warmup:\n\x00" as *const u8 as *const i8);
            }
            return;
        }
        if level.warmupTime == 0 {
            return;
        }
        // if the warmup is changed at the console, restart it
        if g_warmup.modificationCount != level.warmupModificationCount {
            level.warmupModificationCount = g_warmup.modificationCount;
            level.warmupTime = -(1)
        }
        // if all players have arrived, start the countdown
        if level.warmupTime < 0 {
            if level.numPlayingClients == 2 {
                // fudge by -1 to account for extra delays
                if g_warmup.integer > 1 {
                    level.warmupTime = level.time + (g_warmup.integer - 1) * 1000
                } else {
                    level.warmupTime = 0
                }
                crate::src::game::g_syscalls::trap_SetConfigstring(
                    5i32,
                    crate::src::qcommon::q_shared::va(
                        b"%i\x00" as *const u8 as *mut i8,
                        level.warmupTime,
                    ),
                );
            }
            return;
        }
        // if the warmup time has counted down, restart
        if level.time > level.warmupTime {
            level.warmupTime += 10000;
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"g_restarted\x00" as *const u8 as *const i8,
                b"1\x00" as *const u8 as *const i8,
            );
            crate::src::game::g_syscalls::trap_SendConsoleCommand(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"map_restart 0\n\x00" as *const u8 as *const i8,
            );
            level.restarted = crate::src::qcommon::q_shared::qtrue;
            return;
        }
    } else if g_gametype.integer != crate::bg_public_h::GT_SINGLE_PLAYER as i32
        && level.warmupTime != 0
    {
        let mut counts: [i32; 4] = [0; 4];
        let mut notEnough: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        if g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
            counts[crate::bg_public_h::TEAM_BLUE as usize] =
                crate::src::game::g_client::TeamCount(-(1), crate::bg_public_h::TEAM_BLUE);
            counts[crate::bg_public_h::TEAM_RED as usize] =
                crate::src::game::g_client::TeamCount(-(1), crate::bg_public_h::TEAM_RED);
            if counts[crate::bg_public_h::TEAM_RED as usize] < 1
                || counts[crate::bg_public_h::TEAM_BLUE as usize] < 1
            {
                notEnough = crate::src::qcommon::q_shared::qtrue
            }
        } else if level.numPlayingClients < 2 {
            notEnough = crate::src::qcommon::q_shared::qtrue
        }
        if notEnough as u64 != 0 {
            if level.warmupTime != -(1) {
                level.warmupTime = -(1);
                crate::src::game::g_syscalls::trap_SetConfigstring(
                    5,
                    crate::src::qcommon::q_shared::va(
                        b"%i\x00" as *const u8 as *mut i8,
                        level.warmupTime,
                    ),
                );
                G_LogPrintf(b"Warmup:\n\x00" as *const u8 as *const i8);
            }
            return;
            // still waiting for team members
        }
        if level.warmupTime == 0 {
            return;
        }
        // if the warmup is changed at the console, restart it
        if g_warmup.modificationCount != level.warmupModificationCount {
            level.warmupModificationCount = g_warmup.modificationCount;
            level.warmupTime = -(1)
        }
        // if all players have arrived, start the countdown
        if level.warmupTime < 0 {
            // fudge by -1 to account for extra delays
            if g_warmup.integer > 1 {
                level.warmupTime = level.time + (g_warmup.integer - 1) * 1000
            } else {
                level.warmupTime = 0
            }
            crate::src::game::g_syscalls::trap_SetConfigstring(
                5,
                crate::src::qcommon::q_shared::va(
                    b"%i\x00" as *const u8 as *mut i8,
                    level.warmupTime,
                ),
            );
            return;
        }
        // if the warmup time has counted down, restart
        if level.time > level.warmupTime {
            level.warmupTime += 10000;
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"g_restarted\x00" as *const u8 as *const i8,
                b"1\x00" as *const u8 as *const i8,
            );
            crate::src::game::g_syscalls::trap_SendConsoleCommand(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"map_restart 0\n\x00" as *const u8 as *const i8,
            );
            level.restarted = crate::src::qcommon::q_shared::qtrue;
            return;
        }
    };
}
/*
==================
CheckVote
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckVote() {
    if level.voteExecuteTime != 0 && level.voteExecuteTime < level.time {
        level.voteExecuteTime = 0;
        crate::src::game::g_syscalls::trap_SendConsoleCommand(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            crate::src::qcommon::q_shared::va(
                b"%s\n\x00" as *const u8 as *mut i8,
                level.voteString.as_mut_ptr(),
            ),
        );
    }
    if level.voteTime == 0 {
        return;
    }
    if level.time - level.voteTime >= 30000 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1i32),
            b"print \"Vote failed.\n\"\x00" as *const u8 as *const i8,
        );
    } else if level.voteYes > level.numVotingClients / 2 {
        // ATVI Q3 1.32 Patch #9, WNF
        // execute the command, then remove the vote
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1),
            b"print \"Vote passed.\n\"\x00" as *const u8 as *const i8,
        );
        level.voteExecuteTime = level.time + 3000
    } else if level.voteNo >= level.numVotingClients / 2 {
        // same behavior as a timeout
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1i32),
            b"print \"Vote failed.\n\"\x00" as *const u8 as *const i8,
        );
    } else {
        // still waiting for a majority
        return;
    }
    level.voteTime = 0;
    crate::src::game::g_syscalls::trap_SetConfigstring(8, b"\x00" as *const u8 as *const i8);
}
/*
==================
PrintTeam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn PrintTeam(mut team: i32, mut message: *mut i8) {
    let mut i: i32 = 0;
    i = 0;
    while i < level.maxclients {
        if !((*level.clients.offset(i as isize)).sess.sessionTeam != team as u32) {
            crate::src::game::g_syscalls::trap_SendServerCommand(i, message);
        }
        i += 1
    }
}
/*
==================
SetLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SetLeader(mut team: i32, mut client: i32) {
    let mut i: i32 = 0;
    if (*level.clients.offset(client as isize)).pers.connected == crate::g_local_h::CON_DISCONNECTED
    {
        PrintTeam(
            team,
            crate::src::qcommon::q_shared::va(
                b"print \"%s is not connected\n\"\x00" as *const u8 as *mut i8,
                (*level.clients.offset(client as isize))
                    .pers
                    .netname
                    .as_mut_ptr(),
            ),
        );
        return;
    }
    if (*level.clients.offset(client as isize)).sess.sessionTeam != team as u32 {
        PrintTeam(
            team,
            crate::src::qcommon::q_shared::va(
                b"print \"%s is not on the team anymore\n\"\x00" as *const u8 as *mut i8,
                (*level.clients.offset(client as isize))
                    .pers
                    .netname
                    .as_mut_ptr(),
            ),
        );
        return;
    }

    for i in 0..level.maxclients {
        if !((*level.clients.offset(i as isize)).sess.sessionTeam != team as u32) {
            if (*level.clients.offset(i as isize)).sess.teamLeader as u64 != 0 {
                (*level.clients.offset(i as isize)).sess.teamLeader =
                    crate::src::qcommon::q_shared::qfalse;
                crate::src::game::g_client::ClientUserinfoChanged(i);
            }
        }
    }
    (*level.clients.offset(client as isize)).sess.teamLeader = crate::src::qcommon::q_shared::qtrue;
    crate::src::game::g_client::ClientUserinfoChanged(client);
    PrintTeam(
        team,
        crate::src::qcommon::q_shared::va(
            b"print \"%s is the new team leader\n\"\x00" as *const u8 as *mut i8,
            (*level.clients.offset(client as isize))
                .pers
                .netname
                .as_mut_ptr(),
        ),
    );
}
/*
==================
CheckTeamLeader
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckTeamLeader(mut team: i32) {
    let mut i: i32 = 0;
    i = 0;
    while i < level.maxclients {
        if !((*level.clients.offset(i as isize)).sess.sessionTeam != team as u32) {
            if (*level.clients.offset(i as isize)).sess.teamLeader as u64 != 0 {
                break;
            }
        }
        i += 1
    }
    if i >= level.maxclients {
        i = 0;
        while i < level.maxclients {
            if !((*level.clients.offset(i as isize)).sess.sessionTeam != team as u32) {
                if g_entities[i as usize].r.svFlags & 0x8 == 0 {
                    (*level.clients.offset(i as isize)).sess.teamLeader =
                        crate::src::qcommon::q_shared::qtrue;
                    break;
                }
            }
            i += 1
        }
        if i >= level.maxclients {
            i = 0;
            while i < level.maxclients {
                if (*level.clients.offset(i as isize)).sess.sessionTeam != team as u32 {
                    i += 1
                } else {
                    (*level.clients.offset(i as isize)).sess.teamLeader =
                        crate::src::qcommon::q_shared::qtrue;
                    break;
                }
            }
        }
    };
}
/*
==================
CheckTeamVote
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckTeamVote(mut team: i32) {
    let mut cs_offset: i32 = 0;
    if team == crate::bg_public_h::TEAM_RED as i32 {
        cs_offset = 0
    } else if team == crate::bg_public_h::TEAM_BLUE as i32 {
        cs_offset = 1
    } else {
        return;
    }
    if level.teamVoteTime[cs_offset as usize] == 0 {
        return;
    }
    if level.time - level.teamVoteTime[cs_offset as usize] >= 30000 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1i32),
            b"print \"Team vote failed.\n\"\x00" as *const u8 as *const i8,
        );
    } else if level.teamVoteYes[cs_offset as usize]
        > level.numteamVotingClients[cs_offset as usize] / 2
    {
        // execute the command, then remove the vote
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1),
            b"print \"Team vote passed.\n\"\x00" as *const u8 as *const i8,
        );
        //
        if crate::src::qcommon::q_shared::Q_strncmp(
            b"leader\x00" as *const u8 as *const i8,
            level.teamVoteString[cs_offset as usize].as_mut_ptr(),
            6,
        ) == 0
        {
            //set the team leader
            SetLeader(
                team,
                atoi(
                    level.teamVoteString[cs_offset as usize]
                        .as_mut_ptr()
                        .offset(7isize),
                ),
            );
        } else {
            crate::src::game::g_syscalls::trap_SendConsoleCommand(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                crate::src::qcommon::q_shared::va(
                    b"%s\n\x00" as *const u8 as *mut i8,
                    level.teamVoteString[cs_offset as usize].as_mut_ptr(),
                ),
            );
        }
    } else if level.teamVoteNo[cs_offset as usize]
        >= level.numteamVotingClients[cs_offset as usize] / 2
    {
        // same behavior as a timeout
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1i32),
            b"print \"Team vote failed.\n\"\x00" as *const u8 as *const i8,
        );
    } else {
        // still waiting for a majority
        return;
    }
    level.teamVoteTime[cs_offset as usize] = 0;
    crate::src::game::g_syscalls::trap_SetConfigstring(
        12 + cs_offset,
        b"\x00" as *const u8 as *const i8,
    );
}
/*
==================
CheckCvars
==================
*/
#[no_mangle]

pub unsafe extern "C" fn CheckCvars() {
    static mut lastMod: i32 = -(1);
    if g_password.modificationCount != lastMod {
        lastMod = g_password.modificationCount;
        if *g_password.string.as_mut_ptr() as i32 != 0
            && crate::src::qcommon::q_shared::Q_stricmp(
                g_password.string.as_mut_ptr(),
                b"none\x00" as *const u8 as *const i8,
            ) != 0
        {
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"g_needpass\x00" as *const u8 as *const i8,
                b"1\x00" as *const u8 as *const i8,
            );
        } else {
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"g_needpass\x00" as *const u8 as *const i8,
                b"0\x00" as *const u8 as *const i8,
            );
        }
    };
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
// g_local.h -- local definitions for game module
//==================================================================
// the "gameversion" client command will print this plus compile date
// msec
// gentity->flags
// not the first on the team
// spawn point not for bot use
// spawn point just for bots
// force gesture on client
// movers are things like doors, plats, buttons, etc
//============================================================================
// communicated by server to clients
// shared by both the server system and game
// DO NOT MODIFY ANYTHING ABOVE THIS, THE SERVER
// EXPECTS THE FIELDS IN THAT ORDER!
//================================
// NULL if not a client
// set in QuakeEd
// set in QuakeEd
// if true, FreeEntity will only unlink
// bodyque uses this
// FL_* variables
// level.time when the object was freed
// events will be cleared EVENT_VALID_MSEC after set
// if true, it can be pushed by movers and fall off edges
// all game items are physicsObjects,
// 1.0 = continuous bounce, 0.0 = no bounce
// brushes with this content value will be collided against
// when moving.  items and corpses do not collide against
// players, for instance
// movers
// body queue sinking, etc
// movers call this when hitting endpoint
// wind tunnel
// quad will increase this without increasing radius
// next entity in team
// master of the team
// timing variables
// for bonus items
// Beginning a team game, spawn at base
// Now actively playing
// client data that stays across multiple levels or tournament restarts
// this is achieved by writing all the data to cvar strings at game shutdown
// time and reading them back at connection time.  Anything added here
// MUST be dealt with in G_InitSessionData() / G_ReadSessionData() / G_WriteSessionData()
// for determining next-in-line to play
// for chasecam and follow mode
// tournament stats
// true when this client is a team leader
//
// client data that stays across multiple respawns, but is cleared
// on each level change or team change at ClientBegin()
// we would lose angles if not persistant
// true if "ip" info key is "localhost"
// the first spawn should be at a cool location
// based on cg_predictItems userinfo
//
// for handicapping
// level.time the client entered the game
// status in teamplay games
// to prevent people from constantly calling votes
// to prevent people from constantly calling votes
// send team overlay updates?
// this structure is cleared on each ClientSpawn(),
// except for 'client->pers' and 'client->sess'
// ps MUST be the first element, because the server expects it
// communicated by server to clients
// the rest of the structure is private to game
// wishes to leave the intermission
// level.time of last usercmd_t, for EF_CONNECTION
// we can't just use pers.lastCommand.time, because
// of the g_sycronousclients case
// sum up damage over an entire frame, so
// shotgun blasts give a single big kick
// damage absorbed by armor
// damage taken out of health
// impact damage
// origin for vector calculation
// if true, don't use the damage_from vector
// for "impressive" reward sound
// total number of shots
// total number of hits
//
// last client that this client killed
// last client that damaged this client
// type of damage the client did
// timers
// can respawn when time > this, force after g_forcerespwan
// kick players when time > this
// qtrue if the five seoond warning has been given
// clear the EF_AWARD_IMPRESSIVE, etc when time > this
// for multiple kill rewards
// used for hook
// grapple hook if out
// time the player switched teams
// timeResidual is used to handle events that happen every second
// like health / armor countdowns and regeneration
//
// this structure is cleared as each map is entered
//
// [maxclients]
// MAX_CLIENTS <= num_entities <= ENTITYNUM_MAX_NORMAL
// restart match at this time
// store latched cvars here that we want to get at often
// in msec
// so movers can back up when blocked
// level.time the map was started
// last time of client team location update
// don't use any old session data, because
// we changed gametype
// waiting for a map_restart to fire
// includes connecting clients
// connected, non-spectators
// sorted by score
// clientNums for auto-follow spectators
// sound index for standing in lava
// for detecting if g_warmup is changed
// voting state
// level.time vote was called
// time the vote is executed
// set by CalculateRanks
// team voting state
// level.time vote was called
// set by CalculateRanks
// spawn variables
// the G_Spawn*() functions are valid
// key / value pairs
// intermission state
// intermission was qualified, but
// wait INTERMISSION_DELAY_TIME before
// actually going there so the last
// frag can be watched.  Disable future
// kills during this delay
// time the intermission was started
// at least one client wants to exit
// also used for spectator spawns
// target_locations get linked
// head of the location list
// dead bodies
//
// g_spawn.c
//
// spawn string returns a temporary reference, you must CopyString() if you want to keep it
//
// g_cmds.c
//
//
// g_items.c
//
//
// g_utils.c
//
//
// g_combat.c
//
// damage flags
// damage was indirect
// armour does not protect from this damage
// do not affect velocity, just view angles
// armor, shields, invulnerability, and godmode have no effect
//
// g_missile.c
//
//
// g_mover.c
//
//
// g_trigger.c
//
//
// g_misc.c
//
//
// g_weapon.c
//
//
// g_client.c
//
//
// g_svcmds.c
//
//
// g_weapon.c
//
//
// g_cmds.c
//
//
// g_main.c
//
/*
=============
G_RunThink

Runs thinking code for this frame if necessary
=============
*/
#[no_mangle]

pub unsafe extern "C" fn G_RunThink(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut thinktime: i32 = 0;
    thinktime = (*ent).nextthink;
    if thinktime <= 0 {
        return;
    }
    if thinktime > level.time {
        return;
    }
    (*ent).nextthink = 0;
    if (*ent).think.is_none() {
        G_Error(b"NULL ent->think\x00" as *const u8 as *const i8);
    }
    (*ent).think.expect("non-null function pointer")(ent);
}
/*
================
G_RunFrame

Advances the non-player objects in the world
================
*/
#[no_mangle]

pub unsafe extern "C" fn G_RunFrame(mut levelTime: i32) {
    let mut i: i32 = 0;
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    // if we are waiting for the level to restart, do nothing
    if level.restarted as u64 != 0 {
        return;
    }
    level.framenum += 1;
    level.previousTime = level.time;
    level.time = levelTime;
    // get any cvar changes
    G_UpdateCvars();
    //
    // go through all allocated objects
    //
    ent = &mut *g_entities.as_mut_ptr().offset(0) as *mut crate::g_local_h::gentity_t;
    let mut current_block_24: u64;
    i = 0;
    while i < level.num_entities {
        if !((*ent).inuse as u64 == 0) {
            // clear events that are too old
            if level.time - (*ent).eventTime > 300 {
                if (*ent).s.event != 0 {
                    (*ent).s.event = 0; // &= EV_EVENT_BITS;
                    if !(*ent).client.is_null() {
                        (*(*ent).client).ps.externalEvent = 0
                        // predicted events should never be set to zero
                        //ent->client->ps.events[0] = 0;
                        //ent->client->ps.events[1] = 0;
                    }
                }
                if (*ent).freeAfterEvent as u64 != 0 {
                    // tempEntities or dropped items completely go away after their event
                    crate::src::game::g_utils::G_FreeEntity(ent);
                    current_block_24 = 17216689946888361452;
                } else {
                    if (*ent).unlinkAfterEvent as u64 != 0 {
                        // items that will respawn will hide themselves after their pickup event
                        (*ent).unlinkAfterEvent = crate::src::qcommon::q_shared::qfalse;
                        crate::src::game::g_syscalls::trap_UnlinkEntity(ent);
                    }
                    current_block_24 = 17478428563724192186;
                }
            } else {
                current_block_24 = 17478428563724192186;
            }
            match current_block_24 {
                17216689946888361452 => {}
                _ =>
                // temporary entities don't think
                {
                    if !((*ent).freeAfterEvent as u64 != 0) {
                        if !((*ent).r.linked as u64 == 0 && (*ent).neverFree != 0) {
                            if (*ent).s.eType == crate::bg_public_h::ET_MISSILE as i32 {
                                crate::src::game::g_missile::G_RunMissile(ent);
                            } else if (*ent).s.eType == crate::bg_public_h::ET_ITEM as i32
                                || (*ent).physicsObject != 0
                            {
                                crate::src::game::g_items::G_RunItem(ent);
                            } else if (*ent).s.eType == crate::bg_public_h::ET_MOVER as i32 {
                                crate::src::game::g_mover::G_RunMover(ent);
                            } else if i < 64 {
                                crate::src::game::g_active::G_RunClient(ent);
                            } else {
                                G_RunThink(ent);
                            }
                        }
                    }
                }
            }
        }
        i += 1;
        ent = ent.offset(1)
    }
    // perform final fixups on the players
    ent = &mut *g_entities.as_mut_ptr().offset(0) as *mut crate::g_local_h::gentity_t;
    i = 0;
    while i < level.maxclients {
        if (*ent).inuse as u64 != 0 {
            crate::src::game::g_active::ClientEndFrame(ent);
        }
        i += 1;
        ent = ent.offset(1)
    }
    // see if it is time to do a tournement restart
    CheckTournament();
    // see if it is time to end the level
    CheckExitRules();
    // update to team status?
    crate::src::game::g_team::CheckTeamStatus();
    // cancel vote if timed out
    CheckVote();
    // check team votes
    CheckTeamVote(crate::bg_public_h::TEAM_RED as i32);
    CheckTeamVote(crate::bg_public_h::TEAM_BLUE as i32);
    // for tracking changes
    CheckCvars();
    if g_listEntity.integer != 0 {
        i = 0;
        while i < (1) << 10 {
            G_Printf(
                b"%4i: %s\n\x00" as *const u8 as *const i8,
                i,
                g_entities[i as usize].classname,
            );
            i += 1
        }
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_listEntity\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
    };
}
unsafe extern "C" fn run_static_initializers() {
    gameCvarTableSize = (::std::mem::size_of::<[cvarTable_t; 46]>())
        .wrapping_div(::std::mem::size_of::<cvarTable_t>()) as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
