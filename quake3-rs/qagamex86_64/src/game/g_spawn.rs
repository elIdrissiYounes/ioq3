use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }
    use crate::stdlib::strtod;
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}
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
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
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
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::g_items::G_SpawnItem;
pub use crate::src::game::g_main::g_doWarmup;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_motd;
pub use crate::src::game::g_main::g_restarted;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_LogPrintf;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_mem::G_Alloc;
pub use crate::src::game::g_spawn::stdlib_float_h::atof;
pub use crate::src::game::g_spawn::stdlib_h::atoi;
pub use crate::src::game::g_syscalls::trap_AdjustAreaPortalState;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_GetEntityToken;
pub use crate::src::game::g_syscalls::trap_LinkEntity;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_utils::G_FreeEntity;
pub use crate::src::game::g_utils::G_Spawn;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
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
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
use crate::stdlib::memcpy;
use crate::stdlib::sscanf;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
use crate::stdlib::strstr;
pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
extern "C" {
    #[no_mangle]
    pub fn SP_info_player_start(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_info_player_deathmatch(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_info_player_intermission(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_plat(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_static(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_rotating(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_bobbing(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_pendulum(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_button(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_door(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_train(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_func_timer(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_trigger_always(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_trigger_multiple(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_trigger_push(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_trigger_teleport(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_trigger_hurt(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_remove_powerups(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_give(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_delay(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_speaker(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_print(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_laser(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_score(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_teleporter(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_relay(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_kill(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_position(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_location(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_target_push(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_light(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_info_null(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_info_notnull(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_info_camp(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_path_corner(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_misc_teleporter_dest(self_0: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_misc_model(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_misc_portal_camera(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_misc_portal_surface(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_shooter_rocket(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_shooter_plasma(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_shooter_grenade(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_team_CTF_redplayer(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_team_CTF_blueplayer(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_team_CTF_redspawn(ent: *mut crate::g_local_h::gentity_t);
    #[no_mangle]
    pub fn SP_team_CTF_bluespawn(ent: *mut crate::g_local_h::gentity_t);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spawn_t {
    pub name: *mut i8,
    pub spawn: Option<unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> ()>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct field_t {
    pub name: *mut i8,
    pub ofs: crate::stddef_h::size_t,
    pub type_0: fieldtype_t,
}

pub type fieldtype_t = u32;

pub const F_ANGLEHACK: fieldtype_t = 4;

pub const F_VECTOR: fieldtype_t = 3;

pub const F_STRING: fieldtype_t = 2;

pub const F_FLOAT: fieldtype_t = 1;

pub const F_INT: fieldtype_t = 0;
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

pub unsafe extern "C" fn G_SpawnString(
    mut key: *const i8,
    mut defaultString: *const i8,
    mut out: *mut *mut i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    if crate::src::game::g_main::level.spawning as u64 == 0 {
        *out = defaultString as *mut i8
        //		G_Error( "G_SpawnString() called while not spawning" );
    }

    for i in 0..crate::src::game::g_main::level.numSpawnVars {
        if crate::src::qcommon::q_shared::Q_stricmp(
            key,
            crate::src::game::g_main::level.spawnVars[i as usize][0],
        ) == 0
        {
            *out = crate::src::game::g_main::level.spawnVars[i as usize][1];
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    *out = defaultString as *mut i8;
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]

pub unsafe extern "C" fn G_SpawnFloat(
    mut key: *const i8,
    mut defaultString: *const i8,
    mut out: *mut f32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut present: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    present = G_SpawnString(key, defaultString, &mut s);
    *out = atof(s) as f32;
    return present;
}
#[no_mangle]

pub unsafe extern "C" fn G_SpawnInt(
    mut key: *const i8,
    mut defaultString: *const i8,
    mut out: *mut i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut present: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    present = G_SpawnString(key, defaultString, &mut s);
    *out = atoi(s);
    return present;
}
#[no_mangle]

pub unsafe extern "C" fn G_SpawnVector(
    mut key: *const i8,
    mut defaultString: *const i8,
    mut out: *mut f32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut s: *mut i8 = 0 as *mut i8;
    let mut present: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    present = G_SpawnString(key, defaultString, &mut s);
    crate::stdlib::sscanf(
        s,
        b"%f %f %f\x00" as *const u8 as *const i8,
        &mut *out.offset(0isize) as *mut f32,
        &mut *out.offset(1isize) as *mut f32,
        &mut *out.offset(2isize) as *mut f32,
    );
    return present;
}
// Initialized in run_static_initializers
#[no_mangle]

pub static mut fields: [field_t; 20] = [field_t {
    name: 0 as *mut i8,
    ofs: 0,
    type_0: F_INT,
}; 20];
#[no_mangle]

pub unsafe extern "C" fn SP_item_botroam(mut ent: *mut crate::g_local_h::gentity_t) {}
#[no_mangle]

pub static mut spawns: [spawn_t; 49] = unsafe {
    [
        {
            let mut init = spawn_t {
                name: b"info_player_start\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_info_player_start
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_player_deathmatch\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_info_player_deathmatch
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_player_intermission\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_info_player_intermission
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_null\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_info_null as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_notnull\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_info_notnull
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_camp\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_info_camp as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_plat\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_plat as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_button\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_button
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_door\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_door as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_static\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_static
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_rotating\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_rotating
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_bobbing\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_bobbing
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_pendulum\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_pendulum
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_train\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_train
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_group\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_info_null as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_timer\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_func_timer
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_always\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_trigger_always
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_multiple\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_trigger_multiple
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_push\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_trigger_push
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_teleport\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_trigger_teleport
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_hurt\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_trigger_hurt
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_give\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_give
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_remove_powerups\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_remove_powerups
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_delay\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_delay
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_speaker\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_speaker
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_print\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_print
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_laser\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_laser
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_score\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_score
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_teleporter\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_teleporter
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_relay\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_relay
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_kill\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_kill
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_position\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_position
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_location\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_location
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_push\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_target_push
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"light\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_light as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"path_corner\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_path_corner
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_teleporter_dest\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_misc_teleporter_dest
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_model\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_misc_model
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_portal_surface\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_misc_portal_surface
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_portal_camera\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_misc_portal_camera
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"shooter_rocket\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_shooter_rocket
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"shooter_grenade\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_shooter_grenade
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"shooter_plasma\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_shooter_plasma
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"team_CTF_redplayer\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_team_CTF_redplayer
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"team_CTF_blueplayer\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_team_CTF_blueplayer
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"team_CTF_redspawn\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_team_CTF_redspawn
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"team_CTF_bluespawn\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_team_CTF_bluespawn
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"item_botroam\x00" as *const u8 as *mut i8,
                spawn: Some(
                    SP_item_botroam
                        as unsafe extern "C" fn(_: *mut crate::g_local_h::gentity_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: 0 as *mut i8,
                spawn: None,
            };
            init
        },
    ]
};
/*
===============
G_CallSpawn

Finds the spawn function for the entity and calls it,
returning qfalse if not found
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CallSpawn(
    mut ent: *mut crate::g_local_h::gentity_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut s: *mut spawn_t = 0 as *mut spawn_t;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    if (*ent).classname.is_null() {
        crate::src::game::g_main::G_Printf(
            b"G_CallSpawn: NULL classname\n\x00" as *const u8 as *const i8,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    // check item spawn functions
    item = crate::src::game::bg_misc::bg_itemlist
        .as_mut_ptr()
        .offset(1);
    while !(*item).classname.is_null() {
        if crate::stdlib::strcmp((*item).classname, (*ent).classname) == 0 {
            crate::src::game::g_items::G_SpawnItem(ent, item);
            return crate::src::qcommon::q_shared::qtrue;
        }
        item = item.offset(1)
    }
    // check normal spawn functions
    s = spawns.as_mut_ptr();
    while !(*s).name.is_null() {
        if crate::stdlib::strcmp((*s).name, (*ent).classname) == 0 {
            // found it
            (*s).spawn.expect("non-null function pointer")(ent);
            return crate::src::qcommon::q_shared::qtrue;
        }
        s = s.offset(1)
    }
    crate::src::game::g_main::G_Printf(
        b"%s doesn\'t have a spawn function\n\x00" as *const u8 as *const i8,
        (*ent).classname,
    );
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=============
G_NewString

Builds a copy of the string, translating \n to real linefeeds
so message texts can be multi-line
=============
*/
#[no_mangle]

pub unsafe extern "C" fn G_NewString(mut string: *const i8) -> *mut i8 {
    let mut newb: *mut i8 = 0 as *mut i8;
    let mut new_p: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut l: i32 = 0;
    l = crate::stdlib::strlen(string).wrapping_add(1usize) as i32;
    newb = crate::src::game::g_mem::G_Alloc(l) as *mut i8;
    new_p = newb;
    // turn \n into a real linefeed
    i = 0;
    while i < l {
        if *string.offset(i as isize) as i32 == '\\' as i32 && i < l - 1 {
            i += 1;
            if *string.offset(i as isize) as i32 == 'n' as i32 {
                let fresh0 = new_p;
                new_p = new_p.offset(1);
                *fresh0 = '\n' as i8
            } else {
                let fresh1 = new_p;
                new_p = new_p.offset(1);
                *fresh1 = '\\' as i8
            }
        } else {
            let fresh2 = new_p;
            new_p = new_p.offset(1);
            *fresh2 = *string.offset(i as isize)
        }
        i += 1
    }
    return newb;
}
/*
===============
G_ParseField

Takes a key/value pair and sets the binary values
in a gentity
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_ParseField(
    mut key: *const i8,
    mut value: *const i8,
    mut ent: *mut crate::g_local_h::gentity_t,
) {
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut b: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut v: f32 = 0.;
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    f = fields.as_mut_ptr();
    while !(*f).name.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp((*f).name, key) == 0 {
            // found it
            b = ent as *mut crate::src::qcommon::q_shared::byte;
            match (*f).type_0 {
                2 => {
                    let ref mut fresh3 = *(b.offset((*f).ofs as isize) as *mut *mut i8);
                    *fresh3 = G_NewString(value)
                }
                3 => {
                    crate::stdlib::sscanf(
                        value,
                        b"%f %f %f\x00" as *const u8 as *const i8,
                        &mut *vec.as_mut_ptr().offset(0isize)
                            as *mut crate::src::qcommon::q_shared::vec_t,
                        &mut *vec.as_mut_ptr().offset(1isize)
                            as *mut crate::src::qcommon::q_shared::vec_t,
                        &mut *vec.as_mut_ptr().offset(2isize)
                            as *mut crate::src::qcommon::q_shared::vec_t,
                    );
                    *(b.offset((*f).ofs as isize) as *mut f32).offset(0) = vec[0];
                    *(b.offset((*f).ofs as isize) as *mut f32).offset(1) = vec[1];
                    *(b.offset((*f).ofs as isize) as *mut f32).offset(2) = vec[2]
                }
                0 => *(b.offset((*f).ofs as isize) as *mut i32) = atoi(value),
                1 => *(b.offset((*f).ofs as isize) as *mut f32) = atof(value) as f32,
                4 => {
                    v = atof(value) as f32;
                    *(b.offset((*f).ofs as isize) as *mut f32).offset(0) = 0f32;
                    *(b.offset((*f).ofs as isize) as *mut f32).offset(1) = v;
                    *(b.offset((*f).ofs as isize) as *mut f32).offset(2) = 0f32
                }
                _ => {}
            }
            return;
        }
        f = f.offset(1)
    }
}
/*
===================
G_SpawnGEntityFromSpawnVars

Spawn an entity and fill in all of the level fields from
level.spawnVars[], then call the class specific spawn function
===================
*/
#[no_mangle]

pub unsafe extern "C" fn G_SpawnGEntityFromSpawnVars() {
    let mut i: i32 = 0;
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut value: *mut i8 = 0 as *mut i8;
    let mut gametypeName: *mut i8 = 0 as *mut i8;
    static mut gametypeNames: [*mut i8; 8] = [
        b"ffa\x00" as *const u8 as *mut i8,
        b"tournament\x00" as *const u8 as *mut i8,
        b"single\x00" as *const u8 as *mut i8,
        b"team\x00" as *const u8 as *mut i8,
        b"ctf\x00" as *const u8 as *mut i8,
        b"oneflag\x00" as *const u8 as *mut i8,
        b"obelisk\x00" as *const u8 as *mut i8,
        b"harvester\x00" as *const u8 as *mut i8,
    ];
    // get the next free entity
    ent = crate::src::game::g_utils::G_Spawn();
    i = 0;
    while i < crate::src::game::g_main::level.numSpawnVars {
        G_ParseField(
            crate::src::game::g_main::level.spawnVars[i as usize][0],
            crate::src::game::g_main::level.spawnVars[i as usize][1],
            ent,
        );
        i += 1
    }
    // check for "notsingle" flag
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_SINGLE_PLAYER as i32 {
        G_SpawnInt(
            b"notsingle\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
            &mut i,
        );
        if i != 0 {
            if (*ent).s.eType == crate::bg_public_h::ET_MOVER as i32 {
                crate::src::game::g_syscalls::trap_LinkEntity(ent);
                crate::src::game::g_syscalls::trap_AdjustAreaPortalState(
                    ent,
                    crate::src::qcommon::q_shared::qtrue,
                );
            }
            crate::src::game::g_utils::G_FreeEntity(ent);
            return;
        }
    }
    // check for "notteam" flag (GT_FFA, GT_TOURNAMENT, GT_SINGLE_PLAYER)
    if crate::src::game::g_main::g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        G_SpawnInt(
            b"notteam\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
            &mut i,
        );
        if i != 0 {
            if (*ent).s.eType == crate::bg_public_h::ET_MOVER as i32 {
                crate::src::game::g_syscalls::trap_LinkEntity(ent);
                crate::src::game::g_syscalls::trap_AdjustAreaPortalState(
                    ent,
                    crate::src::qcommon::q_shared::qtrue,
                );
            }
            crate::src::game::g_utils::G_FreeEntity(ent);
            return;
        }
    } else {
        G_SpawnInt(
            b"notfree\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
            &mut i,
        );
        if i != 0 {
            if (*ent).s.eType == crate::bg_public_h::ET_MOVER as i32 {
                crate::src::game::g_syscalls::trap_LinkEntity(ent);
                crate::src::game::g_syscalls::trap_AdjustAreaPortalState(
                    ent,
                    crate::src::qcommon::q_shared::qtrue,
                );
            }
            crate::src::game::g_utils::G_FreeEntity(ent);
            return;
        }
    }
    G_SpawnInt(
        b"notq3a\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        &mut i,
    );
    if i != 0 {
        if (*ent).s.eType == crate::bg_public_h::ET_MOVER as i32 {
            crate::src::game::g_syscalls::trap_LinkEntity(ent);
            crate::src::game::g_syscalls::trap_AdjustAreaPortalState(
                ent,
                crate::src::qcommon::q_shared::qtrue,
            );
        }
        crate::src::game::g_utils::G_FreeEntity(ent);
        return;
    }
    if G_SpawnString(
        b"gametype\x00" as *const u8 as *const i8,
        0 as *const i8,
        &mut value,
    ) as u64
        != 0
    {
        if crate::src::game::g_main::g_gametype.integer >= crate::bg_public_h::GT_FFA as i32
            && crate::src::game::g_main::g_gametype.integer
                < crate::bg_public_h::GT_MAX_GAME_TYPE as i32
        {
            gametypeName = gametypeNames[crate::src::game::g_main::g_gametype.integer as usize];
            s = crate::stdlib::strstr(value, gametypeName);
            if s.is_null() {
                if (*ent).s.eType == crate::bg_public_h::ET_MOVER as i32 {
                    crate::src::game::g_syscalls::trap_LinkEntity(ent);
                    crate::src::game::g_syscalls::trap_AdjustAreaPortalState(
                        ent,
                        crate::src::qcommon::q_shared::qtrue,
                    );
                }
                crate::src::game::g_utils::G_FreeEntity(ent);
                return;
            }
        }
    }
    // move editor origin to pos
    (*ent).s.pos.trBase[0] = (*ent).s.origin[0];
    (*ent).s.pos.trBase[1] = (*ent).s.origin[1];
    (*ent).s.pos.trBase[2] = (*ent).s.origin[2];
    (*ent).r.currentOrigin[0] = (*ent).s.origin[0];
    (*ent).r.currentOrigin[1] = (*ent).s.origin[1];
    (*ent).r.currentOrigin[2] = (*ent).s.origin[2];
    // if we didn't get a classname, don't bother spawning anything
    if G_CallSpawn(ent) as u64 == 0 {
        crate::src::game::g_utils::G_FreeEntity(ent);
    };
}
/*
====================
G_AddSpawnVarToken
====================
*/
#[no_mangle]

pub unsafe extern "C" fn G_AddSpawnVarToken(mut string: *const i8) -> *mut i8 {
    let mut l: i32 = 0;
    let mut dest: *mut i8 = 0 as *mut i8;
    l = crate::stdlib::strlen(string) as i32;
    if crate::src::game::g_main::level.numSpawnVarChars + l + 1 > 4096 {
        crate::src::game::g_main::G_Error(
            b"G_AddSpawnVarToken: MAX_SPAWN_VARS_CHARS\x00" as *const u8 as *const i8,
        );
    }
    dest = crate::src::game::g_main::level
        .spawnVarChars
        .as_mut_ptr()
        .offset(crate::src::game::g_main::level.numSpawnVarChars as isize);
    crate::stdlib::memcpy(
        dest as *mut libc::c_void,
        string as *const libc::c_void,
        (l + 1) as usize,
    );
    crate::src::game::g_main::level.numSpawnVarChars += l + 1;
    return dest;
}
/*
====================
G_ParseSpawnVars

Parses a brace bounded set of key / value pairs out of the
level's entity strings into level.spawnVars[]

This does not actually spawn an entity.
====================
*/
#[no_mangle]

pub unsafe extern "C" fn G_ParseSpawnVars() -> crate::src::qcommon::q_shared::qboolean {
    let mut keyname: [i8; 1024] = [0; 1024];
    let mut com_token: [i8; 1024] = [0; 1024];
    crate::src::game::g_main::level.numSpawnVars = 0;
    crate::src::game::g_main::level.numSpawnVarChars = 0;
    // parse the opening brace
    if crate::src::game::g_syscalls::trap_GetEntityToken(
        com_token.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    ) as u64
        == 0
    {
        // end of spawn string
        return crate::src::qcommon::q_shared::qfalse;
    }
    if com_token[0] as i32 != '{' as i32 {
        crate::src::game::g_main::G_Error(
            b"G_ParseSpawnVars: found %s when expecting {\x00" as *const u8 as *const i8,
            com_token.as_mut_ptr(),
        );
    }
    loop
    // go through all the key / value pairs
    // parse key
    {
        if crate::src::game::g_syscalls::trap_GetEntityToken(
            keyname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        ) as u64
            == 0
        {
            crate::src::game::g_main::G_Error(
                b"G_ParseSpawnVars: EOF without closing brace\x00" as *const u8 as *const i8,
            );
        }
        if keyname[0] as i32 == '}' as i32 {
            break;
        }
        // parse value
        if crate::src::game::g_syscalls::trap_GetEntityToken(
            com_token.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        ) as u64
            == 0
        {
            crate::src::game::g_main::G_Error(
                b"G_ParseSpawnVars: EOF without closing brace\x00" as *const u8 as *const i8,
            );
        }
        if com_token[0] as i32 == '}' as i32 {
            crate::src::game::g_main::G_Error(
                b"G_ParseSpawnVars: closing brace without data\x00" as *const u8 as *const i8,
            );
        }
        if crate::src::game::g_main::level.numSpawnVars == 64 {
            crate::src::game::g_main::G_Error(
                b"G_ParseSpawnVars: MAX_SPAWN_VARS\x00" as *const u8 as *const i8,
            );
        }
        crate::src::game::g_main::level.spawnVars
            [crate::src::game::g_main::level.numSpawnVars as usize][0] =
            G_AddSpawnVarToken(keyname.as_mut_ptr());
        crate::src::game::g_main::level.spawnVars
            [crate::src::game::g_main::level.numSpawnVars as usize][1] =
            G_AddSpawnVarToken(com_token.as_mut_ptr());
        crate::src::game::g_main::level.numSpawnVars += 1
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*QUAKED worldspawn (0 0 0) ?

Every map should have exactly one worldspawn.
"music"		music wav file
"gravity"	800 is default gravity
"message"	Text to print during connection process
*/
#[no_mangle]

pub unsafe extern "C" fn SP_worldspawn() {
    let mut s: *mut i8 = 0 as *mut i8;
    G_SpawnString(
        b"classname\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        &mut s,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(s, b"worldspawn\x00" as *const u8 as *const i8) != 0
    {
        crate::src::game::g_main::G_Error(
            b"SP_worldspawn: The first entity isn\'t \'worldspawn\'\x00" as *const u8 as *const i8,
        );
    }
    // make some data visible to connecting client
    crate::src::game::g_syscalls::trap_SetConfigstring(
        20,
        b"baseq3-1\x00" as *const u8 as *const i8,
    ); // map specific message
    crate::src::game::g_syscalls::trap_SetConfigstring(
        21,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *mut i8,
            crate::src::game::g_main::level.startTime,
        ),
    ); // message of the day
    G_SpawnString(
        b"music\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        &mut s,
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(2, s);
    G_SpawnString(
        b"message\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        &mut s,
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(3, s);
    crate::src::game::g_syscalls::trap_SetConfigstring(
        4,
        crate::src::game::g_main::g_motd.string.as_mut_ptr(),
    );
    G_SpawnString(
        b"gravity\x00" as *const u8 as *const i8,
        b"800\x00" as *const u8 as *const i8,
        &mut s,
    );
    crate::src::game::g_syscalls::trap_Cvar_Set(b"g_gravity\x00" as *const u8 as *const i8, s);
    G_SpawnString(
        b"enableDust\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        &mut s,
    );
    crate::src::game::g_syscalls::trap_Cvar_Set(b"g_enableDust\x00" as *const u8 as *const i8, s);
    G_SpawnString(
        b"enableBreath\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        &mut s,
    );
    crate::src::game::g_syscalls::trap_Cvar_Set(b"g_enableBreath\x00" as *const u8 as *const i8, s);
    crate::src::game::g_main::g_entities[(((1i32) << 10) - 2) as usize]
        .s
        .number = ((1) << 10) - 2;
    crate::src::game::g_main::g_entities[(((1i32) << 10) - 2) as usize]
        .r
        .ownerNum = ((1) << 10) - 1;
    crate::src::game::g_main::g_entities[(((1i32) << 10) - 2) as usize].classname =
        b"worldspawn\x00" as *const u8 as *mut i8;
    crate::src::game::g_main::g_entities[(((1i32) << 10) - 1) as usize]
        .s
        .number = ((1) << 10) - 1;
    crate::src::game::g_main::g_entities[(((1i32) << 10) - 1) as usize]
        .r
        .ownerNum = ((1) << 10) - 1;
    crate::src::game::g_main::g_entities[(((1i32) << 10) - 1) as usize].classname =
        b"nothing\x00" as *const u8 as *mut i8;
    // see if we want a warmup time
    crate::src::game::g_syscalls::trap_SetConfigstring(5, b"\x00" as *const u8 as *const i8);
    if crate::src::game::g_main::g_restarted.integer != 0 {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_restarted\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_main::level.warmupTime = 0
    } else if crate::src::game::g_main::g_doWarmup.integer != 0 {
        // Turn it on
        crate::src::game::g_main::level.warmupTime = -(1);
        crate::src::game::g_syscalls::trap_SetConfigstring(
            5,
            crate::src::qcommon::q_shared::va(
                b"%i\x00" as *const u8 as *mut i8,
                crate::src::game::g_main::level.warmupTime,
            ),
        );
        crate::src::game::g_main::G_LogPrintf(b"Warmup:\n\x00" as *const u8 as *const i8);
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
/*
==============
G_SpawnEntitiesFromString

Parses textual entity definitions out of an entstring and spawns gentities.
==============
*/
#[no_mangle]

pub unsafe extern "C" fn G_SpawnEntitiesFromString() {
    // allow calls to G_Spawn*()
    crate::src::game::g_main::level.spawning = crate::src::qcommon::q_shared::qtrue;
    crate::src::game::g_main::level.numSpawnVars = 0;
    // the worldspawn is not an actual entity, but it still
    // has a "spawn" function to perform any global setup
    // needed by a level (setting configstrings or cvars, etc)
    if G_ParseSpawnVars() as u64 == 0 {
        crate::src::game::g_main::G_Error(
            b"SpawnEntities: no entities\x00" as *const u8 as *const i8,
        );
    }
    SP_worldspawn();
    // parse ents
    while G_ParseSpawnVars() as u64 != 0 {
        G_SpawnGEntityFromSpawnVars();
    }
    crate::src::game::g_main::level.spawning = crate::src::qcommon::q_shared::qfalse;
    // any future calls to G_Spawn*() will be errors
}
unsafe extern "C" fn run_static_initializers() {
    fields = [
        {
            let mut init = field_t {
                name: b"classname\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).classname as *mut *mut i8
                    as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"origin\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).s.origin
                    as *mut crate::src::qcommon::q_shared::vec3_t
                    as crate::stddef_h::size_t,
                type_0: F_VECTOR,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"model\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).model as *mut *mut i8
                    as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"model2\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).model2 as *mut *mut i8
                    as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"spawnflags\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).spawnflags as *mut i32
                    as crate::stddef_h::size_t,
                type_0: F_INT,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"speed\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).speed as *mut f32
                    as crate::stddef_h::size_t,
                type_0: F_FLOAT,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"target\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).target as *mut *mut i8
                    as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"targetname\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetname as *mut *mut i8
                    as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"message\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).message as *mut *mut i8
                    as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"team\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).team as *mut *mut i8
                    as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"wait\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).wait as *mut f32
                    as crate::stddef_h::size_t,
                type_0: F_FLOAT,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"random\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).random as *mut f32
                    as crate::stddef_h::size_t,
                type_0: F_FLOAT,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"count\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).count as *mut i32
                    as crate::stddef_h::size_t,
                type_0: F_INT,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"health\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).health as *mut i32
                    as crate::stddef_h::size_t,
                type_0: F_INT,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"dmg\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).damage as *mut i32
                    as crate::stddef_h::size_t,
                type_0: F_INT,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"angles\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).s.angles
                    as *mut crate::src::qcommon::q_shared::vec3_t
                    as crate::stddef_h::size_t,
                type_0: F_VECTOR,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"angle\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).s.angles
                    as *mut crate::src::qcommon::q_shared::vec3_t
                    as crate::stddef_h::size_t,
                type_0: F_ANGLEHACK,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"targetShaderName\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetShaderName
                    as *mut *mut i8 as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"targetShaderNewName\x00" as *const u8 as *mut i8,
                ofs: &mut (*(0 as *mut crate::g_local_h::gentity_t)).targetShaderNewName
                    as *mut *mut i8 as crate::stddef_h::size_t,
                type_0: F_STRING,
            };
            init
        },
        {
            let mut init = field_t {
                name: 0 as *mut i8,
                ofs: 0,
                type_0: F_INT,
            };
            init
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
