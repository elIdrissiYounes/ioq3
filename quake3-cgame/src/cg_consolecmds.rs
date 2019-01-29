use bg_public_h::{animation_s, animation_t};
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
    centity_s, centity_t, cg_t, lerpFrame_t, playerEntity_t, score_t, trap_AddCommand, trap_Args,
    trap_Argv, trap_Cvar_Set, trap_Cvar_VariableStringBuffer, trap_SendClientCommand,
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
    byte, cvarHandle_t, entityState_s, entityState_t, playerState_s, playerState_t, qboolean,
    qfalse, qhandle_t, qtrue, trType_t, trajectory_t, va, vec3_t, vec_t, vmCvar_t, Com_sprintf,
    Q_stricmp, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::atoi;
use tr_types_h::{
    refEntityType_t, refEntity_t, refdef_t, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE,
};

//
// cg_consolecmds.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_ConsoleCommand() -> qboolean {
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    cmd = CG_Argv(0i32);
    i = 0i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[consoleCommand_t; 21]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<consoleCommand_t>() as libc::c_ulong)
    {
        if 0 == Q_stricmp(cmd, commands[i as usize].cmd) {
            commands[i as usize]
                .function
                .expect("non-null function pointer")();
            return qtrue;
        }
        i += 1
    }
    return qfalse;
}
static mut commands: [consoleCommand_t; 21] = [
    consoleCommand_t {
        cmd: b"testgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TestGun_f),
    },
    consoleCommand_t {
        cmd: b"testmodel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TestModel_f),
    },
    consoleCommand_t {
        cmd: b"nextframe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TestModelNextFrame_f),
    },
    consoleCommand_t {
        cmd: b"prevframe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TestModelPrevFrame_f),
    },
    consoleCommand_t {
        cmd: b"nextskin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TestModelNextSkin_f),
    },
    consoleCommand_t {
        cmd: b"prevskin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TestModelPrevSkin_f),
    },
    consoleCommand_t {
        cmd: b"viewpos\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_Viewpos_f),
    },
    consoleCommand_t {
        cmd: b"+scores\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_ScoresDown_f),
    },
    consoleCommand_t {
        cmd: b"-scores\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_ScoresUp_f),
    },
    consoleCommand_t {
        cmd: b"+zoom\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_ZoomDown_f),
    },
    consoleCommand_t {
        cmd: b"-zoom\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_ZoomUp_f),
    },
    consoleCommand_t {
        cmd: b"sizeup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_SizeUp_f),
    },
    consoleCommand_t {
        cmd: b"sizedown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_SizeDown_f),
    },
    consoleCommand_t {
        cmd: b"weapnext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_NextWeapon_f),
    },
    consoleCommand_t {
        cmd: b"weapprev\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_PrevWeapon_f),
    },
    consoleCommand_t {
        cmd: b"weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_Weapon_f),
    },
    consoleCommand_t {
        cmd: b"tcmd\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TargetCommand_f),
    },
    consoleCommand_t {
        cmd: b"tell_target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TellTarget_f),
    },
    consoleCommand_t {
        cmd: b"tell_attacker\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_TellAttacker_f),
    },
    consoleCommand_t {
        cmd: b"startOrbit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_StartOrbit_f),
    },
    consoleCommand_t {
        cmd: b"loaddeferred\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        function: Some(CG_LoadDeferredPlayers),
    },
];
/*
==================
CG_StartOrbit_f
==================
*/
unsafe extern "C" fn CG_StartOrbit_f() {
    let mut var: [libc::c_char; 1024] = [0; 1024];
    trap_Cvar_VariableStringBuffer(
        b"developer\x00" as *const u8 as *const libc::c_char,
        var.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == atoi(var.as_mut_ptr()) {
        return;
    }
    if cg_cameraOrbit.value != 0i32 as libc::c_float {
        trap_Cvar_Set(
            b"cg_cameraOrbit\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Set(
            b"cg_thirdPerson\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        trap_Cvar_Set(
            b"cg_cameraOrbit\x00" as *const u8 as *const libc::c_char,
            b"5\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Set(
            b"cg_thirdPerson\x00" as *const u8 as *const libc::c_char,
            b"1\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Set(
            b"cg_thirdPersonAngle\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Set(
            b"cg_thirdPersonRange\x00" as *const u8 as *const libc::c_char,
            b"100\x00" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn CG_TellAttacker_f() {
    let mut clientNum: libc::c_int = 0;
    let mut command: [libc::c_char; 128] = [0; 128];
    let mut message: [libc::c_char; 128] = [0; 128];
    clientNum = CG_LastAttacker();
    if clientNum == -1i32 {
        return;
    }
    trap_Args(message.as_mut_ptr(), 128i32);
    Com_sprintf(
        command.as_mut_ptr(),
        128i32,
        b"tell %i %s\x00" as *const u8 as *const libc::c_char,
        clientNum,
        message.as_mut_ptr(),
    );
    trap_SendClientCommand(command.as_mut_ptr());
}
unsafe extern "C" fn CG_TellTarget_f() {
    let mut clientNum: libc::c_int = 0;
    let mut command: [libc::c_char; 128] = [0; 128];
    let mut message: [libc::c_char; 128] = [0; 128];
    clientNum = CG_CrosshairPlayer();
    if clientNum == -1i32 {
        return;
    }
    trap_Args(message.as_mut_ptr(), 128i32);
    Com_sprintf(
        command.as_mut_ptr(),
        128i32,
        b"tell %i %s\x00" as *const u8 as *const libc::c_char,
        clientNum,
        message.as_mut_ptr(),
    );
    trap_SendClientCommand(command.as_mut_ptr());
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
// cg_consolecmds.c -- text commands typed in at the local console, or
// executed by a key binding
#[no_mangle]
pub unsafe extern "C" fn CG_TargetCommand_f() {
    let mut targetNum: libc::c_int = 0;
    let mut test: [libc::c_char; 4] = [0; 4];
    targetNum = CG_CrosshairPlayer();
    if targetNum == -1i32 {
        return;
    }
    trap_Argv(1i32, test.as_mut_ptr(), 4i32);
    trap_SendClientCommand(va(
        b"gc %i %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        targetNum,
        atoi(test.as_mut_ptr()),
    ));
}
/*
=================
CG_SizeDown_f

Keybinding command
=================
*/
unsafe extern "C" fn CG_SizeDown_f() {
    trap_Cvar_Set(
        b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cg_viewsize.integer - 10i32,
        ),
    );
}
/*
=================
CG_SizeUp_f

Keybinding command
=================
*/
unsafe extern "C" fn CG_SizeUp_f() {
    trap_Cvar_Set(
        b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cg_viewsize.integer + 10i32,
        ),
    );
}
unsafe extern "C" fn CG_ScoresUp_f() {
    if 0 != cg.showScores as u64 {
        cg.showScores = qfalse;
        cg.scoreFadeTime = cg.time
    };
}
unsafe extern "C" fn CG_ScoresDown_f() {
    if cg.scoresRequestTime + 2000i32 < cg.time {
        cg.scoresRequestTime = cg.time;
        trap_SendClientCommand(b"score\x00" as *const u8 as *const libc::c_char);
        if 0 == cg.showScores as u64 {
            cg.showScores = qtrue;
            cg.numScores = 0i32
        }
    } else {
        cg.showScores = qtrue
    };
}
/*
=============
CG_Viewpos_f

Debugging command to print the current position
=============
*/
unsafe extern "C" fn CG_Viewpos_f() {
    CG_Printf(
        b"(%i %i %i) : %i\n\x00" as *const u8 as *const libc::c_char,
        cg.refdef.vieworg[0usize] as libc::c_int,
        cg.refdef.vieworg[1usize] as libc::c_int,
        cg.refdef.vieworg[2usize] as libc::c_int,
        cg.refdefViewAngles[1usize] as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_InitConsoleCommands() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[consoleCommand_t; 21]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<consoleCommand_t>() as libc::c_ulong)
    {
        trap_AddCommand(commands[i as usize].cmd);
        i += 1
    }
    trap_AddCommand(b"kill\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"say\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"say_team\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"tell\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"give\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"god\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"notarget\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"noclip\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"where\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"team\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"follow\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"follownext\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"followprev\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"levelshot\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"addbot\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"setviewpos\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"callvote\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"vote\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"callteamvote\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"teamvote\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"stats\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"teamtask\x00" as *const u8 as *const libc::c_char);
    trap_AddCommand(b"loaddefered\x00" as *const u8 as *const libc::c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct consoleCommand_t {
    pub cmd: *mut libc::c_char,
    pub function: Option<unsafe extern "C" fn() -> ()>,
}
