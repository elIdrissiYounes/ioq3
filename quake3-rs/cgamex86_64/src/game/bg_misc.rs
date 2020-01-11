use ::libc;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::holdable_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::powerup_t;
pub use crate::bg_public_h::weapon_t;
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
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::HI_INVULNERABILITY;
pub use crate::bg_public_h::HI_KAMIKAZE;
pub use crate::bg_public_h::HI_MEDKIT;
pub use crate::bg_public_h::HI_NONE;
pub use crate::bg_public_h::HI_NUM_HOLDABLE;
pub use crate::bg_public_h::HI_PORTAL;
pub use crate::bg_public_h::HI_TELEPORTER;
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::PERS_ASSIST_COUNT;
pub use crate::bg_public_h::PERS_ATTACKEE_ARMOR;
pub use crate::bg_public_h::PERS_ATTACKER;
pub use crate::bg_public_h::PERS_CAPTURES;
pub use crate::bg_public_h::PERS_DEFEND_COUNT;
pub use crate::bg_public_h::PERS_EXCELLENT_COUNT;
pub use crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT;
pub use crate::bg_public_h::PERS_HITS;
pub use crate::bg_public_h::PERS_IMPRESSIVE_COUNT;
pub use crate::bg_public_h::PERS_KILLED;
pub use crate::bg_public_h::PERS_PLAYEREVENTS;
pub use crate::bg_public_h::PERS_RANK;
pub use crate::bg_public_h::PERS_SCORE;
pub use crate::bg_public_h::PERS_SPAWN_COUNT;
pub use crate::bg_public_h::PERS_TEAM;
pub use crate::bg_public_h::PM_DEAD;
pub use crate::bg_public_h::PM_FREEZE;
pub use crate::bg_public_h::PM_INTERMISSION;
pub use crate::bg_public_h::PM_NOCLIP;
pub use crate::bg_public_h::PM_NORMAL;
pub use crate::bg_public_h::PM_SPECTATOR;
pub use crate::bg_public_h::PM_SPINTERMISSION;
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
pub use crate::bg_public_h::STAT_ARMOR;
pub use crate::bg_public_h::STAT_CLIENTS_READY;
pub use crate::bg_public_h::STAT_DEAD_YAW;
pub use crate::bg_public_h::STAT_HEALTH;
pub use crate::bg_public_h::STAT_HOLDABLE_ITEM;
pub use crate::bg_public_h::STAT_MAX_HEALTH;
pub use crate::bg_public_h::STAT_WEAPONS;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::bg_public_h::WP_BFG;
pub use crate::bg_public_h::WP_GAUNTLET;
pub use crate::bg_public_h::WP_GRAPPLING_HOOK;
pub use crate::bg_public_h::WP_GRENADE_LAUNCHER;
pub use crate::bg_public_h::WP_LIGHTNING;
pub use crate::bg_public_h::WP_MACHINEGUN;
pub use crate::bg_public_h::WP_NONE;
pub use crate::bg_public_h::WP_NUM_WEAPONS;
pub use crate::bg_public_h::WP_PLASMAGUN;
pub use crate::bg_public_h::WP_RAILGUN;
pub use crate::bg_public_h::WP_ROCKET_LAUNCHER;
pub use crate::bg_public_h::WP_SHOTGUN;
pub use crate::src::cgame::cg_main::Com_Error;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_math::AngleNormalize180;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
use crate::stdlib::cos;
use crate::stdlib::fabs;
use crate::stdlib::sin;
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
// bg_misc.c -- both games misc functions, all completely stateless
/*QUAKED item_***** ( 0 0 0 ) (-16 -16 -16) (16 16 16) suspended
DO NOT USE THIS CLASS, IT JUST HOLDS GENERAL INFORMATION.
The suspended flag will allow items to hang in the air, otherwise they are dropped to the next surface.

If an item is the target of another entity, it will not spawn in until fired.

An item fires all of its targets when it is picked up.  If the toucher can't carry it, the targets won't be fired.

"notfree" if set to 1, don't spawn in free for all games
"notteam" if set to 1, don't spawn in team games
"notsingle" if set to 1, don't spawn in single player games
"wait"	override the default wait before respawning.  -1 = never respawn automatically, which can be used with targeted spawning.
"random" random number of plus or minus seconds varied from the respawn time
"count" override quantity or duration on most items.
*/
#[no_mangle]

pub static mut bg_itemlist: [crate::bg_public_h::gitem_t; 37] = [
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: 0 as *mut i8,
            pickup_sound: 0 as *mut i8,
            world_model: [0 as *mut i8, 0 as *mut i8, 0 as *mut i8, 0 as *mut i8],
            icon: 0 as *mut i8,
            pickup_name: 0 as *mut i8,
            quantity: 0,
            giType: crate::bg_public_h::IT_BAD,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_armor_shard\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/ar1_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/armor/shard.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/armor/shard_sphere.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconr_shard\x00" as *const u8 as *mut i8,
            pickup_name: b"Armor Shard\x00" as *const u8 as *mut i8,
            quantity: 5,
            giType: crate::bg_public_h::IT_ARMOR,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_armor_combat\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/ar2_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/armor/armor_yel.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconr_yellow\x00" as *const u8 as *mut i8,
            pickup_name: b"Armor\x00" as *const u8 as *mut i8,
            quantity: 50,
            giType: crate::bg_public_h::IT_ARMOR,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_armor_body\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/ar2_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/armor/armor_red.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconr_red\x00" as *const u8 as *mut i8,
            pickup_name: b"Heavy Armor\x00" as *const u8 as *mut i8,
            quantity: 100,
            giType: crate::bg_public_h::IT_ARMOR,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_health_small\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/s_health.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/health/small_cross.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/health/small_sphere.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconh_green\x00" as *const u8 as *mut i8,
            pickup_name: b"5 Health\x00" as *const u8 as *mut i8,
            quantity: 5,
            giType: crate::bg_public_h::IT_HEALTH,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_health\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/n_health.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/health/medium_cross.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/health/medium_sphere.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconh_yellow\x00" as *const u8 as *mut i8,
            pickup_name: b"25 Health\x00" as *const u8 as *mut i8,
            quantity: 25,
            giType: crate::bg_public_h::IT_HEALTH,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_health_large\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/l_health.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/health/large_cross.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/health/large_sphere.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconh_red\x00" as *const u8 as *mut i8,
            pickup_name: b"50 Health\x00" as *const u8 as *mut i8,
            quantity: 50,
            giType: crate::bg_public_h::IT_HEALTH,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_health_mega\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/m_health.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/health/mega_cross.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/health/mega_sphere.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconh_mega\x00" as *const u8 as *mut i8,
            pickup_name: b"Mega Health\x00" as *const u8 as *mut i8,
            quantity: 100,
            giType: crate::bg_public_h::IT_HEALTH,
            giTag: 0,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_gauntlet\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/gauntlet/gauntlet.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_gauntlet\x00" as *const u8 as *mut i8,
            pickup_name: b"Gauntlet\x00" as *const u8 as *mut i8,
            quantity: 0,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_GAUNTLET as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_shotgun\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/shotgun/shotgun.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_shotgun\x00" as *const u8 as *mut i8,
            pickup_name: b"Shotgun\x00" as *const u8 as *mut i8,
            quantity: 10,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_SHOTGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_machinegun\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/machinegun/machinegun.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_machinegun\x00" as *const u8 as *mut i8,
            pickup_name: b"Machinegun\x00" as *const u8 as *mut i8,
            quantity: 40,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_MACHINEGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_grenadelauncher\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/grenadel/grenadel.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_grenade\x00" as *const u8 as *mut i8,
            pickup_name: b"Grenade Launcher\x00" as *const u8 as *mut i8,
            quantity: 10,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_GRENADE_LAUNCHER as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"sound/weapons/grenade/hgrenb1a.wav sound/weapons/grenade/hgrenb2a.wav\x00"
                as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_rocketlauncher\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/rocketl/rocketl.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_rocket\x00" as *const u8 as *mut i8,
            pickup_name: b"Rocket Launcher\x00" as *const u8 as *mut i8,
            quantity: 10,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_ROCKET_LAUNCHER as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_lightning\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/lightning/lightning.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_lightning\x00" as *const u8 as *mut i8,
            pickup_name: b"Lightning Gun\x00" as *const u8 as *mut i8,
            quantity: 100,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_LIGHTNING as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_railgun\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/railgun/railgun.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_railgun\x00" as *const u8 as *mut i8,
            pickup_name: b"Railgun\x00" as *const u8 as *mut i8,
            quantity: 10,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_RAILGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_plasmagun\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/plasma/plasma.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_plasma\x00" as *const u8 as *mut i8,
            pickup_name: b"Plasma Gun\x00" as *const u8 as *mut i8,
            quantity: 50,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_PLASMAGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_bfg\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/bfg/bfg.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_bfg\x00" as *const u8 as *mut i8,
            pickup_name: b"BFG10K\x00" as *const u8 as *mut i8,
            quantity: 20,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_BFG as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"weapon_grapplinghook\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/weapons2/grapple/grapple.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconw_grapple\x00" as *const u8 as *mut i8,
            pickup_name: b"Grappling Hook\x00" as *const u8 as *mut i8,
            quantity: 0,
            giType: crate::bg_public_h::IT_WEAPON,
            giTag: crate::bg_public_h::WP_GRAPPLING_HOOK as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_shells\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/shotgunam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_shotgun\x00" as *const u8 as *mut i8,
            pickup_name: b"Shells\x00" as *const u8 as *mut i8,
            quantity: 10,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_SHOTGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_bullets\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/machinegunam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_machinegun\x00" as *const u8 as *mut i8,
            pickup_name: b"Bullets\x00" as *const u8 as *mut i8,
            quantity: 50,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_MACHINEGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_grenades\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/grenadeam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_grenade\x00" as *const u8 as *mut i8,
            pickup_name: b"Grenades\x00" as *const u8 as *mut i8,
            quantity: 5,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_GRENADE_LAUNCHER as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_cells\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/plasmaam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_plasma\x00" as *const u8 as *mut i8,
            pickup_name: b"Cells\x00" as *const u8 as *mut i8,
            quantity: 30,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_PLASMAGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_lightning\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/lightningam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_lightning\x00" as *const u8 as *mut i8,
            pickup_name: b"Lightning\x00" as *const u8 as *mut i8,
            quantity: 60,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_LIGHTNING as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_rockets\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/rocketam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_rocket\x00" as *const u8 as *mut i8,
            pickup_name: b"Rockets\x00" as *const u8 as *mut i8,
            quantity: 5,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_ROCKET_LAUNCHER as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_slugs\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/railgunam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_railgun\x00" as *const u8 as *mut i8,
            pickup_name: b"Slugs\x00" as *const u8 as *mut i8,
            quantity: 10,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_RAILGUN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"ammo_bfg\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/ammo/bfgam.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/icona_bfg\x00" as *const u8 as *mut i8,
            pickup_name: b"Bfg Ammo\x00" as *const u8 as *mut i8,
            quantity: 15,
            giType: crate::bg_public_h::IT_AMMO,
            giTag: crate::bg_public_h::WP_BFG as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"holdable_teleporter\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/holdable.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/holdable/teleporter.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/teleporter\x00" as *const u8 as *mut i8,
            pickup_name: b"Personal Teleporter\x00" as *const u8 as *mut i8,
            quantity: 60,
            giType: crate::bg_public_h::IT_HOLDABLE,
            giTag: crate::bg_public_h::HI_TELEPORTER as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"holdable_medkit\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/holdable.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/holdable/medkit.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/holdable/medkit_sphere.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/medkit\x00" as *const u8 as *mut i8,
            pickup_name: b"Medkit\x00" as *const u8 as *mut i8,
            quantity: 60,
            giType: crate::bg_public_h::IT_HOLDABLE,
            giTag: crate::bg_public_h::HI_MEDKIT as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"sound/items/use_medkit.wav\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_quad\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/quaddamage.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/instant/quad.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/instant/quad_ring.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/quad\x00" as *const u8 as *mut i8,
            pickup_name: b"Quad Damage\x00" as *const u8 as *mut i8,
            quantity: 30,
            giType: crate::bg_public_h::IT_POWERUP,
            giTag: crate::bg_public_h::PW_QUAD as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"sound/items/damage2.wav sound/items/damage3.wav\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_enviro\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/protect.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/instant/enviro.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/instant/enviro_ring.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/envirosuit\x00" as *const u8 as *mut i8,
            pickup_name: b"Battle Suit\x00" as *const u8 as *mut i8,
            quantity: 30,
            giType: crate::bg_public_h::IT_POWERUP,
            giTag: crate::bg_public_h::PW_BATTLESUIT as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"sound/items/airout.wav sound/items/protect3.wav\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_haste\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/haste.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/instant/haste.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/instant/haste_ring.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/haste\x00" as *const u8 as *mut i8,
            pickup_name: b"Speed\x00" as *const u8 as *mut i8,
            quantity: 30,
            giType: crate::bg_public_h::IT_POWERUP,
            giTag: crate::bg_public_h::PW_HASTE as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_invis\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/invisibility.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/instant/invis.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/instant/invis_ring.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/invis\x00" as *const u8 as *mut i8,
            pickup_name: b"Invisibility\x00" as *const u8 as *mut i8,
            quantity: 30,
            giType: crate::bg_public_h::IT_POWERUP,
            giTag: crate::bg_public_h::PW_INVIS as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_regen\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/regeneration.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/instant/regen.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/instant/regen_ring.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/regen\x00" as *const u8 as *mut i8,
            pickup_name: b"Regeneration\x00" as *const u8 as *mut i8,
            quantity: 30,
            giType: crate::bg_public_h::IT_POWERUP,
            giTag: crate::bg_public_h::PW_REGEN as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"sound/items/regen.wav\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"item_flight\x00" as *const u8 as *mut i8,
            pickup_sound: b"sound/items/flight.wav\x00" as *const u8 as *mut i8,
            world_model: [
                b"models/powerups/instant/flight.md3\x00" as *const u8 as *mut i8,
                b"models/powerups/instant/flight_ring.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/flight\x00" as *const u8 as *mut i8,
            pickup_name: b"Flight\x00" as *const u8 as *mut i8,
            quantity: 60,
            giType: crate::bg_public_h::IT_POWERUP,
            giTag: crate::bg_public_h::PW_FLIGHT as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"sound/items/flight.wav\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"team_CTF_redflag\x00" as *const u8 as *mut i8,
            pickup_sound: 0 as *mut i8,
            world_model: [
                b"models/flags/r_flag.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconf_red1\x00" as *const u8 as *mut i8,
            pickup_name: b"Red Flag\x00" as *const u8 as *mut i8,
            quantity: 0,
            giType: crate::bg_public_h::IT_TEAM,
            giTag: crate::bg_public_h::PW_REDFLAG as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: b"team_CTF_blueflag\x00" as *const u8 as *mut i8,
            pickup_sound: 0 as *mut i8,
            world_model: [
                b"models/flags/b_flag.md3\x00" as *const u8 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
                0 as *mut i8,
            ],
            icon: b"icons/iconf_blu1\x00" as *const u8 as *mut i8,
            pickup_name: b"Blue Flag\x00" as *const u8 as *mut i8,
            quantity: 0,
            giType: crate::bg_public_h::IT_TEAM,
            giTag: crate::bg_public_h::PW_BLUEFLAG as i32,
            precaches: b"\x00" as *const u8 as *mut i8,
            sounds: b"\x00" as *const u8 as *mut i8,
        };
        init
    },
    {
        let mut init = crate::bg_public_h::gitem_s {
            classname: 0 as *mut i8,
            pickup_sound: 0 as *mut i8,
            world_model: [0 as *mut i8; 4],
            icon: 0 as *mut i8,
            pickup_name: 0 as *mut i8,
            quantity: 0,
            giType: crate::bg_public_h::IT_BAD,
            giTag: 0,
            precaches: 0 as *mut i8,
            sounds: 0 as *mut i8,
        };
        init
    },
];
// Initialized in run_static_initializers
#[no_mangle]

pub static mut bg_numItems: i32 = 0;
/*
==============
BG_FindItemForPowerup
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BG_FindItemForPowerup(
    mut pw: crate::bg_public_h::powerup_t,
) -> *mut crate::bg_public_h::gitem_t {
    let mut i: i32 = 0;

    for i in 0..bg_numItems {
        if (bg_itemlist[i as usize].giType == crate::bg_public_h::IT_POWERUP
            || bg_itemlist[i as usize].giType == crate::bg_public_h::IT_TEAM
            || bg_itemlist[i as usize].giType == crate::bg_public_h::IT_PERSISTANT_POWERUP)
            && bg_itemlist[i as usize].giTag as u32 == pw
        {
            return &mut *bg_itemlist.as_mut_ptr().offset(i as isize)
                as *mut crate::bg_public_h::gitem_t;
        }
    }
    return 0 as *mut crate::bg_public_h::gitem_t;
}
/*
==============
BG_FindItemForHoldable
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BG_FindItemForHoldable(
    mut pw: crate::bg_public_h::holdable_t,
) -> *mut crate::bg_public_h::gitem_t {
    let mut i: i32 = 0;

    for i in 0..bg_numItems {
        if bg_itemlist[i as usize].giType == crate::bg_public_h::IT_HOLDABLE
            && bg_itemlist[i as usize].giTag as u32 == pw
        {
            return &mut *bg_itemlist.as_mut_ptr().offset(i as isize)
                as *mut crate::bg_public_h::gitem_t;
        }
    }
    crate::src::cgame::cg_main::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"HoldableItem not found\x00" as *const u8 as *const i8,
    );
}
/*
===============
BG_FindItemForWeapon

===============
*/
#[no_mangle]

pub unsafe extern "C" fn BG_FindItemForWeapon(
    mut weapon: crate::bg_public_h::weapon_t,
) -> *mut crate::bg_public_h::gitem_t {
    let mut it: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    it = bg_itemlist.as_mut_ptr().offset(1);
    while !(*it).classname.is_null() {
        if (*it).giType == crate::bg_public_h::IT_WEAPON && (*it).giTag as u32 == weapon {
            return it;
        }
        it = it.offset(1)
    }
    crate::src::cgame::cg_main::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"Couldn\'t find item for weapon %i\x00" as *const u8 as *const i8,
        weapon,
    );
}
// included in both the game dll and the client
/*
===============
BG_FindItem

===============
*/
#[no_mangle]

pub unsafe extern "C" fn BG_FindItem(
    mut pickupName: *const i8,
) -> *mut crate::bg_public_h::gitem_t {
    let mut it: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    it = bg_itemlist.as_mut_ptr().offset(1);
    while !(*it).classname.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp((*it).pickup_name, pickupName) == 0 {
            return it;
        }
        it = it.offset(1)
    }
    return 0 as *mut crate::bg_public_h::gitem_t;
}
/*
============
BG_PlayerTouchesItem

Items can be picked up without actually touching their physical bounds to make
grabbing them easier
============
*/
#[no_mangle]

pub unsafe extern "C" fn BG_PlayerTouchesItem(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut item: *mut crate::src::qcommon::q_shared::entityState_t,
    mut atTime: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    BG_EvaluateTrajectory(&mut (*item).pos, atTime, origin.as_mut_ptr());
    // we are ignoring ducked differences here
    if (*ps).origin[0] - origin[0] > 44f32
        || (*ps).origin[0] - origin[0] < -50f32
        || (*ps).origin[1] - origin[1] > 36f32
        || (*ps).origin[1] - origin[1] < -36f32
        || (*ps).origin[2] - origin[2] > 36f32
        || (*ps).origin[2] - origin[2] < -36f32
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
================
BG_CanItemBeGrabbed

Returns false if the item should not be picked up.
This needs to be the same for client side prediction and server use.
================
*/
#[no_mangle]

pub unsafe extern "C" fn BG_CanItemBeGrabbed(
    mut gametype: i32,
    mut ent: *const crate::src::qcommon::q_shared::entityState_t,
    mut ps: *const crate::src::qcommon::q_shared::playerState_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t; // weapons are always picked up
    if (*ent).modelindex < 1 || (*ent).modelindex >= bg_numItems {
        crate::src::cgame::cg_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"BG_CanItemBeGrabbed: index out of range\x00" as *const u8 as *const i8,
        );
    }
    item = &mut *bg_itemlist.as_mut_ptr().offset((*ent).modelindex as isize)
        as *mut crate::bg_public_h::gitem_t;
    match (*item).giType {
        1 => return crate::src::qcommon::q_shared::qtrue,
        2 => {
            if (*ps).ammo[(*item).giTag as usize] >= 200 {
                return crate::src::qcommon::q_shared::qfalse;
                // can't hold any more
            }
            return crate::src::qcommon::q_shared::qtrue;
        }
        3 => {
            if (*ps).stats[crate::bg_public_h::STAT_ARMOR as usize]
                >= (*ps).stats[crate::bg_public_h::STAT_MAX_HEALTH as usize] * 2
            {
                return crate::src::qcommon::q_shared::qfalse;
            }
            return crate::src::qcommon::q_shared::qtrue;
        }
        4 => {
            // small and mega healths will go over the max, otherwise
            // don't pick up if already at max
            if (*item).quantity == 5 || (*item).quantity == 100 {
                if (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize]
                    >= (*ps).stats[crate::bg_public_h::STAT_MAX_HEALTH as usize] * 2
                {
                    return crate::src::qcommon::q_shared::qfalse;
                } // powerups are always picked up
                return crate::src::qcommon::q_shared::qtrue;
            }
            if (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize]
                >= (*ps).stats[crate::bg_public_h::STAT_MAX_HEALTH as usize]
            {
                return crate::src::qcommon::q_shared::qfalse;
            }
            return crate::src::qcommon::q_shared::qtrue;
        }
        5 => return crate::src::qcommon::q_shared::qtrue,
        8 => {
            // team items, such as flags
            if gametype == crate::bg_public_h::GT_CTF as i32 {
                // ent->modelindex2 is non-zero on items if they are dropped
                // we need to know this because we can pick up our dropped flag (and return it)
                // but we can't pick up our flag at base
                if (*ps).persistant[crate::bg_public_h::PERS_TEAM as usize]
                    == crate::bg_public_h::TEAM_RED as i32
                {
                    if (*item).giTag == crate::bg_public_h::PW_BLUEFLAG as i32
                        || (*item).giTag == crate::bg_public_h::PW_REDFLAG as i32
                            && (*ent).modelindex2 != 0
                        || (*item).giTag == crate::bg_public_h::PW_REDFLAG as i32
                            && (*ps).powerups[crate::bg_public_h::PW_BLUEFLAG as usize] != 0
                    {
                        return crate::src::qcommon::q_shared::qtrue;
                    }
                } else if (*ps).persistant[crate::bg_public_h::PERS_TEAM as usize]
                    == crate::bg_public_h::TEAM_BLUE as i32
                {
                    if (*item).giTag == crate::bg_public_h::PW_REDFLAG as i32
                        || (*item).giTag == crate::bg_public_h::PW_BLUEFLAG as i32
                            && (*ent).modelindex2 != 0
                        || (*item).giTag == crate::bg_public_h::PW_BLUEFLAG as i32
                            && (*ps).powerups[crate::bg_public_h::PW_REDFLAG as usize] != 0
                    {
                        return crate::src::qcommon::q_shared::qtrue;
                    }
                }
            }
            return crate::src::qcommon::q_shared::qfalse;
        }
        6 => {
            // can only hold one item at a time
            if (*ps).stats[crate::bg_public_h::STAT_HOLDABLE_ITEM as usize] != 0 {
                return crate::src::qcommon::q_shared::qfalse;
            }
            return crate::src::qcommon::q_shared::qtrue;
        }
        0 => {
            crate::src::cgame::cg_main::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"BG_CanItemBeGrabbed: IT_BAD\x00" as *const u8 as *const i8,
            );
        }
        _ => {}
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//======================================================================
/*
================
BG_EvaluateTrajectory

================
*/
#[no_mangle]

pub unsafe extern "C" fn BG_EvaluateTrajectory(
    mut tr: *const crate::src::qcommon::q_shared::trajectory_t,
    mut atTime: i32,
    mut result: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut deltaTime: f32 = 0.; // milliseconds to seconds
    let mut phase: f32 = 0.; // milliseconds to seconds
    match (*tr).trType {
        0 | 1 => {
            *result.offset(0) = (*tr).trBase[0]; // milliseconds to seconds
            *result.offset(1) = (*tr).trBase[1]; // FIXME: local gravity...
            *result.offset(2) = (*tr).trBase[2]
        }
        2 => {
            deltaTime = ((atTime - (*tr).trTime) as f64 * 0.001) as f32;
            *result.offset(0) = (*tr).trBase[0] + (*tr).trDelta[0] * deltaTime;
            *result.offset(1) = (*tr).trBase[1] + (*tr).trDelta[1] * deltaTime;
            *result.offset(2) = (*tr).trBase[2] + (*tr).trDelta[2] * deltaTime
        }
        4 => {
            deltaTime = (atTime - (*tr).trTime) as f32 / (*tr).trDuration as f32;
            phase = crate::stdlib::sin(deltaTime as f64 * 3.14159265358979323846 * 2f64) as f32;
            *result.offset(0) = (*tr).trBase[0] + (*tr).trDelta[0] * phase;
            *result.offset(1) = (*tr).trBase[1] + (*tr).trDelta[1] * phase;
            *result.offset(2) = (*tr).trBase[2] + (*tr).trDelta[2] * phase
        }
        3 => {
            if atTime > (*tr).trTime + (*tr).trDuration {
                atTime = (*tr).trTime + (*tr).trDuration
            }
            deltaTime = ((atTime - (*tr).trTime) as f64 * 0.001) as f32;
            if deltaTime < 0f32 {
                deltaTime = 0f32
            }
            *result.offset(0) = (*tr).trBase[0] + (*tr).trDelta[0] * deltaTime;
            *result.offset(1) = (*tr).trBase[1] + (*tr).trDelta[1] * deltaTime;
            *result.offset(2) = (*tr).trBase[2] + (*tr).trDelta[2] * deltaTime
        }
        5 => {
            deltaTime = ((atTime - (*tr).trTime) as f64 * 0.001) as f32;
            *result.offset(0) = (*tr).trBase[0] + (*tr).trDelta[0] * deltaTime;
            *result.offset(1) = (*tr).trBase[1] + (*tr).trDelta[1] * deltaTime;
            *result.offset(2) = (*tr).trBase[2] + (*tr).trDelta[2] * deltaTime;
            let ref mut fresh0 = *result.offset(2);
            *fresh0 = (*fresh0 as f64 - 0.5 * 800f64 * deltaTime as f64 * deltaTime as f64)
                as crate::src::qcommon::q_shared::vec_t
        }
        _ => {
            crate::src::cgame::cg_main::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"BG_EvaluateTrajectory: unknown trType: %i\x00" as *const u8 as *const i8,
                (*tr).trType,
            );
        }
    };
}
/*
================
BG_EvaluateTrajectoryDelta

For determining velocity at a given time
================
*/
#[no_mangle]

pub unsafe extern "C" fn BG_EvaluateTrajectoryDelta(
    mut tr: *const crate::src::qcommon::q_shared::trajectory_t,
    mut atTime: i32,
    mut result: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut deltaTime: f32 = 0.; // derivative of sin = cos
    let mut phase: f32 = 0.; // milliseconds to seconds
    match (*tr).trType {
        0 | 1 => {
            let ref mut fresh1 = *result.offset(2); // FIXME: local gravity...
            *fresh1 = 0f32;
            let ref mut fresh2 = *result.offset(1);
            *fresh2 = *fresh1;
            *result.offset(0) = *fresh2
        }
        2 => {
            *result.offset(0) = (*tr).trDelta[0];
            *result.offset(1) = (*tr).trDelta[1];
            *result.offset(2) = (*tr).trDelta[2]
        }
        4 => {
            deltaTime = (atTime - (*tr).trTime) as f32 / (*tr).trDuration as f32;
            phase = crate::stdlib::cos(deltaTime as f64 * 3.14159265358979323846 * 2f64) as f32;
            phase = (phase as f64 * 0.5) as f32;
            *result.offset(0) = (*tr).trDelta[0] * phase;
            *result.offset(1) = (*tr).trDelta[1] * phase;
            *result.offset(2) = (*tr).trDelta[2] * phase
        }
        3 => {
            if atTime > (*tr).trTime + (*tr).trDuration {
                let ref mut fresh3 = *result.offset(2);
                *fresh3 = 0f32;
                let ref mut fresh4 = *result.offset(1);
                *fresh4 = *fresh3;
                *result.offset(0) = *fresh4;
                return;
            }
            *result.offset(0) = (*tr).trDelta[0];
            *result.offset(1) = (*tr).trDelta[1];
            *result.offset(2) = (*tr).trDelta[2]
        }
        5 => {
            deltaTime = ((atTime - (*tr).trTime) as f64 * 0.001) as f32;
            *result.offset(0) = (*tr).trDelta[0];
            *result.offset(1) = (*tr).trDelta[1];
            *result.offset(2) = (*tr).trDelta[2];
            let ref mut fresh5 = *result.offset(2);
            *fresh5 -= 800f32 * deltaTime
        }
        _ => {
            crate::src::cgame::cg_main::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"BG_EvaluateTrajectoryDelta: unknown trType: %i\x00" as *const u8 as *const i8,
                (*tr).trType,
            );
        }
    };
}
#[no_mangle]

pub static mut eventnames: [*mut i8; 83] = [
    b"EV_NONE\x00" as *const u8 as *mut i8,
    b"EV_FOOTSTEP\x00" as *const u8 as *mut i8,
    b"EV_FOOTSTEP_METAL\x00" as *const u8 as *mut i8,
    b"EV_FOOTSPLASH\x00" as *const u8 as *mut i8,
    b"EV_FOOTWADE\x00" as *const u8 as *mut i8,
    b"EV_SWIM\x00" as *const u8 as *mut i8,
    b"EV_STEP_4\x00" as *const u8 as *mut i8,
    b"EV_STEP_8\x00" as *const u8 as *mut i8,
    b"EV_STEP_12\x00" as *const u8 as *mut i8,
    b"EV_STEP_16\x00" as *const u8 as *mut i8,
    b"EV_FALL_SHORT\x00" as *const u8 as *mut i8,
    b"EV_FALL_MEDIUM\x00" as *const u8 as *mut i8,
    b"EV_FALL_FAR\x00" as *const u8 as *mut i8,
    b"EV_JUMP_PAD\x00" as *const u8 as *mut i8,
    b"EV_JUMP\x00" as *const u8 as *mut i8,
    b"EV_WATER_TOUCH\x00" as *const u8 as *mut i8,
    b"EV_WATER_LEAVE\x00" as *const u8 as *mut i8,
    b"EV_WATER_UNDER\x00" as *const u8 as *mut i8,
    b"EV_WATER_CLEAR\x00" as *const u8 as *mut i8,
    b"EV_ITEM_PICKUP\x00" as *const u8 as *mut i8,
    b"EV_GLOBAL_ITEM_PICKUP\x00" as *const u8 as *mut i8,
    b"EV_NOAMMO\x00" as *const u8 as *mut i8,
    b"EV_CHANGE_WEAPON\x00" as *const u8 as *mut i8,
    b"EV_FIRE_WEAPON\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM0\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM1\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM2\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM3\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM4\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM5\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM6\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM7\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM8\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM9\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM10\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM11\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM12\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM13\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM14\x00" as *const u8 as *mut i8,
    b"EV_USE_ITEM15\x00" as *const u8 as *mut i8,
    b"EV_ITEM_RESPAWN\x00" as *const u8 as *mut i8,
    b"EV_ITEM_POP\x00" as *const u8 as *mut i8,
    b"EV_PLAYER_TELEPORT_IN\x00" as *const u8 as *mut i8,
    b"EV_PLAYER_TELEPORT_OUT\x00" as *const u8 as *mut i8,
    b"EV_GRENADE_BOUNCE\x00" as *const u8 as *mut i8,
    b"EV_GENERAL_SOUND\x00" as *const u8 as *mut i8,
    b"EV_GLOBAL_SOUND\x00" as *const u8 as *mut i8,
    b"EV_GLOBAL_TEAM_SOUND\x00" as *const u8 as *mut i8,
    b"EV_BULLET_HIT_FLESH\x00" as *const u8 as *mut i8,
    b"EV_BULLET_HIT_WALL\x00" as *const u8 as *mut i8,
    b"EV_MISSILE_HIT\x00" as *const u8 as *mut i8,
    b"EV_MISSILE_MISS\x00" as *const u8 as *mut i8,
    b"EV_MISSILE_MISS_METAL\x00" as *const u8 as *mut i8,
    b"EV_RAILTRAIL\x00" as *const u8 as *mut i8,
    b"EV_SHOTGUN\x00" as *const u8 as *mut i8,
    b"EV_BULLET\x00" as *const u8 as *mut i8,
    b"EV_PAIN\x00" as *const u8 as *mut i8,
    b"EV_DEATH1\x00" as *const u8 as *mut i8,
    b"EV_DEATH2\x00" as *const u8 as *mut i8,
    b"EV_DEATH3\x00" as *const u8 as *mut i8,
    b"EV_OBITUARY\x00" as *const u8 as *mut i8,
    b"EV_POWERUP_QUAD\x00" as *const u8 as *mut i8,
    b"EV_POWERUP_BATTLESUIT\x00" as *const u8 as *mut i8,
    b"EV_POWERUP_REGEN\x00" as *const u8 as *mut i8,
    b"EV_GIB_PLAYER\x00" as *const u8 as *mut i8,
    b"EV_SCOREPLUM\x00" as *const u8 as *mut i8,
    b"EV_PROXIMITY_MINE_STICK\x00" as *const u8 as *mut i8,
    b"EV_PROXIMITY_MINE_TRIGGER\x00" as *const u8 as *mut i8,
    b"EV_KAMIKAZE\x00" as *const u8 as *mut i8,
    b"EV_OBELISKEXPLODE\x00" as *const u8 as *mut i8,
    b"EV_OBELISKPAIN\x00" as *const u8 as *mut i8,
    b"EV_INVUL_IMPACT\x00" as *const u8 as *mut i8,
    b"EV_JUICED\x00" as *const u8 as *mut i8,
    b"EV_LIGHTNINGBOLT\x00" as *const u8 as *mut i8,
    b"EV_DEBUG_LINE\x00" as *const u8 as *mut i8,
    b"EV_STOPLOOPINGSOUND\x00" as *const u8 as *mut i8,
    b"EV_TAUNT\x00" as *const u8 as *mut i8,
    b"EV_TAUNT_YES\x00" as *const u8 as *mut i8,
    b"EV_TAUNT_NO\x00" as *const u8 as *mut i8,
    b"EV_TAUNT_FOLLOWME\x00" as *const u8 as *mut i8,
    b"EV_TAUNT_GETFLAG\x00" as *const u8 as *mut i8,
    b"EV_TAUNT_GUARDBASE\x00" as *const u8 as *mut i8,
    b"EV_TAUNT_PATROL\x00" as *const u8 as *mut i8,
];
#[no_mangle]

pub unsafe extern "C" fn BG_AddPredictableEventToPlayerstate(
    mut newEvent: i32,
    mut eventParm: i32,
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    (*ps).events[((*ps).eventSequence & 2i32 - 1) as usize] = newEvent;
    (*ps).eventParms[((*ps).eventSequence & 2i32 - 1) as usize] = eventParm;
    (*ps).eventSequence += 1;
}
/*
========================
BG_TouchJumpPad
========================
*/
#[no_mangle]

pub unsafe extern "C" fn BG_TouchJumpPad(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut jumppad: *mut crate::src::qcommon::q_shared::entityState_t,
) {
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut p: f32 = 0.;
    let mut effectNum: i32 = 0;
    // spectators don't use jump pads
    if (*ps).pm_type != crate::bg_public_h::PM_NORMAL as i32 {
        return;
    }
    // flying characters don't hit bounce pads
    if (*ps).powerups[crate::bg_public_h::PW_FLIGHT as usize] != 0 {
        return;
    }
    // if we didn't hit this same jumppad the previous frame
    // then don't play the event sound again if we are in a fat trigger
    if (*ps).jumppad_ent != (*jumppad).number {
        crate::src::qcommon::q_math::vectoangles(
            (*jumppad).origin2.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            angles.as_mut_ptr(),
        );
        p = crate::stdlib::fabs(crate::src::qcommon::q_math::AngleNormalize180(angles[0]) as f64)
            as f32;
        if p < 45f32 {
            effectNum = 0
        } else {
            effectNum = 1
        }
        BG_AddPredictableEventToPlayerstate(crate::bg_public_h::EV_JUMP_PAD as i32, effectNum, ps);
    }
    // remember hitting this jumppad this frame
    (*ps).jumppad_ent = (*jumppad).number;
    (*ps).jumppad_frame = (*ps).pmove_framecount;
    // give the player the velocity from the jumppad
    (*ps).velocity[0] = (*jumppad).origin2[0];
    (*ps).velocity[1] = (*jumppad).origin2[1];
    (*ps).velocity[2] = (*jumppad).origin2[2];
}
/*
========================
BG_PlayerStateToEntityState

This is done after each set of usercmd_t on the server,
and after local prediction on the client
========================
*/
#[no_mangle]

pub unsafe extern "C" fn BG_PlayerStateToEntityState(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut s: *mut crate::src::qcommon::q_shared::entityState_t,
    mut snap: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: i32 = 0;
    if (*ps).pm_type == crate::bg_public_h::PM_INTERMISSION as i32
        || (*ps).pm_type == crate::bg_public_h::PM_SPECTATOR as i32
    {
        (*s).eType = crate::bg_public_h::ET_INVISIBLE as i32
    } else if (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= -(40) {
        (*s).eType = crate::bg_public_h::ET_INVISIBLE as i32
    } else {
        (*s).eType = crate::bg_public_h::ET_PLAYER as i32
    }
    (*s).number = (*ps).clientNum;
    (*s).pos.trType = crate::src::qcommon::q_shared::TR_INTERPOLATE;
    (*s).pos.trBase[0] = (*ps).origin[0];
    (*s).pos.trBase[1] = (*ps).origin[1];
    (*s).pos.trBase[2] = (*ps).origin[2];
    if snap as u64 != 0 {
        (*s).pos.trBase[0] = (*s).pos.trBase[0] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).pos.trBase[1] = (*s).pos.trBase[1] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).pos.trBase[2] = (*s).pos.trBase[2] as i32 as crate::src::qcommon::q_shared::vec_t
    }
    // set the trDelta for flag direction
    (*s).pos.trDelta[0] = (*ps).velocity[0]; // ET_PLAYER looks here instead of at number
    (*s).pos.trDelta[1] = (*ps).velocity[1];
    (*s).pos.trDelta[2] = (*ps).velocity[2];
    (*s).apos.trType = crate::src::qcommon::q_shared::TR_INTERPOLATE;
    (*s).apos.trBase[0] = (*ps).viewangles[0];
    (*s).apos.trBase[1] = (*ps).viewangles[1];
    (*s).apos.trBase[2] = (*ps).viewangles[2];
    if snap as u64 != 0 {
        (*s).apos.trBase[0] = (*s).apos.trBase[0] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).apos.trBase[1] = (*s).apos.trBase[1] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).apos.trBase[2] = (*s).apos.trBase[2] as i32 as crate::src::qcommon::q_shared::vec_t
    }
    (*s).angles2[1] = (*ps).movementDir as crate::src::qcommon::q_shared::vec_t;
    (*s).legsAnim = (*ps).legsAnim;
    (*s).torsoAnim = (*ps).torsoAnim;
    (*s).clientNum = (*ps).clientNum;
    // so corpses can also reference the proper config
    (*s).eFlags = (*ps).eFlags;
    if (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= 0 {
        (*s).eFlags |= 0x1
    } else {
        (*s).eFlags &= !(0x1)
    }
    if (*ps).externalEvent != 0 {
        (*s).event = (*ps).externalEvent;
        (*s).eventParm = (*ps).externalEventParm
    } else if (*ps).entityEventSequence < (*ps).eventSequence {
        let mut seq: i32 = 0;
        if (*ps).entityEventSequence < (*ps).eventSequence - 2 {
            (*ps).entityEventSequence = (*ps).eventSequence - 2
        }
        seq = (*ps).entityEventSequence & 2 - 1;
        (*s).event = (*ps).events[seq as usize] | ((*ps).entityEventSequence & 3) << 8;
        (*s).eventParm = (*ps).eventParms[seq as usize];
        (*ps).entityEventSequence += 1
    }
    (*s).weapon = (*ps).weapon;
    (*s).groundEntityNum = (*ps).groundEntityNum;
    (*s).powerups = 0;

    for i in 0..16 {
        if (*ps).powerups[i as usize] != 0 {
            (*s).powerups |= (1) << i
        }
    }
    (*s).loopSound = (*ps).loopSound;
    (*s).generic1 = (*ps).generic1;
}
/*
========================
BG_PlayerStateToEntityStateExtraPolate

This is done after each set of usercmd_t on the server,
and after local prediction on the client
========================
*/
#[no_mangle]

pub unsafe extern "C" fn BG_PlayerStateToEntityStateExtraPolate(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut s: *mut crate::src::qcommon::q_shared::entityState_t,
    mut time: i32,
    mut snap: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: i32 = 0;
    if (*ps).pm_type == crate::bg_public_h::PM_INTERMISSION as i32
        || (*ps).pm_type == crate::bg_public_h::PM_SPECTATOR as i32
    {
        (*s).eType = crate::bg_public_h::ET_INVISIBLE as i32
    } else if (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= -(40) {
        (*s).eType = crate::bg_public_h::ET_INVISIBLE as i32
    } else {
        (*s).eType = crate::bg_public_h::ET_PLAYER as i32
    }
    (*s).number = (*ps).clientNum;
    (*s).pos.trType = crate::src::qcommon::q_shared::TR_LINEAR_STOP;
    (*s).pos.trBase[0] = (*ps).origin[0];
    (*s).pos.trBase[1] = (*ps).origin[1];
    (*s).pos.trBase[2] = (*ps).origin[2];
    if snap as u64 != 0 {
        (*s).pos.trBase[0] = (*s).pos.trBase[0] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).pos.trBase[1] = (*s).pos.trBase[1] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).pos.trBase[2] = (*s).pos.trBase[2] as i32 as crate::src::qcommon::q_shared::vec_t
    }
    // set the trDelta for flag direction and linear prediction
    (*s).pos.trDelta[0] = (*ps).velocity[0];
    (*s).pos.trDelta[1] = (*ps).velocity[1];
    (*s).pos.trDelta[2] = (*ps).velocity[2];
    // set the time for linear prediction
    (*s).pos.trTime = time;
    // set maximum extra polation time
    (*s).pos.trDuration = 50; // 1000 / sv_fps (default = 20)
    (*s).apos.trType = crate::src::qcommon::q_shared::TR_INTERPOLATE; // ET_PLAYER looks here instead of at number
    (*s).apos.trBase[0] = (*ps).viewangles[0];
    (*s).apos.trBase[1] = (*ps).viewangles[1];
    (*s).apos.trBase[2] = (*ps).viewangles[2];
    if snap as u64 != 0 {
        (*s).apos.trBase[0] = (*s).apos.trBase[0] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).apos.trBase[1] = (*s).apos.trBase[1] as i32 as crate::src::qcommon::q_shared::vec_t;
        (*s).apos.trBase[2] = (*s).apos.trBase[2] as i32 as crate::src::qcommon::q_shared::vec_t
    }
    (*s).angles2[1] = (*ps).movementDir as crate::src::qcommon::q_shared::vec_t;
    (*s).legsAnim = (*ps).legsAnim;
    (*s).torsoAnim = (*ps).torsoAnim;
    (*s).clientNum = (*ps).clientNum;
    // so corpses can also reference the proper config
    (*s).eFlags = (*ps).eFlags;
    if (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= 0 {
        (*s).eFlags |= 0x1
    } else {
        (*s).eFlags &= !(0x1)
    }
    if (*ps).externalEvent != 0 {
        (*s).event = (*ps).externalEvent;
        (*s).eventParm = (*ps).externalEventParm
    } else if (*ps).entityEventSequence < (*ps).eventSequence {
        let mut seq: i32 = 0;
        if (*ps).entityEventSequence < (*ps).eventSequence - 2 {
            (*ps).entityEventSequence = (*ps).eventSequence - 2
        }
        seq = (*ps).entityEventSequence & 2 - 1;
        (*s).event = (*ps).events[seq as usize] | ((*ps).entityEventSequence & 3) << 8;
        (*s).eventParm = (*ps).eventParms[seq as usize];
        (*ps).entityEventSequence += 1
    }
    (*s).weapon = (*ps).weapon;
    (*s).groundEntityNum = (*ps).groundEntityNum;
    (*s).powerups = 0;

    for i in 0..16 {
        if (*ps).powerups[i as usize] != 0 {
            (*s).powerups |= (1) << i
        }
    }
    (*s).loopSound = (*ps).loopSound;
    (*s).generic1 = (*ps).generic1;
}
unsafe extern "C" fn run_static_initializers() {
    bg_numItems = (::std::mem::size_of::<[crate::bg_public_h::gitem_t; 37]>())
        .wrapping_div(::std::mem::size_of::<crate::bg_public_h::gitem_t>())
        .wrapping_sub(1usize) as i32
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
