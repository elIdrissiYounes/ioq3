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
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, weapon_t, ET_BEAM, ET_EVENTS,
    ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL,
    ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH,
    EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE,
    EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP,
    EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP,
    EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP,
    EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED, EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT,
    EV_MISSILE_HIT, EV_MISSILE_MISS, EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE,
    EV_OBELISKPAIN, EV_OBITUARY, EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT,
    EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD, EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK,
    EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL, EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16,
    EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND, EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME,
    EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO, EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0,
    EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11, EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14,
    EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3, EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6,
    EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9, EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH,
    EV_WATER_UNDER, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP,
    IT_POWERUP, IT_TEAM, IT_WEAPON, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
    trap_LinkEntity, trap_UnlinkEntity, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED,
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
    byte, cplane_s, cplane_t, entityState_s, entityState_t, fileHandle_t, playerState_s,
    playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t, usercmd_s, usercmd_t,
    vec3_t, vec_t, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{rand, sin};
extern crate libc;

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
// g_misc.c
//
#[no_mangle]
pub unsafe extern "C" fn TeleportPlayer(
    mut player: *mut gentity_t,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
) {
    let mut tent: *mut gentity_t = 0 as *mut gentity_t;
    let mut noAngles: qboolean = qfalse;
    noAngles = (*angles.offset(0isize) as libc::c_double > 999999.0f64) as libc::c_int as qboolean;
    if (*(*player).client).sess.sessionTeam as libc::c_uint
        != TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        tent = G_TempEntity(
            (*(*player).client).ps.origin.as_mut_ptr(),
            EV_PLAYER_TELEPORT_OUT as libc::c_int,
        );
        (*tent).s.clientNum = (*player).s.clientNum;
        tent = G_TempEntity(origin, EV_PLAYER_TELEPORT_IN as libc::c_int);
        (*tent).s.clientNum = (*player).s.clientNum
    }
    trap_UnlinkEntity(player);
    (*(*player).client).ps.origin[0usize] = *origin.offset(0isize);
    (*(*player).client).ps.origin[1usize] = *origin.offset(1isize);
    (*(*player).client).ps.origin[2usize] = *origin.offset(2isize);
    (*(*player).client).ps.origin[2usize] += 1i32 as libc::c_float;
    if 0 == noAngles as u64 {
        AngleVectors(
            angles as *const vec_t,
            (*(*player).client).ps.velocity.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        (*(*player).client).ps.velocity[0usize] =
            (*(*player).client).ps.velocity[0usize] * 400i32 as libc::c_float;
        (*(*player).client).ps.velocity[1usize] =
            (*(*player).client).ps.velocity[1usize] * 400i32 as libc::c_float;
        (*(*player).client).ps.velocity[2usize] =
            (*(*player).client).ps.velocity[2usize] * 400i32 as libc::c_float;
        (*(*player).client).ps.pm_time = 160i32;
        (*(*player).client).ps.pm_flags |= 64i32;
        SetClientViewAngle(player, angles);
    }
    (*(*player).client).ps.eFlags ^= 0x4i32;
    if (*(*player).client).sess.sessionTeam as libc::c_uint
        != TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        G_KillBox(player);
    }
    BG_PlayerStateToEntityState(&mut (*(*player).client).ps, &mut (*player).s, qtrue);
    (*player).r.currentOrigin[0usize] = (*(*player).client).ps.origin[0usize];
    (*player).r.currentOrigin[1usize] = (*(*player).client).ps.origin[1usize];
    (*player).r.currentOrigin[2usize] = (*(*player).client).ps.origin[2usize];
    if (*(*player).client).sess.sessionTeam as libc::c_uint
        != TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        trap_LinkEntity(player);
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
// g_misc.c
/*QUAKED func_group (0 0 0) ?
Used to group brushes together just for editor convenience.  They are turned into normal brushes by the utilities.
*/
/*QUAKED info_camp (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for calculations in the utilities (spotlights, etc), but removed during gameplay.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_info_camp(mut self_0: *mut gentity_t) {
    G_SetOrigin(self_0, (*self_0).s.origin.as_mut_ptr());
}
/*QUAKED info_null (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for calculations in the utilities (spotlights, etc), but removed during gameplay.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_info_null(mut self_0: *mut gentity_t) {
    G_FreeEntity(self_0);
}
/*QUAKED info_notnull (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for in-game calculation, like jumppad targets.
target_position does the same thing
*/
#[no_mangle]
pub unsafe extern "C" fn SP_info_notnull(mut self_0: *mut gentity_t) {
    G_SetOrigin(self_0, (*self_0).s.origin.as_mut_ptr());
}
/*QUAKED light (0 1 0) (-8 -8 -8) (8 8 8) linear
Non-displayed light.
"light" overrides the default 300 intensity.
Linear checbox gives linear falloff instead of inverse square
Lights pointed at a target will be spotlights.
"radius" overrides the default 64 unit radius of a spotlight at the target point.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_light(mut self_0: *mut gentity_t) {
    G_FreeEntity(self_0);
}
/*QUAKED misc_teleporter_dest (1 0 0) (-32 -32 -24) (32 32 -16)
Point teleporters at these.
Now that we don't have teleport destination pads, this is just
an info_notnull
*/
#[no_mangle]
pub unsafe extern "C" fn SP_misc_teleporter_dest(mut ent: *mut gentity_t) {}
//===========================================================
/*QUAKED misc_model (1 0 0) (-16 -16 -16) (16 16 16)
"model"		arbitrary .md3 file to display
*/
#[no_mangle]
pub unsafe extern "C" fn SP_misc_model(mut ent: *mut gentity_t) {
    G_FreeEntity(ent);
}
//===========================================================
#[no_mangle]
pub unsafe extern "C" fn locateCamera(mut ent: *mut gentity_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut target: *mut gentity_t = 0 as *mut gentity_t;
    let mut owner: *mut gentity_t = 0 as *mut gentity_t;
    owner = G_PickTarget((*ent).target);
    if owner.is_null() {
        G_Printf(
            b"Couldn\'t find target for misc_partal_surface\n\x00" as *const u8
                as *const libc::c_char,
        );
        G_FreeEntity(ent);
        return;
    }
    (*ent).r.ownerNum = (*owner).s.number;
    if 0 != (*owner).spawnflags & 1i32 {
        (*ent).s.frame = 25i32
    } else if 0 != (*owner).spawnflags & 2i32 {
        (*ent).s.frame = 75i32
    }
    if 0 != (*owner).spawnflags & 4i32 {
        (*ent).s.powerups = 0i32
    } else {
        (*ent).s.powerups = 1i32
    }
    (*ent).s.clientNum = (*owner).s.clientNum;
    (*ent).s.origin2[0usize] = (*owner).s.origin[0usize];
    (*ent).s.origin2[1usize] = (*owner).s.origin[1usize];
    (*ent).s.origin2[2usize] = (*owner).s.origin[2usize];
    target = G_PickTarget((*owner).target);
    if !target.is_null() {
        dir[0usize] = (*target).s.origin[0usize] - (*owner).s.origin[0usize];
        dir[1usize] = (*target).s.origin[1usize] - (*owner).s.origin[1usize];
        dir[2usize] = (*target).s.origin[2usize] - (*owner).s.origin[2usize];
        VectorNormalize(dir.as_mut_ptr());
    } else {
        G_SetMovedir((*owner).s.angles.as_mut_ptr(), dir.as_mut_ptr());
    }
    (*ent).s.eventParm = DirToByte(dir.as_mut_ptr());
}
/*QUAKED misc_portal_surface (0 0 1) (-8 -8 -8) (8 8 8)
The portal surface nearest this entity will show a view from the targeted misc_portal_camera, or a mirror view if untargeted.
This must be within 64 world units of the surface!
*/
#[no_mangle]
pub unsafe extern "C" fn SP_misc_portal_surface(mut ent: *mut gentity_t) {
    (*ent).r.mins[2usize] = 0i32 as vec_t;
    (*ent).r.mins[1usize] = (*ent).r.mins[2usize];
    (*ent).r.mins[0usize] = (*ent).r.mins[1usize];
    (*ent).r.maxs[2usize] = 0i32 as vec_t;
    (*ent).r.maxs[1usize] = (*ent).r.maxs[2usize];
    (*ent).r.maxs[0usize] = (*ent).r.maxs[1usize];
    trap_LinkEntity(ent);
    (*ent).r.svFlags = 0x40i32;
    (*ent).s.eType = ET_PORTAL as libc::c_int;
    if (*ent).target.is_null() {
        (*ent).s.origin2[0usize] = (*ent).s.origin[0usize];
        (*ent).s.origin2[1usize] = (*ent).s.origin[1usize];
        (*ent).s.origin2[2usize] = (*ent).s.origin[2usize]
    } else {
        (*ent).think = Some(locateCamera);
        (*ent).nextthink = level.time + 100i32
    };
}
/*QUAKED misc_portal_camera (0 0 1) (-8 -8 -8) (8 8 8) slowrotate fastrotate noswing
The target for a misc_portal_director.  You can set either angles or target another entity to determine the direction of view.
"roll" an angle modifier to orient the camera around the target vector;
*/
#[no_mangle]
pub unsafe extern "C" fn SP_misc_portal_camera(mut ent: *mut gentity_t) {
    let mut roll: libc::c_float = 0.;
    (*ent).r.mins[2usize] = 0i32 as vec_t;
    (*ent).r.mins[1usize] = (*ent).r.mins[2usize];
    (*ent).r.mins[0usize] = (*ent).r.mins[1usize];
    (*ent).r.maxs[2usize] = 0i32 as vec_t;
    (*ent).r.maxs[1usize] = (*ent).r.maxs[2usize];
    (*ent).r.maxs[0usize] = (*ent).r.maxs[1usize];
    trap_LinkEntity(ent);
    G_SpawnFloat(
        b"roll\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut roll,
    );
    (*ent).s.clientNum =
        (roll as libc::c_double / 360.0f64 * 256i32 as libc::c_double) as libc::c_int;
}
/*
======================================================================

  SHOOTERS

======================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn Use_Shooter(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    let mut dir: vec3_t = [0.; 3];
    let mut deg: libc::c_float = 0.;
    let mut up: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    if !(*ent).enemy.is_null() {
        dir[0usize] = (*(*ent).enemy).r.currentOrigin[0usize] - (*ent).s.origin[0usize];
        dir[1usize] = (*(*ent).enemy).r.currentOrigin[1usize] - (*ent).s.origin[1usize];
        dir[2usize] = (*(*ent).enemy).r.currentOrigin[2usize] - (*ent).s.origin[2usize];
        VectorNormalize(dir.as_mut_ptr());
    } else {
        dir[0usize] = (*ent).movedir[0usize];
        dir[1usize] = (*ent).movedir[1usize];
        dir[2usize] = (*ent).movedir[2usize]
    }
    PerpendicularVector(up.as_mut_ptr(), dir.as_mut_ptr() as *const vec_t);
    CrossProduct(
        up.as_mut_ptr() as *const vec_t,
        dir.as_mut_ptr() as *const vec_t,
        right.as_mut_ptr(),
    );
    deg = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * (*ent).random as libc::c_double) as libc::c_float;
    dir[0usize] = dir[0usize] + up[0usize] * deg;
    dir[1usize] = dir[1usize] + up[1usize] * deg;
    dir[2usize] = dir[2usize] + up[2usize] * deg;
    deg = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * (*ent).random as libc::c_double) as libc::c_float;
    dir[0usize] = dir[0usize] + right[0usize] * deg;
    dir[1usize] = dir[1usize] + right[1usize] * deg;
    dir[2usize] = dir[2usize] + right[2usize] * deg;
    VectorNormalize(dir.as_mut_ptr());
    match (*ent).s.weapon {
        4 => {
            fire_grenade(ent, (*ent).s.origin.as_mut_ptr(), dir.as_mut_ptr());
        }
        5 => {
            fire_rocket(ent, (*ent).s.origin.as_mut_ptr(), dir.as_mut_ptr());
        }
        8 => {
            fire_plasma(ent, (*ent).s.origin.as_mut_ptr(), dir.as_mut_ptr());
        }
        _ => {}
    }
    G_AddEvent(ent, EV_FIRE_WEAPON as libc::c_int, 0i32);
}
unsafe extern "C" fn InitShooter_Finish(mut ent: *mut gentity_t) {
    (*ent).enemy = G_PickTarget((*ent).target);
    (*ent).think = None;
    (*ent).nextthink = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn InitShooter(mut ent: *mut gentity_t, mut weapon: libc::c_int) {
    (*ent).use_0 = Some(Use_Shooter);
    (*ent).s.weapon = weapon;
    RegisterItem(BG_FindItemForWeapon(weapon as weapon_t));
    G_SetMovedir((*ent).s.angles.as_mut_ptr(), (*ent).movedir.as_mut_ptr());
    if 0. == (*ent).random {
        (*ent).random = 1.0f64 as libc::c_float
    }
    (*ent).random =
        sin(3.14159265358979323846f64 * (*ent).random as libc::c_double / 180i32 as libc::c_double)
            as libc::c_float;
    if !(*ent).target.is_null() {
        (*ent).think = Some(InitShooter_Finish);
        (*ent).nextthink = level.time + 500i32
    }
    trap_LinkEntity(ent);
}
/*QUAKED shooter_rocket (1 0 0) (-16 -16 -16) (16 16 16)
Fires at either the target or the current direction.
"random" the number of degrees of deviance from the taget. (1.0 default)
*/
#[no_mangle]
pub unsafe extern "C" fn SP_shooter_rocket(mut ent: *mut gentity_t) {
    InitShooter(ent, WP_ROCKET_LAUNCHER as libc::c_int);
}
/*QUAKED shooter_plasma (1 0 0) (-16 -16 -16) (16 16 16)
Fires at either the target or the current direction.
"random" is the number of degrees of deviance from the taget. (1.0 default)
*/
#[no_mangle]
pub unsafe extern "C" fn SP_shooter_plasma(mut ent: *mut gentity_t) {
    InitShooter(ent, WP_PLASMAGUN as libc::c_int);
}
/*QUAKED shooter_grenade (1 0 0) (-16 -16 -16) (16 16 16)
Fires at either the target or the current direction.
"random" is the number of degrees of deviance from the taget. (1.0 default)
*/
#[no_mangle]
pub unsafe extern "C" fn SP_shooter_grenade(mut ent: *mut gentity_t) {
    InitShooter(ent, WP_GRENADE_LAUNCHER as libc::c_int);
}
