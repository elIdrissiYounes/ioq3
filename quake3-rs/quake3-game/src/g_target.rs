#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           libc,
           ptr_wrapping_offset_from)]
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
    MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, PW_AMMOREGEN,
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
    trap_LinkEntity, trap_SendServerCommand, trap_SetConfigstring, trap_Trace, trap_UnlinkEntity,
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
use g_team::{
    OnSameTeam, SP_team_CTF_blueplayer, SP_team_CTF_bluespawn, SP_team_CTF_redplayer,
    SP_team_CTF_redspawn, Team_CheckDroppedItem, Team_ReturnFlag,
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
use g_variadic_h::{G_Error, G_Printf};
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
    va, vec3_t, vec_t, Com_sprintf, Q_stricmp, Q_strncpyz, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stddef_h::size_t;
use stdlib::{memset, rand, strstr};
extern crate libc;

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
//==========================================================
/*QUAKED target_give (1 0 0) (-8 -8 -8) (8 8 8)
Gives the activator all the items pointed to.
*/
#[no_mangle]
pub unsafe extern "C" fn Use_Target_Give(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    let mut t: *mut gentity_t = 0 as *mut gentity_t;
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
    if (*activator).client.is_null() {
        return;
    }
    if (*ent).target.is_null() {
        return;
    }
    memset(
        &mut trace as *mut trace_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<trace_t>() as libc::c_ulong,
    );
    t = 0 as *mut gentity_t;
    loop {
        t = G_Find(
            t,
            &mut (*(0 as *mut gentity_t)).targetname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            (*ent).target,
        );
        if t.is_null() {
            break;
        }
        if (*t).item.is_null() {
            continue;
        }
        Touch_Item(t, activator, &mut trace);
        (*t).nextthink = 0i32;
        trap_UnlinkEntity(t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_give(mut ent: *mut gentity_t) {
    (*ent).use_0 = Some(Use_Target_Give);
}
//==========================================================
/*QUAKED target_remove_powerups (1 0 0) (-8 -8 -8) (8 8 8)
takes away all the activators powerups.
Used to drop flight powerups into death puts.
*/
#[no_mangle]
pub unsafe extern "C" fn Use_target_remove_powerups(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    if (*activator).client.is_null() {
        return;
    }
    if 0 != (*(*activator).client).ps.powerups[PW_REDFLAG as libc::c_int as usize] {
        Team_ReturnFlag(TEAM_RED as libc::c_int);
    } else if 0 != (*(*activator).client).ps.powerups[PW_BLUEFLAG as libc::c_int as usize] {
        Team_ReturnFlag(TEAM_BLUE as libc::c_int);
    } else if 0 != (*(*activator).client).ps.powerups[PW_NEUTRALFLAG as libc::c_int as usize] {
        Team_ReturnFlag(TEAM_FREE as libc::c_int);
    }
    memset(
        (*(*activator).client).ps.powerups.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_remove_powerups(mut ent: *mut gentity_t) {
    (*ent).use_0 = Some(Use_target_remove_powerups);
}
//==========================================================
/*QUAKED target_delay (1 0 0) (-8 -8 -8) (8 8 8)
"wait" seconds to pause before firing targets.
"random" delay variance, total delay = delay +/- random seconds
*/
#[no_mangle]
pub unsafe extern "C" fn Think_Target_Delay(mut ent: *mut gentity_t) {
    G_UseTargets(ent, (*ent).activator);
}
#[no_mangle]
pub unsafe extern "C" fn Use_Target_Delay(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    (*ent).nextthink = (level.time as libc::c_double
        + ((*ent).wait as libc::c_double
            + (*ent).random as libc::c_double
                * (2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64)))
            * 1000i32 as libc::c_double) as libc::c_int;
    (*ent).think = Some(Think_Target_Delay);
    (*ent).activator = activator;
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_delay(mut ent: *mut gentity_t) {
    if 0 == G_SpawnFloat(
        b"delay\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    ) as u64
    {
        G_SpawnFloat(
            b"wait\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
            &mut (*ent).wait,
        );
    }
    if 0. == (*ent).wait {
        (*ent).wait = 1i32 as libc::c_float
    }
    (*ent).use_0 = Some(Use_Target_Delay);
}
//==========================================================
/*QUAKED target_score (1 0 0) (-8 -8 -8) (8 8 8)
"count" number of points to add, default 1

The activator is given this many points.
*/
#[no_mangle]
pub unsafe extern "C" fn Use_Target_Score(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    AddScore(activator, (*ent).r.currentOrigin.as_mut_ptr(), (*ent).count);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_score(mut ent: *mut gentity_t) {
    if 0 == (*ent).count {
        (*ent).count = 1i32
    }
    (*ent).use_0 = Some(Use_Target_Score);
}
//==========================================================
/*QUAKED target_print (1 0 0) (-8 -8 -8) (8 8 8) redteam blueteam private
"message"	text to print
If "private", only the activator gets the message.  If no checks, all clients get the message.
*/
#[no_mangle]
pub unsafe extern "C" fn Use_Target_Print(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    if !(*activator).client.is_null() && 0 != (*ent).spawnflags & 4i32 {
        trap_SendServerCommand(
            activator.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int,
            va(
                b"cp \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ent).message,
            ),
        );
        return;
    }
    if 0 != (*ent).spawnflags & 3i32 {
        if 0 != (*ent).spawnflags & 1i32 {
            G_TeamCommand(
                TEAM_RED,
                va(
                    b"cp \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*ent).message,
                ),
            );
        }
        if 0 != (*ent).spawnflags & 2i32 {
            G_TeamCommand(
                TEAM_BLUE,
                va(
                    b"cp \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*ent).message,
                ),
            );
        }
        return;
    }
    trap_SendServerCommand(
        -1i32,
        va(
            b"cp \"%s\"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*ent).message,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_print(mut ent: *mut gentity_t) {
    (*ent).use_0 = Some(Use_Target_Print);
}
//==========================================================
/*QUAKED target_speaker (1 0 0) (-8 -8 -8) (8 8 8) looped-on looped-off global activator
"noise"		wav file to play

A global sound will play full volume throughout the level.
Activator sounds will play on the player that activated the target.
Global and activator sounds can't be combined with looping.
Normal sounds play each time the target is used.
Looped sounds will be toggled by use functions.
Multiple identical looping sounds will just increase volume without any speed cost.
"wait" : Seconds between auto triggerings, 0 = don't auto trigger
"random"	wait variance, default is 0
*/
#[no_mangle]
pub unsafe extern "C" fn Use_Target_Speaker(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    if 0 != (*ent).spawnflags & 3i32 {
        if 0 != (*ent).s.loopSound {
            (*ent).s.loopSound = 0i32
        } else {
            (*ent).s.loopSound = (*ent).noise_index
        }
    } else if 0 != (*ent).spawnflags & 8i32 {
        G_AddEvent(
            activator,
            EV_GENERAL_SOUND as libc::c_int,
            (*ent).noise_index,
        );
    } else if 0 != (*ent).spawnflags & 4i32 {
        G_AddEvent(ent, EV_GLOBAL_SOUND as libc::c_int, (*ent).noise_index);
    } else {
        G_AddEvent(ent, EV_GENERAL_SOUND as libc::c_int, (*ent).noise_index);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_speaker(mut ent: *mut gentity_t) {
    let mut buffer: [libc::c_char; 64] = [0; 64];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    );
    G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).random,
    );
    if 0 == G_SpawnString(
        b"noise\x00" as *const u8 as *const libc::c_char,
        b"NOSOUND\x00" as *const u8 as *const libc::c_char,
        &mut s,
    ) as u64
    {
        G_Error(
            b"target_speaker without a noise key at %s\x00" as *const u8 as *const libc::c_char,
            vtos((*ent).s.origin.as_mut_ptr() as *const vec_t),
        );
    }
    if *s.offset(0isize) as libc::c_int == '*' as i32 {
        (*ent).spawnflags |= 8i32
    }
    if strstr(s, b".wav\x00" as *const u8 as *const libc::c_char).is_null() {
        Com_sprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%s.wav\x00" as *const u8 as *const libc::c_char,
            s,
        );
    } else {
        Q_strncpyz(
            buffer.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
    }
    (*ent).noise_index = G_SoundIndex(buffer.as_mut_ptr());
    (*ent).s.eType = ET_SPEAKER as libc::c_int;
    (*ent).s.eventParm = (*ent).noise_index;
    (*ent).s.frame = ((*ent).wait * 10i32 as libc::c_float) as libc::c_int;
    (*ent).s.clientNum = ((*ent).random * 10i32 as libc::c_float) as libc::c_int;
    if 0 != (*ent).spawnflags & 1i32 {
        (*ent).s.loopSound = (*ent).noise_index
    }
    (*ent).use_0 = Some(Use_Target_Speaker);
    if 0 != (*ent).spawnflags & 4i32 {
        (*ent).r.svFlags |= 0x20i32
    }
    (*ent).s.pos.trBase[0usize] = (*ent).s.origin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).s.origin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).s.origin[2usize];
    trap_LinkEntity(ent);
}
//==========================================================
/*QUAKED target_laser (0 .5 .8) (-8 -8 -8) (8 8 8) START_ON
When triggered, fires a laser.  You can either set a target or a direction.
*/
#[no_mangle]
pub unsafe extern "C" fn target_laser_think(mut self_0: *mut gentity_t) {
    let mut end: vec3_t = [0.; 3];
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
    let mut point: vec3_t = [0.; 3];
    if !(*self_0).enemy.is_null() {
        point[0usize] = ((*(*self_0).enemy).s.origin[0usize] as libc::c_double
            + (*(*self_0).enemy).r.mins[0usize] as libc::c_double * 0.5f64)
            as vec_t;
        point[1usize] = ((*(*self_0).enemy).s.origin[1usize] as libc::c_double
            + (*(*self_0).enemy).r.mins[1usize] as libc::c_double * 0.5f64)
            as vec_t;
        point[2usize] = ((*(*self_0).enemy).s.origin[2usize] as libc::c_double
            + (*(*self_0).enemy).r.mins[2usize] as libc::c_double * 0.5f64)
            as vec_t;
        point[0usize] = (point[0usize] as libc::c_double
            + (*(*self_0).enemy).r.maxs[0usize] as libc::c_double * 0.5f64)
            as vec_t;
        point[1usize] = (point[1usize] as libc::c_double
            + (*(*self_0).enemy).r.maxs[1usize] as libc::c_double * 0.5f64)
            as vec_t;
        point[2usize] = (point[2usize] as libc::c_double
            + (*(*self_0).enemy).r.maxs[2usize] as libc::c_double * 0.5f64)
            as vec_t;
        (*self_0).movedir[0usize] = point[0usize] - (*self_0).s.origin[0usize];
        (*self_0).movedir[1usize] = point[1usize] - (*self_0).s.origin[1usize];
        (*self_0).movedir[2usize] = point[2usize] - (*self_0).s.origin[2usize];
        VectorNormalize((*self_0).movedir.as_mut_ptr());
    }
    end[0usize] = (*self_0).s.origin[0usize] + (*self_0).movedir[0usize] * 2048i32 as libc::c_float;
    end[1usize] = (*self_0).s.origin[1usize] + (*self_0).movedir[1usize] * 2048i32 as libc::c_float;
    end[2usize] = (*self_0).s.origin[2usize] + (*self_0).movedir[2usize] * 2048i32 as libc::c_float;
    trap_Trace(
        &mut tr,
        (*self_0).s.origin.as_mut_ptr() as *const vec_t,
        0 as *const vec_t,
        0 as *const vec_t,
        end.as_mut_ptr() as *const vec_t,
        (*self_0).s.number,
        1i32 | 0x2000000i32 | 0x4000000i32,
    );
    if 0 != tr.entityNum {
        G_Damage(
            &mut *g_entities.as_mut_ptr().offset(tr.entityNum as isize),
            self_0,
            (*self_0).activator,
            (*self_0).movedir.as_mut_ptr(),
            tr.endpos.as_mut_ptr(),
            (*self_0).damage,
            0x4i32,
            MOD_TARGET_LASER as libc::c_int,
        );
    }
    (*self_0).s.origin2[0usize] = tr.endpos[0usize];
    (*self_0).s.origin2[1usize] = tr.endpos[1usize];
    (*self_0).s.origin2[2usize] = tr.endpos[2usize];
    trap_LinkEntity(self_0);
    (*self_0).nextthink = level.time + 100i32;
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_on(mut self_0: *mut gentity_t) {
    if (*self_0).activator.is_null() {
        (*self_0).activator = self_0
    }
    target_laser_think(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_off(mut self_0: *mut gentity_t) {
    trap_UnlinkEntity(self_0);
    (*self_0).nextthink = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_use(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    (*self_0).activator = activator;
    if (*self_0).nextthink > 0i32 {
        target_laser_off(self_0);
    } else {
        target_laser_on(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_start(mut self_0: *mut gentity_t) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    (*self_0).s.eType = ET_BEAM as libc::c_int;
    if !(*self_0).target.is_null() {
        ent = G_Find(
            0 as *mut gentity_t,
            &mut (*(0 as *mut gentity_t)).targetname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            (*self_0).target,
        );
        if ent.is_null() {
            G_Printf(
                b"%s at %s: %s is a bad target\n\x00" as *const u8 as *const libc::c_char,
                (*self_0).classname,
                vtos((*self_0).s.origin.as_mut_ptr() as *const vec_t),
                (*self_0).target,
            );
        }
        (*self_0).enemy = ent
    } else {
        G_SetMovedir(
            (*self_0).s.angles.as_mut_ptr(),
            (*self_0).movedir.as_mut_ptr(),
        );
    }
    (*self_0).use_0 = Some(target_laser_use);
    (*self_0).think = Some(target_laser_think);
    if 0 == (*self_0).damage {
        (*self_0).damage = 1i32
    }
    if 0 != (*self_0).spawnflags & 1i32 {
        target_laser_on(self_0);
    } else {
        target_laser_off(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_laser(mut self_0: *mut gentity_t) {
    (*self_0).think = Some(target_laser_start);
    (*self_0).nextthink = level.time + 100i32;
}
//==========================================================
#[no_mangle]
pub unsafe extern "C" fn target_teleporter_use(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    let mut dest: *mut gentity_t = 0 as *mut gentity_t;
    if (*activator).client.is_null() {
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
        activator,
        (*dest).s.origin.as_mut_ptr(),
        (*dest).s.angles.as_mut_ptr(),
    );
}
/*QUAKED target_teleporter (1 0 0) (-8 -8 -8) (8 8 8)
The activator will be teleported away.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_target_teleporter(mut self_0: *mut gentity_t) {
    if (*self_0).targetname.is_null() {
        G_Printf(
            b"untargeted %s at %s\n\x00" as *const u8 as *const libc::c_char,
            (*self_0).classname,
            vtos((*self_0).s.origin.as_mut_ptr() as *const vec_t),
        );
    }
    (*self_0).use_0 = Some(target_teleporter_use);
}
//==========================================================
/*QUAKED target_relay (.5 .5 .5) (-8 -8 -8) (8 8 8) RED_ONLY BLUE_ONLY RANDOM
This doesn't perform any actions except fire its targets.
The activator can be forced to be from a certain team.
if RANDOM is checked, only one of the targets will be fired, not all of them
*/
#[no_mangle]
pub unsafe extern "C" fn target_relay_use(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    if 0 != (*self_0).spawnflags & 1i32
        && !(*activator).client.is_null()
        && (*(*activator).client).sess.sessionTeam as libc::c_uint
            != TEAM_RED as libc::c_int as libc::c_uint
    {
        return;
    }
    if 0 != (*self_0).spawnflags & 2i32
        && !(*activator).client.is_null()
        && (*(*activator).client).sess.sessionTeam as libc::c_uint
            != TEAM_BLUE as libc::c_int as libc::c_uint
    {
        return;
    }
    if 0 != (*self_0).spawnflags & 4i32 {
        let mut ent: *mut gentity_t = 0 as *mut gentity_t;
        ent = G_PickTarget((*self_0).target);
        if !ent.is_null() && (*ent).use_0.is_some() {
            (*ent).use_0.expect("non-null function pointer")(ent, self_0, activator);
        }
        return;
    }
    G_UseTargets(self_0, activator);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_relay(mut self_0: *mut gentity_t) {
    (*self_0).use_0 = Some(target_relay_use);
}
//==========================================================
/*QUAKED target_kill (.5 .5 .5) (-8 -8 -8) (8 8 8)
Kills the activator.
*/
#[no_mangle]
pub unsafe extern "C" fn target_kill_use(
    mut self_0: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    G_Damage(
        activator,
        0 as *mut gentity_t,
        0 as *mut gentity_t,
        0 as *mut vec_t,
        0 as *mut vec_t,
        100000i32,
        0x8i32,
        MOD_TELEFRAG as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_kill(mut self_0: *mut gentity_t) {
    (*self_0).use_0 = Some(target_kill_use);
}
/*QUAKED target_position (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for in-game calculation, like jumppad targets.
*/
#[no_mangle]
pub unsafe extern "C" fn SP_target_position(mut self_0: *mut gentity_t) {
    G_SetOrigin(self_0, (*self_0).s.origin.as_mut_ptr());
}
unsafe extern "C" fn target_location_linkup(mut ent: *mut gentity_t) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if 0 != level.locationLinked as u64 {
        return;
    }
    level.locationLinked = qtrue;
    level.locationHead = 0 as *mut gentity_t;
    trap_SetConfigstring(
        32i32 + 256i32 + 256i32 + 64i32,
        b"unknown\x00" as *const u8 as *const libc::c_char,
    );
    i = 0i32;
    ent = g_entities.as_mut_ptr();
    n = 1i32;
    while i < level.num_entities {
        if !(*ent).classname.is_null()
            && 0 == Q_stricmp(
                (*ent).classname,
                b"target_location\x00" as *const u8 as *const libc::c_char,
            )
        {
            (*ent).health = n;
            trap_SetConfigstring(32i32 + 256i32 + 256i32 + 64i32 + n, (*ent).message);
            n += 1;
            (*ent).nextTrain = level.locationHead;
            level.locationHead = ent
        }
        i += 1;
        ent = ent.offset(1isize)
    }
}
// All linked together now
/*QUAKED target_location (0 0.5 0) (-8 -8 -8) (8 8 8)
Set "message" to the name of this location.
Set "count" to 0-7 for color.
0:white 1:red 2:green 3:yellow 4:blue 5:cyan 6:magenta 7:white

Closest target_location in sight used for the location, if none
in site, closest in distance
*/
#[no_mangle]
pub unsafe extern "C" fn SP_target_location(mut self_0: *mut gentity_t) {
    (*self_0).think = Some(target_location_linkup);
    (*self_0).nextthink = level.time + 200i32;
    G_SetOrigin(self_0, (*self_0).s.origin.as_mut_ptr());
}
