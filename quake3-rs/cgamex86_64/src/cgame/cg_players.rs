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

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::bg_public_h::gametype_t;
pub use crate::bg_public_h::gender_t;
pub use crate::bg_public_h::team_t;
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
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::cgame::cg_main::Com_Printf;
pub use crate::src::game::bg_misc::BG_EvaluateTrajectory;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::AngleMod;
pub use crate::src::qcommon::q_math::AngleSubtract;
pub use crate::src::qcommon::q_math::AnglesSubtract;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::VectorNormalize;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
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
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
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
pub use crate::cg_local_h::leBounceSoundType_t;
pub use crate::cg_local_h::leMarkType_t;
pub use crate::cg_local_h::leType_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::localEntity_s;
pub use crate::cg_local_h::localEntity_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::cg_local_h::FOOTSTEP_BOOT;
pub use crate::cg_local_h::FOOTSTEP_ENERGY;
pub use crate::cg_local_h::FOOTSTEP_FLESH;
pub use crate::cg_local_h::FOOTSTEP_MECH;
pub use crate::cg_local_h::FOOTSTEP_METAL;
pub use crate::cg_local_h::FOOTSTEP_NORMAL;
pub use crate::cg_local_h::FOOTSTEP_SPLASH;
pub use crate::cg_local_h::FOOTSTEP_TOTAL;
pub use crate::cg_local_h::LEBS_BLOOD;
pub use crate::cg_local_h::LEBS_BRASS;
pub use crate::cg_local_h::LEBS_NONE;
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
pub use crate::src::cgame::cg_effects::CG_SmokePuff;
pub use crate::src::cgame::cg_ents::CG_PositionEntityOnTag;
pub use crate::src::cgame::cg_ents::CG_PositionRotatedEntityOnTag;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_animSpeed;
pub use crate::src::cgame::cg_main::cg_buildScript;
pub use crate::src::cgame::cg_main::cg_cameraMode;
pub use crate::src::cgame::cg_main::cg_debugAnim;
pub use crate::src::cgame::cg_main::cg_debugPosition;
pub use crate::src::cgame::cg_main::cg_deferPlayers;
pub use crate::src::cgame::cg_main::cg_drawFriend;
pub use crate::src::cgame::cg_main::cg_entities;
pub use crate::src::cgame::cg_main::cg_forceModel;
pub use crate::src::cgame::cg_main::cg_noPlayerAnims;
pub use crate::src::cgame::cg_main::cg_shadows;
pub use crate::src::cgame::cg_main::cg_swingSpeed;
pub use crate::src::cgame::cg_main::cgs;
pub use crate::src::cgame::cg_main::CG_ConfigString;
pub use crate::src::cgame::cg_main::CG_Error;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_marks::CG_ImpactMark;
pub use crate::src::cgame::cg_players::stdlib_float_h::atof;
pub use crate::src::cgame::cg_players::stdlib_h::atoi;
pub use crate::src::cgame::cg_predict::CG_PointContents;
pub use crate::src::cgame::cg_syscalls::trap_CM_BoxTrace;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::cgame::cg_syscalls::trap_FS_FCloseFile;
pub use crate::src::cgame::cg_syscalls::trap_FS_FOpenFile;
pub use crate::src::cgame::cg_syscalls::trap_FS_Read;
pub use crate::src::cgame::cg_syscalls::trap_MemoryRemaining;
pub use crate::src::cgame::cg_syscalls::trap_R_AddLightToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::cgame::cg_syscalls::trap_R_LerpTag;
pub use crate::src::cgame::cg_syscalls::trap_R_LightForPoint;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterModel;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::cgame::cg_syscalls::trap_R_RegisterSkin;
pub use crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound;
pub use crate::src::cgame::cg_syscalls::trap_S_RegisterSound;
pub use crate::src::cgame::cg_weapons::CG_AddPlayerWeapon;
use crate::stdlib::acos;
use crate::stdlib::fabs;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use crate::stdlib::rand;
use crate::stdlib::strchr;
use crate::stdlib::strcmp;
pub use crate::stdlib::strtod;
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
// cg_players.c -- handle the media and animation for player entities
#[no_mangle]

pub static mut cg_customSoundNames: [*mut i8; 32] = [
    b"*death1.wav\x00" as *const u8 as *mut i8,
    b"*death2.wav\x00" as *const u8 as *mut i8,
    b"*death3.wav\x00" as *const u8 as *mut i8,
    b"*jump1.wav\x00" as *const u8 as *mut i8,
    b"*pain25_1.wav\x00" as *const u8 as *mut i8,
    b"*pain50_1.wav\x00" as *const u8 as *mut i8,
    b"*pain75_1.wav\x00" as *const u8 as *mut i8,
    b"*pain100_1.wav\x00" as *const u8 as *mut i8,
    b"*falling1.wav\x00" as *const u8 as *mut i8,
    b"*gasp.wav\x00" as *const u8 as *mut i8,
    b"*drown.wav\x00" as *const u8 as *mut i8,
    b"*fall1.wav\x00" as *const u8 as *mut i8,
    b"*taunt.wav\x00" as *const u8 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
    0 as *mut i8,
];
/*
================
CG_CustomSound

================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_CustomSound(
    mut clientNum: i32,
    mut soundName: *const i8,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut i: i32 = 0;
    if *soundName.offset(0) as i32 != '*' as i32 {
        return crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
            soundName,
            crate::src::qcommon::q_shared::qfalse,
        );
    }
    if clientNum < 0 || clientNum >= 64 {
        clientNum = 0
    }
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::cg_local_h::clientInfo_t;
    i = 0;
    while i < 32 && !cg_customSoundNames[i as usize].is_null() {
        if crate::stdlib::strcmp(soundName, cg_customSoundNames[i as usize]) == 0 {
            return (*ci).sounds[i as usize];
        }
        i += 1
    }
    crate::src::cgame::cg_main::CG_Error(
        b"Unknown custom sound: %s\x00" as *const u8 as *const i8,
        soundName,
    );
}
/*
=============================================================================

CLIENT INFO

=============================================================================
*/
/*
======================
CG_ParseAnimationFile

Read a configuration file containing animation counts and rates
models/players/visor/animation.cfg, etc
======================
*/

unsafe extern "C" fn CG_ParseAnimationFile(
    mut filename: *const i8,
    mut ci: *mut crate::cg_local_h::clientInfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut text_p: *mut i8 = 0 as *mut i8;
    let mut prev: *mut i8 = 0 as *mut i8;
    let mut len: i32 = 0;
    let mut i: i32 = 0;
    let mut token: *mut i8 = 0 as *mut i8;
    let mut fps: f32 = 0.;
    let mut skip: i32 = 0;
    let mut text: [i8; 20000] = [0; 20000];
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut animations: *mut crate::bg_public_h::animation_t =
        0 as *mut crate::bg_public_h::animation_t;
    animations = (*ci).animations.as_mut_ptr();
    // load the file
    len = crate::src::cgame::cg_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if len <= 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if len as usize >= (::std::mem::size_of::<[i8; 20000]>()).wrapping_sub(1usize) {
        crate::src::cgame::cg_main::CG_Printf(
            b"File %s too long\n\x00" as *const u8 as *const i8,
            filename,
        );
        crate::src::cgame::cg_syscalls::trap_FS_FCloseFile(f);
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::cgame::cg_syscalls::trap_FS_Read(text.as_mut_ptr() as *mut libc::c_void, len, f);
    text[len as usize] = 0;
    crate::src::cgame::cg_syscalls::trap_FS_FCloseFile(f);
    // parse the text
    text_p = text.as_mut_ptr(); // quite the compiler warning
    skip = 0;
    (*ci).footsteps = crate::cg_local_h::FOOTSTEP_NORMAL;
    (*ci).headOffset[2] = 0f32;
    (*ci).headOffset[1] = (*ci).headOffset[2];
    (*ci).headOffset[0] = (*ci).headOffset[1];
    (*ci).gender = crate::bg_public_h::GENDER_MALE;
    (*ci).fixedlegs = crate::src::qcommon::q_shared::qfalse;
    (*ci).fixedtorso = crate::src::qcommon::q_shared::qfalse;
    loop
    // read optional parameters
    {
        prev = text_p; // so we can unget
        token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
        if *token.offset(0) == 0 {
            break;
        }
        if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"footsteps\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token.offset(0) == 0 {
                break;
            }
            if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"default\x00" as *const u8 as *const i8,
            ) == 0
                || crate::src::qcommon::q_shared::Q_stricmp(
                    token,
                    b"normal\x00" as *const u8 as *const i8,
                ) == 0
            {
                (*ci).footsteps = crate::cg_local_h::FOOTSTEP_NORMAL
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"boot\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*ci).footsteps = crate::cg_local_h::FOOTSTEP_BOOT
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"flesh\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*ci).footsteps = crate::cg_local_h::FOOTSTEP_FLESH
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"mech\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*ci).footsteps = crate::cg_local_h::FOOTSTEP_MECH
            } else if crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"energy\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*ci).footsteps = crate::cg_local_h::FOOTSTEP_ENERGY
            } else {
                crate::src::cgame::cg_main::CG_Printf(
                    b"Bad footsteps parm in %s: %s\n\x00" as *const u8 as *const i8,
                    filename,
                    token,
                );
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"headoffset\x00" as *const u8 as *const i8,
        ) == 0
        {
            i = 0;
            while i < 3 {
                token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
                if *token.offset(0) == 0 {
                    break;
                }
                (*ci).headOffset[i as usize] = atof(token) as crate::src::qcommon::q_shared::vec_t;
                i += 1
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"sex\x00" as *const u8 as *const i8,
        ) == 0
        {
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token.offset(0) == 0 {
                break;
            }
            if *token.offset(0) as i32 == 'f' as i32 || *token.offset(0) as i32 == 'F' as i32 {
                (*ci).gender = crate::bg_public_h::GENDER_FEMALE
            } else if *token.offset(0) as i32 == 'n' as i32 || *token.offset(0) as i32 == 'N' as i32
            {
                (*ci).gender = crate::bg_public_h::GENDER_NEUTER
            } else {
                (*ci).gender = crate::bg_public_h::GENDER_MALE
            }
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"fixedlegs\x00" as *const u8 as *const i8,
        ) == 0
        {
            (*ci).fixedlegs = crate::src::qcommon::q_shared::qtrue
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"fixedtorso\x00" as *const u8 as *const i8,
        ) == 0
        {
            (*ci).fixedtorso = crate::src::qcommon::q_shared::qtrue
        } else if *token.offset(0) as i32 >= '0' as i32 && *token.offset(0) as i32 <= '9' as i32 {
            // if it is a number, start parsing animations
            text_p = prev; // unget the token
            break;
        } else {
            crate::src::cgame::cg_main::Com_Printf(
                b"unknown token \'%s\' in %s\n\x00" as *const u8 as *const i8,
                token,
                filename,
            );
        }
    }
    // read information for each frame
    i = 0;
    while i < crate::bg_public_h::MAX_ANIMATIONS as i32 {
        token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
        if *token.offset(0) == 0 {
            if !(i >= crate::bg_public_h::TORSO_GETFLAG as i32
                && i <= crate::bg_public_h::TORSO_NEGATIVE as i32)
            {
                break;
            }
            (*animations.offset(i as isize)).firstFrame =
                (*animations.offset(crate::bg_public_h::TORSO_GESTURE as i32 as isize)).firstFrame;
            (*animations.offset(i as isize)).frameLerp =
                (*animations.offset(crate::bg_public_h::TORSO_GESTURE as i32 as isize)).frameLerp;
            (*animations.offset(i as isize)).initialLerp =
                (*animations.offset(crate::bg_public_h::TORSO_GESTURE as i32 as isize)).initialLerp;
            (*animations.offset(i as isize)).loopFrames =
                (*animations.offset(crate::bg_public_h::TORSO_GESTURE as i32 as isize)).loopFrames;
            (*animations.offset(i as isize)).numFrames =
                (*animations.offset(crate::bg_public_h::TORSO_GESTURE as i32 as isize)).numFrames;
            (*animations.offset(i as isize)).reversed =
                crate::src::qcommon::q_shared::qfalse as i32;
            (*animations.offset(i as isize)).flipflop = crate::src::qcommon::q_shared::qfalse as i32
        } else {
            (*animations.offset(i as isize)).firstFrame = atoi(token);
            // leg only frames are adjusted to not count the upper body only frames
            if i == crate::bg_public_h::LEGS_WALKCR as i32 {
                skip = (*animations.offset(crate::bg_public_h::LEGS_WALKCR as i32 as isize))
                    .firstFrame
                    - (*animations.offset(crate::bg_public_h::TORSO_GESTURE as i32 as isize))
                        .firstFrame
            }
            if i >= crate::bg_public_h::LEGS_WALKCR as i32
                && i < crate::bg_public_h::TORSO_GETFLAG as i32
            {
                (*animations.offset(i as isize)).firstFrame -= skip
            }
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token.offset(0) == 0 {
                break;
            }
            (*animations.offset(i as isize)).numFrames = atoi(token);
            (*animations.offset(i as isize)).reversed =
                crate::src::qcommon::q_shared::qfalse as i32;
            (*animations.offset(i as isize)).flipflop =
                crate::src::qcommon::q_shared::qfalse as i32;
            // if numFrames is negative the animation is reversed
            if (*animations.offset(i as isize)).numFrames < 0 {
                (*animations.offset(i as isize)).numFrames =
                    -(*animations.offset(i as isize)).numFrames;
                (*animations.offset(i as isize)).reversed =
                    crate::src::qcommon::q_shared::qtrue as i32
            }
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token.offset(0) == 0 {
                break;
            }
            (*animations.offset(i as isize)).loopFrames = atoi(token);
            token = crate::src::qcommon::q_shared::COM_Parse(&mut text_p);
            if *token.offset(0) == 0 {
                break;
            }
            fps = atof(token) as f32;
            if fps == 0f32 {
                fps = 1f32
            }
            (*animations.offset(i as isize)).frameLerp = (1000f32 / fps) as i32;
            (*animations.offset(i as isize)).initialLerp = (1000f32 / fps) as i32
        }
        i += 1
    }
    if i != crate::bg_public_h::MAX_ANIMATIONS as i32 {
        crate::src::cgame::cg_main::CG_Printf(
            b"Error parsing animation file: %s\n\x00" as *const u8 as *const i8,
            filename,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    // crouch backward animation
    crate::stdlib::memcpy(
        &mut *animations.offset(crate::bg_public_h::LEGS_BACKCR as i32 as isize)
            as *mut crate::bg_public_h::animation_t as *mut libc::c_void,
        &mut *animations.offset(crate::bg_public_h::LEGS_WALKCR as i32 as isize)
            as *mut crate::bg_public_h::animation_t as *const libc::c_void,
        ::std::mem::size_of::<crate::bg_public_h::animation_t>(),
    );
    (*animations.offset(crate::bg_public_h::LEGS_BACKCR as i32 as isize)).reversed =
        crate::src::qcommon::q_shared::qtrue as i32;
    // walk backward animation
    crate::stdlib::memcpy(
        &mut *animations.offset(crate::bg_public_h::LEGS_BACKWALK as i32 as isize)
            as *mut crate::bg_public_h::animation_t as *mut libc::c_void,
        &mut *animations.offset(crate::bg_public_h::LEGS_WALK as i32 as isize)
            as *mut crate::bg_public_h::animation_t as *const libc::c_void,
        ::std::mem::size_of::<crate::bg_public_h::animation_t>(),
    );
    (*animations.offset(crate::bg_public_h::LEGS_BACKWALK as i32 as isize)).reversed =
        crate::src::qcommon::q_shared::qtrue as i32;
    // flag moving fast
    (*animations.offset(crate::bg_public_h::FLAG_RUN as i32 as isize)).firstFrame = 0;
    (*animations.offset(crate::bg_public_h::FLAG_RUN as i32 as isize)).numFrames = 16;
    (*animations.offset(crate::bg_public_h::FLAG_RUN as i32 as isize)).loopFrames = 16;
    (*animations.offset(crate::bg_public_h::FLAG_RUN as i32 as isize)).frameLerp = 1000 / 15;
    (*animations.offset(crate::bg_public_h::FLAG_RUN as i32 as isize)).initialLerp = 1000 / 15;
    (*animations.offset(crate::bg_public_h::FLAG_RUN as i32 as isize)).reversed =
        crate::src::qcommon::q_shared::qfalse as i32;
    // flag not moving or moving slowly
    (*animations.offset(crate::bg_public_h::FLAG_STAND as i32 as isize)).firstFrame = 16;
    (*animations.offset(crate::bg_public_h::FLAG_STAND as i32 as isize)).numFrames = 5;
    (*animations.offset(crate::bg_public_h::FLAG_STAND as i32 as isize)).loopFrames = 0;
    (*animations.offset(crate::bg_public_h::FLAG_STAND as i32 as isize)).frameLerp = 1000 / 20;
    (*animations.offset(crate::bg_public_h::FLAG_STAND as i32 as isize)).initialLerp = 1000 / 20;
    (*animations.offset(crate::bg_public_h::FLAG_STAND as i32 as isize)).reversed =
        crate::src::qcommon::q_shared::qfalse as i32;
    // flag speeding up
    (*animations.offset(crate::bg_public_h::FLAG_STAND2RUN as i32 as isize)).firstFrame = 16;
    (*animations.offset(crate::bg_public_h::FLAG_STAND2RUN as i32 as isize)).numFrames = 5;
    (*animations.offset(crate::bg_public_h::FLAG_STAND2RUN as i32 as isize)).loopFrames = 1;
    (*animations.offset(crate::bg_public_h::FLAG_STAND2RUN as i32 as isize)).frameLerp = 1000 / 15;
    (*animations.offset(crate::bg_public_h::FLAG_STAND2RUN as i32 as isize)).initialLerp =
        1000 / 15;
    (*animations.offset(crate::bg_public_h::FLAG_STAND2RUN as i32 as isize)).reversed =
        crate::src::qcommon::q_shared::qtrue as i32;
    //
    // new anims changes
    //
    //	animations[TORSO_GETFLAG].flipflop = qtrue;
    //	animations[TORSO_GUARDBASE].flipflop = qtrue;
    //	animations[TORSO_PATROL].flipflop = qtrue;
    //	animations[TORSO_AFFIRMATIVE].flipflop = qtrue;
    //	animations[TORSO_NEGATIVE].flipflop = qtrue;
    //
    return crate::src::qcommon::q_shared::qtrue;
}
/*
==========================
CG_FileExists
==========================
*/

unsafe extern "C" fn CG_FileExists(
    mut filename: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut len: i32 = 0;
    len = crate::src::cgame::cg_syscalls::trap_FS_FOpenFile(
        filename,
        0 as *mut crate::src::qcommon::q_shared::fileHandle_t,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if len > 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==========================
CG_FindClientModelFile
==========================
*/

unsafe extern "C" fn CG_FindClientModelFile(
    mut filename: *mut i8,
    mut length: i32,
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut teamName: *const i8,
    mut modelName: *const i8,
    mut skinName: *const i8,
    mut base: *const i8,
    mut ext: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut team: *mut i8 = 0 as *mut i8;
    let mut charactersFolder: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
        match (*ci).team {
            2 => team = b"blue\x00" as *const u8 as *mut i8,
            _ => team = b"red\x00" as *const u8 as *mut i8,
        }
    } else {
        team = b"default\x00" as *const u8 as *mut i8
    }
    charactersFolder = b"\x00" as *const u8 as *mut i8;
    loop {
        for i in 0..2 {
            if i == 0 && !teamName.is_null() && *teamName as i32 != 0 {
                //								"models/players/characters/james/stroggs/lower_lily_red.skin"
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s%s_%s_%s.%s\x00" as *const u8 as *const i8,
                    charactersFolder,
                    modelName,
                    teamName,
                    base,
                    skinName,
                    team,
                    ext,
                );
            } else {
                //								"models/players/characters/james/lower_lily_red.skin"
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s_%s_%s.%s\x00" as *const u8 as *const i8,
                    charactersFolder,
                    modelName,
                    base,
                    skinName,
                    team,
                    ext,
                );
            }

            if CG_FileExists(filename) as u64 != 0 {
                return crate::src::qcommon::q_shared::qtrue;
            }

            if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
                if i == 0 && !teamName.is_null() && *teamName as i32 != 0 {
                    //								"models/players/characters/james/stroggs/lower_red.skin"
                    crate::src::qcommon::q_shared::Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const i8,
                        charactersFolder,
                        modelName,
                        teamName,
                        base,
                        team,
                        ext,
                    );
                } else {
                    //								"models/players/characters/james/lower_red.skin"
                    crate::src::qcommon::q_shared::Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const i8,
                        charactersFolder,
                        modelName,
                        base,
                        team,
                        ext,
                    );
                }
            } else if i == 0 && !teamName.is_null() && *teamName as i32 != 0 {
                //								"models/players/characters/james/stroggs/lower_lily.skin"
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const i8,
                    charactersFolder,
                    modelName,
                    teamName,
                    base,
                    skinName,
                    ext,
                );
            } else {
                //								"models/players/characters/james/lower_lily.skin"
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const i8,
                    charactersFolder,
                    modelName,
                    base,
                    skinName,
                    ext,
                );
            }

            if CG_FileExists(filename) as u64 != 0 {
                return crate::src::qcommon::q_shared::qtrue;
            }

            if teamName.is_null() || *teamName == 0 {
                break;
            }
        }
        // if tried the heads folder first
        if *charactersFolder.offset(0) != 0 {
            break;
        }
        charactersFolder = b"characters/\x00" as *const u8 as *mut i8
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==========================
CG_FindClientHeadFile
==========================
*/

unsafe extern "C" fn CG_FindClientHeadFile(
    mut filename: *mut i8,
    mut length: i32,
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut teamName: *const i8,
    mut headModelName: *const i8,
    mut headSkinName: *const i8,
    mut base: *const i8,
    mut ext: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut team: *mut i8 = 0 as *mut i8;
    let mut headsFolder: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
        match (*ci).team {
            2 => team = b"blue\x00" as *const u8 as *mut i8,
            _ => team = b"red\x00" as *const u8 as *mut i8,
        }
    } else {
        team = b"default\x00" as *const u8 as *mut i8
    }
    if *headModelName.offset(0) as i32 == '*' as i32 {
        headsFolder = b"heads/\x00" as *const u8 as *mut i8;
        headModelName = headModelName.offset(1)
    } else {
        headsFolder = b"\x00" as *const u8 as *mut i8
    }
    loop {
        for i in 0..2 {
            if i == 0 && !teamName.is_null() && *teamName as i32 != 0 {
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s/%s%s_%s.%s\x00" as *const u8 as *const i8,
                    headsFolder,
                    headModelName,
                    headSkinName,
                    teamName,
                    base,
                    team,
                    ext,
                );
            } else {
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s/%s_%s.%s\x00" as *const u8 as *const i8,
                    headsFolder,
                    headModelName,
                    headSkinName,
                    base,
                    team,
                    ext,
                );
            }

            if CG_FileExists(filename) as u64 != 0 {
                return crate::src::qcommon::q_shared::qtrue;
            }

            if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
                if i == 0 && !teamName.is_null() && *teamName as i32 != 0 {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const i8,
                        headsFolder,
                        headModelName,
                        teamName,
                        base,
                        team,
                        ext,
                    );
                } else {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const i8,
                        headsFolder,
                        headModelName,
                        base,
                        team,
                        ext,
                    );
                }
            } else if i == 0 && !teamName.is_null() && *teamName as i32 != 0 {
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const i8,
                    headsFolder,
                    headModelName,
                    teamName,
                    base,
                    headSkinName,
                    ext,
                );
            } else {
                crate::src::qcommon::q_shared::Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const i8,
                    headsFolder,
                    headModelName,
                    base,
                    headSkinName,
                    ext,
                );
            }

            if CG_FileExists(filename) as u64 != 0 {
                return crate::src::qcommon::q_shared::qtrue;
            }

            if teamName.is_null() || *teamName == 0 {
                break;
            }
        }
        // if tried the heads folder first
        if *headsFolder.offset(0) != 0 {
            break;
        }
        headsFolder = b"heads/\x00" as *const u8 as *mut i8
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==========================
CG_RegisterClientSkin
==========================
*/

unsafe extern "C" fn CG_RegisterClientSkin(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut teamName: *const i8,
    mut modelName: *const i8,
    mut skinName: *const i8,
    mut headModelName: *const i8,
    mut headSkinName: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut filename: [i8; 64] = [0; 64];
    /*
    Com_sprintf( filename, sizeof( filename ), "models/players/%s/%slower_%s.skin", modelName, teamName, skinName );
    ci->legsSkin = trap_R_RegisterSkin( filename );
    if (!ci->legsSkin) {
        Com_sprintf( filename, sizeof( filename ), "models/players/characters/%s/%slower_%s.skin", modelName, teamName, skinName );
        ci->legsSkin = trap_R_RegisterSkin( filename );
        if (!ci->legsSkin) {
            Com_Printf( "Leg skin load failure: %s\n", filename );
        }
    }


    Com_sprintf( filename, sizeof( filename ), "models/players/%s/%supper_%s.skin", modelName, teamName, skinName );
    ci->torsoSkin = trap_R_RegisterSkin( filename );
    if (!ci->torsoSkin) {
        Com_sprintf( filename, sizeof( filename ), "models/players/characters/%s/%supper_%s.skin", modelName, teamName, skinName );
        ci->torsoSkin = trap_R_RegisterSkin( filename );
        if (!ci->torsoSkin) {
            Com_Printf( "Torso skin load failure: %s\n", filename );
        }
    }
    */
    if CG_FindClientModelFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        ci,
        teamName,
        modelName,
        skinName,
        b"lower\x00" as *const u8 as *const i8,
        b"skin\x00" as *const u8 as *const i8,
    ) as u64
        != 0
    {
        (*ci).legsSkin = crate::src::cgame::cg_syscalls::trap_R_RegisterSkin(filename.as_mut_ptr())
    }
    if (*ci).legsSkin == 0 {
        crate::src::cgame::cg_main::Com_Printf(
            b"Leg skin load failure: %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
    }
    if CG_FindClientModelFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        ci,
        teamName,
        modelName,
        skinName,
        b"upper\x00" as *const u8 as *const i8,
        b"skin\x00" as *const u8 as *const i8,
    ) as u64
        != 0
    {
        (*ci).torsoSkin = crate::src::cgame::cg_syscalls::trap_R_RegisterSkin(filename.as_mut_ptr())
    }
    if (*ci).torsoSkin == 0 {
        crate::src::cgame::cg_main::Com_Printf(
            b"Torso skin load failure: %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
    }
    if CG_FindClientHeadFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        ci,
        teamName,
        headModelName,
        headSkinName,
        b"head\x00" as *const u8 as *const i8,
        b"skin\x00" as *const u8 as *const i8,
    ) as u64
        != 0
    {
        (*ci).headSkin = crate::src::cgame::cg_syscalls::trap_R_RegisterSkin(filename.as_mut_ptr())
    }
    if (*ci).headSkin == 0 {
        crate::src::cgame::cg_main::Com_Printf(
            b"Head skin load failure: %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
    }
    // if any skins failed to load
    if (*ci).legsSkin == 0 || (*ci).torsoSkin == 0 || (*ci).headSkin == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
==========================
CG_RegisterClientModelname
==========================
*/

unsafe extern "C" fn CG_RegisterClientModelname(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut modelName: *const i8,
    mut skinName: *const i8,
    mut headModelName: *const i8,
    mut headSkinName: *const i8,
    mut teamName: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut filename: [i8; 64] = [0; 64];
    let mut headName: *const i8 = 0 as *const i8;
    let mut newTeamName: [i8; 64] = [0; 64];
    if *headModelName.offset(0) as i32 == '\u{0}' as i32 {
        headName = modelName
    } else {
        headName = headModelName
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/lower.md3\x00" as *const u8 as *const i8,
        modelName,
    );
    (*ci).legsModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
    if (*ci).legsModel == 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"models/players/characters/%s/lower.md3\x00" as *const u8 as *const i8,
            modelName,
        );
        (*ci).legsModel =
            crate::src::cgame::cg_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
        if (*ci).legsModel == 0 {
            crate::src::cgame::cg_main::Com_Printf(
                b"Failed to load model file %s\n\x00" as *const u8 as *const i8,
                filename.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/upper.md3\x00" as *const u8 as *const i8,
        modelName,
    );
    (*ci).torsoModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
    if (*ci).torsoModel == 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"models/players/characters/%s/upper.md3\x00" as *const u8 as *const i8,
            modelName,
        );
        (*ci).torsoModel =
            crate::src::cgame::cg_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
        if (*ci).torsoModel == 0 {
            crate::src::cgame::cg_main::Com_Printf(
                b"Failed to load model file %s\n\x00" as *const u8 as *const i8,
                filename.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    if *headName.offset(0) as i32 == '*' as i32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"models/players/heads/%s/%s.md3\x00" as *const u8 as *const i8,
            &*headModelName.offset(1isize) as *const i8,
            &*headModelName.offset(1isize) as *const i8,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"models/players/%s/head.md3\x00" as *const u8 as *const i8,
            headName,
        );
    }
    (*ci).headModel = crate::src::cgame::cg_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
    // if the head model could not be found and we didn't load from the heads folder try to load from there
    if (*ci).headModel == 0 && *headName.offset(0) as i32 != '*' as i32 {
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"models/players/heads/%s/%s.md3\x00" as *const u8 as *const i8,
            headModelName,
            headModelName,
        );
        (*ci).headModel =
            crate::src::cgame::cg_syscalls::trap_R_RegisterModel(filename.as_mut_ptr())
    }
    if (*ci).headModel == 0 {
        crate::src::cgame::cg_main::Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    // if any skins failed to load, return failure
    if CG_RegisterClientSkin(ci, teamName, modelName, skinName, headName, headSkinName) as u64 == 0
    {
        if !teamName.is_null() && *teamName as i32 != 0 {
            crate::src::cgame::cg_main::Com_Printf(
                b"Failed to load skin file: %s : %s : %s, %s : %s\n\x00" as *const u8 as *const i8,
                teamName,
                modelName,
                skinName,
                headName,
                headSkinName,
            );
            if (*ci).team == crate::bg_public_h::TEAM_BLUE {
                crate::src::qcommon::q_shared::Com_sprintf(
                    newTeamName.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 64]>() as i32,
                    b"%s/\x00" as *const u8 as *const i8,
                    b"Pagans\x00" as *const u8 as *const i8,
                );
            } else {
                crate::src::qcommon::q_shared::Com_sprintf(
                    newTeamName.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 64]>() as i32,
                    b"%s/\x00" as *const u8 as *const i8,
                    b"Stroggs\x00" as *const u8 as *const i8,
                );
            }
            if CG_RegisterClientSkin(
                ci,
                newTeamName.as_mut_ptr(),
                modelName,
                skinName,
                headName,
                headSkinName,
            ) as u64
                == 0
            {
                crate::src::cgame::cg_main::Com_Printf(
                    b"Failed to load skin file: %s : %s : %s, %s : %s\n\x00" as *const u8
                        as *const i8,
                    newTeamName.as_mut_ptr(),
                    modelName,
                    skinName,
                    headName,
                    headSkinName,
                );
                return crate::src::qcommon::q_shared::qfalse;
            }
        } else {
            crate::src::cgame::cg_main::Com_Printf(
                b"Failed to load skin file: %s : %s, %s : %s\n\x00" as *const u8 as *const i8,
                modelName,
                skinName,
                headName,
                headSkinName,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    // load the animations
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/animation.cfg\x00" as *const u8 as *const i8,
        modelName,
    );
    if CG_ParseAnimationFile(filename.as_mut_ptr(), ci) as u64 == 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"models/players/characters/%s/animation.cfg\x00" as *const u8 as *const i8,
            modelName,
        );
        if CG_ParseAnimationFile(filename.as_mut_ptr(), ci) as u64 == 0 {
            crate::src::cgame::cg_main::Com_Printf(
                b"Failed to load animation file %s\n\x00" as *const u8 as *const i8,
                filename.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    if CG_FindClientHeadFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        ci,
        teamName,
        headName,
        headSkinName,
        b"icon\x00" as *const u8 as *const i8,
        b"skin\x00" as *const u8 as *const i8,
    ) as u64
        != 0
    {
        (*ci).modelIcon =
            crate::src::cgame::cg_syscalls::trap_R_RegisterShaderNoMip(filename.as_mut_ptr())
    } else if CG_FindClientHeadFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        ci,
        teamName,
        headName,
        headSkinName,
        b"icon\x00" as *const u8 as *const i8,
        b"tga\x00" as *const u8 as *const i8,
    ) as u64
        != 0
    {
        (*ci).modelIcon =
            crate::src::cgame::cg_syscalls::trap_R_RegisterShaderNoMip(filename.as_mut_ptr())
    }
    if (*ci).modelIcon == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
====================
CG_ColorFromString
====================
*/

unsafe extern "C" fn CG_ColorFromString(
    mut v: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut val: i32 = 0;
    let ref mut fresh0 = *color.offset(2);
    *fresh0 = 0f32;
    let ref mut fresh1 = *color.offset(1);
    *fresh1 = *fresh0;
    *color.offset(0) = *fresh1;
    val = atoi(v);
    if val < 1 || val > 7 {
        *color.offset(0) = 1f32;
        *color.offset(1) = 1f32;
        *color.offset(2) = 1f32;
        return;
    }
    if val & 1 != 0 {
        *color.offset(2) = 1.0f32
    }
    if val & 2 != 0 {
        *color.offset(1) = 1.0f32
    }
    if val & 4 != 0 {
        *color.offset(0) = 1.0f32
    };
}
/*
===================
CG_LoadClientInfo

Load it now, taking the disk hits.
This will usually be deferred to a safe time
===================
*/

unsafe extern "C" fn CG_LoadClientInfo(
    mut clientNum: i32,
    mut ci: *mut crate::cg_local_h::clientInfo_t,
) {
    let mut dir: *const i8 = 0 as *const i8;
    let mut fallback: *const i8 = 0 as *const i8;
    let mut i: i32 = 0;
    let mut modelloaded: i32 = 0;
    let mut s: *const i8 = 0 as *const i8;
    let mut teamname: [i8; 64] = [0; 64];
    teamname[0] = 0;
    modelloaded = crate::src::qcommon::q_shared::qtrue as i32;
    if CG_RegisterClientModelname(
        ci,
        (*ci).modelName.as_mut_ptr(),
        (*ci).skinName.as_mut_ptr(),
        (*ci).headModelName.as_mut_ptr(),
        (*ci).headSkinName.as_mut_ptr(),
        teamname.as_mut_ptr(),
    ) as u64
        == 0
    {
        if crate::src::cgame::cg_main::cg_buildScript.integer != 0 {
            crate::src::cgame::cg_main::CG_Error(
                b"CG_RegisterClientModelname( %s, %s, %s, %s %s ) failed\x00" as *const u8
                    as *const i8,
                (*ci).modelName.as_mut_ptr(),
                (*ci).skinName.as_mut_ptr(),
                (*ci).headModelName.as_mut_ptr(),
                (*ci).headSkinName.as_mut_ptr(),
                teamname.as_mut_ptr(),
            );
        }
        // fall back to default team name
        if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
            // keep skin name
            if (*ci).team == crate::bg_public_h::TEAM_BLUE {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    teamname.as_mut_ptr(),
                    b"Pagans\x00" as *const u8 as *const i8,
                    ::std::mem::size_of::<[i8; 64]>() as i32,
                );
            } else {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    teamname.as_mut_ptr(),
                    b"Stroggs\x00" as *const u8 as *const i8,
                    ::std::mem::size_of::<[i8; 64]>() as i32,
                );
            }
            if CG_RegisterClientModelname(
                ci,
                b"sarge\x00" as *const u8 as *const i8,
                (*ci).skinName.as_mut_ptr(),
                b"sarge\x00" as *const u8 as *const i8,
                (*ci).skinName.as_mut_ptr(),
                teamname.as_mut_ptr(),
            ) as u64
                == 0
            {
                crate::src::cgame::cg_main::CG_Error(
                    b"DEFAULT_TEAM_MODEL / skin (%s/%s) failed to register\x00" as *const u8
                        as *const i8,
                    b"sarge\x00" as *const u8 as *const i8,
                    (*ci).skinName.as_mut_ptr(),
                );
            }
        } else if CG_RegisterClientModelname(
            ci,
            b"sarge\x00" as *const u8 as *const i8,
            b"default\x00" as *const u8 as *const i8,
            b"sarge\x00" as *const u8 as *const i8,
            b"default\x00" as *const u8 as *const i8,
            teamname.as_mut_ptr(),
        ) as u64
            == 0
        {
            crate::src::cgame::cg_main::CG_Error(
                b"DEFAULT_MODEL (%s) failed to register\x00" as *const u8 as *const i8,
                b"sarge\x00" as *const u8 as *const i8,
            );
        }
        modelloaded = crate::src::qcommon::q_shared::qfalse as i32
    }
    (*ci).newAnims = crate::src::qcommon::q_shared::qfalse;
    if (*ci).torsoModel != 0 {
        let mut tag: crate::src::qcommon::q_shared::orientation_t =
            crate::src::qcommon::q_shared::orientation_t {
                origin: [0.; 3],
                axis: [[0.; 3]; 3],
            };
        // if the torso model has the "tag_flag"
        if crate::src::cgame::cg_syscalls::trap_R_LerpTag(
            &mut tag,
            (*ci).torsoModel,
            0,
            0,
            1f32,
            b"tag_flag\x00" as *const u8 as *const i8,
        ) != 0
        {
            (*ci).newAnims = crate::src::qcommon::q_shared::qtrue
        }
    }
    // sounds
    dir = (*ci).modelName.as_mut_ptr();
    fallback = if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
        b"sarge\x00" as *const u8 as *const i8
    } else {
        b"sarge\x00" as *const u8 as *const i8
    };
    i = 0;
    while i < 32 {
        s = cg_customSoundNames[i as usize];
        if s.is_null() {
            break;
        }
        (*ci).sounds[i as usize] = 0;
        // if the model didn't load use the sounds of the default model
        if modelloaded != 0 {
            (*ci).sounds[i as usize] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                crate::src::qcommon::q_shared::va(
                    b"sound/player/%s/%s\x00" as *const u8 as *mut i8,
                    dir,
                    s.offset(1isize),
                ),
                crate::src::qcommon::q_shared::qfalse,
            )
        }
        if (*ci).sounds[i as usize] == 0 {
            (*ci).sounds[i as usize] = crate::src::cgame::cg_syscalls::trap_S_RegisterSound(
                crate::src::qcommon::q_shared::va(
                    b"sound/player/%s/%s\x00" as *const u8 as *mut i8,
                    fallback,
                    s.offset(1isize),
                ),
                crate::src::qcommon::q_shared::qfalse,
            )
        }
        i += 1
    }
    (*ci).deferred = crate::src::qcommon::q_shared::qfalse;
    // reset any existing players and bodies, because they might be in bad
    // frames for this new model
    i = 0;
    while i < (1) << 10 {
        if crate::src::cgame::cg_main::cg_entities[i as usize]
            .currentState
            .clientNum
            == clientNum
            && crate::src::cgame::cg_main::cg_entities[i as usize]
                .currentState
                .eType
                == crate::bg_public_h::ET_PLAYER as i32
        {
            CG_ResetPlayerEntity(
                &mut *crate::src::cgame::cg_main::cg_entities
                    .as_mut_ptr()
                    .offset(i as isize),
            );
        }
        i += 1
    }
}
/*
======================
CG_CopyClientInfoModel
======================
*/

unsafe extern "C" fn CG_CopyClientInfoModel(
    mut from: *mut crate::cg_local_h::clientInfo_t,
    mut to: *mut crate::cg_local_h::clientInfo_t,
) {
    (*to).headOffset[0] = (*from).headOffset[0];
    (*to).headOffset[1] = (*from).headOffset[1];
    (*to).headOffset[2] = (*from).headOffset[2];
    (*to).footsteps = (*from).footsteps;
    (*to).gender = (*from).gender;
    (*to).legsModel = (*from).legsModel;
    (*to).legsSkin = (*from).legsSkin;
    (*to).torsoModel = (*from).torsoModel;
    (*to).torsoSkin = (*from).torsoSkin;
    (*to).headModel = (*from).headModel;
    (*to).headSkin = (*from).headSkin;
    (*to).modelIcon = (*from).modelIcon;
    (*to).newAnims = (*from).newAnims;
    crate::stdlib::memcpy(
        (*to).animations.as_mut_ptr() as *mut libc::c_void,
        (*from).animations.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[crate::bg_public_h::animation_t; 37]>(),
    );
    crate::stdlib::memcpy(
        (*to).sounds.as_mut_ptr() as *mut libc::c_void,
        (*from).sounds.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::sfxHandle_t; 32]>(),
    );
}
/*
======================
CG_ScanForExistingClientInfo
======================
*/

unsafe extern "C" fn CG_ScanForExistingClientInfo(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut match_0: *mut crate::cg_local_h::clientInfo_t =
        0 as *mut crate::cg_local_h::clientInfo_t;

    for i in 0..crate::src::cgame::cg_main::cgs.maxclients {
        match_0 = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset(i as isize) as *mut crate::cg_local_h::clientInfo_t;

        if !((*match_0).infoValid as u64 == 0) {
            if !((*match_0).deferred as u64 != 0) {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    (*ci).modelName.as_mut_ptr(),
                    (*match_0).modelName.as_mut_ptr(),
                ) == 0
                    && crate::src::qcommon::q_shared::Q_stricmp(
                        (*ci).skinName.as_mut_ptr(),
                        (*match_0).skinName.as_mut_ptr(),
                    ) == 0
                    && crate::src::qcommon::q_shared::Q_stricmp(
                        (*ci).headModelName.as_mut_ptr(),
                        (*match_0).headModelName.as_mut_ptr(),
                    ) == 0
                    && crate::src::qcommon::q_shared::Q_stricmp(
                        (*ci).headSkinName.as_mut_ptr(),
                        (*match_0).headSkinName.as_mut_ptr(),
                    ) == 0
                    && crate::src::qcommon::q_shared::Q_stricmp(
                        (*ci).blueTeam.as_mut_ptr(),
                        (*match_0).blueTeam.as_mut_ptr(),
                    ) == 0
                    && crate::src::qcommon::q_shared::Q_stricmp(
                        (*ci).redTeam.as_mut_ptr(),
                        (*match_0).redTeam.as_mut_ptr(),
                    ) == 0
                    && ((crate::src::cgame::cg_main::cgs.gametype) < crate::bg_public_h::GT_TEAM
                        || (*ci).team == (*match_0).team)
                {
                    // this clientinfo is identical, so use its handles
                    (*ci).deferred = crate::src::qcommon::q_shared::qfalse;
                    CG_CopyClientInfoModel(match_0, ci);
                    return crate::src::qcommon::q_shared::qtrue;
                }
            }
        }
    }
    // nothing matches, so defer the load
    return crate::src::qcommon::q_shared::qfalse;
}
/*
======================
CG_SetDeferredClientInfo

We aren't going to load it now, so grab some other
client's info to use until we have some spare time.
======================
*/

unsafe extern "C" fn CG_SetDeferredClientInfo(
    mut clientNum: i32,
    mut ci: *mut crate::cg_local_h::clientInfo_t,
) {
    let mut i: i32 = 0;
    let mut match_0: *mut crate::cg_local_h::clientInfo_t =
        0 as *mut crate::cg_local_h::clientInfo_t;
    // if someone else is already the same models and skins we
    // can just load the client info
    i = 0;
    while i < crate::src::cgame::cg_main::cgs.maxclients {
        match_0 = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset(i as isize) as *mut crate::cg_local_h::clientInfo_t;
        if !((*match_0).infoValid as u64 == 0 || (*match_0).deferred != 0) {
            if !(crate::src::qcommon::q_shared::Q_stricmp(
                (*ci).skinName.as_mut_ptr(),
                (*match_0).skinName.as_mut_ptr(),
            ) != 0
                || crate::src::qcommon::q_shared::Q_stricmp(
                    (*ci).modelName.as_mut_ptr(),
                    (*match_0).modelName.as_mut_ptr(),
                ) != 0
                || crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM
                    && (*ci).team != (*match_0).team)
            {
                // just load the real info cause it uses the same models and skins
                CG_LoadClientInfo(clientNum, ci);
                return;
            }
        }
        i += 1
    }
    // if we are in teamplay, only grab a model if the skin is correct
    if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
        i = 0;
        while i < crate::src::cgame::cg_main::cgs.maxclients {
            match_0 = &mut *crate::src::cgame::cg_main::cgs
                .clientinfo
                .as_mut_ptr()
                .offset(i as isize) as *mut crate::cg_local_h::clientInfo_t;
            if !((*match_0).infoValid as u64 == 0 || (*match_0).deferred != 0) {
                if !(crate::src::qcommon::q_shared::Q_stricmp(
                    (*ci).skinName.as_mut_ptr(),
                    (*match_0).skinName.as_mut_ptr(),
                ) != 0
                    || crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM
                        && (*ci).team != (*match_0).team)
                {
                    (*ci).deferred = crate::src::qcommon::q_shared::qtrue;
                    CG_CopyClientInfoModel(match_0, ci);
                    return;
                }
            }
            i += 1
        }
        // load the full model, because we don't ever want to show
        // an improper team skin.  This will cause a hitch for the first
        // player, when the second enters.  Combat shouldn't be going on
        // yet, so it shouldn't matter
        CG_LoadClientInfo(clientNum, ci);
        return;
    }
    // find the first valid clientinfo and grab its stuff
    i = 0;
    while i < crate::src::cgame::cg_main::cgs.maxclients {
        match_0 = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset(i as isize) as *mut crate::cg_local_h::clientInfo_t;
        if (*match_0).infoValid as u64 == 0 {
            i += 1
        } else {
            (*ci).deferred = crate::src::qcommon::q_shared::qtrue;
            CG_CopyClientInfoModel(match_0, ci);
            return;
        }
    }
    // we should never get here...
    crate::src::cgame::cg_main::CG_Printf(
        b"CG_SetDeferredClientInfo: no valid clients!\n\x00" as *const u8 as *const i8,
    );
    CG_LoadClientInfo(clientNum, ci);
}
/*
======================
CG_NewClientInfo
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_NewClientInfo(mut clientNum: i32) {
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut newInfo: crate::cg_local_h::clientInfo_t = crate::cg_local_h::clientInfo_t {
        infoValid: crate::src::qcommon::q_shared::qfalse,
        name: [0; 64],
        team: crate::bg_public_h::TEAM_FREE,
        botSkill: 0,
        color1: [0.; 3],
        color2: [0.; 3],
        c1RGBA: [0; 4],
        c2RGBA: [0; 4],
        score: 0,
        location: 0,
        health: 0,
        armor: 0,
        curWeapon: 0,
        handicap: 0,
        wins: 0,
        losses: 0,
        teamTask: 0,
        teamLeader: crate::src::qcommon::q_shared::qfalse,
        powerups: 0,
        medkitUsageTime: 0,
        invulnerabilityStartTime: 0,
        invulnerabilityStopTime: 0,
        breathPuffTime: 0,
        modelName: [0; 64],
        skinName: [0; 64],
        headModelName: [0; 64],
        headSkinName: [0; 64],
        redTeam: [0; 32],
        blueTeam: [0; 32],
        deferred: crate::src::qcommon::q_shared::qfalse,
        newAnims: crate::src::qcommon::q_shared::qfalse,
        fixedlegs: crate::src::qcommon::q_shared::qfalse,
        fixedtorso: crate::src::qcommon::q_shared::qfalse,
        headOffset: [0.; 3],
        footsteps: crate::cg_local_h::FOOTSTEP_NORMAL,
        gender: crate::bg_public_h::GENDER_MALE,
        legsModel: 0,
        legsSkin: 0,
        torsoModel: 0,
        torsoSkin: 0,
        headModel: 0,
        headSkin: 0,
        modelIcon: 0,
        animations: [crate::bg_public_h::animation_t {
            firstFrame: 0,
            numFrames: 0,
            loopFrames: 0,
            frameLerp: 0,
            initialLerp: 0,
            reversed: 0,
            flipflop: 0,
        }; 37],
        sounds: [0; 32],
    };
    let mut configstring: *const i8 = 0 as *const i8;
    let mut v: *const i8 = 0 as *const i8;
    let mut slash: *mut i8 = 0 as *mut i8;
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::cg_local_h::clientInfo_t;
    configstring = crate::src::cgame::cg_main::CG_ConfigString(clientNum + (32 + 256 + 256));
    if *configstring.offset(0) == 0 {
        crate::stdlib::memset(
            ci as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::cg_local_h::clientInfo_t>(),
        );
        return;
        // player just left
    }
    // build into a temp buffer so the defer checks can use
    // the old value
    crate::stdlib::memset(
        &mut newInfo as *mut crate::cg_local_h::clientInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::cg_local_h::clientInfo_t>(),
    );
    // isolate the player's name
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"n\x00" as *const u8 as *const i8,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        newInfo.name.as_mut_ptr(),
        v,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    // colors
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"c1\x00" as *const u8 as *const i8,
    );
    CG_ColorFromString(v, newInfo.color1.as_mut_ptr());
    newInfo.c1RGBA[0] = (255f32 * newInfo.color1[0]) as crate::src::qcommon::q_shared::byte;
    newInfo.c1RGBA[1] = (255f32 * newInfo.color1[1]) as crate::src::qcommon::q_shared::byte;
    newInfo.c1RGBA[2] = (255f32 * newInfo.color1[2]) as crate::src::qcommon::q_shared::byte;
    newInfo.c1RGBA[3] = 255;
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"c2\x00" as *const u8 as *const i8,
    );
    CG_ColorFromString(v, newInfo.color2.as_mut_ptr());
    newInfo.c2RGBA[0] = (255f32 * newInfo.color2[0]) as crate::src::qcommon::q_shared::byte;
    newInfo.c2RGBA[1] = (255f32 * newInfo.color2[1]) as crate::src::qcommon::q_shared::byte;
    newInfo.c2RGBA[2] = (255f32 * newInfo.color2[2]) as crate::src::qcommon::q_shared::byte;
    newInfo.c2RGBA[3] = 255;
    // bot skill
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"skill\x00" as *const u8 as *const i8,
    );
    newInfo.botSkill = atoi(v);
    // handicap
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"hc\x00" as *const u8 as *const i8,
    );
    newInfo.handicap = atoi(v);
    // wins
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"w\x00" as *const u8 as *const i8,
    );
    newInfo.wins = atoi(v);
    // losses
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"l\x00" as *const u8 as *const i8,
    );
    newInfo.losses = atoi(v);
    // team
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"t\x00" as *const u8 as *const i8,
    );
    newInfo.team = atoi(v) as crate::bg_public_h::team_t;
    // team task
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"tt\x00" as *const u8 as *const i8,
    );
    newInfo.teamTask = atoi(v);
    // team leader
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"tl\x00" as *const u8 as *const i8,
    );
    newInfo.teamLeader = atoi(v) as crate::src::qcommon::q_shared::qboolean;
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"g_redteam\x00" as *const u8 as *const i8,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(newInfo.redTeam.as_mut_ptr(), v, 32);
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"g_blueteam\x00" as *const u8 as *const i8,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(newInfo.blueTeam.as_mut_ptr(), v, 32);
    // model
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"model\x00" as *const u8 as *const i8,
    );
    if crate::src::cgame::cg_main::cg_forceModel.integer != 0 {
        // forcemodel makes everyone use a single model
        // to prevent load hitches
        let mut modelStr: [i8; 64] = [0; 64];
        let mut skin: *mut i8 = 0 as *mut i8;
        if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.modelName.as_mut_ptr(),
                b"sarge\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
        } else {
            crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer(
                b"model\x00" as *const u8 as *const i8,
                modelStr.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            skin = crate::stdlib::strchr(modelStr.as_mut_ptr(), '/' as i32);
            if skin.is_null() {
                skin = b"default\x00" as *const u8 as *mut i8
            } else {
                let fresh2 = skin;
                skin = skin.offset(1);
                *fresh2 = 0i8
            }
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                skin,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.modelName.as_mut_ptr(),
                modelStr.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
        }
        if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
            // keep skin name
            slash = crate::stdlib::strchr(v, '/' as i32);
            if !slash.is_null() {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    newInfo.skinName.as_mut_ptr(),
                    slash.offset(1isize),
                    ::std::mem::size_of::<[i8; 64]>() as i32,
                );
            }
        }
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(
            newInfo.modelName.as_mut_ptr(),
            v,
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        slash = crate::stdlib::strchr(newInfo.modelName.as_mut_ptr(), '/' as i32);
        if slash.is_null() {
            // modelName didn not include a skin name
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
        } else {
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                slash.offset(1),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            // truncate modelName
            *slash = 0
        }
    }
    // head model
    v = crate::src::qcommon::q_shared::Info_ValueForKey(
        configstring,
        b"hmodel\x00" as *const u8 as *const i8,
    );
    if crate::src::cgame::cg_main::cg_forceModel.integer != 0 {
        // forcemodel makes everyone use a single model
        // to prevent load hitches
        let mut modelStr_0: [i8; 64] = [0; 64];
        let mut skin_0: *mut i8 = 0 as *mut i8;
        if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.headModelName.as_mut_ptr(),
                b"sarge\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
        } else {
            crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer(
                b"headmodel\x00" as *const u8 as *const i8,
                modelStr_0.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            skin_0 = crate::stdlib::strchr(modelStr_0.as_mut_ptr(), '/' as i32);
            if skin_0.is_null() {
                skin_0 = b"default\x00" as *const u8 as *mut i8
            } else {
                let fresh3 = skin_0;
                skin_0 = skin_0.offset(1);
                *fresh3 = 0i8
            }
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                skin_0,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.headModelName.as_mut_ptr(),
                modelStr_0.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
        }
        if crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM {
            // keep skin name
            slash = crate::stdlib::strchr(v, '/' as i32);
            if !slash.is_null() {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    newInfo.headSkinName.as_mut_ptr(),
                    slash.offset(1isize),
                    ::std::mem::size_of::<[i8; 64]>() as i32,
                );
            }
        }
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(
            newInfo.headModelName.as_mut_ptr(),
            v,
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        slash = crate::stdlib::strchr(newInfo.headModelName.as_mut_ptr(), '/' as i32);
        if slash.is_null() {
            // modelName didn not include a skin name
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const i8,
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
        } else {
            crate::src::qcommon::q_shared::Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                slash.offset(1),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            // truncate modelName
            *slash = 0
        }
    }
    // scan for an existing clientinfo that matches this modelname
    // so we can avoid loading checks if possible
    if CG_ScanForExistingClientInfo(&mut newInfo) as u64 == 0 {
        let mut forceDefer: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        forceDefer = (crate::src::cgame::cg_syscalls::trap_MemoryRemaining() < 4000000)
            as crate::src::qcommon::q_shared::qboolean;
        // if we are defering loads, just have it pick the first valid
        if forceDefer != 0
            || crate::src::cgame::cg_main::cg_deferPlayers.integer != 0
                && crate::src::cgame::cg_main::cg_buildScript.integer == 0
                && crate::src::cgame::cg_main::cg.loading as u64 == 0
        {
            // keep whatever they had if it won't violate team skins
            CG_SetDeferredClientInfo(clientNum, &mut newInfo);
            // if we are low on memory, leave them with this model
            if forceDefer as u64 != 0 {
                crate::src::cgame::cg_main::CG_Printf(
                    b"Memory is low. Using deferred model.\n\x00" as *const u8 as *const i8,
                );
                newInfo.deferred = crate::src::qcommon::q_shared::qfalse
            }
        } else {
            CG_LoadClientInfo(clientNum, &mut newInfo);
        }
    }
    // replace whatever was there with the new one
    newInfo.infoValid = crate::src::qcommon::q_shared::qtrue;
    *ci = newInfo;
}
//
// cg_predict.c
//
/*
======================
CG_LoadDeferredPlayers

Called each frame when a player is dead
and the scoreboard is up
so deferred players can be loaded
======================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_LoadDeferredPlayers() {
    let mut i: i32 = 0;
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    // scan for a deferred player to load
    i = 0;
    ci = crate::src::cgame::cg_main::cgs.clientinfo.as_mut_ptr();
    while i < crate::src::cgame::cg_main::cgs.maxclients {
        if (*ci).infoValid != 0 && (*ci).deferred != 0 {
            // if we are low on memory, leave it deferred
            if crate::src::cgame::cg_syscalls::trap_MemoryRemaining() < 4000000 {
                crate::src::cgame::cg_main::CG_Printf(
                    b"Memory is low. Using deferred model.\n\x00" as *const u8 as *const i8,
                );
                (*ci).deferred = crate::src::qcommon::q_shared::qfalse
            } else {
                CG_LoadClientInfo(i, ci);
            }
            //			break;
        }
        i += 1;
        ci = ci.offset(1)
    }
}
/*
=============================================================================

PLAYER ANIMATION

=============================================================================
*/
/*
===============
CG_SetLerpFrameAnimation

may include ANIM_TOGGLEBIT
===============
*/

unsafe extern "C" fn CG_SetLerpFrameAnimation(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut lf: *mut crate::cg_local_h::lerpFrame_t,
    mut newAnimation: i32,
) {
    let mut anim: *mut crate::bg_public_h::animation_t = 0 as *mut crate::bg_public_h::animation_t;
    (*lf).animationNumber = newAnimation;
    newAnimation &= !(128);
    if newAnimation < 0 || newAnimation >= crate::bg_public_h::MAX_TOTALANIMATIONS as i32 {
        crate::src::cgame::cg_main::CG_Error(
            b"Bad animation number: %i\x00" as *const u8 as *const i8,
            newAnimation,
        );
    }
    anim = &mut *(*ci).animations.as_mut_ptr().offset(newAnimation as isize)
        as *mut crate::bg_public_h::animation_t;
    (*lf).animation = anim;
    (*lf).animationTime = (*lf).frameTime + (*anim).initialLerp;
    if crate::src::cgame::cg_main::cg_debugAnim.integer != 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"Anim: %i\n\x00" as *const u8 as *const i8,
            newAnimation,
        );
    };
}
/*
===============
CG_RunLerpFrame

Sets cg.snap, cg.oldFrame, and cg.backlerp
cg.time should be between oldFrameTime and frameTime after exit
===============
*/

unsafe extern "C" fn CG_RunLerpFrame(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut lf: *mut crate::cg_local_h::lerpFrame_t,
    mut newAnimation: i32,
    mut speedScale: f32,
) {
    let mut f: i32 = 0;
    let mut numFrames: i32 = 0;
    let mut anim: *mut crate::bg_public_h::animation_t = 0 as *mut crate::bg_public_h::animation_t;
    // debugging tool to get no animations
    if crate::src::cgame::cg_main::cg_animSpeed.integer == 0 {
        (*lf).backlerp = 0f32;
        (*lf).frame = (*lf).backlerp as i32;
        (*lf).oldFrame = (*lf).frame;
        return;
    }
    // see if the animation sequence is switching
    if newAnimation != (*lf).animationNumber || (*lf).animation.is_null() {
        CG_SetLerpFrameAnimation(ci, lf, newAnimation);
    }
    // if we have passed the current frame, move it to
    // oldFrame and calculate a new frame
    if crate::src::cgame::cg_main::cg.time >= (*lf).frameTime {
        (*lf).oldFrame = (*lf).frame;
        (*lf).oldFrameTime = (*lf).frameTime;
        // get the next frame based on the animation
        anim = (*lf).animation;
        if (*anim).frameLerp == 0 {
            return;
            // shouldn't happen
        }
        if crate::src::cgame::cg_main::cg.time < (*lf).animationTime {
            (*lf).frameTime = (*lf).animationTime
        // initial lerp
        } else {
            (*lf).frameTime = (*lf).oldFrameTime + (*anim).frameLerp
        } // adjust for haste, etc
        f = ((*lf).frameTime - (*lf).animationTime) / (*anim).frameLerp;
        f = (f as f32 * speedScale) as i32;
        numFrames = (*anim).numFrames;
        if (*anim).flipflop != 0 {
            numFrames *= 2
        }
        if f >= numFrames {
            f -= numFrames;
            if (*anim).loopFrames != 0 {
                f %= (*anim).loopFrames;
                f += (*anim).numFrames - (*anim).loopFrames
            } else {
                f = numFrames - 1;
                // the animation is stuck at the end, so it
                // can immediately transition to another sequence
                (*lf).frameTime = crate::src::cgame::cg_main::cg.time
            }
        }
        if (*anim).reversed != 0 {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1 - f
        } else if (*anim).flipflop != 0 && f >= (*anim).numFrames {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1 - f % (*anim).numFrames
        } else {
            (*lf).frame = (*anim).firstFrame + f
        }
        if crate::src::cgame::cg_main::cg.time > (*lf).frameTime {
            (*lf).frameTime = crate::src::cgame::cg_main::cg.time;
            if crate::src::cgame::cg_main::cg_debugAnim.integer != 0 {
                crate::src::cgame::cg_main::CG_Printf(
                    b"Clamp lf->frameTime\n\x00" as *const u8 as *const i8,
                );
            }
        }
    }
    if (*lf).frameTime > crate::src::cgame::cg_main::cg.time + 200 {
        (*lf).frameTime = crate::src::cgame::cg_main::cg.time
    }
    if (*lf).oldFrameTime > crate::src::cgame::cg_main::cg.time {
        (*lf).oldFrameTime = crate::src::cgame::cg_main::cg.time
    }
    // calculate current lerp value
    if (*lf).frameTime == (*lf).oldFrameTime {
        (*lf).backlerp = 0f32
    } else {
        (*lf).backlerp =
            (1.0 - ((crate::src::cgame::cg_main::cg.time - (*lf).oldFrameTime) as f32
                / ((*lf).frameTime - (*lf).oldFrameTime) as f32) as f64) as f32
    };
}
/*
===============
CG_ClearLerpFrame
===============
*/

unsafe extern "C" fn CG_ClearLerpFrame(
    mut ci: *mut crate::cg_local_h::clientInfo_t,
    mut lf: *mut crate::cg_local_h::lerpFrame_t,
    mut animationNumber: i32,
) {
    (*lf).oldFrameTime = crate::src::cgame::cg_main::cg.time;
    (*lf).frameTime = (*lf).oldFrameTime;
    CG_SetLerpFrameAnimation(ci, lf, animationNumber);
    (*lf).frame = (*(*lf).animation).firstFrame;
    (*lf).oldFrame = (*lf).frame;
}
/*
===============
CG_PlayerAnimation
===============
*/

unsafe extern "C" fn CG_PlayerAnimation(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut legsOld: *mut i32,
    mut legs: *mut i32,
    mut legsBackLerp: *mut f32,
    mut torsoOld: *mut i32,
    mut torso: *mut i32,
    mut torsoBackLerp: *mut f32,
) {
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut clientNum: i32 = 0;
    let mut speedScale: f32 = 0.;
    clientNum = (*cent).currentState.clientNum;
    if crate::src::cgame::cg_main::cg_noPlayerAnims.integer != 0 {
        *torso = 0;
        *torsoOld = *torso;
        *legs = *torsoOld;
        *legsOld = *legs;
        return;
    }
    if (*cent).currentState.powerups & (1) << crate::bg_public_h::PW_HASTE as i32 != 0 {
        speedScale = 1.5
    } else {
        speedScale = 1f32
    }
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::cg_local_h::clientInfo_t;
    // do the shuffle turn frames locally
    if (*cent).pe.legs.yawing != 0
        && (*cent).currentState.legsAnim & !(128) == crate::bg_public_h::LEGS_IDLE as i32
    {
        CG_RunLerpFrame(
            ci,
            &mut (*cent).pe.legs,
            crate::bg_public_h::LEGS_TURN as i32,
            speedScale,
        );
    } else {
        CG_RunLerpFrame(
            ci,
            &mut (*cent).pe.legs,
            (*cent).currentState.legsAnim,
            speedScale,
        );
    }
    *legsOld = (*cent).pe.legs.oldFrame;
    *legs = (*cent).pe.legs.frame;
    *legsBackLerp = (*cent).pe.legs.backlerp;
    CG_RunLerpFrame(
        ci,
        &mut (*cent).pe.torso,
        (*cent).currentState.torsoAnim,
        speedScale,
    );
    *torsoOld = (*cent).pe.torso.oldFrame;
    *torso = (*cent).pe.torso.frame;
    *torsoBackLerp = (*cent).pe.torso.backlerp;
}
/*
=============================================================================

PLAYER ANGLES

=============================================================================
*/
/*
==================
CG_SwingAngles
==================
*/

unsafe extern "C" fn CG_SwingAngles(
    mut destination: f32,
    mut swingTolerance: f32,
    mut clampTolerance: f32,
    mut speed: f32,
    mut angle: *mut f32,
    mut swinging: *mut crate::src::qcommon::q_shared::qboolean,
) {
    let mut swing: f32 = 0.;
    let mut move_0: f32 = 0.;
    let mut scale: f32 = 0.;
    if *swinging as u64 == 0 {
        // see if a swing should be started
        swing = crate::src::qcommon::q_math::AngleSubtract(*angle, destination);
        if swing > swingTolerance || swing < -swingTolerance {
            *swinging = crate::src::qcommon::q_shared::qtrue
        }
    }
    if *swinging as u64 == 0 {
        return;
    }
    // modify the speed depending on the delta
    // so it doesn't seem so linear
    swing = crate::src::qcommon::q_math::AngleSubtract(destination, *angle);
    scale = crate::stdlib::fabs(swing as f64) as f32;
    if (scale as f64) < swingTolerance as f64 * 0.5 {
        scale = 0.5
    } else if scale < swingTolerance {
        scale = 1f32
    } else {
        scale = 2f32
    }
    // swing towards the destination angle
    if swing >= 0f32 {
        move_0 = crate::src::cgame::cg_main::cg.frametime as f32 * scale * speed;
        if move_0 >= swing {
            move_0 = swing;
            *swinging = crate::src::qcommon::q_shared::qfalse
        }
        *angle = crate::src::qcommon::q_math::AngleMod(*angle + move_0)
    } else if swing < 0f32 {
        move_0 = crate::src::cgame::cg_main::cg.frametime as f32 * scale * -speed;
        if move_0 <= swing {
            move_0 = swing;
            *swinging = crate::src::qcommon::q_shared::qfalse
        }
        *angle = crate::src::qcommon::q_math::AngleMod(*angle + move_0)
    }
    // clamp to no more than tolerance
    swing = crate::src::qcommon::q_math::AngleSubtract(destination, *angle);
    if swing > clampTolerance {
        *angle = crate::src::qcommon::q_math::AngleMod(destination - (clampTolerance - 1f32))
    } else if swing < -clampTolerance {
        *angle = crate::src::qcommon::q_math::AngleMod(destination + (clampTolerance - 1f32))
    };
}
/*
=================
CG_AddPainTwitch
=================
*/

unsafe extern "C" fn CG_AddPainTwitch(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut torsoAngles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut t: i32 = 0;
    let mut f: f32 = 0.;
    t = crate::src::cgame::cg_main::cg.time - (*cent).pe.painTime;
    if t >= 200 {
        return;
    }
    f = (1.0 - (t as f32 / 200f32) as f64) as f32;
    if (*cent).pe.painDirection != 0 {
        let ref mut fresh4 = *torsoAngles.offset(2);
        *fresh4 += 20f32 * f
    } else {
        let ref mut fresh5 = *torsoAngles.offset(2);
        *fresh5 -= 20f32 * f
    };
}
/*
===============
CG_PlayerAngles

Handles separate torso motion

  legs pivot based on direction of movement

  head always looks exactly at cent->lerpAngles

  if motion < 20 degrees, show in head only
  if < 45 degrees, also show in torso
===============
*/

unsafe extern "C" fn CG_PlayerAngles(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut legs: *mut crate::src::qcommon::q_shared::vec3_t,
    mut torso: *mut crate::src::qcommon::q_shared::vec3_t,
    mut head: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut legsAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut torsoAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut headAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dest: f32 = 0.;
    static mut movementOffsets: [i32; 8] = [0, 22, 45, -(22), 0, 22, -(45), -(22)];
    let mut velocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut speed: f32 = 0.;
    let mut dir: i32 = 0;
    let mut clientNum: i32 = 0;
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    headAngles[0] = (*cent).lerpAngles[0];
    headAngles[1] = (*cent).lerpAngles[1];
    headAngles[2] = (*cent).lerpAngles[2];
    headAngles[1] = crate::src::qcommon::q_math::AngleMod(headAngles[1]);
    legsAngles[2] = 0f32;
    legsAngles[1] = legsAngles[2];
    legsAngles[0] = legsAngles[1];
    torsoAngles[2] = 0f32;
    torsoAngles[1] = torsoAngles[2];
    torsoAngles[0] = torsoAngles[1];
    // --------- yaw -------------
    // allow yaw to drift a bit
    if (*cent).currentState.legsAnim & !(128) != crate::bg_public_h::LEGS_IDLE as i32
        || (*cent).currentState.torsoAnim & !(128) != crate::bg_public_h::TORSO_STAND as i32
            && (*cent).currentState.torsoAnim & !(128) != crate::bg_public_h::TORSO_STAND2 as i32
    {
        // if not standing still, always point all in the same direction
        (*cent).pe.torso.yawing = crate::src::qcommon::q_shared::qtrue; // always center
                                                                        // always center
        (*cent).pe.torso.pitching = crate::src::qcommon::q_shared::qtrue; // always center
        (*cent).pe.legs.yawing = crate::src::qcommon::q_shared::qtrue
    }
    // adjust legs for movement dir
    if (*cent).currentState.eFlags & 0x1 != 0 {
        // don't let dead bodies twitch
        dir = 0
    } else {
        dir = (*cent).currentState.angles2[1] as i32;
        if dir < 0 || dir > 7 {
            crate::src::cgame::cg_main::CG_Error(
                b"Bad player movement angle\x00" as *const u8 as *const i8,
            );
        }
    }
    legsAngles[1] = headAngles[1] + movementOffsets[dir as usize] as f32;
    torsoAngles[1] = (headAngles[1] as f64 + 0.25 * movementOffsets[dir as usize] as f64)
        as crate::src::qcommon::q_shared::vec_t;
    // torso
    CG_SwingAngles(
        torsoAngles[1],
        25f32,
        90f32,
        crate::src::cgame::cg_main::cg_swingSpeed.value,
        &mut (*cent).pe.torso.yawAngle,
        &mut (*cent).pe.torso.yawing,
    );
    CG_SwingAngles(
        legsAngles[1],
        40f32,
        90f32,
        crate::src::cgame::cg_main::cg_swingSpeed.value,
        &mut (*cent).pe.legs.yawAngle,
        &mut (*cent).pe.legs.yawing,
    );
    torsoAngles[1] = (*cent).pe.torso.yawAngle;
    legsAngles[1] = (*cent).pe.legs.yawAngle;
    // --------- pitch -------------
    // only show a fraction of the pitch angle in the torso
    if headAngles[0] > 180f32 {
        dest = (-360f32 + headAngles[0]) * 0.75
    } else {
        dest = headAngles[0] * 0.75
    }
    CG_SwingAngles(
        dest,
        15f32,
        30f32,
        0.1,
        &mut (*cent).pe.torso.pitchAngle,
        &mut (*cent).pe.torso.pitching,
    );
    torsoAngles[0] = (*cent).pe.torso.pitchAngle;
    //
    clientNum = (*cent).currentState.clientNum;
    if clientNum >= 0 && clientNum < 64 {
        ci = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset(clientNum as isize) as *mut crate::cg_local_h::clientInfo_t;
        if (*ci).fixedtorso as u64 != 0 {
            torsoAngles[0] = 0.0
        }
    }
    // --------- roll -------------
    // lean towards the direction of travel
    velocity[0] = (*cent).currentState.pos.trDelta[0];
    velocity[1] = (*cent).currentState.pos.trDelta[1];
    velocity[2] = (*cent).currentState.pos.trDelta[2];
    speed = crate::src::qcommon::q_math::VectorNormalize(velocity.as_mut_ptr());
    if speed != 0. {
        let mut axis: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
        let mut side: f32 = 0.;
        speed *= 0.05;
        crate::src::qcommon::q_math::AnglesToAxis(
            legsAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            axis.as_mut_ptr(),
        );
        side = speed
            * (velocity[0] * axis[1][0] + velocity[1] * axis[1][1] + velocity[2] * axis[1][2]);
        legsAngles[2] -= side;
        side = speed
            * (velocity[0] * axis[0][0] + velocity[1] * axis[0][1] + velocity[2] * axis[0][2]);
        legsAngles[0] += side
    }
    //
    clientNum = (*cent).currentState.clientNum;
    if clientNum >= 0 && clientNum < 64 {
        ci = &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset(clientNum as isize) as *mut crate::cg_local_h::clientInfo_t;
        if (*ci).fixedlegs as u64 != 0 {
            legsAngles[1] = torsoAngles[1];
            legsAngles[0] = 0.0;
            legsAngles[2] = 0.0
        }
    }
    // pain twitch
    CG_AddPainTwitch(cent, torsoAngles.as_mut_ptr());
    // pull the angles back out of the hierarchial chain
    crate::src::qcommon::q_math::AnglesSubtract(
        headAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
        headAngles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AnglesSubtract(
        torsoAngles.as_mut_ptr(),
        legsAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
    );
    crate::src::qcommon::q_math::AnglesToAxis(
        legsAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        legs,
    );
    crate::src::qcommon::q_math::AnglesToAxis(
        torsoAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        torso,
    );
    crate::src::qcommon::q_math::AnglesToAxis(
        headAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        head,
    );
}
//==========================================================================
/*
===============
CG_HasteTrail
===============
*/

unsafe extern "C" fn CG_HasteTrail(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut smoke: *mut crate::cg_local_h::localEntity_t =
        0 as *mut crate::cg_local_h::localEntity_t;
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut anim: i32 = 0;
    if (*cent).trailTime > crate::src::cgame::cg_main::cg.time {
        return;
    }
    anim = (*cent).pe.legs.animationNumber & !(128);
    if anim != crate::bg_public_h::LEGS_RUN as i32 && anim != crate::bg_public_h::LEGS_BACK as i32 {
        return;
    }
    (*cent).trailTime += 100;
    if (*cent).trailTime < crate::src::cgame::cg_main::cg.time {
        (*cent).trailTime = crate::src::cgame::cg_main::cg.time
    }
    origin[0] = (*cent).lerpOrigin[0];
    origin[1] = (*cent).lerpOrigin[1];
    origin[2] = (*cent).lerpOrigin[2];
    origin[2] -= 16f32;
    smoke = crate::src::cgame::cg_effects::CG_SmokePuff(
        origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
            as *const crate::src::qcommon::q_shared::vec_t,
        8f32,
        1f32,
        1f32,
        1f32,
        1f32,
        500f32,
        crate::src::cgame::cg_main::cg.time,
        0,
        0,
        crate::src::cgame::cg_main::cgs.media.hastePuffShader,
    );
    // use the optimized local entity add
    (*smoke).leType = crate::cg_local_h::LE_SCALE_FADE;
}
/*
===============
CG_TrailItem
===============
*/

unsafe extern "C" fn CG_TrailItem(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut hModel: crate::src::qcommon::q_shared::qhandle_t,
) {
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
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut axis: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    angles[0] = (*cent).lerpAngles[0];
    angles[1] = (*cent).lerpAngles[1];
    angles[2] = (*cent).lerpAngles[2];
    angles[0] = 0f32;
    angles[2] = 0f32;
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        axis.as_mut_ptr(),
    );
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = (*cent).lerpOrigin[0] + axis[0][0] * -16f32;
    ent.origin[1] = (*cent).lerpOrigin[1] + axis[0][1] * -16f32;
    ent.origin[2] = (*cent).lerpOrigin[2] + axis[0][2] * -16f32;
    ent.origin[2] += 16f32;
    angles[1] += 90f32;
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ent.axis.as_mut_ptr(),
    );
    ent.hModel = hModel;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_PlayerFlag
===============
*/

unsafe extern "C" fn CG_PlayerFlag(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut hSkin: crate::src::qcommon::q_shared::qhandle_t,
    mut torso: *mut crate::tr_types_h::refEntity_t,
) {
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut pole: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut flag: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut legsAnim: i32 = 0;
    let mut flagAnim: i32 = 0;
    let mut updateangles: i32 = 0;
    let mut angle: f32 = 0.;
    let mut d: f32 = 0.;
    // show the flag pole model
    crate::stdlib::memset(
        &mut pole as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    pole.hModel = crate::src::cgame::cg_main::cgs.media.flagPoleModel;
    pole.lightingOrigin[0] = (*torso).lightingOrigin[0];
    pole.lightingOrigin[1] = (*torso).lightingOrigin[1];
    pole.lightingOrigin[2] = (*torso).lightingOrigin[2];
    pole.shadowPlane = (*torso).shadowPlane;
    pole.renderfx = (*torso).renderfx;
    crate::src::cgame::cg_ents::CG_PositionEntityOnTag(
        &mut pole,
        torso,
        (*torso).hModel,
        b"tag_flag\x00" as *const u8 as *mut i8,
    );
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut pole);
    // show the flag model
    crate::stdlib::memset(
        &mut flag as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    flag.hModel = crate::src::cgame::cg_main::cgs.media.flagFlapModel;
    flag.customSkin = hSkin;
    flag.lightingOrigin[0] = (*torso).lightingOrigin[0];
    flag.lightingOrigin[1] = (*torso).lightingOrigin[1];
    flag.lightingOrigin[2] = (*torso).lightingOrigin[2];
    flag.shadowPlane = (*torso).shadowPlane;
    flag.renderfx = (*torso).renderfx;
    angles[2] = 0f32;
    angles[1] = angles[2];
    angles[0] = angles[1];
    updateangles = crate::src::qcommon::q_shared::qfalse as i32;
    legsAnim = (*cent).currentState.legsAnim & !(128);
    if legsAnim == crate::bg_public_h::LEGS_IDLE as i32
        || legsAnim == crate::bg_public_h::LEGS_IDLECR as i32
    {
        flagAnim = crate::bg_public_h::FLAG_STAND as i32
    } else if legsAnim == crate::bg_public_h::LEGS_WALK as i32
        || legsAnim == crate::bg_public_h::LEGS_WALKCR as i32
    {
        flagAnim = crate::bg_public_h::FLAG_STAND as i32;
        updateangles = crate::src::qcommon::q_shared::qtrue as i32
    } else {
        flagAnim = crate::bg_public_h::FLAG_RUN as i32;
        updateangles = crate::src::qcommon::q_shared::qtrue as i32
    }
    if updateangles != 0 {
        dir[0] = (*cent).currentState.pos.trDelta[0];
        dir[1] = (*cent).currentState.pos.trDelta[1];
        dir[2] = (*cent).currentState.pos.trDelta[2];
        /*
        d = DotProduct(pole.axis[2], dir);
        angle = Q_acos(d);

        d = DotProduct(pole.axis[1], dir);
        if (d < 0) {
            angle = 360 - angle * 180 / M_PI;
        }
        else {
            angle = angle * 180 / M_PI;
        }
        if (angle > 340 && angle < 20) {
            flagAnim = FLAG_RUNUP;
        }
        if (angle > 160 && angle < 200) {
            flagAnim = FLAG_RUNDOWN;
        }
        */
        dir[2] += 100f32;
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        d = pole.axis[2][0] * dir[0] + pole.axis[2][1] * dir[1] + pole.axis[2][2] * dir[2];
        if crate::stdlib::fabs(d as f64) < 0.9 {
            // add gravity
            // if there is enough movement orthogonal to the flag pole
            //
            d = pole.axis[0][0] * dir[0] + pole.axis[0][1] * dir[1] + pole.axis[0][2] * dir[2];
            if d > 1.0 {
                d = 1.0
            } else if d < -1.0 {
                d = -1.0
            }
            angle = crate::stdlib::acos(d as f64) as f32;
            d = pole.axis[1][0] * dir[0] + pole.axis[1][1] * dir[1] + pole.axis[1][2] * dir[2];
            if d < 0f32 {
                angles[1] = (360f64 - (angle * 180f32) as f64 / 3.14159265358979323846)
                    as crate::src::qcommon::q_shared::vec_t
            } else {
                angles[1] = ((angle * 180f32) as f64 / 3.14159265358979323846)
                    as crate::src::qcommon::q_shared::vec_t
            }
            if angles[1] < 0f32 {
                angles[1] += 360f32
            }
            if angles[1] > 360f32 {
                angles[1] -= 360f32
            }
            //vectoangles( cent->currentState.pos.trDelta, tmpangles );
            //angles[YAW] = tmpangles[YAW] + 45 - cent->pe.torso.yawAngle;
            // change the yaw angle
            CG_SwingAngles(
                angles[1usize],
                25f32,
                90f32,
                0.15f32,
                &mut (*cent).pe.flag.yawAngle,
                &mut (*cent).pe.flag.yawing,
            );
        }
    }
    // set the yaw angle
    angles[1] = (*cent).pe.flag.yawAngle;
    // lerp the flag animation frames
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset((*cent).currentState.clientNum as isize)
        as *mut crate::cg_local_h::clientInfo_t;
    CG_RunLerpFrame(ci, &mut (*cent).pe.flag, flagAnim, 1f32);
    flag.oldframe = (*cent).pe.flag.oldFrame;
    flag.frame = (*cent).pe.flag.frame;
    flag.backlerp = (*cent).pe.flag.backlerp;
    crate::src::qcommon::q_math::AnglesToAxis(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        flag.axis.as_mut_ptr(),
    );
    crate::src::cgame::cg_ents::CG_PositionRotatedEntityOnTag(
        &mut flag,
        &mut pole,
        pole.hModel,
        b"tag_flag\x00" as *const u8 as *mut i8,
    );
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut flag);
}
/*
===============
CG_PlayerPowerups
===============
*/

unsafe extern "C" fn CG_PlayerPowerups(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut torso: *mut crate::tr_types_h::refEntity_t,
) {
    let mut powerups: i32 = 0;
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    powerups = (*cent).currentState.powerups;
    if powerups == 0 {
        return;
    }
    // quad gives a dlight
    if powerups & (1) << crate::bg_public_h::PW_QUAD as i32 != 0 {
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (200i32 + (crate::stdlib::rand() & 31i32)) as f32,
            0.2f32,
            0.2f32,
            1f32,
        );
    }
    // flight plays a looped sound
    if powerups & (1) << crate::bg_public_h::PW_FLIGHT as i32 != 0 {
        crate::src::cgame::cg_syscalls::trap_S_AddLoopingSound(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            crate::src::qcommon::q_math::vec3_origin.as_mut_ptr()
                as *const crate::src::qcommon::q_shared::vec_t,
            crate::src::cgame::cg_main::cgs.media.flightSound,
        );
    }
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset((*cent).currentState.clientNum as isize)
        as *mut crate::cg_local_h::clientInfo_t;
    // redflag
    if powerups & (1) << crate::bg_public_h::PW_REDFLAG as i32 != 0 {
        if (*ci).newAnims as u64 != 0 {
            CG_PlayerFlag(
                cent,
                crate::src::cgame::cg_main::cgs.media.redFlagFlapSkin,
                torso,
            );
        } else {
            CG_TrailItem(cent, crate::src::cgame::cg_main::cgs.media.redFlagModel);
        }
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (200i32 + (crate::stdlib::rand() & 31i32)) as f32,
            1f32,
            0.2f32,
            0.2f32,
        );
    }
    // blueflag
    if powerups & (1) << crate::bg_public_h::PW_BLUEFLAG as i32 != 0 {
        if (*ci).newAnims as u64 != 0 {
            CG_PlayerFlag(
                cent,
                crate::src::cgame::cg_main::cgs.media.blueFlagFlapSkin,
                torso,
            );
        } else {
            CG_TrailItem(cent, crate::src::cgame::cg_main::cgs.media.blueFlagModel);
        }
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (200i32 + (crate::stdlib::rand() & 31i32)) as f32,
            0.2f32,
            0.2f32,
            1f32,
        );
    }
    // neutralflag
    if powerups & (1) << crate::bg_public_h::PW_NEUTRALFLAG as i32 != 0 {
        if (*ci).newAnims as u64 != 0 {
            CG_PlayerFlag(
                cent,
                crate::src::cgame::cg_main::cgs.media.neutralFlagFlapSkin,
                torso,
            );
        } else {
            CG_TrailItem(cent, crate::src::cgame::cg_main::cgs.media.neutralFlagModel);
        }
        crate::src::cgame::cg_syscalls::trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (200i32 + (crate::stdlib::rand() & 31i32)) as f32,
            1f32,
            1f32,
            1f32,
        );
    }
    // haste leaves smoke trails
    if powerups & (1) << crate::bg_public_h::PW_HASTE as i32 != 0 {
        CG_HasteTrail(cent);
    };
}
/*
===============
CG_PlayerFloatSprite

Float a sprite over the player's head
===============
*/

unsafe extern "C" fn CG_PlayerFloatSprite(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut shader: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut rf: i32 = 0;
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
    if (*cent).currentState.number == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum
        && crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 == 0
    {
        rf = 0x2
    // only show in mirrors
    } else {
        rf = 0
    }
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = (*cent).lerpOrigin[0];
    ent.origin[1] = (*cent).lerpOrigin[1];
    ent.origin[2] = (*cent).lerpOrigin[2];
    ent.origin[2] += 48f32;
    ent.reType = crate::tr_types_h::RT_SPRITE;
    ent.customShader = shader;
    ent.radius = 10f32;
    ent.renderfx = rf;
    ent.shaderRGBA[0] = 255;
    ent.shaderRGBA[1] = 255;
    ent.shaderRGBA[2] = 255;
    ent.shaderRGBA[3] = 255;
    crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_PlayerSprites

Float sprites over the player's head
===============
*/

unsafe extern "C" fn CG_PlayerSprites(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut team: i32 = 0;
    if (*cent).currentState.eFlags & 0x2000 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.connectionShader);
        return;
    }
    if (*cent).currentState.eFlags & 0x1000 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.balloonShader);
        return;
    }
    if (*cent).currentState.eFlags & 0x8000 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.medalImpressive);
        return;
    }
    if (*cent).currentState.eFlags & 0x8 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.medalExcellent);
        return;
    }
    if (*cent).currentState.eFlags & 0x40 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.medalGauntlet);
        return;
    }
    if (*cent).currentState.eFlags & 0x10000 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.medalDefend);
        return;
    }
    if (*cent).currentState.eFlags & 0x20000 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.medalAssist);
        return;
    }
    if (*cent).currentState.eFlags & 0x800 != 0 {
        CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.medalCapture);
        return;
    }
    team = crate::src::cgame::cg_main::cgs.clientinfo[(*cent).currentState.clientNum as usize].team
        as i32;
    if (*cent).currentState.eFlags & 0x1 == 0
        && (*crate::src::cgame::cg_main::cg.snap).ps.persistant
            [crate::bg_public_h::PERS_TEAM as usize]
            == team
        && crate::src::cgame::cg_main::cgs.gametype >= crate::bg_public_h::GT_TEAM
    {
        if crate::src::cgame::cg_main::cg_drawFriend.integer != 0 {
            CG_PlayerFloatSprite(cent, crate::src::cgame::cg_main::cgs.media.friendShader);
        }
        return;
    };
}

unsafe extern "C" fn CG_PlayerShadow(
    mut cent: *mut crate::cg_local_h::centity_t,
    mut shadowPlane: *mut f32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [-15f32, -15f32, 0f32];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [15f32, 15f32, 2f32];
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
    let mut alpha: f32 = 0.;
    *shadowPlane = 0f32;
    if crate::src::cgame::cg_main::cg_shadows.integer == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // no shadows when invisible
    if (*cent).currentState.powerups & (1) << crate::bg_public_h::PW_INVIS as i32 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // send a trace down from the player to the ground
    end[0] = (*cent).lerpOrigin[0];
    end[1] = (*cent).lerpOrigin[1];
    end[2] = (*cent).lerpOrigin[2];
    end[2] -= 128f32;
    crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
        &mut trace,
        (*cent).lerpOrigin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        mins.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        maxs.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0,
        1 | 0x10000 | 0x2000000,
    );
    // no shadow if too high
    if trace.fraction as f64 == 1.0 || trace.startsolid != 0 || trace.allsolid != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    *shadowPlane = trace.endpos[2] + 1f32;
    if crate::src::cgame::cg_main::cg_shadows.integer != 1 {
        // no mark for stencil or projection shadows
        return crate::src::qcommon::q_shared::qtrue;
    }
    // fade the shadow out with height
    alpha = (1.0 - trace.fraction as f64) as f32;
    // hack / FPE - bogus planes?
    //assert( DotProduct( trace.plane.normal, trace.plane.normal ) != 0.0f )
    // add the mark as a temporary, so it goes directly to the renderer
    // without taking a spot in the cg_marks array
    crate::src::cgame::cg_marks::CG_ImpactMark(
        crate::src::cgame::cg_main::cgs.media.shadowMarkShader,
        trace.endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        trace.plane.normal.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*cent).pe.legs.yawAngle,
        alpha,
        alpha,
        alpha,
        1f32,
        crate::src::qcommon::q_shared::qfalse,
        24f32,
        crate::src::qcommon::q_shared::qtrue,
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
CG_PlayerSplash

Draw a mark at the water surface
===============
*/

unsafe extern "C" fn CG_PlayerSplash(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
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
    let mut contents: i32 = 0;
    let mut verts: [crate::tr_types_h::polyVert_t; 4] = [crate::tr_types_h::polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    if crate::src::cgame::cg_main::cg_shadows.integer == 0 {
        return;
    }
    end[0] = (*cent).lerpOrigin[0];
    end[1] = (*cent).lerpOrigin[1];
    end[2] = (*cent).lerpOrigin[2];
    end[2] -= 24f32;
    // if the feet aren't in liquid, don't make a mark
    // this won't handle moving water brushes, but they wouldn't draw right anyway...
    contents = crate::src::cgame::cg_predict::CG_PointContents(
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0,
    );
    if contents & (32 | 16 | 8) == 0 {
        return;
    }
    start[0] = (*cent).lerpOrigin[0];
    start[1] = (*cent).lerpOrigin[1];
    start[2] = (*cent).lerpOrigin[2];
    start[2] += 32f32;
    // if the head isn't out of liquid, don't make a mark
    contents = crate::src::cgame::cg_predict::CG_PointContents(
        start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0,
    );
    if contents & (1 | 32 | 16 | 8) != 0 {
        return;
    }
    // trace down to find the surface
    crate::src::cgame::cg_syscalls::trap_CM_BoxTrace(
        &mut trace,
        start.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        end.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0 as *const crate::src::qcommon::q_shared::vec_t,
        0,
        32 | 16 | 8,
    );
    if trace.fraction as f64 == 1.0 {
        return;
    }
    // create a mark polygon
    verts[0].xyz[0] = trace.endpos[0];
    verts[0].xyz[1] = trace.endpos[1];
    verts[0].xyz[2] = trace.endpos[2];
    verts[0].xyz[0] -= 32f32;
    verts[0].xyz[1] -= 32f32;
    verts[0].st[0] = 0f32;
    verts[0].st[1] = 0f32;
    verts[0].modulate[0] = 255;
    verts[0].modulate[1] = 255;
    verts[0].modulate[2] = 255;
    verts[0].modulate[3] = 255;
    verts[1].xyz[0] = trace.endpos[0];
    verts[1].xyz[1] = trace.endpos[1];
    verts[1].xyz[2] = trace.endpos[2];
    verts[1].xyz[0] -= 32f32;
    verts[1].xyz[1] += 32f32;
    verts[1].st[0] = 0f32;
    verts[1].st[1] = 1f32;
    verts[1].modulate[0] = 255;
    verts[1].modulate[1] = 255;
    verts[1].modulate[2] = 255;
    verts[1].modulate[3] = 255;
    verts[2].xyz[0] = trace.endpos[0];
    verts[2].xyz[1] = trace.endpos[1];
    verts[2].xyz[2] = trace.endpos[2];
    verts[2].xyz[0] += 32f32;
    verts[2].xyz[1] += 32f32;
    verts[2].st[0] = 1f32;
    verts[2].st[1] = 1f32;
    verts[2].modulate[0] = 255;
    verts[2].modulate[1] = 255;
    verts[2].modulate[2] = 255;
    verts[2].modulate[3] = 255;
    verts[3].xyz[0] = trace.endpos[0];
    verts[3].xyz[1] = trace.endpos[1];
    verts[3].xyz[2] = trace.endpos[2];
    verts[3].xyz[0] += 32f32;
    verts[3].xyz[1] -= 32f32;
    verts[3].st[0] = 1f32;
    verts[3].st[1] = 0f32;
    verts[3].modulate[0] = 255;
    verts[3].modulate[1] = 255;
    verts[3].modulate[2] = 255;
    verts[3].modulate[3] = 255;
    crate::src::cgame::cg_syscalls::trap_R_AddPolyToScene(
        crate::src::cgame::cg_main::cgs.media.wakeMarkShader,
        4,
        verts.as_mut_ptr(),
    );
}
/*
===============
CG_AddRefEntityWithPowerups

Adds a piece with modifications or duplications for powerups
Also called by CG_Missile for quad rockets, but nobody can tell...
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_AddRefEntityWithPowerups(
    mut ent: *mut crate::tr_types_h::refEntity_t,
    mut state: *mut crate::src::qcommon::q_shared::entityState_t,
    mut team: i32,
) {
    if (*state).powerups & (1) << crate::bg_public_h::PW_INVIS as i32 != 0 {
        (*ent).customShader = crate::src::cgame::cg_main::cgs.media.invisShader;
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(ent);
    } else {
        /*
        if ( state->eFlags & EF_KAMIKAZE ) {
            if (team == TEAM_BLUE)
                ent->customShader = cgs.media.blueKamikazeShader;
            else
                ent->customShader = cgs.media.redKamikazeShader;
            trap_R_AddRefEntityToScene( ent );
        }
        else {*/
        crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(ent);
        //}
        if (*state).powerups & (1) << crate::bg_public_h::PW_QUAD as i32 != 0 {
            if team == crate::bg_public_h::TEAM_RED as i32 {
                (*ent).customShader = crate::src::cgame::cg_main::cgs.media.redQuadShader
            } else {
                (*ent).customShader = crate::src::cgame::cg_main::cgs.media.quadShader
            }
            crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(ent);
        }
        if (*state).powerups & (1) << crate::bg_public_h::PW_REGEN as i32 != 0 {
            if crate::src::cgame::cg_main::cg.time / 100 % 10 == 1 {
                (*ent).customShader = crate::src::cgame::cg_main::cgs.media.regenShader;
                crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(ent);
            }
        }
        if (*state).powerups & (1) << crate::bg_public_h::PW_BATTLESUIT as i32 != 0 {
            (*ent).customShader = crate::src::cgame::cg_main::cgs.media.battleSuitShader;
            crate::src::cgame::cg_syscalls::trap_R_AddRefEntityToScene(ent);
        }
    };
}
/*
=================
CG_LightVerts
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_LightVerts(
    mut normal: *mut crate::src::qcommon::q_shared::vec_t,
    mut numVerts: i32,
    mut verts: *mut crate::tr_types_h::polyVert_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut incoming: f32 = 0.;
    let mut ambientLight: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut lightDir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut directedLight: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::cgame::cg_syscalls::trap_R_LightForPoint(
        (*verts.offset(0)).xyz.as_mut_ptr(),
        ambientLight.as_mut_ptr(),
        directedLight.as_mut_ptr(),
        lightDir.as_mut_ptr(),
    );

    for i in 0..numVerts {
        incoming = *normal.offset(0) * lightDir[0]
            + *normal.offset(1) * lightDir[1]
            + *normal.offset(2) * lightDir[2];

        if incoming <= 0f32 {
            (*verts.offset(i as isize)).modulate[0] =
                ambientLight[0] as crate::src::qcommon::q_shared::byte;
            (*verts.offset(i as isize)).modulate[1] =
                ambientLight[1] as crate::src::qcommon::q_shared::byte;
            (*verts.offset(i as isize)).modulate[2] =
                ambientLight[2] as crate::src::qcommon::q_shared::byte;
            (*verts.offset(i as isize)).modulate[3] = 255u8
        } else {
            j = (ambientLight[0] + incoming * directedLight[0]) as i32;
            if j > 255 {
                j = 255
            }
            (*verts.offset(i as isize)).modulate[0] = j as crate::src::qcommon::q_shared::byte;
            j = (ambientLight[1] + incoming * directedLight[1]) as i32;
            if j > 255 {
                j = 255
            }
            (*verts.offset(i as isize)).modulate[1] = j as crate::src::qcommon::q_shared::byte;
            j = (ambientLight[2] + incoming * directedLight[2]) as i32;
            if j > 255 {
                j = 255
            }
            (*verts.offset(i as isize)).modulate[2] = j as crate::src::qcommon::q_shared::byte;
            (*verts.offset(i as isize)).modulate[3] = 255u8
        }
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
===============
CG_Player
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_Player(mut cent: *mut crate::cg_local_h::centity_t) {
    let mut ci: *mut crate::cg_local_h::clientInfo_t = 0 as *mut crate::cg_local_h::clientInfo_t;
    let mut legs: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut torso: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut head: crate::tr_types_h::refEntity_t = crate::tr_types_h::refEntity_t {
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
    let mut clientNum: i32 = 0;
    let mut renderfx: i32 = 0;
    let mut shadow: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut shadowPlane: f32 = 0.;
    // the client number is stored in clientNum.  It can't be derived
    // from the entity number, because a single client may have
    // multiple corpses on the level using the same clientinfo
    clientNum = (*cent).currentState.clientNum;
    if clientNum < 0 || clientNum >= 64 {
        crate::src::cgame::cg_main::CG_Error(
            b"Bad clientNum on player entity\x00" as *const u8 as *const i8,
        );
    }
    ci = &mut *crate::src::cgame::cg_main::cgs
        .clientinfo
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::cg_local_h::clientInfo_t;
    // it is possible to see corpses from disconnected players that may
    // not have valid clientinfo
    if (*ci).infoValid as u64 == 0 {
        return;
    }
    // get the player model information
    renderfx = 0;
    if (*cent).currentState.number == (*crate::src::cgame::cg_main::cg.snap).ps.clientNum {
        if crate::src::cgame::cg_main::cg.renderingThirdPerson as u64 == 0 {
            renderfx = 0x2
        // only draw in mirrors
        } else if crate::src::cgame::cg_main::cg_cameraMode.integer != 0 {
            return;
        }
    }
    crate::stdlib::memset(
        &mut legs as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    crate::stdlib::memset(
        &mut torso as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    crate::stdlib::memset(
        &mut head as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    // get the rotation information
    CG_PlayerAngles(
        cent,
        legs.axis.as_mut_ptr(),
        torso.axis.as_mut_ptr(),
        head.axis.as_mut_ptr(),
    );
    // get the animation state (after rotation, to allow feet shuffle)
    CG_PlayerAnimation(
        cent,
        &mut legs.oldframe,
        &mut legs.frame,
        &mut legs.backlerp,
        &mut torso.oldframe,
        &mut torso.frame,
        &mut torso.backlerp,
    );
    // add the talk baloon or disconnect icon
    CG_PlayerSprites(cent);
    // add the shadow
    shadow = CG_PlayerShadow(cent, &mut shadowPlane);
    // add a water splash if partially in and out of water
    CG_PlayerSplash(cent); // use the same origin for all
    if crate::src::cgame::cg_main::cg_shadows.integer == 3 && shadow != 0 {
        renderfx |= 0x100
    }
    renderfx |= 0x80;
    //
    // add the legs
    //
    legs.hModel = (*ci).legsModel; // don't positionally lerp at all
    legs.customSkin = (*ci).legsSkin;
    legs.origin[0] = (*cent).lerpOrigin[0];
    legs.origin[1] = (*cent).lerpOrigin[1];
    legs.origin[2] = (*cent).lerpOrigin[2];
    legs.lightingOrigin[0] = (*cent).lerpOrigin[0];
    legs.lightingOrigin[1] = (*cent).lerpOrigin[1];
    legs.lightingOrigin[2] = (*cent).lerpOrigin[2];
    legs.shadowPlane = shadowPlane;
    legs.renderfx = renderfx;
    legs.oldorigin[0] = legs.origin[0];
    legs.oldorigin[1] = legs.origin[1];
    legs.oldorigin[2] = legs.origin[2];
    CG_AddRefEntityWithPowerups(&mut legs, &mut (*cent).currentState, (*ci).team as i32);
    // if the model failed, allow the default nullmodel to be displayed
    if legs.hModel == 0 {
        return;
    }
    //
    // add the torso
    //
    torso.hModel = (*ci).torsoModel;
    if torso.hModel == 0 {
        return;
    }
    torso.customSkin = (*ci).torsoSkin;
    torso.lightingOrigin[0] = (*cent).lerpOrigin[0];
    torso.lightingOrigin[1] = (*cent).lerpOrigin[1];
    torso.lightingOrigin[2] = (*cent).lerpOrigin[2];
    crate::src::cgame::cg_ents::CG_PositionRotatedEntityOnTag(
        &mut torso,
        &mut legs,
        (*ci).legsModel,
        b"tag_torso\x00" as *const u8 as *mut i8,
    );
    torso.shadowPlane = shadowPlane;
    torso.renderfx = renderfx;
    CG_AddRefEntityWithPowerups(&mut torso, &mut (*cent).currentState, (*ci).team as i32);
    // MISSIONPACK
    //
    // add the head
    //
    head.hModel = (*ci).headModel;
    if head.hModel == 0 {
        return;
    }
    head.customSkin = (*ci).headSkin;
    head.lightingOrigin[0] = (*cent).lerpOrigin[0];
    head.lightingOrigin[1] = (*cent).lerpOrigin[1];
    head.lightingOrigin[2] = (*cent).lerpOrigin[2];
    crate::src::cgame::cg_ents::CG_PositionRotatedEntityOnTag(
        &mut head,
        &mut torso,
        (*ci).torsoModel,
        b"tag_head\x00" as *const u8 as *mut i8,
    );
    head.shadowPlane = shadowPlane;
    head.renderfx = renderfx;
    CG_AddRefEntityWithPowerups(&mut head, &mut (*cent).currentState, (*ci).team as i32);
    //
    // add the gun / barrel / flash
    //
    crate::src::cgame::cg_weapons::CG_AddPlayerWeapon(
        &mut torso,
        0 as *mut crate::src::qcommon::q_shared::playerState_t,
        cent,
        (*ci).team as i32,
    );
    // add powerups floating behind the player
    CG_PlayerPowerups(cent, &mut torso);
}
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//=====================================================================
/*
===============
CG_ResetPlayerEntity

A player just came into view or teleported, so reset all animation info
===============
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ResetPlayerEntity(mut cent: *mut crate::cg_local_h::centity_t) {
    (*cent).errorTime = -(99999); // guarantee no error decay added
    (*cent).extrapolated = crate::src::qcommon::q_shared::qfalse;
    CG_ClearLerpFrame(
        &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*cent).currentState.clientNum as isize),
        &mut (*cent).pe.legs,
        (*cent).currentState.legsAnim,
    );
    CG_ClearLerpFrame(
        &mut *crate::src::cgame::cg_main::cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*cent).currentState.clientNum as isize),
        &mut (*cent).pe.torso,
        (*cent).currentState.torsoAnim,
    );
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
    (*cent).rawOrigin[0] = (*cent).lerpOrigin[0];
    (*cent).rawOrigin[1] = (*cent).lerpOrigin[1];
    (*cent).rawOrigin[2] = (*cent).lerpOrigin[2];
    (*cent).rawAngles[0] = (*cent).lerpAngles[0];
    (*cent).rawAngles[1] = (*cent).lerpAngles[1];
    (*cent).rawAngles[2] = (*cent).lerpAngles[2];
    crate::stdlib::memset(
        &mut (*cent).pe.legs as *mut crate::cg_local_h::lerpFrame_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::cg_local_h::lerpFrame_t>(),
    );
    (*cent).pe.legs.yawAngle = (*cent).rawAngles[1];
    (*cent).pe.legs.yawing = crate::src::qcommon::q_shared::qfalse;
    (*cent).pe.legs.pitchAngle = 0f32;
    (*cent).pe.legs.pitching = crate::src::qcommon::q_shared::qfalse;
    crate::stdlib::memset(
        &mut (*cent).pe.torso as *mut crate::cg_local_h::lerpFrame_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::cg_local_h::lerpFrame_t>(),
    );
    (*cent).pe.torso.yawAngle = (*cent).rawAngles[1];
    (*cent).pe.torso.yawing = crate::src::qcommon::q_shared::qfalse;
    (*cent).pe.torso.pitchAngle = (*cent).rawAngles[0];
    (*cent).pe.torso.pitching = crate::src::qcommon::q_shared::qfalse;
    if crate::src::cgame::cg_main::cg_debugPosition.integer != 0 {
        crate::src::cgame::cg_main::CG_Printf(
            b"%i ResetPlayerEntity yaw=%f\n\x00" as *const u8 as *const i8,
            (*cent).currentState.number,
            (*cent).pe.torso.yawAngle as f64,
        );
    };
}
