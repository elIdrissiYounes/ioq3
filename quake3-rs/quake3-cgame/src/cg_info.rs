use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, gitem_s, gitem_t, itemType_t, team_t,
    GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR,
    IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON,
    TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, lerpFrame_t,
    playerEntity_t, score_t, trap_Cvar_VariableStringBuffer, trap_R_DrawStretchPic,
    trap_R_RegisterShader, trap_R_RegisterShaderNoMip, trap_R_SetColor, trap_S_RegisterSound,
    trap_UpdateScreen, FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH,
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
    qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, va, vec3_t, vec4_t, vec_t,
    Com_sprintf, Info_ValueForKey, Q_CleanStr, Q_strncpyz, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, strrchr};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

//
// cg_info.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_LoadingString(mut s: *const libc::c_char) {
    Q_strncpyz(
        cg.infoScreenText.as_mut_ptr(),
        s,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    trap_UpdateScreen();
}
#[no_mangle]
pub unsafe extern "C" fn CG_LoadingItem(mut itemNum: libc::c_int) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    item = &mut *bg_itemlist.as_mut_ptr().offset(itemNum as isize) as *mut gitem_t;
    if !(*item).icon.is_null() && loadingItemIconCount < 26i32 {
        let fresh0 = loadingItemIconCount;
        loadingItemIconCount = loadingItemIconCount + 1;
        loadingItemIcons[fresh0 as usize] = trap_R_RegisterShaderNoMip((*item).icon)
    }
    CG_LoadingString((*item).pickup_name);
}
static mut loadingItemIconCount: libc::c_int = 0;
static mut loadingItemIcons: [qhandle_t; 26] = [0; 26];
#[no_mangle]
pub unsafe extern "C" fn CG_LoadingClient(mut clientNum: libc::c_int) {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut personality: [libc::c_char; 64] = [0; 64];
    let mut model: [libc::c_char; 64] = [0; 64];
    let mut iconName: [libc::c_char; 64] = [0; 64];
    info = CG_ConfigString(32i32 + 256i32 + 256i32 + clientNum);
    if loadingPlayerIconCount < 16i32 {
        Q_strncpyz(
            model.as_mut_ptr(),
            Info_ValueForKey(info, b"model\x00" as *const u8 as *const libc::c_char),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        skin = strrchr(model.as_mut_ptr(), '/' as i32);
        if !skin.is_null() {
            let fresh1 = skin;
            skin = skin.offset(1);
            *fresh1 = '\u{0}' as i32 as libc::c_char
        } else {
            skin = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        Com_sprintf(
            iconName.as_mut_ptr(),
            64i32,
            b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
            model.as_mut_ptr(),
            skin,
        );
        loadingPlayerIcons[loadingPlayerIconCount as usize] =
            trap_R_RegisterShaderNoMip(iconName.as_mut_ptr());
        if 0 == loadingPlayerIcons[loadingPlayerIconCount as usize] {
            Com_sprintf(
                iconName.as_mut_ptr(),
                64i32,
                b"models/players/characters/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
                model.as_mut_ptr(),
                skin,
            );
            loadingPlayerIcons[loadingPlayerIconCount as usize] =
                trap_R_RegisterShaderNoMip(iconName.as_mut_ptr())
        }
        if 0 == loadingPlayerIcons[loadingPlayerIconCount as usize] {
            Com_sprintf(
                iconName.as_mut_ptr(),
                64i32,
                b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
                b"sarge\x00" as *const u8 as *const libc::c_char,
                b"default\x00" as *const u8 as *const libc::c_char,
            );
            loadingPlayerIcons[loadingPlayerIconCount as usize] =
                trap_R_RegisterShaderNoMip(iconName.as_mut_ptr())
        }
        if 0 != loadingPlayerIcons[loadingPlayerIconCount as usize] {
            loadingPlayerIconCount += 1
        }
    }
    Q_strncpyz(
        personality.as_mut_ptr(),
        Info_ValueForKey(info, b"n\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_CleanStr(personality.as_mut_ptr());
    if cgs.gametype as libc::c_uint == GT_SINGLE_PLAYER as libc::c_int as libc::c_uint {
        trap_S_RegisterSound(
            va(
                b"sound/player/announce/%s.wav\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                personality.as_mut_ptr(),
            ),
            qtrue,
        );
    }
    CG_LoadingString(personality.as_mut_ptr());
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
// cg_info.c -- display information while data is being loading
static mut loadingPlayerIconCount: libc::c_int = 0;
static mut loadingPlayerIcons: [qhandle_t; 16] = [0; 16];
#[no_mangle]
pub unsafe extern "C" fn CG_DrawInformation() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut sysInfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut y: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut levelshot: qhandle_t = 0;
    let mut detail: qhandle_t = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    info = CG_ConfigString(0i32);
    sysInfo = CG_ConfigString(1i32);
    s = Info_ValueForKey(info, b"mapname\x00" as *const u8 as *const libc::c_char);
    levelshot = trap_R_RegisterShaderNoMip(va(
        b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    ));
    if 0 == levelshot {
        levelshot = trap_R_RegisterShaderNoMip(
            b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char,
        )
    }
    trap_R_SetColor(0 as *const libc::c_float);
    CG_DrawPic(
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        640i32 as libc::c_float,
        480i32 as libc::c_float,
        levelshot,
    );
    detail = trap_R_RegisterShader(b"levelShotDetail\x00" as *const u8 as *const libc::c_char);
    trap_R_DrawStretchPic(
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        cgs.glconfig.vidWidth as libc::c_float,
        cgs.glconfig.vidHeight as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        2.5f64 as libc::c_float,
        2i32 as libc::c_float,
        detail,
    );
    CG_DrawLoadingIcons();
    if 0 != cg.infoScreenText[0usize] {
        UI_DrawProportionalString(
            320i32,
            128i32 - 32i32,
            va(
                b"Loading... %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                cg.infoScreenText.as_mut_ptr(),
            ),
            0x1i32 | 0x10i32 | 0x800i32,
            colorWhite.as_mut_ptr(),
        );
    } else {
        UI_DrawProportionalString(
            320i32,
            128i32 - 32i32,
            b"Awaiting snapshot...\x00" as *const u8 as *const libc::c_char,
            0x1i32 | 0x10i32 | 0x800i32,
            colorWhite.as_mut_ptr(),
        );
    }
    y = 180i32 - 32i32;
    trap_Cvar_VariableStringBuffer(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == atoi(buf.as_mut_ptr()) {
        Q_strncpyz(
            buf.as_mut_ptr(),
            Info_ValueForKey(info, b"sv_hostname\x00" as *const u8 as *const libc::c_char),
            1024i32,
        );
        Q_CleanStr(buf.as_mut_ptr());
        UI_DrawProportionalString(
            320i32,
            y,
            buf.as_mut_ptr(),
            0x1i32 | 0x10i32 | 0x800i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27i32;
        s = Info_ValueForKey(sysInfo, b"sv_pure\x00" as *const u8 as *const libc::c_char);
        if *s.offset(0isize) as libc::c_int == '1' as i32 {
            UI_DrawProportionalString(
                320i32,
                y,
                b"Pure Server\x00" as *const u8 as *const libc::c_char,
                0x1i32 | 0x10i32 | 0x800i32,
                colorWhite.as_mut_ptr(),
            );
            y += 27i32
        }
        s = CG_ConfigString(4i32);
        if 0 != *s.offset(0isize) {
            UI_DrawProportionalString(
                320i32,
                y,
                s,
                0x1i32 | 0x10i32 | 0x800i32,
                colorWhite.as_mut_ptr(),
            );
            y += 27i32
        }
        y += 10i32
    }
    s = CG_ConfigString(3i32);
    if 0 != *s.offset(0isize) {
        UI_DrawProportionalString(
            320i32,
            y,
            s,
            0x1i32 | 0x10i32 | 0x800i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27i32
    }
    s = Info_ValueForKey(
        sysInfo,
        b"sv_cheats\x00" as *const u8 as *const libc::c_char,
    );
    if *s.offset(0isize) as libc::c_int == '1' as i32 {
        UI_DrawProportionalString(
            320i32,
            y,
            b"CHEATS ARE ENABLED\x00" as *const u8 as *const libc::c_char,
            0x1i32 | 0x10i32 | 0x800i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27i32
    }
    match cgs.gametype as libc::c_uint {
        0 => s = b"Free For All\x00" as *const u8 as *const libc::c_char,
        2 => s = b"Single Player\x00" as *const u8 as *const libc::c_char,
        1 => s = b"Tournament\x00" as *const u8 as *const libc::c_char,
        3 => s = b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
        4 => s = b"Capture The Flag\x00" as *const u8 as *const libc::c_char,
        _ => s = b"Unknown Gametype\x00" as *const u8 as *const libc::c_char,
    }
    UI_DrawProportionalString(
        320i32,
        y,
        s,
        0x1i32 | 0x10i32 | 0x800i32,
        colorWhite.as_mut_ptr(),
    );
    y += 27i32;
    value = atoi(Info_ValueForKey(
        info,
        b"timelimit\x00" as *const u8 as *const libc::c_char,
    ));
    if 0 != value {
        UI_DrawProportionalString(
            320i32,
            y,
            va(
                b"timelimit %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                value,
            ),
            0x1i32 | 0x10i32 | 0x800i32,
            colorWhite.as_mut_ptr(),
        );
        y += 27i32
    }
    if (cgs.gametype as libc::c_uint) < GT_CTF as libc::c_int as libc::c_uint {
        value = atoi(Info_ValueForKey(
            info,
            b"fraglimit\x00" as *const u8 as *const libc::c_char,
        ));
        if 0 != value {
            UI_DrawProportionalString(
                320i32,
                y,
                va(
                    b"fraglimit %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    value,
                ),
                0x1i32 | 0x10i32 | 0x800i32,
                colorWhite.as_mut_ptr(),
            );
            y += 27i32
        }
    }
    if cgs.gametype as libc::c_uint >= GT_CTF as libc::c_int as libc::c_uint {
        value = atoi(Info_ValueForKey(
            info,
            b"capturelimit\x00" as *const u8 as *const libc::c_char,
        ));
        if 0 != value {
            UI_DrawProportionalString(
                320i32,
                y,
                va(
                    b"capturelimit %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    value,
                ),
                0x1i32 | 0x10i32 | 0x800i32,
                colorWhite.as_mut_ptr(),
            );
        }
    };
}
/*
===================
CG_DrawLoadingIcons
===================
*/
unsafe extern "C" fn CG_DrawLoadingIcons() {
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    n = 0i32;
    while n < loadingPlayerIconCount {
        x = 16i32 + n * 78i32;
        y = 324i32 - 40i32;
        CG_DrawPic(
            x as libc::c_float,
            y as libc::c_float,
            64i32 as libc::c_float,
            64i32 as libc::c_float,
            loadingPlayerIcons[n as usize],
        );
        n += 1
    }
    n = 0i32;
    while n < loadingItemIconCount {
        y = 400i32 - 40i32;
        if n >= 13i32 {
            y += 40i32
        }
        x = 16i32 + n % 13i32 * 48i32;
        CG_DrawPic(
            x as libc::c_float,
            y as libc::c_float,
            32i32 as libc::c_float,
            32i32 as libc::c_float,
            loadingItemIcons[n as usize],
        );
        n += 1
    }
}
