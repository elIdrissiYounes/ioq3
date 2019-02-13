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
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, unnamed_1, unnamed_2, unnamed_3,
    unnamed_4, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON, EV_DEATH1,
    EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT,
    EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND,
    EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE,
    EV_INVUL_IMPACT, EV_ITEM_PICKUP, EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED, EV_JUMP, EV_JUMP_PAD,
    EV_KAMIKAZE, EV_LIGHTNINGBOLT, EV_MISSILE_HIT, EV_MISSILE_MISS, EV_MISSILE_MISS_METAL,
    EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE, EV_OBELISKPAIN, EV_OBITUARY, EV_PAIN,
    EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT, EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD,
    EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK, EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL,
    EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16, EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND,
    EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME, EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO,
    EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0, EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11,
    EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14, EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3,
    EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6, EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9,
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, GTS_BLUEOBELISK_ATTACKED,
    GTS_BLUETEAM_SCORED, GTS_BLUETEAM_TOOK_LEAD, GTS_BLUE_CAPTURE, GTS_BLUE_RETURN, GTS_BLUE_TAKEN,
    GTS_KAMIKAZE, GTS_REDOBELISK_ATTACKED, GTS_REDTEAM_SCORED, GTS_REDTEAM_TOOK_LEAD,
    GTS_RED_CAPTURE, GTS_RED_RETURN, GTS_RED_TAKEN, GTS_TEAMS_ARE_TIED, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO,
    IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM,
    IT_WEAPON, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES,
    PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS,
    PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT,
    PERS_TEAM, PW_AMMOREGEN, PW_BATTLESUIT, PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE,
    PW_INVIS, PW_INVULNERABILITY, PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG,
    PW_REGEN, PW_SCOUT, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH,
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
    spectatorState_t, trap_InPVS, trap_SendServerCommand, trap_SetConfigstring, CON_CONNECTED,
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
use g_trigger::{
    SP_func_timer, SP_target_push, SP_trigger_always, SP_trigger_hurt, SP_trigger_multiple,
    SP_trigger_push, SP_trigger_teleport,
};
use g_utils::{
    tv, vectoyaw, vtos, G_AddEvent, G_AddPredictableEvent, G_Find, G_FreeEntity, G_InitGentity,
    G_KillBox, G_ModelIndex, G_PickTarget, G_SetMovedir, G_SetOrigin, G_Sound, G_SoundIndex,
    G_Spawn, G_TeamCommand, G_TempEntity, G_UseTargets,
};
use g_variadic_h::{G_Printf, PrintMsg};
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
    _flag_status, byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t,
    fileHandle_t, flagStatus_t, playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t,
    trace_t, trajectory_t, usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, Com_sprintf,
    FLAG_ATBASE, FLAG_DROPPED, FLAG_TAKEN, FLAG_TAKEN_BLUE, FLAG_TAKEN_RED, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stddef_h::size_t;
use stdlib::{__compar_fn_t, memset, qsort, rand, sqrt, strcmp, strcpy, strlen};

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
//
// g_team.c
//
#[no_mangle]
pub unsafe extern "C" fn OnSameTeam(
    mut ent1: *mut gentity_t,
    mut ent2: *mut gentity_t,
) -> qboolean {
    if (*ent1).client.is_null() || (*ent2).client.is_null() {
        return qfalse;
    }
    if g_gametype.integer < GT_TEAM as libc::c_int {
        return qfalse;
    }
    if (*(*ent1).client).sess.sessionTeam as libc::c_uint
        == (*(*ent2).client).sess.sessionTeam as libc::c_uint
    {
        return qtrue;
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn Team_CheckDroppedItem(mut dropped: *mut gentity_t) {
    if (*(*dropped).item).giTag == PW_REDFLAG as libc::c_int {
        Team_SetFlagStatus(TEAM_RED as libc::c_int, FLAG_DROPPED);
    } else if (*(*dropped).item).giTag == PW_BLUEFLAG as libc::c_int {
        Team_SetFlagStatus(TEAM_BLUE as libc::c_int, FLAG_DROPPED);
    } else if (*(*dropped).item).giTag == PW_NEUTRALFLAG as libc::c_int {
        Team_SetFlagStatus(TEAM_FREE as libc::c_int, FLAG_DROPPED);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Team_SetFlagStatus(mut team: libc::c_int, mut status: flagStatus_t) {
    let mut modified: qboolean = qfalse;
    match team {
        1 => {
            if teamgame.redStatus as libc::c_uint != status as libc::c_uint {
                teamgame.redStatus = status;
                modified = qtrue
            }
        }
        2 => {
            if teamgame.blueStatus as libc::c_uint != status as libc::c_uint {
                teamgame.blueStatus = status;
                modified = qtrue
            }
        }
        0 => {
            if teamgame.flagStatus as libc::c_uint != status as libc::c_uint {
                teamgame.flagStatus = status;
                modified = qtrue
            }
        }
        _ => {}
    }
    if 0 != modified as u64 {
        let mut st: [libc::c_char; 4] = [0; 4];
        if g_gametype.integer == GT_CTF as libc::c_int {
            st[0usize] = ctfFlagStatusRemap[teamgame.redStatus as usize];
            st[1usize] = ctfFlagStatusRemap[teamgame.blueStatus as usize];
            st[2usize] = 0i32 as libc::c_char
        } else {
            st[0usize] = oneFlagStatusRemap[teamgame.flagStatus as usize];
            st[1usize] = 0i32 as libc::c_char
        }
        trap_SetConfigstring(23i32, st.as_mut_ptr());
    };
}
#[no_mangle]
pub static mut teamgame: teamgame_t = teamgame_s {
    last_flag_capture: 0.,
    last_capture_team: 0,
    redStatus: FLAG_ATBASE,
    blueStatus: FLAG_ATBASE,
    flagStatus: FLAG_ATBASE,
    redTakenTime: 0,
    blueTakenTime: 0,
    redObeliskAttackedTime: 0,
    blueObeliskAttackedTime: 0,
};
static mut oneFlagStatusRemap: [libc::c_char; 5] = [
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
];
static mut ctfFlagStatusRemap: [libc::c_char; 5] = [
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '*' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
];
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
// what you get for capture
// what your team gets for capture
// what you get for recovery
// what you get for picking up enemy flag
// what you get for fragging enemy flag carrier
// seconds until auto return
// bonus for fraggin someone who has recently hurt your flag carrier
// bonus for fraggin someone while either you or your target are near your flag carrier
// bonus for fraggin someone while either you or your target are near your flag
// awarded for returning a flag that causes a capture to happen almost immediately
// award for fragging a flag carrier if a capture happens almost immediately
// the radius around an object being defended where a target will be worth extra frags
// the radius around an object being defended where an attacker will get extra frags when making kills
// speed of grapple in flight
// speed player is pulled at
// Prototypes
#[no_mangle]
pub unsafe extern "C" fn OtherTeam(mut team: libc::c_int) -> libc::c_int {
    if team == TEAM_RED as libc::c_int {
        return TEAM_BLUE as libc::c_int;
    } else {
        if team == TEAM_BLUE as libc::c_int {
            return TEAM_RED as libc::c_int;
        }
    }
    return team;
}
#[no_mangle]
pub unsafe extern "C" fn TeamName(mut team: libc::c_int) -> *const libc::c_char {
    if team == TEAM_RED as libc::c_int {
        return b"RED\x00" as *const u8 as *const libc::c_char;
    } else {
        if team == TEAM_BLUE as libc::c_int {
            return b"BLUE\x00" as *const u8 as *const libc::c_char;
        } else {
            if team == TEAM_SPECTATOR as libc::c_int {
                return b"SPECTATOR\x00" as *const u8 as *const libc::c_char;
            }
        }
    }
    return b"FREE\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn TeamColorString(mut team: libc::c_int) -> *const libc::c_char {
    if team == TEAM_RED as libc::c_int {
        return b"^1\x00" as *const u8 as *const libc::c_char;
    } else {
        if team == TEAM_BLUE as libc::c_int {
            return b"^4\x00" as *const u8 as *const libc::c_char;
        } else {
            if team == TEAM_SPECTATOR as libc::c_int {
                return b"^3\x00" as *const u8 as *const libc::c_char;
            }
        }
    }
    return b"^7\x00" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn AddTeamScore(
    mut origin: *mut vec_t,
    mut team: libc::c_int,
    mut score: libc::c_int,
) {
    let mut te: *mut gentity_t = 0 as *mut gentity_t;
    te = G_TempEntity(origin, EV_GLOBAL_TEAM_SOUND as libc::c_int);
    (*te).r.svFlags |= 0x20i32;
    if team == TEAM_RED as libc::c_int {
        if level.teamScores[TEAM_RED as libc::c_int as usize] + score
            == level.teamScores[TEAM_BLUE as libc::c_int as usize]
        {
            (*te).s.eventParm = GTS_TEAMS_ARE_TIED as libc::c_int
        } else if level.teamScores[TEAM_RED as libc::c_int as usize]
            <= level.teamScores[TEAM_BLUE as libc::c_int as usize]
            && level.teamScores[TEAM_RED as libc::c_int as usize] + score
                > level.teamScores[TEAM_BLUE as libc::c_int as usize]
        {
            (*te).s.eventParm = GTS_REDTEAM_TOOK_LEAD as libc::c_int
        } else {
            (*te).s.eventParm = GTS_REDTEAM_SCORED as libc::c_int
        }
    } else if level.teamScores[TEAM_BLUE as libc::c_int as usize] + score
        == level.teamScores[TEAM_RED as libc::c_int as usize]
    {
        (*te).s.eventParm = GTS_TEAMS_ARE_TIED as libc::c_int
    } else if level.teamScores[TEAM_BLUE as libc::c_int as usize]
        <= level.teamScores[TEAM_RED as libc::c_int as usize]
        && level.teamScores[TEAM_BLUE as libc::c_int as usize] + score
            > level.teamScores[TEAM_RED as libc::c_int as usize]
    {
        (*te).s.eventParm = GTS_BLUETEAM_TOOK_LEAD as libc::c_int
    } else {
        (*te).s.eventParm = GTS_BLUETEAM_SCORED as libc::c_int
    }
    level.teamScores[team as usize] += score;
}
#[no_mangle]
pub unsafe extern "C" fn Team_DroppedFlagThink(mut ent: *mut gentity_t) {
    let mut team: libc::c_int = TEAM_FREE as libc::c_int;
    if (*(*ent).item).giTag == PW_REDFLAG as libc::c_int {
        team = TEAM_RED as libc::c_int
    } else if (*(*ent).item).giTag == PW_BLUEFLAG as libc::c_int {
        team = TEAM_BLUE as libc::c_int
    } else if (*(*ent).item).giTag == PW_NEUTRALFLAG as libc::c_int {
        team = TEAM_FREE as libc::c_int
    }
    Team_ReturnFlagSound(Team_ResetFlag(team), team);
}
#[no_mangle]
pub unsafe extern "C" fn Team_ResetFlag(mut team: libc::c_int) -> *mut gentity_t {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut rent: *mut gentity_t = 0 as *mut gentity_t;
    match team {
        1 => c = b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => c = b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 => {
            c = b"team_CTF_neutralflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        _ => return 0 as *mut gentity_t,
    }
    ent = 0 as *mut gentity_t;
    loop {
        ent = G_Find(
            ent,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            c,
        );
        if ent.is_null() {
            break;
        }
        if 0 != (*ent).flags & 0x1000i32 {
            G_FreeEntity(ent);
        } else {
            rent = ent;
            RespawnItem(ent);
        }
    }
    Team_SetFlagStatus(team, FLAG_ATBASE);
    return rent;
}
#[no_mangle]
pub unsafe extern "C" fn Team_ReturnFlagSound(mut ent: *mut gentity_t, mut team: libc::c_int) {
    let mut te: *mut gentity_t = 0 as *mut gentity_t;
    if ent.is_null() {
        G_Printf(
            b"Warning:  NULL passed to Team_ReturnFlagSound\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    te = G_TempEntity(
        (*ent).s.pos.trBase.as_mut_ptr(),
        EV_GLOBAL_TEAM_SOUND as libc::c_int,
    );
    if team == TEAM_BLUE as libc::c_int {
        (*te).s.eventParm = GTS_RED_RETURN as libc::c_int
    } else {
        (*te).s.eventParm = GTS_BLUE_RETURN as libc::c_int
    }
    (*te).r.svFlags |= 0x20i32;
}
#[no_mangle]
pub unsafe extern "C" fn Team_FragBonuses(
    mut targ: *mut gentity_t,
    mut inflictor: *mut gentity_t,
    mut attacker: *mut gentity_t,
) {
    let mut i: libc::c_int = 0;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut flag_pw: libc::c_int = 0;
    let mut enemy_flag_pw: libc::c_int = 0;
    let mut otherteam: libc::c_int = 0;
    let mut tokens: libc::c_int = 0;
    let mut flag: *mut gentity_t = 0 as *mut gentity_t;
    let mut carrier: *mut gentity_t = 0 as *mut gentity_t;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    let mut team: libc::c_int = 0;
    if (*targ).client.is_null()
        || (*attacker).client.is_null()
        || targ == attacker
        || 0 != OnSameTeam(targ, attacker) as libc::c_uint
    {
        return;
    }
    team = (*(*targ).client).sess.sessionTeam as libc::c_int;
    otherteam = OtherTeam((*(*targ).client).sess.sessionTeam as libc::c_int);
    if otherteam < 0i32 {
        return;
    }
    if team == TEAM_RED as libc::c_int {
        flag_pw = PW_REDFLAG as libc::c_int;
        enemy_flag_pw = PW_BLUEFLAG as libc::c_int
    } else {
        flag_pw = PW_BLUEFLAG as libc::c_int;
        enemy_flag_pw = PW_REDFLAG as libc::c_int
    }
    tokens = 0i32;
    if 0 != (*(*targ).client).ps.powerups[enemy_flag_pw as usize] {
        (*(*attacker).client).pers.teamState.lastfraggedcarrier = level.time as libc::c_float;
        AddScore(attacker, (*targ).r.currentOrigin.as_mut_ptr(), 2i32);
        (*(*attacker).client).pers.teamState.fragcarrier += 1;
        PrintMsg(
            0 as *mut gentity_t,
            b"%s^7 fragged %s\'s flag carrier!\n\x00" as *const u8 as *const libc::c_char,
            (*(*attacker).client).pers.netname.as_mut_ptr(),
            TeamName(team),
        );
        i = 0i32;
        while i < g_maxclients.integer {
            ent = g_entities.as_mut_ptr().offset(i as isize);
            if 0 != (*ent).inuse as libc::c_uint
                && (*(*ent).client).sess.sessionTeam as libc::c_uint == otherteam as libc::c_uint
            {
                (*(*ent).client).pers.teamState.lasthurtcarrier = 0i32 as libc::c_float
            }
            i += 1
        }
        return;
    }
    if 0 != tokens {
        (*(*attacker).client).pers.teamState.lastfraggedcarrier = level.time as libc::c_float;
        AddScore(
            attacker,
            (*targ).r.currentOrigin.as_mut_ptr(),
            2i32 * tokens * tokens,
        );
        (*(*attacker).client).pers.teamState.fragcarrier += 1;
        PrintMsg(
            0 as *mut gentity_t,
            b"%s^7 fragged %s\'s skull carrier!\n\x00" as *const u8 as *const libc::c_char,
            (*(*attacker).client).pers.netname.as_mut_ptr(),
            TeamName(team),
        );
        i = 0i32;
        while i < g_maxclients.integer {
            ent = g_entities.as_mut_ptr().offset(i as isize);
            if 0 != (*ent).inuse as libc::c_uint
                && (*(*ent).client).sess.sessionTeam as libc::c_uint == otherteam as libc::c_uint
            {
                (*(*ent).client).pers.teamState.lasthurtcarrier = 0i32 as libc::c_float
            }
            i += 1
        }
        return;
    }
    if 0. != (*(*targ).client).pers.teamState.lasthurtcarrier
        && level.time as libc::c_float - (*(*targ).client).pers.teamState.lasthurtcarrier
            < 8000i32 as libc::c_float
        && 0 == (*(*attacker).client).ps.powerups[flag_pw as usize]
    {
        AddScore(attacker, (*targ).r.currentOrigin.as_mut_ptr(), 2i32);
        (*(*attacker).client).pers.teamState.carrierdefense += 1;
        (*(*targ).client).pers.teamState.lasthurtcarrier = 0i32 as libc::c_float;
        (*(*attacker).client).ps.persistant[PERS_DEFEND_COUNT as libc::c_int as usize] += 1;
        (*(*attacker).client).ps.eFlags &=
            !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
        (*(*attacker).client).ps.eFlags |= 0x10000i32;
        (*(*attacker).client).rewardTime = level.time + 2000i32;
        return;
    }
    match (*(*attacker).client).sess.sessionTeam as libc::c_uint {
        1 => c = b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => c = b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => return,
    }
    i = 0i32;
    while i < g_maxclients.integer {
        carrier = g_entities.as_mut_ptr().offset(i as isize);
        if 0 != (*carrier).inuse as libc::c_uint
            && 0 != (*(*carrier).client).ps.powerups[flag_pw as usize]
        {
            break;
        }
        carrier = 0 as *mut gentity_t;
        i += 1
    }
    flag = 0 as *mut gentity_t;
    loop {
        flag = G_Find(
            flag,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            c,
        );
        if flag.is_null() {
            break;
        }
        if 0 == (*flag).flags & 0x1000i32 {
            break;
        }
    }
    if flag.is_null() {
        return;
    }
    v1[0usize] = (*targ).r.currentOrigin[0usize] - (*flag).r.currentOrigin[0usize];
    v1[1usize] = (*targ).r.currentOrigin[1usize] - (*flag).r.currentOrigin[1usize];
    v1[2usize] = (*targ).r.currentOrigin[2usize] - (*flag).r.currentOrigin[2usize];
    v2[0usize] = (*attacker).r.currentOrigin[0usize] - (*flag).r.currentOrigin[0usize];
    v2[1usize] = (*attacker).r.currentOrigin[1usize] - (*flag).r.currentOrigin[1usize];
    v2[2usize] = (*attacker).r.currentOrigin[2usize] - (*flag).r.currentOrigin[2usize];
    if (VectorLength(v1.as_mut_ptr() as *const vec_t) < 1000i32 as libc::c_float
        && 0 != trap_InPVS(
            (*flag).r.currentOrigin.as_mut_ptr() as *const vec_t,
            (*targ).r.currentOrigin.as_mut_ptr() as *const vec_t,
        ) as libc::c_uint
        || VectorLength(v2.as_mut_ptr() as *const vec_t) < 1000i32 as libc::c_float
            && 0 != trap_InPVS(
                (*flag).r.currentOrigin.as_mut_ptr() as *const vec_t,
                (*attacker).r.currentOrigin.as_mut_ptr() as *const vec_t,
            ) as libc::c_uint)
        && (*(*attacker).client).sess.sessionTeam as libc::c_uint
            != (*(*targ).client).sess.sessionTeam as libc::c_uint
    {
        AddScore(attacker, (*targ).r.currentOrigin.as_mut_ptr(), 1i32);
        (*(*attacker).client).pers.teamState.basedefense += 1;
        (*(*attacker).client).ps.persistant[PERS_DEFEND_COUNT as libc::c_int as usize] += 1;
        (*(*attacker).client).ps.eFlags &=
            !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
        (*(*attacker).client).ps.eFlags |= 0x10000i32;
        (*(*attacker).client).rewardTime = level.time + 2000i32;
        return;
    }
    if !carrier.is_null() && carrier != attacker {
        v1[0usize] = (*targ).r.currentOrigin[0usize] - (*carrier).r.currentOrigin[0usize];
        v1[1usize] = (*targ).r.currentOrigin[1usize] - (*carrier).r.currentOrigin[1usize];
        v1[2usize] = (*targ).r.currentOrigin[2usize] - (*carrier).r.currentOrigin[2usize];
        v2[0usize] = (*attacker).r.currentOrigin[0usize] - (*carrier).r.currentOrigin[0usize];
        v2[1usize] = (*attacker).r.currentOrigin[1usize] - (*carrier).r.currentOrigin[1usize];
        v2[2usize] = (*attacker).r.currentOrigin[2usize] - (*carrier).r.currentOrigin[2usize];
        if (VectorLength(v1.as_mut_ptr() as *const vec_t) < 1000i32 as libc::c_float
            && 0 != trap_InPVS(
                (*carrier).r.currentOrigin.as_mut_ptr() as *const vec_t,
                (*targ).r.currentOrigin.as_mut_ptr() as *const vec_t,
            ) as libc::c_uint
            || VectorLength(v2.as_mut_ptr() as *const vec_t) < 1000i32 as libc::c_float
                && 0 != trap_InPVS(
                    (*carrier).r.currentOrigin.as_mut_ptr() as *const vec_t,
                    (*attacker).r.currentOrigin.as_mut_ptr() as *const vec_t,
                ) as libc::c_uint)
            && (*(*attacker).client).sess.sessionTeam as libc::c_uint
                != (*(*targ).client).sess.sessionTeam as libc::c_uint
        {
            AddScore(attacker, (*targ).r.currentOrigin.as_mut_ptr(), 1i32);
            (*(*attacker).client).pers.teamState.carrierdefense += 1;
            (*(*attacker).client).ps.persistant[PERS_DEFEND_COUNT as libc::c_int as usize] += 1;
            (*(*attacker).client).ps.eFlags &=
                !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
            (*(*attacker).client).ps.eFlags |= 0x10000i32;
            (*(*attacker).client).rewardTime = level.time + 2000i32;
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Team_CheckHurtCarrier(
    mut targ: *mut gentity_t,
    mut attacker: *mut gentity_t,
) {
    let mut flag_pw: libc::c_int = 0;
    if (*targ).client.is_null() || (*attacker).client.is_null() {
        return;
    }
    if (*(*targ).client).sess.sessionTeam as libc::c_uint == TEAM_RED as libc::c_int as libc::c_uint
    {
        flag_pw = PW_BLUEFLAG as libc::c_int
    } else {
        flag_pw = PW_REDFLAG as libc::c_int
    }
    if 0 != (*(*targ).client).ps.powerups[flag_pw as usize]
        && (*(*targ).client).sess.sessionTeam as libc::c_uint
            != (*(*attacker).client).sess.sessionTeam as libc::c_uint
    {
        (*(*attacker).client).pers.teamState.lasthurtcarrier = level.time as libc::c_float
    }
    if 0 != (*(*targ).client).ps.generic1
        && (*(*targ).client).sess.sessionTeam as libc::c_uint
            != (*(*attacker).client).sess.sessionTeam as libc::c_uint
    {
        (*(*attacker).client).pers.teamState.lasthurtcarrier = level.time as libc::c_float
    };
}
#[no_mangle]
pub unsafe extern "C" fn Team_InitGame() {
    memset(
        &mut teamgame as *mut teamgame_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<teamgame_t>() as libc::c_ulong,
    );
    match g_gametype.integer {
        4 => {
            teamgame.redStatus = 4294967295 as flagStatus_t;
            Team_SetFlagStatus(TEAM_RED as libc::c_int, FLAG_ATBASE);
            teamgame.blueStatus = 4294967295 as flagStatus_t;
            Team_SetFlagStatus(TEAM_BLUE as libc::c_int, FLAG_ATBASE);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn Team_ReturnFlag(mut team: libc::c_int) {
    Team_ReturnFlagSound(Team_ResetFlag(team), team);
    if team == TEAM_FREE as libc::c_int {
        PrintMsg(
            0 as *mut gentity_t,
            b"The flag has returned!\n\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        PrintMsg(
            0 as *mut gentity_t,
            b"The %s flag has returned!\n\x00" as *const u8 as *const libc::c_char,
            TeamName(team),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Team_FreeEntity(mut ent: *mut gentity_t) {
    if (*(*ent).item).giTag == PW_REDFLAG as libc::c_int {
        Team_ReturnFlag(TEAM_RED as libc::c_int);
    } else if (*(*ent).item).giTag == PW_BLUEFLAG as libc::c_int {
        Team_ReturnFlag(TEAM_BLUE as libc::c_int);
    } else if (*(*ent).item).giTag == PW_NEUTRALFLAG as libc::c_int {
        Team_ReturnFlag(TEAM_FREE as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SelectCTFSpawnPoint(
    mut team: team_t,
    mut teamstate: libc::c_int,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
    mut isbot: qboolean,
) -> *mut gentity_t {
    let mut spot: *mut gentity_t = 0 as *mut gentity_t;
    spot = SelectRandomTeamSpawnPoint(teamstate, team);
    if spot.is_null() {
        return SelectSpawnPoint(vec3_origin.as_mut_ptr(), origin, angles, isbot);
    }
    *origin.offset(0isize) = (*spot).s.origin[0usize];
    *origin.offset(1isize) = (*spot).s.origin[1usize];
    *origin.offset(2isize) = (*spot).s.origin[2usize];
    let ref mut fresh0 = *origin.offset(2isize);
    *fresh0 += 9i32 as libc::c_float;
    *angles.offset(0isize) = (*spot).s.angles[0usize];
    *angles.offset(1isize) = (*spot).s.angles[1usize];
    *angles.offset(2isize) = (*spot).s.angles[2usize];
    return spot;
}
/*---------------------------------------------------------------------------*/
/*
================
SelectRandomTeamSpawnPoint

go to a random point that doesn't telefrag
================
*/
#[no_mangle]
pub unsafe extern "C" fn SelectRandomTeamSpawnPoint(
    mut teamstate: libc::c_int,
    mut team: team_t,
) -> *mut gentity_t {
    let mut spot: *mut gentity_t = 0 as *mut gentity_t;
    let mut count: libc::c_int = 0;
    let mut selection: libc::c_int = 0;
    let mut spots: [*mut gentity_t; 32] = [0 as *mut gentity_t; 32];
    let mut classname: *mut libc::c_char = 0 as *mut libc::c_char;
    if teamstate == TEAM_BEGIN as libc::c_int {
        if team as libc::c_uint == TEAM_RED as libc::c_int as libc::c_uint {
            classname =
                b"team_CTF_redplayer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else if team as libc::c_uint == TEAM_BLUE as libc::c_int as libc::c_uint {
            classname =
                b"team_CTF_blueplayer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            return 0 as *mut gentity_t;
        }
    } else if team as libc::c_uint == TEAM_RED as libc::c_int as libc::c_uint {
        classname =
            b"team_CTF_redspawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if team as libc::c_uint == TEAM_BLUE as libc::c_int as libc::c_uint {
        classname =
            b"team_CTF_bluespawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        return 0 as *mut gentity_t;
    }
    count = 0i32;
    spot = 0 as *mut gentity_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            classname,
        );
        if spot.is_null() {
            break;
        }
        if 0 != SpotWouldTelefrag(spot) as u64 {
            continue;
        }
        spots[count as usize] = spot;
        count += 1;
        if count == 32i32 {
            break;
        }
    }
    if 0 == count {
        return G_Find(
            0 as *mut gentity_t,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            classname,
        );
    }
    selection = rand() % count;
    return spots[selection as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Team_GetLocation(mut ent: *mut gentity_t) -> *mut gentity_t {
    let mut eloc: *mut gentity_t = 0 as *mut gentity_t;
    let mut best: *mut gentity_t = 0 as *mut gentity_t;
    let mut bestlen: libc::c_float = 0.;
    let mut len: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    best = 0 as *mut gentity_t;
    bestlen = (3i32 as libc::c_double * 8192.0f64 * 8192.0f64) as libc::c_float;
    origin[0usize] = (*ent).r.currentOrigin[0usize];
    origin[1usize] = (*ent).r.currentOrigin[1usize];
    origin[2usize] = (*ent).r.currentOrigin[2usize];
    eloc = level.locationHead;
    while !eloc.is_null() {
        len = (origin[0usize] - (*eloc).r.currentOrigin[0usize])
            * (origin[0usize] - (*eloc).r.currentOrigin[0usize])
            + (origin[1usize] - (*eloc).r.currentOrigin[1usize])
                * (origin[1usize] - (*eloc).r.currentOrigin[1usize])
            + (origin[2usize] - (*eloc).r.currentOrigin[2usize])
                * (origin[2usize] - (*eloc).r.currentOrigin[2usize]);
        if !(len > bestlen) {
            if !(0
                == trap_InPVS(
                    origin.as_mut_ptr() as *const vec_t,
                    (*eloc).r.currentOrigin.as_mut_ptr() as *const vec_t,
                ) as u64)
            {
                bestlen = len;
                best = eloc
            }
        }
        eloc = (*eloc).nextTrain
    }
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn Team_GetLocationMsg(
    mut ent: *mut gentity_t,
    mut loc: *mut libc::c_char,
    mut loclen: libc::c_int,
) -> qboolean {
    let mut best: *mut gentity_t = 0 as *mut gentity_t;
    best = Team_GetLocation(ent);
    if best.is_null() {
        return qfalse;
    }
    if 0 != (*best).count {
        if (*best).count < 0i32 {
            (*best).count = 0i32
        }
        if (*best).count > 7i32 {
            (*best).count = 7i32
        }
        Com_sprintf(
            loc,
            loclen,
            b"%c%c%s^7\x00" as *const u8 as *const libc::c_char,
            '^' as i32,
            (*best).count + '0' as i32,
            (*best).message,
        );
    } else {
        Com_sprintf(
            loc,
            loclen,
            b"%s\x00" as *const u8 as *const libc::c_char,
            (*best).message,
        );
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn TeamplayInfoMessage(mut ent: *mut gentity_t) {
    let mut entry: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 8192] = [0; 8192];
    let mut stringlength: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut player: *mut gentity_t = 0 as *mut gentity_t;
    let mut cnt: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut clients: [libc::c_int; 32] = [0; 32];
    let mut team: libc::c_int = 0;
    if 0 == (*(*ent).client).pers.teamInfo as u64 {
        return;
    }
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        if (*(*ent).client).sess.spectatorState as libc::c_uint
            != SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
            || (*(*ent).client).sess.spectatorClient < 0i32
        {
            return;
        }
        team = (*g_entities[(*(*ent).client).sess.spectatorClient as usize].client)
            .sess
            .sessionTeam as libc::c_int
    } else {
        team = (*(*ent).client).sess.sessionTeam as libc::c_int
    }
    if team != TEAM_RED as libc::c_int && team != TEAM_BLUE as libc::c_int {
        return;
    }
    i = 0i32;
    cnt = 0i32;
    while i < g_maxclients.integer && cnt < 32i32 {
        player = g_entities
            .as_mut_ptr()
            .offset(level.sortedClients[i as usize] as isize);
        if 0 != (*player).inuse as libc::c_uint
            && (*(*player).client).sess.sessionTeam as libc::c_uint == team as libc::c_uint
        {
            let fresh1 = cnt;
            cnt = cnt + 1;
            clients[fresh1 as usize] = level.sortedClients[i as usize]
        }
        i += 1
    }
    qsort(
        clients.as_mut_ptr() as *mut libc::c_void,
        cnt as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(SortClients),
    );
    string[0usize] = 0i32 as libc::c_char;
    stringlength = 0i32;
    i = 0i32;
    cnt = 0i32;
    while i < g_maxclients.integer && cnt < 32i32 {
        player = g_entities.as_mut_ptr().offset(i as isize);
        if 0 != (*player).inuse as libc::c_uint
            && (*(*player).client).sess.sessionTeam as libc::c_uint == team as libc::c_uint
        {
            h = (*(*player).client).ps.stats[STAT_HEALTH as libc::c_int as usize];
            a = (*(*player).client).ps.stats[STAT_ARMOR as libc::c_int as usize];
            if h < 0i32 {
                h = 0i32
            }
            if a < 0i32 {
                a = 0i32
            }
            Com_sprintf(
                entry.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b" %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char,
                i,
                (*(*player).client).pers.teamState.location,
                h,
                a,
                (*(*player).client).ps.weapon,
                (*player).s.powerups,
            );
            j = strlen(entry.as_mut_ptr()) as libc::c_int;
            if (stringlength + j) as libc::c_ulong
                >= ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
            {
                break;
            }
            strcpy(
                string.as_mut_ptr().offset(stringlength as isize),
                entry.as_mut_ptr(),
            );
            stringlength += j;
            cnt += 1
        }
        i += 1
    }
    trap_SendServerCommand(
        ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
        va(
            b"tinfo %i %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cnt,
            string.as_mut_ptr(),
        ),
    );
}
/*---------------------------------------------------------------------------*/
unsafe extern "C" fn SortClients(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(a as *mut libc::c_int) - *(b as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CheckTeamStatus() {
    let mut i: libc::c_int = 0;
    let mut loc: *mut gentity_t = 0 as *mut gentity_t;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    if level.time - level.lastTeamLocationTime > 1000i32 {
        level.lastTeamLocationTime = level.time;
        i = 0i32;
        while i < g_maxclients.integer {
            ent = g_entities.as_mut_ptr().offset(i as isize);
            if !((*(*ent).client).pers.connected as libc::c_uint
                != CON_CONNECTED as libc::c_int as libc::c_uint)
            {
                if 0 != (*ent).inuse as libc::c_uint
                    && ((*(*ent).client).sess.sessionTeam as libc::c_uint
                        == TEAM_RED as libc::c_int as libc::c_uint
                        || (*(*ent).client).sess.sessionTeam as libc::c_uint
                            == TEAM_BLUE as libc::c_int as libc::c_uint)
                {
                    loc = Team_GetLocation(ent);
                    if !loc.is_null() {
                        (*(*ent).client).pers.teamState.location = (*loc).health
                    } else {
                        (*(*ent).client).pers.teamState.location = 0i32
                    }
                }
            }
            i += 1
        }
        i = 0i32;
        while i < g_maxclients.integer {
            ent = g_entities.as_mut_ptr().offset(i as isize);
            if !((*(*ent).client).pers.connected as libc::c_uint
                != CON_CONNECTED as libc::c_int as libc::c_uint)
            {
                if 0 != (*ent).inuse as u64 {
                    TeamplayInfoMessage(ent);
                }
            }
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Team(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> libc::c_int {
    let mut team: libc::c_int = 0;
    let mut cl: *mut gclient_t = (*other).client;
    if strcmp(
        (*ent).classname,
        b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        team = TEAM_RED as libc::c_int
    } else if strcmp(
        (*ent).classname,
        b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        team = TEAM_BLUE as libc::c_int
    } else {
        PrintMsg(
            other,
            b"Don\'t know what team the flag is on.\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0i32;
    }
    if team as libc::c_uint == (*cl).sess.sessionTeam as libc::c_uint {
        return Team_TouchOurFlag(ent, other, team);
    }
    return Team_TouchEnemyFlag(ent, other, team);
}
#[no_mangle]
pub unsafe extern "C" fn Team_TouchEnemyFlag(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut team: libc::c_int,
) -> libc::c_int {
    let mut cl: *mut gclient_t = (*other).client;
    PrintMsg(
        0 as *mut gentity_t,
        b"%s^7 got the %s flag!\n\x00" as *const u8 as *const libc::c_char,
        (*(*other).client).pers.netname.as_mut_ptr(),
        TeamName(team),
    );
    if team == TEAM_RED as libc::c_int {
        (*cl).ps.powerups[PW_REDFLAG as libc::c_int as usize] = 2147483647i32
    } else {
        (*cl).ps.powerups[PW_BLUEFLAG as libc::c_int as usize] = 2147483647i32
    }
    Team_SetFlagStatus(team, FLAG_TAKEN);
    (*cl).pers.teamState.flagsince = level.time as libc::c_float;
    Team_TakeFlagSound(ent, team);
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn Team_TakeFlagSound(mut ent: *mut gentity_t, mut team: libc::c_int) {
    let mut te: *mut gentity_t = 0 as *mut gentity_t;
    if ent.is_null() {
        G_Printf(
            b"Warning:  NULL passed to Team_TakeFlagSound\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    match team {
        1 => {
            if teamgame.blueStatus as libc::c_uint != FLAG_ATBASE as libc::c_int as libc::c_uint {
                if teamgame.blueTakenTime > level.time - 10000i32 {
                    return;
                }
            }
            teamgame.blueTakenTime = level.time
        }
        2 => {
            if teamgame.redStatus as libc::c_uint != FLAG_ATBASE as libc::c_int as libc::c_uint {
                if teamgame.redTakenTime > level.time - 10000i32 {
                    return;
                }
            }
            teamgame.redTakenTime = level.time
        }
        _ => {}
    }
    te = G_TempEntity(
        (*ent).s.pos.trBase.as_mut_ptr(),
        EV_GLOBAL_TEAM_SOUND as libc::c_int,
    );
    if team == TEAM_BLUE as libc::c_int {
        (*te).s.eventParm = GTS_RED_TAKEN as libc::c_int
    } else {
        (*te).s.eventParm = GTS_BLUE_TAKEN as libc::c_int
    }
    (*te).r.svFlags |= 0x20i32;
}
// Reset Flag will delete this entity
/*
==============
Team_DroppedFlagThink
==============
*/
#[no_mangle]
pub unsafe extern "C" fn Team_TouchOurFlag(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut team: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut player: *mut gentity_t = 0 as *mut gentity_t;
    let mut cl: *mut gclient_t = (*other).client;
    let mut enemy_flag: libc::c_int = 0;
    if (*cl).sess.sessionTeam as libc::c_uint == TEAM_RED as libc::c_int as libc::c_uint {
        enemy_flag = PW_BLUEFLAG as libc::c_int
    } else {
        enemy_flag = PW_REDFLAG as libc::c_int
    }
    if 0 != (*ent).flags & 0x1000i32 {
        PrintMsg(
            0 as *mut gentity_t,
            b"%s^7 returned the %s flag!\n\x00" as *const u8 as *const libc::c_char,
            (*cl).pers.netname.as_mut_ptr(),
            TeamName(team),
        );
        AddScore(other, (*ent).r.currentOrigin.as_mut_ptr(), 1i32);
        (*(*other).client).pers.teamState.flagrecovery += 1;
        (*(*other).client).pers.teamState.lastreturnedflag = level.time as libc::c_float;
        Team_ReturnFlagSound(Team_ResetFlag(team), team);
        return 0i32;
    }
    if 0 == (*cl).ps.powerups[enemy_flag as usize] {
        return 0i32;
    }
    PrintMsg(
        0 as *mut gentity_t,
        b"%s^7 captured the %s flag!\n\x00" as *const u8 as *const libc::c_char,
        (*cl).pers.netname.as_mut_ptr(),
        TeamName(OtherTeam(team)),
    );
    (*cl).ps.powerups[enemy_flag as usize] = 0i32;
    teamgame.last_flag_capture = level.time as libc::c_float;
    teamgame.last_capture_team = team;
    AddTeamScore(
        (*ent).s.pos.trBase.as_mut_ptr(),
        (*(*other).client).sess.sessionTeam as libc::c_int,
        1i32,
    );
    Team_ForceGesture((*(*other).client).sess.sessionTeam as libc::c_int);
    (*(*other).client).pers.teamState.captures += 1;
    (*(*other).client).ps.eFlags &=
        !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
    (*(*other).client).ps.eFlags |= 0x800i32;
    (*(*other).client).rewardTime = level.time + 2000i32;
    (*(*other).client).ps.persistant[PERS_CAPTURES as libc::c_int as usize] += 1;
    AddScore(other, (*ent).r.currentOrigin.as_mut_ptr(), 5i32);
    Team_CaptureFlagSound(ent, team);
    i = 0i32;
    while i < g_maxclients.integer {
        player = &mut g_entities[i as usize] as *mut gentity_t;
        // also make sure we don't award assist bonuses to the flag carrier himself.
        if !(0 == (*player).inuse as u64 || player == other) {
            if (*(*player).client).sess.sessionTeam as libc::c_uint
                != (*cl).sess.sessionTeam as libc::c_uint
            {
                (*(*player).client).pers.teamState.lasthurtcarrier = -5i32 as libc::c_float
            } else if (*(*player).client).sess.sessionTeam as libc::c_uint
                == (*cl).sess.sessionTeam as libc::c_uint
            {
                if (*(*player).client).pers.teamState.lastreturnedflag + 10000i32 as libc::c_float
                    > level.time as libc::c_float
                {
                    AddScore(player, (*ent).r.currentOrigin.as_mut_ptr(), 1i32);
                    (*(*other).client).pers.teamState.assists += 1;
                    (*(*player).client).ps.persistant[PERS_ASSIST_COUNT as libc::c_int as usize] +=
                        1;
                    (*(*player).client).ps.eFlags &=
                        !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
                    (*(*player).client).ps.eFlags |= 0x20000i32;
                    (*(*player).client).rewardTime = level.time + 2000i32
                }
                if (*(*player).client).pers.teamState.lastfraggedcarrier + 10000i32 as libc::c_float
                    > level.time as libc::c_float
                {
                    AddScore(player, (*ent).r.currentOrigin.as_mut_ptr(), 2i32);
                    (*(*other).client).pers.teamState.assists += 1;
                    (*(*player).client).ps.persistant[PERS_ASSIST_COUNT as libc::c_int as usize] +=
                        1;
                    (*(*player).client).ps.eFlags &=
                        !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
                    (*(*player).client).ps.eFlags |= 0x20000i32;
                    (*(*player).client).rewardTime = level.time + 2000i32
                }
            }
        }
        i += 1
    }
    Team_ResetFlags();
    CalculateRanks();
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Team_ResetFlags() {
    if g_gametype.integer == GT_CTF as libc::c_int {
        Team_ResetFlag(TEAM_RED as libc::c_int);
        Team_ResetFlag(TEAM_BLUE as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Team_CaptureFlagSound(mut ent: *mut gentity_t, mut team: libc::c_int) {
    let mut te: *mut gentity_t = 0 as *mut gentity_t;
    if ent.is_null() {
        G_Printf(
            b"Warning:  NULL passed to Team_CaptureFlagSound\n\x00" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    te = G_TempEntity(
        (*ent).s.pos.trBase.as_mut_ptr(),
        EV_GLOBAL_TEAM_SOUND as libc::c_int,
    );
    if team == TEAM_BLUE as libc::c_int {
        (*te).s.eventParm = GTS_BLUE_CAPTURE as libc::c_int
    } else {
        (*te).s.eventParm = GTS_RED_CAPTURE as libc::c_int
    }
    (*te).r.svFlags |= 0x20i32;
}
/*
================
Team_ForceGesture
================
*/
#[no_mangle]
pub unsafe extern "C" fn Team_ForceGesture(mut team: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    i = 0i32;
    while i < 64i32 {
        ent = &mut g_entities[i as usize] as *mut gentity_t;
        if !(0 == (*ent).inuse as u64) {
            if !(*ent).client.is_null() {
                if !((*(*ent).client).sess.sessionTeam as libc::c_uint != team as libc::c_uint) {
                    (*ent).flags |= 0x8000i32
                }
            }
        }
        i += 1
    }
}
#[no_mangle]
pub static mut neutralObelisk: *mut gentity_t = 0 as *const gentity_t as *mut gentity_t;
/*-----------------------------------------------------------------*/
/*QUAKED team_CTF_redplayer (1 0 0) (-16 -16 -16) (16 16 32)
Only in CTF games.  Red players spawn here at game start.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_team_CTF_redplayer(mut ent: *mut gentity_t) {}
/*QUAKED team_CTF_blueplayer (0 0 1) (-16 -16 -16) (16 16 32)
Only in CTF games.  Blue players spawn here at game start.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_team_CTF_blueplayer(mut ent: *mut gentity_t) {}
/*QUAKED team_CTF_redspawn (1 0 0) (-16 -16 -24) (16 16 32)
potential spawning position for red team in CTF games.
Targets will be fired when someone spawns in on them.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_team_CTF_redspawn(mut ent: *mut gentity_t) {}
/*QUAKED team_CTF_bluespawn (0 0 1) (-16 -16 -24) (16 16 32)
potential spawning position for blue team in CTF games.
Targets will be fired when someone spawns in on them.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_team_CTF_bluespawn(mut ent: *mut gentity_t) {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct teamgame_s {
    pub last_flag_capture: libc::c_float,
    pub last_capture_team: libc::c_int,
    pub redStatus: flagStatus_t,
    pub blueStatus: flagStatus_t,
    pub flagStatus: flagStatus_t,
    pub redTakenTime: libc::c_int,
    pub blueTakenTime: libc::c_int,
    pub redObeliskAttackedTime: libc::c_int,
    pub blueObeliskAttackedTime: libc::c_int,
}
pub type teamgame_t = teamgame_s;
