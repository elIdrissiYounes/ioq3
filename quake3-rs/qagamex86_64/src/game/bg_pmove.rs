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

    // __Q_SHARED_H
}

pub use crate::bg_local_h::pml_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::pmove_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
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
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
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
pub use crate::bg_public_h::WEAPON_DROPPING;
pub use crate::bg_public_h::WEAPON_FIRING;
pub use crate::bg_public_h::WEAPON_RAISING;
pub use crate::bg_public_h::WEAPON_READY;
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
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::BG_AddPredictableEventToPlayerstate;
pub use crate::src::game::bg_pmove::q_shared_h::VectorLength;
pub use crate::src::game::bg_slidemove::PM_SlideMove;
pub use crate::src::game::bg_slidemove::PM_StepSlideMove;
pub use crate::src::game::g_main::Com_Printf;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;

extern "C" {
    /*
    ================
    PmoveSingle

    ================
    */
    #[no_mangle]
    pub fn trap_SnapVector(v: *mut f32);
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
// bg_pmove.c -- both games player movement code
// takes a playerstate and a usercmd as input and returns a modifed playerstate
#[no_mangle]

pub static mut pm: *mut crate::bg_public_h::pmove_t = 0 as *mut crate::bg_public_h::pmove_t;
#[no_mangle]

pub static mut pml: crate::bg_local_h::pml_t = crate::bg_local_h::pml_t {
    forward: [0.; 3],
    right: [0.; 3],
    up: [0.; 3],
    frametime: 0.,
    msec: 0,
    walking: crate::src::qcommon::q_shared::qfalse,
    groundPlane: crate::src::qcommon::q_shared::qfalse,
    groundTrace: crate::src::qcommon::q_shared::trace_t {
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
    },
    impactSpeed: 0.,
    previous_origin: [0.; 3],
    previous_velocity: [0.; 3],
    previous_waterlevel: 0,
};
// movement parameters
#[no_mangle]

pub static mut pm_stopspeed: f32 = 100.0;
#[no_mangle]

pub static mut pm_duckScale: f32 = 0.25;
#[no_mangle]

pub static mut pm_swimScale: f32 = 0.50;
#[no_mangle]

pub static mut pm_accelerate: f32 = 10.0;
#[no_mangle]

pub static mut pm_airaccelerate: f32 = 1.0;
#[no_mangle]

pub static mut pm_wateraccelerate: f32 = 4.0;
#[no_mangle]

pub static mut pm_flyaccelerate: f32 = 8.0;
#[no_mangle]

pub static mut pm_friction: f32 = 6.0;
#[no_mangle]

pub static mut pm_waterfriction: f32 = 1.0;
#[no_mangle]

pub static mut pm_flightfriction: f32 = 3.0;
#[no_mangle]

pub static mut pm_spectatorfriction: f32 = 5.0;
#[no_mangle]

pub static mut c_pmove: i32 = 0;
/*
===============
PM_AddEvent

===============
*/
#[no_mangle]

pub unsafe extern "C" fn PM_AddEvent(mut newEvent: i32) {
    crate::src::game::bg_misc::BG_AddPredictableEventToPlayerstate(newEvent, 0, (*pm).ps);
}
/*
===============
PM_AddTouchEnt
===============
*/
#[no_mangle]

pub unsafe extern "C" fn PM_AddTouchEnt(mut entityNum: i32) {
    let mut _i: i32 = 0;
    if entityNum == ((1) << 10) - 2 {
        return;
    }
    if (*pm).numtouch == 32 {
        return;
    }

    for i in 0..(*pm).numtouch {
        if (*pm).touchents[i as usize] == entityNum {
            return;
        }
    }
    // add it
    (*pm).touchents[(*pm).numtouch as usize] = entityNum;
    (*pm).numtouch += 1;
}
/*
===================
PM_StartTorsoAnim
===================
*/

unsafe extern "C" fn PM_StartTorsoAnim(mut anim: i32) {
    if (*(*pm).ps).pm_type >= crate::bg_public_h::PM_DEAD as i32 {
        return;
    }
    (*(*pm).ps).torsoAnim = (*(*pm).ps).torsoAnim & 128 ^ 128 | anim;
}

unsafe extern "C" fn PM_StartLegsAnim(mut anim: i32) {
    if (*(*pm).ps).pm_type >= crate::bg_public_h::PM_DEAD as i32 {
        return;
    }
    if (*(*pm).ps).legsTimer > 0 {
        return;
        // a high priority animation is running
    }
    (*(*pm).ps).legsAnim = (*(*pm).ps).legsAnim & 128 ^ 128 | anim;
}

unsafe extern "C" fn PM_ContinueLegsAnim(mut anim: i32) {
    if (*(*pm).ps).legsAnim & !(128) == anim {
        return;
    }
    if (*(*pm).ps).legsTimer > 0 {
        return;
        // a high priority animation is running
    }
    PM_StartLegsAnim(anim);
}

unsafe extern "C" fn PM_ContinueTorsoAnim(mut anim: i32) {
    if (*(*pm).ps).torsoAnim & !(128) == anim {
        return;
    }
    if (*(*pm).ps).torsoTimer > 0 {
        return;
        // a high priority animation is running
    }
    PM_StartTorsoAnim(anim);
}

unsafe extern "C" fn PM_ForceLegsAnim(mut anim: i32) {
    (*(*pm).ps).legsTimer = 0;
    PM_StartLegsAnim(anim);
}
// movement parameters
/*
==================
PM_ClipVelocity

Slide off of the impacting surface
==================
*/
#[no_mangle]

pub unsafe extern "C" fn PM_ClipVelocity(
    mut in_0: *mut crate::src::qcommon::q_shared::vec_t,
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
    mut overbounce: f32,
) {
    let mut backoff: f32 = 0.;
    let mut change: f32 = 0.;
    let mut i: i32 = 0;
    backoff = *in_0.offset(0) * *normal.offset(0)
        + *in_0.offset(1) * *normal.offset(1)
        + *in_0.offset(2) * *normal.offset(2);
    if backoff < 0f32 {
        backoff *= overbounce
    } else {
        backoff /= overbounce
    }
    i = 0;
    while i < 3 {
        change = *normal.offset(i as isize) * backoff;
        *out.offset(i as isize) = *in_0.offset(i as isize) - change;
        i += 1
    }
}
/*
==================
PM_Friction

Handles both ground friction and water friction
==================
*/

unsafe extern "C" fn PM_Friction() {
    let mut vec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vel: *mut f32 = 0 as *mut f32;
    let mut speed: f32 = 0.;
    let mut newspeed: f32 = 0.;
    let mut control: f32 = 0.;
    let mut drop_0: f32 = 0.;
    vel = (*(*pm).ps).velocity.as_mut_ptr();
    vec[0] = *vel.offset(0);
    vec[1] = *vel.offset(1);
    vec[2] = *vel.offset(2);
    if pml.walking as u64 != 0 {
        vec[2] = 0f32
        // ignore slope movement
    } // allow sinking underwater
    speed = VectorLength(vec.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    if speed < 1f32 {
        *vel.offset(0) = 0f32;
        *vel.offset(1) = 0f32;
        // FIXME: still have z friction underwater?
        return;
    }
    drop_0 = 0f32;
    // apply ground friction
    if (*pm).waterlevel <= 1 {
        if pml.walking != 0 && pml.groundTrace.surfaceFlags & 0x2 == 0 {
            // if getting knocked back, no friction
            if (*(*pm).ps).pm_flags & 64 == 0 {
                control = if speed < pm_stopspeed {
                    pm_stopspeed
                } else {
                    speed
                };
                drop_0 += control * pm_friction * pml.frametime
            }
        }
    }
    // apply water friction even if just wading
    if (*pm).waterlevel != 0 {
        drop_0 += speed * pm_waterfriction * (*pm).waterlevel as f32 * pml.frametime
    }
    // apply flying friction
    if (*(*pm).ps).powerups[crate::bg_public_h::PW_FLIGHT as usize] != 0 {
        drop_0 += speed * pm_flightfriction * pml.frametime
    }
    if (*(*pm).ps).pm_type == crate::bg_public_h::PM_SPECTATOR as i32 {
        drop_0 += speed * pm_spectatorfriction * pml.frametime
    }
    // scale the velocity
    newspeed = speed - drop_0;
    if newspeed < 0f32 {
        newspeed = 0f32
    }
    newspeed /= speed;
    *vel.offset(0) = *vel.offset(0) * newspeed;
    *vel.offset(1) = *vel.offset(1) * newspeed;
    *vel.offset(2) = *vel.offset(2) * newspeed;
}
/*
==============
PM_Accelerate

Handles user intended acceleration
==============
*/

unsafe extern "C" fn PM_Accelerate(
    mut wishdir: *mut crate::src::qcommon::q_shared::vec_t,
    mut wishspeed: f32,
    mut accel: f32,
) {
    // q2 style
    let mut i: i32 = 0;
    let mut addspeed: f32 = 0.;
    let mut accelspeed: f32 = 0.;
    let mut currentspeed: f32 = 0.;
    currentspeed = (*(*pm).ps).velocity[0] * *wishdir.offset(0)
        + (*(*pm).ps).velocity[1] * *wishdir.offset(1)
        + (*(*pm).ps).velocity[2] * *wishdir.offset(2);
    addspeed = wishspeed - currentspeed;
    if addspeed <= 0f32 {
        return;
    }
    accelspeed = accel * pml.frametime * wishspeed;
    if accelspeed > addspeed {
        accelspeed = addspeed
    }
    i = 0;
    while i < 3 {
        (*(*pm).ps).velocity[i as usize] += accelspeed * *wishdir.offset(i as isize);
        i += 1
    }
}
/*
============
PM_CmdScale

Returns the scale factor to apply to cmd movements
This allows the clients to use axial -127 to 127 values for all directions
without getting a sqrt(2) distortion in speed.
============
*/

unsafe extern "C" fn PM_CmdScale(mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t) -> f32 {
    let mut max: i32 = 0;
    let mut total: f32 = 0.;
    let mut scale: f32 = 0.;
    max = crate::stdlib::abs((*cmd).forwardmove as i32);
    if crate::stdlib::abs((*cmd).rightmove as i32) > max {
        max = crate::stdlib::abs((*cmd).rightmove as i32)
    }
    if crate::stdlib::abs((*cmd).upmove as i32) > max {
        max = crate::stdlib::abs((*cmd).upmove as i32)
    }
    if max == 0 {
        return 0f32;
    }
    total = crate::stdlib::sqrt(
        ((*cmd).forwardmove as i32 * (*cmd).forwardmove as i32
            + (*cmd).rightmove as i32 * (*cmd).rightmove as i32
            + (*cmd).upmove as i32 * (*cmd).upmove as i32) as f64,
    ) as f32;
    scale = (((*(*pm).ps).speed as f32 * max as f32) as f64 / (127.0 * total as f64)) as f32;
    return scale;
}
/*
================
PM_SetMovementDir

Determine the rotation of the legs relative
to the facing dir
================
*/

unsafe extern "C" fn PM_SetMovementDir() {
    if (*pm).cmd.forwardmove as i32 != 0 || (*pm).cmd.rightmove as i32 != 0 {
        if (*pm).cmd.rightmove as i32 == 0 && (*pm).cmd.forwardmove as i32 > 0 {
            (*(*pm).ps).movementDir = 0
        } else if ((*pm).cmd.rightmove as i32) < 0 && (*pm).cmd.forwardmove as i32 > 0 {
            (*(*pm).ps).movementDir = 1
        } else if ((*pm).cmd.rightmove as i32) < 0 && (*pm).cmd.forwardmove as i32 == 0 {
            (*(*pm).ps).movementDir = 2
        } else if ((*pm).cmd.rightmove as i32) < 0 && ((*pm).cmd.forwardmove as i32) < 0 {
            (*(*pm).ps).movementDir = 3
        } else if (*pm).cmd.rightmove as i32 == 0 && ((*pm).cmd.forwardmove as i32) < 0 {
            (*(*pm).ps).movementDir = 4
        } else if (*pm).cmd.rightmove as i32 > 0 && ((*pm).cmd.forwardmove as i32) < 0 {
            (*(*pm).ps).movementDir = 5
        } else if (*pm).cmd.rightmove as i32 > 0 && (*pm).cmd.forwardmove as i32 == 0 {
            (*(*pm).ps).movementDir = 6
        } else if (*pm).cmd.rightmove as i32 > 0 && (*pm).cmd.forwardmove as i32 > 0 {
            (*(*pm).ps).movementDir = 7
        }
    } else if (*(*pm).ps).movementDir == 2 {
        (*(*pm).ps).movementDir = 1
    } else if (*(*pm).ps).movementDir == 6 {
        (*(*pm).ps).movementDir = 7
    };
}
// if they aren't actively going directly sideways,
// change the animation to the diagonal so they
// don't stop too crooked
/*
=============
PM_CheckJump
=============
*/

unsafe extern "C" fn PM_CheckJump() -> crate::src::qcommon::q_shared::qboolean {
    if (*(*pm).ps).pm_flags & 512 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
        // don't allow jump until all buttons are up
    }
    if ((*pm).cmd.upmove as i32) < 10 {
        // not holding jump
        return crate::src::qcommon::q_shared::qfalse;
    }
    // must wait for jump to be released
    if (*(*pm).ps).pm_flags & 2 != 0 {
        // clear upmove so cmdscale doesn't lower running speed
        (*pm).cmd.upmove = 0; // jumping away
        return crate::src::qcommon::q_shared::qfalse;
    }
    pml.groundPlane = crate::src::qcommon::q_shared::qfalse;
    pml.walking = crate::src::qcommon::q_shared::qfalse;
    (*(*pm).ps).pm_flags |= 2;
    (*(*pm).ps).groundEntityNum = ((1) << 10) - 1;
    (*(*pm).ps).velocity[2] = 270f32;
    PM_AddEvent(crate::bg_public_h::EV_JUMP as i32);
    if (*pm).cmd.forwardmove as i32 >= 0 {
        PM_ForceLegsAnim(crate::bg_public_h::LEGS_JUMP as i32);
        (*(*pm).ps).pm_flags &= !(8)
    } else {
        PM_ForceLegsAnim(crate::bg_public_h::LEGS_JUMPB as i32);
        (*(*pm).ps).pm_flags |= 8
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
=============
PM_CheckWaterJump
=============
*/

unsafe extern "C" fn PM_CheckWaterJump() -> crate::src::qcommon::q_shared::qboolean {
    let mut spot: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cont: i32 = 0;
    let mut flatforward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*(*pm).ps).pm_time != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // check for water jump
    if (*pm).waterlevel != 2 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    flatforward[0] = pml.forward[0];
    flatforward[1] = pml.forward[1];
    flatforward[2] = 0f32;
    crate::src::qcommon::q_math::VectorNormalize(flatforward.as_mut_ptr());
    spot[0] = (*(*pm).ps).origin[0] + flatforward[0] * 30f32;
    spot[1] = (*(*pm).ps).origin[1] + flatforward[1] * 30f32;
    spot[2] = (*(*pm).ps).origin[2] + flatforward[2] * 30f32;
    spot[2] += 4f32;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        spot.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*(*pm).ps).clientNum,
    );
    if cont & 1 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    spot[2] += 16f32;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        spot.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*(*pm).ps).clientNum,
    );
    if cont & (1 | 0x10000 | 0x2000000) != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // jump out of water
    (*(*pm).ps).velocity[0] = pml.forward[0] * 200f32;
    (*(*pm).ps).velocity[1] = pml.forward[1] * 200f32;
    (*(*pm).ps).velocity[2] = pml.forward[2] * 200f32;
    (*(*pm).ps).velocity[2] = 350f32;
    (*(*pm).ps).pm_flags |= 256;
    (*(*pm).ps).pm_time = 2000;
    return crate::src::qcommon::q_shared::qtrue;
}
//============================================================================
/*
===================
PM_WaterJumpMove

Flying out of the water
===================
*/

unsafe extern "C" fn PM_WaterJumpMove() {
    // waterjump has no control, but falls
    crate::src::game::bg_slidemove::PM_StepSlideMove(crate::src::qcommon::q_shared::qtrue);
    (*(*pm).ps).velocity[2] -= (*(*pm).ps).gravity as f32 * pml.frametime;
    if (*(*pm).ps).velocity[2] < 0f32 {
        // cancel as soon as we are falling down again
        (*(*pm).ps).pm_flags &= !(256 | 32 | 64);
        (*(*pm).ps).pm_time = 0
    };
}
/*
===================
PM_WaterMove

===================
*/

unsafe extern "C" fn PM_WaterMove() {
    let mut _i: i32 = 0;
    let mut wishvel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut wishdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut scale: f32 = 0.;
    let mut vel: f32 = 0.;
    if PM_CheckWaterJump() as u64 != 0 {
        PM_WaterJumpMove();
        return;
    }
    PM_Friction();
    scale = PM_CmdScale(&mut (*pm).cmd);
    //
    // user intentions
    //
    if scale == 0. {
        wishvel[0] = 0f32;
        wishvel[1] = 0f32;
        wishvel[2] = -60f32
    // sink towards bottom
    } else {
        for i in 0..3 {
            wishvel[i as usize] =
                scale * pml.forward[i as usize] * (*pm).cmd.forwardmove as i32 as f32
                    + scale * pml.right[i as usize] * (*pm).cmd.rightmove as i32 as f32;
        }
        wishvel[2] += scale * (*pm).cmd.upmove as i32 as f32
    }
    wishdir[0] = wishvel[0];
    wishdir[1] = wishvel[1];
    wishdir[2] = wishvel[2];
    wishspeed = crate::src::qcommon::q_math::VectorNormalize(wishdir.as_mut_ptr());
    if wishspeed > (*(*pm).ps).speed as f32 * pm_swimScale {
        wishspeed = (*(*pm).ps).speed as f32 * pm_swimScale
    }
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_wateraccelerate);
    // make sure we can go up slopes easily under water
    if pml.groundPlane != 0
        && (*(*pm).ps).velocity[0] * pml.groundTrace.plane.normal[0]
            + (*(*pm).ps).velocity[1] * pml.groundTrace.plane.normal[1]
            + (*(*pm).ps).velocity[2] * pml.groundTrace.plane.normal[2]
            < 0f32
    {
        vel = VectorLength(
            (*(*pm).ps).velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
        );
        // slide along the ground plane
        PM_ClipVelocity(
            (*(*pm).ps).velocity.as_mut_ptr(),
            pml.groundTrace.plane.normal.as_mut_ptr(),
            (*(*pm).ps).velocity.as_mut_ptr(),
            1.001,
        );
        crate::src::qcommon::q_math::VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
        (*(*pm).ps).velocity[0] = (*(*pm).ps).velocity[0] * vel;
        (*(*pm).ps).velocity[1] = (*(*pm).ps).velocity[1] * vel;
        (*(*pm).ps).velocity[2] = (*(*pm).ps).velocity[2] * vel
    }
    crate::src::game::bg_slidemove::PM_SlideMove(crate::src::qcommon::q_shared::qfalse);
}
/*
===================
PM_FlyMove

Only with the flight powerup
===================
*/

unsafe extern "C" fn PM_FlyMove() {
    let mut _i: i32 = 0;
    let mut wishvel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut wishdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut scale: f32 = 0.;
    // normal slowdown
    PM_Friction();
    scale = PM_CmdScale(&mut (*pm).cmd);
    //
    // user intentions
    //
    if scale == 0. {
        wishvel[0] = 0f32;
        wishvel[1] = 0f32;
        wishvel[2] = 0f32
    } else {
        for i in 0..3 {
            wishvel[i as usize] =
                scale * pml.forward[i as usize] * (*pm).cmd.forwardmove as i32 as f32
                    + scale * pml.right[i as usize] * (*pm).cmd.rightmove as i32 as f32;
        }
        wishvel[2] += scale * (*pm).cmd.upmove as i32 as f32
    }
    wishdir[0] = wishvel[0];
    wishdir[1] = wishvel[1];
    wishdir[2] = wishvel[2];
    wishspeed = crate::src::qcommon::q_math::VectorNormalize(wishdir.as_mut_ptr());
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_flyaccelerate);
    crate::src::game::bg_slidemove::PM_StepSlideMove(crate::src::qcommon::q_shared::qfalse);
}
/*
===================
PM_AirMove

===================
*/

unsafe extern "C" fn PM_AirMove() {
    let mut _i: i32 = 0;
    let mut wishvel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut fmove: f32 = 0.;
    let mut smove: f32 = 0.;
    let mut wishdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut cmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    PM_Friction();
    fmove = (*pm).cmd.forwardmove as f32;
    smove = (*pm).cmd.rightmove as f32;
    cmd = (*pm).cmd;
    scale = PM_CmdScale(&mut cmd);
    // set the movementDir so clients can rotate the legs for strafing
    PM_SetMovementDir();
    // project moves down to flat plane
    pml.forward[2] = 0f32;
    pml.right[2] = 0f32;
    crate::src::qcommon::q_math::VectorNormalize(pml.forward.as_mut_ptr());
    crate::src::qcommon::q_math::VectorNormalize(pml.right.as_mut_ptr());

    for i in 0..2 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
    }
    wishvel[2] = 0f32;
    wishdir[0] = wishvel[0];
    wishdir[1] = wishvel[1];
    wishdir[2] = wishvel[2];
    wishspeed = crate::src::qcommon::q_math::VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    // not on ground, so little effect on velocity
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_airaccelerate);
    // we may have a ground plane that is very steep, even
    // though we don't have a groundentity
    // slide along the steep plane
    if pml.groundPlane as u64 != 0 {
        PM_ClipVelocity(
            (*(*pm).ps).velocity.as_mut_ptr(),
            pml.groundTrace.plane.normal.as_mut_ptr(),
            (*(*pm).ps).velocity.as_mut_ptr(),
            1.001f32,
        );
    }
    crate::src::game::bg_slidemove::PM_StepSlideMove(crate::src::qcommon::q_shared::qtrue);
}
/*
===================
PM_GrappleMove

===================
*/

unsafe extern "C" fn PM_GrappleMove() {
    let mut vel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut v: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut vlen: f32 = 0.;
    v[0] = pml.forward[0] * -16f32;
    v[1] = pml.forward[1] * -16f32;
    v[2] = pml.forward[2] * -16f32;
    v[0] = (*(*pm).ps).grapplePoint[0] + v[0];
    v[1] = (*(*pm).ps).grapplePoint[1] + v[1];
    v[2] = (*(*pm).ps).grapplePoint[2] + v[2];
    vel[0] = v[0] - (*(*pm).ps).origin[0];
    vel[1] = v[1] - (*(*pm).ps).origin[1];
    vel[2] = v[2] - (*(*pm).ps).origin[2];
    vlen = VectorLength(vel.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
    crate::src::qcommon::q_math::VectorNormalize(vel.as_mut_ptr());
    if vlen <= 100f32 {
        vel[0] = vel[0] * (10f32 * vlen);
        vel[1] = vel[1] * (10f32 * vlen);
        vel[2] = vel[2] * (10f32 * vlen)
    } else {
        vel[0] = vel[0] * 800f32;
        vel[1] = vel[1] * 800f32;
        vel[2] = vel[2] * 800f32
    }
    (*(*pm).ps).velocity[0] = vel[0];
    (*(*pm).ps).velocity[1] = vel[1];
    (*(*pm).ps).velocity[2] = vel[2];
    pml.groundPlane = crate::src::qcommon::q_shared::qfalse;
}
/*
===================
PM_WalkMove

===================
*/

unsafe extern "C" fn PM_WalkMove() {
    let mut _i: i32 = 0;
    let mut wishvel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut fmove: f32 = 0.;
    let mut smove: f32 = 0.;
    let mut wishdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut cmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    let mut accelerate: f32 = 0.;
    let mut vel: f32 = 0.;
    if (*pm).waterlevel > 2
        && pml.forward[0] * pml.groundTrace.plane.normal[0]
            + pml.forward[1] * pml.groundTrace.plane.normal[1]
            + pml.forward[2] * pml.groundTrace.plane.normal[2]
            > 0f32
    {
        // begin swimming
        PM_WaterMove();
        return;
    }
    if PM_CheckJump() as u64 != 0 {
        // jumped away
        if (*pm).waterlevel > 1 {
            PM_WaterMove();
        } else {
            PM_AirMove();
        }
        return;
    }
    PM_Friction();
    fmove = (*pm).cmd.forwardmove as f32;
    smove = (*pm).cmd.rightmove as f32;
    cmd = (*pm).cmd;
    scale = PM_CmdScale(&mut cmd);
    // set the movementDir so clients can rotate the legs for strafing
    PM_SetMovementDir();
    // project moves down to flat plane
    pml.forward[2] = 0f32;
    pml.right[2] = 0f32;
    // project the forward and right directions onto the ground plane
    PM_ClipVelocity(
        pml.forward.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        pml.forward.as_mut_ptr(),
        1.001,
    );
    PM_ClipVelocity(
        pml.right.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        pml.right.as_mut_ptr(),
        1.001,
    );
    //
    crate::src::qcommon::q_math::VectorNormalize(pml.forward.as_mut_ptr());
    crate::src::qcommon::q_math::VectorNormalize(pml.right.as_mut_ptr());

    for i in 0..3 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
    }
    // when going up or down slopes the wish velocity should Not be zero
    //	wishvel[2] = 0;
    wishdir[0] = wishvel[0];
    wishdir[1] = wishvel[1];
    wishdir[2] = wishvel[2];
    wishspeed = crate::src::qcommon::q_math::VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    // clamp the speed lower if ducking
    if (*(*pm).ps).pm_flags & 1 != 0 {
        if wishspeed > (*(*pm).ps).speed as f32 * pm_duckScale {
            wishspeed = (*(*pm).ps).speed as f32 * pm_duckScale
        }
    }
    // clamp the speed lower if wading or walking on the bottom
    if (*pm).waterlevel != 0 {
        let mut waterScale: f32 = 0.;
        waterScale = ((*pm).waterlevel as f64 / 3.0) as f32;
        waterScale = (1.0 - (1.0 - pm_swimScale as f64) * waterScale as f64) as f32;
        if wishspeed > (*(*pm).ps).speed as f32 * waterScale {
            wishspeed = (*(*pm).ps).speed as f32 * waterScale
        }
    }
    // when a player gets hit, they temporarily lose
    // full control, which allows them to be moved a bit
    if pml.groundTrace.surfaceFlags & 0x2 != 0 || (*(*pm).ps).pm_flags & 64 != 0 {
        accelerate = pm_airaccelerate
    } else {
        accelerate = pm_accelerate
    }
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, accelerate);
    //Com_Printf("velocity = %1.1f %1.1f %1.1f\n", pm->ps->velocity[0], pm->ps->velocity[1], pm->ps->velocity[2]);
    //Com_Printf("velocity1 = %1.1f\n", VectorLength(pm->ps->velocity));
    if pml.groundTrace.surfaceFlags & 0x2 != 0 || (*(*pm).ps).pm_flags & 64 != 0 {
        (*(*pm).ps).velocity[2] -= (*(*pm).ps).gravity as f32 * pml.frametime
    }
    vel = VectorLength(
        (*(*pm).ps).velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
    );
    // slide along the ground plane
    PM_ClipVelocity(
        (*(*pm).ps).velocity.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        (*(*pm).ps).velocity.as_mut_ptr(),
        1.001,
    );
    // don't decrease velocity when going up or down a slope
    crate::src::qcommon::q_math::VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
    (*(*pm).ps).velocity[0] = (*(*pm).ps).velocity[0] * vel;
    (*(*pm).ps).velocity[1] = (*(*pm).ps).velocity[1] * vel;
    (*(*pm).ps).velocity[2] = (*(*pm).ps).velocity[2] * vel;
    // don't do anything if standing still
    if (*(*pm).ps).velocity[0] == 0. && (*(*pm).ps).velocity[1] == 0. {
        return;
    }
    crate::src::game::bg_slidemove::PM_StepSlideMove(crate::src::qcommon::q_shared::qfalse);
    //Com_Printf("velocity2 = %1.1f\n", VectorLength(pm->ps->velocity));
}
/*
==============
PM_DeadMove
==============
*/

unsafe extern "C" fn PM_DeadMove() {
    let mut forward: f32 = 0.;
    if pml.walking as u64 == 0 {
        return;
    }
    // extra friction
    forward = VectorLength(
        (*(*pm).ps).velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
    );
    forward -= 20f32;
    if forward <= 0f32 {
        (*(*pm).ps).velocity[2] = 0f32;
        (*(*pm).ps).velocity[1] = (*(*pm).ps).velocity[2];
        (*(*pm).ps).velocity[0] = (*(*pm).ps).velocity[1]
    } else {
        crate::src::qcommon::q_math::VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
        (*(*pm).ps).velocity[0] = (*(*pm).ps).velocity[0] * forward;
        (*(*pm).ps).velocity[1] = (*(*pm).ps).velocity[1] * forward;
        (*(*pm).ps).velocity[2] = (*(*pm).ps).velocity[2] * forward
    };
}
/*
===============
PM_NoclipMove
===============
*/

unsafe extern "C" fn PM_NoclipMove() {
    let mut speed: f32 = 0.;
    let mut drop_0: f32 = 0.;
    let mut friction: f32 = 0.;
    let mut control: f32 = 0.;
    let mut newspeed: f32 = 0.;
    let mut _i: i32 = 0;
    let mut wishvel: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut fmove: f32 = 0.;
    let mut smove: f32 = 0.;
    let mut wishdir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut wishspeed: f32 = 0.;
    let mut scale: f32 = 0.;
    (*(*pm).ps).viewheight = 26;
    // friction
    speed = VectorLength(
        (*(*pm).ps).velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
    ); // extra friction
    if speed < 1f32 {
        (*(*pm).ps).velocity[0] = crate::src::qcommon::q_math::vec3_origin[0];
        (*(*pm).ps).velocity[1] = crate::src::qcommon::q_math::vec3_origin[1];
        (*(*pm).ps).velocity[2] = crate::src::qcommon::q_math::vec3_origin[2]
    } else {
        drop_0 = 0f32;
        friction = (pm_friction as f64 * 1.5) as f32;
        control = if speed < pm_stopspeed {
            pm_stopspeed
        } else {
            speed
        };
        drop_0 += control * friction * pml.frametime;
        // scale the velocity
        newspeed = speed - drop_0;
        if newspeed < 0f32 {
            newspeed = 0f32
        }
        newspeed /= speed;
        (*(*pm).ps).velocity[0] = (*(*pm).ps).velocity[0] * newspeed;
        (*(*pm).ps).velocity[1] = (*(*pm).ps).velocity[1] * newspeed;
        (*(*pm).ps).velocity[2] = (*(*pm).ps).velocity[2] * newspeed
    }
    // accelerate
    scale = PM_CmdScale(&mut (*pm).cmd);
    fmove = (*pm).cmd.forwardmove as f32;
    smove = (*pm).cmd.rightmove as f32;

    for i in 0..3 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
    }
    wishvel[2] += (*pm).cmd.upmove as i32 as f32;
    wishdir[0] = wishvel[0];
    wishdir[1] = wishvel[1];
    wishdir[2] = wishvel[2];
    wishspeed = crate::src::qcommon::q_math::VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_accelerate);
    // move
    (*(*pm).ps).origin[0] = (*(*pm).ps).origin[0] + (*(*pm).ps).velocity[0] * pml.frametime;
    (*(*pm).ps).origin[1] = (*(*pm).ps).origin[1] + (*(*pm).ps).velocity[1] * pml.frametime;
    (*(*pm).ps).origin[2] = (*(*pm).ps).origin[2] + (*(*pm).ps).velocity[2] * pml.frametime;
}
//============================================================================
/*
================
PM_FootstepForSurface

Returns an event number appropriate for the groundsurface
================
*/

unsafe extern "C" fn PM_FootstepForSurface() -> i32 {
    if pml.groundTrace.surfaceFlags & 0x2000 != 0 {
        return 0i32;
    }
    if pml.groundTrace.surfaceFlags & 0x1000 != 0 {
        return crate::bg_public_h::EV_FOOTSTEP_METAL as i32;
    }
    return crate::bg_public_h::EV_FOOTSTEP as i32;
}
/*
=================
PM_CrashLand

Check for hard landings that generate sound events
=================
*/

unsafe extern "C" fn PM_CrashLand() {
    let mut delta: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut vel: f32 = 0.;
    let mut acc: f32 = 0.;
    let mut t: f32 = 0.;
    let mut a: f32 = 0.;
    let mut b: f32 = 0.;
    let mut c: f32 = 0.;
    let mut den: f32 = 0.;
    // decide which landing animation to use
    if (*(*pm).ps).pm_flags & 8 != 0 {
        PM_ForceLegsAnim(crate::bg_public_h::LEGS_LANDB as i32);
    } else {
        PM_ForceLegsAnim(crate::bg_public_h::LEGS_LAND as i32);
    }
    (*(*pm).ps).legsTimer = 130;
    // calculate the exact velocity on landing
    dist = (*(*pm).ps).origin[2] - pml.previous_origin[2];
    vel = pml.previous_velocity[2];
    acc = -(*(*pm).ps).gravity as f32;
    a = acc / 2f32;
    b = vel;
    c = -dist;
    den = b * b - 4f32 * a * c;
    if den < 0f32 {
        return;
    }
    t = ((-b as f64 - crate::stdlib::sqrt(den as f64)) / (2f32 * a) as f64) as f32;
    delta = vel + t * acc;
    delta = ((delta * delta) as f64 * 0.0001) as f32;
    // ducking while falling doubles damage
    if (*(*pm).ps).pm_flags & 1 != 0 {
        delta *= 2f32
    }
    // never take falling damage if completely underwater
    if (*pm).waterlevel == 3 {
        return;
    }
    // reduce falling damage if there is standing water
    if (*pm).waterlevel == 2 {
        delta = (delta as f64 * 0.25) as f32
    }
    if (*pm).waterlevel == 1 {
        delta = (delta as f64 * 0.5) as f32
    }
    if delta < 1f32 {
        return;
    }
    // create a local entity event to play the sound
    // SURF_NODAMAGE is used for bounce pads where you don't ever
    // want to take damage or play a crunch sound
    if pml.groundTrace.surfaceFlags & 0x1 == 0 {
        if delta > 60f32 {
            PM_AddEvent(crate::bg_public_h::EV_FALL_FAR as i32);
        } else if delta > 40f32 {
            // this is a pain grunt, so don't play it if dead
            if (*(*pm).ps).stats[crate::bg_public_h::STAT_HEALTH as usize] > 0 {
                PM_AddEvent(crate::bg_public_h::EV_FALL_MEDIUM as i32);
            }
        } else if delta > 7f32 {
            PM_AddEvent(crate::bg_public_h::EV_FALL_SHORT as i32);
        } else {
            PM_AddEvent(PM_FootstepForSurface());
        }
    }
    // start footstep cycle over
    (*(*pm).ps).bobCycle = 0;
}
/*
=============
PM_CheckStuck
=============
*/
/*
void PM_CheckStuck(void) {
    trace_t trace;

    pm->trace (&trace, pm->ps->origin, pm->mins, pm->maxs, pm->ps->origin, pm->ps->clientNum, pm->tracemask);
    if (trace.allsolid) {
        //int shit = qtrue;
    }
}
*/
/*
=============
PM_CorrectAllSolid
=============
*/

unsafe extern "C" fn PM_CorrectAllSolid(
    mut trace: *mut crate::src::qcommon::q_shared::trace_t,
) -> i32 {
    let mut _i: i32 = 0;
    let mut _j: i32 = 0;
    let mut _k: i32 = 0;
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*pm).debugLevel != 0 {
        crate::src::game::g_main::Com_Printf(
            b"%i:allsolid\n\x00" as *const u8 as *const i8,
            c_pmove,
        );
    }
    // jitter around

    for i in -(1)..=1 {
        for j in -(1)..=1 {
            for k in -(1)..=1 {
                point[0] = (*(*pm).ps).origin[0];

                point[1] = (*(*pm).ps).origin[1];

                point[2] = (*(*pm).ps).origin[2];

                point[0] += i as f32;

                point[1] += j as f32;

                point[2] += k as f32;

                (*pm).trace.expect("non-null function pointer")(
                    trace,
                    point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*pm).mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*pm).maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*(*pm).ps).clientNum,
                    (*pm).tracemask,
                );

                if (*trace).allsolid as u64 == 0 {
                    point[0] = (*(*pm).ps).origin[0];
                    point[1] = (*(*pm).ps).origin[1];
                    point[2] = ((*(*pm).ps).origin[2] as f64 - 0.25)
                        as crate::src::qcommon::q_shared::vec_t;
                    (*pm).trace.expect("non-null function pointer")(
                        trace,
                        (*(*pm).ps).origin.as_mut_ptr()
                            as *const crate::src::qcommon::q_shared::vec_t,
                        (*pm).mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        (*pm).maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                        (*(*pm).ps).clientNum,
                        (*pm).tracemask,
                    );
                    pml.groundTrace = *trace;
                    return crate::src::qcommon::q_shared::qtrue as i32;
                }
            }
        }
    }
    (*(*pm).ps).groundEntityNum = ((1) << 10) - 1;
    pml.groundPlane = crate::src::qcommon::q_shared::qfalse;
    pml.walking = crate::src::qcommon::q_shared::qfalse;
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
=============
PM_GroundTraceMissed

The ground trace didn't hit a surface, so we are in freefall
=============
*/

unsafe extern "C" fn PM_GroundTraceMissed() {
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
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if (*(*pm).ps).groundEntityNum != ((1) << 10) - 1 {
        // we just transitioned into freefall
        if (*pm).debugLevel != 0 {
            crate::src::game::g_main::Com_Printf(
                b"%i:lift\n\x00" as *const u8 as *const i8,
                c_pmove,
            );
        }
        // if they aren't in a jumping animation and the ground is a ways away, force into it
        // if we didn't do the trace, the player would be backflipping down staircases
        point[0] = (*(*pm).ps).origin[0];
        point[1] = (*(*pm).ps).origin[1];
        point[2] = (*(*pm).ps).origin[2];
        point[2] -= 64f32;
        (*pm).trace.expect("non-null function pointer")(
            &mut trace,
            (*(*pm).ps).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*pm).mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*pm).maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*(*pm).ps).clientNum,
            (*pm).tracemask,
        );
        if trace.fraction as f64 == 1.0 {
            if (*pm).cmd.forwardmove as i32 >= 0 {
                PM_ForceLegsAnim(crate::bg_public_h::LEGS_JUMP as i32);
                (*(*pm).ps).pm_flags &= !(8)
            } else {
                PM_ForceLegsAnim(crate::bg_public_h::LEGS_JUMPB as i32);
                (*(*pm).ps).pm_flags |= 8
            }
        }
    }
    (*(*pm).ps).groundEntityNum = ((1) << 10) - 1;
    pml.groundPlane = crate::src::qcommon::q_shared::qfalse;
    pml.walking = crate::src::qcommon::q_shared::qfalse;
}
/*
=============
PM_GroundTrace
=============
*/

unsafe extern "C" fn PM_GroundTrace() {
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    point[0] = (*(*pm).ps).origin[0];
    point[1] = (*(*pm).ps).origin[1];
    point[2] = ((*(*pm).ps).origin[2] as f64 - 0.25) as crate::src::qcommon::q_shared::vec_t;
    (*pm).trace.expect("non-null function pointer")(
        &mut trace,
        (*(*pm).ps).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*pm).mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*pm).maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*(*pm).ps).clientNum,
        (*pm).tracemask,
    );
    pml.groundTrace = trace;
    // do something corrective if the trace starts in a solid...
    if trace.allsolid as u64 != 0 {
        if PM_CorrectAllSolid(&mut trace) == 0 {
            return;
        }
    }
    // if the trace didn't hit anything, we are in free fall
    if trace.fraction as f64 == 1.0 {
        PM_GroundTraceMissed();
        pml.groundPlane = crate::src::qcommon::q_shared::qfalse;
        pml.walking = crate::src::qcommon::q_shared::qfalse;
        return;
    }
    // check if getting thrown off the ground
    if (*(*pm).ps).velocity[2] > 0f32
        && (*(*pm).ps).velocity[0] * trace.plane.normal[0]
            + (*(*pm).ps).velocity[1] * trace.plane.normal[1]
            + (*(*pm).ps).velocity[2] * trace.plane.normal[2]
            > 10f32
    {
        if (*pm).debugLevel != 0 {
            crate::src::game::g_main::Com_Printf(
                b"%i:kickoff\n\x00" as *const u8 as *const i8,
                c_pmove,
            );
        }
        // go into jump animation
        if (*pm).cmd.forwardmove as i32 >= 0 {
            PM_ForceLegsAnim(crate::bg_public_h::LEGS_JUMP as i32);
            (*(*pm).ps).pm_flags &= !(8)
        } else {
            PM_ForceLegsAnim(crate::bg_public_h::LEGS_JUMPB as i32);
            (*(*pm).ps).pm_flags |= 8
        }
        (*(*pm).ps).groundEntityNum = ((1) << 10) - 1;
        pml.groundPlane = crate::src::qcommon::q_shared::qfalse;
        pml.walking = crate::src::qcommon::q_shared::qfalse;
        return;
    }
    // slopes that are too steep will not be considered onground
    if trace.plane.normal[2] < 0.7 {
        if (*pm).debugLevel != 0 {
            crate::src::game::g_main::Com_Printf(
                b"%i:steep\n\x00" as *const u8 as *const i8,
                c_pmove,
            );
        }
        // FIXME: if they can't slide down the slope, let them
        // walk (sharp crevices)
        (*(*pm).ps).groundEntityNum = ((1) << 10) - 1;
        pml.groundPlane = crate::src::qcommon::q_shared::qtrue;
        pml.walking = crate::src::qcommon::q_shared::qfalse;
        return;
    }
    pml.groundPlane = crate::src::qcommon::q_shared::qtrue;
    pml.walking = crate::src::qcommon::q_shared::qtrue;
    // hitting solid ground will end a waterjump
    if (*(*pm).ps).pm_flags & 256 != 0 {
        (*(*pm).ps).pm_flags &= !(256 | 32);
        (*(*pm).ps).pm_time = 0
    }
    if (*(*pm).ps).groundEntityNum == ((1) << 10) - 1 {
        // just hit the ground
        if (*pm).debugLevel != 0 {
            crate::src::game::g_main::Com_Printf(
                b"%i:Land\n\x00" as *const u8 as *const i8,
                c_pmove,
            );
        }
        PM_CrashLand();
        // don't do landing time if we were just going down a slope
        if pml.previous_velocity[2] < -200f32 {
            // don't allow another jump for a little while
            (*(*pm).ps).pm_flags |= 32;
            (*(*pm).ps).pm_time = 250
        }
    }
    (*(*pm).ps).groundEntityNum = trace.entityNum;
    // don't reset the z velocity for slopes
    //	pm->ps->velocity[2] = 0;
    PM_AddTouchEnt(trace.entityNum);
}
/*
=============
PM_SetWaterLevel	FIXME: avoid this twice?  certainly if not moving
=============
*/

unsafe extern "C" fn PM_SetWaterLevel() {
    let mut point: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cont: i32 = 0;
    let mut sample1: i32 = 0;
    let mut sample2: i32 = 0;
    //
    // get waterlevel, accounting for ducking
    //
    (*pm).waterlevel = 0;
    (*pm).watertype = 0;
    point[0] = (*(*pm).ps).origin[0];
    point[1] = (*(*pm).ps).origin[1];
    point[2] = (*(*pm).ps).origin[2] + -24f32 + 1f32;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*(*pm).ps).clientNum,
    );
    if cont & (32 | 8 | 16) != 0 {
        sample2 = (*(*pm).ps).viewheight - -(24);
        sample1 = sample2 / 2;
        (*pm).watertype = cont;
        (*pm).waterlevel = 1;
        point[2] = (*(*pm).ps).origin[2] + -24f32 + sample1 as f32;
        cont = (*pm).pointcontents.expect("non-null function pointer")(
            point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*(*pm).ps).clientNum,
        );
        if cont & (32 | 8 | 16) != 0 {
            (*pm).waterlevel = 2;
            point[2] = (*(*pm).ps).origin[2] + -24f32 + sample2 as f32;
            cont = (*pm).pointcontents.expect("non-null function pointer")(
                point.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*(*pm).ps).clientNum,
            );
            if cont & (32 | 8 | 16) != 0 {
                (*pm).waterlevel = 3
            }
        }
    };
}
/*
==============
PM_CheckDuck

Sets mins, maxs, and pm->ps->viewheight
==============
*/

unsafe extern "C" fn PM_CheckDuck() {
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
    if (*(*pm).ps).powerups[crate::bg_public_h::PW_INVULNERABILITY as usize] != 0 {
        if (*(*pm).ps).pm_flags & 16384 != 0 {
            // invulnerability sphere has a 42 units radius
            (*pm).mins[0] = -42f32;
            (*pm).mins[1] = -42f32;
            (*pm).mins[2] = -42f32;
            (*pm).maxs[0] = 42f32;
            (*pm).maxs[1] = 42f32;
            (*pm).maxs[2] = 42f32
        } else {
            (*pm).mins[0] = -15f32;
            (*pm).mins[1] = -15f32;
            (*pm).mins[2] = -24f32;
            (*pm).maxs[0] = 15f32;
            (*pm).maxs[1] = 15f32;
            (*pm).maxs[2] = 16f32
        }
        (*(*pm).ps).pm_flags |= 1;
        (*(*pm).ps).viewheight = 12;
        return;
    }
    (*(*pm).ps).pm_flags &= !(16384);
    (*pm).mins[0] = -15f32;
    (*pm).mins[1] = -15f32;
    (*pm).maxs[0] = 15f32;
    (*pm).maxs[1] = 15f32;
    (*pm).mins[2] = -24f32;
    if (*(*pm).ps).pm_type == crate::bg_public_h::PM_DEAD as i32 {
        (*pm).maxs[2] = -8f32;
        (*(*pm).ps).viewheight = -(16);
        return;
    }
    if ((*pm).cmd.upmove as i32) < 0 {
        // duck
        (*(*pm).ps).pm_flags |= 1
    } else if (*(*pm).ps).pm_flags & 1 != 0 {
        // stand up if possible
        // try to stand up
        (*pm).maxs[2] = 32f32;
        (*pm).trace.expect("non-null function pointer")(
            &mut trace,
            (*(*pm).ps).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*pm).mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*pm).maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*(*pm).ps).origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*(*pm).ps).clientNum,
            (*pm).tracemask,
        );
        if trace.allsolid as u64 == 0 {
            (*(*pm).ps).pm_flags &= !(1)
        }
    }
    if (*(*pm).ps).pm_flags & 1 != 0 {
        (*pm).maxs[2] = 16f32;
        (*(*pm).ps).viewheight = 12
    } else {
        (*pm).maxs[2] = 32f32;
        (*(*pm).ps).viewheight = 26
    };
}
//===================================================================
/*
===============
PM_Footsteps
===============
*/

unsafe extern "C" fn PM_Footsteps() {
    let mut bobmove: f32 = 0.;
    let mut old: i32 = 0;
    let mut footstep: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    //
    // calculate speed and cycle to be used for
    // all cyclic walking effects
    //
    (*pm).xyspeed = crate::stdlib::sqrt(
        ((*(*pm).ps).velocity[0] * (*(*pm).ps).velocity[0]
            + (*(*pm).ps).velocity[1] * (*(*pm).ps).velocity[1]) as f64,
    ) as f32;
    if (*(*pm).ps).groundEntityNum == ((1) << 10) - 1 {
        if (*(*pm).ps).powerups[crate::bg_public_h::PW_INVULNERABILITY as usize] != 0 {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_IDLECR as i32);
        }
        // airborne leaves position in cycle intact, but doesn't advance
        if (*pm).waterlevel > 1 {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_SWIM as i32);
        }
        return;
    }
    // if not trying to move
    if (*pm).cmd.forwardmove == 0 && (*pm).cmd.rightmove == 0 {
        if (*pm).xyspeed < 5f32 {
            (*(*pm).ps).bobCycle = 0; // start at beginning of cycle again
            if (*(*pm).ps).pm_flags & 1 != 0 {
                PM_ContinueLegsAnim(crate::bg_public_h::LEGS_IDLECR as i32);
            // ducked characters bob much faster
            } else {
                PM_ContinueLegsAnim(crate::bg_public_h::LEGS_IDLE as i32);
            }
        }
        return;
    }
    footstep = crate::src::qcommon::q_shared::qfalse;
    if (*(*pm).ps).pm_flags & 1 != 0 {
        bobmove = 0.5;
        if (*(*pm).ps).pm_flags & 16 != 0 {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_BACKCR as i32);
        } else {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_WALKCR as i32);
        }
    // ducked characters never play footsteps
    /*
    } else 	if ( pm->ps->pm_flags & PMF_BACKWARDS_RUN ) {
        if ( !( pm->cmd.buttons & BUTTON_WALKING ) ) {
            bobmove = 0.4;	// faster speeds bob faster
            footstep = qtrue;
        } else {
            bobmove = 0.3;
        }
        PM_ContinueLegsAnim( LEGS_BACK );
    */
    } else if (*pm).cmd.buttons & 16 == 0 {
        bobmove = 0.4; // faster speeds bob faster
        if (*(*pm).ps).pm_flags & 16 != 0 {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_BACK as i32); // walking bobs slow
        } else {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_RUN as i32);
        }
        footstep = crate::src::qcommon::q_shared::qtrue
    } else {
        bobmove = 0.3;
        if (*(*pm).ps).pm_flags & 16 != 0 {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_BACKWALK as i32);
        } else {
            PM_ContinueLegsAnim(crate::bg_public_h::LEGS_WALK as i32);
        }
    }
    // check for footstep / splash sounds
    old = (*(*pm).ps).bobCycle;
    (*(*pm).ps).bobCycle = (old as f32 + bobmove * pml.msec as f32) as i32 & 255;
    // if we just crossed a cycle boundary, play an appropriate footstep event
    if (old + 64 ^ (*(*pm).ps).bobCycle + 64) & 128 != 0 {
        if (*pm).waterlevel == 0 {
            // on ground will only play sounds if running
            if footstep != 0 && (*pm).noFootsteps as u64 == 0 {
                PM_AddEvent(PM_FootstepForSurface());
            }
        } else if (*pm).waterlevel == 1 {
            // splashing
            PM_AddEvent(crate::bg_public_h::EV_FOOTSPLASH as i32);
        } else if (*pm).waterlevel == 2 {
            // wading / swimming at surface
            PM_AddEvent(crate::bg_public_h::EV_SWIM as i32);
        } else {
            ((*pm).waterlevel) == 3i32;
        }
    };
}
/*
==============
PM_WaterEvents

Generate sound events for entering and leaving water
==============
*/

unsafe extern "C" fn PM_WaterEvents() {
    // FIXME?
    //
    // if just entered a water volume, play a sound
    //
    if pml.previous_waterlevel == 0 && (*pm).waterlevel != 0 {
        PM_AddEvent(crate::bg_public_h::EV_WATER_TOUCH as i32);
    }
    //
    // if just completely exited a water volume, play a sound
    //
    if pml.previous_waterlevel != 0 && (*pm).waterlevel == 0 {
        PM_AddEvent(crate::bg_public_h::EV_WATER_LEAVE as i32);
    }
    //
    // check for head just going under water
    //
    if pml.previous_waterlevel != 3 && (*pm).waterlevel == 3 {
        PM_AddEvent(crate::bg_public_h::EV_WATER_UNDER as i32);
    }
    //
    // check for head just coming out of water
    //
    if pml.previous_waterlevel == 3 && (*pm).waterlevel != 3 {
        PM_AddEvent(crate::bg_public_h::EV_WATER_CLEAR as i32);
    };
}
/*
===============
PM_BeginWeaponChange
===============
*/

unsafe extern "C" fn PM_BeginWeaponChange(mut weapon: i32) {
    if weapon <= crate::bg_public_h::WP_NONE as i32
        || weapon >= crate::bg_public_h::WP_NUM_WEAPONS as i32
    {
        return;
    }
    if (*(*pm).ps).stats[crate::bg_public_h::STAT_WEAPONS as usize] & (1) << weapon == 0 {
        return;
    }
    if (*(*pm).ps).weaponstate == crate::bg_public_h::WEAPON_DROPPING as i32 {
        return;
    }
    PM_AddEvent(crate::bg_public_h::EV_CHANGE_WEAPON as i32);
    (*(*pm).ps).weaponstate = crate::bg_public_h::WEAPON_DROPPING as i32;
    (*(*pm).ps).weaponTime += 200;
    PM_StartTorsoAnim(crate::bg_public_h::TORSO_DROP as i32);
}
/*
===============
PM_FinishWeaponChange
===============
*/

unsafe extern "C" fn PM_FinishWeaponChange() {
    let mut weapon: i32 = 0;
    weapon = (*pm).cmd.weapon as i32;
    if weapon < crate::bg_public_h::WP_NONE as i32
        || weapon >= crate::bg_public_h::WP_NUM_WEAPONS as i32
    {
        weapon = crate::bg_public_h::WP_NONE as i32
    }
    if (*(*pm).ps).stats[crate::bg_public_h::STAT_WEAPONS as usize] & (1) << weapon == 0 {
        weapon = crate::bg_public_h::WP_NONE as i32
    }
    (*(*pm).ps).weapon = weapon;
    (*(*pm).ps).weaponstate = crate::bg_public_h::WEAPON_RAISING as i32;
    (*(*pm).ps).weaponTime += 250;
    PM_StartTorsoAnim(crate::bg_public_h::TORSO_RAISE as i32);
}
/*
==============
PM_TorsoAnimation

==============
*/

unsafe extern "C" fn PM_TorsoAnimation() {
    if (*(*pm).ps).weaponstate == crate::bg_public_h::WEAPON_READY as i32 {
        if (*(*pm).ps).weapon == crate::bg_public_h::WP_GAUNTLET as i32 {
            PM_ContinueTorsoAnim(crate::bg_public_h::TORSO_STAND2 as i32);
        } else {
            PM_ContinueTorsoAnim(crate::bg_public_h::TORSO_STAND as i32);
        }
        return;
    };
}
/*
==============
PM_Weapon

Generates weapon events and modifes the weapon counter
==============
*/

unsafe extern "C" fn PM_Weapon() {
    let mut addTime: i32 = 0;
    // don't allow attack until all buttons are up
    if (*(*pm).ps).pm_flags & 512 != 0 {
        return;
    }
    // ignore if spectator
    if (*(*pm).ps).persistant[crate::bg_public_h::PERS_TEAM as usize]
        == crate::bg_public_h::TEAM_SPECTATOR as i32
    {
        return;
    }
    // check for dead player
    if (*(*pm).ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= 0 {
        (*(*pm).ps).weapon = crate::bg_public_h::WP_NONE as i32;
        return;
    }
    // check for item using
    if (*pm).cmd.buttons & 4 != 0 {
        if (*(*pm).ps).pm_flags & 1024 == 0 {
            if !((*crate::src::game::bg_misc::bg_itemlist.as_mut_ptr().offset(
                (*(*pm).ps).stats[crate::bg_public_h::STAT_HOLDABLE_ITEM as usize] as isize,
            ))
            .giTag
                == crate::bg_public_h::HI_MEDKIT as i32
                && (*(*pm).ps).stats[crate::bg_public_h::STAT_HEALTH as usize]
                    >= (*(*pm).ps).stats[crate::bg_public_h::STAT_MAX_HEALTH as usize] + 25)
            {
                (*(*pm).ps).pm_flags |= 1024;
                PM_AddEvent(
                    crate::bg_public_h::EV_USE_ITEM0 as i32
                        + (*crate::src::game::bg_misc::bg_itemlist.as_mut_ptr().offset(
                            (*(*pm).ps).stats[crate::bg_public_h::STAT_HOLDABLE_ITEM as usize]
                                as isize,
                        ))
                        .giTag,
                );
                (*(*pm).ps).stats[crate::bg_public_h::STAT_HOLDABLE_ITEM as usize] = 0
            }
            return;
        }
    } else {
        (*(*pm).ps).pm_flags &= !(1024)
    }
    // make weapon function
    if (*(*pm).ps).weaponTime > 0 {
        (*(*pm).ps).weaponTime -= pml.msec
    }
    // check for weapon change
    // can't change if weapon is firing, but can change
    // again if lowering or raising
    if (*(*pm).ps).weaponTime <= 0
        || (*(*pm).ps).weaponstate != crate::bg_public_h::WEAPON_FIRING as i32
    {
        if (*(*pm).ps).weapon != (*pm).cmd.weapon as i32 {
            PM_BeginWeaponChange((*pm).cmd.weapon as i32);
        }
    }
    if (*(*pm).ps).weaponTime > 0 {
        return;
    }
    // change weapon if time
    if (*(*pm).ps).weaponstate == crate::bg_public_h::WEAPON_DROPPING as i32 {
        PM_FinishWeaponChange();
        return;
    }
    if (*(*pm).ps).weaponstate == crate::bg_public_h::WEAPON_RAISING as i32 {
        (*(*pm).ps).weaponstate = crate::bg_public_h::WEAPON_READY as i32;
        if (*(*pm).ps).weapon == crate::bg_public_h::WP_GAUNTLET as i32 {
            PM_StartTorsoAnim(crate::bg_public_h::TORSO_STAND2 as i32);
        } else {
            PM_StartTorsoAnim(crate::bg_public_h::TORSO_STAND as i32);
        }
        return;
    }
    // check for fire
    if (*pm).cmd.buttons & 1 == 0 {
        (*(*pm).ps).weaponTime = 0;
        (*(*pm).ps).weaponstate = crate::bg_public_h::WEAPON_READY as i32;
        return;
    }
    // start the animation even if out of ammo
    if (*(*pm).ps).weapon == crate::bg_public_h::WP_GAUNTLET as i32 {
        // the guantlet only "fires" when it actually hits something
        if (*pm).gauntletHit as u64 == 0 {
            (*(*pm).ps).weaponTime = 0;
            (*(*pm).ps).weaponstate = crate::bg_public_h::WEAPON_READY as i32;
            return;
        }
        PM_StartTorsoAnim(crate::bg_public_h::TORSO_ATTACK2 as i32);
    } else {
        PM_StartTorsoAnim(crate::bg_public_h::TORSO_ATTACK as i32);
    }
    (*(*pm).ps).weaponstate = crate::bg_public_h::WEAPON_FIRING as i32;
    // check for out of ammo
    if (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] == 0 {
        PM_AddEvent(crate::bg_public_h::EV_NOAMMO as i32);
        (*(*pm).ps).weaponTime += 500;
        return;
    }
    // take an ammo away if not infinite
    if (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] != -(1) {
        (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] -= 1
    }
    // fire weapon
    PM_AddEvent(crate::bg_public_h::EV_FIRE_WEAPON as i32);
    match (*(*pm).ps).weapon {
        6 => addTime = 50,
        3 => addTime = 1000,
        2 => addTime = 100,
        4 => addTime = 800,
        5 => addTime = 800,
        8 => addTime = 100,
        7 => addTime = 1500,
        9 => addTime = 200,
        10 => addTime = 400,
        1 | _ => addTime = 400,
    }
    if (*(*pm).ps).powerups[crate::bg_public_h::PW_HASTE as usize] != 0 {
        addTime = (addTime as f64 / 1.3) as i32
    }
    (*(*pm).ps).weaponTime += addTime;
}
/*
================
PM_Animate
================
*/

unsafe extern "C" fn PM_Animate() {
    if (*pm).cmd.buttons & 8 != 0 {
        if (*(*pm).ps).torsoTimer == 0 {
            PM_StartTorsoAnim(crate::bg_public_h::TORSO_GESTURE as i32);
            (*(*pm).ps).torsoTimer = 34 * 66 + 50;
            PM_AddEvent(crate::bg_public_h::EV_TAUNT as i32);
        }
    };
}
/*
================
PM_DropTimers
================
*/

unsafe extern "C" fn PM_DropTimers() {
    // drop misc timing counter
    if (*(*pm).ps).pm_time != 0 {
        if pml.msec >= (*(*pm).ps).pm_time {
            (*(*pm).ps).pm_flags &= !(256 | 32 | 64);
            (*(*pm).ps).pm_time = 0
        } else {
            (*(*pm).ps).pm_time -= pml.msec
        }
    }
    // drop animation counter
    if (*(*pm).ps).legsTimer > 0 {
        (*(*pm).ps).legsTimer -= pml.msec;
        if (*(*pm).ps).legsTimer < 0 {
            (*(*pm).ps).legsTimer = 0
        }
    }
    if (*(*pm).ps).torsoTimer > 0 {
        (*(*pm).ps).torsoTimer -= pml.msec;
        if (*(*pm).ps).torsoTimer < 0 {
            (*(*pm).ps).torsoTimer = 0
        }
    };
}
// if a full pmove isn't done on the client, you can just update the angles
/*
================
PM_UpdateViewAngles

This can be used as another entry point when only the viewangles
are being updated instead of a full move
================
*/
#[no_mangle]

pub unsafe extern "C" fn PM_UpdateViewAngles(
    mut ps: *mut crate::src::qcommon::q_shared::playerState_t,
    mut cmd: *const crate::src::qcommon::q_shared::usercmd_t,
) {
    let mut temp: i16 = 0;
    let mut i: i32 = 0;
    if (*ps).pm_type == crate::bg_public_h::PM_INTERMISSION as i32
        || (*ps).pm_type == crate::bg_public_h::PM_SPINTERMISSION as i32
    {
        return;
        // no view changes at all
    }
    if (*ps).pm_type != crate::bg_public_h::PM_SPECTATOR as i32
        && (*ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= 0
    {
        return;
        // no view changes at all
    }
    // circularly clamp the angles with deltas
    i = 0;
    while i < 3 {
        temp = ((*cmd).angles[i as usize] + (*ps).delta_angles[i as usize]) as i16;
        if i == 0 {
            // don't let the player look up or down more than 90 degrees
            if temp as i32 > 16000 {
                (*ps).delta_angles[i as usize] = 16000 - (*cmd).angles[i as usize];
                temp = 16000
            } else if (temp as i32) < -(16000) {
                (*ps).delta_angles[i as usize] = -(16000) - (*cmd).angles[i as usize];
                temp = -16000
            }
        }
        (*ps).viewangles[i as usize] =
            (temp as i32 as f64 * (360.0 / 65536f64)) as crate::src::qcommon::q_shared::vec_t;
        i += 1
    }
}
#[no_mangle]

pub unsafe extern "C" fn PmoveSingle(mut pmove: *mut crate::bg_public_h::pmove_t) {
    pm = pmove;
    // this counter lets us debug movement problems with a journal
    // by setting a conditional breakpoint fot the previous frame
    c_pmove += 1;
    // clear results
    (*pm).numtouch = 0;
    (*pm).watertype = 0;
    (*pm).waterlevel = 0;
    if (*(*pm).ps).stats[crate::bg_public_h::STAT_HEALTH as usize] <= 0 {
        (*pm).tracemask &= !(0x2000000)
        // corpses can fly through bodies
    }
    // make sure walking button is clear if they are running, to avoid
    // proxy no-footsteps cheats
    if crate::stdlib::abs((*pm).cmd.forwardmove as i32) > 64
        || crate::stdlib::abs((*pm).cmd.rightmove as i32) > 64
    {
        (*pm).cmd.buttons &= !(16)
    }
    // set the talk balloon flag
    if (*pm).cmd.buttons & 2 != 0 {
        (*(*pm).ps).eFlags |= 0x1000
    } else {
        (*(*pm).ps).eFlags &= !(0x1000)
    }
    // set the firing flag for continuous beam weapons
    if (*(*pm).ps).pm_flags & 512 == 0
        && (*(*pm).ps).pm_type != crate::bg_public_h::PM_INTERMISSION as i32
        && (*(*pm).ps).pm_type != crate::bg_public_h::PM_NOCLIP as i32
        && (*pm).cmd.buttons & 1 != 0
        && (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] != 0
    {
        (*(*pm).ps).eFlags |= 0x100
    } else {
        (*(*pm).ps).eFlags &= !(0x100)
    }
    // clear the respawned flag if attack and use are cleared
    if (*(*pm).ps).stats[crate::bg_public_h::STAT_HEALTH as usize] > 0
        && (*pm).cmd.buttons & (1 | 4) == 0
    {
        (*(*pm).ps).pm_flags &= !(512)
    }
    // if talk button is down, dissallow all other input
    // this is to prevent any possible intercept proxy from
    // adding fake talk balloons
    if (*pmove).cmd.buttons & 2 != 0 {
        // keep the talk button set tho for when the cmd.serverTime > 66 msec
        // and the same cmd is used multiple times in Pmove
        (*pmove).cmd.buttons = 2;
        (*pmove).cmd.forwardmove = 0i8;
        (*pmove).cmd.rightmove = 0i8;
        (*pmove).cmd.upmove = 0i8
    }
    // clear all pmove local vars
    crate::stdlib::memset(
        &mut pml as *mut crate::bg_local_h::pml_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::bg_local_h::pml_t>(),
    );
    // determine the time
    pml.msec = (*pmove).cmd.serverTime - (*(*pm).ps).commandTime;
    if pml.msec < 1 {
        pml.msec = 1
    } else if pml.msec > 200 {
        pml.msec = 200
    }
    (*(*pm).ps).commandTime = (*pmove).cmd.serverTime;
    // save old org in case we get stuck
    pml.previous_origin[0] = (*(*pm).ps).origin[0];
    pml.previous_origin[1] = (*(*pm).ps).origin[1];
    pml.previous_origin[2] = (*(*pm).ps).origin[2];
    // save old velocity for crashlanding
    pml.previous_velocity[0] = (*(*pm).ps).velocity[0];
    pml.previous_velocity[1] = (*(*pm).ps).velocity[1];
    pml.previous_velocity[2] = (*(*pm).ps).velocity[2];
    pml.frametime = (pml.msec as f64 * 0.001) as f32;
    // update the viewangles
    PM_UpdateViewAngles((*pm).ps, &mut (*pm).cmd);
    crate::src::qcommon::q_math::AngleVectors(
        (*(*pm).ps).viewangles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        pml.forward.as_mut_ptr(),
        pml.right.as_mut_ptr(),
        pml.up.as_mut_ptr(),
    );
    if ((*pm).cmd.upmove as i32) < 10 {
        // not holding jump
        (*(*pm).ps).pm_flags &= !(2)
    }
    // decide if backpedaling animations should be used
    if ((*pm).cmd.forwardmove as i32) < 0 {
        (*(*pm).ps).pm_flags |= 16
    } else if (*pm).cmd.forwardmove as i32 > 0
        || (*pm).cmd.forwardmove as i32 == 0 && (*pm).cmd.rightmove as i32 != 0
    {
        (*(*pm).ps).pm_flags &= !(16)
    }
    if (*(*pm).ps).pm_type >= crate::bg_public_h::PM_DEAD as i32 {
        (*pm).cmd.forwardmove = 0;
        (*pm).cmd.rightmove = 0;
        (*pm).cmd.upmove = 0
    }
    if (*(*pm).ps).pm_type == crate::bg_public_h::PM_SPECTATOR as i32 {
        PM_CheckDuck();
        PM_FlyMove();
        PM_DropTimers();
        return;
    }
    if (*(*pm).ps).pm_type == crate::bg_public_h::PM_NOCLIP as i32 {
        PM_NoclipMove();
        PM_DropTimers();
        return;
    }
    if (*(*pm).ps).pm_type == crate::bg_public_h::PM_FREEZE as i32 {
        return;
        // no movement at all
    }
    if (*(*pm).ps).pm_type == crate::bg_public_h::PM_INTERMISSION as i32
        || (*(*pm).ps).pm_type == crate::bg_public_h::PM_SPINTERMISSION as i32
    {
        return;
        // no movement at all
    }
    // set watertype, and waterlevel
    PM_SetWaterLevel();
    pml.previous_waterlevel = (*pmove).waterlevel;
    // set mins, maxs, and viewheight
    PM_CheckDuck();
    // set groundentity
    PM_GroundTrace();
    if (*(*pm).ps).pm_type == crate::bg_public_h::PM_DEAD as i32 {
        PM_DeadMove();
    }
    PM_DropTimers();
    if (*(*pm).ps).powerups[crate::bg_public_h::PW_FLIGHT as usize] != 0 {
        // flight powerup doesn't allow jump and has different friction
        PM_FlyMove();
    } else if (*(*pm).ps).pm_flags & 2048 != 0 {
        PM_GrappleMove();
        // We can wiggle a bit
        PM_AirMove();
    } else if (*(*pm).ps).pm_flags & 256 != 0 {
        PM_WaterJumpMove();
    } else if (*pm).waterlevel > 1 {
        // swimming
        PM_WaterMove();
    } else if pml.walking as u64 != 0 {
        // walking on ground
        PM_WalkMove();
    } else {
        // airborne
        PM_AirMove();
    }
    PM_Animate();
    // set groundentity, watertype, and waterlevel
    PM_GroundTrace();
    PM_SetWaterLevel();
    // weapons
    PM_Weapon();
    // torso animation
    PM_TorsoAnimation();
    // footstep events / legs animations
    PM_Footsteps();
    // entering / leaving water splashes
    PM_WaterEvents();
    // snap some parts of playerstate to save network bandwidth
    trap_SnapVector((*(*pm).ps).velocity.as_mut_ptr());
}
/*
================
Pmove

Can be called by either the server or the client
================
*/
#[no_mangle]

pub unsafe extern "C" fn Pmove(mut pmove: *mut crate::bg_public_h::pmove_t) {
    let mut finalTime: i32 = 0;
    finalTime = (*pmove).cmd.serverTime;
    if finalTime < (*(*pmove).ps).commandTime {
        return;
        // should not happen
    }
    if finalTime > (*(*pmove).ps).commandTime + 1000 {
        (*(*pmove).ps).commandTime = finalTime - 1000
    }
    (*(*pmove).ps).pmove_framecount = (*(*pmove).ps).pmove_framecount + 1 & ((1) << 6) - 1;
    // chop the move up if it is too long, to prevent framerate
    // dependent behavior
    while (*(*pmove).ps).commandTime != finalTime {
        let mut msec: i32 = 0;
        msec = finalTime - (*(*pmove).ps).commandTime;
        if (*pmove).pmove_fixed != 0 {
            if msec > (*pmove).pmove_msec {
                msec = (*pmove).pmove_msec
            }
        } else if msec > 66 {
            msec = 66
        }
        (*pmove).cmd.serverTime = (*(*pmove).ps).commandTime + msec;
        PmoveSingle(pmove);
        if (*(*pmove).ps).pm_flags & 2 != 0 {
            (*pmove).cmd.upmove = 20i8
        }
    }
    //PM_CheckStuck();
}
