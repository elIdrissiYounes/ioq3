use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorInverse(mut v: *mut crate::src::qcommon::q_shared::vec_t) {
        *v.offset(0) = -*v.offset(0);
        *v.offset(1) = -*v.offset(1);
        *v.offset(2) = -*v.offset(2);
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

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
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
pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cgMedia_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::cgs_t;
pub use crate::cg_local_h::clientInfo_t;
pub use crate::cg_local_h::footstep_t;
pub use crate::cg_local_h::itemInfo_t;
pub use crate::cg_local_h::lerpFrame_t;
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
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_ents::q_shared_h::CrossProduct;
pub use crate::src::cgame::cg_ents::q_shared_h::VectorInverse;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_items;
pub use crate::src::cgame::cg_main::cg_simpleItems;
pub use crate::src::cgame::cg_main::cg_smoothClients;
pub use crate::src::cgame::cg_main::cg_weapons;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_players::CG_AddRefEntityWithPowerups;
pub use crate::src::cgame::cg_players::CG_Player;
pub use crate::src::cgame::cg_syscalls::trap_R_AddLightToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_LerpTag;
pub use crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound;
pub use crate::src::cgame::cg_syscalls::trap_S_AddRealLoopingSound;
pub use crate::src::cgame::cg_syscalls::trap_S_StartSound;
pub use crate::src::cgame::cg_syscalls::trap_S_UpdateEntityPosition;
pub use crate::src::cgame::cg_weapons::CG_GrappleTrail;
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::game::bg_misc::bg_numItems;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta;
pub use crate::src::game::bg_misc::BG_PlayerStateToEntityState;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_math::AxisCopy;
pub use crate::src::qcommon::q_math::ByteToDir;
pub use crate::src::qcommon::q_math::LerpAngle;
pub use crate::src::qcommon::q_math::MatrixMultiply;
pub use crate::src::qcommon::q_math::PerpendicularVector;
pub use crate::src::qcommon::q_math::RotateAroundDirection;
pub use crate::src::qcommon::q_math::VectorNormalize2;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
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
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
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
use crate::stdlib::cos;
use crate::stdlib::memset;
use crate::stdlib::rand;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
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
// cg_ents.c -- present snapshot entities, happens every single frame
/*
======================
CG_PositionEntityOnTag

Modifies the entities position and axis by the given
tag location
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PositionEntityOnTag(
    mut entity: *mut crate::tr_types_h::refEntity_t,
    mut parent: *const crate::tr_types_h::refEntity_t,
    mut parentModel: crate::src::qcommon::q_shared::qhandle_t,
    mut tagName: *mut i8,
) {
    let mut i: i32 = 0;
    let mut lerped: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    // lerp the tag
    crate::src::cgame::cg_syscalls::trap_R_LerpTag(
        &mut lerped,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0 - (*parent).backlerp as f64) as f32,
        tagName,
    );
    // FIXME: allow origin offsets along tag?
    (*entity).origin[0] = (*parent).origin[0];
    (*entity).origin[1] = (*parent).origin[1];
    (*entity).origin[2] = (*parent).origin[2];

    for i in 0..3 {
        (*entity).origin[0] =
            (*entity).origin[0] + (*parent).axis[i as usize][0] * lerped.origin[i as usize];

        (*entity).origin[1] =
            (*entity).origin[1] + (*parent).axis[i as usize][1] * lerped.origin[i as usize];

        (*entity).origin[2] =
            (*entity).origin[2] + (*parent).axis[i as usize][2] * lerped.origin[i as usize];
    }
    // had to cast away the const to avoid compiler problems...
    crate::src::qcommon::q_math::MatrixMultiply(
        lerped.axis.as_mut_ptr(),
        (*(parent as *mut crate::tr_types_h::refEntity_t))
            .axis
            .as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
    (*entity).backlerp = (*parent).backlerp;
}
/*
======================
CG_PositionRotatedEntityOnTag

Modifies the entities position and axis by the given
tag location
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_PositionRotatedEntityOnTag(
    mut entity: *mut crate::tr_types_h::refEntity_t,
    mut parent: *const crate::tr_types_h::refEntity_t,
    mut parentModel: crate::src::qcommon::q_shared::qhandle_t,
    mut tagName: *mut i8,
) {
    let mut i: i32 = 0;
    let mut lerped: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    let mut tempAxis: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    //AxisClear( entity->axis );
    // lerp the tag
    crate::src::cgame::cg_syscalls::trap_R_LerpTag(
        &mut lerped,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0 - (*parent).backlerp as f64) as f32,
        tagName,
    );
    // FIXME: allow origin offsets along tag?
    (*entity).origin[0] = (*parent).origin[0];
    (*entity).origin[1] = (*parent).origin[1];
    (*entity).origin[2] = (*parent).origin[2];

    for i in 0..3 {
        (*entity).origin[0] =
            (*entity).origin[0] + (*parent).axis[i as usize][0] * lerped.origin[i as usize];

        (*entity).origin[1] =
            (*entity).origin[1] + (*parent).axis[i as usize][1] * lerped.origin[i as usize];

        (*entity).origin[2] =
            (*entity).origin[2] + (*parent).axis[i as usize][2] * lerped.origin[i as usize];
    }
    // had to cast away the const to avoid compiler problems...
    crate::src::qcommon::q_math::MatrixMultiply(
        (*entity).axis.as_mut_ptr(),
        lerped.axis.as_mut_ptr(),
        tempAxis.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::MatrixMultiply(
        tempAxis.as_mut_ptr(),
        (*(parent as *mut crate::tr_types_h::refEntity_t))
            .axis
            .as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
}
/*
==========================================================================

FUNCTIONS CALLED EACH FRAME

==========================================================================
*/
/*
======================
CG_SetEntitySoundPosition

Also called by event processing code
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_SetEntitySoundPosition(mut cent: *mut crate::cg_local_h::centity_t) {
    if (*cent).currentState.solid == 0xffffff {
        let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        let mut v: *mut f32 = 0 as *mut f32;
        v = crate::src::cgame::cg_main::cgs.inlineModelMidpoints
            [(*cent).currentState.modelindex as usize]
            .as_mut_ptr();
        origin[0] = (*cent).lerpOrigin[0] + *v.offset(0);
        origin[1] = (*cent).lerpOrigin[1] + *v.offset(1);
        origin[2] = (*cent).lerpOrigin[2] + *v.offset(2);
        crate::src::cgame::cg_syscalls::trap_S_UpdateEntityPosition(
            (*cent).currentState.number,
            origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
    } else {
        crate::src::cgame::cg_syscalls::trap_S_UpdateEntityPosition(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        );
    };
}
/*
==================
CG_EntityEffects

Add continuous entity effects, like local entity emission and lighting
==================
*/

unsafe extern "C" fn CG_EntityEffects(mut cent: *mut crate::cg_local_h::centity_t) {
    // update sound origins
    CG_SetEntitySoundPosition(cent);
    // add loop sound
    if (*cent).currentState.loopSound != 0 {
        if (*cent).currentState.eType != crate::bg_public_h::ET_SPEAKER as i32 {
            crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::cgame::cg_main::cgs.gameSounds[(*cent).currentState.loopSound as usize],
            );
        } else {
            crate::src::cgame::cg_syscalls::trap_S_AddRealLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                    as *const crate::src::qcommon::q_shared::vec_t,
                crate::src::cgame::cg_main::cgs.gameSounds[(*cent).currentState.loopSound as usize],
            );
        }
    }
    // constant light glow
    if (*cent).currentState.constantLight != 0 {
        let mut cl: i32 = 0;
        let mut i: f32 = 0.;
        let mut r: f32 = 0.;
        let mut g: f32 = 0.;
        let mut b: f32 = 0.;
        cl = (*cent).currentState.constantLight;
        r = ((cl & 0xff) as f32 as f64 / 255.0) as f32;
        g = ((cl >> 8 & 0xff) as f32 as f64 / 255.0) as f32;
        b = ((cl >> 16 & 0xff) as f32 as f64 / 255.0) as f32;
        i = ((cl >> 24 & 0xff) as f32 as f64 * 4.0) as f32;
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            i,
            r,
            g,
            b,
        );
    };
}
/*
==================
CG_General
==================
*/

unsafe extern "C" fn CG_General(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // if set to invisible, skip
    if (*s1).modelindex == 0 {
        return;
    }
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    // set frame
    ent.frame = (*s1).frame;
    ent.oldframe = ent.frame;
    ent.backlerp = 0f32;
    ent.origin[0] = (*cent).lerpOrigin[0];
    ent.origin[1] = (*cent).lerpOrigin[1];
    ent.origin[2] = (*cent).lerpOrigin[2];
    ent.oldorigin[0] = (*cent).lerpOrigin[0];
    ent.oldorigin[1] = (*cent).lerpOrigin[1];
    ent.oldorigin[2] = (*cent).lerpOrigin[2];
    ent.hModel = crate::src::cgame::cg_main::cgs.gameModels[(*s1).modelindex as usize];
    // player model
    if (*s1).number == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum {
        ent.renderfx |= 0x2
        // only draw from mirrors
    }
    // convert angles to axis
    crate::src::qcommon::q_math::AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis.as_mut_ptr(),
    );
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
==================
CG_Speaker

Speaker entities can automatically play sounds
==================
*/

unsafe extern "C" fn CG_Speaker(mut cent: *mut crate::cg_local_h::centity_t) {
    if (*cent).currentState.clientNum == 0 {
        // FIXME: use something other than clientNum...
        return;
        // not auto triggering
    }
    if crate::src::cgame::cg_main::cg.time < (*cent).miscTime {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_S_StartSound(
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        (*cent).currentState.number,
        crate::src::qcommon::q_shared::CHAN_ITEM as i32,
        crate::src::cgame::cg_main::cgs.gameSounds[(*cent).currentState.eventParm as usize],
    );
    //	ent->s.frame = ent->wait * 10;
    //	ent->s.clientNum = ent->random * 10;
    (*cent).miscTime = ((crate::src::cgame::cg_main::cg.time + (*cent).currentState.frame * 100)
        as f64
        + ((*cent).currentState.clientNum * 100i32) as f64
            * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
        as i32;
}
/*
==================
CG_Item
==================
*/

unsafe extern "C" fn CG_Item(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut es: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut msec: i32 = 0;
    let mut frac: f32 = 0.;
    let mut scale: f32 = 0.;
    let mut wi: *mut crate::cg_local_h::weaponInfo_t = 0 as *mut crate::cg_local_h::weaponInfo_t;
    es = &mut (*cent).currentState;
    if (*es).modelindex >= crate::src::game::bg_misc::bg_numItems {
        crate::src::cgame::cg_main::CG_Error(
            b"Bad item index %i on entity\x00" as *const u8 as *const i8,
            (*es).modelindex,
        );
    }
    // if set to invisible, skip
    if (*es).modelindex == 0 || (*es).eFlags & 0x80 != 0 {
        return;
    }
    item = &mut *crate::src::game::bg_misc::bg_itemlist
        .as_mut_ptr()
        .offset((*es).modelindex as isize) as *mut crate::bg_public_h::gitem_t;
    if crate::src::cgame::cg_main::cg_simpleItems.integer != 0
        && (*item).giType != crate::bg_public_h::IT_TEAM
    {
        crate::stdlib::memset(
            &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
        );
        ent.reType = crate::tr_types_h::RT_SPRITE;
        ent.origin[0] = (*cent).lerpOrigin[0];
        ent.origin[1] = (*cent).lerpOrigin[1];
        ent.origin[2] = (*cent).lerpOrigin[2];
        ent.radius = 14f32;
        ent.customShader = crate::src::cgame::cg_main::cg_items[(*es).modelindex as usize].icon;
        ent.shaderRGBA[0] = 255;
        ent.shaderRGBA[1] = 255;
        ent.shaderRGBA[2] = 255;
        ent.shaderRGBA[3] = 255;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
        return;
    }
    // items bob up and down continuously
    scale = (0.005 + (*cent).currentState.number as f64 * 0.00001) as f32;
    (*cent).lerpOrigin[2] = ((*cent).lerpOrigin[2] as f64
        + (4f64
            + crate::stdlib::cos(
                ((crate::src::cgame::cg_main::cg.time + 1000) as f32 * scale) as f64,
            ) * 4f64)) as crate::src::qcommon::q_shared::vec_t;
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    // autorotate at one of two speeds
    if (*item).giType == crate::bg_public_h::IT_HEALTH {
        (*cent).lerpAngles[0] = crate::src::cgame::cg_main::cg.autoAnglesFast[0];
        (*cent).lerpAngles[1] = crate::src::cgame::cg_main::cg.autoAnglesFast[1];
        (*cent).lerpAngles[2] = crate::src::cgame::cg_main::cg.autoAnglesFast[2];
        crate::src::qcommon::q_math::AxisCopy(
            crate::src::cgame::cg_main::cg.autoAxisFast.as_mut_ptr(),
            ent.axis.as_mut_ptr(),
        );
    } else {
        (*cent).lerpAngles[0] = crate::src::cgame::cg_main::cg.autoAngles[0];
        (*cent).lerpAngles[1] = crate::src::cgame::cg_main::cg.autoAngles[1];
        (*cent).lerpAngles[2] = crate::src::cgame::cg_main::cg.autoAngles[2];
        crate::src::qcommon::q_math::AxisCopy(
            crate::src::cgame::cg_main::cg.autoAxis.as_mut_ptr(),
            ent.axis.as_mut_ptr(),
        );
    }
    wi = 0 as *mut crate::cg_local_h::weaponInfo_t;
    // the weapons have their origin where they attatch to player
    // models, so we need to offset them or they will rotate
    // eccentricly
    if (*item).giType == crate::bg_public_h::IT_WEAPON {
        wi = &mut *crate::src::cgame::cg_main::cg_weapons
            .as_mut_ptr()
            .offset((*item).giTag as isize) as *mut crate::cg_local_h::weaponInfo_t;
        (*cent).lerpOrigin[0] -= (*wi).weaponMidpoint[0] * ent.axis[0][0]
            + (*wi).weaponMidpoint[1] * ent.axis[1][0]
            + (*wi).weaponMidpoint[2] * ent.axis[2][0];
        (*cent).lerpOrigin[1] -= (*wi).weaponMidpoint[0] * ent.axis[0][1]
            + (*wi).weaponMidpoint[1] * ent.axis[1][1]
            + (*wi).weaponMidpoint[2] * ent.axis[2][1];
        (*cent).lerpOrigin[2] -= (*wi).weaponMidpoint[0] * ent.axis[0][2]
            + (*wi).weaponMidpoint[1] * ent.axis[1][2]
            + (*wi).weaponMidpoint[2] * ent.axis[2][2];
        (*cent).lerpOrigin[2] += 8f32
        // an extra height boost
    }
    if (*item).giType == crate::bg_public_h::IT_WEAPON
        && (*item).giTag == crate::bg_public_h::WP_RAILGUN as i32
    {
        let mut ci: *mut crate::cg_local_h::clientInfo_t = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*crate::src::cgame::cg_main::cg.snap).ps.clientNum as isize)
            as *mut crate::cg_local_h::clientInfo_t;
        ent.shaderRGBA[0] = (*ci).c1RGBA[0];
        ent.shaderRGBA[1] = (*ci).c1RGBA[1];
        ent.shaderRGBA[2] = (*ci).c1RGBA[2];
        ent.shaderRGBA[3] = (*ci).c1RGBA[3]
    }
    ent.hModel = crate::src::cgame::cg_main::cg_items[(*es).modelindex as usize].models[0];
    ent.origin[0] = (*cent).lerpOrigin[0];
    ent.origin[1] = (*cent).lerpOrigin[1];
    ent.origin[2] = (*cent).lerpOrigin[2];
    ent.oldorigin[0] = (*cent).lerpOrigin[0];
    ent.oldorigin[1] = (*cent).lerpOrigin[1];
    ent.oldorigin[2] = (*cent).lerpOrigin[2];
    ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qfalse;
    // if just respawned, slowly scale up
    msec = crate::src::cgame::cg_main::cg.time - (*cent).miscTime;
    if msec >= 0 && msec < 1000 {
        frac = msec as f32 / 1000f32;
        ent.axis[0][0] = ent.axis[0][0] * frac;
        ent.axis[0][1] = ent.axis[0][1] * frac;
        ent.axis[0][2] = ent.axis[0][2] * frac;
        ent.axis[1][0] = ent.axis[1][0] * frac;
        ent.axis[1][1] = ent.axis[1][1] * frac;
        ent.axis[1][2] = ent.axis[1][2] * frac;
        ent.axis[2][0] = ent.axis[2][0] * frac;
        ent.axis[2][1] = ent.axis[2][1] * frac;
        ent.axis[2][2] = ent.axis[2][2] * frac;
        ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qtrue
    } else {
        frac = 1f32
    }
    // items without glow textures need to keep a minimum light value
    // so they are always visible
    if (*item).giType == crate::bg_public_h::IT_WEAPON
        || (*item).giType == crate::bg_public_h::IT_ARMOR
    {
        ent.renderfx |= 0x1
    }
    // increase the size of the weapons when they are presented as items
    if (*item).giType == crate::bg_public_h::IT_WEAPON {
        ent.axis[0][0] = (ent.axis[0][0] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[0][1] = (ent.axis[0][1] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[0][2] = (ent.axis[0][2] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[1][0] = (ent.axis[1][0] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[1][1] = (ent.axis[1][1] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[1][2] = (ent.axis[1][2] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[2][0] = (ent.axis[2][0] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[2][1] = (ent.axis[2][1] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.axis[2][2] = (ent.axis[2][2] as f64 * 1.5) as crate::src::qcommon::q_shared::vec_t;
        ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qtrue
    }
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
    if (*item).giType == crate::bg_public_h::IT_WEAPON && !wi.is_null() && (*wi).barrelModel != 0 {
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
        let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        crate::stdlib::memset(
            &mut barrel as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
        );
        barrel.hModel = (*wi).barrelModel;
        barrel.lightingOrigin[0] = ent.lightingOrigin[0];
        barrel.lightingOrigin[1] = ent.lightingOrigin[1];
        barrel.lightingOrigin[2] = ent.lightingOrigin[2];
        barrel.shadowPlane = ent.shadowPlane;
        barrel.renderfx = ent.renderfx;
        angles[1] = 0f32;
        angles[0] = 0f32;
        angles[2] = 0f32;
        crate::src::qcommon::q_math::AnglesToAxis(
            angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            barrel.axis.as_mut_ptr(),
        );
        CG_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut ent,
            (*wi).weaponModel,
            b"tag_barrel\x00" as *const u8 as *mut i8,
        );
        barrel.nonNormalizedAxes = ent.nonNormalizedAxes;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut barrel);
    }
    // accompanying rings / spheres for powerups
    if crate::src::cgame::cg_main::cg_simpleItems.integer == 0 {
        let mut spinAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        spinAngles[2] = 0f32;
        spinAngles[1] = spinAngles[2];
        spinAngles[0] = spinAngles[1];
        if (*item).giType == crate::bg_public_h::IT_HEALTH
            || (*item).giType == crate::bg_public_h::IT_POWERUP
        {
            ent.hModel = crate::src::cgame::cg_main::cg_items[(*es).modelindex as usize].models[1];
            if ent.hModel != 0 {
                if (*item).giType == crate::bg_public_h::IT_POWERUP {
                    ent.origin[2] += 12f32;
                    spinAngles[1] =
                        ((crate::src::cgame::cg_main::cg.time & 1023) * 360) as f32 / -1024.0
                }
                crate::src::qcommon::q_math::AnglesToAxis(
                    spinAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    ent.axis.as_mut_ptr(),
                );
                // scale up if respawning
                if frac as f64 != 1.0 {
                    ent.axis[0][0] = ent.axis[0][0] * frac;
                    ent.axis[0][1] = ent.axis[0][1] * frac;
                    ent.axis[0][2] = ent.axis[0][2] * frac;
                    ent.axis[1][0] = ent.axis[1][0] * frac;
                    ent.axis[1][1] = ent.axis[1][1] * frac;
                    ent.axis[1][2] = ent.axis[1][2] * frac;
                    ent.axis[2][0] = ent.axis[2][0] * frac;
                    ent.axis[2][1] = ent.axis[2][1] * frac;
                    ent.axis[2][2] = ent.axis[2][2] * frac;
                    ent.nonNormalizedAxes = crate::src::qcommon::q_shared::qtrue
                }
                crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
            }
        }
    };
}
//============================================================================
/*
===============
CG_Missile
===============
*/

unsafe extern "C" fn CG_Missile(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut weapon: *const crate::cg_local_h::weaponInfo_t =
        0 as *const crate::cg_local_h::weaponInfo_t;
    //	int	col;
    s1 = &mut (*cent).currentState;
    if (*s1).weapon >= crate::bg_public_h::WP_NUM_WEAPONS as i32 {
        (*s1).weapon = 0
    }
    weapon = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset((*s1).weapon as isize) as *mut crate::cg_local_h::weaponInfo_t;
    // calculate the axis
    (*cent).lerpAngles[0] = (*s1).angles[0];
    (*cent).lerpAngles[1] = (*s1).angles[1];
    (*cent).lerpAngles[2] = (*s1).angles[2];
    // add trails
    if (*weapon).missileTrailFunc.is_some() {
        (*weapon)
            .missileTrailFunc
            .expect("non-null function pointer")(cent, weapon);
    }
    /*
        if ( cent->currentState.modelindex == TEAM_RED ) {
            col = 1;
        }
        else if ( cent->currentState.modelindex == TEAM_BLUE ) {
            col = 2;
        }
        else {
            col = 0;
        }

        // add dynamic light
        if ( weapon->missileDlight ) {
            trap_R_AddLightToScene(cent->lerpOrigin, weapon->missileDlight,
                weapon->missileDlightColor[col][0], weapon->missileDlightColor[col][1], weapon->missileDlightColor[col][2] );
        }
    */
    // add dynamic light
    if (*weapon).missileDlight != 0. {
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*weapon).missileDlight,
            (*weapon).missileDlightColor[0usize],
            (*weapon).missileDlightColor[1usize],
            (*weapon).missileDlightColor[2usize],
        );
    }
    // add missile sound
    if (*weapon).missileSound != 0 {
        let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        crate::src::game::bg_misc::BG_EvaluateTrajectoryDelta(
            &mut (*cent).currentState.pos,
            crate::src::cgame::cg_main::cg.time,
            velocity.as_mut_ptr(),
        );
        crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            velocity.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*weapon).missileSound,
        );
    }
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = (*cent).lerpOrigin[0];
    ent.origin[1] = (*cent).lerpOrigin[1];
    ent.origin[2] = (*cent).lerpOrigin[2];
    ent.oldorigin[0] = (*cent).lerpOrigin[0];
    ent.oldorigin[1] = (*cent).lerpOrigin[1];
    ent.oldorigin[2] = (*cent).lerpOrigin[2];
    if (*cent).currentState.weapon == crate::bg_public_h::WP_PLASMAGUN as i32 {
        ent.reType = crate::tr_types_h::RT_SPRITE;
        ent.radius = 16f32;
        ent.rotation = 0f32;
        ent.customShader = crate::src::cgame::cg_main::cgs.media.plasmaBallShader;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
        return;
    }
    // flicker between two skins
    ent.skinNum = crate::src::cgame::cg_main::cg.clientFrame & 1;
    ent.hModel = (*weapon).missileModel;
    ent.renderfx = (*weapon).missileRenderfx | 0x40;
    // convert direction of travel into axis
    if crate::src::qcommon::q_math::VectorNormalize2(
        (*s1).pos.trDelta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[0].as_mut_ptr(),
    ) == 0f32
    {
        ent.axis[0][2] = 1f32
    }
    // spin as it moves
    if (*s1).pos.trType != crate::src::qcommon::q_shared::TR_STATIONARY {
        crate::src::qcommon::q_math::RotateAroundDirection(
            ent.axis.as_mut_ptr(),
            (crate::src::cgame::cg_main::cg.time / 4i32) as f32,
        );
    } else {
        crate::src::qcommon::q_math::RotateAroundDirection(
            ent.axis.as_mut_ptr(),
            (*s1).time as f32,
        );
    }
    // add to refresh list, possibly with quad glow
    crate::src::cgame::cg_players::CG_AddRefEntityWithPowerups(
        &mut ent,
        s1,
        crate::bg_public_h::TEAM_FREE as i32,
    );
}
/*
===============
CG_Grapple

This is called when the grapple is sitting up against the wall
===============
*/

unsafe extern "C" fn CG_Grapple(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut weapon: *const crate::cg_local_h::weaponInfo_t =
        0 as *const crate::cg_local_h::weaponInfo_t;
    s1 = &mut (*cent).currentState;
    if (*s1).weapon >= crate::bg_public_h::WP_NUM_WEAPONS as i32 {
        (*s1).weapon = 0
    }
    weapon = &mut *crate::src::cgame::cg_main::cg_weapons
        .as_mut_ptr()
        .offset((*s1).weapon as isize) as *mut crate::cg_local_h::weaponInfo_t;
    // calculate the axis
    (*cent).lerpAngles[0] = (*s1).angles[0];
    (*cent).lerpAngles[1] = (*s1).angles[1];
    (*cent).lerpAngles[2] = (*s1).angles[2];
    // FIXME add grapple pull sound here..?
    // Will draw cable if needed
    crate::src::cgame::cg_weapons::CG_GrappleTrail(cent, weapon);
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = (*cent).lerpOrigin[0];
    ent.origin[1] = (*cent).lerpOrigin[1];
    ent.origin[2] = (*cent).lerpOrigin[2];
    ent.oldorigin[0] = (*cent).lerpOrigin[0];
    ent.oldorigin[1] = (*cent).lerpOrigin[1];
    ent.oldorigin[2] = (*cent).lerpOrigin[2];
    // flicker between two skins
    ent.skinNum = crate::src::cgame::cg_main::cg.clientFrame & 1;
    ent.hModel = (*weapon).missileModel;
    ent.renderfx = (*weapon).missileRenderfx | 0x40;
    // convert direction of travel into axis
    if crate::src::qcommon::q_math::VectorNormalize2(
        (*s1).pos.trDelta.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[0].as_mut_ptr(),
    ) == 0f32
    {
        ent.axis[0][2] = 1f32
    }
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_Mover
===============
*/

unsafe extern "C" fn CG_Mover(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = (*cent).lerpOrigin[0];
    ent.origin[1] = (*cent).lerpOrigin[1];
    ent.origin[2] = (*cent).lerpOrigin[2];
    ent.oldorigin[0] = (*cent).lerpOrigin[0];
    ent.oldorigin[1] = (*cent).lerpOrigin[1];
    ent.oldorigin[2] = (*cent).lerpOrigin[2];
    crate::src::qcommon::q_math::AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis.as_mut_ptr(),
    );
    ent.renderfx = 0x40;
    // flicker between two skins (FIXME?)
    ent.skinNum = crate::src::cgame::cg_main::cg.time >> 6 & 1;
    // get the model, either as a bmodel or a modelindex
    if (*s1).solid == 0xffffff {
        ent.hModel = crate::src::cgame::cg_main::cgs.inlineDrawModel[(*s1).modelindex as usize]
    } else {
        ent.hModel = crate::src::cgame::cg_main::cgs.gameModels[(*s1).modelindex as usize]
    }
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
    // add the secondary model
    if (*s1).modelindex2 != 0 {
        ent.skinNum = 0;
        ent.hModel = crate::src::cgame::cg_main::cgs.gameModels[(*s1).modelindex2 as usize];
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
    };
}
/*
===============
CG_Beam

Also called as an event
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Beam(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = (*s1).pos.trBase[0];
    ent.origin[1] = (*s1).pos.trBase[1];
    ent.origin[2] = (*s1).pos.trBase[2];
    ent.oldorigin[0] = (*s1).origin2[0];
    ent.oldorigin[1] = (*s1).origin2[1];
    ent.oldorigin[2] = (*s1).origin2[2];
    crate::src::qcommon::q_math::AxisClear(ent.axis.as_mut_ptr());
    ent.reType = crate::tr_types_h::RT_BEAM;
    ent.renderfx = 0x40;
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_Portal
===============
*/

unsafe extern "C" fn CG_Portal(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ent: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut s1: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    s1 = &mut (*cent).currentState;
    // create the render entity
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = (*cent).lerpOrigin[0];
    ent.origin[1] = (*cent).lerpOrigin[1];
    ent.origin[2] = (*cent).lerpOrigin[2];
    ent.oldorigin[0] = (*s1).origin2[0];
    ent.oldorigin[1] = (*s1).origin2[1];
    ent.oldorigin[2] = (*s1).origin2[2];
    crate::src::qcommon::q_math::ByteToDir((*s1).eventParm, ent.axis[0].as_mut_ptr());
    crate::src::qcommon::q_math::PerpendicularVector(
        ent.axis[1].as_mut_ptr(),
        ent.axis[0].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    );
    // negating this tends to get the directions like they want
    // we really should have a camera roll value
    ent.axis[1][0] = crate::src::qcommon::q_math::vec3_origin[0] - ent.axis[1][0]; // rotation speed
    ent.axis[1][1] = crate::src::qcommon::q_math::vec3_origin[1] - ent.axis[1][1]; // roll offset
    ent.axis[1][2] = crate::src::qcommon::q_math::vec3_origin[2] - ent.axis[1][2];
    CrossProduct(
        ent.axis[0].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[1].as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis[2].as_mut_ptr(),
    );
    ent.reType = crate::tr_types_h::RT_PORTALSURFACE;
    ent.oldframe = (*s1).powerups;
    ent.frame = (*s1).frame;
    ent.skinNum = ((*s1).clientNum as f64 / 256.0 * 360f64) as i32;
    // add to refresh list
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
================
CG_CreateRotationMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CreateRotationMatrix(
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    crate::src::qcommon::q_math::AngleVectors(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        (*matrix.offset(0)).as_mut_ptr(),
        (*matrix.offset(1)).as_mut_ptr(),
        (*matrix.offset(2)).as_mut_ptr(),
    );
    VectorInverse((*matrix.offset(1)).as_mut_ptr());
}
/*
================
CG_TransposeMatrix
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_TransposeMatrix(
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
    mut transpose: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 0;
    while i < 3 {
        for j in 0..3 {
            (*transpose.offset(i as isize))[j as usize] = (*matrix.offset(j as isize))[i as usize];
        }
        i += 1
    }
}
/*
================
CG_RotatePoint
================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_RotatePoint(
    mut point: *mut crate::src::qcommon::q_shared::vec_t,
    mut matrix: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut tvec: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    tvec[0] = *point.offset(0);
    tvec[1] = *point.offset(1);
    tvec[2] = *point.offset(2);
    *point.offset(0) = (*matrix.offset(0))[0] * tvec[0]
        + (*matrix.offset(0))[1] * tvec[1]
        + (*matrix.offset(0))[2] * tvec[2];
    *point.offset(1) = (*matrix.offset(1))[0] * tvec[0]
        + (*matrix.offset(1))[1] * tvec[1]
        + (*matrix.offset(1))[2] * tvec[2];
    *point.offset(2) = (*matrix.offset(2))[0] * tvec[0]
        + (*matrix.offset(2))[1] * tvec[1]
        + (*matrix.offset(2))[2] * tvec[2];
}
/*
=========================
CG_AdjustPositionForMover

Also called by client movement prediction code
=========================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AdjustPositionForMover(
    mut in_0: *const crate::src::qcommon::q_shared::vec_t,
    mut moverNum: i32,
    mut fromTime: i32,
    mut toTime: i32,
    mut out: *mut crate::src::qcommon::q_shared::vec_t,
    mut angles_in: *mut crate::src::qcommon::q_shared::vec_t,
    mut angles_out: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut oldOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut deltaOrigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut oldAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut deltaAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut matrix: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut transpose: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    let mut org: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut org2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut move2: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if moverNum <= 0 || moverNum >= ((1) << 10) - 2 {
        *out.offset(0) = *in_0.offset(0);
        *out.offset(1) = *in_0.offset(1);
        *out.offset(2) = *in_0.offset(2);
        *angles_out.offset(0) = *angles_in.offset(0);
        *angles_out.offset(1) = *angles_in.offset(1);
        *angles_out.offset(2) = *angles_in.offset(2);
        return;
    }
    cent = &mut *crate::src::cgame::cg_main::cg_entities
        .as_mut_ptr()
        .offset(moverNum as isize) as *mut crate::cg_local_h::centity_t;
    if (*cent).currentState.eType != crate::bg_public_h::ET_MOVER as i32 {
        *out.offset(0) = *in_0.offset(0);
        *out.offset(1) = *in_0.offset(1);
        *out.offset(2) = *in_0.offset(2);
        *angles_out.offset(0) = *angles_in.offset(0);
        *angles_out.offset(1) = *angles_in.offset(1);
        *angles_out.offset(2) = *angles_in.offset(2);
        return;
    }
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        fromTime,
        oldOrigin.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos,
        fromTime,
        oldAngles.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        toTime,
        origin.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos,
        toTime,
        angles.as_mut_ptr(),
    );
    deltaOrigin[0] = origin[0] - oldOrigin[0];
    deltaOrigin[1] = origin[1] - oldOrigin[1];
    deltaOrigin[2] = origin[2] - oldOrigin[2];
    deltaAngles[0] = angles[0] - oldAngles[0];
    deltaAngles[1] = angles[1] - oldAngles[1];
    deltaAngles[2] = angles[2] - oldAngles[2];
    // origin change when on a rotating object
    CG_CreateRotationMatrix(deltaAngles.as_mut_ptr(), transpose.as_mut_ptr());
    CG_TransposeMatrix(transpose.as_mut_ptr(), matrix.as_mut_ptr());
    org[0] = *in_0.offset(0) - oldOrigin[0];
    org[1] = *in_0.offset(1) - oldOrigin[1];
    org[2] = *in_0.offset(2) - oldOrigin[2];
    org2[0] = org[0];
    org2[1] = org[1];
    org2[2] = org[2];
    CG_RotatePoint(org2.as_mut_ptr(), matrix.as_mut_ptr());
    move2[0] = org2[0] - org[0];
    move2[1] = org2[1] - org[1];
    move2[2] = org2[2] - org[2];
    deltaOrigin[0] = deltaOrigin[0] + move2[0];
    deltaOrigin[1] = deltaOrigin[1] + move2[1];
    deltaOrigin[2] = deltaOrigin[2] + move2[2];
    *out.offset(0) = *in_0.offset(0) + deltaOrigin[0];
    *out.offset(1) = *in_0.offset(1) + deltaOrigin[1];
    *out.offset(2) = *in_0.offset(2) + deltaOrigin[2];
    *angles_out.offset(0) = *angles_in.offset(0) + deltaAngles[0];
    *angles_out.offset(1) = *angles_in.offset(1) + deltaAngles[1];
    *angles_out.offset(2) = *angles_in.offset(2) + deltaAngles[2];
}
/*
=============================
CG_InterpolateEntityPosition
=============================
*/

unsafe extern "C" fn CG_InterpolateEntityPosition(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut current: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut next: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut f: f32 = 0.;
    // it would be an internal error to find an entity that interpolates without
    // a snapshot ahead of the current one
    if crate::src::cgame::cg_main::cg.nextSnap.is_null() {
        crate::src::cgame::cg_main::CG_Error(
            b"CG_InterpoateEntityPosition: cg.nextSnap == NULL\x00" as *const u8 as *const i8,
        );
    }
    f = crate::src::cgame::cg_main::cg.frameInterpolation;
    // this will linearize a sine or parabolic curve, but it is important
    // to not extrapolate player positions if more recent data is available
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        (*crate::src::cgame::cg_main::cg.snap).serverTime,
        current.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).nextState.pos,
        (*crate::src::cgame::cg_main::cg.nextSnap).serverTime,
        next.as_mut_ptr(),
    );
    (*cent).lerpOrigin[0] = current[0] + f * (next[0] - current[0]);
    (*cent).lerpOrigin[1] = current[1] + f * (next[1] - current[1]);
    (*cent).lerpOrigin[2] = current[2] + f * (next[2] - current[2]);
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos,
        (*crate::src::cgame::cg_main::cg.snap).serverTime,
        current.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).nextState.apos,
        (*crate::src::cgame::cg_main::cg.nextSnap).serverTime,
        next.as_mut_ptr(),
    );
    (*cent).lerpAngles[0] = crate::src::qcommon::q_math::LerpAngle(current[0], next[0], f);
    (*cent).lerpAngles[1] = crate::src::qcommon::q_math::LerpAngle(current[1], next[1], f);
    (*cent).lerpAngles[2] = crate::src::qcommon::q_math::LerpAngle(current[2], next[2], f);
}
/*
===============
CG_CalcEntityLerpPositions

===============
*/

unsafe extern "C" fn CG_CalcEntityLerpPositions(mut cent: *mut crate::cg_local_h::centity_t) {
    // if this player does not want to see extrapolated players
    if crate::src::cgame::cg_main::cg_smoothClients.integer == 0 {
        // make sure the clients use TR_INTERPOLATE
        if (*cent).currentState.number < 64 {
            (*cent).currentState.pos.trType = crate::src::qcommon::q_shared::TR_INTERPOLATE;
            (*cent).nextState.pos.trType = crate::src::qcommon::q_shared::TR_INTERPOLATE
        }
    }
    if (*cent).interpolate != 0
        && (*cent).currentState.pos.trType == crate::src::qcommon::q_shared::TR_INTERPOLATE
    {
        CG_InterpolateEntityPosition(cent);
        return;
    }
    // first see if we can interpolate between two snaps for
    // linear extrapolated clients
    if (*cent).interpolate != 0
        && (*cent).currentState.pos.trType == crate::src::qcommon::q_shared::TR_LINEAR_STOP
        && (*cent).currentState.number < 64
    {
        CG_InterpolateEntityPosition(cent);
        return;
    }
    // just use the current frame and evaluate as best we can
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        crate::src::cgame::cg_main::cg.time,
        (*cent).lerpOrigin.as_mut_ptr(),
    );
    crate::src::game::bg_misc::BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos,
        crate::src::cgame::cg_main::cg.time,
        (*cent).lerpAngles.as_mut_ptr(),
    );
    // adjust for riding a mover if it wasn't rolled into the predicted
    // player state
    if cent
        != &mut crate::src::cgame::cg_main::cg.predictedPlayerEntity
            as *mut crate::cg_local_h::centity_t
    {
        CG_AdjustPositionForMover(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*cent).currentState.groundEntityNum,
            (*crate::src::cgame::cg_main::cg.snap).serverTime,
            crate::src::cgame::cg_main::cg.time,
            (*cent).lerpOrigin.as_mut_ptr(),
            (*cent).lerpAngles.as_mut_ptr(),
            (*cent).lerpAngles.as_mut_ptr(),
        );
    };
}
/*
===============
CG_TeamBase
===============
*/

unsafe extern "C" fn CG_TeamBase(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut model: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    if crate::src::cgame::cg_main::cgs.gametype == crate::bg_public_h::GT_CTF {
        // show the flag base
        crate::stdlib::memset(
            &mut model as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
        );
        model.reType = crate::tr_types_h::RT_MODEL;
        model.lightingOrigin[0] = (*cent).lerpOrigin[0];
        model.lightingOrigin[1] = (*cent).lerpOrigin[1];
        model.lightingOrigin[2] = (*cent).lerpOrigin[2];
        model.origin[0] = (*cent).lerpOrigin[0];
        model.origin[1] = (*cent).lerpOrigin[1];
        model.origin[2] = (*cent).lerpOrigin[2];
        crate::src::qcommon::q_math::AnglesToAxis(
            (*cent).currentState.angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            model.axis.as_mut_ptr(),
        );
        if (*cent).currentState.modelindex == crate::bg_public_h::TEAM_RED as i32 {
            model.hModel = crate::src::cgame::cg_main::cgs.media.redFlagBaseModel
        } else if (*cent).currentState.modelindex == crate::bg_public_h::TEAM_BLUE as i32 {
            model.hModel = crate::src::cgame::cg_main::cgs.media.blueFlagBaseModel
        } else {
            model.hModel = crate::src::cgame::cg_main::cgs.media.neutralFlagBaseModel
        }
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut model);
    };
}
/*
===============
CG_AddCEntity

===============
*/

unsafe extern "C" fn CG_AddCEntity(mut cent: *mut crate::cg_local_h::centity_t) {
    // event-only entities will have been dealt with already
    if (*cent).currentState.eType >= crate::bg_public_h::ET_EVENTS as i32 {
        return;
    }
    // calculate the current origin
    CG_CalcEntityLerpPositions(cent);
    // add automatic effects
    CG_EntityEffects(cent);
    match (*cent).currentState.eType {
        10 | 8 | 9 => {}
        0 => {
            CG_General(cent);
        }
        1 => {
            crate::src::cgame::cg_players::CG_Player(cent);
        }
        2 => {
            CG_Item(cent);
        }
        3 => {
            CG_Missile(cent);
        }
        4 => {
            CG_Mover(cent);
        }
        5 => {
            CG_Beam(cent);
        }
        6 => {
            CG_Portal(cent);
        }
        7 => {
            CG_Speaker(cent);
        }
        11 => {
            CG_Grapple(cent);
        }
        12 => {
            CG_TeamBase(cent);
        }
        _ => {
            crate::src::cgame::cg_main::CG_Error(
                b"Bad entity type: %i\x00" as *const u8 as *const i8,
                (*cent).currentState.eType,
            );
        }
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
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
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
/*
===============
CG_AddPacketEntities

===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddPacketEntities() {
    let mut num: i32 = 0;
    let mut cent: *mut crate::cg_local_h::centity_t = 0 as *mut crate::cg_local_h::centity_t;
    let mut ps: *mut crate::src::qcommon::q_shared::playerState_t =
        0 as *mut crate::src::qcommon::q_shared::playerState_t;
    // set cg.frameInterpolation
    if !crate::src::cgame::cg_main::cg.nextSnap.is_null() {
        let mut delta: i32 = 0;
        delta = (*crate::src::cgame::cg_main::cg.nextSnap).serverTime
            - (*crate::src::cgame::cg_main::cg.snap).serverTime;
        if delta == 0 {
            crate::src::cgame::cg_main::cg.frameInterpolation = 0f32
        } else {
            crate::src::cgame::cg_main::cg.frameInterpolation =
                (crate::src::cgame::cg_main::cg.time
                    - (*crate::src::cgame::cg_main::cg.snap).serverTime) as f32
                    / delta as f32
        }
    } else {
        crate::src::cgame::cg_main::cg.frameInterpolation = 0f32
        // actually, it should never be used, because
        // no entities should be marked as interpolating
    }
    // the auto-rotating items will all have the same axis
    crate::src::cgame::cg_main::cg.autoAngles[0] = 0f32;
    crate::src::cgame::cg_main::cg.autoAngles[1] =
        (((crate::src::cgame::cg_main::cg.time & 2047) * 360) as f64 / 2048.0)
            as crate::src::qcommon::q_shared::vec_t;
    crate::src::cgame::cg_main::cg.autoAngles[2] = 0f32;
    crate::src::cgame::cg_main::cg.autoAnglesFast[0] = 0f32;
    crate::src::cgame::cg_main::cg.autoAnglesFast[1] =
        ((crate::src::cgame::cg_main::cg.time & 1023) * 360) as f32 / 1024.0;
    crate::src::cgame::cg_main::cg.autoAnglesFast[2] = 0f32;
    crate::src::qcommon::q_math::AnglesToAxis(
        crate::src::cgame::cg_main::cg.autoAngles.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg.autoAxis.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AnglesToAxis(
        crate::src::cgame::cg_main::cg.autoAnglesFast.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::cgame::cg_main::cg.autoAxisFast.as_mut_ptr(),
    );
    // generate and add the entity from the playerstate
    ps = &mut crate::src::cgame::cg_main::cg.predictedPlayerState;
    crate::src::game::bg_misc::BG_PlayerStateToEntityState(
        ps,
        &mut crate::src::cgame::cg_main::cg
            .predictedPlayerEntity
            .currentState,
        crate::src::qcommon::q_shared::qfalse,
    );
    CG_AddCEntity(&mut crate::src::cgame::cg_main::cg.predictedPlayerEntity);
    // lerp the non-predicted value for lightning gun origins
    CG_CalcEntityLerpPositions(
        &mut *crate::src::cgame::cg_main::cg_entities
            .as_mut_ptr()
            .offset((*crate::src::cgame::cg_main::cg.snap).ps.clientNum as isize),
    );
    // add each entity sent over by the server
    num = 0;
    while num < (*crate::src::cgame::cg_main::cg.snap).numEntities {
        cent = &mut *crate::src::cgame::cg_main::cg_entities.as_mut_ptr().offset(
            (*(*crate::src::cgame::cg_main::cg.snap)
                .entities
                .as_mut_ptr()
                .offset(num as isize))
            .number as isize,
        ) as *mut crate::cg_local_h::centity_t;
        CG_AddCEntity(cent);
        num += 1
    }
}
