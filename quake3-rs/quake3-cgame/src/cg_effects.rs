use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, GENDER_FEMALE, GENDER_MALE,
    GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED,
    TEAM_SPECTATOR,
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
use cg_ents::{
    CG_AddPacketEntities, CG_AdjustPositionForMover, CG_Beam, CG_PositionEntityOnTag,
    CG_PositionRotatedEntityOnTag, CG_SetEntitySoundPosition,
};
use cg_event::{CG_CheckEvents, CG_EntityEvent, CG_PainEvent, CG_PlaceString};
use cg_info::{CG_DrawInformation, CG_LoadingClient, CG_LoadingItem, CG_LoadingString};
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, leBounceSoundType_t,
    leMarkType_t, leType_t, lerpFrame_t, localEntity_s, localEntity_t, playerEntity_t, score_t,
    unnamed, FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL,
    FOOTSTEP_NORMAL, FOOTSTEP_SPLASH, FOOTSTEP_TOTAL, LEBS_BLOOD, LEBS_BRASS, LEBS_NONE,
    LEF_PUFF_DONT_SCALE, LEF_SOUND1, LEF_SOUND2, LEF_TUMBLE, LEMT_BLOOD, LEMT_BURN, LEMT_NONE,
    LE_EXPLOSION, LE_FADE_RGB, LE_FALL_SCALE_FADE, LE_FRAGMENT, LE_MARK, LE_MOVE_SCALE_FADE,
    LE_SCALE_FADE, LE_SCOREPLUM, LE_SPRITE_EXPLOSION,
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
    axisDefault, byte, cvarHandle_t, entityState_s, entityState_t, gameState_t, playerState_s,
    playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, vec3_t,
    vec_t, vmCvar_t, AnglesToAxis, AxisClear, AxisCopy, Q_random, RotateAroundDirection,
    VectorNormalize, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::rand;
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

//
// cg_effects.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_SmokePuff(
    mut p: *const vec_t,
    mut vel: *const vec_t,
    mut radius: libc::c_float,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut a: libc::c_float,
    mut duration: libc::c_float,
    mut startTime: libc::c_int,
    mut fadeInTime: libc::c_int,
    mut leFlags: libc::c_int,
    mut hShader: qhandle_t,
) -> *mut localEntity_t {
    static mut seed: libc::c_int = 0x92i32;
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    le = CG_AllocLocalEntity();
    (*le).leFlags = leFlags;
    (*le).radius = radius;
    re = &mut (*le).refEntity;
    (*re).rotation = Q_random(&mut seed) * 360i32 as libc::c_float;
    (*re).radius = radius;
    (*re).shaderTime = startTime as libc::c_float / 1000.0f32;
    (*le).leType = LE_MOVE_SCALE_FADE;
    (*le).startTime = startTime;
    (*le).fadeInTime = fadeInTime;
    (*le).endTime = (startTime as libc::c_float + duration) as libc::c_int;
    if fadeInTime > startTime {
        (*le).lifeRate =
            (1.0f64 / ((*le).endTime - (*le).fadeInTime) as libc::c_double) as libc::c_float
    } else {
        (*le).lifeRate =
            (1.0f64 / ((*le).endTime - (*le).startTime) as libc::c_double) as libc::c_float
    }
    (*le).color[0usize] = r;
    (*le).color[1usize] = g;
    (*le).color[2usize] = b;
    (*le).color[3usize] = a;
    (*le).pos.trType = TR_LINEAR;
    (*le).pos.trTime = startTime;
    (*le).pos.trDelta[0usize] = *vel.offset(0isize);
    (*le).pos.trDelta[1usize] = *vel.offset(1isize);
    (*le).pos.trDelta[2usize] = *vel.offset(2isize);
    (*le).pos.trBase[0usize] = *p.offset(0isize);
    (*le).pos.trBase[1usize] = *p.offset(1isize);
    (*le).pos.trBase[2usize] = *p.offset(2isize);
    (*re).origin[0usize] = *p.offset(0isize);
    (*re).origin[1usize] = *p.offset(1isize);
    (*re).origin[2usize] = *p.offset(2isize);
    (*re).customShader = hShader;
    if cgs.glconfig.hardwareType as libc::c_uint == GLHW_RAGEPRO as libc::c_int as libc::c_uint {
        (*re).customShader = cgs.media.smokePuffRageProShader;
        (*re).shaderRGBA[0usize] = 0xffi32 as byte;
        (*re).shaderRGBA[1usize] = 0xffi32 as byte;
        (*re).shaderRGBA[2usize] = 0xffi32 as byte;
        (*re).shaderRGBA[3usize] = 0xffi32 as byte
    } else {
        (*re).shaderRGBA[0usize] = ((*le).color[0usize] * 0xffi32 as libc::c_float) as byte;
        (*re).shaderRGBA[1usize] = ((*le).color[1usize] * 0xffi32 as libc::c_float) as byte;
        (*re).shaderRGBA[2usize] = ((*le).color[2usize] * 0xffi32 as libc::c_float) as byte;
        (*re).shaderRGBA[3usize] = 0xffi32 as byte
    }
    (*re).reType = RT_SPRITE;
    (*re).radius = (*le).radius;
    return le;
}
#[no_mangle]
pub unsafe extern "C" fn CG_BubbleTrail(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut spacing: libc::c_float,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if 0 != cg_noProjectileTrail.integer {
        return;
    }
    move_0[0usize] = *start.offset(0isize);
    move_0[1usize] = *start.offset(1isize);
    move_0[2usize] = *start.offset(2isize);
    vec[0usize] = *end.offset(0isize) - *start.offset(0isize);
    vec[1usize] = *end.offset(1isize) - *start.offset(1isize);
    vec[2usize] = *end.offset(2isize) - *start.offset(2isize);
    len = VectorNormalize(vec.as_mut_ptr());
    i = rand() % spacing as libc::c_int;
    move_0[0usize] = move_0[0usize] + vec[0usize] * i as libc::c_float;
    move_0[1usize] = move_0[1usize] + vec[1usize] * i as libc::c_float;
    move_0[2usize] = move_0[2usize] + vec[2usize] * i as libc::c_float;
    vec[0usize] = vec[0usize] * spacing;
    vec[1usize] = vec[1usize] * spacing;
    vec[2usize] = vec[2usize] * spacing;
    while (i as libc::c_float) < len {
        let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
        let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
        le = CG_AllocLocalEntity();
        (*le).leFlags = LEF_PUFF_DONT_SCALE as libc::c_int;
        (*le).leType = LE_MOVE_SCALE_FADE;
        (*le).startTime = cg.time;
        (*le).endTime = ((cg.time + 1000i32) as libc::c_float
            + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                * 250i32 as libc::c_float) as libc::c_int;
        (*le).lifeRate =
            (1.0f64 / ((*le).endTime - (*le).startTime) as libc::c_double) as libc::c_float;
        re = &mut (*le).refEntity;
        (*re).shaderTime = cg.time as libc::c_float / 1000.0f32;
        (*re).reType = RT_SPRITE;
        (*re).rotation = 0i32 as libc::c_float;
        (*re).radius = 3i32 as libc::c_float;
        (*re).customShader = cgs.media.waterBubbleShader;
        (*re).shaderRGBA[0usize] = 0xffi32 as byte;
        (*re).shaderRGBA[1usize] = 0xffi32 as byte;
        (*re).shaderRGBA[2usize] = 0xffi32 as byte;
        (*re).shaderRGBA[3usize] = 0xffi32 as byte;
        (*le).color[3usize] = 1.0f64 as libc::c_float;
        (*le).pos.trType = TR_LINEAR;
        (*le).pos.trTime = cg.time;
        (*le).pos.trBase[0usize] = move_0[0usize];
        (*le).pos.trBase[1usize] = move_0[1usize];
        (*le).pos.trBase[2usize] = move_0[2usize];
        (*le).pos.trDelta[0usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 5i32 as libc::c_double) as vec_t;
        (*le).pos.trDelta[1usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 5i32 as libc::c_double) as vec_t;
        (*le).pos.trDelta[2usize] = (2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 5i32 as libc::c_double
            + 6i32 as libc::c_double) as vec_t;
        move_0[0usize] = move_0[0usize] + vec[0usize];
        move_0[1usize] = move_0[1usize] + vec[1usize];
        move_0[2usize] = move_0[2usize] + vec[2usize];
        i = (i as libc::c_float + spacing) as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn CG_SpawnEffect(mut org: *mut vec_t) {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    le = CG_AllocLocalEntity();
    (*le).leFlags = 0i32;
    (*le).leType = LE_FADE_RGB;
    (*le).startTime = cg.time;
    (*le).endTime = cg.time + 500i32;
    (*le).lifeRate =
        (1.0f64 / ((*le).endTime - (*le).startTime) as libc::c_double) as libc::c_float;
    (*le).color[3usize] = 1.0f64 as libc::c_float;
    (*le).color[2usize] = (*le).color[3usize];
    (*le).color[1usize] = (*le).color[2usize];
    (*le).color[0usize] = (*le).color[1usize];
    re = &mut (*le).refEntity;
    (*re).reType = RT_MODEL;
    (*re).shaderTime = cg.time as libc::c_float / 1000.0f32;
    (*re).customShader = cgs.media.teleportEffectShader;
    (*re).hModel = cgs.media.teleportEffectModel;
    AxisClear((*re).axis.as_mut_ptr());
    (*re).origin[0usize] = *org.offset(0isize);
    (*re).origin[1usize] = *org.offset(1isize);
    (*re).origin[2usize] = *org.offset(2isize);
    (*re).origin[2usize] -= 24i32 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ScorePlum(
    mut client: libc::c_int,
    mut org: *mut vec_t,
    mut score: libc::c_int,
) {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut angles: vec3_t = [0.; 3];
    static mut lastPos: vec3_t = [0.; 3];
    if client != cg.predictedPlayerState.clientNum || cg_scorePlum.integer == 0i32 {
        return;
    }
    le = CG_AllocLocalEntity();
    (*le).leFlags = 0i32;
    (*le).leType = LE_SCOREPLUM;
    (*le).startTime = cg.time;
    (*le).endTime = cg.time + 4000i32;
    (*le).lifeRate =
        (1.0f64 / ((*le).endTime - (*le).startTime) as libc::c_double) as libc::c_float;
    (*le).color[3usize] = 1.0f64 as libc::c_float;
    (*le).color[2usize] = (*le).color[3usize];
    (*le).color[1usize] = (*le).color[2usize];
    (*le).color[0usize] = (*le).color[1usize];
    (*le).radius = score as libc::c_float;
    (*le).pos.trBase[0usize] = *org.offset(0isize);
    (*le).pos.trBase[1usize] = *org.offset(1isize);
    (*le).pos.trBase[2usize] = *org.offset(2isize);
    if *org.offset(2isize) >= lastPos[2usize] - 20i32 as libc::c_float
        && *org.offset(2isize) <= lastPos[2usize] + 20i32 as libc::c_float
    {
        (*le).pos.trBase[2usize] -= 20i32 as libc::c_float
    }
    lastPos[0usize] = *org.offset(0isize);
    lastPos[1usize] = *org.offset(1isize);
    lastPos[2usize] = *org.offset(2isize);
    re = &mut (*le).refEntity;
    (*re).reType = RT_SPRITE;
    (*re).radius = 16i32 as libc::c_float;
    angles[2usize] = 0i32 as vec_t;
    angles[1usize] = angles[2usize];
    angles[0usize] = angles[1usize];
    AnglesToAxis(angles.as_mut_ptr() as *const vec_t, (*re).axis.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CG_GibPlayer(mut playerOrigin: *mut vec_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
    if 0 == cg_blood.integer {
        return;
    }
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    if 0 != rand() & 1i32 {
        CG_LaunchGib(
            origin.as_mut_ptr(),
            velocity.as_mut_ptr(),
            cgs.media.gibSkull,
        );
    } else {
        CG_LaunchGib(
            origin.as_mut_ptr(),
            velocity.as_mut_ptr(),
            cgs.media.gibBrain,
        );
    }
    if 0 == cg_gibs.integer {
        return;
    }
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        cgs.media.gibAbdomen,
    );
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.gibArm);
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        cgs.media.gibChest,
    );
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        cgs.media.gibFist,
    );
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        cgs.media.gibFoot,
    );
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        cgs.media.gibForearm,
    );
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(
        origin.as_mut_ptr(),
        velocity.as_mut_ptr(),
        cgs.media.gibIntestine,
    );
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.gibLeg);
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 250i32 as libc::c_double) as vec_t;
    velocity[2usize] = (250i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 250i32 as libc::c_double) as vec_t;
    CG_LaunchGib(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.gibLeg);
}
/*
==================
CG_LaunchGib
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_LaunchGib(
    mut origin: *mut vec_t,
    mut velocity: *mut vec_t,
    mut hModel: qhandle_t,
) {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    le = CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    (*le).leType = LE_FRAGMENT;
    (*le).startTime = cg.time;
    (*le).endTime = (((*le).startTime + 5000i32) as libc::c_float
        + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
            * 3000i32 as libc::c_float) as libc::c_int;
    (*re).origin[0usize] = *origin.offset(0isize);
    (*re).origin[1usize] = *origin.offset(1isize);
    (*re).origin[2usize] = *origin.offset(2isize);
    AxisCopy(axisDefault.as_mut_ptr(), (*re).axis.as_mut_ptr());
    (*re).hModel = hModel;
    (*le).pos.trType = TR_GRAVITY;
    (*le).pos.trBase[0usize] = *origin.offset(0isize);
    (*le).pos.trBase[1usize] = *origin.offset(1isize);
    (*le).pos.trBase[2usize] = *origin.offset(2isize);
    (*le).pos.trDelta[0usize] = *velocity.offset(0isize);
    (*le).pos.trDelta[1usize] = *velocity.offset(1isize);
    (*le).pos.trDelta[2usize] = *velocity.offset(2isize);
    (*le).pos.trTime = cg.time;
    (*le).bounceFactor = 0.6f32;
    (*le).leBounceSoundType = LEBS_BLOOD;
    (*le).leMarkType = LEMT_BLOOD;
}
#[no_mangle]
pub unsafe extern "C" fn CG_BigExplode(mut playerOrigin: *mut vec_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut velocity: vec3_t = [0.; 3];
    if 0 == cg_blood.integer {
        return;
    }
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double) as vec_t;
    velocity[2usize] = (150i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 100i32 as libc::c_double) as vec_t;
    CG_LaunchExplode(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.smoke2);
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double) as vec_t;
    velocity[2usize] = (150i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 100i32 as libc::c_double) as vec_t;
    CG_LaunchExplode(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.smoke2);
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double
        * 1.5f64) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double
        * 1.5f64) as vec_t;
    velocity[2usize] = (150i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 100i32 as libc::c_double) as vec_t;
    CG_LaunchExplode(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.smoke2);
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double
        * 2.0f64) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double
        * 2.0f64) as vec_t;
    velocity[2usize] = (150i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 100i32 as libc::c_double) as vec_t;
    CG_LaunchExplode(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.smoke2);
    origin[0usize] = *playerOrigin.offset(0isize);
    origin[1usize] = *playerOrigin.offset(1isize);
    origin[2usize] = *playerOrigin.offset(2isize);
    velocity[0usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double
        * 2.5f64) as vec_t;
    velocity[1usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 100i32 as libc::c_double
        * 2.5f64) as vec_t;
    velocity[2usize] = (150i32 as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                as libc::c_double
                - 0.5f64)
            * 100i32 as libc::c_double) as vec_t;
    CG_LaunchExplode(origin.as_mut_ptr(), velocity.as_mut_ptr(), cgs.media.smoke2);
}
/*
==================
CG_LaunchExplode
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_LaunchExplode(
    mut origin: *mut vec_t,
    mut velocity: *mut vec_t,
    mut hModel: qhandle_t,
) {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    le = CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    (*le).leType = LE_FRAGMENT;
    (*le).startTime = cg.time;
    (*le).endTime = (((*le).startTime + 10000i32) as libc::c_float
        + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
            * 6000i32 as libc::c_float) as libc::c_int;
    (*re).origin[0usize] = *origin.offset(0isize);
    (*re).origin[1usize] = *origin.offset(1isize);
    (*re).origin[2usize] = *origin.offset(2isize);
    AxisCopy(axisDefault.as_mut_ptr(), (*re).axis.as_mut_ptr());
    (*re).hModel = hModel;
    (*le).pos.trType = TR_GRAVITY;
    (*le).pos.trBase[0usize] = *origin.offset(0isize);
    (*le).pos.trBase[1usize] = *origin.offset(1isize);
    (*le).pos.trBase[2usize] = *origin.offset(2isize);
    (*le).pos.trDelta[0usize] = *velocity.offset(0isize);
    (*le).pos.trDelta[1usize] = *velocity.offset(1isize);
    (*le).pos.trDelta[2usize] = *velocity.offset(2isize);
    (*le).pos.trTime = cg.time;
    (*le).bounceFactor = 0.1f32;
    (*le).leBounceSoundType = LEBS_BRASS;
    (*le).leMarkType = LEMT_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn CG_Bleed(mut origin: *mut vec_t, mut entityNum: libc::c_int) {
    let mut ex: *mut localEntity_t = 0 as *mut localEntity_t;
    if 0 == cg_blood.integer {
        return;
    }
    ex = CG_AllocLocalEntity();
    (*ex).leType = LE_EXPLOSION;
    (*ex).startTime = cg.time;
    (*ex).endTime = (*ex).startTime + 500i32;
    (*ex).refEntity.origin[0usize] = *origin.offset(0isize);
    (*ex).refEntity.origin[1usize] = *origin.offset(1isize);
    (*ex).refEntity.origin[2usize] = *origin.offset(2isize);
    (*ex).refEntity.reType = RT_SPRITE;
    (*ex).refEntity.rotation = (rand() % 360i32) as libc::c_float;
    (*ex).refEntity.radius = 24i32 as libc::c_float;
    (*ex).refEntity.customShader = cgs.media.bloodExplosionShader;
    if entityNum == (*cg.snap).ps.clientNum {
        (*ex).refEntity.renderfx |= 0x2i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_MakeExplosion(
    mut origin: *mut vec_t,
    mut dir: *mut vec_t,
    mut hModel: qhandle_t,
    mut shader: qhandle_t,
    mut msec: libc::c_int,
    mut isSprite: qboolean,
) -> *mut localEntity_t {
    let mut ang: libc::c_float = 0.;
    let mut ex: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut offset: libc::c_int = 0;
    let mut tmpVec: vec3_t = [0.; 3];
    let mut newOrigin: vec3_t = [0.; 3];
    if msec <= 0i32 {
        CG_Error(
            b"CG_MakeExplosion: msec = %i\x00" as *const u8 as *const libc::c_char,
            msec,
        );
    }
    offset = rand() & 63i32;
    ex = CG_AllocLocalEntity();
    if 0 != isSprite as u64 {
        (*ex).leType = LE_SPRITE_EXPLOSION;
        (*ex).refEntity.rotation = (rand() % 360i32) as libc::c_float;
        tmpVec[0usize] = *dir.offset(0isize) * 16i32 as libc::c_float;
        tmpVec[1usize] = *dir.offset(1isize) * 16i32 as libc::c_float;
        tmpVec[2usize] = *dir.offset(2isize) * 16i32 as libc::c_float;
        newOrigin[0usize] = tmpVec[0usize] + *origin.offset(0isize);
        newOrigin[1usize] = tmpVec[1usize] + *origin.offset(1isize);
        newOrigin[2usize] = tmpVec[2usize] + *origin.offset(2isize)
    } else {
        (*ex).leType = LE_EXPLOSION;
        newOrigin[0usize] = *origin.offset(0isize);
        newOrigin[1usize] = *origin.offset(1isize);
        newOrigin[2usize] = *origin.offset(2isize);
        if dir.is_null() {
            AxisClear((*ex).refEntity.axis.as_mut_ptr());
        } else {
            ang = (rand() % 360i32) as libc::c_float;
            (*ex).refEntity.axis[0usize][0usize] = *dir.offset(0isize);
            (*ex).refEntity.axis[0usize][1usize] = *dir.offset(1isize);
            (*ex).refEntity.axis[0usize][2usize] = *dir.offset(2isize);
            RotateAroundDirection((*ex).refEntity.axis.as_mut_ptr(), ang);
        }
    }
    (*ex).startTime = cg.time - offset;
    (*ex).endTime = (*ex).startTime + msec;
    (*ex).refEntity.shaderTime = (*ex).startTime as libc::c_float / 1000.0f32;
    (*ex).refEntity.hModel = hModel;
    (*ex).refEntity.customShader = shader;
    (*ex).refEntity.origin[0usize] = newOrigin[0usize];
    (*ex).refEntity.origin[1usize] = newOrigin[1usize];
    (*ex).refEntity.origin[2usize] = newOrigin[2usize];
    (*ex).refEntity.oldorigin[0usize] = newOrigin[0usize];
    (*ex).refEntity.oldorigin[1usize] = newOrigin[1usize];
    (*ex).refEntity.oldorigin[2usize] = newOrigin[2usize];
    (*ex).color[2usize] = 1.0f64 as libc::c_float;
    (*ex).color[1usize] = (*ex).color[2usize];
    (*ex).color[0usize] = (*ex).color[1usize];
    return ex;
}
