#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, unnamed, unnamed_0, unnamed_1,
    unnamed_2, BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3, BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, ET_BEAM,
    ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER,
    ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, FLAG_RUN, FLAG_STAND,
    FLAG_STAND2RUN, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
    LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK, LEGS_IDLE, LEGS_IDLECR, LEGS_JUMP, LEGS_JUMPB,
    LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM, LEGS_TURN, LEGS_WALK, LEGS_WALKCR, MAX_ANIMATIONS,
    MAX_TOTALANIMATIONS, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES,
    PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS,
    PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT,
    PERS_TEAM, PW_AMMOREGEN, PW_BATTLESUIT, PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE,
    PW_INVIS, PW_INVULNERABILITY, PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG,
    PW_REGEN, PW_SCOUT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    TORSO_AFFIRMATIVE, TORSO_ATTACK, TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE,
    TORSO_GETFLAG, TORSO_GUARDBASE, TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND,
    TORSO_STAND2,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
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
use cg_ents::{
    CG_AddPacketEntities, CG_AdjustPositionForMover, CG_Beam, CG_PositionEntityOnTag,
    CG_PositionRotatedEntityOnTag, CG_SetEntitySoundPosition,
};
use cg_event::{CG_CheckEvents, CG_EntityEvent, CG_PainEvent, CG_PlaceString};
use cg_info::{CG_DrawInformation, CG_LoadingClient, CG_LoadingItem, CG_LoadingString};
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, leBounceSoundType_t,
    leMarkType_t, leType_t, lerpFrame_t, localEntity_s, localEntity_t, playerEntity_t, score_t,
    trap_CM_BoxTrace, trap_Cvar_VariableStringBuffer, trap_FS_FCloseFile, trap_FS_FOpenFile,
    trap_FS_Read, trap_MemoryRemaining, trap_R_AddLightToScene, trap_R_AddPolyToScene,
    trap_R_AddRefEntityToScene, trap_R_LerpTag, trap_R_LightForPoint, trap_R_RegisterModel,
    trap_R_RegisterShaderNoMip, trap_R_RegisterSkin, trap_S_AddLoopingSound, trap_S_RegisterSound,
    FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL,
    FOOTSTEP_SPLASH, FOOTSTEP_TOTAL, LEBS_BLOOD, LEBS_BRASS, LEBS_NONE, LEMT_BLOOD, LEMT_BURN,
    LEMT_NONE, LE_EXPLOSION, LE_FADE_RGB, LE_FALL_SCALE_FADE, LE_FRAGMENT, LE_MARK,
    LE_MOVE_SCALE_FADE, LE_SCALE_FADE, LE_SCOREPLUM, LE_SPRITE_EXPLOSION,
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
use cg_playerstate::{CG_Respawn, CG_TransitionPlayerState};
use cg_predict::{CG_BuildSolidList, CG_PointContents, CG_PredictPlayerState, CG_Trace};
use cg_public_h::snapshot_t;
use cg_scoreboard::{CG_DrawOldScoreboard, CG_DrawTourneyScoreboard};
use cg_servercmds::{
    CG_ExecuteNewServerCommands, CG_ParseServerinfo, CG_SetConfigValues, CG_ShaderStateChanged,
};
use cg_snapshot::CG_ProcessSnapshots;
use cg_variadic_h::{CG_Error, CG_Printf};
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
use q_math::{
    axisDefault, colorWhite, g_color_table, vec3_origin, vectoangles, AngleMod, AngleNormalize180,
    AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis, AxisClear, AxisCopy, ByteToDir,
    LerpAngle, MatrixMultiply, PerpendicularVector, Q_crandom, Q_random, RotateAroundDirection,
    RotatePointAroundVector, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, clipHandle_t, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t,
    fileHandle_t, fsMode_t, gameState_t, orientation_t, playerState_s, playerState_t, qboolean,
    qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trace_t, trajectory_t, va, vec3_t, vec_t,
    vmCvar_t, COM_Parse, Com_Printf, Com_sprintf, Info_ValueForKey, Q_stricmp, Q_strncpyz,
    FS_APPEND, FS_APPEND_SYNC, FS_READ, FS_WRITE, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{acos, atof, atoi, fabs, memcpy, memset, rand, strchr, strcmp};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, polyVert_t, refEntityType_t, refEntity_t,
    refdef_t, textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D,
    GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING,
    RT_MAX_REF_ENTITY_TYPE, RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS,
    RT_SPRITE, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};
extern crate libc;

//
// cg_player.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_Player(mut cent: *mut centity_t) {
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut legs: refEntity_t = refEntity_t {
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
    let mut torso: refEntity_t = refEntity_t {
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
    let mut head: refEntity_t = refEntity_t {
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
    let mut clientNum: libc::c_int = 0;
    let mut renderfx: libc::c_int = 0;
    let mut shadow: qboolean = qfalse;
    let mut shadowPlane: libc::c_float = 0.;
    clientNum = (*cent).currentState.clientNum;
    if clientNum < 0i32 || clientNum >= 64i32 {
        CG_Error(b"Bad clientNum on player entity\x00" as *const u8 as *const libc::c_char);
    }
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
    if 0 == (*ci).infoValid as u64 {
        return;
    }
    renderfx = 0i32;
    if (*cent).currentState.number == (*cg.snap).ps.clientNum {
        if 0 == cg.renderingThirdPerson as u64 {
            renderfx = 0x2i32
        } else if 0 != cg_cameraMode.integer {
            return;
        }
    }
    memset(
        &mut legs as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    memset(
        &mut torso as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    memset(
        &mut head as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    CG_PlayerAngles(
        cent,
        legs.axis.as_mut_ptr(),
        torso.axis.as_mut_ptr(),
        head.axis.as_mut_ptr(),
    );
    CG_PlayerAnimation(
        cent,
        &mut legs.oldframe,
        &mut legs.frame,
        &mut legs.backlerp,
        &mut torso.oldframe,
        &mut torso.frame,
        &mut torso.backlerp,
    );
    CG_PlayerSprites(cent);
    shadow = CG_PlayerShadow(cent, &mut shadowPlane);
    CG_PlayerSplash(cent);
    if cg_shadows.integer == 3i32 && 0 != shadow as libc::c_uint {
        renderfx |= 0x100i32
    }
    renderfx |= 0x80i32;
    legs.hModel = (*ci).legsModel;
    legs.customSkin = (*ci).legsSkin;
    legs.origin[0usize] = (*cent).lerpOrigin[0usize];
    legs.origin[1usize] = (*cent).lerpOrigin[1usize];
    legs.origin[2usize] = (*cent).lerpOrigin[2usize];
    legs.lightingOrigin[0usize] = (*cent).lerpOrigin[0usize];
    legs.lightingOrigin[1usize] = (*cent).lerpOrigin[1usize];
    legs.lightingOrigin[2usize] = (*cent).lerpOrigin[2usize];
    legs.shadowPlane = shadowPlane;
    legs.renderfx = renderfx;
    legs.oldorigin[0usize] = legs.origin[0usize];
    legs.oldorigin[1usize] = legs.origin[1usize];
    legs.oldorigin[2usize] = legs.origin[2usize];
    CG_AddRefEntityWithPowerups(
        &mut legs,
        &mut (*cent).currentState,
        (*ci).team as libc::c_int,
    );
    if 0 == legs.hModel {
        return;
    }
    torso.hModel = (*ci).torsoModel;
    if 0 == torso.hModel {
        return;
    }
    torso.customSkin = (*ci).torsoSkin;
    torso.lightingOrigin[0usize] = (*cent).lerpOrigin[0usize];
    torso.lightingOrigin[1usize] = (*cent).lerpOrigin[1usize];
    torso.lightingOrigin[2usize] = (*cent).lerpOrigin[2usize];
    CG_PositionRotatedEntityOnTag(
        &mut torso,
        &mut legs,
        (*ci).legsModel,
        b"tag_torso\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    torso.shadowPlane = shadowPlane;
    torso.renderfx = renderfx;
    CG_AddRefEntityWithPowerups(
        &mut torso,
        &mut (*cent).currentState,
        (*ci).team as libc::c_int,
    );
    head.hModel = (*ci).headModel;
    if 0 == head.hModel {
        return;
    }
    head.customSkin = (*ci).headSkin;
    head.lightingOrigin[0usize] = (*cent).lerpOrigin[0usize];
    head.lightingOrigin[1usize] = (*cent).lerpOrigin[1usize];
    head.lightingOrigin[2usize] = (*cent).lerpOrigin[2usize];
    CG_PositionRotatedEntityOnTag(
        &mut head,
        &mut torso,
        (*ci).torsoModel,
        b"tag_head\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    head.shadowPlane = shadowPlane;
    head.renderfx = renderfx;
    CG_AddRefEntityWithPowerups(
        &mut head,
        &mut (*cent).currentState,
        (*ci).team as libc::c_int,
    );
    CG_AddPlayerWeapon(
        &mut torso,
        0 as *mut playerState_t,
        cent,
        (*ci).team as libc::c_int,
    );
    CG_PlayerPowerups(cent, &mut torso);
}
/*
===============
CG_PlayerPowerups
===============
*/
unsafe extern "C" fn CG_PlayerPowerups(mut cent: *mut centity_t, mut torso: *mut refEntity_t) {
    let mut powerups: libc::c_int = 0;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    powerups = (*cent).currentState.powerups;
    if 0 == powerups {
        return;
    }
    if 0 != powerups & 1i32 << PW_QUAD as libc::c_int {
        trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            (200i32 + (rand() & 31i32)) as libc::c_float,
            0.2f32,
            0.2f32,
            1i32 as libc::c_float,
        );
    }
    if 0 != powerups & 1i32 << PW_FLIGHT as libc::c_int {
        trap_S_AddLoopingSound(
            (*cent).currentState.number,
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            vec3_origin.as_mut_ptr() as *const vec_t,
            cgs.media.flightSound,
        );
    }
    ci = &mut *cgs
        .clientinfo
        .as_mut_ptr()
        .offset((*cent).currentState.clientNum as isize) as *mut clientInfo_t;
    if 0 != powerups & 1i32 << PW_REDFLAG as libc::c_int {
        if 0 != (*ci).newAnims as u64 {
            CG_PlayerFlag(cent, cgs.media.redFlagFlapSkin, torso);
        } else {
            CG_TrailItem(cent, cgs.media.redFlagModel);
        }
        trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            (200i32 + (rand() & 31i32)) as libc::c_float,
            1.0f64 as libc::c_float,
            0.2f32,
            0.2f32,
        );
    }
    if 0 != powerups & 1i32 << PW_BLUEFLAG as libc::c_int {
        if 0 != (*ci).newAnims as u64 {
            CG_PlayerFlag(cent, cgs.media.blueFlagFlapSkin, torso);
        } else {
            CG_TrailItem(cent, cgs.media.blueFlagModel);
        }
        trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            (200i32 + (rand() & 31i32)) as libc::c_float,
            0.2f32,
            0.2f32,
            1.0f64 as libc::c_float,
        );
    }
    if 0 != powerups & 1i32 << PW_NEUTRALFLAG as libc::c_int {
        if 0 != (*ci).newAnims as u64 {
            CG_PlayerFlag(cent, cgs.media.neutralFlagFlapSkin, torso);
        } else {
            CG_TrailItem(cent, cgs.media.neutralFlagModel);
        }
        trap_R_AddLightToScene(
            (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
            (200i32 + (rand() & 31i32)) as libc::c_float,
            1.0f64 as libc::c_float,
            1.0f64 as libc::c_float,
            1.0f64 as libc::c_float,
        );
    }
    if 0 != powerups & 1i32 << PW_HASTE as libc::c_int {
        CG_HasteTrail(cent);
    };
}
//==========================================================================
/*
===============
CG_HasteTrail
===============
*/
unsafe extern "C" fn CG_HasteTrail(mut cent: *mut centity_t) {
    let mut smoke: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut origin: vec3_t = [0.; 3];
    let mut anim: libc::c_int = 0;
    if (*cent).trailTime > cg.time {
        return;
    }
    anim = (*cent).pe.legs.animationNumber & !128i32;
    if anim != LEGS_RUN as libc::c_int && anim != LEGS_BACK as libc::c_int {
        return;
    }
    (*cent).trailTime += 100i32;
    if (*cent).trailTime < cg.time {
        (*cent).trailTime = cg.time
    }
    origin[0usize] = (*cent).lerpOrigin[0usize];
    origin[1usize] = (*cent).lerpOrigin[1usize];
    origin[2usize] = (*cent).lerpOrigin[2usize];
    origin[2usize] -= 16i32 as libc::c_float;
    smoke = CG_SmokePuff(
        origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        8i32 as libc::c_float,
        1i32 as libc::c_float,
        1i32 as libc::c_float,
        1i32 as libc::c_float,
        1i32 as libc::c_float,
        500i32 as libc::c_float,
        cg.time,
        0i32,
        0i32,
        cgs.media.hastePuffShader,
    );
    (*smoke).leType = LE_SCALE_FADE;
}
/*
===============
CG_TrailItem
===============
*/
unsafe extern "C" fn CG_TrailItem(mut cent: *mut centity_t, mut hModel: qhandle_t) {
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
    let mut angles: vec3_t = [0.; 3];
    let mut axis: [vec3_t; 3] = [[0.; 3]; 3];
    angles[0usize] = (*cent).lerpAngles[0usize];
    angles[1usize] = (*cent).lerpAngles[1usize];
    angles[2usize] = (*cent).lerpAngles[2usize];
    angles[0usize] = 0i32 as vec_t;
    angles[2usize] = 0i32 as vec_t;
    AnglesToAxis(angles.as_mut_ptr() as *const vec_t, axis.as_mut_ptr());
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] =
        (*cent).lerpOrigin[0usize] + axis[0usize][0usize] * -16i32 as libc::c_float;
    ent.origin[1usize] =
        (*cent).lerpOrigin[1usize] + axis[0usize][1usize] * -16i32 as libc::c_float;
    ent.origin[2usize] =
        (*cent).lerpOrigin[2usize] + axis[0usize][2usize] * -16i32 as libc::c_float;
    ent.origin[2usize] += 16i32 as libc::c_float;
    angles[1usize] += 90i32 as libc::c_float;
    AnglesToAxis(angles.as_mut_ptr() as *const vec_t, ent.axis.as_mut_ptr());
    ent.hModel = hModel;
    trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_PlayerFlag
===============
*/
unsafe extern "C" fn CG_PlayerFlag(
    mut cent: *mut centity_t,
    mut hSkin: qhandle_t,
    mut torso: *mut refEntity_t,
) {
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut pole: refEntity_t = refEntity_t {
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
    let mut flag: refEntity_t = refEntity_t {
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
    let mut dir: vec3_t = [0.; 3];
    let mut legsAnim: libc::c_int = 0;
    let mut flagAnim: libc::c_int = 0;
    let mut updateangles: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut d: libc::c_float = 0.;
    memset(
        &mut pole as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    pole.hModel = cgs.media.flagPoleModel;
    pole.lightingOrigin[0usize] = (*torso).lightingOrigin[0usize];
    pole.lightingOrigin[1usize] = (*torso).lightingOrigin[1usize];
    pole.lightingOrigin[2usize] = (*torso).lightingOrigin[2usize];
    pole.shadowPlane = (*torso).shadowPlane;
    pole.renderfx = (*torso).renderfx;
    CG_PositionEntityOnTag(
        &mut pole,
        torso,
        (*torso).hModel,
        b"tag_flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_R_AddRefEntityToScene(&mut pole);
    memset(
        &mut flag as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    flag.hModel = cgs.media.flagFlapModel;
    flag.customSkin = hSkin;
    flag.lightingOrigin[0usize] = (*torso).lightingOrigin[0usize];
    flag.lightingOrigin[1usize] = (*torso).lightingOrigin[1usize];
    flag.lightingOrigin[2usize] = (*torso).lightingOrigin[2usize];
    flag.shadowPlane = (*torso).shadowPlane;
    flag.renderfx = (*torso).renderfx;
    angles[2usize] = 0i32 as vec_t;
    angles[1usize] = angles[2usize];
    angles[0usize] = angles[1usize];
    updateangles = qfalse as libc::c_int;
    legsAnim = (*cent).currentState.legsAnim & !128i32;
    if legsAnim == LEGS_IDLE as libc::c_int || legsAnim == LEGS_IDLECR as libc::c_int {
        flagAnim = FLAG_STAND as libc::c_int
    } else if legsAnim == LEGS_WALK as libc::c_int || legsAnim == LEGS_WALKCR as libc::c_int {
        flagAnim = FLAG_STAND as libc::c_int;
        updateangles = qtrue as libc::c_int
    } else {
        flagAnim = FLAG_RUN as libc::c_int;
        updateangles = qtrue as libc::c_int
    }
    if 0 != updateangles {
        dir[0usize] = (*cent).currentState.pos.trDelta[0usize];
        dir[1usize] = (*cent).currentState.pos.trDelta[1usize];
        dir[2usize] = (*cent).currentState.pos.trDelta[2usize];
        dir[2usize] += 100i32 as libc::c_float;
        VectorNormalize(dir.as_mut_ptr());
        d = pole.axis[2usize][0usize] * dir[0usize]
            + pole.axis[2usize][1usize] * dir[1usize]
            + pole.axis[2usize][2usize] * dir[2usize];
        if fabs(d as libc::c_double) < 0.9f64 {
            d = pole.axis[0usize][0usize] * dir[0usize]
                + pole.axis[0usize][1usize] * dir[1usize]
                + pole.axis[0usize][2usize] * dir[2usize];
            if d > 1.0f32 {
                d = 1.0f32
            } else if d < -1.0f32 {
                d = -1.0f32
            }
            angle = acos(d as libc::c_double) as libc::c_float;
            d = pole.axis[1usize][0usize] * dir[0usize]
                + pole.axis[1usize][1usize] * dir[1usize]
                + pole.axis[1usize][2usize] * dir[2usize];
            if d < 0i32 as libc::c_float {
                angles[1usize] = (360i32 as libc::c_double
                    - (angle * 180i32 as libc::c_float) as libc::c_double
                        / 3.14159265358979323846f64) as vec_t
            } else {
                angles[1usize] = ((angle * 180i32 as libc::c_float) as libc::c_double
                    / 3.14159265358979323846f64) as vec_t
            }
            if angles[1usize] < 0i32 as libc::c_float {
                angles[1usize] += 360i32 as libc::c_float
            }
            if angles[1usize] > 360i32 as libc::c_float {
                angles[1usize] -= 360i32 as libc::c_float
            }
            CG_SwingAngles(
                angles[1usize],
                25i32 as libc::c_float,
                90i32 as libc::c_float,
                0.15f32,
                &mut (*cent).pe.flag.yawAngle,
                &mut (*cent).pe.flag.yawing,
            );
        }
    }
    angles[1usize] = (*cent).pe.flag.yawAngle;
    ci = &mut *cgs
        .clientinfo
        .as_mut_ptr()
        .offset((*cent).currentState.clientNum as isize) as *mut clientInfo_t;
    CG_RunLerpFrame(ci, &mut (*cent).pe.flag, flagAnim, 1i32 as libc::c_float);
    flag.oldframe = (*cent).pe.flag.oldFrame;
    flag.frame = (*cent).pe.flag.frame;
    flag.backlerp = (*cent).pe.flag.backlerp;
    AnglesToAxis(angles.as_mut_ptr() as *const vec_t, flag.axis.as_mut_ptr());
    CG_PositionRotatedEntityOnTag(
        &mut flag,
        &mut pole,
        pole.hModel,
        b"tag_flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_R_AddRefEntityToScene(&mut flag);
}
/*
===============
CG_RunLerpFrame

Sets cg.snap, cg.oldFrame, and cg.backlerp
cg.time should be between oldFrameTime and frameTime after exit
===============
*/
unsafe extern "C" fn CG_RunLerpFrame(
    mut ci: *mut clientInfo_t,
    mut lf: *mut lerpFrame_t,
    mut newAnimation: libc::c_int,
    mut speedScale: libc::c_float,
) {
    let mut f: libc::c_int = 0;
    let mut numFrames: libc::c_int = 0;
    let mut anim: *mut animation_t = 0 as *mut animation_t;
    if cg_animSpeed.integer == 0i32 {
        (*lf).backlerp = 0i32 as libc::c_float;
        (*lf).frame = (*lf).backlerp as libc::c_int;
        (*lf).oldFrame = (*lf).frame;
        return;
    }
    if newAnimation != (*lf).animationNumber || (*lf).animation.is_null() {
        CG_SetLerpFrameAnimation(ci, lf, newAnimation);
    }
    if cg.time >= (*lf).frameTime {
        (*lf).oldFrame = (*lf).frame;
        (*lf).oldFrameTime = (*lf).frameTime;
        anim = (*lf).animation;
        if 0 == (*anim).frameLerp {
            return;
        }
        if cg.time < (*lf).animationTime {
            (*lf).frameTime = (*lf).animationTime
        } else {
            (*lf).frameTime = (*lf).oldFrameTime + (*anim).frameLerp
        }
        f = ((*lf).frameTime - (*lf).animationTime) / (*anim).frameLerp;
        f = (f as libc::c_float * speedScale) as libc::c_int;
        numFrames = (*anim).numFrames;
        if 0 != (*anim).flipflop {
            numFrames *= 2i32
        }
        if f >= numFrames {
            f -= numFrames;
            if 0 != (*anim).loopFrames {
                f %= (*anim).loopFrames;
                f += (*anim).numFrames - (*anim).loopFrames
            } else {
                f = numFrames - 1i32;
                (*lf).frameTime = cg.time
            }
        }
        if 0 != (*anim).reversed {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1i32 - f
        } else if 0 != (*anim).flipflop && f >= (*anim).numFrames {
            (*lf).frame = (*anim).firstFrame + (*anim).numFrames - 1i32 - f % (*anim).numFrames
        } else {
            (*lf).frame = (*anim).firstFrame + f
        }
        if cg.time > (*lf).frameTime {
            (*lf).frameTime = cg.time;
            if 0 != cg_debugAnim.integer {
                CG_Printf(b"Clamp lf->frameTime\n\x00" as *const u8 as *const libc::c_char);
            }
        }
    }
    if (*lf).frameTime > cg.time + 200i32 {
        (*lf).frameTime = cg.time
    }
    if (*lf).oldFrameTime > cg.time {
        (*lf).oldFrameTime = cg.time
    }
    if (*lf).frameTime == (*lf).oldFrameTime {
        (*lf).backlerp = 0i32 as libc::c_float
    } else {
        (*lf).backlerp = (1.0f64
            - ((cg.time - (*lf).oldFrameTime) as libc::c_float
                / ((*lf).frameTime - (*lf).oldFrameTime) as libc::c_float)
                as libc::c_double) as libc::c_float
    };
}
//			break;
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
    mut ci: *mut clientInfo_t,
    mut lf: *mut lerpFrame_t,
    mut newAnimation: libc::c_int,
) {
    let mut anim: *mut animation_t = 0 as *mut animation_t;
    (*lf).animationNumber = newAnimation;
    newAnimation &= !128i32;
    if newAnimation < 0i32 || newAnimation >= MAX_TOTALANIMATIONS as libc::c_int {
        CG_Error(
            b"Bad animation number: %i\x00" as *const u8 as *const libc::c_char,
            newAnimation,
        );
    }
    anim = &mut *(*ci).animations.as_mut_ptr().offset(newAnimation as isize) as *mut animation_t;
    (*lf).animation = anim;
    (*lf).animationTime = (*lf).frameTime + (*anim).initialLerp;
    if 0 != cg_debugAnim.integer {
        CG_Printf(
            b"Anim: %i\n\x00" as *const u8 as *const libc::c_char,
            newAnimation,
        );
    };
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
    mut destination: libc::c_float,
    mut swingTolerance: libc::c_float,
    mut clampTolerance: libc::c_float,
    mut speed: libc::c_float,
    mut angle: *mut libc::c_float,
    mut swinging: *mut qboolean,
) {
    let mut swing: libc::c_float = 0.;
    let mut move_0: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    if 0 == *swinging as u64 {
        swing = AngleSubtract(*angle, destination);
        if swing > swingTolerance || swing < -swingTolerance {
            *swinging = qtrue
        }
    }
    if 0 == *swinging as u64 {
        return;
    }
    swing = AngleSubtract(destination, *angle);
    scale = fabs(swing as libc::c_double) as libc::c_float;
    if (scale as libc::c_double) < swingTolerance as libc::c_double * 0.5f64 {
        scale = 0.5f64 as libc::c_float
    } else if scale < swingTolerance {
        scale = 1.0f64 as libc::c_float
    } else {
        scale = 2.0f64 as libc::c_float
    }
    if swing >= 0i32 as libc::c_float {
        move_0 = cg.frametime as libc::c_float * scale * speed;
        if move_0 >= swing {
            move_0 = swing;
            *swinging = qfalse
        }
        *angle = AngleMod(*angle + move_0)
    } else if swing < 0i32 as libc::c_float {
        move_0 = cg.frametime as libc::c_float * scale * -speed;
        if move_0 <= swing {
            move_0 = swing;
            *swinging = qfalse
        }
        *angle = AngleMod(*angle + move_0)
    }
    swing = AngleSubtract(destination, *angle);
    if swing > clampTolerance {
        *angle = AngleMod(destination - (clampTolerance - 1i32 as libc::c_float))
    } else if swing < -clampTolerance {
        *angle = AngleMod(destination + (clampTolerance - 1i32 as libc::c_float))
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_AddRefEntityWithPowerups(
    mut ent: *mut refEntity_t,
    mut state: *mut entityState_t,
    mut team: libc::c_int,
) {
    if 0 != (*state).powerups & 1i32 << PW_INVIS as libc::c_int {
        (*ent).customShader = cgs.media.invisShader;
        trap_R_AddRefEntityToScene(ent);
    } else {
        trap_R_AddRefEntityToScene(ent);
        if 0 != (*state).powerups & 1i32 << PW_QUAD as libc::c_int {
            if team == TEAM_RED as libc::c_int {
                (*ent).customShader = cgs.media.redQuadShader
            } else {
                (*ent).customShader = cgs.media.quadShader
            }
            trap_R_AddRefEntityToScene(ent);
        }
        if 0 != (*state).powerups & 1i32 << PW_REGEN as libc::c_int {
            if cg.time / 100i32 % 10i32 == 1i32 {
                (*ent).customShader = cgs.media.regenShader;
                trap_R_AddRefEntityToScene(ent);
            }
        }
        if 0 != (*state).powerups & 1i32 << PW_BATTLESUIT as libc::c_int {
            (*ent).customShader = cgs.media.battleSuitShader;
            trap_R_AddRefEntityToScene(ent);
        }
    };
}
/*
===============
CG_PlayerSplash

Draw a mark at the water surface
===============
*/
unsafe extern "C" fn CG_PlayerSplash(mut cent: *mut centity_t) {
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
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
    let mut contents: libc::c_int = 0;
    let mut verts: [polyVert_t; 4] = [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    if 0 == cg_shadows.integer {
        return;
    }
    end[0usize] = (*cent).lerpOrigin[0usize];
    end[1usize] = (*cent).lerpOrigin[1usize];
    end[2usize] = (*cent).lerpOrigin[2usize];
    end[2usize] -= 24i32 as libc::c_float;
    contents = CG_PointContents(end.as_mut_ptr() as *const vec_t, 0i32);
    if 0 == contents & (32i32 | 16i32 | 8i32) {
        return;
    }
    start[0usize] = (*cent).lerpOrigin[0usize];
    start[1usize] = (*cent).lerpOrigin[1usize];
    start[2usize] = (*cent).lerpOrigin[2usize];
    start[2usize] += 32i32 as libc::c_float;
    contents = CG_PointContents(start.as_mut_ptr() as *const vec_t, 0i32);
    if 0 != contents & (1i32 | 32i32 | 16i32 | 8i32) {
        return;
    }
    trap_CM_BoxTrace(
        &mut trace,
        start.as_mut_ptr() as *const vec_t,
        end.as_mut_ptr() as *const vec_t,
        0 as *const vec_t,
        0 as *const vec_t,
        0i32,
        32i32 | 16i32 | 8i32,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        return;
    }
    verts[0usize].xyz[0usize] = trace.endpos[0usize];
    verts[0usize].xyz[1usize] = trace.endpos[1usize];
    verts[0usize].xyz[2usize] = trace.endpos[2usize];
    verts[0usize].xyz[0usize] -= 32i32 as libc::c_float;
    verts[0usize].xyz[1usize] -= 32i32 as libc::c_float;
    verts[0usize].st[0usize] = 0i32 as libc::c_float;
    verts[0usize].st[1usize] = 0i32 as libc::c_float;
    verts[0usize].modulate[0usize] = 255i32 as byte;
    verts[0usize].modulate[1usize] = 255i32 as byte;
    verts[0usize].modulate[2usize] = 255i32 as byte;
    verts[0usize].modulate[3usize] = 255i32 as byte;
    verts[1usize].xyz[0usize] = trace.endpos[0usize];
    verts[1usize].xyz[1usize] = trace.endpos[1usize];
    verts[1usize].xyz[2usize] = trace.endpos[2usize];
    verts[1usize].xyz[0usize] -= 32i32 as libc::c_float;
    verts[1usize].xyz[1usize] += 32i32 as libc::c_float;
    verts[1usize].st[0usize] = 0i32 as libc::c_float;
    verts[1usize].st[1usize] = 1i32 as libc::c_float;
    verts[1usize].modulate[0usize] = 255i32 as byte;
    verts[1usize].modulate[1usize] = 255i32 as byte;
    verts[1usize].modulate[2usize] = 255i32 as byte;
    verts[1usize].modulate[3usize] = 255i32 as byte;
    verts[2usize].xyz[0usize] = trace.endpos[0usize];
    verts[2usize].xyz[1usize] = trace.endpos[1usize];
    verts[2usize].xyz[2usize] = trace.endpos[2usize];
    verts[2usize].xyz[0usize] += 32i32 as libc::c_float;
    verts[2usize].xyz[1usize] += 32i32 as libc::c_float;
    verts[2usize].st[0usize] = 1i32 as libc::c_float;
    verts[2usize].st[1usize] = 1i32 as libc::c_float;
    verts[2usize].modulate[0usize] = 255i32 as byte;
    verts[2usize].modulate[1usize] = 255i32 as byte;
    verts[2usize].modulate[2usize] = 255i32 as byte;
    verts[2usize].modulate[3usize] = 255i32 as byte;
    verts[3usize].xyz[0usize] = trace.endpos[0usize];
    verts[3usize].xyz[1usize] = trace.endpos[1usize];
    verts[3usize].xyz[2usize] = trace.endpos[2usize];
    verts[3usize].xyz[0usize] += 32i32 as libc::c_float;
    verts[3usize].xyz[1usize] -= 32i32 as libc::c_float;
    verts[3usize].st[0usize] = 1i32 as libc::c_float;
    verts[3usize].st[1usize] = 0i32 as libc::c_float;
    verts[3usize].modulate[0usize] = 255i32 as byte;
    verts[3usize].modulate[1usize] = 255i32 as byte;
    verts[3usize].modulate[2usize] = 255i32 as byte;
    verts[3usize].modulate[3usize] = 255i32 as byte;
    trap_R_AddPolyToScene(cgs.media.wakeMarkShader, 4i32, verts.as_mut_ptr());
}
/*
===============
CG_PlayerShadow

Returns the Z component of the surface being shadowed

  should it return a full plane instead of a Z?
===============
*/
unsafe extern "C" fn CG_PlayerShadow(
    mut cent: *mut centity_t,
    mut shadowPlane: *mut libc::c_float,
) -> qboolean {
    let mut end: vec3_t = [0.; 3];
    let mut mins: vec3_t = [-15i32 as vec_t, -15i32 as vec_t, 0i32 as vec_t];
    let mut maxs: vec3_t = [15i32 as vec_t, 15i32 as vec_t, 2i32 as vec_t];
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
    let mut alpha: libc::c_float = 0.;
    *shadowPlane = 0i32 as libc::c_float;
    if cg_shadows.integer == 0i32 {
        return qfalse;
    }
    if 0 != (*cent).currentState.powerups & 1i32 << PW_INVIS as libc::c_int {
        return qfalse;
    }
    end[0usize] = (*cent).lerpOrigin[0usize];
    end[1usize] = (*cent).lerpOrigin[1usize];
    end[2usize] = (*cent).lerpOrigin[2usize];
    end[2usize] -= 128i32 as libc::c_float;
    trap_CM_BoxTrace(
        &mut trace,
        (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
        end.as_mut_ptr() as *const vec_t,
        mins.as_mut_ptr() as *const vec_t,
        maxs.as_mut_ptr() as *const vec_t,
        0i32,
        1i32 | 0x10000i32 | 0x2000000i32,
    );
    if trace.fraction as libc::c_double == 1.0f64
        || 0 != trace.startsolid as libc::c_uint
        || 0 != trace.allsolid as libc::c_uint
    {
        return qfalse;
    }
    *shadowPlane = trace.endpos[2usize] + 1i32 as libc::c_float;
    if cg_shadows.integer != 1i32 {
        return qtrue;
    }
    alpha = (1.0f64 - trace.fraction as libc::c_double) as libc::c_float;
    CG_ImpactMark(
        cgs.media.shadowMarkShader,
        trace.endpos.as_mut_ptr() as *const vec_t,
        trace.plane.normal.as_mut_ptr() as *const vec_t,
        (*cent).pe.legs.yawAngle,
        alpha,
        alpha,
        alpha,
        1i32 as libc::c_float,
        qfalse,
        24i32 as libc::c_float,
        qtrue,
    );
    return qtrue;
}
/*
===============
CG_PlayerSprites

Float sprites over the player's head
===============
*/
unsafe extern "C" fn CG_PlayerSprites(mut cent: *mut centity_t) {
    let mut team: libc::c_int = 0;
    if 0 != (*cent).currentState.eFlags & 0x2000i32 {
        CG_PlayerFloatSprite(cent, cgs.media.connectionShader);
        return;
    }
    if 0 != (*cent).currentState.eFlags & 0x1000i32 {
        CG_PlayerFloatSprite(cent, cgs.media.balloonShader);
        return;
    }
    if 0 != (*cent).currentState.eFlags & 0x8000i32 {
        CG_PlayerFloatSprite(cent, cgs.media.medalImpressive);
        return;
    }
    if 0 != (*cent).currentState.eFlags & 0x8i32 {
        CG_PlayerFloatSprite(cent, cgs.media.medalExcellent);
        return;
    }
    if 0 != (*cent).currentState.eFlags & 0x40i32 {
        CG_PlayerFloatSprite(cent, cgs.media.medalGauntlet);
        return;
    }
    if 0 != (*cent).currentState.eFlags & 0x10000i32 {
        CG_PlayerFloatSprite(cent, cgs.media.medalDefend);
        return;
    }
    if 0 != (*cent).currentState.eFlags & 0x20000i32 {
        CG_PlayerFloatSprite(cent, cgs.media.medalAssist);
        return;
    }
    if 0 != (*cent).currentState.eFlags & 0x800i32 {
        CG_PlayerFloatSprite(cent, cgs.media.medalCapture);
        return;
    }
    team = cgs.clientinfo[(*cent).currentState.clientNum as usize].team as libc::c_int;
    if 0 == (*cent).currentState.eFlags & 0x1i32
        && (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == team
        && cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
    {
        if 0 != cg_drawFriend.integer {
            CG_PlayerFloatSprite(cent, cgs.media.friendShader);
        }
        return;
    };
}
/*
===============
CG_PlayerFloatSprite

Float a sprite over the player's head
===============
*/
unsafe extern "C" fn CG_PlayerFloatSprite(mut cent: *mut centity_t, mut shader: qhandle_t) {
    let mut rf: libc::c_int = 0;
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
    if (*cent).currentState.number == (*cg.snap).ps.clientNum && 0 == cg.renderingThirdPerson as u64
    {
        rf = 0x2i32
    } else {
        rf = 0i32
    }
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.origin[0usize] = (*cent).lerpOrigin[0usize];
    ent.origin[1usize] = (*cent).lerpOrigin[1usize];
    ent.origin[2usize] = (*cent).lerpOrigin[2usize];
    ent.origin[2usize] += 48i32 as libc::c_float;
    ent.reType = RT_SPRITE;
    ent.customShader = shader;
    ent.radius = 10i32 as libc::c_float;
    ent.renderfx = rf;
    ent.shaderRGBA[0usize] = 255i32 as byte;
    ent.shaderRGBA[1usize] = 255i32 as byte;
    ent.shaderRGBA[2usize] = 255i32 as byte;
    ent.shaderRGBA[3usize] = 255i32 as byte;
    trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_PlayerAnimation
===============
*/
unsafe extern "C" fn CG_PlayerAnimation(
    mut cent: *mut centity_t,
    mut legsOld: *mut libc::c_int,
    mut legs: *mut libc::c_int,
    mut legsBackLerp: *mut libc::c_float,
    mut torsoOld: *mut libc::c_int,
    mut torso: *mut libc::c_int,
    mut torsoBackLerp: *mut libc::c_float,
) {
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut clientNum: libc::c_int = 0;
    let mut speedScale: libc::c_float = 0.;
    clientNum = (*cent).currentState.clientNum;
    if 0 != cg_noPlayerAnims.integer {
        *torso = 0i32;
        *torsoOld = *torso;
        *legs = *torsoOld;
        *legsOld = *legs;
        return;
    }
    if 0 != (*cent).currentState.powerups & 1i32 << PW_HASTE as libc::c_int {
        speedScale = 1.5f64 as libc::c_float
    } else {
        speedScale = 1i32 as libc::c_float
    }
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
    if 0 != (*cent).pe.legs.yawing as libc::c_uint
        && (*cent).currentState.legsAnim & !128i32 == LEGS_IDLE as libc::c_int
    {
        CG_RunLerpFrame(
            ci,
            &mut (*cent).pe.legs,
            LEGS_TURN as libc::c_int,
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
    mut cent: *mut centity_t,
    mut legs: *mut vec3_t,
    mut torso: *mut vec3_t,
    mut head: *mut vec3_t,
) {
    let mut legsAngles: vec3_t = [0.; 3];
    let mut torsoAngles: vec3_t = [0.; 3];
    let mut headAngles: vec3_t = [0.; 3];
    let mut dest: libc::c_float = 0.;
    static mut movementOffsets: [libc::c_int; 8] =
        [0i32, 22i32, 45i32, -22i32, 0i32, 22i32, -45i32, -22i32];
    let mut velocity: vec3_t = [0.; 3];
    let mut speed: libc::c_float = 0.;
    let mut dir: libc::c_int = 0;
    let mut clientNum: libc::c_int = 0;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    headAngles[0usize] = (*cent).lerpAngles[0usize];
    headAngles[1usize] = (*cent).lerpAngles[1usize];
    headAngles[2usize] = (*cent).lerpAngles[2usize];
    headAngles[1usize] = AngleMod(headAngles[1usize]);
    legsAngles[2usize] = 0i32 as vec_t;
    legsAngles[1usize] = legsAngles[2usize];
    legsAngles[0usize] = legsAngles[1usize];
    torsoAngles[2usize] = 0i32 as vec_t;
    torsoAngles[1usize] = torsoAngles[2usize];
    torsoAngles[0usize] = torsoAngles[1usize];
    if (*cent).currentState.legsAnim & !128i32 != LEGS_IDLE as libc::c_int
        || (*cent).currentState.torsoAnim & !128i32 != TORSO_STAND as libc::c_int
            && (*cent).currentState.torsoAnim & !128i32 != TORSO_STAND2 as libc::c_int
    {
        (*cent).pe.torso.yawing = qtrue;
        (*cent).pe.torso.pitching = qtrue;
        (*cent).pe.legs.yawing = qtrue
    }
    if 0 != (*cent).currentState.eFlags & 0x1i32 {
        dir = 0i32
    } else {
        dir = (*cent).currentState.angles2[1usize] as libc::c_int;
        if dir < 0i32 || dir > 7i32 {
            CG_Error(b"Bad player movement angle\x00" as *const u8 as *const libc::c_char);
        }
    }
    legsAngles[1usize] = headAngles[1usize] + movementOffsets[dir as usize] as libc::c_float;
    torsoAngles[1usize] = (headAngles[1usize] as libc::c_double
        + 0.25f64 * movementOffsets[dir as usize] as libc::c_double)
        as vec_t;
    CG_SwingAngles(
        torsoAngles[1usize],
        25i32 as libc::c_float,
        90i32 as libc::c_float,
        cg_swingSpeed.value,
        &mut (*cent).pe.torso.yawAngle,
        &mut (*cent).pe.torso.yawing,
    );
    CG_SwingAngles(
        legsAngles[1usize],
        40i32 as libc::c_float,
        90i32 as libc::c_float,
        cg_swingSpeed.value,
        &mut (*cent).pe.legs.yawAngle,
        &mut (*cent).pe.legs.yawing,
    );
    torsoAngles[1usize] = (*cent).pe.torso.yawAngle;
    legsAngles[1usize] = (*cent).pe.legs.yawAngle;
    if headAngles[0usize] > 180i32 as libc::c_float {
        dest = (-360i32 as libc::c_float + headAngles[0usize]) * 0.75f32
    } else {
        dest = headAngles[0usize] * 0.75f32
    }
    CG_SwingAngles(
        dest,
        15i32 as libc::c_float,
        30i32 as libc::c_float,
        0.1f32,
        &mut (*cent).pe.torso.pitchAngle,
        &mut (*cent).pe.torso.pitching,
    );
    torsoAngles[0usize] = (*cent).pe.torso.pitchAngle;
    clientNum = (*cent).currentState.clientNum;
    if clientNum >= 0i32 && clientNum < 64i32 {
        ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
        if 0 != (*ci).fixedtorso as u64 {
            torsoAngles[0usize] = 0.0f32
        }
    }
    velocity[0usize] = (*cent).currentState.pos.trDelta[0usize];
    velocity[1usize] = (*cent).currentState.pos.trDelta[1usize];
    velocity[2usize] = (*cent).currentState.pos.trDelta[2usize];
    speed = VectorNormalize(velocity.as_mut_ptr());
    if 0. != speed {
        let mut axis: [vec3_t; 3] = [[0.; 3]; 3];
        let mut side: libc::c_float = 0.;
        speed *= 0.05f32;
        AnglesToAxis(legsAngles.as_mut_ptr() as *const vec_t, axis.as_mut_ptr());
        side = speed
            * (velocity[0usize] * axis[1usize][0usize]
                + velocity[1usize] * axis[1usize][1usize]
                + velocity[2usize] * axis[1usize][2usize]);
        legsAngles[2usize] -= side;
        side = speed
            * (velocity[0usize] * axis[0usize][0usize]
                + velocity[1usize] * axis[0usize][1usize]
                + velocity[2usize] * axis[0usize][2usize]);
        legsAngles[0usize] += side
    }
    clientNum = (*cent).currentState.clientNum;
    if clientNum >= 0i32 && clientNum < 64i32 {
        ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
        if 0 != (*ci).fixedlegs as u64 {
            legsAngles[1usize] = torsoAngles[1usize];
            legsAngles[0usize] = 0.0f32;
            legsAngles[2usize] = 0.0f32
        }
    }
    CG_AddPainTwitch(cent, torsoAngles.as_mut_ptr());
    AnglesSubtract(
        headAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
        headAngles.as_mut_ptr(),
    );
    AnglesSubtract(
        torsoAngles.as_mut_ptr(),
        legsAngles.as_mut_ptr(),
        torsoAngles.as_mut_ptr(),
    );
    AnglesToAxis(legsAngles.as_mut_ptr() as *const vec_t, legs);
    AnglesToAxis(torsoAngles.as_mut_ptr() as *const vec_t, torso);
    AnglesToAxis(headAngles.as_mut_ptr() as *const vec_t, head);
}
/*
=================
CG_AddPainTwitch
=================
*/
unsafe extern "C" fn CG_AddPainTwitch(mut cent: *mut centity_t, mut torsoAngles: *mut vec_t) {
    let mut t: libc::c_int = 0;
    let mut f: libc::c_float = 0.;
    t = cg.time - (*cent).pe.painTime;
    if t >= 200i32 {
        return;
    }
    f = (1.0f64 - (t as libc::c_float / 200i32 as libc::c_float) as libc::c_double)
        as libc::c_float;
    if 0 != (*cent).pe.painDirection {
        let ref mut fresh0 = *torsoAngles.offset(2isize);
        *fresh0 += 20i32 as libc::c_float * f
    } else {
        let ref mut fresh1 = *torsoAngles.offset(2isize);
        *fresh1 -= 20i32 as libc::c_float * f
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_ResetPlayerEntity(mut cent: *mut centity_t) {
    (*cent).errorTime = -99999i32;
    (*cent).extrapolated = qfalse;
    CG_ClearLerpFrame(
        &mut *cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*cent).currentState.clientNum as isize),
        &mut (*cent).pe.legs,
        (*cent).currentState.legsAnim,
    );
    CG_ClearLerpFrame(
        &mut *cgs
            .clientinfo
            .as_mut_ptr()
            .offset((*cent).currentState.clientNum as isize),
        &mut (*cent).pe.torso,
        (*cent).currentState.torsoAnim,
    );
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
    (*cent).rawOrigin[0usize] = (*cent).lerpOrigin[0usize];
    (*cent).rawOrigin[1usize] = (*cent).lerpOrigin[1usize];
    (*cent).rawOrigin[2usize] = (*cent).lerpOrigin[2usize];
    (*cent).rawAngles[0usize] = (*cent).lerpAngles[0usize];
    (*cent).rawAngles[1usize] = (*cent).lerpAngles[1usize];
    (*cent).rawAngles[2usize] = (*cent).lerpAngles[2usize];
    memset(
        &mut (*cent).pe.legs as *mut lerpFrame_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<lerpFrame_t>() as libc::c_ulong,
    );
    (*cent).pe.legs.yawAngle = (*cent).rawAngles[1usize];
    (*cent).pe.legs.yawing = qfalse;
    (*cent).pe.legs.pitchAngle = 0i32 as libc::c_float;
    (*cent).pe.legs.pitching = qfalse;
    memset(
        &mut (*cent).pe.torso as *mut lerpFrame_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<lerpFrame_t>() as libc::c_ulong,
    );
    (*cent).pe.torso.yawAngle = (*cent).rawAngles[1usize];
    (*cent).pe.torso.yawing = qfalse;
    (*cent).pe.torso.pitchAngle = (*cent).rawAngles[0usize];
    (*cent).pe.torso.pitching = qfalse;
    if 0 != cg_debugPosition.integer {
        CG_Printf(
            b"%i ResetPlayerEntity yaw=%f\n\x00" as *const u8 as *const libc::c_char,
            (*cent).currentState.number,
            (*cent).pe.torso.yawAngle as libc::c_double,
        );
    };
}
/*
===============
CG_ClearLerpFrame
===============
*/
unsafe extern "C" fn CG_ClearLerpFrame(
    mut ci: *mut clientInfo_t,
    mut lf: *mut lerpFrame_t,
    mut animationNumber: libc::c_int,
) {
    (*lf).oldFrameTime = cg.time;
    (*lf).frameTime = (*lf).oldFrameTime;
    CG_SetLerpFrameAnimation(ci, lf, animationNumber);
    (*lf).frame = (*(*lf).animation).firstFrame;
    (*lf).oldFrame = (*lf).frame;
}
#[no_mangle]
pub unsafe extern "C" fn CG_NewClientInfo(mut clientNum: libc::c_int) {
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut newInfo: clientInfo_t = clientInfo_t {
        infoValid: qfalse,
        name: [0; 64],
        team: TEAM_FREE,
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
        teamLeader: qfalse,
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
        deferred: qfalse,
        newAnims: qfalse,
        fixedlegs: qfalse,
        fixedtorso: qfalse,
        headOffset: [0.; 3],
        footsteps: FOOTSTEP_NORMAL,
        gender: GENDER_MALE,
        legsModel: 0,
        legsSkin: 0,
        torsoModel: 0,
        torsoSkin: 0,
        headModel: 0,
        headSkin: 0,
        modelIcon: 0,
        animations: [animation_s {
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
    let mut configstring: *const libc::c_char = 0 as *const libc::c_char;
    let mut v: *const libc::c_char = 0 as *const libc::c_char;
    let mut slash: *mut libc::c_char = 0 as *mut libc::c_char;
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
    configstring = CG_ConfigString(clientNum + (32i32 + 256i32 + 256i32));
    if 0 == *configstring.offset(0isize) {
        memset(
            ci as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<clientInfo_t>() as libc::c_ulong,
        );
        return;
    }
    memset(
        &mut newInfo as *mut clientInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<clientInfo_t>() as libc::c_ulong,
    );
    v = Info_ValueForKey(configstring, b"n\x00" as *const u8 as *const libc::c_char);
    Q_strncpyz(
        newInfo.name.as_mut_ptr(),
        v,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    v = Info_ValueForKey(configstring, b"c1\x00" as *const u8 as *const libc::c_char);
    CG_ColorFromString(v, newInfo.color1.as_mut_ptr());
    newInfo.c1RGBA[0usize] = (255i32 as libc::c_float * newInfo.color1[0usize]) as byte;
    newInfo.c1RGBA[1usize] = (255i32 as libc::c_float * newInfo.color1[1usize]) as byte;
    newInfo.c1RGBA[2usize] = (255i32 as libc::c_float * newInfo.color1[2usize]) as byte;
    newInfo.c1RGBA[3usize] = 255i32 as byte;
    v = Info_ValueForKey(configstring, b"c2\x00" as *const u8 as *const libc::c_char);
    CG_ColorFromString(v, newInfo.color2.as_mut_ptr());
    newInfo.c2RGBA[0usize] = (255i32 as libc::c_float * newInfo.color2[0usize]) as byte;
    newInfo.c2RGBA[1usize] = (255i32 as libc::c_float * newInfo.color2[1usize]) as byte;
    newInfo.c2RGBA[2usize] = (255i32 as libc::c_float * newInfo.color2[2usize]) as byte;
    newInfo.c2RGBA[3usize] = 255i32 as byte;
    v = Info_ValueForKey(
        configstring,
        b"skill\x00" as *const u8 as *const libc::c_char,
    );
    newInfo.botSkill = atoi(v);
    v = Info_ValueForKey(configstring, b"hc\x00" as *const u8 as *const libc::c_char);
    newInfo.handicap = atoi(v);
    v = Info_ValueForKey(configstring, b"w\x00" as *const u8 as *const libc::c_char);
    newInfo.wins = atoi(v);
    v = Info_ValueForKey(configstring, b"l\x00" as *const u8 as *const libc::c_char);
    newInfo.losses = atoi(v);
    v = Info_ValueForKey(configstring, b"t\x00" as *const u8 as *const libc::c_char);
    newInfo.team = atoi(v) as team_t;
    v = Info_ValueForKey(configstring, b"tt\x00" as *const u8 as *const libc::c_char);
    newInfo.teamTask = atoi(v);
    v = Info_ValueForKey(configstring, b"tl\x00" as *const u8 as *const libc::c_char);
    newInfo.teamLeader = atoi(v) as qboolean;
    v = Info_ValueForKey(
        configstring,
        b"g_redteam\x00" as *const u8 as *const libc::c_char,
    );
    Q_strncpyz(newInfo.redTeam.as_mut_ptr(), v, 32i32);
    v = Info_ValueForKey(
        configstring,
        b"g_blueteam\x00" as *const u8 as *const libc::c_char,
    );
    Q_strncpyz(newInfo.blueTeam.as_mut_ptr(), v, 32i32);
    v = Info_ValueForKey(
        configstring,
        b"model\x00" as *const u8 as *const libc::c_char,
    );
    if 0 != cg_forceModel.integer {
        let mut modelStr: [libc::c_char; 64] = [0; 64];
        let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
        if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
            Q_strncpyz(
                newInfo.modelName.as_mut_ptr(),
                b"sarge\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
        } else {
            trap_Cvar_VariableStringBuffer(
                b"model\x00" as *const u8 as *const libc::c_char,
                modelStr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            skin = strchr(modelStr.as_mut_ptr(), '/' as i32);
            if skin.is_null() {
                skin = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            } else {
                let fresh2 = skin;
                skin = skin.offset(1);
                *fresh2 = 0i32 as libc::c_char
            }
            Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                skin,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            Q_strncpyz(
                newInfo.modelName.as_mut_ptr(),
                modelStr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
        }
        if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
            slash = strchr(v, '/' as i32);
            if !slash.is_null() {
                Q_strncpyz(
                    newInfo.skinName.as_mut_ptr(),
                    slash.offset(1isize),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                );
            }
        }
    } else {
        Q_strncpyz(
            newInfo.modelName.as_mut_ptr(),
            v,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        slash = strchr(newInfo.modelName.as_mut_ptr(), '/' as i32);
        if slash.is_null() {
            Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
        } else {
            Q_strncpyz(
                newInfo.skinName.as_mut_ptr(),
                slash.offset(1isize),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            *slash = 0i32 as libc::c_char
        }
    }
    v = Info_ValueForKey(
        configstring,
        b"hmodel\x00" as *const u8 as *const libc::c_char,
    );
    if 0 != cg_forceModel.integer {
        let mut modelStr_0: [libc::c_char; 64] = [0; 64];
        let mut skin_0: *mut libc::c_char = 0 as *mut libc::c_char;
        if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
            Q_strncpyz(
                newInfo.headModelName.as_mut_ptr(),
                b"sarge\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
        } else {
            trap_Cvar_VariableStringBuffer(
                b"headmodel\x00" as *const u8 as *const libc::c_char,
                modelStr_0.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            skin_0 = strchr(modelStr_0.as_mut_ptr(), '/' as i32);
            if skin_0.is_null() {
                skin_0 = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            } else {
                let fresh3 = skin_0;
                skin_0 = skin_0.offset(1);
                *fresh3 = 0i32 as libc::c_char
            }
            Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                skin_0,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            Q_strncpyz(
                newInfo.headModelName.as_mut_ptr(),
                modelStr_0.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
        }
        if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
            slash = strchr(v, '/' as i32);
            if !slash.is_null() {
                Q_strncpyz(
                    newInfo.headSkinName.as_mut_ptr(),
                    slash.offset(1isize),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                );
            }
        }
    } else {
        Q_strncpyz(
            newInfo.headModelName.as_mut_ptr(),
            v,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        slash = strchr(newInfo.headModelName.as_mut_ptr(), '/' as i32);
        if slash.is_null() {
            Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                b"default\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
        } else {
            Q_strncpyz(
                newInfo.headSkinName.as_mut_ptr(),
                slash.offset(1isize),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            *slash = 0i32 as libc::c_char
        }
    }
    if 0 == CG_ScanForExistingClientInfo(&mut newInfo) as u64 {
        let mut forceDefer: qboolean = qfalse;
        forceDefer = (trap_MemoryRemaining() < 4000000i32) as libc::c_int as qboolean;
        if 0 != forceDefer as libc::c_uint
            || 0 != cg_deferPlayers.integer && 0 == cg_buildScript.integer && 0 == cg.loading as u64
        {
            CG_SetDeferredClientInfo(clientNum, &mut newInfo);
            if 0 != forceDefer as u64 {
                CG_Printf(
                    b"Memory is low. Using deferred model.\n\x00" as *const u8
                        as *const libc::c_char,
                );
                newInfo.deferred = qfalse
            }
        } else {
            CG_LoadClientInfo(clientNum, &mut newInfo);
        }
    }
    newInfo.infoValid = qtrue;
    *ci = newInfo;
}
/*
===================
CG_LoadClientInfo

Load it now, taking the disk hits.
This will usually be deferred to a safe time
===================
*/
unsafe extern "C" fn CG_LoadClientInfo(mut clientNum: libc::c_int, mut ci: *mut clientInfo_t) {
    let mut dir: *const libc::c_char = 0 as *const libc::c_char;
    let mut fallback: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut modelloaded: libc::c_int = 0;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut teamname: [libc::c_char; 64] = [0; 64];
    teamname[0usize] = 0i32 as libc::c_char;
    modelloaded = qtrue as libc::c_int;
    if 0 == CG_RegisterClientModelname(
        ci,
        (*ci).modelName.as_mut_ptr(),
        (*ci).skinName.as_mut_ptr(),
        (*ci).headModelName.as_mut_ptr(),
        (*ci).headSkinName.as_mut_ptr(),
        teamname.as_mut_ptr(),
    ) as u64
    {
        if 0 != cg_buildScript.integer {
            CG_Error(
                b"CG_RegisterClientModelname( %s, %s, %s, %s %s ) failed\x00" as *const u8
                    as *const libc::c_char,
                (*ci).modelName.as_mut_ptr(),
                (*ci).skinName.as_mut_ptr(),
                (*ci).headModelName.as_mut_ptr(),
                (*ci).headSkinName.as_mut_ptr(),
                teamname.as_mut_ptr(),
            );
        }
        if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
            if (*ci).team as libc::c_uint == TEAM_BLUE as libc::c_int as libc::c_uint {
                Q_strncpyz(
                    teamname.as_mut_ptr(),
                    b"Pagans\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                );
            } else {
                Q_strncpyz(
                    teamname.as_mut_ptr(),
                    b"Stroggs\x00" as *const u8 as *const libc::c_char,
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                );
            }
            if 0 == CG_RegisterClientModelname(
                ci,
                b"sarge\x00" as *const u8 as *const libc::c_char,
                (*ci).skinName.as_mut_ptr(),
                b"sarge\x00" as *const u8 as *const libc::c_char,
                (*ci).skinName.as_mut_ptr(),
                teamname.as_mut_ptr(),
            ) as u64
            {
                CG_Error(
                    b"DEFAULT_TEAM_MODEL / skin (%s/%s) failed to register\x00" as *const u8
                        as *const libc::c_char,
                    b"sarge\x00" as *const u8 as *const libc::c_char,
                    (*ci).skinName.as_mut_ptr(),
                );
            }
        } else if 0
            == CG_RegisterClientModelname(
                ci,
                b"sarge\x00" as *const u8 as *const libc::c_char,
                b"default\x00" as *const u8 as *const libc::c_char,
                b"sarge\x00" as *const u8 as *const libc::c_char,
                b"default\x00" as *const u8 as *const libc::c_char,
                teamname.as_mut_ptr(),
            ) as u64
        {
            CG_Error(
                b"DEFAULT_MODEL (%s) failed to register\x00" as *const u8 as *const libc::c_char,
                b"sarge\x00" as *const u8 as *const libc::c_char,
            );
        }
        modelloaded = qfalse as libc::c_int
    }
    (*ci).newAnims = qfalse;
    if 0 != (*ci).torsoModel {
        let mut tag: orientation_t = orientation_t {
            origin: [0.; 3],
            axis: [[0.; 3]; 3],
        };
        if 0 != trap_R_LerpTag(
            &mut tag,
            (*ci).torsoModel,
            0i32,
            0i32,
            1i32 as libc::c_float,
            b"tag_flag\x00" as *const u8 as *const libc::c_char,
        ) {
            (*ci).newAnims = qtrue
        }
    }
    dir = (*ci).modelName.as_mut_ptr();
    fallback = if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        b"sarge\x00" as *const u8 as *const libc::c_char
    } else {
        b"sarge\x00" as *const u8 as *const libc::c_char
    };
    i = 0i32;
    while i < 32i32 {
        s = cg_customSoundNames[i as usize];
        if s.is_null() {
            break;
        }
        (*ci).sounds[i as usize] = 0i32;
        if 0 != modelloaded {
            (*ci).sounds[i as usize] = trap_S_RegisterSound(
                va(
                    b"sound/player/%s/%s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    dir,
                    s.offset(1isize),
                ),
                qfalse,
            )
        }
        if 0 == (*ci).sounds[i as usize] {
            (*ci).sounds[i as usize] = trap_S_RegisterSound(
                va(
                    b"sound/player/%s/%s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    fallback,
                    s.offset(1isize),
                ),
                qfalse,
            )
        }
        i += 1
    }
    (*ci).deferred = qfalse;
    i = 0i32;
    while i < 1i32 << 10i32 {
        if cg_entities[i as usize].currentState.clientNum == clientNum
            && cg_entities[i as usize].currentState.eType == ET_PLAYER as libc::c_int
        {
            CG_ResetPlayerEntity(&mut *cg_entities.as_mut_ptr().offset(i as isize));
        }
        i += 1
    }
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
// cg_players.c -- handle the media and animation for player entities
#[no_mangle]
pub static mut cg_customSoundNames: [*mut libc::c_char; 32] = [
    b"*death1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*death2.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*death3.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*jump1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*pain25_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*pain50_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*pain75_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*pain100_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*falling1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*gasp.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*drown.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*fall1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"*taunt.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
/*
==========================
CG_RegisterClientModelname
==========================
*/
unsafe extern "C" fn CG_RegisterClientModelname(
    mut ci: *mut clientInfo_t,
    mut modelName: *const libc::c_char,
    mut skinName: *const libc::c_char,
    mut headModelName: *const libc::c_char,
    mut headSkinName: *const libc::c_char,
    mut teamName: *const libc::c_char,
) -> qboolean {
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut headName: *const libc::c_char = 0 as *const libc::c_char;
    let mut newTeamName: [libc::c_char; 64] = [0; 64];
    if *headModelName.offset(0isize) as libc::c_int == '\u{0}' as i32 {
        headName = modelName
    } else {
        headName = headModelName
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/lower.md3\x00" as *const u8 as *const libc::c_char,
        modelName,
    );
    (*ci).legsModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if 0 == (*ci).legsModel {
        Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"models/players/characters/%s/lower.md3\x00" as *const u8 as *const libc::c_char,
            modelName,
        );
        (*ci).legsModel = trap_R_RegisterModel(filename.as_mut_ptr());
        if 0 == (*ci).legsModel {
            Com_Printf(
                b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
            );
            return qfalse;
        }
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/upper.md3\x00" as *const u8 as *const libc::c_char,
        modelName,
    );
    (*ci).torsoModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if 0 == (*ci).torsoModel {
        Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"models/players/characters/%s/upper.md3\x00" as *const u8 as *const libc::c_char,
            modelName,
        );
        (*ci).torsoModel = trap_R_RegisterModel(filename.as_mut_ptr());
        if 0 == (*ci).torsoModel {
            Com_Printf(
                b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
            );
            return qfalse;
        }
    }
    if *headName.offset(0isize) as libc::c_int == '*' as i32 {
        Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"models/players/heads/%s/%s.md3\x00" as *const u8 as *const libc::c_char,
            &*headModelName.offset(1isize) as *const libc::c_char,
            &*headModelName.offset(1isize) as *const libc::c_char,
        );
    } else {
        Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"models/players/%s/head.md3\x00" as *const u8 as *const libc::c_char,
            headName,
        );
    }
    (*ci).headModel = trap_R_RegisterModel(filename.as_mut_ptr());
    if 0 == (*ci).headModel && *headName.offset(0isize) as libc::c_int != '*' as i32 {
        Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"models/players/heads/%s/%s.md3\x00" as *const u8 as *const libc::c_char,
            headModelName,
            headModelName,
        );
        (*ci).headModel = trap_R_RegisterModel(filename.as_mut_ptr())
    }
    if 0 == (*ci).headModel {
        Com_Printf(
            b"Failed to load model file %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return qfalse;
    }
    if 0 == CG_RegisterClientSkin(ci, teamName, modelName, skinName, headName, headSkinName) as u64
    {
        if !teamName.is_null() && 0 != *teamName as libc::c_int {
            Com_Printf(
                b"Failed to load skin file: %s : %s : %s, %s : %s\n\x00" as *const u8
                    as *const libc::c_char,
                teamName,
                modelName,
                skinName,
                headName,
                headSkinName,
            );
            if (*ci).team as libc::c_uint == TEAM_BLUE as libc::c_int as libc::c_uint {
                Com_sprintf(
                    newTeamName.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    b"%s/\x00" as *const u8 as *const libc::c_char,
                    b"Pagans\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                Com_sprintf(
                    newTeamName.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                    b"%s/\x00" as *const u8 as *const libc::c_char,
                    b"Stroggs\x00" as *const u8 as *const libc::c_char,
                );
            }
            if 0 == CG_RegisterClientSkin(
                ci,
                newTeamName.as_mut_ptr(),
                modelName,
                skinName,
                headName,
                headSkinName,
            ) as u64
            {
                Com_Printf(
                    b"Failed to load skin file: %s : %s : %s, %s : %s\n\x00" as *const u8
                        as *const libc::c_char,
                    newTeamName.as_mut_ptr(),
                    modelName,
                    skinName,
                    headName,
                    headSkinName,
                );
                return qfalse;
            }
        } else {
            Com_Printf(
                b"Failed to load skin file: %s : %s, %s : %s\n\x00" as *const u8
                    as *const libc::c_char,
                modelName,
                skinName,
                headName,
                headSkinName,
            );
            return qfalse;
        }
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/players/%s/animation.cfg\x00" as *const u8 as *const libc::c_char,
        modelName,
    );
    if 0 == CG_ParseAnimationFile(filename.as_mut_ptr(), ci) as u64 {
        Com_sprintf(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"models/players/characters/%s/animation.cfg\x00" as *const u8 as *const libc::c_char,
            modelName,
        );
        if 0 == CG_ParseAnimationFile(filename.as_mut_ptr(), ci) as u64 {
            Com_Printf(
                b"Failed to load animation file %s\n\x00" as *const u8 as *const libc::c_char,
                filename.as_mut_ptr(),
            );
            return qfalse;
        }
    }
    if 0 != CG_FindClientHeadFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        ci,
        teamName,
        headName,
        headSkinName,
        b"icon\x00" as *const u8 as *const libc::c_char,
        b"skin\x00" as *const u8 as *const libc::c_char,
    ) as u64
    {
        (*ci).modelIcon = trap_R_RegisterShaderNoMip(filename.as_mut_ptr())
    } else if 0
        != CG_FindClientHeadFile(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            ci,
            teamName,
            headName,
            headSkinName,
            b"icon\x00" as *const u8 as *const libc::c_char,
            b"tga\x00" as *const u8 as *const libc::c_char,
        ) as u64
    {
        (*ci).modelIcon = trap_R_RegisterShaderNoMip(filename.as_mut_ptr())
    }
    if 0 == (*ci).modelIcon {
        return qfalse;
    }
    return qtrue;
}
/*
==========================
CG_FindClientHeadFile
==========================
*/
unsafe extern "C" fn CG_FindClientHeadFile(
    mut filename: *mut libc::c_char,
    mut length: libc::c_int,
    mut ci: *mut clientInfo_t,
    mut teamName: *const libc::c_char,
    mut headModelName: *const libc::c_char,
    mut headSkinName: *const libc::c_char,
    mut base: *const libc::c_char,
    mut ext: *const libc::c_char,
) -> qboolean {
    let mut team: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut headsFolder: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        match (*ci).team as libc::c_uint {
            2 => team = b"blue\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            _ => team = b"red\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        }
    } else {
        team = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if *headModelName.offset(0isize) as libc::c_int == '*' as i32 {
        headsFolder = b"heads/\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        headModelName = headModelName.offset(1isize)
    } else {
        headsFolder = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    loop {
        i = 0i32;
        while i < 2i32 {
            if i == 0i32 && !teamName.is_null() && 0 != *teamName as libc::c_int {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s/%s%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    headsFolder,
                    headModelName,
                    headSkinName,
                    teamName,
                    base,
                    team,
                    ext,
                );
            } else {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s/%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    headsFolder,
                    headModelName,
                    headSkinName,
                    base,
                    team,
                    ext,
                );
            }
            if 0 != CG_FileExists(filename) as u64 {
                return qtrue;
            }
            if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
                if i == 0i32 && !teamName.is_null() && 0 != *teamName as libc::c_int {
                    Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                        headsFolder,
                        headModelName,
                        teamName,
                        base,
                        team,
                        ext,
                    );
                } else {
                    Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                        headsFolder,
                        headModelName,
                        base,
                        team,
                        ext,
                    );
                }
            } else if i == 0i32 && !teamName.is_null() && 0 != *teamName as libc::c_int {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    headsFolder,
                    headModelName,
                    teamName,
                    base,
                    headSkinName,
                    ext,
                );
            } else {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    headsFolder,
                    headModelName,
                    base,
                    headSkinName,
                    ext,
                );
            }
            if 0 != CG_FileExists(filename) as u64 {
                return qtrue;
            }
            if teamName.is_null() || 0 == *teamName {
                break;
            }
            i += 1
        }
        // if tried the heads folder first
        if 0 != *headsFolder.offset(0isize) {
            break;
        }
        headsFolder = b"heads/\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    return qfalse;
}
/*
==========================
CG_FileExists
==========================
*/
unsafe extern "C" fn CG_FileExists(mut filename: *const libc::c_char) -> qboolean {
    let mut len: libc::c_int = 0;
    len = trap_FS_FOpenFile(filename, 0 as *mut fileHandle_t, FS_READ);
    if len > 0i32 {
        return qtrue;
    }
    return qfalse;
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
    mut filename: *const libc::c_char,
    mut ci: *mut clientInfo_t,
) -> qboolean {
    let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prev: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fps: libc::c_float = 0.;
    let mut skip: libc::c_int = 0;
    let mut text: [libc::c_char; 20000] = [0; 20000];
    let mut f: fileHandle_t = 0;
    let mut animations: *mut animation_t = 0 as *mut animation_t;
    animations = (*ci).animations.as_mut_ptr();
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if len <= 0i32 {
        return qfalse;
    }
    if len as libc::c_ulong
        >= (::std::mem::size_of::<[libc::c_char; 20000]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
    {
        CG_Printf(
            b"File %s too long\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        trap_FS_FCloseFile(f);
        return qfalse;
    }
    trap_FS_Read(text.as_mut_ptr() as *mut libc::c_void, len, f);
    text[len as usize] = 0i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    text_p = text.as_mut_ptr();
    skip = 0i32;
    (*ci).footsteps = FOOTSTEP_NORMAL;
    (*ci).headOffset[2usize] = 0i32 as vec_t;
    (*ci).headOffset[1usize] = (*ci).headOffset[2usize];
    (*ci).headOffset[0usize] = (*ci).headOffset[1usize];
    (*ci).gender = GENDER_MALE;
    (*ci).fixedlegs = qfalse;
    (*ci).fixedtorso = qfalse;
    loop {
        prev = text_p;
        token = COM_Parse(&mut text_p);
        if 0 == *token.offset(0isize) {
            break;
        }
        if 0 == Q_stricmp(token, b"footsteps\x00" as *const u8 as *const libc::c_char) {
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            if 0 == Q_stricmp(token, b"default\x00" as *const u8 as *const libc::c_char)
                || 0 == Q_stricmp(token, b"normal\x00" as *const u8 as *const libc::c_char)
            {
                (*ci).footsteps = FOOTSTEP_NORMAL
            } else if 0 == Q_stricmp(token, b"boot\x00" as *const u8 as *const libc::c_char) {
                (*ci).footsteps = FOOTSTEP_BOOT
            } else if 0 == Q_stricmp(token, b"flesh\x00" as *const u8 as *const libc::c_char) {
                (*ci).footsteps = FOOTSTEP_FLESH
            } else if 0 == Q_stricmp(token, b"mech\x00" as *const u8 as *const libc::c_char) {
                (*ci).footsteps = FOOTSTEP_MECH
            } else if 0 == Q_stricmp(token, b"energy\x00" as *const u8 as *const libc::c_char) {
                (*ci).footsteps = FOOTSTEP_ENERGY
            } else {
                CG_Printf(
                    b"Bad footsteps parm in %s: %s\n\x00" as *const u8 as *const libc::c_char,
                    filename,
                    token,
                );
            }
        } else if 0 == Q_stricmp(token, b"headoffset\x00" as *const u8 as *const libc::c_char) {
            i = 0i32;
            while i < 3i32 {
                token = COM_Parse(&mut text_p);
                if 0 == *token.offset(0isize) {
                    break;
                }
                (*ci).headOffset[i as usize] = atof(token) as vec_t;
                i += 1
            }
        } else if 0 == Q_stricmp(token, b"sex\x00" as *const u8 as *const libc::c_char) {
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            if *token.offset(0isize) as libc::c_int == 'f' as i32
                || *token.offset(0isize) as libc::c_int == 'F' as i32
            {
                (*ci).gender = GENDER_FEMALE
            } else if *token.offset(0isize) as libc::c_int == 'n' as i32
                || *token.offset(0isize) as libc::c_int == 'N' as i32
            {
                (*ci).gender = GENDER_NEUTER
            } else {
                (*ci).gender = GENDER_MALE
            }
        } else if 0 == Q_stricmp(token, b"fixedlegs\x00" as *const u8 as *const libc::c_char) {
            (*ci).fixedlegs = qtrue
        } else if 0 == Q_stricmp(token, b"fixedtorso\x00" as *const u8 as *const libc::c_char) {
            (*ci).fixedtorso = qtrue
        } else if *token.offset(0isize) as libc::c_int >= '0' as i32
            && *token.offset(0isize) as libc::c_int <= '9' as i32
        {
            text_p = prev;
            break;
        } else {
            Com_Printf(
                b"unknown token \'%s\' in %s\n\x00" as *const u8 as *const libc::c_char,
                token,
                filename,
            );
        }
    }
    i = 0i32;
    while i < MAX_ANIMATIONS as libc::c_int {
        token = COM_Parse(&mut text_p);
        if 0 == *token.offset(0isize) {
            if !(i >= TORSO_GETFLAG as libc::c_int && i <= TORSO_NEGATIVE as libc::c_int) {
                break;
            }
            (*animations.offset(i as isize)).firstFrame =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).firstFrame;
            (*animations.offset(i as isize)).frameLerp =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).frameLerp;
            (*animations.offset(i as isize)).initialLerp =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).initialLerp;
            (*animations.offset(i as isize)).loopFrames =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).loopFrames;
            (*animations.offset(i as isize)).numFrames =
                (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).numFrames;
            (*animations.offset(i as isize)).reversed = qfalse as libc::c_int;
            (*animations.offset(i as isize)).flipflop = qfalse as libc::c_int
        } else {
            (*animations.offset(i as isize)).firstFrame = atoi(token);
            if i == LEGS_WALKCR as libc::c_int {
                skip = (*animations.offset(LEGS_WALKCR as libc::c_int as isize)).firstFrame
                    - (*animations.offset(TORSO_GESTURE as libc::c_int as isize)).firstFrame
            }
            if i >= LEGS_WALKCR as libc::c_int && i < TORSO_GETFLAG as libc::c_int {
                (*animations.offset(i as isize)).firstFrame -= skip
            }
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            (*animations.offset(i as isize)).numFrames = atoi(token);
            (*animations.offset(i as isize)).reversed = qfalse as libc::c_int;
            (*animations.offset(i as isize)).flipflop = qfalse as libc::c_int;
            if (*animations.offset(i as isize)).numFrames < 0i32 {
                (*animations.offset(i as isize)).numFrames =
                    -(*animations.offset(i as isize)).numFrames;
                (*animations.offset(i as isize)).reversed = qtrue as libc::c_int
            }
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            (*animations.offset(i as isize)).loopFrames = atoi(token);
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                break;
            }
            fps = atof(token) as libc::c_float;
            if fps == 0i32 as libc::c_float {
                fps = 1i32 as libc::c_float
            }
            (*animations.offset(i as isize)).frameLerp =
                (1000i32 as libc::c_float / fps) as libc::c_int;
            (*animations.offset(i as isize)).initialLerp =
                (1000i32 as libc::c_float / fps) as libc::c_int
        }
        i += 1
    }
    if i != MAX_ANIMATIONS as libc::c_int {
        CG_Printf(
            b"Error parsing animation file: %s\n\x00" as *const u8 as *const libc::c_char,
            filename,
        );
        return qfalse;
    }
    memcpy(
        &mut *animations.offset(LEGS_BACKCR as libc::c_int as isize) as *mut animation_t
            as *mut libc::c_void,
        &mut *animations.offset(LEGS_WALKCR as libc::c_int as isize) as *mut animation_t
            as *const libc::c_void,
        ::std::mem::size_of::<animation_t>() as libc::c_ulong,
    );
    (*animations.offset(LEGS_BACKCR as libc::c_int as isize)).reversed = qtrue as libc::c_int;
    memcpy(
        &mut *animations.offset(LEGS_BACKWALK as libc::c_int as isize) as *mut animation_t
            as *mut libc::c_void,
        &mut *animations.offset(LEGS_WALK as libc::c_int as isize) as *mut animation_t
            as *const libc::c_void,
        ::std::mem::size_of::<animation_t>() as libc::c_ulong,
    );
    (*animations.offset(LEGS_BACKWALK as libc::c_int as isize)).reversed = qtrue as libc::c_int;
    (*animations.offset(FLAG_RUN as libc::c_int as isize)).firstFrame = 0i32;
    (*animations.offset(FLAG_RUN as libc::c_int as isize)).numFrames = 16i32;
    (*animations.offset(FLAG_RUN as libc::c_int as isize)).loopFrames = 16i32;
    (*animations.offset(FLAG_RUN as libc::c_int as isize)).frameLerp = 1000i32 / 15i32;
    (*animations.offset(FLAG_RUN as libc::c_int as isize)).initialLerp = 1000i32 / 15i32;
    (*animations.offset(FLAG_RUN as libc::c_int as isize)).reversed = qfalse as libc::c_int;
    (*animations.offset(FLAG_STAND as libc::c_int as isize)).firstFrame = 16i32;
    (*animations.offset(FLAG_STAND as libc::c_int as isize)).numFrames = 5i32;
    (*animations.offset(FLAG_STAND as libc::c_int as isize)).loopFrames = 0i32;
    (*animations.offset(FLAG_STAND as libc::c_int as isize)).frameLerp = 1000i32 / 20i32;
    (*animations.offset(FLAG_STAND as libc::c_int as isize)).initialLerp = 1000i32 / 20i32;
    (*animations.offset(FLAG_STAND as libc::c_int as isize)).reversed = qfalse as libc::c_int;
    (*animations.offset(FLAG_STAND2RUN as libc::c_int as isize)).firstFrame = 16i32;
    (*animations.offset(FLAG_STAND2RUN as libc::c_int as isize)).numFrames = 5i32;
    (*animations.offset(FLAG_STAND2RUN as libc::c_int as isize)).loopFrames = 1i32;
    (*animations.offset(FLAG_STAND2RUN as libc::c_int as isize)).frameLerp = 1000i32 / 15i32;
    (*animations.offset(FLAG_STAND2RUN as libc::c_int as isize)).initialLerp = 1000i32 / 15i32;
    (*animations.offset(FLAG_STAND2RUN as libc::c_int as isize)).reversed = qtrue as libc::c_int;
    return qtrue;
}
/*
==========================
CG_RegisterClientSkin
==========================
*/
unsafe extern "C" fn CG_RegisterClientSkin(
    mut ci: *mut clientInfo_t,
    mut teamName: *const libc::c_char,
    mut modelName: *const libc::c_char,
    mut skinName: *const libc::c_char,
    mut headModelName: *const libc::c_char,
    mut headSkinName: *const libc::c_char,
) -> qboolean {
    let mut filename: [libc::c_char; 64] = [0; 64];
    if 0 != CG_FindClientModelFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        ci,
        teamName,
        modelName,
        skinName,
        b"lower\x00" as *const u8 as *const libc::c_char,
        b"skin\x00" as *const u8 as *const libc::c_char,
    ) as u64
    {
        (*ci).legsSkin = trap_R_RegisterSkin(filename.as_mut_ptr())
    }
    if 0 == (*ci).legsSkin {
        Com_Printf(
            b"Leg skin load failure: %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
    }
    if 0 != CG_FindClientModelFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        ci,
        teamName,
        modelName,
        skinName,
        b"upper\x00" as *const u8 as *const libc::c_char,
        b"skin\x00" as *const u8 as *const libc::c_char,
    ) as u64
    {
        (*ci).torsoSkin = trap_R_RegisterSkin(filename.as_mut_ptr())
    }
    if 0 == (*ci).torsoSkin {
        Com_Printf(
            b"Torso skin load failure: %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
    }
    if 0 != CG_FindClientHeadFile(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        ci,
        teamName,
        headModelName,
        headSkinName,
        b"head\x00" as *const u8 as *const libc::c_char,
        b"skin\x00" as *const u8 as *const libc::c_char,
    ) as u64
    {
        (*ci).headSkin = trap_R_RegisterSkin(filename.as_mut_ptr())
    }
    if 0 == (*ci).headSkin {
        Com_Printf(
            b"Head skin load failure: %s\n\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
    }
    if 0 == (*ci).legsSkin || 0 == (*ci).torsoSkin || 0 == (*ci).headSkin {
        return qfalse;
    }
    return qtrue;
}
/*
==========================
CG_FindClientModelFile
==========================
*/
unsafe extern "C" fn CG_FindClientModelFile(
    mut filename: *mut libc::c_char,
    mut length: libc::c_int,
    mut ci: *mut clientInfo_t,
    mut teamName: *const libc::c_char,
    mut modelName: *const libc::c_char,
    mut skinName: *const libc::c_char,
    mut base: *const libc::c_char,
    mut ext: *const libc::c_char,
) -> qboolean {
    let mut team: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut charactersFolder: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        match (*ci).team as libc::c_uint {
            2 => team = b"blue\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            _ => team = b"red\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        }
    } else {
        team = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    charactersFolder = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    loop {
        i = 0i32;
        while i < 2i32 {
            if i == 0i32 && !teamName.is_null() && 0 != *teamName as libc::c_int {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s%s_%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    charactersFolder,
                    modelName,
                    teamName,
                    base,
                    skinName,
                    team,
                    ext,
                );
            } else {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s_%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    charactersFolder,
                    modelName,
                    base,
                    skinName,
                    team,
                    ext,
                );
            }
            if 0 != CG_FileExists(filename) as u64 {
                return qtrue;
            }
            if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
                if i == 0i32 && !teamName.is_null() && 0 != *teamName as libc::c_int {
                    Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                        charactersFolder,
                        modelName,
                        teamName,
                        base,
                        team,
                        ext,
                    );
                } else {
                    Com_sprintf(
                        filename,
                        length,
                        b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                        charactersFolder,
                        modelName,
                        base,
                        team,
                        ext,
                    );
                }
            } else if i == 0i32 && !teamName.is_null() && 0 != *teamName as libc::c_int {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    charactersFolder,
                    modelName,
                    teamName,
                    base,
                    skinName,
                    ext,
                );
            } else {
                Com_sprintf(
                    filename,
                    length,
                    b"models/players/%s%s/%s_%s.%s\x00" as *const u8 as *const libc::c_char,
                    charactersFolder,
                    modelName,
                    base,
                    skinName,
                    ext,
                );
            }
            if 0 != CG_FileExists(filename) as u64 {
                return qtrue;
            }
            if teamName.is_null() || 0 == *teamName {
                break;
            }
            i += 1
        }
        // if tried the heads folder first
        if 0 != *charactersFolder.offset(0isize) {
            break;
        }
        charactersFolder =
            b"characters/\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    return qfalse;
}
/*
======================
CG_SetDeferredClientInfo

We aren't going to load it now, so grab some other
client's info to use until we have some spare time.
======================
*/
unsafe extern "C" fn CG_SetDeferredClientInfo(
    mut clientNum: libc::c_int,
    mut ci: *mut clientInfo_t,
) {
    let mut i: libc::c_int = 0;
    let mut match_0: *mut clientInfo_t = 0 as *mut clientInfo_t;
    i = 0i32;
    while i < cgs.maxclients {
        match_0 = &mut *cgs.clientinfo.as_mut_ptr().offset(i as isize) as *mut clientInfo_t;
        if !(0 == (*match_0).infoValid as u64 || 0 != (*match_0).deferred as libc::c_uint) {
            if !(0
                != Q_stricmp(
                    (*ci).skinName.as_mut_ptr(),
                    (*match_0).skinName.as_mut_ptr(),
                )
                || 0 != Q_stricmp(
                    (*ci).modelName.as_mut_ptr(),
                    (*match_0).modelName.as_mut_ptr(),
                )
                || cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
                    && (*ci).team as libc::c_uint != (*match_0).team as libc::c_uint)
            {
                //			 Q_stricmp( ci->headModelName, match->headModelName ) ||
                //			 Q_stricmp( ci->headSkinName, match->headSkinName ) ||
                CG_LoadClientInfo(clientNum, ci);
                return;
            }
        }
        i += 1
    }
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        i = 0i32;
        while i < cgs.maxclients {
            match_0 = &mut *cgs.clientinfo.as_mut_ptr().offset(i as isize) as *mut clientInfo_t;
            if !(0 == (*match_0).infoValid as u64 || 0 != (*match_0).deferred as libc::c_uint) {
                if !(0
                    != Q_stricmp(
                        (*ci).skinName.as_mut_ptr(),
                        (*match_0).skinName.as_mut_ptr(),
                    )
                    || cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
                        && (*ci).team as libc::c_uint != (*match_0).team as libc::c_uint)
                {
                    (*ci).deferred = qtrue;
                    CG_CopyClientInfoModel(match_0, ci);
                    return;
                }
            }
            i += 1
        }
        CG_LoadClientInfo(clientNum, ci);
        return;
    }
    i = 0i32;
    while i < cgs.maxclients {
        match_0 = &mut *cgs.clientinfo.as_mut_ptr().offset(i as isize) as *mut clientInfo_t;
        if 0 == (*match_0).infoValid as u64 {
            i += 1
        } else {
            (*ci).deferred = qtrue;
            CG_CopyClientInfoModel(match_0, ci);
            return;
        }
    }
    CG_Printf(
        b"CG_SetDeferredClientInfo: no valid clients!\n\x00" as *const u8 as *const libc::c_char,
    );
    CG_LoadClientInfo(clientNum, ci);
}
/*
======================
CG_CopyClientInfoModel
======================
*/
unsafe extern "C" fn CG_CopyClientInfoModel(
    mut from: *mut clientInfo_t,
    mut to: *mut clientInfo_t,
) {
    (*to).headOffset[0usize] = (*from).headOffset[0usize];
    (*to).headOffset[1usize] = (*from).headOffset[1usize];
    (*to).headOffset[2usize] = (*from).headOffset[2usize];
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
    memcpy(
        (*to).animations.as_mut_ptr() as *mut libc::c_void,
        (*from).animations.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[animation_t; 37]>() as libc::c_ulong,
    );
    memcpy(
        (*to).sounds.as_mut_ptr() as *mut libc::c_void,
        (*from).sounds.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[sfxHandle_t; 32]>() as libc::c_ulong,
    );
}
/*
======================
CG_ScanForExistingClientInfo
======================
*/
unsafe extern "C" fn CG_ScanForExistingClientInfo(mut ci: *mut clientInfo_t) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut match_0: *mut clientInfo_t = 0 as *mut clientInfo_t;
    i = 0i32;
    while i < cgs.maxclients {
        match_0 = &mut *cgs.clientinfo.as_mut_ptr().offset(i as isize) as *mut clientInfo_t;
        if !(0 == (*match_0).infoValid as u64) {
            if !(0 != (*match_0).deferred as u64) {
                if 0 == Q_stricmp(
                    (*ci).modelName.as_mut_ptr(),
                    (*match_0).modelName.as_mut_ptr(),
                ) && 0
                    == Q_stricmp(
                        (*ci).skinName.as_mut_ptr(),
                        (*match_0).skinName.as_mut_ptr(),
                    )
                    && 0 == Q_stricmp(
                        (*ci).headModelName.as_mut_ptr(),
                        (*match_0).headModelName.as_mut_ptr(),
                    )
                    && 0 == Q_stricmp(
                        (*ci).headSkinName.as_mut_ptr(),
                        (*match_0).headSkinName.as_mut_ptr(),
                    )
                    && 0 == Q_stricmp(
                        (*ci).blueTeam.as_mut_ptr(),
                        (*match_0).blueTeam.as_mut_ptr(),
                    )
                    && 0 == Q_stricmp((*ci).redTeam.as_mut_ptr(), (*match_0).redTeam.as_mut_ptr())
                    && ((cgs.gametype as libc::c_uint) < GT_TEAM as libc::c_int as libc::c_uint
                        || (*ci).team as libc::c_uint == (*match_0).team as libc::c_uint)
                {
                    (*ci).deferred = qfalse;
                    CG_CopyClientInfoModel(match_0, ci);
                    return qtrue;
                }
            }
        }
        i += 1
    }
    return qfalse;
}
/*
====================
CG_ColorFromString
====================
*/
unsafe extern "C" fn CG_ColorFromString(mut v: *const libc::c_char, mut color: *mut vec_t) {
    let mut val: libc::c_int = 0;
    let ref mut fresh5 = *color.offset(1isize);
    let ref mut fresh4 = *color.offset(2isize);
    *fresh4 = 0i32 as vec_t;
    *fresh5 = *fresh4;
    *color.offset(0isize) = *fresh5;
    val = atoi(v);
    if val < 1i32 || val > 7i32 {
        *color.offset(0isize) = 1i32 as vec_t;
        *color.offset(1isize) = 1i32 as vec_t;
        *color.offset(2isize) = 1i32 as vec_t;
        return;
    }
    if 0 != val & 1i32 {
        *color.offset(2isize) = 1.0f32
    }
    if 0 != val & 2i32 {
        *color.offset(1isize) = 1.0f32
    }
    if 0 != val & 4i32 {
        *color.offset(0isize) = 1.0f32
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_CustomSound(
    mut clientNum: libc::c_int,
    mut soundName: *const libc::c_char,
) -> sfxHandle_t {
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut i: libc::c_int = 0;
    if *soundName.offset(0isize) as libc::c_int != '*' as i32 {
        return trap_S_RegisterSound(soundName, qfalse);
    }
    if clientNum < 0i32 || clientNum >= 64i32 {
        clientNum = 0i32
    }
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
    i = 0i32;
    while i < 32i32 && !cg_customSoundNames[i as usize].is_null() {
        if 0 == strcmp(soundName, cg_customSoundNames[i as usize]) {
            return (*ci).sounds[i as usize];
        }
        i += 1
    }
    CG_Error(
        b"Unknown custom sound: %s\x00" as *const u8 as *const libc::c_char,
        soundName,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_LoadDeferredPlayers() {
    let mut i: libc::c_int = 0;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    i = 0i32;
    ci = cgs.clientinfo.as_mut_ptr();
    while i < cgs.maxclients {
        if 0 != (*ci).infoValid as libc::c_uint && 0 != (*ci).deferred as libc::c_uint {
            // if we are low on memory, leave it deferred
            if trap_MemoryRemaining() < 4000000i32 {
                CG_Printf(
                    b"Memory is low. Using deferred model.\n\x00" as *const u8
                        as *const libc::c_char,
                );
                (*ci).deferred = qfalse
            } else {
                CG_LoadClientInfo(i, ci);
            }
        }
        i += 1;
        ci = ci.offset(1isize)
    }
}
/*
=================
CG_LightVerts
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_LightVerts(
    mut normal: *mut vec_t,
    mut numVerts: libc::c_int,
    mut verts: *mut polyVert_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut incoming: libc::c_float = 0.;
    let mut ambientLight: vec3_t = [0.; 3];
    let mut lightDir: vec3_t = [0.; 3];
    let mut directedLight: vec3_t = [0.; 3];
    trap_R_LightForPoint(
        (*verts.offset(0isize)).xyz.as_mut_ptr(),
        ambientLight.as_mut_ptr(),
        directedLight.as_mut_ptr(),
        lightDir.as_mut_ptr(),
    );
    i = 0i32;
    while i < numVerts {
        incoming = *normal.offset(0isize) * lightDir[0usize]
            + *normal.offset(1isize) * lightDir[1usize]
            + *normal.offset(2isize) * lightDir[2usize];
        if incoming <= 0i32 as libc::c_float {
            (*verts.offset(i as isize)).modulate[0usize] = ambientLight[0usize] as byte;
            (*verts.offset(i as isize)).modulate[1usize] = ambientLight[1usize] as byte;
            (*verts.offset(i as isize)).modulate[2usize] = ambientLight[2usize] as byte;
            (*verts.offset(i as isize)).modulate[3usize] = 255i32 as byte
        } else {
            j = (ambientLight[0usize] + incoming * directedLight[0usize]) as libc::c_int;
            if j > 255i32 {
                j = 255i32
            }
            (*verts.offset(i as isize)).modulate[0usize] = j as byte;
            j = (ambientLight[1usize] + incoming * directedLight[1usize]) as libc::c_int;
            if j > 255i32 {
                j = 255i32
            }
            (*verts.offset(i as isize)).modulate[1usize] = j as byte;
            j = (ambientLight[2usize] + incoming * directedLight[2usize]) as libc::c_int;
            if j > 255i32 {
                j = 255i32
            }
            (*verts.offset(i as isize)).modulate[2usize] = j as byte;
            (*verts.offset(i as isize)).modulate[3usize] = 255i32 as byte
        }
        i += 1
    }
    return qtrue as libc::c_int;
}
