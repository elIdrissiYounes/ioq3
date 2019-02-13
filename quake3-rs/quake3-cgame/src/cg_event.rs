use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, gitem_s, gitem_t, holdable_t, itemType_t,
    team_t, unnamed_0, unnamed_1, unnamed_2, unnamed_3, unnamed_4, unnamed_5, unnamed_6, unnamed_7,
    BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3, BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, ET_BEAM, ET_EVENTS,
    ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL,
    ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH,
    EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE,
    EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP,
    EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP,
    EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP,
    EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED, EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT,
    EV_MISSILE_HIT, EV_MISSILE_MISS, EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE,
    EV_OBELISKPAIN, EV_OBITUARY, EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT,
    EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD, EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK,
    EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL, EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16,
    EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND, EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME,
    EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO, EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0,
    EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11, EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14,
    EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3, EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6,
    EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9, EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH,
    EV_WATER_UNDER, FLAG_RUN, FLAG_STAND, FLAG_STAND2RUN, GENDER_FEMALE, GENDER_MALE,
    GENDER_NEUTER, GTS_BLUEOBELISK_ATTACKED, GTS_BLUETEAM_SCORED, GTS_BLUETEAM_TOOK_LEAD,
    GTS_BLUE_CAPTURE, GTS_BLUE_RETURN, GTS_BLUE_TAKEN, GTS_KAMIKAZE, GTS_REDOBELISK_ATTACKED,
    GTS_REDTEAM_SCORED, GTS_REDTEAM_TOOK_LEAD, GTS_RED_CAPTURE, GTS_RED_RETURN, GTS_RED_TAKEN,
    GTS_TEAMS_ARE_TIED, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, HI_INVULNERABILITY, HI_KAMIKAZE, HI_MEDKIT, HI_NONE,
    HI_NUM_HOLDABLE, HI_PORTAL, HI_TELEPORTER, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK,
    LEGS_IDLE, LEGS_IDLECR, LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM,
    LEGS_TURN, LEGS_WALK, LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS, MOD_BFG,
    MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE, MOD_GRENADE,
    MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA, MOD_PLASMA_SPLASH,
    MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME, MOD_SUICIDE,
    MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, PERS_ASSIST_COUNT,
    PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PW_AMMOREGEN, PW_BATTLESUIT, PW_BLUEFLAG,
    PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY, PW_NEUTRALFLAG,
    PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, TEAM_BLUE, TEAM_FREE,
    TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, TORSO_AFFIRMATIVE, TORSO_ATTACK, TORSO_ATTACK2,
    TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE, TORSO_GETFLAG, TORSO_GUARDBASE, TORSO_NEGATIVE,
    TORSO_PATROL, TORSO_RAISE, TORSO_STAND, TORSO_STAND2, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK,
    WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN,
    WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
use cg_info::{CG_DrawInformation, CG_LoadingClient, CG_LoadingItem, CG_LoadingString};
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, impactSound_t,
    leBounceSoundType_t, leMarkType_t, leType_t, lerpFrame_t, localEntity_s, localEntity_t,
    playerEntity_t, score_t, trap_S_RegisterSound, trap_S_StartSound, trap_S_StopLoopingSound,
    unnamed_8, FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL,
    FOOTSTEP_NORMAL, FOOTSTEP_SPLASH, FOOTSTEP_TOTAL, IMPACTSOUND_DEFAULT, IMPACTSOUND_FLESH,
    IMPACTSOUND_METAL, LEBS_BLOOD, LEBS_BRASS, LEBS_NONE, LEF_PUFF_DONT_SCALE, LEF_SOUND1,
    LEF_SOUND2, LEF_TUMBLE, LEMT_BLOOD, LEMT_BURN, LEMT_NONE, LE_EXPLOSION, LE_FADE_RGB,
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
    byte, cvarHandle_t, entityState_s, entityState_t, gameState_t, playerState_s, playerState_t,
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, unnamed, va, vec3_t,
    vec_t, vmCvar_t, Com_sprintf, Info_ValueForKey, Q_strncpyz, CHAN_ANNOUNCER, CHAN_AUTO,
    CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{rand, strcat, strcpy};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
};

//
// cg_events.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_CheckEvents(mut cent: *mut centity_t) {
    if (*cent).currentState.eType > ET_EVENTS as libc::c_int {
        if 0 != (*cent).previousEvent {
            return;
        }
        if 0 != (*cent).currentState.eFlags & 0x10i32 {
            (*cent).currentState.number = (*cent).currentState.otherEntityNum
        }
        (*cent).previousEvent = 1i32;
        (*cent).currentState.event = (*cent).currentState.eType - ET_EVENTS as libc::c_int
    } else {
        if (*cent).currentState.event == (*cent).previousEvent {
            return;
        }
        (*cent).previousEvent = (*cent).currentState.event;
        if (*cent).currentState.event & !(0x100i32 | 0x200i32) == 0i32 {
            return;
        }
    }
    BG_EvaluateTrajectory(
        &mut (*cent).currentState.pos,
        (*cg.snap).serverTime,
        (*cent).lerpOrigin.as_mut_ptr(),
    );
    CG_SetEntitySoundPosition(cent);
    CG_EntityEvent(cent, (*cent).lerpOrigin.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CG_EntityEvent(mut cent: *mut centity_t, mut position: *mut vec_t) {
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    let mut event: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut clientNum: libc::c_int = 0;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    es = &mut (*cent).currentState;
    event = (*es).event & !(0x100i32 | 0x200i32);
    if 0 != cg_debugEvents.integer {
        CG_Printf(
            b"ent:%3i  event:%3i \x00" as *const u8 as *const libc::c_char,
            (*es).number,
            event,
        );
    }
    if 0 == event {
        if 0 != cg_debugEvents.integer {
            CG_Printf(b"ZEROEVENT\n\x00" as *const u8 as *const libc::c_char);
        }
        return;
    }
    clientNum = (*es).clientNum;
    if clientNum < 0i32 || clientNum >= 64i32 {
        clientNum = 0i32
    }
    ci = &mut cgs.clientinfo[clientNum as usize] as *mut clientInfo_t;
    let mut current_block_479: u64;
    match event {
        1 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FOOTSTEP\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != cg_footsteps.integer {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_BODY as libc::c_int,
                    cgs.media.footsteps[(*ci).footsteps as usize][(rand() & 3i32) as usize],
                );
            }
            current_block_479 = 10325342238833500232;
        }
        2 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FOOTSTEP_METAL\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != cg_footsteps.integer {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_BODY as libc::c_int,
                    cgs.media.footsteps[FOOTSTEP_METAL as libc::c_int as usize]
                        [(rand() & 3i32) as usize],
                );
            }
            current_block_479 = 10325342238833500232;
        }
        3 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FOOTSPLASH\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != cg_footsteps.integer {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_BODY as libc::c_int,
                    cgs.media.footsteps[FOOTSTEP_SPLASH as libc::c_int as usize]
                        [(rand() & 3i32) as usize],
                );
            }
            current_block_479 = 10325342238833500232;
        }
        4 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FOOTWADE\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != cg_footsteps.integer {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_BODY as libc::c_int,
                    cgs.media.footsteps[FOOTSTEP_SPLASH as libc::c_int as usize]
                        [(rand() & 3i32) as usize],
                );
            }
            current_block_479 = 10325342238833500232;
        }
        5 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_SWIM\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != cg_footsteps.integer {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_BODY as libc::c_int,
                    cgs.media.footsteps[FOOTSTEP_SPLASH as libc::c_int as usize]
                        [(rand() & 3i32) as usize],
                );
            }
            current_block_479 = 10325342238833500232;
        }
        10 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FALL_SHORT\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.landSound,
            );
            if clientNum == cg.predictedPlayerState.clientNum {
                cg.landChange = -8i32 as libc::c_float;
                cg.landTime = cg.time
            }
            current_block_479 = 10325342238833500232;
        }
        11 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FALL_MEDIUM\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_VOICE as libc::c_int,
                CG_CustomSound(
                    (*es).number,
                    b"*pain100_1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            if clientNum == cg.predictedPlayerState.clientNum {
                cg.landChange = -16i32 as libc::c_float;
                cg.landTime = cg.time
            }
            current_block_479 = 10325342238833500232;
        }
        12 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FALL_FAR\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                CG_CustomSound(
                    (*es).number,
                    b"*fall1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            (*cent).pe.painTime = cg.time;
            if clientNum == cg.predictedPlayerState.clientNum {
                cg.landChange = -24i32 as libc::c_float;
                cg.landTime = cg.time
            }
            current_block_479 = 10325342238833500232;
        }
        6 | 7 | 8 => {
            // smooth out step up transitions
            current_block_479 = 10326049763699356704;
        }
        9 => {
            current_block_479 = 10326049763699356704;
        }
        13 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_JUMP_PAD\n\x00" as *const u8 as *const libc::c_char);
            }
            let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
            CG_SmokePuff(
                (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
                up.as_mut_ptr() as *const vec_t,
                32i32 as libc::c_float,
                1i32 as libc::c_float,
                1i32 as libc::c_float,
                1i32 as libc::c_float,
                0.33f32,
                1000i32 as libc::c_float,
                cg.time,
                0i32,
                LEF_PUFF_DONT_SCALE as libc::c_int,
                cgs.media.smokePuffShader,
            );
            trap_S_StartSound(
                (*cent).lerpOrigin.as_mut_ptr(),
                -1i32,
                CHAN_VOICE as libc::c_int,
                cgs.media.jumpPadSound,
            );
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_VOICE as libc::c_int,
                CG_CustomSound(
                    (*es).number,
                    b"*jump1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            current_block_479 = 10325342238833500232;
        }
        14 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_JUMP\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_VOICE as libc::c_int,
                CG_CustomSound(
                    (*es).number,
                    b"*jump1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            current_block_479 = 10325342238833500232;
        }
        76 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_TAUNT\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_VOICE as libc::c_int,
                CG_CustomSound(
                    (*es).number,
                    b"*taunt.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            current_block_479 = 10325342238833500232;
        }
        15 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_WATER_TOUCH\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.watrInSound,
            );
            current_block_479 = 10325342238833500232;
        }
        16 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_WATER_LEAVE\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.watrOutSound,
            );
            current_block_479 = 10325342238833500232;
        }
        17 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_WATER_UNDER\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.watrUnSound,
            );
            current_block_479 = 10325342238833500232;
        }
        18 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_WATER_CLEAR\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                CG_CustomSound(
                    (*es).number,
                    b"*gasp.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
            current_block_479 = 10325342238833500232;
        }
        19 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_ITEM_PICKUP\n\x00" as *const u8 as *const libc::c_char);
            }
            let mut item: *mut gitem_t = 0 as *mut gitem_t;
            let mut index: libc::c_int = 0;
            index = (*es).eventParm;
            if index < 1i32 || index >= bg_numItems {
                current_block_479 = 10325342238833500232;
            } else {
                item = &mut *bg_itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
                if (*item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint
                    || (*item).giType as libc::c_uint == IT_TEAM as libc::c_int as libc::c_uint
                {
                    trap_S_StartSound(
                        0 as *mut vec_t,
                        (*es).number,
                        CHAN_AUTO as libc::c_int,
                        cgs.media.n_healthSound,
                    );
                } else if !((*item).giType as libc::c_uint
                    == IT_PERSISTANT_POWERUP as libc::c_int as libc::c_uint)
                {
                    trap_S_StartSound(
                        0 as *mut vec_t,
                        (*es).number,
                        CHAN_AUTO as libc::c_int,
                        trap_S_RegisterSound((*item).pickup_sound, qfalse),
                    );
                }
                if (*es).number == (*cg.snap).ps.clientNum {
                    CG_ItemPickup(index);
                }
                current_block_479 = 10325342238833500232;
            }
        }
        20 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_GLOBAL_ITEM_PICKUP\n\x00" as *const u8 as *const libc::c_char);
            }
            let mut item_0: *mut gitem_t = 0 as *mut gitem_t;
            let mut index_0: libc::c_int = 0;
            index_0 = (*es).eventParm;
            if index_0 < 1i32 || index_0 >= bg_numItems {
                current_block_479 = 10325342238833500232;
            } else {
                item_0 = &mut *bg_itemlist.as_mut_ptr().offset(index_0 as isize) as *mut gitem_t;
                if !(*item_0).pickup_sound.is_null() {
                    trap_S_StartSound(
                        0 as *mut vec_t,
                        (*cg.snap).ps.clientNum,
                        CHAN_AUTO as libc::c_int,
                        trap_S_RegisterSound((*item_0).pickup_sound, qfalse),
                    );
                }
                if (*es).number == (*cg.snap).ps.clientNum {
                    CG_ItemPickup(index_0);
                }
                current_block_479 = 10325342238833500232;
            }
        }
        21 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_NOAMMO\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*es).number == (*cg.snap).ps.clientNum {
                CG_OutOfAmmoChange();
            }
            current_block_479 = 10325342238833500232;
        }
        22 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_CHANGE_WEAPON\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.selectSound,
            );
            current_block_479 = 10325342238833500232;
        }
        23 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_FIRE_WEAPON\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_FireWeapon(cent);
            current_block_479 = 10325342238833500232;
        }
        24 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM0\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        25 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM1\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        26 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM2\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        27 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM3\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        28 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM4\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        29 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM5\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        30 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM6\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        31 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM7\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        32 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM8\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        33 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM9\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        34 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM10\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        35 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM11\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        36 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM12\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        37 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM13\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        38 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM14\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        39 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_USE_ITEM15\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_UseItem(cent);
            current_block_479 = 10325342238833500232;
        }
        42 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_PLAYER_TELEPORT_IN\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.teleInSound,
            );
            CG_SpawnEffect(position);
            current_block_479 = 10325342238833500232;
        }
        43 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_PLAYER_TELEPORT_OUT\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.teleOutSound,
            );
            CG_SpawnEffect(position);
            current_block_479 = 10325342238833500232;
        }
        41 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_ITEM_POP\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.respawnSound,
            );
            current_block_479 = 10325342238833500232;
        }
        40 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_ITEM_RESPAWN\n\x00" as *const u8 as *const libc::c_char);
            }
            (*cent).miscTime = cg.time;
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_AUTO as libc::c_int,
                cgs.media.respawnSound,
            );
            current_block_479 = 10325342238833500232;
        }
        44 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_GRENADE_BOUNCE\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != rand() & 1i32 {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_AUTO as libc::c_int,
                    cgs.media.hgrenb1aSound,
                );
            } else {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_AUTO as libc::c_int,
                    cgs.media.hgrenb2aSound,
                );
            }
            current_block_479 = 10325342238833500232;
        }
        65 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_SCOREPLUM\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_ScorePlum(
                (*cent).currentState.otherEntityNum,
                (*cent).lerpOrigin.as_mut_ptr(),
                (*cent).currentState.time,
            );
            current_block_479 = 10325342238833500232;
        }
        50 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_MISSILE_HIT\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_MissileHitPlayer(
                (*es).weapon,
                position,
                dir.as_mut_ptr(),
                (*es).otherEntityNum,
            );
            current_block_479 = 10325342238833500232;
        }
        51 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_MISSILE_MISS\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_MissileHitWall(
                (*es).weapon,
                0i32,
                position,
                dir.as_mut_ptr(),
                IMPACTSOUND_DEFAULT,
            );
            current_block_479 = 10325342238833500232;
        }
        52 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_MISSILE_MISS_METAL\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_MissileHitWall(
                (*es).weapon,
                0i32,
                position,
                dir.as_mut_ptr(),
                IMPACTSOUND_METAL,
            );
            current_block_479 = 10325342238833500232;
        }
        53 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_RAILTRAIL\n\x00" as *const u8 as *const libc::c_char);
            }
            (*cent).currentState.weapon = WP_RAILGUN as libc::c_int;
            if (*es).clientNum == (*cg.snap).ps.clientNum && 0 == cg.renderingThirdPerson as u64 {
                if cg_drawGun.integer == 2i32 {
                    (*es).origin2[0usize] = (*es).origin2[0usize]
                        + cg.refdef.viewaxis[1usize][0usize] * 8i32 as libc::c_float;
                    (*es).origin2[1usize] = (*es).origin2[1usize]
                        + cg.refdef.viewaxis[1usize][1usize] * 8i32 as libc::c_float;
                    (*es).origin2[2usize] = (*es).origin2[2usize]
                        + cg.refdef.viewaxis[1usize][2usize] * 8i32 as libc::c_float
                } else if cg_drawGun.integer == 3i32 {
                    (*es).origin2[0usize] = (*es).origin2[0usize]
                        + cg.refdef.viewaxis[1usize][0usize] * 4i32 as libc::c_float;
                    (*es).origin2[1usize] = (*es).origin2[1usize]
                        + cg.refdef.viewaxis[1usize][1usize] * 4i32 as libc::c_float;
                    (*es).origin2[2usize] = (*es).origin2[2usize]
                        + cg.refdef.viewaxis[1usize][2usize] * 4i32 as libc::c_float
                }
            }
            CG_RailTrail(
                ci,
                (*es).origin2.as_mut_ptr(),
                (*es).pos.trBase.as_mut_ptr(),
            );
            if (*es).eventParm != 255i32 {
                ByteToDir((*es).eventParm, dir.as_mut_ptr());
                CG_MissileHitWall(
                    (*es).weapon,
                    (*es).clientNum,
                    position,
                    dir.as_mut_ptr(),
                    IMPACTSOUND_DEFAULT,
                );
            }
            current_block_479 = 10325342238833500232;
        }
        49 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_BULLET_HIT_WALL\n\x00" as *const u8 as *const libc::c_char);
            }
            ByteToDir((*es).eventParm, dir.as_mut_ptr());
            CG_Bullet(
                (*es).pos.trBase.as_mut_ptr(),
                (*es).otherEntityNum,
                dir.as_mut_ptr(),
                qfalse,
                (1i32 << 10i32) - 2i32,
            );
            current_block_479 = 10325342238833500232;
        }
        48 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_BULLET_HIT_FLESH\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Bullet(
                (*es).pos.trBase.as_mut_ptr(),
                (*es).otherEntityNum,
                dir.as_mut_ptr(),
                qtrue,
                (*es).eventParm,
            );
            current_block_479 = 10325342238833500232;
        }
        54 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_SHOTGUN\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_ShotgunFire(es);
            current_block_479 = 10325342238833500232;
        }
        45 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_GENERAL_SOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != cgs.gameSounds[(*es).eventParm as usize] {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_VOICE as libc::c_int,
                    cgs.gameSounds[(*es).eventParm as usize],
                );
            } else {
                s = CG_ConfigString(32i32 + 256i32 + (*es).eventParm);
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_VOICE as libc::c_int,
                    CG_CustomSound((*es).number, s),
                );
            }
            current_block_479 = 10325342238833500232;
        }
        46 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_GLOBAL_SOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != cgs.gameSounds[(*es).eventParm as usize] {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*cg.snap).ps.clientNum,
                    CHAN_AUTO as libc::c_int,
                    cgs.gameSounds[(*es).eventParm as usize],
                );
            } else {
                s = CG_ConfigString(32i32 + 256i32 + (*es).eventParm);
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*cg.snap).ps.clientNum,
                    CHAN_AUTO as libc::c_int,
                    CG_CustomSound((*es).number, s),
                );
            }
            current_block_479 = 10325342238833500232;
        }
        47 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_GLOBAL_TEAM_SOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            match (*es).eventParm {
                0 => {
                    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                        == TEAM_RED as libc::c_int
                    {
                        CG_AddBufferedSound(cgs.media.captureYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.captureOpponentSound);
                    }
                }
                1 => {
                    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                        == TEAM_BLUE as libc::c_int
                    {
                        CG_AddBufferedSound(cgs.media.captureYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.captureOpponentSound);
                    }
                }
                2 => {
                    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                        == TEAM_RED as libc::c_int
                    {
                        CG_AddBufferedSound(cgs.media.returnYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.returnOpponentSound);
                    }
                    CG_AddBufferedSound(cgs.media.blueFlagReturnedSound);
                }
                3 => {
                    if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                        == TEAM_BLUE as libc::c_int
                    {
                        CG_AddBufferedSound(cgs.media.returnYourTeamSound);
                    } else {
                        CG_AddBufferedSound(cgs.media.returnOpponentSound);
                    }
                    CG_AddBufferedSound(cgs.media.redFlagReturnedSound);
                }
                4 => {
                    if !(0 != (*cg.snap).ps.powerups[PW_BLUEFLAG as libc::c_int as usize]
                        || 0 != (*cg.snap).ps.powerups[PW_NEUTRALFLAG as libc::c_int as usize])
                    {
                        if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                            == TEAM_BLUE as libc::c_int
                        {
                            CG_AddBufferedSound(cgs.media.enemyTookYourFlagSound);
                        } else if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                            == TEAM_RED as libc::c_int
                        {
                            CG_AddBufferedSound(cgs.media.yourTeamTookEnemyFlagSound);
                        }
                    }
                }
                5 => {
                    if !(0 != (*cg.snap).ps.powerups[PW_REDFLAG as libc::c_int as usize]
                        || 0 != (*cg.snap).ps.powerups[PW_NEUTRALFLAG as libc::c_int as usize])
                    {
                        if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                            == TEAM_RED as libc::c_int
                        {
                            CG_AddBufferedSound(cgs.media.enemyTookYourFlagSound);
                        } else if (*cg.snap).ps.persistant[PERS_TEAM as libc::c_int as usize]
                            == TEAM_BLUE as libc::c_int
                        {
                            CG_AddBufferedSound(cgs.media.yourTeamTookEnemyFlagSound);
                        }
                    }
                }
                8 => {
                    CG_AddBufferedSound(cgs.media.redScoredSound);
                }
                9 => {
                    CG_AddBufferedSound(cgs.media.blueScoredSound);
                }
                10 => {
                    CG_AddBufferedSound(cgs.media.redLeadsSound);
                }
                11 => {
                    CG_AddBufferedSound(cgs.media.blueLeadsSound);
                }
                12 => {
                    CG_AddBufferedSound(cgs.media.teamsTiedSound);
                }
                _ => {}
            }
            current_block_479 = 10325342238833500232;
        }
        56 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_PAIN\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*cent).currentState.number != (*cg.snap).ps.clientNum {
                CG_PainEvent(cent, (*es).eventParm);
            }
            current_block_479 = 10325342238833500232;
        }
        57 | 58 | 59 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_DEATHx\n\x00" as *const u8 as *const libc::c_char);
            }
            if CG_WaterLevel(cent) == 3i32 {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_VOICE as libc::c_int,
                    CG_CustomSound(
                        (*es).number,
                        b"*drown.wav\x00" as *const u8 as *const libc::c_char,
                    ),
                );
            } else {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_VOICE as libc::c_int,
                    CG_CustomSound(
                        (*es).number,
                        va(
                            b"*death%i.wav\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            event - EV_DEATH1 as libc::c_int + 1i32,
                        ),
                    ),
                );
            }
            current_block_479 = 10325342238833500232;
        }
        60 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_OBITUARY\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Obituary(es);
            current_block_479 = 10325342238833500232;
        }
        61 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_POWERUP_QUAD\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*es).number == (*cg.snap).ps.clientNum {
                cg.powerupActive = PW_QUAD as libc::c_int;
                cg.powerupTime = cg.time
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_ITEM as libc::c_int,
                cgs.media.quadSound,
            );
            current_block_479 = 10325342238833500232;
        }
        62 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_POWERUP_BATTLESUIT\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*es).number == (*cg.snap).ps.clientNum {
                cg.powerupActive = PW_BATTLESUIT as libc::c_int;
                cg.powerupTime = cg.time
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_ITEM as libc::c_int,
                cgs.media.protectSound,
            );
            current_block_479 = 10325342238833500232;
        }
        63 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_POWERUP_REGEN\n\x00" as *const u8 as *const libc::c_char);
            }
            if (*es).number == (*cg.snap).ps.clientNum {
                cg.powerupActive = PW_REGEN as libc::c_int;
                cg.powerupTime = cg.time
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_ITEM as libc::c_int,
                cgs.media.regenSound,
            );
            current_block_479 = 10325342238833500232;
        }
        64 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_GIB_PLAYER\n\x00" as *const u8 as *const libc::c_char);
            }
            if 0 == (*es).eFlags & 0x200i32 {
                trap_S_StartSound(
                    0 as *mut vec_t,
                    (*es).number,
                    CHAN_BODY as libc::c_int,
                    cgs.media.gibSound,
                );
            }
            CG_GibPlayer((*cent).lerpOrigin.as_mut_ptr());
            current_block_479 = 10325342238833500232;
        }
        75 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_STOPLOOPINGSOUND\n\x00" as *const u8 as *const libc::c_char);
            }
            trap_S_StopLoopingSound((*es).number);
            (*es).loopSound = 0i32;
            current_block_479 = 10325342238833500232;
        }
        74 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_DEBUG_LINE\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Beam(cent);
            current_block_479 = 10325342238833500232;
        }
        _ => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"UNKNOWN\n\x00" as *const u8 as *const libc::c_char);
            }
            CG_Error(
                b"Unknown event: %i\x00" as *const u8 as *const libc::c_char,
                event,
            );
        }
    }
    match current_block_479 {
        10326049763699356704 => {
            if 0 != cg_debugEvents.integer {
                CG_Printf(b"EV_STEP\n\x00" as *const u8 as *const libc::c_char);
            }
            let mut oldStep: libc::c_float = 0.;
            let mut delta: libc::c_int = 0;
            let mut step: libc::c_int = 0;
            if !(clientNum != cg.predictedPlayerState.clientNum) {
                // if we are interpolating, we don't need to smooth steps
                if !(0 != cg.demoPlayback as libc::c_uint
                    || 0 != (*cg.snap).ps.pm_flags & 4096i32
                    || 0 != cg_nopredict.integer
                    || 0 != cg_synchronousClients.integer)
                {
                    delta = cg.time - cg.stepTime;
                    if delta < 200i32 {
                        oldStep = cg.stepChange * (200i32 - delta) as libc::c_float
                            / 200i32 as libc::c_float
                    } else {
                        oldStep = 0i32 as libc::c_float
                    }
                    step = 4i32 * (event - EV_STEP_4 as libc::c_int + 1i32);
                    cg.stepChange = oldStep + step as libc::c_float;
                    if cg.stepChange > 32i32 as libc::c_float {
                        cg.stepChange = 32i32 as libc::c_float
                    }
                    cg.stepTime = cg.time
                }
            }
        }
        _ => {}
    };
}
/*
=============
CG_Obituary
=============
*/
unsafe extern "C" fn CG_Obituary(mut ent: *mut entityState_t) {
    let mut mod_0: libc::c_int = 0;
    let mut target: libc::c_int = 0;
    let mut attacker: libc::c_int = 0;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut targetInfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut attackerInfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut targetName: [libc::c_char; 32] = [0; 32];
    let mut attackerName: [libc::c_char; 32] = [0; 32];
    let mut gender: gender_t = GENDER_MALE;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    target = (*ent).otherEntityNum;
    attacker = (*ent).otherEntityNum2;
    mod_0 = (*ent).eventParm;
    if target < 0i32 || target >= 64i32 {
        CG_Error(b"CG_Obituary: target out of range\x00" as *const u8 as *const libc::c_char);
    }
    ci = &mut cgs.clientinfo[target as usize] as *mut clientInfo_t;
    if attacker < 0i32 || attacker >= 64i32 {
        attacker = (1i32 << 10i32) - 2i32;
        attackerInfo = 0 as *const libc::c_char
    } else {
        attackerInfo = CG_ConfigString(32i32 + 256i32 + 256i32 + attacker)
    }
    targetInfo = CG_ConfigString(32i32 + 256i32 + 256i32 + target);
    if targetInfo.is_null() {
        return;
    }
    Q_strncpyz(
        targetName.as_mut_ptr(),
        Info_ValueForKey(targetInfo, b"n\x00" as *const u8 as *const libc::c_char),
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(2i32 as libc::c_ulong) as libc::c_int,
    );
    strcat(
        targetName.as_mut_ptr(),
        b"^7\x00" as *const u8 as *const libc::c_char,
    );
    message2 = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    match mod_0 {
        20 => message = b"suicides\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        19 => message = b"cratered\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        17 => {
            message = b"was squished\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        14 => {
            message =
                b"sank like a rock\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        15 => message = b"melted\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 => {
            message = b"does a back flip into the lava\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char
        }
        21 => {
            message = b"saw the light\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        22 => {
            message = b"was in the wrong place\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char
        }
        _ => message = 0 as *mut libc::c_char,
    }
    if attacker == target {
        gender = (*ci).gender;
        match mod_0 {
            5 => {
                if gender as libc::c_uint == GENDER_FEMALE as libc::c_int as libc::c_uint {
                    message = b"tripped on her own grenade\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as libc::c_uint == GENDER_NEUTER as libc::c_int as libc::c_uint {
                    message = b"tripped on its own grenade\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"tripped on his own grenade\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
            7 => {
                if gender as libc::c_uint == GENDER_FEMALE as libc::c_int as libc::c_uint {
                    message = b"blew herself up\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as libc::c_uint == GENDER_NEUTER as libc::c_int as libc::c_uint {
                    message = b"blew itself up\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"blew himself up\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
            9 => {
                if gender as libc::c_uint == GENDER_FEMALE as libc::c_int as libc::c_uint {
                    message = b"melted herself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as libc::c_uint == GENDER_NEUTER as libc::c_int as libc::c_uint {
                    message = b"melted itself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"melted himself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
            13 => {
                message = b"should have used a smaller gun\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            _ => {
                if gender as libc::c_uint == GENDER_FEMALE as libc::c_int as libc::c_uint {
                    message = b"killed herself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else if gender as libc::c_uint == GENDER_NEUTER as libc::c_int as libc::c_uint {
                    message = b"killed itself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                } else {
                    message = b"killed himself\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char
                }
            }
        }
    }
    if !message.is_null() {
        CG_Printf(
            b"%s %s.\n\x00" as *const u8 as *const libc::c_char,
            targetName.as_mut_ptr(),
            message,
        );
        return;
    }
    if attacker == (*cg.snap).ps.clientNum {
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        if (cgs.gametype as libc::c_uint) < GT_TEAM as libc::c_int as libc::c_uint {
            s = va(
                b"You fragged %s\n%s place with %i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                targetName.as_mut_ptr(),
                CG_PlaceString((*cg.snap).ps.persistant[PERS_RANK as libc::c_int as usize] + 1i32),
                (*cg.snap).ps.persistant[PERS_SCORE as libc::c_int as usize],
            )
        } else {
            s = va(
                b"You fragged %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                targetName.as_mut_ptr(),
            )
        }
        CG_CenterPrint(
            s,
            (480i32 as libc::c_double * 0.30f64) as libc::c_int,
            16i32,
        );
    }
    if attackerInfo.is_null() {
        attacker = (1i32 << 10i32) - 2i32;
        strcpy(
            attackerName.as_mut_ptr(),
            b"noname\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        Q_strncpyz(
            attackerName.as_mut_ptr(),
            Info_ValueForKey(attackerInfo, b"n\x00" as *const u8 as *const libc::c_char),
            (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_sub(2i32 as libc::c_ulong) as libc::c_int,
        );
        strcat(
            attackerName.as_mut_ptr(),
            b"^7\x00" as *const u8 as *const libc::c_char,
        );
        if target == (*cg.snap).ps.clientNum {
            Q_strncpyz(
                cg.killerName.as_mut_ptr(),
                attackerName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            );
        }
    }
    if attacker != (1i32 << 10i32) - 2i32 {
        match mod_0 {
            23 => {
                message =
                    b"was caught by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            2 => {
                message =
                    b"was pummeled by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            3 => {
                message = b"was machinegunned by\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            1 => {
                message = b"was gunned down by\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            4 => {
                message = b"ate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s grenade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            5 => {
                message =
                    b"was shredded by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s shrapnel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            6 => {
                message = b"ate\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            7 => {
                message =
                    b"almost dodged\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            8 => {
                message =
                    b"was melted by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            9 => {
                message =
                    b"was melted by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 =
                    b"\'s plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            10 => {
                message =
                    b"was railed by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            11 => {
                message = b"was electrocuted by\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            12 | 13 => {
                message =
                    b"was blasted by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 = b"\'s BFG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            18 => {
                message =
                    b"tried to invade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
                message2 = b"\'s personal space\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            }
            _ => {
                message =
                    b"was killed by\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
        }
        if !message.is_null() {
            CG_Printf(
                b"%s %s %s%s\n\x00" as *const u8 as *const libc::c_char,
                targetName.as_mut_ptr(),
                message,
                attackerName.as_mut_ptr(),
                message2,
            );
            return;
        }
    }
    CG_Printf(
        b"%s died.\n\x00" as *const u8 as *const libc::c_char,
        targetName.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_PlaceString(mut rank: libc::c_int) -> *const libc::c_char {
    static mut str: [libc::c_char; 64] = [0; 64];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 != rank & 0x4000i32 {
        rank &= !0x4000i32;
        t = b"Tied for \x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        t = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if rank == 1i32 {
        s = b"^41st^7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank == 2i32 {
        s = b"^12nd^7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank == 3i32 {
        s = b"^33rd^7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank == 11i32 {
        s = b"11th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank == 12i32 {
        s = b"12th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank == 13i32 {
        s = b"13th\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if rank % 10i32 == 1i32 {
        s = va(
            b"%ist\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    } else if rank % 10i32 == 2i32 {
        s = va(
            b"%ind\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    } else if rank % 10i32 == 3i32 {
        s = va(
            b"%ird\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    } else {
        s = va(
            b"%ith\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank,
        )
    }
    Com_sprintf(
        str.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        t,
        s,
    );
    return str.as_mut_ptr();
}
/*
================
CG_WaterLevel

Returns waterlevel for entity origin
================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_WaterLevel(mut cent: *mut centity_t) -> libc::c_int {
    let mut point: vec3_t = [0.; 3];
    let mut contents: libc::c_int = 0;
    let mut sample1: libc::c_int = 0;
    let mut sample2: libc::c_int = 0;
    let mut anim: libc::c_int = 0;
    let mut waterlevel: libc::c_int = 0;
    let mut viewheight: libc::c_int = 0;
    anim = (*cent).currentState.legsAnim & !128i32;
    if anim == LEGS_WALKCR as libc::c_int || anim == LEGS_IDLECR as libc::c_int {
        viewheight = 12i32
    } else {
        viewheight = 26i32
    }
    waterlevel = 0i32;
    point[0usize] = (*cent).lerpOrigin[0usize];
    point[1usize] = (*cent).lerpOrigin[1usize];
    point[2usize] = (*cent).lerpOrigin[2usize] + -24i32 as libc::c_float + 1i32 as libc::c_float;
    contents = CG_PointContents(point.as_mut_ptr() as *const vec_t, -1i32);
    if 0 != contents & (32i32 | 8i32 | 16i32) {
        sample2 = viewheight - -24i32;
        sample1 = sample2 / 2i32;
        waterlevel = 1i32;
        point[2usize] =
            (*cent).lerpOrigin[2usize] + -24i32 as libc::c_float + sample1 as libc::c_float;
        contents = CG_PointContents(point.as_mut_ptr() as *const vec_t, -1i32);
        if 0 != contents & (32i32 | 8i32 | 16i32) {
            waterlevel = 2i32;
            point[2usize] =
                (*cent).lerpOrigin[2usize] + -24i32 as libc::c_float + sample2 as libc::c_float;
            contents = CG_PointContents(point.as_mut_ptr() as *const vec_t, -1i32);
            if 0 != contents & (32i32 | 8i32 | 16i32) {
                waterlevel = 3i32
            }
        }
    }
    return waterlevel;
}
#[no_mangle]
pub unsafe extern "C" fn CG_PainEvent(mut cent: *mut centity_t, mut health: libc::c_int) {
    let mut snd: *mut libc::c_char = 0 as *mut libc::c_char;
    if cg.time - (*cent).pe.painTime < 500i32 {
        return;
    }
    if health < 25i32 {
        snd = b"*pain25_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if health < 50i32 {
        snd = b"*pain50_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else if health < 75i32 {
        snd = b"*pain75_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        snd = b"*pain100_1.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if CG_WaterLevel(cent) == 3i32 {
        if 0 != rand() & 1i32 {
            trap_S_StartSound(
                0 as *mut vec_t,
                (*cent).currentState.number,
                CHAN_VOICE as libc::c_int,
                CG_CustomSound(
                    (*cent).currentState.number,
                    b"sound/player/gurp1.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        } else {
            trap_S_StartSound(
                0 as *mut vec_t,
                (*cent).currentState.number,
                CHAN_VOICE as libc::c_int,
                CG_CustomSound(
                    (*cent).currentState.number,
                    b"sound/player/gurp2.wav\x00" as *const u8 as *const libc::c_char,
                ),
            );
        }
    } else {
        trap_S_StartSound(
            0 as *mut vec_t,
            (*cent).currentState.number,
            CHAN_VOICE as libc::c_int,
            CG_CustomSound((*cent).currentState.number, snd),
        );
    }
    (*cent).pe.painTime = cg.time;
    (*cent).pe.painDirection ^= 1i32;
}
//==========================================================================
/*
===============
CG_UseItem
===============
*/
unsafe extern "C" fn CG_UseItem(mut cent: *mut centity_t) {
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut itemNum: libc::c_int = 0;
    let mut clientNum: libc::c_int = 0;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    es = &mut (*cent).currentState;
    itemNum = ((*es).event & !(0x100i32 | 0x200i32)) - EV_USE_ITEM0 as libc::c_int;
    if itemNum < 0i32 || itemNum > HI_NUM_HOLDABLE as libc::c_int {
        itemNum = 0i32
    }
    if (*es).number == (*cg.snap).ps.clientNum {
        if 0 == itemNum {
            CG_CenterPrint(
                b"No item to use\x00" as *const u8 as *const libc::c_char,
                (480i32 as libc::c_double * 0.30f64) as libc::c_int,
                16i32,
            );
        } else {
            item = BG_FindItemForHoldable(itemNum as holdable_t);
            CG_CenterPrint(
                va(
                    b"Use %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*item).pickup_name,
                ),
                (480i32 as libc::c_double * 0.30f64) as libc::c_int,
                16i32,
            );
        }
    }
    match itemNum {
        1 => {}
        2 => {
            clientNum = (*cent).currentState.clientNum;
            if clientNum >= 0i32 && clientNum < 64i32 {
                ci = &mut cgs.clientinfo[clientNum as usize] as *mut clientInfo_t;
                (*ci).medkitUsageTime = cg.time
            }
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_BODY as libc::c_int,
                cgs.media.medkitSound,
            );
        }
        0 | _ => {
            trap_S_StartSound(
                0 as *mut vec_t,
                (*es).number,
                CHAN_BODY as libc::c_int,
                cgs.media.useNothingSound,
            );
        }
    };
}
/*
================
CG_ItemPickup

A new item was picked up this frame
================
*/
unsafe extern "C" fn CG_ItemPickup(mut itemNum: libc::c_int) {
    cg.itemPickup = itemNum;
    cg.itemPickupTime = cg.time;
    cg.itemPickupBlendTime = cg.time;
    if (*bg_itemlist.as_mut_ptr().offset(itemNum as isize)).giType as libc::c_uint
        == IT_WEAPON as libc::c_int as libc::c_uint
    {
        if 0 != cg_autoswitch.integer
            && (*bg_itemlist.as_mut_ptr().offset(itemNum as isize)).giTag
                != WP_MACHINEGUN as libc::c_int
        {
            cg.weaponSelectTime = cg.time;
            cg.weaponSelect = (*bg_itemlist.as_mut_ptr().offset(itemNum as isize)).giTag
        }
    };
}
