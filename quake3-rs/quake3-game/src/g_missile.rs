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
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, unnamed_1, unnamed_2, ET_BEAM,
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
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, IT_AMMO, IT_ARMOR, IT_BAD,
    IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, MOD_BFG,
    MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE, MOD_GRENADE,
    MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA, MOD_PLASMA_SPLASH,
    MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME, MOD_SUICIDE,
    MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, TEAM_BLUE, TEAM_FREE,
    TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK,
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
    trap_LinkEntity, trap_Trace, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2,
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
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, entityState_s, entityState_t, fileHandle_t, playerState_s,
    playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t, usercmd_s, usercmd_t,
    vec3_t, vec_t, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{sqrt, strcmp};
extern crate libc;

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
// damage flags
// damage was indirect
// armour does not protect from this damage
// do not affect velocity, just view angles
// armor, shields, invulnerability, and godmode have no effect
//
// g_missile.c
//
#[no_mangle]
pub unsafe extern "C" fn G_RunMissile(mut ent: *mut gentity_t) {
    let mut origin: vec3_t = [0.; 3];
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
    let mut passent: libc::c_int = 0;
    BG_EvaluateTrajectory(&mut (*ent).s.pos, level.time, origin.as_mut_ptr());
    if !(*ent).target_ent.is_null() {
        passent = (*(*ent).target_ent).s.number
    } else {
        passent = (*ent).r.ownerNum
    }
    trap_Trace(
        &mut tr,
        (*ent).r.currentOrigin.as_mut_ptr() as *const vec_t,
        (*ent).r.mins.as_mut_ptr() as *const vec_t,
        (*ent).r.maxs.as_mut_ptr() as *const vec_t,
        origin.as_mut_ptr() as *const vec_t,
        passent,
        (*ent).clipmask,
    );
    if 0 != tr.startsolid as libc::c_uint || 0 != tr.allsolid as libc::c_uint {
        trap_Trace(
            &mut tr,
            (*ent).r.currentOrigin.as_mut_ptr() as *const vec_t,
            (*ent).r.mins.as_mut_ptr() as *const vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const vec_t,
            (*ent).r.currentOrigin.as_mut_ptr() as *const vec_t,
            passent,
            (*ent).clipmask,
        );
        tr.fraction = 0i32 as libc::c_float
    } else {
        (*ent).r.currentOrigin[0usize] = tr.endpos[0usize];
        (*ent).r.currentOrigin[1usize] = tr.endpos[1usize];
        (*ent).r.currentOrigin[2usize] = tr.endpos[2usize]
    }
    trap_LinkEntity(ent);
    if tr.fraction != 1i32 as libc::c_float {
        if 0 != tr.surfaceFlags & 0x10i32 {
            if !(*ent).parent.is_null()
                && !(*(*ent).parent).client.is_null()
                && (*(*(*ent).parent).client).hook == ent
            {
                (*(*(*ent).parent).client).hook = 0 as *mut gentity_t
            }
            G_FreeEntity(ent);
            return;
        }
        G_MissileImpact(ent, &mut tr);
        if (*ent).s.eType != ET_MISSILE as libc::c_int {
            return;
        }
    }
    G_RunThink(ent);
}
/*
================
G_MissileImpact
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_MissileImpact(mut ent: *mut gentity_t, mut trace: *mut trace_t) {
    let mut other: *mut gentity_t = 0 as *mut gentity_t;
    let mut hitClient: qboolean = qfalse;
    other = &mut *g_entities.as_mut_ptr().offset((*trace).entityNum as isize) as *mut gentity_t;
    if 0 == (*other).takedamage as u64 && 0 != (*ent).s.eFlags & (0x10i32 | 0x20i32) {
        G_BounceMissile(ent, trace);
        G_AddEvent(ent, EV_GRENADE_BOUNCE as libc::c_int, 0i32);
        return;
    }
    if 0 != (*other).takedamage as u64 {
        if 0 != (*ent).damage {
            let mut velocity: vec3_t = [0.; 3];
            if 0 != LogAccuracyHit(
                other,
                &mut *g_entities.as_mut_ptr().offset((*ent).r.ownerNum as isize),
            ) as u64
            {
                (*g_entities[(*ent).r.ownerNum as usize].client).accuracy_hits += 1;
                hitClient = qtrue
            }
            BG_EvaluateTrajectoryDelta(&mut (*ent).s.pos, level.time, velocity.as_mut_ptr());
            if VectorLength(velocity.as_mut_ptr() as *const vec_t) == 0i32 as libc::c_float {
                velocity[2usize] = 1i32 as vec_t
            }
            G_Damage(
                other,
                ent,
                &mut *g_entities.as_mut_ptr().offset((*ent).r.ownerNum as isize),
                velocity.as_mut_ptr(),
                (*ent).s.origin.as_mut_ptr(),
                (*ent).damage,
                0i32,
                (*ent).methodOfDeath,
            );
        }
    }
    if 0 == strcmp(
        (*ent).classname,
        b"hook\x00" as *const u8 as *const libc::c_char,
    ) {
        let mut nent: *mut gentity_t = 0 as *mut gentity_t;
        let mut v: vec3_t = [0.; 3];
        nent = G_Spawn();
        if 0 != (*other).takedamage as libc::c_uint && !(*other).client.is_null() {
            G_AddEvent(
                nent,
                EV_MISSILE_HIT as libc::c_int,
                DirToByte((*trace).plane.normal.as_mut_ptr()),
            );
            (*nent).s.otherEntityNum = (*other).s.number;
            (*ent).enemy = other;
            v[0usize] = ((*other).r.currentOrigin[0usize] as libc::c_double
                + ((*other).r.mins[0usize] + (*other).r.maxs[0usize]) as libc::c_double * 0.5f64)
                as vec_t;
            v[1usize] = ((*other).r.currentOrigin[1usize] as libc::c_double
                + ((*other).r.mins[1usize] + (*other).r.maxs[1usize]) as libc::c_double * 0.5f64)
                as vec_t;
            v[2usize] = ((*other).r.currentOrigin[2usize] as libc::c_double
                + ((*other).r.mins[2usize] + (*other).r.maxs[2usize]) as libc::c_double * 0.5f64)
                as vec_t;
            SnapVectorTowards(v.as_mut_ptr(), (*ent).s.pos.trBase.as_mut_ptr());
        } else {
            v[0usize] = (*trace).endpos[0usize];
            v[1usize] = (*trace).endpos[1usize];
            v[2usize] = (*trace).endpos[2usize];
            G_AddEvent(
                nent,
                EV_MISSILE_MISS as libc::c_int,
                DirToByte((*trace).plane.normal.as_mut_ptr()),
            );
            (*ent).enemy = 0 as *mut gentity_t
        }
        SnapVectorTowards(v.as_mut_ptr(), (*ent).s.pos.trBase.as_mut_ptr());
        (*nent).freeAfterEvent = qtrue;
        (*nent).s.eType = ET_GENERAL as libc::c_int;
        (*ent).s.eType = ET_GRAPPLE as libc::c_int;
        G_SetOrigin(ent, v.as_mut_ptr());
        G_SetOrigin(nent, v.as_mut_ptr());
        (*ent).think = Some(Weapon_HookThink);
        (*ent).nextthink = level.time + 100i32;
        (*(*(*ent).parent).client).ps.pm_flags |= 2048i32;
        (*(*(*ent).parent).client).ps.grapplePoint[0usize] = (*ent).r.currentOrigin[0usize];
        (*(*(*ent).parent).client).ps.grapplePoint[1usize] = (*ent).r.currentOrigin[1usize];
        (*(*(*ent).parent).client).ps.grapplePoint[2usize] = (*ent).r.currentOrigin[2usize];
        trap_LinkEntity(ent);
        trap_LinkEntity(nent);
        return;
    }
    if 0 != (*other).takedamage as libc::c_uint && !(*other).client.is_null() {
        G_AddEvent(
            ent,
            EV_MISSILE_HIT as libc::c_int,
            DirToByte((*trace).plane.normal.as_mut_ptr()),
        );
        (*ent).s.otherEntityNum = (*other).s.number
    } else if 0 != (*trace).surfaceFlags & 0x1000i32 {
        G_AddEvent(
            ent,
            EV_MISSILE_MISS_METAL as libc::c_int,
            DirToByte((*trace).plane.normal.as_mut_ptr()),
        );
    } else {
        G_AddEvent(
            ent,
            EV_MISSILE_MISS as libc::c_int,
            DirToByte((*trace).plane.normal.as_mut_ptr()),
        );
    }
    (*ent).freeAfterEvent = qtrue;
    (*ent).s.eType = ET_GENERAL as libc::c_int;
    SnapVectorTowards(
        (*trace).endpos.as_mut_ptr(),
        (*ent).s.pos.trBase.as_mut_ptr(),
    );
    G_SetOrigin(ent, (*trace).endpos.as_mut_ptr());
    if 0 != (*ent).splashDamage {
        if 0 != G_RadiusDamage(
            (*trace).endpos.as_mut_ptr(),
            (*ent).parent,
            (*ent).splashDamage as libc::c_float,
            (*ent).splashRadius as libc::c_float,
            other,
            (*ent).splashMethodOfDeath,
        ) as u64
        {
            if 0 == hitClient as u64 {
                (*g_entities[(*ent).r.ownerNum as usize].client).accuracy_hits += 1
            }
        }
    }
    trap_LinkEntity(ent);
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
/*
================
G_BounceMissile

================
*/
#[no_mangle]
pub unsafe extern "C" fn G_BounceMissile(mut ent: *mut gentity_t, mut trace: *mut trace_t) {
    let mut velocity: vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    let mut hitTime: libc::c_int = 0;
    hitTime = (level.previousTime as libc::c_float
        + (level.time - level.previousTime) as libc::c_float * (*trace).fraction)
        as libc::c_int;
    BG_EvaluateTrajectoryDelta(&mut (*ent).s.pos, hitTime, velocity.as_mut_ptr());
    dot = velocity[0usize] * (*trace).plane.normal[0usize]
        + velocity[1usize] * (*trace).plane.normal[1usize]
        + velocity[2usize] * (*trace).plane.normal[2usize];
    (*ent).s.pos.trDelta[0usize] =
        velocity[0usize] + (*trace).plane.normal[0usize] * (-2i32 as libc::c_float * dot);
    (*ent).s.pos.trDelta[1usize] =
        velocity[1usize] + (*trace).plane.normal[1usize] * (-2i32 as libc::c_float * dot);
    (*ent).s.pos.trDelta[2usize] =
        velocity[2usize] + (*trace).plane.normal[2usize] * (-2i32 as libc::c_float * dot);
    if 0 != (*ent).s.eFlags & 0x20i32 {
        (*ent).s.pos.trDelta[0usize] =
            ((*ent).s.pos.trDelta[0usize] as libc::c_double * 0.65f64) as vec_t;
        (*ent).s.pos.trDelta[1usize] =
            ((*ent).s.pos.trDelta[1usize] as libc::c_double * 0.65f64) as vec_t;
        (*ent).s.pos.trDelta[2usize] =
            ((*ent).s.pos.trDelta[2usize] as libc::c_double * 0.65f64) as vec_t;
        if (*trace).plane.normal[2usize] as libc::c_double > 0.2f64
            && VectorLength((*ent).s.pos.trDelta.as_mut_ptr() as *const vec_t)
                < 40i32 as libc::c_float
        {
            G_SetOrigin(ent, (*trace).endpos.as_mut_ptr());
            (*ent).s.time = level.time / 4i32;
            return;
        }
    }
    (*ent).r.currentOrigin[0usize] = (*ent).r.currentOrigin[0usize] + (*trace).plane.normal[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).r.currentOrigin[1usize] + (*trace).plane.normal[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).r.currentOrigin[2usize] + (*trace).plane.normal[2usize];
    (*ent).s.pos.trBase[0usize] = (*ent).r.currentOrigin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).r.currentOrigin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).r.currentOrigin[2usize];
    (*ent).s.pos.trTime = level.time;
}
#[no_mangle]
pub unsafe extern "C" fn fire_plasma(
    mut self_0: *mut gentity_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
) -> *mut gentity_t {
    let mut bolt: *mut gentity_t = 0 as *mut gentity_t;
    VectorNormalize(dir);
    bolt = G_Spawn();
    (*bolt).classname = b"plasma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = level.time + 10000i32;
    (*bolt).think = Some(G_ExplodeMissile);
    (*bolt).s.eType = ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80i32;
    (*bolt).s.weapon = WP_PLASMAGUN as libc::c_int;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 20i32;
    (*bolt).splashDamage = 15i32;
    (*bolt).splashRadius = 20i32;
    (*bolt).methodOfDeath = MOD_PLASMA as libc::c_int;
    (*bolt).splashMethodOfDeath = MOD_PLASMA_SPLASH as libc::c_int;
    (*bolt).clipmask = 1i32 | 0x2000000i32 | 0x4000000i32;
    (*bolt).target_ent = 0 as *mut gentity_t;
    (*bolt).s.pos.trType = TR_LINEAR;
    (*bolt).s.pos.trTime = level.time - 50i32;
    (*bolt).s.pos.trBase[0usize] = *start.offset(0isize);
    (*bolt).s.pos.trBase[1usize] = *start.offset(1isize);
    (*bolt).s.pos.trBase[2usize] = *start.offset(2isize);
    (*bolt).s.pos.trDelta[0usize] = *dir.offset(0isize) * 2000i32 as libc::c_float;
    (*bolt).s.pos.trDelta[1usize] = *dir.offset(1isize) * 2000i32 as libc::c_float;
    (*bolt).s.pos.trDelta[2usize] = *dir.offset(2isize) * 2000i32 as libc::c_float;
    (*bolt).s.pos.trDelta[0usize] = (*bolt).s.pos.trDelta[0usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[1usize] = (*bolt).s.pos.trDelta[1usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[2usize] = (*bolt).s.pos.trDelta[2usize] as libc::c_int as vec_t;
    (*bolt).r.currentOrigin[0usize] = *start.offset(0isize);
    (*bolt).r.currentOrigin[1usize] = *start.offset(1isize);
    (*bolt).r.currentOrigin[2usize] = *start.offset(2isize);
    return bolt;
}
/*
================
G_ExplodeMissile

Explode a missile without an impact
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_ExplodeMissile(mut ent: *mut gentity_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    BG_EvaluateTrajectory(&mut (*ent).s.pos, level.time, origin.as_mut_ptr());
    origin[0usize] = origin[0usize] as libc::c_int as vec_t;
    origin[1usize] = origin[1usize] as libc::c_int as vec_t;
    origin[2usize] = origin[2usize] as libc::c_int as vec_t;
    G_SetOrigin(ent, origin.as_mut_ptr());
    dir[1usize] = 0i32 as vec_t;
    dir[0usize] = dir[1usize];
    dir[2usize] = 1i32 as vec_t;
    (*ent).s.eType = ET_GENERAL as libc::c_int;
    G_AddEvent(
        ent,
        EV_MISSILE_MISS as libc::c_int,
        DirToByte(dir.as_mut_ptr()),
    );
    (*ent).freeAfterEvent = qtrue;
    if 0 != (*ent).splashDamage {
        if 0 != G_RadiusDamage(
            (*ent).r.currentOrigin.as_mut_ptr(),
            (*ent).parent,
            (*ent).splashDamage as libc::c_float,
            (*ent).splashRadius as libc::c_float,
            ent,
            (*ent).splashMethodOfDeath,
        ) as u64
        {
            (*g_entities[(*ent).r.ownerNum as usize].client).accuracy_hits += 1
        }
    }
    trap_LinkEntity(ent);
}
#[no_mangle]
pub unsafe extern "C" fn fire_grenade(
    mut self_0: *mut gentity_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
) -> *mut gentity_t {
    let mut bolt: *mut gentity_t = 0 as *mut gentity_t;
    VectorNormalize(dir);
    bolt = G_Spawn();
    (*bolt).classname = b"grenade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = level.time + 2500i32;
    (*bolt).think = Some(G_ExplodeMissile);
    (*bolt).s.eType = ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80i32;
    (*bolt).s.weapon = WP_GRENADE_LAUNCHER as libc::c_int;
    (*bolt).s.eFlags = 0x20i32;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 100i32;
    (*bolt).splashDamage = 100i32;
    (*bolt).splashRadius = 150i32;
    (*bolt).methodOfDeath = MOD_GRENADE as libc::c_int;
    (*bolt).splashMethodOfDeath = MOD_GRENADE_SPLASH as libc::c_int;
    (*bolt).clipmask = 1i32 | 0x2000000i32 | 0x4000000i32;
    (*bolt).target_ent = 0 as *mut gentity_t;
    (*bolt).s.pos.trType = TR_GRAVITY;
    (*bolt).s.pos.trTime = level.time - 50i32;
    (*bolt).s.pos.trBase[0usize] = *start.offset(0isize);
    (*bolt).s.pos.trBase[1usize] = *start.offset(1isize);
    (*bolt).s.pos.trBase[2usize] = *start.offset(2isize);
    (*bolt).s.pos.trDelta[0usize] = *dir.offset(0isize) * 700i32 as libc::c_float;
    (*bolt).s.pos.trDelta[1usize] = *dir.offset(1isize) * 700i32 as libc::c_float;
    (*bolt).s.pos.trDelta[2usize] = *dir.offset(2isize) * 700i32 as libc::c_float;
    (*bolt).s.pos.trDelta[0usize] = (*bolt).s.pos.trDelta[0usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[1usize] = (*bolt).s.pos.trDelta[1usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[2usize] = (*bolt).s.pos.trDelta[2usize] as libc::c_int as vec_t;
    (*bolt).r.currentOrigin[0usize] = *start.offset(0isize);
    (*bolt).r.currentOrigin[1usize] = *start.offset(1isize);
    (*bolt).r.currentOrigin[2usize] = *start.offset(2isize);
    return bolt;
}
#[no_mangle]
pub unsafe extern "C" fn fire_rocket(
    mut self_0: *mut gentity_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
) -> *mut gentity_t {
    let mut bolt: *mut gentity_t = 0 as *mut gentity_t;
    VectorNormalize(dir);
    bolt = G_Spawn();
    (*bolt).classname = b"rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = level.time + 15000i32;
    (*bolt).think = Some(G_ExplodeMissile);
    (*bolt).s.eType = ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80i32;
    (*bolt).s.weapon = WP_ROCKET_LAUNCHER as libc::c_int;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 100i32;
    (*bolt).splashDamage = 100i32;
    (*bolt).splashRadius = 120i32;
    (*bolt).methodOfDeath = MOD_ROCKET as libc::c_int;
    (*bolt).splashMethodOfDeath = MOD_ROCKET_SPLASH as libc::c_int;
    (*bolt).clipmask = 1i32 | 0x2000000i32 | 0x4000000i32;
    (*bolt).target_ent = 0 as *mut gentity_t;
    (*bolt).s.pos.trType = TR_LINEAR;
    (*bolt).s.pos.trTime = level.time - 50i32;
    (*bolt).s.pos.trBase[0usize] = *start.offset(0isize);
    (*bolt).s.pos.trBase[1usize] = *start.offset(1isize);
    (*bolt).s.pos.trBase[2usize] = *start.offset(2isize);
    (*bolt).s.pos.trDelta[0usize] = *dir.offset(0isize) * 900i32 as libc::c_float;
    (*bolt).s.pos.trDelta[1usize] = *dir.offset(1isize) * 900i32 as libc::c_float;
    (*bolt).s.pos.trDelta[2usize] = *dir.offset(2isize) * 900i32 as libc::c_float;
    (*bolt).s.pos.trDelta[0usize] = (*bolt).s.pos.trDelta[0usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[1usize] = (*bolt).s.pos.trDelta[1usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[2usize] = (*bolt).s.pos.trDelta[2usize] as libc::c_int as vec_t;
    (*bolt).r.currentOrigin[0usize] = *start.offset(0isize);
    (*bolt).r.currentOrigin[1usize] = *start.offset(1isize);
    (*bolt).r.currentOrigin[2usize] = *start.offset(2isize);
    return bolt;
}
#[no_mangle]
pub unsafe extern "C" fn fire_bfg(
    mut self_0: *mut gentity_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
) -> *mut gentity_t {
    let mut bolt: *mut gentity_t = 0 as *mut gentity_t;
    VectorNormalize(dir);
    bolt = G_Spawn();
    (*bolt).classname = b"bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bolt).nextthink = level.time + 10000i32;
    (*bolt).think = Some(G_ExplodeMissile);
    (*bolt).s.eType = ET_MISSILE as libc::c_int;
    (*bolt).r.svFlags = 0x80i32;
    (*bolt).s.weapon = WP_BFG as libc::c_int;
    (*bolt).r.ownerNum = (*self_0).s.number;
    (*bolt).parent = self_0;
    (*bolt).damage = 100i32;
    (*bolt).splashDamage = 100i32;
    (*bolt).splashRadius = 120i32;
    (*bolt).methodOfDeath = MOD_BFG as libc::c_int;
    (*bolt).splashMethodOfDeath = MOD_BFG_SPLASH as libc::c_int;
    (*bolt).clipmask = 1i32 | 0x2000000i32 | 0x4000000i32;
    (*bolt).target_ent = 0 as *mut gentity_t;
    (*bolt).s.pos.trType = TR_LINEAR;
    (*bolt).s.pos.trTime = level.time - 50i32;
    (*bolt).s.pos.trBase[0usize] = *start.offset(0isize);
    (*bolt).s.pos.trBase[1usize] = *start.offset(1isize);
    (*bolt).s.pos.trBase[2usize] = *start.offset(2isize);
    (*bolt).s.pos.trDelta[0usize] = *dir.offset(0isize) * 2000i32 as libc::c_float;
    (*bolt).s.pos.trDelta[1usize] = *dir.offset(1isize) * 2000i32 as libc::c_float;
    (*bolt).s.pos.trDelta[2usize] = *dir.offset(2isize) * 2000i32 as libc::c_float;
    (*bolt).s.pos.trDelta[0usize] = (*bolt).s.pos.trDelta[0usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[1usize] = (*bolt).s.pos.trDelta[1usize] as libc::c_int as vec_t;
    (*bolt).s.pos.trDelta[2usize] = (*bolt).s.pos.trDelta[2usize] as libc::c_int as vec_t;
    (*bolt).r.currentOrigin[0usize] = *start.offset(0isize);
    (*bolt).r.currentOrigin[1usize] = *start.offset(1isize);
    (*bolt).r.currentOrigin[2usize] = *start.offset(2isize);
    return bolt;
}
#[no_mangle]
pub unsafe extern "C" fn fire_grapple(
    mut self_0: *mut gentity_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
) -> *mut gentity_t {
    let mut hook: *mut gentity_t = 0 as *mut gentity_t;
    VectorNormalize(dir);
    hook = G_Spawn();
    (*hook).classname = b"hook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*hook).nextthink = level.time + 10000i32;
    (*hook).think = Some(Weapon_HookFree);
    (*hook).s.eType = ET_MISSILE as libc::c_int;
    (*hook).r.svFlags = 0x80i32;
    (*hook).s.weapon = WP_GRAPPLING_HOOK as libc::c_int;
    (*hook).r.ownerNum = (*self_0).s.number;
    (*hook).methodOfDeath = MOD_GRAPPLE as libc::c_int;
    (*hook).clipmask = 1i32 | 0x2000000i32 | 0x4000000i32;
    (*hook).parent = self_0;
    (*hook).target_ent = 0 as *mut gentity_t;
    (*hook).s.pos.trType = TR_LINEAR;
    (*hook).s.pos.trTime = level.time - 50i32;
    (*hook).s.otherEntityNum = (*self_0).s.number;
    (*hook).s.pos.trBase[0usize] = *start.offset(0isize);
    (*hook).s.pos.trBase[1usize] = *start.offset(1isize);
    (*hook).s.pos.trBase[2usize] = *start.offset(2isize);
    (*hook).s.pos.trDelta[0usize] = *dir.offset(0isize) * 800i32 as libc::c_float;
    (*hook).s.pos.trDelta[1usize] = *dir.offset(1isize) * 800i32 as libc::c_float;
    (*hook).s.pos.trDelta[2usize] = *dir.offset(2isize) * 800i32 as libc::c_float;
    (*hook).s.pos.trDelta[0usize] = (*hook).s.pos.trDelta[0usize] as libc::c_int as vec_t;
    (*hook).s.pos.trDelta[1usize] = (*hook).s.pos.trDelta[1usize] as libc::c_int as vec_t;
    (*hook).s.pos.trDelta[2usize] = (*hook).s.pos.trDelta[2usize] as libc::c_int as vec_t;
    (*hook).r.currentOrigin[0usize] = *start.offset(0isize);
    (*hook).r.currentOrigin[1usize] = *start.offset(1isize);
    (*hook).r.currentOrigin[2usize] = *start.offset(2isize);
    (*(*self_0).client).hook = hook;
    return hook;
}
