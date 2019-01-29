use ai_main::{
    bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown, BotAIShutdownClient,
    BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, team_t, unnamed_0, unnamed_1, unnamed_2, unnamed_3, unnamed_4,
    unnamed_5, unnamed_6, BG_PlayerStateToEntityState, BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3,
    BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL,
    EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM,
    EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE,
    EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND,
    EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP, EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED,
    EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT, EV_MISSILE_HIT, EV_MISSILE_MISS,
    EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE, EV_OBELISKPAIN, EV_OBITUARY,
    EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT, EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD,
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
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, STAT_ARMOR, STAT_CLIENTS_READY,
    STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE,
    TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, TORSO_AFFIRMATIVE, TORSO_ATTACK,
    TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE, TORSO_GETFLAG, TORSO_GUARDBASE,
    TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND, TORSO_STAND2, WEAPON_DROPPING,
    WEAPON_FIRING, WEAPON_RAISING, WEAPON_READY, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK,
    WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN,
    WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
};
use g_active::{ClientEndFrame, ClientThink, G_RunClient};
use g_arenas::{
    podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f, UpdateTournamentInfo,
};
use g_bot::{
    G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
    Svcmd_BotList_f,
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
    spectatorState_t, trap_DropClient, trap_EntitiesInBox, trap_GetUsercmd, trap_GetUserinfo,
    trap_LinkEntity, trap_PointContents, trap_SendConsoleCommand, trap_SendServerCommand,
    trap_SetConfigstring, trap_UnlinkEntity, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED,
    MOVER_1TO2, MOVER_2TO1, MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW, SPECTATOR_FREE,
    SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
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
    SP_team_CTF_redspawn, SelectCTFSpawnPoint, Team_CheckDroppedItem,
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
use g_variadic_h::{G_Error, G_LogPrintf};
use g_weapon::{
    CheckGauntletAttack, FireWeapon, LogAccuracyHit, SnapVectorTowards, Weapon_HookFree,
    Weapon_HookThink,
};
use libc;
use q_shared_h::{
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    unnamed, usercmd_s, usercmd_t, va, vec3_origin, vec3_t, vec_t, vmCvar_t, Info_Validate,
    Info_ValueForKey, Q_IsColorString, Q_stricmp, Q_strncpyz, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
    TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stddef_h::size_t;
use stdlib::{atoi, memset, rand, sqrt, strcmp, strcpy};

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
//
// g_client.c
//
#[no_mangle]
pub unsafe extern "C" fn TeamCount(
    mut ignoreClientNum: libc::c_int,
    mut team: team_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0i32;
    i = 0i32;
    while i < level.maxclients {
        if !(i == ignoreClientNum) {
            if !((*level.clients.offset(i as isize)).pers.connected as libc::c_uint
                == CON_DISCONNECTED as libc::c_int as libc::c_uint)
            {
                if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                    == team as libc::c_uint
                {
                    count += 1
                }
            }
        }
        i += 1
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn TeamLeader(mut team: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < level.maxclients {
        if !((*level.clients.offset(i as isize)).pers.connected as libc::c_uint
            == CON_DISCONNECTED as libc::c_int as libc::c_uint)
        {
            if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                == team as libc::c_uint
            {
                if 0 != (*level.clients.offset(i as isize)).sess.teamLeader as u64 {
                    return i;
                }
            }
        }
        i += 1
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn PickTeam(mut ignoreClientNum: libc::c_int) -> team_t {
    let mut counts: [libc::c_int; 4] = [0; 4];
    counts[TEAM_BLUE as libc::c_int as usize] = TeamCount(ignoreClientNum, TEAM_BLUE);
    counts[TEAM_RED as libc::c_int as usize] = TeamCount(ignoreClientNum, TEAM_RED);
    if counts[TEAM_BLUE as libc::c_int as usize] > counts[TEAM_RED as libc::c_int as usize] {
        return TEAM_RED;
    }
    if counts[TEAM_RED as libc::c_int as usize] > counts[TEAM_BLUE as libc::c_int as usize] {
        return TEAM_BLUE;
    }
    if level.teamScores[TEAM_BLUE as libc::c_int as usize]
        > level.teamScores[TEAM_RED as libc::c_int as usize]
    {
        return TEAM_RED;
    }
    return TEAM_BLUE;
}
#[no_mangle]
pub unsafe extern "C" fn SetClientViewAngle(mut ent: *mut gentity_t, mut angle: *mut vec_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        let mut cmdAngle: libc::c_int = 0;
        cmdAngle = (*angle.offset(i as isize) * 65536i32 as libc::c_float / 360i32 as libc::c_float)
            as libc::c_int
            & 65535i32;
        (*(*ent).client).ps.delta_angles[i as usize] =
            cmdAngle - (*(*ent).client).pers.cmd.angles[i as usize];
        i += 1
    }
    (*ent).s.angles[0usize] = *angle.offset(0isize);
    (*ent).s.angles[1usize] = *angle.offset(1isize);
    (*ent).s.angles[2usize] = *angle.offset(2isize);
    (*(*ent).client).ps.viewangles[0usize] = (*ent).s.angles[0usize];
    (*(*ent).client).ps.viewangles[1usize] = (*ent).s.angles[1usize];
    (*(*ent).client).ps.viewangles[2usize] = (*ent).s.angles[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn SelectSpawnPoint(
    mut avoidPoint: *mut vec_t,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
    mut isbot: qboolean,
) -> *mut gentity_t {
    return SelectRandomFurthestSpawnPoint(avoidPoint, origin, angles, isbot);
}
/*
===========
SelectRandomFurthestSpawnPoint

Chooses a player start, deathmatch start, etc
============
*/
#[no_mangle]
pub unsafe extern "C" fn SelectRandomFurthestSpawnPoint(
    mut avoidPoint: *mut vec_t,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
    mut isbot: qboolean,
) -> *mut gentity_t {
    let mut spot: *mut gentity_t = 0 as *mut gentity_t;
    let mut delta: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut list_dist: [libc::c_float; 128] = [0.; 128];
    let mut list_spot: [*mut gentity_t; 128] = [0 as *mut gentity_t; 128];
    let mut numSpots: libc::c_int = 0;
    let mut rnd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    numSpots = 0i32;
    spot = 0 as *mut gentity_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char,
        );
        if spot.is_null() {
            break;
        }
        if 0 != SpotWouldTelefrag(spot) as u64 {
            continue;
        }
        if !(0 != (*spot).flags & 0x2000i32 && 0 != isbot as libc::c_uint
            || 0 != (*spot).flags & 0x4000i32 && 0 == isbot as u64)
        {
            // spot is not for this human/bot player
            delta[0usize] = (*spot).s.origin[0usize] - *avoidPoint.offset(0isize);
            delta[1usize] = (*spot).s.origin[1usize] - *avoidPoint.offset(1isize);
            delta[2usize] = (*spot).s.origin[2usize] - *avoidPoint.offset(2isize);
            dist = VectorLength(delta.as_mut_ptr() as *const vec_t);
            i = 0i32;
            while i < numSpots {
                if dist > list_dist[i as usize] {
                    if numSpots >= 128i32 {
                        numSpots = 128i32 - 1i32
                    }
                    j = numSpots;
                    while j > i {
                        list_dist[j as usize] = list_dist[(j - 1i32) as usize];
                        list_spot[j as usize] = list_spot[(j - 1i32) as usize];
                        j -= 1
                    }
                    list_dist[i as usize] = dist;
                    list_spot[i as usize] = spot;
                    numSpots += 1;
                    break;
                } else {
                    i += 1
                }
            }
            if i >= numSpots && numSpots < 128i32 {
                list_dist[numSpots as usize] = dist;
                list_spot[numSpots as usize] = spot;
                numSpots += 1
            }
        }
    }
    if 0 == numSpots {
        spot = G_Find(
            0 as *mut gentity_t,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char,
        );
        if spot.is_null() {
            G_Error(b"Couldn\'t find a spawn point\x00" as *const u8 as *const libc::c_char);
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
    rnd = ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
        * (numSpots / 2i32) as libc::c_float) as libc::c_int;
    *origin.offset(0isize) = (*list_spot[rnd as usize]).s.origin[0usize];
    *origin.offset(1isize) = (*list_spot[rnd as usize]).s.origin[1usize];
    *origin.offset(2isize) = (*list_spot[rnd as usize]).s.origin[2usize];
    let ref mut fresh1 = *origin.offset(2isize);
    *fresh1 += 9i32 as libc::c_float;
    *angles.offset(0isize) = (*list_spot[rnd as usize]).s.angles[0usize];
    *angles.offset(1isize) = (*list_spot[rnd as usize]).s.angles[1usize];
    *angles.offset(2isize) = (*list_spot[rnd as usize]).s.angles[2usize];
    return list_spot[rnd as usize];
}
#[no_mangle]
pub unsafe extern "C" fn SpotWouldTelefrag(mut spot: *mut gentity_t) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut touch: [libc::c_int; 1024] = [0; 1024];
    let mut hit: *mut gentity_t = 0 as *mut gentity_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    mins[0usize] = (*spot).s.origin[0usize] + playerMins[0usize];
    mins[1usize] = (*spot).s.origin[1usize] + playerMins[1usize];
    mins[2usize] = (*spot).s.origin[2usize] + playerMins[2usize];
    maxs[0usize] = (*spot).s.origin[0usize] + playerMaxs[0usize];
    maxs[1usize] = (*spot).s.origin[1usize] + playerMaxs[1usize];
    maxs[2usize] = (*spot).s.origin[2usize] + playerMaxs[2usize];
    num = trap_EntitiesInBox(
        mins.as_mut_ptr() as *const vec_t,
        maxs.as_mut_ptr() as *const vec_t,
        touch.as_mut_ptr(),
        1i32 << 10i32,
    );
    i = 0i32;
    while i < num {
        hit = &mut g_entities[touch[i as usize] as usize] as *mut gentity_t;
        if !(*hit).client.is_null() {
            return qtrue;
        }
        i += 1
    }
    return qfalse;
}
static mut playerMaxs: vec3_t = [15i32 as vec_t, 15i32 as vec_t, 32i32 as vec_t];
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
// g_client.c -- client functions that don't happen every frame
static mut playerMins: vec3_t = [-15i32 as vec_t, -15i32 as vec_t, -24i32 as vec_t];
#[no_mangle]
pub unsafe extern "C" fn CopyToBodyQue(mut ent: *mut gentity_t) {
    let mut body: *mut gentity_t = 0 as *mut gentity_t;
    let mut contents: libc::c_int = 0;
    trap_UnlinkEntity(ent);
    contents = trap_PointContents((*ent).s.origin.as_mut_ptr() as *const vec_t, -1i32);
    if 0 != contents as libc::c_uint & 0x80000000u32 {
        return;
    }
    body = level.bodyQue[level.bodyQueIndex as usize];
    level.bodyQueIndex = (level.bodyQueIndex + 1i32) % 8i32;
    (*body).s = (*ent).s;
    (*body).s.eFlags = 0x1i32;
    (*body).s.powerups = 0i32;
    (*body).s.loopSound = 0i32;
    (*body).s.number =
        body.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int;
    (*body).timestamp = level.time;
    (*body).physicsObject = qtrue;
    (*body).physicsBounce = 0i32 as libc::c_float;
    if (*body).s.groundEntityNum == (1i32 << 10i32) - 1i32 {
        (*body).s.pos.trType = TR_GRAVITY;
        (*body).s.pos.trTime = level.time;
        (*body).s.pos.trDelta[0usize] = (*(*ent).client).ps.velocity[0usize];
        (*body).s.pos.trDelta[1usize] = (*(*ent).client).ps.velocity[1usize];
        (*body).s.pos.trDelta[2usize] = (*(*ent).client).ps.velocity[2usize]
    } else {
        (*body).s.pos.trType = TR_STATIONARY
    }
    (*body).s.event = 0i32;
    match (*body).s.legsAnim & !128i32 {
        0 | 1 => {
            (*body).s.legsAnim = BOTH_DEAD1 as libc::c_int;
            (*body).s.torsoAnim = (*body).s.legsAnim
        }
        2 | 3 => {
            (*body).s.legsAnim = BOTH_DEAD2 as libc::c_int;
            (*body).s.torsoAnim = (*body).s.legsAnim
        }
        4 | 5 | _ => {
            (*body).s.legsAnim = BOTH_DEAD3 as libc::c_int;
            (*body).s.torsoAnim = (*body).s.legsAnim
        }
    }
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
    (*body).r.contents = 0x4000000i32;
    (*body).r.ownerNum = (*ent).s.number;
    (*body).nextthink = level.time + 5000i32;
    (*body).think = Some(BodySink);
    (*body).die = Some(body_die);
    if (*ent).health <= -40i32 {
        (*body).takedamage = qfalse
    } else {
        (*body).takedamage = qtrue
    }
    (*body).r.currentOrigin[0usize] = (*body).s.pos.trBase[0usize];
    (*body).r.currentOrigin[1usize] = (*body).s.pos.trBase[1usize];
    (*body).r.currentOrigin[2usize] = (*body).s.pos.trBase[2usize];
    trap_LinkEntity(body);
}
/*
=============
BodySink

After sitting around for five seconds, fall into the ground and disappear
=============
*/
#[no_mangle]
pub unsafe extern "C" fn BodySink(mut ent: *mut gentity_t) {
    if level.time - (*ent).timestamp > 6500i32 {
        trap_UnlinkEntity(ent);
        (*ent).physicsObject = qfalse;
        return;
    }
    (*ent).nextthink = level.time + 100i32;
    (*ent).s.pos.trBase[2usize] -= 1i32 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn ClientRespawn(mut ent: *mut gentity_t) {
    CopyToBodyQue(ent);
    ClientSpawn(ent);
}
#[no_mangle]
pub unsafe extern "C" fn ClientSpawn(mut ent: *mut gentity_t) {
    let mut index: libc::c_int = 0;
    let mut spawn_origin: vec3_t = [0.; 3];
    let mut spawn_angles: vec3_t = [0.; 3];
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: libc::c_int = 0;
    let mut saved: clientPersistant_t = clientPersistant_t {
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
    };
    let mut savedSess: clientSession_t = clientSession_t {
        sessionTeam: TEAM_FREE,
        spectatorNum: 0,
        spectatorState: SPECTATOR_NOT,
        spectatorClient: 0,
        wins: 0,
        losses: 0,
        teamLeader: qfalse,
    };
    let mut persistant: [libc::c_int; 16] = [0; 16];
    let mut spawnPoint: *mut gentity_t = 0 as *mut gentity_t;
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut flags: libc::c_int = 0;
    let mut savedPing: libc::c_int = 0;
    //	char	*savedAreaBits;
    let mut accuracy_hits: libc::c_int = 0;
    let mut accuracy_shots: libc::c_int = 0;
    let mut eventSequence: libc::c_int = 0;
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    index = ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int;
    client = (*ent).client;
    spawn_origin[2usize] = 0i32 as vec_t;
    spawn_origin[1usize] = spawn_origin[2usize];
    spawn_origin[0usize] = spawn_origin[1usize];
    if (*client).sess.sessionTeam as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint {
        spawnPoint = SelectSpectatorSpawnPoint(spawn_origin.as_mut_ptr(), spawn_angles.as_mut_ptr())
    } else if g_gametype.integer >= GT_CTF as libc::c_int {
        spawnPoint = SelectCTFSpawnPoint(
            (*client).sess.sessionTeam,
            (*client).pers.teamState.state as libc::c_int,
            spawn_origin.as_mut_ptr(),
            spawn_angles.as_mut_ptr(),
            (0 != (*ent).r.svFlags & 0x8i32) as libc::c_int as qboolean,
        )
    } else if 0 == (*client).pers.initialSpawn as u64
        && 0 != (*client).pers.localClient as libc::c_uint
    {
        (*client).pers.initialSpawn = qtrue;
        spawnPoint = SelectInitialSpawnPoint(
            spawn_origin.as_mut_ptr(),
            spawn_angles.as_mut_ptr(),
            (0 != (*ent).r.svFlags & 0x8i32) as libc::c_int as qboolean,
        )
    } else {
        spawnPoint = SelectSpawnPoint(
            (*client).ps.origin.as_mut_ptr(),
            spawn_origin.as_mut_ptr(),
            spawn_angles.as_mut_ptr(),
            (0 != (*ent).r.svFlags & 0x8i32) as libc::c_int as qboolean,
        )
    }
    (*client).pers.teamState.state = TEAM_ACTIVE;
    (*ent).s.eFlags &= !0x200i32;
    flags = (*(*ent).client).ps.eFlags & (0x4i32 | 0x4000i32 | 0x80000i32);
    flags ^= 0x4i32;
    saved = (*client).pers;
    savedSess = (*client).sess;
    savedPing = (*client).ps.ping;
    accuracy_hits = (*client).accuracy_hits;
    accuracy_shots = (*client).accuracy_shots;
    i = 0i32;
    while i < 16i32 {
        persistant[i as usize] = (*client).ps.persistant[i as usize];
        i += 1
    }
    eventSequence = (*client).ps.eventSequence;
    memset(
        client as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<gclient_t>() as libc::c_ulong,
    );
    (*client).pers = saved;
    (*client).sess = savedSess;
    (*client).ps.ping = savedPing;
    (*client).accuracy_hits = accuracy_hits;
    (*client).accuracy_shots = accuracy_shots;
    (*client).lastkilled_client = -1i32;
    i = 0i32;
    while i < 16i32 {
        (*client).ps.persistant[i as usize] = persistant[i as usize];
        i += 1
    }
    (*client).ps.eventSequence = eventSequence;
    (*client).ps.persistant[PERS_SPAWN_COUNT as libc::c_int as usize] += 1;
    (*client).ps.persistant[PERS_TEAM as libc::c_int as usize] =
        (*client).sess.sessionTeam as libc::c_int;
    (*client).airOutTime = level.time + 12000i32;
    trap_GetUserinfo(
        index,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    (*client).pers.maxHealth = atoi(Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"handicap\x00" as *const u8 as *const libc::c_char,
    ));
    if (*client).pers.maxHealth < 1i32 || (*client).pers.maxHealth > 100i32 {
        (*client).pers.maxHealth = 100i32
    }
    (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] = (*client).pers.maxHealth;
    (*client).ps.eFlags = flags;
    (*ent).s.groundEntityNum = (1i32 << 10i32) - 1i32;
    (*ent).client = &mut *level.clients.offset(index as isize) as *mut gclient_s;
    (*ent).takedamage = qtrue;
    (*ent).inuse = qtrue;
    (*ent).classname = b"player\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*ent).r.contents = 0x2000000i32;
    (*ent).clipmask = 1i32 | 0x10000i32 | 0x2000000i32;
    (*ent).die = Some(player_die);
    (*ent).waterlevel = 0i32;
    (*ent).watertype = 0i32;
    (*ent).flags = 0i32;
    (*ent).r.mins[0usize] = playerMins[0usize];
    (*ent).r.mins[1usize] = playerMins[1usize];
    (*ent).r.mins[2usize] = playerMins[2usize];
    (*ent).r.maxs[0usize] = playerMaxs[0usize];
    (*ent).r.maxs[1usize] = playerMaxs[1usize];
    (*ent).r.maxs[2usize] = playerMaxs[2usize];
    (*client).ps.clientNum = index;
    (*client).ps.stats[STAT_WEAPONS as libc::c_int as usize] = 1i32 << WP_MACHINEGUN as libc::c_int;
    if g_gametype.integer == GT_TEAM as libc::c_int {
        (*client).ps.ammo[WP_MACHINEGUN as libc::c_int as usize] = 50i32
    } else {
        (*client).ps.ammo[WP_MACHINEGUN as libc::c_int as usize] = 100i32
    }
    (*client).ps.stats[STAT_WEAPONS as libc::c_int as usize] |= 1i32 << WP_GAUNTLET as libc::c_int;
    (*client).ps.ammo[WP_GAUNTLET as libc::c_int as usize] = -1i32;
    (*client).ps.ammo[WP_GRAPPLING_HOOK as libc::c_int as usize] = -1i32;
    (*client).ps.stats[STAT_HEALTH as libc::c_int as usize] =
        (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] + 25i32;
    (*ent).health = (*client).ps.stats[STAT_HEALTH as libc::c_int as usize];
    G_SetOrigin(ent, spawn_origin.as_mut_ptr());
    (*client).ps.origin[0usize] = spawn_origin[0usize];
    (*client).ps.origin[1usize] = spawn_origin[1usize];
    (*client).ps.origin[2usize] = spawn_origin[2usize];
    (*client).ps.pm_flags |= 512i32;
    trap_GetUsercmd(
        client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int,
        &mut (*(*ent).client).pers.cmd,
    );
    SetClientViewAngle(ent, spawn_angles.as_mut_ptr());
    (*client).ps.pm_flags |= 64i32;
    (*client).ps.pm_time = 100i32;
    (*client).respawnTime = level.time;
    (*client).inactivityTime = level.time + g_inactivity.integer * 1000i32;
    (*client).latched_buttons = 0i32;
    (*client).ps.torsoAnim = TORSO_STAND as libc::c_int;
    (*client).ps.legsAnim = LEGS_IDLE as libc::c_int;
    if 0 == level.intermissiontime {
        if (*(*ent).client).sess.sessionTeam as libc::c_uint
            != TEAM_SPECTATOR as libc::c_int as libc::c_uint
        {
            G_KillBox(ent);
            (*client).ps.weapon = WP_MACHINEGUN as libc::c_int;
            (*client).ps.weaponstate = WEAPON_READY as libc::c_int;
            G_UseTargets(spawnPoint, ent);
            (*client).ps.weapon = 1i32;
            i = WP_NUM_WEAPONS as libc::c_int - 1i32;
            while i > 0i32 {
                if 0 != (*client).ps.stats[STAT_WEAPONS as libc::c_int as usize] & 1i32 << i {
                    (*client).ps.weapon = i;
                    break;
                } else {
                    i -= 1
                }
            }
            (*ent).r.currentOrigin[0usize] = (*(*ent).client).ps.origin[0usize];
            (*ent).r.currentOrigin[1usize] = (*(*ent).client).ps.origin[1usize];
            (*ent).r.currentOrigin[2usize] = (*(*ent).client).ps.origin[2usize];
            tent = G_TempEntity(
                (*(*ent).client).ps.origin.as_mut_ptr(),
                EV_PLAYER_TELEPORT_IN as libc::c_int,
            );
            (*tent).s.clientNum = (*ent).s.clientNum;
            trap_LinkEntity(ent);
        }
    } else {
        MoveClientToIntermission(ent);
    }
    (*client).ps.commandTime = level.time - 100i32;
    (*(*ent).client).pers.cmd.serverTime = level.time;
    ClientThink(ent.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int);
    if (*(*ent).client).sess.spectatorState as libc::c_uint
        != SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
    {
        ClientEndFrame(ent);
    }
    BG_PlayerStateToEntityState(&mut (*client).ps, &mut (*ent).s, qtrue);
}
/*
gentity_t	*spot;
gentity_t	*nearestSpot;

nearestSpot = SelectNearestDeathmatchSpawnPoint( avoidPoint );

spot = SelectRandomDeathmatchSpawnPoint ( );
if ( spot == nearestSpot ) {
    // roll again if it would be real close to point of death
    spot = SelectRandomDeathmatchSpawnPoint ( );
    if ( spot == nearestSpot ) {
        // last try
        spot = SelectRandomDeathmatchSpawnPoint ( );
    }
}

// find a single player start spot
if (!spot) {
    G_Error( "Couldn't find a spawn point" );
}

VectorCopy (spot->s.origin, origin);
origin[2] += 9;
VectorCopy (spot->s.angles, angles);

return spot;
*/
/*
===========
SelectInitialSpawnPoint

Try to find a spawn point marked 'initial', otherwise
use normal spawn selection.
============
*/
#[no_mangle]
pub unsafe extern "C" fn SelectInitialSpawnPoint(
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
    mut isbot: qboolean,
) -> *mut gentity_t {
    let mut spot: *mut gentity_t = 0 as *mut gentity_t;
    spot = 0 as *mut gentity_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char,
        );
        if spot.is_null() {
            break;
        }
        if 0 != (*spot).flags & 0x2000i32 && 0 != isbot as libc::c_uint
            || 0 != (*spot).flags & 0x4000i32 && 0 == isbot as u64
        {
            continue;
        }
        if 0 != (*spot).spawnflags & 0x1i32 {
            break;
        }
    }
    if spot.is_null() || 0 != SpotWouldTelefrag(spot) as libc::c_uint {
        return SelectSpawnPoint(vec3_origin.as_mut_ptr(), origin, angles, isbot);
    }
    *origin.offset(0isize) = (*spot).s.origin[0usize];
    *origin.offset(1isize) = (*spot).s.origin[1usize];
    *origin.offset(2isize) = (*spot).s.origin[2usize];
    let ref mut fresh2 = *origin.offset(2isize);
    *fresh2 += 9i32 as libc::c_float;
    *angles.offset(0isize) = (*spot).s.angles[0usize];
    *angles.offset(1isize) = (*spot).s.angles[1usize];
    *angles.offset(2isize) = (*spot).s.angles[2usize];
    return spot;
}
/*
===========
SelectSpectatorSpawnPoint

============
*/
#[no_mangle]
pub unsafe extern "C" fn SelectSpectatorSpawnPoint(
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
) -> *mut gentity_t {
    FindIntermissionPoint();
    *origin.offset(0isize) = level.intermission_origin[0usize];
    *origin.offset(1isize) = level.intermission_origin[1usize];
    *origin.offset(2isize) = level.intermission_origin[2usize];
    *angles.offset(0isize) = level.intermission_angle[0usize];
    *angles.offset(1isize) = level.intermission_angle[1usize];
    *angles.offset(2isize) = level.intermission_angle[2usize];
    return 0 as *mut gentity_t;
}
#[no_mangle]
pub unsafe extern "C" fn InitBodyQue() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    level.bodyQueIndex = 0i32;
    i = 0i32;
    while i < 8i32 {
        ent = G_Spawn();
        (*ent).classname = b"bodyque\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*ent).neverFree = qtrue;
        level.bodyQue[i as usize] = ent;
        i += 1
    }
}
//
// g_client.c
//
#[no_mangle]
pub unsafe extern "C" fn ClientConnect(
    mut clientNum: libc::c_int,
    mut firstTime: qboolean,
    mut isBot: qboolean,
) -> *mut libc::c_char {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    //	char		*areabits;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    ent = &mut g_entities[clientNum as usize] as *mut gentity_t;
    trap_GetUserinfo(
        clientNum,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    value = Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"ip\x00" as *const u8 as *const libc::c_char,
    );
    if 0 != G_FilterPacket(value) as u64 {
        return b"You are banned from this server.\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if 0 == isBot as u64
        && strcmp(value, b"localhost\x00" as *const u8 as *const libc::c_char) != 0i32
    {
        value = Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"password\x00" as *const u8 as *const libc::c_char,
        );
        if 0 != g_password.string[0usize] as libc::c_int
            && 0 != Q_stricmp(
                g_password.string.as_mut_ptr(),
                b"none\x00" as *const u8 as *const libc::c_char,
            )
            && strcmp(g_password.string.as_mut_ptr(), value) != 0i32
        {
            return b"Invalid password\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if 0 != (*ent).inuse as u64 {
        G_LogPrintf(
            b"Forcing disconnect on active client: %i\n\x00" as *const u8 as *const libc::c_char,
            clientNum,
        );
        ClientDisconnect(clientNum);
    }
    (*ent).client = level.clients.offset(clientNum as isize);
    client = (*ent).client;
    memset(
        client as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<gclient_t>() as libc::c_ulong,
    );
    (*client).pers.connected = CON_CONNECTING;
    value = Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"ip\x00" as *const u8 as *const libc::c_char,
    );
    if 0 == strcmp(value, b"localhost\x00" as *const u8 as *const libc::c_char) {
        (*client).pers.localClient = qtrue
    }
    if 0 != isBot as u64 {
        (*ent).r.svFlags |= 0x8i32;
        (*ent).inuse = qtrue;
        if 0 == G_BotConnect(
            clientNum,
            (0 == firstTime as u64) as libc::c_int as qboolean,
        ) as u64
        {
            return b"BotConnectfailed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    if 0 != firstTime as libc::c_uint || 0 != level.newSession as libc::c_uint {
        G_InitSessionData(client, userinfo.as_mut_ptr());
    }
    G_ReadSessionData(client);
    G_LogPrintf(
        b"ClientConnect: %i\n\x00" as *const u8 as *const libc::c_char,
        clientNum,
    );
    ClientUserinfoChanged(clientNum);
    if 0 != firstTime as u64 {
        trap_SendServerCommand(
            -1i32,
            va(
                b"print \"%s^7 connected\n\"\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*client).pers.netname.as_mut_ptr(),
            ),
        );
    }
    if g_gametype.integer >= GT_TEAM as libc::c_int
        && (*client).sess.sessionTeam as libc::c_uint
            != TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        BroadcastTeamChange(client, -1i32);
    }
    CalculateRanks();
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ClientUserinfoChanged(mut clientNum: libc::c_int) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut teamTask: libc::c_int = 0;
    let mut teamLeader: libc::c_int = 0;
    let mut health: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model: [libc::c_char; 64] = [0; 64];
    let mut headModel: [libc::c_char; 64] = [0; 64];
    let mut oldname: [libc::c_char; 1024] = [0; 1024];
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut c1: [libc::c_char; 1024] = [0; 1024];
    let mut c2: [libc::c_char; 1024] = [0; 1024];
    let mut redTeam: [libc::c_char; 1024] = [0; 1024];
    let mut blueTeam: [libc::c_char; 1024] = [0; 1024];
    let mut userinfo: [libc::c_char; 1024] = [0; 1024];
    ent = g_entities.as_mut_ptr().offset(clientNum as isize);
    client = (*ent).client;
    trap_GetUserinfo(
        clientNum,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == Info_Validate(userinfo.as_mut_ptr()) as u64 {
        strcpy(
            userinfo.as_mut_ptr(),
            b"\\name\\badinfo\x00" as *const u8 as *const libc::c_char,
        );
        trap_DropClient(
            clientNum,
            b"Invalid userinfo\x00" as *const u8 as *const libc::c_char,
        );
    }
    s = Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"cg_predictItems\x00" as *const u8 as *const libc::c_char,
    );
    if 0 == atoi(s) {
        (*client).pers.predictItemPickup = qfalse
    } else {
        (*client).pers.predictItemPickup = qtrue
    }
    Q_strncpyz(
        oldname.as_mut_ptr(),
        (*client).pers.netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    s = Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"name\x00" as *const u8 as *const libc::c_char,
    );
    ClientCleanName(
        s,
        (*client).pers.netname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
    );
    if (*client).sess.sessionTeam as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint {
        if (*client).sess.spectatorState as libc::c_uint
            == SPECTATOR_SCOREBOARD as libc::c_int as libc::c_uint
        {
            Q_strncpyz(
                (*client).pers.netname.as_mut_ptr(),
                b"scoreboard\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 36]>() as libc::c_ulong as libc::c_int,
            );
        }
    }
    if (*client).pers.connected as libc::c_uint == CON_CONNECTED as libc::c_int as libc::c_uint {
        if 0 != strcmp(oldname.as_mut_ptr(), (*client).pers.netname.as_mut_ptr()) {
            trap_SendServerCommand(
                -1i32,
                va(
                    b"print \"%s^7 renamed to %s\n\"\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    oldname.as_mut_ptr(),
                    (*client).pers.netname.as_mut_ptr(),
                ),
            );
        }
    }
    health = atoi(Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"handicap\x00" as *const u8 as *const libc::c_char,
    ));
    (*client).pers.maxHealth = health;
    if (*client).pers.maxHealth < 1i32 || (*client).pers.maxHealth > 100i32 {
        (*client).pers.maxHealth = 100i32
    }
    (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] = (*client).pers.maxHealth;
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        Q_strncpyz(
            model.as_mut_ptr(),
            Info_ValueForKey(
                userinfo.as_mut_ptr(),
                b"team_model\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        Q_strncpyz(
            headModel.as_mut_ptr(),
            Info_ValueForKey(
                userinfo.as_mut_ptr(),
                b"team_headmodel\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        Q_strncpyz(
            model.as_mut_ptr(),
            Info_ValueForKey(
                userinfo.as_mut_ptr(),
                b"model\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        Q_strncpyz(
            headModel.as_mut_ptr(),
            Info_ValueForKey(
                userinfo.as_mut_ptr(),
                b"headmodel\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
    }
    s = Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"teamoverlay\x00" as *const u8 as *const libc::c_char,
    );
    if 0 == *s || atoi(s) != 0i32 {
        (*client).pers.teamInfo = qtrue
    } else {
        (*client).pers.teamInfo = qfalse
    }
    teamTask = atoi(Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"teamtask\x00" as *const u8 as *const libc::c_char,
    ));
    teamLeader = (*client).sess.teamLeader as libc::c_int;
    Q_strncpyz(
        c1.as_mut_ptr(),
        Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"color1\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Q_strncpyz(
        c2.as_mut_ptr(),
        Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"color2\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Q_strncpyz(
        redTeam.as_mut_ptr(),
        Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"g_redteam\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Q_strncpyz(
        blueTeam.as_mut_ptr(),
        Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"g_blueteam\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != (*ent).r.svFlags & 0x8i32 {
        s =
            va(b"n\\%s\\t\\%i\\model\\%s\\hmodel\\%s\\c1\\%s\\c2\\%s\\hc\\%i\\w\\%i\\l\\%i\\skill\\%s\\tt\\%d\\tl\\%d\x00"
                   as *const u8 as *const libc::c_char as *mut libc::c_char,
               (*client).pers.netname.as_mut_ptr(),
               (*client).sess.sessionTeam as libc::c_uint, model.as_mut_ptr(),
               headModel.as_mut_ptr(), c1.as_mut_ptr(), c2.as_mut_ptr(),
               (*client).pers.maxHealth, (*client).sess.wins,
               (*client).sess.losses,
               Info_ValueForKey(userinfo.as_mut_ptr(),
                                b"skill\x00" as *const u8 as
                                    *const libc::c_char), teamTask,
               teamLeader)
    } else {
        s =
            va(b"n\\%s\\t\\%i\\model\\%s\\hmodel\\%s\\g_redteam\\%s\\g_blueteam\\%s\\c1\\%s\\c2\\%s\\hc\\%i\\w\\%i\\l\\%i\\tt\\%d\\tl\\%d\x00"
                   as *const u8 as *const libc::c_char as *mut libc::c_char,
               (*client).pers.netname.as_mut_ptr(),
               (*client).sess.sessionTeam as libc::c_uint, model.as_mut_ptr(),
               headModel.as_mut_ptr(), redTeam.as_mut_ptr(),
               blueTeam.as_mut_ptr(), c1.as_mut_ptr(), c2.as_mut_ptr(),
               (*client).pers.maxHealth, (*client).sess.wins,
               (*client).sess.losses, teamTask, teamLeader)
    }
    trap_SetConfigstring(32i32 + 256i32 + 256i32 + clientNum, s);
    G_LogPrintf(
        b"ClientUserinfoChanged: %i %s\n\x00" as *const u8 as *const libc::c_char,
        clientNum,
        s,
    );
}
/*
===========
ForceClientSkin

Forces a client's skin (for teamplay)
===========
*/
/*
static void ForceClientSkin( gclient_t *client, char *model, const char *skin ) {
    char *p;

    if ((p = strrchr(model, '/')) != 0) {
        *p = 0;
    }

    Q_strcat(model, MAX_QPATH, "/");
    Q_strcat(model, MAX_QPATH, skin);
}
*/
/*
===========
ClientCleanName
============
*/
unsafe extern "C" fn ClientCleanName(
    mut in_0: *const libc::c_char,
    mut out: *mut libc::c_char,
    mut outSize: libc::c_int,
) {
    let mut outpos: libc::c_int = 0i32;
    let mut colorlessLen: libc::c_int = 0i32;
    let mut spaces: libc::c_int = 0i32;
    while *in_0 as libc::c_int == ' ' as i32 {
        in_0 = in_0.offset(1isize)
    }
    let mut current_block_12: u64;
    while 0 != *in_0 as libc::c_int && outpos < outSize - 1i32 {
        *out.offset(outpos as isize) = *in_0;
        if *in_0 as libc::c_int == ' ' as i32 {
            // don't allow too many consecutive spaces
            if spaces > 2i32 {
                current_block_12 = 4644295000439058019;
            } else {
                spaces += 1;
                current_block_12 = 26972500619410423;
            }
        } else if outpos > 0i32
            && *out.offset((outpos - 1i32) as isize) as libc::c_int == '^' as i32
        {
            if 0 != Q_IsColorString(&mut *out.offset((outpos - 1i32) as isize)) as u64 {
                colorlessLen -= 1;
                if *in_0 as libc::c_int - '0' as i32 & 0x7i32 == 0i32 {
                    outpos -= 1;
                    current_block_12 = 4644295000439058019;
                } else {
                    current_block_12 = 26972500619410423;
                }
            } else {
                spaces = 0i32;
                colorlessLen += 1;
                current_block_12 = 26972500619410423;
            }
        } else {
            spaces = 0i32;
            colorlessLen += 1;
            current_block_12 = 26972500619410423;
        }
        match current_block_12 {
            26972500619410423 => outpos += 1,
            _ => {}
        }
        in_0 = in_0.offset(1isize)
    }
    *out.offset(outpos as isize) = '\u{0}' as i32 as libc::c_char;
    if *out as libc::c_int == '\u{0}' as i32 || colorlessLen == 0i32 {
        Q_strncpyz(
            out,
            b"UnnamedPlayer\x00" as *const u8 as *const libc::c_char,
            outSize,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClientDisconnect(mut clientNum: libc::c_int) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    G_RemoveQueuedBotBegin(clientNum);
    ent = g_entities.as_mut_ptr().offset(clientNum as isize);
    if (*ent).client.is_null()
        || (*(*ent).client).pers.connected as libc::c_uint
            == CON_DISCONNECTED as libc::c_int as libc::c_uint
    {
        return;
    }
    i = 0i32;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
            == TEAM_SPECTATOR as libc::c_int as libc::c_uint
            && (*level.clients.offset(i as isize)).sess.spectatorState as libc::c_uint
                == SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
            && (*level.clients.offset(i as isize)).sess.spectatorClient == clientNum
        {
            StopFollowing(&mut g_entities[i as usize]);
        }
        i += 1
    }
    if (*(*ent).client).pers.connected as libc::c_uint
        == CON_CONNECTED as libc::c_int as libc::c_uint
        && (*(*ent).client).sess.sessionTeam as libc::c_uint
            != TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        tent = G_TempEntity(
            (*(*ent).client).ps.origin.as_mut_ptr(),
            EV_PLAYER_TELEPORT_OUT as libc::c_int,
        );
        (*tent).s.clientNum = (*ent).s.clientNum;
        TossClientItems(ent);
    }
    G_LogPrintf(
        b"ClientDisconnect: %i\n\x00" as *const u8 as *const libc::c_char,
        clientNum,
    );
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int
        && 0 == level.intermissiontime
        && 0 == level.warmupTime
        && level.sortedClients[1usize] == clientNum
    {
        let ref mut fresh3 = (*level.clients.offset(level.sortedClients[0usize] as isize))
            .sess
            .wins;
        *fresh3 += 1;
        ClientUserinfoChanged(level.sortedClients[0usize]);
    }
    if g_gametype.integer == GT_TOURNAMENT as libc::c_int
        && (*(*ent).client).sess.sessionTeam as libc::c_uint
            == TEAM_FREE as libc::c_int as libc::c_uint
        && 0 != level.intermissiontime
    {
        trap_SendConsoleCommand(
            EXEC_APPEND as libc::c_int,
            b"map_restart 0\n\x00" as *const u8 as *const libc::c_char,
        );
        level.restarted = qtrue;
        level.changemap = 0 as *mut libc::c_char;
        level.intermissiontime = 0i32
    }
    trap_UnlinkEntity(ent);
    (*ent).s.modelindex = 0i32;
    (*ent).inuse = qfalse;
    (*ent).classname = b"disconnected\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*(*ent).client).pers.connected = CON_DISCONNECTED;
    (*(*ent).client).ps.persistant[PERS_TEAM as libc::c_int as usize] = TEAM_FREE as libc::c_int;
    (*(*ent).client).sess.sessionTeam = TEAM_FREE;
    trap_SetConfigstring(
        32i32 + 256i32 + 256i32 + clientNum,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    CalculateRanks();
    if 0 != (*ent).r.svFlags & 0x8i32 {
        BotAIShutdownClient(clientNum, qfalse);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClientBegin(mut clientNum: libc::c_int) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut flags: libc::c_int = 0;
    ent = g_entities.as_mut_ptr().offset(clientNum as isize);
    client = level.clients.offset(clientNum as isize);
    if 0 != (*ent).r.linked as u64 {
        trap_UnlinkEntity(ent);
    }
    G_InitGentity(ent);
    (*ent).touch = None;
    (*ent).pain = None;
    (*ent).client = client;
    (*client).pers.connected = CON_CONNECTED;
    (*client).pers.enterTime = level.time;
    (*client).pers.teamState.state = TEAM_BEGIN;
    flags = (*client).ps.eFlags;
    memset(
        &mut (*client).ps as *mut playerState_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<playerState_t>() as libc::c_ulong,
    );
    (*client).ps.eFlags = flags;
    ClientSpawn(ent);
    if (*client).sess.sessionTeam as libc::c_uint != TEAM_SPECTATOR as libc::c_int as libc::c_uint {
        if g_gametype.integer != GT_TOURNAMENT as libc::c_int {
            trap_SendServerCommand(
                -1i32,
                va(
                    b"print \"%s^7 entered the game\n\"\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*client).pers.netname.as_mut_ptr(),
                ),
            );
        }
    }
    G_LogPrintf(
        b"ClientBegin: %i\n\x00" as *const u8 as *const libc::c_char,
        clientNum,
    );
    CalculateRanks();
}
/*QUAKED info_player_deathmatch (1 0 1) (-16 -16 -24) (16 16 32) initial
potential spawning position for deathmatch games.
The first time a player enters the game, they will be at an 'initial' spot.
Targets will be fired when someone spawns in on them.
"nobots" will prevent bots from using this spot.
"nohumans" will prevent non-bots from using this spot.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_deathmatch(mut ent: *mut gentity_t) {
    let mut i: libc::c_int = 0;
    G_SpawnInt(
        b"nobots\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut i,
    );
    if 0 != i {
        (*ent).flags |= 0x2000i32
    }
    G_SpawnInt(
        b"nohumans\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut i,
    );
    if 0 != i {
        (*ent).flags |= 0x4000i32
    };
}
/*QUAKED info_player_start (1 0 0) (-16 -16 -24) (16 16 32)
equivalent to info_player_deathmatch
*/
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_start(mut ent: *mut gentity_t) {
    (*ent).classname =
        b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    SP_info_player_deathmatch(ent);
}
/*QUAKED info_player_intermission (1 0 1) (-16 -16 -24) (16 16 32)
The intermission will be viewed from this point.  Target an info_notnull for the view direction.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_intermission(mut ent: *mut gentity_t) {}
/*
================
SelectNearestDeathmatchSpawnPoint

Find the spot that we DON'T want to use
================
*/
#[no_mangle]
pub unsafe extern "C" fn SelectNearestDeathmatchSpawnPoint(mut from: *mut vec_t) -> *mut gentity_t {
    let mut spot: *mut gentity_t = 0 as *mut gentity_t;
    let mut delta: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    let mut nearestDist: libc::c_float = 0.;
    let mut nearestSpot: *mut gentity_t = 0 as *mut gentity_t;
    nearestDist = 999999i32 as libc::c_float;
    nearestSpot = 0 as *mut gentity_t;
    spot = 0 as *mut gentity_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char,
        );
        if spot.is_null() {
            break;
        }
        delta[0usize] = (*spot).s.origin[0usize] - *from.offset(0isize);
        delta[1usize] = (*spot).s.origin[1usize] - *from.offset(1isize);
        delta[2usize] = (*spot).s.origin[2usize] - *from.offset(2isize);
        dist = VectorLength(delta.as_mut_ptr() as *const vec_t);
        if dist < nearestDist {
            nearestDist = dist;
            nearestSpot = spot
        }
    }
    return nearestSpot;
}
/*
================
SelectRandomDeathmatchSpawnPoint

go to a random point that doesn't telefrag
================
*/
#[no_mangle]
pub unsafe extern "C" fn SelectRandomDeathmatchSpawnPoint(mut isbot: qboolean) -> *mut gentity_t {
    let mut spot: *mut gentity_t = 0 as *mut gentity_t;
    let mut count: libc::c_int = 0;
    let mut selection: libc::c_int = 0;
    let mut spots: [*mut gentity_t; 128] = [0 as *mut gentity_t; 128];
    count = 0i32;
    spot = 0 as *mut gentity_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char,
        );
        if !(!spot.is_null() && count < 128i32) {
            break;
        }
        if 0 != SpotWouldTelefrag(spot) as u64 {
            continue;
        }
        if !(0 != (*spot).flags & 0x2000i32 && 0 != isbot as libc::c_uint
            || 0 != (*spot).flags & 0x4000i32 && 0 == isbot as u64)
        {
            // spot is not for this human/bot player
            spots[count as usize] = spot;
            count += 1
        }
    }
    if 0 == count {
        return G_Find(
            0 as *mut gentity_t,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char,
        );
    }
    selection = rand() % count;
    return spots[selection as usize];
}
