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
    unnamed_2, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, PERS_ASSIST_COUNT,
    PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT,
    PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY,
    PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, lerpFrame_t,
    playerEntity_t, score_t, trap_SendClientCommand, FOOTSTEP_BOOT, FOOTSTEP_ENERGY,
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
    byte, cvarHandle_t, entityState_s, entityState_t, gameState_t, playerState_s, playerState_t,
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, va, vec3_t, vec4_t,
    vec_t, vmCvar_t, Com_Printf, Com_sprintf, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::strlen;
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};
extern crate libc;

//
// cg_scoreboard.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_DrawOldScoreboard() -> qboolean {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut fade: libc::c_float = 0.;
    let mut fadeColor: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxClients: libc::c_int = 0;
    let mut lineHeight: libc::c_int = 0;
    let mut topBorderSize: libc::c_int = 0;
    let mut bottomBorderSize: libc::c_int = 0;
    if 0 != cg_paused.integer {
        cg.deferredPlayerLoading = 0i32;
        return qfalse;
    }
    if cgs.gametype as libc::c_uint == GT_SINGLE_PLAYER as libc::c_int as libc::c_uint
        && cg.predictedPlayerState.pm_type == PM_INTERMISSION as libc::c_int
    {
        cg.deferredPlayerLoading = 0i32;
        return qfalse;
    }
    if 0 != cg.warmup && 0 == cg.showScores as u64 {
        return qfalse;
    }
    if 0 != cg.showScores as libc::c_uint
        || cg.predictedPlayerState.pm_type == PM_DEAD as libc::c_int
        || cg.predictedPlayerState.pm_type == PM_INTERMISSION as libc::c_int
    {
        fade = 1.0f64 as libc::c_float;
        fadeColor = colorWhite.as_mut_ptr()
    } else {
        fadeColor = CG_FadeColor(cg.scoreFadeTime, 200i32);
        if fadeColor.is_null() {
            cg.deferredPlayerLoading = 0i32;
            cg.killerName[0usize] = 0i32 as libc::c_char;
            return qfalse;
        }
        fade = *fadeColor
    }
    if 0 != cg.killerName[0usize] {
        s = va(
            b"Fragged by %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cg.killerName.as_mut_ptr(),
        );
        w = CG_DrawStrlen(s) * 16i32;
        x = (640i32 - w) / 2i32;
        y = 40i32;
        CG_DrawBigString(x, y, s, fade);
    }
    if (cgs.gametype as libc::c_uint) < GT_TEAM as libc::c_int as libc::c_uint {
        if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
            != TEAM_SPECTATOR as libc::c_int
        {
            s = va(
                b"%s place with %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                CG_PlaceString((*cg.snap).ps.persistant[PERS_RANK as libc::c_int as usize] + 1i32),
                (*cg.snap).ps.persistant[PERS_SCORE as libc::c_int as usize],
            );
            w = CG_DrawStrlen(s) * 16i32;
            x = (640i32 - w) / 2i32;
            y = 60i32;
            CG_DrawBigString(x, y, s, fade);
        }
    } else {
        if cg.teamScores[0usize] == cg.teamScores[1usize] {
            s = va(
                b"Teams are tied at %i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cg.teamScores[0usize],
            )
        } else if cg.teamScores[0usize] >= cg.teamScores[1usize] {
            s = va(
                b"Red leads %i to %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cg.teamScores[0usize],
                cg.teamScores[1usize],
            )
        } else {
            s = va(
                b"Blue leads %i to %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cg.teamScores[1usize],
                cg.teamScores[0usize],
            )
        }
        w = CG_DrawStrlen(s) * 16i32;
        x = (640i32 - w) / 2i32;
        y = 60i32;
        CG_DrawBigString(x, y, s, fade);
    }
    y = 86i32;
    CG_DrawPic(
        (112i32 + 16i32 + 6i32 * 16i32 / 2i32) as libc::c_float,
        y as libc::c_float,
        64i32 as libc::c_float,
        32i32 as libc::c_float,
        cgs.media.scoreboardScore,
    );
    CG_DrawPic(
        (112i32 + 12i32 * 16i32 + 8i32 - 6i32 * 16i32 / 2i32) as libc::c_float,
        y as libc::c_float,
        64i32 as libc::c_float,
        32i32 as libc::c_float,
        cgs.media.scoreboardPing,
    );
    CG_DrawPic(
        (112i32 + 17i32 * 16i32 + 8i32 - 6i32 * 16i32 / 2i32) as libc::c_float,
        y as libc::c_float,
        64i32 as libc::c_float,
        32i32 as libc::c_float,
        cgs.media.scoreboardTime,
    );
    CG_DrawPic(
        (112i32 + 22i32 * 16i32 - 6i32 * 16i32 / 2i32) as libc::c_float,
        y as libc::c_float,
        64i32 as libc::c_float,
        32i32 as libc::c_float,
        cgs.media.scoreboardName,
    );
    y = 86i32 + 32i32;
    if cg.numScores > (420i32 - (86i32 + 32i32)) / 40i32 {
        maxClients = (420i32 - (86i32 + 32i32)) / 16i32 - 1i32;
        lineHeight = 16i32;
        topBorderSize = 8i32;
        bottomBorderSize = 16i32
    } else {
        maxClients = (420i32 - (86i32 + 32i32)) / 40i32;
        lineHeight = 40i32;
        topBorderSize = 16i32;
        bottomBorderSize = 16i32
    }
    localClient = qfalse;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        y += lineHeight / 2i32;
        if cg.teamScores[0usize] >= cg.teamScores[1usize] {
            n1 = CG_TeamScoreboard(y, TEAM_RED, fade, maxClients, lineHeight);
            CG_DrawTeamBackground(
                0i32,
                y - topBorderSize,
                640i32,
                n1 * lineHeight + bottomBorderSize,
                0.33f32,
                TEAM_RED as libc::c_int,
            );
            y += n1 * lineHeight + 16i32;
            maxClients -= n1;
            n2 = CG_TeamScoreboard(y, TEAM_BLUE, fade, maxClients, lineHeight);
            CG_DrawTeamBackground(
                0i32,
                y - topBorderSize,
                640i32,
                n2 * lineHeight + bottomBorderSize,
                0.33f32,
                TEAM_BLUE as libc::c_int,
            );
            y += n2 * lineHeight + 16i32;
            maxClients -= n2
        } else {
            n1 = CG_TeamScoreboard(y, TEAM_BLUE, fade, maxClients, lineHeight);
            CG_DrawTeamBackground(
                0i32,
                y - topBorderSize,
                640i32,
                n1 * lineHeight + bottomBorderSize,
                0.33f32,
                TEAM_BLUE as libc::c_int,
            );
            y += n1 * lineHeight + 16i32;
            maxClients -= n1;
            n2 = CG_TeamScoreboard(y, TEAM_RED, fade, maxClients, lineHeight);
            CG_DrawTeamBackground(
                0i32,
                y - topBorderSize,
                640i32,
                n2 * lineHeight + bottomBorderSize,
                0.33f32,
                TEAM_RED as libc::c_int,
            );
            y += n2 * lineHeight + 16i32;
            maxClients -= n2
        }
        n1 = CG_TeamScoreboard(y, TEAM_SPECTATOR, fade, maxClients, lineHeight);
        y += n1 * lineHeight + 16i32
    } else {
        n1 = CG_TeamScoreboard(y, TEAM_FREE, fade, maxClients, lineHeight);
        y += n1 * lineHeight + 16i32;
        n2 = CG_TeamScoreboard(y, TEAM_SPECTATOR, fade, maxClients - n1, lineHeight);
        y += n2 * lineHeight + 16i32
    }
    if 0 == localClient as u64 {
        i = 0i32;
        while i < cg.numScores {
            if cg.scores[i as usize].client == (*cg.snap).ps.clientNum {
                CG_DrawClientScore(
                    y,
                    &mut *cg.scores.as_mut_ptr().offset(i as isize),
                    fadeColor,
                    fade,
                    (lineHeight == 40i32) as libc::c_int as qboolean,
                );
                break;
            } else {
                i += 1
            }
        }
    }
    cg.deferredPlayerLoading += 1;
    if cg.deferredPlayerLoading > 10i32 {
        CG_LoadDeferredPlayers();
    }
    return qtrue;
}
/*
=================
CG_DrawScoreboard
=================
*/
unsafe extern "C" fn CG_DrawClientScore(
    mut y: libc::c_int,
    mut score: *mut score_t,
    mut color: *mut libc::c_float,
    mut fade: libc::c_float,
    mut largeFormat: qboolean,
) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut headAngles: vec3_t = [0.; 3];
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut iconx: libc::c_int = 0;
    let mut headx: libc::c_int = 0;
    if (*score).client < 0i32 || (*score).client >= cgs.maxclients {
        Com_Printf(
            b"Bad score->client: %i\n\x00" as *const u8 as *const libc::c_char,
            (*score).client,
        );
        return;
    }
    ci = &mut *cgs.clientinfo.as_mut_ptr().offset((*score).client as isize) as *mut clientInfo_t;
    iconx = 0i32 + 32i32 + 6i32 * 16i32 / 2i32;
    headx = 0i32 + 64i32 + 6i32 * 16i32 / 2i32;
    if 0 != (*ci).powerups & 1i32 << PW_NEUTRALFLAG as libc::c_int {
        if 0 != largeFormat as u64 {
            CG_DrawFlagModel(
                iconx as libc::c_float,
                (y - (32i32 - 16i32) / 2i32) as libc::c_float,
                32i32 as libc::c_float,
                32i32 as libc::c_float,
                TEAM_FREE as libc::c_int,
                qfalse,
            );
        } else {
            CG_DrawFlagModel(
                iconx as libc::c_float,
                y as libc::c_float,
                16i32 as libc::c_float,
                16i32 as libc::c_float,
                TEAM_FREE as libc::c_int,
                qfalse,
            );
        }
    } else if 0 != (*ci).powerups & 1i32 << PW_REDFLAG as libc::c_int {
        if 0 != largeFormat as u64 {
            CG_DrawFlagModel(
                iconx as libc::c_float,
                (y - (32i32 - 16i32) / 2i32) as libc::c_float,
                32i32 as libc::c_float,
                32i32 as libc::c_float,
                TEAM_RED as libc::c_int,
                qfalse,
            );
        } else {
            CG_DrawFlagModel(
                iconx as libc::c_float,
                y as libc::c_float,
                16i32 as libc::c_float,
                16i32 as libc::c_float,
                TEAM_RED as libc::c_int,
                qfalse,
            );
        }
    } else if 0 != (*ci).powerups & 1i32 << PW_BLUEFLAG as libc::c_int {
        if 0 != largeFormat as u64 {
            CG_DrawFlagModel(
                iconx as libc::c_float,
                (y - (32i32 - 16i32) / 2i32) as libc::c_float,
                32i32 as libc::c_float,
                32i32 as libc::c_float,
                TEAM_BLUE as libc::c_int,
                qfalse,
            );
        } else {
            CG_DrawFlagModel(
                iconx as libc::c_float,
                y as libc::c_float,
                16i32 as libc::c_float,
                16i32 as libc::c_float,
                TEAM_BLUE as libc::c_int,
                qfalse,
            );
        }
    } else {
        if (*ci).botSkill > 0i32 && (*ci).botSkill <= 5i32 {
            if 0 != cg_drawIcons.integer {
                if 0 != largeFormat as u64 {
                    CG_DrawPic(
                        iconx as libc::c_float,
                        (y - (32i32 - 16i32) / 2i32) as libc::c_float,
                        32i32 as libc::c_float,
                        32i32 as libc::c_float,
                        cgs.media.botSkillShaders[((*ci).botSkill - 1i32) as usize],
                    );
                } else {
                    CG_DrawPic(
                        iconx as libc::c_float,
                        y as libc::c_float,
                        16i32 as libc::c_float,
                        16i32 as libc::c_float,
                        cgs.media.botSkillShaders[((*ci).botSkill - 1i32) as usize],
                    );
                }
            }
        } else if (*ci).handicap < 100i32 {
            Com_sprintf(
                string.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"%i\x00" as *const u8 as *const libc::c_char,
                (*ci).handicap,
            );
            if cgs.gametype as libc::c_uint == GT_TOURNAMENT as libc::c_int as libc::c_uint {
                CG_DrawSmallStringColor(iconx, y - 16i32 / 2i32, string.as_mut_ptr(), color);
            } else {
                CG_DrawSmallStringColor(iconx, y, string.as_mut_ptr(), color);
            }
        }
        if cgs.gametype as libc::c_uint == GT_TOURNAMENT as libc::c_int as libc::c_uint {
            Com_sprintf(
                string.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                b"%i/%i\x00" as *const u8 as *const libc::c_char,
                (*ci).wins,
                (*ci).losses,
            );
            if (*ci).handicap < 100i32 && 0 == (*ci).botSkill {
                CG_DrawSmallStringColor(iconx, y + 16i32 / 2i32, string.as_mut_ptr(), color);
            } else {
                CG_DrawSmallStringColor(iconx, y, string.as_mut_ptr(), color);
            }
        }
    }
    headAngles[2usize] = 0i32 as vec_t;
    headAngles[1usize] = headAngles[2usize];
    headAngles[0usize] = headAngles[1usize];
    headAngles[1usize] = 180i32 as vec_t;
    if 0 != largeFormat as u64 {
        CG_DrawHead(
            headx as libc::c_float,
            (y - (48i32 - 16i32) / 2i32) as libc::c_float,
            48i32 as libc::c_float,
            48i32 as libc::c_float,
            (*score).client,
            headAngles.as_mut_ptr(),
        );
    } else {
        CG_DrawHead(
            headx as libc::c_float,
            y as libc::c_float,
            16i32 as libc::c_float,
            16i32 as libc::c_float,
            (*score).client,
            headAngles.as_mut_ptr(),
        );
    }
    if (*score).ping == -1i32 {
        Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b" connecting    %s\x00" as *const u8 as *const libc::c_char,
            (*ci).name.as_mut_ptr(),
        );
    } else if (*ci).team as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint {
        Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b" SPECT %3i %4i %s\x00" as *const u8 as *const libc::c_char,
            (*score).ping,
            (*score).time,
            (*ci).name.as_mut_ptr(),
        );
    } else {
        Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            b"%5i %4i %4i %s\x00" as *const u8 as *const libc::c_char,
            (*score).score,
            (*score).ping,
            (*score).time,
            (*ci).name.as_mut_ptr(),
        );
    }
    if (*score).client == (*cg.snap).ps.clientNum {
        let mut hcolor: [libc::c_float; 4] = [0.; 4];
        let mut rank: libc::c_int = 0;
        localClient = qtrue;
        if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
            == TEAM_SPECTATOR as libc::c_int
            || cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
        {
            rank = -1i32
        } else {
            rank = (*cg.snap).ps.persistant[PERS_RANK as libc::c_int as usize] & !0x4000i32
        }
        if rank == 0i32 {
            hcolor[0usize] = 0i32 as libc::c_float;
            hcolor[1usize] = 0i32 as libc::c_float;
            hcolor[2usize] = 0.7f32
        } else if rank == 1i32 {
            hcolor[0usize] = 0.7f32;
            hcolor[1usize] = 0i32 as libc::c_float;
            hcolor[2usize] = 0i32 as libc::c_float
        } else if rank == 2i32 {
            hcolor[0usize] = 0.7f32;
            hcolor[1usize] = 0.7f32;
            hcolor[2usize] = 0i32 as libc::c_float
        } else {
            hcolor[0usize] = 0.7f32;
            hcolor[1usize] = 0.7f32;
            hcolor[2usize] = 0.7f32
        }
        hcolor[3usize] = (fade as libc::c_double * 0.7f64) as libc::c_float;
        CG_FillRect(
            (112i32 + 16i32 + 6i32 * 16i32 / 2i32) as libc::c_float,
            y as libc::c_float,
            (640i32 - 112i32 - 16i32) as libc::c_float,
            (16i32 + 1i32) as libc::c_float,
            hcolor.as_mut_ptr(),
        );
    }
    CG_DrawBigString(112i32 + 6i32 * 16i32 / 2i32, y, string.as_mut_ptr(), fade);
    if 0 != (*cg.snap).ps.stats[STAT_CLIENTS_READY as libc::c_int as usize]
        & 1i32 << (*score).client
    {
        CG_DrawBigStringColor(
            iconx,
            y,
            b"READY\x00" as *const u8 as *const libc::c_char,
            color,
        );
    };
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
// cg_scoreboard -- draw the scoreboard on top of the game screen
// Where the status bar starts, so we don't overwrite it
// interleaved height
// Used when interleaved
// Normal
// width 6
// width 6
// width 6
// width 5
// width 5
// width 15
// The new and improved score board
//
// In cases where the number of clients is high, the score board heads are interleaved
// here's the layout
//
//	0   32   80  112  144   240  320  400   <-- pixel position
//  bot head bot head score ping time name
//
//  wins/losses are drawn on bot icon now
// true if local client has been displayed
static mut localClient: qboolean = qfalse;
/*
=================
CG_TeamScoreboard
=================
*/
unsafe extern "C" fn CG_TeamScoreboard(
    mut y: libc::c_int,
    mut team: team_t,
    mut fade: libc::c_float,
    mut maxClients: libc::c_int,
    mut lineHeight: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut score: *mut score_t = 0 as *mut score_t;
    let mut color: [libc::c_float; 4] = [0.; 4];
    let mut count: libc::c_int = 0;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    color[2usize] = 1.0f64 as libc::c_float;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    color[3usize] = fade;
    count = 0i32;
    i = 0i32;
    while i < cg.numScores && count < maxClients {
        score = &mut *cg.scores.as_mut_ptr().offset(i as isize) as *mut score_t;
        ci =
            &mut *cgs.clientinfo.as_mut_ptr().offset((*score).client as isize) as *mut clientInfo_t;
        if !(team as libc::c_uint != (*ci).team as libc::c_uint) {
            CG_DrawClientScore(
                y + lineHeight * count,
                score,
                color.as_mut_ptr(),
                fade,
                (lineHeight == 40i32) as libc::c_int as qboolean,
            );
            count += 1
        }
        i += 1
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawTourneyScoreboard() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut color: vec4_t = [0.; 4];
    let mut min: libc::c_int = 0;
    let mut tens: libc::c_int = 0;
    let mut ones: libc::c_int = 0;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if cg.scoresRequestTime + 2000i32 < cg.time {
        cg.scoresRequestTime = cg.time;
        trap_SendClientCommand(b"score\x00" as *const u8 as *const libc::c_char);
    }
    color[2usize] = 0i32 as vec_t;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    color[3usize] = 1i32 as vec_t;
    CG_FillRect(
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        640i32 as libc::c_float,
        480i32 as libc::c_float,
        color.as_mut_ptr(),
    );
    color[0usize] = 1i32 as vec_t;
    color[1usize] = 1i32 as vec_t;
    color[2usize] = 1i32 as vec_t;
    color[3usize] = 1i32 as vec_t;
    s = CG_ConfigString(4i32);
    if 0 == *s.offset(0isize) {
        s = b"Scoreboard\x00" as *const u8 as *const libc::c_char
    }
    CG_CenterGiantLine(8i32 as libc::c_float, s);
    ones = cg.time / 1000i32;
    min = ones / 60i32;
    ones %= 60i32;
    tens = ones / 10i32;
    ones %= 10i32;
    s = va(
        b"%i:%i%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        min,
        tens,
        ones,
    );
    CG_CenterGiantLine(64i32 as libc::c_float, s);
    y = 160i32;
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint {
        CG_DrawStringExt(
            8i32,
            y,
            b"Red Team\x00" as *const u8 as *const libc::c_char,
            color.as_mut_ptr(),
            qtrue,
            qtrue,
            32i32,
            48i32,
            0i32,
        );
        s = va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cg.teamScores[0usize],
        );
        CG_DrawStringExt(
            (632i32 as libc::c_ulong).wrapping_sub((32i32 as libc::c_ulong).wrapping_mul(strlen(s)))
                as libc::c_int,
            y,
            s,
            color.as_mut_ptr(),
            qtrue,
            qtrue,
            32i32,
            48i32,
            0i32,
        );
        y += 64i32;
        CG_DrawStringExt(
            8i32,
            y,
            b"Blue Team\x00" as *const u8 as *const libc::c_char,
            color.as_mut_ptr(),
            qtrue,
            qtrue,
            32i32,
            48i32,
            0i32,
        );
        s = va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cg.teamScores[1usize],
        );
        CG_DrawStringExt(
            (632i32 as libc::c_ulong).wrapping_sub((32i32 as libc::c_ulong).wrapping_mul(strlen(s)))
                as libc::c_int,
            y,
            s,
            color.as_mut_ptr(),
            qtrue,
            qtrue,
            32i32,
            48i32,
            0i32,
        );
    } else {
        i = 0i32;
        while i < 64i32 {
            ci = &mut *cgs.clientinfo.as_mut_ptr().offset(i as isize) as *mut clientInfo_t;
            if !(0 == (*ci).infoValid as u64) {
                if !((*ci).team as libc::c_uint != TEAM_FREE as libc::c_int as libc::c_uint) {
                    CG_DrawStringExt(
                        8i32,
                        y,
                        (*ci).name.as_mut_ptr(),
                        color.as_mut_ptr(),
                        qtrue,
                        qtrue,
                        32i32,
                        48i32,
                        0i32,
                    );
                    s = va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        (*ci).score,
                    );
                    CG_DrawStringExt(
                        (632i32 as libc::c_ulong)
                            .wrapping_sub((32i32 as libc::c_ulong).wrapping_mul(strlen(s)))
                            as libc::c_int,
                        y,
                        s,
                        color.as_mut_ptr(),
                        qtrue,
                        qtrue,
                        32i32,
                        48i32,
                        0i32,
                    );
                    y += 64i32
                }
            }
            i += 1
        }
    };
}
//================================================================================
/*
================
CG_CenterGiantLine
================
*/
unsafe extern "C" fn CG_CenterGiantLine(mut y: libc::c_float, mut string: *const libc::c_char) {
    let mut x: libc::c_float = 0.;
    let mut color: vec4_t = [0.; 4];
    color[0usize] = 1i32 as vec_t;
    color[1usize] = 1i32 as vec_t;
    color[2usize] = 1i32 as vec_t;
    color[3usize] = 1i32 as vec_t;
    x = (0.5f64 * (640i32 - 32i32 * CG_DrawStrlen(string)) as libc::c_double) as libc::c_float;
    CG_DrawStringExt(
        x as libc::c_int,
        y as libc::c_int,
        string,
        color.as_mut_ptr(),
        qtrue,
        qtrue,
        32i32,
        48i32,
        0i32,
    );
}
