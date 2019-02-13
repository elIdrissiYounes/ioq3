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
    gitem_s, gitem_t, itemType_t, team_t, unnamed_0, unnamed_1, unnamed_2, unnamed_3, ET_BEAM,
    ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER,
    ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, IT_AMMO, IT_ARMOR,
    IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, MOD_BFG,
    MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE, MOD_GRENADE,
    MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA, MOD_PLASMA_SPLASH,
    MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME, MOD_SUICIDE,
    MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, PM_DEAD, PM_FREEZE,
    PM_INTERMISSION, PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN,
    PW_BATTLESUIT, PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS,
    PW_INVULNERABILITY, PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN,
    PW_SCOUT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
    trap_LinkEntity, trap_SetBrushModel, trap_UnlinkEntity, CON_CONNECTED, CON_CONNECTING,
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
    unnamed, usercmd_s, usercmd_t, vec3_t, vec_t, vmCvar_t, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY,
    CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, TR_GRAVITY, TR_INTERPOLATE,
    TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{rand, sqrt};

unsafe extern "C" fn VectorCompare(mut v1: *const vec_t, mut v2: *const vec_t) -> libc::c_int {
    if *v1.offset(0isize) != *v2.offset(0isize)
        || *v1.offset(1isize) != *v2.offset(1isize)
        || *v1.offset(2isize) != *v2.offset(2isize)
    {
        return 0i32;
    }
    return 1i32;
}
//
// g_trigger.c
//
#[no_mangle]
pub unsafe extern "C" fn trigger_teleporter_touch(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    let mut dest: *mut gentity_t = 0 as *mut gentity_t;
    if (*other).client.is_null() {
        return;
    }
    if (*(*other).client).ps.pm_type == PM_DEAD as libc::c_int {
        return;
    }
    if 0 != (*self_0).spawnflags & 1i32
        && (*(*other).client).sess.sessionTeam as libc::c_uint
            != TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        return;
    }
    dest = G_PickTarget((*self_0).target);
    if dest.is_null() {
        G_Printf(
            b"Couldn\'t find teleporter destination\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    TeleportPlayer(
        other,
        (*dest).s.origin.as_mut_ptr(),
        (*dest).s.angles.as_mut_ptr(),
    );
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
pub unsafe extern "C" fn InitTrigger(mut self_0: *mut gentity_t) {
    if 0 == VectorCompare(
        (*self_0).s.angles.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
    ) {
        G_SetMovedir(
            (*self_0).s.angles.as_mut_ptr(),
            (*self_0).movedir.as_mut_ptr(),
        );
    }
    trap_SetBrushModel(self_0, (*self_0).model);
    (*self_0).r.contents = 0x40000000i32;
    (*self_0).r.svFlags = 0x1i32;
}
// the wait time has passed, so set back up for another activation
#[no_mangle]
pub unsafe extern "C" fn multi_wait(mut ent: *mut gentity_t) {
    (*ent).nextthink = 0i32;
}
// the trigger was just activated
// ent->activator should be set to the activator so it can be held through a delay
// so wait for the delay time before firing
#[no_mangle]
pub unsafe extern "C" fn multi_trigger(mut ent: *mut gentity_t, mut activator: *mut gentity_t) {
    (*ent).activator = activator;
    if 0 != (*ent).nextthink {
        return;
    }
    if !(*activator).client.is_null() {
        if 0 != (*ent).spawnflags & 1i32
            && (*(*activator).client).sess.sessionTeam as libc::c_uint
                != TEAM_RED as libc::c_int as libc::c_uint
        {
            return;
        }
        if 0 != (*ent).spawnflags & 2i32
            && (*(*activator).client).sess.sessionTeam as libc::c_uint
                != TEAM_BLUE as libc::c_int as libc::c_uint
        {
            return;
        }
    }
    G_UseTargets(ent, (*ent).activator);
    if (*ent).wait > 0i32 as libc::c_float {
        (*ent).think = Some(multi_wait);
        (*ent).nextthink = (level.time as libc::c_double
            + ((*ent).wait as libc::c_double
                + (*ent).random as libc::c_double
                    * (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64)))
                * 1000i32 as libc::c_double) as libc::c_int
    } else {
        (*ent).touch = None;
        (*ent).nextthink = level.time + 100i32;
        (*ent).think = Some(G_FreeEntity)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Use_Multi(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    multi_trigger(ent, activator);
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Multi(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    multi_trigger(self_0, other);
}
/*QUAKED trigger_multiple (.5 .5 .5) ? RED_ONLY BLUE_ONLY
"wait" : Seconds between triggerings, 0.5 default, -1 = one time only.
"random"	wait variance, default is 0
Variable sized repeatable trigger.  Must be targeted at one or more entities.
so, the basic time between firing is a random time between
(wait - random) and (wait + random)
*/
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_multiple(mut ent: *mut gentity_t) {
    G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"0.5\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    );
    G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).random,
    );
    if (*ent).random >= (*ent).wait && (*ent).wait >= 0i32 as libc::c_float {
        (*ent).random = (*ent).wait - 100i32 as libc::c_float;
        G_Printf(b"trigger_multiple has random >= wait\n\x00" as *const u8 as *const libc::c_char);
    }
    (*ent).touch = Some(Touch_Multi);
    (*ent).use_0 = Some(Use_Multi);
    InitTrigger(ent);
    trap_LinkEntity(ent);
}
/*
==============================================================================

trigger_always

==============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn trigger_always_think(mut ent: *mut gentity_t) {
    G_UseTargets(ent, ent);
    G_FreeEntity(ent);
}
/*QUAKED trigger_always (.5 .5 .5) (-8 -8 -8) (8 8 8)
This trigger will always fire.  It is activated by the world.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_always(mut ent: *mut gentity_t) {
    (*ent).nextthink = level.time + 300i32;
    (*ent).think = Some(trigger_always_think);
}
/*
==============================================================================

trigger_push

==============================================================================
*/
#[no_mangle]
pub unsafe extern "C" fn trigger_push_touch(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    BG_TouchJumpPad(&mut (*(*other).client).ps, &mut (*self_0).s);
}
/*
=================
AimAtTarget

Calculate origin2 so the target apogee will be hit
=================
*/
#[no_mangle]
pub unsafe extern "C" fn AimAtTarget(mut self_0: *mut gentity_t) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut origin: vec3_t = [0.; 3];
    let mut height: libc::c_float = 0.;
    let mut gravity: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut forward: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    origin[0usize] = (*self_0).r.absmin[0usize] + (*self_0).r.absmax[0usize];
    origin[1usize] = (*self_0).r.absmin[1usize] + (*self_0).r.absmax[1usize];
    origin[2usize] = (*self_0).r.absmin[2usize] + (*self_0).r.absmax[2usize];
    origin[0usize] = (origin[0usize] as libc::c_double * 0.5f64) as vec_t;
    origin[1usize] = (origin[1usize] as libc::c_double * 0.5f64) as vec_t;
    origin[2usize] = (origin[2usize] as libc::c_double * 0.5f64) as vec_t;
    ent = G_PickTarget((*self_0).target);
    if ent.is_null() {
        G_FreeEntity(self_0);
        return;
    }
    height = (*ent).s.origin[2usize] - origin[2usize];
    gravity = g_gravity.value;
    time = sqrt(height as libc::c_double / (0.5f64 * gravity as libc::c_double)) as libc::c_float;
    if 0. == time {
        G_FreeEntity(self_0);
        return;
    }
    (*self_0).s.origin2[0usize] = (*ent).s.origin[0usize] - origin[0usize];
    (*self_0).s.origin2[1usize] = (*ent).s.origin[1usize] - origin[1usize];
    (*self_0).s.origin2[2usize] = (*ent).s.origin[2usize] - origin[2usize];
    (*self_0).s.origin2[2usize] = 0i32 as vec_t;
    dist = VectorNormalize((*self_0).s.origin2.as_mut_ptr());
    forward = dist / time;
    (*self_0).s.origin2[0usize] = (*self_0).s.origin2[0usize] * forward;
    (*self_0).s.origin2[1usize] = (*self_0).s.origin2[1usize] * forward;
    (*self_0).s.origin2[2usize] = (*self_0).s.origin2[2usize] * forward;
    (*self_0).s.origin2[2usize] = time * gravity;
}
/*QUAKED trigger_push (.5 .5 .5) ?
Must point at a target_position, which will be the apex of the leap.
This will be client side predicted, unlike target_push
*/
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_push(mut self_0: *mut gentity_t) {
    InitTrigger(self_0);
    (*self_0).r.svFlags &= !0x1i32;
    G_SoundIndex(
        b"sound/world/jumppad.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).s.eType = ET_PUSH_TRIGGER as libc::c_int;
    (*self_0).touch = Some(trigger_push_touch);
    (*self_0).think = Some(AimAtTarget);
    (*self_0).nextthink = level.time + 100i32;
    trap_LinkEntity(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn Use_target_push(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    if (*activator).client.is_null() {
        return;
    }
    if (*(*activator).client).ps.pm_type != PM_NORMAL as libc::c_int {
        return;
    }
    if 0 != (*(*activator).client).ps.powerups[PW_FLIGHT as libc::c_int as usize] {
        return;
    }
    (*(*activator).client).ps.velocity[0usize] = (*self_0).s.origin2[0usize];
    (*(*activator).client).ps.velocity[1usize] = (*self_0).s.origin2[1usize];
    (*(*activator).client).ps.velocity[2usize] = (*self_0).s.origin2[2usize];
    if (*activator).fly_sound_debounce_time < level.time {
        (*activator).fly_sound_debounce_time = level.time + 1500i32;
        G_Sound(activator, CHAN_AUTO as libc::c_int, (*self_0).noise_index);
    };
}
/*QUAKED target_push (.5 .5 .5) (-8 -8 -8) (8 8 8) bouncepad
Pushes the activator in the direction.of angle, or towards a target apex.
"speed"		defaults to 1000
if "bouncepad", play bounce noise instead of windfly
*/
#[no_mangle]
pub unsafe extern "C" fn SP_target_push(mut self_0: *mut gentity_t) {
    if 0. == (*self_0).speed {
        (*self_0).speed = 1000i32 as libc::c_float
    }
    G_SetMovedir(
        (*self_0).s.angles.as_mut_ptr(),
        (*self_0).s.origin2.as_mut_ptr(),
    );
    (*self_0).s.origin2[0usize] = (*self_0).s.origin2[0usize] * (*self_0).speed;
    (*self_0).s.origin2[1usize] = (*self_0).s.origin2[1usize] * (*self_0).speed;
    (*self_0).s.origin2[2usize] = (*self_0).s.origin2[2usize] * (*self_0).speed;
    if 0 != (*self_0).spawnflags & 1i32 {
        (*self_0).noise_index = G_SoundIndex(
            b"sound/world/jumppad.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    } else {
        (*self_0).noise_index = G_SoundIndex(
            b"sound/misc/windfly.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        )
    }
    if !(*self_0).target.is_null() {
        (*self_0).r.absmin[0usize] = (*self_0).s.origin[0usize];
        (*self_0).r.absmin[1usize] = (*self_0).s.origin[1usize];
        (*self_0).r.absmin[2usize] = (*self_0).s.origin[2usize];
        (*self_0).r.absmax[0usize] = (*self_0).s.origin[0usize];
        (*self_0).r.absmax[1usize] = (*self_0).s.origin[1usize];
        (*self_0).r.absmax[2usize] = (*self_0).s.origin[2usize];
        (*self_0).think = Some(AimAtTarget);
        (*self_0).nextthink = level.time + 100i32
    }
    (*self_0).use_0 = Some(Use_target_push);
}
/*QUAKED trigger_teleport (.5 .5 .5) ? SPECTATOR
Allows client side prediction of teleportation events.
Must point at a target_position, which will be the teleport destination.

If spectator is set, only spectators can use this teleport
Spectator teleporters are not normally placed in the editor, but are created
automatically near doors to allow spectators to move through them
*/
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_teleport(mut self_0: *mut gentity_t) {
    InitTrigger(self_0);
    if 0 != (*self_0).spawnflags & 1i32 {
        (*self_0).r.svFlags |= 0x1i32
    } else {
        (*self_0).r.svFlags &= !0x1i32
    }
    G_SoundIndex(
        b"sound/world/jumppad.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).s.eType = ET_TELEPORT_TRIGGER as libc::c_int;
    (*self_0).touch = Some(trigger_teleporter_touch);
    trap_LinkEntity(self_0);
}
/*
==============================================================================

trigger_hurt

==============================================================================
*/
/*QUAKED trigger_hurt (.5 .5 .5) ? START_OFF - SILENT NO_PROTECTION SLOW
Any entity that touches this will be hurt.
It does dmg points of damage each server frame
Targeting the trigger will toggle its on / off state.

SILENT			suppresses playing the sound
SLOW			changes the damage rate to once per second
NO_PROTECTION	*nothing* stops the damage

"dmg"			default 5 (whole numbers only)

*/
#[no_mangle]
pub unsafe extern "C" fn hurt_use(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    if 0 != (*self_0).r.linked as u64 {
        trap_UnlinkEntity(self_0);
    } else {
        trap_LinkEntity(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn hurt_touch(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    let mut dflags: libc::c_int = 0;
    if 0 == (*other).takedamage as u64 {
        return;
    }
    if (*self_0).timestamp > level.time {
        return;
    }
    if 0 != (*self_0).spawnflags & 16i32 {
        (*self_0).timestamp = level.time + 1000i32
    } else {
        (*self_0).timestamp = level.time + 100i32
    }
    if 0 == (*self_0).spawnflags & 4i32 {
        G_Sound(other, CHAN_AUTO as libc::c_int, (*self_0).noise_index);
    }
    if 0 != (*self_0).spawnflags & 8i32 {
        dflags = 0x8i32
    } else {
        dflags = 0i32
    }
    G_Damage(
        other,
        self_0,
        self_0,
        0 as *mut vec_t,
        0 as *mut vec_t,
        (*self_0).damage,
        dflags,
        MOD_TRIGGER_HURT as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_hurt(mut self_0: *mut gentity_t) {
    InitTrigger(self_0);
    (*self_0).noise_index = G_SoundIndex(
        b"sound/world/electro.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).touch = Some(hurt_touch);
    if 0 == (*self_0).damage {
        (*self_0).damage = 5i32
    }
    (*self_0).use_0 = Some(hurt_use);
    if 0 != (*self_0).spawnflags & 1i32 {
        trap_UnlinkEntity(self_0);
    } else {
        trap_LinkEntity(self_0);
    };
}
/*
==============================================================================

timer

==============================================================================
*/
/*QUAKED func_timer (0.3 0.1 0.6) (-8 -8 -8) (8 8 8) START_ON
This should be renamed trigger_timer...
Repeatedly fires its targets.
Can be turned on or off by using.

"wait"			base time between triggering all targets, default is 1
"random"		wait variance, default is 0
so, the basic time between firing is a random time between
(wait - random) and (wait + random)

*/
#[no_mangle]
pub unsafe extern "C" fn func_timer_think(mut self_0: *mut gentity_t) {
    G_UseTargets(self_0, (*self_0).activator);
    (*self_0).nextthink = (level.time as libc::c_double
        + 1000i32 as libc::c_double
            * ((*self_0).wait as libc::c_double
                + 2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64)
                    * (*self_0).random as libc::c_double)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_timer_use(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    (*self_0).activator = activator;
    if 0 != (*self_0).nextthink {
        (*self_0).nextthink = 0i32;
        return;
    }
    func_timer_think(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_timer(mut self_0: *mut gentity_t) {
    G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        &mut (*self_0).random,
    );
    G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        &mut (*self_0).wait,
    );
    (*self_0).use_0 = Some(func_timer_use);
    (*self_0).think = Some(func_timer_think);
    if (*self_0).random >= (*self_0).wait {
        (*self_0).random = (*self_0).wait - 100i32 as libc::c_float;
        G_Printf(
            b"func_timer at %s has random >= wait\n\x00" as *const u8 as *const libc::c_char,
            vtos((*self_0).s.origin.as_mut_ptr() as *const vec_t),
        );
    }
    if 0 != (*self_0).spawnflags & 1i32 {
        (*self_0).nextthink = level.time + 100i32;
        (*self_0).activator = self_0
    }
    (*self_0).r.svFlags = 0x1i32;
}
