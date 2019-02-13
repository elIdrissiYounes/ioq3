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
    unnamed_5, BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3, BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, ET_BEAM,
    ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER,
    ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET,
    EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3,
    EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH,
    EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND, EV_GIB_PLAYER,
    EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE,
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
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, FLAG_RUN, FLAG_STAND,
    FLAG_STAND2RUN, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK,
    LEGS_IDLE, LEGS_IDLECR, LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM,
    LEGS_TURN, LEGS_WALK, LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS, PERS_ASSIST_COUNT,
    PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS,
    TEAM_RED, TEAM_SPECTATOR, TORSO_AFFIRMATIVE, TORSO_ATTACK, TORSO_ATTACK2, TORSO_DROP,
    TORSO_FOLLOWME, TORSO_GESTURE, TORSO_GETFLAG, TORSO_GUARDBASE, TORSO_NEGATIVE, TORSO_PATROL,
    TORSO_RAISE, TORSO_STAND, TORSO_STAND2, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK,
    WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN,
    WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
use g_active::{ClientEndFrame, ClientThink, G_RunClient};
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
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gentity_s, gentity_t,
    level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t, spectatorState_t,
    trap_Cvar_VariableIntegerValue, trap_LinkEntity, trap_SendConsoleCommand, CON_CONNECTED,
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
use g_variadic_h::G_Printf;
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
    unnamed, usercmd_s, usercmd_t, vec3_t, vec_t, vmCvar_t, Com_sprintf, EXEC_APPEND, EXEC_INSERT,
    EXEC_NOW, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{strcat, strlen};

//
// g_arenas.c
//
#[no_mangle]
pub unsafe extern "C" fn UpdateTournamentInfo() {
    let mut i: libc::c_int = 0;
    let mut player: *mut gentity_t = 0 as *mut gentity_t;
    let mut playerClientNum: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut accuracy: libc::c_int = 0;
    let mut perfect: libc::c_int = 0;
    let mut msglen: libc::c_int = 0;
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    player = 0 as *mut gentity_t;
    i = 0i32;
    while i < level.maxclients {
        player = &mut g_entities[i as usize] as *mut gentity_t;
        if !(0 == (*player).inuse as u64) {
            if 0 == (*player).r.svFlags & 0x8i32 {
                break;
            }
        }
        i += 1
    }
    if player.is_null() || i == level.maxclients {
        return;
    }
    playerClientNum = i;
    CalculateRanks();
    if (*level.clients.offset(playerClientNum as isize))
        .sess
        .sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        Com_sprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"postgame %i %i 0 0 0 0 0 0\x00" as *const u8 as *const libc::c_char,
            level.numNonSpectatorClients,
            playerClientNum,
        );
    } else {
        if 0 != (*(*player).client).accuracy_shots {
            accuracy =
                (*(*player).client).accuracy_hits * 100i32 / (*(*player).client).accuracy_shots
        } else {
            accuracy = 0i32
        }
        perfect = if (*level.clients.offset(playerClientNum as isize))
            .ps
            .persistant[PERS_RANK as libc::c_int as usize]
            == 0i32
            && (*(*player).client).ps.persistant[PERS_KILLED as libc::c_int as usize] == 0i32
        {
            1i32
        } else {
            0i32
        };
        Com_sprintf(
            msg.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"postgame %i %i %i %i %i %i %i %i\x00" as *const u8 as *const libc::c_char,
            level.numNonSpectatorClients,
            playerClientNum,
            accuracy,
            (*(*player).client).ps.persistant[PERS_IMPRESSIVE_COUNT as libc::c_int as usize],
            (*(*player).client).ps.persistant[PERS_EXCELLENT_COUNT as libc::c_int as usize],
            (*(*player).client).ps.persistant[PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize],
            (*(*player).client).ps.persistant[PERS_SCORE as libc::c_int as usize],
            perfect,
        );
    }
    msglen = strlen(msg.as_mut_ptr()) as libc::c_int;
    i = 0i32;
    while i < level.numNonSpectatorClients {
        n = level.sortedClients[i as usize];
        Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b" %i %i %i\x00" as *const u8 as *const libc::c_char,
            n,
            (*level.clients.offset(n as isize)).ps.persistant[PERS_RANK as libc::c_int as usize],
            (*level.clients.offset(n as isize)).ps.persistant[PERS_SCORE as libc::c_int as usize],
        );
        msglen = (msglen as libc::c_ulong).wrapping_add(strlen(buf.as_mut_ptr())) as libc::c_int
            as libc::c_int;
        if msglen as libc::c_ulong >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        {
            break;
        }
        strcat(msg.as_mut_ptr(), buf.as_mut_ptr());
        i += 1
    }
    trap_SendConsoleCommand(EXEC_APPEND as libc::c_int, msg.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SpawnModelsOnVictoryPads() {
    let mut player: *mut gentity_t = 0 as *mut gentity_t;
    let mut podium: *mut gentity_t = 0 as *mut gentity_t;
    podium1 = 0 as *mut gentity_t;
    podium2 = 0 as *mut gentity_t;
    podium3 = 0 as *mut gentity_t;
    podium = SpawnPodium();
    player = SpawnModelOnVictoryPad(
        podium,
        offsetFirst.as_mut_ptr(),
        &mut g_entities[level.sortedClients[0usize] as usize],
        (*level.clients.offset(level.sortedClients[0usize] as isize))
            .ps
            .persistant[PERS_RANK as libc::c_int as usize]
            & !0x4000i32,
    );
    if !player.is_null() {
        (*player).nextthink = level.time + 2000i32;
        (*player).think = Some(CelebrateStart);
        podium1 = player
    }
    player = SpawnModelOnVictoryPad(
        podium,
        offsetSecond.as_mut_ptr(),
        &mut g_entities[level.sortedClients[1usize] as usize],
        (*level.clients.offset(level.sortedClients[1usize] as isize))
            .ps
            .persistant[PERS_RANK as libc::c_int as usize]
            & !0x4000i32,
    );
    if !player.is_null() {
        podium2 = player
    }
    if level.numNonSpectatorClients > 2i32 {
        player = SpawnModelOnVictoryPad(
            podium,
            offsetThird.as_mut_ptr(),
            &mut g_entities[level.sortedClients[2usize] as usize],
            (*level.clients.offset(level.sortedClients[2usize] as isize))
                .ps
                .persistant[PERS_RANK as libc::c_int as usize]
                & !0x4000i32,
        );
        if !player.is_null() {
            podium3 = player
        }
    };
}
#[no_mangle]
pub static mut podium3: *mut gentity_t = 0 as *const gentity_t as *mut gentity_t;
static mut offsetThird: vec3_t = [-19i32 as vec_t, -60i32 as vec_t, 45i32 as vec_t];
unsafe extern "C" fn SpawnModelOnVictoryPad(
    mut pad: *mut gentity_t,
    mut offset: *mut vec_t,
    mut ent: *mut gentity_t,
    mut place: libc::c_int,
) -> *mut gentity_t {
    let mut body: *mut gentity_t = 0 as *mut gentity_t;
    let mut vec: vec3_t = [0.; 3];
    let mut f: vec3_t = [0.; 3];
    let mut r: vec3_t = [0.; 3];
    let mut u: vec3_t = [0.; 3];
    body = G_Spawn();
    if body.is_null() {
        G_Printf(b"^1ERROR: out of gentities\n\x00" as *const u8 as *const libc::c_char);
        return 0 as *mut gentity_t;
    }
    (*body).classname = (*(*ent).client).pers.netname.as_mut_ptr();
    (*body).client = (*ent).client;
    (*body).s = (*ent).s;
    (*body).s.eType = ET_PLAYER as libc::c_int;
    (*body).s.eFlags = 0i32;
    (*body).s.powerups = 0i32;
    (*body).s.loopSound = 0i32;
    (*body).s.number =
        body.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int;
    (*body).timestamp = level.time;
    (*body).physicsObject = qtrue;
    (*body).physicsBounce = 0i32 as libc::c_float;
    (*body).s.event = 0i32;
    (*body).s.pos.trType = TR_STATIONARY;
    (*body).s.groundEntityNum = (1i32 << 10i32) - 2i32;
    (*body).s.legsAnim = LEGS_IDLE as libc::c_int;
    (*body).s.torsoAnim = TORSO_STAND as libc::c_int;
    if (*body).s.weapon == WP_NONE as libc::c_int {
        (*body).s.weapon = WP_MACHINEGUN as libc::c_int
    }
    if (*body).s.weapon == WP_GAUNTLET as libc::c_int {
        (*body).s.torsoAnim = TORSO_STAND2 as libc::c_int
    }
    (*body).s.event = 0i32;
    (*body).r.svFlags = (*ent).r.svFlags;
    (*body).r.mins[0usize] = (*ent).r.mins[0usize];
    (*body).r.mins[1usize] = (*ent).r.mins[1usize];
    (*body).r.mins[2usize] = (*ent).r.mins[2usize];
    (*body).r.maxs[0usize] = (*ent).r.maxs[0usize];
    (*body).r.maxs[1usize] = (*ent).r.maxs[1usize];
    (*body).r.maxs[2usize] = (*ent).r.maxs[2usize];
    (*body).r.absmin[0usize] = (*ent).r.absmin[0usize];
    (*body).r.absmin[1usize] = (*ent).r.absmin[1usize];
    (*body).r.absmin[2usize] = (*ent).r.absmin[2usize];
    (*body).r.absmax[0usize] = (*ent).r.absmax[0usize];
    (*body).r.absmax[1usize] = (*ent).r.absmax[1usize];
    (*body).r.absmax[2usize] = (*ent).r.absmax[2usize];
    (*body).clipmask = 1i32 | 0x10000i32;
    (*body).r.contents = 0x2000000i32;
    (*body).r.ownerNum = (*ent).r.ownerNum;
    (*body).takedamage = qfalse;
    vec[0usize] = level.intermission_origin[0usize] - (*pad).r.currentOrigin[0usize];
    vec[1usize] = level.intermission_origin[1usize] - (*pad).r.currentOrigin[1usize];
    vec[2usize] = level.intermission_origin[2usize] - (*pad).r.currentOrigin[2usize];
    vectoangles(
        vec.as_mut_ptr() as *const vec_t,
        (*body).s.apos.trBase.as_mut_ptr(),
    );
    (*body).s.apos.trBase[0usize] = 0i32 as vec_t;
    (*body).s.apos.trBase[2usize] = 0i32 as vec_t;
    AngleVectors(
        (*body).s.apos.trBase.as_mut_ptr() as *const vec_t,
        f.as_mut_ptr(),
        r.as_mut_ptr(),
        u.as_mut_ptr(),
    );
    vec[0usize] = (*pad).r.currentOrigin[0usize] + f[0usize] * *offset.offset(0isize);
    vec[1usize] = (*pad).r.currentOrigin[1usize] + f[1usize] * *offset.offset(0isize);
    vec[2usize] = (*pad).r.currentOrigin[2usize] + f[2usize] * *offset.offset(0isize);
    vec[0usize] = vec[0usize] + r[0usize] * *offset.offset(1isize);
    vec[1usize] = vec[1usize] + r[1usize] * *offset.offset(1isize);
    vec[2usize] = vec[2usize] + r[2usize] * *offset.offset(1isize);
    vec[0usize] = vec[0usize] + u[0usize] * *offset.offset(2isize);
    vec[1usize] = vec[1usize] + u[1usize] * *offset.offset(2isize);
    vec[2usize] = vec[2usize] + u[2usize] * *offset.offset(2isize);
    G_SetOrigin(body, vec.as_mut_ptr());
    trap_LinkEntity(body);
    (*body).count = place;
    return body;
}
#[no_mangle]
pub static mut podium2: *mut gentity_t = 0 as *const gentity_t as *mut gentity_t;
static mut offsetSecond: vec3_t = [-10i32 as vec_t, 60i32 as vec_t, 54i32 as vec_t];
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
//
// g_arenas.c
//
#[no_mangle]
pub static mut podium1: *mut gentity_t = 0 as *const gentity_t as *mut gentity_t;
unsafe extern "C" fn CelebrateStart(mut player: *mut gentity_t) {
    (*player).s.torsoAnim = (*player).s.torsoAnim & 128i32 ^ 128i32 | TORSO_GESTURE as libc::c_int;
    (*player).nextthink = level.time + (34i32 * 66i32 + 50i32);
    (*player).think = Some(CelebrateStop);
    G_AddEvent(player, EV_TAUNT as libc::c_int, 0i32);
}
unsafe extern "C" fn CelebrateStop(mut player: *mut gentity_t) {
    let mut anim: libc::c_int = 0;
    if (*player).s.weapon == WP_GAUNTLET as libc::c_int {
        anim = TORSO_STAND2 as libc::c_int
    } else {
        anim = TORSO_STAND as libc::c_int
    }
    (*player).s.torsoAnim = (*player).s.torsoAnim & 128i32 ^ 128i32 | anim;
}
static mut offsetFirst: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 74i32 as vec_t];
unsafe extern "C" fn SpawnPodium() -> *mut gentity_t {
    let mut podium: *mut gentity_t = 0 as *mut gentity_t;
    let mut vec: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    podium = G_Spawn();
    if podium.is_null() {
        return 0 as *mut gentity_t;
    }
    (*podium).classname = b"podium\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*podium).s.eType = ET_GENERAL as libc::c_int;
    (*podium).s.number =
        podium.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int;
    (*podium).clipmask = 1i32;
    (*podium).r.contents = 1i32;
    (*podium).s.modelindex = G_ModelIndex(
        b"models/mapobjects/podium/podium4.md3\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    AngleVectors(
        level.intermission_angle.as_mut_ptr() as *const vec_t,
        vec.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    origin[0usize] = level.intermission_origin[0usize]
        + vec[0usize]
            * trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[1usize] = level.intermission_origin[1usize]
        + vec[1usize]
            * trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2usize] = level.intermission_origin[2usize]
        + vec[2usize]
            * trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2usize] -=
        trap_Cvar_VariableIntegerValue(b"g_podiumDrop\x00" as *const u8 as *const libc::c_char)
            as libc::c_float;
    G_SetOrigin(podium, origin.as_mut_ptr());
    vec[0usize] = level.intermission_origin[0usize] - (*podium).r.currentOrigin[0usize];
    vec[1usize] = level.intermission_origin[1usize] - (*podium).r.currentOrigin[1usize];
    vec[2usize] = level.intermission_origin[2usize] - (*podium).r.currentOrigin[2usize];
    (*podium).s.apos.trBase[1usize] = vectoyaw(vec.as_mut_ptr() as *const vec_t);
    trap_LinkEntity(podium);
    (*podium).think = Some(PodiumPlacementThink);
    (*podium).nextthink = level.time + 100i32;
    return podium;
}
unsafe extern "C" fn PodiumPlacementThink(mut podium: *mut gentity_t) {
    let mut vec: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut f: vec3_t = [0.; 3];
    let mut r: vec3_t = [0.; 3];
    let mut u: vec3_t = [0.; 3];
    (*podium).nextthink = level.time + 100i32;
    AngleVectors(
        level.intermission_angle.as_mut_ptr() as *const vec_t,
        vec.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    origin[0usize] = level.intermission_origin[0usize]
        + vec[0usize]
            * trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[1usize] = level.intermission_origin[1usize]
        + vec[1usize]
            * trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2usize] = level.intermission_origin[2usize]
        + vec[2usize]
            * trap_Cvar_VariableIntegerValue(
                b"g_podiumDist\x00" as *const u8 as *const libc::c_char,
            ) as libc::c_float;
    origin[2usize] -=
        trap_Cvar_VariableIntegerValue(b"g_podiumDrop\x00" as *const u8 as *const libc::c_char)
            as libc::c_float;
    G_SetOrigin(podium, origin.as_mut_ptr());
    if !podium1.is_null() {
        vec[0usize] = level.intermission_origin[0usize] - (*podium).r.currentOrigin[0usize];
        vec[1usize] = level.intermission_origin[1usize] - (*podium).r.currentOrigin[1usize];
        vec[2usize] = level.intermission_origin[2usize] - (*podium).r.currentOrigin[2usize];
        vectoangles(
            vec.as_mut_ptr() as *const vec_t,
            (*podium1).s.apos.trBase.as_mut_ptr(),
        );
        (*podium1).s.apos.trBase[0usize] = 0i32 as vec_t;
        (*podium1).s.apos.trBase[2usize] = 0i32 as vec_t;
        AngleVectors(
            (*podium1).s.apos.trBase.as_mut_ptr() as *const vec_t,
            f.as_mut_ptr(),
            r.as_mut_ptr(),
            u.as_mut_ptr(),
        );
        vec[0usize] = (*podium).r.currentOrigin[0usize] + f[0usize] * offsetFirst[0usize];
        vec[1usize] = (*podium).r.currentOrigin[1usize] + f[1usize] * offsetFirst[0usize];
        vec[2usize] = (*podium).r.currentOrigin[2usize] + f[2usize] * offsetFirst[0usize];
        vec[0usize] = vec[0usize] + r[0usize] * offsetFirst[1usize];
        vec[1usize] = vec[1usize] + r[1usize] * offsetFirst[1usize];
        vec[2usize] = vec[2usize] + r[2usize] * offsetFirst[1usize];
        vec[0usize] = vec[0usize] + u[0usize] * offsetFirst[2usize];
        vec[1usize] = vec[1usize] + u[1usize] * offsetFirst[2usize];
        vec[2usize] = vec[2usize] + u[2usize] * offsetFirst[2usize];
        G_SetOrigin(podium1, vec.as_mut_ptr());
    }
    if !podium2.is_null() {
        vec[0usize] = level.intermission_origin[0usize] - (*podium).r.currentOrigin[0usize];
        vec[1usize] = level.intermission_origin[1usize] - (*podium).r.currentOrigin[1usize];
        vec[2usize] = level.intermission_origin[2usize] - (*podium).r.currentOrigin[2usize];
        vectoangles(
            vec.as_mut_ptr() as *const vec_t,
            (*podium2).s.apos.trBase.as_mut_ptr(),
        );
        (*podium2).s.apos.trBase[0usize] = 0i32 as vec_t;
        (*podium2).s.apos.trBase[2usize] = 0i32 as vec_t;
        AngleVectors(
            (*podium2).s.apos.trBase.as_mut_ptr() as *const vec_t,
            f.as_mut_ptr(),
            r.as_mut_ptr(),
            u.as_mut_ptr(),
        );
        vec[0usize] = (*podium).r.currentOrigin[0usize] + f[0usize] * offsetSecond[0usize];
        vec[1usize] = (*podium).r.currentOrigin[1usize] + f[1usize] * offsetSecond[0usize];
        vec[2usize] = (*podium).r.currentOrigin[2usize] + f[2usize] * offsetSecond[0usize];
        vec[0usize] = vec[0usize] + r[0usize] * offsetSecond[1usize];
        vec[1usize] = vec[1usize] + r[1usize] * offsetSecond[1usize];
        vec[2usize] = vec[2usize] + r[2usize] * offsetSecond[1usize];
        vec[0usize] = vec[0usize] + u[0usize] * offsetSecond[2usize];
        vec[1usize] = vec[1usize] + u[1usize] * offsetSecond[2usize];
        vec[2usize] = vec[2usize] + u[2usize] * offsetSecond[2usize];
        G_SetOrigin(podium2, vec.as_mut_ptr());
    }
    if !podium3.is_null() {
        vec[0usize] = level.intermission_origin[0usize] - (*podium).r.currentOrigin[0usize];
        vec[1usize] = level.intermission_origin[1usize] - (*podium).r.currentOrigin[1usize];
        vec[2usize] = level.intermission_origin[2usize] - (*podium).r.currentOrigin[2usize];
        vectoangles(
            vec.as_mut_ptr() as *const vec_t,
            (*podium3).s.apos.trBase.as_mut_ptr(),
        );
        (*podium3).s.apos.trBase[0usize] = 0i32 as vec_t;
        (*podium3).s.apos.trBase[2usize] = 0i32 as vec_t;
        AngleVectors(
            (*podium3).s.apos.trBase.as_mut_ptr() as *const vec_t,
            f.as_mut_ptr(),
            r.as_mut_ptr(),
            u.as_mut_ptr(),
        );
        vec[0usize] = (*podium).r.currentOrigin[0usize] + f[0usize] * offsetThird[0usize];
        vec[1usize] = (*podium).r.currentOrigin[1usize] + f[1usize] * offsetThird[0usize];
        vec[2usize] = (*podium).r.currentOrigin[2usize] + f[2usize] * offsetThird[0usize];
        vec[0usize] = vec[0usize] + r[0usize] * offsetThird[1usize];
        vec[1usize] = vec[1usize] + r[1usize] * offsetThird[1usize];
        vec[2usize] = vec[2usize] + r[2usize] * offsetThird[1usize];
        vec[0usize] = vec[0usize] + u[0usize] * offsetThird[2usize];
        vec[1usize] = vec[1usize] + u[1usize] * offsetThird[2usize];
        vec[2usize] = vec[2usize] + u[2usize] * offsetThird[2usize];
        G_SetOrigin(podium3, vec.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn Svcmd_AbortPodium_f() {
    if g_gametype.integer != GT_SINGLE_PLAYER as libc::c_int {
        return;
    }
    if !podium1.is_null() {
        (*podium1).nextthink = level.time;
        (*podium1).think = Some(CelebrateStop)
    };
}
