use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0) * *v.offset(0)
                + *v.offset(1) * *v.offset(1)
                + *v.offset(2) * *v.offset(2)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn Distance(
        mut p1: *const crate::src::qcommon::q_shared::vec_t,
        mut p2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        v[0] = *p2.offset(0) - *p1.offset(0);
        v[1] = *p2.offset(1) - *p1.offset(1);
        v[2] = *p2.offset(2) - *p1.offset(2);
        return VectorLength(v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    }
    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0) = *v1.offset(1) * *v2.offset(2) - *v1.offset(2) * *v2.offset(1);
        *cross.offset(1) = *v1.offset(2) * *v2.offset(0) - *v1.offset(0) * *v2.offset(2);
        *cross.offset(2) = *v1.offset(0) * *v2.offset(1) - *v1.offset(1) * *v2.offset(0);
    }

    // __Q_SHARED_H
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::weapon_t;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
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
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
pub use crate::bg_public_h::GENDER_FEMALE;
pub use crate::bg_public_h::GENDER_MALE;
pub use crate::bg_public_h::GENDER_NEUTER;
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
pub use crate::bg_public_h::LEGS_BACK;
pub use crate::bg_public_h::LEGS_BACKCR;
pub use crate::bg_public_h::LEGS_BACKWALK;
pub use crate::bg_public_h::LEGS_IDLE;
pub use crate::bg_public_h::LEGS_IDLECR;
pub use crate::bg_public_h::LEGS_JUMP;
pub use crate::bg_public_h::LEGS_JUMPB;
pub use crate::bg_public_h::LEGS_LAND;
pub use crate::bg_public_h::LEGS_LANDB;
pub use crate::bg_public_h::LEGS_RUN;
pub use crate::bg_public_h::LEGS_SWIM;
pub use crate::bg_public_h::LEGS_TURN;
pub use crate::bg_public_h::LEGS_WALK;
pub use crate::bg_public_h::LEGS_WALKCR;
pub use crate::bg_public_h::MAX_ANIMATIONS;
pub use crate::bg_public_h::MAX_TOTALANIMATIONS;
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
pub use crate::bg_public_h::TORSO_AFFIRMATIVE;
pub use crate::bg_public_h::TORSO_ATTACK;
pub use crate::bg_public_h::TORSO_ATTACK2;
pub use crate::bg_public_h::TORSO_DROP;
pub use crate::bg_public_h::TORSO_FOLLOWME;
pub use crate::bg_public_h::TORSO_GESTURE;
pub use crate::bg_public_h::TORSO_GETFLAG;
pub use crate::bg_public_h::TORSO_GUARDBASE;
pub use crate::bg_public_h::TORSO_NEGATIVE;
pub use crate::bg_public_h::TORSO_PATROL;
pub use crate::bg_public_h::TORSO_RAISE;
pub use crate::bg_public_h::TORSO_STAND;
pub use crate::bg_public_h::TORSO_STAND2;
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
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_weapons::q_shared_h::CrossProduct;
pub use crate::src::cgame::cg_weapons::q_shared_h::Distance;
pub use crate::src::cgame::cg_weapons::q_shared_h::VectorLength;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::bg_numItems;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::qcommon::q_math::axisDefault;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleMod;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_math::AxisCopy;
pub use crate::src::qcommon::q_math::MatrixMultiply;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::Q_crandom;
pub use crate::src::qcommon::q_math::RotatePointAroundVector;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::gameState_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::polyVert_t;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::impactSound_t;
pub use crate::cg_local_h::itemInfo_t;
pub use crate::cg_local_h::leBounceSoundType_t;
pub use crate::cg_local_h::leMarkType_t;
pub use crate::cg_local_h::leType_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::localEntity_s;
pub use crate::cg_local_h::localEntity_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::weaponInfo_s;
pub use crate::cg_local_h::weaponInfo_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_local_h::IMPACTSOUND_DEFAULT;
pub use crate::cg_local_h::IMPACTSOUND_FLESH;
pub use crate::cg_local_h::IMPACTSOUND_METAL;
pub use crate::cg_local_h::LEBS_BLOOD;
pub use crate::cg_local_h::LEBS_BRASS;
pub use crate::cg_local_h::LEBS_NONE;
pub use crate::cg_local_h::LEF_PUFF_DONT_SCALE;
pub use crate::cg_local_h::LEF_SOUND1;
pub use crate::cg_local_h::LEF_SOUND2;
pub use crate::cg_local_h::LEF_TUMBLE;
pub use crate::cg_local_h::LEMT_BLOOD;
pub use crate::cg_local_h::LEMT_BURN;
pub use crate::cg_local_h::LEMT_NONE;
pub use crate::cg_local_h::LE_EXPLOSION;
pub use crate::cg_local_h::LE_FADE_RGB;
pub use crate::cg_local_h::LE_FALL_SCALE_FADE;
pub use crate::cg_local_h::LE_FRAGMENT;
pub use crate::cg_local_h::LE_MARK;
pub use crate::cg_local_h::LE_MOVE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCALE_FADE;
pub use crate::cg_local_h::LE_SCOREPLUM;
pub use crate::cg_local_h::LE_SPRITE_EXPLOSION;
pub use crate::src::cgame::cg_drawtools::CG_DrawBigStringColor;
pub use crate::src::cgame::cg_drawtools::CG_DrawPic;
pub use crate::src::cgame::cg_drawtools::CG_DrawStrlen;
pub use crate::src::cgame::cg_drawtools::CG_FadeColor;
pub use crate::src::cgame::cg_effects::CG_Bleed;
pub use crate::src::cgame::cg_effects::CG_BubbleTrail;
pub use crate::src::cgame::cg_effects::CG_MakeExplosion;
pub use crate::src::cgame::cg_effects::CG_SmokePuff;
pub use crate::src::cgame::cg_ents::CG_PositionRotatedEntityOnTag;
pub use crate::src::cgame::cg_localents::CG_AllocLocalEntity;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_brassTime;
pub use crate::src::cgame::cg_main::cg_drawGun;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_fov;
pub use crate::src::cgame::cg_main::cg_gun_frame;
pub use crate::src::cgame::cg_main::cg_gun_x;
pub use crate::src::cgame::cg_main::cg_gun_y;
pub use crate::src::cgame::cg_main::cg_gun_z;
pub use crate::src::cgame::cg_main::cg_items;
pub use crate::src::cgame::cg_main::cg_noProjectileTrail;
pub use crate::src::cgame::cg_main::cg_oldPlasma;
pub use crate::src::cgame::cg_main::cg_oldRail;
pub use crate::src::cgame::cg_main::cg_oldRocket;
pub use crate::src::cgame::cg_main::cg_railTrailTime;
pub use crate::src::cgame::cg_main::cg_tracerChance;
pub use crate::src::cgame::cg_main::cg_tracerLength;
pub use crate::src::cgame::cg_main::cg_tracerWidth;
pub use crate::src::cgame::cg_main::cg_trueLightning;
pub use crate::src::cgame::cg_main::cg_weapons;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Argv;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_marks::CG_ImpactMark;
pub use crate::src::cgame::cg_particles::CG_ParticleExplosion;
pub use crate::src::cgame::cg_predict::CG_PointContents;
pub use crate::src::cgame::cg_predict::CG_Trace;
pub use crate::src::cgame::cg_syscalls::trap_CM_BoxTrace;
pub use crate::src::cgame::cg_syscalls::trap_R_AddLightToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_LerpTag;
pub use crate::src::cgame::cg_syscalls::trap_R_ModelBounds;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterModel;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterShader;
pub use crate::src::cgame::cg_syscalls::trap_R_SetColor;
pub use crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound;
pub use crate::src::cgame::cg_syscalls::trap_S_RegisterSound;
pub use crate::src::cgame::cg_syscalls::trap_S_StartSound;
pub use crate::src::cgame::cg_weapons::stdlib_h::atoi;

pub use crate::stdlib::rand;

pub use crate::stdlib::strtol;
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
// cg_weapons.c -- events and effects dealing with weapons
/*
==========================
CG_MachineGunEjectBrass
==========================
*/

unsafe extern "C" fn CG_MachineGunEjectBrass(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut xvelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut xoffset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut waterScale: f32 = 1.0;
    let mut v: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    if crate::src::cgame::cg_main::cg_brassTime.integer <= 0 {
        return;
    }
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    velocity[0] = 0f32;
    velocity[1] = (-50f64
        + 40f64 * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
        as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (100f64
        + 50f64 * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
        as crate::src::qcommon::q_shared::vec_t;
    (*le).leType = crate::cg_local_h::LE_FRAGMENT;
    (*le).startTime = crate::src::cgame::cg_main::cg.time;
    (*le).endTime = (((*le).startTime + crate::src::cgame::cg_main::cg_brassTime.integer) as f32
        + (crate::src::cgame::cg_main::cg_brassTime.integer / 4) as f32
            * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32)) as i32;
    (*le).pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
    (*le).pos.trTime = crate::src::cgame::cg_main::cg.time - (crate::stdlib::rand() & 15);
    crate::src::qcommon::q_math::AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        v.as_mut_ptr(),
    );
    offset[0] = 8f32;
    offset[1] = -4f32;
    offset[2] = 24f32;
    xoffset[0] = offset[0] * v[0][0] + offset[1] * v[1][0] + offset[2] * v[2][0];
    xoffset[1] = offset[0] * v[0][1] + offset[1] * v[1][1] + offset[2] * v[2][1];
    xoffset[2] = offset[0] * v[0][2] + offset[1] * v[1][2] + offset[2] * v[2][2];
    (*re).origin[0] = (*cent).lerpOrigin[0] + xoffset[0];
    (*re).origin[1] = (*cent).lerpOrigin[1] + xoffset[1];
    (*re).origin[2] = (*cent).lerpOrigin[2] + xoffset[2];
    (*le).pos.trBase[0] = (*re).origin[0];
    (*le).pos.trBase[1] = (*re).origin[1];
    (*le).pos.trBase[2] = (*re).origin[2];
    if crate::src::cgame::cg_predict::CG_PointContents(
        (*re).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        -(1),
    ) & 32
        != 0
    {
        waterScale = 0.10
    }
    xvelocity[0] = velocity[0] * v[0][0] + velocity[1] * v[1][0] + velocity[2] * v[2][0];
    xvelocity[1] = velocity[0] * v[0][1] + velocity[1] * v[1][1] + velocity[2] * v[2][1];
    xvelocity[2] = velocity[0] * v[0][2] + velocity[1] * v[1][2] + velocity[2] * v[2][2];
    (*le).pos.trDelta[0] = xvelocity[0] * waterScale;
    (*le).pos.trDelta[1] = xvelocity[1] * waterScale;
    (*le).pos.trDelta[2] = xvelocity[2] * waterScale;
    crate::src::qcommon::q_math::AxisCopy(
        crate::src::qcommon::q_math::axisDefault.as_mut_ptr(),
        (*re).axis.as_mut_ptr(),
    );
    (*re).hModel = crate::src::cgame::cg_main::cgs.media.machinegunBrassModel;
    (*le).bounceFactor = (0.4 * waterScale as f64) as f32;
    (*le).angles.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    (*le).angles.trTime = crate::src::cgame::cg_main::cg.time;
    (*le).angles.trBase[0] = (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
    (*le).angles.trBase[1] = (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
    (*le).angles.trBase[2] = (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
    (*le).angles.trDelta[0] = 2f32;
    (*le).angles.trDelta[1] = 1f32;
    (*le).angles.trDelta[2] = 0f32;
    (*le).leFlags = crate::cg_local_h::LEF_TUMBLE as i32;
    (*le).leBounceSoundType = crate::cg_local_h::LEBS_BRASS;
    (*le).leMarkType = crate::cg_local_h::LEMT_NONE;
}
/*
==========================
CG_ShotgunEjectBrass
==========================
*/

unsafe extern "C" fn CG_ShotgunEjectBrass(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut xvelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut xoffset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut i: i32 = 0;
    if crate::src::cgame::cg_main::cg_brassTime.integer <= 0 {
        return;
    }
    i = 0;
    while i < 2 {
        let mut waterScale: f32 = 1.0;
        le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
        re = &mut (*le).refEntity;
        velocity[0] = (60f64
            + 60f64 * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
            as crate::src::qcommon::q_shared::vec_t;
        if i == 0 {
            velocity[1] = (40f64
                + 10f64
                    * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
                as crate::src::qcommon::q_shared::vec_t
        } else {
            velocity[1] = (-40f64
                + 10f64
                    * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
                as crate::src::qcommon::q_shared::vec_t
        }
        velocity[2] = (100f64
            + 50f64 * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
            as crate::src::qcommon::q_shared::vec_t;
        (*le).leType = crate::cg_local_h::LE_FRAGMENT;
        (*le).startTime = crate::src::cgame::cg_main::cg.time;
        (*le).endTime =
            (((*le).startTime + crate::src::cgame::cg_main::cg_brassTime.integer * 3) as f32
                + crate::src::cgame::cg_main::cg_brassTime.integer as f32
                    * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32)) as i32;
        (*le).pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
        (*le).pos.trTime = crate::src::cgame::cg_main::cg.time;
        crate::src::qcommon::q_math::AnglesToAxis(
            (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            v.as_mut_ptr(),
        );
        offset[0] = 8f32;
        offset[1] = 0f32;
        offset[2] = 24f32;
        xoffset[0] = offset[0] * v[0][0] + offset[1] * v[1][0] + offset[2] * v[2][0];
        xoffset[1] = offset[0] * v[0][1] + offset[1] * v[1][1] + offset[2] * v[2][1];
        xoffset[2] = offset[0] * v[0][2] + offset[1] * v[1][2] + offset[2] * v[2][2];
        (*re).origin[0] = (*cent).lerpOrigin[0] + xoffset[0];
        (*re).origin[1] = (*cent).lerpOrigin[1] + xoffset[1];
        (*re).origin[2] = (*cent).lerpOrigin[2] + xoffset[2];
        (*le).pos.trBase[0] = (*re).origin[0];
        (*le).pos.trBase[1] = (*re).origin[1];
        (*le).pos.trBase[2] = (*re).origin[2];
        if crate::src::cgame::cg_predict::CG_PointContents(
            (*re).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            -(1),
        ) & 32
            != 0
        {
            waterScale = 0.10
        }
        xvelocity[0] = velocity[0] * v[0][0] + velocity[1] * v[1][0] + velocity[2] * v[2][0];
        xvelocity[1] = velocity[0] * v[0][1] + velocity[1] * v[1][1] + velocity[2] * v[2][1];
        xvelocity[2] = velocity[0] * v[0][2] + velocity[1] * v[1][2] + velocity[2] * v[2][2];
        (*le).pos.trDelta[0] = xvelocity[0] * waterScale;
        (*le).pos.trDelta[1] = xvelocity[1] * waterScale;
        (*le).pos.trDelta[2] = xvelocity[2] * waterScale;
        crate::src::qcommon::q_math::AxisCopy(
            crate::src::qcommon::q_math::axisDefault.as_mut_ptr(),
            (*re).axis.as_mut_ptr(),
        );
        (*re).hModel = crate::src::cgame::cg_main::cgs.media.shotgunBrassModel;
        (*le).bounceFactor = 0.3;
        (*le).angles.trType = crate::src::qcommon::q_shared::TR_LINEAR;
        (*le).angles.trTime = crate::src::cgame::cg_main::cg.time;
        (*le).angles.trBase[0] =
            (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
        (*le).angles.trBase[1] =
            (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
        (*le).angles.trBase[2] =
            (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
        (*le).angles.trDelta[0] = 1f32;
        (*le).angles.trDelta[1] = 0.5;
        (*le).angles.trDelta[2] = 0f32;
        (*le).leFlags = crate::cg_local_h::LEF_TUMBLE as i32;
        (*le).leBounceSoundType = crate::cg_local_h::LEBS_BRASS;
        (*le).leMarkType = crate::cg_local_h::LEMT_NONE;
        i += 1
    }
}
/*
==========================
CG_RailTrail
==========================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_RailTrail(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut axis: [crate::src::qcommon::q_shared::vec3_t; 36] = [[0.; 3]; 36];
    let mut move_0: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut temp: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut len: f32 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut skip: i32 = 0;
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let ref mut fresh0 = *start.offset(2);
    *fresh0 -= 4f32;
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    (*le).leType = crate::cg_local_h::LE_FADE_RGB;
    (*le).startTime = crate::src::cgame::cg_main::cg.time;
    (*le).endTime = (crate::src::cgame::cg_main::cg.time as f32
        + crate::src::cgame::cg_main::cg_railTrailTime.value) as i32;
    (*le).lifeRate = (1.0 / ((*le).endTime - (*le).startTime) as f64) as f32;
    (*re).shaderTime = crate::src::cgame::cg_main::cg.time as f32 / 1000.0;
    (*re).reType = crate::tr_types_h::RT_RAIL_CORE;
    (*re).customShader = crate::src::cgame::cg_main::cgs.media.railCoreShader;
    (*re).origin[0] = *start.offset(0);
    (*re).origin[1] = *start.offset(1);
    (*re).origin[2] = *start.offset(2);
    (*re).oldorigin[0] = *end.offset(0);
    (*re).oldorigin[1] = *end.offset(1);
    (*re).oldorigin[2] = *end.offset(2);
    (*re).shaderRGBA[0] = ((*ci).color1[0] * 255f32) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[1] = ((*ci).color1[1] * 255f32) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[2] = ((*ci).color1[2] * 255f32) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[3] = 255;
    (*le).color[0] = ((*ci).color1[0] as f64 * 0.75) as f32;
    (*le).color[1] = ((*ci).color1[1] as f64 * 0.75) as f32;
    (*le).color[2] = ((*ci).color1[2] as f64 * 0.75) as f32;
    (*le).color[3] = 1.0;
    crate::src::qcommon::q_math::AxisClear((*re).axis.as_mut_ptr());
    if crate::src::cgame::cg_main::cg_oldRail.integer != 0 {
        // nudge down a bit so it isn't exactly in center
        (*re).origin[2] -= 8f32;
        (*re).oldorigin[2] -= 8f32;
        return;
    }
    move_0[0] = *start.offset(0);
    move_0[1] = *start.offset(1);
    move_0[2] = *start.offset(2);
    vec[0] = *end.offset(0) - *start.offset(0);
    vec[1] = *end.offset(1) - *start.offset(1);
    vec[2] = *end.offset(2) - *start.offset(2);
    len = crate::src::qcommon::q_math::VectorNormalize(vec.as_mut_ptr());
    crate::src::qcommon::q_math::PerpendicularVector(
        temp.as_mut_ptr(),
        vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    i = 0;
    while i < 36 {
        crate::src::qcommon::q_math::RotatePointAroundVector(
            axis[i as usize].as_mut_ptr(),
            vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            temp.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (i * 10) as f32,
        );
        i += 1
        //banshee 2.4 was 10
    }
    move_0[0] = move_0[0] + vec[0] * 20f32;
    move_0[1] = move_0[1] + vec[1] * 20f32;
    move_0[2] = move_0[2] + vec[2] * 20f32;
    vec[0] = vec[0] * 5f32;
    vec[1] = vec[1] * 5f32;
    vec[2] = vec[2] * 5f32;
    skip = -(1);
    j = 18;
    i = 0;
    while (i as f32) < len {
        if i != skip {
            skip = i + 5;
            le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
            re = &mut (*le).refEntity;
            (*le).leFlags = crate::cg_local_h::LEF_PUFF_DONT_SCALE as i32;
            (*le).leType = crate::cg_local_h::LE_MOVE_SCALE_FADE;
            (*le).startTime = crate::src::cgame::cg_main::cg.time;
            (*le).endTime = crate::src::cgame::cg_main::cg.time + (i >> 1) + 600;
            (*le).lifeRate = (1.0 / ((*le).endTime - (*le).startTime) as f64) as f32;
            (*re).shaderTime = crate::src::cgame::cg_main::cg.time as f32 / 1000.0;
            (*re).reType = crate::tr_types_h::RT_SPRITE;
            (*re).radius = 1.1;
            (*re).customShader = crate::src::cgame::cg_main::cgs.media.railRingsShader;
            (*re).shaderRGBA[0] = ((*ci).color2[0] * 255f32) as crate::src::qcommon::q_shared::byte;
            (*re).shaderRGBA[1] = ((*ci).color2[1] * 255f32) as crate::src::qcommon::q_shared::byte;
            (*re).shaderRGBA[2] = ((*ci).color2[2] * 255f32) as crate::src::qcommon::q_shared::byte;
            (*re).shaderRGBA[3] = 255;
            (*le).color[0] = ((*ci).color2[0] as f64 * 0.75) as f32;
            (*le).color[1] = ((*ci).color2[1] as f64 * 0.75) as f32;
            (*le).color[2] = ((*ci).color2[2] as f64 * 0.75) as f32;
            (*le).color[3] = 1.0;
            (*le).pos.trType = crate::src::qcommon::q_shared::TR_LINEAR;
            (*le).pos.trTime = crate::src::cgame::cg_main::cg.time;
            move2[0] = move_0[0];
            move2[1] = move_0[1];
            move2[2] = move_0[2];
            move2[0] = move2[0] + axis[j as usize][0] * 4f32;
            move2[1] = move2[1] + axis[j as usize][1] * 4f32;
            move2[2] = move2[2] + axis[j as usize][2] * 4f32;
            (*le).pos.trBase[0] = move2[0];
            (*le).pos.trBase[1] = move2[1];
            (*le).pos.trBase[2] = move2[2];
            (*le).pos.trDelta[0] = axis[j as usize][0] * 6f32;
            (*le).pos.trDelta[1] = axis[j as usize][1] * 6f32;
            (*le).pos.trDelta[2] = axis[j as usize][2] * 6f32
        }
        move_0[0] = move_0[0] + vec[0];
        move_0[1] = move_0[1] + vec[1];
        move_0[2] = move_0[2] + vec[2];
        j = (j + 1) % 36;
        i += 5
    }
}
/*
==========================
CG_RocketTrail
==========================
*/

unsafe extern "C" fn CG_RocketTrail(
    mut ent: *mut crate::cg_local_h::centity_t,
    mut wi: *const crate::cg_local_h::weaponInfo_t,
) {
    let mut step: i32 = 0;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lastPos: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut t: i32 = 0;
    let mut startTime: i32 = 0;
    let mut contents: i32 = 0;
    let mut lastContents: i32 = 0;
    let mut es: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut smoke: *mut crate::cg_local_h::localEntity_t =
        0 as *mut crate::cg_local_h::localEntity_t;
    if crate::src::cgame::cg_main::cg_noProjectileTrail.integer != 0 {
        return;
    }
    up[0] = 0f32;
    up[1] = 0f32;
    up[2] = 0f32;
    step = 50;
    es = &mut (*ent).currentState;
    startTime = (*ent).trailTime;
    t = step * ((startTime + step) / step);
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*es).pos,
        crate::src::cgame::cg_main::cg.time,
        origin.as_mut_ptr(),
    );
    contents = crate::src::cgame::cg_predict::CG_PointContents(
        origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        -(1),
    );
    // if object (e.g. grenade) is stationary, don't toss up smoke
    if (*es).pos.trType == crate::src::qcommon::q_shared::TR_STATIONARY {
        (*ent).trailTime = crate::src::cgame::cg_main::cg.time;
        return;
    }
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*es).pos,
        (*ent).trailTime,
        lastPos.as_mut_ptr(),
    );
    lastContents = crate::src::cgame::cg_predict::CG_PointContents(
        lastPos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        -(1),
    );
    (*ent).trailTime = crate::src::cgame::cg_main::cg.time;
    if contents & (32 | 16 | 8) != 0 {
        if contents & lastContents & 32 != 0 {
            crate::src::cgame::cg_effects::CG_BubbleTrail(
                lastPos.as_mut_ptr(),
                origin.as_mut_ptr(),
                8f32,
            );
        }
        return;
    }
    while t <= (*ent).trailTime {
        crate::src::game::bg_misc::BG_EvaluateTrajectory(&mut (*es).pos, t, lastPos.as_mut_ptr());
        smoke = crate::src::cgame::cg_effects::CG_SmokePuff(
            lastPos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*wi).trailRadius,
            1f32,
            1f32,
            1f32,
            0.33,
            (*wi).wiTrailTime,
            t,
            0,
            0,
            crate::src::cgame::cg_main::cgs.media.smokePuffShader,
        );
        // use the optimized local entity add
        (*smoke).leType = crate::cg_local_h::LE_SCALE_FADE;
        t += step
    }
}
/*
==========================
CG_PlasmaTrail
==========================
*/

unsafe extern "C" fn CG_PlasmaTrail(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut wi: *const crate::cg_local_h::weaponInfo_t,
) {
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut re: *mut crate::tr_types_h::refEntity_t = 0 as *mut crate::tr_types_h::refEntity_t;
    let mut es: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut xvelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut offset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut xoffset: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut waterScale: f32 = 1.0;
    if crate::src::cgame::cg_main::cg_noProjectileTrail.integer != 0
        || crate::src::cgame::cg_main::cg_oldPlasma.integer != 0
    {
        return;
    }
    es = &mut (*cent).currentState;
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*es).pos,
        crate::src::cgame::cg_main::cg.time,
        origin.as_mut_ptr(),
    );
    le = crate::src::cgame::cg_localents::CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    velocity[0] = (60f64
        - 120f64 * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
        as crate::src::qcommon::q_shared::vec_t;
    velocity[1] = (40f64
        - 80f64 * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
        as crate::src::qcommon::q_shared::vec_t;
    velocity[2] = (100f64
        - 200f64 * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
        as crate::src::qcommon::q_shared::vec_t;
    (*le).leType = crate::cg_local_h::LE_MOVE_SCALE_FADE;
    (*le).leFlags = crate::cg_local_h::LEF_TUMBLE as i32;
    (*le).leBounceSoundType = crate::cg_local_h::LEBS_NONE;
    (*le).leMarkType = crate::cg_local_h::LEMT_NONE;
    (*le).startTime = crate::src::cgame::cg_main::cg.time;
    (*le).endTime = (*le).startTime + 600;
    (*le).pos.trType = crate::src::qcommon::q_shared::TR_GRAVITY;
    (*le).pos.trTime = crate::src::cgame::cg_main::cg.time;
    crate::src::qcommon::q_math::AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        v.as_mut_ptr(),
    );
    offset[0] = 2f32;
    offset[1] = 2f32;
    offset[2] = 2f32;
    xoffset[0] = offset[0] * v[0][0] + offset[1] * v[1][0] + offset[2] * v[2][0];
    xoffset[1] = offset[0] * v[0][1] + offset[1] * v[1][1] + offset[2] * v[2][1];
    xoffset[2] = offset[0] * v[0][2] + offset[1] * v[1][2] + offset[2] * v[2][2];
    (*re).origin[0] = origin[0] + xoffset[0];
    (*re).origin[1] = origin[1] + xoffset[1];
    (*re).origin[2] = origin[2] + xoffset[2];
    (*le).pos.trBase[0] = (*re).origin[0];
    (*le).pos.trBase[1] = (*re).origin[1];
    (*le).pos.trBase[2] = (*re).origin[2];
    if crate::src::cgame::cg_predict::CG_PointContents(
        (*re).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        -(1),
    ) & 32
        != 0
    {
        waterScale = 0.10
    }
    xvelocity[0] = velocity[0] * v[0][0] + velocity[1] * v[1][0] + velocity[2] * v[2][0];
    xvelocity[1] = velocity[0] * v[0][1] + velocity[1] * v[1][1] + velocity[2] * v[2][1];
    xvelocity[2] = velocity[0] * v[0][2] + velocity[1] * v[1][2] + velocity[2] * v[2][2];
    (*le).pos.trDelta[0] = xvelocity[0] * waterScale;
    (*le).pos.trDelta[1] = xvelocity[1] * waterScale;
    (*le).pos.trDelta[2] = xvelocity[2] * waterScale;
    crate::src::qcommon::q_math::AxisCopy(
        crate::src::qcommon::q_math::axisDefault.as_mut_ptr(),
        (*re).axis.as_mut_ptr(),
    );
    (*re).shaderTime = crate::src::cgame::cg_main::cg.time as f32 / 1000.0;
    (*re).reType = crate::tr_types_h::RT_SPRITE;
    (*re).radius = 0.25;
    (*re).customShader = crate::src::cgame::cg_main::cgs.media.railRingsShader;
    (*le).bounceFactor = 0.3;
    (*re).shaderRGBA[0] =
        ((*wi).flashDlightColor[0] * 63f32) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[1] =
        ((*wi).flashDlightColor[1] * 63f32) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[2] =
        ((*wi).flashDlightColor[2] * 63f32) as crate::src::qcommon::q_shared::byte;
    (*re).shaderRGBA[3] = 63;
    (*le).color[0] = ((*wi).flashDlightColor[0] as f64 * 0.2) as f32;
    (*le).color[1] = ((*wi).flashDlightColor[1] as f64 * 0.2) as f32;
    (*le).color[2] = ((*wi).flashDlightColor[2] as f64 * 0.2) as f32;
    (*le).color[3] = 0.25;
    (*le).angles.trType = crate::src::qcommon::q_shared::TR_LINEAR;
    (*le).angles.trTime = crate::src::cgame::cg_main::cg.time;
    (*le).angles.trBase[0] = (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
    (*le).angles.trBase[1] = (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
    (*le).angles.trBase[2] = (crate::stdlib::rand() & 31) as crate::src::qcommon::q_shared::vec_t;
    (*le).angles.trDelta[0] = 1f32;
    (*le).angles.trDelta[1] = 0.5;
    (*le).angles.trDelta[2] = 0f32;
}
/*
==========================
CG_GrappleTrail
==========================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_GrappleTrail(
    mut ent: *mut crate::cg_local_h::centity_t,
    mut _wi: *const crate::cg_local_h::weaponInfo_t,
) {
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut es: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut beam: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    es = &mut (*ent).currentState;
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*es).pos,
        crate::src::cgame::cg_main::cg.time,
        origin.as_mut_ptr(),
    );
    (*ent).trailTime = crate::src::cgame::cg_main::cg.time;
    crate::stdlib::memset(
        &mut beam as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    //FIXME adjust for muzzle position
    beam.origin[0] = crate::src::cgame::cg_main::cg_entities
        [(*ent).currentState.otherEntityNum as usize]
        .lerpOrigin[0]; // Don't draw if close
    beam.origin[1] = crate::src::cgame::cg_main::cg_entities
        [(*ent).currentState.otherEntityNum as usize]
        .lerpOrigin[1];
    beam.origin[2] = crate::src::cgame::cg_main::cg_entities
        [(*ent).currentState.otherEntityNum as usize]
        .lerpOrigin[2];
    beam.origin[2] += 26f32;
    crate::src::qcommon::q_math::AngleVectors(
        crate::src::cgame::cg_main::cg_entities[(*ent).currentState.otherEntityNum as usize]
            .lerpAngles
            .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        up.as_mut_ptr(),
    );
    beam.origin[0] = beam.origin[0] + up[0] * -6f32;
    beam.origin[1] = beam.origin[1] + up[1] * -6f32;
    beam.origin[2] = beam.origin[2] + up[2] * -6f32;
    beam.oldorigin[0] = origin[0];
    beam.oldorigin[1] = origin[1];
    beam.oldorigin[2] = origin[2];
    if Distance(
        beam.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        beam.oldorigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    ) < 64f32
    {
        return;
    }
    beam.reType = crate::tr_types_h::RT_LIGHTNING;
    beam.customShader = crate::src::cgame::cg_main::cgs.media.lightningShader;
    crate::src::qcommon::q_math::AxisClear(beam.axis.as_mut_ptr());
    beam.shaderRGBA[0] = 0xff;
    beam.shaderRGBA[1] = 0xff;
    beam.shaderRGBA[2] = 0xff;
    beam.shaderRGBA[3] = 0xff;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut beam);
}
/*
==========================
CG_GrenadeTrail
==========================
*/

unsafe extern "C" fn CG_GrenadeTrail(
    mut ent: *mut crate::cg_local_h::centity_t,
    mut wi: *const crate::cg_local_h::weaponInfo_t,
) {
    CG_RocketTrail(ent, wi);
}
/*
=================
CG_RegisterWeapon

The server says this item is used on this level
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_RegisterWeapon(mut weaponNum: i32) {
    let mut weaponInfo: *mut crate::cg_local_h::weaponInfo_t =
        0 as *mut crate::cg_local_h::weaponInfo_t;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut ammo: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut path: [i8; 64] = [0; 64];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut _i: i32 = 0;
    weaponInfo = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset(weaponNum as isize) as *mut crate::cg_local_h::weaponInfo_t;
    if weaponNum == 0 {
        return;
    }
    if (*weaponInfo).registered as u64 != 0 {
        return;
    }
    crate::stdlib::memset(
        weaponInfo as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::cg_local_h::weaponInfo_t>(),
    );
    (*weaponInfo).registered = crate::src::qcommon::q_shared::qtrue;
    item = crate::src::game::bg_misc::bg_itemlist
        .as_mut_ptr()
        .offset(1);
    while !(*item).classname.is_null() {
        if (*item).giType == crate::bg_public_h::IT_WEAPON && (*item).giTag == weaponNum {
            (*weaponInfo).item = item;
            break;
        } else {
            item = item.offset(1)
        }
    }
    if (*item).classname.is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"Couldn\'t find weapon %i\x00" as *const u8 as *const i8,
            weaponNum,
        );
    }
    CG_RegisterItemVisuals(
        item.wrapping_offset_from(crate::src::game::bg_misc::bg_itemlist.as_mut_ptr()) as i32,
    );
    // load cmodel before model so filecache works
    (*weaponInfo).weaponModel =
        crate::src::cgame::cg_syscalls::trap_R_RegisterModel((*item).world_model[0]);
    // calc midpoint for rotation
    crate::src::cgame::cg_syscalls::trap_R_ModelBounds(
        (*weaponInfo).weaponModel,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );

    for i in 0..3 {
        (*weaponInfo).weaponMidpoint[i as usize] = (mins[i as usize] as f64
            + 0.5 * (maxs[i as usize] - mins[i as usize]) as f64)
            as crate::src::qcommon::q_shared::vec_t;
    }
    (*weaponInfo).weaponIcon = crate::src::cgame::cg_syscalls::trap_R_RegisterShader((*item).icon);
    (*weaponInfo).ammoIcon = crate::src::cgame::cg_syscalls::trap_R_RegisterShader((*item).icon);
    ammo = crate::src::game::bg_misc::bg_itemlist
        .as_mut_ptr()
        .offset(1);
    while !(*ammo).classname.is_null() {
        if (*ammo).giType == crate::bg_public_h::IT_AMMO && (*ammo).giTag == weaponNum {
            break;
        }
        ammo = ammo.offset(1)
    }
    if !(*ammo).classname.is_null() && !(*ammo).world_model[0].is_null() {
        (*weaponInfo).ammoModel =
            crate::src::cgame::cg_syscalls::trap_R_RegisterModel((*ammo).world_model[0])
    }
    crate::src::qcommon::q_shared::COM_StripExtension(
        (*item).world_model[0],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"_flash.md3\x00" as *const u8 as *const i8,
    );
    (*weaponInfo).flashModel =
        crate::src::cgame::cg_syscalls::trap_R_RegisterModel(path.as_mut_ptr());
    crate::src::qcommon::q_shared::COM_StripExtension(
        (*item).world_model[0],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"_barrel.md3\x00" as *const u8 as *const i8,
    );
    (*weaponInfo).barrelModel =
        crate::src::cgame::cg_syscalls::trap_R_RegisterModel(path.as_mut_ptr());
    crate::src::qcommon::q_shared::COM_StripExtension(
        (*item).world_model[0],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"_hand.md3\x00" as *const u8 as *const i8,
    );
    (*weaponInfo).handsModel =
        crate::src::cgame::cg_syscalls::trap_R_RegisterModel(path.as_mut_ptr());
    if (*weaponInfo).handsModel == 0 {
        (*weaponInfo).handsModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
            b"models/weapons2/shotgun/shotgun_hand.md3\x00" as *const u8 as *const i8,
        )
    }
    match weaponNum {
        1 => {
            (*weaponInfo).flashDlightColor[0] = 0.6;
            (*weaponInfo).flashDlightColor[1] = 0.6;
            (*weaponInfo).flashDlightColor[2] = 1.0;
            (*weaponInfo).firingSound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/melee/fstrun.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/melee/fstatck.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            )
        }
        6 => {
            (*weaponInfo).flashDlightColor[0] = 0.6;
            (*weaponInfo).flashDlightColor[1] = 0.6;
            (*weaponInfo).flashDlightColor[2] = 1.0;
            (*weaponInfo).readySound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/melee/fsthum.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).firingSound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/lightning/lg_hum.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/lightning/lg_fire.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::cgame::cg_main::cgs.media.lightningShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"lightningBoltNew\x00" as *const u8 as *const i8,
                );
            crate::src::cgame::cg_main::cgs
                .media
                .lightningExplosionModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
                b"models/weaphits/crackle.md3\x00" as *const u8 as *const i8,
            );
            crate::src::cgame::cg_main::cgs.media.sfx_lghit1 =
                crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                    b"sound/weapons/lightning/lg_hit.wav\x00" as *const u8 as *const i8,
                    crate::src::qcommon::q_shared::qfalse,
                );
            crate::src::cgame::cg_main::cgs.media.sfx_lghit2 =
                crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                    b"sound/weapons/lightning/lg_hit2.wav\x00" as *const u8 as *const i8,
                    crate::src::qcommon::q_shared::qfalse,
                );
            crate::src::cgame::cg_main::cgs.media.sfx_lghit3 =
                crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                    b"sound/weapons/lightning/lg_hit3.wav\x00" as *const u8 as *const i8,
                    crate::src::qcommon::q_shared::qfalse,
                )
        }
        10 => {
            (*weaponInfo).flashDlightColor[0] = 0.6;
            (*weaponInfo).flashDlightColor[1] = 0.6;
            (*weaponInfo).flashDlightColor[2] = 1.0;
            (*weaponInfo).missileModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
                b"models/ammo/rocket/rocket.md3\x00" as *const u8 as *const i8,
            );
            (*weaponInfo).missileTrailFunc = Some(
                CG_GrappleTrail
                    as unsafe extern "C" fn(
                        _: *mut crate::cg_local_h::centity_t,
                        _: *const crate::cg_local_h::weaponInfo_t,
                    ) -> (),
            );
            (*weaponInfo).missileDlight = 200f32;
            (*weaponInfo).missileDlightColor[0] = 1f32;
            (*weaponInfo).missileDlightColor[1] = 0.75;
            (*weaponInfo).missileDlightColor[2] = 0f32;
            (*weaponInfo).readySound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/melee/fsthum.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).firingSound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/melee/fstrun.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::cgame::cg_main::cgs.media.lightningShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"lightningBoltNew\x00" as *const u8 as *const i8,
                )
        }
        2 => {
            (*weaponInfo).flashDlightColor[0] = 1f32;
            (*weaponInfo).flashDlightColor[1] = 1f32;
            (*weaponInfo).flashDlightColor[2] = 0f32;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf1b.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashSound[1] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf2b.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashSound[2] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf3b.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashSound[3] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf4b.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).ejectBrassFunc = Some(
                CG_MachineGunEjectBrass
                    as unsafe extern "C" fn(_: *mut crate::cg_local_h::centity_t) -> (),
            );
            crate::src::cgame::cg_main::cgs.media.bulletExplosionShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"bulletExplosion\x00" as *const u8 as *const i8,
                )
        }
        3 => {
            (*weaponInfo).flashDlightColor[0] = 1f32;
            (*weaponInfo).flashDlightColor[1] = 1f32;
            (*weaponInfo).flashDlightColor[2] = 0f32;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/shotgun/sshotf1b.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).ejectBrassFunc = Some(
                CG_ShotgunEjectBrass
                    as unsafe extern "C" fn(_: *mut crate::cg_local_h::centity_t) -> (),
            )
        }
        5 => {
            (*weaponInfo).missileModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
                b"models/ammo/rocket/rocket.md3\x00" as *const u8 as *const i8,
            );
            (*weaponInfo).missileSound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/rocket/rockfly.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).missileTrailFunc = Some(
                CG_RocketTrail
                    as unsafe extern "C" fn(
                        _: *mut crate::cg_local_h::centity_t,
                        _: *const crate::cg_local_h::weaponInfo_t,
                    ) -> (),
            );
            (*weaponInfo).missileDlight = 200f32;
            (*weaponInfo).wiTrailTime = 2000f32;
            (*weaponInfo).trailRadius = 64f32;
            (*weaponInfo).missileDlightColor[0] = 1f32;
            (*weaponInfo).missileDlightColor[1] = 0.75;
            (*weaponInfo).missileDlightColor[2] = 0f32;
            (*weaponInfo).flashDlightColor[0] = 1f32;
            (*weaponInfo).flashDlightColor[1] = 0.75;
            (*weaponInfo).flashDlightColor[2] = 0f32;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/rocket/rocklf1a.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::cgame::cg_main::cgs.media.rocketExplosionShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"rocketExplosion\x00" as *const u8 as *const i8,
                )
        }
        4 => {
            (*weaponInfo).missileModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
                b"models/ammo/grenade1.md3\x00" as *const u8 as *const i8,
            );
            (*weaponInfo).missileTrailFunc = Some(
                CG_GrenadeTrail
                    as unsafe extern "C" fn(
                        _: *mut crate::cg_local_h::centity_t,
                        _: *const crate::cg_local_h::weaponInfo_t,
                    ) -> (),
            );
            (*weaponInfo).wiTrailTime = 700f32;
            (*weaponInfo).trailRadius = 32f32;
            (*weaponInfo).flashDlightColor[0] = 1f32;
            (*weaponInfo).flashDlightColor[1] = 0.70;
            (*weaponInfo).flashDlightColor[2] = 0f32;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/grenade/grenlf1a.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::cgame::cg_main::cgs.media.grenadeExplosionShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"grenadeExplosion\x00" as *const u8 as *const i8,
                )
        }
        8 => {
            //		weaponInfo->missileModel = cgs.media.invulnerabilityPowerupModel;
            (*weaponInfo).missileTrailFunc = Some(
                CG_PlasmaTrail
                    as unsafe extern "C" fn(
                        _: *mut crate::cg_local_h::centity_t,
                        _: *const crate::cg_local_h::weaponInfo_t,
                    ) -> (),
            );
            (*weaponInfo).missileSound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/plasma/lasfly.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashDlightColor[0] = 0.6;
            (*weaponInfo).flashDlightColor[1] = 0.6;
            (*weaponInfo).flashDlightColor[2] = 1.0;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/plasma/hyprbf1a.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::cgame::cg_main::cgs.media.plasmaExplosionShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"plasmaExplosion\x00" as *const u8 as *const i8,
                );
            crate::src::cgame::cg_main::cgs.media.railRingsShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"railDisc\x00" as *const u8 as *const i8,
                )
        }
        7 => {
            (*weaponInfo).readySound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/railgun/rg_hum.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashDlightColor[0] = 1f32;
            (*weaponInfo).flashDlightColor[1] = 0.5;
            (*weaponInfo).flashDlightColor[2] = 0f32;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/railgun/railgf1a.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::cgame::cg_main::cgs.media.railExplosionShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"railExplosion\x00" as *const u8 as *const i8,
                );
            crate::src::cgame::cg_main::cgs.media.railRingsShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"railDisc\x00" as *const u8 as *const i8,
                );
            crate::src::cgame::cg_main::cgs.media.railCoreShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"railCore\x00" as *const u8 as *const i8,
                )
        }
        9 => {
            (*weaponInfo).readySound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/bfg/bfg_hum.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            (*weaponInfo).flashDlightColor[0] = 1f32;
            (*weaponInfo).flashDlightColor[1] = 0.7;
            (*weaponInfo).flashDlightColor[2] = 1f32;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/bfg/bfg_fire.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            );
            crate::src::cgame::cg_main::cgs.media.bfgExplosionShader =
                crate::src::cgame::cg_syscalls::trap_R_RegisterShader(
                    b"bfgExplosion\x00" as *const u8 as *const i8,
                );
            (*weaponInfo).missileModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(
                b"models/weaphits/bfg.md3\x00" as *const u8 as *const i8,
            );
            (*weaponInfo).missileSound = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/rocket/rockfly.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            )
        }
        _ => {
            (*weaponInfo).flashDlightColor[0] = 1f32;
            (*weaponInfo).flashDlightColor[1] = 1f32;
            (*weaponInfo).flashDlightColor[2] = 1f32;
            (*weaponInfo).flashSound[0] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                b"sound/weapons/rocket/rocklf1a.wav\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::qfalse,
            )
        }
    };
}
/*
=================
CG_RegisterItemVisuals

The server says this item is used on this level
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_RegisterItemVisuals(mut itemNum: i32) {
    let mut itemInfo: *mut crate::cg_local_h::itemInfo_t = 0 as *mut crate::cg_local_h::itemInfo_t;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    if itemNum < 0 || itemNum >= crate::src::game::bg_misc::bg_numItems {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_RegisterItemVisuals: itemNum %d out of range [0-%d]\x00" as *const u8 as *const i8,
            itemNum,
            crate::src::game::bg_misc::bg_numItems - 1i32,
        );
    }
    itemInfo = &mut *crate::src::cgame::cg_main::cg_items
        .as_mut_ptr()
        .offset(itemNum as isize) as *mut crate::cg_local_h::itemInfo_t;
    if (*itemInfo).registered as u64 != 0 {
        return;
    }
    item = &mut *crate::src::game::bg_misc::bg_itemlist
        .as_mut_ptr()
        .offset(itemNum as isize) as *mut crate::bg_public_h::gitem_t;
    crate::stdlib::memset(
        itemInfo as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::cg_local_h::itemInfo_t>(),
    );
    (*itemInfo).registered = crate::src::qcommon::q_shared::qtrue;
    (*itemInfo).models[0] =
        crate::src::cgame::cg_syscalls::trap_R_RegisterModel((*item).world_model[0]);
    (*itemInfo).icon = crate::src::cgame::cg_syscalls::trap_R_RegisterShader((*item).icon);
    if (*item).giType == crate::bg_public_h::IT_WEAPON {
        CG_RegisterWeapon((*item).giTag);
    }
    //
    // powerups have an accompanying ring or sphere
    //
    if (*item).giType == crate::bg_public_h::IT_POWERUP
        || (*item).giType == crate::bg_public_h::IT_HEALTH
        || (*item).giType == crate::bg_public_h::IT_ARMOR
        || (*item).giType == crate::bg_public_h::IT_HOLDABLE
    {
        if !(*item).world_model[1].is_null() {
            (*itemInfo).models[1] =
                crate::src::cgame::cg_syscalls::trap_R_RegisterModel((*item).world_model[1])
        }
    };
}
/*
========================================================================================

VIEW WEAPON

========================================================================================
*/
/*
=================
CG_MapTorsoToWeaponFrame

=================
*/

unsafe extern "C" fn CG_MapTorsoToWeaponFrame(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut frame: i32,
) -> i32 {
    // change weapon
    if frame >= (*ci).animations[crate::bg_public_h::TORSO_DROP as usize].firstFrame
        && frame < (*ci).animations[crate::bg_public_h::TORSO_DROP as usize].firstFrame + 9
    {
        return frame - (*ci).animations[crate::bg_public_h::TORSO_DROP as usize].firstFrame + 6i32;
    }
    // stand attack
    if frame >= (*ci).animations[crate::bg_public_h::TORSO_ATTACK as usize].firstFrame
        && frame < (*ci).animations[crate::bg_public_h::TORSO_ATTACK as usize].firstFrame + 6
    {
        return 1i32 + frame
            - (*ci).animations[crate::bg_public_h::TORSO_ATTACK as usize].firstFrame;
    }
    // stand attack 2
    if frame >= (*ci).animations[crate::bg_public_h::TORSO_ATTACK2 as usize].firstFrame
        && frame < (*ci).animations[crate::bg_public_h::TORSO_ATTACK2 as usize].firstFrame + 6
    {
        return 1i32 + frame
            - (*ci).animations[crate::bg_public_h::TORSO_ATTACK2 as usize].firstFrame;
    }
    return 0;
}
/*
==============
CG_CalculateWeaponPosition
==============
*/

unsafe extern "C" fn CG_CalculateWeaponPosition(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut scale: f32 = 0.;
    let mut delta: i32 = 0;
    let mut fracsin: f32 = 0.;
    *origin.offset(0) = crate::src::cgame::cg_main::cg.refdef.vieworg[0];
    *origin.offset(1) = crate::src::cgame::cg_main::cg.refdef.vieworg[1];
    *origin.offset(2) = crate::src::cgame::cg_main::cg.refdef.vieworg[2];
    *angles.offset(0) = crate::src::cgame::cg_main::cg.refdefViewAngles[0];
    *angles.offset(1) = crate::src::cgame::cg_main::cg.refdefViewAngles[1];
    *angles.offset(2) = crate::src::cgame::cg_main::cg.refdefViewAngles[2];
    // on odd legs, invert some angles
    if crate::src::cgame::cg_main::cg.bobcycle & 1 != 0 {
        scale = -crate::src::cgame::cg_main::cg.xyspeed
    } else {
        scale = crate::src::cgame::cg_main::cg.xyspeed
    }
    // gun angles from bobbing
    let ref mut fresh1 = *angles.offset(2);
    *fresh1 = (*fresh1 as f64 + (scale * crate::src::cgame::cg_main::cg.bobfracsin) as f64 * 0.005)
        as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh2 = *angles.offset(1);
    *fresh2 = (*fresh2 as f64 + (scale * crate::src::cgame::cg_main::cg.bobfracsin) as f64 * 0.01)
        as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh3 = *angles.offset(0);
    *fresh3 = (*fresh3 as f64
        + (crate::src::cgame::cg_main::cg.xyspeed * crate::src::cgame::cg_main::cg.bobfracsin)
            as f64
            * 0.005) as crate::src::qcommon::q_shared::vec_t;
    // drop the weapon when landing
    delta = crate::src::cgame::cg_main::cg.time - crate::src::cgame::cg_main::cg.landTime;
    if delta < 150 {
        let ref mut fresh4 = *origin.offset(2);
        *fresh4 = (*fresh4 as f64
            + crate::src::cgame::cg_main::cg.landChange as f64 * 0.25 * delta as f64 / 150f64)
            as crate::src::qcommon::q_shared::vec_t
    } else if delta < 150 + 300 {
        let ref mut fresh5 = *origin.offset(2);
        *fresh5 = (*fresh5 as f64
            + crate::src::cgame::cg_main::cg.landChange as f64 * 0.25 * (150 + 300 - delta) as f64
                / 300f64) as crate::src::qcommon::q_shared::vec_t
    }
    // idle drift
    scale = crate::src::cgame::cg_main::cg.xyspeed + 40f32;
    fracsin = crate::stdlib::sin(crate::src::cgame::cg_main::cg.time as f64 * 0.001) as f32;
    let ref mut fresh6 = *angles.offset(2);
    *fresh6 =
        (*fresh6 as f64 + (scale * fracsin) as f64 * 0.01) as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh7 = *angles.offset(1);
    *fresh7 =
        (*fresh7 as f64 + (scale * fracsin) as f64 * 0.01) as crate::src::qcommon::q_shared::vec_t;
    let ref mut fresh8 = *angles.offset(0);
    *fresh8 =
        (*fresh8 as f64 + (scale * fracsin) as f64 * 0.01) as crate::src::qcommon::q_shared::vec_t;
}
/*
===============
CG_LightningBolt

Origin will be the exact tag point, which is slightly
different than the muzzle point used for determining hits.
The cent should be the non-predicted cent if it is from the player,
so the endpoint will reflect the simulated strike (lagging the predicted
angle)
===============
*/

unsafe extern "C" fn CG_LightningBolt(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) {
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
    let mut beam: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut muzzlePoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut endPoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut anim: i32 = 0;
    if (*cent).currentState.weapon != crate::bg_public_h::WP_LIGHTNING as i32 {
        return;
    }
    crate::stdlib::memset(
        &mut beam as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    // CPMA  "true" lightning
    if (*cent).currentState.number
        == crate::src::cgame::cg_main::cg
            .predictedPlayerState
            .clientNum
        && crate::src::cgame::cg_main::cg_trueLightning.value != 0f32
    {
        let mut angle: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut _i: i32 = 0;

        for i in 0..3 {
            let mut a: f32 = (*cent).lerpAngles[i as usize]
                - crate::src::cgame::cg_main::cg.refdefViewAngles[i as usize];

            if a > 180f32 {
                a -= 360f32
            }

            if a < -180f32 {
                a += 360f32
            }

            angle[i as usize] = (crate::src::cgame::cg_main::cg.refdefViewAngles[i as usize] as f64
                + a as f64 * (1.0 - crate::src::cgame::cg_main::cg_trueLightning.value as f64))
                as crate::src::qcommon::q_shared::vec_t;

            if angle[i as usize] < 0f32 {
                angle[i as usize] += 360f32
            }

            if angle[i as usize] > 360f32 {
                angle[i as usize] -= 360f32
            }
        }
        crate::src::qcommon::q_math::AngleVectors(
            angle.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            forward.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
        muzzlePoint[0] = (*cent).lerpOrigin[0];
        muzzlePoint[1] = (*cent).lerpOrigin[1];
        muzzlePoint[2] = (*cent).lerpOrigin[2]
    //		VectorCopy(cg.refdef.vieworg, muzzlePoint );
    } else {
        // !CPMA
        crate::src::qcommon::q_math::AngleVectors(
            (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            forward.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
        muzzlePoint[0] = (*cent).lerpOrigin[0];
        muzzlePoint[1] = (*cent).lerpOrigin[1];
        muzzlePoint[2] = (*cent).lerpOrigin[2]
    }
    anim = (*cent).currentState.legsAnim & !(128);
    if anim == crate::bg_public_h::LEGS_WALKCR as i32
        || anim == crate::bg_public_h::LEGS_IDLECR as i32
    {
        muzzlePoint[2] += 12f32
    } else {
        muzzlePoint[2] += 26f32
    }
    muzzlePoint[0] = muzzlePoint[0] + forward[0] * 14f32;
    muzzlePoint[1] = muzzlePoint[1] + forward[1] * 14f32;
    muzzlePoint[2] = muzzlePoint[2] + forward[2] * 14f32;
    // project forward by the lightning range
    endPoint[0] = muzzlePoint[0] + forward[0] * 768f32;
    endPoint[1] = muzzlePoint[1] + forward[1] * 768f32;
    endPoint[2] = muzzlePoint[2] + forward[2] * 768f32;
    // see if it hit a wall
    crate::src::cgame::cg_predict::CG_Trace(
        &mut trace,
        muzzlePoint.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        endPoint.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*cent).currentState.number,
        1 | 0x2000000 | 0x4000000,
    );
    // this is the endpoint
    beam.oldorigin[0] = trace.endpos[0];
    beam.oldorigin[1] = trace.endpos[1];
    beam.oldorigin[2] = trace.endpos[2];
    // use the provided origin, even though it may be slightly
    // different than the muzzle origin
    beam.origin[0] = *origin.offset(0);
    beam.origin[1] = *origin.offset(1);
    beam.origin[2] = *origin.offset(2);
    beam.reType = crate::tr_types_h::RT_LIGHTNING;
    beam.customShader = crate::src::cgame::cg_main::cgs.media.lightningShader;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut beam);
    // add the impact flare if it hit something
    if (trace.fraction as f64) < 1.0 {
        let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        dir[0] = beam.oldorigin[0] - beam.origin[0];
        dir[1] = beam.oldorigin[1] - beam.origin[1];
        dir[2] = beam.oldorigin[2] - beam.origin[2];
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        crate::stdlib::memset(
            &mut beam as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
        );
        beam.hModel = crate::src::cgame::cg_main::cgs
            .media
            .lightningExplosionModel;
        beam.origin[0] = trace.endpos[0] + dir[0] * -16f32;
        beam.origin[1] = trace.endpos[1] + dir[1] * -16f32;
        beam.origin[2] = trace.endpos[2] + dir[2] * -16f32;
        // make a random orientation
        angles[0] = (crate::stdlib::rand() % 360) as crate::src::qcommon::q_shared::vec_t;
        angles[1] = (crate::stdlib::rand() % 360) as crate::src::qcommon::q_shared::vec_t;
        angles[2] = (crate::stdlib::rand() % 360) as crate::src::qcommon::q_shared::vec_t;
        crate::src::qcommon::q_math::AnglesToAxis(
            angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            beam.axis.as_mut_ptr(),
        );
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut beam);
    };
}

unsafe extern "C" fn CG_MachinegunSpinAngle(mut cent: *mut crate::cg_local_h::centity_t) -> f32 {
    let mut delta: i32 = 0;
    let mut angle: f32 = 0.;
    let mut speed: f32 = 0.;
    delta = crate::src::cgame::cg_main::cg.time - (*cent).pe.barrelTime;
    if (*cent).pe.barrelSpinning as u64 != 0 {
        angle = ((*cent).pe.barrelAngle as f64 + delta as f64 * 0.9) as f32
    } else {
        if delta > 1000 {
            delta = 1000
        }
        speed = (0.5 * (0.9 + ((1000 - delta) as f32 / 1000f32) as f64)) as f32;
        angle = (*cent).pe.barrelAngle + delta as f32 * speed
    }
    if (*cent).pe.barrelSpinning == ((*cent).currentState.eFlags & 0x100 == 0) as u32 {
        (*cent).pe.barrelTime = crate::src::cgame::cg_main::cg.time;
        (*cent).pe.barrelAngle = crate::src::qcommon::q_math::AngleMod(angle);
        (*cent).pe.barrelSpinning =
            ((*cent).currentState.eFlags & 0x100 != 0) as crate::src::qcommon::q_shared::qboolean
    }
    return angle;
}
/*
========================
CG_AddWeaponWithPowerups
========================
*/

unsafe extern "C" fn CG_AddWeaponWithPowerups(
    mut gun: *mut crate::tr_types_h::refEntity_t,
    mut powerups: i32,
) {
    // add powerup effects
    if powerups & (1) << crate::bg_public_h::PW_INVIS as i32 != 0 {
        (*gun).customShader = crate::src::cgame::cg_main::cgs.media.invisShader;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(gun);
    } else {
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(gun);
        if powerups & (1) << crate::bg_public_h::PW_BATTLESUIT as i32 != 0 {
            (*gun).customShader = crate::src::cgame::cg_main::cgs.media.battleWeaponShader;
            crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(gun);
        }
        if powerups & (1) << crate::bg_public_h::PW_QUAD as i32 != 0 {
            (*gun).customShader = crate::src::cgame::cg_main::cgs.media.quadWeaponShader;
            crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(gun);
        }
    };
}
/*
=============
CG_AddPlayerWeapon

Used for both the view weapon (ps is valid) and the world modelother character models (ps is NULL)
The main player will have this called for BOTH cases, so effects like light and
sound should only be done on the world model case.
=============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddPlayerWeapon(
    mut parent: *mut crate::tr_types_h::refEntity_t,
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut cent: *mut crate::cg_local_h::centity_t,
    mut _team: i32,
) {
    let mut gun: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut barrel: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut flash: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut weaponNum: crate::bg_public_h::weapon_t = crate::bg_public_h::WP_NONE;
    let mut weapon: *mut crate::cg_local_h::weaponInfo_t =
        0 as *mut crate::cg_local_h::weaponInfo_t;
    let mut nonPredictedCent: *mut crate::cg_local_h::centity_t =
        0 as *mut crate::cg_local_h::centity_t;
    let mut lerped: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    weaponNum = (*cent).currentState.weapon as crate::bg_public_h::weapon_t;
    CG_RegisterWeapon(weaponNum as i32);
    weapon = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset(weaponNum as isize) as *mut crate::cg_local_h::weaponInfo_t;
    // add the weapon
    crate::stdlib::memset(
        &mut gun as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    gun.lightingOrigin[0] = (*parent).lightingOrigin[0];
    gun.lightingOrigin[1] = (*parent).lightingOrigin[1];
    gun.lightingOrigin[2] = (*parent).lightingOrigin[2];
    gun.shadowPlane = (*parent).shadowPlane;
    gun.renderfx = (*parent).renderfx;
    // set custom shading for railgun refire rate
    if weaponNum == crate::bg_public_h::WP_RAILGUN {
        let mut ci: *mut crate::cg_local_h::clientInfo_t = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*cent).currentState.clientNum as isize)
            as *mut crate::cg_local_h::clientInfo_t;
        if (*cent).pe.railFireTime + 1500 > crate::src::cgame::cg_main::cg.time {
            let mut scale: i32 =
                255 * (crate::src::cgame::cg_main::cg.time - (*cent).pe.railFireTime) / 1500;
            gun.shaderRGBA[0] =
                ((*ci).c1RGBA[0] as i32 * scale >> 8) as crate::src::qcommon::q_shared::byte;
            gun.shaderRGBA[1] =
                ((*ci).c1RGBA[1] as i32 * scale >> 8) as crate::src::qcommon::q_shared::byte;
            gun.shaderRGBA[2] =
                ((*ci).c1RGBA[2] as i32 * scale >> 8) as crate::src::qcommon::q_shared::byte;
            gun.shaderRGBA[3] = 255
        } else {
            gun.shaderRGBA[0] = (*ci).c1RGBA[0];
            gun.shaderRGBA[1] = (*ci).c1RGBA[1];
            gun.shaderRGBA[2] = (*ci).c1RGBA[2];
            gun.shaderRGBA[3] = (*ci).c1RGBA[3]
        }
    }
    gun.hModel = (*weapon).weaponModel;
    if gun.hModel == 0 {
        return;
    }
    if ps.is_null() {
        // add weapon ready sound
        (*cent).pe.lightningFiring = crate::src::qcommon::q_shared::qfalse as i32;
        if (*cent).currentState.eFlags & 0x100 != 0 && (*weapon).firingSound != 0 {
            // lightning gun and guantlet make a different sound when fire is held down
            crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                (*weapon).firingSound,
            );
            (*cent).pe.lightningFiring = crate::src::qcommon::q_shared::qtrue as i32
        } else if (*weapon).readySound != 0 {
            crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                (*weapon).readySound,
            );
        }
    }
    crate::src::cgame::cg_syscalls::trap_R_LerpTag(
        &mut lerped,
        (*parent).hModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0 - (*parent).backlerp as f64) as f32,
        b"tag_weapon\x00" as *const u8 as *const i8,
    );
    gun.origin[0] = (*parent).origin[0];
    gun.origin[1] = (*parent).origin[1];
    gun.origin[2] = (*parent).origin[2];
    gun.origin[0] = gun.origin[0] + (*parent).axis[0][0] * lerped.origin[0];
    gun.origin[1] = gun.origin[1] + (*parent).axis[0][1] * lerped.origin[0];
    gun.origin[2] = gun.origin[2] + (*parent).axis[0][2] * lerped.origin[0];
    // Make weapon appear left-handed for 2 and centered for 3
    if !ps.is_null() && crate::src::cgame::cg_main::cg_drawGun.integer == 2 {
        gun.origin[0] = gun.origin[0] + (*parent).axis[1][0] * -lerped.origin[1];
        gun.origin[1] = gun.origin[1] + (*parent).axis[1][1] * -lerped.origin[1];
        gun.origin[2] = gun.origin[2] + (*parent).axis[1][2] * -lerped.origin[1]
    } else if ps.is_null() || crate::src::cgame::cg_main::cg_drawGun.integer != 3 {
        gun.origin[0] = gun.origin[0] + (*parent).axis[1][0] * lerped.origin[1];
        gun.origin[1] = gun.origin[1] + (*parent).axis[1][1] * lerped.origin[1];
        gun.origin[2] = gun.origin[2] + (*parent).axis[1][2] * lerped.origin[1]
    }
    gun.origin[0] = gun.origin[0] + (*parent).axis[2][0] * lerped.origin[2];
    gun.origin[1] = gun.origin[1] + (*parent).axis[2][1] * lerped.origin[2];
    gun.origin[2] = gun.origin[2] + (*parent).axis[2][2] * lerped.origin[2];
    crate::src::qcommon::q_math::MatrixMultiply(
        lerped.axis.as_mut_ptr(),
        (*parent).axis.as_mut_ptr(),
        gun.axis.as_mut_ptr(),
    );
    gun.backlerp = (*parent).backlerp;
    CG_AddWeaponWithPowerups(&mut gun, (*cent).currentState.powerups);
    // add the spinning barrel
    if (*weapon).barrelModel != 0 {
        crate::stdlib::memset(
            &mut barrel as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
        );
        barrel.lightingOrigin[0] = (*parent).lightingOrigin[0];
        barrel.lightingOrigin[1] = (*parent).lightingOrigin[1];
        barrel.lightingOrigin[2] = (*parent).lightingOrigin[2];
        barrel.shadowPlane = (*parent).shadowPlane;
        barrel.renderfx = (*parent).renderfx;
        barrel.hModel = (*weapon).barrelModel;
        angles[1] = 0f32;
        angles[0] = 0f32;
        angles[2] = CG_MachinegunSpinAngle(cent);
        crate::src::qcommon::q_math::AnglesToAxis(
            angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            barrel.axis.as_mut_ptr(),
        );
        crate::src::cgame::cg_ents::CG_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut gun,
            (*weapon).weaponModel,
            b"tag_barrel\x00" as *const u8 as *mut i8,
        );
        CG_AddWeaponWithPowerups(&mut barrel, (*cent).currentState.powerups);
    }
    // make sure we aren't looking at cg.predictedPlayerEntity for LG
    nonPredictedCent = &mut *crate::src::cgame::cg_main::cg_entities
        .as_mut_ptr()
        .offset((*cent).currentState.clientNum as isize)
        as *mut crate::cg_local_h::centity_t;
    // if the index of the nonPredictedCent is not the same as the clientNum
    // then this is a fake player (like on the single player podiums), so
    // go ahead and use the cent
    if nonPredictedCent.wrapping_offset_from(crate::src::cgame::cg_main::cg_entities.as_mut_ptr())
        != (*cent).currentState.clientNum as isize
    {
        nonPredictedCent = cent
    }
    // add the flash
    if !((weaponNum == crate::bg_public_h::WP_LIGHTNING
        || weaponNum == crate::bg_public_h::WP_GAUNTLET
        || weaponNum == crate::bg_public_h::WP_GRAPPLING_HOOK)
        && (*nonPredictedCent).currentState.eFlags & 0x100 != 0)
    {
        // impulse flash
        if crate::src::cgame::cg_main::cg.time - (*cent).muzzleFlashTime > 20 {
            return;
        }
    }
    crate::stdlib::memset(
        &mut flash as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    flash.lightingOrigin[0] = (*parent).lightingOrigin[0];
    flash.lightingOrigin[1] = (*parent).lightingOrigin[1];
    flash.lightingOrigin[2] = (*parent).lightingOrigin[2];
    flash.shadowPlane = (*parent).shadowPlane;
    flash.renderfx = (*parent).renderfx;
    flash.hModel = (*weapon).flashModel;
    if flash.hModel == 0 {
        return;
    }
    angles[1] = 0f32;
    angles[0] = 0f32;
    angles[2] = (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 10f64)
        as crate::src::qcommon::q_shared::vec_t;
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        flash.axis.as_mut_ptr(),
    );
    // colorize the railgun blast
    if weaponNum == crate::bg_public_h::WP_RAILGUN {
        let mut ci_0: *mut crate::cg_local_h::clientInfo_t =
            0 as *mut crate::cg_local_h::clientInfo_t;
        ci_0 = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*cent).currentState.clientNum as isize)
            as *mut crate::cg_local_h::clientInfo_t;
        flash.shaderRGBA[0] = (255f32 * (*ci_0).color1[0]) as crate::src::qcommon::q_shared::byte;
        flash.shaderRGBA[1] = (255f32 * (*ci_0).color1[1]) as crate::src::qcommon::q_shared::byte;
        flash.shaderRGBA[2] = (255f32 * (*ci_0).color1[2]) as crate::src::qcommon::q_shared::byte
    }
    crate::src::cgame::cg_ents::CG_PositionRotatedEntityOnTag(
        &mut flash,
        &mut gun,
        (*weapon).weaponModel,
        b"tag_flash\x00" as *const u8 as *mut i8,
    );
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut flash);
    if !ps.is_null()
        || crate::src::cgame::cg_main::cg.renderingThirdPerson != 0
        || (*cent).currentState.number
            != crate::src::cgame::cg_main::cg
                .predictedPlayerState
                .clientNum
    {
        // add lightning bolt
        CG_LightningBolt(nonPredictedCent, flash.origin.as_mut_ptr());
        if (*weapon).flashDlightColor[0] != 0.
            || (*weapon).flashDlightColor[1] != 0.
            || (*weapon).flashDlightColor[2] != 0.
        {
            crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
                flash.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (300i32 + (crate::stdlib::rand() & 31i32)) as f32,
                (*weapon).flashDlightColor[0usize],
                (*weapon).flashDlightColor[1usize],
                (*weapon).flashDlightColor[2usize],
            );
        }
    };
}
/*
==============
CG_AddViewWeapon

Add the weapon, and flash for the player's view
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddViewWeapon(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
) {
    let mut hand: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
        reType: crate::tr_types_h::RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: crate::src::qcommon::q_shared::qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut fovOffset: f32 = 0.;
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut weapon: *mut crate::cg_local_h::weaponInfo_t =
        0 as *mut crate::cg_local_h::weaponInfo_t;
    if (*ps).persistant[crate::bg_public_h::PERS_TEAM as usize]
        == crate::bg_public_h::TEAM_SPECTATOR as i32
    {
        return;
    }
    if (*ps).pm_type == crate::bg_public_h::PM_INTERMISSION as i32 {
        return;
    }
    // no gun if in third person view or a camera is active
    //if ( cg.renderingThirdPerson || cg.cameraMode) {
    if crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 != 0 {
        return;
    }
    // allow the gun to be completely removed
    if crate::src::cgame::cg_main::cg_drawGun.integer == 0 {
        let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        if crate::src::cgame::cg_main::cg.predictedPlayerState.eFlags & 0x100 != 0 {
            // special hack for lightning gun...
            origin[0] = crate::src::cgame::cg_main::cg.refdef.vieworg[0];
            origin[1] = crate::src::cgame::cg_main::cg.refdef.vieworg[1];
            origin[2] = crate::src::cgame::cg_main::cg.refdef.vieworg[2];
            origin[0] = origin[0] + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][0] * -8f32;
            origin[1] = origin[1] + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][1] * -8f32;
            origin[2] = origin[2] + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][2] * -8f32;
            CG_LightningBolt(
                &mut *crate::src::cgame::cg_main::cg_entities
                    .as_mut_ptr()
                    .offset((*ps).clientNum as isize),
                origin.as_mut_ptr(),
            );
        }
        return;
    }
    // don't draw if testing a gun model
    if crate::src::cgame::cg_main::cg.testGun as u64 != 0 {
        return;
    }
    // drop gun lower at higher fov
    if crate::src::cgame::cg_main::cg_fov.integer > 90 {
        fovOffset = (-0.2 * (crate::src::cgame::cg_main::cg_fov.integer - 90) as f64) as f32
    } else {
        fovOffset = 0f32
    } // &cg_entities[cg.snap->ps.clientNum];
    cent = &mut crate::src::cgame::cg_main::cg.predictedPlayerEntity;
    CG_RegisterWeapon((*ps).weapon);
    weapon = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset((*ps).weapon as isize) as *mut crate::cg_local_h::weaponInfo_t;
    crate::stdlib::memset(
        &mut hand as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    // set up gun position
    CG_CalculateWeaponPosition(hand.origin.as_mut_ptr(), angles.as_mut_ptr());
    hand.origin[0] = hand.origin[0]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][0]
            * crate::src::cgame::cg_main::cg_gun_x.value;
    hand.origin[1] = hand.origin[1]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][1]
            * crate::src::cgame::cg_main::cg_gun_x.value;
    hand.origin[2] = hand.origin[2]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[0][2]
            * crate::src::cgame::cg_main::cg_gun_x.value;
    hand.origin[0] = hand.origin[0]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[1][0]
            * crate::src::cgame::cg_main::cg_gun_y.value;
    hand.origin[1] = hand.origin[1]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[1][1]
            * crate::src::cgame::cg_main::cg_gun_y.value;
    hand.origin[2] = hand.origin[2]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[1][2]
            * crate::src::cgame::cg_main::cg_gun_y.value;
    hand.origin[0] = hand.origin[0]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][0]
            * (crate::src::cgame::cg_main::cg_gun_z.value + fovOffset);
    hand.origin[1] = hand.origin[1]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][1]
            * (crate::src::cgame::cg_main::cg_gun_z.value + fovOffset);
    hand.origin[2] = hand.origin[2]
        + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][2]
            * (crate::src::cgame::cg_main::cg_gun_z.value + fovOffset);
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        hand.axis.as_mut_ptr(),
    );
    // map torso animations to weapon animations
    if crate::src::cgame::cg_main::cg_gun_frame.integer != 0 {
        // development tool
        hand.oldframe = crate::src::cgame::cg_main::cg_gun_frame.integer;
        hand.frame = hand.oldframe;
        hand.backlerp = 0f32
    } else {
        // get clientinfo for animation map
        ci = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*cent).currentState.clientNum as isize)
            as *mut crate::cg_local_h::clientInfo_t;
        hand.frame = CG_MapTorsoToWeaponFrame(ci, (*cent).pe.torso.frame);
        hand.oldframe = CG_MapTorsoToWeaponFrame(ci, (*cent).pe.torso.oldFrame);
        hand.backlerp = (*cent).pe.torso.backlerp
    }
    hand.hModel = (*weapon).handsModel;
    hand.renderfx = 0x8 | 0x4 | 0x1;
    // add everything onto the hand
    CG_AddPlayerWeapon(
        &mut hand,
        ps,
        &mut crate::src::cgame::cg_main::cg.predictedPlayerEntity,
        (*ps).persistant[crate::bg_public_h::PERS_TEAM as usize],
    );
}
/*
==============================================================================

WEAPON SELECTION

==============================================================================
*/
/*
===================
CG_DrawWeaponSelect
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_DrawWeaponSelect() {
    let mut i: i32 = 0;
    let mut bits: i32 = 0;
    let mut count: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    let mut name: *mut i8 = 0 as *mut i8;
    let mut color: *mut f32 = 0 as *mut f32;
    // don't display if dead
    if crate::src::cgame::cg_main::cg.predictedPlayerState.stats
        [crate::bg_public_h::STAT_HEALTH as usize]
        <= 0
    {
        return;
    }
    color = crate::src::cgame::cg_drawtools::CG_FadeColor(
        crate::src::cgame::cg_main::cg.weaponSelectTime,
        1400,
    );
    if color.is_null() {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(color);
    // showing weapon select clears pickup item display, but not the blend blob
    crate::src::cgame::cg_main::cg.itemPickupTime = 0;
    // count the number of weapons owned
    bits =
        (*crate::src::cgame::cg_main::cg.snap).ps.stats[crate::bg_public_h::STAT_WEAPONS as usize];
    count = 0;
    i = 1;
    while i < 16 {
        if bits & (1) << i != 0 {
            count += 1
        }
        i += 1
    }
    x = 320 - count * 20;
    y = 380;
    i = 1;
    while i < 16 {
        if !(bits & (1) << i == 0) {
            CG_RegisterWeapon(i);
            // draw weapon icon
            crate::src::cgame::cg_drawtools::CG_DrawPic(
                x as f32,
                y as f32,
                32f32,
                32f32,
                crate::src::cgame::cg_main::cg_weapons[i as usize].weaponIcon,
            );
            // draw selection marker
            if i == crate::src::cgame::cg_main::cg.weaponSelect {
                crate::src::cgame::cg_drawtools::CG_DrawPic(
                    (x - 4i32) as f32,
                    (y - 4i32) as f32,
                    40f32,
                    40f32,
                    crate::src::cgame::cg_main::cgs.media.selectShader,
                );
            }
            // no ammo cross on top
            if (*crate::src::cgame::cg_main::cg.snap).ps.ammo[i as usize] == 0 {
                crate::src::cgame::cg_drawtools::CG_DrawPic(
                    x as f32,
                    y as f32,
                    32f32,
                    32f32,
                    crate::src::cgame::cg_main::cgs.media.noammoShader,
                );
            }
            x += 40
        }
        i += 1
    }
    // draw the selected name
    if !crate::src::cgame::cg_main::cg_weapons[crate::src::cgame::cg_main::cg.weaponSelect as usize]
        .item
        .is_null()
    {
        name = (*crate::src::cgame::cg_main::cg_weapons
            [crate::src::cgame::cg_main::cg.weaponSelect as usize]
            .item)
            .pickup_name;
        if !name.is_null() {
            w = crate::src::cgame::cg_drawtools::CG_DrawStrlen(name) * 16;
            x = (640 - w) / 2;
            crate::src::cgame::cg_drawtools::CG_DrawBigStringColor(x, y - 22i32, name, color);
        }
    }
    crate::src::cgame::cg_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
===============
CG_WeaponSelectable
===============
*/

unsafe extern "C" fn CG_WeaponSelectable(mut i: i32) -> crate::src::qcommon::q_shared::qboolean {
    if (*crate::src::cgame::cg_main::cg.snap).ps.ammo[i as usize] == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << i
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
CG_NextWeapon_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_NextWeapon_f() {
    let mut i: i32 = 0;
    let mut original: i32 = 0;
    if crate::src::cgame::cg_main::cg.snap.is_null() {
        return;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_flags & 4096 != 0 {
        return;
    }
    crate::src::cgame::cg_main::cg.weaponSelectTime = crate::src::cgame::cg_main::cg.time;
    original = crate::src::cgame::cg_main::cg.weaponSelect;
    i = 0;
    while i < 16 {
        crate::src::cgame::cg_main::cg.weaponSelect += 1;
        if crate::src::cgame::cg_main::cg.weaponSelect == 16 {
            crate::src::cgame::cg_main::cg.weaponSelect = 0
        }
        if !(crate::src::cgame::cg_main::cg.weaponSelect == crate::bg_public_h::WP_GAUNTLET as i32)
        {
            if CG_WeaponSelectable(crate::src::cgame::cg_main::cg.weaponSelect) as u64 != 0 {
                break;
            }
        }
        i += 1
        // never cycle to gauntlet
    }
    if i == 16 {
        crate::src::cgame::cg_main::cg.weaponSelect = original
    };
}
/*
===============
CG_PrevWeapon_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PrevWeapon_f() {
    let mut i: i32 = 0;
    let mut original: i32 = 0;
    if crate::src::cgame::cg_main::cg.snap.is_null() {
        return;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_flags & 4096 != 0 {
        return;
    }
    crate::src::cgame::cg_main::cg.weaponSelectTime = crate::src::cgame::cg_main::cg.time;
    original = crate::src::cgame::cg_main::cg.weaponSelect;
    i = 0;
    while i < 16 {
        crate::src::cgame::cg_main::cg.weaponSelect -= 1;
        if crate::src::cgame::cg_main::cg.weaponSelect == -(1) {
            crate::src::cgame::cg_main::cg.weaponSelect = 16 - 1
        }
        if !(crate::src::cgame::cg_main::cg.weaponSelect == crate::bg_public_h::WP_GAUNTLET as i32)
        {
            if CG_WeaponSelectable(crate::src::cgame::cg_main::cg.weaponSelect) as u64 != 0 {
                break;
            }
        }
        i += 1
        // never cycle to gauntlet
    }
    if i == 16 {
        crate::src::cgame::cg_main::cg.weaponSelect = original
    };
}
/*
===============
CG_Weapon_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Weapon_f() {
    let mut num: i32 = 0;
    if crate::src::cgame::cg_main::cg.snap.is_null() {
        return;
    }
    if (*crate::src::cgame::cg_main::cg.snap).ps.pm_flags & 4096 != 0 {
        return;
    }
    num = atoi(crate::src::cgame::cg_main::CG_Argv(1));
    if num < 1 || num > 16 - 1 {
        return;
    }
    crate::src::cgame::cg_main::cg.weaponSelectTime = crate::src::cgame::cg_main::cg.time;
    if (*crate::src::cgame::cg_main::cg.snap).ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << num
        == 0
    {
        return;
        // don't have the weapon
    }
    crate::src::cgame::cg_main::cg.weaponSelect = num;
}
/*
===================
CG_OutOfAmmoChange

The current weapon has just run out of ammo
===================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_OutOfAmmoChange() {
    let mut i: i32 = 0;
    crate::src::cgame::cg_main::cg.weaponSelectTime = crate::src::cgame::cg_main::cg.time;
    i = 16 - 1;
    while i > 0 {
        if CG_WeaponSelectable(i) as u64 != 0 {
            crate::src::cgame::cg_main::cg.weaponSelect = i;
            break;
        } else {
            i -= 1
        }
    }
}
/*
===================================================================================================

WEAPON EVENTS

===================================================================================================
*/
/*
================
CG_FireWeapon

Caused by an EV_FIRE_WEAPON event
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_FireWeapon(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut c: i32 = 0;
    let mut weap: *mut crate::cg_local_h::weaponInfo_t = 0 as *mut crate::cg_local_h::weaponInfo_t;
    ent = &mut (*cent).currentState;
    if (*ent).weapon == crate::bg_public_h::WP_NONE as i32 {
        return;
    }
    if (*ent).weapon >= crate::bg_public_h::WP_NUM_WEAPONS as i32 {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_FireWeapon: ent->weapon >= WP_NUM_WEAPONS\x00" as *const u8 as *const i8,
        );
    }
    weap = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset((*ent).weapon as isize) as *mut crate::cg_local_h::weaponInfo_t;
    // mark the entity as muzzle flashing, so when it is added it will
    // append the flash to the weapon model
    (*cent).muzzleFlashTime = crate::src::cgame::cg_main::cg.time;
    // lightning gun only does this this on initial press
    if (*ent).weapon == crate::bg_public_h::WP_LIGHTNING as i32 {
        if (*cent).pe.lightningFiring != 0 {
            return;
        }
    }
    if (*ent).weapon == crate::bg_public_h::WP_RAILGUN as i32 {
        (*cent).pe.railFireTime = crate::src::cgame::cg_main::cg.time
    }
    // play quad sound if needed
    if (*cent).currentState.powerups & (1) << crate::bg_public_h::PW_QUAD as i32 != 0 {
        crate::src::cgame::cg_syscalls::trap_S_StartSound(
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            (*cent).currentState.number,
            crate::src::qcommon::q_shared::CHAN_ITEM as i32,
            crate::src::cgame::cg_main::cgs.media.quadSound,
        );
    }
    // play a sound
    c = 0;
    while c < 4 {
        if (*weap).flashSound[c as usize] == 0 {
            break;
        }
        c += 1
    }
    if c > 0 {
        c = crate::stdlib::rand() % c;
        if (*weap).flashSound[c as usize] != 0 {
            crate::src::cgame::cg_syscalls::trap_S_StartSound(
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                (*ent).number,
                crate::src::qcommon::q_shared::CHAN_WEAPON as i32,
                (*weap).flashSound[c as usize],
            );
        }
    }
    // do brass ejection
    if (*weap).ejectBrassFunc.is_some() && crate::src::cgame::cg_main::cg_brassTime.integer > 0 {
        (*weap).ejectBrassFunc.expect("non-null function pointer")(cent);
    };
}
/*
=================
CG_MissileHitWall

Caused by an EV_MISSILE_MISS event, or directly by local bullet tracing
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_MissileHitWall(
    mut weapon: i32,
    mut clientNum: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut _soundType: crate::cg_local_h::impactSound_t,
) {
    let mut mod_0: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut mark: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut shader: crate::src::qcommon::q_shared::qhandle_t = 0;
    let mut sfx: crate::src::qcommon::q_shared::sfxHandle_t = 0;
    let mut radius: f32 = 0.;
    let mut light: f32 = 0.;
    let mut lightColor: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut le: *mut crate::cg_local_h::localEntity_t = 0 as *mut crate::cg_local_h::localEntity_t;
    let mut r: i32 = 0;
    let mut alphaFade: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut isSprite: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut duration: i32 = 0;
    let mut sprOrg: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut sprVel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    mod_0 = 0;
    shader = 0;
    light = 0f32;
    lightColor[0] = 1f32;
    lightColor[1] = 1f32;
    lightColor[2] = 0f32;
    // set defaults
    isSprite = crate::src::qcommon::q_shared::qfalse;
    duration = 600;
    match weapon {
        4 => {
            mod_0 = crate::src::cgame::cg_main::cgs.media.dishFlashModel;
            shader = crate::src::cgame::cg_main::cgs.media.grenadeExplosionShader;
            sfx = crate::src::cgame::cg_main::cgs.media.sfx_rockexp;
            mark = crate::src::cgame::cg_main::cgs.media.burnMarkShader;
            radius = 64f32;
            light = 300f32;
            isSprite = crate::src::qcommon::q_shared::qtrue
        }
        5 => {
            mod_0 = crate::src::cgame::cg_main::cgs.media.dishFlashModel;
            shader = crate::src::cgame::cg_main::cgs.media.rocketExplosionShader;
            sfx = crate::src::cgame::cg_main::cgs.media.sfx_rockexp;
            mark = crate::src::cgame::cg_main::cgs.media.burnMarkShader;
            radius = 64f32;
            light = 300f32;
            isSprite = crate::src::qcommon::q_shared::qtrue;
            duration = 1000;
            lightColor[0] = 1f32;
            lightColor[1] = 0.75;
            lightColor[2] = 0f32;
            if crate::src::cgame::cg_main::cg_oldRocket.integer == 0 {
                // explosion sprite animation
                sprOrg[0] = *origin.offset(0) + *dir.offset(0) * 24f32;
                sprOrg[1] = *origin.offset(1) + *dir.offset(1) * 24f32;
                sprOrg[2] = *origin.offset(2) + *dir.offset(2) * 24f32;
                sprVel[0] = *dir.offset(0) * 64f32;
                sprVel[1] = *dir.offset(1) * 64f32;
                sprVel[2] = *dir.offset(2) * 64f32;
                crate::src::cgame::cg_particles::CG_ParticleExplosion(
                    b"explode1\x00" as *const u8 as *mut i8,
                    sprOrg.as_mut_ptr(),
                    sprVel.as_mut_ptr(),
                    1400i32,
                    20i32,
                    30i32,
                );
            }
        }
        7 => {
            mod_0 = crate::src::cgame::cg_main::cgs.media.ringFlashModel;
            shader = crate::src::cgame::cg_main::cgs.media.railExplosionShader;
            //sfx = cgs.media.sfx_railg;
            sfx = crate::src::cgame::cg_main::cgs.media.sfx_plasmaexp;
            mark = crate::src::cgame::cg_main::cgs.media.energyMarkShader;
            radius = 24f32
        }
        8 => {
            mod_0 = crate::src::cgame::cg_main::cgs.media.ringFlashModel;
            shader = crate::src::cgame::cg_main::cgs.media.plasmaExplosionShader;
            sfx = crate::src::cgame::cg_main::cgs.media.sfx_plasmaexp;
            mark = crate::src::cgame::cg_main::cgs.media.energyMarkShader;
            radius = 16f32
        }
        9 => {
            mod_0 = crate::src::cgame::cg_main::cgs.media.dishFlashModel;
            shader = crate::src::cgame::cg_main::cgs.media.bfgExplosionShader;
            sfx = crate::src::cgame::cg_main::cgs.media.sfx_rockexp;
            mark = crate::src::cgame::cg_main::cgs.media.burnMarkShader;
            radius = 32f32;
            isSprite = crate::src::qcommon::q_shared::qtrue
        }
        3 => {
            mod_0 = crate::src::cgame::cg_main::cgs.media.bulletFlashModel;
            shader = crate::src::cgame::cg_main::cgs.media.bulletExplosionShader;
            mark = crate::src::cgame::cg_main::cgs.media.bulletMarkShader;
            sfx = 0;
            radius = 4f32
        }
        2 => {
            mod_0 = crate::src::cgame::cg_main::cgs.media.bulletFlashModel;
            shader = crate::src::cgame::cg_main::cgs.media.bulletExplosionShader;
            mark = crate::src::cgame::cg_main::cgs.media.bulletMarkShader;
            r = crate::stdlib::rand() & 3;
            if r == 0 {
                sfx = crate::src::cgame::cg_main::cgs.media.sfx_ric1
            } else if r == 1 {
                sfx = crate::src::cgame::cg_main::cgs.media.sfx_ric2
            } else {
                sfx = crate::src::cgame::cg_main::cgs.media.sfx_ric3
            }
            radius = 8f32
        }
        6 | _ => {
            // no explosion at LG impact, it is added with the beam
            r = crate::stdlib::rand() & 3;
            if r < 2 {
                sfx = crate::src::cgame::cg_main::cgs.media.sfx_lghit2
            } else if r == 2 {
                sfx = crate::src::cgame::cg_main::cgs.media.sfx_lghit1
            } else {
                sfx = crate::src::cgame::cg_main::cgs.media.sfx_lghit3
            }
            mark = crate::src::cgame::cg_main::cgs.media.holeMarkShader;
            radius = 12f32
        }
    }
    if sfx != 0 {
        crate::src::cgame::cg_syscalls::trap_S_StartSound(
            origin,
            ((1i32) << 10i32) - 2i32,
            crate::src::qcommon::q_shared::CHAN_AUTO as i32,
            sfx,
        );
    }
    //
    // create the explosion
    //
    if mod_0 != 0 {
        le = crate::src::cgame::cg_effects::CG_MakeExplosion(
            origin, dir, mod_0, shader, duration, isSprite,
        );
        (*le).light = light;
        (*le).lightColor[0] = lightColor[0];
        (*le).lightColor[1] = lightColor[1];
        (*le).lightColor[2] = lightColor[2];
        if weapon == crate::bg_public_h::WP_RAILGUN as i32 {
            // colorize with client color
            (*le).color[0] =
                crate::src::cgame::cg_main::cgs.clientinfo[clientNum as usize].color1[0];
            (*le).color[1] =
                crate::src::cgame::cg_main::cgs.clientinfo[clientNum as usize].color1[1];
            (*le).color[2] =
                crate::src::cgame::cg_main::cgs.clientinfo[clientNum as usize].color1[2];
            (*le).refEntity.shaderRGBA[0] =
                ((*le).color[0] * 255f32) as crate::src::qcommon::q_shared::byte;
            (*le).refEntity.shaderRGBA[1] =
                ((*le).color[1] * 255f32) as crate::src::qcommon::q_shared::byte;
            (*le).refEntity.shaderRGBA[2] =
                ((*le).color[2] * 255f32) as crate::src::qcommon::q_shared::byte;
            (*le).refEntity.shaderRGBA[3] = 0xff
        }
    }
    //
    // impact mark
    //
    alphaFade = (mark == crate::src::cgame::cg_main::cgs.media.energyMarkShader)
        as crate::src::qcommon::q_shared::qboolean; // plasma fades alpha, all others fade color
    if weapon == crate::bg_public_h::WP_RAILGUN as i32 {
        let mut color: *mut f32 = 0 as *mut f32;
        // colorize with client color
        color = crate::src::cgame::cg_main::cgs.clientinfo[clientNum as usize]
            .color1
            .as_mut_ptr();
        crate::src::cgame::cg_marks::CG_ImpactMark(
            mark,
            origin as *const crate::src::qcommon::q_shared::vec_t,
            dir as *const crate::src::qcommon::q_shared::vec_t,
            (crate::stdlib::rand() & 0x7fffi32) as f32 / 32767f32 * 360f32,
            *color.offset(0isize),
            *color.offset(1isize),
            *color.offset(2isize),
            1f32,
            alphaFade,
            radius,
            crate::src::qcommon::q_shared::qfalse,
        );
    } else {
        crate::src::cgame::cg_marks::CG_ImpactMark(
            mark,
            origin as *const crate::src::qcommon::q_shared::vec_t,
            dir as *const crate::src::qcommon::q_shared::vec_t,
            (crate::stdlib::rand() & 0x7fffi32) as f32 / 32767f32 * 360f32,
            1f32,
            1f32,
            1f32,
            1f32,
            alphaFade,
            radius,
            crate::src::qcommon::q_shared::qfalse,
        );
    };
}
/*
=================
CG_MissileHitPlayer
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_MissileHitPlayer(
    mut weapon: i32,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut entityNum: i32,
) {
    crate::src::cgame::cg_effects::CG_Bleed(origin, entityNum);
    // some weapons will make an explosion with the blood, while
    // others will just make the blood
    match weapon {
        4 | 5 | 8 | 9 => {
            CG_MissileHitWall(
                weapon,
                0i32,
                origin,
                dir,
                crate::cg_local_h::IMPACTSOUND_FLESH,
            );
        }
        _ => {}
    };
}
/*
============================================================================

SHOTGUN TRACING

============================================================================
*/
/*
================
CG_ShotgunPellet
================
*/

unsafe extern "C" fn CG_ShotgunPellet(
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut skipNum: i32,
) {
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
    let mut sourceContentType: i32 = 0;
    let mut destContentType: i32 = 0;
    crate::src::cgame::cg_predict::CG_Trace(
        &mut tr,
        start as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        end as *const crate::src::qcommon::q_shared::vec_t,
        skipNum,
        1 | 0x2000000 | 0x4000000,
    );
    sourceContentType = crate::src::cgame::cg_predict::CG_PointContents(
        start as *const crate::src::qcommon::q_shared::vec_t,
        0,
    );
    destContentType = crate::src::cgame::cg_predict::CG_PointContents(
        tr.endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0,
    );
    // FIXME: should probably move this cruft into CG_BubbleTrail
    if sourceContentType == destContentType {
        if sourceContentType & 32 != 0 {
            crate::src::cgame::cg_effects::CG_BubbleTrail(start, tr.endpos.as_mut_ptr(), 32f32);
        }
    } else if sourceContentType & 32 != 0 {
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
        crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
            &mut trace,
            end as *const crate::src::qcommon::q_shared::vec_t,
            start as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0,
            32,
        );
        crate::src::cgame::cg_effects::CG_BubbleTrail(start, trace.endpos.as_mut_ptr(), 32f32);
    } else if destContentType & 32 != 0 {
        let mut trace_0: crate::src::qcommon::q_shared::trace_t =
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
        crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
            &mut trace_0,
            start as *const crate::src::qcommon::q_shared::vec_t,
            end as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0 as *const crate::src::qcommon::q_shared::vec_t,
            0,
            32,
        );
        crate::src::cgame::cg_effects::CG_BubbleTrail(
            tr.endpos.as_mut_ptr(),
            trace_0.endpos.as_mut_ptr(),
            32f32,
        );
    }
    if tr.surfaceFlags & 0x10 != 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg_entities[tr.entityNum as usize]
        .currentState
        .eType
        == crate::bg_public_h::ET_PLAYER as i32
    {
        CG_MissileHitPlayer(
            crate::bg_public_h::WP_SHOTGUN as i32,
            tr.endpos.as_mut_ptr(),
            tr.plane.normal.as_mut_ptr(),
            tr.entityNum,
        );
    } else {
        if tr.surfaceFlags & 0x10 != 0 {
            // SURF_NOIMPACT will not make a flame puff or a mark
            return;
        }
        if tr.surfaceFlags & 0x1000 != 0 {
            CG_MissileHitWall(
                crate::bg_public_h::WP_SHOTGUN as i32,
                0i32,
                tr.endpos.as_mut_ptr(),
                tr.plane.normal.as_mut_ptr(),
                crate::cg_local_h::IMPACTSOUND_METAL,
            );
        } else {
            CG_MissileHitWall(
                crate::bg_public_h::WP_SHOTGUN as i32,
                0i32,
                tr.endpos.as_mut_ptr(),
                tr.plane.normal.as_mut_ptr(),
                crate::cg_local_h::IMPACTSOUND_DEFAULT,
            );
        }
    };
}
/*
================
CG_ShotgunPattern

Perform the same traces the server did to locate the
hit splashes
================
*/

unsafe extern "C" fn CG_ShotgunPattern(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut origin2: *mut crate::src::qcommon::q_shared::vec_t,
    mut seed: i32,
    mut otherEntNum: i32,
) {
    let mut i: i32 = 0;
    let mut r: f32 = 0.;
    let mut u: f32 = 0.;
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // derive the right and up vectors from the forward vector, because
    // the client won't have any other information
    crate::src::qcommon::q_math::VectorNormalize2(
        origin2 as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::PerpendicularVector(
        right.as_mut_ptr(),
        forward.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    CrossProduct(
        forward.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        right.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        up.as_mut_ptr(),
    );
    // generate the "random" spread pattern
    i = 0;
    while i < 11 {
        r = crate::src::qcommon::q_math::Q_crandom(&mut seed) * 700f32 * 16f32;
        u = crate::src::qcommon::q_math::Q_crandom(&mut seed) * 700f32 * 16f32;
        end[0] = *origin.offset(0) + forward[0] * (8192i32 * 16) as f32;
        end[1] = *origin.offset(1) + forward[1] * (8192i32 * 16) as f32;
        end[2] = *origin.offset(2) + forward[2] * (8192i32 * 16) as f32;
        end[0] = end[0] + right[0] * r;
        end[1] = end[1] + right[1] * r;
        end[2] = end[2] + right[2] * r;
        end[0] = end[0] + up[0] * u;
        end[1] = end[1] + up[1] * u;
        end[2] = end[2] + up[2] * u;
        CG_ShotgunPellet(origin, end.as_mut_ptr(), otherEntNum);
        i += 1
    }
}
/*
==============
CG_ShotgunFire
==============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ShotgunFire(mut es: *mut crate::src::qcommon::q_shared::entityState_t) {
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut contents: i32 = 0;
    v[0] = (*es).origin2[0] - (*es).pos.trBase[0];
    v[1] = (*es).origin2[1] - (*es).pos.trBase[1];
    v[2] = (*es).origin2[2] - (*es).pos.trBase[2];
    crate::src::qcommon::q_math::VectorNormalize(v.as_mut_ptr());
    v[0] = v[0] * 32f32;
    v[1] = v[1] * 32f32;
    v[2] = v[2] * 32f32;
    v[0] = (*es).pos.trBase[0] + v[0];
    v[1] = (*es).pos.trBase[1] + v[1];
    v[2] = (*es).pos.trBase[2] + v[2];
    if crate::src::cgame::cg_main::cgs.glconfig.hardwareType != crate::tr_types_h::GLHW_RAGEPRO {
        // ragepro can't alpha fade, so don't even bother with smoke
        let mut up: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        contents = crate::src::cgame::cg_predict::CG_PointContents(
            (*es).pos.trBase.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            0,
        );
        if contents & 32 == 0 {
            up[0] = 0f32;
            up[1] = 0f32;
            up[2] = 8f32;
            crate::src::cgame::cg_effects::CG_SmokePuff(
                v.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                32f32,
                1f32,
                1f32,
                1f32,
                0.33f32,
                900f32,
                crate::src::cgame::cg_main::cg.time,
                0i32,
                crate::cg_local_h::LEF_PUFF_DONT_SCALE as i32,
                crate::src::cgame::cg_main::cgs.media.shotgunSmokePuffShader,
            );
        }
    }
    CG_ShotgunPattern(
        (*es).pos.trBase.as_mut_ptr(),
        (*es).origin2.as_mut_ptr(),
        (*es).eventParm,
        (*es).otherEntityNum,
    );
}
/*
============================================================================

BULLETS

============================================================================
*/
/*
===============
CG_Tracer
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Tracer(
    mut source: *mut crate::src::qcommon::q_shared::vec_t,
    mut dest: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut verts: [crate::tr_types_h::polyVert_t; 4] = [crate::tr_types_h::polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    let mut line: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut len: f32 = 0.;
    let mut begin: f32 = 0.;
    let mut end: f32 = 0.;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut finish: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut midpoint: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // tracer
    forward[0] = *dest.offset(0) - *source.offset(0);
    forward[1] = *dest.offset(1) - *source.offset(1);
    forward[2] = *dest.offset(2) - *source.offset(2);
    len = crate::src::qcommon::q_math::VectorNormalize(forward.as_mut_ptr());
    // start at least a little ways from the muzzle
    if len < 100f32 {
        return;
    }
    begin = 50f32 + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * (len - 60f32);
    end = begin + crate::src::cgame::cg_main::cg_tracerLength.value;
    if end > len {
        end = len
    }
    start[0] = *source.offset(0) + forward[0] * begin;
    start[1] = *source.offset(1) + forward[1] * begin;
    start[2] = *source.offset(2) + forward[2] * begin;
    finish[0] = *source.offset(0) + forward[0] * end;
    finish[1] = *source.offset(1) + forward[1] * end;
    finish[2] = *source.offset(2) + forward[2] * end;
    line[0] = forward[0] * crate::src::cgame::cg_main::cg.refdef.viewaxis[1][0]
        + forward[1] * crate::src::cgame::cg_main::cg.refdef.viewaxis[1][1]
        + forward[2] * crate::src::cgame::cg_main::cg.refdef.viewaxis[1][2];
    line[1] = forward[0] * crate::src::cgame::cg_main::cg.refdef.viewaxis[2][0]
        + forward[1] * crate::src::cgame::cg_main::cg.refdef.viewaxis[2][1]
        + forward[2] * crate::src::cgame::cg_main::cg.refdef.viewaxis[2][2];
    right[0] = crate::src::cgame::cg_main::cg.refdef.viewaxis[1][0] * line[1];
    right[1] = crate::src::cgame::cg_main::cg.refdef.viewaxis[1][1] * line[1];
    right[2] = crate::src::cgame::cg_main::cg.refdef.viewaxis[1][2] * line[1];
    right[0] = right[0] + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][0] * -line[0];
    right[1] = right[1] + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][1] * -line[0];
    right[2] = right[2] + crate::src::cgame::cg_main::cg.refdef.viewaxis[2][2] * -line[0];
    crate::src::qcommon::q_math::VectorNormalize(right.as_mut_ptr());
    verts[0].xyz[0] = finish[0] + right[0] * crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[0].xyz[1] = finish[1] + right[1] * crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[0].xyz[2] = finish[2] + right[2] * crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[0].st[0] = 0f32;
    verts[0].st[1] = 1f32;
    verts[0].modulate[0] = 255;
    verts[0].modulate[1] = 255;
    verts[0].modulate[2] = 255;
    verts[0].modulate[3] = 255;
    verts[1].xyz[0] = finish[0] + right[0] * -crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[1].xyz[1] = finish[1] + right[1] * -crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[1].xyz[2] = finish[2] + right[2] * -crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[1].st[0] = 1f32;
    verts[1].st[1] = 0f32;
    verts[1].modulate[0] = 255;
    verts[1].modulate[1] = 255;
    verts[1].modulate[2] = 255;
    verts[1].modulate[3] = 255;
    verts[2].xyz[0] = start[0] + right[0] * -crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[2].xyz[1] = start[1] + right[1] * -crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[2].xyz[2] = start[2] + right[2] * -crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[2].st[0] = 1f32;
    verts[2].st[1] = 1f32;
    verts[2].modulate[0] = 255;
    verts[2].modulate[1] = 255;
    verts[2].modulate[2] = 255;
    verts[2].modulate[3] = 255;
    verts[3].xyz[0] = start[0] + right[0] * crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[3].xyz[1] = start[1] + right[1] * crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[3].xyz[2] = start[2] + right[2] * crate::src::cgame::cg_main::cg_tracerWidth.value;
    verts[3].st[0] = 0f32;
    verts[3].st[1] = 0f32;
    verts[3].modulate[0] = 255;
    verts[3].modulate[1] = 255;
    verts[3].modulate[2] = 255;
    verts[3].modulate[3] = 255;
    crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene(
        crate::src::cgame::cg_main::cgs.media.tracerShader,
        4,
        verts.as_mut_ptr(),
    );
    midpoint[0] = ((start[0] + finish[0]) as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    midpoint[1] = ((start[1] + finish[1]) as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    midpoint[2] = ((start[2] + finish[2]) as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    // add the tracer sound
    crate::src::cgame::cg_syscalls::trap_S_StartSound(
        midpoint.as_mut_ptr(),
        ((1) << 10) - 2,
        crate::src::qcommon::q_shared::CHAN_AUTO as i32,
        crate::src::cgame::cg_main::cgs.media.tracerSound,
    );
}
/*
======================
CG_CalcMuzzlePoint
======================
*/

unsafe extern "C" fn CG_CalcMuzzlePoint(
    mut entityNum: i32,
    mut muzzle: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut anim: i32 = 0;
    if entityNum == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum {
        *muzzle.offset(0) = (*crate::src::cgame::cg_main::cg.snap).ps.origin[0];
        *muzzle.offset(1) = (*crate::src::cgame::cg_main::cg.snap).ps.origin[1];
        *muzzle.offset(2) = (*crate::src::cgame::cg_main::cg.snap).ps.origin[2];
        let ref mut fresh9 = *muzzle.offset(2);
        *fresh9 += (*crate::src::cgame::cg_main::cg.snap).ps.viewheight as f32;
        crate::src::qcommon::q_math::AngleVectors(
            (*crate::src::cgame::cg_main::cg.snap)
                .ps
                .viewangles
                .as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            forward.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
        *muzzle.offset(0) = *muzzle.offset(0) + forward[0] * 14f32;
        *muzzle.offset(1) = *muzzle.offset(1) + forward[1] * 14f32;
        *muzzle.offset(2) = *muzzle.offset(2) + forward[2] * 14f32;
        return crate::src::qcommon::q_shared::qtrue;
    }
    cent = &mut *crate::src::cgame::cg_main::cg_entities
        .as_mut_ptr()
        .offset(entityNum as isize) as *mut crate::cg_local_h::centity_t;
    if (*cent).currentValid as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    *muzzle.offset(0) = (*cent).currentState.pos.trBase[0];
    *muzzle.offset(1) = (*cent).currentState.pos.trBase[1];
    *muzzle.offset(2) = (*cent).currentState.pos.trBase[2];
    crate::src::qcommon::q_math::AngleVectors(
        (*cent).currentState.apos.trBase.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    anim = (*cent).currentState.legsAnim & !(128);
    if anim == crate::bg_public_h::LEGS_WALKCR as i32
        || anim == crate::bg_public_h::LEGS_IDLECR as i32
    {
        let ref mut fresh10 = *muzzle.offset(2);
        *fresh10 += 12f32
    } else {
        let ref mut fresh11 = *muzzle.offset(2);
        *fresh11 += 26f32
    }
    *muzzle.offset(0) = *muzzle.offset(0) + forward[0] * 14f32;
    *muzzle.offset(1) = *muzzle.offset(1) + forward[1] * 14f32;
    *muzzle.offset(2) = *muzzle.offset(2) + forward[2] * 14f32;
    return crate::src::qcommon::q_shared::qtrue;
}
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
//
// cg_events.c
//
//
// cg_ents.c
//
//
// cg_weapons.c
//
/*
======================
CG_Bullet

Renders bullet effects.
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Bullet(
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut sourceEntityNum: i32,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut flesh: crate::src::qcommon::q_shared::qboolean,
    mut fleshEntityNum: i32,
) {
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
    let mut sourceContentType: i32 = 0;
    let mut destContentType: i32 = 0;
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    // if the shooter is currently valid, calc a source point and possibly
    // do trail effects
    if sourceEntityNum >= 0 && crate::src::cgame::cg_main::cg_tracerChance.value > 0f32 {
        if CG_CalcMuzzlePoint(sourceEntityNum, start.as_mut_ptr()) as u64 != 0 {
            sourceContentType = crate::src::cgame::cg_predict::CG_PointContents(
                start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                0,
            );
            destContentType = crate::src::cgame::cg_predict::CG_PointContents(
                end as *const crate::src::qcommon::q_shared::vec_t,
                0,
            );
            // do a complete bubble trail if necessary
            if sourceContentType == destContentType && sourceContentType & 32 != 0 {
                crate::src::cgame::cg_effects::CG_BubbleTrail(start.as_mut_ptr(), end, 32f32);
            } else if sourceContentType & 32 != 0 {
                crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
                    &mut trace,
                    end as *const crate::src::qcommon::q_shared::vec_t,
                    start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    0 as *const crate::src::qcommon::q_shared::vec_t,
                    0 as *const crate::src::qcommon::q_shared::vec_t,
                    0,
                    32,
                );
                crate::src::cgame::cg_effects::CG_BubbleTrail(
                    start.as_mut_ptr(),
                    trace.endpos.as_mut_ptr(),
                    32f32,
                );
            } else if destContentType & 32 != 0 {
                crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
                    &mut trace,
                    start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    end as *const crate::src::qcommon::q_shared::vec_t,
                    0 as *const crate::src::qcommon::q_shared::vec_t,
                    0 as *const crate::src::qcommon::q_shared::vec_t,
                    0,
                    32,
                );
                crate::src::cgame::cg_effects::CG_BubbleTrail(
                    trace.endpos.as_mut_ptr(),
                    end,
                    32f32,
                );
            }
            // bubble trail from water into air
            // bubble trail from air into water
            // draw a tracer
            if ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32)
                < crate::src::cgame::cg_main::cg_tracerChance.value
            {
                CG_Tracer(start.as_mut_ptr(), end);
            }
        }
    }
    // impact splash and mark
    if flesh as u64 != 0 {
        crate::src::cgame::cg_effects::CG_Bleed(end, fleshEntityNum);
    } else {
        CG_MissileHitWall(
            crate::bg_public_h::WP_MACHINEGUN as i32,
            0i32,
            end,
            normal,
            crate::cg_local_h::IMPACTSOUND_DEFAULT,
        );
    };
}
