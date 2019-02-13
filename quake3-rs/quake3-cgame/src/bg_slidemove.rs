use bg_local_h::pml_t;
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    pmove_t, unnamed, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON,
    EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT,
    EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND,
    EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE,
    EV_INVUL_IMPACT, EV_ITEM_PICKUP, EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED, EV_JUMP, EV_JUMP_PAD,
    EV_KAMIKAZE, EV_LIGHTNINGBOLT, EV_MISSILE_HIT, EV_MISSILE_MISS, EV_MISSILE_MISS_METAL,
    EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE, EV_OBELISKPAIN, EV_OBITUARY, EV_PAIN,
    EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT, EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD,
    EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK, EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL,
    EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16, EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND,
    EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME, EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO,
    EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0, EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11,
    EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14, EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3,
    EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6, EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9,
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER,
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
use cg_ents::{
    CG_AddPacketEntities, CG_AdjustPositionForMover, CG_Beam, CG_PositionEntityOnTag,
    CG_PositionRotatedEntityOnTag, CG_SetEntitySoundPosition,
};
use cg_event::{CG_CheckEvents, CG_EntityEvent, CG_PainEvent, CG_PlaceString};
use cg_info::{CG_DrawInformation, CG_LoadingClient, CG_LoadingItem, CG_LoadingString};
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
    byte, cplane_s, cplane_t, playerState_s, playerState_t, qboolean, qfalse, qtrue, trace_t,
    usercmd_s, usercmd_t, vec3_t, vec_t, Com_Printf,
};

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
#[no_mangle]
pub unsafe extern "C" fn PM_SlideMove(mut gravity: qboolean) -> qboolean {
    let mut bumpcount: libc::c_int = 0;
    let mut numbumps: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut numplanes: libc::c_int = 0;
    let mut planes: [vec3_t; 5] = [[0.; 3]; 5];
    let mut primal_velocity: vec3_t = [0.; 3];
    let mut clipVelocity: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
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
    let mut end: vec3_t = [0.; 3];
    let mut time_left: libc::c_float = 0.;
    let mut into: libc::c_float = 0.;
    let mut endVelocity: vec3_t = [0.; 3];
    let mut endClipVelocity: vec3_t = [0.; 3];
    numbumps = 4i32;
    primal_velocity[0usize] = (*(*pm).ps).velocity[0usize];
    primal_velocity[1usize] = (*(*pm).ps).velocity[1usize];
    primal_velocity[2usize] = (*(*pm).ps).velocity[2usize];
    if 0 != gravity as u64 {
        endVelocity[0usize] = (*(*pm).ps).velocity[0usize];
        endVelocity[1usize] = (*(*pm).ps).velocity[1usize];
        endVelocity[2usize] = (*(*pm).ps).velocity[2usize];
        endVelocity[2usize] -= (*(*pm).ps).gravity as libc::c_float * pml.frametime;
        (*(*pm).ps).velocity[2usize] = (((*(*pm).ps).velocity[2usize] + endVelocity[2usize])
            as libc::c_double
            * 0.5f64) as vec_t;
        primal_velocity[2usize] = endVelocity[2usize];
        if 0 != pml.groundPlane as u64 {
            PM_ClipVelocity(
                (*(*pm).ps).velocity.as_mut_ptr(),
                pml.groundTrace.plane.normal.as_mut_ptr(),
                (*(*pm).ps).velocity.as_mut_ptr(),
                1.001f32,
            );
        }
    }
    time_left = pml.frametime;
    if 0 != pml.groundPlane as u64 {
        numplanes = 1i32;
        planes[0usize][0usize] = pml.groundTrace.plane.normal[0usize];
        planes[0usize][1usize] = pml.groundTrace.plane.normal[1usize];
        planes[0usize][2usize] = pml.groundTrace.plane.normal[2usize]
    } else {
        numplanes = 0i32
    }
    VectorNormalize2(
        (*(*pm).ps).velocity.as_mut_ptr() as *const vec_t,
        planes[numplanes as usize].as_mut_ptr(),
    );
    numplanes += 1;
    bumpcount = 0i32;
    while bumpcount < numbumps {
        end[0usize] = (*(*pm).ps).origin[0usize] + (*(*pm).ps).velocity[0usize] * time_left;
        end[1usize] = (*(*pm).ps).origin[1usize] + (*(*pm).ps).velocity[1usize] * time_left;
        end[2usize] = (*(*pm).ps).origin[2usize] + (*(*pm).ps).velocity[2usize] * time_left;
        (*pm).trace.expect("non-null function pointer")(
            &mut trace,
            (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
            (*pm).mins.as_mut_ptr() as *const vec_t,
            (*pm).maxs.as_mut_ptr() as *const vec_t,
            end.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).clientNum,
            (*pm).tracemask,
        );
        if 0 != trace.allsolid as u64 {
            (*(*pm).ps).velocity[2usize] = 0i32 as vec_t;
            return qtrue;
        }
        if trace.fraction > 0i32 as libc::c_float {
            (*(*pm).ps).origin[0usize] = trace.endpos[0usize];
            (*(*pm).ps).origin[1usize] = trace.endpos[1usize];
            (*(*pm).ps).origin[2usize] = trace.endpos[2usize]
        }
        if trace.fraction == 1i32 as libc::c_float {
            // moved the entire distance
            break;
        } else {
            PM_AddTouchEnt(trace.entityNum);
            time_left -= time_left * trace.fraction;
            if numplanes >= 5i32 {
                (*(*pm).ps).velocity[2usize] = 0i32 as vec_t;
                (*(*pm).ps).velocity[1usize] = (*(*pm).ps).velocity[2usize];
                (*(*pm).ps).velocity[0usize] = (*(*pm).ps).velocity[1usize];
                return qtrue;
            }
            i = 0i32;
            while i < numplanes {
                if (trace.plane.normal[0usize] * planes[i as usize][0usize]
                    + trace.plane.normal[1usize] * planes[i as usize][1usize]
                    + trace.plane.normal[2usize] * planes[i as usize][2usize])
                    as libc::c_double
                    > 0.99f64
                {
                    (*(*pm).ps).velocity[0usize] =
                        trace.plane.normal[0usize] + (*(*pm).ps).velocity[0usize];
                    (*(*pm).ps).velocity[1usize] =
                        trace.plane.normal[1usize] + (*(*pm).ps).velocity[1usize];
                    (*(*pm).ps).velocity[2usize] =
                        trace.plane.normal[2usize] + (*(*pm).ps).velocity[2usize];
                    break;
                } else {
                    i += 1
                }
            }
            if !(i < numplanes) {
                planes[numplanes as usize][0usize] = trace.plane.normal[0usize];
                planes[numplanes as usize][1usize] = trace.plane.normal[1usize];
                planes[numplanes as usize][2usize] = trace.plane.normal[2usize];
                numplanes += 1;
                i = 0i32;
                while i < numplanes {
                    into = (*(*pm).ps).velocity[0usize] * planes[i as usize][0usize]
                        + (*(*pm).ps).velocity[1usize] * planes[i as usize][1usize]
                        + (*(*pm).ps).velocity[2usize] * planes[i as usize][2usize];
                    if into as libc::c_double >= 0.1f64 {
                        // move doesn't interact with the plane
                        i += 1
                    } else {
                        if -into > pml.impactSpeed {
                            pml.impactSpeed = -into
                        }
                        PM_ClipVelocity(
                            (*(*pm).ps).velocity.as_mut_ptr(),
                            planes[i as usize].as_mut_ptr(),
                            clipVelocity.as_mut_ptr(),
                            1.001f32,
                        );
                        if 0 != gravity as u64 {
                            PM_ClipVelocity(
                                endVelocity.as_mut_ptr(),
                                planes[i as usize].as_mut_ptr(),
                                endClipVelocity.as_mut_ptr(),
                                1.001f32,
                            );
                        }
                        j = 0i32;
                        while j < numplanes {
                            if !(j == i) {
                                if !((clipVelocity[0usize] * planes[j as usize][0usize]
                                    + clipVelocity[1usize] * planes[j as usize][1usize]
                                    + clipVelocity[2usize] * planes[j as usize][2usize])
                                    as libc::c_double
                                    >= 0.1f64)
                                {
                                    // move doesn't interact with the plane
                                    PM_ClipVelocity(
                                        clipVelocity.as_mut_ptr(),
                                        planes[j as usize].as_mut_ptr(),
                                        clipVelocity.as_mut_ptr(),
                                        1.001f32,
                                    );
                                    if 0 != gravity as u64 {
                                        PM_ClipVelocity(
                                            endClipVelocity.as_mut_ptr(),
                                            planes[j as usize].as_mut_ptr(),
                                            endClipVelocity.as_mut_ptr(),
                                            1.001f32,
                                        );
                                    }
                                    // see if it goes back into the first clip plane
                                    if !(clipVelocity[0usize] * planes[i as usize][0usize]
                                        + clipVelocity[1usize] * planes[i as usize][1usize]
                                        + clipVelocity[2usize] * planes[i as usize][2usize]
                                        >= 0i32 as libc::c_float)
                                    {
                                        CrossProduct(
                                            planes[i as usize].as_mut_ptr() as *const vec_t,
                                            planes[j as usize].as_mut_ptr() as *const vec_t,
                                            dir.as_mut_ptr(),
                                        );
                                        VectorNormalize(dir.as_mut_ptr());
                                        d = dir[0usize] * (*(*pm).ps).velocity[0usize]
                                            + dir[1usize] * (*(*pm).ps).velocity[1usize]
                                            + dir[2usize] * (*(*pm).ps).velocity[2usize];
                                        clipVelocity[0usize] = dir[0usize] * d;
                                        clipVelocity[1usize] = dir[1usize] * d;
                                        clipVelocity[2usize] = dir[2usize] * d;
                                        if 0 != gravity as u64 {
                                            CrossProduct(
                                                planes[i as usize].as_mut_ptr() as *const vec_t,
                                                planes[j as usize].as_mut_ptr() as *const vec_t,
                                                dir.as_mut_ptr(),
                                            );
                                            VectorNormalize(dir.as_mut_ptr());
                                            d = dir[0usize] * endVelocity[0usize]
                                                + dir[1usize] * endVelocity[1usize]
                                                + dir[2usize] * endVelocity[2usize];
                                            endClipVelocity[0usize] = dir[0usize] * d;
                                            endClipVelocity[1usize] = dir[1usize] * d;
                                            endClipVelocity[2usize] = dir[2usize] * d
                                        }
                                        k = 0i32;
                                        while k < numplanes {
                                            if !(k == i || k == j) {
                                                if !((clipVelocity[0usize]
                                                    * planes[k as usize][0usize]
                                                    + clipVelocity[1usize]
                                                        * planes[k as usize][1usize]
                                                    + clipVelocity[2usize]
                                                        * planes[k as usize][2usize])
                                                    as libc::c_double
                                                    >= 0.1f64)
                                                {
                                                    // move doesn't interact with the plane
                                                    (*(*pm).ps).velocity[2usize] = 0i32 as vec_t;
                                                    (*(*pm).ps).velocity[1usize] =
                                                        (*(*pm).ps).velocity[2usize];
                                                    (*(*pm).ps).velocity[0usize] =
                                                        (*(*pm).ps).velocity[1usize];
                                                    return qtrue;
                                                }
                                            }
                                            k += 1
                                        }
                                    }
                                }
                            }
                            j += 1
                        }
                        (*(*pm).ps).velocity[0usize] = clipVelocity[0usize];
                        (*(*pm).ps).velocity[1usize] = clipVelocity[1usize];
                        (*(*pm).ps).velocity[2usize] = clipVelocity[2usize];
                        if 0 != gravity as u64 {
                            endVelocity[0usize] = endClipVelocity[0usize];
                            endVelocity[1usize] = endClipVelocity[1usize];
                            endVelocity[2usize] = endClipVelocity[2usize]
                        }
                        break;
                    }
                }
            }
            bumpcount += 1
        }
    }
    if 0 != gravity as u64 {
        (*(*pm).ps).velocity[0usize] = endVelocity[0usize];
        (*(*pm).ps).velocity[1usize] = endVelocity[1usize];
        (*(*pm).ps).velocity[2usize] = endVelocity[2usize]
    }
    if 0 != (*(*pm).ps).pm_time {
        (*(*pm).ps).velocity[0usize] = primal_velocity[0usize];
        (*(*pm).ps).velocity[1usize] = primal_velocity[1usize];
        (*(*pm).ps).velocity[2usize] = primal_velocity[2usize]
    }
    return (bumpcount != 0i32) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn PM_StepSlideMove(mut gravity: qboolean) {
    let mut start_o: vec3_t = [0.; 3];
    let mut start_v: vec3_t = [0.; 3];
    //	vec3_t		down_o, down_v;
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
    //	float		down_dist, up_dist;
    //	vec3_t		delta, delta2;
    let mut up: vec3_t = [0.; 3];
    let mut down: vec3_t = [0.; 3];
    let mut stepSize: libc::c_float = 0.;
    start_o[0usize] = (*(*pm).ps).origin[0usize];
    start_o[1usize] = (*(*pm).ps).origin[1usize];
    start_o[2usize] = (*(*pm).ps).origin[2usize];
    start_v[0usize] = (*(*pm).ps).velocity[0usize];
    start_v[1usize] = (*(*pm).ps).velocity[1usize];
    start_v[2usize] = (*(*pm).ps).velocity[2usize];
    if PM_SlideMove(gravity) as libc::c_uint == 0i32 as libc::c_uint {
        return;
    }
    down[0usize] = start_o[0usize];
    down[1usize] = start_o[1usize];
    down[2usize] = start_o[2usize];
    down[2usize] -= 18i32 as libc::c_float;
    (*pm).trace.expect("non-null function pointer")(
        &mut trace,
        start_o.as_mut_ptr() as *const vec_t,
        (*pm).mins.as_mut_ptr() as *const vec_t,
        (*pm).maxs.as_mut_ptr() as *const vec_t,
        down.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
        (*pm).tracemask,
    );
    up[0usize] = 0i32 as vec_t;
    up[1usize] = 0i32 as vec_t;
    up[2usize] = 1i32 as vec_t;
    if (*(*pm).ps).velocity[2usize] > 0i32 as libc::c_float
        && (trace.fraction as libc::c_double == 1.0f64
            || ((trace.plane.normal[0usize] * up[0usize]
                + trace.plane.normal[1usize] * up[1usize]
                + trace.plane.normal[2usize] * up[2usize]) as libc::c_double)
                < 0.7f64)
    {
        return;
    }
    up[0usize] = start_o[0usize];
    up[1usize] = start_o[1usize];
    up[2usize] = start_o[2usize];
    up[2usize] += 18i32 as libc::c_float;
    (*pm).trace.expect("non-null function pointer")(
        &mut trace,
        start_o.as_mut_ptr() as *const vec_t,
        (*pm).mins.as_mut_ptr() as *const vec_t,
        (*pm).maxs.as_mut_ptr() as *const vec_t,
        up.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
        (*pm).tracemask,
    );
    if 0 != trace.allsolid as u64 {
        if 0 != (*pm).debugLevel {
            Com_Printf(
                b"%i:bend can\'t step\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        return;
    }
    stepSize = trace.endpos[2usize] - start_o[2usize];
    (*(*pm).ps).origin[0usize] = trace.endpos[0usize];
    (*(*pm).ps).origin[1usize] = trace.endpos[1usize];
    (*(*pm).ps).origin[2usize] = trace.endpos[2usize];
    (*(*pm).ps).velocity[0usize] = start_v[0usize];
    (*(*pm).ps).velocity[1usize] = start_v[1usize];
    (*(*pm).ps).velocity[2usize] = start_v[2usize];
    PM_SlideMove(gravity);
    down[0usize] = (*(*pm).ps).origin[0usize];
    down[1usize] = (*(*pm).ps).origin[1usize];
    down[2usize] = (*(*pm).ps).origin[2usize];
    down[2usize] -= stepSize;
    (*pm).trace.expect("non-null function pointer")(
        &mut trace,
        (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
        (*pm).mins.as_mut_ptr() as *const vec_t,
        (*pm).maxs.as_mut_ptr() as *const vec_t,
        down.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
        (*pm).tracemask,
    );
    if 0 == trace.allsolid as u64 {
        (*(*pm).ps).origin[0usize] = trace.endpos[0usize];
        (*(*pm).ps).origin[1usize] = trace.endpos[1usize];
        (*(*pm).ps).origin[2usize] = trace.endpos[2usize]
    }
    if (trace.fraction as libc::c_double) < 1.0f64 {
        PM_ClipVelocity(
            (*(*pm).ps).velocity.as_mut_ptr(),
            trace.plane.normal.as_mut_ptr(),
            (*(*pm).ps).velocity.as_mut_ptr(),
            1.001f32,
        );
    }
    let mut delta: libc::c_float = 0.;
    delta = (*(*pm).ps).origin[2usize] - start_o[2usize];
    if delta > 2i32 as libc::c_float {
        if delta < 7i32 as libc::c_float {
            PM_AddEvent(EV_STEP_4 as libc::c_int);
        } else if delta < 11i32 as libc::c_float {
            PM_AddEvent(EV_STEP_8 as libc::c_int);
        } else if delta < 15i32 as libc::c_float {
            PM_AddEvent(EV_STEP_12 as libc::c_int);
        } else {
            PM_AddEvent(EV_STEP_16 as libc::c_int);
        }
    }
    if 0 != (*pm).debugLevel {
        Com_Printf(
            b"%i:stepped\n\x00" as *const u8 as *const libc::c_char,
            c_pmove,
        );
    };
}
