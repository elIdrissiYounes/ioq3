use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, team_t, unnamed_0, unnamed_1, unnamed_2,
    GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, PERS_ASSIST_COUNT,
    PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, STAT_ARMOR, STAT_CLIENTS_READY,
    STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE,
    TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, lerpFrame_t,
    playerEntity_t, score_t, trap_Argc, trap_Cvar_Set, trap_R_AddRefEntityToScene,
    trap_R_ClearScene, trap_R_RegisterModel, trap_S_ClearLoopingSounds, trap_S_Respatialize,
    trap_S_StartLocalSound, trap_S_StartSound, trap_SetUserCmdValue, FOOTSTEP_BOOT,
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
use cg_variadic_h::CG_Printf;
use cg_weapons::{
    CG_AddPlayerWeapon, CG_AddViewWeapon, CG_Bullet, CG_DrawWeaponSelect, CG_FireWeapon,
    CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f, CG_OutOfAmmoChange,
    CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire, CG_Weapon_f,
};
use libc;
use q_shared_h::{
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, gameState_t,
    playerState_s, playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t,
    trace_t, trajectory_t, unnamed, va, vec3_t, vec_t, vmCvar_t, AngleVectors, AnglesToAxis,
    Q_strncpyz, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND,
    CHAN_VOICE, CHAN_WEAPON, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE,
    TR_STATIONARY,
};
use stdlib::{atan2, atof, cos, fabs, memcpy, memset, sin, sqrt, tan};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    stereoFrame_t, textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D,
    GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING,
    RT_MAX_REF_ENTITY_TYPE, RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS,
    RT_SPRITE, STEREO_CENTER, STEREO_LEFT, STEREO_RIGHT, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};

//
// cg_view.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_TestModel_f() {
    let mut angles: vec3_t = [0.; 3];
    cg.testGun = qfalse;
    memset(
        &mut cg.testModelEntity as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    if trap_Argc() < 2i32 {
        return;
    }
    Q_strncpyz(cg.testModelName.as_mut_ptr(), CG_Argv(1i32), 64i32);
    cg.testModelEntity.hModel = trap_R_RegisterModel(cg.testModelName.as_mut_ptr());
    if trap_Argc() == 3i32 {
        cg.testModelEntity.backlerp = atof(CG_Argv(2i32)) as libc::c_float;
        cg.testModelEntity.frame = 1i32;
        cg.testModelEntity.oldframe = 0i32
    }
    if 0 == cg.testModelEntity.hModel {
        CG_Printf(b"Can\'t register model\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    cg.testModelEntity.origin[0usize] =
        cg.refdef.vieworg[0usize] + cg.refdef.viewaxis[0usize][0usize] * 100i32 as libc::c_float;
    cg.testModelEntity.origin[1usize] =
        cg.refdef.vieworg[1usize] + cg.refdef.viewaxis[0usize][1usize] * 100i32 as libc::c_float;
    cg.testModelEntity.origin[2usize] =
        cg.refdef.vieworg[2usize] + cg.refdef.viewaxis[0usize][2usize] * 100i32 as libc::c_float;
    angles[0usize] = 0i32 as vec_t;
    angles[1usize] = 180i32 as libc::c_float + cg.refdefViewAngles[1usize];
    angles[2usize] = 0i32 as vec_t;
    AnglesToAxis(
        angles.as_mut_ptr() as *const vec_t,
        cg.testModelEntity.axis.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_TestGun_f() {
    CG_TestModel_f();
    if 0 == cg.testModelEntity.hModel {
        return;
    }
    cg.testGun = qtrue;
    cg.testModelEntity.renderfx = 0x1i32 | 0x8i32 | 0x4i32;
}
#[no_mangle]
pub unsafe extern "C" fn CG_TestModelNextFrame_f() {
    cg.testModelEntity.frame += 1;
    CG_Printf(
        b"frame %i\n\x00" as *const u8 as *const libc::c_char,
        cg.testModelEntity.frame,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_TestModelPrevFrame_f() {
    cg.testModelEntity.frame -= 1;
    if cg.testModelEntity.frame < 0i32 {
        cg.testModelEntity.frame = 0i32
    }
    CG_Printf(
        b"frame %i\n\x00" as *const u8 as *const libc::c_char,
        cg.testModelEntity.frame,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_TestModelNextSkin_f() {
    cg.testModelEntity.skinNum += 1;
    CG_Printf(
        b"skin %i\n\x00" as *const u8 as *const libc::c_char,
        cg.testModelEntity.skinNum,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_TestModelPrevSkin_f() {
    cg.testModelEntity.skinNum -= 1;
    if cg.testModelEntity.skinNum < 0i32 {
        cg.testModelEntity.skinNum = 0i32
    }
    CG_Printf(
        b"skin %i\n\x00" as *const u8 as *const libc::c_char,
        cg.testModelEntity.skinNum,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_ZoomDown_f() {
    if 0 != cg.zoomed as u64 {
        return;
    }
    cg.zoomed = qtrue;
    cg.zoomTime = cg.time;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ZoomUp_f() {
    if 0 == cg.zoomed as u64 {
        return;
    }
    cg.zoomed = qfalse;
    cg.zoomTime = cg.time;
}
#[no_mangle]
pub unsafe extern "C" fn CG_AddBufferedSound(mut sfx: sfxHandle_t) {
    if 0 == sfx {
        return;
    }
    cg.soundBuffer[cg.soundBufferIn as usize] = sfx;
    cg.soundBufferIn = (cg.soundBufferIn + 1i32) % 20i32;
    if cg.soundBufferIn == cg.soundBufferOut {
        cg.soundBufferOut += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawActiveFrame(
    mut serverTime: libc::c_int,
    mut stereoView: stereoFrame_t,
    mut demoPlayback: qboolean,
) {
    let mut inwater: libc::c_int = 0;
    cg.time = serverTime;
    cg.demoPlayback = demoPlayback;
    CG_UpdateCvars();
    if cg.infoScreenText[0usize] as libc::c_int != 0i32 {
        CG_DrawInformation();
        return;
    }
    trap_S_ClearLoopingSounds(qfalse);
    trap_R_ClearScene();
    CG_ProcessSnapshots();
    if cg.snap.is_null() || 0 != (*cg.snap).snapFlags & 2i32 {
        CG_DrawInformation();
        return;
    }
    trap_SetUserCmdValue(cg.weaponSelect, cg.zoomSensitivity);
    cg.clientFrame += 1;
    CG_PredictPlayerState();
    cg.renderingThirdPerson = ((*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
        != TEAM_SPECTATOR as libc::c_int
        && (0 != cg_thirdPerson.integer
            || (*cg.snap).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32))
        as libc::c_int as qboolean;
    inwater = CG_CalcViewValues();
    if 0 == cg.renderingThirdPerson as u64 {
        CG_DamageBlendBlob();
    }
    if 0 == cg.hyperspace as u64 {
        CG_AddPacketEntities();
        CG_AddMarks();
        CG_AddParticles();
        CG_AddLocalEntities();
    }
    CG_AddViewWeapon(&mut cg.predictedPlayerState);
    CG_PlayBufferedSounds();
    if 0 != cg.testModelEntity.hModel {
        CG_AddTestModel();
    }
    cg.refdef.time = cg.time;
    memcpy(
        cg.refdef.areamask.as_mut_ptr() as *mut libc::c_void,
        (*cg.snap).areamask.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[byte; 32]>() as libc::c_ulong,
    );
    CG_PowerupTimerSounds();
    trap_S_Respatialize(
        (*cg.snap).ps.clientNum,
        cg.refdef.vieworg.as_mut_ptr() as *const vec_t,
        cg.refdef.viewaxis.as_mut_ptr(),
        inwater,
    );
    if stereoView as libc::c_uint != STEREO_RIGHT as libc::c_int as libc::c_uint {
        cg.frametime = cg.time - cg.oldTime;
        if cg.frametime < 0i32 {
            cg.frametime = 0i32
        }
        cg.oldTime = cg.time;
        CG_AddLagometerFrameInfo();
    }
    if cg_timescale.value != cg_timescaleFadeEnd.value {
        if cg_timescale.value < cg_timescaleFadeEnd.value {
            cg_timescale.value += cg_timescaleFadeSpeed.value * cg.frametime as libc::c_float
                / 1000i32 as libc::c_float;
            if cg_timescale.value > cg_timescaleFadeEnd.value {
                cg_timescale.value = cg_timescaleFadeEnd.value
            }
        } else {
            cg_timescale.value -= cg_timescaleFadeSpeed.value * cg.frametime as libc::c_float
                / 1000i32 as libc::c_float;
            if cg_timescale.value < cg_timescaleFadeEnd.value {
                cg_timescale.value = cg_timescaleFadeEnd.value
            }
        }
        if 0. != cg_timescaleFadeSpeed.value {
            trap_Cvar_Set(
                b"timescale\x00" as *const u8 as *const libc::c_char,
                va(
                    b"%f\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cg_timescale.value as libc::c_double,
                ),
            );
        }
    }
    CG_DrawActive(stereoView);
    if 0 != cg_stats.integer {
        CG_Printf(
            b"cg.clientFrame:%i\n\x00" as *const u8 as *const libc::c_char,
            cg.clientFrame,
        );
    };
}
/*
=====================
CG_PowerupTimerSounds
=====================
*/
unsafe extern "C" fn CG_PowerupTimerSounds() {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    i = 0i32;
    while i < 16i32 {
        t = (*cg.snap).ps.powerups[i as usize];
        if !(t <= cg.time) {
            if !(t - cg.time >= 5i32 * 1000i32) {
                if (t - cg.time) / 1000i32 != (t - cg.oldTime) / 1000i32 {
                    trap_S_StartSound(
                        0 as *mut vec_t,
                        (*cg.snap).ps.clientNum,
                        CHAN_ITEM as libc::c_int,
                        cgs.media.wearOffSound,
                    );
                }
            }
        }
        i += 1
    }
}
unsafe extern "C" fn CG_AddTestModel() {
    let mut i: libc::c_int = 0;
    cg.testModelEntity.hModel = trap_R_RegisterModel(cg.testModelName.as_mut_ptr());
    if 0 == cg.testModelEntity.hModel {
        CG_Printf(b"Can\'t register model\n\x00" as *const u8 as *const libc::c_char);
        return;
    }
    if 0 != cg.testGun as u64 {
        cg.testModelEntity.origin[0usize] = cg.refdef.vieworg[0usize];
        cg.testModelEntity.origin[1usize] = cg.refdef.vieworg[1usize];
        cg.testModelEntity.origin[2usize] = cg.refdef.vieworg[2usize];
        cg.testModelEntity.axis[0usize][0usize] = cg.refdef.viewaxis[0usize][0usize];
        cg.testModelEntity.axis[0usize][1usize] = cg.refdef.viewaxis[0usize][1usize];
        cg.testModelEntity.axis[0usize][2usize] = cg.refdef.viewaxis[0usize][2usize];
        cg.testModelEntity.axis[1usize][0usize] = cg.refdef.viewaxis[1usize][0usize];
        cg.testModelEntity.axis[1usize][1usize] = cg.refdef.viewaxis[1usize][1usize];
        cg.testModelEntity.axis[1usize][2usize] = cg.refdef.viewaxis[1usize][2usize];
        cg.testModelEntity.axis[2usize][0usize] = cg.refdef.viewaxis[2usize][0usize];
        cg.testModelEntity.axis[2usize][1usize] = cg.refdef.viewaxis[2usize][1usize];
        cg.testModelEntity.axis[2usize][2usize] = cg.refdef.viewaxis[2usize][2usize];
        i = 0i32;
        while i < 3i32 {
            cg.testModelEntity.origin[i as usize] +=
                cg.refdef.viewaxis[0usize][i as usize] * cg_gun_x.value;
            cg.testModelEntity.origin[i as usize] +=
                cg.refdef.viewaxis[1usize][i as usize] * cg_gun_y.value;
            cg.testModelEntity.origin[i as usize] +=
                cg.refdef.viewaxis[2usize][i as usize] * cg_gun_z.value;
            i += 1
        }
    }
    trap_R_AddRefEntityToScene(&mut cg.testModelEntity);
}
/*
=====================
CG_PlayBufferedSounds
=====================
*/
unsafe extern "C" fn CG_PlayBufferedSounds() {
    if cg.soundTime < cg.time {
        if cg.soundBufferOut != cg.soundBufferIn && 0 != cg.soundBuffer[cg.soundBufferOut as usize]
        {
            trap_S_StartLocalSound(
                cg.soundBuffer[cg.soundBufferOut as usize],
                CHAN_ANNOUNCER as libc::c_int,
            );
            cg.soundBuffer[cg.soundBufferOut as usize] = 0i32;
            cg.soundBufferOut = (cg.soundBufferOut + 1i32) % 20i32;
            cg.soundTime = cg.time + 750i32
        }
    };
}
/*
===============
CG_DamageBlendBlob

===============
*/
unsafe extern "C" fn CG_DamageBlendBlob() {
    let mut t: libc::c_int = 0;
    let mut maxTime: libc::c_int = 0;
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
    if 0 == cg_blood.integer {
        return;
    }
    if 0. == cg.damageValue {
        return;
    }
    if cgs.glconfig.hardwareType as libc::c_uint == GLHW_RAGEPRO as libc::c_int as libc::c_uint {
        return;
    }
    maxTime = 500i32;
    t = (cg.time as libc::c_float - cg.damageTime) as libc::c_int;
    if t <= 0i32 || t >= maxTime {
        return;
    }
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    ent.reType = RT_SPRITE;
    ent.renderfx = 0x4i32;
    ent.origin[0usize] =
        cg.refdef.vieworg[0usize] + cg.refdef.viewaxis[0usize][0usize] * 8i32 as libc::c_float;
    ent.origin[1usize] =
        cg.refdef.vieworg[1usize] + cg.refdef.viewaxis[0usize][1usize] * 8i32 as libc::c_float;
    ent.origin[2usize] =
        cg.refdef.vieworg[2usize] + cg.refdef.viewaxis[0usize][2usize] * 8i32 as libc::c_float;
    ent.origin[0usize] = ent.origin[0usize]
        + cg.refdef.viewaxis[1usize][0usize] * (cg.damageX * -8i32 as libc::c_float);
    ent.origin[1usize] = ent.origin[1usize]
        + cg.refdef.viewaxis[1usize][1usize] * (cg.damageX * -8i32 as libc::c_float);
    ent.origin[2usize] = ent.origin[2usize]
        + cg.refdef.viewaxis[1usize][2usize] * (cg.damageX * -8i32 as libc::c_float);
    ent.origin[0usize] = ent.origin[0usize]
        + cg.refdef.viewaxis[2usize][0usize] * (cg.damageY * 8i32 as libc::c_float);
    ent.origin[1usize] = ent.origin[1usize]
        + cg.refdef.viewaxis[2usize][1usize] * (cg.damageY * 8i32 as libc::c_float);
    ent.origin[2usize] = ent.origin[2usize]
        + cg.refdef.viewaxis[2usize][2usize] * (cg.damageY * 8i32 as libc::c_float);
    ent.radius = cg.damageValue * 3i32 as libc::c_float;
    ent.customShader = cgs.media.viewBloodShader;
    ent.shaderRGBA[0usize] = 255i32 as byte;
    ent.shaderRGBA[1usize] = 255i32 as byte;
    ent.shaderRGBA[2usize] = 255i32 as byte;
    ent.shaderRGBA[3usize] = (200i32 as libc::c_double
        * (1.0f64 - (t as libc::c_float / maxTime as libc::c_float) as libc::c_double))
        as byte;
    trap_R_AddRefEntityToScene(&mut ent);
}
/*
===============
CG_CalcViewValues

Sets cg.refdef view values
===============
*/
unsafe extern "C" fn CG_CalcViewValues() -> libc::c_int {
    let mut ps: *mut playerState_t = 0 as *mut playerState_t;
    memset(
        &mut cg.refdef as *mut refdef_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refdef_t>() as libc::c_ulong,
    );
    CG_CalcVrect();
    ps = &mut cg.predictedPlayerState;
    if (*ps).pm_type == PM_INTERMISSION as libc::c_int {
        cg.refdef.vieworg[0usize] = (*ps).origin[0usize];
        cg.refdef.vieworg[1usize] = (*ps).origin[1usize];
        cg.refdef.vieworg[2usize] = (*ps).origin[2usize];
        cg.refdefViewAngles[0usize] = (*ps).viewangles[0usize];
        cg.refdefViewAngles[1usize] = (*ps).viewangles[1usize];
        cg.refdefViewAngles[2usize] = (*ps).viewangles[2usize];
        AnglesToAxis(
            cg.refdefViewAngles.as_mut_ptr() as *const vec_t,
            cg.refdef.viewaxis.as_mut_ptr(),
        );
        return CG_CalcFov();
    }
    cg.bobcycle = ((*ps).bobCycle & 128i32) >> 7i32;
    cg.bobfracsin = fabs(sin(
        ((*ps).bobCycle & 127i32) as libc::c_double / 127.0f64 * 3.14159265358979323846f64
    )) as libc::c_float;
    cg.xyspeed = sqrt(
        ((*ps).velocity[0usize] * (*ps).velocity[0usize]
            + (*ps).velocity[1usize] * (*ps).velocity[1usize]) as libc::c_double,
    ) as libc::c_float;
    cg.refdef.vieworg[0usize] = (*ps).origin[0usize];
    cg.refdef.vieworg[1usize] = (*ps).origin[1usize];
    cg.refdef.vieworg[2usize] = (*ps).origin[2usize];
    cg.refdefViewAngles[0usize] = (*ps).viewangles[0usize];
    cg.refdefViewAngles[1usize] = (*ps).viewangles[1usize];
    cg.refdefViewAngles[2usize] = (*ps).viewangles[2usize];
    if 0 != cg_cameraOrbit.integer {
        if cg.time > cg.nextOrbitTime {
            cg.nextOrbitTime = cg.time + cg_cameraOrbitDelay.integer;
            cg_thirdPersonAngle.value += cg_cameraOrbit.value
        }
    }
    if cg_errorDecay.value > 0i32 as libc::c_float {
        let mut t: libc::c_int = 0;
        let mut f: libc::c_float = 0.;
        t = cg.time - cg.predictedErrorTime;
        f = (cg_errorDecay.value - t as libc::c_float) / cg_errorDecay.value;
        if f > 0i32 as libc::c_float && f < 1i32 as libc::c_float {
            cg.refdef.vieworg[0usize] = cg.refdef.vieworg[0usize] + cg.predictedError[0usize] * f;
            cg.refdef.vieworg[1usize] = cg.refdef.vieworg[1usize] + cg.predictedError[1usize] * f;
            cg.refdef.vieworg[2usize] = cg.refdef.vieworg[2usize] + cg.predictedError[2usize] * f
        } else {
            cg.predictedErrorTime = 0i32
        }
    }
    if 0 != cg.renderingThirdPerson as u64 {
        CG_OffsetThirdPersonView();
    } else {
        CG_OffsetFirstPersonView();
    }
    AnglesToAxis(
        cg.refdefViewAngles.as_mut_ptr() as *const vec_t,
        cg.refdef.viewaxis.as_mut_ptr(),
    );
    if 0 != cg.hyperspace as u64 {
        cg.refdef.rdflags |= 0x1i32 | 0x4i32
    }
    return CG_CalcFov();
}
/*
====================
CG_CalcFov

Fixed fov at intermissions, otherwise account for fov variable and zooms.
====================
*/
unsafe extern "C" fn CG_CalcFov() -> libc::c_int {
    let mut x: libc::c_float = 0.;
    let mut phase: libc::c_float = 0.;
    let mut v: libc::c_float = 0.;
    let mut contents: libc::c_int = 0;
    let mut fov_x: libc::c_float = 0.;
    let mut fov_y: libc::c_float = 0.;
    let mut zoomFov: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    let mut inwater: libc::c_int = 0;
    if cg.predictedPlayerState.pm_type == PM_INTERMISSION as libc::c_int {
        fov_x = 90i32 as libc::c_float
    } else {
        if 0 != cgs.dmflags & 16i32 {
            fov_x = 90i32 as libc::c_float
        } else {
            fov_x = cg_fov.value;
            if fov_x < 1i32 as libc::c_float {
                fov_x = 1i32 as libc::c_float
            } else if fov_x > 160i32 as libc::c_float {
                fov_x = 160i32 as libc::c_float
            }
        }
        zoomFov = cg_zoomFov.value;
        if zoomFov < 1i32 as libc::c_float {
            zoomFov = 1i32 as libc::c_float
        } else if zoomFov > 160i32 as libc::c_float {
            zoomFov = 160i32 as libc::c_float
        }
        if 0 != cg.zoomed as u64 {
            f = (cg.time - cg.zoomTime) as libc::c_float / 150i32 as libc::c_float;
            if f as libc::c_double > 1.0f64 {
                fov_x = zoomFov
            } else {
                fov_x = fov_x + f * (zoomFov - fov_x)
            }
        } else {
            f = (cg.time - cg.zoomTime) as libc::c_float / 150i32 as libc::c_float;
            if f as libc::c_double <= 1.0f64 {
                fov_x = zoomFov + f * (fov_x - zoomFov)
            }
        }
    }
    x = (cg.refdef.width as libc::c_double
        / tan((fov_x / 360i32 as libc::c_float) as libc::c_double * 3.14159265358979323846f64))
        as libc::c_float;
    fov_y = atan2(cg.refdef.height as libc::c_double, x as libc::c_double) as libc::c_float;
    fov_y = ((fov_y * 360i32 as libc::c_float) as libc::c_double / 3.14159265358979323846f64)
        as libc::c_float;
    contents = CG_PointContents(cg.refdef.vieworg.as_mut_ptr() as *const vec_t, -1i32);
    if 0 != contents & (32i32 | 16i32 | 8i32) {
        phase = (cg.time as libc::c_double / 1000.0f64
            * 0.4f64
            * 3.14159265358979323846f64
            * 2i32 as libc::c_double) as libc::c_float;
        v = (1i32 as libc::c_double * sin(phase as libc::c_double)) as libc::c_float;
        fov_x += v;
        fov_y -= v;
        inwater = qtrue as libc::c_int
    } else {
        inwater = qfalse as libc::c_int
    }
    cg.refdef.fov_x = fov_x;
    cg.refdef.fov_y = fov_y;
    if 0 == cg.zoomed as u64 {
        cg.zoomSensitivity = 1i32 as libc::c_float
    } else {
        cg.zoomSensitivity = (cg.refdef.fov_y as libc::c_double / 75.0f64) as libc::c_float
    }
    return inwater;
}
/*
===============
CG_OffsetFirstPersonView

===============
*/
unsafe extern "C" fn CG_OffsetFirstPersonView() {
    let mut origin: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut bob: libc::c_float = 0.;
    let mut ratio: libc::c_float = 0.;
    let mut delta: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    let mut predictedVelocity: vec3_t = [0.; 3];
    let mut timeDelta: libc::c_int = 0;
    if (*cg.snap).ps.pm_type == PM_INTERMISSION as libc::c_int {
        return;
    }
    origin = cg.refdef.vieworg.as_mut_ptr();
    angles = cg.refdefViewAngles.as_mut_ptr();
    if (*cg.snap).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        *angles.offset(2isize) = 40i32 as libc::c_float;
        *angles.offset(0isize) = -15i32 as libc::c_float;
        *angles.offset(1isize) =
            (*cg.snap).ps.stats[STAT_DEAD_YAW as libc::c_int as usize] as libc::c_float;
        *origin.offset(2isize) += cg.predictedPlayerState.viewheight as libc::c_float;
        return;
    }
    if 0. != cg.damageTime {
        ratio = cg.time as libc::c_float - cg.damageTime;
        if ratio < 100i32 as libc::c_float {
            ratio /= 100i32 as libc::c_float;
            *angles.offset(0isize) += ratio * cg.v_dmg_pitch;
            *angles.offset(2isize) += ratio * cg.v_dmg_roll
        } else {
            ratio = (1.0f64
                - ((ratio - 100i32 as libc::c_float) / 400i32 as libc::c_float) as libc::c_double)
                as libc::c_float;
            if ratio > 0i32 as libc::c_float {
                *angles.offset(0isize) += ratio * cg.v_dmg_pitch;
                *angles.offset(2isize) += ratio * cg.v_dmg_roll
            }
        }
    }
    predictedVelocity[0usize] = cg.predictedPlayerState.velocity[0usize];
    predictedVelocity[1usize] = cg.predictedPlayerState.velocity[1usize];
    predictedVelocity[2usize] = cg.predictedPlayerState.velocity[2usize];
    delta = predictedVelocity[0usize] * cg.refdef.viewaxis[0usize][0usize]
        + predictedVelocity[1usize] * cg.refdef.viewaxis[0usize][1usize]
        + predictedVelocity[2usize] * cg.refdef.viewaxis[0usize][2usize];
    *angles.offset(0isize) += delta * cg_runpitch.value;
    delta = predictedVelocity[0usize] * cg.refdef.viewaxis[1usize][0usize]
        + predictedVelocity[1usize] * cg.refdef.viewaxis[1usize][1usize]
        + predictedVelocity[2usize] * cg.refdef.viewaxis[1usize][2usize];
    *angles.offset(2isize) -= delta * cg_runroll.value;
    speed = if cg.xyspeed > 200i32 as libc::c_float {
        cg.xyspeed
    } else {
        200i32 as libc::c_float
    };
    delta = cg.bobfracsin * cg_bobpitch.value * speed;
    if 0 != cg.predictedPlayerState.pm_flags & 1i32 {
        delta *= 3i32 as libc::c_float
    }
    *angles.offset(0isize) += delta;
    delta = cg.bobfracsin * cg_bobroll.value * speed;
    if 0 != cg.predictedPlayerState.pm_flags & 1i32 {
        delta *= 3i32 as libc::c_float
    }
    if 0 != cg.bobcycle & 1i32 {
        delta = -delta
    }
    *angles.offset(2isize) += delta;
    *origin.offset(2isize) += cg.predictedPlayerState.viewheight as libc::c_float;
    timeDelta = cg.time - cg.duckTime;
    if timeDelta < 100i32 {
        cg.refdef.vieworg[2usize] -=
            cg.duckChange * (100i32 - timeDelta) as libc::c_float / 100i32 as libc::c_float
    }
    bob = cg.bobfracsin * cg.xyspeed * cg_bobup.value;
    if bob > 6i32 as libc::c_float {
        bob = 6i32 as libc::c_float
    }
    *origin.offset(2isize) += bob;
    delta = (cg.time - cg.landTime) as libc::c_float;
    if delta < 150i32 as libc::c_float {
        f = delta / 150i32 as libc::c_float;
        cg.refdef.vieworg[2usize] += cg.landChange * f
    } else if delta < (150i32 + 300i32) as libc::c_float {
        delta -= 150i32 as libc::c_float;
        f = (1.0f64 - (delta / 300i32 as libc::c_float) as libc::c_double) as libc::c_float;
        cg.refdef.vieworg[2usize] += cg.landChange * f
    }
    CG_StepOffset();
}
// this causes a compiler bug on mac MrC compiler
unsafe extern "C" fn CG_StepOffset() {
    let mut timeDelta: libc::c_int = 0;
    timeDelta = cg.time - cg.stepTime;
    if timeDelta < 200i32 {
        cg.refdef.vieworg[2usize] -=
            cg.stepChange * (200i32 - timeDelta) as libc::c_float / 200i32 as libc::c_float
    };
}
//==============================================================================
/*
===============
CG_OffsetThirdPersonView

===============
*/
unsafe extern "C" fn CG_OffsetThirdPersonView() {
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut view: vec3_t = [0.; 3];
    let mut focusAngles: vec3_t = [0.; 3];
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
    static mut mins: vec3_t = [-4i32 as vec_t, -4i32 as vec_t, -4i32 as vec_t];
    static mut maxs: vec3_t = [4i32 as vec_t, 4i32 as vec_t, 4i32 as vec_t];
    let mut focusPoint: vec3_t = [0.; 3];
    let mut focusDist: libc::c_float = 0.;
    let mut forwardScale: libc::c_float = 0.;
    let mut sideScale: libc::c_float = 0.;
    cg.refdef.vieworg[2usize] += cg.predictedPlayerState.viewheight as libc::c_float;
    focusAngles[0usize] = cg.refdefViewAngles[0usize];
    focusAngles[1usize] = cg.refdefViewAngles[1usize];
    focusAngles[2usize] = cg.refdefViewAngles[2usize];
    if cg.predictedPlayerState.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        focusAngles[1usize] =
            cg.predictedPlayerState.stats[STAT_DEAD_YAW as libc::c_int as usize] as vec_t;
        cg.refdefViewAngles[1usize] =
            cg.predictedPlayerState.stats[STAT_DEAD_YAW as libc::c_int as usize] as vec_t
    }
    if focusAngles[0usize] > 45i32 as libc::c_float {
        focusAngles[0usize] = 45i32 as vec_t
    }
    AngleVectors(
        focusAngles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    focusPoint[0usize] = cg.refdef.vieworg[0usize] + forward[0usize] * 512i32 as libc::c_float;
    focusPoint[1usize] = cg.refdef.vieworg[1usize] + forward[1usize] * 512i32 as libc::c_float;
    focusPoint[2usize] = cg.refdef.vieworg[2usize] + forward[2usize] * 512i32 as libc::c_float;
    view[0usize] = cg.refdef.vieworg[0usize];
    view[1usize] = cg.refdef.vieworg[1usize];
    view[2usize] = cg.refdef.vieworg[2usize];
    view[2usize] += 8i32 as libc::c_float;
    cg.refdefViewAngles[0usize] = (cg.refdefViewAngles[0usize] as libc::c_double * 0.5f64) as vec_t;
    AngleVectors(
        cg.refdefViewAngles.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    forwardScale = cos(
        (cg_thirdPersonAngle.value / 180i32 as libc::c_float) as libc::c_double
            * 3.14159265358979323846f64,
    ) as libc::c_float;
    sideScale = sin(
        (cg_thirdPersonAngle.value / 180i32 as libc::c_float) as libc::c_double
            * 3.14159265358979323846f64,
    ) as libc::c_float;
    view[0usize] = view[0usize] + forward[0usize] * (-cg_thirdPersonRange.value * forwardScale);
    view[1usize] = view[1usize] + forward[1usize] * (-cg_thirdPersonRange.value * forwardScale);
    view[2usize] = view[2usize] + forward[2usize] * (-cg_thirdPersonRange.value * forwardScale);
    view[0usize] = view[0usize] + right[0usize] * (-cg_thirdPersonRange.value * sideScale);
    view[1usize] = view[1usize] + right[1usize] * (-cg_thirdPersonRange.value * sideScale);
    view[2usize] = view[2usize] + right[2usize] * (-cg_thirdPersonRange.value * sideScale);
    if 0 == cg_cameraMode.integer {
        CG_Trace(
            &mut trace,
            cg.refdef.vieworg.as_mut_ptr() as *const vec_t,
            mins.as_mut_ptr() as *const vec_t,
            maxs.as_mut_ptr() as *const vec_t,
            view.as_mut_ptr() as *const vec_t,
            cg.predictedPlayerState.clientNum,
            1i32,
        );
        if trace.fraction as libc::c_double != 1.0f64 {
            view[0usize] = trace.endpos[0usize];
            view[1usize] = trace.endpos[1usize];
            view[2usize] = trace.endpos[2usize];
            view[2usize] = (view[2usize] as libc::c_double
                + (1.0f64 - trace.fraction as libc::c_double) * 32i32 as libc::c_double)
                as vec_t;
            CG_Trace(
                &mut trace,
                cg.refdef.vieworg.as_mut_ptr() as *const vec_t,
                mins.as_mut_ptr() as *const vec_t,
                maxs.as_mut_ptr() as *const vec_t,
                view.as_mut_ptr() as *const vec_t,
                cg.predictedPlayerState.clientNum,
                1i32,
            );
            view[0usize] = trace.endpos[0usize];
            view[1usize] = trace.endpos[1usize];
            view[2usize] = trace.endpos[2usize]
        }
    }
    cg.refdef.vieworg[0usize] = view[0usize];
    cg.refdef.vieworg[1usize] = view[1usize];
    cg.refdef.vieworg[2usize] = view[2usize];
    focusPoint[0usize] = focusPoint[0usize] - cg.refdef.vieworg[0usize];
    focusPoint[1usize] = focusPoint[1usize] - cg.refdef.vieworg[1usize];
    focusPoint[2usize] = focusPoint[2usize] - cg.refdef.vieworg[2usize];
    focusDist = sqrt(
        (focusPoint[0usize] * focusPoint[0usize] + focusPoint[1usize] * focusPoint[1usize])
            as libc::c_double,
    ) as libc::c_float;
    if focusDist < 1i32 as libc::c_float {
        focusDist = 1i32 as libc::c_float
    }
    cg.refdefViewAngles[0usize] = (-180i32 as libc::c_double / 3.14159265358979323846f64
        * atan2(
            focusPoint[2usize] as libc::c_double,
            focusDist as libc::c_double,
        )) as vec_t;
    cg.refdefViewAngles[1usize] -= cg_thirdPersonAngle.value;
}
//============================================================================
/*
=================
CG_CalcVrect

Sets the coordinates of the rendered window
=================
*/
unsafe extern "C" fn CG_CalcVrect() {
    let mut size: libc::c_int = 0;
    if (*cg.snap).ps.pm_type == PM_INTERMISSION as libc::c_int {
        size = 100i32
    } else if cg_viewsize.integer < 30i32 {
        trap_Cvar_Set(
            b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
            b"30\x00" as *const u8 as *const libc::c_char,
        );
        size = 30i32
    } else if cg_viewsize.integer > 100i32 {
        trap_Cvar_Set(
            b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
            b"100\x00" as *const u8 as *const libc::c_char,
        );
        size = 100i32
    } else {
        size = cg_viewsize.integer
    }
    cg.refdef.width = cgs.glconfig.vidWidth * size / 100i32;
    cg.refdef.width &= !1i32;
    cg.refdef.height = cgs.glconfig.vidHeight * size / 100i32;
    cg.refdef.height &= !1i32;
    cg.refdef.x = (cgs.glconfig.vidWidth - cg.refdef.width) / 2i32;
    cg.refdef.y = (cgs.glconfig.vidHeight - cg.refdef.height) / 2i32;
}
