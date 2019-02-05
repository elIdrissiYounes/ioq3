use bg_public_h::{
    animation_s, animation_t, bg_itemlist, bg_numItems, gametype_t, gender_t, gitem_s, gitem_t,
    itemType_t, team_t, unnamed_0, unnamed_1, BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta,
    BG_PlayerStateToEntityState, ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM,
    ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM,
    ET_TELEPORT_TRIGGER, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO,
    IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM,
    IT_WEAPON, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, WP_BFG, WP_GAUNTLET,
    WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS,
    WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
};
use cg_consolecmds::{CG_ConsoleCommand, CG_InitConsoleCommands};
use cg_draw::{
    drawTeamOverlayModificationCount, numSortedTeamPlayers, sortedTeamPlayers,
    CG_AddLagometerFrameInfo, CG_AddLagometerSnapshotInfo, CG_CenterPrint, CG_DrawActive,
    CG_DrawFlagModel, CG_DrawHead, CG_DrawTeamBackground,
};
use cg_drawtools::{
    CG_AdjustFrom640, CG_ColorForHealth, CG_DrawBigString, CG_DrawBigStringColor, CG_DrawPic,
    CG_DrawSmallString, CG_DrawSmallStringColor, CG_DrawStringExt, CG_DrawStrlen, CG_FadeColor,
    CG_FillRect, CG_GetColorForHealth, CG_TileClear, UI_DrawProportionalString,
};
use cg_effects::{
    CG_Bleed, CG_BubbleTrail, CG_GibPlayer, CG_MakeExplosion, CG_ScorePlum, CG_SmokePuff,
    CG_SpawnEffect,
};
use cg_event::{CG_CheckEvents, CG_EntityEvent, CG_PainEvent, CG_PlaceString};
use cg_info::{CG_DrawInformation, CG_LoadingClient, CG_LoadingItem, CG_LoadingString};
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, itemInfo_t,
    lerpFrame_t, playerEntity_t, score_t, trap_R_AddLightToScene, trap_R_AddRefEntityToScene,
    trap_R_LerpTag, trap_S_AddLoopingSound, trap_S_AddRealLoopingSound, trap_S_StartSound,
    trap_S_UpdateEntityPosition, weaponInfo_s, weaponInfo_t, FOOTSTEP_BOOT, FOOTSTEP_ENERGY,
    FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL, FOOTSTEP_SPLASH,
    FOOTSTEP_TOTAL,
};
use cg_localents::{CG_AddLocalEntities, CG_AllocLocalEntity, CG_InitLocalEntities};
use cg_main::{
    cg, cg_addMarks, cg_animSpeed, cg_autoswitch, cg_blood, cg_bobpitch, cg_bobroll, cg_bobup,
    cg_brassTime, cg_buildScript, cg_cameraMode, cg_cameraOrbit, cg_cameraOrbitDelay,
    cg_centertime, cg_crosshairHealth, cg_crosshairSize, cg_crosshairX, cg_crosshairY,
    cg_debugAnim, cg_debugEvents, cg_debugPosition, cg_deferPlayers, cg_draw2D, cg_draw3dIcons,
    cg_drawAmmoWarning, cg_drawAttacker, cg_drawCrosshair, cg_drawCrosshairNames, cg_drawFPS,
    cg_drawFriend, cg_drawGun, cg_drawIcons, cg_drawRewards, cg_drawSnapshot, cg_drawStatus,
    cg_drawTeamOverlay, cg_drawTimer, cg_entities, cg_errorDecay, cg_footsteps, cg_forceModel,
    cg_fov, cg_gibs, cg_gun_frame, cg_gun_x, cg_gun_y, cg_gun_z, cg_items, cg_lagometer,
    cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail, cg_oldRocket,
    cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll, cg_scorePlum,
    cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats, cg_swingSpeed,
    cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly, cg_thirdPerson,
    cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale, cg_timescaleFadeEnd,
    cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength, cg_tracerWidth, cg_trueLightning,
    cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed, pmove_msec, CG_Argv,
    CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer, CG_LastAttacker, CG_StartMusic,
    CG_UpdateCvars,
};
use cg_marks::{CG_AddMarks, CG_ImpactMark, CG_InitMarkPolys};
use cg_particles::{CG_AddParticles, CG_ClearParticles, CG_ParticleExplosion};
use cg_players::{
    CG_AddRefEntityWithPowerups, CG_CustomSound, CG_LoadDeferredPlayers, CG_NewClientInfo,
    CG_Player, CG_ResetPlayerEntity,
};
use cg_playerstate::{CG_Respawn, CG_TransitionPlayerState};
use cg_predict::{CG_BuildSolidList, CG_PointContents, CG_PredictPlayerState, CG_Trace};
use cg_public_h::snapshot_t;
use cg_scoreboard::{CG_DrawOldScoreboard, CG_DrawTourneyScoreboard};
use cg_servercmds::{
    CG_ExecuteNewServerCommands, CG_ParseServerinfo, CG_SetConfigValues, CG_ShaderStateChanged,
};
use cg_snapshot::CG_ProcessSnapshots;
use cg_variadic_h::CG_Error;
use cg_view::{
    CG_AddBufferedSound, CG_DrawActiveFrame, CG_TestGun_f, CG_TestModelNextFrame_f,
    CG_TestModelNextSkin_f, CG_TestModelPrevFrame_f, CG_TestModelPrevSkin_f, CG_TestModel_f,
    CG_ZoomDown_f, CG_ZoomUp_f,
};
use cg_weapons::{
    CG_AddPlayerWeapon, CG_AddViewWeapon, CG_Bullet, CG_DrawWeaponSelect, CG_FireWeapon,
    CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f, CG_OutOfAmmoChange,
    CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire, CG_Weapon_f,
};
use libc;
use q_shared_h::{
    byte, clipHandle_t, cvarHandle_t, entityState_s, entityState_t, gameState_t, orientation_t,
    playerState_s, playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t,
    trajectory_t, unnamed, vec3_origin, vec3_t, vec_t, vmCvar_t, AngleVectors, AnglesToAxis,
    AxisClear, AxisCopy, ByteToDir, LerpAngle, MatrixMultiply, PerpendicularVector,
    RotateAroundDirection, VectorNormalize2, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM,
    CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{cos, memset, rand};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

unsafe extern "C" fn VectorInverse(mut v: *mut vec_t) {
    *v.offset(0isize) = -*v.offset(0isize);
    *v.offset(1isize) = -*v.offset(1isize);
    *v.offset(2isize) = -*v.offset(2isize);
}
unsafe extern "C" fn CrossProduct(
    mut v1: *const vec_t,
    mut v2: *const vec_t,
    mut cross: *mut vec_t,
) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) - *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) - *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) - *v1.offset(1isize) * *v2.offset(0isize);
}
//
// cg_ents.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_SetEntitySoundPosition(mut cent: *mut centity_t) {
    if (*cent).currentState.solid == 0xffffffi32 {
        let mut origin: vec3_t = [0.; 3];
        let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
        v = cgs.inlineModelMidpoints[(*cent).currentState.modelindex as usize].as_mut_ptr();
        origin[0usize] = (*cent).lerpOrigin[0usize] + *v.offset(0isize);
        origin[1usize] = (*cent).lerpOrigin[1usize] + *v.offset(1isize);
        origin[2usize] = (*cent).lerpOrigin[2usize] + *v.offset(2isize);
        trap_S_UpdateEntityPosition(
            (*cent).currentState.number,
            origin.as_mut_ptr() as *const vec_t,
        );
    } else {
        trap_S_UpdateEntityPosition(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_AddPacketEntities() {
    let mut num: libc::c_int = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    if !cg.nextSnap.is_null() {
        let mut delta: libc::c_int = 0;
        delta = (*cg.nextSnap).serverTime - (*cg.snap).serverTime;
        if delta == 0i32 {
            cg.frameInterpolation = 0i32 as libc::c_float
        } else {
            cg.frameInterpolation =
                (cg.time - (*cg.snap).serverTime) as libc::c_float / delta as libc::c_float
        }
    } else {
        cg.frameInterpolation = 0i32 as libc::c_float
    }
    cg.autoAngles[0usize] = 0i32 as vec_t;
    cg.autoAngles[1usize] = (((cg.time & 2047i32) * 360i32) as libc::c_double / 2048.0f64) as vec_t;
    cg.autoAngles[2usize] = 0i32 as vec_t;
    cg.autoAnglesFast[0usize] = 0i32 as vec_t;
    cg.autoAnglesFast[1usize] = ((cg.time & 1023i32) * 360i32) as libc::c_float / 1024.0f32;
    cg.autoAnglesFast[2usize] = 0i32 as vec_t;
    AnglesToAxis(
        cg.autoAngles.as_mut_ptr() as *const vec_t,
        cg.autoAxis.as_mut_ptr(),
    );
    AnglesToAxis(
        cg.autoAnglesFast.as_mut_ptr() as *const vec_t,
        cg.autoAxisFast.as_mut_ptr(),
    );
    ps = &mut cg.predictedPlayerState;
    BG_PlayerStateToEntityState(ps, &mut cg.predictedPlayerEntity.currentState, qfalse);
    CG_AddCEntity(&mut cg.predictedPlayerEntity);
    CG_CalcEntityLerpPositions(&mut cg_entities[(*cg.snap).ps.clientNum as usize]);
    num = 0i32;
    while num < (*cg.snap).numEntities {
        cent =
            &mut cg_entities[(*cg.snap).entities[num as usize].number as usize] as *mut centity_t;
        CG_AddCEntity(cent);
        num += 1
    }
}
/*
===============
CG_AddCEntity

===============
*/
unsafe extern "C" fn CG_AddCEntity(mut cent: *mut centity_t) {
    if (*cent).currentState.eType >= ET_EVENTS as libc::c_int {
        return;
    }
    CG_CalcEntityLerpPositions(cent);
    CG_EntityEffects(cent);
    match (*cent).currentState.eType {
        10 | 8 | 9 => {}
        0 => {
            CG_General(cent);
        }
        1 => {
            CG_Player(cent);
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
            CG_Error(
                b"Bad entity type: %i\x00" as *const u8 as *const libc::c_char,
                (*cent).currentState.eType,
            );
        }
    };
}
/*
===============
CG_TeamBase
===============
*/
unsafe extern "C" fn CG_TeamBase(mut cent: *mut centity_t) {
    let mut model: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint {
        memset(
            &mut model as *mut refEntity_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
        );
        model.reType = RT_MODEL;
        model.lightingOrigin[0usize] = (*cent).lerpOrigin[0usize];
        model.lightingOrigin[1usize] = (*cent).lerpOrigin[1usize];
        model.lightingOrigin[2usize] = (*cent).lerpOrigin[2usize];
        model.origin[0usize] = (*cent).lerpOrigin[0usize];
        model.origin[1usize] = (*cent).lerpOrigin[1usize];
        model.origin[2usize] = (*cent).lerpOrigin[2usize];
        AnglesToAxis(
            (*cent).currentState.angles.as_mut_ptr() as *const vec_t,
            model.axis.as_mut_ptr(),
        );
        if (*cent).currentState.modelindex == TEAM_RED as libc::c_int {
            model.hModel = cgs.media.redFlagBaseModel
        } else if (*cent).currentState.modelindex == TEAM_BLUE as libc::c_int {
            model.hModel = cgs.media.blueFlagBaseModel
        } else {
            model.hModel = cgs.media.neutralFlagBaseModel
        }
        trap_R_AddRefEntityToScene(&mut model);
    };
}
/*
===============
CG_Grapple

This is called when the grapple is sitting up against the wall
===============
*/
unsafe extern "C" fn CG_Grapple(mut cent: *mut centity_t) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut s1: *mut entityState_t = 0 as *mut entityState_t;
    let mut weapon: *const weaponInfo_t = 0 as *const weaponInfo_t;
    s1 = &mut (*cent).currentState;
    if (*s1).weapon >= WP_NUM_WEAPONS as libc::c_int {
        (*s1).weapon = 0i32
    }
    weapon = &mut cg_weapons[(*s1).weapon as usize] as *mut weaponInfo_t;
    (*cent).lerpAngles[0usize] = (*s1).angles[0usize];
    (*cent).lerpAngles[1usize] = (*s1).angles[1usize];
    (*cent).lerpAngles[2usize] = (*s1).angles[2usize];
    CG_GrappleTrail(cent, weapon);
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] = (*cent).lerpOrigin[0usize];
    ent.origin[1usize] = (*cent).lerpOrigin[1usize];
    ent.origin[2usize] = (*cent).lerpOrigin[2usize];
    ent.oldorigin[0usize] = (*cent).lerpOrigin[0usize];
    ent.oldorigin[1usize] = (*cent).lerpOrigin[1usize];
    ent.oldorigin[2usize] = (*cent).lerpOrigin[2usize];
    ent.skinNum = cg.clientFrame & 1i32;
    ent.hModel = (*weapon).missileModel;
    ent.renderfx = (*weapon).missileRenderfx | 0x40i32;
    if VectorNormalize2(
        (*s1).pos.trDelta.as_mut_ptr() as *const vec_t,
        ent.axis[0usize].as_mut_ptr(),
    ) == 0i32 as libc::c_float
    {
        ent.axis[0usize][2usize] = 1i32 as vec_t
    }
    trap_R_AddRefEntityToScene(&mut ent);
}
/*
==================
CG_Speaker

Speaker entities can automatically play sounds
==================
*/
unsafe extern "C" fn CG_Speaker(mut cent: *mut centity_t) {
    if 0 == (*cent).currentState.clientNum {
        return;
    }
    if cg.time < (*cent).miscTime {
        return;
    }
    trap_S_StartSound(
        0 as *mut vec_t,
        (*cent).currentState.number,
        CHAN_ITEM as libc::c_int,
        cgs.gameSounds[(*cent).currentState.eventParm as usize],
    );
    (*cent).miscTime = ((cg.time + (*cent).currentState.frame * 100i32) as libc::c_double
        + ((*cent).currentState.clientNum * 100i32) as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))) as libc::c_int;
}
/*
===============
CG_Portal
===============
*/
unsafe extern "C" fn CG_Portal(mut cent: *mut centity_t) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut s1: *mut entityState_t = 0 as *mut entityState_t;
    s1 = &mut (*cent).currentState;
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] = (*cent).lerpOrigin[0usize];
    ent.origin[1usize] = (*cent).lerpOrigin[1usize];
    ent.origin[2usize] = (*cent).lerpOrigin[2usize];
    ent.oldorigin[0usize] = (*s1).origin2[0usize];
    ent.oldorigin[1usize] = (*s1).origin2[1usize];
    ent.oldorigin[2usize] = (*s1).origin2[2usize];
    ByteToDir((*s1).eventParm, ent.axis[0usize].as_mut_ptr());
    PerpendicularVector(
        ent.axis[1usize].as_mut_ptr(),
        ent.axis[0usize].as_mut_ptr() as *const vec_t,
    );
    ent.axis[1usize][0usize] = vec3_origin[0usize] - ent.axis[1usize][0usize];
    ent.axis[1usize][1usize] = vec3_origin[1usize] - ent.axis[1usize][1usize];
    ent.axis[1usize][2usize] = vec3_origin[2usize] - ent.axis[1usize][2usize];
    CrossProduct(
        ent.axis[0usize].as_mut_ptr() as *const vec_t,
        ent.axis[1usize].as_mut_ptr() as *const vec_t,
        ent.axis[2usize].as_mut_ptr(),
    );
    ent.reType = RT_PORTALSURFACE;
    ent.oldframe = (*s1).powerups;
    ent.frame = (*s1).frame;
    ent.skinNum =
        ((*s1).clientNum as libc::c_double / 256.0f64 * 360i32 as libc::c_double) as libc::c_int;
    trap_R_AddRefEntityToScene(&mut ent);
}
#[no_mangle]
pub unsafe extern "C" fn CG_Beam(mut cent: *mut centity_t) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut s1: *mut entityState_t = 0 as *mut entityState_t;
    s1 = &mut (*cent).currentState;
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] = (*s1).pos.trBase[0usize];
    ent.origin[1usize] = (*s1).pos.trBase[1usize];
    ent.origin[2usize] = (*s1).pos.trBase[2usize];
    ent.oldorigin[0usize] = (*s1).origin2[0usize];
    ent.oldorigin[1usize] = (*s1).origin2[1usize];
    ent.oldorigin[2usize] = (*s1).origin2[2usize];
    AxisClear(ent.axis.as_mut_ptr());
    ent.reType = RT_BEAM;
    ent.renderfx = 0x40i32;
    trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_Mover
===============
*/
unsafe extern "C" fn CG_Mover(mut cent: *mut centity_t) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut s1: *mut entityState_t = 0 as *mut entityState_t;
    s1 = &mut (*cent).currentState;
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] = (*cent).lerpOrigin[0usize];
    ent.origin[1usize] = (*cent).lerpOrigin[1usize];
    ent.origin[2usize] = (*cent).lerpOrigin[2usize];
    ent.oldorigin[0usize] = (*cent).lerpOrigin[0usize];
    ent.oldorigin[1usize] = (*cent).lerpOrigin[1usize];
    ent.oldorigin[2usize] = (*cent).lerpOrigin[2usize];
    AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const vec_t,
        ent.axis.as_mut_ptr(),
    );
    ent.renderfx = 0x40i32;
    ent.skinNum = cg.time >> 6i32 & 1i32;
    if (*s1).solid == 0xffffffi32 {
        ent.hModel = cgs.inlineDrawModel[(*s1).modelindex as usize]
    } else {
        ent.hModel = cgs.gameModels[(*s1).modelindex as usize]
    }
    trap_R_AddRefEntityToScene(&mut ent);
    if 0 != (*s1).modelindex2 {
        ent.skinNum = 0i32;
        ent.hModel = cgs.gameModels[(*s1).modelindex2 as usize];
        trap_R_AddRefEntityToScene(&mut ent);
    };
}
//============================================================================
/*
===============
CG_Missile
===============
*/
unsafe extern "C" fn CG_Missile(mut cent: *mut centity_t) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut s1: *mut entityState_t = 0 as *mut entityState_t;
    let mut weapon: *const weaponInfo_t = 0 as *const weaponInfo_t;
    s1 = &mut (*cent).currentState;
    if (*s1).weapon >= WP_NUM_WEAPONS as libc::c_int {
        (*s1).weapon = 0i32
    }
    weapon = &mut cg_weapons[(*s1).weapon as usize] as *mut weaponInfo_t;
    (*cent).lerpAngles[0usize] = (*s1).angles[0usize];
    (*cent).lerpAngles[1usize] = (*s1).angles[1usize];
    (*cent).lerpAngles[2usize] = (*s1).angles[2usize];
    if (*weapon).missileTrailFunc.is_some() {
        (*weapon)
            .missileTrailFunc
            .expect("non-null function pointer")(cent, weapon);
    }
    if 0. != (*weapon).missileDlight {
        trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            (*weapon).missileDlight,
            (*weapon).missileDlightColor[0usize],
            (*weapon).missileDlightColor[1usize],
            (*weapon).missileDlightColor[2usize],
        );
    }
    if 0 != (*weapon).missileSound {
        let mut velocity: vec3_t = [0.; 3];
        BG_EvaluateTrajectoryDelta(
            &mut (*cent).currentState.pos,
            cg.time,
            velocity.as_mut_ptr(),
        );
        trap_S_AddLoopingSound(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            velocity.as_mut_ptr() as *const vec_t,
            (*weapon).missileSound,
        );
    }
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] = (*cent).lerpOrigin[0usize];
    ent.origin[1usize] = (*cent).lerpOrigin[1usize];
    ent.origin[2usize] = (*cent).lerpOrigin[2usize];
    ent.oldorigin[0usize] = (*cent).lerpOrigin[0usize];
    ent.oldorigin[1usize] = (*cent).lerpOrigin[1usize];
    ent.oldorigin[2usize] = (*cent).lerpOrigin[2usize];
    if (*cent).currentState.weapon == WP_PLASMAGUN as libc::c_int {
        ent.reType = RT_SPRITE;
        ent.radius = 16i32 as libc::c_float;
        ent.rotation = 0i32 as libc::c_float;
        ent.customShader = cgs.media.plasmaBallShader;
        trap_R_AddRefEntityToScene(&mut ent);
        return;
    }
    ent.skinNum = cg.clientFrame & 1i32;
    ent.hModel = (*weapon).missileModel;
    ent.renderfx = (*weapon).missileRenderfx | 0x40i32;
    if VectorNormalize2(
        (*s1).pos.trDelta.as_mut_ptr() as *const vec_t,
        ent.axis[0usize].as_mut_ptr(),
    ) == 0i32 as libc::c_float
    {
        ent.axis[0usize][2usize] = 1i32 as vec_t
    }
    if (*s1).pos.trType as libc::c_uint != TR_STATIONARY as libc::c_int as libc::c_uint {
        RotateAroundDirection(ent.axis.as_mut_ptr(), (cg.time / 4i32) as libc::c_float);
    } else {
        RotateAroundDirection(ent.axis.as_mut_ptr(), (*s1).time as libc::c_float);
    }
    CG_AddRefEntityWithPowerups(&mut ent, s1, TEAM_FREE as libc::c_int);
}
/*
==================
CG_Item
==================
*/
unsafe extern "C" fn CG_Item(mut cent: *mut centity_t) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut msec: libc::c_int = 0;
    let mut frac: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut wi: *mut weaponInfo_t = 0 as *mut weaponInfo_t;
    es = &mut (*cent).currentState;
    if (*es).modelindex >= bg_numItems {
        CG_Error(
            b"Bad item index %i on entity\x00" as *const u8 as *const libc::c_char,
            (*es).modelindex,
        );
    }
    if 0 == (*es).modelindex || 0 != (*es).eFlags & 0x80i32 {
        return;
    }
    item = &mut *bg_itemlist.as_mut_ptr().offset((*es).modelindex as isize) as *mut gitem_t;
    if 0 != cg_simpleItems.integer
        && (*item).giType as libc::c_uint != IT_TEAM as libc::c_int as libc::c_uint
    {
        memset(
            &mut ent as *mut refEntity_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
        );
        ent.reType = RT_SPRITE;
        ent.origin[0usize] = (*cent).lerpOrigin[0usize];
        ent.origin[1usize] = (*cent).lerpOrigin[1usize];
        ent.origin[2usize] = (*cent).lerpOrigin[2usize];
        ent.radius = 14i32 as libc::c_float;
        ent.customShader = cg_items[(*es).modelindex as usize].icon;
        ent.shaderRGBA[0usize] = 255i32 as byte;
        ent.shaderRGBA[1usize] = 255i32 as byte;
        ent.shaderRGBA[2usize] = 255i32 as byte;
        ent.shaderRGBA[3usize] = 255i32 as byte;
        trap_R_AddRefEntityToScene(&mut ent);
        return;
    }
    scale =
        (0.005f64 + (*cent).currentState.number as libc::c_double * 0.00001f64) as libc::c_float;
    (*cent).lerpOrigin[2usize] = ((*cent).lerpOrigin[2usize] as libc::c_double
        + (4i32 as libc::c_double
            + cos(((cg.time + 1000i32) as libc::c_float * scale) as libc::c_double)
                * 4i32 as libc::c_double)) as vec_t;
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    if (*item).giType as libc::c_uint == IT_HEALTH as libc::c_int as libc::c_uint {
        (*cent).lerpAngles[0usize] = cg.autoAnglesFast[0usize];
        (*cent).lerpAngles[1usize] = cg.autoAnglesFast[1usize];
        (*cent).lerpAngles[2usize] = cg.autoAnglesFast[2usize];
        AxisCopy(cg.autoAxisFast.as_mut_ptr(), ent.axis.as_mut_ptr());
    } else {
        (*cent).lerpAngles[0usize] = cg.autoAngles[0usize];
        (*cent).lerpAngles[1usize] = cg.autoAngles[1usize];
        (*cent).lerpAngles[2usize] = cg.autoAngles[2usize];
        AxisCopy(cg.autoAxis.as_mut_ptr(), ent.axis.as_mut_ptr());
    }
    wi = 0 as *mut weaponInfo_t;
    if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint {
        wi = &mut cg_weapons[(*item).giTag as usize] as *mut weaponInfo_t;
        (*cent).lerpOrigin[0usize] -= (*wi).weaponMidpoint[0usize] * ent.axis[0usize][0usize]
            + (*wi).weaponMidpoint[1usize] * ent.axis[1usize][0usize]
            + (*wi).weaponMidpoint[2usize] * ent.axis[2usize][0usize];
        (*cent).lerpOrigin[1usize] -= (*wi).weaponMidpoint[0usize] * ent.axis[0usize][1usize]
            + (*wi).weaponMidpoint[1usize] * ent.axis[1usize][1usize]
            + (*wi).weaponMidpoint[2usize] * ent.axis[2usize][1usize];
        (*cent).lerpOrigin[2usize] -= (*wi).weaponMidpoint[0usize] * ent.axis[0usize][2usize]
            + (*wi).weaponMidpoint[1usize] * ent.axis[1usize][2usize]
            + (*wi).weaponMidpoint[2usize] * ent.axis[2usize][2usize];
        (*cent).lerpOrigin[2usize] += 8i32 as libc::c_float
    }
    if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint
        && (*item).giTag == WP_RAILGUN as libc::c_int
    {
        let mut ci: *mut clientInfo_t =
            &mut cgs.clientinfo[(*cg.snap).ps.clientNum as usize] as *mut clientInfo_t;
        ent.shaderRGBA[0usize] = (*ci).c1RGBA[0usize];
        ent.shaderRGBA[1usize] = (*ci).c1RGBA[1usize];
        ent.shaderRGBA[2usize] = (*ci).c1RGBA[2usize];
        ent.shaderRGBA[3usize] = (*ci).c1RGBA[3usize]
    }
    ent.hModel = cg_items[(*es).modelindex as usize].models[0usize];
    ent.origin[0usize] = (*cent).lerpOrigin[0usize];
    ent.origin[1usize] = (*cent).lerpOrigin[1usize];
    ent.origin[2usize] = (*cent).lerpOrigin[2usize];
    ent.oldorigin[0usize] = (*cent).lerpOrigin[0usize];
    ent.oldorigin[1usize] = (*cent).lerpOrigin[1usize];
    ent.oldorigin[2usize] = (*cent).lerpOrigin[2usize];
    ent.nonNormalizedAxes = qfalse;
    msec = cg.time - (*cent).miscTime;
    if msec >= 0i32 && msec < 1000i32 {
        frac = msec as libc::c_float / 1000i32 as libc::c_float;
        ent.axis[0usize][0usize] = ent.axis[0usize][0usize] * frac;
        ent.axis[0usize][1usize] = ent.axis[0usize][1usize] * frac;
        ent.axis[0usize][2usize] = ent.axis[0usize][2usize] * frac;
        ent.axis[1usize][0usize] = ent.axis[1usize][0usize] * frac;
        ent.axis[1usize][1usize] = ent.axis[1usize][1usize] * frac;
        ent.axis[1usize][2usize] = ent.axis[1usize][2usize] * frac;
        ent.axis[2usize][0usize] = ent.axis[2usize][0usize] * frac;
        ent.axis[2usize][1usize] = ent.axis[2usize][1usize] * frac;
        ent.axis[2usize][2usize] = ent.axis[2usize][2usize] * frac;
        ent.nonNormalizedAxes = qtrue
    } else {
        frac = 1.0f64 as libc::c_float
    }
    if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint
        || (*item).giType as libc::c_uint == IT_ARMOR as libc::c_int as libc::c_uint
    {
        ent.renderfx |= 0x1i32
    }
    if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint {
        ent.axis[0usize][0usize] = (ent.axis[0usize][0usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[0usize][1usize] = (ent.axis[0usize][1usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[0usize][2usize] = (ent.axis[0usize][2usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[1usize][0usize] = (ent.axis[1usize][0usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[1usize][1usize] = (ent.axis[1usize][1usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[1usize][2usize] = (ent.axis[1usize][2usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[2usize][0usize] = (ent.axis[2usize][0usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[2usize][1usize] = (ent.axis[2usize][1usize] as libc::c_double * 1.5f64) as vec_t;
        ent.axis[2usize][2usize] = (ent.axis[2usize][2usize] as libc::c_double * 1.5f64) as vec_t;
        ent.nonNormalizedAxes = qtrue
    }
    trap_R_AddRefEntityToScene(&mut ent);
    if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint
        && !wi.is_null()
        && 0 != (*wi).barrelModel
    {
        let mut barrel: refEntity_t = refEntity_t {
            reType: RT_MODEL,
            renderfx: 0,
            hModel: 0,
            lightingOrigin: [0.; 3],
            shadowPlane: 0.,
            axis: [[0.; 3]; 3],
            nonNormalizedAxes: qfalse,
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
        let mut angles: vec3_t = [0.; 3];
        memset(
            &mut barrel as *mut refEntity_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
        );
        barrel.hModel = (*wi).barrelModel;
        barrel.lightingOrigin[0usize] = ent.lightingOrigin[0usize];
        barrel.lightingOrigin[1usize] = ent.lightingOrigin[1usize];
        barrel.lightingOrigin[2usize] = ent.lightingOrigin[2usize];
        barrel.shadowPlane = ent.shadowPlane;
        barrel.renderfx = ent.renderfx;
        angles[1usize] = 0i32 as vec_t;
        angles[0usize] = 0i32 as vec_t;
        angles[2usize] = 0i32 as vec_t;
        AnglesToAxis(
            angles.as_mut_ptr() as *const vec_t,
            barrel.axis.as_mut_ptr(),
        );
        CG_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut ent,
            (*wi).weaponModel,
            b"tag_barrel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        barrel.nonNormalizedAxes = ent.nonNormalizedAxes;
        trap_R_AddRefEntityToScene(&mut barrel);
    }
    if 0 == cg_simpleItems.integer {
        let mut spinAngles: vec3_t = [0.; 3];
        spinAngles[2usize] = 0i32 as vec_t;
        spinAngles[1usize] = spinAngles[2usize];
        spinAngles[0usize] = spinAngles[1usize];
        if (*item).giType as libc::c_uint == IT_HEALTH as libc::c_int as libc::c_uint
            || (*item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint
        {
            ent.hModel = cg_items[(*es).modelindex as usize].models[1usize];
            if ent.hModel != 0i32 {
                if (*item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint {
                    ent.origin[2usize] += 12i32 as libc::c_float;
                    spinAngles[1usize] =
                        ((cg.time & 1023i32) * 360i32) as libc::c_float / -1024.0f32
                }
                AnglesToAxis(
                    spinAngles.as_mut_ptr() as *const vec_t,
                    ent.axis.as_mut_ptr(),
                );
                if frac as libc::c_double != 1.0f64 {
                    ent.axis[0usize][0usize] = ent.axis[0usize][0usize] * frac;
                    ent.axis[0usize][1usize] = ent.axis[0usize][1usize] * frac;
                    ent.axis[0usize][2usize] = ent.axis[0usize][2usize] * frac;
                    ent.axis[1usize][0usize] = ent.axis[1usize][0usize] * frac;
                    ent.axis[1usize][1usize] = ent.axis[1usize][1usize] * frac;
                    ent.axis[1usize][2usize] = ent.axis[1usize][2usize] * frac;
                    ent.axis[2usize][0usize] = ent.axis[2usize][0usize] * frac;
                    ent.axis[2usize][1usize] = ent.axis[2usize][1usize] * frac;
                    ent.axis[2usize][2usize] = ent.axis[2usize][2usize] * frac;
                    ent.nonNormalizedAxes = qtrue
                }
                trap_R_AddRefEntityToScene(&mut ent);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_PositionRotatedEntityOnTag(
    mut entity: *mut refEntity_t,
    mut parent: *const refEntity_t,
    mut parentModel: qhandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut lerped: orientation_t = orientation_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    let mut tempAxis: [vec3_t; 3] = [[0.; 3]; 3];
    trap_R_LerpTag(
        &mut lerped,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as libc::c_double) as libc::c_float,
        tagName,
    );
    (*entity).origin[0usize] = (*parent).origin[0usize];
    (*entity).origin[1usize] = (*parent).origin[1usize];
    (*entity).origin[2usize] = (*parent).origin[2usize];
    i = 0i32;
    while i < 3i32 {
        (*entity).origin[0usize] = (*entity).origin[0usize]
            + (*parent).axis[i as usize][0usize] * lerped.origin[i as usize];
        (*entity).origin[1usize] = (*entity).origin[1usize]
            + (*parent).axis[i as usize][1usize] * lerped.origin[i as usize];
        (*entity).origin[2usize] = (*entity).origin[2usize]
            + (*parent).axis[i as usize][2usize] * lerped.origin[i as usize];
        i += 1
    }
    MatrixMultiply(
        (*entity).axis.as_mut_ptr(),
        lerped.axis.as_mut_ptr(),
        tempAxis.as_mut_ptr(),
    );
    MatrixMultiply(
        tempAxis.as_mut_ptr(),
        (*(parent as *mut refEntity_t)).axis.as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
}
/*
==================
CG_General
==================
*/
unsafe extern "C" fn CG_General(mut cent: *mut centity_t) {
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
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
    let mut s1: *mut entityState_t = 0 as *mut entityState_t;
    s1 = &mut (*cent).currentState;
    if 0 == (*s1).modelindex {
        return;
    }
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.frame = (*s1).frame;
    ent.oldframe = ent.frame;
    ent.backlerp = 0i32 as libc::c_float;
    ent.origin[0usize] = (*cent).lerpOrigin[0usize];
    ent.origin[1usize] = (*cent).lerpOrigin[1usize];
    ent.origin[2usize] = (*cent).lerpOrigin[2usize];
    ent.oldorigin[0usize] = (*cent).lerpOrigin[0usize];
    ent.oldorigin[1usize] = (*cent).lerpOrigin[1usize];
    ent.oldorigin[2usize] = (*cent).lerpOrigin[2usize];
    ent.hModel = cgs.gameModels[(*s1).modelindex as usize];
    if (*s1).number == (*cg.snap).ps.clientNum {
        ent.renderfx |= 0x2i32
    }
    AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const vec_t,
        ent.axis.as_mut_ptr(),
    );
    trap_R_AddRefEntityToScene(&mut ent);
}
/*
==================
CG_EntityEffects

Add continuous entity effects, like local entity emission and lighting
==================
*/
unsafe extern "C" fn CG_EntityEffects(mut cent: *mut centity_t) {
    CG_SetEntitySoundPosition(cent);
    if 0 != (*cent).currentState.loopSound {
        if (*cent).currentState.eType != ET_SPEAKER as libc::c_int {
            trap_S_AddLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
                vec3_origin.as_mut_ptr() as *const vec_t,
                cgs.gameSounds[(*cent).currentState.loopSound as usize],
            );
        } else {
            trap_S_AddRealLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
                vec3_origin.as_mut_ptr() as *const vec_t,
                cgs.gameSounds[(*cent).currentState.loopSound as usize],
            );
        }
    }
    if 0 != (*cent).currentState.constantLight {
        let mut cl: libc::c_int = 0;
        let mut i: libc::c_float = 0.;
        let mut r: libc::c_float = 0.;
        let mut g: libc::c_float = 0.;
        let mut b: libc::c_float = 0.;
        cl = (*cent).currentState.constantLight;
        r = ((cl & 0xffi32) as libc::c_float as libc::c_double / 255.0f64) as libc::c_float;
        g = ((cl >> 8i32 & 0xffi32) as libc::c_float as libc::c_double / 255.0f64) as libc::c_float;
        b = ((cl >> 16i32 & 0xffi32) as libc::c_float as libc::c_double / 255.0f64)
            as libc::c_float;
        i = ((cl >> 24i32 & 0xffi32) as libc::c_float as libc::c_double * 4.0f64) as libc::c_float;
        trap_R_AddLightToScene((*cent).lerpOrigin.as_mut_ptr() as *const vec_t, i, r, g, b);
    };
}
/*
===============
CG_CalcEntityLerpPositions

===============
*/
unsafe extern "C" fn CG_CalcEntityLerpPositions(mut cent: *mut centity_t) {
    if 0 == cg_smoothClients.integer {
        if (*cent).currentState.number < 64i32 {
            (*cent).currentState.pos.trType = TR_INTERPOLATE;
            (*cent).nextState.pos.trType = TR_INTERPOLATE
        }
    }
    if 0 != (*cent).interpolate as libc::c_uint
        && (*cent).currentState.pos.trType as libc::c_uint
            == TR_INTERPOLATE as libc::c_int as libc::c_uint
    {
        CG_InterpolateEntityPosition(cent);
        return;
    }
    if 0 != (*cent).interpolate as libc::c_uint
        && (*cent).currentState.pos.trType as libc::c_uint
            == TR_LINEAR_STOP as libc::c_int as libc::c_uint
        && (*cent).currentState.number < 64i32
    {
        CG_InterpolateEntityPosition(cent);
        return;
    }
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        cg.time,
        (*cent).lerpOrigin.as_mut_ptr(),
    );
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos,
        cg.time,
        (*cent).lerpAngles.as_mut_ptr(),
    );
    if cent != &mut cg.predictedPlayerEntity as *mut centity_t {
        CG_AdjustPositionForMover(
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            (*cent).currentState.groundEntityNum,
            (*cg.snap).serverTime,
            cg.time,
            (*cent).lerpOrigin.as_mut_ptr(),
            (*cent).lerpAngles.as_mut_ptr(),
            (*cent).lerpAngles.as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_AdjustPositionForMover(
    mut in_0: *const vec_t,
    mut moverNum: libc::c_int,
    mut fromTime: libc::c_int,
    mut toTime: libc::c_int,
    mut out: *mut vec_t,
    mut angles_in: *mut vec_t,
    mut angles_out: *mut vec_t,
) {
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut oldOrigin: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut deltaOrigin: vec3_t = [0.; 3];
    let mut oldAngles: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut deltaAngles: vec3_t = [0.; 3];
    let mut matrix: [vec3_t; 3] = [[0.; 3]; 3];
    let mut transpose: [vec3_t; 3] = [[0.; 3]; 3];
    let mut org: vec3_t = [0.; 3];
    let mut org2: vec3_t = [0.; 3];
    let mut move2: vec3_t = [0.; 3];
    if moverNum <= 0i32 || moverNum >= (1i32 << 10i32) - 2i32 {
        *out.offset(0isize) = *in_0.offset(0isize);
        *out.offset(1isize) = *in_0.offset(1isize);
        *out.offset(2isize) = *in_0.offset(2isize);
        *angles_out.offset(0isize) = *angles_in.offset(0isize);
        *angles_out.offset(1isize) = *angles_in.offset(1isize);
        *angles_out.offset(2isize) = *angles_in.offset(2isize);
        return;
    }
    cent = &mut cg_entities[moverNum as usize] as *mut centity_t;
    if (*cent).currentState.eType != ET_MOVER as libc::c_int {
        *out.offset(0isize) = *in_0.offset(0isize);
        *out.offset(1isize) = *in_0.offset(1isize);
        *out.offset(2isize) = *in_0.offset(2isize);
        *angles_out.offset(0isize) = *angles_in.offset(0isize);
        *angles_out.offset(1isize) = *angles_in.offset(1isize);
        *angles_out.offset(2isize) = *angles_in.offset(2isize);
        return;
    }
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        fromTime,
        oldOrigin.as_mut_ptr(),
    );
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos,
        fromTime,
        oldAngles.as_mut_ptr(),
    );
    BG_EvaluateTrajectory(&mut (*cent).currentState.pos, toTime, origin.as_mut_ptr());
    BG_EvaluateTrajectory(&mut (*cent).currentState.apos, toTime, angles.as_mut_ptr());
    deltaOrigin[0usize] = origin[0usize] - oldOrigin[0usize];
    deltaOrigin[1usize] = origin[1usize] - oldOrigin[1usize];
    deltaOrigin[2usize] = origin[2usize] - oldOrigin[2usize];
    deltaAngles[0usize] = angles[0usize] - oldAngles[0usize];
    deltaAngles[1usize] = angles[1usize] - oldAngles[1usize];
    deltaAngles[2usize] = angles[2usize] - oldAngles[2usize];
    CG_CreateRotationMatrix(deltaAngles.as_mut_ptr(), transpose.as_mut_ptr());
    CG_TransposeMatrix(transpose.as_mut_ptr(), matrix.as_mut_ptr());
    org[0usize] = *in_0.offset(0isize) - oldOrigin[0usize];
    org[1usize] = *in_0.offset(1isize) - oldOrigin[1usize];
    org[2usize] = *in_0.offset(2isize) - oldOrigin[2usize];
    org2[0usize] = org[0usize];
    org2[1usize] = org[1usize];
    org2[2usize] = org[2usize];
    CG_RotatePoint(org2.as_mut_ptr(), matrix.as_mut_ptr());
    move2[0usize] = org2[0usize] - org[0usize];
    move2[1usize] = org2[1usize] - org[1usize];
    move2[2usize] = org2[2usize] - org[2usize];
    deltaOrigin[0usize] = deltaOrigin[0usize] + move2[0usize];
    deltaOrigin[1usize] = deltaOrigin[1usize] + move2[1usize];
    deltaOrigin[2usize] = deltaOrigin[2usize] + move2[2usize];
    *out.offset(0isize) = *in_0.offset(0isize) + deltaOrigin[0usize];
    *out.offset(1isize) = *in_0.offset(1isize) + deltaOrigin[1usize];
    *out.offset(2isize) = *in_0.offset(2isize) + deltaOrigin[2usize];
    *angles_out.offset(0isize) = *angles_in.offset(0isize) + deltaAngles[0usize];
    *angles_out.offset(1isize) = *angles_in.offset(1isize) + deltaAngles[1usize];
    *angles_out.offset(2isize) = *angles_in.offset(2isize) + deltaAngles[2usize];
}
/*
================
CG_RotatePoint
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_RotatePoint(mut point: *mut vec_t, mut matrix: *mut vec3_t) {
    let mut tvec: vec3_t = [0.; 3];
    tvec[0usize] = *point.offset(0isize);
    tvec[1usize] = *point.offset(1isize);
    tvec[2usize] = *point.offset(2isize);
    *point.offset(0isize) = (*matrix.offset(0isize))[0usize] * tvec[0usize]
        + (*matrix.offset(0isize))[1usize] * tvec[1usize]
        + (*matrix.offset(0isize))[2usize] * tvec[2usize];
    *point.offset(1isize) = (*matrix.offset(1isize))[0usize] * tvec[0usize]
        + (*matrix.offset(1isize))[1usize] * tvec[1usize]
        + (*matrix.offset(1isize))[2usize] * tvec[2usize];
    *point.offset(2isize) = (*matrix.offset(2isize))[0usize] * tvec[0usize]
        + (*matrix.offset(2isize))[1usize] * tvec[1usize]
        + (*matrix.offset(2isize))[2usize] * tvec[2usize];
}
/*
================
CG_TransposeMatrix
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_TransposeMatrix(mut matrix: *mut vec3_t, mut transpose: *mut vec3_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        j = 0i32;
        while j < 3i32 {
            (*transpose.offset(i as isize))[j as usize] = (*matrix.offset(j as isize))[i as usize];
            j += 1
        }
        i += 1
    }
}
/*
================
CG_CreateRotationMatrix
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_CreateRotationMatrix(mut angles: *mut vec_t, mut matrix: *mut vec3_t) {
    AngleVectors(
        angles as *const vec_t,
        (*matrix.offset(0isize)).as_mut_ptr(),
        (*matrix.offset(1isize)).as_mut_ptr(),
        (*matrix.offset(2isize)).as_mut_ptr(),
    );
    VectorInverse((*matrix.offset(1isize)).as_mut_ptr());
}
/*
=============================
CG_InterpolateEntityPosition
=============================
*/
unsafe extern "C" fn CG_InterpolateEntityPosition(mut cent: *mut centity_t) {
    let mut current: vec3_t = [0.; 3];
    let mut next: vec3_t = [0.; 3];
    let mut f: libc::c_float = 0.;
    if cg.nextSnap.is_null() {
        CG_Error(
            b"CG_InterpoateEntityPosition: cg.nextSnap == NULL\x00" as *const u8
                as *const libc::c_char,
        );
    }
    f = cg.frameInterpolation;
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        (*cg.snap).serverTime,
        current.as_mut_ptr(),
    );
    BG_EvaluateTrajectory(
        &mut (*cent).nextState.pos,
        (*cg.nextSnap).serverTime,
        next.as_mut_ptr(),
    );
    (*cent).lerpOrigin[0usize] = current[0usize] + f * (next[0usize] - current[0usize]);
    (*cent).lerpOrigin[1usize] = current[1usize] + f * (next[1usize] - current[1usize]);
    (*cent).lerpOrigin[2usize] = current[2usize] + f * (next[2usize] - current[2usize]);
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.apos,
        (*cg.snap).serverTime,
        current.as_mut_ptr(),
    );
    BG_EvaluateTrajectory(
        &mut (*cent).nextState.apos,
        (*cg.nextSnap).serverTime,
        next.as_mut_ptr(),
    );
    (*cent).lerpAngles[0usize] = LerpAngle(current[0usize], next[0usize], f);
    (*cent).lerpAngles[1usize] = LerpAngle(current[1usize], next[1usize], f);
    (*cent).lerpAngles[2usize] = LerpAngle(current[2usize], next[2usize], f);
}
#[no_mangle]
pub unsafe extern "C" fn CG_PositionEntityOnTag(
    mut entity: *mut refEntity_t,
    mut parent: *const refEntity_t,
    mut parentModel: qhandle_t,
    mut tagName: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut lerped: orientation_t = orientation_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    trap_R_LerpTag(
        &mut lerped,
        parentModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as libc::c_double) as libc::c_float,
        tagName,
    );
    (*entity).origin[0usize] = (*parent).origin[0usize];
    (*entity).origin[1usize] = (*parent).origin[1usize];
    (*entity).origin[2usize] = (*parent).origin[2usize];
    i = 0i32;
    while i < 3i32 {
        (*entity).origin[0usize] = (*entity).origin[0usize]
            + (*parent).axis[i as usize][0usize] * lerped.origin[i as usize];
        (*entity).origin[1usize] = (*entity).origin[1usize]
            + (*parent).axis[i as usize][1usize] * lerped.origin[i as usize];
        (*entity).origin[2usize] = (*entity).origin[2usize]
            + (*parent).axis[i as usize][2usize] * lerped.origin[i as usize];
        i += 1
    }
    MatrixMultiply(
        lerped.axis.as_mut_ptr(),
        (*(parent as *mut refEntity_t)).axis.as_mut_ptr(),
        (*entity).axis.as_mut_ptr(),
    );
    (*entity).backlerp = (*parent).backlerp;
}
