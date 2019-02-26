#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           libc,
           ptr_wrapping_offset_from)]
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, gitem_s, gitem_t, itemType_t, powerup_t,
    team_t, unnamed_0, unnamed_1, unnamed_2, unnamed_3, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER,
    GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER,
    GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR,
    PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT,
    PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY,
    PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, WEAPON_DROPPING,
    WEAPON_FIRING, WEAPON_RAISING, WEAPON_READY,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
use cg_consolecmds::{CG_ConsoleCommand, CG_InitConsoleCommands};
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
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, itemInfo_t,
    lerpFrame_t, playerEntity_t, score_t, trap_Cvar_VariableStringBuffer, trap_GetCurrentCmdNumber,
    trap_GetUserCmd, trap_Milliseconds, trap_R_AddRefEntityToScene, trap_R_ClearScene,
    trap_R_DrawStretchPic, trap_R_ModelBounds, trap_R_RegisterShader, trap_R_RenderScene,
    trap_R_SetColor, trap_S_StartLocalSound, weaponInfo_s, weaponInfo_t, FOOTSTEP_BOOT,
    FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL,
    FOOTSTEP_SPLASH, FOOTSTEP_TOTAL,
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
    gameState_t, playerState_s, playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t,
    trType_t, trace_t, trajectory_t, unnamed, usercmd_s, usercmd_t, va, vec3_t, vec4_t, vec_t,
    vmCvar_t, Com_sprintf, Info_ValueForKey, Q_PrintStrlen, Q_strncpyz, CHAN_ANNOUNCER, CHAN_AUTO,
    CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atof, cos, memset, rand, sin, strlen, tan};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    stereoFrame_t, textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D,
    GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING,
    RT_MAX_REF_ENTITY_TYPE, RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS,
    RT_SPRITE, STEREO_CENTER, STEREO_LEFT, STEREO_RIGHT, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};
extern crate libc;

//
// cg_draw.c, cg_newDraw.c
//
#[no_mangle]
pub static mut sortedTeamPlayers: [libc::c_int; 32] = [0; 32];
#[no_mangle]
pub static mut numSortedTeamPlayers: libc::c_int = 0;
#[no_mangle]
pub static mut drawTeamOverlayModificationCount: libc::c_int = -1i32;
#[no_mangle]
pub static mut systemChat: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut teamChat1: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut teamChat2: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn CG_AddLagometerFrameInfo() {
    let mut offset: libc::c_int = 0;
    offset = cg.time - cg.latestSnapshotTime;
    lagometer.frameSamples[(lagometer.frameCount & 128i32 - 1i32) as usize] = offset;
    lagometer.frameCount += 1;
}
#[no_mangle]
pub static mut lagometer: lagometer_t = lagometer_t {
    frameSamples: [0; 128],
    frameCount: 0,
    snapshotFlags: [0; 128],
    snapshotSamples: [0; 128],
    snapshotCount: 0,
};
#[no_mangle]
pub unsafe extern "C" fn CG_AddLagometerSnapshotInfo(mut snap: *mut snapshot_t) {
    if snap.is_null() {
        lagometer.snapshotSamples[(lagometer.snapshotCount & 128i32 - 1i32) as usize] = -1i32;
        lagometer.snapshotCount += 1;
        return;
    }
    lagometer.snapshotSamples[(lagometer.snapshotCount & 128i32 - 1i32) as usize] = (*snap).ping;
    lagometer.snapshotFlags[(lagometer.snapshotCount & 128i32 - 1i32) as usize] = (*snap).snapFlags;
    lagometer.snapshotCount += 1;
}
#[no_mangle]
pub unsafe extern "C" fn CG_CenterPrint(
    mut str: *const libc::c_char,
    mut y: libc::c_int,
    mut charWidth: libc::c_int,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    Q_strncpyz(
        cg.centerPrint.as_mut_ptr(),
        str,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    cg.centerPrintTime = cg.time;
    cg.centerPrintY = y;
    cg.centerPrintCharWidth = charWidth;
    cg.centerPrintLines = 1i32;
    s = cg.centerPrint.as_mut_ptr();
    while 0 != *s {
        if *s as libc::c_int == '\n' as i32 {
            cg.centerPrintLines += 1
        }
        s = s.offset(1isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawHead(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut clientNum: libc::c_int,
    mut headAngles: *mut vec_t,
) {
    let mut cm: clipHandle_t = 0;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut len: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset(clientNum as isize) as *mut clientInfo_t;
    if 0 != cg_draw3dIcons.integer {
        cm = (*ci).headModel;
        if 0 == cm {
            return;
        }
        trap_R_ModelBounds(cm, mins.as_mut_ptr(), maxs.as_mut_ptr());
        origin[2usize] = (-0.5f64 * (mins[2usize] + maxs[2usize]) as libc::c_double) as vec_t;
        origin[1usize] = (0.5f64 * (mins[1usize] + maxs[1usize]) as libc::c_double) as vec_t;
        len = (0.7f64 * (maxs[2usize] - mins[2usize]) as libc::c_double) as libc::c_float;
        origin[0usize] = (len as libc::c_double / 0.268f64) as vec_t;
        origin[0usize] = origin[0usize] + (*ci).headOffset[0usize];
        origin[1usize] = origin[1usize] + (*ci).headOffset[1usize];
        origin[2usize] = origin[2usize] + (*ci).headOffset[2usize];
        CG_Draw3DModel(
            x,
            y,
            w,
            h,
            (*ci).headModel,
            (*ci).headSkin,
            origin.as_mut_ptr(),
            headAngles,
        );
    } else if 0 != cg_drawIcons.integer {
        CG_DrawPic(x, y, w, h, (*ci).modelIcon);
    }
    if 0 != (*ci).deferred as u64 {
        CG_DrawPic(x, y, w, h, cgs.media.deferShader);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_Draw3DModel(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut model: qhandle_t,
    mut skin: qhandle_t,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
) {
    let mut refdef: refdef_t = refdef_t {
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
    if 0 == cg_draw3dIcons.integer || 0 == cg_drawIcons.integer {
        return;
    }
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    memset(
        &mut refdef as *mut refdef_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refdef_t>() as libc::c_ulong,
    );
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    AnglesToAxis(angles as *const vec_t, ent.axis.as_mut_ptr());
    ent.origin[0usize] = *origin.offset(0isize);
    ent.origin[1usize] = *origin.offset(1isize);
    ent.origin[2usize] = *origin.offset(2isize);
    ent.hModel = model;
    ent.customSkin = skin;
    ent.renderfx = 0x40i32;
    refdef.rdflags = 0x1i32;
    AxisClear(refdef.viewaxis.as_mut_ptr());
    refdef.fov_x = 30i32 as libc::c_float;
    refdef.fov_y = 30i32 as libc::c_float;
    refdef.x = x as libc::c_int;
    refdef.y = y as libc::c_int;
    refdef.width = w as libc::c_int;
    refdef.height = h as libc::c_int;
    refdef.time = cg.time;
    trap_R_ClearScene();
    trap_R_AddRefEntityToScene(&mut ent);
    trap_R_RenderScene(&mut refdef);
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawActive(mut stereoView: stereoFrame_t) {
    if cg.snap.is_null() {
        CG_DrawInformation();
        return;
    }
    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_SPECTATOR as libc::c_int
        && 0 != (*cg.snap).ps.pm_flags & 8192i32
    {
        CG_DrawTourneyScoreboard();
        return;
    }
    CG_TileClear();
    if stereoView as libc::c_uint != STEREO_CENTER as libc::c_int as libc::c_uint {
        CG_DrawCrosshair3D();
    }
    trap_R_RenderScene(&mut cg.refdef);
    CG_Draw2D(stereoView);
}
//==================================================================================
/*
=================
CG_Draw2D
=================
*/
unsafe extern "C" fn CG_Draw2D(mut stereoFrame: stereoFrame_t) {
    if 0 != cg.levelShot as u64 {
        return;
    }
    if cg_draw2D.integer == 0i32 {
        return;
    }
    if (*cg.snap).ps.pm_type == PM_INTERMISSION as libc::c_int {
        CG_DrawIntermission();
        return;
    }
    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_SPECTATOR as libc::c_int
    {
        CG_DrawSpectator();
        if stereoFrame as libc::c_uint == STEREO_CENTER as libc::c_int as libc::c_uint {
            CG_DrawCrosshair();
        }
        CG_DrawCrosshairNames();
    } else if 0 == cg.showScores as u64
        && (*cg.snap).ps.stats[STAT_HEALTH as libc::c_int as usize] > 0i32
    {
        CG_DrawStatusBar();
        CG_DrawAmmoWarning();
        if stereoFrame as libc::c_uint == STEREO_CENTER as libc::c_int as libc::c_uint {
            CG_DrawCrosshair();
        }
        CG_DrawCrosshairNames();
        CG_DrawWeaponSelect();
        CG_DrawHoldableItem();
        CG_DrawReward();
    }
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        CG_DrawTeamInfo();
    }
    CG_DrawVote();
    CG_DrawTeamVote();
    CG_DrawLagometer();
    CG_DrawUpperRight(stereoFrame);
    CG_DrawLowerRight();
    CG_DrawLowerLeft();
    if 0 == CG_DrawFollow() as u64 {
        CG_DrawWarmup();
    }
    cg.scoreBoardShowing = CG_DrawScoreboard();
    if 0 == cg.scoreBoardShowing as u64 {
        CG_DrawCenterString();
    };
}
/*
===================
CG_DrawCenterString
===================
*/
unsafe extern "C" fn CG_DrawCenterString() {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    if 0 == cg.centerPrintTime {
        return;
    }
    color = CG_FadeColor(
        cg.centerPrintTime,
        (1000i32 as libc::c_float * cg_centertime.value) as libc::c_int,
    );
    if color.is_null() {
        return;
    }
    trap_R_SetColor(color);
    start = cg.centerPrint.as_mut_ptr();
    y = cg.centerPrintY - cg.centerPrintLines * 16i32 / 2i32;
    loop {
        let mut linebuffer: [libc::c_char; 1024] = [0; 1024];
        l = 0i32;
        while l < 50i32 {
            if 0 == *start.offset(l as isize)
                || *start.offset(l as isize) as libc::c_int == '\n' as i32
            {
                break;
            }
            linebuffer[l as usize] = *start.offset(l as isize);
            l += 1
        }
        linebuffer[l as usize] = 0i32 as libc::c_char;
        w = cg.centerPrintCharWidth * CG_DrawStrlen(linebuffer.as_mut_ptr());
        x = (640i32 - w) / 2i32;
        CG_DrawStringExt(
            x,
            y,
            linebuffer.as_mut_ptr(),
            color,
            qfalse,
            qtrue,
            cg.centerPrintCharWidth,
            (cg.centerPrintCharWidth as libc::c_double * 1.5f64) as libc::c_int,
            0i32,
        );
        y = (y as libc::c_double + cg.centerPrintCharWidth as libc::c_double * 1.5f64)
            as libc::c_int;
        while 0 != *start as libc::c_int && *start as libc::c_int != '\n' as i32 {
            start = start.offset(1isize)
        }
        if 0 == *start {
            break;
        }
        start = start.offset(1isize)
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
unsafe extern "C" fn CG_DrawScoreboard() -> qboolean {
    return CG_DrawOldScoreboard();
}
/*
=================
CG_DrawWarmup
=================
*/
unsafe extern "C" fn CG_DrawWarmup() {
    let mut w: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut cw: libc::c_int = 0;
    let mut ci1: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut ci2: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    sec = cg.warmup;
    if 0 == sec {
        return;
    }
    if sec < 0i32 {
        s = b"Waiting for players\x00" as *const u8 as *const libc::c_char;
        w = CG_DrawStrlen(s) * 16i32;
        CG_DrawBigString(320i32 - w / 2i32, 24i32, s, 1.0f32);
        cg.warmupCount = 0i32;
        return;
    }
    if cgs.gametype as libc::c_uint == GT_TOURNAMENT as libc::c_int as libc::c_uint {
        ci1 = 0 as *mut clientInfo_t;
        ci2 = 0 as *mut clientInfo_t;
        i = 0i32;
        while i < cgs.maxclients {
            if 0 != cgs.clientinfo[i as usize].infoValid as libc::c_uint
                && cgs.clientinfo[i as usize].team as libc::c_uint
                    == TEAM_FREE as libc::c_int as libc::c_uint
            {
                if ci1.is_null() {
                    ci1 = &mut *cgs.clientinfo.as_mut_ptr().offset(i as isize) as *mut clientInfo_t
                } else {
                    ci2 = &mut *cgs.clientinfo.as_mut_ptr().offset(i as isize) as *mut clientInfo_t
                }
            }
            i += 1
        }
        if !ci1.is_null() && !ci2.is_null() {
            s = va(
                b"%s vs %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*ci1).name.as_mut_ptr(),
                (*ci2).name.as_mut_ptr(),
            );
            w = CG_DrawStrlen(s);
            if w > 640i32 / 32i32 {
                cw = 640i32 / w
            } else {
                cw = 32i32
            }
            CG_DrawStringExt(
                320i32 - w * cw / 2i32,
                20i32,
                s,
                colorWhite.as_mut_ptr(),
                qfalse,
                qtrue,
                cw,
                (cw as libc::c_float * 1.5f32) as libc::c_int,
                0i32,
            );
        }
    } else {
        if cgs.gametype as libc::c_uint == GT_FFA as libc::c_int as libc::c_uint {
            s = b"Free For All\x00" as *const u8 as *const libc::c_char
        } else if cgs.gametype as libc::c_uint == GT_TEAM as libc::c_int as libc::c_uint {
            s = b"Team Deathmatch\x00" as *const u8 as *const libc::c_char
        } else if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint {
            s = b"Capture the Flag\x00" as *const u8 as *const libc::c_char
        } else {
            s = b"\x00" as *const u8 as *const libc::c_char
        }
        w = CG_DrawStrlen(s);
        if w > 640i32 / 32i32 {
            cw = 640i32 / w
        } else {
            cw = 32i32
        }
        CG_DrawStringExt(
            320i32 - w * cw / 2i32,
            25i32,
            s,
            colorWhite.as_mut_ptr(),
            qfalse,
            qtrue,
            cw,
            (cw as libc::c_float * 1.1f32) as libc::c_int,
            0i32,
        );
    }
    sec = (sec - cg.time) / 1000i32;
    if sec < 0i32 {
        cg.warmup = 0i32;
        sec = 0i32
    }
    s = va(
        b"Starts in: %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sec + 1i32,
    );
    if sec != cg.warmupCount {
        cg.warmupCount = sec;
        match sec {
            0 => {
                trap_S_StartLocalSound(cgs.media.count1Sound, CHAN_ANNOUNCER as libc::c_int);
            }
            1 => {
                trap_S_StartLocalSound(cgs.media.count2Sound, CHAN_ANNOUNCER as libc::c_int);
            }
            2 => {
                trap_S_StartLocalSound(cgs.media.count3Sound, CHAN_ANNOUNCER as libc::c_int);
            }
            _ => {}
        }
    }
    match cg.warmupCount {
        0 => cw = 28i32,
        1 => cw = 24i32,
        2 => cw = 20i32,
        _ => cw = 16i32,
    }
    w = CG_DrawStrlen(s);
    CG_DrawStringExt(
        320i32 - w * cw / 2i32,
        70i32,
        s,
        colorWhite.as_mut_ptr(),
        qfalse,
        qtrue,
        cw,
        (cw as libc::c_double * 1.5f64) as libc::c_int,
        0i32,
    );
}
/*
=================
CG_DrawFollow
=================
*/
unsafe extern "C" fn CG_DrawFollow() -> qboolean {
    let mut x: libc::c_float = 0.;
    let mut color: vec4_t = [0.; 4];
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    if 0 == (*cg.snap).ps.pm_flags & 4096i32 {
        return qfalse;
    }
    color[0usize] = 1i32 as vec_t;
    color[1usize] = 1i32 as vec_t;
    color[2usize] = 1i32 as vec_t;
    color[3usize] = 1i32 as vec_t;
    CG_DrawBigString(
        320i32 - 9i32 * 8i32,
        24i32,
        b"following\x00" as *const u8 as *const libc::c_char,
        1.0f32,
    );
    name = cgs.clientinfo[(*cg.snap).ps.clientNum as usize]
        .name
        .as_mut_ptr();
    x = (0.5f64 * (640i32 - 32i32 * CG_DrawStrlen(name)) as libc::c_double) as libc::c_float;
    CG_DrawStringExt(
        x as libc::c_int,
        40i32,
        name,
        color.as_mut_ptr(),
        qtrue,
        qtrue,
        32i32,
        48i32,
        0i32,
    );
    return qtrue;
}
// MISSIONPACK
/*
=====================
CG_DrawLowerLeft

=====================
*/
unsafe extern "C" fn CG_DrawLowerLeft() {
    let mut y: libc::c_float = 0.;
    y = (480i32 - 48i32) as libc::c_float;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
        && cg_drawTeamOverlay.integer == 3i32
    {
        y = CG_DrawTeamOverlay(y, qfalse, qfalse)
    }
    CG_DrawPickupItem(y as libc::c_int);
}
// MISSIONPACK
/*
===================
CG_DrawPickupItem
===================
*/
unsafe extern "C" fn CG_DrawPickupItem(mut y: libc::c_int) -> libc::c_int {
    let mut value: libc::c_int = 0;
    let mut fadeColor: *mut libc::c_float = 0 as *mut libc::c_float;
    if (*cg.snap).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        return y;
    }
    y -= 48i32;
    value = cg.itemPickup;
    if 0 != value {
        fadeColor = CG_FadeColor(cg.itemPickupTime, 3000i32);
        if !fadeColor.is_null() {
            CG_RegisterItemVisuals(value);
            trap_R_SetColor(fadeColor);
            CG_DrawPic(
                8i32 as libc::c_float,
                y as libc::c_float,
                48i32 as libc::c_float,
                48i32 as libc::c_float,
                cg_items[value as usize].icon,
            );
            CG_DrawBigString(
                48i32 + 16i32,
                y + (48i32 / 2i32 - 16i32 / 2i32),
                (*bg_itemlist.as_mut_ptr().offset(value as isize)).pickup_name,
                *fadeColor.offset(0isize),
            );
            trap_R_SetColor(0 as *const libc::c_float);
        }
    }
    return y;
}
/*
=================
CG_DrawTeamOverlay
=================
*/
unsafe extern "C" fn CG_DrawTeamOverlay(
    mut y: libc::c_float,
    mut right: qboolean,
    mut upper: qboolean,
) -> libc::c_float {
    let mut x: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut xx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut hcolor: vec4_t = [0.; 4];
    let mut pwidth: libc::c_int = 0;
    let mut lwidth: libc::c_int = 0;
    let mut plyrs: libc::c_int = 0;
    let mut st: [libc::c_char; 16] = [0; 16];
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut ret_y: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    if 0 == cg_drawTeamOverlay.integer {
        return y;
    }
    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] != TEAM_RED as libc::c_int
        && (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] != TEAM_BLUE as libc::c_int
    {
        return y;
    }
    plyrs = 0i32;
    pwidth = 0i32;
    count = if numSortedTeamPlayers > 8i32 {
        8i32
    } else {
        numSortedTeamPlayers
    };
    i = 0i32;
    while i < count {
        ci = cgs
            .clientinfo
            .as_mut_ptr()
            .offset(sortedTeamPlayers[i as usize] as isize);
        if 0 != (*ci).infoValid as libc::c_uint
            && (*ci).team as libc::c_uint
                == (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] as libc::c_uint
        {
            plyrs += 1;
            len = CG_DrawStrlen((*ci).name.as_mut_ptr());
            if len > pwidth {
                pwidth = len
            }
        }
        i += 1
    }
    if 0 == plyrs {
        return y;
    }
    if pwidth > 12i32 {
        pwidth = 12i32
    }
    lwidth = 0i32;
    i = 1i32;
    while i < 64i32 {
        p = CG_ConfigString(32i32 + 256i32 + 256i32 + 64i32 + i);
        if !p.is_null() && 0 != *p as libc::c_int {
            len = CG_DrawStrlen(p);
            if len > lwidth {
                lwidth = len
            }
        }
        i += 1
    }
    if lwidth > 16i32 {
        lwidth = 16i32
    }
    w = (pwidth + lwidth + 4i32 + 7i32) * 8i32;
    if 0 != right as u64 {
        x = 640i32 - w
    } else {
        x = 0i32
    }
    h = plyrs * (16i32 / 2i32);
    if 0 != upper as u64 {
        ret_y = (y + h as libc::c_float) as libc::c_int
    } else {
        y -= h as libc::c_float;
        ret_y = y as libc::c_int
    }
    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_RED as libc::c_int {
        hcolor[0usize] = 1.0f32;
        hcolor[1usize] = 0.0f32;
        hcolor[2usize] = 0.0f32;
        hcolor[3usize] = 0.33f32
    } else {
        hcolor[0usize] = 0.0f32;
        hcolor[1usize] = 0.0f32;
        hcolor[2usize] = 1.0f32;
        hcolor[3usize] = 0.33f32
    }
    trap_R_SetColor(hcolor.as_mut_ptr());
    CG_DrawPic(
        x as libc::c_float,
        y,
        w as libc::c_float,
        h as libc::c_float,
        cgs.media.teamStatusBar,
    );
    trap_R_SetColor(0 as *const libc::c_float);
    i = 0i32;
    while i < count {
        ci = cgs
            .clientinfo
            .as_mut_ptr()
            .offset(sortedTeamPlayers[i as usize] as isize);
        if 0 != (*ci).infoValid as libc::c_uint
            && (*ci).team as libc::c_uint
                == (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] as libc::c_uint
        {
            hcolor[3usize] = 1.0f64 as vec_t;
            hcolor[2usize] = hcolor[3usize];
            hcolor[1usize] = hcolor[2usize];
            hcolor[0usize] = hcolor[1usize];
            xx = x + 8i32;
            CG_DrawStringExt(
                xx,
                y as libc::c_int,
                (*ci).name.as_mut_ptr(),
                hcolor.as_mut_ptr(),
                qfalse,
                qfalse,
                8i32,
                16i32 / 2i32,
                12i32,
            );
            if 0 != lwidth {
                p = CG_ConfigString(32i32 + 256i32 + 256i32 + 64i32 + (*ci).location);
                if p.is_null() || 0 == *p {
                    p = b"unknown\x00" as *const u8 as *const libc::c_char
                }
                xx = x + 8i32 * 2i32 + 8i32 * pwidth;
                CG_DrawStringExt(
                    xx,
                    y as libc::c_int,
                    p,
                    hcolor.as_mut_ptr(),
                    qfalse,
                    qfalse,
                    8i32,
                    16i32 / 2i32,
                    16i32,
                );
            }
            CG_GetColorForHealth((*ci).health, (*ci).armor, hcolor.as_mut_ptr());
            Com_sprintf(
                st.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
                b"%3i %3i\x00" as *const u8 as *const libc::c_char,
                (*ci).health,
                (*ci).armor,
            );
            xx = x + 8i32 * 3i32 + 8i32 * pwidth + 8i32 * lwidth;
            CG_DrawStringExt(
                xx,
                y as libc::c_int,
                st.as_mut_ptr(),
                hcolor.as_mut_ptr(),
                qfalse,
                qfalse,
                8i32,
                16i32 / 2i32,
                0i32,
            );
            xx += 8i32 * 3i32;
            if 0 != cg_weapons[(*ci).curWeapon as usize].weaponIcon {
                CG_DrawPic(
                    xx as libc::c_float,
                    y,
                    8i32 as libc::c_float,
                    (16i32 / 2i32) as libc::c_float,
                    cg_weapons[(*ci).curWeapon as usize].weaponIcon,
                );
            } else {
                CG_DrawPic(
                    xx as libc::c_float,
                    y,
                    8i32 as libc::c_float,
                    (16i32 / 2i32) as libc::c_float,
                    cgs.media.deferShader,
                );
            }
            if 0 != right as u64 {
                xx = x
            } else {
                xx = x + w - 8i32
            }
            j = 0i32;
            while j <= PW_NUM_POWERUPS as libc::c_int {
                if 0 != (*ci).powerups & 1i32 << j {
                    item = BG_FindItemForPowerup(j as powerup_t);
                    if !item.is_null() {
                        CG_DrawPic(
                            xx as libc::c_float,
                            y,
                            8i32 as libc::c_float,
                            (16i32 / 2i32) as libc::c_float,
                            trap_R_RegisterShader((*item).icon),
                        );
                        if 0 != right as u64 {
                            xx -= 8i32
                        } else {
                            xx += 8i32
                        }
                    }
                }
                j += 1
            }
            y += (16i32 / 2i32) as libc::c_float
        }
        i += 1
    }
    return ret_y as libc::c_float;
}
// MISSIONPACK
/*
=====================
CG_DrawLowerRight

=====================
*/
unsafe extern "C" fn CG_DrawLowerRight() {
    let mut y: libc::c_float = 0.;
    y = (480i32 - 48i32) as libc::c_float;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
        && cg_drawTeamOverlay.integer == 2i32
    {
        y = CG_DrawTeamOverlay(y, qtrue, qfalse)
    }
    y = CG_DrawScores(y);
    CG_DrawPowerups(y);
}
// MISSIONPACK
/*
================
CG_DrawPowerups
================
*/
unsafe extern "C" fn CG_DrawPowerups(mut y: libc::c_float) -> libc::c_float {
    let mut sorted: [libc::c_int; 16] = [0; 16];
    let mut sortedTime: [libc::c_int; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut active: libc::c_int = 0;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    let mut t: libc::c_int = 0;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut x: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut size: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    static mut colors: [[libc::c_float; 4]; 2] = [
        [0.2f32, 1.0f32, 0.2f32, 1.0f32],
        [1.0f32, 0.2f32, 0.2f32, 1.0f32],
    ];
    ps = &mut (*cg.snap).ps;
    if (*ps).stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        return y;
    }
    active = 0i32;
    i = 0i32;
    while i < 16i32 {
        if !(0 == (*ps).powerups[i as usize]) {
            // ZOID--don't draw if the power up has unlimited time
            // This is true of the CTF flags
            if !((*ps).powerups[i as usize] == 2147483647i32) {
                t = (*ps).powerups[i as usize] - cg.time;
                if !(t <= 0i32) {
                    j = 0i32;
                    while j < active {
                        if sortedTime[j as usize] >= t {
                            k = active - 1i32;
                            while k >= j {
                                sorted[(k + 1i32) as usize] = sorted[k as usize];
                                sortedTime[(k + 1i32) as usize] = sortedTime[k as usize];
                                k -= 1
                            }
                            break;
                        } else {
                            j += 1
                        }
                    }
                    sorted[j as usize] = i;
                    sortedTime[j as usize] = t;
                    active += 1
                }
            }
        }
        i += 1
    }
    x = 640i32 - 48i32 - 32i32 * 2i32;
    i = 0i32;
    while i < active {
        item = BG_FindItemForPowerup(sorted[i as usize] as powerup_t);
        if !item.is_null() {
            color = 1i32;
            y -= 48i32 as libc::c_float;
            trap_R_SetColor(colors[color as usize].as_mut_ptr());
            CG_DrawField(x, y as libc::c_int, 2i32, sortedTime[i as usize] / 1000i32);
            t = (*ps).powerups[sorted[i as usize] as usize];
            if t - cg.time >= 5i32 * 1000i32 {
                trap_R_SetColor(0 as *const libc::c_float);
            } else {
                let mut modulate: vec4_t = [0.; 4];
                f = (t - cg.time) as libc::c_float / 1000i32 as libc::c_float;
                f -= f as libc::c_int as libc::c_float;
                modulate[3usize] = f;
                modulate[2usize] = modulate[3usize];
                modulate[1usize] = modulate[2usize];
                modulate[0usize] = modulate[1usize];
                trap_R_SetColor(modulate.as_mut_ptr());
            }
            if cg.powerupActive == sorted[i as usize] && cg.time - cg.powerupTime < 200i32 {
                f = (1.0f64
                    - ((cg.time as libc::c_float - cg.powerupTime as libc::c_float)
                        / 200i32 as libc::c_float) as libc::c_double)
                    as libc::c_float;
                size = (48i32 as libc::c_double
                    * (1.0f64 + (1.5f64 - 1.0f64) * f as libc::c_double))
                    as libc::c_float
            } else {
                size = 48i32 as libc::c_float
            }
            CG_DrawPic(
                640i32 as libc::c_float - size,
                y + (48i32 / 2i32) as libc::c_float - size / 2i32 as libc::c_float,
                size,
                size,
                trap_R_RegisterShader((*item).icon),
            );
        }
        i += 1
    }
    trap_R_SetColor(0 as *const libc::c_float);
    return y;
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
// cg_draw.c -- draw all of the graphical elements during
// active (after loading) gameplay
/*
==============
CG_DrawField

Draws large numbers for status bar and powerups
==============
*/
unsafe extern "C" fn CG_DrawField(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut value: libc::c_int,
) {
    let mut num: [libc::c_char; 16] = [0; 16];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut frame: libc::c_int = 0;
    if width < 1i32 {
        return;
    }
    if width > 5i32 {
        width = 5i32
    }
    match width {
        1 => {
            value = if value > 9i32 { 9i32 } else { value };
            value = if value < 0i32 { 0i32 } else { value }
        }
        2 => {
            value = if value > 99i32 { 99i32 } else { value };
            value = if value < -9i32 { -9i32 } else { value }
        }
        3 => {
            value = if value > 999i32 { 999i32 } else { value };
            value = if value < -99i32 { -99i32 } else { value }
        }
        4 => {
            value = if value > 9999i32 { 9999i32 } else { value };
            value = if value < -999i32 { -999i32 } else { value }
        }
        _ => {}
    }
    Com_sprintf(
        num.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"%i\x00" as *const u8 as *const libc::c_char,
        value,
    );
    l = strlen(num.as_mut_ptr()) as libc::c_int;
    if l > width {
        l = width
    }
    x += 2i32 + 32i32 * (width - l);
    ptr = num.as_mut_ptr();
    while 0 != *ptr as libc::c_int && 0 != l {
        if *ptr as libc::c_int == '-' as i32 {
            frame = 10i32
        } else {
            frame = *ptr as libc::c_int - '0' as i32
        }
        CG_DrawPic(
            x as libc::c_float,
            y as libc::c_float,
            32i32 as libc::c_float,
            48i32 as libc::c_float,
            cgs.media.numberShaders[frame as usize],
        );
        x += 32i32;
        ptr = ptr.offset(1isize);
        l -= 1
    }
}
/*
===========================================================================================

  LOWER RIGHT CORNER

===========================================================================================
*/
/*
=================
CG_DrawScores

Draw the small two score display
=================
*/
unsafe extern "C" fn CG_DrawScores(mut y: libc::c_float) -> libc::c_float {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut color: vec4_t = [0.; 4];
    let mut y1: libc::c_float = 0.;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    s1 = cgs.scores1;
    s2 = cgs.scores2;
    y -= (16i32 + 8i32) as libc::c_float;
    y1 = y;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        x = 640i32;
        color[0usize] = 0.0f32;
        color[1usize] = 0.0f32;
        color[2usize] = 1.0f32;
        color[3usize] = 0.33f32;
        s = va(
            b"%2i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s2,
        );
        w = CG_DrawStrlen(s) * 16i32 + 8i32;
        x -= w;
        CG_FillRect(
            x as libc::c_float,
            y - 4i32 as libc::c_float,
            w as libc::c_float,
            (16i32 + 8i32) as libc::c_float,
            color.as_mut_ptr(),
        );
        if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_BLUE as libc::c_int {
            CG_DrawPic(
                x as libc::c_float,
                y - 4i32 as libc::c_float,
                w as libc::c_float,
                (16i32 + 8i32) as libc::c_float,
                cgs.media.selectShader,
            );
        }
        CG_DrawBigString(x + 4i32, y as libc::c_int, s, 1.0f32);
        if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint {
            item = BG_FindItemForPowerup(PW_BLUEFLAG);
            if !item.is_null() {
                y1 = y - 16i32 as libc::c_float - 8i32 as libc::c_float;
                if cgs.blueflag >= 0i32 && cgs.blueflag <= 2i32 {
                    CG_DrawPic(
                        x as libc::c_float,
                        y1 - 4i32 as libc::c_float,
                        w as libc::c_float,
                        (16i32 + 8i32) as libc::c_float,
                        cgs.media.blueFlagShader[cgs.blueflag as usize],
                    );
                }
            }
        }
        color[0usize] = 1.0f32;
        color[1usize] = 0.0f32;
        color[2usize] = 0.0f32;
        color[3usize] = 0.33f32;
        s = va(
            b"%2i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s1,
        );
        w = CG_DrawStrlen(s) * 16i32 + 8i32;
        x -= w;
        CG_FillRect(
            x as libc::c_float,
            y - 4i32 as libc::c_float,
            w as libc::c_float,
            (16i32 + 8i32) as libc::c_float,
            color.as_mut_ptr(),
        );
        if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_RED as libc::c_int {
            CG_DrawPic(
                x as libc::c_float,
                y - 4i32 as libc::c_float,
                w as libc::c_float,
                (16i32 + 8i32) as libc::c_float,
                cgs.media.selectShader,
            );
        }
        CG_DrawBigString(x + 4i32, y as libc::c_int, s, 1.0f32);
        if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint {
            item = BG_FindItemForPowerup(PW_REDFLAG);
            if !item.is_null() {
                y1 = y - 16i32 as libc::c_float - 8i32 as libc::c_float;
                if cgs.redflag >= 0i32 && cgs.redflag <= 2i32 {
                    CG_DrawPic(
                        x as libc::c_float,
                        y1 - 4i32 as libc::c_float,
                        w as libc::c_float,
                        (16i32 + 8i32) as libc::c_float,
                        cgs.media.redFlagShader[cgs.redflag as usize],
                    );
                }
            }
        }
        if cgs.gametype as libc::c_uint >= GT_CTF as libc::c_int as libc::c_uint {
            v = cgs.capturelimit
        } else {
            v = cgs.fraglimit
        }
        if 0 != v {
            s = va(
                b"%2i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                v,
            );
            w = CG_DrawStrlen(s) * 16i32 + 8i32;
            x -= w;
            CG_DrawBigString(x + 4i32, y as libc::c_int, s, 1.0f32);
        }
    } else {
        let mut spectator: qboolean = qfalse;
        x = 640i32;
        score = (*cg.snap).ps.persistant[PERS_SCORE as libc::c_int as usize];
        spectator = ((*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
            == TEAM_SPECTATOR as libc::c_int) as libc::c_int as qboolean;
        if s1 != score {
            s2 = score
        }
        if s2 != -9999i32 {
            s = va(
                b"%2i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                s2,
            );
            w = CG_DrawStrlen(s) * 16i32 + 8i32;
            x -= w;
            if 0 == spectator as u64 && score == s2 && score != s1 {
                color[0usize] = 1.0f32;
                color[1usize] = 0.0f32;
                color[2usize] = 0.0f32;
                color[3usize] = 0.33f32;
                CG_FillRect(
                    x as libc::c_float,
                    y - 4i32 as libc::c_float,
                    w as libc::c_float,
                    (16i32 + 8i32) as libc::c_float,
                    color.as_mut_ptr(),
                );
                CG_DrawPic(
                    x as libc::c_float,
                    y - 4i32 as libc::c_float,
                    w as libc::c_float,
                    (16i32 + 8i32) as libc::c_float,
                    cgs.media.selectShader,
                );
            } else {
                color[0usize] = 0.5f32;
                color[1usize] = 0.5f32;
                color[2usize] = 0.5f32;
                color[3usize] = 0.33f32;
                CG_FillRect(
                    x as libc::c_float,
                    y - 4i32 as libc::c_float,
                    w as libc::c_float,
                    (16i32 + 8i32) as libc::c_float,
                    color.as_mut_ptr(),
                );
            }
            CG_DrawBigString(x + 4i32, y as libc::c_int, s, 1.0f32);
        }
        if s1 != -9999i32 {
            s = va(
                b"%2i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                s1,
            );
            w = CG_DrawStrlen(s) * 16i32 + 8i32;
            x -= w;
            if 0 == spectator as u64 && score == s1 {
                color[0usize] = 0.0f32;
                color[1usize] = 0.0f32;
                color[2usize] = 1.0f32;
                color[3usize] = 0.33f32;
                CG_FillRect(
                    x as libc::c_float,
                    y - 4i32 as libc::c_float,
                    w as libc::c_float,
                    (16i32 + 8i32) as libc::c_float,
                    color.as_mut_ptr(),
                );
                CG_DrawPic(
                    x as libc::c_float,
                    y - 4i32 as libc::c_float,
                    w as libc::c_float,
                    (16i32 + 8i32) as libc::c_float,
                    cgs.media.selectShader,
                );
            } else {
                color[0usize] = 0.5f32;
                color[1usize] = 0.5f32;
                color[2usize] = 0.5f32;
                color[3usize] = 0.33f32;
                CG_FillRect(
                    x as libc::c_float,
                    y - 4i32 as libc::c_float,
                    w as libc::c_float,
                    (16i32 + 8i32) as libc::c_float,
                    color.as_mut_ptr(),
                );
            }
            CG_DrawBigString(x + 4i32, y as libc::c_int, s, 1.0f32);
        }
        if 0 != cgs.fraglimit {
            s = va(
                b"%2i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cgs.fraglimit,
            );
            w = CG_DrawStrlen(s) * 16i32 + 8i32;
            x -= w;
            CG_DrawBigString(x + 4i32, y as libc::c_int, s, 1.0f32);
        }
    }
    return y1 - 8i32 as libc::c_float;
}
//#endif
/*
=====================
CG_DrawUpperRight

=====================
*/
unsafe extern "C" fn CG_DrawUpperRight(mut stereoFrame: stereoFrame_t) {
    let mut y: libc::c_float = 0.;
    y = 0i32 as libc::c_float;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
        && cg_drawTeamOverlay.integer == 1i32
    {
        y = CG_DrawTeamOverlay(y, qtrue, qtrue)
    }
    if 0 != cg_drawSnapshot.integer {
        y = CG_DrawSnapshot(y)
    }
    if 0 != cg_drawFPS.integer
        && (stereoFrame as libc::c_uint == STEREO_CENTER as libc::c_int as libc::c_uint
            || stereoFrame as libc::c_uint == STEREO_RIGHT as libc::c_int as libc::c_uint)
    {
        y = CG_DrawFPS(y)
    }
    if 0 != cg_drawTimer.integer {
        y = CG_DrawTimer(y)
    }
    if 0 != cg_drawAttacker.integer {
        CG_DrawAttacker(y);
    };
}
/*
===========================================================================================

  UPPER RIGHT CORNER

===========================================================================================
*/
/*
================
CG_DrawAttacker

================
*/
unsafe extern "C" fn CG_DrawAttacker(mut y: libc::c_float) -> libc::c_float {
    let mut t: libc::c_int = 0;
    let mut size: libc::c_float = 0.;
    let mut angles: vec3_t = [0.; 3];
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut clientNum: libc::c_int = 0;
    if cg.predictedPlayerState.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        return y;
    }
    if 0 == cg.attackerTime {
        return y;
    }
    clientNum = cg.predictedPlayerState.persistant[PERS_ATTACKER as libc::c_int as usize];
    if clientNum < 0i32 || clientNum >= 64i32 || clientNum == (*cg.snap).ps.clientNum {
        return y;
    }
    if 0 == cgs.clientinfo[clientNum as usize].infoValid as u64 {
        cg.attackerTime = 0i32;
        return y;
    }
    t = cg.time - cg.attackerTime;
    if t > 10000i32 {
        cg.attackerTime = 0i32;
        return y;
    }
    size = (48i32 as libc::c_double * 1.25f64) as libc::c_float;
    angles[0usize] = 0i32 as vec_t;
    angles[1usize] = 180i32 as vec_t;
    angles[2usize] = 0i32 as vec_t;
    CG_DrawHead(
        640i32 as libc::c_float - size,
        y,
        size,
        size,
        clientNum,
        angles.as_mut_ptr(),
    );
    info = CG_ConfigString(32i32 + 256i32 + 256i32 + clientNum);
    name = Info_ValueForKey(info, b"n\x00" as *const u8 as *const libc::c_char);
    y += size;
    CG_DrawBigString(
        640i32 - Q_PrintStrlen(name) * 16i32,
        y as libc::c_int,
        name,
        0.5f64 as libc::c_float,
    );
    return y + 16i32 as libc::c_float + 2i32 as libc::c_float;
}
/*
=================
CG_DrawTimer
=================
*/
unsafe extern "C" fn CG_DrawTimer(mut y: libc::c_float) -> libc::c_float {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: libc::c_int = 0;
    let mut mins: libc::c_int = 0;
    let mut seconds: libc::c_int = 0;
    let mut tens: libc::c_int = 0;
    let mut msec: libc::c_int = 0;
    msec = cg.time - cgs.levelStartTime;
    seconds = msec / 1000i32;
    mins = seconds / 60i32;
    seconds -= mins * 60i32;
    tens = seconds / 10i32;
    seconds -= tens * 10i32;
    s = va(
        b"%i:%i%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        mins,
        tens,
        seconds,
    );
    w = CG_DrawStrlen(s) * 16i32;
    CG_DrawBigString(
        635i32 - w,
        (y + 2i32 as libc::c_float) as libc::c_int,
        s,
        1.0f32,
    );
    return y + 16i32 as libc::c_float + 4i32 as libc::c_float;
}
/*
==================
CG_DrawFPS
==================
*/
unsafe extern "C" fn CG_DrawFPS(mut y: libc::c_float) -> libc::c_float {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: libc::c_int = 0;
    static mut previousTimes: [libc::c_int; 4] = [0; 4];
    static mut index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut fps: libc::c_int = 0;
    static mut previous: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut frameTime: libc::c_int = 0;
    t = trap_Milliseconds();
    frameTime = t - previous;
    previous = t;
    previousTimes[(index % 4i32) as usize] = frameTime;
    index += 1;
    if index > 4i32 {
        total = 0i32;
        i = 0i32;
        while i < 4i32 {
            total += previousTimes[i as usize];
            i += 1
        }
        if 0 == total {
            total = 1i32
        }
        fps = 1000i32 * 4i32 / total;
        s = va(
            b"%ifps\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            fps,
        );
        w = CG_DrawStrlen(s) * 16i32;
        CG_DrawBigString(
            635i32 - w,
            (y + 2i32 as libc::c_float) as libc::c_int,
            s,
            1.0f32,
        );
    }
    return y + 16i32 as libc::c_float + 4i32 as libc::c_float;
}
/*
==================
CG_DrawSnapshot
==================
*/
unsafe extern "C" fn CG_DrawSnapshot(mut y: libc::c_float) -> libc::c_float {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: libc::c_int = 0;
    s = va(
        b"time:%i snap:%i cmd:%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*cg.snap).serverTime,
        cg.latestSnapshotNum,
        cgs.serverCommandSequence,
    );
    w = CG_DrawStrlen(s) * 16i32;
    CG_DrawBigString(
        635i32 - w,
        (y + 2i32 as libc::c_float) as libc::c_int,
        s,
        1.0f32,
    );
    return y + 16i32 as libc::c_float + 4i32 as libc::c_float;
}
/*
==============
CG_DrawLagometer
==============
*/
unsafe extern "C" fn CG_DrawLagometer() {
    let mut a: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_float = 0.;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    let mut mid: libc::c_float = 0.;
    let mut range: libc::c_float = 0.;
    let mut color: libc::c_int = 0;
    let mut vscale: libc::c_float = 0.;
    if 0 == cg_lagometer.integer || 0 != cgs.localServer as libc::c_uint {
        CG_DrawDisconnect();
        return;
    }
    x = 640i32 - 48i32;
    y = 480i32 - 48i32;
    trap_R_SetColor(0 as *const libc::c_float);
    CG_DrawPic(
        x as libc::c_float,
        y as libc::c_float,
        48i32 as libc::c_float,
        48i32 as libc::c_float,
        cgs.media.lagometerShader,
    );
    ax = x as libc::c_float;
    ay = y as libc::c_float;
    aw = 48i32 as libc::c_float;
    ah = 48i32 as libc::c_float;
    CG_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    color = -1i32;
    range = ah / 3i32 as libc::c_float;
    mid = ay + range;
    vscale = range / 300i32 as libc::c_float;
    a = 0i32;
    while (a as libc::c_float) < aw {
        i = lagometer.frameCount - 1i32 - a & 128i32 - 1i32;
        v = lagometer.frameSamples[i as usize] as libc::c_float;
        v *= vscale;
        if v > 0i32 as libc::c_float {
            if color != 1i32 {
                color = 1i32;
                trap_R_SetColor(
                    g_color_table[('3' as i32 - '0' as i32 & 0x7i32) as usize].as_mut_ptr(),
                );
            }
            if v > range {
                v = range
            }
            trap_R_DrawStretchPic(
                ax + aw - a as libc::c_float,
                mid - v,
                1i32 as libc::c_float,
                v,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                cgs.media.whiteShader,
            );
        } else if v < 0i32 as libc::c_float {
            if color != 2i32 {
                color = 2i32;
                trap_R_SetColor(
                    g_color_table[('4' as i32 - '0' as i32 & 0x7i32) as usize].as_mut_ptr(),
                );
            }
            v = -v;
            if v > range {
                v = range
            }
            trap_R_DrawStretchPic(
                ax + aw - a as libc::c_float,
                mid,
                1i32 as libc::c_float,
                v,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                cgs.media.whiteShader,
            );
        }
        a += 1
    }
    range = ah / 2i32 as libc::c_float;
    vscale = range / 900i32 as libc::c_float;
    a = 0i32;
    while (a as libc::c_float) < aw {
        i = lagometer.snapshotCount - 1i32 - a & 128i32 - 1i32;
        v = lagometer.snapshotSamples[i as usize] as libc::c_float;
        if v > 0i32 as libc::c_float {
            if 0 != lagometer.snapshotFlags[i as usize] & 1i32 {
                if color != 5i32 {
                    color = 5i32;
                    trap_R_SetColor(
                        g_color_table[('3' as i32 - '0' as i32 & 0x7i32) as usize].as_mut_ptr(),
                    );
                }
            } else if color != 3i32 {
                color = 3i32;
                trap_R_SetColor(
                    g_color_table[('2' as i32 - '0' as i32 & 0x7i32) as usize].as_mut_ptr(),
                );
            }
            v = v * vscale;
            if v > range {
                v = range
            }
            trap_R_DrawStretchPic(
                ax + aw - a as libc::c_float,
                ay + ah - v,
                1i32 as libc::c_float,
                v,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                cgs.media.whiteShader,
            );
        } else if v < 0i32 as libc::c_float {
            if color != 4i32 {
                color = 4i32;
                trap_R_SetColor(
                    g_color_table[('1' as i32 - '0' as i32 & 0x7i32) as usize].as_mut_ptr(),
                );
            }
            trap_R_DrawStretchPic(
                ax + aw - a as libc::c_float,
                ay + ah - range,
                1i32 as libc::c_float,
                range,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                0i32 as libc::c_float,
                cgs.media.whiteShader,
            );
        }
        a += 1
    }
    trap_R_SetColor(0 as *const libc::c_float);
    if 0 != cg_nopredict.integer || 0 != cg_synchronousClients.integer {
        CG_DrawBigString(
            x,
            y,
            b"snc\x00" as *const u8 as *const libc::c_char,
            1.0f64 as libc::c_float,
        );
    }
    CG_DrawDisconnect();
}
/*
==============
CG_DrawDisconnect

Should we draw something differnet for long lag vs no packets?
==============
*/
unsafe extern "C" fn CG_DrawDisconnect() {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut cmdNum: libc::c_int = 0;
    let mut cmd: usercmd_t = usercmd_s {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    };
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut w: libc::c_int = 0;
    cmdNum = trap_GetCurrentCmdNumber() - 64i32 + 1i32;
    trap_GetUserCmd(cmdNum, &mut cmd);
    if cmd.serverTime <= (*cg.snap).ps.commandTime || cmd.serverTime > cg.time {
        return;
    }
    s = b"Connection Interrupted\x00" as *const u8 as *const libc::c_char;
    w = CG_DrawStrlen(s) * 16i32;
    CG_DrawBigString(320i32 - w / 2i32, 100i32, s, 1.0f32);
    if 0 != cg.time >> 9i32 & 1i32 {
        return;
    }
    x = (640i32 - 48i32) as libc::c_float;
    y = (480i32 - 48i32) as libc::c_float;
    CG_DrawPic(
        x,
        y,
        48i32 as libc::c_float,
        48i32 as libc::c_float,
        trap_R_RegisterShader(b"gfx/2d/net.tga\x00" as *const u8 as *const libc::c_char),
    );
}
/*
=================
CG_DrawTeamVote
=================
*/
unsafe extern "C" fn CG_DrawTeamVote() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sec: libc::c_int = 0;
    let mut cs_offset: libc::c_int = 0;
    if cgs.clientinfo[cg.clientNum as usize].team as libc::c_uint
        == TEAM_RED as libc::c_int as libc::c_uint
    {
        cs_offset = 0i32
    } else if cgs.clientinfo[cg.clientNum as usize].team as libc::c_uint
        == TEAM_BLUE as libc::c_int as libc::c_uint
    {
        cs_offset = 1i32
    } else {
        return;
    }
    if 0 == cgs.teamVoteTime[cs_offset as usize] {
        return;
    }
    if 0 != cgs.teamVoteModified[cs_offset as usize] as u64 {
        cgs.teamVoteModified[cs_offset as usize] = qfalse;
        trap_S_StartLocalSound(cgs.media.talkSound, CHAN_LOCAL_SOUND as libc::c_int);
    }
    sec = (30000i32 - (cg.time - cgs.teamVoteTime[cs_offset as usize])) / 1000i32;
    if sec < 0i32 {
        sec = 0i32
    }
    s = va(
        b"TEAMVOTE(%i):%s yes:%i no:%i\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        sec,
        cgs.teamVoteString[cs_offset as usize].as_mut_ptr(),
        cgs.teamVoteYes[cs_offset as usize],
        cgs.teamVoteNo[cs_offset as usize],
    );
    CG_DrawSmallString(0i32, 90i32, s, 1.0f32);
}
/*
=================
CG_DrawVote
=================
*/
unsafe extern "C" fn CG_DrawVote() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sec: libc::c_int = 0;
    if 0 == cgs.voteTime {
        return;
    }
    if 0 != cgs.voteModified as u64 {
        cgs.voteModified = qfalse;
        trap_S_StartLocalSound(cgs.media.talkSound, CHAN_LOCAL_SOUND as libc::c_int);
    }
    sec = (30000i32 - (cg.time - cgs.voteTime)) / 1000i32;
    if sec < 0i32 {
        sec = 0i32
    }
    s = va(
        b"VOTE(%i):%s yes:%i no:%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sec,
        cgs.voteString.as_mut_ptr(),
        cgs.voteYes,
        cgs.voteNo,
    );
    CG_DrawSmallString(0i32, 58i32, s, 1.0f32);
}
// MISSIONPACK
//===========================================================================================
/*
=================
CG_DrawTeamInfo
=================
*/
unsafe extern "C" fn CG_DrawTeamInfo() {
    let mut h: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut hcolor: vec4_t = [0.; 4];
    let mut chatHeight: libc::c_int = 0;
    if cg_teamChatHeight.integer < 8i32 {
        chatHeight = cg_teamChatHeight.integer
    } else {
        chatHeight = 8i32
    }
    if chatHeight <= 0i32 {
        return;
    }
    if cgs.teamLastChatPos != cgs.teamChatPos {
        if cg.time - cgs.teamChatMsgTimes[(cgs.teamLastChatPos % chatHeight) as usize]
            > cg_teamChatTime.integer
        {
            cgs.teamLastChatPos += 1
        }
        h = (cgs.teamChatPos - cgs.teamLastChatPos) * (16i32 / 2i32);
        if cgs.clientinfo[cg.clientNum as usize].team as libc::c_uint
            == TEAM_RED as libc::c_int as libc::c_uint
        {
            hcolor[0usize] = 1.0f32;
            hcolor[1usize] = 0.0f32;
            hcolor[2usize] = 0.0f32;
            hcolor[3usize] = 0.33f32
        } else if cgs.clientinfo[cg.clientNum as usize].team as libc::c_uint
            == TEAM_BLUE as libc::c_int as libc::c_uint
        {
            hcolor[0usize] = 0.0f32;
            hcolor[1usize] = 0.0f32;
            hcolor[2usize] = 1.0f32;
            hcolor[3usize] = 0.33f32
        } else {
            hcolor[0usize] = 0.0f32;
            hcolor[1usize] = 1.0f32;
            hcolor[2usize] = 0.0f32;
            hcolor[3usize] = 0.33f32
        }
        trap_R_SetColor(hcolor.as_mut_ptr());
        CG_DrawPic(
            0i32 as libc::c_float,
            (420i32 - h) as libc::c_float,
            640i32 as libc::c_float,
            h as libc::c_float,
            cgs.media.teamStatusBar,
        );
        trap_R_SetColor(0 as *const libc::c_float);
        hcolor[2usize] = 1.0f32;
        hcolor[1usize] = hcolor[2usize];
        hcolor[0usize] = hcolor[1usize];
        hcolor[3usize] = 1.0f32;
        i = cgs.teamChatPos - 1i32;
        while i >= cgs.teamLastChatPos {
            CG_DrawStringExt(
                0i32 + 8i32,
                420i32 - (cgs.teamChatPos - i) * (16i32 / 2i32),
                cgs.teamChatMsgs[(i % chatHeight) as usize].as_mut_ptr(),
                hcolor.as_mut_ptr(),
                qfalse,
                qfalse,
                8i32,
                16i32 / 2i32,
                0i32,
            );
            i -= 1
        }
    };
}
// MISSIONPACK
// MISSIONPACK
/*
===================
CG_DrawReward
===================
*/
unsafe extern "C" fn CG_DrawReward() {
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut buf: [libc::c_char; 32] = [0; 32];
    if 0 == cg_drawRewards.integer {
        return;
    }
    color = CG_FadeColor(cg.rewardTime, 3000i32);
    if color.is_null() {
        if cg.rewardStack > 0i32 {
            i = 0i32;
            while i < cg.rewardStack {
                cg.rewardSound[i as usize] = cg.rewardSound[(i + 1i32) as usize];
                cg.rewardShader[i as usize] = cg.rewardShader[(i + 1i32) as usize];
                cg.rewardCount[i as usize] = cg.rewardCount[(i + 1i32) as usize];
                i += 1
            }
            cg.rewardTime = cg.time;
            cg.rewardStack -= 1;
            color = CG_FadeColor(cg.rewardTime, 3000i32);
            trap_S_StartLocalSound(cg.rewardSound[0usize], CHAN_ANNOUNCER as libc::c_int);
        } else {
            return;
        }
    }
    trap_R_SetColor(color);
    if cg.rewardCount[0usize] >= 10i32 {
        y = 56i32 as libc::c_float;
        x = (320i32 - 48i32 / 2i32) as libc::c_float;
        CG_DrawPic(
            x,
            y,
            (48i32 - 4i32) as libc::c_float,
            (48i32 - 4i32) as libc::c_float,
            cg.rewardShader[0usize],
        );
        Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%d\x00" as *const u8 as *const libc::c_char,
            cg.rewardCount[0usize],
        );
        x = ((640i32 - 8i32 * CG_DrawStrlen(buf.as_mut_ptr())) / 2i32) as libc::c_float;
        CG_DrawStringExt(
            x as libc::c_int,
            (y + 48i32 as libc::c_float) as libc::c_int,
            buf.as_mut_ptr(),
            color,
            qfalse,
            qtrue,
            8i32,
            16i32,
            0i32,
        );
    } else {
        count = cg.rewardCount[0usize];
        y = 56i32 as libc::c_float;
        x = (320i32 - count * 48i32 / 2i32) as libc::c_float;
        i = 0i32;
        while i < count {
            CG_DrawPic(
                x,
                y,
                (48i32 - 4i32) as libc::c_float,
                (48i32 - 4i32) as libc::c_float,
                cg.rewardShader[0usize],
            );
            x += 48i32 as libc::c_float;
            i += 1
        }
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
// MISSIONPACK
/*
===================
CG_DrawHoldableItem
===================
*/
unsafe extern "C" fn CG_DrawHoldableItem() {
    let mut value: libc::c_int = 0;
    value = (*cg.snap).ps.stats[STAT_HOLDABLE_ITEM as libc::c_int as usize];
    if 0 != value {
        CG_RegisterItemVisuals(value);
        CG_DrawPic(
            (640i32 - 48i32) as libc::c_float,
            ((480i32 - 48i32) / 2i32) as libc::c_float,
            48i32 as libc::c_float,
            48i32 as libc::c_float,
            cg_items[value as usize].icon,
        );
    };
}
/*
=====================
CG_DrawCrosshairNames
=====================
*/
unsafe extern "C" fn CG_DrawCrosshairNames() {
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut w: libc::c_float = 0.;
    if 0 == cg_drawCrosshair.integer {
        return;
    }
    if 0 == cg_drawCrosshairNames.integer {
        return;
    }
    if 0 != cg.renderingThirdPerson as u64 {
        return;
    }
    CG_ScanForCrosshairEntity();
    color = CG_FadeColor(cg.crosshairClientTime, 1000i32);
    if color.is_null() {
        trap_R_SetColor(0 as *const libc::c_float);
        return;
    }
    name = cgs.clientinfo[cg.crosshairClientNum as usize]
        .name
        .as_mut_ptr();
    w = (CG_DrawStrlen(name) * 16i32) as libc::c_float;
    CG_DrawBigString(
        (320i32 as libc::c_float - w / 2i32 as libc::c_float) as libc::c_int,
        170i32,
        name,
        *color.offset(3isize) * 0.5f32,
    );
    trap_R_SetColor(0 as *const libc::c_float);
}
/*
=================
CG_ScanForCrosshairEntity
=================
*/
unsafe extern "C" fn CG_ScanForCrosshairEntity() {
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
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut content: libc::c_int = 0;
    start[0usize] = cg.refdef.vieworg[0usize];
    start[1usize] = cg.refdef.vieworg[1usize];
    start[2usize] = cg.refdef.vieworg[2usize];
    end[0usize] = start[0usize] + cg.refdef.viewaxis[0usize][0usize] * 131072i32 as libc::c_float;
    end[1usize] = start[1usize] + cg.refdef.viewaxis[0usize][1usize] * 131072i32 as libc::c_float;
    end[2usize] = start[2usize] + cg.refdef.viewaxis[0usize][2usize] * 131072i32 as libc::c_float;
    CG_Trace(
        &mut trace,
        start.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        end.as_mut_ptr() as *const vec_t,
        (*cg.snap).ps.clientNum,
        1i32 | 0x2000000i32,
    );
    if trace.entityNum >= 64i32 {
        return;
    }
    content = CG_PointContents(trace.endpos.as_mut_ptr() as *const vec_t, 0i32);
    if 0 != content & 64i32 {
        return;
    }
    if 0 != cg_entities[trace.entityNum as usize].currentState.powerups
        & 1i32 << PW_INVIS as libc::c_int
    {
        return;
    }
    cg.crosshairClientNum = trace.entityNum;
    cg.crosshairClientTime = cg.time;
}
/*
================================================================================

CROSSHAIR

================================================================================
*/
/*
=================
CG_DrawCrosshair
=================
*/
unsafe extern "C" fn CG_DrawCrosshair() {
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut hShader: qhandle_t = 0;
    let mut f: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut ca: libc::c_int = 0;
    if 0 == cg_drawCrosshair.integer {
        return;
    }
    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_SPECTATOR as libc::c_int
    {
        return;
    }
    if 0 != cg.renderingThirdPerson as u64 {
        return;
    }
    if 0 != cg_crosshairHealth.integer {
        let mut hcolor: vec4_t = [0.; 4];
        CG_ColorForHealth(hcolor.as_mut_ptr());
        trap_R_SetColor(hcolor.as_mut_ptr());
    } else {
        trap_R_SetColor(0 as *const libc::c_float);
    }
    h = cg_crosshairSize.value;
    w = h;
    f = (cg.time - cg.itemPickupBlendTime) as libc::c_float;
    if f > 0i32 as libc::c_float && f < 200i32 as libc::c_float {
        f /= 200i32 as libc::c_float;
        w *= 1i32 as libc::c_float + f;
        h *= 1i32 as libc::c_float + f
    }
    x = cg_crosshairX.integer as libc::c_float;
    y = cg_crosshairY.integer as libc::c_float;
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    ca = cg_drawCrosshair.integer;
    if ca < 0i32 {
        ca = 0i32
    }
    hShader = cgs.media.crosshairShader[(ca % 10i32) as usize];
    trap_R_DrawStretchPic(
        ((x + cg.refdef.x as libc::c_float) as libc::c_double
            + 0.5f64 * (cg.refdef.width as libc::c_float - w) as libc::c_double)
            as libc::c_float,
        ((y + cg.refdef.y as libc::c_float) as libc::c_double
            + 0.5f64 * (cg.refdef.height as libc::c_float - h) as libc::c_double)
            as libc::c_float,
        w,
        h,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        1i32 as libc::c_float,
        hShader,
    );
    trap_R_SetColor(0 as *const libc::c_float);
}
/*
=================
CG_DrawAmmoWarning
=================
*/
unsafe extern "C" fn CG_DrawAmmoWarning() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut w: libc::c_int = 0;
    if cg_drawAmmoWarning.integer == 0i32 {
        return;
    }
    if 0 == cg.lowAmmoWarning {
        return;
    }
    if cg.lowAmmoWarning == 2i32 {
        s = b"OUT OF AMMO\x00" as *const u8 as *const libc::c_char
    } else {
        s = b"LOW AMMO WARNING\x00" as *const u8 as *const libc::c_char
    }
    w = CG_DrawStrlen(s) * 16i32;
    CG_DrawBigString(320i32 - w / 2i32, 64i32, s, 1.0f32);
}
/*
================
CG_DrawStatusBar

================
*/
unsafe extern "C" fn CG_DrawStatusBar() {
    let mut color: libc::c_int = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    let mut value: libc::c_int = 0;
    let mut hcolor: vec4_t = [0.; 4];
    let mut angles: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    static mut colors: [[libc::c_float; 4]; 4] = [
        [1.0f32, 0.69f32, 0.0f32, 1.0f32],
        [1.0f32, 0.2f32, 0.2f32, 1.0f32],
        [0.5f32, 0.5f32, 0.5f32, 1.0f32],
        [1.0f32, 1.0f32, 1.0f32, 1.0f32],
    ];
    if cg_drawStatus.integer == 0i32 {
        return;
    }
    CG_DrawTeamBackground(
        0i32,
        420i32,
        640i32,
        60i32,
        0.33f32,
        (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize],
    );
    cent = &mut *cg_entities
        .as_mut_ptr()
        .offset((*cg.snap).ps.clientNum as isize) as *mut centity_t;
    ps = &mut (*cg.snap).ps;
    angles[2usize] = 0i32 as vec_t;
    angles[1usize] = angles[2usize];
    angles[0usize] = angles[1usize];
    if 0 != (*cent).currentState.weapon
        && 0 != cg_weapons[(*cent).currentState.weapon as usize].ammoModel
    {
        origin[0usize] = 70i32 as vec_t;
        origin[1usize] = 0i32 as vec_t;
        origin[2usize] = 0i32 as vec_t;
        angles[1usize] = (90i32 as libc::c_double
            + 20i32 as libc::c_double * sin(cg.time as libc::c_double / 1000.0f64))
            as vec_t;
        CG_Draw3DModel(
            (32i32 * 3i32 + 4i32) as libc::c_float,
            432i32 as libc::c_float,
            48i32 as libc::c_float,
            48i32 as libc::c_float,
            cg_weapons[(*cent).currentState.weapon as usize].ammoModel,
            0i32,
            origin.as_mut_ptr(),
            angles.as_mut_ptr(),
        );
    }
    CG_DrawStatusBarHead((185i32 + 32i32 * 3i32 + 4i32) as libc::c_float);
    if 0 != cg.predictedPlayerState.powerups[PW_REDFLAG as libc::c_int as usize] {
        CG_DrawStatusBarFlag(
            (185i32 + 32i32 * 3i32 + 4i32 + 48i32) as libc::c_float,
            TEAM_RED as libc::c_int,
        );
    } else if 0 != cg.predictedPlayerState.powerups[PW_BLUEFLAG as libc::c_int as usize] {
        CG_DrawStatusBarFlag(
            (185i32 + 32i32 * 3i32 + 4i32 + 48i32) as libc::c_float,
            TEAM_BLUE as libc::c_int,
        );
    } else if 0 != cg.predictedPlayerState.powerups[PW_NEUTRALFLAG as libc::c_int as usize] {
        CG_DrawStatusBarFlag(
            (185i32 + 32i32 * 3i32 + 4i32 + 48i32) as libc::c_float,
            TEAM_FREE as libc::c_int,
        );
    }
    if 0 != (*ps).stats[STAT_ARMOR as libc::c_int as usize] {
        origin[0usize] = 90i32 as vec_t;
        origin[1usize] = 0i32 as vec_t;
        origin[2usize] = -10i32 as vec_t;
        angles[1usize] = (((cg.time & 2047i32) * 360i32) as libc::c_double / 2048.0f64) as vec_t;
        CG_Draw3DModel(
            (370i32 + 32i32 * 3i32 + 4i32) as libc::c_float,
            432i32 as libc::c_float,
            48i32 as libc::c_float,
            48i32 as libc::c_float,
            cgs.media.armorModel,
            0i32,
            origin.as_mut_ptr(),
            angles.as_mut_ptr(),
        );
    }
    if 0 != (*cent).currentState.weapon {
        value = (*ps).ammo[(*cent).currentState.weapon as usize];
        if value > -1i32 {
            if cg.predictedPlayerState.weaponstate == WEAPON_FIRING as libc::c_int
                && cg.predictedPlayerState.weaponTime > 100i32
            {
                color = 2i32
            } else if value >= 0i32 {
                color = 0i32
            } else {
                color = 1i32
            }
            trap_R_SetColor(colors[color as usize].as_mut_ptr());
            CG_DrawField(0i32, 432i32, 3i32, value);
            trap_R_SetColor(0 as *const libc::c_float);
            if 0 == cg_draw3dIcons.integer && 0 != cg_drawIcons.integer {
                let mut icon: qhandle_t = 0;
                icon = cg_weapons[cg.predictedPlayerState.weapon as usize].ammoIcon;
                if 0 != icon {
                    CG_DrawPic(
                        (32i32 * 3i32 + 4i32) as libc::c_float,
                        432i32 as libc::c_float,
                        48i32 as libc::c_float,
                        48i32 as libc::c_float,
                        icon,
                    );
                }
            }
        }
    }
    value = (*ps).stats[STAT_HEALTH as libc::c_int as usize];
    if value > 100i32 {
        trap_R_SetColor(colors[3usize].as_mut_ptr());
    } else if value > 25i32 {
        trap_R_SetColor(colors[0usize].as_mut_ptr());
    } else if value > 0i32 {
        color = cg.time >> 8i32 & 1i32;
        trap_R_SetColor(colors[color as usize].as_mut_ptr());
    } else {
        trap_R_SetColor(colors[1usize].as_mut_ptr());
    }
    CG_DrawField(185i32, 432i32, 3i32, value);
    CG_ColorForHealth(hcolor.as_mut_ptr());
    trap_R_SetColor(hcolor.as_mut_ptr());
    value = (*ps).stats[STAT_ARMOR as libc::c_int as usize];
    if value > 0i32 {
        trap_R_SetColor(colors[0usize].as_mut_ptr());
        CG_DrawField(370i32, 432i32, 3i32, value);
        trap_R_SetColor(0 as *const libc::c_float);
        if 0 == cg_draw3dIcons.integer && 0 != cg_drawIcons.integer {
            CG_DrawPic(
                (370i32 + 32i32 * 3i32 + 4i32) as libc::c_float,
                432i32 as libc::c_float,
                48i32 as libc::c_float,
                48i32 as libc::c_float,
                cgs.media.armorIcon,
            );
        }
    };
}
// MISSIONPACK
/*
================
CG_DrawStatusBarFlag

================
*/
unsafe extern "C" fn CG_DrawStatusBarFlag(mut x: libc::c_float, mut team: libc::c_int) {
    CG_DrawFlagModel(
        x,
        (480i32 - 48i32) as libc::c_float,
        48i32 as libc::c_float,
        48i32 as libc::c_float,
        team,
        qfalse,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawFlagModel(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut team: libc::c_int,
    mut force2D: qboolean,
) {
    let mut cm: qhandle_t = 0;
    let mut len: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut handle: qhandle_t = 0;
    if 0 == force2D as u64 && 0 != cg_draw3dIcons.integer {
        angles[2usize] = 0i32 as vec_t;
        angles[1usize] = angles[2usize];
        angles[0usize] = angles[1usize];
        cm = cgs.media.redFlagModel;
        trap_R_ModelBounds(cm, mins.as_mut_ptr(), maxs.as_mut_ptr());
        origin[2usize] = (-0.5f64 * (mins[2usize] + maxs[2usize]) as libc::c_double) as vec_t;
        origin[1usize] = (0.5f64 * (mins[1usize] + maxs[1usize]) as libc::c_double) as vec_t;
        len = (0.5f64 * (maxs[2usize] - mins[2usize]) as libc::c_double) as libc::c_float;
        origin[0usize] = (len as libc::c_double / 0.268f64) as vec_t;
        angles[1usize] =
            (60i32 as libc::c_double * sin(cg.time as libc::c_double / 2000.0f64)) as vec_t;
        if team == TEAM_RED as libc::c_int {
            handle = cgs.media.redFlagModel
        } else if team == TEAM_BLUE as libc::c_int {
            handle = cgs.media.blueFlagModel
        } else if team == TEAM_FREE as libc::c_int {
            handle = cgs.media.neutralFlagModel
        } else {
            return;
        }
        CG_Draw3DModel(
            x,
            y,
            w,
            h,
            handle,
            0i32,
            origin.as_mut_ptr(),
            angles.as_mut_ptr(),
        );
    } else if 0 != cg_drawIcons.integer {
        let mut item: *mut gitem_t = 0 as *mut gitem_t;
        if team == TEAM_RED as libc::c_int {
            item = BG_FindItemForPowerup(PW_REDFLAG)
        } else if team == TEAM_BLUE as libc::c_int {
            item = BG_FindItemForPowerup(PW_BLUEFLAG)
        } else if team == TEAM_FREE as libc::c_int {
            item = BG_FindItemForPowerup(PW_NEUTRALFLAG)
        } else {
            return;
        }
        if !item.is_null() {
            CG_DrawPic(
                x,
                y,
                w,
                h,
                cg_items
                    [item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as usize]
                    .icon,
            );
        }
    };
}
/*
================
CG_DrawStatusBarHead

================
*/
unsafe extern "C" fn CG_DrawStatusBarHead(mut x: libc::c_float) {
    let mut angles: vec3_t = [0.; 3];
    let mut size: libc::c_float = 0.;
    let mut stretch: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    angles[2usize] = 0i32 as vec_t;
    angles[1usize] = angles[2usize];
    angles[0usize] = angles[1usize];
    if 0. != cg.damageTime && cg.time as libc::c_float - cg.damageTime < 500i32 as libc::c_float {
        frac = (cg.time as libc::c_float - cg.damageTime) / 500i32 as libc::c_float;
        size = (48i32 as libc::c_double * 1.25f64 * (1.5f64 - frac as libc::c_double * 0.5f64))
            as libc::c_float;
        stretch = (size as libc::c_double - 48i32 as libc::c_double * 1.25f64) as libc::c_float;
        x = (x as libc::c_double
            - (stretch as libc::c_double * 0.5f64
                + (cg.damageX * stretch) as libc::c_double * 0.5f64)) as libc::c_float;
        cg.headStartYaw = 180i32 as libc::c_float + cg.damageX * 45i32 as libc::c_float;
        cg.headEndYaw = (180i32 as libc::c_double
            + 20i32 as libc::c_double
                * cos(2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64)
                    * 3.14159265358979323846f64)) as libc::c_float;
        cg.headEndPitch = (5i32 as libc::c_double
            * cos(2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 3.14159265358979323846f64)) as libc::c_float;
        cg.headStartTime = cg.time;
        cg.headEndTime = ((cg.time + 100i32) as libc::c_float
            + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                * 2000i32 as libc::c_float) as libc::c_int
    } else {
        if cg.time >= cg.headEndTime {
            cg.headStartYaw = cg.headEndYaw;
            cg.headStartPitch = cg.headEndPitch;
            cg.headStartTime = cg.headEndTime;
            cg.headEndTime = ((cg.time + 100i32) as libc::c_float
                + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                    * 2000i32 as libc::c_float) as libc::c_int;
            cg.headEndYaw = (180i32 as libc::c_double
                + 20i32 as libc::c_double
                    * cos(2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64)
                        * 3.14159265358979323846f64)) as libc::c_float;
            cg.headEndPitch = (5i32 as libc::c_double
                * cos(2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64)
                    * 3.14159265358979323846f64)) as libc::c_float
        }
        size = (48i32 as libc::c_double * 1.25f64) as libc::c_float
    }
    if cg.headStartTime > cg.time {
        cg.headStartTime = cg.time
    }
    frac = (cg.time - cg.headStartTime) as libc::c_float
        / (cg.headEndTime - cg.headStartTime) as libc::c_float;
    frac = frac * frac * (3i32 as libc::c_float - 2i32 as libc::c_float * frac);
    angles[1usize] = cg.headStartYaw + (cg.headEndYaw - cg.headStartYaw) * frac;
    angles[0usize] = cg.headStartPitch + (cg.headEndPitch - cg.headStartPitch) * frac;
    CG_DrawHead(
        x,
        480i32 as libc::c_float - size,
        size,
        size,
        (*cg.snap).ps.clientNum,
        angles.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawTeamBackground(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut alpha: libc::c_float,
    mut team: libc::c_int,
) {
    let mut hcolor: vec4_t = [0.; 4];
    hcolor[3usize] = alpha;
    if team == TEAM_RED as libc::c_int {
        hcolor[0usize] = 1i32 as vec_t;
        hcolor[1usize] = 0i32 as vec_t;
        hcolor[2usize] = 0i32 as vec_t
    } else if team == TEAM_BLUE as libc::c_int {
        hcolor[0usize] = 0i32 as vec_t;
        hcolor[1usize] = 0i32 as vec_t;
        hcolor[2usize] = 1i32 as vec_t
    } else {
        return;
    }
    trap_R_SetColor(hcolor.as_mut_ptr());
    CG_DrawPic(
        x as libc::c_float,
        y as libc::c_float,
        w as libc::c_float,
        h as libc::c_float,
        cgs.media.teamStatusBar,
    );
    trap_R_SetColor(0 as *const libc::c_float);
}
//==============================================================================
/*
=================
CG_DrawSpectator
=================
*/
unsafe extern "C" fn CG_DrawSpectator() {
    CG_DrawBigString(
        320i32 - 9i32 * 8i32,
        440i32,
        b"SPECTATOR\x00" as *const u8 as *const libc::c_char,
        1.0f32,
    );
    if cgs.gametype as libc::c_uint == GT_TOURNAMENT as libc::c_int as libc::c_uint {
        CG_DrawBigString(
            320i32 - 15i32 * 8i32,
            460i32,
            b"waiting to play\x00" as *const u8 as *const libc::c_char,
            1.0f32,
        );
    } else if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        CG_DrawBigString(
            320i32 - 39i32 * 8i32,
            460i32,
            b"press ESC and use the JOIN menu to play\x00" as *const u8 as *const libc::c_char,
            1.0f32,
        );
    };
}
/*
=================
CG_DrawIntermission
=================
*/
unsafe extern "C" fn CG_DrawIntermission() {
    if cgs.gametype as libc::c_uint == GT_SINGLE_PLAYER as libc::c_int as libc::c_uint {
        CG_DrawCenterString();
        return;
    }
    cg.scoreFadeTime = cg.time;
    cg.scoreBoardShowing = CG_DrawScoreboard();
}
/*
=================
CG_DrawCrosshair3D
=================
*/
unsafe extern "C" fn CG_DrawCrosshair3D() {
    let mut w: libc::c_float = 0.;
    let mut hShader: qhandle_t = 0;
    let mut f: libc::c_float = 0.;
    let mut ca: libc::c_int = 0;
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
    let mut endpos: vec3_t = [0.; 3];
    let mut stereoSep: libc::c_float = 0.;
    let mut zProj: libc::c_float = 0.;
    let mut maxdist: libc::c_float = 0.;
    let mut xmax: libc::c_float = 0.;
    let mut rendererinfos: [libc::c_char; 128] = [0; 128];
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
    if 0 == cg_drawCrosshair.integer {
        return;
    }
    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize] == TEAM_SPECTATOR as libc::c_int
    {
        return;
    }
    if 0 != cg.renderingThirdPerson as u64 {
        return;
    }
    w = cg_crosshairSize.value;
    f = (cg.time - cg.itemPickupBlendTime) as libc::c_float;
    if f > 0i32 as libc::c_float && f < 200i32 as libc::c_float {
        f /= 200i32 as libc::c_float;
        w *= 1i32 as libc::c_float + f
    }
    ca = cg_drawCrosshair.integer;
    if ca < 0i32 {
        ca = 0i32
    }
    hShader = cgs.media.crosshairShader[(ca % 10i32) as usize];
    trap_Cvar_VariableStringBuffer(
        b"r_zProj\x00" as *const u8 as *const libc::c_char,
        rendererinfos.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    zProj = atof(rendererinfos.as_mut_ptr()) as libc::c_float;
    trap_Cvar_VariableStringBuffer(
        b"r_stereoSeparation\x00" as *const u8 as *const libc::c_char,
        rendererinfos.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    stereoSep = (zProj as libc::c_double / atof(rendererinfos.as_mut_ptr())) as libc::c_float;
    xmax = (zProj as libc::c_double
        * tan(
            cg.refdef.fov_x as libc::c_double * 3.14159265358979323846f64
                / 360.0f32 as libc::c_double,
        )) as libc::c_float;
    maxdist =
        cgs.glconfig.vidWidth as libc::c_float * stereoSep * zProj / (2i32 as libc::c_float * xmax);
    endpos[0usize] = cg.refdef.vieworg[0usize] + cg.refdef.viewaxis[0usize][0usize] * maxdist;
    endpos[1usize] = cg.refdef.vieworg[1usize] + cg.refdef.viewaxis[0usize][1usize] * maxdist;
    endpos[2usize] = cg.refdef.vieworg[2usize] + cg.refdef.viewaxis[0usize][2usize] * maxdist;
    CG_Trace(
        &mut trace,
        cg.refdef.vieworg.as_mut_ptr() as *const vec_t,
        0 as *const vec_t,
        0 as *const vec_t,
        endpos.as_mut_ptr() as *const vec_t,
        0i32,
        1i32 | 0x2000000i32 | 0x4000000i32,
    );
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.reType = RT_SPRITE;
    ent.renderfx = 0x8i32 | 0x10i32;
    ent.origin[0usize] = trace.endpos[0usize];
    ent.origin[1usize] = trace.endpos[1usize];
    ent.origin[2usize] = trace.endpos[2usize];
    ent.radius = w / 640i32 as libc::c_float * xmax * trace.fraction * maxdist / zProj;
    ent.customShader = hShader;
    trap_R_AddRefEntityToScene(&mut ent);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lagometer_t {
    pub frameSamples: [libc::c_int; 128],
    pub frameCount: libc::c_int,
    pub snapshotFlags: [libc::c_int; 128],
    pub snapshotSamples: [libc::c_int; 128],
    pub snapshotCount: libc::c_int,
}
