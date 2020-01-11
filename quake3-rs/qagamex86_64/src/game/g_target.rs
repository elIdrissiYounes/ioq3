use ::libc;

pub use crate::stddef_h::size_t;

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
pub use crate::bg_public_h::EV_BULLET;
pub use crate::bg_public_h::EV_BULLET_HIT_FLESH;
pub use crate::bg_public_h::EV_BULLET_HIT_WALL;
pub use crate::bg_public_h::EV_CHANGE_WEAPON;
pub use crate::bg_public_h::EV_DEATH1;
pub use crate::bg_public_h::EV_DEATH2;
pub use crate::bg_public_h::EV_DEATH3;
pub use crate::bg_public_h::EV_DEBUG_LINE;
pub use crate::bg_public_h::EV_FALL_FAR;
pub use crate::bg_public_h::EV_FALL_MEDIUM;
pub use crate::bg_public_h::EV_FALL_SHORT;
pub use crate::bg_public_h::EV_FIRE_WEAPON;
pub use crate::bg_public_h::EV_FOOTSPLASH;
pub use crate::bg_public_h::EV_FOOTSTEP;
pub use crate::bg_public_h::EV_FOOTSTEP_METAL;
pub use crate::bg_public_h::EV_FOOTWADE;
pub use crate::bg_public_h::EV_GENERAL_SOUND;
pub use crate::bg_public_h::EV_GIB_PLAYER;
pub use crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP;
pub use crate::bg_public_h::EV_GLOBAL_SOUND;
pub use crate::bg_public_h::EV_GLOBAL_TEAM_SOUND;
pub use crate::bg_public_h::EV_GRENADE_BOUNCE;
pub use crate::bg_public_h::EV_INVUL_IMPACT;
pub use crate::bg_public_h::EV_ITEM_PICKUP;
pub use crate::bg_public_h::EV_ITEM_POP;
pub use crate::bg_public_h::EV_ITEM_RESPAWN;
pub use crate::bg_public_h::EV_JUICED;
pub use crate::bg_public_h::EV_JUMP;
pub use crate::bg_public_h::EV_JUMP_PAD;
pub use crate::bg_public_h::EV_KAMIKAZE;
pub use crate::bg_public_h::EV_LIGHTNINGBOLT;
pub use crate::bg_public_h::EV_MISSILE_HIT;
pub use crate::bg_public_h::EV_MISSILE_MISS;
pub use crate::bg_public_h::EV_MISSILE_MISS_METAL;
pub use crate::bg_public_h::EV_NOAMMO;
pub use crate::bg_public_h::EV_NONE;
pub use crate::bg_public_h::EV_OBELISKEXPLODE;
pub use crate::bg_public_h::EV_OBELISKPAIN;
pub use crate::bg_public_h::EV_OBITUARY;
pub use crate::bg_public_h::EV_PAIN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_IN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_OUT;
pub use crate::bg_public_h::EV_POWERUP_BATTLESUIT;
pub use crate::bg_public_h::EV_POWERUP_QUAD;
pub use crate::bg_public_h::EV_POWERUP_REGEN;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_STICK;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_TRIGGER;
pub use crate::bg_public_h::EV_RAILTRAIL;
pub use crate::bg_public_h::EV_SCOREPLUM;
pub use crate::bg_public_h::EV_SHOTGUN;
pub use crate::bg_public_h::EV_STEP_12;
pub use crate::bg_public_h::EV_STEP_16;
pub use crate::bg_public_h::EV_STEP_4;
pub use crate::bg_public_h::EV_STEP_8;
pub use crate::bg_public_h::EV_STOPLOOPINGSOUND;
pub use crate::bg_public_h::EV_SWIM;
pub use crate::bg_public_h::EV_TAUNT;
pub use crate::bg_public_h::EV_TAUNT_FOLLOWME;
pub use crate::bg_public_h::EV_TAUNT_GETFLAG;
pub use crate::bg_public_h::EV_TAUNT_GUARDBASE;
pub use crate::bg_public_h::EV_TAUNT_NO;
pub use crate::bg_public_h::EV_TAUNT_PATROL;
pub use crate::bg_public_h::EV_TAUNT_YES;
pub use crate::bg_public_h::EV_USE_ITEM0;
pub use crate::bg_public_h::EV_USE_ITEM1;
pub use crate::bg_public_h::EV_USE_ITEM10;
pub use crate::bg_public_h::EV_USE_ITEM11;
pub use crate::bg_public_h::EV_USE_ITEM12;
pub use crate::bg_public_h::EV_USE_ITEM13;
pub use crate::bg_public_h::EV_USE_ITEM14;
pub use crate::bg_public_h::EV_USE_ITEM15;
pub use crate::bg_public_h::EV_USE_ITEM2;
pub use crate::bg_public_h::EV_USE_ITEM3;
pub use crate::bg_public_h::EV_USE_ITEM4;
pub use crate::bg_public_h::EV_USE_ITEM5;
pub use crate::bg_public_h::EV_USE_ITEM6;
pub use crate::bg_public_h::EV_USE_ITEM7;
pub use crate::bg_public_h::EV_USE_ITEM8;
pub use crate::bg_public_h::EV_USE_ITEM9;
pub use crate::bg_public_h::EV_WATER_CLEAR;
pub use crate::bg_public_h::EV_WATER_LEAVE;
pub use crate::bg_public_h::EV_WATER_TOUCH;
pub use crate::bg_public_h::EV_WATER_UNDER;
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::MOD_BFG;
pub use crate::bg_public_h::MOD_BFG_SPLASH;
pub use crate::bg_public_h::MOD_CRUSH;
pub use crate::bg_public_h::MOD_FALLING;
pub use crate::bg_public_h::MOD_GAUNTLET;
pub use crate::bg_public_h::MOD_GRAPPLE;
pub use crate::bg_public_h::MOD_GRENADE;
pub use crate::bg_public_h::MOD_GRENADE_SPLASH;
pub use crate::bg_public_h::MOD_LAVA;
pub use crate::bg_public_h::MOD_LIGHTNING;
pub use crate::bg_public_h::MOD_MACHINEGUN;
pub use crate::bg_public_h::MOD_PLASMA;
pub use crate::bg_public_h::MOD_PLASMA_SPLASH;
pub use crate::bg_public_h::MOD_RAILGUN;
pub use crate::bg_public_h::MOD_ROCKET;
pub use crate::bg_public_h::MOD_ROCKET_SPLASH;
pub use crate::bg_public_h::MOD_SHOTGUN;
pub use crate::bg_public_h::MOD_SLIME;
pub use crate::bg_public_h::MOD_SUICIDE;
pub use crate::bg_public_h::MOD_TARGET_LASER;
pub use crate::bg_public_h::MOD_TELEFRAG;
pub use crate::bg_public_h::MOD_TRIGGER_HURT;
pub use crate::bg_public_h::MOD_UNKNOWN;
pub use crate::bg_public_h::MOD_WATER;
pub use crate::bg_public_h::PW_AMMOREGEN;
pub use crate::bg_public_h::PW_BATTLESUIT;
pub use crate::bg_public_h::PW_BLUEFLAG;
pub use crate::bg_public_h::PW_DOUBLER;
pub use crate::bg_public_h::PW_FLIGHT;
pub use crate::bg_public_h::PW_GUARD;
pub use crate::bg_public_h::PW_HASTE;
pub use crate::bg_public_h::PW_INVIS;
pub use crate::bg_public_h::PW_INVULNERABILITY;
pub use crate::bg_public_h::PW_NEUTRALFLAG;
pub use crate::bg_public_h::PW_NONE;
pub use crate::bg_public_h::PW_NUM_POWERUPS;
pub use crate::bg_public_h::PW_QUAD;
pub use crate::bg_public_h::PW_REDFLAG;
pub use crate::bg_public_h::PW_REGEN;
pub use crate::bg_public_h::PW_SCOUT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
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
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
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
pub use crate::src::game::g_combat::AddScore;
pub use crate::src::game::g_combat::G_Damage;
pub use crate::src::game::g_items::Touch_Item;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_misc::TeleportPlayer;
pub use crate::src::game::g_spawn::G_SpawnFloat;
pub use crate::src::game::g_spawn::G_SpawnString;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_syscalls::trap_Trace;
pub use crate::src::game::g_syscalls::trap_UnlinkEntity;

pub use crate::src::game::g_utils::vtos;
pub use crate::src::game::g_utils::G_AddEvent;
pub use crate::src::game::g_utils::G_Find;
pub use crate::src::game::g_utils::G_PickTarget;
pub use crate::src::game::g_utils::G_SetMovedir;
pub use crate::src::game::g_utils::G_SetOrigin;
pub use crate::src::game::g_utils::G_SoundIndex;
pub use crate::src::game::g_utils::G_TeamCommand;
pub use crate::src::game::g_utils::G_UseTargets;

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
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    let mut t: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
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
    crate::stdlib::memset(
        &mut trace as *mut crate::src::qcommon::q_shared::trace_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::trace_t>(),
    );
    t = 0 as *mut crate::g_local_h::gentity_t;
    loop {
        t = crate::src::game::g_utils::G_Find(
            t,
            &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetname as *mut *mut i8 as i32,
            (*ent).target,
        );
        if t.is_null() {
            break;
        }
        if (*t).item.is_null() {
            continue;
        }
        crate::src::game::g_items::Touch_Item(t, activator, &mut trace);
        // make sure it isn't going to respawn or show any events
        (*t).nextthink = 0;
        crate::src::game::g_syscalls::trap_UnlinkEntity(t);
    }
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_give(mut ent: *mut crate::g_local_h::gentity_t) {
    (*ent).use_0 = Some(
        Use_Target_Give
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
}
//==========================================================
/*QUAKED target_remove_powerups (1 0 0) (-8 -8 -8) (8 8 8)
takes away all the activators powerups.
Used to drop flight powerups into death puts.
*/
#[no_mangle]

pub unsafe extern "C" fn Use_target_remove_powerups(
    mut _ent: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    if (*activator).client.is_null() {
        return;
    }
    if (*(*activator).client).ps.powerups[crate::bg_public_h::PW_REDFLAG as usize] != 0 {
        crate::src::game::g_team::Team_ReturnFlag(crate::bg_public_h::TEAM_RED as i32);
    } else if (*(*activator).client).ps.powerups[crate::bg_public_h::PW_BLUEFLAG as usize] != 0 {
        crate::src::game::g_team::Team_ReturnFlag(crate::bg_public_h::TEAM_BLUE as i32);
    } else if (*(*activator).client).ps.powerups[crate::bg_public_h::PW_NEUTRALFLAG as usize] != 0 {
        crate::src::game::g_team::Team_ReturnFlag(crate::bg_public_h::TEAM_FREE as i32);
    }
    crate::stdlib::memset(
        (*(*activator).client).ps.powerups.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[i32; 16]>(),
    );
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_remove_powerups(mut ent: *mut crate::g_local_h::gentity_t) {
    (*ent).use_0 = Some(
        Use_target_remove_powerups
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
}
//==========================================================
/*QUAKED target_delay (1 0 0) (-8 -8 -8) (8 8 8)
"wait" seconds to pause before firing targets.
"random" delay variance, total delay = delay +/- random seconds
*/
#[no_mangle]

pub unsafe extern "C" fn Think_Target_Delay(mut ent: *mut crate::g_local_h::gentity_t) {
    crate::src::game::g_utils::G_UseTargets(ent, (*ent).activator);
}
#[no_mangle]

pub unsafe extern "C" fn Use_Target_Delay(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    (*ent).nextthink = (crate::src::game::g_main::level.time as f64
        + ((*ent).wait as f64
            + (*ent).random as f64
                * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
            * 1000f64) as i32;
    (*ent).think =
        Some(Think_Target_Delay as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    (*ent).activator = activator;
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_delay(mut ent: *mut crate::g_local_h::gentity_t) {
    // check delay for backwards compatibility
    if crate::src::game::g_spawn::G_SpawnFloat(
        b"delay\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        &mut (*ent).wait,
    ) as u64
        == 0
    {
        crate::src::game::g_spawn::G_SpawnFloat(
            b"wait\x00" as *const u8 as *const i8,
            b"1\x00" as *const u8 as *const i8,
            &mut (*ent).wait,
        );
    }
    if (*ent).wait == 0. {
        (*ent).wait = 1f32
    }
    (*ent).use_0 = Some(
        Use_Target_Delay
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
}
//==========================================================
/*QUAKED target_score (1 0 0) (-8 -8 -8) (8 8 8)
"count" number of points to add, default 1

The activator is given this many points.
*/
#[no_mangle]

pub unsafe extern "C" fn Use_Target_Score(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    crate::src::game::g_combat::AddScore(
        activator,
        (*ent).r.currentOrigin.as_mut_ptr(),
        (*ent).count,
    );
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_score(mut ent: *mut crate::g_local_h::gentity_t) {
    if (*ent).count == 0 {
        (*ent).count = 1
    }
    (*ent).use_0 = Some(
        Use_Target_Score
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
}
//==========================================================
/*QUAKED target_print (1 0 0) (-8 -8 -8) (8 8 8) redteam blueteam private
"message"	text to print
If "private", only the activator gets the message.  If no checks, all clients get the message.
*/
#[no_mangle]

pub unsafe extern "C" fn Use_Target_Print(
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    if !(*activator).client.is_null() && (*ent).spawnflags & 4 != 0 {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            activator.wrapping_offset_from(crate::src::game::g_main::g_entities.as_mut_ptr())
                as i32,
            crate::src::qcommon::q_shared::va(
                b"cp \"%s\"\x00" as *const u8 as *mut i8,
                (*ent).message,
            ),
        );
        return;
    }
    if (*ent).spawnflags & 3 != 0 {
        if (*ent).spawnflags & 1 != 0 {
            crate::src::game::g_utils::G_TeamCommand(
                crate::bg_public_h::TEAM_RED,
                crate::src::qcommon::q_shared::va(
                    b"cp \"%s\"\x00" as *const u8 as *mut i8,
                    (*ent).message,
                ),
            );
        }
        if (*ent).spawnflags & 2 != 0 {
            crate::src::game::g_utils::G_TeamCommand(
                crate::bg_public_h::TEAM_BLUE,
                crate::src::qcommon::q_shared::va(
                    b"cp \"%s\"\x00" as *const u8 as *mut i8,
                    (*ent).message,
                ),
            );
        }
        return;
    }
    crate::src::game::g_syscalls::trap_SendServerCommand(
        -(1),
        crate::src::qcommon::q_shared::va(b"cp \"%s\"\x00" as *const u8 as *mut i8, (*ent).message),
    );
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_print(mut ent: *mut crate::g_local_h::gentity_t) {
    (*ent).use_0 = Some(
        Use_Target_Print
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
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
    mut ent: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    if (*ent).spawnflags & 3 != 0 {
        // looping sound toggles
        if (*ent).s.loopSound != 0 {
            (*ent).s.loopSound = 0
        } else {
            (*ent).s.loopSound = (*ent).noise_index
        } // turn it off
          // start it
    } else if (*ent).spawnflags & 8 != 0 {
        crate::src::game::g_utils::G_AddEvent(
            activator,
            crate::bg_public_h::EV_GENERAL_SOUND as i32,
            (*ent).noise_index,
        );
    } else if (*ent).spawnflags & 4 != 0 {
        crate::src::game::g_utils::G_AddEvent(
            ent,
            crate::bg_public_h::EV_GLOBAL_SOUND as i32,
            (*ent).noise_index,
        );
    } else {
        crate::src::game::g_utils::G_AddEvent(
            ent,
            crate::bg_public_h::EV_GENERAL_SOUND as i32,
            (*ent).noise_index,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_speaker(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut buffer: [i8; 64] = [0; 64];
    let mut s: *mut i8 = 0 as *mut i8;
    crate::src::game::g_spawn::G_SpawnFloat(
        b"wait\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        &mut (*ent).wait,
    );
    crate::src::game::g_spawn::G_SpawnFloat(
        b"random\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        &mut (*ent).random,
    );
    if crate::src::game::g_spawn::G_SpawnString(
        b"noise\x00" as *const u8 as *const i8,
        b"NOSOUND\x00" as *const u8 as *const i8,
        &mut s,
    ) as u64
        == 0
    {
        crate::src::game::g_main::G_Error(
            b"target_speaker without a noise key at %s\x00" as *const u8 as *const i8,
            crate::src::game::g_utils::vtos(
                (*ent).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            ),
        );
    }
    // force all client relative sounds to be "activator" speakers that
    // play on the entity that activates it
    if *s.offset(0) as i32 == '*' as i32 {
        (*ent).spawnflags |= 8
    }
    if crate::stdlib::strstr(s, b".wav\x00" as *const u8 as *const i8).is_null() {
        crate::src::qcommon::q_shared::Com_sprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"%s.wav\x00" as *const u8 as *const i8,
            s,
        );
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(
            buffer.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
    }
    (*ent).noise_index = crate::src::game::g_utils::G_SoundIndex(buffer.as_mut_ptr());
    // a repeating speaker can be done completely client side
    (*ent).s.eType = crate::bg_public_h::ET_SPEAKER as i32;
    (*ent).s.eventParm = (*ent).noise_index;
    (*ent).s.frame = ((*ent).wait * 10f32) as i32;
    (*ent).s.clientNum = ((*ent).random * 10f32) as i32;
    // check for prestarted looping sound
    if (*ent).spawnflags & 1 != 0 {
        (*ent).s.loopSound = (*ent).noise_index
    }
    (*ent).use_0 = Some(
        Use_Target_Speaker
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
    if (*ent).spawnflags & 4 != 0 {
        (*ent).r.svFlags |= 0x20
    }
    (*ent).s.pos.trBase[0] = (*ent).s.origin[0];
    (*ent).s.pos.trBase[1] = (*ent).s.origin[1];
    (*ent).s.pos.trBase[2] = (*ent).s.origin[2];
    // must link the entity so we get areas and clusters so
    // the server can determine who to send updates to
    crate::src::game::g_syscalls::trap_LinkEntity(ent);
}
//==========================================================
/*QUAKED target_laser (0 .5 .8) (-8 -8 -8) (8 8 8) START_ON
When triggered, fires a laser.  You can either set a target or a direction.
*/
#[no_mangle]

pub unsafe extern "C" fn target_laser_think(mut self_0: *mut crate::g_local_h::gentity_t) {
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut tr: crate::src::qcommon::q_shared::trace_t = crate::src::qcommon::q_shared::trace_t {
        allsolid: crate::src::qcommon::q_shared::qfalse,
        startsolid: crate::src::qcommon::q_shared::qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: crate::src::qcommon::q_shared::cplane_t {
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
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // if pointed at another entity, set movedir to point at it
    if !(*self_0).enemy.is_null() {
        point[0] = ((*(*self_0).enemy).s.origin[0] as f64
            + (*(*self_0).enemy).r.mins[0] as f64 * 0.5)
            as crate::src::qcommon::q_shared::vec_t;
        point[1] = ((*(*self_0).enemy).s.origin[1] as f64
            + (*(*self_0).enemy).r.mins[1] as f64 * 0.5)
            as crate::src::qcommon::q_shared::vec_t;
        point[2] = ((*(*self_0).enemy).s.origin[2] as f64
            + (*(*self_0).enemy).r.mins[2] as f64 * 0.5)
            as crate::src::qcommon::q_shared::vec_t;
        point[0] = (point[0] as f64 + (*(*self_0).enemy).r.maxs[0] as f64 * 0.5)
            as crate::src::qcommon::q_shared::vec_t;
        point[1] = (point[1] as f64 + (*(*self_0).enemy).r.maxs[1] as f64 * 0.5)
            as crate::src::qcommon::q_shared::vec_t;
        point[2] = (point[2] as f64 + (*(*self_0).enemy).r.maxs[2] as f64 * 0.5)
            as crate::src::qcommon::q_shared::vec_t;
        (*self_0).movedir[0] = point[0] - (*self_0).s.origin[0];
        (*self_0).movedir[1] = point[1] - (*self_0).s.origin[1];
        (*self_0).movedir[2] = point[2] - (*self_0).s.origin[2];
        crate::src::qcommon::q_math::VectorNormalize((*self_0).movedir.as_mut_ptr());
    }
    // fire forward and see what we hit
    end[0] = (*self_0).s.origin[0] + (*self_0).movedir[0] * 2048f32;
    end[1] = (*self_0).s.origin[1] + (*self_0).movedir[1] * 2048f32;
    end[2] = (*self_0).s.origin[2] + (*self_0).movedir[2] * 2048f32;
    crate::src::game::g_syscalls::trap_Trace(
        &mut tr,
        (*self_0).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*self_0).s.number,
        1 | 0x2000000 | 0x4000000,
    );
    if tr.entityNum != 0 {
        // hurt it if we can
        crate::src::game::g_combat::G_Damage(
            &mut *crate::src::game::g_main::g_entities
                .as_mut_ptr()
                .offset(tr.entityNum as isize),
            self_0,
            (*self_0).activator,
            (*self_0).movedir.as_mut_ptr(),
            tr.endpos.as_mut_ptr(),
            (*self_0).damage,
            0x4i32,
            crate::bg_public_h::MOD_TARGET_LASER as i32,
        );
    }
    (*self_0).s.origin2[0] = tr.endpos[0];
    (*self_0).s.origin2[1] = tr.endpos[1];
    (*self_0).s.origin2[2] = tr.endpos[2];
    crate::src::game::g_syscalls::trap_LinkEntity(self_0);
    (*self_0).nextthink = crate::src::game::g_main::level.time + 100;
}
#[no_mangle]

pub unsafe extern "C" fn target_laser_on(mut self_0: *mut crate::g_local_h::gentity_t) {
    if (*self_0).activator.is_null() {
        (*self_0).activator = self_0
    }
    target_laser_think(self_0);
}
#[no_mangle]

pub unsafe extern "C" fn target_laser_off(mut self_0: *mut crate::g_local_h::gentity_t) {
    crate::src::game::g_syscalls::trap_UnlinkEntity(self_0);
    (*self_0).nextthink = 0;
}
#[no_mangle]

pub unsafe extern "C" fn target_laser_use(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    (*self_0).activator = activator;
    if (*self_0).nextthink > 0 {
        target_laser_off(self_0);
    } else {
        target_laser_on(self_0);
    };
}
#[no_mangle]

pub unsafe extern "C" fn target_laser_start(mut self_0: *mut crate::g_local_h::gentity_t) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    (*self_0).s.eType = crate::bg_public_h::ET_BEAM as i32;
    if !(*self_0).target.is_null() {
        ent = crate::src::game::g_utils::G_Find(
            0 as *mut crate::g_local_h::gentity_t,
            &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetname as *mut *mut i8 as i32,
            (*self_0).target,
        );
        if ent.is_null() {
            crate::src::game::g_main::G_Printf(
                b"%s at %s: %s is a bad target\n\x00" as *const u8 as *const i8,
                (*self_0).classname,
                crate::src::game::g_utils::vtos(
                    (*self_0).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                ),
                (*self_0).target,
            );
        }
        (*self_0).enemy = ent
    } else {
        crate::src::game::g_utils::G_SetMovedir(
            (*self_0).s.angles.as_mut_ptr(),
            (*self_0).movedir.as_mut_ptr(),
        );
    }
    (*self_0).use_0 = Some(
        target_laser_use
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
    (*self_0).think =
        Some(target_laser_think as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    if (*self_0).damage == 0 {
        (*self_0).damage = 1
    }
    if (*self_0).spawnflags & 1 != 0 {
        target_laser_on(self_0);
    } else {
        target_laser_off(self_0);
    };
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_laser(mut self_0: *mut crate::g_local_h::gentity_t) {
    // let everything else get spawned before we start firing
    (*self_0).think =
        Some(target_laser_start as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ());
    (*self_0).nextthink = crate::src::game::g_main::level.time + 100;
}
//==========================================================
#[no_mangle]

pub unsafe extern "C" fn target_teleporter_use(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    let mut dest: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    if (*activator).client.is_null() {
        return;
    }
    dest = crate::src::game::g_utils::G_PickTarget((*self_0).target);
    if dest.is_null() {
        crate::src::game::g_main::G_Printf(
            b"Couldn\'t find teleporter destination\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    crate::src::game::g_misc::TeleportPlayer(
        activator,
        (*dest).s.origin.as_mut_ptr(),
        (*dest).s.angles.as_mut_ptr(),
    );
}
/*QUAKED target_teleporter (1 0 0) (-8 -8 -8) (8 8 8)
The activator will be teleported away.
*/
#[no_mangle]

pub unsafe extern "C" fn SP_target_teleporter(mut self_0: *mut crate::g_local_h::gentity_t) {
    if (*self_0).targetname.is_null() {
        crate::src::game::g_main::G_Printf(
            b"untargeted %s at %s\n\x00" as *const u8 as *const i8,
            (*self_0).classname,
            crate::src::game::g_utils::vtos(
                (*self_0).s.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
            ),
        );
    }
    (*self_0).use_0 = Some(
        target_teleporter_use
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
}
//==========================================================
/*QUAKED target_relay (.5 .5 .5) (-8 -8 -8) (8 8 8) RED_ONLY BLUE_ONLY RANDOM
This doesn't perform any actions except fire its targets.
The activator can be forced to be from a certain team.
if RANDOM is checked, only one of the targets will be fired, not all of them
*/
#[no_mangle]

pub unsafe extern "C" fn target_relay_use(
    mut self_0: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    if (*self_0).spawnflags & 1 != 0
        && !(*activator).client.is_null()
        && (*(*activator).client).sess.sessionTeam != crate::bg_public_h::TEAM_RED
    {
        return;
    }
    if (*self_0).spawnflags & 2 != 0
        && !(*activator).client.is_null()
        && (*(*activator).client).sess.sessionTeam != crate::bg_public_h::TEAM_BLUE
    {
        return;
    }
    if (*self_0).spawnflags & 4 != 0 {
        let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
        ent = crate::src::game::g_utils::G_PickTarget((*self_0).target);
        if !ent.is_null() && (*ent).use_0.is_some() {
            (*ent).use_0.expect("non-null function pointer")(ent, self_0, activator);
        }
        return;
    }
    crate::src::game::g_utils::G_UseTargets(self_0, activator);
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_relay(mut self_0: *mut crate::g_local_h::gentity_t) {
    (*self_0).use_0 = Some(
        target_relay_use
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
}
//==========================================================
/*QUAKED target_kill (.5 .5 .5) (-8 -8 -8) (8 8 8)
Kills the activator.
*/
#[no_mangle]

pub unsafe extern "C" fn target_kill_use(
    mut _self_0: *mut crate::g_local_h::gentity_t,
    mut _other: *mut crate::g_local_h::gentity_t,
    mut activator: *mut crate::g_local_h::gentity_t,
) {
    crate::src::game::g_combat::G_Damage(
        activator,
        0 as *mut crate::g_local_h::gentity_t,
        0 as *mut crate::g_local_h::gentity_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        100000,
        0x8,
        crate::bg_public_h::MOD_TELEFRAG as i32,
    );
}
#[no_mangle]

pub unsafe extern "C" fn SP_target_kill(mut self_0: *mut crate::g_local_h::gentity_t) {
    (*self_0).use_0 = Some(
        target_kill_use
            as unsafe extern "C" fn(
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
                _: *mut crate::g_local_h::gentity_t,
            ) -> (),
    );
}
/*QUAKED target_position (0 0.5 0) (-4 -4 -4) (4 4 4)
Used as a positional target for in-game calculation, like jumppad targets.
*/
#[no_mangle]

pub unsafe extern "C" fn SP_target_position(mut self_0: *mut crate::g_local_h::gentity_t) {
    crate::src::game::g_utils::G_SetOrigin(self_0, (*self_0).s.origin.as_mut_ptr());
}

unsafe extern "C" fn target_location_linkup(mut ent: *mut crate::g_local_h::gentity_t) {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    if crate::src::game::g_main::level.locationLinked as u64 != 0 {
        return;
    }
    crate::src::game::g_main::level.locationLinked = crate::src::qcommon::q_shared::qtrue;
    crate::src::game::g_main::level.locationHead = 0 as *mut crate::g_local_h::gentity_t;
    crate::src::game::g_syscalls::trap_SetConfigstring(
        32 + 256 + 256 + 64,
        b"unknown\x00" as *const u8 as *const i8,
    );
    i = 0;
    ent = crate::src::game::g_main::g_entities.as_mut_ptr();
    n = 1;
    while i < crate::src::game::g_main::level.num_entities {
        if !(*ent).classname.is_null()
            && crate::src::qcommon::q_shared::Q_stricmp(
                (*ent).classname,
                b"target_location\x00" as *const u8 as *const i8,
            ) == 0
        {
            // lets overload some variables!
            (*ent).health = n; // use for location marking
            crate::src::game::g_syscalls::trap_SetConfigstring(
                32 + 256 + 256 + 64 + n,
                (*ent).message,
            );
            n += 1;
            (*ent).nextTrain = crate::src::game::g_main::level.locationHead;
            crate::src::game::g_main::level.locationHead = ent
        }
        i += 1;
        ent = ent.offset(1)
    }
    // All linked together now
}
/*QUAKED target_location (0 0.5 0) (-8 -8 -8) (8 8 8)
Set "message" to the name of this location.
Set "count" to 0-7 for color.
0:white 1:red 2:green 3:yellow 4:blue 5:cyan 6:magenta 7:white

Closest target_location in sight used for the location, if none
in site, closest in distance
*/
#[no_mangle]

pub unsafe extern "C" fn SP_target_location(mut self_0: *mut crate::g_local_h::gentity_t) {
    (*self_0).think = Some(
        target_location_linkup as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
    ); // Let them all spawn first
    (*self_0).nextthink = crate::src::game::g_main::level.time + 200;
    crate::src::game::g_utils::G_SetOrigin(self_0, (*self_0).s.origin.as_mut_ptr());
}
