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
pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::weapon_t;
pub use crate::bg_public_h::BOTH_DEAD1;
pub use crate::bg_public_h::BOTH_DEAD2;
pub use crate::bg_public_h::BOTH_DEAD3;
pub use crate::bg_public_h::BOTH_DEATH1;
pub use crate::bg_public_h::BOTH_DEATH2;
pub use crate::bg_public_h::BOTH_DEATH3;
pub use crate::bg_public_h::FLAG_RUN;
pub use crate::bg_public_h::FLAG_STAND;
pub use crate::bg_public_h::FLAG_STAND2RUN;
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
pub use crate::src::game::bg_misc::bg_itemlist;
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::Com_Printf;
pub use crate::src::q3_ui::ui_atoms::UI_AdjustFrom640;
pub use crate::src::q3_ui::ui_players::stdlib_float_h::atof;
pub use crate::src::q3_ui::ui_players::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_qmenu::weaponChangeSound;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_math::AngleMod;
pub use crate::src::qcommon::q_math::AngleSubtract;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::AnglesSubtract;
pub use crate::src::qcommon::q_math::AnglesToAxis;
pub use crate::src::qcommon::q_math::AxisClear;
pub use crate::src::qcommon::q_math::MatrixMultiply;
pub use crate::src::qcommon::q_math::Q_fabs;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::clipHandle_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::orientation_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_StripExtension;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::ui::ui_syscalls::trap_CM_LerpTag;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_Error;
pub use crate::src::ui::ui_syscalls::trap_FS_FCloseFile;
pub use crate::src::ui::ui_syscalls::trap_FS_FOpenFile;
pub use crate::src::ui::ui_syscalls::trap_FS_Read;
pub use crate::src::ui::ui_syscalls::trap_R_AddLightToScene;
pub use crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene;
pub use crate::src::ui::ui_syscalls::trap_R_ClearScene;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterModel;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterSkin;
pub use crate::src::ui::ui_syscalls::trap_R_RenderScene;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;
use crate::stdlib::atan2;
use crate::stdlib::fabs;
use crate::stdlib::memset;
pub use crate::stdlib::rand;
use crate::stdlib::sin;
use crate::stdlib::strchr;
pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
use crate::stdlib::tan;
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
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::lerpFrame_t;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::playerInfo_t;
pub use crate::ui_local_h::uiStatic_t;

static mut dp_realtime: i32 = 0;

static mut jumpHeight: f32 = 0.;
/*
===============
UI_PlayerInfo_SetWeapon
===============
*/

unsafe extern "C" fn UI_PlayerInfo_SetWeapon(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut weaponNum: crate::bg_public_h::weapon_t,
) {
    let mut item: *mut crate::bg_public_h::gitem_t = 0 as *mut crate::bg_public_h::gitem_t;
    let mut path: [i8; 64] = [0; 64];
    (*pi).currentWeapon = weaponNum;
    loop {
        (*pi).realWeapon = weaponNum as i32;
        (*pi).weaponModel = 0;
        (*pi).barrelModel = 0;
        (*pi).flashModel = 0;
        if weaponNum == crate::bg_public_h::WP_NONE {
            return;
        }
        item = crate::src::game::bg_misc::bg_itemlist
            .as_mut_ptr()
            .offset(1);
        while !(*item).classname.is_null() {
            if !((*item).giType != crate::bg_public_h::IT_WEAPON) {
                if (*item).giTag as u32 == weaponNum {
                    break;
                }
            }
            item = item.offset(1)
        }
        if !(*item).classname.is_null() {
            (*pi).weaponModel =
                crate::src::ui::ui_syscalls::trap_R_RegisterModel((*item).world_model[0])
        }
        if !((*pi).weaponModel == 0) {
            break;
        }
        if weaponNum == crate::bg_public_h::WP_MACHINEGUN {
            weaponNum = crate::bg_public_h::WP_NONE
        } else {
            weaponNum = crate::bg_public_h::WP_MACHINEGUN
        }
    }
    if weaponNum == crate::bg_public_h::WP_MACHINEGUN
        || weaponNum == crate::bg_public_h::WP_GAUNTLET
        || weaponNum == crate::bg_public_h::WP_BFG
    {
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
        (*pi).barrelModel = crate::src::ui::ui_syscalls::trap_R_RegisterModel(path.as_mut_ptr())
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
    (*pi).flashModel = crate::src::ui::ui_syscalls::trap_R_RegisterModel(path.as_mut_ptr());
    match weaponNum {
        1 => {
            (*pi).flashDlightColor[0] = 0.6f32;
            (*pi).flashDlightColor[1] = 0.6f32;
            (*pi).flashDlightColor[2] = 1f32
        }
        2 => {
            (*pi).flashDlightColor[0] = 1f32;
            (*pi).flashDlightColor[1] = 1f32;
            (*pi).flashDlightColor[2] = 0f32
        }
        3 => {
            (*pi).flashDlightColor[0] = 1f32;
            (*pi).flashDlightColor[1] = 1f32;
            (*pi).flashDlightColor[2] = 0f32
        }
        4 => {
            (*pi).flashDlightColor[0] = 1f32;
            (*pi).flashDlightColor[1] = 0.7f32;
            (*pi).flashDlightColor[2] = 0.5f32
        }
        5 => {
            (*pi).flashDlightColor[0] = 1f32;
            (*pi).flashDlightColor[1] = 0.75f32;
            (*pi).flashDlightColor[2] = 0f32
        }
        6 => {
            (*pi).flashDlightColor[0] = 0.6f32;
            (*pi).flashDlightColor[1] = 0.6f32;
            (*pi).flashDlightColor[2] = 1f32
        }
        7 => {
            (*pi).flashDlightColor[0] = 1f32;
            (*pi).flashDlightColor[1] = 0.5f32;
            (*pi).flashDlightColor[2] = 0f32
        }
        8 => {
            (*pi).flashDlightColor[0] = 0.6f32;
            (*pi).flashDlightColor[1] = 0.6f32;
            (*pi).flashDlightColor[2] = 1f32
        }
        9 => {
            (*pi).flashDlightColor[0] = 1f32;
            (*pi).flashDlightColor[1] = 0.7f32;
            (*pi).flashDlightColor[2] = 1f32
        }
        10 => {
            (*pi).flashDlightColor[0] = 0.6f32;
            (*pi).flashDlightColor[1] = 0.6f32;
            (*pi).flashDlightColor[2] = 1f32
        }
        _ => {
            (*pi).flashDlightColor[0] = 1f32;
            (*pi).flashDlightColor[1] = 1f32;
            (*pi).flashDlightColor[2] = 1f32
        }
    };
}
/*
===============
UI_ForceLegsAnim
===============
*/

unsafe extern "C" fn UI_ForceLegsAnim(mut pi: *mut crate::ui_local_h::playerInfo_t, mut anim: i32) {
    (*pi).legsAnim = (*pi).legsAnim & 128 ^ 128 | anim;
    if anim == crate::bg_public_h::LEGS_JUMP as i32 {
        (*pi).legsAnimationTimer = 1000
    };
}
/*
===============
UI_SetLegsAnim
===============
*/

unsafe extern "C" fn UI_SetLegsAnim(mut pi: *mut crate::ui_local_h::playerInfo_t, mut anim: i32) {
    if (*pi).pendingLegsAnim != 0 {
        anim = (*pi).pendingLegsAnim;
        (*pi).pendingLegsAnim = 0
    }
    UI_ForceLegsAnim(pi, anim);
}
/*
===============
UI_ForceTorsoAnim
===============
*/

unsafe extern "C" fn UI_ForceTorsoAnim(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut anim: i32,
) {
    (*pi).torsoAnim = (*pi).torsoAnim & 128 ^ 128 | anim;
    if anim == crate::bg_public_h::TORSO_GESTURE as i32 {
        (*pi).torsoAnimationTimer = 2300
    }
    if anim == crate::bg_public_h::TORSO_ATTACK as i32
        || anim == crate::bg_public_h::TORSO_ATTACK2 as i32
    {
        (*pi).torsoAnimationTimer = 500
    };
}
/*
===============
UI_SetTorsoAnim
===============
*/

unsafe extern "C" fn UI_SetTorsoAnim(mut pi: *mut crate::ui_local_h::playerInfo_t, mut anim: i32) {
    if (*pi).pendingTorsoAnim != 0 {
        anim = (*pi).pendingTorsoAnim;
        (*pi).pendingTorsoAnim = 0
    }
    UI_ForceTorsoAnim(pi, anim);
}
/*
===============
UI_TorsoSequencing
===============
*/

unsafe extern "C" fn UI_TorsoSequencing(mut pi: *mut crate::ui_local_h::playerInfo_t) {
    let mut currentAnim: i32 = 0;
    currentAnim = (*pi).torsoAnim & !(128);
    if (*pi).weapon != (*pi).currentWeapon {
        if currentAnim != crate::bg_public_h::TORSO_DROP as i32 {
            (*pi).torsoAnimationTimer = 300;
            UI_ForceTorsoAnim(pi, crate::bg_public_h::TORSO_DROP as i32);
        }
    }
    if (*pi).torsoAnimationTimer > 0 {
        return;
    }
    if currentAnim == crate::bg_public_h::TORSO_GESTURE as i32 {
        UI_SetTorsoAnim(pi, crate::bg_public_h::TORSO_STAND as i32);
        return;
    }
    if currentAnim == crate::bg_public_h::TORSO_ATTACK as i32
        || currentAnim == crate::bg_public_h::TORSO_ATTACK2 as i32
    {
        UI_SetTorsoAnim(pi, crate::bg_public_h::TORSO_STAND as i32);
        return;
    }
    if currentAnim == crate::bg_public_h::TORSO_DROP as i32 {
        UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        (*pi).torsoAnimationTimer = 300;
        UI_ForceTorsoAnim(pi, crate::bg_public_h::TORSO_RAISE as i32);
        return;
    }
    if currentAnim == crate::bg_public_h::TORSO_RAISE as i32 {
        UI_SetTorsoAnim(pi, crate::bg_public_h::TORSO_STAND as i32);
        return;
    };
}
/*
===============
UI_LegsSequencing
===============
*/

unsafe extern "C" fn UI_LegsSequencing(mut pi: *mut crate::ui_local_h::playerInfo_t) {
    let mut currentAnim: i32 = 0;
    currentAnim = (*pi).legsAnim & !(128);
    if (*pi).legsAnimationTimer > 0 {
        if currentAnim == crate::bg_public_h::LEGS_JUMP as i32 {
            jumpHeight = (56f64
                * crate::stdlib::sin(
                    3.14159265358979323846 * (1000i32 - (*pi).legsAnimationTimer) as f64 / 1000f64,
                )) as f32
        }
        return;
    }
    if currentAnim == crate::bg_public_h::LEGS_JUMP as i32 {
        UI_ForceLegsAnim(pi, crate::bg_public_h::LEGS_LAND as i32);
        (*pi).legsAnimationTimer = 130;
        jumpHeight = 0f32;
        return;
    }
    if currentAnim == crate::bg_public_h::LEGS_LAND as i32 {
        UI_SetLegsAnim(pi, crate::bg_public_h::LEGS_IDLE as i32);
        return;
    };
}
/*
======================
UI_PositionEntityOnTag
======================
*/

unsafe extern "C" fn UI_PositionEntityOnTag(
    mut entity: *mut crate::tr_types_h::refEntity_t,
    mut parent: *const crate::tr_types_h::refEntity_t,
    mut parentModel: crate::src::qcommon::q_shared::clipHandle_t,
    mut tagName: *mut i8,
) {
    let mut i: i32 = 0;
    let mut lerped: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    // lerp the tag
    crate::src::ui::ui_syscalls::trap_CM_LerpTag(
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
    // cast away const because of compiler problems
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
UI_PositionRotatedEntityOnTag
======================
*/

unsafe extern "C" fn UI_PositionRotatedEntityOnTag(
    mut entity: *mut crate::tr_types_h::refEntity_t,
    mut parent: *const crate::tr_types_h::refEntity_t,
    mut parentModel: crate::src::qcommon::q_shared::clipHandle_t,
    mut tagName: *mut i8,
) {
    let mut i: i32 = 0;
    let mut lerped: crate::src::qcommon::q_shared::orientation_t =
        crate::src::qcommon::q_shared::orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
    let mut tempAxis: [crate::src::qcommon::q_shared::vec3_t; 3] = [[0.; 3]; 3];
    // lerp the tag
    crate::src::ui::ui_syscalls::trap_CM_LerpTag(
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
    // cast away const because of compiler problems
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
===============
UI_SetLerpFrameAnimation
===============
*/

unsafe extern "C" fn UI_SetLerpFrameAnimation(
    mut ci: *mut crate::ui_local_h::playerInfo_t,
    mut lf: *mut crate::ui_local_h::lerpFrame_t,
    mut newAnimation: i32,
) {
    let mut anim: *mut crate::bg_public_h::animation_t = 0 as *mut crate::bg_public_h::animation_t;
    (*lf).animationNumber = newAnimation;
    newAnimation &= !(128);
    if newAnimation < 0 || newAnimation >= crate::bg_public_h::MAX_ANIMATIONS as i32 {
        crate::src::ui::ui_syscalls::trap_Error(crate::src::qcommon::q_shared::va(
            b"Bad animation number: %i\x00" as *const u8 as *mut i8,
            newAnimation,
        ));
    }
    anim = &mut *(*ci).animations.as_mut_ptr().offset(newAnimation as isize)
        as *mut crate::bg_public_h::animation_t;
    (*lf).animation = anim;
    (*lf).animationTime = (*lf).frameTime + (*anim).initialLerp;
}
/*
===============
UI_RunLerpFrame
===============
*/

unsafe extern "C" fn UI_RunLerpFrame(
    mut ci: *mut crate::ui_local_h::playerInfo_t,
    mut lf: *mut crate::ui_local_h::lerpFrame_t,
    mut newAnimation: i32,
) {
    let mut f: i32 = 0;
    let mut numFrames: i32 = 0;
    let mut anim: *mut crate::bg_public_h::animation_t = 0 as *mut crate::bg_public_h::animation_t;
    // see if the animation sequence is switching
    if newAnimation != (*lf).animationNumber || (*lf).animation.is_null() {
        UI_SetLerpFrameAnimation(ci, lf, newAnimation);
    }
    // if we have passed the current frame, move it to
    // oldFrame and calculate a new frame
    if dp_realtime >= (*lf).frameTime {
        (*lf).oldFrame = (*lf).frame;
        (*lf).oldFrameTime = (*lf).frameTime;
        // get the next frame based on the animation
        anim = (*lf).animation;
        if (*anim).frameLerp == 0 {
            return;
            // shouldn't happen
        }
        if dp_realtime < (*lf).animationTime {
            (*lf).frameTime = (*lf).animationTime
        // initial lerp
        } else {
            (*lf).frameTime = (*lf).oldFrameTime + (*anim).frameLerp
        }
        f = ((*lf).frameTime - (*lf).animationTime) / (*anim).frameLerp;
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
                (*lf).frameTime = dp_realtime
            }
        }
        if (*anim).reversed != 0 {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1 - f
        } else if (*anim).flipflop != 0 && f >= (*anim).numFrames {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1 - f % (*anim).numFrames
        } else {
            (*lf).frame = (*anim).firstFrame + f
        }
        if dp_realtime > (*lf).frameTime {
            (*lf).frameTime = dp_realtime
        }
    }
    if (*lf).frameTime > dp_realtime + 200 {
        (*lf).frameTime = dp_realtime
    }
    if (*lf).oldFrameTime > dp_realtime {
        (*lf).oldFrameTime = dp_realtime
    }
    // calculate current lerp value
    if (*lf).frameTime == (*lf).oldFrameTime {
        (*lf).backlerp = 0f32
    } else {
        (*lf).backlerp =
            (1.0 - ((dp_realtime - (*lf).oldFrameTime) as f32
                / ((*lf).frameTime - (*lf).oldFrameTime) as f32) as f64) as f32
    };
}
/*
===============
UI_PlayerAnimation
===============
*/

unsafe extern "C" fn UI_PlayerAnimation(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut legsOld: *mut i32,
    mut legs: *mut i32,
    mut legsBackLerp: *mut f32,
    mut torsoOld: *mut i32,
    mut torso: *mut i32,
    mut torsoBackLerp: *mut f32,
) {
    // legs animation
    (*pi).legsAnimationTimer -= crate::src::q3_ui::ui_atoms::uis.frametime;
    if (*pi).legsAnimationTimer < 0 {
        (*pi).legsAnimationTimer = 0
    }
    UI_LegsSequencing(pi);
    if (*pi).legs.yawing != 0 && (*pi).legsAnim & !(128) == crate::bg_public_h::LEGS_IDLE as i32 {
        UI_RunLerpFrame(pi, &mut (*pi).legs, crate::bg_public_h::LEGS_TURN as i32);
    } else {
        UI_RunLerpFrame(pi, &mut (*pi).legs, (*pi).legsAnim);
    }
    *legsOld = (*pi).legs.oldFrame;
    *legs = (*pi).legs.frame;
    *legsBackLerp = (*pi).legs.backlerp;
    // torso animation
    (*pi).torsoAnimationTimer -= crate::src::q3_ui::ui_atoms::uis.frametime;
    if (*pi).torsoAnimationTimer < 0 {
        (*pi).torsoAnimationTimer = 0
    }
    UI_TorsoSequencing(pi);
    UI_RunLerpFrame(pi, &mut (*pi).torso, (*pi).torsoAnim);
    *torsoOld = (*pi).torso.oldFrame;
    *torso = (*pi).torso.frame;
    *torsoBackLerp = (*pi).torso.backlerp;
}
/*
==================
UI_SwingAngles
==================
*/

unsafe extern "C" fn UI_SwingAngles(
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
        move_0 = crate::src::q3_ui::ui_atoms::uis.frametime as f32 * scale * speed;
        if move_0 >= swing {
            move_0 = swing;
            *swinging = crate::src::qcommon::q_shared::qfalse
        }
        *angle = crate::src::qcommon::q_math::AngleMod(*angle + move_0)
    } else if swing < 0f32 {
        move_0 = crate::src::q3_ui::ui_atoms::uis.frametime as f32 * scale * -speed;
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
======================
UI_MovedirAdjustment
======================
*/

unsafe extern "C" fn UI_MovedirAdjustment(mut pi: *mut crate::ui_local_h::playerInfo_t) -> f32 {
    let mut relativeAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut moveVector: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    relativeAngles[0] = (*pi).viewAngles[0] - (*pi).moveAngles[0];
    relativeAngles[1] = (*pi).viewAngles[1] - (*pi).moveAngles[1];
    relativeAngles[2] = (*pi).viewAngles[2] - (*pi).moveAngles[2];
    crate::src::qcommon::q_math::AngleVectors(
        relativeAngles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        moveVector.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    if (crate::src::qcommon::q_math::Q_fabs(moveVector[0]) as f64) < 0.01 {
        moveVector[0] = 0f32
    }
    if (crate::src::qcommon::q_math::Q_fabs(moveVector[1]) as f64) < 0.01 {
        moveVector[1] = 0f32
    }
    if moveVector[1] == 0f32 && moveVector[0] > 0f32 {
        return 0f32;
    }
    if moveVector[1] < 0f32 && moveVector[0] > 0f32 {
        return 22f32;
    }
    if moveVector[1] < 0f32 && moveVector[0] == 0f32 {
        return 45f32;
    }
    if moveVector[1] < 0f32 && moveVector[0] < 0f32 {
        return -22f32;
    }
    if moveVector[1] == 0f32 && moveVector[0] < 0f32 {
        return 0f32;
    }
    if moveVector[1] > 0f32 && moveVector[0] < 0f32 {
        return 22f32;
    }
    if moveVector[1] > 0f32 && moveVector[0] == 0f32 {
        return -45f32;
    }
    return -22f32;
}
/*
===============
UI_PlayerAngles
===============
*/

unsafe extern "C" fn UI_PlayerAngles(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut legs: *mut crate::src::qcommon::q_shared::vec3_t,
    mut torso: *mut crate::src::qcommon::q_shared::vec3_t,
    mut head: *mut crate::src::qcommon::q_shared::vec3_t,
) {
    let mut legsAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut torsoAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut headAngles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dest: f32 = 0.;
    let mut adjust: f32 = 0.;
    headAngles[0] = (*pi).viewAngles[0];
    headAngles[1] = (*pi).viewAngles[1];
    headAngles[2] = (*pi).viewAngles[2];
    headAngles[1] = crate::src::qcommon::q_math::AngleMod(headAngles[1]);
    legsAngles[2] = 0f32;
    legsAngles[1] = legsAngles[2];
    legsAngles[0] = legsAngles[1];
    torsoAngles[2] = 0f32;
    torsoAngles[1] = torsoAngles[2];
    torsoAngles[0] = torsoAngles[1];
    // --------- yaw -------------
    // allow yaw to drift a bit
    if (*pi).legsAnim & !(128) != crate::bg_public_h::LEGS_IDLE as i32
        || (*pi).torsoAnim & !(128) != crate::bg_public_h::TORSO_STAND as i32
    {
        // if not standing still, always point all in the same direction
        (*pi).torso.yawing = crate::src::qcommon::q_shared::qtrue; // always center
                                                                   // always center
        (*pi).torso.pitching = crate::src::qcommon::q_shared::qtrue; // always center
        (*pi).legs.yawing = crate::src::qcommon::q_shared::qtrue
    }
    // adjust legs for movement dir
    adjust = UI_MovedirAdjustment(pi);
    legsAngles[1] = headAngles[1] + adjust;
    torsoAngles[1] =
        (headAngles[1] as f64 + 0.25 * adjust as f64) as crate::src::qcommon::q_shared::vec_t;
    // torso
    UI_SwingAngles(
        torsoAngles[1],
        25f32,
        90f32,
        0.3,
        &mut (*pi).torso.yawAngle,
        &mut (*pi).torso.yawing,
    );
    UI_SwingAngles(
        legsAngles[1],
        40f32,
        90f32,
        0.3,
        &mut (*pi).legs.yawAngle,
        &mut (*pi).legs.yawing,
    );
    torsoAngles[1] = (*pi).torso.yawAngle;
    legsAngles[1] = (*pi).legs.yawAngle;
    // --------- pitch -------------
    // only show a fraction of the pitch angle in the torso
    if headAngles[0] > 180f32 {
        dest = ((-360f32 + headAngles[0]) as f64 * 0.75) as f32
    } else {
        dest = (headAngles[0] as f64 * 0.75) as f32
    }
    UI_SwingAngles(
        dest,
        15f32,
        30f32,
        0.1,
        &mut (*pi).torso.pitchAngle,
        &mut (*pi).torso.pitching,
    );
    torsoAngles[0] = (*pi).torso.pitchAngle;
    if (*pi).fixedtorso as u64 != 0 {
        torsoAngles[0] = 0.0
    }
    if (*pi).fixedlegs as u64 != 0 {
        legsAngles[1] = torsoAngles[1];
        legsAngles[0] = 0.0;
        legsAngles[2] = 0.0
    }
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
/*
===============
UI_PlayerFloatSprite
===============
*/

unsafe extern "C" fn UI_PlayerFloatSprite(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut shader: crate::src::qcommon::q_shared::qhandle_t,
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
    crate::stdlib::memset(
        &mut ent as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
    );
    ent.origin[0] = *origin.offset(0);
    ent.origin[1] = *origin.offset(1);
    ent.origin[2] = *origin.offset(2);
    ent.origin[2] += 48f32;
    ent.reType = crate::tr_types_h::RT_SPRITE;
    ent.customShader = shader;
    ent.radius = 10f32;
    ent.renderfx = 0;
    crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(&mut ent);
}
/*
======================
UI_MachinegunSpinAngle
======================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_MachinegunSpinAngle(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
) -> f32 {
    let mut delta: i32 = 0;
    let mut angle: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut torsoAnim: i32 = 0;
    delta = dp_realtime - (*pi).barrelTime;
    if (*pi).barrelSpinning as u64 != 0 {
        angle = (*pi).barrelAngle + delta as f32 * 0.9
    } else {
        if delta > 1000 {
            delta = 1000
        }
        speed = (0.5 * (0.9 + (1000 - delta) as f32 / 1000f32) as f64) as f32;
        angle = (*pi).barrelAngle + delta as f32 * speed
    }
    torsoAnim = (*pi).torsoAnim & !(128);
    if torsoAnim == crate::bg_public_h::TORSO_ATTACK2 as i32 {
        torsoAnim = crate::bg_public_h::TORSO_ATTACK as i32
    }
    if (*pi).barrelSpinning == !(torsoAnim == crate::bg_public_h::TORSO_ATTACK as i32) as u32 {
        (*pi).barrelTime = dp_realtime;
        (*pi).barrelAngle = crate::src::qcommon::q_math::AngleMod(angle);
        (*pi).barrelSpinning = (torsoAnim == crate::bg_public_h::TORSO_ATTACK as i32)
            as crate::src::qcommon::q_shared::qboolean
    }
    return angle;
}
/*
===============
UI_DrawPlayer
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawPlayer(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut time: i32,
) {
    let mut refdef: crate::tr_types_h::refdef_t = crate::tr_types_h::refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        text: [[0; 32]; 8],
    };
    let mut legs: crate::tr_types_h::refEntity_t = {
        let mut init = crate::tr_types_h::refEntity_t {
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
        init
    };
    let mut torso: crate::tr_types_h::refEntity_t = {
        let mut init = crate::tr_types_h::refEntity_t {
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
        init
    };
    let mut head: crate::tr_types_h::refEntity_t = {
        let mut init = crate::tr_types_h::refEntity_t {
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
        init
    };
    let mut gun: crate::tr_types_h::refEntity_t = {
        let mut init = crate::tr_types_h::refEntity_t {
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
        init
    };
    let mut barrel: crate::tr_types_h::refEntity_t = {
        let mut init = crate::tr_types_h::refEntity_t {
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
        init
    };
    let mut flash: crate::tr_types_h::refEntity_t = {
        let mut init = crate::tr_types_h::refEntity_t {
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
        init
    };
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut renderfx: i32 = 0;
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [-16f32, -16f32, -24f32];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [16f32, 16f32, 32f32];
    let mut len: f32 = 0.;
    let mut xx: f32 = 0.;
    if (*pi).legsModel == 0
        || (*pi).torsoModel == 0
        || (*pi).headModel == 0
        || (*pi).animations[0].numFrames == 0
    {
        return;
    }
    dp_realtime = time;
    if (*pi).pendingWeapon != crate::bg_public_h::WP_NUM_WEAPONS && dp_realtime > (*pi).weaponTimer
    {
        (*pi).weapon = (*pi).pendingWeapon;
        (*pi).lastWeapon = (*pi).pendingWeapon;
        (*pi).pendingWeapon = crate::bg_public_h::WP_NUM_WEAPONS;
        (*pi).weaponTimer = 0;
        if (*pi).currentWeapon != (*pi).weapon {
            crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
                crate::src::q3_ui::ui_qmenu::weaponChangeSound,
                crate::src::qcommon::q_shared::CHAN_LOCAL as i32,
            );
        }
    }
    crate::src::q3_ui::ui_atoms::UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    y -= jumpHeight;
    crate::stdlib::memset(
        &mut refdef as *mut crate::tr_types_h::refdef_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::tr_types_h::refdef_t>(),
    );
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
    refdef.rdflags = 0x1;
    crate::src::qcommon::q_math::AxisClear(refdef.viewaxis.as_mut_ptr());
    refdef.x = x as i32;
    refdef.y = y as i32;
    refdef.width = w as i32;
    refdef.height = h as i32;
    refdef.fov_x = (refdef.width as f32 / crate::src::q3_ui::ui_atoms::uis.xscale / 640.0 * 90.0)
        as i32 as f32;
    xx = ((refdef.width as f32 / crate::src::q3_ui::ui_atoms::uis.xscale) as f64
        / crate::stdlib::tan((refdef.fov_x / 360f32) as f64 * 3.14159265358979323846))
        as f32;
    refdef.fov_y = crate::stdlib::atan2(
        (refdef.height as f32 / crate::src::q3_ui::ui_atoms::uis.yscale) as f64,
        xx as f64,
    ) as f32;
    refdef.fov_y = (refdef.fov_y as f64 * (360f64 / 3.14159265358979323846)) as f32;
    // calculate distance so the player nearly fills the box
    len = (0.7 * (maxs[2] - mins[2]) as f64) as f32;
    origin[0] = (len as f64
        / crate::stdlib::tan(refdef.fov_x as f64 * 3.14159265358979323846 / 180f64 * 0.5))
        as crate::src::qcommon::q_shared::vec_t;
    origin[1] = (0.5 * (mins[1] + maxs[1]) as f64) as crate::src::qcommon::q_shared::vec_t;
    origin[2] = (-0.5 * (mins[2] + maxs[2]) as f64) as crate::src::qcommon::q_shared::vec_t;
    refdef.time = dp_realtime;
    crate::src::ui::ui_syscalls::trap_R_ClearScene();
    // get the rotation information
    UI_PlayerAngles(
        pi,
        legs.axis.as_mut_ptr(),
        torso.axis.as_mut_ptr(),
        head.axis.as_mut_ptr(),
    );
    // get the animation state (after rotation, to allow feet shuffle)
    UI_PlayerAnimation(
        pi,
        &mut legs.oldframe,
        &mut legs.frame,
        &mut legs.backlerp,
        &mut torso.oldframe,
        &mut torso.frame,
        &mut torso.backlerp,
    );
    renderfx = 0x80 | 0x40;
    //
    // add the legs
    //
    legs.hModel = (*pi).legsModel;
    legs.customSkin = (*pi).legsSkin;
    legs.origin[0] = origin[0];
    legs.origin[1] = origin[1];
    legs.origin[2] = origin[2];
    legs.lightingOrigin[0] = origin[0];
    legs.lightingOrigin[1] = origin[1];
    legs.lightingOrigin[2] = origin[2];
    legs.renderfx = renderfx;
    legs.oldorigin[0] = legs.origin[0];
    legs.oldorigin[1] = legs.origin[1];
    legs.oldorigin[2] = legs.origin[2];
    crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(&mut legs);
    if legs.hModel == 0 {
        return;
    }
    //
    // add the torso
    //
    torso.hModel = (*pi).torsoModel;
    if torso.hModel == 0 {
        return;
    }
    torso.customSkin = (*pi).torsoSkin;
    torso.lightingOrigin[0] = origin[0];
    torso.lightingOrigin[1] = origin[1];
    torso.lightingOrigin[2] = origin[2];
    UI_PositionRotatedEntityOnTag(
        &mut torso,
        &mut legs,
        (*pi).legsModel,
        b"tag_torso\x00" as *const u8 as *mut i8,
    );
    torso.renderfx = renderfx;
    crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(&mut torso);
    //
    // add the head
    //
    head.hModel = (*pi).headModel;
    if head.hModel == 0 {
        return;
    }
    head.customSkin = (*pi).headSkin;
    head.lightingOrigin[0] = origin[0];
    head.lightingOrigin[1] = origin[1];
    head.lightingOrigin[2] = origin[2];
    UI_PositionRotatedEntityOnTag(
        &mut head,
        &mut torso,
        (*pi).torsoModel,
        b"tag_head\x00" as *const u8 as *mut i8,
    );
    head.renderfx = renderfx;
    crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(&mut head);
    //
    // add the gun
    //
    if (*pi).currentWeapon != crate::bg_public_h::WP_NONE {
        crate::stdlib::memset(
            &mut gun as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
        );
        gun.hModel = (*pi).weaponModel;
        if (*pi).currentWeapon == crate::bg_public_h::WP_RAILGUN {
            gun.shaderRGBA[0] = (*pi).c1RGBA[0];
            gun.shaderRGBA[1] = (*pi).c1RGBA[1];
            gun.shaderRGBA[2] = (*pi).c1RGBA[2];
            gun.shaderRGBA[3] = (*pi).c1RGBA[3]
        } else {
            gun.shaderRGBA[0] =
                crate::src::qcommon::q_math::colorWhite[0] as crate::src::qcommon::q_shared::byte;
            gun.shaderRGBA[1] =
                crate::src::qcommon::q_math::colorWhite[1] as crate::src::qcommon::q_shared::byte;
            gun.shaderRGBA[2] =
                crate::src::qcommon::q_math::colorWhite[2] as crate::src::qcommon::q_shared::byte;
            gun.shaderRGBA[3] =
                crate::src::qcommon::q_math::colorWhite[3] as crate::src::qcommon::q_shared::byte
        }
        gun.lightingOrigin[0] = origin[0];
        gun.lightingOrigin[1] = origin[1];
        gun.lightingOrigin[2] = origin[2];
        UI_PositionEntityOnTag(
            &mut gun,
            &mut torso,
            (*pi).torsoModel,
            b"tag_weapon\x00" as *const u8 as *mut i8,
        );
        gun.renderfx = renderfx;
        crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(&mut gun);
    }
    //
    // add the spinning barrel
    //
    if (*pi).realWeapon == crate::bg_public_h::WP_MACHINEGUN as i32
        || (*pi).realWeapon == crate::bg_public_h::WP_GAUNTLET as i32
        || (*pi).realWeapon == crate::bg_public_h::WP_BFG as i32
    {
        let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
        crate::stdlib::memset(
            &mut barrel as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
        );
        barrel.lightingOrigin[0] = origin[0];
        barrel.lightingOrigin[1] = origin[1];
        barrel.lightingOrigin[2] = origin[2];
        barrel.renderfx = renderfx;
        barrel.hModel = (*pi).barrelModel;
        angles[1] = 0f32;
        angles[0] = 0f32;
        angles[2] = UI_MachinegunSpinAngle(pi);
        crate::src::qcommon::q_math::AnglesToAxis(
            angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            barrel.axis.as_mut_ptr(),
        );
        UI_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut gun,
            (*pi).weaponModel,
            b"tag_barrel\x00" as *const u8 as *mut i8,
        );
        crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(&mut barrel);
    }
    //
    // add muzzle flash
    //
    if dp_realtime <= (*pi).muzzleFlashTime {
        if (*pi).flashModel != 0 {
            crate::stdlib::memset(
                &mut flash as *mut crate::tr_types_h::refEntity_t as *mut libc::c_void,
                0,
                ::std::mem::size_of::<crate::tr_types_h::refEntity_t>(),
            );
            flash.hModel = (*pi).flashModel;
            if (*pi).currentWeapon == crate::bg_public_h::WP_RAILGUN {
                flash.shaderRGBA[0] = (*pi).c1RGBA[0];
                flash.shaderRGBA[1] = (*pi).c1RGBA[1];
                flash.shaderRGBA[2] = (*pi).c1RGBA[2];
                flash.shaderRGBA[3] = (*pi).c1RGBA[3]
            } else {
                flash.shaderRGBA[0] = crate::src::qcommon::q_math::colorWhite[0]
                    as crate::src::qcommon::q_shared::byte;
                flash.shaderRGBA[1] = crate::src::qcommon::q_math::colorWhite[1]
                    as crate::src::qcommon::q_shared::byte;
                flash.shaderRGBA[2] = crate::src::qcommon::q_math::colorWhite[2]
                    as crate::src::qcommon::q_shared::byte;
                flash.shaderRGBA[3] = crate::src::qcommon::q_math::colorWhite[3]
                    as crate::src::qcommon::q_shared::byte
            }
            flash.lightingOrigin[0] = origin[0];
            flash.lightingOrigin[1] = origin[1];
            flash.lightingOrigin[2] = origin[2];
            UI_PositionEntityOnTag(
                &mut flash,
                &mut gun,
                (*pi).weaponModel,
                b"tag_flash\x00" as *const u8 as *mut i8,
            );
            flash.renderfx = renderfx;
            crate::src::ui::ui_syscalls::trap_R_AddRefEntityToScene(&mut flash);
        }
        // make a dlight for the flash
        if (*pi).flashDlightColor[0] != 0.
            || (*pi).flashDlightColor[1] != 0.
            || (*pi).flashDlightColor[2] != 0.
        {
            crate::src::ui::ui_syscalls::trap_R_AddLightToScene(
                flash.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (200i32 + (crate::stdlib::rand() & 31i32)) as f32,
                (*pi).flashDlightColor[0usize],
                (*pi).flashDlightColor[1usize],
                (*pi).flashDlightColor[2usize],
            );
        }
    }
    //
    // add the chat icon
    //
    if (*pi).chat as u64 != 0 {
        UI_PlayerFloatSprite(
            pi,
            origin.as_mut_ptr(),
            crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
                b"sprites/balloon3\x00" as *const u8 as *const i8,
            ),
        );
    }
    //
    // add an accent light
    //
    origin[0] -= 100f32; // + = behind, - = in front
    origin[1] += 100f32; // + = left, - = right
    origin[2] += 100f32; // + = above, - = below
    crate::src::ui::ui_syscalls::trap_R_AddLightToScene(
        origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        500f32,
        1f32,
        1f32,
        1f32,
    );
    origin[0] -= 100f32;
    origin[1] -= 100f32;
    origin[2] -= 100f32;
    crate::src::ui::ui_syscalls::trap_R_AddLightToScene(
        origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        500f32,
        1f32,
        0f32,
        0f32,
    );
    crate::src::ui::ui_syscalls::trap_R_RenderScene(&mut refdef);
}
/*
==========================
UI_RegisterClientSkin
==========================
*/

unsafe extern "C" fn UI_RegisterClientSkin(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut modelName: *const i8,
    mut skinName: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut filename: [i8; 64] = [0; 64];
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/lower_%s.skin\x00" as *const u8 as *const i8,
        modelName,
        skinName,
    );
    (*pi).legsSkin = crate::src::ui::ui_syscalls::trap_R_RegisterSkin(filename.as_mut_ptr());
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/upper_%s.skin\x00" as *const u8 as *const i8,
        modelName,
        skinName,
    );
    (*pi).torsoSkin = crate::src::ui::ui_syscalls::trap_R_RegisterSkin(filename.as_mut_ptr());
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/head_%s.skin\x00" as *const u8 as *const i8,
        modelName,
        skinName,
    );
    (*pi).headSkin = crate::src::ui::ui_syscalls::trap_R_RegisterSkin(filename.as_mut_ptr());
    if (*pi).legsSkin == 0 || (*pi).torsoSkin == 0 || (*pi).headSkin == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
======================
UI_ParseAnimationFile
======================
*/

unsafe extern "C" fn UI_ParseAnimationFile(
    mut filename: *const i8,
    mut pi: *mut crate::ui_local_h::playerInfo_t,
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
    animations = (*pi).animations.as_mut_ptr();
    crate::stdlib::memset(
        animations as *mut libc::c_void,
        0,
        (::std::mem::size_of::<crate::bg_public_h::animation_t>())
            .wrapping_mul(crate::bg_public_h::MAX_ANIMATIONS as usize),
    );
    (*pi).fixedlegs = crate::src::qcommon::q_shared::qfalse;
    (*pi).fixedtorso = crate::src::qcommon::q_shared::qfalse;
    // load the file
    len = crate::src::ui::ui_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if len <= 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if len as usize >= (::std::mem::size_of::<[i8; 20000]>()).wrapping_sub(1usize) {
        crate::src::q3_ui::ui_atoms::Com_Printf(
            b"File %s too long\n\x00" as *const u8 as *const i8,
            filename,
        );
        crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::ui::ui_syscalls::trap_FS_Read(text.as_mut_ptr() as *mut libc::c_void, len, f);
    text[len as usize] = 0;
    crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
    // parse the text
    text_p = text.as_mut_ptr(); // quite the compiler warning
    skip = 0;
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
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"fixedlegs\x00" as *const u8 as *const i8,
        ) == 0
        {
            (*pi).fixedlegs = crate::src::qcommon::q_shared::qtrue
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"fixedtorso\x00" as *const u8 as *const i8,
        ) == 0
        {
            (*pi).fixedtorso = crate::src::qcommon::q_shared::qtrue
        } else if *token.offset(0) as i32 >= '0' as i32 && *token.offset(0) as i32 <= '9' as i32 {
            // if it is a number, start parsing animations
            text_p = prev; // unget the token
            break;
        } else {
            crate::src::q3_ui::ui_atoms::Com_Printf(
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
        crate::src::q3_ui::ui_atoms::Com_Printf(
            b"Error parsing animation file: %s\n\x00" as *const u8 as *const i8,
            filename,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
==========================
UI_RegisterClientModelname
==========================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_RegisterClientModelname(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut modelSkinName: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut modelName: [i8; 64] = [0; 64];
    let mut skinName: [i8; 64] = [0; 64];
    let mut filename: [i8; 64] = [0; 64];
    let mut slash: *mut i8 = 0 as *mut i8;
    (*pi).torsoModel = 0;
    (*pi).headModel = 0;
    if *modelSkinName.offset(0) == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        modelName.as_mut_ptr(),
        modelSkinName,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    slash = crate::stdlib::strchr(modelName.as_mut_ptr(), '/' as i32);
    if slash.is_null() {
        // modelName did not include a skin name
        crate::src::qcommon::q_shared::Q_strncpyz(
            skinName.as_mut_ptr(),
            b"default\x00" as *const u8 as *const i8,
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
    } else {
        crate::src::qcommon::q_shared::Q_strncpyz(
            skinName.as_mut_ptr(),
            slash.offset(1),
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        // truncate modelName
        *slash = 0
    }
    // load cmodels before models so filecache works
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/lower.md3\x00" as *const u8 as *const i8,
        modelName.as_mut_ptr(),
    );
    (*pi).legsModel = crate::src::ui::ui_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
    if (*pi).legsModel == 0 {
        crate::src::q3_ui::ui_atoms::Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/upper.md3\x00" as *const u8 as *const i8,
        modelName.as_mut_ptr(),
    );
    (*pi).torsoModel = crate::src::ui::ui_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
    if (*pi).torsoModel == 0 {
        crate::src::q3_ui::ui_atoms::Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/head.md3\x00" as *const u8 as *const i8,
        modelName.as_mut_ptr(),
    );
    (*pi).headModel = crate::src::ui::ui_syscalls::trap_R_RegisterModel(filename.as_mut_ptr());
    if (*pi).headModel == 0 {
        crate::src::q3_ui::ui_atoms::Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    // if any skins failed to load, fall back to default
    if UI_RegisterClientSkin(pi, modelName.as_mut_ptr(), skinName.as_mut_ptr()) as u64 == 0 {
        if UI_RegisterClientSkin(
            pi,
            modelName.as_mut_ptr(),
            b"default\x00" as *const u8 as *const i8,
        ) as u64
            == 0
        {
            crate::src::q3_ui::ui_atoms::Com_Printf(
                b"Failed to load skin file: %s : %s\n\x00" as *const u8 as *const i8,
                modelName.as_mut_ptr(),
                skinName.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    // load the animations
    crate::src::qcommon::q_shared::Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"models/players/%s/animation.cfg\x00" as *const u8 as *const i8,
        modelName.as_mut_ptr(),
    );
    if UI_ParseAnimationFile(filename.as_mut_ptr(), pi) as u64 == 0 {
        crate::src::q3_ui::ui_atoms::Com_Printf(
            b"Failed to load animation file %s\n\x00" as *const u8 as *const i8,
            filename.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
UI_PlayerInfo_SetModel
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PlayerInfo_SetModel(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut model: *const i8,
) {
    crate::stdlib::memset(
        pi as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::ui_local_h::playerInfo_t>(),
    );
    UI_RegisterClientModelname(pi, model);
    (*pi).weapon = crate::bg_public_h::WP_MACHINEGUN;
    (*pi).currentWeapon = (*pi).weapon;
    (*pi).lastWeapon = (*pi).weapon;
    (*pi).pendingWeapon = crate::bg_public_h::WP_NUM_WEAPONS;
    (*pi).weaponTimer = 0;
    (*pi).chat = crate::src::qcommon::q_shared::qfalse;
    (*pi).newModel = crate::src::qcommon::q_shared::qtrue;
    UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
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
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
//
// ui_playermodel.c
//
//
// ui_playersettings.c
//
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
/*
===============
UI_PlayerInfo_SetInfo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PlayerInfo_SetInfo(
    mut pi: *mut crate::ui_local_h::playerInfo_t,
    mut legsAnim: i32,
    mut torsoAnim: i32,
    mut viewAngles: *mut crate::src::qcommon::q_shared::vec_t,
    mut moveAngles: *mut crate::src::qcommon::q_shared::vec_t,
    mut weaponNumber: crate::bg_public_h::weapon_t,
    mut chat: crate::src::qcommon::q_shared::qboolean,
) {
    let mut currentAnim: i32 = 0;
    let mut weaponNum: crate::bg_public_h::weapon_t = crate::bg_public_h::WP_NONE;
    let mut c: i32 = 0;
    (*pi).chat = chat;
    c = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"color1\x00" as *const u8 as *const i8,
    ) as i32;
    (*pi).color1[2] = 0f32;
    (*pi).color1[1] = (*pi).color1[2];
    (*pi).color1[0] = (*pi).color1[1];
    if c < 1 || c > 7 {
        (*pi).color1[0] = 1f32;
        (*pi).color1[1] = 1f32;
        (*pi).color1[2] = 1f32
    } else {
        if c & 1 != 0 {
            (*pi).color1[2] = 1.0f32
        }
        if c & 2 != 0 {
            (*pi).color1[1] = 1.0f32
        }
        if c & 4 != 0 {
            (*pi).color1[0] = 1.0f32
        }
    }
    (*pi).c1RGBA[0] = (255f32 * (*pi).color1[0]) as crate::src::qcommon::q_shared::byte;
    (*pi).c1RGBA[1] = (255f32 * (*pi).color1[1]) as crate::src::qcommon::q_shared::byte;
    (*pi).c1RGBA[2] = (255f32 * (*pi).color1[2]) as crate::src::qcommon::q_shared::byte;
    (*pi).c1RGBA[3] = 255u8;
    // view angles
    (*pi).viewAngles[0] = *viewAngles.offset(0);
    (*pi).viewAngles[1] = *viewAngles.offset(1);
    (*pi).viewAngles[2] = *viewAngles.offset(2);
    // move angles
    (*pi).moveAngles[0] = *moveAngles.offset(0);
    (*pi).moveAngles[1] = *moveAngles.offset(1);
    (*pi).moveAngles[2] = *moveAngles.offset(2);
    if (*pi).newModel as u64 != 0 {
        (*pi).newModel = crate::src::qcommon::q_shared::qfalse;
        jumpHeight = 0f32;
        (*pi).pendingLegsAnim = 0;
        UI_ForceLegsAnim(pi, legsAnim);
        (*pi).legs.yawAngle = *viewAngles.offset(1);
        (*pi).legs.yawing = crate::src::qcommon::q_shared::qfalse;
        (*pi).pendingTorsoAnim = 0;
        UI_ForceTorsoAnim(pi, torsoAnim);
        (*pi).torso.yawAngle = *viewAngles.offset(1);
        (*pi).torso.yawing = crate::src::qcommon::q_shared::qfalse;
        if weaponNumber != crate::bg_public_h::WP_NUM_WEAPONS {
            (*pi).weapon = weaponNumber;
            (*pi).currentWeapon = weaponNumber;
            (*pi).lastWeapon = weaponNumber;
            (*pi).pendingWeapon = crate::bg_public_h::WP_NUM_WEAPONS;
            (*pi).weaponTimer = 0;
            UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        }
        return;
    }
    // weapon
    if weaponNumber == crate::bg_public_h::WP_NUM_WEAPONS {
        (*pi).pendingWeapon = crate::bg_public_h::WP_NUM_WEAPONS;
        (*pi).weaponTimer = 0
    } else if weaponNumber != crate::bg_public_h::WP_NONE {
        (*pi).pendingWeapon = weaponNumber;
        (*pi).weaponTimer = dp_realtime + 250
    }
    weaponNum = (*pi).lastWeapon;
    (*pi).weapon = weaponNum;
    if torsoAnim == crate::bg_public_h::BOTH_DEATH1 as i32
        || legsAnim == crate::bg_public_h::BOTH_DEATH1 as i32
    {
        legsAnim = crate::bg_public_h::BOTH_DEATH1 as i32;
        torsoAnim = legsAnim;
        (*pi).currentWeapon = crate::bg_public_h::WP_NONE;
        (*pi).weapon = (*pi).currentWeapon;
        UI_PlayerInfo_SetWeapon(pi, (*pi).weapon);
        jumpHeight = 0f32;
        (*pi).pendingLegsAnim = 0;
        UI_ForceLegsAnim(pi, legsAnim);
        (*pi).pendingTorsoAnim = 0;
        UI_ForceTorsoAnim(pi, torsoAnim);
        return;
    }
    // leg animation
    currentAnim = (*pi).legsAnim & !(128);
    if legsAnim != crate::bg_public_h::LEGS_JUMP as i32
        && (currentAnim == crate::bg_public_h::LEGS_JUMP as i32
            || currentAnim == crate::bg_public_h::LEGS_LAND as i32)
    {
        (*pi).pendingLegsAnim = legsAnim
    } else if legsAnim != currentAnim {
        jumpHeight = 0f32;
        (*pi).pendingLegsAnim = 0;
        UI_ForceLegsAnim(pi, legsAnim);
    }
    // torso animation
    if torsoAnim == crate::bg_public_h::TORSO_STAND as i32
        || torsoAnim == crate::bg_public_h::TORSO_STAND2 as i32
    {
        if weaponNum == crate::bg_public_h::WP_NONE || weaponNum == crate::bg_public_h::WP_GAUNTLET
        {
            torsoAnim = crate::bg_public_h::TORSO_STAND2 as i32
        } else {
            torsoAnim = crate::bg_public_h::TORSO_STAND as i32
        }
    }
    if torsoAnim == crate::bg_public_h::TORSO_ATTACK as i32
        || torsoAnim == crate::bg_public_h::TORSO_ATTACK2 as i32
    {
        if weaponNum == crate::bg_public_h::WP_NONE || weaponNum == crate::bg_public_h::WP_GAUNTLET
        {
            torsoAnim = crate::bg_public_h::TORSO_ATTACK2 as i32
        } else {
            torsoAnim = crate::bg_public_h::TORSO_ATTACK as i32
        }
        (*pi).muzzleFlashTime = dp_realtime + 20
        //FIXME play firing sound here
    }
    currentAnim = (*pi).torsoAnim & !(128);
    if weaponNum != (*pi).currentWeapon
        || currentAnim == crate::bg_public_h::TORSO_RAISE as i32
        || currentAnim == crate::bg_public_h::TORSO_DROP as i32
    {
        (*pi).pendingTorsoAnim = torsoAnim
    } else if (currentAnim == crate::bg_public_h::TORSO_GESTURE as i32
        || currentAnim == crate::bg_public_h::TORSO_ATTACK as i32)
        && torsoAnim != currentAnim
    {
        (*pi).pendingTorsoAnim = torsoAnim
    } else if torsoAnim != currentAnim {
        (*pi).pendingTorsoAnim = 0;
        UI_ForceTorsoAnim(pi, torsoAnim);
    };
}
