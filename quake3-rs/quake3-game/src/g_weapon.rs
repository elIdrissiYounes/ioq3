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
    unnamed_4, unnamed_5, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON,
    EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT,
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
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO,
    IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM,
    IT_WEAPON, MOD_BFG, MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE,
    MOD_GRENADE, MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA,
    MOD_PLASMA_SPLASH, MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME,
    MOD_SUICIDE, MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER,
    PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT,
    PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED,
    PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PW_AMMOREGEN,
    PW_BATTLESUIT, PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS,
    PW_INVULNERABILITY, PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN,
    PW_SCOUT, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM,
    STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN,
    WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gentity_s, gentity_t,
    level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t, spectatorState_t,
    trap_LinkEntity, trap_Trace, trap_UnlinkEntity, CON_CONNECTED, CON_CONNECTING,
    CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1, MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW,
    SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
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
use libc;
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    usercmd_s, usercmd_t, vec3_t, vec_t, vmCvar_t, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{ceil, cos, floor, rand, sin};

unsafe extern "C" fn CrossProduct(
    mut v1: *const vec_t,
    mut v2: *const vec_t,
    mut cross: *mut vec_t,
) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) - *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) - *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) - *v1.offset(1isize) * *v2.offset(0isize);
}
//
// g_weapon.c
//
#[no_mangle]
pub unsafe extern "C" fn LogAccuracyHit(
    mut target: *mut gentity_t,
    mut attacker: *mut gentity_t,
) -> qboolean {
    if 0 == (*target).takedamage as u64 {
        return qfalse;
    }
    if target == attacker {
        return qfalse;
    }
    if (*target).client.is_null() {
        return qfalse;
    }
    if (*attacker).client.is_null() {
        return qfalse;
    }
    if (*(*target).client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        return qfalse;
    }
    if 0 != OnSameTeam(target, attacker) as u64 {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CalcMuzzlePoint(
    mut ent: *mut gentity_t,
    mut forward_0: *mut vec_t,
    mut right_0: *mut vec_t,
    mut up_0: *mut vec_t,
    mut muzzlePoint: *mut vec_t,
) {
    *muzzlePoint.offset(0isize) = (*ent).s.pos.trBase[0usize];
    *muzzlePoint.offset(1isize) = (*ent).s.pos.trBase[1usize];
    *muzzlePoint.offset(2isize) = (*ent).s.pos.trBase[2usize];
    let ref mut fresh0 = *muzzlePoint.offset(2isize);
    *fresh0 += (*(*ent).client).ps.viewheight as libc::c_float;
    *muzzlePoint.offset(0isize) =
        *muzzlePoint.offset(0isize) + *forward_0.offset(0isize) * 14i32 as libc::c_float;
    *muzzlePoint.offset(1isize) =
        *muzzlePoint.offset(1isize) + *forward_0.offset(1isize) * 14i32 as libc::c_float;
    *muzzlePoint.offset(2isize) =
        *muzzlePoint.offset(2isize) + *forward_0.offset(2isize) * 14i32 as libc::c_float;
    *muzzlePoint.offset(0isize) = *muzzlePoint.offset(0isize) as libc::c_int as vec_t;
    *muzzlePoint.offset(1isize) = *muzzlePoint.offset(1isize) as libc::c_int as vec_t;
    *muzzlePoint.offset(2isize) = *muzzlePoint.offset(2isize) as libc::c_int as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn SnapVectorTowards(mut v: *mut vec_t, mut to: *mut vec_t) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        if *to.offset(i as isize) <= *v.offset(i as isize) {
            *v.offset(i as isize) = floor(*v.offset(i as isize) as libc::c_double) as vec_t
        } else {
            *v.offset(i as isize) = ceil(*v.offset(i as isize) as libc::c_double) as vec_t
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn CheckGauntletAttack(mut ent: *mut gentity_t) -> qboolean {
    let mut tr: trace_t = trace_t {
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
    let mut end: vec3_t = [0.; 3];
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut traceEnt: *mut gentity_t = 0 as *mut gentity_t;
    let mut damage: libc::c_int = 0;
    AngleVectors(
        (*(*ent).client).ps.viewangles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    CalcMuzzlePoint(
        ent,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
        muzzle.as_mut_ptr(),
    );
    end[0usize] = muzzle[0usize] + forward[0usize] * 32i32 as libc::c_float;
    end[1usize] = muzzle[1usize] + forward[1usize] * 32i32 as libc::c_float;
    end[2usize] = muzzle[2usize] + forward[2usize] * 32i32 as libc::c_float;
    trap_Trace(
        &mut tr,
        muzzle.as_mut_ptr() as *const vec_t,
        0 as *const vec_t,
        0 as *const vec_t,
        end.as_mut_ptr() as *const vec_t,
        (*ent).s.number,
        1i32 | 0x2000000i32 | 0x4000000i32,
    );
    if 0 != tr.surfaceFlags & 0x10i32 {
        return qfalse;
    }
    if 0 != (*(*ent).client).noclip as u64 {
        return qfalse;
    }
    traceEnt = &mut g_entities[tr.entityNum as usize] as *mut gentity_t;
    if 0 != (*traceEnt).takedamage as libc::c_uint && !(*traceEnt).client.is_null() {
        tent = G_TempEntity(tr.endpos.as_mut_ptr(), EV_MISSILE_HIT as libc::c_int);
        (*tent).s.otherEntityNum = (*traceEnt).s.number;
        (*tent).s.eventParm = DirToByte(tr.plane.normal.as_mut_ptr());
        (*tent).s.weapon = (*ent).s.weapon
    }
    if 0 == (*traceEnt).takedamage as u64 {
        return qfalse;
    }
    if 0 != (*(*ent).client).ps.powerups[PW_QUAD as libc::c_int as usize] {
        G_AddEvent(ent, EV_POWERUP_QUAD as libc::c_int, 0i32);
        s_quadFactor = g_quadfactor.value
    } else {
        s_quadFactor = 1i32 as libc::c_float
    }
    damage = (50i32 as libc::c_float * s_quadFactor) as libc::c_int;
    G_Damage(
        traceEnt,
        ent,
        ent,
        forward.as_mut_ptr(),
        tr.endpos.as_mut_ptr(),
        damage,
        0i32,
        MOD_GAUNTLET as libc::c_int,
    );
    return qtrue;
}
static mut forward: vec3_t = [0.; 3];
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
// g_weapon.c
// perform the server side effects of a weapon firing
static mut s_quadFactor: libc::c_float = 0.;
static mut muzzle: vec3_t = [0.; 3];
static mut up: vec3_t = [0.; 3];
static mut right: vec3_t = [0.; 3];
#[no_mangle]
pub unsafe extern "C" fn Weapon_HookFree(mut ent: *mut gentity_t) {
    (*(*(*ent).parent).client).hook = 0 as *mut gentity_t;
    (*(*(*ent).parent).client).ps.pm_flags &= !2048i32;
    G_FreeEntity(ent);
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_HookThink(mut ent: *mut gentity_t) {
    if !(*ent).enemy.is_null() {
        let mut v: vec3_t = [0.; 3];
        let mut oldorigin: vec3_t = [0.; 3];
        oldorigin[0usize] = (*ent).r.currentOrigin[0usize];
        oldorigin[1usize] = (*ent).r.currentOrigin[1usize];
        oldorigin[2usize] = (*ent).r.currentOrigin[2usize];
        v[0usize] = ((*(*ent).enemy).r.currentOrigin[0usize] as libc::c_double
            + ((*(*ent).enemy).r.mins[0usize] + (*(*ent).enemy).r.maxs[0usize]) as libc::c_double
                * 0.5f64) as vec_t;
        v[1usize] = ((*(*ent).enemy).r.currentOrigin[1usize] as libc::c_double
            + ((*(*ent).enemy).r.mins[1usize] + (*(*ent).enemy).r.maxs[1usize]) as libc::c_double
                * 0.5f64) as vec_t;
        v[2usize] = ((*(*ent).enemy).r.currentOrigin[2usize] as libc::c_double
            + ((*(*ent).enemy).r.mins[2usize] + (*(*ent).enemy).r.maxs[2usize]) as libc::c_double
                * 0.5f64) as vec_t;
        SnapVectorTowards(v.as_mut_ptr(), oldorigin.as_mut_ptr());
        G_SetOrigin(ent, v.as_mut_ptr());
    }
    (*(*(*ent).parent).client).ps.grapplePoint[0usize] = (*ent).r.currentOrigin[0usize];
    (*(*(*ent).parent).client).ps.grapplePoint[1usize] = (*ent).r.currentOrigin[1usize];
    (*(*(*ent).parent).client).ps.grapplePoint[2usize] = (*ent).r.currentOrigin[2usize];
}
//
// g_weapon.c
//
#[no_mangle]
pub unsafe extern "C" fn FireWeapon(mut ent: *mut gentity_t) {
    if 0 != (*(*ent).client).ps.powerups[PW_QUAD as libc::c_int as usize] {
        s_quadFactor = g_quadfactor.value
    } else {
        s_quadFactor = 1i32 as libc::c_float
    }
    if (*ent).s.weapon != WP_GRAPPLING_HOOK as libc::c_int
        && (*ent).s.weapon != WP_GAUNTLET as libc::c_int
    {
        (*(*ent).client).accuracy_shots += 1
    }
    AngleVectors(
        (*(*ent).client).ps.viewangles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    CalcMuzzlePointOrigin(
        ent,
        (*(*ent).client).oldOrigin.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
        muzzle.as_mut_ptr(),
    );
    match (*ent).s.weapon {
        1 => {
            Weapon_Gauntlet(ent);
        }
        6 => {
            Weapon_LightningFire(ent);
        }
        3 => {
            weapon_supershotgun_fire(ent);
        }
        2 => {
            if g_gametype.integer != GT_TEAM as libc::c_int {
                Bullet_Fire(
                    ent,
                    200i32 as libc::c_float,
                    7i32,
                    MOD_MACHINEGUN as libc::c_int,
                );
            } else {
                Bullet_Fire(
                    ent,
                    200i32 as libc::c_float,
                    5i32,
                    MOD_MACHINEGUN as libc::c_int,
                );
            }
        }
        4 => {
            weapon_grenadelauncher_fire(ent);
        }
        5 => {
            Weapon_RocketLauncher_Fire(ent);
        }
        8 => {
            Weapon_Plasmagun_Fire(ent);
        }
        7 => {
            weapon_railgun_fire(ent);
        }
        9 => {
            BFG_Fire(ent);
        }
        10 => {
            Weapon_GrapplingHook_Fire(ent);
        }
        _ => {}
    };
}
/*
======================================================================

GRAPPLING HOOK

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Weapon_GrapplingHook_Fire(mut ent: *mut gentity_t) {
    if 0 == (*(*ent).client).fireHeld as u64 && (*(*ent).client).hook.is_null() {
        fire_grapple(ent, muzzle.as_mut_ptr(), forward.as_mut_ptr());
    }
    (*(*ent).client).fireHeld = qtrue;
}
/*
======================================================================

BFG

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn BFG_Fire(mut ent: *mut gentity_t) {
    let mut m: *mut gentity_t = 0 as *mut gentity_t;
    m = fire_bfg(ent, muzzle.as_mut_ptr(), forward.as_mut_ptr());
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
}
//	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
/*
======================================================================

RAILGUN

======================================================================
*/
/*
=================
weapon_railgun_fire
=================
*/
#[no_mangle]
pub unsafe extern "C" fn weapon_railgun_fire(mut ent: *mut gentity_t) {
    let mut end: vec3_t = [0.; 3];
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
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut traceEnt: *mut gentity_t = 0 as *mut gentity_t;
    let mut damage: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut hits: libc::c_int = 0;
    let mut unlinked: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    let mut unlinkedEntities: [*mut gentity_t; 4] = [0 as *mut gentity_t; 4];
    damage = (100i32 as libc::c_float * s_quadFactor) as libc::c_int;
    end[0usize] = muzzle[0usize] + forward[0usize] * 8192i32 as libc::c_float;
    end[1usize] = muzzle[1usize] + forward[1usize] * 8192i32 as libc::c_float;
    end[2usize] = muzzle[2usize] + forward[2usize] * 8192i32 as libc::c_float;
    unlinked = 0i32;
    hits = 0i32;
    passent = (*ent).s.number;
    loop {
        trap_Trace(
            &mut trace,
            muzzle.as_mut_ptr() as *const vec_t,
            0 as *const vec_t,
            0 as *const vec_t,
            end.as_mut_ptr() as *const vec_t,
            passent,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        if trace.entityNum >= (1i32 << 10i32) - 2i32 {
            break;
        }
        traceEnt = &mut g_entities[trace.entityNum as usize] as *mut gentity_t;
        if 0 != (*traceEnt).takedamage as u64 {
            if 0 != LogAccuracyHit(traceEnt, ent) as u64 {
                hits += 1
            }
            G_Damage(
                traceEnt,
                ent,
                ent,
                forward.as_mut_ptr(),
                trace.endpos.as_mut_ptr(),
                damage,
                0i32,
                MOD_RAILGUN as libc::c_int,
            );
        }
        if 0 != trace.contents & 1i32 {
            // we hit something solid enough to stop the beam
            break;
        } else {
            trap_UnlinkEntity(traceEnt);
            unlinkedEntities[unlinked as usize] = traceEnt;
            unlinked += 1;
            if !(unlinked < 4i32) {
                break;
            }
        }
    }
    i = 0i32;
    while i < unlinked {
        trap_LinkEntity(unlinkedEntities[i as usize]);
        i += 1
    }
    SnapVectorTowards(trace.endpos.as_mut_ptr(), muzzle.as_mut_ptr());
    tent = G_TempEntity(trace.endpos.as_mut_ptr(), EV_RAILTRAIL as libc::c_int);
    (*tent).s.clientNum = (*ent).s.clientNum;
    (*tent).s.origin2[0usize] = muzzle[0usize];
    (*tent).s.origin2[1usize] = muzzle[1usize];
    (*tent).s.origin2[2usize] = muzzle[2usize];
    (*tent).s.origin2[0usize] = (*tent).s.origin2[0usize] + right[0usize] * 4i32 as libc::c_float;
    (*tent).s.origin2[1usize] = (*tent).s.origin2[1usize] + right[1usize] * 4i32 as libc::c_float;
    (*tent).s.origin2[2usize] = (*tent).s.origin2[2usize] + right[2usize] * 4i32 as libc::c_float;
    (*tent).s.origin2[0usize] = (*tent).s.origin2[0usize] + up[0usize] * -1i32 as libc::c_float;
    (*tent).s.origin2[1usize] = (*tent).s.origin2[1usize] + up[1usize] * -1i32 as libc::c_float;
    (*tent).s.origin2[2usize] = (*tent).s.origin2[2usize] + up[2usize] * -1i32 as libc::c_float;
    if 0 != trace.surfaceFlags & 0x10i32 {
        (*tent).s.eventParm = 255i32
    } else {
        (*tent).s.eventParm = DirToByte(trace.plane.normal.as_mut_ptr())
    }
    (*tent).s.clientNum = (*ent).s.clientNum;
    if hits == 0i32 {
        (*(*ent).client).accurateCount = 0i32
    } else {
        (*(*ent).client).accurateCount += hits;
        if (*(*ent).client).accurateCount >= 2i32 {
            (*(*ent).client).accurateCount -= 2i32;
            (*(*ent).client).ps.persistant[PERS_IMPRESSIVE_COUNT as libc::c_int as usize] += 1;
            (*(*ent).client).ps.eFlags &=
                !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
            (*(*ent).client).ps.eFlags |= 0x8000i32;
            (*(*ent).client).rewardTime = level.time + 2000i32
        }
        (*(*ent).client).accuracy_hits += 1
    };
}
//	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
/*
======================================================================

PLASMA GUN

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Weapon_Plasmagun_Fire(mut ent: *mut gentity_t) {
    let mut m: *mut gentity_t = 0 as *mut gentity_t;
    m = fire_plasma(ent, muzzle.as_mut_ptr(), forward.as_mut_ptr());
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
}
//	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
/*
======================================================================

ROCKET

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Weapon_RocketLauncher_Fire(mut ent: *mut gentity_t) {
    let mut m: *mut gentity_t = 0 as *mut gentity_t;
    m = fire_rocket(ent, muzzle.as_mut_ptr(), forward.as_mut_ptr());
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
}
/*
======================================================================

GRENADE LAUNCHER

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn weapon_grenadelauncher_fire(mut ent: *mut gentity_t) {
    let mut m: *mut gentity_t = 0 as *mut gentity_t;
    forward[2usize] += 0.2f32;
    VectorNormalize(forward.as_mut_ptr());
    m = fire_grenade(ent, muzzle.as_mut_ptr(), forward.as_mut_ptr());
    (*m).damage = ((*m).damage as libc::c_float * s_quadFactor) as libc::c_int;
    (*m).splashDamage = ((*m).splashDamage as libc::c_float * s_quadFactor) as libc::c_int;
}
// wimpier MG in teamplay
#[no_mangle]
pub unsafe extern "C" fn Bullet_Fire(
    mut ent: *mut gentity_t,
    mut spread: libc::c_float,
    mut damage: libc::c_int,
    mut mod_0: libc::c_int,
) {
    let mut tr: trace_t = trace_t {
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
    let mut end: vec3_t = [0.; 3];
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut traceEnt: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    damage = (damage as libc::c_float * s_quadFactor) as libc::c_int;
    r = (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
        * 3.14159265358979323846f64
        * 2.0f32 as libc::c_double) as libc::c_float;
    u = (sin(r as libc::c_double)
        * (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64))
        * spread as libc::c_double
        * 16i32 as libc::c_double) as libc::c_float;
    r = (cos(r as libc::c_double)
        * (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64))
        * spread as libc::c_double
        * 16i32 as libc::c_double) as libc::c_float;
    end[0usize] = muzzle[0usize] + forward[0usize] * (8192i32 * 16i32) as libc::c_float;
    end[1usize] = muzzle[1usize] + forward[1usize] * (8192i32 * 16i32) as libc::c_float;
    end[2usize] = muzzle[2usize] + forward[2usize] * (8192i32 * 16i32) as libc::c_float;
    end[0usize] = end[0usize] + right[0usize] * r;
    end[1usize] = end[1usize] + right[1usize] * r;
    end[2usize] = end[2usize] + right[2usize] * r;
    end[0usize] = end[0usize] + up[0usize] * u;
    end[1usize] = end[1usize] + up[1usize] * u;
    end[2usize] = end[2usize] + up[2usize] * u;
    passent = (*ent).s.number;
    i = 0i32;
    if i < 10i32 {
        trap_Trace(
            &mut tr,
            muzzle.as_mut_ptr() as *const vec_t,
            0 as *const vec_t,
            0 as *const vec_t,
            end.as_mut_ptr() as *const vec_t,
            passent,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        if 0 != tr.surfaceFlags & 0x10i32 {
            return;
        }
        traceEnt = &mut g_entities[tr.entityNum as usize] as *mut gentity_t;
        SnapVectorTowards(tr.endpos.as_mut_ptr(), muzzle.as_mut_ptr());
        if 0 != (*traceEnt).takedamage as libc::c_uint && !(*traceEnt).client.is_null() {
            tent = G_TempEntity(tr.endpos.as_mut_ptr(), EV_BULLET_HIT_FLESH as libc::c_int);
            (*tent).s.eventParm = (*traceEnt).s.number;
            if 0 != LogAccuracyHit(traceEnt, ent) as u64 {
                (*(*ent).client).accuracy_hits += 1
            }
        } else {
            tent = G_TempEntity(tr.endpos.as_mut_ptr(), EV_BULLET_HIT_WALL as libc::c_int);
            (*tent).s.eventParm = DirToByte(tr.plane.normal.as_mut_ptr())
        }
        (*tent).s.otherEntityNum = (*ent).s.number;
        if 0 != (*traceEnt).takedamage as u64 {
            G_Damage(
                traceEnt,
                ent,
                ent,
                forward.as_mut_ptr(),
                tr.endpos.as_mut_ptr(),
                damage,
                0i32,
                mod_0,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn weapon_supershotgun_fire(mut ent: *mut gentity_t) {
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    tent = G_TempEntity(muzzle.as_mut_ptr(), EV_SHOTGUN as libc::c_int);
    (*tent).s.origin2[0usize] = forward[0usize] * 4096i32 as libc::c_float;
    (*tent).s.origin2[1usize] = forward[1usize] * 4096i32 as libc::c_float;
    (*tent).s.origin2[2usize] = forward[2usize] * 4096i32 as libc::c_float;
    (*tent).s.origin2[0usize] = (*tent).s.origin2[0usize] as libc::c_int as vec_t;
    (*tent).s.origin2[1usize] = (*tent).s.origin2[1usize] as libc::c_int as vec_t;
    (*tent).s.origin2[2usize] = (*tent).s.origin2[2usize] as libc::c_int as vec_t;
    (*tent).s.eventParm = rand() & 255i32;
    (*tent).s.otherEntityNum = (*ent).s.number;
    ShotgunPattern(
        (*tent).s.pos.trBase.as_mut_ptr(),
        (*tent).s.origin2.as_mut_ptr(),
        (*tent).s.eventParm,
        ent,
    );
}
// this should match CG_ShotgunPattern
#[no_mangle]
pub unsafe extern "C" fn ShotgunPattern(
    mut origin: *mut vec_t,
    mut origin2: *mut vec_t,
    mut seed: libc::c_int,
    mut ent: *mut gentity_t,
) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut end: vec3_t = [0.; 3];
    let mut forward_0: vec3_t = [0.; 3];
    let mut right_0: vec3_t = [0.; 3];
    let mut up_0: vec3_t = [0.; 3];
    let mut hitClient: qboolean = qfalse;
    VectorNormalize2(origin2 as *const vec_t, forward_0.as_mut_ptr());
    PerpendicularVector(right_0.as_mut_ptr(), forward_0.as_mut_ptr() as *const vec_t);
    CrossProduct(
        forward_0.as_mut_ptr() as *const vec_t,
        right_0.as_mut_ptr() as *const vec_t,
        up_0.as_mut_ptr(),
    );
    i = 0i32;
    while i < 11i32 {
        r = Q_crandom(&mut seed) * 700i32 as libc::c_float * 16i32 as libc::c_float;
        u = Q_crandom(&mut seed) * 700i32 as libc::c_float * 16i32 as libc::c_float;
        end[0usize] =
            *origin.offset(0isize) + forward_0[0usize] * (8192i32 * 16i32) as libc::c_float;
        end[1usize] =
            *origin.offset(1isize) + forward_0[1usize] * (8192i32 * 16i32) as libc::c_float;
        end[2usize] =
            *origin.offset(2isize) + forward_0[2usize] * (8192i32 * 16i32) as libc::c_float;
        end[0usize] = end[0usize] + right_0[0usize] * r;
        end[1usize] = end[1usize] + right_0[1usize] * r;
        end[2usize] = end[2usize] + right_0[2usize] * r;
        end[0usize] = end[0usize] + up_0[0usize] * u;
        end[1usize] = end[1usize] + up_0[1usize] * u;
        end[2usize] = end[2usize] + up_0[2usize] * u;
        if 0 != ShotgunPellet(origin, end.as_mut_ptr(), ent) as libc::c_uint
            && 0 == hitClient as u64
        {
            hitClient = qtrue;
            (*(*ent).client).accuracy_hits += 1
        }
        i += 1
    }
}
//	VectorAdd( m->s.pos.trDelta, ent->client->ps.velocity, m->s.pos.trDelta );	// "real" physics
/*
======================================================================

SHOTGUN

======================================================================
*/
// DEFAULT_SHOTGUN_SPREAD and DEFAULT_SHOTGUN_COUNT	are in bg_public.h, because
// client predicts same spreads
#[no_mangle]
pub unsafe extern "C" fn ShotgunPellet(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut ent: *mut gentity_t,
) -> qboolean {
    let mut tr: trace_t = trace_t {
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
    let mut damage: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    let mut traceEnt: *mut gentity_t = 0 as *mut gentity_t;
    let mut tr_start: vec3_t = [0.; 3];
    let mut tr_end: vec3_t = [0.; 3];
    let mut hitClient: qboolean = qfalse;
    passent = (*ent).s.number;
    tr_start[0usize] = *start.offset(0isize);
    tr_start[1usize] = *start.offset(1isize);
    tr_start[2usize] = *start.offset(2isize);
    tr_end[0usize] = *end.offset(0isize);
    tr_end[1usize] = *end.offset(1isize);
    tr_end[2usize] = *end.offset(2isize);
    i = 0i32;
    if i < 10i32 {
        trap_Trace(
            &mut tr,
            tr_start.as_mut_ptr() as *const vec_t,
            0 as *const vec_t,
            0 as *const vec_t,
            tr_end.as_mut_ptr() as *const vec_t,
            passent,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        traceEnt = &mut g_entities[tr.entityNum as usize] as *mut gentity_t;
        if 0 != tr.surfaceFlags & 0x10i32 {
            return qfalse;
        }
        if 0 != (*traceEnt).takedamage as u64 {
            damage = (10i32 as libc::c_float * s_quadFactor) as libc::c_int;
            if 0 != LogAccuracyHit(traceEnt, ent) as u64 {
                hitClient = qtrue
            }
            G_Damage(
                traceEnt,
                ent,
                ent,
                forward.as_mut_ptr(),
                tr.endpos.as_mut_ptr(),
                damage,
                0i32,
                MOD_SHOTGUN as libc::c_int,
            );
            return hitClient;
        }
        return qfalse;
    }
    return qfalse;
}
/*
======================================================================

LIGHTNING GUN

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Weapon_LightningFire(mut ent: *mut gentity_t) {
    let mut tr: trace_t = trace_t {
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
    let mut end: vec3_t = [0.; 3];
    let mut traceEnt: *mut gentity_t = 0 as *mut gentity_t;
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut damage: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut passent: libc::c_int = 0;
    damage = (8i32 as libc::c_float * s_quadFactor) as libc::c_int;
    passent = (*ent).s.number;
    i = 0i32;
    if i < 10i32 {
        end[0usize] = muzzle[0usize] + forward[0usize] * 768i32 as libc::c_float;
        end[1usize] = muzzle[1usize] + forward[1usize] * 768i32 as libc::c_float;
        end[2usize] = muzzle[2usize] + forward[2usize] * 768i32 as libc::c_float;
        trap_Trace(
            &mut tr,
            muzzle.as_mut_ptr() as *const vec_t,
            0 as *const vec_t,
            0 as *const vec_t,
            end.as_mut_ptr() as *const vec_t,
            passent,
            1i32 | 0x2000000i32 | 0x4000000i32,
        );
        if tr.entityNum == (1i32 << 10i32) - 1i32 {
            return;
        }
        traceEnt = &mut g_entities[tr.entityNum as usize] as *mut gentity_t;
        if 0 != (*traceEnt).takedamage as u64 {
            if 0 != LogAccuracyHit(traceEnt, ent) as u64 {
                (*(*ent).client).accuracy_hits += 1
            }
            G_Damage(
                traceEnt,
                ent,
                ent,
                forward.as_mut_ptr(),
                tr.endpos.as_mut_ptr(),
                damage,
                0i32,
                MOD_LIGHTNING as libc::c_int,
            );
        }
        if 0 != (*traceEnt).takedamage as libc::c_uint && !(*traceEnt).client.is_null() {
            tent = G_TempEntity(tr.endpos.as_mut_ptr(), EV_MISSILE_HIT as libc::c_int);
            (*tent).s.otherEntityNum = (*traceEnt).s.number;
            (*tent).s.eventParm = DirToByte(tr.plane.normal.as_mut_ptr());
            (*tent).s.weapon = (*ent).s.weapon
        } else if 0 == tr.surfaceFlags & 0x10i32 {
            tent = G_TempEntity(tr.endpos.as_mut_ptr(), EV_MISSILE_MISS as libc::c_int);
            (*tent).s.eventParm = DirToByte(tr.plane.normal.as_mut_ptr())
        }
    };
}
/*
======================================================================

GAUNTLET

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Weapon_Gauntlet(mut ent: *mut gentity_t) {}
/*
===============
CalcMuzzlePointOrigin

set muzzle location relative to pivoting eye
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CalcMuzzlePointOrigin(
    mut ent: *mut gentity_t,
    mut origin: *mut vec_t,
    mut forward_0: *mut vec_t,
    mut right_0: *mut vec_t,
    mut up_0: *mut vec_t,
    mut muzzlePoint: *mut vec_t,
) {
    *muzzlePoint.offset(0isize) = (*ent).s.pos.trBase[0usize];
    *muzzlePoint.offset(1isize) = (*ent).s.pos.trBase[1usize];
    *muzzlePoint.offset(2isize) = (*ent).s.pos.trBase[2usize];
    let ref mut fresh1 = *muzzlePoint.offset(2isize);
    *fresh1 += (*(*ent).client).ps.viewheight as libc::c_float;
    *muzzlePoint.offset(0isize) =
        *muzzlePoint.offset(0isize) + *forward_0.offset(0isize) * 14i32 as libc::c_float;
    *muzzlePoint.offset(1isize) =
        *muzzlePoint.offset(1isize) + *forward_0.offset(1isize) * 14i32 as libc::c_float;
    *muzzlePoint.offset(2isize) =
        *muzzlePoint.offset(2isize) + *forward_0.offset(2isize) * 14i32 as libc::c_float;
    *muzzlePoint.offset(0isize) = *muzzlePoint.offset(0isize) as libc::c_int as vec_t;
    *muzzlePoint.offset(1isize) = *muzzlePoint.offset(1isize) as libc::c_int as vec_t;
    *muzzlePoint.offset(2isize) = *muzzlePoint.offset(2isize) as libc::c_int as vec_t;
}
/*
================
G_BounceProjectile
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_BounceProjectile(
    mut start: *mut vec_t,
    mut impact: *mut vec_t,
    mut dir: *mut vec_t,
    mut endout: *mut vec_t,
) {
    let mut v: vec3_t = [0.; 3];
    let mut newv: vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    v[0usize] = *impact.offset(0isize) - *start.offset(0isize);
    v[1usize] = *impact.offset(1isize) - *start.offset(1isize);
    v[2usize] = *impact.offset(2isize) - *start.offset(2isize);
    dot = v[0usize] * *dir.offset(0isize)
        + v[1usize] * *dir.offset(1isize)
        + v[2usize] * *dir.offset(2isize);
    newv[0usize] = v[0usize] + *dir.offset(0isize) * (-2i32 as libc::c_float * dot);
    newv[1usize] = v[1usize] + *dir.offset(1isize) * (-2i32 as libc::c_float * dot);
    newv[2usize] = v[2usize] + *dir.offset(2isize) * (-2i32 as libc::c_float * dot);
    VectorNormalize(newv.as_mut_ptr());
    *endout.offset(0isize) = *impact.offset(0isize) + newv[0usize] * 8192i32 as libc::c_float;
    *endout.offset(1isize) = *impact.offset(1isize) + newv[1usize] * 8192i32 as libc::c_float;
    *endout.offset(2isize) = *impact.offset(2isize) + newv[2usize] * 8192i32 as libc::c_float;
}
