use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, GENDER_FEMALE, GENDER_MALE,
    GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED,
    TEAM_SPECTATOR,
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
    trap_R_AddPolyToScene, trap_R_RegisterShader, FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH,
    FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL, FOOTSTEP_SPLASH, FOOTSTEP_TOTAL, LEBS_BLOOD,
    LEBS_BRASS, LEBS_NONE, LEMT_BLOOD, LEMT_BURN, LEMT_NONE, LE_EXPLOSION, LE_FADE_RGB,
    LE_FALL_SCALE_FADE, LE_FRAGMENT, LE_MARK, LE_MOVE_SCALE_FADE, LE_SCALE_FADE, LE_SCOREPLUM,
    LE_SPRITE_EXPLOSION,
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
use libc;
use q_math::{
    axisDefault, colorWhite, g_color_table, vec3_origin, vectoangles, AngleMod, AngleNormalize180,
    AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis, AxisClear, AxisCopy, ByteToDir,
    LerpAngle, MatrixMultiply, PerpendicularVector, Q_crandom, Q_random, RotateAroundDirection,
    RotatePointAroundVector, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, entityState_s, entityState_t, gameState_t, playerState_s,
    playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trace_t,
    trajectory_t, va, vec3_t, vec_t, COM_Parse, Q_stricmp, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atof, atoi, cos, floor, memset, rand, sin, sqrt};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, polyVert_t, refEntityType_t, refEntity_t,
    refdef_t, textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D,
    GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING,
    RT_MAX_REF_ENTITY_TYPE, RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS,
    RT_SPRITE, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
unsafe extern "C" fn Distance(mut p1: *const vec_t, mut p2: *const vec_t) -> vec_t {
    let mut v: vec3_t = [0.; 3];
    v[0usize] = *p2.offset(0isize) - *p1.offset(0isize);
    v[1usize] = *p2.offset(1isize) - *p1.offset(1isize);
    v[2usize] = *p2.offset(2isize) - *p1.offset(2isize);
    return VectorLength(v.as_mut_ptr() as *const vec_t);
}
#[no_mangle]
pub unsafe extern "C" fn CG_ClearParticles() {
    let mut i: libc::c_int = 0;
    memset(
        particles.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[cparticle_t; 1024]>() as libc::c_ulong,
    );
    free_particles = &mut particles[0usize] as *mut cparticle_t;
    active_particles = 0 as *mut cparticle_t;
    i = 0i32;
    while i < cl_numparticles - 1 {
        particles[i as usize].next = &mut particles[(i + 1i32) as usize] as *mut cparticle_t;
        particles[i as usize].type_0 = 0i32;
        i += 1
    }
    particles[(cl_numparticles - 1i32) as usize].next = 0 as *mut particle_s;
    oldtime = cg.time as libc::c_float;
    i = 0i32;
    while !shaderAnimNames[i as usize].is_null() {
        let mut j: libc::c_int = 0;
        j = 0i32;
        while j < shaderAnimCounts[i as usize] {
            shaderAnims[i as usize][j as usize] = trap_R_RegisterShader(va(
                b"%s%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                shaderAnimNames[i as usize],
                j + 1i32,
            ));
            j += 1
        }
        i += 1
    }
    numShaderAnims = i;
    initparticles = qtrue;
}
#[no_mangle]
pub static mut initparticles: qboolean = qfalse;
static mut numShaderAnims: libc::c_int = 0;
static mut shaderAnimNames: [*mut libc::c_char; 32] = [
    b"explode1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
static mut shaderAnims: [[qhandle_t; 64]; 32] = [[0; 64]; 32];
static mut shaderAnimCounts: [libc::c_int; 32] = [
    23i32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
];
#[no_mangle]
pub static mut oldtime: libc::c_float = 0.;
#[no_mangle]
pub static mut cl_numparticles: libc::c_int = 1024i32;
#[no_mangle]
pub static mut particles: [cparticle_t; 1024] = [particle_s {
    next: 0 as *const particle_s as *mut particle_s,
    time: 0.,
    endtime: 0.,
    org: [0.; 3],
    vel: [0.; 3],
    accel: [0.; 3],
    color: 0,
    colorvel: 0.,
    alpha: 0.,
    alphavel: 0.,
    type_0: 0,
    pshader: 0,
    height: 0.,
    width: 0.,
    endheight: 0.,
    endwidth: 0.,
    start: 0.,
    end: 0.,
    startfade: 0.,
    rotate: qfalse,
    snum: 0,
    link: qfalse,
    shaderAnim: 0,
    roll: 0,
    accumroll: 0,
}; 1024];
// done.
#[no_mangle]
pub static mut active_particles: *mut cparticle_t = 0 as *const cparticle_t as *mut cparticle_t;
#[no_mangle]
pub static mut free_particles: *mut cparticle_t = 0 as *const cparticle_t as *mut cparticle_t;
#[no_mangle]
pub unsafe extern "C" fn CG_AddParticles() {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut alpha: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut time2: libc::c_float = 0.;
    let mut org: vec3_t = [0.; 3];
    let mut active: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut tail: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut rotate_ang: vec3_t = [0.; 3];
    if 0 == initparticles as u64 {
        CG_ClearParticles();
    }
    vforward[0usize] = cg.refdef.viewaxis[0usize][0usize];
    vforward[1usize] = cg.refdef.viewaxis[0usize][1usize];
    vforward[2usize] = cg.refdef.viewaxis[0usize][2usize];
    vright[0usize] = cg.refdef.viewaxis[1usize][0usize];
    vright[1usize] = cg.refdef.viewaxis[1usize][1usize];
    vright[2usize] = cg.refdef.viewaxis[1usize][2usize];
    vup[0usize] = cg.refdef.viewaxis[2usize][0usize];
    vup[1usize] = cg.refdef.viewaxis[2usize][1usize];
    vup[2usize] = cg.refdef.viewaxis[2usize][2usize];
    vectoangles(
        cg.refdef.viewaxis[0usize].as_mut_ptr() as *const vec_t,
        rotate_ang.as_mut_ptr(),
    );
    roll = (roll as libc::c_double
        + (cg.time as libc::c_float - oldtime) as libc::c_double * 0.1f64)
        as libc::c_float;
    rotate_ang[2usize] =
        (rotate_ang[2usize] as libc::c_double + roll as libc::c_double * 0.9f64) as vec_t;
    AngleVectors(
        rotate_ang.as_mut_ptr() as *const vec_t,
        rforward.as_mut_ptr(),
        rright.as_mut_ptr(),
        rup.as_mut_ptr(),
    );
    oldtime = cg.time as libc::c_float;
    active = 0 as *mut cparticle_t;
    tail = 0 as *mut cparticle_t;
    let mut current_block_54: u64;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        time =
            ((cg.time as libc::c_float - (*p).time) as libc::c_double * 0.001f64) as libc::c_float;
        alpha = (*p).alpha + time * (*p).alphavel;
        if alpha <= 0i32 as libc::c_float {
            // faded out
            (*p).next = free_particles;
            free_particles = p;
            (*p).type_0 = 0i32;
            (*p).color = 0i32;
            (*p).alpha = 0i32 as libc::c_float
        } else {
            if (*p).type_0 == P_SMOKE as libc::c_int
                || (*p).type_0 == P_ANIM as libc::c_int
                || (*p).type_0 == P_BLEED as libc::c_int
                || (*p).type_0 == P_SMOKE_IMPACT as libc::c_int
            {
                if cg.time as libc::c_float > (*p).endtime {
                    (*p).next = free_particles;
                    free_particles = p;
                    (*p).type_0 = 0i32;
                    (*p).color = 0i32;
                    (*p).alpha = 0i32 as libc::c_float;
                    current_block_54 = 12599329904712511516;
                } else {
                    current_block_54 = 11459959175219260272;
                }
            } else {
                current_block_54 = 11459959175219260272;
            }
            match current_block_54 {
                12599329904712511516 => {}
                _ => {
                    if (*p).type_0 == P_WEATHER_FLURRY as libc::c_int {
                        if cg.time as libc::c_float > (*p).endtime {
                            (*p).next = free_particles;
                            free_particles = p;
                            (*p).type_0 = 0i32;
                            (*p).color = 0i32;
                            (*p).alpha = 0i32 as libc::c_float;
                            current_block_54 = 12599329904712511516;
                        } else {
                            current_block_54 = 5529461102203738653;
                        }
                    } else {
                        current_block_54 = 5529461102203738653;
                    }
                    match current_block_54 {
                        12599329904712511516 => {}
                        _ => {
                            if (*p).type_0 == P_FLAT_SCALEUP_FADE as libc::c_int {
                                if cg.time as libc::c_float > (*p).endtime {
                                    (*p).next = free_particles;
                                    free_particles = p;
                                    (*p).type_0 = 0i32;
                                    (*p).color = 0i32;
                                    (*p).alpha = 0i32 as libc::c_float;
                                    current_block_54 = 12599329904712511516;
                                } else {
                                    current_block_54 = 6717214610478484138;
                                }
                            } else {
                                current_block_54 = 6717214610478484138;
                            }
                            match current_block_54 {
                                12599329904712511516 => {}
                                _ => {
                                    if ((*p).type_0 == P_BAT as libc::c_int
                                        || (*p).type_0 == P_SPRITE as libc::c_int)
                                        && (*p).endtime < 0i32 as libc::c_float
                                    {
                                        CG_AddParticleToScene(p, (*p).org.as_mut_ptr(), alpha);
                                        (*p).next = free_particles;
                                        free_particles = p;
                                        (*p).type_0 = 0i32;
                                        (*p).color = 0i32;
                                        (*p).alpha = 0i32 as libc::c_float
                                    } else {
                                        (*p).next = 0 as *mut particle_s;
                                        if tail.is_null() {
                                            tail = p;
                                            active = tail
                                        } else {
                                            (*tail).next = p;
                                            tail = p
                                        }
                                        if alpha as libc::c_double > 1.0f64 {
                                            alpha = 1i32 as libc::c_float
                                        }
                                        time2 = time * time;
                                        org[0usize] = (*p).org[0usize]
                                            + (*p).vel[0usize] * time
                                            + (*p).accel[0usize] * time2;
                                        org[1usize] = (*p).org[1usize]
                                            + (*p).vel[1usize] * time
                                            + (*p).accel[1usize] * time2;
                                        org[2usize] = (*p).org[2usize]
                                            + (*p).vel[2usize] * time
                                            + (*p).accel[2usize] * time2;
                                        CG_AddParticleToScene(p, org.as_mut_ptr(), alpha);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        p = next
    }
    active_particles = active;
}
/*
=====================
CG_AddParticleToScene
=====================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_AddParticleToScene(
    mut p: *mut cparticle_t,
    mut org: *mut vec_t,
    mut alpha: libc::c_float,
) {
    let mut point: vec3_t = [0.; 3];
    let mut verts: [polyVert_t; 4] = [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    let mut width: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut time2: libc::c_float = 0.;
    let mut ratio: libc::c_float = 0.;
    let mut invratio: libc::c_float = 0.;
    let mut color: vec3_t = [0.; 3];
    let mut TRIverts: [polyVert_t; 3] = [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 3];
    let mut rright2: vec3_t = [0.; 3];
    let mut rup2: vec3_t = [0.; 3];
    if (*p).type_0 == P_WEATHER as libc::c_int
        || (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int
        || (*p).type_0 == P_WEATHER_FLURRY as libc::c_int
        || (*p).type_0 == P_BUBBLE as libc::c_int
        || (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int
    {
        if (*p).type_0 != P_WEATHER_FLURRY as libc::c_int {
            if (*p).type_0 == P_BUBBLE as libc::c_int
                || (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int
            {
                if *org.offset(2isize) > (*p).end {
                    (*p).time = cg.time as libc::c_float;
                    (*p).org[0usize] = *org.offset(0isize);
                    (*p).org[1usize] = *org.offset(1isize);
                    (*p).org[2usize] = *org.offset(2isize);
                    (*p).org[2usize] = ((*p).start as libc::c_double
                        + 2.0f64
                            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                                as libc::c_double
                                - 0.5f64)
                            * 4i32 as libc::c_double)
                        as vec_t;
                    if (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int {
                        (*p).vel[0usize] = (2.0f64
                            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                                as libc::c_double
                                - 0.5f64)
                            * 4i32 as libc::c_double)
                            as vec_t;
                        (*p).vel[1usize] = (2.0f64
                            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                                as libc::c_double
                                - 0.5f64)
                            * 4i32 as libc::c_double)
                            as vec_t
                    }
                }
            } else if *org.offset(2isize) < (*p).end {
                (*p).time = cg.time as libc::c_float;
                (*p).org[0usize] = *org.offset(0isize);
                (*p).org[1usize] = *org.offset(1isize);
                (*p).org[2usize] = *org.offset(2isize);
                while (*p).org[2usize] < (*p).end {
                    (*p).org[2usize] += (*p).start - (*p).end
                }
                if (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int {
                    (*p).vel[0usize] = (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64)
                        * 16i32 as libc::c_double) as vec_t;
                    (*p).vel[1usize] = (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64)
                        * 16i32 as libc::c_double) as vec_t
                }
            }
            if 0 == (*p).link as u64 {
                return;
            }
            (*p).alpha = 1i32 as libc::c_float
        }
        if Distance(
            (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
            org as *const vec_t,
        ) > 1024i32 as libc::c_float
        {
            return;
        }
        if (*p).type_0 == P_BUBBLE as libc::c_int
            || (*p).type_0 == P_BUBBLE_TURBULENT as libc::c_int
        {
            point[0usize] = *org.offset(0isize) + vup[0usize] * -(*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * -(*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * -(*p).height;
            point[0usize] = point[0usize] + vright[0usize] * -(*p).width;
            point[1usize] = point[1usize] + vright[1usize] * -(*p).width;
            point[2usize] = point[2usize] + vright[2usize] * -(*p).width;
            verts[0usize].xyz[0usize] = point[0usize];
            verts[0usize].xyz[1usize] = point[1usize];
            verts[0usize].xyz[2usize] = point[2usize];
            verts[0usize].st[0usize] = 0i32 as libc::c_float;
            verts[0usize].st[1usize] = 0i32 as libc::c_float;
            verts[0usize].modulate[0usize] = 255i32 as byte;
            verts[0usize].modulate[1usize] = 255i32 as byte;
            verts[0usize].modulate[2usize] = 255i32 as byte;
            verts[0usize].modulate[3usize] = (255i32 as libc::c_float * (*p).alpha) as byte;
            point[0usize] = *org.offset(0isize) + vup[0usize] * -(*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * -(*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * -(*p).height;
            point[0usize] = point[0usize] + vright[0usize] * (*p).width;
            point[1usize] = point[1usize] + vright[1usize] * (*p).width;
            point[2usize] = point[2usize] + vright[2usize] * (*p).width;
            verts[1usize].xyz[0usize] = point[0usize];
            verts[1usize].xyz[1usize] = point[1usize];
            verts[1usize].xyz[2usize] = point[2usize];
            verts[1usize].st[0usize] = 0i32 as libc::c_float;
            verts[1usize].st[1usize] = 1i32 as libc::c_float;
            verts[1usize].modulate[0usize] = 255i32 as byte;
            verts[1usize].modulate[1usize] = 255i32 as byte;
            verts[1usize].modulate[2usize] = 255i32 as byte;
            verts[1usize].modulate[3usize] = (255i32 as libc::c_float * (*p).alpha) as byte;
            point[0usize] = *org.offset(0isize) + vup[0usize] * (*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * (*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * (*p).height;
            point[0usize] = point[0usize] + vright[0usize] * (*p).width;
            point[1usize] = point[1usize] + vright[1usize] * (*p).width;
            point[2usize] = point[2usize] + vright[2usize] * (*p).width;
            verts[2usize].xyz[0usize] = point[0usize];
            verts[2usize].xyz[1usize] = point[1usize];
            verts[2usize].xyz[2usize] = point[2usize];
            verts[2usize].st[0usize] = 1i32 as libc::c_float;
            verts[2usize].st[1usize] = 1i32 as libc::c_float;
            verts[2usize].modulate[0usize] = 255i32 as byte;
            verts[2usize].modulate[1usize] = 255i32 as byte;
            verts[2usize].modulate[2usize] = 255i32 as byte;
            verts[2usize].modulate[3usize] = (255i32 as libc::c_float * (*p).alpha) as byte;
            point[0usize] = *org.offset(0isize) + vup[0usize] * (*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * (*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * (*p).height;
            point[0usize] = point[0usize] + vright[0usize] * -(*p).width;
            point[1usize] = point[1usize] + vright[1usize] * -(*p).width;
            point[2usize] = point[2usize] + vright[2usize] * -(*p).width;
            verts[3usize].xyz[0usize] = point[0usize];
            verts[3usize].xyz[1usize] = point[1usize];
            verts[3usize].xyz[2usize] = point[2usize];
            verts[3usize].st[0usize] = 1i32 as libc::c_float;
            verts[3usize].st[1usize] = 0i32 as libc::c_float;
            verts[3usize].modulate[0usize] = 255i32 as byte;
            verts[3usize].modulate[1usize] = 255i32 as byte;
            verts[3usize].modulate[2usize] = 255i32 as byte;
            verts[3usize].modulate[3usize] = (255i32 as libc::c_float * (*p).alpha) as byte
        } else {
            point[0usize] = *org.offset(0isize) + vup[0usize] * -(*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * -(*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * -(*p).height;
            point[0usize] = point[0usize] + vright[0usize] * -(*p).width;
            point[1usize] = point[1usize] + vright[1usize] * -(*p).width;
            point[2usize] = point[2usize] + vright[2usize] * -(*p).width;
            TRIverts[0usize].xyz[0usize] = point[0usize];
            TRIverts[0usize].xyz[1usize] = point[1usize];
            TRIverts[0usize].xyz[2usize] = point[2usize];
            TRIverts[0usize].st[0usize] = 1i32 as libc::c_float;
            TRIverts[0usize].st[1usize] = 0i32 as libc::c_float;
            TRIverts[0usize].modulate[0usize] = 255i32 as byte;
            TRIverts[0usize].modulate[1usize] = 255i32 as byte;
            TRIverts[0usize].modulate[2usize] = 255i32 as byte;
            TRIverts[0usize].modulate[3usize] = (255i32 as libc::c_float * (*p).alpha) as byte;
            point[0usize] = *org.offset(0isize) + vup[0usize] * (*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * (*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * (*p).height;
            point[0usize] = point[0usize] + vright[0usize] * -(*p).width;
            point[1usize] = point[1usize] + vright[1usize] * -(*p).width;
            point[2usize] = point[2usize] + vright[2usize] * -(*p).width;
            TRIverts[1usize].xyz[0usize] = point[0usize];
            TRIverts[1usize].xyz[1usize] = point[1usize];
            TRIverts[1usize].xyz[2usize] = point[2usize];
            TRIverts[1usize].st[0usize] = 0i32 as libc::c_float;
            TRIverts[1usize].st[1usize] = 0i32 as libc::c_float;
            TRIverts[1usize].modulate[0usize] = 255i32 as byte;
            TRIverts[1usize].modulate[1usize] = 255i32 as byte;
            TRIverts[1usize].modulate[2usize] = 255i32 as byte;
            TRIverts[1usize].modulate[3usize] = (255i32 as libc::c_float * (*p).alpha) as byte;
            point[0usize] = *org.offset(0isize) + vup[0usize] * (*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * (*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * (*p).height;
            point[0usize] = point[0usize] + vright[0usize] * (*p).width;
            point[1usize] = point[1usize] + vright[1usize] * (*p).width;
            point[2usize] = point[2usize] + vright[2usize] * (*p).width;
            TRIverts[2usize].xyz[0usize] = point[0usize];
            TRIverts[2usize].xyz[1usize] = point[1usize];
            TRIverts[2usize].xyz[2usize] = point[2usize];
            TRIverts[2usize].st[0usize] = 0i32 as libc::c_float;
            TRIverts[2usize].st[1usize] = 1i32 as libc::c_float;
            TRIverts[2usize].modulate[0usize] = 255i32 as byte;
            TRIverts[2usize].modulate[1usize] = 255i32 as byte;
            TRIverts[2usize].modulate[2usize] = 255i32 as byte;
            TRIverts[2usize].modulate[3usize] = (255i32 as libc::c_float * (*p).alpha) as byte
        }
    } else if (*p).type_0 == P_SPRITE as libc::c_int {
        let mut rr: vec3_t = [0.; 3];
        let mut ru: vec3_t = [0.; 3];
        let mut rotate_ang: vec3_t = [0.; 3];
        color[0usize] = 1.0f64 as vec_t;
        color[1usize] = 1.0f64 as vec_t;
        color[2usize] = 0.5f64 as vec_t;
        time = cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if 0 != (*p).roll {
            vectoangles(
                cg.refdef.viewaxis[0usize].as_mut_ptr() as *const vec_t,
                rotate_ang.as_mut_ptr(),
            );
            rotate_ang[2usize] += (*p).roll as libc::c_float;
            AngleVectors(
                rotate_ang.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rr.as_mut_ptr(),
                ru.as_mut_ptr(),
            );
        }
        if 0 != (*p).roll {
            point[0usize] = *org.offset(0isize) + ru[0usize] * -height;
            point[1usize] = *org.offset(1isize) + ru[1usize] * -height;
            point[2usize] = *org.offset(2isize) + ru[2usize] * -height;
            point[0usize] = point[0usize] + rr[0usize] * -width;
            point[1usize] = point[1usize] + rr[1usize] * -width;
            point[2usize] = point[2usize] + rr[2usize] * -width
        } else {
            point[0usize] = *org.offset(0isize) + vup[0usize] * -height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * -height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * -height;
            point[0usize] = point[0usize] + vright[0usize] * -width;
            point[1usize] = point[1usize] + vright[1usize] * -width;
            point[2usize] = point[2usize] + vright[2usize] * -width
        }
        verts[0usize].xyz[0usize] = point[0usize];
        verts[0usize].xyz[1usize] = point[1usize];
        verts[0usize].xyz[2usize] = point[2usize];
        verts[0usize].st[0usize] = 0i32 as libc::c_float;
        verts[0usize].st[1usize] = 0i32 as libc::c_float;
        verts[0usize].modulate[0usize] = 255i32 as byte;
        verts[0usize].modulate[1usize] = 255i32 as byte;
        verts[0usize].modulate[2usize] = 255i32 as byte;
        verts[0usize].modulate[3usize] = 255i32 as byte;
        if 0 != (*p).roll {
            point[0usize] = point[0usize] + ru[0usize] * (2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + ru[1usize] * (2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + ru[2usize] * (2i32 as libc::c_float * height)
        } else {
            point[0usize] = point[0usize] + vup[0usize] * (2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + vup[1usize] * (2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + vup[2usize] * (2i32 as libc::c_float * height)
        }
        verts[1usize].xyz[0usize] = point[0usize];
        verts[1usize].xyz[1usize] = point[1usize];
        verts[1usize].xyz[2usize] = point[2usize];
        verts[1usize].st[0usize] = 0i32 as libc::c_float;
        verts[1usize].st[1usize] = 1i32 as libc::c_float;
        verts[1usize].modulate[0usize] = 255i32 as byte;
        verts[1usize].modulate[1usize] = 255i32 as byte;
        verts[1usize].modulate[2usize] = 255i32 as byte;
        verts[1usize].modulate[3usize] = 255i32 as byte;
        if 0 != (*p).roll {
            point[0usize] = point[0usize] + rr[0usize] * (2i32 as libc::c_float * width);
            point[1usize] = point[1usize] + rr[1usize] * (2i32 as libc::c_float * width);
            point[2usize] = point[2usize] + rr[2usize] * (2i32 as libc::c_float * width)
        } else {
            point[0usize] = point[0usize] + vright[0usize] * (2i32 as libc::c_float * width);
            point[1usize] = point[1usize] + vright[1usize] * (2i32 as libc::c_float * width);
            point[2usize] = point[2usize] + vright[2usize] * (2i32 as libc::c_float * width)
        }
        verts[2usize].xyz[0usize] = point[0usize];
        verts[2usize].xyz[1usize] = point[1usize];
        verts[2usize].xyz[2usize] = point[2usize];
        verts[2usize].st[0usize] = 1i32 as libc::c_float;
        verts[2usize].st[1usize] = 1i32 as libc::c_float;
        verts[2usize].modulate[0usize] = 255i32 as byte;
        verts[2usize].modulate[1usize] = 255i32 as byte;
        verts[2usize].modulate[2usize] = 255i32 as byte;
        verts[2usize].modulate[3usize] = 255i32 as byte;
        if 0 != (*p).roll {
            point[0usize] = point[0usize] + ru[0usize] * (-2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + ru[1usize] * (-2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + ru[2usize] * (-2i32 as libc::c_float * height)
        } else {
            point[0usize] = point[0usize] + vup[0usize] * (-2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + vup[1usize] * (-2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + vup[2usize] * (-2i32 as libc::c_float * height)
        }
        verts[3usize].xyz[0usize] = point[0usize];
        verts[3usize].xyz[1usize] = point[1usize];
        verts[3usize].xyz[2usize] = point[2usize];
        verts[3usize].st[0usize] = 1i32 as libc::c_float;
        verts[3usize].st[1usize] = 0i32 as libc::c_float;
        verts[3usize].modulate[0usize] = 255i32 as byte;
        verts[3usize].modulate[1usize] = 255i32 as byte;
        verts[3usize].modulate[2usize] = 255i32 as byte;
        verts[3usize].modulate[3usize] = 255i32 as byte
    } else if (*p).type_0 == P_SMOKE as libc::c_int || (*p).type_0 == P_SMOKE_IMPACT as libc::c_int
    {
        if (*p).type_0 == P_SMOKE_IMPACT as libc::c_int
            && Distance(
                (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
                org as *const vec_t,
            ) > 1024i32 as libc::c_float
        {
            return;
        }
        if (*p).color == 2i32 {
            color[0usize] = 0.22f32;
            color[1usize] = 0.0f32;
            color[2usize] = 0.0f32
        } else if (*p).color == 4i32 {
            let mut len: libc::c_float = 0.;
            let mut greyit: libc::c_float = 0.;
            let mut val: libc::c_float = 0.;
            len = Distance(
                (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
                org as *const vec_t,
            );
            if 0. == len {
                len = 1i32 as libc::c_float
            }
            val = 4096i32 as libc::c_float / len;
            greyit = (0.25f64 * val as libc::c_double) as libc::c_float;
            if greyit as libc::c_double > 0.5f64 {
                greyit = 0.5f64 as libc::c_float
            }
            color[0usize] = greyit;
            color[1usize] = greyit;
            color[2usize] = greyit
        } else {
            color[0usize] = 1.0f64 as vec_t;
            color[1usize] = 1.0f64 as vec_t;
            color[2usize] = 1.0f64 as vec_t
        }
        time = cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        if cg.time as libc::c_float > (*p).startfade {
            invratio = 1i32 as libc::c_float
                - (cg.time as libc::c_float - (*p).startfade) / ((*p).endtime - (*p).startfade);
            if (*p).color == 3i32 {
                let mut fval: libc::c_float = 0.;
                fval = invratio * invratio;
                if fval < 0i32 as libc::c_float {
                    fval = 0i32 as libc::c_float
                }
                color[0usize] = fval;
                color[1usize] = fval;
                color[2usize] = fval
            }
            invratio *= (*p).alpha
        } else {
            invratio = 1i32 as libc::c_float * (*p).alpha
        }
        if cgs.glconfig.hardwareType as libc::c_uint == GLHW_RAGEPRO as libc::c_int as libc::c_uint
        {
            invratio = 1i32 as libc::c_float
        }
        if invratio > 1i32 as libc::c_float {
            invratio = 1i32 as libc::c_float
        }
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (*p).type_0 != P_SMOKE_IMPACT as libc::c_int {
            let mut temp: vec3_t = [0.; 3];
            vectoangles(rforward.as_mut_ptr() as *const vec_t, temp.as_mut_ptr());
            (*p).accumroll += (*p).roll;
            temp[2usize] = (temp[2usize] as libc::c_double
                + (*p).accumroll as libc::c_double * 0.1f64) as vec_t;
            AngleVectors(
                temp.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rright2.as_mut_ptr(),
                rup2.as_mut_ptr(),
            );
        } else {
            rright2[0usize] = rright[0usize];
            rright2[1usize] = rright[1usize];
            rright2[2usize] = rright[2usize];
            rup2[0usize] = rup[0usize];
            rup2[1usize] = rup[1usize];
            rup2[2usize] = rup[2usize]
        }
        if 0 != (*p).rotate as u64 {
            point[0usize] = *org.offset(0isize) + rup2[0usize] * -height;
            point[1usize] = *org.offset(1isize) + rup2[1usize] * -height;
            point[2usize] = *org.offset(2isize) + rup2[2usize] * -height;
            point[0usize] = point[0usize] + rright2[0usize] * -width;
            point[1usize] = point[1usize] + rright2[1usize] * -width;
            point[2usize] = point[2usize] + rright2[2usize] * -width
        } else {
            point[0usize] = *org.offset(0isize) + vup[0usize] * -(*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * -(*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * -(*p).height;
            point[0usize] = point[0usize] + vright[0usize] * -(*p).width;
            point[1usize] = point[1usize] + vright[1usize] * -(*p).width;
            point[2usize] = point[2usize] + vright[2usize] * -(*p).width
        }
        verts[0usize].xyz[0usize] = point[0usize];
        verts[0usize].xyz[1usize] = point[1usize];
        verts[0usize].xyz[2usize] = point[2usize];
        verts[0usize].st[0usize] = 0i32 as libc::c_float;
        verts[0usize].st[1usize] = 0i32 as libc::c_float;
        verts[0usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[0usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[0usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[0usize].modulate[3usize] = (255i32 as libc::c_float * invratio) as byte;
        if 0 != (*p).rotate as u64 {
            point[0usize] = *org.offset(0isize) + rup2[0usize] * -height;
            point[1usize] = *org.offset(1isize) + rup2[1usize] * -height;
            point[2usize] = *org.offset(2isize) + rup2[2usize] * -height;
            point[0usize] = point[0usize] + rright2[0usize] * width;
            point[1usize] = point[1usize] + rright2[1usize] * width;
            point[2usize] = point[2usize] + rright2[2usize] * width
        } else {
            point[0usize] = *org.offset(0isize) + vup[0usize] * -(*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * -(*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * -(*p).height;
            point[0usize] = point[0usize] + vright[0usize] * (*p).width;
            point[1usize] = point[1usize] + vright[1usize] * (*p).width;
            point[2usize] = point[2usize] + vright[2usize] * (*p).width
        }
        verts[1usize].xyz[0usize] = point[0usize];
        verts[1usize].xyz[1usize] = point[1usize];
        verts[1usize].xyz[2usize] = point[2usize];
        verts[1usize].st[0usize] = 0i32 as libc::c_float;
        verts[1usize].st[1usize] = 1i32 as libc::c_float;
        verts[1usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[1usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[1usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[1usize].modulate[3usize] = (255i32 as libc::c_float * invratio) as byte;
        if 0 != (*p).rotate as u64 {
            point[0usize] = *org.offset(0isize) + rup2[0usize] * height;
            point[1usize] = *org.offset(1isize) + rup2[1usize] * height;
            point[2usize] = *org.offset(2isize) + rup2[2usize] * height;
            point[0usize] = point[0usize] + rright2[0usize] * width;
            point[1usize] = point[1usize] + rright2[1usize] * width;
            point[2usize] = point[2usize] + rright2[2usize] * width
        } else {
            point[0usize] = *org.offset(0isize) + vup[0usize] * (*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * (*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * (*p).height;
            point[0usize] = point[0usize] + vright[0usize] * (*p).width;
            point[1usize] = point[1usize] + vright[1usize] * (*p).width;
            point[2usize] = point[2usize] + vright[2usize] * (*p).width
        }
        verts[2usize].xyz[0usize] = point[0usize];
        verts[2usize].xyz[1usize] = point[1usize];
        verts[2usize].xyz[2usize] = point[2usize];
        verts[2usize].st[0usize] = 1i32 as libc::c_float;
        verts[2usize].st[1usize] = 1i32 as libc::c_float;
        verts[2usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[2usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[2usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[2usize].modulate[3usize] = (255i32 as libc::c_float * invratio) as byte;
        if 0 != (*p).rotate as u64 {
            point[0usize] = *org.offset(0isize) + rup2[0usize] * height;
            point[1usize] = *org.offset(1isize) + rup2[1usize] * height;
            point[2usize] = *org.offset(2isize) + rup2[2usize] * height;
            point[0usize] = point[0usize] + rright2[0usize] * -width;
            point[1usize] = point[1usize] + rright2[1usize] * -width;
            point[2usize] = point[2usize] + rright2[2usize] * -width
        } else {
            point[0usize] = *org.offset(0isize) + vup[0usize] * (*p).height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * (*p).height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * (*p).height;
            point[0usize] = point[0usize] + vright[0usize] * -(*p).width;
            point[1usize] = point[1usize] + vright[1usize] * -(*p).width;
            point[2usize] = point[2usize] + vright[2usize] * -(*p).width
        }
        verts[3usize].xyz[0usize] = point[0usize];
        verts[3usize].xyz[1usize] = point[1usize];
        verts[3usize].xyz[2usize] = point[2usize];
        verts[3usize].st[0usize] = 1i32 as libc::c_float;
        verts[3usize].st[1usize] = 0i32 as libc::c_float;
        verts[3usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[3usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[3usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[3usize].modulate[3usize] = (255i32 as libc::c_float * invratio) as byte
    } else if (*p).type_0 == P_BLEED as libc::c_int {
        let mut rr_0: vec3_t = [0.; 3];
        let mut ru_0: vec3_t = [0.; 3];
        let mut rotate_ang_0: vec3_t = [0.; 3];
        let mut alpha_0: libc::c_float = 0.;
        alpha_0 = (*p).alpha;
        if cgs.glconfig.hardwareType as libc::c_uint == GLHW_RAGEPRO as libc::c_int as libc::c_uint
        {
            alpha_0 = 1i32 as libc::c_float
        }
        if 0 != (*p).roll {
            vectoangles(
                cg.refdef.viewaxis[0usize].as_mut_ptr() as *const vec_t,
                rotate_ang_0.as_mut_ptr(),
            );
            rotate_ang_0[2usize] += (*p).roll as libc::c_float;
            AngleVectors(
                rotate_ang_0.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rr_0.as_mut_ptr(),
                ru_0.as_mut_ptr(),
            );
        } else {
            ru_0[0usize] = vup[0usize];
            ru_0[1usize] = vup[1usize];
            ru_0[2usize] = vup[2usize];
            rr_0[0usize] = vright[0usize];
            rr_0[1usize] = vright[1usize];
            rr_0[2usize] = vright[2usize]
        }
        point[0usize] = *org.offset(0isize) + ru_0[0usize] * -(*p).height;
        point[1usize] = *org.offset(1isize) + ru_0[1usize] * -(*p).height;
        point[2usize] = *org.offset(2isize) + ru_0[2usize] * -(*p).height;
        point[0usize] = point[0usize] + rr_0[0usize] * -(*p).width;
        point[1usize] = point[1usize] + rr_0[1usize] * -(*p).width;
        point[2usize] = point[2usize] + rr_0[2usize] * -(*p).width;
        verts[0usize].xyz[0usize] = point[0usize];
        verts[0usize].xyz[1usize] = point[1usize];
        verts[0usize].xyz[2usize] = point[2usize];
        verts[0usize].st[0usize] = 0i32 as libc::c_float;
        verts[0usize].st[1usize] = 0i32 as libc::c_float;
        verts[0usize].modulate[0usize] = 111i32 as byte;
        verts[0usize].modulate[1usize] = 19i32 as byte;
        verts[0usize].modulate[2usize] = 9i32 as byte;
        verts[0usize].modulate[3usize] = (255i32 as libc::c_float * alpha_0) as byte;
        point[0usize] = *org.offset(0isize) + ru_0[0usize] * -(*p).height;
        point[1usize] = *org.offset(1isize) + ru_0[1usize] * -(*p).height;
        point[2usize] = *org.offset(2isize) + ru_0[2usize] * -(*p).height;
        point[0usize] = point[0usize] + rr_0[0usize] * (*p).width;
        point[1usize] = point[1usize] + rr_0[1usize] * (*p).width;
        point[2usize] = point[2usize] + rr_0[2usize] * (*p).width;
        verts[1usize].xyz[0usize] = point[0usize];
        verts[1usize].xyz[1usize] = point[1usize];
        verts[1usize].xyz[2usize] = point[2usize];
        verts[1usize].st[0usize] = 0i32 as libc::c_float;
        verts[1usize].st[1usize] = 1i32 as libc::c_float;
        verts[1usize].modulate[0usize] = 111i32 as byte;
        verts[1usize].modulate[1usize] = 19i32 as byte;
        verts[1usize].modulate[2usize] = 9i32 as byte;
        verts[1usize].modulate[3usize] = (255i32 as libc::c_float * alpha_0) as byte;
        point[0usize] = *org.offset(0isize) + ru_0[0usize] * (*p).height;
        point[1usize] = *org.offset(1isize) + ru_0[1usize] * (*p).height;
        point[2usize] = *org.offset(2isize) + ru_0[2usize] * (*p).height;
        point[0usize] = point[0usize] + rr_0[0usize] * (*p).width;
        point[1usize] = point[1usize] + rr_0[1usize] * (*p).width;
        point[2usize] = point[2usize] + rr_0[2usize] * (*p).width;
        verts[2usize].xyz[0usize] = point[0usize];
        verts[2usize].xyz[1usize] = point[1usize];
        verts[2usize].xyz[2usize] = point[2usize];
        verts[2usize].st[0usize] = 1i32 as libc::c_float;
        verts[2usize].st[1usize] = 1i32 as libc::c_float;
        verts[2usize].modulate[0usize] = 111i32 as byte;
        verts[2usize].modulate[1usize] = 19i32 as byte;
        verts[2usize].modulate[2usize] = 9i32 as byte;
        verts[2usize].modulate[3usize] = (255i32 as libc::c_float * alpha_0) as byte;
        point[0usize] = *org.offset(0isize) + ru_0[0usize] * (*p).height;
        point[1usize] = *org.offset(1isize) + ru_0[1usize] * (*p).height;
        point[2usize] = *org.offset(2isize) + ru_0[2usize] * (*p).height;
        point[0usize] = point[0usize] + rr_0[0usize] * -(*p).width;
        point[1usize] = point[1usize] + rr_0[1usize] * -(*p).width;
        point[2usize] = point[2usize] + rr_0[2usize] * -(*p).width;
        verts[3usize].xyz[0usize] = point[0usize];
        verts[3usize].xyz[1usize] = point[1usize];
        verts[3usize].xyz[2usize] = point[2usize];
        verts[3usize].st[0usize] = 1i32 as libc::c_float;
        verts[3usize].st[1usize] = 0i32 as libc::c_float;
        verts[3usize].modulate[0usize] = 111i32 as byte;
        verts[3usize].modulate[1usize] = 19i32 as byte;
        verts[3usize].modulate[2usize] = 9i32 as byte;
        verts[3usize].modulate[3usize] = (255i32 as libc::c_float * alpha_0) as byte
    } else if (*p).type_0 == P_FLAT_SCALEUP as libc::c_int {
        let mut sinR: libc::c_float = 0.;
        let mut cosR: libc::c_float = 0.;
        if (*p).color == 2i32 {
            color[0usize] = 1i32 as vec_t;
            color[1usize] = 1i32 as vec_t;
            color[2usize] = 1i32 as vec_t
        } else {
            color[0usize] = 0.5f64 as vec_t;
            color[1usize] = 0.5f64 as vec_t;
            color[2usize] = 0.5f64 as vec_t
        }
        time = cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if width > (*p).endwidth {
            width = (*p).endwidth
        }
        if height > (*p).endheight {
            height = (*p).endheight
        }
        sinR = (height as libc::c_double
            * sin((*p).roll as libc::c_double * 3.14159265358979323846f64
                / 180.0f32 as libc::c_double)
            * sqrt(2i32 as libc::c_double)) as libc::c_float;
        cosR = (width as libc::c_double
            * cos((*p).roll as libc::c_double * 3.14159265358979323846f64
                / 180.0f32 as libc::c_double)
            * sqrt(2i32 as libc::c_double)) as libc::c_float;
        verts[0usize].xyz[0usize] = *org.offset(0isize);
        verts[0usize].xyz[1usize] = *org.offset(1isize);
        verts[0usize].xyz[2usize] = *org.offset(2isize);
        verts[0usize].xyz[0usize] -= sinR;
        verts[0usize].xyz[1usize] -= cosR;
        verts[0usize].st[0usize] = 0i32 as libc::c_float;
        verts[0usize].st[1usize] = 0i32 as libc::c_float;
        verts[0usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[0usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[0usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[0usize].modulate[3usize] = 255i32 as byte;
        verts[1usize].xyz[0usize] = *org.offset(0isize);
        verts[1usize].xyz[1usize] = *org.offset(1isize);
        verts[1usize].xyz[2usize] = *org.offset(2isize);
        verts[1usize].xyz[0usize] -= cosR;
        verts[1usize].xyz[1usize] += sinR;
        verts[1usize].st[0usize] = 0i32 as libc::c_float;
        verts[1usize].st[1usize] = 1i32 as libc::c_float;
        verts[1usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[1usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[1usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[1usize].modulate[3usize] = 255i32 as byte;
        verts[2usize].xyz[0usize] = *org.offset(0isize);
        verts[2usize].xyz[1usize] = *org.offset(1isize);
        verts[2usize].xyz[2usize] = *org.offset(2isize);
        verts[2usize].xyz[0usize] += sinR;
        verts[2usize].xyz[1usize] += cosR;
        verts[2usize].st[0usize] = 1i32 as libc::c_float;
        verts[2usize].st[1usize] = 1i32 as libc::c_float;
        verts[2usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[2usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[2usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[2usize].modulate[3usize] = 255i32 as byte;
        verts[3usize].xyz[0usize] = *org.offset(0isize);
        verts[3usize].xyz[1usize] = *org.offset(1isize);
        verts[3usize].xyz[2usize] = *org.offset(2isize);
        verts[3usize].xyz[0usize] += cosR;
        verts[3usize].xyz[1usize] -= sinR;
        verts[3usize].st[0usize] = 1i32 as libc::c_float;
        verts[3usize].st[1usize] = 0i32 as libc::c_float;
        verts[3usize].modulate[0usize] = (255i32 as libc::c_float * color[0usize]) as byte;
        verts[3usize].modulate[1usize] = (255i32 as libc::c_float * color[1usize]) as byte;
        verts[3usize].modulate[2usize] = (255i32 as libc::c_float * color[2usize]) as byte;
        verts[3usize].modulate[3usize] = 255i32 as byte
    } else if (*p).type_0 == P_FLAT as libc::c_int {
        verts[0usize].xyz[0usize] = *org.offset(0isize);
        verts[0usize].xyz[1usize] = *org.offset(1isize);
        verts[0usize].xyz[2usize] = *org.offset(2isize);
        verts[0usize].xyz[0usize] -= (*p).height;
        verts[0usize].xyz[1usize] -= (*p).width;
        verts[0usize].st[0usize] = 0i32 as libc::c_float;
        verts[0usize].st[1usize] = 0i32 as libc::c_float;
        verts[0usize].modulate[0usize] = 255i32 as byte;
        verts[0usize].modulate[1usize] = 255i32 as byte;
        verts[0usize].modulate[2usize] = 255i32 as byte;
        verts[0usize].modulate[3usize] = 255i32 as byte;
        verts[1usize].xyz[0usize] = *org.offset(0isize);
        verts[1usize].xyz[1usize] = *org.offset(1isize);
        verts[1usize].xyz[2usize] = *org.offset(2isize);
        verts[1usize].xyz[0usize] -= (*p).height;
        verts[1usize].xyz[1usize] += (*p).width;
        verts[1usize].st[0usize] = 0i32 as libc::c_float;
        verts[1usize].st[1usize] = 1i32 as libc::c_float;
        verts[1usize].modulate[0usize] = 255i32 as byte;
        verts[1usize].modulate[1usize] = 255i32 as byte;
        verts[1usize].modulate[2usize] = 255i32 as byte;
        verts[1usize].modulate[3usize] = 255i32 as byte;
        verts[2usize].xyz[0usize] = *org.offset(0isize);
        verts[2usize].xyz[1usize] = *org.offset(1isize);
        verts[2usize].xyz[2usize] = *org.offset(2isize);
        verts[2usize].xyz[0usize] += (*p).height;
        verts[2usize].xyz[1usize] += (*p).width;
        verts[2usize].st[0usize] = 1i32 as libc::c_float;
        verts[2usize].st[1usize] = 1i32 as libc::c_float;
        verts[2usize].modulate[0usize] = 255i32 as byte;
        verts[2usize].modulate[1usize] = 255i32 as byte;
        verts[2usize].modulate[2usize] = 255i32 as byte;
        verts[2usize].modulate[3usize] = 255i32 as byte;
        verts[3usize].xyz[0usize] = *org.offset(0isize);
        verts[3usize].xyz[1usize] = *org.offset(1isize);
        verts[3usize].xyz[2usize] = *org.offset(2isize);
        verts[3usize].xyz[0usize] += (*p).height;
        verts[3usize].xyz[1usize] -= (*p).width;
        verts[3usize].st[0usize] = 1i32 as libc::c_float;
        verts[3usize].st[1usize] = 0i32 as libc::c_float;
        verts[3usize].modulate[0usize] = 255i32 as byte;
        verts[3usize].modulate[1usize] = 255i32 as byte;
        verts[3usize].modulate[2usize] = 255i32 as byte;
        verts[3usize].modulate[3usize] = 255i32 as byte
    } else if (*p).type_0 == P_ANIM as libc::c_int {
        let mut rr_1: vec3_t = [0.; 3];
        let mut ru_1: vec3_t = [0.; 3];
        let mut rotate_ang_1: vec3_t = [0.; 3];
        let mut i: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        time = cg.time as libc::c_float - (*p).time;
        time2 = (*p).endtime - (*p).time;
        ratio = time / time2;
        if ratio >= 1.0f32 {
            ratio = 0.9999f32
        }
        width = (*p).width + ratio * ((*p).endwidth - (*p).width);
        height = (*p).height + ratio * ((*p).endheight - (*p).height);
        if (Distance(
            (*cg.snap).ps.origin.as_mut_ptr() as *const vec_t,
            org as *const vec_t,
        ) as libc::c_double)
            < width as libc::c_double / 1.5f64
        {
            return;
        }
        i = (*p).shaderAnim;
        j = floor(
            (ratio * shaderAnimCounts[(*p).shaderAnim as usize] as libc::c_float) as libc::c_double,
        ) as libc::c_int;
        (*p).pshader = shaderAnims[i as usize][j as usize];
        if 0 != (*p).roll {
            vectoangles(
                cg.refdef.viewaxis[0usize].as_mut_ptr() as *const vec_t,
                rotate_ang_1.as_mut_ptr(),
            );
            rotate_ang_1[2usize] += (*p).roll as libc::c_float;
            AngleVectors(
                rotate_ang_1.as_mut_ptr() as *const vec_t,
                0 as *mut vec_t,
                rr_1.as_mut_ptr(),
                ru_1.as_mut_ptr(),
            );
        }
        if 0 != (*p).roll {
            point[0usize] = *org.offset(0isize) + ru_1[0usize] * -height;
            point[1usize] = *org.offset(1isize) + ru_1[1usize] * -height;
            point[2usize] = *org.offset(2isize) + ru_1[2usize] * -height;
            point[0usize] = point[0usize] + rr_1[0usize] * -width;
            point[1usize] = point[1usize] + rr_1[1usize] * -width;
            point[2usize] = point[2usize] + rr_1[2usize] * -width
        } else {
            point[0usize] = *org.offset(0isize) + vup[0usize] * -height;
            point[1usize] = *org.offset(1isize) + vup[1usize] * -height;
            point[2usize] = *org.offset(2isize) + vup[2usize] * -height;
            point[0usize] = point[0usize] + vright[0usize] * -width;
            point[1usize] = point[1usize] + vright[1usize] * -width;
            point[2usize] = point[2usize] + vright[2usize] * -width
        }
        verts[0usize].xyz[0usize] = point[0usize];
        verts[0usize].xyz[1usize] = point[1usize];
        verts[0usize].xyz[2usize] = point[2usize];
        verts[0usize].st[0usize] = 0i32 as libc::c_float;
        verts[0usize].st[1usize] = 0i32 as libc::c_float;
        verts[0usize].modulate[0usize] = 255i32 as byte;
        verts[0usize].modulate[1usize] = 255i32 as byte;
        verts[0usize].modulate[2usize] = 255i32 as byte;
        verts[0usize].modulate[3usize] = 255i32 as byte;
        if 0 != (*p).roll {
            point[0usize] = point[0usize] + ru_1[0usize] * (2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + ru_1[1usize] * (2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + ru_1[2usize] * (2i32 as libc::c_float * height)
        } else {
            point[0usize] = point[0usize] + vup[0usize] * (2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + vup[1usize] * (2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + vup[2usize] * (2i32 as libc::c_float * height)
        }
        verts[1usize].xyz[0usize] = point[0usize];
        verts[1usize].xyz[1usize] = point[1usize];
        verts[1usize].xyz[2usize] = point[2usize];
        verts[1usize].st[0usize] = 0i32 as libc::c_float;
        verts[1usize].st[1usize] = 1i32 as libc::c_float;
        verts[1usize].modulate[0usize] = 255i32 as byte;
        verts[1usize].modulate[1usize] = 255i32 as byte;
        verts[1usize].modulate[2usize] = 255i32 as byte;
        verts[1usize].modulate[3usize] = 255i32 as byte;
        if 0 != (*p).roll {
            point[0usize] = point[0usize] + rr_1[0usize] * (2i32 as libc::c_float * width);
            point[1usize] = point[1usize] + rr_1[1usize] * (2i32 as libc::c_float * width);
            point[2usize] = point[2usize] + rr_1[2usize] * (2i32 as libc::c_float * width)
        } else {
            point[0usize] = point[0usize] + vright[0usize] * (2i32 as libc::c_float * width);
            point[1usize] = point[1usize] + vright[1usize] * (2i32 as libc::c_float * width);
            point[2usize] = point[2usize] + vright[2usize] * (2i32 as libc::c_float * width)
        }
        verts[2usize].xyz[0usize] = point[0usize];
        verts[2usize].xyz[1usize] = point[1usize];
        verts[2usize].xyz[2usize] = point[2usize];
        verts[2usize].st[0usize] = 1i32 as libc::c_float;
        verts[2usize].st[1usize] = 1i32 as libc::c_float;
        verts[2usize].modulate[0usize] = 255i32 as byte;
        verts[2usize].modulate[1usize] = 255i32 as byte;
        verts[2usize].modulate[2usize] = 255i32 as byte;
        verts[2usize].modulate[3usize] = 255i32 as byte;
        if 0 != (*p).roll {
            point[0usize] = point[0usize] + ru_1[0usize] * (-2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + ru_1[1usize] * (-2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + ru_1[2usize] * (-2i32 as libc::c_float * height)
        } else {
            point[0usize] = point[0usize] + vup[0usize] * (-2i32 as libc::c_float * height);
            point[1usize] = point[1usize] + vup[1usize] * (-2i32 as libc::c_float * height);
            point[2usize] = point[2usize] + vup[2usize] * (-2i32 as libc::c_float * height)
        }
        verts[3usize].xyz[0usize] = point[0usize];
        verts[3usize].xyz[1usize] = point[1usize];
        verts[3usize].xyz[2usize] = point[2usize];
        verts[3usize].st[0usize] = 1i32 as libc::c_float;
        verts[3usize].st[1usize] = 0i32 as libc::c_float;
        verts[3usize].modulate[0usize] = 255i32 as byte;
        verts[3usize].modulate[1usize] = 255i32 as byte;
        verts[3usize].modulate[2usize] = 255i32 as byte;
        verts[3usize].modulate[3usize] = 255i32 as byte
    }
    if 0 == (*p).pshader {
        return;
    }
    if (*p).type_0 == P_WEATHER as libc::c_int
        || (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int
        || (*p).type_0 == P_WEATHER_FLURRY as libc::c_int
    {
        trap_R_AddPolyToScene((*p).pshader, 3i32, TRIverts.as_mut_ptr());
    } else {
        trap_R_AddPolyToScene((*p).pshader, 4i32, verts.as_mut_ptr());
    };
}
#[no_mangle]
pub static mut vup: vec3_t = [0.; 3];
#[no_mangle]
pub static mut vright: vec3_t = [0.; 3];
#[no_mangle]
pub static mut rup: vec3_t = [0.; 3];
#[no_mangle]
pub static mut rright: vec3_t = [0.; 3];
#[no_mangle]
pub static mut rforward: vec3_t = [0.; 3];
// Ridah, made this static so it doesn't interfere with other files
static mut roll: libc::c_float = 0.0f64 as libc::c_float;
#[no_mangle]
pub static mut vforward: vec3_t = [0.; 3];
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleSnow(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
    mut origin2: *mut vec_t,
    mut turb: libc::c_int,
    mut range: libc::c_float,
    mut snum: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if 0 == pshader {
        CG_Printf(b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).color = 0i32;
    (*p).alpha = 0.40f32;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).start = *origin.offset(2isize);
    (*p).end = *origin2.offset(2isize);
    (*p).pshader = pshader;
    (*p).height = 1i32 as libc::c_float;
    (*p).width = 1i32 as libc::c_float;
    (*p).vel[2usize] = -50i32 as vec_t;
    if 0 != turb {
        (*p).type_0 = P_WEATHER_TURBULENT as libc::c_int;
        (*p).vel[2usize] = (-50i32 as libc::c_double * 1.3f64) as vec_t
    } else {
        (*p).type_0 = P_WEATHER as libc::c_int
    }
    (*p).org[0usize] = *origin.offset(0isize);
    (*p).org[1usize] = *origin.offset(1isize);
    (*p).org[2usize] = *origin.offset(2isize);
    (*p).org[0usize] = ((*p).org[0usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * range as libc::c_double) as vec_t;
    (*p).org[1usize] = ((*p).org[1usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * range as libc::c_double) as vec_t;
    (*p).org[2usize] = ((*p).org[2usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * ((*p).start - (*p).end) as libc::c_double) as vec_t;
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[0usize] = (*p).vel[1usize];
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    if 0 != turb {
        (*p).vel[0usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 16i32 as libc::c_double) as vec_t;
        (*p).vel[1usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 16i32 as libc::c_double) as vec_t
    }
    (*p).snum = snum;
    (*p).link = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleSmoke(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    // using cent->density = enttime
    //		 cent->frame = startfade
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if 0 == pshader {
        CG_Printf(b"CG_ParticleSmoke == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).endtime = (cg.time + (*cent).currentState.time) as libc::c_float;
    (*p).startfade = (cg.time + (*cent).currentState.time2) as libc::c_float;
    (*p).color = 0i32;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).start = (*cent).currentState.origin[2usize];
    (*p).end = (*cent).currentState.origin2[2usize];
    (*p).pshader = pshader;
    (*p).rotate = qfalse;
    (*p).height = 8i32 as libc::c_float;
    (*p).width = 8i32 as libc::c_float;
    (*p).endheight = 32i32 as libc::c_float;
    (*p).endwidth = 32i32 as libc::c_float;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0usize] = (*cent).currentState.origin[0usize];
    (*p).org[1usize] = (*cent).currentState.origin[1usize];
    (*p).org[2usize] = (*cent).currentState.origin[2usize];
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[0usize] = (*p).vel[1usize];
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).vel[2usize] = 5i32 as vec_t;
    if (*cent).currentState.frame == 1i32 {
        (*p).vel[2usize] *= -1i32 as libc::c_float
    }
    (*p).roll = (8i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 4i32 as libc::c_double) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CG_AddParticleShrapnel(mut le: *mut localEntity_t) {}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleSnowFlurry(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut turb: qboolean = qtrue;
    if 0 == pshader {
        CG_Printf(
            b"CG_ParticleSnowFlurry pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).color = 0i32;
    (*p).alpha = 0.90f32;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).start = (*cent).currentState.origin2[0usize];
    (*p).end = (*cent).currentState.origin2[1usize];
    (*p).endtime = (cg.time + (*cent).currentState.time) as libc::c_float;
    (*p).startfade = (cg.time + (*cent).currentState.time2) as libc::c_float;
    (*p).pshader = pshader;
    if rand() % 100i32 > 90i32 {
        (*p).height = 32i32 as libc::c_float;
        (*p).width = 32i32 as libc::c_float;
        (*p).alpha = 0.10f32
    } else {
        (*p).height = 1i32 as libc::c_float;
        (*p).width = 1i32 as libc::c_float
    }
    (*p).vel[2usize] = -20i32 as vec_t;
    (*p).type_0 = P_WEATHER_FLURRY as libc::c_int;
    if 0 != turb as u64 {
        (*p).vel[2usize] = -10i32 as vec_t
    }
    (*p).org[0usize] = (*cent).currentState.origin[0usize];
    (*p).org[1usize] = (*cent).currentState.origin[1usize];
    (*p).org[2usize] = (*cent).currentState.origin[2usize];
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[0usize] = (*p).vel[1usize];
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).vel[0usize] = ((*p).vel[0usize] as libc::c_double
        + (((*cent).currentState.angles[0usize] * 32i32 as libc::c_float) as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 16i32 as libc::c_double)) as vec_t;
    (*p).vel[1usize] = ((*p).vel[1usize] as libc::c_double
        + (((*cent).currentState.angles[1usize] * 32i32 as libc::c_float) as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 16i32 as libc::c_double)) as vec_t;
    (*p).vel[2usize] += (*cent).currentState.angles[2usize];
    if 0 != turb as u64 {
        (*p).accel[0usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 16i32 as libc::c_double) as vec_t;
        (*p).accel[1usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 16i32 as libc::c_double) as vec_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleBulletDebris(
    mut org: *mut vec_t,
    mut vel: *mut vec_t,
    mut duration: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).endtime = (cg.time + duration) as libc::c_float;
    (*p).startfade = (cg.time + duration / 2i32) as libc::c_float;
    (*p).color = 3i32;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).height = 0.5f64 as libc::c_float;
    (*p).width = 0.5f64 as libc::c_float;
    (*p).endheight = 0.5f64 as libc::c_float;
    (*p).endwidth = 0.5f64 as libc::c_float;
    (*p).pshader = cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0usize] = *org.offset(0isize);
    (*p).org[1usize] = *org.offset(1isize);
    (*p).org[2usize] = *org.offset(2isize);
    (*p).vel[0usize] = *vel.offset(0isize);
    (*p).vel[1usize] = *vel.offset(1isize);
    (*p).vel[2usize] = *vel.offset(2isize);
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).accel[2usize] = -60i32 as vec_t;
    (*p).vel[2usize] += -20i32 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleSparks(
    mut org: *mut vec_t,
    mut vel: *mut vec_t,
    mut duration: libc::c_int,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut speed: libc::c_float,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).endtime = (cg.time + duration) as libc::c_float;
    (*p).startfade = (cg.time + duration / 2i32) as libc::c_float;
    (*p).color = 3i32;
    (*p).alpha = 0.4f32;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).height = 0.5f64 as libc::c_float;
    (*p).width = 0.5f64 as libc::c_float;
    (*p).endheight = 0.5f64 as libc::c_float;
    (*p).endwidth = 0.5f64 as libc::c_float;
    (*p).pshader = cgs.media.tracerShader;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0usize] = *org.offset(0isize);
    (*p).org[1usize] = *org.offset(1isize);
    (*p).org[2usize] = *org.offset(2isize);
    (*p).org[0usize] = ((*p).org[0usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * x as libc::c_double) as vec_t;
    (*p).org[1usize] = ((*p).org[1usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * y as libc::c_double) as vec_t;
    (*p).vel[0usize] = *vel.offset(0isize);
    (*p).vel[1usize] = *vel.offset(1isize);
    (*p).vel[2usize] = *vel.offset(2isize);
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).vel[0usize] = ((*p).vel[0usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 4i32 as libc::c_double) as vec_t;
    (*p).vel[1usize] = ((*p).vel[1usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 4i32 as libc::c_double) as vec_t;
    (*p).vel[2usize] = ((*p).vel[2usize] as libc::c_double
        + (20i32 as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 10i32 as libc::c_double)
            * speed as libc::c_double) as vec_t;
    (*p).accel[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 4i32 as libc::c_double) as vec_t;
    (*p).accel[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 4i32 as libc::c_double) as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleDust(
    mut cent: *mut centity_t,
    mut origin: *mut vec_t,
    mut dir: *mut vec_t,
) {
    let mut length: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut crittersize: libc::c_float = 0.;
    let mut angles: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: libc::c_int = 0;
    dist = 0i32 as libc::c_float;
    *dir.offset(0isize) = -*dir.offset(0isize);
    *dir.offset(1isize) = -*dir.offset(1isize);
    *dir.offset(2isize) = -*dir.offset(2isize);
    length = VectorLength(dir as *const vec_t);
    vectoangles(dir as *const vec_t, angles.as_mut_ptr());
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    crittersize = 32i32 as libc::c_float;
    if 0. != length {
        dist = length / crittersize
    }
    if dist < 1i32 as libc::c_float {
        dist = 1i32 as libc::c_float
    }
    point[0usize] = *origin.offset(0isize);
    point[1usize] = *origin.offset(1isize);
    point[2usize] = *origin.offset(2isize);
    i = 0i32;
    while (i as libc::c_float) < dist {
        point[0usize] = point[0usize] + forward[0usize] * crittersize;
        point[1usize] = point[1usize] + forward[1usize] * crittersize;
        point[2usize] = point[2usize] + forward[2usize] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = cg.time as libc::c_float;
        (*p).alpha = 5.0f64 as libc::c_float;
        (*p).alphavel = 0i32 as libc::c_float;
        (*p).roll = 0i32;
        (*p).pshader = cgs.media.smokePuffShader;
        if 0. != length {
            (*p).endtime = ((cg.time + 4500i32) as libc::c_double
                + 2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64)
                    * 3500i32 as libc::c_double) as libc::c_float
        } else {
            (*p).endtime = ((cg.time + 750i32) as libc::c_double
                + 2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64)
                    * 500i32 as libc::c_double) as libc::c_float
        }
        (*p).startfade = cg.time as libc::c_float;
        (*p).width = 32i32 as libc::c_float;
        (*p).height = 32i32 as libc::c_float;
        (*p).endheight = (32i32 as libc::c_double * 3.0f64) as libc::c_float;
        (*p).endwidth = (32i32 as libc::c_double * 3.0f64) as libc::c_float;
        if 0. == length {
            (*p).width *= 0.2f32;
            (*p).height *= 0.2f32;
            (*p).endheight = 16i32 as libc::c_float;
            (*p).endwidth = 16i32 as libc::c_float
        }
        (*p).type_0 = P_SMOKE as libc::c_int;
        (*p).org[0usize] = point[0usize];
        (*p).org[1usize] = point[1usize];
        (*p).org[2usize] = point[2usize];
        (*p).vel[0usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 6i32 as libc::c_double) as vec_t;
        (*p).vel[1usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 6i32 as libc::c_double) as vec_t;
        (*p).vel[2usize] = (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
            * 20i32 as libc::c_float;
        (*p).accel[0usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 3i32 as libc::c_double) as vec_t;
        (*p).accel[1usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 3i32 as libc::c_double) as vec_t;
        (*p).accel[2usize] = (-40i32 as libc::c_double * 0.4f64) as vec_t;
        (*p).accel[2usize] = 0i32 as vec_t;
        (*p).accel[1usize] = (*p).accel[2usize];
        (*p).accel[0usize] = (*p).accel[1usize];
        (*p).rotate = qfalse;
        (*p).roll = rand() % 179i32;
        (*p).alpha = 0.75f64 as libc::c_float;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleMisc(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
    mut size: libc::c_int,
    mut duration: libc::c_int,
    mut alpha: libc::c_float,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if 0 == pshader {
        CG_Printf(
            b"CG_ParticleImpactSmokePuff pshader == ZERO!\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).roll = rand() % 179i32;
    (*p).pshader = pshader;
    if duration > 0i32 {
        (*p).endtime = (cg.time + duration) as libc::c_float
    } else {
        (*p).endtime = duration as libc::c_float
    }
    (*p).startfade = cg.time as libc::c_float;
    (*p).width = size as libc::c_float;
    (*p).height = size as libc::c_float;
    (*p).endheight = size as libc::c_float;
    (*p).endwidth = size as libc::c_float;
    (*p).type_0 = P_SPRITE as libc::c_int;
    (*p).org[0usize] = *origin.offset(0isize);
    (*p).org[1usize] = *origin.offset(1isize);
    (*p).org[2usize] = *origin.offset(2isize);
    (*p).rotate = qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleExplosion(
    mut animStr: *mut libc::c_char,
    mut origin: *mut vec_t,
    mut vel: *mut vec_t,
    mut duration: libc::c_int,
    mut sizeStart: libc::c_int,
    mut sizeEnd: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut anim: libc::c_int = 0;
    if animStr < 10i32 as *mut libc::c_char {
        CG_Error(
            b"CG_ParticleExplosion: animStr is probably an index rather than a string\x00"
                as *const u8 as *const libc::c_char,
        );
    }
    anim = 0i32;
    while !shaderAnimNames[anim as usize].is_null() {
        if 0 == Q_stricmp(animStr, shaderAnimNames[anim as usize]) {
            break;
        }
        anim += 1
    }
    if shaderAnimNames[anim as usize].is_null() {
        CG_Error(
            b"CG_ParticleExplosion: unknown animation string: %s\x00" as *const u8
                as *const libc::c_char,
            animStr,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).alpha = 0.5f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    if duration < 0i32 {
        duration *= -1i32;
        (*p).roll = 0i32
    } else {
        (*p).roll = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 179i32 as libc::c_double) as libc::c_int
    }
    (*p).shaderAnim = anim;
    (*p).width = sizeStart as libc::c_float;
    (*p).height = sizeStart as libc::c_float * shaderAnimSTRatio[anim as usize];
    (*p).endheight = sizeEnd as libc::c_float;
    (*p).endwidth = sizeEnd as libc::c_float * shaderAnimSTRatio[anim as usize];
    (*p).endtime = (cg.time + duration) as libc::c_float;
    (*p).type_0 = P_ANIM as libc::c_int;
    (*p).org[0usize] = *origin.offset(0isize);
    (*p).org[1usize] = *origin.offset(1isize);
    (*p).org[2usize] = *origin.offset(2isize);
    (*p).vel[0usize] = *vel.offset(0isize);
    (*p).vel[1usize] = *vel.offset(1isize);
    (*p).vel[2usize] = *vel.offset(2isize);
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
}
static mut shaderAnimSTRatio: [libc::c_float; 32] = [
    1.0f32, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    0., 0., 0., 0., 0., 0., 0., 0., 0.,
];
#[no_mangle]
pub unsafe extern "C" fn CG_NewParticleArea(mut num: libc::c_int) -> libc::c_int {
    // const char *str;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    let mut origin: vec3_t = [0.; 3];
    let mut origin2: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut range: libc::c_float = 0i32 as libc::c_float;
    let mut turb: libc::c_int = 0;
    let mut numparticles: libc::c_int = 0;
    let mut snum: libc::c_int = 0;
    str = CG_ConfigString(num) as *mut libc::c_char;
    if 0 == *str.offset(0isize) {
        return 0i32;
    }
    token = COM_Parse(&mut str);
    type_0 = atoi(token);
    if type_0 == 1i32 {
        range = 128i32 as libc::c_float
    } else if type_0 == 2i32 {
        range = 64i32 as libc::c_float
    } else if type_0 == 3i32 {
        range = 32i32 as libc::c_float
    } else if type_0 == 0i32 {
        range = 256i32 as libc::c_float
    } else if type_0 == 4i32 {
        range = 8i32 as libc::c_float
    } else if type_0 == 5i32 {
        range = 16i32 as libc::c_float
    } else if type_0 == 6i32 {
        range = 32i32 as libc::c_float
    } else if type_0 == 7i32 {
        range = 64i32 as libc::c_float
    }
    i = 0i32;
    while i < 3i32 {
        token = COM_Parse(&mut str);
        origin[i as usize] = atof(token) as vec_t;
        i += 1
    }
    i = 0i32;
    while i < 3i32 {
        token = COM_Parse(&mut str);
        origin2[i as usize] = atof(token) as vec_t;
        i += 1
    }
    token = COM_Parse(&mut str);
    numparticles = atoi(token);
    token = COM_Parse(&mut str);
    turb = atoi(token);
    token = COM_Parse(&mut str);
    snum = atoi(token);
    i = 0i32;
    while i < numparticles {
        if type_0 >= 4i32 {
            CG_ParticleBubble(
                cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        } else {
            CG_ParticleSnow(
                cgs.media.waterBubbleShader,
                origin.as_mut_ptr(),
                origin2.as_mut_ptr(),
                turb,
                range,
                snum,
            );
        }
        i += 1
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleBubble(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
    mut origin2: *mut vec_t,
    mut turb: libc::c_int,
    mut range: libc::c_float,
    mut snum: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut randsize: libc::c_float = 0.;
    if 0 == pshader {
        CG_Printf(b"CG_ParticleSnow pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).color = 0i32;
    (*p).alpha = 0.40f32;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).start = *origin.offset(2isize);
    (*p).end = *origin2.offset(2isize);
    (*p).pshader = pshader;
    randsize = (1i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 0.5f64) as libc::c_float;
    (*p).height = randsize;
    (*p).width = randsize;
    (*p).vel[2usize] = (50i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 10i32 as libc::c_double) as vec_t;
    if 0 != turb {
        (*p).type_0 = P_BUBBLE_TURBULENT as libc::c_int;
        (*p).vel[2usize] = (50i32 as libc::c_double * 1.3f64) as vec_t
    } else {
        (*p).type_0 = P_BUBBLE as libc::c_int
    }
    (*p).org[0usize] = *origin.offset(0isize);
    (*p).org[1usize] = *origin.offset(1isize);
    (*p).org[2usize] = *origin.offset(2isize);
    (*p).org[0usize] = ((*p).org[0usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * range as libc::c_double) as vec_t;
    (*p).org[1usize] = ((*p).org[1usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * range as libc::c_double) as vec_t;
    (*p).org[2usize] = ((*p).org[2usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * ((*p).start - (*p).end) as libc::c_double) as vec_t;
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[0usize] = (*p).vel[1usize];
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    if 0 != turb {
        (*p).vel[0usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 4i32 as libc::c_double) as vec_t;
        (*p).vel[1usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 4i32 as libc::c_double) as vec_t
    }
    (*p).snum = snum;
    (*p).link = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CG_SnowLink(mut cent: *mut centity_t, mut particleOn: qboolean) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: libc::c_int = 0;
    id = (*cent).currentState.frame;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_WEATHER as libc::c_int
            || (*p).type_0 == P_WEATHER_TURBULENT as libc::c_int
        {
            if (*p).snum == id {
                if 0 != particleOn as u64 {
                    (*p).link = qtrue
                } else {
                    (*p).link = qfalse
                }
            }
        }
        p = next
    }
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleImpactSmokePuff(
    mut pshader: qhandle_t,
    mut origin: *mut vec_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if 0 == pshader {
        CG_Printf(
            b"CG_ParticleImpactSmokePuff pshader == ZERO!\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).alpha = 0.25f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).roll = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 179i32 as libc::c_double) as libc::c_int;
    (*p).pshader = pshader;
    (*p).endtime = (cg.time + 1000i32) as libc::c_float;
    (*p).startfade = (cg.time + 100i32) as libc::c_float;
    (*p).width = (rand() % 4i32 + 8i32) as libc::c_float;
    (*p).height = (rand() % 4i32 + 8i32) as libc::c_float;
    (*p).endheight = (*p).height * 2i32 as libc::c_float;
    (*p).endwidth = (*p).width * 2i32 as libc::c_float;
    (*p).endtime = (cg.time + 500i32) as libc::c_float;
    (*p).type_0 = P_SMOKE_IMPACT as libc::c_int;
    (*p).org[0usize] = *origin.offset(0isize);
    (*p).org[1usize] = *origin.offset(1isize);
    (*p).org[2usize] = *origin.offset(2isize);
    (*p).vel[0usize] = 0i32 as vec_t;
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[2usize] = 20i32 as vec_t;
    (*p).accel[0usize] = 0i32 as vec_t;
    (*p).accel[1usize] = 0i32 as vec_t;
    (*p).accel[2usize] = 20i32 as vec_t;
    (*p).rotate = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CG_Particle_Bleed(
    mut pshader: qhandle_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut fleshEntityNum: libc::c_int,
    mut duration: libc::c_int,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if 0 == pshader {
        CG_Printf(b"CG_Particle_Bleed pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).roll = 0i32;
    (*p).pshader = pshader;
    (*p).endtime = (cg.time + duration) as libc::c_float;
    if 0 != fleshEntityNum {
        (*p).startfade = cg.time as libc::c_float
    } else {
        (*p).startfade = (cg.time + 100i32) as libc::c_float
    }
    (*p).width = 4i32 as libc::c_float;
    (*p).height = 4i32 as libc::c_float;
    (*p).endheight = (4i32 + rand() % 3i32) as libc::c_float;
    (*p).endwidth = (*p).endheight;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0usize] = *start.offset(0isize);
    (*p).org[1usize] = *start.offset(1isize);
    (*p).org[2usize] = *start.offset(2isize);
    (*p).vel[0usize] = 0i32 as vec_t;
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[2usize] = -20i32 as vec_t;
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179i32;
    (*p).color = 2i32;
    (*p).alpha = 0.75f64 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CG_Particle_OilParticle(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut time: libc::c_int = 0;
    let mut time2: libc::c_int = 0;
    let mut ratio: libc::c_float = 0.;
    let mut duration: libc::c_float = 1500i32 as libc::c_float;
    time = cg.time;
    time2 = cg.time + (*cent).currentState.time;
    ratio = 1i32 as libc::c_float - time as libc::c_float / time2 as libc::c_float;
    if 0 == pshader {
        CG_Printf(b"CG_Particle_OilParticle == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).roll = 0i32;
    (*p).pshader = pshader;
    (*p).endtime = cg.time as libc::c_float + duration;
    (*p).startfade = (*p).endtime;
    (*p).width = 1i32 as libc::c_float;
    (*p).height = 3i32 as libc::c_float;
    (*p).endheight = 3i32 as libc::c_float;
    (*p).endwidth = 1i32 as libc::c_float;
    (*p).type_0 = P_SMOKE as libc::c_int;
    (*p).org[0usize] = (*cent).currentState.origin[0usize];
    (*p).org[1usize] = (*cent).currentState.origin[1usize];
    (*p).org[2usize] = (*cent).currentState.origin[2usize];
    (*p).vel[0usize] = (*cent).currentState.origin2[0usize] * (16i32 as libc::c_float * ratio);
    (*p).vel[1usize] = (*cent).currentState.origin2[1usize] * (16i32 as libc::c_float * ratio);
    (*p).vel[2usize] = (*cent).currentState.origin2[2usize];
    (*p).snum = 1.0f32 as libc::c_int;
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).accel[2usize] = -20i32 as vec_t;
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179i32;
    (*p).alpha = 0.75f64 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CG_Particle_OilSlick(mut pshader: qhandle_t, mut cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    if 0 == pshader {
        CG_Printf(b"CG_Particle_OilSlick == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    if 0. != (*cent).currentState.angles2[2usize] {
        (*p).endtime = cg.time as libc::c_float + (*cent).currentState.angles2[2usize]
    } else {
        (*p).endtime = (cg.time + 60000i32) as libc::c_float
    }
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).roll = 0i32;
    (*p).pshader = pshader;
    if 0. != (*cent).currentState.angles2[0usize] || 0. != (*cent).currentState.angles2[1usize] {
        (*p).width = (*cent).currentState.angles2[0usize];
        (*p).height = (*cent).currentState.angles2[0usize];
        (*p).endheight = (*cent).currentState.angles2[1usize];
        (*p).endwidth = (*cent).currentState.angles2[1usize]
    } else {
        (*p).width = 8i32 as libc::c_float;
        (*p).height = 8i32 as libc::c_float;
        (*p).endheight = 16i32 as libc::c_float;
        (*p).endwidth = 16i32 as libc::c_float
    }
    (*p).type_0 = P_FLAT_SCALEUP as libc::c_int;
    (*p).snum = 1.0f64 as libc::c_int;
    (*p).org[0usize] = (*cent).currentState.origin[0usize];
    (*p).org[1usize] = (*cent).currentState.origin[1usize];
    (*p).org[2usize] = (*cent).currentState.origin[2usize];
    (*p).org[2usize] = ((*p).org[2usize] as libc::c_double
        + (0.55f64
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 0.5f64)) as vec_t;
    (*p).vel[0usize] = 0i32 as vec_t;
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[2usize] = 0i32 as vec_t;
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179i32;
    (*p).alpha = 0.75f64 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CG_OilSlickRemove(mut cent: *mut centity_t) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut id: libc::c_int = 0;
    id = 1.0f32 as libc::c_int;
    if 0 == id {
        CG_Printf(b"CG_OilSlickRevove NULL id\n\x00" as *const u8 as *const libc::c_char);
    }
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).type_0 == P_FLAT_SCALEUP as libc::c_int {
            if (*p).snum == id {
                (*p).endtime = (cg.time + 100i32) as libc::c_float;
                (*p).startfade = (*p).endtime;
                (*p).type_0 = P_FLAT_SCALEUP_FADE as libc::c_int
            }
        }
        p = next
    }
}
#[no_mangle]
pub unsafe extern "C" fn ValidBloodPool(mut start: *mut vec_t) -> qboolean {
    let mut angles: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut this_pos: vec3_t = [0.; 3];
    let mut x_pos: vec3_t = [0.; 3];
    let mut center_pos: vec3_t = [0.; 3];
    let mut end_pos: vec3_t = [0.; 3];
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut fwidth: libc::c_int = 0;
    let mut fheight: libc::c_int = 0;
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
    let mut normal: vec3_t = [0.; 3];
    fwidth = 16i32;
    fheight = 16i32;
    normal[0usize] = 0i32 as vec_t;
    normal[1usize] = 0i32 as vec_t;
    normal[2usize] = 1i32 as vec_t;
    vectoangles(normal.as_mut_ptr() as *const vec_t, angles.as_mut_ptr());
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        0 as *mut vec_t,
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    center_pos[0usize] = (*start.offset(0isize) as libc::c_double
        + normal[0usize] as libc::c_double * 0.5f64) as vec_t;
    center_pos[1usize] = (*start.offset(1isize) as libc::c_double
        + normal[1usize] as libc::c_double * 0.5f64) as vec_t;
    center_pos[2usize] = (*start.offset(2isize) as libc::c_double
        + normal[2usize] as libc::c_double * 0.5f64) as vec_t;
    x = -fwidth / 2i32;
    while x < fwidth {
        x_pos[0usize] = center_pos[0usize] + right[0usize] * x as libc::c_float;
        x_pos[1usize] = center_pos[1usize] + right[1usize] * x as libc::c_float;
        x_pos[2usize] = center_pos[2usize] + right[2usize] * x as libc::c_float;
        y = -fheight / 2i32;
        while y < fheight {
            this_pos[0usize] = x_pos[0usize] + up[0usize] * y as libc::c_float;
            this_pos[1usize] = x_pos[1usize] + up[1usize] * y as libc::c_float;
            this_pos[2usize] = x_pos[2usize] + up[2usize] * y as libc::c_float;
            end_pos[0usize] = (this_pos[0usize] as libc::c_double
                + normal[0usize] as libc::c_double * (-0.5f64 * 2i32 as libc::c_double))
                as vec_t;
            end_pos[1usize] = (this_pos[1usize] as libc::c_double
                + normal[1usize] as libc::c_double * (-0.5f64 * 2i32 as libc::c_double))
                as vec_t;
            end_pos[2usize] = (this_pos[2usize] as libc::c_double
                + normal[2usize] as libc::c_double * (-0.5f64 * 2i32 as libc::c_double))
                as vec_t;
            CG_Trace(
                &mut trace,
                this_pos.as_mut_ptr() as *const vec_t,
                0 as *const vec_t,
                0 as *const vec_t,
                end_pos.as_mut_ptr() as *const vec_t,
                -1i32,
                1i32,
            );
            if trace.entityNum < (1i32 << 10i32) - 2i32 {
                return qfalse;
            }
            if !(0 == trace.startsolid as u64 && trace.fraction < 1i32 as libc::c_float) {
                return qfalse;
            }
            y += fheight
        }
        x += fwidth
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CG_BloodPool(
    mut le: *mut localEntity_t,
    mut pshader: qhandle_t,
    mut tr: *mut trace_t,
) {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut legit: qboolean = qfalse;
    let mut start: vec3_t = [0.; 3];
    let mut rndSize: libc::c_float = 0.;
    if 0 == pshader {
        CG_Printf(b"CG_BloodPool pshader == ZERO!\n\x00" as *const u8 as *const libc::c_char);
    }
    if free_particles.is_null() {
        return;
    }
    start[0usize] = (*tr).endpos[0usize];
    start[1usize] = (*tr).endpos[1usize];
    start[2usize] = (*tr).endpos[2usize];
    legit = ValidBloodPool(start.as_mut_ptr());
    if 0 == legit as u64 {
        return;
    }
    p = free_particles;
    free_particles = (*p).next;
    (*p).next = active_particles;
    active_particles = p;
    (*p).time = cg.time as libc::c_float;
    (*p).endtime = (cg.time + 3000i32) as libc::c_float;
    (*p).startfade = (*p).endtime;
    (*p).alpha = 1.0f64 as libc::c_float;
    (*p).alphavel = 0i32 as libc::c_float;
    (*p).roll = 0i32;
    (*p).pshader = pshader;
    rndSize = (0.4f64
        + ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            * 0.6f64) as libc::c_float;
    (*p).width = 8i32 as libc::c_float * rndSize;
    (*p).height = 8i32 as libc::c_float * rndSize;
    (*p).endheight = 16i32 as libc::c_float * rndSize;
    (*p).endwidth = 16i32 as libc::c_float * rndSize;
    (*p).type_0 = P_FLAT_SCALEUP as libc::c_int;
    (*p).org[0usize] = start[0usize];
    (*p).org[1usize] = start[1usize];
    (*p).org[2usize] = start[2usize];
    (*p).vel[0usize] = 0i32 as vec_t;
    (*p).vel[1usize] = 0i32 as vec_t;
    (*p).vel[2usize] = 0i32 as vec_t;
    (*p).accel[2usize] = 0i32 as vec_t;
    (*p).accel[1usize] = (*p).accel[2usize];
    (*p).accel[0usize] = (*p).accel[1usize];
    (*p).rotate = qfalse;
    (*p).roll = rand() % 179i32;
    (*p).alpha = 0.75f64 as libc::c_float;
    (*p).color = 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParticleBloodCloud(
    mut cent: *mut centity_t,
    mut origin: *mut vec_t,
    mut dir: *mut vec_t,
) {
    let mut length: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut crittersize: libc::c_float = 0.;
    let mut angles: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut i: libc::c_int = 0;
    dist = 0i32 as libc::c_float;
    length = VectorLength(dir as *const vec_t);
    vectoangles(dir as *const vec_t, angles.as_mut_ptr());
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    crittersize = 32i32 as libc::c_float;
    if 0. != length {
        dist = length / crittersize
    }
    if dist < 1i32 as libc::c_float {
        dist = 1i32 as libc::c_float
    }
    point[0usize] = *origin.offset(0isize);
    point[1usize] = *origin.offset(1isize);
    point[2usize] = *origin.offset(2isize);
    i = 0i32;
    while (i as libc::c_float) < dist {
        point[0usize] = point[0usize] + forward[0usize] * crittersize;
        point[1usize] = point[1usize] + forward[1usize] * crittersize;
        point[2usize] = point[2usize] + forward[2usize] * crittersize;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        (*p).next = active_particles;
        active_particles = p;
        (*p).time = cg.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = 0i32 as libc::c_float;
        (*p).roll = 0i32;
        (*p).pshader = cgs.media.smokePuffShader;
        (*p).endtime = ((cg.time + 350i32) as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 100i32 as libc::c_double) as libc::c_float;
        (*p).startfade = cg.time as libc::c_float;
        (*p).width = 32i32 as libc::c_float;
        (*p).height = 32i32 as libc::c_float;
        (*p).endheight = 32i32 as libc::c_float;
        (*p).endwidth = 32i32 as libc::c_float;
        (*p).type_0 = P_SMOKE as libc::c_int;
        (*p).org[0usize] = *origin.offset(0isize);
        (*p).org[1usize] = *origin.offset(1isize);
        (*p).org[2usize] = *origin.offset(2isize);
        (*p).vel[0usize] = 0i32 as vec_t;
        (*p).vel[1usize] = 0i32 as vec_t;
        (*p).vel[2usize] = -1i32 as vec_t;
        (*p).accel[2usize] = 0i32 as vec_t;
        (*p).accel[1usize] = (*p).accel[2usize];
        (*p).accel[0usize] = (*p).accel[1usize];
        (*p).rotate = qfalse;
        (*p).roll = rand() % 179i32;
        (*p).color = 2i32;
        (*p).alpha = 0.75f64 as libc::c_float;
        i += 1
    }
}
pub const P_ROTATE: unnamed = 4;
pub const P_FLAT: unnamed = 2;
pub const P_WEATHER_FLURRY: unnamed = 11;
pub const P_BUBBLE: unnamed = 13;
pub const P_SMOKE_IMPACT: unnamed = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct particle_s {
    pub next: *mut particle_s,
    pub time: libc::c_float,
    pub endtime: libc::c_float,
    pub org: vec3_t,
    pub vel: vec3_t,
    pub accel: vec3_t,
    pub color: libc::c_int,
    pub colorvel: libc::c_float,
    pub alpha: libc::c_float,
    pub alphavel: libc::c_float,
    pub type_0: libc::c_int,
    pub pshader: qhandle_t,
    pub height: libc::c_float,
    pub width: libc::c_float,
    pub endheight: libc::c_float,
    pub endwidth: libc::c_float,
    pub start: libc::c_float,
    pub end: libc::c_float,
    pub startfade: libc::c_float,
    pub rotate: qboolean,
    pub snum: libc::c_int,
    pub link: qboolean,
    pub shaderAnim: libc::c_int,
    pub roll: libc::c_int,
    pub accumroll: libc::c_int,
}
pub const P_ANIM: unnamed = 6;
pub const P_WEATHER: unnamed = 1;
pub type cparticle_t = particle_s;
pub const P_FLAT_SCALEUP_FADE: unnamed = 10;
pub const P_SPRITE: unnamed = 15;
pub const P_BLEED: unnamed = 8;
pub const P_WEATHER_TURBULENT: unnamed = 5;
pub const P_SMOKE: unnamed = 3;
pub const P_NONE: unnamed = 0;
pub const P_BUBBLE_TURBULENT: unnamed = 14;
pub const P_BAT: unnamed = 7;
pub type unnamed = libc::c_uint;
pub const P_FLAT_SCALEUP: unnamed = 9;
