use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, unnamed, GENDER_FEMALE, GENDER_MALE,
    GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW,
    STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE, TEAM_FREE,
    TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
use cg_consolecmds::{CG_ConsoleCommand, CG_InitConsoleCommands};
use cg_draw::{
    drawTeamOverlayModificationCount, numSortedTeamPlayers, sortedTeamPlayers,
    CG_AddLagometerFrameInfo, CG_AddLagometerSnapshotInfo, CG_CenterPrint, CG_DrawActive,
    CG_DrawFlagModel, CG_DrawHead, CG_DrawTeamBackground,
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
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, lerpFrame_t,
    playerEntity_t, score_t, trap_R_DrawStretchPic, trap_R_SetColor, FOOTSTEP_BOOT,
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
use libc;
use q_math::{
    axisDefault, colorWhite, g_color_table, vec3_origin, vectoangles, AngleMod, AngleNormalize180,
    AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis, AxisClear, AxisCopy, ByteToDir,
    LerpAngle, MatrixMultiply, PerpendicularVector, Q_crandom, Q_random, RotateAroundDirection,
    RotatePointAroundVector, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, entityState_s, entityState_t, gameState_t, playerState_s, playerState_t, qboolean,
    qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, vec3_t, vec4_t, vec_t,
    Q_IsColorString, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{memcpy, sin};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

//
// cg_drawtools.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_AdjustFrom640(
    mut x: *mut libc::c_float,
    mut y: *mut libc::c_float,
    mut w: *mut libc::c_float,
    mut h: *mut libc::c_float,
) {
    *x *= cgs.screenXScale;
    *y *= cgs.screenYScale;
    *w *= cgs.screenXScale;
    *h *= cgs.screenYScale;
}
#[no_mangle]
pub unsafe extern "C" fn CG_FillRect(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut color: *const libc::c_float,
) {
    trap_R_SetColor(color);
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        cgs.media.whiteShader,
    );
    trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawPic(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut hShader: qhandle_t,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        1i32 as libc::c_float,
        hShader,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawStringExt(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut string: *const libc::c_char,
    mut setColor: *const libc::c_float,
    mut forceColor: qboolean,
    mut shadow: qboolean,
    mut charWidth: libc::c_int,
    mut charHeight: libc::c_int,
    mut maxChars: libc::c_int,
) {
    let mut color: vec4_t = [0.; 4];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut xx: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    if maxChars <= 0i32 {
        maxChars = 32767i32
    }
    if 0 != shadow as u64 {
        color[2usize] = 0i32 as vec_t;
        color[1usize] = color[2usize];
        color[0usize] = color[1usize];
        color[3usize] = *setColor.offset(3isize);
        trap_R_SetColor(color.as_mut_ptr());
        s = string;
        xx = x;
        cnt = 0i32;
        while 0 != *s as libc::c_int && cnt < maxChars {
            if 0 != Q_IsColorString(s) as u64 {
                s = s.offset(2isize)
            } else {
                CG_DrawChar(
                    xx + 2i32,
                    y + 2i32,
                    charWidth,
                    charHeight,
                    *s as libc::c_int,
                );
                cnt += 1;
                xx += charWidth;
                s = s.offset(1isize)
            }
        }
    }
    s = string;
    xx = x;
    cnt = 0i32;
    trap_R_SetColor(setColor);
    while 0 != *s as libc::c_int && cnt < maxChars {
        if 0 != Q_IsColorString(s) as u64 {
            if 0 == forceColor as u64 {
                memcpy(
                    color.as_mut_ptr() as *mut libc::c_void,
                    g_color_table[(*s.offset(1isize) as libc::c_int - '0' as i32 & 0x7i32) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<vec4_t>() as libc::c_ulong,
                );
                color[3usize] = *setColor.offset(3isize);
                trap_R_SetColor(color.as_mut_ptr());
            }
            s = s.offset(2isize)
        } else {
            CG_DrawChar(xx, y, charWidth, charHeight, *s as libc::c_int);
            xx += charWidth;
            cnt += 1;
            s = s.offset(1isize)
        }
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
/*
===============
CG_DrawChar

Coordinates and size in 640*480 virtual screen size
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CG_DrawChar(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut ch: libc::c_int,
) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut size: libc::c_float = 0.;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    ch &= 255i32;
    if ch == ' ' as i32 {
        return;
    }
    ax = x as libc::c_float;
    ay = y as libc::c_float;
    aw = width as libc::c_float;
    ah = height as libc::c_float;
    CG_AdjustFrom640(&mut ax, &mut ay, &mut aw, &mut ah);
    row = ch >> 4i32;
    col = ch & 15i32;
    frow = (row as libc::c_double * 0.0625f64) as libc::c_float;
    fcol = (col as libc::c_double * 0.0625f64) as libc::c_float;
    size = 0.0625f64 as libc::c_float;
    trap_R_DrawStretchPic(
        ax,
        ay,
        aw,
        ah,
        fcol,
        frow,
        fcol + size,
        frow + size,
        cgs.media.charsetShader,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawBigString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut alpha: libc::c_float,
) {
    let mut color: [libc::c_float; 4] = [0.; 4];
    color[2usize] = 1.0f64 as libc::c_float;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    color[3usize] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        qfalse,
        qtrue,
        16i32,
        16i32,
        0i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawBigStringColor(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut color: *mut vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const libc::c_float,
        qtrue,
        qtrue,
        16i32,
        16i32,
        0i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawSmallString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut alpha: libc::c_float,
) {
    let mut color: [libc::c_float; 4] = [0.; 4];
    color[2usize] = 1.0f64 as libc::c_float;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    color[3usize] = alpha;
    CG_DrawStringExt(
        x,
        y,
        s,
        color.as_mut_ptr(),
        qfalse,
        qfalse,
        8i32,
        16i32,
        0i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawSmallStringColor(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *const libc::c_char,
    mut color: *mut vec_t,
) {
    CG_DrawStringExt(
        x,
        y,
        s,
        color as *const libc::c_float,
        qtrue,
        qfalse,
        8i32,
        16i32,
        0i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawStrlen(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = str;
    let mut count: libc::c_int = 0i32;
    while 0 != *s {
        if 0 != Q_IsColorString(s) as u64 {
            s = s.offset(2isize)
        } else {
            count += 1;
            s = s.offset(1isize)
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn CG_FadeColor(
    mut startMsec: libc::c_int,
    mut totalMsec: libc::c_int,
) -> *mut libc::c_float {
    static mut color: vec4_t = [0.; 4];
    let mut t: libc::c_int = 0;
    if startMsec == 0i32 {
        return 0 as *mut libc::c_float;
    }
    t = cg.time - startMsec;
    if t >= totalMsec {
        return 0 as *mut libc::c_float;
    }
    if totalMsec - t < 200i32 {
        color[3usize] =
            ((totalMsec - t) as libc::c_double * 1.0f64 / 200i32 as libc::c_double) as vec_t
    } else {
        color[3usize] = 1.0f64 as vec_t
    }
    color[2usize] = 1i32 as vec_t;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    return color.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn CG_TeamColor(mut team: libc::c_int) -> *mut libc::c_float {
    static mut red: vec4_t = [1i32 as vec_t, 0.2f32, 0.2f32, 1i32 as vec_t];
    static mut blue: vec4_t = [0.2f32, 0.2f32, 1i32 as vec_t, 1i32 as vec_t];
    static mut other: vec4_t = [1i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t];
    static mut spectator: vec4_t = [0.7f32, 0.7f32, 0.7f32, 1i32 as vec_t];
    match team {
        1 => return red.as_mut_ptr(),
        2 => return blue.as_mut_ptr(),
        3 => return spectator.as_mut_ptr(),
        _ => return other.as_mut_ptr(),
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_TileClear() {
    let mut top: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    w = cgs.glconfig.vidWidth;
    h = cgs.glconfig.vidHeight;
    if cg.refdef.x == 0i32 && cg.refdef.y == 0i32 && cg.refdef.width == w && cg.refdef.height == h {
        return;
    }
    top = cg.refdef.y;
    bottom = top + cg.refdef.height - 1i32;
    left = cg.refdef.x;
    right = left + cg.refdef.width - 1i32;
    CG_TileClearBox(0i32, 0i32, w, top, cgs.media.backTileShader);
    CG_TileClearBox(0i32, bottom, w, h - bottom, cgs.media.backTileShader);
    CG_TileClearBox(
        0i32,
        top,
        left,
        bottom - top + 1i32,
        cgs.media.backTileShader,
    );
    CG_TileClearBox(
        right,
        top,
        w - right,
        bottom - top + 1i32,
        cgs.media.backTileShader,
    );
}
/*
=============
CG_TileClearBox

This repeats a 64*64 tile graphic to fill the screen around a sized down
refresh window.
=============
*/
unsafe extern "C" fn CG_TileClearBox(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut hShader: qhandle_t,
) {
    let mut s1: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut s2: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    s1 = (x as libc::c_double / 64.0f64) as libc::c_float;
    t1 = (y as libc::c_double / 64.0f64) as libc::c_float;
    s2 = ((x + w) as libc::c_double / 64.0f64) as libc::c_float;
    t2 = ((y + h) as libc::c_double / 64.0f64) as libc::c_float;
    trap_R_DrawStretchPic(
        x as libc::c_float,
        y as libc::c_float,
        w as libc::c_float,
        h as libc::c_float,
        s1,
        t1,
        s2,
        t2,
        hShader,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_ColorForHealth(mut hcolor: *mut vec_t) {
    CG_GetColorForHealth(
        (*cg.snap).ps.stats[STAT_HEALTH as libc::c_int as usize],
        (*cg.snap).ps.stats[STAT_ARMOR as libc::c_int as usize],
        hcolor,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_GetColorForHealth(
    mut health: libc::c_int,
    mut armor: libc::c_int,
    mut hcolor: *mut vec_t,
) {
    let mut count: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    if health <= 0i32 {
        let ref mut fresh1 = *hcolor.offset(1isize);
        let ref mut fresh0 = *hcolor.offset(2isize);
        *fresh0 = 0i32 as vec_t;
        *fresh1 = *fresh0;
        *hcolor.offset(0isize) = *fresh1;
        *hcolor.offset(3isize) = 1i32 as vec_t;
        return;
    }
    count = armor;
    max = (health as libc::c_double * 0.66f64 / (1.0f64 - 0.66f64)) as libc::c_int;
    if max < count {
        count = max
    }
    health += count;
    *hcolor.offset(0isize) = 1.0f64 as vec_t;
    *hcolor.offset(3isize) = 1.0f64 as vec_t;
    if health >= 100i32 {
        *hcolor.offset(2isize) = 1.0f64 as vec_t
    } else if health < 66i32 {
        *hcolor.offset(2isize) = 0i32 as vec_t
    } else {
        *hcolor.offset(2isize) = ((health - 66i32) as libc::c_double / 33.0f64) as vec_t
    }
    if health > 60i32 {
        *hcolor.offset(1isize) = 1.0f64 as vec_t
    } else if health < 30i32 {
        *hcolor.offset(1isize) = 0i32 as vec_t
    } else {
        *hcolor.offset(1isize) = ((health - 30i32) as libc::c_double / 30.0f64) as vec_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawProportionalString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut drawcolor: vec4_t = [0.; 4];
    let mut width: libc::c_int = 0;
    let mut sizeScale: libc::c_float = 0.;
    sizeScale = UI_ProportionalSizeScale(style);
    match style & 0x7i32 {
        1 => {
            width = (UI_ProportionalStringWidth(str) as libc::c_float * sizeScale) as libc::c_int;
            x -= width / 2i32
        }
        2 => {
            width = (UI_ProportionalStringWidth(str) as libc::c_float * sizeScale) as libc::c_int;
            x -= width
        }
        0 | _ => {}
    }
    if 0 != style & 0x800i32 {
        drawcolor[2usize] = 0i32 as vec_t;
        drawcolor[1usize] = drawcolor[2usize];
        drawcolor[0usize] = drawcolor[1usize];
        drawcolor[3usize] = *color.offset(3isize);
        UI_DrawProportionalString2(
            x + 2i32,
            y + 2i32,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            cgs.media.charsetProp,
        );
    }
    if 0 != style & 0x2000i32 {
        drawcolor[0usize] = (*color.offset(0isize) as libc::c_double * 0.8f64) as vec_t;
        drawcolor[1usize] = (*color.offset(1isize) as libc::c_double * 0.8f64) as vec_t;
        drawcolor[2usize] = (*color.offset(2isize) as libc::c_double * 0.8f64) as vec_t;
        drawcolor[3usize] = *color.offset(3isize);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            cgs.media.charsetProp,
        );
        return;
    }
    if 0 != style & 0x4000i32 {
        drawcolor[0usize] = (*color.offset(0isize) as libc::c_double * 0.8f64) as vec_t;
        drawcolor[1usize] = (*color.offset(1isize) as libc::c_double * 0.8f64) as vec_t;
        drawcolor[2usize] = (*color.offset(2isize) as libc::c_double * 0.8f64) as vec_t;
        drawcolor[3usize] = *color.offset(3isize);
        UI_DrawProportionalString2(x, y, str, color, sizeScale, cgs.media.charsetProp);
        drawcolor[0usize] = *color.offset(0isize);
        drawcolor[1usize] = *color.offset(1isize);
        drawcolor[2usize] = *color.offset(2isize);
        drawcolor[3usize] = (0.5f64 + 0.5f64 * sin((cg.time / 75i32) as libc::c_double)) as vec_t;
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            cgs.media.charsetPropGlow,
        );
        return;
    }
    UI_DrawProportionalString2(x, y, str, color, sizeScale, cgs.media.charsetProp);
}
unsafe extern "C" fn UI_DrawProportionalString2(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
    mut sizeScale: libc::c_float,
    mut charset: qhandle_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_uchar = 0;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut fwidth: libc::c_float = 0.;
    let mut fheight: libc::c_float = 0.;
    trap_R_SetColor(color as *const libc::c_float);
    ax = x as libc::c_float * cgs.screenXScale + cgs.screenXBias;
    ay = y as libc::c_float * cgs.screenYScale;
    s = str;
    while 0 != *s {
        ch = (*s as libc::c_int & 127i32) as libc::c_uchar;
        if ch as libc::c_int == ' ' as i32 {
            aw = 8i32 as libc::c_float * cgs.screenXScale * sizeScale
        } else if propMap[ch as usize][2usize] != -1i32 {
            fcol = propMap[ch as usize][0usize] as libc::c_float / 256.0f32;
            frow = propMap[ch as usize][1usize] as libc::c_float / 256.0f32;
            fwidth = propMap[ch as usize][2usize] as libc::c_float / 256.0f32;
            fheight = 27i32 as libc::c_float / 256.0f32;
            aw = propMap[ch as usize][2usize] as libc::c_float * cgs.screenXScale * sizeScale;
            ah = 27i32 as libc::c_float * cgs.screenYScale * sizeScale;
            trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                charset,
            );
        } else {
            aw = 0i32 as libc::c_float
        }
        ax += aw + 3i32 as libc::c_float * cgs.screenXScale * sizeScale;
        s = s.offset(1isize)
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
/*
=================
UI_DrawProportionalString2
=================
*/
static mut propMap: [[libc::c_int; 3]; 128] = [
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, 8i32],
    [11i32, 122i32, 7i32],
    [154i32, 181i32, 14i32],
    [55i32, 122i32, 17i32],
    [79i32, 122i32, 18i32],
    [101i32, 122i32, 23i32],
    [153i32, 122i32, 18i32],
    [9i32, 93i32, 7i32],
    [207i32, 122i32, 8i32],
    [230i32, 122i32, 9i32],
    [177i32, 122i32, 18i32],
    [30i32, 152i32, 18i32],
    [85i32, 181i32, 7i32],
    [34i32, 93i32, 11i32],
    [110i32, 181i32, 6i32],
    [130i32, 152i32, 14i32],
    [22i32, 64i32, 17i32],
    [41i32, 64i32, 12i32],
    [58i32, 64i32, 17i32],
    [78i32, 64i32, 18i32],
    [98i32, 64i32, 19i32],
    [120i32, 64i32, 18i32],
    [141i32, 64i32, 18i32],
    [204i32, 64i32, 16i32],
    [162i32, 64i32, 17i32],
    [182i32, 64i32, 18i32],
    [59i32, 181i32, 7i32],
    [35i32, 181i32, 7i32],
    [203i32, 152i32, 14i32],
    [56i32, 93i32, 14i32],
    [228i32, 152i32, 14i32],
    [177i32, 181i32, 18i32],
    [28i32, 122i32, 22i32],
    [5i32, 4i32, 18i32],
    [27i32, 4i32, 18i32],
    [48i32, 4i32, 18i32],
    [69i32, 4i32, 17i32],
    [90i32, 4i32, 13i32],
    [106i32, 4i32, 13i32],
    [121i32, 4i32, 18i32],
    [143i32, 4i32, 17i32],
    [164i32, 4i32, 8i32],
    [175i32, 4i32, 16i32],
    [195i32, 4i32, 18i32],
    [216i32, 4i32, 12i32],
    [230i32, 4i32, 23i32],
    [6i32, 34i32, 18i32],
    [27i32, 34i32, 18i32],
    [48i32, 34i32, 18i32],
    [68i32, 34i32, 18i32],
    [90i32, 34i32, 17i32],
    [110i32, 34i32, 18i32],
    [130i32, 34i32, 14i32],
    [146i32, 34i32, 18i32],
    [166i32, 34i32, 19i32],
    [185i32, 34i32, 29i32],
    [215i32, 34i32, 18i32],
    [234i32, 34i32, 18i32],
    [5i32, 64i32, 14i32],
    [60i32, 152i32, 7i32],
    [106i32, 151i32, 13i32],
    [83i32, 152i32, 7i32],
    [128i32, 122i32, 17i32],
    [4i32, 152i32, 21i32],
    [134i32, 181i32, 5i32],
    [5i32, 4i32, 18i32],
    [27i32, 4i32, 18i32],
    [48i32, 4i32, 18i32],
    [69i32, 4i32, 17i32],
    [90i32, 4i32, 13i32],
    [106i32, 4i32, 13i32],
    [121i32, 4i32, 18i32],
    [143i32, 4i32, 17i32],
    [164i32, 4i32, 8i32],
    [175i32, 4i32, 16i32],
    [195i32, 4i32, 18i32],
    [216i32, 4i32, 12i32],
    [230i32, 4i32, 23i32],
    [6i32, 34i32, 18i32],
    [27i32, 34i32, 18i32],
    [48i32, 34i32, 18i32],
    [68i32, 34i32, 18i32],
    [90i32, 34i32, 17i32],
    [110i32, 34i32, 18i32],
    [130i32, 34i32, 14i32],
    [146i32, 34i32, 18i32],
    [166i32, 34i32, 19i32],
    [185i32, 34i32, 29i32],
    [215i32, 34i32, 18i32],
    [234i32, 34i32, 18i32],
    [5i32, 64i32, 14i32],
    [153i32, 152i32, 13i32],
    [11i32, 181i32, 5i32],
    [180i32, 152i32, 13i32],
    [79i32, 93i32, 17i32],
    [0i32, 0i32, -1i32],
];
#[no_mangle]
pub unsafe extern "C" fn UI_ProportionalStringWidth(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut charWidth: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    s = str;
    width = 0i32;
    while 0 != *s {
        ch = *s as libc::c_int & 127i32;
        charWidth = propMap[ch as usize][2usize];
        if charWidth != -1i32 {
            width += charWidth;
            width += 3i32
        }
        s = s.offset(1isize)
    }
    width -= 3i32;
    return width;
}
/*
=================
UI_ProportionalSizeScale
=================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_ProportionalSizeScale(mut style: libc::c_int) -> libc::c_float {
    if 0 != style & 0x10i32 {
        return 0.75f64 as libc::c_float;
    }
    return 1.00f64 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawRect(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut size: libc::c_float,
    mut color: *const libc::c_float,
) {
    trap_R_SetColor(color);
    CG_DrawTopBottom(x, y, width, height, size);
    CG_DrawSides(
        x,
        y + size,
        width,
        height - size * 2i32 as libc::c_float,
        size,
    );
    trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawSides(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut size: libc::c_float,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= cgs.screenXScale;
    trap_R_DrawStretchPic(
        x,
        y,
        size,
        h,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        cgs.media.whiteShader,
    );
    trap_R_DrawStretchPic(
        x + w - size,
        y,
        size,
        h,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        cgs.media.whiteShader,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawTopBottom(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut size: libc::c_float,
) {
    CG_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    size *= cgs.screenYScale;
    trap_R_DrawStretchPic(
        x,
        y,
        w,
        size,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        cgs.media.whiteShader,
    );
    trap_R_DrawStretchPic(
        x,
        y + h - size,
        w,
        size,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        cgs.media.whiteShader,
    );
}
// SPACE
// !
// "
// #
// $
// %
// &
// '
// (
// )
// *
// +
// ,
// -
// .
// /
// 0
// 1
// 2
// 3
// 4
// 5
// 6
// 7
// 8
// 9
// :
// ;
// <
// =
// >
// ?
// @
// A
// B
// C
// D
// E
// F
// G
// H
// I
// J
// K
// L
// M
// N
// O
// P
// Q
// R
// S
// T
// U
// V
// W
// X
// Y
// Z
// [
// '\'
// ]
// ^
// _
// '
// A
// B
// C
// D
// E
// F
// G
// H
// I
// J
// K
// L
// M
// N
// O
// P
// Q
// R
// S
// T
// U
// V
// W
// X
// Y
// Z
// {
// |
// }
// ~
// DEL
static mut propMapB: [[libc::c_int; 3]; 26] = [
    [11i32, 12i32, 33i32],
    [49i32, 12i32, 31i32],
    [85i32, 12i32, 31i32],
    [120i32, 12i32, 30i32],
    [156i32, 12i32, 21i32],
    [183i32, 12i32, 21i32],
    [207i32, 12i32, 32i32],
    [13i32, 55i32, 30i32],
    [49i32, 55i32, 13i32],
    [66i32, 55i32, 29i32],
    [101i32, 55i32, 31i32],
    [135i32, 55i32, 21i32],
    [158i32, 55i32, 40i32],
    [204i32, 55i32, 32i32],
    [12i32, 97i32, 31i32],
    [48i32, 97i32, 31i32],
    [82i32, 97i32, 30i32],
    [118i32, 97i32, 30i32],
    [153i32, 97i32, 30i32],
    [185i32, 97i32, 25i32],
    [213i32, 97i32, 30i32],
    [11i32, 139i32, 32i32],
    [42i32, 139i32, 51i32],
    [93i32, 139i32, 32i32],
    [126i32, 139i32, 31i32],
    [158i32, 139i32, 25i32],
];
/*
=================
UI_DrawBannerString
=================
*/
unsafe extern "C" fn UI_DrawBannerString2(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_uchar = 0;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut fwidth: libc::c_float = 0.;
    let mut fheight: libc::c_float = 0.;
    trap_R_SetColor(color as *const libc::c_float);
    ax = x as libc::c_float * cgs.screenXScale + cgs.screenXBias;
    ay = y as libc::c_float * cgs.screenYScale;
    s = str;
    while 0 != *s {
        ch = (*s as libc::c_int & 127i32) as libc::c_uchar;
        if ch as libc::c_int == ' ' as i32 {
            ax += (12i32 as libc::c_float + 4i32 as libc::c_float) * cgs.screenXScale
        } else if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32 {
            ch = (ch as libc::c_int - 'A' as i32) as libc::c_uchar;
            fcol = propMapB[ch as usize][0usize] as libc::c_float / 256.0f32;
            frow = propMapB[ch as usize][1usize] as libc::c_float / 256.0f32;
            fwidth = propMapB[ch as usize][2usize] as libc::c_float / 256.0f32;
            fheight = 36i32 as libc::c_float / 256.0f32;
            aw = propMapB[ch as usize][2usize] as libc::c_float * cgs.screenXScale;
            ah = 36i32 as libc::c_float * cgs.screenYScale;
            trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                cgs.media.charsetPropB,
            );
            ax += aw + 4i32 as libc::c_float * cgs.screenXScale
        }
        s = s.offset(1isize)
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawBannerString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut drawcolor: vec4_t = [0.; 4];
    s = str;
    width = 0i32;
    while 0 != *s {
        ch = *s as libc::c_int;
        if ch == ' ' as i32 {
            width += 12i32
        } else if ch >= 'A' as i32 && ch <= 'Z' as i32 {
            width += propMapB[(ch - 'A' as i32) as usize][2usize] + 4i32
        }
        s = s.offset(1isize)
    }
    width -= 4i32;
    match style & 0x7i32 {
        1 => x -= width / 2i32,
        2 => x -= width,
        0 | _ => {}
    }
    if 0 != style & 0x800i32 {
        drawcolor[2usize] = 0i32 as vec_t;
        drawcolor[1usize] = drawcolor[2usize];
        drawcolor[0usize] = drawcolor[1usize];
        drawcolor[3usize] = *color.offset(3isize);
        UI_DrawBannerString2(x + 2i32, y + 2i32, str, drawcolor.as_mut_ptr());
    }
    UI_DrawBannerString2(x, y, str, color);
}
