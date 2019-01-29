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
    playerEntity_t, score_t, trap_Argc, trap_Cvar_Set, trap_GetGameState, trap_GetServerCommand,
    trap_R_RegisterModel, trap_R_RemapShader, trap_S_ClearLoopingSounds, trap_S_RegisterSound,
    trap_S_StartLocalSound, FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH,
    FOOTSTEP_METAL, FOOTSTEP_NORMAL, FOOTSTEP_SPLASH, FOOTSTEP_TOTAL,
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
use q_shared_h::{
    byte, cvarHandle_t, entityState_s, entityState_t, gameState_t, playerState_s, playerState_t,
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, unnamed, va, vec3_t,
    vec_t, vmCvar_t, Com_sprintf, Info_ValueForKey, Q_IsColorString, Q_stricmp, Q_strncpyz,
    CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE,
    CHAN_WEAPON, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, memset, strcmp, strncpy, strstr};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

//
// cg_servercmds.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_ExecuteNewServerCommands(mut latestSequence: libc::c_int) {
    while cgs.serverCommandSequence < latestSequence {
        cgs.serverCommandSequence += 1;
        if 0 != trap_GetServerCommand(cgs.serverCommandSequence) as u64 {
            CG_ServerCommand();
        }
    }
}
/*
=================
CG_ServerCommand

The string has been tokenized and can be retrieved with
Cmd_Argc() / Cmd_Argv()
=================
*/
unsafe extern "C" fn CG_ServerCommand() {
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut text: [libc::c_char; 150] = [0; 150];
    cmd = CG_Argv(0i32);
    if 0 == *cmd.offset(0isize) {
        return;
    }
    if 0 == strcmp(cmd, b"cp\x00" as *const u8 as *const libc::c_char) {
        CG_CenterPrint(
            CG_Argv(1i32),
            (480i32 as libc::c_double * 0.30f64) as libc::c_int,
            16i32,
        );
        return;
    }
    if 0 == strcmp(cmd, b"cs\x00" as *const u8 as *const libc::c_char) {
        CG_ConfigStringModified();
        return;
    }
    if 0 == strcmp(cmd, b"print\x00" as *const u8 as *const libc::c_char) {
        CG_Printf(b"%s\x00" as *const u8 as *const libc::c_char, CG_Argv(1i32));
        return;
    }
    if 0 == strcmp(cmd, b"chat\x00" as *const u8 as *const libc::c_char) {
        if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
            && 0 != cg_teamChatsOnly.integer
        {
            return;
        }
        trap_S_StartLocalSound(cgs.media.talkSound, CHAN_LOCAL_SOUND as libc::c_int);
        Q_strncpyz(text.as_mut_ptr(), CG_Argv(1i32), 150i32);
        CG_RemoveChatEscapeChar(text.as_mut_ptr());
        CG_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            text.as_mut_ptr(),
        );
        return;
    }
    if 0 == strcmp(cmd, b"tchat\x00" as *const u8 as *const libc::c_char) {
        trap_S_StartLocalSound(cgs.media.talkSound, CHAN_LOCAL_SOUND as libc::c_int);
        Q_strncpyz(text.as_mut_ptr(), CG_Argv(1i32), 150i32);
        CG_RemoveChatEscapeChar(text.as_mut_ptr());
        CG_AddToTeamChat(text.as_mut_ptr());
        CG_Printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            text.as_mut_ptr(),
        );
        return;
    }
    if 0 == strcmp(cmd, b"scores\x00" as *const u8 as *const libc::c_char) {
        CG_ParseScores();
        return;
    }
    if 0 == strcmp(cmd, b"tinfo\x00" as *const u8 as *const libc::c_char) {
        CG_ParseTeamInfo();
        return;
    }
    if 0 == strcmp(cmd, b"map_restart\x00" as *const u8 as *const libc::c_char) {
        CG_MapRestart();
        return;
    }
    if Q_stricmp(cmd, b"remapShader\x00" as *const u8 as *const libc::c_char) == 0i32 {
        if trap_Argc() == 4i32 {
            let mut shader1: [libc::c_char; 64] = [0; 64];
            let mut shader2: [libc::c_char; 64] = [0; 64];
            let mut shader3: [libc::c_char; 64] = [0; 64];
            Q_strncpyz(
                shader1.as_mut_ptr(),
                CG_Argv(1i32),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            Q_strncpyz(
                shader2.as_mut_ptr(),
                CG_Argv(2i32),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            Q_strncpyz(
                shader3.as_mut_ptr(),
                CG_Argv(3i32),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            );
            trap_R_RemapShader(
                shader1.as_mut_ptr(),
                shader2.as_mut_ptr(),
                shader3.as_mut_ptr(),
            );
        }
        return;
    }
    if 0 == strcmp(cmd, b"loaddefered\x00" as *const u8 as *const libc::c_char) {
        CG_LoadDeferredPlayers();
        return;
    }
    if 0 == strcmp(
        cmd,
        b"clientLevelShot\x00" as *const u8 as *const libc::c_char,
    ) {
        cg.levelShot = qtrue;
        return;
    }
    CG_Printf(
        b"Unknown client game command: %s\n\x00" as *const u8 as *const libc::c_char,
        cmd,
    );
}
/*
===============
CG_MapRestart

The server has issued a map_restart, so the next snapshot
is completely new and should not be interpolated to.

A tournement restart will clear everything, but doesn't
require a reload of all the media
===============
*/
unsafe extern "C" fn CG_MapRestart() {
    if 0 != cg_showmiss.integer {
        CG_Printf(b"CG_MapRestart\n\x00" as *const u8 as *const libc::c_char);
    }
    CG_InitLocalEntities();
    CG_InitMarkPolys();
    CG_ClearParticles();
    cg.fraglimitWarnings = 0i32;
    cg.timelimitWarnings = 0i32;
    cg.rewardTime = 0i32;
    cg.rewardStack = 0i32;
    cg.intermissionStarted = qfalse;
    cg.levelShot = qfalse;
    cgs.voteTime = 0i32;
    cg.mapRestart = qtrue;
    CG_StartMusic();
    trap_S_ClearLoopingSounds(qtrue);
    if cg.warmup == 0i32 {
        trap_S_StartLocalSound(cgs.media.countFightSound, CHAN_ANNOUNCER as libc::c_int);
        CG_CenterPrint(
            b"FIGHT!\x00" as *const u8 as *const libc::c_char,
            120i32,
            32i32 * 2i32,
        );
    }
    trap_Cvar_Set(
        b"cg_thirdPerson\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
CG_ParseTeamInfo

=================
*/
unsafe extern "C" fn CG_ParseTeamInfo() {
    let mut i: libc::c_int = 0;
    let mut client: libc::c_int = 0;
    numSortedTeamPlayers = atoi(CG_Argv(1i32));
    if numSortedTeamPlayers < 0i32 || numSortedTeamPlayers > 32i32 {
        CG_Error(
            b"CG_ParseTeamInfo: numSortedTeamPlayers out of range (%d)\x00" as *const u8
                as *const libc::c_char,
            numSortedTeamPlayers,
        );
    }
    i = 0i32;
    while i < numSortedTeamPlayers {
        client = atoi(CG_Argv(i * 6i32 + 2i32));
        if client < 0i32 || client >= 64i32 {
            CG_Error(
                b"CG_ParseTeamInfo: bad client number: %d\x00" as *const u8 as *const libc::c_char,
                client,
            );
        }
        sortedTeamPlayers[i as usize] = client;
        cgs.clientinfo[client as usize].location = atoi(CG_Argv(i * 6i32 + 3i32));
        cgs.clientinfo[client as usize].health = atoi(CG_Argv(i * 6i32 + 4i32));
        cgs.clientinfo[client as usize].armor = atoi(CG_Argv(i * 6i32 + 5i32));
        cgs.clientinfo[client as usize].curWeapon = atoi(CG_Argv(i * 6i32 + 6i32));
        cgs.clientinfo[client as usize].powerups = atoi(CG_Argv(i * 6i32 + 7i32));
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
// cg_servercmds.c -- reliably sequenced text commands sent by the server
// these are processed at snapshot transition time, so there will definitely
// be a valid snapshot this frame
/*
=================
CG_ParseScores

=================
*/
unsafe extern "C" fn CG_ParseScores() {
    let mut i: libc::c_int = 0;
    let mut powerups: libc::c_int = 0;
    cg.numScores = atoi(CG_Argv(1i32));
    if cg.numScores > 64i32 {
        cg.numScores = 64i32
    }
    cg.teamScores[0usize] = atoi(CG_Argv(2i32));
    cg.teamScores[1usize] = atoi(CG_Argv(3i32));
    memset(
        cg.scores.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[score_t; 64]>() as libc::c_ulong,
    );
    i = 0i32;
    while i < cg.numScores {
        cg.scores[i as usize].client = atoi(CG_Argv(i * 14i32 + 4i32));
        cg.scores[i as usize].score = atoi(CG_Argv(i * 14i32 + 5i32));
        cg.scores[i as usize].ping = atoi(CG_Argv(i * 14i32 + 6i32));
        cg.scores[i as usize].time = atoi(CG_Argv(i * 14i32 + 7i32));
        cg.scores[i as usize].scoreFlags = atoi(CG_Argv(i * 14i32 + 8i32));
        powerups = atoi(CG_Argv(i * 14i32 + 9i32));
        cg.scores[i as usize].accuracy = atoi(CG_Argv(i * 14i32 + 10i32));
        cg.scores[i as usize].impressiveCount = atoi(CG_Argv(i * 14i32 + 11i32));
        cg.scores[i as usize].excellentCount = atoi(CG_Argv(i * 14i32 + 12i32));
        cg.scores[i as usize].guantletCount = atoi(CG_Argv(i * 14i32 + 13i32));
        cg.scores[i as usize].defendCount = atoi(CG_Argv(i * 14i32 + 14i32));
        cg.scores[i as usize].assistCount = atoi(CG_Argv(i * 14i32 + 15i32));
        cg.scores[i as usize].perfect = atoi(CG_Argv(i * 14i32 + 16i32)) as qboolean;
        cg.scores[i as usize].captures = atoi(CG_Argv(i * 14i32 + 17i32));
        if cg.scores[i as usize].client < 0i32 || cg.scores[i as usize].client >= 64i32 {
            cg.scores[i as usize].client = 0i32
        }
        cgs.clientinfo[cg.scores[i as usize].client as usize].score = cg.scores[i as usize].score;
        cgs.clientinfo[cg.scores[i as usize].client as usize].powerups = powerups;
        cg.scores[i as usize].team =
            cgs.clientinfo[cg.scores[i as usize].client as usize].team as libc::c_int;
        i += 1
    }
}
/*
=======================
CG_AddToTeamChat

=======================
*/
unsafe extern "C" fn CG_AddToTeamChat(mut str: *const libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ls: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastcolor: libc::c_int = 0;
    let mut chatHeight: libc::c_int = 0;
    if cg_teamChatHeight.integer < 8i32 {
        chatHeight = cg_teamChatHeight.integer
    } else {
        chatHeight = 8i32
    }
    if chatHeight <= 0i32 || cg_teamChatTime.integer <= 0i32 {
        cgs.teamLastChatPos = 0i32;
        cgs.teamChatPos = cgs.teamLastChatPos;
        return;
    }
    len = 0i32;
    p = cgs.teamChatMsgs[(cgs.teamChatPos % chatHeight) as usize].as_mut_ptr();
    *p = 0i32 as libc::c_char;
    lastcolor = '7' as i32;
    ls = 0 as *mut libc::c_char;
    while 0 != *str {
        if len > 80i32 - 1i32 {
            if !ls.is_null() {
                str = str.offset(-(p.wrapping_offset_from(ls) as libc::c_long as isize));
                str = str.offset(1isize);
                p = p.offset(-(p.wrapping_offset_from(ls) as libc::c_long as isize))
            }
            *p = 0i32 as libc::c_char;
            cgs.teamChatMsgTimes[(cgs.teamChatPos % chatHeight) as usize] = cg.time;
            cgs.teamChatPos += 1;
            p = cgs.teamChatMsgs[(cgs.teamChatPos % chatHeight) as usize].as_mut_ptr();
            *p = 0i32 as libc::c_char;
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = '^' as i32 as libc::c_char;
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = lastcolor as libc::c_char;
            len = 0i32;
            ls = 0 as *mut libc::c_char
        }
        if 0 != Q_IsColorString(str) as u64 {
            let fresh3 = p;
            p = p.offset(1);
            let fresh2 = str;
            str = str.offset(1);
            *fresh3 = *fresh2;
            lastcolor = *str as libc::c_int;
            let fresh5 = p;
            p = p.offset(1);
            let fresh4 = str;
            str = str.offset(1);
            *fresh5 = *fresh4
        } else {
            if *str as libc::c_int == ' ' as i32 {
                ls = p
            }
            let fresh7 = p;
            p = p.offset(1);
            let fresh6 = str;
            str = str.offset(1);
            *fresh7 = *fresh6;
            len += 1
        }
    }
    *p = 0i32 as libc::c_char;
    cgs.teamChatMsgTimes[(cgs.teamChatPos % chatHeight) as usize] = cg.time;
    cgs.teamChatPos += 1;
    if cgs.teamChatPos - cgs.teamLastChatPos > chatHeight {
        cgs.teamLastChatPos = cgs.teamChatPos - chatHeight
    };
}
// MISSIONPACK
/*
=================
CG_RemoveChatEscapeChar
=================
*/
unsafe extern "C" fn CG_RemoveChatEscapeChar(mut text: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = 0i32;
    i = 0i32;
    while 0 != *text.offset(i as isize) {
        if !(*text.offset(i as isize) as libc::c_int == '\u{19}' as i32) {
            let fresh8 = l;
            l = l + 1;
            *text.offset(fresh8 as isize) = *text.offset(i as isize)
        }
        i += 1
    }
    *text.offset(l as isize) = '\u{0}' as i32 as libc::c_char;
}
/*
================
CG_ConfigStringModified

================
*/
unsafe extern "C" fn CG_ConfigStringModified() {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut num: libc::c_int = 0;
    num = atoi(CG_Argv(1i32));
    trap_GetGameState(&mut cgs.gameState);
    str = CG_ConfigString(num);
    if num == 2i32 {
        CG_StartMusic();
    } else if num == 0i32 {
        CG_ParseServerinfo();
    } else if num == 5i32 {
        CG_ParseWarmup();
    } else if num == 6i32 {
        cgs.scores1 = atoi(str)
    } else if num == 7i32 {
        cgs.scores2 = atoi(str)
    } else if num == 21i32 {
        cgs.levelStartTime = atoi(str)
    } else if num == 8i32 {
        cgs.voteTime = atoi(str);
        cgs.voteModified = qtrue
    } else if num == 10i32 {
        cgs.voteYes = atoi(str);
        cgs.voteModified = qtrue
    } else if num == 11i32 {
        cgs.voteNo = atoi(str);
        cgs.voteModified = qtrue
    } else if num == 9i32 {
        Q_strncpyz(
            cgs.voteString.as_mut_ptr(),
            str,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
    } else if num >= 12i32 && num <= 12i32 + 1i32 {
        cgs.teamVoteTime[(num - 12i32) as usize] = atoi(str);
        cgs.teamVoteModified[(num - 12i32) as usize] = qtrue
    } else if num >= 16i32 && num <= 16i32 + 1i32 {
        cgs.teamVoteYes[(num - 16i32) as usize] = atoi(str);
        cgs.teamVoteModified[(num - 16i32) as usize] = qtrue
    } else if num >= 18i32 && num <= 18i32 + 1i32 {
        cgs.teamVoteNo[(num - 18i32) as usize] = atoi(str);
        cgs.teamVoteModified[(num - 18i32) as usize] = qtrue
    } else if num >= 14i32 && num <= 14i32 + 1i32 {
        Q_strncpyz(
            cgs.teamVoteString[(num - 14i32) as usize].as_mut_ptr(),
            str,
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
    } else if num == 22i32 {
        cg.intermissionStarted = atoi(str) as qboolean
    } else if num >= 32i32 && num < 32i32 + 256i32 {
        cgs.gameModels[(num - 32i32) as usize] = trap_R_RegisterModel(str)
    } else if num >= 32i32 + 256i32 && num < 32i32 + 256i32 + 256i32 {
        if *str.offset(0isize) as libc::c_int != '*' as i32 {
            cgs.gameSounds[(num - (32i32 + 256i32)) as usize] = trap_S_RegisterSound(str, qfalse)
        }
    } else if num >= 32i32 + 256i32 + 256i32 && num < 32i32 + 256i32 + 256i32 + 64i32 {
        CG_NewClientInfo(num - (32i32 + 256i32 + 256i32));
        CG_BuildSpectatorString();
    } else if num == 23i32 {
        if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint {
            cgs.redflag = *str.offset(0isize) as libc::c_int - '0' as i32;
            cgs.blueflag = *str.offset(1isize) as libc::c_int - '0' as i32
        }
    } else if num == 24i32 {
        CG_ShaderStateChanged();
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_ShaderStateChanged() {
    let mut originalShader: [libc::c_char; 64] = [0; 64];
    let mut newShader: [libc::c_char; 64] = [0; 64];
    let mut timeOffset: [libc::c_char; 16] = [0; 16];
    let mut o: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    o = CG_ConfigString(24i32);
    while !o.is_null() && 0 != *o as libc::c_int {
        n = strstr(o, b"=\x00" as *const u8 as *const libc::c_char);
        if !(!n.is_null() && 0 != *n as libc::c_int) {
            break;
        }
        strncpy(
            originalShader.as_mut_ptr(),
            o,
            n.wrapping_offset_from(o) as libc::c_long as libc::c_ulong,
        );
        originalShader[n.wrapping_offset_from(o) as libc::c_long as usize] = 0i32 as libc::c_char;
        n = n.offset(1isize);
        t = strstr(n, b":\x00" as *const u8 as *const libc::c_char);
        if !(!t.is_null() && 0 != *t as libc::c_int) {
            break;
        }
        strncpy(
            newShader.as_mut_ptr(),
            n,
            t.wrapping_offset_from(n) as libc::c_long as libc::c_ulong,
        );
        newShader[t.wrapping_offset_from(n) as libc::c_long as usize] = 0i32 as libc::c_char;
        t = t.offset(1isize);
        o = strstr(t, b"@\x00" as *const u8 as *const libc::c_char);
        if !o.is_null() {
            strncpy(
                timeOffset.as_mut_ptr(),
                t,
                o.wrapping_offset_from(t) as libc::c_long as libc::c_ulong,
            );
            timeOffset[o.wrapping_offset_from(t) as libc::c_long as usize] = 0i32 as libc::c_char;
            o = o.offset(1isize);
            trap_R_RemapShader(
                originalShader.as_mut_ptr(),
                newShader.as_mut_ptr(),
                timeOffset.as_mut_ptr(),
            );
        }
    }
}
/*
==================
CG_ParseWarmup
==================
*/
unsafe extern "C" fn CG_ParseWarmup() {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut warmup: libc::c_int = 0;
    info = CG_ConfigString(5i32);
    warmup = atoi(info);
    cg.warmupCount = -1i32;
    if !(warmup == 0i32 && 0 != cg.warmup) {
        if warmup > 0i32 && cg.warmup <= 0i32 {
            trap_S_StartLocalSound(cgs.media.countPrepareSound, CHAN_ANNOUNCER as libc::c_int);
        }
    }
    cg.warmup = warmup;
}
#[no_mangle]
pub unsafe extern "C" fn CG_ParseServerinfo() {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapname: *mut libc::c_char = 0 as *mut libc::c_char;
    info = CG_ConfigString(0i32);
    cgs.gametype = atoi(Info_ValueForKey(
        info,
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    )) as gametype_t;
    trap_Cvar_Set(
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cgs.gametype as libc::c_uint,
        ),
    );
    cgs.dmflags = atoi(Info_ValueForKey(
        info,
        b"dmflags\x00" as *const u8 as *const libc::c_char,
    ));
    cgs.teamflags = atoi(Info_ValueForKey(
        info,
        b"teamflags\x00" as *const u8 as *const libc::c_char,
    ));
    cgs.fraglimit = atoi(Info_ValueForKey(
        info,
        b"fraglimit\x00" as *const u8 as *const libc::c_char,
    ));
    cgs.capturelimit = atoi(Info_ValueForKey(
        info,
        b"capturelimit\x00" as *const u8 as *const libc::c_char,
    ));
    cgs.timelimit = atoi(Info_ValueForKey(
        info,
        b"timelimit\x00" as *const u8 as *const libc::c_char,
    ));
    cgs.maxclients = atoi(Info_ValueForKey(
        info,
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ));
    mapname = Info_ValueForKey(info, b"mapname\x00" as *const u8 as *const libc::c_char);
    Com_sprintf(
        cgs.mapname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char,
        mapname,
    );
    Q_strncpyz(
        cgs.redTeam.as_mut_ptr(),
        Info_ValueForKey(info, b"g_redTeam\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    trap_Cvar_Set(
        b"g_redTeam\x00" as *const u8 as *const libc::c_char,
        cgs.redTeam.as_mut_ptr(),
    );
    Q_strncpyz(
        cgs.blueTeam.as_mut_ptr(),
        Info_ValueForKey(info, b"g_blueTeam\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    trap_Cvar_Set(
        b"g_blueTeam\x00" as *const u8 as *const libc::c_char,
        cgs.blueTeam.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_SetConfigValues() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    cgs.scores1 = atoi(CG_ConfigString(6i32));
    cgs.scores2 = atoi(CG_ConfigString(7i32));
    cgs.levelStartTime = atoi(CG_ConfigString(21i32));
    if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint {
        s = CG_ConfigString(23i32);
        cgs.redflag = *s.offset(0isize) as libc::c_int - '0' as i32;
        cgs.blueflag = *s.offset(1isize) as libc::c_int - '0' as i32
    }
    cg.warmup = atoi(CG_ConfigString(5i32));
}
