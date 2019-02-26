#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_local_h::pml_t;
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, pmove_t, unnamed, unnamed_0, unnamed_1, unnamed_2, unnamed_3,
    unnamed_4, unnamed_5, unnamed_6, unnamed_7, unnamed_8, BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3,
    BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL,
    EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM,
    EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE,
    EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND,
    EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP, EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED,
    EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT, EV_MISSILE_HIT, EV_MISSILE_MISS,
    EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE, EV_OBELISKPAIN, EV_OBITUARY,
    EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT, EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD,
    EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK, EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL,
    EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16, EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND,
    EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME, EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO,
    EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0, EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11,
    EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14, EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3,
    EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6, EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9,
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, FLAG_RUN, FLAG_STAND,
    FLAG_STAND2RUN, HI_INVULNERABILITY, HI_KAMIKAZE, HI_MEDKIT, HI_NONE, HI_NUM_HOLDABLE,
    HI_PORTAL, HI_TELEPORTER, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK,
    LEGS_IDLE, LEGS_IDLECR, LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM,
    LEGS_TURN, LEGS_WALK, LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS, PERS_ASSIST_COUNT,
    PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT,
    PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY,
    PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    TORSO_AFFIRMATIVE, TORSO_ATTACK, TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE,
    TORSO_GETFLAG, TORSO_GUARDBASE, TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND,
    TORSO_STAND2, WEAPON_DROPPING, WEAPON_FIRING, WEAPON_RAISING, WEAPON_READY, WP_BFG,
    WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE,
    WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
use stdlib::{abs, memset, sqrt};
extern crate libc;

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
// if a full pmove isn't done on the client, you can just update the angles
#[no_mangle]
pub unsafe extern "C" fn PM_UpdateViewAngles(
    mut ps: *mut playerState_t,
    mut cmd: *const usercmd_t,
) {
    let mut temp: libc::c_short = 0;
    let mut i: libc::c_int = 0;
    if (*ps).pm_type == PM_INTERMISSION as libc::c_int
        || (*ps).pm_type == PM_SPINTERMISSION as libc::c_int
    {
        return;
    }
    if (*ps).pm_type != PM_SPECTATOR as libc::c_int
        && (*ps).stats[STAT_HEALTH as libc::c_int as usize] <= 0i32
    {
        return;
    }
    i = 0i32;
    while i < 3i32 {
        temp = ((*cmd).angles[i as usize] + (*ps).delta_angles[i as usize]) as libc::c_short;
        if i == 0i32 {
            if temp as libc::c_int > 16000i32 {
                (*ps).delta_angles[i as usize] = 16000i32 - (*cmd).angles[i as usize];
                temp = 16000i32 as libc::c_short
            } else if (temp as libc::c_int) < -16000i32 {
                (*ps).delta_angles[i as usize] = -16000i32 - (*cmd).angles[i as usize];
                temp = -16000i32 as libc::c_short
            }
        }
        (*ps).viewangles[i as usize] = (temp as libc::c_int as libc::c_double
            * (360.0f64 / 65536i32 as libc::c_double))
            as vec_t;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn Pmove(mut pmove: *mut pmove_t) {
    let mut finalTime: libc::c_int = 0;
    finalTime = (*pmove).cmd.serverTime;
    if finalTime < (*(*pmove).ps).commandTime {
        return;
    }
    if finalTime > (*(*pmove).ps).commandTime + 1000i32 {
        (*(*pmove).ps).commandTime = finalTime - 1000i32
    }
    (*(*pmove).ps).pmove_framecount =
        (*(*pmove).ps).pmove_framecount + 1i32 & (1i32 << 6i32) - 1i32;
    while (*(*pmove).ps).commandTime != finalTime {
        let mut msec: libc::c_int = 0;
        msec = finalTime - (*(*pmove).ps).commandTime;
        if 0 != (*pmove).pmove_fixed {
            if msec > (*pmove).pmove_msec {
                msec = (*pmove).pmove_msec
            }
        } else if msec > 66i32 {
            msec = 66i32
        }
        (*pmove).cmd.serverTime = (*(*pmove).ps).commandTime + msec;
        PmoveSingle(pmove);
        if 0 != (*(*pmove).ps).pm_flags & 2i32 {
            (*pmove).cmd.upmove = 20i32 as libc::c_schar
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PmoveSingle(mut pmove: *mut pmove_t) {
    pm = pmove;
    c_pmove += 1;
    (*pm).numtouch = 0i32;
    (*pm).watertype = 0i32;
    (*pm).waterlevel = 0i32;
    if (*(*pm).ps).stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        (*pm).tracemask &= !0x2000000i32
    }
    if abs((*pm).cmd.forwardmove as libc::c_int) > 64i32
        || abs((*pm).cmd.rightmove as libc::c_int) > 64i32
    {
        (*pm).cmd.buttons &= !16i32
    }
    if 0 != (*pm).cmd.buttons & 2i32 {
        (*(*pm).ps).eFlags |= 0x1000i32
    } else {
        (*(*pm).ps).eFlags &= !0x1000i32
    }
    if 0 == (*(*pm).ps).pm_flags & 512i32
        && (*(*pm).ps).pm_type != PM_INTERMISSION as libc::c_int
        && (*(*pm).ps).pm_type != PM_NOCLIP as libc::c_int
        && 0 != (*pm).cmd.buttons & 1i32
        && 0 != (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize]
    {
        (*(*pm).ps).eFlags |= 0x100i32
    } else {
        (*(*pm).ps).eFlags &= !0x100i32
    }
    if (*(*pm).ps).stats[STAT_HEALTH as libc::c_int as usize] > 0i32
        && 0 == (*pm).cmd.buttons & (1i32 | 4i32)
    {
        (*(*pm).ps).pm_flags &= !512i32
    }
    if 0 != (*pmove).cmd.buttons & 2i32 {
        (*pmove).cmd.buttons = 2i32;
        (*pmove).cmd.forwardmove = 0i32 as libc::c_schar;
        (*pmove).cmd.rightmove = 0i32 as libc::c_schar;
        (*pmove).cmd.upmove = 0i32 as libc::c_schar
    }
    memset(
        &mut pml as *mut pml_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<pml_t>() as libc::c_ulong,
    );
    pml.msec = (*pmove).cmd.serverTime - (*(*pm).ps).commandTime;
    if pml.msec < 1i32 {
        pml.msec = 1i32
    } else if pml.msec > 200i32 {
        pml.msec = 200i32
    }
    (*(*pm).ps).commandTime = (*pmove).cmd.serverTime;
    pml.previous_origin[0usize] = (*(*pm).ps).origin[0usize];
    pml.previous_origin[1usize] = (*(*pm).ps).origin[1usize];
    pml.previous_origin[2usize] = (*(*pm).ps).origin[2usize];
    pml.previous_velocity[0usize] = (*(*pm).ps).velocity[0usize];
    pml.previous_velocity[1usize] = (*(*pm).ps).velocity[1usize];
    pml.previous_velocity[2usize] = (*(*pm).ps).velocity[2usize];
    pml.frametime = (pml.msec as libc::c_double * 0.001f64) as libc::c_float;
    PM_UpdateViewAngles((*pm).ps, &mut (*pm).cmd);
    AngleVectors(
        (*(*pm).ps).viewangles.as_mut_ptr() as *const vec_t,
        pml.forward.as_mut_ptr(),
        pml.right.as_mut_ptr(),
        pml.up.as_mut_ptr(),
    );
    if ((*pm).cmd.upmove as libc::c_int) < 10i32 {
        (*(*pm).ps).pm_flags &= !2i32
    }
    if ((*pm).cmd.forwardmove as libc::c_int) < 0i32 {
        (*(*pm).ps).pm_flags |= 16i32
    } else if (*pm).cmd.forwardmove as libc::c_int > 0i32
        || (*pm).cmd.forwardmove as libc::c_int == 0i32 && 0 != (*pm).cmd.rightmove as libc::c_int
    {
        (*(*pm).ps).pm_flags &= !16i32
    }
    if (*(*pm).ps).pm_type >= PM_DEAD as libc::c_int {
        (*pm).cmd.forwardmove = 0i32 as libc::c_schar;
        (*pm).cmd.rightmove = 0i32 as libc::c_schar;
        (*pm).cmd.upmove = 0i32 as libc::c_schar
    }
    if (*(*pm).ps).pm_type == PM_SPECTATOR as libc::c_int {
        PM_CheckDuck();
        PM_FlyMove();
        PM_DropTimers();
        return;
    }
    if (*(*pm).ps).pm_type == PM_NOCLIP as libc::c_int {
        PM_NoclipMove();
        PM_DropTimers();
        return;
    }
    if (*(*pm).ps).pm_type == PM_FREEZE as libc::c_int {
        return;
    }
    if (*(*pm).ps).pm_type == PM_INTERMISSION as libc::c_int
        || (*(*pm).ps).pm_type == PM_SPINTERMISSION as libc::c_int
    {
        return;
    }
    PM_SetWaterLevel();
    pml.previous_waterlevel = (*pmove).waterlevel;
    PM_CheckDuck();
    PM_GroundTrace();
    if (*(*pm).ps).pm_type == PM_DEAD as libc::c_int {
        PM_DeadMove();
    }
    PM_DropTimers();
    if 0 != (*(*pm).ps).powerups[PW_FLIGHT as libc::c_int as usize] {
        PM_FlyMove();
    } else if 0 != (*(*pm).ps).pm_flags & 2048i32 {
        PM_GrappleMove();
        PM_AirMove();
    } else if 0 != (*(*pm).ps).pm_flags & 256i32 {
        PM_WaterJumpMove();
    } else if (*pm).waterlevel > 1i32 {
        PM_WaterMove();
    } else if 0 != pml.walking as u64 {
        PM_WalkMove();
    } else {
        PM_AirMove();
    }
    PM_Animate();
    PM_GroundTrace();
    PM_SetWaterLevel();
    PM_Weapon();
    PM_TorsoAnimation();
    PM_Footsteps();
    PM_WaterEvents();
    trap_SnapVector((*(*pm).ps).velocity.as_mut_ptr());
}
#[no_mangle]
pub static mut pm: *mut pmove_t = 0 as *const pmove_t as *mut pmove_t;
// no sound when completely underwater
/*
==============
PM_WaterEvents

Generate sound events for entering and leaving water
==============
*/
// FIXME?
unsafe extern "C" fn PM_WaterEvents() {
    if 0 == pml.previous_waterlevel && 0 != (*pm).waterlevel {
        PM_AddEvent(EV_WATER_TOUCH as libc::c_int);
    }
    if 0 != pml.previous_waterlevel && 0 == (*pm).waterlevel {
        PM_AddEvent(EV_WATER_LEAVE as libc::c_int);
    }
    if pml.previous_waterlevel != 3i32 && (*pm).waterlevel == 3i32 {
        PM_AddEvent(EV_WATER_UNDER as libc::c_int);
    }
    if pml.previous_waterlevel == 3i32 && (*pm).waterlevel != 3i32 {
        PM_AddEvent(EV_WATER_CLEAR as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn PM_AddEvent(mut newEvent: libc::c_int) {
    BG_AddPredictableEventToPlayerstate(newEvent, 0i32, (*pm).ps);
}
#[no_mangle]
pub static mut pml: pml_t = pml_t {
    forward: [0.; 3],
    right: [0.; 3],
    up: [0.; 3],
    frametime: 0.,
    msec: 0,
    walking: qfalse,
    groundPlane: qfalse,
    groundTrace: trace_t {
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
    },
    impactSpeed: 0.,
    previous_origin: [0.; 3],
    previous_velocity: [0.; 3],
    previous_waterlevel: 0,
};
//===================================================================
/*
===============
PM_Footsteps
===============
*/
unsafe extern "C" fn PM_Footsteps() {
    let mut bobmove: libc::c_float = 0.;
    let mut old: libc::c_int = 0;
    let mut footstep: qboolean = qfalse;
    (*pm).xyspeed = sqrt(
        ((*(*pm).ps).velocity[0usize] * (*(*pm).ps).velocity[0usize]
            + (*(*pm).ps).velocity[1usize] * (*(*pm).ps).velocity[1usize])
            as libc::c_double,
    ) as libc::c_float;
    if (*(*pm).ps).groundEntityNum == (1i32 << 10i32) - 1i32 {
        if 0 != (*(*pm).ps).powerups[PW_INVULNERABILITY as libc::c_int as usize] {
            PM_ContinueLegsAnim(LEGS_IDLECR as libc::c_int);
        }
        if (*pm).waterlevel > 1i32 {
            PM_ContinueLegsAnim(LEGS_SWIM as libc::c_int);
        }
        return;
    }
    if 0 == (*pm).cmd.forwardmove && 0 == (*pm).cmd.rightmove {
        if (*pm).xyspeed < 5i32 as libc::c_float {
            (*(*pm).ps).bobCycle = 0i32;
            if 0 != (*(*pm).ps).pm_flags & 1i32 {
                PM_ContinueLegsAnim(LEGS_IDLECR as libc::c_int);
            } else {
                PM_ContinueLegsAnim(LEGS_IDLE as libc::c_int);
            }
        }
        return;
    }
    footstep = qfalse;
    if 0 != (*(*pm).ps).pm_flags & 1i32 {
        bobmove = 0.5f64 as libc::c_float;
        if 0 != (*(*pm).ps).pm_flags & 16i32 {
            PM_ContinueLegsAnim(LEGS_BACKCR as libc::c_int);
        } else {
            PM_ContinueLegsAnim(LEGS_WALKCR as libc::c_int);
        }
    } else if 0 == (*pm).cmd.buttons & 16i32 {
        bobmove = 0.4f32;
        if 0 != (*(*pm).ps).pm_flags & 16i32 {
            PM_ContinueLegsAnim(LEGS_BACK as libc::c_int);
        } else {
            PM_ContinueLegsAnim(LEGS_RUN as libc::c_int);
        }
        footstep = qtrue
    } else {
        bobmove = 0.3f32;
        if 0 != (*(*pm).ps).pm_flags & 16i32 {
            PM_ContinueLegsAnim(LEGS_BACKWALK as libc::c_int);
        } else {
            PM_ContinueLegsAnim(LEGS_WALK as libc::c_int);
        }
    }
    old = (*(*pm).ps).bobCycle;
    (*(*pm).ps).bobCycle =
        (old as libc::c_float + bobmove * pml.msec as libc::c_float) as libc::c_int & 255i32;
    if 0 != (old + 64i32 ^ (*(*pm).ps).bobCycle + 64i32) & 128i32 {
        if (*pm).waterlevel == 0i32 {
            if 0 != footstep as libc::c_uint && 0 == (*pm).noFootsteps as u64 {
                PM_AddEvent(PM_FootstepForSurface());
            }
        } else if (*pm).waterlevel == 1i32 {
            PM_AddEvent(EV_FOOTSPLASH as libc::c_int);
        } else if (*pm).waterlevel == 2i32 {
            PM_AddEvent(EV_SWIM as libc::c_int);
        } else {
            (*pm).waterlevel == 3i32;
        }
    };
}
//============================================================================
/*
================
PM_FootstepForSurface

Returns an event number appropriate for the groundsurface
================
*/
unsafe extern "C" fn PM_FootstepForSurface() -> libc::c_int {
    if 0 != pml.groundTrace.surfaceFlags & 0x2000i32 {
        return 0i32;
    }
    if 0 != pml.groundTrace.surfaceFlags & 0x1000i32 {
        return EV_FOOTSTEP_METAL as libc::c_int;
    }
    return EV_FOOTSTEP as libc::c_int;
}
unsafe extern "C" fn PM_ContinueLegsAnim(mut anim: libc::c_int) {
    if (*(*pm).ps).legsAnim & !128i32 == anim {
        return;
    }
    if (*(*pm).ps).legsTimer > 0i32 {
        return;
    }
    PM_StartLegsAnim(anim);
}
unsafe extern "C" fn PM_StartLegsAnim(mut anim: libc::c_int) {
    if (*(*pm).ps).pm_type >= PM_DEAD as libc::c_int {
        return;
    }
    if (*(*pm).ps).legsTimer > 0i32 {
        return;
    }
    (*(*pm).ps).legsAnim = (*(*pm).ps).legsAnim & 128i32 ^ 128i32 | anim;
}
/*
==============
PM_TorsoAnimation

==============
*/
unsafe extern "C" fn PM_TorsoAnimation() {
    if (*(*pm).ps).weaponstate == WEAPON_READY as libc::c_int {
        if (*(*pm).ps).weapon == WP_GAUNTLET as libc::c_int {
            PM_ContinueTorsoAnim(TORSO_STAND2 as libc::c_int);
        } else {
            PM_ContinueTorsoAnim(TORSO_STAND as libc::c_int);
        }
        return;
    };
}
unsafe extern "C" fn PM_ContinueTorsoAnim(mut anim: libc::c_int) {
    if (*(*pm).ps).torsoAnim & !128i32 == anim {
        return;
    }
    if (*(*pm).ps).torsoTimer > 0i32 {
        return;
    }
    PM_StartTorsoAnim(anim);
}
/*
===================
PM_StartTorsoAnim
===================
*/
unsafe extern "C" fn PM_StartTorsoAnim(mut anim: libc::c_int) {
    if (*(*pm).ps).pm_type >= PM_DEAD as libc::c_int {
        return;
    }
    (*(*pm).ps).torsoAnim = (*(*pm).ps).torsoAnim & 128i32 ^ 128i32 | anim;
}
/*
==============
PM_Weapon

Generates weapon events and modifes the weapon counter
==============
*/
unsafe extern "C" fn PM_Weapon() {
    let mut addTime: libc::c_int = 0;
    if 0 != (*(*pm).ps).pm_flags & 512i32 {
        return;
    }
    if (*(*pm).ps).persistant[PERS_TEAM as libc::c_int as usize] == TEAM_SPECTATOR as libc::c_int {
        return;
    }
    if (*(*pm).ps).stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        (*(*pm).ps).weapon = WP_NONE as libc::c_int;
        return;
    }
    if 0 != (*pm).cmd.buttons & 4i32 {
        if 0 == (*(*pm).ps).pm_flags & 1024i32 {
            if !((*bg_itemlist
                .as_mut_ptr()
                .offset((*(*pm).ps).stats[STAT_HOLDABLE_ITEM as libc::c_int as usize] as isize))
            .giTag
                == HI_MEDKIT as libc::c_int
                && (*(*pm).ps).stats[STAT_HEALTH as libc::c_int as usize]
                    >= (*(*pm).ps).stats[STAT_MAX_HEALTH as libc::c_int as usize] + 25i32)
            {
                (*(*pm).ps).pm_flags |= 1024i32;
                PM_AddEvent(
                    EV_USE_ITEM0 as libc::c_int
                        + (*bg_itemlist.as_mut_ptr().offset(
                            (*(*pm).ps).stats[STAT_HOLDABLE_ITEM as libc::c_int as usize] as isize,
                        ))
                        .giTag,
                );
                (*(*pm).ps).stats[STAT_HOLDABLE_ITEM as libc::c_int as usize] = 0i32
            }
            return;
        }
    } else {
        (*(*pm).ps).pm_flags &= !1024i32
    }
    if (*(*pm).ps).weaponTime > 0i32 {
        (*(*pm).ps).weaponTime -= pml.msec
    }
    if (*(*pm).ps).weaponTime <= 0i32 || (*(*pm).ps).weaponstate != WEAPON_FIRING as libc::c_int {
        if (*(*pm).ps).weapon != (*pm).cmd.weapon as libc::c_int {
            PM_BeginWeaponChange((*pm).cmd.weapon as libc::c_int);
        }
    }
    if (*(*pm).ps).weaponTime > 0i32 {
        return;
    }
    if (*(*pm).ps).weaponstate == WEAPON_DROPPING as libc::c_int {
        PM_FinishWeaponChange();
        return;
    }
    if (*(*pm).ps).weaponstate == WEAPON_RAISING as libc::c_int {
        (*(*pm).ps).weaponstate = WEAPON_READY as libc::c_int;
        if (*(*pm).ps).weapon == WP_GAUNTLET as libc::c_int {
            PM_StartTorsoAnim(TORSO_STAND2 as libc::c_int);
        } else {
            PM_StartTorsoAnim(TORSO_STAND as libc::c_int);
        }
        return;
    }
    if 0 == (*pm).cmd.buttons & 1i32 {
        (*(*pm).ps).weaponTime = 0i32;
        (*(*pm).ps).weaponstate = WEAPON_READY as libc::c_int;
        return;
    }
    if (*(*pm).ps).weapon == WP_GAUNTLET as libc::c_int {
        if 0 == (*pm).gauntletHit as u64 {
            (*(*pm).ps).weaponTime = 0i32;
            (*(*pm).ps).weaponstate = WEAPON_READY as libc::c_int;
            return;
        }
        PM_StartTorsoAnim(TORSO_ATTACK2 as libc::c_int);
    } else {
        PM_StartTorsoAnim(TORSO_ATTACK as libc::c_int);
    }
    (*(*pm).ps).weaponstate = WEAPON_FIRING as libc::c_int;
    if 0 == (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] {
        PM_AddEvent(EV_NOAMMO as libc::c_int);
        (*(*pm).ps).weaponTime += 500i32;
        return;
    }
    if (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] != -1i32 {
        (*(*pm).ps).ammo[(*(*pm).ps).weapon as usize] -= 1
    }
    PM_AddEvent(EV_FIRE_WEAPON as libc::c_int);
    match (*(*pm).ps).weapon {
        6 => addTime = 50i32,
        3 => addTime = 1000i32,
        2 => addTime = 100i32,
        4 => addTime = 800i32,
        5 => addTime = 800i32,
        8 => addTime = 100i32,
        7 => addTime = 1500i32,
        9 => addTime = 200i32,
        10 => addTime = 400i32,
        1 | _ => addTime = 400i32,
    }
    if 0 != (*(*pm).ps).powerups[PW_HASTE as libc::c_int as usize] {
        addTime = (addTime as libc::c_double / 1.3f64) as libc::c_int
    }
    (*(*pm).ps).weaponTime += addTime;
}
/*
===============
PM_FinishWeaponChange
===============
*/
unsafe extern "C" fn PM_FinishWeaponChange() {
    let mut weapon: libc::c_int = 0;
    weapon = (*pm).cmd.weapon as libc::c_int;
    if weapon < WP_NONE as libc::c_int || weapon >= WP_NUM_WEAPONS as libc::c_int {
        weapon = WP_NONE as libc::c_int
    }
    if 0 == (*(*pm).ps).stats[STAT_WEAPONS as libc::c_int as usize] & 1i32 << weapon {
        weapon = WP_NONE as libc::c_int
    }
    (*(*pm).ps).weapon = weapon;
    (*(*pm).ps).weaponstate = WEAPON_RAISING as libc::c_int;
    (*(*pm).ps).weaponTime += 250i32;
    PM_StartTorsoAnim(TORSO_RAISE as libc::c_int);
}
/*
===============
PM_BeginWeaponChange
===============
*/
unsafe extern "C" fn PM_BeginWeaponChange(mut weapon: libc::c_int) {
    if weapon <= WP_NONE as libc::c_int || weapon >= WP_NUM_WEAPONS as libc::c_int {
        return;
    }
    if 0 == (*(*pm).ps).stats[STAT_WEAPONS as libc::c_int as usize] & 1i32 << weapon {
        return;
    }
    if (*(*pm).ps).weaponstate == WEAPON_DROPPING as libc::c_int {
        return;
    }
    PM_AddEvent(EV_CHANGE_WEAPON as libc::c_int);
    (*(*pm).ps).weaponstate = WEAPON_DROPPING as libc::c_int;
    (*(*pm).ps).weaponTime += 200i32;
    PM_StartTorsoAnim(TORSO_DROP as libc::c_int);
}
/*
=============
PM_SetWaterLevel	FIXME: avoid this twice?  certainly if not moving
=============
*/
unsafe extern "C" fn PM_SetWaterLevel() {
    let mut point: vec3_t = [0.; 3];
    let mut cont: libc::c_int = 0;
    let mut sample1: libc::c_int = 0;
    let mut sample2: libc::c_int = 0;
    (*pm).waterlevel = 0i32;
    (*pm).watertype = 0i32;
    point[0usize] = (*(*pm).ps).origin[0usize];
    point[1usize] = (*(*pm).ps).origin[1usize];
    point[2usize] = (*(*pm).ps).origin[2usize] + -24i32 as libc::c_float + 1i32 as libc::c_float;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        point.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
    );
    if 0 != cont & (32i32 | 8i32 | 16i32) {
        sample2 = (*(*pm).ps).viewheight - -24i32;
        sample1 = sample2 / 2i32;
        (*pm).watertype = cont;
        (*pm).waterlevel = 1i32;
        point[2usize] =
            (*(*pm).ps).origin[2usize] + -24i32 as libc::c_float + sample1 as libc::c_float;
        cont = (*pm).pointcontents.expect("non-null function pointer")(
            point.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).clientNum,
        );
        if 0 != cont & (32i32 | 8i32 | 16i32) {
            (*pm).waterlevel = 2i32;
            point[2usize] =
                (*(*pm).ps).origin[2usize] + -24i32 as libc::c_float + sample2 as libc::c_float;
            cont = (*pm).pointcontents.expect("non-null function pointer")(
                point.as_mut_ptr() as *const vec_t,
                (*(*pm).ps).clientNum,
            );
            if 0 != cont & (32i32 | 8i32 | 16i32) {
                (*pm).waterlevel = 3i32
            }
        }
    };
}
/*
=============
PM_GroundTrace
=============
*/
unsafe extern "C" fn PM_GroundTrace() {
    let mut point: vec3_t = [0.; 3];
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
    point[0usize] = (*(*pm).ps).origin[0usize];
    point[1usize] = (*(*pm).ps).origin[1usize];
    point[2usize] = ((*(*pm).ps).origin[2usize] as libc::c_double - 0.25f64) as vec_t;
    (*pm).trace.expect("non-null function pointer")(
        &mut trace,
        (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
        (*pm).mins.as_mut_ptr() as *const vec_t,
        (*pm).maxs.as_mut_ptr() as *const vec_t,
        point.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
        (*pm).tracemask,
    );
    pml.groundTrace = trace;
    if 0 != trace.allsolid as u64 {
        if 0 == PM_CorrectAllSolid(&mut trace) {
            return;
        }
    }
    if trace.fraction as libc::c_double == 1.0f64 {
        PM_GroundTraceMissed();
        pml.groundPlane = qfalse;
        pml.walking = qfalse;
        return;
    }
    if (*(*pm).ps).velocity[2usize] > 0i32 as libc::c_float
        && (*(*pm).ps).velocity[0usize] * trace.plane.normal[0usize]
            + (*(*pm).ps).velocity[1usize] * trace.plane.normal[1usize]
            + (*(*pm).ps).velocity[2usize] * trace.plane.normal[2usize]
            > 10i32 as libc::c_float
    {
        if 0 != (*pm).debugLevel {
            Com_Printf(
                b"%i:kickoff\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        if (*pm).cmd.forwardmove as libc::c_int >= 0i32 {
            PM_ForceLegsAnim(LEGS_JUMP as libc::c_int);
            (*(*pm).ps).pm_flags &= !8i32
        } else {
            PM_ForceLegsAnim(LEGS_JUMPB as libc::c_int);
            (*(*pm).ps).pm_flags |= 8i32
        }
        (*(*pm).ps).groundEntityNum = (1i32 << 10i32) - 1i32;
        pml.groundPlane = qfalse;
        pml.walking = qfalse;
        return;
    }
    if trace.plane.normal[2usize] < 0.7f32 {
        if 0 != (*pm).debugLevel {
            Com_Printf(
                b"%i:steep\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        (*(*pm).ps).groundEntityNum = (1i32 << 10i32) - 1i32;
        pml.groundPlane = qtrue;
        pml.walking = qfalse;
        return;
    }
    pml.groundPlane = qtrue;
    pml.walking = qtrue;
    if 0 != (*(*pm).ps).pm_flags & 256i32 {
        (*(*pm).ps).pm_flags &= !(256i32 | 32i32);
        (*(*pm).ps).pm_time = 0i32
    }
    if (*(*pm).ps).groundEntityNum == (1i32 << 10i32) - 1i32 {
        if 0 != (*pm).debugLevel {
            Com_Printf(
                b"%i:Land\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        PM_CrashLand();
        if pml.previous_velocity[2usize] < -200i32 as libc::c_float {
            (*(*pm).ps).pm_flags |= 32i32;
            (*(*pm).ps).pm_time = 250i32
        }
    }
    (*(*pm).ps).groundEntityNum = trace.entityNum;
    PM_AddTouchEnt(trace.entityNum);
}
#[no_mangle]
pub unsafe extern "C" fn PM_AddTouchEnt(mut entityNum: libc::c_int) {
    let mut i: libc::c_int = 0;
    if entityNum == (1i32 << 10i32) - 2i32 {
        return;
    }
    if (*pm).numtouch == 32i32 {
        return;
    }
    i = 0i32;
    while i < (*pm).numtouch {
        if (*pm).touchents[i as usize] == entityNum {
            return;
        }
        i += 1
    }
    (*pm).touchents[(*pm).numtouch as usize] = entityNum;
    (*pm).numtouch += 1;
}
/*
=================
PM_CrashLand

Check for hard landings that generate sound events
=================
*/
unsafe extern "C" fn PM_CrashLand() {
    let mut delta: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut vel: libc::c_float = 0.;
    let mut acc: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut a: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut den: libc::c_float = 0.;
    if 0 != (*(*pm).ps).pm_flags & 8i32 {
        PM_ForceLegsAnim(LEGS_LANDB as libc::c_int);
    } else {
        PM_ForceLegsAnim(LEGS_LAND as libc::c_int);
    }
    (*(*pm).ps).legsTimer = 130i32;
    dist = (*(*pm).ps).origin[2usize] - pml.previous_origin[2usize];
    vel = pml.previous_velocity[2usize];
    acc = -(*(*pm).ps).gravity as libc::c_float;
    a = acc / 2i32 as libc::c_float;
    b = vel;
    c = -dist;
    den = b * b - 4i32 as libc::c_float * a * c;
    if den < 0i32 as libc::c_float {
        return;
    }
    t = ((-b as libc::c_double - sqrt(den as libc::c_double))
        / (2i32 as libc::c_float * a) as libc::c_double) as libc::c_float;
    delta = vel + t * acc;
    delta = ((delta * delta) as libc::c_double * 0.0001f64) as libc::c_float;
    if 0 != (*(*pm).ps).pm_flags & 1i32 {
        delta *= 2i32 as libc::c_float
    }
    if (*pm).waterlevel == 3i32 {
        return;
    }
    if (*pm).waterlevel == 2i32 {
        delta = (delta as libc::c_double * 0.25f64) as libc::c_float
    }
    if (*pm).waterlevel == 1i32 {
        delta = (delta as libc::c_double * 0.5f64) as libc::c_float
    }
    if delta < 1i32 as libc::c_float {
        return;
    }
    if 0 == pml.groundTrace.surfaceFlags & 0x1i32 {
        if delta > 60i32 as libc::c_float {
            PM_AddEvent(EV_FALL_FAR as libc::c_int);
        } else if delta > 40i32 as libc::c_float {
            if (*(*pm).ps).stats[STAT_HEALTH as libc::c_int as usize] > 0i32 {
                PM_AddEvent(EV_FALL_MEDIUM as libc::c_int);
            }
        } else if delta > 7i32 as libc::c_float {
            PM_AddEvent(EV_FALL_SHORT as libc::c_int);
        } else {
            PM_AddEvent(PM_FootstepForSurface());
        }
    }
    (*(*pm).ps).bobCycle = 0i32;
}
unsafe extern "C" fn PM_ForceLegsAnim(mut anim: libc::c_int) {
    (*(*pm).ps).legsTimer = 0i32;
    PM_StartLegsAnim(anim);
}
#[no_mangle]
pub static mut c_pmove: libc::c_int = 0i32;
/*
=============
PM_GroundTraceMissed

The ground trace didn't hit a surface, so we are in freefall
=============
*/
unsafe extern "C" fn PM_GroundTraceMissed() {
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
    let mut point: vec3_t = [0.; 3];
    if (*(*pm).ps).groundEntityNum != (1i32 << 10i32) - 1i32 {
        if 0 != (*pm).debugLevel {
            Com_Printf(
                b"%i:lift\n\x00" as *const u8 as *const libc::c_char,
                c_pmove,
            );
        }
        point[0usize] = (*(*pm).ps).origin[0usize];
        point[1usize] = (*(*pm).ps).origin[1usize];
        point[2usize] = (*(*pm).ps).origin[2usize];
        point[2usize] -= 64i32 as libc::c_float;
        (*pm).trace.expect("non-null function pointer")(
            &mut trace,
            (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
            (*pm).mins.as_mut_ptr() as *const vec_t,
            (*pm).maxs.as_mut_ptr() as *const vec_t,
            point.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).clientNum,
            (*pm).tracemask,
        );
        if trace.fraction as libc::c_double == 1.0f64 {
            if (*pm).cmd.forwardmove as libc::c_int >= 0i32 {
                PM_ForceLegsAnim(LEGS_JUMP as libc::c_int);
                (*(*pm).ps).pm_flags &= !8i32
            } else {
                PM_ForceLegsAnim(LEGS_JUMPB as libc::c_int);
                (*(*pm).ps).pm_flags |= 8i32
            }
        }
    }
    (*(*pm).ps).groundEntityNum = (1i32 << 10i32) - 1i32;
    pml.groundPlane = qfalse;
    pml.walking = qfalse;
}
/*
=============
PM_CheckStuck
=============
*/
/*
void PM_CheckStuck(void) {
    trace_t trace;

    pm->trace (&trace, pm->ps->origin, pm->mins, pm->maxs, pm->ps->origin, pm->ps->clientNum, pm->tracemask);
    if (trace.allsolid) {
        //int shit = qtrue;
    }
}
*/
/*
=============
PM_CorrectAllSolid
=============
*/
unsafe extern "C" fn PM_CorrectAllSolid(mut trace: *mut trace_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut point: vec3_t = [0.; 3];
    if 0 != (*pm).debugLevel {
        Com_Printf(
            b"%i:allsolid\n\x00" as *const u8 as *const libc::c_char,
            c_pmove,
        );
    }
    i = -1i32;
    while i <= 1i32 {
        j = -1i32;
        while j <= 1i32 {
            k = -1i32;
            while k <= 1i32 {
                point[0usize] = (*(*pm).ps).origin[0usize];
                point[1usize] = (*(*pm).ps).origin[1usize];
                point[2usize] = (*(*pm).ps).origin[2usize];
                point[0usize] += i as libc::c_float;
                point[1usize] += j as libc::c_float;
                point[2usize] += k as libc::c_float;
                (*pm).trace.expect("non-null function pointer")(
                    trace,
                    point.as_mut_ptr() as *const vec_t,
                    (*pm).mins.as_mut_ptr() as *const vec_t,
                    (*pm).maxs.as_mut_ptr() as *const vec_t,
                    point.as_mut_ptr() as *const vec_t,
                    (*(*pm).ps).clientNum,
                    (*pm).tracemask,
                );
                if 0 == (*trace).allsolid as u64 {
                    point[0usize] = (*(*pm).ps).origin[0usize];
                    point[1usize] = (*(*pm).ps).origin[1usize];
                    point[2usize] =
                        ((*(*pm).ps).origin[2usize] as libc::c_double - 0.25f64) as vec_t;
                    (*pm).trace.expect("non-null function pointer")(
                        trace,
                        (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
                        (*pm).mins.as_mut_ptr() as *const vec_t,
                        (*pm).maxs.as_mut_ptr() as *const vec_t,
                        point.as_mut_ptr() as *const vec_t,
                        (*(*pm).ps).clientNum,
                        (*pm).tracemask,
                    );
                    pml.groundTrace = *trace;
                    return qtrue as libc::c_int;
                }
                k += 1
            }
            j += 1
        }
        i += 1
    }
    (*(*pm).ps).groundEntityNum = (1i32 << 10i32) - 1i32;
    pml.groundPlane = qfalse;
    pml.walking = qfalse;
    return qfalse as libc::c_int;
}
/*
================
PM_Animate
================
*/
unsafe extern "C" fn PM_Animate() {
    if 0 != (*pm).cmd.buttons & 8i32 {
        if (*(*pm).ps).torsoTimer == 0i32 {
            PM_StartTorsoAnim(TORSO_GESTURE as libc::c_int);
            (*(*pm).ps).torsoTimer = 34i32 * 66i32 + 50i32;
            PM_AddEvent(EV_TAUNT as libc::c_int);
        }
    };
}
/*
===================
PM_AirMove

===================
*/
unsafe extern "C" fn PM_AirMove() {
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: libc::c_float = 0.;
    let mut smove: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut cmd: usercmd_t = usercmd_s {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    };
    PM_Friction();
    fmove = (*pm).cmd.forwardmove as libc::c_float;
    smove = (*pm).cmd.rightmove as libc::c_float;
    cmd = (*pm).cmd;
    scale = PM_CmdScale(&mut cmd);
    PM_SetMovementDir();
    pml.forward[2usize] = 0i32 as vec_t;
    pml.right[2usize] = 0i32 as vec_t;
    VectorNormalize(pml.forward.as_mut_ptr());
    VectorNormalize(pml.right.as_mut_ptr());
    i = 0i32;
    while i < 2i32 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1
    }
    wishvel[2usize] = 0i32 as vec_t;
    wishdir[0usize] = wishvel[0usize];
    wishdir[1usize] = wishvel[1usize];
    wishdir[2usize] = wishvel[2usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_airaccelerate);
    if 0 != pml.groundPlane as u64 {
        PM_ClipVelocity(
            (*(*pm).ps).velocity.as_mut_ptr(),
            pml.groundTrace.plane.normal.as_mut_ptr(),
            (*(*pm).ps).velocity.as_mut_ptr(),
            1.001f32,
        );
    }
    PM_StepSlideMove(qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn PM_ClipVelocity(
    mut in_0: *mut vec_t,
    mut normal: *mut vec_t,
    mut out: *mut vec_t,
    mut overbounce: libc::c_float,
) {
    let mut backoff: libc::c_float = 0.;
    let mut change: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    backoff = *in_0.offset(0isize) * *normal.offset(0isize)
        + *in_0.offset(1isize) * *normal.offset(1isize)
        + *in_0.offset(2isize) * *normal.offset(2isize);
    if backoff < 0i32 as libc::c_float {
        backoff *= overbounce
    } else {
        backoff /= overbounce
    }
    i = 0i32;
    while i < 3i32 {
        change = *normal.offset(i as isize) * backoff;
        *out.offset(i as isize) = *in_0.offset(i as isize) - change;
        i += 1
    }
}
#[no_mangle]
pub static mut pm_airaccelerate: libc::c_float = 1.0f32;
/*
==============
PM_Accelerate

Handles user intended acceleration
==============
*/
unsafe extern "C" fn PM_Accelerate(
    mut wishdir: *mut vec_t,
    mut wishspeed: libc::c_float,
    mut accel: libc::c_float,
) {
    // q2 style
    let mut i: libc::c_int = 0;
    let mut addspeed: libc::c_float = 0.;
    let mut accelspeed: libc::c_float = 0.;
    let mut currentspeed: libc::c_float = 0.;
    currentspeed = (*(*pm).ps).velocity[0usize] * *wishdir.offset(0isize)
        + (*(*pm).ps).velocity[1usize] * *wishdir.offset(1isize)
        + (*(*pm).ps).velocity[2usize] * *wishdir.offset(2isize);
    addspeed = wishspeed - currentspeed;
    if addspeed <= 0i32 as libc::c_float {
        return;
    }
    accelspeed = accel * pml.frametime * wishspeed;
    if accelspeed > addspeed {
        accelspeed = addspeed
    }
    i = 0i32;
    while i < 3i32 {
        (*(*pm).ps).velocity[i as usize] += accelspeed * *wishdir.offset(i as isize);
        i += 1
    }
}
/*
================
PM_SetMovementDir

Determine the rotation of the legs relative
to the facing dir
================
*/
unsafe extern "C" fn PM_SetMovementDir() {
    if 0 != (*pm).cmd.forwardmove as libc::c_int || 0 != (*pm).cmd.rightmove as libc::c_int {
        if (*pm).cmd.rightmove as libc::c_int == 0i32 && (*pm).cmd.forwardmove as libc::c_int > 0i32
        {
            (*(*pm).ps).movementDir = 0i32
        } else if ((*pm).cmd.rightmove as libc::c_int) < 0i32
            && (*pm).cmd.forwardmove as libc::c_int > 0i32
        {
            (*(*pm).ps).movementDir = 1i32
        } else if ((*pm).cmd.rightmove as libc::c_int) < 0i32
            && (*pm).cmd.forwardmove as libc::c_int == 0i32
        {
            (*(*pm).ps).movementDir = 2i32
        } else if ((*pm).cmd.rightmove as libc::c_int) < 0i32
            && ((*pm).cmd.forwardmove as libc::c_int) < 0i32
        {
            (*(*pm).ps).movementDir = 3i32
        } else if (*pm).cmd.rightmove as libc::c_int == 0i32
            && ((*pm).cmd.forwardmove as libc::c_int) < 0i32
        {
            (*(*pm).ps).movementDir = 4i32
        } else if (*pm).cmd.rightmove as libc::c_int > 0i32
            && ((*pm).cmd.forwardmove as libc::c_int) < 0i32
        {
            (*(*pm).ps).movementDir = 5i32
        } else if (*pm).cmd.rightmove as libc::c_int > 0i32
            && (*pm).cmd.forwardmove as libc::c_int == 0i32
        {
            (*(*pm).ps).movementDir = 6i32
        } else if (*pm).cmd.rightmove as libc::c_int > 0i32
            && (*pm).cmd.forwardmove as libc::c_int > 0i32
        {
            (*(*pm).ps).movementDir = 7i32
        }
    } else if (*(*pm).ps).movementDir == 2i32 {
        (*(*pm).ps).movementDir = 1i32
    } else if (*(*pm).ps).movementDir == 6i32 {
        (*(*pm).ps).movementDir = 7i32
    };
}
/*
============
PM_CmdScale

Returns the scale factor to apply to cmd movements
This allows the clients to use axial -127 to 127 values for all directions
without getting a sqrt(2) distortion in speed.
============
*/
unsafe extern "C" fn PM_CmdScale(mut cmd: *mut usercmd_t) -> libc::c_float {
    let mut max: libc::c_int = 0;
    let mut total: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    max = abs((*cmd).forwardmove as libc::c_int);
    if abs((*cmd).rightmove as libc::c_int) > max {
        max = abs((*cmd).rightmove as libc::c_int)
    }
    if abs((*cmd).upmove as libc::c_int) > max {
        max = abs((*cmd).upmove as libc::c_int)
    }
    if 0 == max {
        return 0i32 as libc::c_float;
    }
    total = sqrt(
        ((*cmd).forwardmove as libc::c_int * (*cmd).forwardmove as libc::c_int
            + (*cmd).rightmove as libc::c_int * (*cmd).rightmove as libc::c_int
            + (*cmd).upmove as libc::c_int * (*cmd).upmove as libc::c_int)
            as libc::c_double,
    ) as libc::c_float;
    scale = (((*(*pm).ps).speed as libc::c_float * max as libc::c_float) as libc::c_double
        / (127.0f64 * total as libc::c_double)) as libc::c_float;
    return scale;
}
/*
==================
PM_Friction

Handles both ground friction and water friction
==================
*/
unsafe extern "C" fn PM_Friction() {
    let mut vec: vec3_t = [0.; 3];
    let mut vel: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut speed: libc::c_float = 0.;
    let mut newspeed: libc::c_float = 0.;
    let mut control: libc::c_float = 0.;
    let mut drop_0: libc::c_float = 0.;
    vel = (*(*pm).ps).velocity.as_mut_ptr();
    vec[0usize] = *vel.offset(0isize);
    vec[1usize] = *vel.offset(1isize);
    vec[2usize] = *vel.offset(2isize);
    if 0 != pml.walking as u64 {
        vec[2usize] = 0i32 as vec_t
    }
    speed = VectorLength(vec.as_mut_ptr() as *const vec_t);
    if speed < 1i32 as libc::c_float {
        *vel.offset(0isize) = 0i32 as libc::c_float;
        *vel.offset(1isize) = 0i32 as libc::c_float;
        return;
    }
    drop_0 = 0i32 as libc::c_float;
    if (*pm).waterlevel <= 1i32 {
        if 0 != pml.walking as libc::c_uint && 0 == pml.groundTrace.surfaceFlags & 0x2i32 {
            if 0 == (*(*pm).ps).pm_flags & 64i32 {
                control = if speed < pm_stopspeed {
                    pm_stopspeed
                } else {
                    speed
                };
                drop_0 += control * pm_friction * pml.frametime
            }
        }
    }
    if 0 != (*pm).waterlevel {
        drop_0 += speed * pm_waterfriction * (*pm).waterlevel as libc::c_float * pml.frametime
    }
    if 0 != (*(*pm).ps).powerups[PW_FLIGHT as libc::c_int as usize] {
        drop_0 += speed * pm_flightfriction * pml.frametime
    }
    if (*(*pm).ps).pm_type == PM_SPECTATOR as libc::c_int {
        drop_0 += speed * pm_spectatorfriction * pml.frametime
    }
    newspeed = speed - drop_0;
    if newspeed < 0i32 as libc::c_float {
        newspeed = 0i32 as libc::c_float
    }
    newspeed /= speed;
    *vel.offset(0isize) = *vel.offset(0isize) * newspeed;
    *vel.offset(1isize) = *vel.offset(1isize) * newspeed;
    *vel.offset(2isize) = *vel.offset(2isize) * newspeed;
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
// bg_pmove.c -- both games player movement code
// takes a playerstate and a usercmd as input and returns a modifed playerstate
// movement parameters
#[no_mangle]
pub static mut pm_spectatorfriction: libc::c_float = 5.0f32;
#[no_mangle]
pub static mut pm_flightfriction: libc::c_float = 3.0f32;
#[no_mangle]
pub static mut pm_waterfriction: libc::c_float = 1.0f32;
#[no_mangle]
pub static mut pm_friction: libc::c_float = 6.0f32;
// movement parameters
#[no_mangle]
pub static mut pm_stopspeed: libc::c_float = 100.0f32;
/*
===================
PM_WalkMove

===================
*/
unsafe extern "C" fn PM_WalkMove() {
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: libc::c_float = 0.;
    let mut smove: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    let mut cmd: usercmd_t = usercmd_s {
        serverTime: 0,
        angles: [0; 3],
        buttons: 0,
        weapon: 0,
        forwardmove: 0,
        rightmove: 0,
        upmove: 0,
    };
    let mut accelerate: libc::c_float = 0.;
    let mut vel: libc::c_float = 0.;
    if (*pm).waterlevel > 2i32
        && pml.forward[0usize] * pml.groundTrace.plane.normal[0usize]
            + pml.forward[1usize] * pml.groundTrace.plane.normal[1usize]
            + pml.forward[2usize] * pml.groundTrace.plane.normal[2usize]
            > 0i32 as libc::c_float
    {
        PM_WaterMove();
        return;
    }
    if 0 != PM_CheckJump() as u64 {
        if (*pm).waterlevel > 1i32 {
            PM_WaterMove();
        } else {
            PM_AirMove();
        }
        return;
    }
    PM_Friction();
    fmove = (*pm).cmd.forwardmove as libc::c_float;
    smove = (*pm).cmd.rightmove as libc::c_float;
    cmd = (*pm).cmd;
    scale = PM_CmdScale(&mut cmd);
    PM_SetMovementDir();
    pml.forward[2usize] = 0i32 as vec_t;
    pml.right[2usize] = 0i32 as vec_t;
    PM_ClipVelocity(
        pml.forward.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        pml.forward.as_mut_ptr(),
        1.001f32,
    );
    PM_ClipVelocity(
        pml.right.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        pml.right.as_mut_ptr(),
        1.001f32,
    );
    VectorNormalize(pml.forward.as_mut_ptr());
    VectorNormalize(pml.right.as_mut_ptr());
    i = 0i32;
    while i < 3i32 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1
    }
    wishdir[0usize] = wishvel[0usize];
    wishdir[1usize] = wishvel[1usize];
    wishdir[2usize] = wishvel[2usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    if 0 != (*(*pm).ps).pm_flags & 1i32 {
        if wishspeed > (*(*pm).ps).speed as libc::c_float * pm_duckScale {
            wishspeed = (*(*pm).ps).speed as libc::c_float * pm_duckScale
        }
    }
    if 0 != (*pm).waterlevel {
        let mut waterScale: libc::c_float = 0.;
        waterScale = ((*pm).waterlevel as libc::c_double / 3.0f64) as libc::c_float;
        waterScale = (1.0f64
            - (1.0f64 - pm_swimScale as libc::c_double) * waterScale as libc::c_double)
            as libc::c_float;
        if wishspeed > (*(*pm).ps).speed as libc::c_float * waterScale {
            wishspeed = (*(*pm).ps).speed as libc::c_float * waterScale
        }
    }
    if 0 != pml.groundTrace.surfaceFlags & 0x2i32 || 0 != (*(*pm).ps).pm_flags & 64i32 {
        accelerate = pm_airaccelerate
    } else {
        accelerate = pm_accelerate
    }
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, accelerate);
    if 0 != pml.groundTrace.surfaceFlags & 0x2i32 || 0 != (*(*pm).ps).pm_flags & 64i32 {
        (*(*pm).ps).velocity[2usize] -= (*(*pm).ps).gravity as libc::c_float * pml.frametime
    }
    vel = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t);
    PM_ClipVelocity(
        (*(*pm).ps).velocity.as_mut_ptr(),
        pml.groundTrace.plane.normal.as_mut_ptr(),
        (*(*pm).ps).velocity.as_mut_ptr(),
        1.001f32,
    );
    VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
    (*(*pm).ps).velocity[0usize] = (*(*pm).ps).velocity[0usize] * vel;
    (*(*pm).ps).velocity[1usize] = (*(*pm).ps).velocity[1usize] * vel;
    (*(*pm).ps).velocity[2usize] = (*(*pm).ps).velocity[2usize] * vel;
    if 0. == (*(*pm).ps).velocity[0usize] && 0. == (*(*pm).ps).velocity[1usize] {
        return;
    }
    PM_StepSlideMove(qfalse);
}
#[no_mangle]
pub static mut pm_accelerate: libc::c_float = 10.0f32;
#[no_mangle]
pub static mut pm_swimScale: libc::c_float = 0.50f32;
#[no_mangle]
pub static mut pm_duckScale: libc::c_float = 0.25f32;
/*
===================
PM_WaterMove

===================
*/
unsafe extern "C" fn PM_WaterMove() {
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut scale: libc::c_float = 0.;
    let mut vel: libc::c_float = 0.;
    if 0 != PM_CheckWaterJump() as u64 {
        PM_WaterJumpMove();
        return;
    }
    PM_Friction();
    scale = PM_CmdScale(&mut (*pm).cmd);
    if 0. == scale {
        wishvel[0usize] = 0i32 as vec_t;
        wishvel[1usize] = 0i32 as vec_t;
        wishvel[2usize] = -60i32 as vec_t
    } else {
        i = 0i32;
        while i < 3i32 {
            wishvel[i as usize] = scale
                * pml.forward[i as usize]
                * (*pm).cmd.forwardmove as libc::c_int as libc::c_float
                + scale
                    * pml.right[i as usize]
                    * (*pm).cmd.rightmove as libc::c_int as libc::c_float;
            i += 1
        }
        wishvel[2usize] += scale * (*pm).cmd.upmove as libc::c_int as libc::c_float
    }
    wishdir[0usize] = wishvel[0usize];
    wishdir[1usize] = wishvel[1usize];
    wishdir[2usize] = wishvel[2usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    if wishspeed > (*(*pm).ps).speed as libc::c_float * pm_swimScale {
        wishspeed = (*(*pm).ps).speed as libc::c_float * pm_swimScale
    }
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_wateraccelerate);
    if 0 != pml.groundPlane as libc::c_uint
        && (*(*pm).ps).velocity[0usize] * pml.groundTrace.plane.normal[0usize]
            + (*(*pm).ps).velocity[1usize] * pml.groundTrace.plane.normal[1usize]
            + (*(*pm).ps).velocity[2usize] * pml.groundTrace.plane.normal[2usize]
            < 0i32 as libc::c_float
    {
        vel = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t);
        PM_ClipVelocity(
            (*(*pm).ps).velocity.as_mut_ptr(),
            pml.groundTrace.plane.normal.as_mut_ptr(),
            (*(*pm).ps).velocity.as_mut_ptr(),
            1.001f32,
        );
        VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
        (*(*pm).ps).velocity[0usize] = (*(*pm).ps).velocity[0usize] * vel;
        (*(*pm).ps).velocity[1usize] = (*(*pm).ps).velocity[1usize] * vel;
        (*(*pm).ps).velocity[2usize] = (*(*pm).ps).velocity[2usize] * vel
    }
    PM_SlideMove(qfalse);
}
#[no_mangle]
pub static mut pm_wateraccelerate: libc::c_float = 4.0f32;
//============================================================================
/*
===================
PM_WaterJumpMove

Flying out of the water
===================
*/
unsafe extern "C" fn PM_WaterJumpMove() {
    PM_StepSlideMove(qtrue);
    (*(*pm).ps).velocity[2usize] -= (*(*pm).ps).gravity as libc::c_float * pml.frametime;
    if (*(*pm).ps).velocity[2usize] < 0i32 as libc::c_float {
        (*(*pm).ps).pm_flags &= !(256i32 | 32i32 | 64i32);
        (*(*pm).ps).pm_time = 0i32
    };
}
/*
=============
PM_CheckWaterJump
=============
*/
unsafe extern "C" fn PM_CheckWaterJump() -> qboolean {
    let mut spot: vec3_t = [0.; 3];
    let mut cont: libc::c_int = 0;
    let mut flatforward: vec3_t = [0.; 3];
    if 0 != (*(*pm).ps).pm_time {
        return qfalse;
    }
    if (*pm).waterlevel != 2i32 {
        return qfalse;
    }
    flatforward[0usize] = pml.forward[0usize];
    flatforward[1usize] = pml.forward[1usize];
    flatforward[2usize] = 0i32 as vec_t;
    VectorNormalize(flatforward.as_mut_ptr());
    spot[0usize] = (*(*pm).ps).origin[0usize] + flatforward[0usize] * 30i32 as libc::c_float;
    spot[1usize] = (*(*pm).ps).origin[1usize] + flatforward[1usize] * 30i32 as libc::c_float;
    spot[2usize] = (*(*pm).ps).origin[2usize] + flatforward[2usize] * 30i32 as libc::c_float;
    spot[2usize] += 4i32 as libc::c_float;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        spot.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
    );
    if 0 == cont & 1i32 {
        return qfalse;
    }
    spot[2usize] += 16i32 as libc::c_float;
    cont = (*pm).pointcontents.expect("non-null function pointer")(
        spot.as_mut_ptr() as *const vec_t,
        (*(*pm).ps).clientNum,
    );
    if 0 != cont & (1i32 | 0x10000i32 | 0x2000000i32) {
        return qfalse;
    }
    (*(*pm).ps).velocity[0usize] = pml.forward[0usize] * 200i32 as libc::c_float;
    (*(*pm).ps).velocity[1usize] = pml.forward[1usize] * 200i32 as libc::c_float;
    (*(*pm).ps).velocity[2usize] = pml.forward[2usize] * 200i32 as libc::c_float;
    (*(*pm).ps).velocity[2usize] = 350i32 as vec_t;
    (*(*pm).ps).pm_flags |= 256i32;
    (*(*pm).ps).pm_time = 2000i32;
    return qtrue;
}
/*
=============
PM_CheckJump
=============
*/
unsafe extern "C" fn PM_CheckJump() -> qboolean {
    if 0 != (*(*pm).ps).pm_flags & 512i32 {
        return qfalse;
    }
    if ((*pm).cmd.upmove as libc::c_int) < 10i32 {
        return qfalse;
    }
    if 0 != (*(*pm).ps).pm_flags & 2i32 {
        (*pm).cmd.upmove = 0i32 as libc::c_schar;
        return qfalse;
    }
    pml.groundPlane = qfalse;
    pml.walking = qfalse;
    (*(*pm).ps).pm_flags |= 2i32;
    (*(*pm).ps).groundEntityNum = (1i32 << 10i32) - 1i32;
    (*(*pm).ps).velocity[2usize] = 270i32 as vec_t;
    PM_AddEvent(EV_JUMP as libc::c_int);
    if (*pm).cmd.forwardmove as libc::c_int >= 0i32 {
        PM_ForceLegsAnim(LEGS_JUMP as libc::c_int);
        (*(*pm).ps).pm_flags &= !8i32
    } else {
        PM_ForceLegsAnim(LEGS_JUMPB as libc::c_int);
        (*(*pm).ps).pm_flags |= 8i32
    }
    return qtrue;
}
/*
===================
PM_GrappleMove

===================
*/
unsafe extern "C" fn PM_GrappleMove() {
    let mut vel: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut vlen: libc::c_float = 0.;
    v[0usize] = pml.forward[0usize] * -16i32 as libc::c_float;
    v[1usize] = pml.forward[1usize] * -16i32 as libc::c_float;
    v[2usize] = pml.forward[2usize] * -16i32 as libc::c_float;
    v[0usize] = (*(*pm).ps).grapplePoint[0usize] + v[0usize];
    v[1usize] = (*(*pm).ps).grapplePoint[1usize] + v[1usize];
    v[2usize] = (*(*pm).ps).grapplePoint[2usize] + v[2usize];
    vel[0usize] = v[0usize] - (*(*pm).ps).origin[0usize];
    vel[1usize] = v[1usize] - (*(*pm).ps).origin[1usize];
    vel[2usize] = v[2usize] - (*(*pm).ps).origin[2usize];
    vlen = VectorLength(vel.as_mut_ptr() as *const vec_t);
    VectorNormalize(vel.as_mut_ptr());
    if vlen <= 100i32 as libc::c_float {
        vel[0usize] = vel[0usize] * (10i32 as libc::c_float * vlen);
        vel[1usize] = vel[1usize] * (10i32 as libc::c_float * vlen);
        vel[2usize] = vel[2usize] * (10i32 as libc::c_float * vlen)
    } else {
        vel[0usize] = vel[0usize] * 800i32 as libc::c_float;
        vel[1usize] = vel[1usize] * 800i32 as libc::c_float;
        vel[2usize] = vel[2usize] * 800i32 as libc::c_float
    }
    (*(*pm).ps).velocity[0usize] = vel[0usize];
    (*(*pm).ps).velocity[1usize] = vel[1usize];
    (*(*pm).ps).velocity[2usize] = vel[2usize];
    pml.groundPlane = qfalse;
}
/*
===================
PM_FlyMove

Only with the flight powerup
===================
*/
unsafe extern "C" fn PM_FlyMove() {
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut scale: libc::c_float = 0.;
    PM_Friction();
    scale = PM_CmdScale(&mut (*pm).cmd);
    if 0. == scale {
        wishvel[0usize] = 0i32 as vec_t;
        wishvel[1usize] = 0i32 as vec_t;
        wishvel[2usize] = 0i32 as vec_t
    } else {
        i = 0i32;
        while i < 3i32 {
            wishvel[i as usize] = scale
                * pml.forward[i as usize]
                * (*pm).cmd.forwardmove as libc::c_int as libc::c_float
                + scale
                    * pml.right[i as usize]
                    * (*pm).cmd.rightmove as libc::c_int as libc::c_float;
            i += 1
        }
        wishvel[2usize] += scale * (*pm).cmd.upmove as libc::c_int as libc::c_float
    }
    wishdir[0usize] = wishvel[0usize];
    wishdir[1usize] = wishvel[1usize];
    wishdir[2usize] = wishvel[2usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_flyaccelerate);
    PM_StepSlideMove(qfalse);
}
#[no_mangle]
pub static mut pm_flyaccelerate: libc::c_float = 8.0f32;
/*
================
PM_DropTimers
================
*/
unsafe extern "C" fn PM_DropTimers() {
    if 0 != (*(*pm).ps).pm_time {
        if pml.msec >= (*(*pm).ps).pm_time {
            (*(*pm).ps).pm_flags &= !(256i32 | 32i32 | 64i32);
            (*(*pm).ps).pm_time = 0i32
        } else {
            (*(*pm).ps).pm_time -= pml.msec
        }
    }
    if (*(*pm).ps).legsTimer > 0i32 {
        (*(*pm).ps).legsTimer -= pml.msec;
        if (*(*pm).ps).legsTimer < 0i32 {
            (*(*pm).ps).legsTimer = 0i32
        }
    }
    if (*(*pm).ps).torsoTimer > 0i32 {
        (*(*pm).ps).torsoTimer -= pml.msec;
        if (*(*pm).ps).torsoTimer < 0i32 {
            (*(*pm).ps).torsoTimer = 0i32
        }
    };
}
//Com_Printf("velocity2 = %1.1f\n", VectorLength(pm->ps->velocity));
/*
==============
PM_DeadMove
==============
*/
unsafe extern "C" fn PM_DeadMove() {
    let mut forward: libc::c_float = 0.;
    if 0 == pml.walking as u64 {
        return;
    }
    forward = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t);
    forward -= 20i32 as libc::c_float;
    if forward <= 0i32 as libc::c_float {
        (*(*pm).ps).velocity[2usize] = 0i32 as vec_t;
        (*(*pm).ps).velocity[1usize] = (*(*pm).ps).velocity[2usize];
        (*(*pm).ps).velocity[0usize] = (*(*pm).ps).velocity[1usize]
    } else {
        VectorNormalize((*(*pm).ps).velocity.as_mut_ptr());
        (*(*pm).ps).velocity[0usize] = (*(*pm).ps).velocity[0usize] * forward;
        (*(*pm).ps).velocity[1usize] = (*(*pm).ps).velocity[1usize] * forward;
        (*(*pm).ps).velocity[2usize] = (*(*pm).ps).velocity[2usize] * forward
    };
}
/*
==============
PM_CheckDuck

Sets mins, maxs, and pm->ps->viewheight
==============
*/
unsafe extern "C" fn PM_CheckDuck() {
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
    if 0 != (*(*pm).ps).powerups[PW_INVULNERABILITY as libc::c_int as usize] {
        if 0 != (*(*pm).ps).pm_flags & 16384i32 {
            (*pm).mins[0usize] = -42i32 as vec_t;
            (*pm).mins[1usize] = -42i32 as vec_t;
            (*pm).mins[2usize] = -42i32 as vec_t;
            (*pm).maxs[0usize] = 42i32 as vec_t;
            (*pm).maxs[1usize] = 42i32 as vec_t;
            (*pm).maxs[2usize] = 42i32 as vec_t
        } else {
            (*pm).mins[0usize] = -15i32 as vec_t;
            (*pm).mins[1usize] = -15i32 as vec_t;
            (*pm).mins[2usize] = -24i32 as vec_t;
            (*pm).maxs[0usize] = 15i32 as vec_t;
            (*pm).maxs[1usize] = 15i32 as vec_t;
            (*pm).maxs[2usize] = 16i32 as vec_t
        }
        (*(*pm).ps).pm_flags |= 1i32;
        (*(*pm).ps).viewheight = 12i32;
        return;
    }
    (*(*pm).ps).pm_flags &= !16384i32;
    (*pm).mins[0usize] = -15i32 as vec_t;
    (*pm).mins[1usize] = -15i32 as vec_t;
    (*pm).maxs[0usize] = 15i32 as vec_t;
    (*pm).maxs[1usize] = 15i32 as vec_t;
    (*pm).mins[2usize] = -24i32 as vec_t;
    if (*(*pm).ps).pm_type == PM_DEAD as libc::c_int {
        (*pm).maxs[2usize] = -8i32 as vec_t;
        (*(*pm).ps).viewheight = -16i32;
        return;
    }
    if ((*pm).cmd.upmove as libc::c_int) < 0i32 {
        (*(*pm).ps).pm_flags |= 1i32
    } else if 0 != (*(*pm).ps).pm_flags & 1i32 {
        (*pm).maxs[2usize] = 32i32 as vec_t;
        (*pm).trace.expect("non-null function pointer")(
            &mut trace,
            (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
            (*pm).mins.as_mut_ptr() as *const vec_t,
            (*pm).maxs.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).origin.as_mut_ptr() as *const vec_t,
            (*(*pm).ps).clientNum,
            (*pm).tracemask,
        );
        if 0 == trace.allsolid as u64 {
            (*(*pm).ps).pm_flags &= !1i32
        }
    }
    if 0 != (*(*pm).ps).pm_flags & 1i32 {
        (*pm).maxs[2usize] = 16i32 as vec_t;
        (*(*pm).ps).viewheight = 12i32
    } else {
        (*pm).maxs[2usize] = 32i32 as vec_t;
        (*(*pm).ps).viewheight = 26i32
    };
}
/*
===============
PM_NoclipMove
===============
*/
unsafe extern "C" fn PM_NoclipMove() {
    let mut speed: libc::c_float = 0.;
    let mut drop_0: libc::c_float = 0.;
    let mut friction: libc::c_float = 0.;
    let mut control: libc::c_float = 0.;
    let mut newspeed: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: libc::c_float = 0.;
    let mut smove: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut scale: libc::c_float = 0.;
    (*(*pm).ps).viewheight = 26i32;
    speed = VectorLength((*(*pm).ps).velocity.as_mut_ptr() as *const vec_t);
    if speed < 1i32 as libc::c_float {
        (*(*pm).ps).velocity[0usize] = vec3_origin[0usize];
        (*(*pm).ps).velocity[1usize] = vec3_origin[1usize];
        (*(*pm).ps).velocity[2usize] = vec3_origin[2usize]
    } else {
        drop_0 = 0i32 as libc::c_float;
        friction = (pm_friction as libc::c_double * 1.5f64) as libc::c_float;
        control = if speed < pm_stopspeed {
            pm_stopspeed
        } else {
            speed
        };
        drop_0 += control * friction * pml.frametime;
        newspeed = speed - drop_0;
        if newspeed < 0i32 as libc::c_float {
            newspeed = 0i32 as libc::c_float
        }
        newspeed /= speed;
        (*(*pm).ps).velocity[0usize] = (*(*pm).ps).velocity[0usize] * newspeed;
        (*(*pm).ps).velocity[1usize] = (*(*pm).ps).velocity[1usize] * newspeed;
        (*(*pm).ps).velocity[2usize] = (*(*pm).ps).velocity[2usize] * newspeed
    }
    scale = PM_CmdScale(&mut (*pm).cmd);
    fmove = (*pm).cmd.forwardmove as libc::c_float;
    smove = (*pm).cmd.rightmove as libc::c_float;
    i = 0i32;
    while i < 3i32 {
        wishvel[i as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1
    }
    wishvel[2usize] += (*pm).cmd.upmove as libc::c_int as libc::c_float;
    wishdir[0usize] = wishvel[0usize];
    wishdir[1usize] = wishvel[1usize];
    wishdir[2usize] = wishvel[2usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    wishspeed *= scale;
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_accelerate);
    (*(*pm).ps).origin[0usize] =
        (*(*pm).ps).origin[0usize] + (*(*pm).ps).velocity[0usize] * pml.frametime;
    (*(*pm).ps).origin[1usize] =
        (*(*pm).ps).origin[1usize] + (*(*pm).ps).velocity[1usize] * pml.frametime;
    (*(*pm).ps).origin[2usize] =
        (*(*pm).ps).origin[2usize] + (*(*pm).ps).velocity[2usize] * pml.frametime;
}
extern "C" {
    /*
    ================
    PmoveSingle

    ================
    */
    #[no_mangle]
    pub fn trap_SnapVector(v: *mut libc::c_float);
}
