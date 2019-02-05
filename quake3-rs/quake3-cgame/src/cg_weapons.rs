use bg_public_h::{
    animation_s, animation_t, bg_itemlist, bg_numItems, gametype_t, gender_t, gitem_s, gitem_t,
    itemType_t, team_t, unnamed_0, unnamed_1, unnamed_2, unnamed_3, unnamed_4, unnamed_5, weapon_t,
    BG_EvaluateTrajectory, BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3, BOTH_DEATH1, BOTH_DEATH2,
    BOTH_DEATH3, ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE,
    ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER,
    FLAG_RUN, FLAG_STAND, FLAG_STAND2RUN, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF,
    GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM,
    GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP,
    IT_POWERUP, IT_TEAM, IT_WEAPON, LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK, LEGS_IDLE, LEGS_IDLECR,
    LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM, LEGS_TURN, LEGS_WALK,
    LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR,
    PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT,
    PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY,
    PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    TORSO_AFFIRMATIVE, TORSO_ATTACK, TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE,
    TORSO_GETFLAG, TORSO_GUARDBASE, TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND,
    TORSO_STAND2, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING,
    WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER,
    WP_SHOTGUN,
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
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, impactSound_t,
    itemInfo_t, leBounceSoundType_t, leMarkType_t, leType_t, lerpFrame_t, localEntity_s,
    localEntity_t, playerEntity_t, score_t, trap_CM_BoxTrace, trap_R_AddLightToScene,
    trap_R_AddPolyToScene, trap_R_AddRefEntityToScene, trap_R_LerpTag, trap_R_ModelBounds,
    trap_R_RegisterModel, trap_R_RegisterShader, trap_R_SetColor, trap_S_AddLoopingSound,
    trap_S_RegisterSound, trap_S_StartSound, unnamed_6, weaponInfo_s, weaponInfo_t, FOOTSTEP_BOOT,
    FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL, FOOTSTEP_NORMAL,
    FOOTSTEP_SPLASH, FOOTSTEP_TOTAL, IMPACTSOUND_DEFAULT, IMPACTSOUND_FLESH, IMPACTSOUND_METAL,
    LEBS_BLOOD, LEBS_BRASS, LEBS_NONE, LEF_PUFF_DONT_SCALE, LEF_SOUND1, LEF_SOUND2, LEF_TUMBLE,
    LEMT_BLOOD, LEMT_BURN, LEMT_NONE, LE_EXPLOSION, LE_FADE_RGB, LE_FALL_SCALE_FADE, LE_FRAGMENT,
    LE_MARK, LE_MOVE_SCALE_FADE, LE_SCALE_FADE, LE_SCOREPLUM, LE_SPRITE_EXPLOSION,
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
use libc;
use q_shared_h::{
    axisDefault, byte, clipHandle_t, cplane_s, cplane_t, cvarHandle_t, entityState_s,
    entityState_t, gameState_t, orientation_t, playerState_s, playerState_t, qboolean, qfalse,
    qhandle_t, qtrue, sfxHandle_t, trType_t, trace_t, trajectory_t, unnamed, vec3_origin, vec3_t,
    vec_t, vmCvar_t, AngleMod, AngleVectors, AnglesToAxis, AxisClear, AxisCopy, COM_StripExtension,
    MatrixMultiply, PerpendicularVector, Q_crandom, Q_strcat, RotatePointAroundVector,
    VectorNormalize, VectorNormalize2, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM, CHAN_LOCAL,
    CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, memset, rand, sin, sqrt};
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
//
// cg_weapons.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_NextWeapon_f() {
    let mut i: libc::c_int = 0;
    let mut original: libc::c_int = 0;
    if cg.snap.is_null() {
        return;
    }
    if 0 != (*cg.snap).ps.pm_flags & 4096i32 {
        return;
    }
    cg.weaponSelectTime = cg.time;
    original = cg.weaponSelect;
    i = 0i32;
    while i < 16i32 {
        cg.weaponSelect += 1;
        if cg.weaponSelect == 16i32 {
            cg.weaponSelect = 0i32
        }
        if !(cg.weaponSelect == WP_GAUNTLET as libc::c_int) {
            // never cycle to gauntlet
            if 0 != CG_WeaponSelectable(cg.weaponSelect) as u64 {
                break;
            }
        }
        i += 1
    }
    if i == 16i32 {
        cg.weaponSelect = original
    };
}
/*
===============
CG_WeaponSelectable
===============
*/
unsafe extern "C" fn CG_WeaponSelectable(mut i: libc::c_int) -> qboolean {
    if 0 == (*cg.snap).ps.ammo[i as usize] {
        return qfalse;
    }
    if 0 == (*cg.snap).ps.stats[STAT_WEAPONS as libc::c_int as usize] & 1i32 << i {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CG_PrevWeapon_f() {
    let mut i: libc::c_int = 0;
    let mut original: libc::c_int = 0;
    if cg.snap.is_null() {
        return;
    }
    if 0 != (*cg.snap).ps.pm_flags & 4096i32 {
        return;
    }
    cg.weaponSelectTime = cg.time;
    original = cg.weaponSelect;
    i = 0i32;
    while i < 16i32 {
        cg.weaponSelect -= 1;
        if cg.weaponSelect == -1i32 {
            cg.weaponSelect = 16i32 - 1i32
        }
        if !(cg.weaponSelect == WP_GAUNTLET as libc::c_int) {
            // never cycle to gauntlet
            if 0 != CG_WeaponSelectable(cg.weaponSelect) as u64 {
                break;
            }
        }
        i += 1
    }
    if i == 16i32 {
        cg.weaponSelect = original
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_Weapon_f() {
    let mut num: libc::c_int = 0;
    if cg.snap.is_null() {
        return;
    }
    if 0 != (*cg.snap).ps.pm_flags & 4096i32 {
        return;
    }
    num = atoi(CG_Argv(1i32));
    if num < 1i32 || num > 16i32 - 1i32 {
        return;
    }
    cg.weaponSelectTime = cg.time;
    if 0 == (*cg.snap).ps.stats[STAT_WEAPONS as libc::c_int as usize] & 1i32 << num {
        return;
    }
    cg.weaponSelect = num;
}
#[no_mangle]
pub unsafe extern "C" fn CG_RegisterWeapon(mut weaponNum: libc::c_int) {
    let mut weaponInfo: *mut weaponInfo_t = 0 as *mut weaponInfo_t;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut ammo: *mut gitem_t = 0 as *mut gitem_t;
    let mut path: [libc::c_char; 64] = [0; 64];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    weaponInfo = &mut cg_weapons[weaponNum as usize] as *mut weaponInfo_t;
    if weaponNum == 0i32 {
        return;
    }
    if 0 != (*weaponInfo).registered as u64 {
        return;
    }
    memset(
        weaponInfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<weaponInfo_t>() as libc::c_ulong,
    );
    (*weaponInfo).registered = qtrue;
    item = bg_itemlist.as_mut_ptr().offset(1isize);
    while !(*item).classname.is_null() {
        if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint
            && (*item).giTag == weaponNum
        {
            (*weaponInfo).item = item;
            break;
        } else {
            item = item.offset(1isize)
        }
    }
    if (*item).classname.is_null() {
        CG_Error(
            b"Couldn\'t find weapon %i\x00" as *const u8 as *const libc::c_char,
            weaponNum,
        );
    }
    CG_RegisterItemVisuals(
        item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as libc::c_int,
    );
    (*weaponInfo).weaponModel = trap_R_RegisterModel((*item).world_model[0usize]);
    trap_R_ModelBounds(
        (*weaponInfo).weaponModel,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    i = 0i32;
    while i < 3i32 {
        (*weaponInfo).weaponMidpoint[i as usize] = (mins[i as usize] as libc::c_double
            + 0.5f64 * (maxs[i as usize] - mins[i as usize]) as libc::c_double)
            as vec_t;
        i += 1
    }
    (*weaponInfo).weaponIcon = trap_R_RegisterShader((*item).icon);
    (*weaponInfo).ammoIcon = trap_R_RegisterShader((*item).icon);
    ammo = bg_itemlist.as_mut_ptr().offset(1isize);
    while !(*ammo).classname.is_null() {
        if (*ammo).giType as libc::c_uint == IT_AMMO as libc::c_int as libc::c_uint
            && (*ammo).giTag == weaponNum
        {
            break;
        }
        ammo = ammo.offset(1isize)
    }
    if !(*ammo).classname.is_null() && !(*ammo).world_model[0usize].is_null() {
        (*weaponInfo).ammoModel = trap_R_RegisterModel((*ammo).world_model[0usize])
    }
    COM_StripExtension(
        (*item).world_model[0usize],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"_flash.md3\x00" as *const u8 as *const libc::c_char,
    );
    (*weaponInfo).flashModel = trap_R_RegisterModel(path.as_mut_ptr());
    COM_StripExtension(
        (*item).world_model[0usize],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"_barrel.md3\x00" as *const u8 as *const libc::c_char,
    );
    (*weaponInfo).barrelModel = trap_R_RegisterModel(path.as_mut_ptr());
    COM_StripExtension(
        (*item).world_model[0usize],
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_strcat(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"_hand.md3\x00" as *const u8 as *const libc::c_char,
    );
    (*weaponInfo).handsModel = trap_R_RegisterModel(path.as_mut_ptr());
    if 0 == (*weaponInfo).handsModel {
        (*weaponInfo).handsModel = trap_R_RegisterModel(
            b"models/weapons2/shotgun/shotgun_hand.md3\x00" as *const u8 as *const libc::c_char,
        )
    }
    match weaponNum {
        1 => {
            (*weaponInfo).flashDlightColor[0usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[1usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[2usize] = 1.0f32;
            (*weaponInfo).firingSound = trap_S_RegisterSound(
                b"sound/weapons/melee/fstrun.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/melee/fstatck.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            )
        }
        6 => {
            (*weaponInfo).flashDlightColor[0usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[1usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[2usize] = 1.0f32;
            (*weaponInfo).readySound = trap_S_RegisterSound(
                b"sound/weapons/melee/fsthum.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).firingSound = trap_S_RegisterSound(
                b"sound/weapons/lightning/lg_hum.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/lightning/lg_fire.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.lightningShader =
                trap_R_RegisterShader(b"lightningBoltNew\x00" as *const u8 as *const libc::c_char);
            cgs.media.lightningExplosionModel = trap_R_RegisterModel(
                b"models/weaphits/crackle.md3\x00" as *const u8 as *const libc::c_char,
            );
            cgs.media.sfx_lghit1 = trap_S_RegisterSound(
                b"sound/weapons/lightning/lg_hit.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.sfx_lghit2 = trap_S_RegisterSound(
                b"sound/weapons/lightning/lg_hit2.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.sfx_lghit3 = trap_S_RegisterSound(
                b"sound/weapons/lightning/lg_hit3.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            )
        }
        10 => {
            (*weaponInfo).flashDlightColor[0usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[1usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[2usize] = 1.0f32;
            (*weaponInfo).missileModel = trap_R_RegisterModel(
                b"models/ammo/rocket/rocket.md3\x00" as *const u8 as *const libc::c_char,
            );
            (*weaponInfo).missileTrailFunc = Some(CG_GrappleTrail);
            (*weaponInfo).missileDlight = 200i32 as libc::c_float;
            (*weaponInfo).missileDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).missileDlightColor[1usize] = 0.75f32;
            (*weaponInfo).missileDlightColor[2usize] = 0i32 as vec_t;
            (*weaponInfo).readySound = trap_S_RegisterSound(
                b"sound/weapons/melee/fsthum.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).firingSound = trap_S_RegisterSound(
                b"sound/weapons/melee/fstrun.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.lightningShader =
                trap_R_RegisterShader(b"lightningBoltNew\x00" as *const u8 as *const libc::c_char)
        }
        2 => {
            (*weaponInfo).flashDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[1usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[2usize] = 0i32 as vec_t;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf1b.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashSound[1usize] = trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf2b.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashSound[2usize] = trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf3b.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashSound[3usize] = trap_S_RegisterSound(
                b"sound/weapons/machinegun/machgf4b.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).ejectBrassFunc = Some(CG_MachineGunEjectBrass);
            cgs.media.bulletExplosionShader =
                trap_R_RegisterShader(b"bulletExplosion\x00" as *const u8 as *const libc::c_char)
        }
        3 => {
            (*weaponInfo).flashDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[1usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[2usize] = 0i32 as vec_t;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/shotgun/sshotf1b.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).ejectBrassFunc = Some(CG_ShotgunEjectBrass)
        }
        5 => {
            (*weaponInfo).missileModel = trap_R_RegisterModel(
                b"models/ammo/rocket/rocket.md3\x00" as *const u8 as *const libc::c_char,
            );
            (*weaponInfo).missileSound = trap_S_RegisterSound(
                b"sound/weapons/rocket/rockfly.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).missileTrailFunc = Some(CG_RocketTrail);
            (*weaponInfo).missileDlight = 200i32 as libc::c_float;
            (*weaponInfo).wiTrailTime = 2000i32 as libc::c_float;
            (*weaponInfo).trailRadius = 64i32 as libc::c_float;
            (*weaponInfo).missileDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).missileDlightColor[1usize] = 0.75f32;
            (*weaponInfo).missileDlightColor[2usize] = 0i32 as vec_t;
            (*weaponInfo).flashDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[1usize] = 0.75f32;
            (*weaponInfo).flashDlightColor[2usize] = 0i32 as vec_t;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/rocket/rocklf1a.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.rocketExplosionShader =
                trap_R_RegisterShader(b"rocketExplosion\x00" as *const u8 as *const libc::c_char)
        }
        4 => {
            (*weaponInfo).missileModel = trap_R_RegisterModel(
                b"models/ammo/grenade1.md3\x00" as *const u8 as *const libc::c_char,
            );
            (*weaponInfo).missileTrailFunc = Some(CG_GrenadeTrail);
            (*weaponInfo).wiTrailTime = 700i32 as libc::c_float;
            (*weaponInfo).trailRadius = 32i32 as libc::c_float;
            (*weaponInfo).flashDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[1usize] = 0.70f32;
            (*weaponInfo).flashDlightColor[2usize] = 0i32 as vec_t;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/grenade/grenlf1a.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.grenadeExplosionShader =
                trap_R_RegisterShader(b"grenadeExplosion\x00" as *const u8 as *const libc::c_char)
        }
        8 => {
            (*weaponInfo).missileTrailFunc = Some(CG_PlasmaTrail);
            (*weaponInfo).missileSound = trap_S_RegisterSound(
                b"sound/weapons/plasma/lasfly.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashDlightColor[0usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[1usize] = 0.6f32;
            (*weaponInfo).flashDlightColor[2usize] = 1.0f32;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/plasma/hyprbf1a.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.plasmaExplosionShader =
                trap_R_RegisterShader(b"plasmaExplosion\x00" as *const u8 as *const libc::c_char);
            cgs.media.railRingsShader =
                trap_R_RegisterShader(b"railDisc\x00" as *const u8 as *const libc::c_char)
        }
        7 => {
            (*weaponInfo).readySound = trap_S_RegisterSound(
                b"sound/weapons/railgun/rg_hum.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[1usize] = 0.5f32;
            (*weaponInfo).flashDlightColor[2usize] = 0i32 as vec_t;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/railgun/railgf1a.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.railExplosionShader =
                trap_R_RegisterShader(b"railExplosion\x00" as *const u8 as *const libc::c_char);
            cgs.media.railRingsShader =
                trap_R_RegisterShader(b"railDisc\x00" as *const u8 as *const libc::c_char);
            cgs.media.railCoreShader =
                trap_R_RegisterShader(b"railCore\x00" as *const u8 as *const libc::c_char)
        }
        9 => {
            (*weaponInfo).readySound = trap_S_RegisterSound(
                b"sound/weapons/bfg/bfg_hum.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            (*weaponInfo).flashDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[1usize] = 0.7f32;
            (*weaponInfo).flashDlightColor[2usize] = 1i32 as vec_t;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/bfg/bfg_fire.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            );
            cgs.media.bfgExplosionShader =
                trap_R_RegisterShader(b"bfgExplosion\x00" as *const u8 as *const libc::c_char);
            (*weaponInfo).missileModel = trap_R_RegisterModel(
                b"models/weaphits/bfg.md3\x00" as *const u8 as *const libc::c_char,
            );
            (*weaponInfo).missileSound = trap_S_RegisterSound(
                b"sound/weapons/rocket/rockfly.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            )
        }
        _ => {
            (*weaponInfo).flashDlightColor[0usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[1usize] = 1i32 as vec_t;
            (*weaponInfo).flashDlightColor[2usize] = 1i32 as vec_t;
            (*weaponInfo).flashSound[0usize] = trap_S_RegisterSound(
                b"sound/weapons/rocket/rocklf1a.wav\x00" as *const u8 as *const libc::c_char,
                qfalse,
            )
        }
    };
}
/*
==========================
CG_PlasmaTrail
==========================
*/
unsafe extern "C" fn CG_PlasmaTrail(mut cent: *mut centity_t, mut wi: *const weaponInfo_t) {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    let mut velocity: vec3_t = [0.; 3];
    let mut xvelocity: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut xoffset: vec3_t = [0.; 3];
    let mut v: [vec3_t; 3] = [[0.; 3]; 3];
    let mut waterScale: libc::c_float = 1.0f32;
    if 0 != cg_noProjectileTrail.integer || 0 != cg_oldPlasma.integer {
        return;
    }
    es = &mut (*cent).currentState;
    BG_EvaluateTrajectory(&mut (*es).pos, cg.time, origin.as_mut_ptr());
    le = CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    velocity[0usize] = (60i32 as libc::c_double
        - 120i32 as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))) as vec_t;
    velocity[1usize] = (40i32 as libc::c_double
        - 80i32 as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))) as vec_t;
    velocity[2usize] = (100i32 as libc::c_double
        - 200i32 as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))) as vec_t;
    (*le).leType = LE_MOVE_SCALE_FADE;
    (*le).leFlags = LEF_TUMBLE as libc::c_int;
    (*le).leBounceSoundType = LEBS_NONE;
    (*le).leMarkType = LEMT_NONE;
    (*le).startTime = cg.time;
    (*le).endTime = (*le).startTime + 600i32;
    (*le).pos.trType = TR_GRAVITY;
    (*le).pos.trTime = cg.time;
    AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const vec_t,
        v.as_mut_ptr(),
    );
    offset[0usize] = 2i32 as vec_t;
    offset[1usize] = 2i32 as vec_t;
    offset[2usize] = 2i32 as vec_t;
    xoffset[0usize] = offset[0usize] * v[0usize][0usize]
        + offset[1usize] * v[1usize][0usize]
        + offset[2usize] * v[2usize][0usize];
    xoffset[1usize] = offset[0usize] * v[0usize][1usize]
        + offset[1usize] * v[1usize][1usize]
        + offset[2usize] * v[2usize][1usize];
    xoffset[2usize] = offset[0usize] * v[0usize][2usize]
        + offset[1usize] * v[1usize][2usize]
        + offset[2usize] * v[2usize][2usize];
    (*re).origin[0usize] = origin[0usize] + xoffset[0usize];
    (*re).origin[1usize] = origin[1usize] + xoffset[1usize];
    (*re).origin[2usize] = origin[2usize] + xoffset[2usize];
    (*le).pos.trBase[0usize] = (*re).origin[0usize];
    (*le).pos.trBase[1usize] = (*re).origin[1usize];
    (*le).pos.trBase[2usize] = (*re).origin[2usize];
    if 0 != CG_PointContents((*re).origin.as_mut_ptr() as *const vec_t, -1i32) & 32i32 {
        waterScale = 0.10f32
    }
    xvelocity[0usize] = velocity[0usize] * v[0usize][0usize]
        + velocity[1usize] * v[1usize][0usize]
        + velocity[2usize] * v[2usize][0usize];
    xvelocity[1usize] = velocity[0usize] * v[0usize][1usize]
        + velocity[1usize] * v[1usize][1usize]
        + velocity[2usize] * v[2usize][1usize];
    xvelocity[2usize] = velocity[0usize] * v[0usize][2usize]
        + velocity[1usize] * v[1usize][2usize]
        + velocity[2usize] * v[2usize][2usize];
    (*le).pos.trDelta[0usize] = xvelocity[0usize] * waterScale;
    (*le).pos.trDelta[1usize] = xvelocity[1usize] * waterScale;
    (*le).pos.trDelta[2usize] = xvelocity[2usize] * waterScale;
    AxisCopy(axisDefault.as_mut_ptr(), (*re).axis.as_mut_ptr());
    (*re).shaderTime = cg.time as libc::c_float / 1000.0f32;
    (*re).reType = RT_SPRITE;
    (*re).radius = 0.25f32;
    (*re).customShader = cgs.media.railRingsShader;
    (*le).bounceFactor = 0.3f32;
    (*re).shaderRGBA[0usize] = ((*wi).flashDlightColor[0usize] * 63i32 as libc::c_float) as byte;
    (*re).shaderRGBA[1usize] = ((*wi).flashDlightColor[1usize] * 63i32 as libc::c_float) as byte;
    (*re).shaderRGBA[2usize] = ((*wi).flashDlightColor[2usize] * 63i32 as libc::c_float) as byte;
    (*re).shaderRGBA[3usize] = 63i32 as byte;
    (*le).color[0usize] =
        ((*wi).flashDlightColor[0usize] as libc::c_double * 0.2f64) as libc::c_float;
    (*le).color[1usize] =
        ((*wi).flashDlightColor[1usize] as libc::c_double * 0.2f64) as libc::c_float;
    (*le).color[2usize] =
        ((*wi).flashDlightColor[2usize] as libc::c_double * 0.2f64) as libc::c_float;
    (*le).color[3usize] = 0.25f32;
    (*le).angles.trType = TR_LINEAR;
    (*le).angles.trTime = cg.time;
    (*le).angles.trBase[0usize] = (rand() & 31i32) as vec_t;
    (*le).angles.trBase[1usize] = (rand() & 31i32) as vec_t;
    (*le).angles.trBase[2usize] = (rand() & 31i32) as vec_t;
    (*le).angles.trDelta[0usize] = 1i32 as vec_t;
    (*le).angles.trDelta[1usize] = 0.5f64 as vec_t;
    (*le).angles.trDelta[2usize] = 0i32 as vec_t;
}
/*
==========================
CG_GrenadeTrail
==========================
*/
unsafe extern "C" fn CG_GrenadeTrail(mut ent: *mut centity_t, mut wi: *const weaponInfo_t) {
    CG_RocketTrail(ent, wi);
}
/*
==========================
CG_RocketTrail
==========================
*/
unsafe extern "C" fn CG_RocketTrail(mut ent: *mut centity_t, mut wi: *const weaponInfo_t) {
    let mut step: libc::c_int = 0;
    let mut origin: vec3_t = [0.; 3];
    let mut lastPos: vec3_t = [0.; 3];
    let mut t: libc::c_int = 0;
    let mut startTime: libc::c_int = 0;
    let mut contents: libc::c_int = 0;
    let mut lastContents: libc::c_int = 0;
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    let mut up: vec3_t = [0.; 3];
    let mut smoke: *mut localEntity_t = 0 as *mut localEntity_t;
    if 0 != cg_noProjectileTrail.integer {
        return;
    }
    up[0usize] = 0i32 as vec_t;
    up[1usize] = 0i32 as vec_t;
    up[2usize] = 0i32 as vec_t;
    step = 50i32;
    es = &mut (*ent).currentState;
    startTime = (*ent).trailTime;
    t = step * ((startTime + step) / step);
    BG_EvaluateTrajectory(&mut (*es).pos, cg.time, origin.as_mut_ptr());
    contents = CG_PointContents(origin.as_mut_ptr() as *const vec_t, -1i32);
    if (*es).pos.trType as libc::c_uint == TR_STATIONARY as libc::c_int as libc::c_uint {
        (*ent).trailTime = cg.time;
        return;
    }
    BG_EvaluateTrajectory(&mut (*es).pos, (*ent).trailTime, lastPos.as_mut_ptr());
    lastContents = CG_PointContents(lastPos.as_mut_ptr() as *const vec_t, -1i32);
    (*ent).trailTime = cg.time;
    if 0 != contents & (32i32 | 16i32 | 8i32) {
        if 0 != contents & lastContents & 32i32 {
            CG_BubbleTrail(
                lastPos.as_mut_ptr(),
                origin.as_mut_ptr(),
                8i32 as libc::c_float,
            );
        }
        return;
    }
    while t <= (*ent).trailTime {
        BG_EvaluateTrajectory(&mut (*es).pos, t, lastPos.as_mut_ptr());
        smoke = CG_SmokePuff(
            lastPos.as_mut_ptr() as *const vec_t,
            up.as_mut_ptr() as *const vec_t,
            (*wi).trailRadius,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            0.33f32,
            (*wi).wiTrailTime,
            t,
            0i32,
            0i32,
            cgs.media.smokePuffShader,
        );
        (*smoke).leType = LE_SCALE_FADE;
        t += step
    }
}
/*
==========================
CG_ShotgunEjectBrass
==========================
*/
unsafe extern "C" fn CG_ShotgunEjectBrass(mut cent: *mut centity_t) {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut velocity: vec3_t = [0.; 3];
    let mut xvelocity: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut xoffset: vec3_t = [0.; 3];
    let mut v: [vec3_t; 3] = [[0.; 3]; 3];
    let mut i: libc::c_int = 0;
    if cg_brassTime.integer <= 0i32 {
        return;
    }
    i = 0i32;
    while i < 2i32 {
        let mut waterScale: libc::c_float = 1.0f32;
        le = CG_AllocLocalEntity();
        re = &mut (*le).refEntity;
        velocity[0usize] = (60i32 as libc::c_double
            + 60i32 as libc::c_double
                * (2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64))) as vec_t;
        if i == 0i32 {
            velocity[1usize] = (40i32 as libc::c_double
                + 10i32 as libc::c_double
                    * (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64))) as vec_t
        } else {
            velocity[1usize] = (-40i32 as libc::c_double
                + 10i32 as libc::c_double
                    * (2.0f64
                        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                            as libc::c_double
                            - 0.5f64))) as vec_t
        }
        velocity[2usize] = (100i32 as libc::c_double
            + 50i32 as libc::c_double
                * (2.0f64
                    * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                        as libc::c_double
                        - 0.5f64))) as vec_t;
        (*le).leType = LE_FRAGMENT;
        (*le).startTime = cg.time;
        (*le).endTime = (((*le).startTime + cg_brassTime.integer * 3i32) as libc::c_float
            + cg_brassTime.integer as libc::c_float
                * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float))
            as libc::c_int;
        (*le).pos.trType = TR_GRAVITY;
        (*le).pos.trTime = cg.time;
        AnglesToAxis(
            (*cent).lerpAngles.as_mut_ptr() as *const vec_t,
            v.as_mut_ptr(),
        );
        offset[0usize] = 8i32 as vec_t;
        offset[1usize] = 0i32 as vec_t;
        offset[2usize] = 24i32 as vec_t;
        xoffset[0usize] = offset[0usize] * v[0usize][0usize]
            + offset[1usize] * v[1usize][0usize]
            + offset[2usize] * v[2usize][0usize];
        xoffset[1usize] = offset[0usize] * v[0usize][1usize]
            + offset[1usize] * v[1usize][1usize]
            + offset[2usize] * v[2usize][1usize];
        xoffset[2usize] = offset[0usize] * v[0usize][2usize]
            + offset[1usize] * v[1usize][2usize]
            + offset[2usize] * v[2usize][2usize];
        (*re).origin[0usize] = (*cent).lerpOrigin[0usize] + xoffset[0usize];
        (*re).origin[1usize] = (*cent).lerpOrigin[1usize] + xoffset[1usize];
        (*re).origin[2usize] = (*cent).lerpOrigin[2usize] + xoffset[2usize];
        (*le).pos.trBase[0usize] = (*re).origin[0usize];
        (*le).pos.trBase[1usize] = (*re).origin[1usize];
        (*le).pos.trBase[2usize] = (*re).origin[2usize];
        if 0 != CG_PointContents((*re).origin.as_mut_ptr() as *const vec_t, -1i32) & 32i32 {
            waterScale = 0.10f32
        }
        xvelocity[0usize] = velocity[0usize] * v[0usize][0usize]
            + velocity[1usize] * v[1usize][0usize]
            + velocity[2usize] * v[2usize][0usize];
        xvelocity[1usize] = velocity[0usize] * v[0usize][1usize]
            + velocity[1usize] * v[1usize][1usize]
            + velocity[2usize] * v[2usize][1usize];
        xvelocity[2usize] = velocity[0usize] * v[0usize][2usize]
            + velocity[1usize] * v[1usize][2usize]
            + velocity[2usize] * v[2usize][2usize];
        (*le).pos.trDelta[0usize] = xvelocity[0usize] * waterScale;
        (*le).pos.trDelta[1usize] = xvelocity[1usize] * waterScale;
        (*le).pos.trDelta[2usize] = xvelocity[2usize] * waterScale;
        AxisCopy(axisDefault.as_mut_ptr(), (*re).axis.as_mut_ptr());
        (*re).hModel = cgs.media.shotgunBrassModel;
        (*le).bounceFactor = 0.3f32;
        (*le).angles.trType = TR_LINEAR;
        (*le).angles.trTime = cg.time;
        (*le).angles.trBase[0usize] = (rand() & 31i32) as vec_t;
        (*le).angles.trBase[1usize] = (rand() & 31i32) as vec_t;
        (*le).angles.trBase[2usize] = (rand() & 31i32) as vec_t;
        (*le).angles.trDelta[0usize] = 1i32 as vec_t;
        (*le).angles.trDelta[1usize] = 0.5f64 as vec_t;
        (*le).angles.trDelta[2usize] = 0i32 as vec_t;
        (*le).leFlags = LEF_TUMBLE as libc::c_int;
        (*le).leBounceSoundType = LEBS_BRASS;
        (*le).leMarkType = LEMT_NONE;
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
// cg_weapons.c -- events and effects dealing with weapons
/*
==========================
CG_MachineGunEjectBrass
==========================
*/
unsafe extern "C" fn CG_MachineGunEjectBrass(mut cent: *mut centity_t) {
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let mut velocity: vec3_t = [0.; 3];
    let mut xvelocity: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut xoffset: vec3_t = [0.; 3];
    let mut waterScale: libc::c_float = 1.0f32;
    let mut v: [vec3_t; 3] = [[0.; 3]; 3];
    if cg_brassTime.integer <= 0i32 {
        return;
    }
    le = CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    velocity[0usize] = 0i32 as vec_t;
    velocity[1usize] = (-50i32 as libc::c_double
        + 40i32 as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))) as vec_t;
    velocity[2usize] = (100i32 as libc::c_double
        + 50i32 as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64))) as vec_t;
    (*le).leType = LE_FRAGMENT;
    (*le).startTime = cg.time;
    (*le).endTime = (((*le).startTime + cg_brassTime.integer) as libc::c_float
        + (cg_brassTime.integer / 4i32) as libc::c_float
            * ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float))
        as libc::c_int;
    (*le).pos.trType = TR_GRAVITY;
    (*le).pos.trTime = cg.time - (rand() & 15i32);
    AnglesToAxis(
        (*cent).lerpAngles.as_mut_ptr() as *const vec_t,
        v.as_mut_ptr(),
    );
    offset[0usize] = 8i32 as vec_t;
    offset[1usize] = -4i32 as vec_t;
    offset[2usize] = 24i32 as vec_t;
    xoffset[0usize] = offset[0usize] * v[0usize][0usize]
        + offset[1usize] * v[1usize][0usize]
        + offset[2usize] * v[2usize][0usize];
    xoffset[1usize] = offset[0usize] * v[0usize][1usize]
        + offset[1usize] * v[1usize][1usize]
        + offset[2usize] * v[2usize][1usize];
    xoffset[2usize] = offset[0usize] * v[0usize][2usize]
        + offset[1usize] * v[1usize][2usize]
        + offset[2usize] * v[2usize][2usize];
    (*re).origin[0usize] = (*cent).lerpOrigin[0usize] + xoffset[0usize];
    (*re).origin[1usize] = (*cent).lerpOrigin[1usize] + xoffset[1usize];
    (*re).origin[2usize] = (*cent).lerpOrigin[2usize] + xoffset[2usize];
    (*le).pos.trBase[0usize] = (*re).origin[0usize];
    (*le).pos.trBase[1usize] = (*re).origin[1usize];
    (*le).pos.trBase[2usize] = (*re).origin[2usize];
    if 0 != CG_PointContents((*re).origin.as_mut_ptr() as *const vec_t, -1i32) & 32i32 {
        waterScale = 0.10f32
    }
    xvelocity[0usize] = velocity[0usize] * v[0usize][0usize]
        + velocity[1usize] * v[1usize][0usize]
        + velocity[2usize] * v[2usize][0usize];
    xvelocity[1usize] = velocity[0usize] * v[0usize][1usize]
        + velocity[1usize] * v[1usize][1usize]
        + velocity[2usize] * v[2usize][1usize];
    xvelocity[2usize] = velocity[0usize] * v[0usize][2usize]
        + velocity[1usize] * v[1usize][2usize]
        + velocity[2usize] * v[2usize][2usize];
    (*le).pos.trDelta[0usize] = xvelocity[0usize] * waterScale;
    (*le).pos.trDelta[1usize] = xvelocity[1usize] * waterScale;
    (*le).pos.trDelta[2usize] = xvelocity[2usize] * waterScale;
    AxisCopy(axisDefault.as_mut_ptr(), (*re).axis.as_mut_ptr());
    (*re).hModel = cgs.media.machinegunBrassModel;
    (*le).bounceFactor = (0.4f64 * waterScale as libc::c_double) as libc::c_float;
    (*le).angles.trType = TR_LINEAR;
    (*le).angles.trTime = cg.time;
    (*le).angles.trBase[0usize] = (rand() & 31i32) as vec_t;
    (*le).angles.trBase[1usize] = (rand() & 31i32) as vec_t;
    (*le).angles.trBase[2usize] = (rand() & 31i32) as vec_t;
    (*le).angles.trDelta[0usize] = 2i32 as vec_t;
    (*le).angles.trDelta[1usize] = 1i32 as vec_t;
    (*le).angles.trDelta[2usize] = 0i32 as vec_t;
    (*le).leFlags = LEF_TUMBLE as libc::c_int;
    (*le).leBounceSoundType = LEBS_BRASS;
    (*le).leMarkType = LEMT_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn CG_GrappleTrail(mut ent: *mut centity_t, mut wi: *const weaponInfo_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut es: *mut entityState_t = 0 as *mut entityState_t;
    let mut forward: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut beam: refEntity_t = refEntity_t {
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
    es = &mut (*ent).currentState;
    BG_EvaluateTrajectory(&mut (*es).pos, cg.time, origin.as_mut_ptr());
    (*ent).trailTime = cg.time;
    memset(
        &mut beam as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    beam.origin[0usize] =
        cg_entities[(*ent).currentState.otherEntityNum as usize].lerpOrigin[0usize];
    beam.origin[1usize] =
        cg_entities[(*ent).currentState.otherEntityNum as usize].lerpOrigin[1usize];
    beam.origin[2usize] =
        cg_entities[(*ent).currentState.otherEntityNum as usize].lerpOrigin[2usize];
    beam.origin[2usize] += 26i32 as libc::c_float;
    AngleVectors(
        cg_entities[(*ent).currentState.otherEntityNum as usize]
            .lerpAngles
            .as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        up.as_mut_ptr(),
    );
    beam.origin[0usize] = beam.origin[0usize] + up[0usize] * -6i32 as libc::c_float;
    beam.origin[1usize] = beam.origin[1usize] + up[1usize] * -6i32 as libc::c_float;
    beam.origin[2usize] = beam.origin[2usize] + up[2usize] * -6i32 as libc::c_float;
    beam.oldorigin[0usize] = origin[0usize];
    beam.oldorigin[1usize] = origin[1usize];
    beam.oldorigin[2usize] = origin[2usize];
    if Distance(
        beam.origin.as_mut_ptr() as *const vec_t,
        beam.oldorigin.as_mut_ptr() as *const vec_t,
    ) < 64i32 as libc::c_float
    {
        return;
    }
    beam.reType = RT_LIGHTNING;
    beam.customShader = cgs.media.lightningShader;
    AxisClear(beam.axis.as_mut_ptr());
    beam.shaderRGBA[0usize] = 0xffi32 as byte;
    beam.shaderRGBA[1usize] = 0xffi32 as byte;
    beam.shaderRGBA[2usize] = 0xffi32 as byte;
    beam.shaderRGBA[3usize] = 0xffi32 as byte;
    trap_R_AddRefEntityToScene(&mut beam);
}
#[no_mangle]
pub unsafe extern "C" fn CG_RegisterItemVisuals(mut itemNum: libc::c_int) {
    let mut itemInfo: *mut itemInfo_t = 0 as *mut itemInfo_t;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    if itemNum < 0i32 || itemNum >= bg_numItems {
        CG_Error(
            b"CG_RegisterItemVisuals: itemNum %d out of range [0-%d]\x00" as *const u8
                as *const libc::c_char,
            itemNum,
            bg_numItems - 1i32,
        );
    }
    itemInfo = &mut cg_items[itemNum as usize] as *mut itemInfo_t;
    if 0 != (*itemInfo).registered as u64 {
        return;
    }
    item = &mut *bg_itemlist.as_mut_ptr().offset(itemNum as isize) as *mut gitem_t;
    memset(
        itemInfo as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<itemInfo_t>() as libc::c_ulong,
    );
    (*itemInfo).registered = qtrue;
    (*itemInfo).models[0usize] = trap_R_RegisterModel((*item).world_model[0usize]);
    (*itemInfo).icon = trap_R_RegisterShader((*item).icon);
    if (*item).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint {
        CG_RegisterWeapon((*item).giTag);
    }
    if (*item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint
        || (*item).giType as libc::c_uint == IT_HEALTH as libc::c_int as libc::c_uint
        || (*item).giType as libc::c_uint == IT_ARMOR as libc::c_int as libc::c_uint
        || (*item).giType as libc::c_uint == IT_HOLDABLE as libc::c_int as libc::c_uint
    {
        if !(*item).world_model[1usize].is_null() {
            (*itemInfo).models[1usize] = trap_R_RegisterModel((*item).world_model[1usize])
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_FireWeapon(mut cent: *mut centity_t) {
    let mut ent: *mut entityState_t = 0 as *mut entityState_t;
    let mut c: libc::c_int = 0;
    let mut weap: *mut weaponInfo_t = 0 as *mut weaponInfo_t;
    ent = &mut (*cent).currentState;
    if (*ent).weapon == WP_NONE as libc::c_int {
        return;
    }
    if (*ent).weapon >= WP_NUM_WEAPONS as libc::c_int {
        CG_Error(
            b"CG_FireWeapon: ent->weapon >= WP_NUM_WEAPONS\x00" as *const u8 as *const libc::c_char,
        );
    }
    weap = &mut cg_weapons[(*ent).weapon as usize] as *mut weaponInfo_t;
    (*cent).muzzleFlashTime = cg.time;
    if (*ent).weapon == WP_LIGHTNING as libc::c_int {
        if 0 != (*cent).pe.lightningFiring {
            return;
        }
    }
    if (*ent).weapon == WP_RAILGUN as libc::c_int {
        (*cent).pe.railFireTime = cg.time
    }
    if 0 != (*cent).currentState.powerups & 1i32 << PW_QUAD as libc::c_int {
        trap_S_StartSound(
            0 as *mut vec_t,
            (*cent).currentState.number,
            CHAN_ITEM as libc::c_int,
            cgs.media.quadSound,
        );
    }
    c = 0i32;
    while c < 4i32 {
        if 0 == (*weap).flashSound[c as usize] {
            break;
        }
        c += 1
    }
    if c > 0i32 {
        c = rand() % c;
        if 0 != (*weap).flashSound[c as usize] {
            trap_S_StartSound(
                0 as *mut vec_t,
                (*ent).number,
                CHAN_WEAPON as libc::c_int,
                (*weap).flashSound[c as usize],
            );
        }
    }
    if (*weap).ejectBrassFunc.is_some() && cg_brassTime.integer > 0i32 {
        (*weap).ejectBrassFunc.expect("non-null function pointer")(cent);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_MissileHitWall(
    mut weapon: libc::c_int,
    mut clientNum: libc::c_int,
    mut origin: *mut vec_t,
    mut dir: *mut vec_t,
    mut soundType: impactSound_t,
) {
    let mut mod_0: qhandle_t = 0;
    let mut mark: qhandle_t = 0;
    let mut shader: qhandle_t = 0;
    let mut sfx: sfxHandle_t = 0;
    let mut radius: libc::c_float = 0.;
    let mut light: libc::c_float = 0.;
    let mut lightColor: vec3_t = [0.; 3];
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut r: libc::c_int = 0;
    let mut alphaFade: qboolean = qfalse;
    let mut isSprite: qboolean = qfalse;
    let mut duration: libc::c_int = 0;
    let mut sprOrg: vec3_t = [0.; 3];
    let mut sprVel: vec3_t = [0.; 3];
    mod_0 = 0i32;
    shader = 0i32;
    light = 0i32 as libc::c_float;
    lightColor[0usize] = 1i32 as vec_t;
    lightColor[1usize] = 1i32 as vec_t;
    lightColor[2usize] = 0i32 as vec_t;
    isSprite = qfalse;
    duration = 600i32;
    match weapon {
        4 => {
            mod_0 = cgs.media.dishFlashModel;
            shader = cgs.media.grenadeExplosionShader;
            sfx = cgs.media.sfx_rockexp;
            mark = cgs.media.burnMarkShader;
            radius = 64i32 as libc::c_float;
            light = 300i32 as libc::c_float;
            isSprite = qtrue
        }
        5 => {
            mod_0 = cgs.media.dishFlashModel;
            shader = cgs.media.rocketExplosionShader;
            sfx = cgs.media.sfx_rockexp;
            mark = cgs.media.burnMarkShader;
            radius = 64i32 as libc::c_float;
            light = 300i32 as libc::c_float;
            isSprite = qtrue;
            duration = 1000i32;
            lightColor[0usize] = 1i32 as vec_t;
            lightColor[1usize] = 0.75f64 as vec_t;
            lightColor[2usize] = 0.0f64 as vec_t;
            if cg_oldRocket.integer == 0i32 {
                sprOrg[0usize] =
                    *origin.offset(0isize) + *dir.offset(0isize) * 24i32 as libc::c_float;
                sprOrg[1usize] =
                    *origin.offset(1isize) + *dir.offset(1isize) * 24i32 as libc::c_float;
                sprOrg[2usize] =
                    *origin.offset(2isize) + *dir.offset(2isize) * 24i32 as libc::c_float;
                sprVel[0usize] = *dir.offset(0isize) * 64i32 as libc::c_float;
                sprVel[1usize] = *dir.offset(1isize) * 64i32 as libc::c_float;
                sprVel[2usize] = *dir.offset(2isize) * 64i32 as libc::c_float;
                CG_ParticleExplosion(
                    b"explode1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    sprOrg.as_mut_ptr(),
                    sprVel.as_mut_ptr(),
                    1400i32,
                    20i32,
                    30i32,
                );
            }
        }
        7 => {
            mod_0 = cgs.media.ringFlashModel;
            shader = cgs.media.railExplosionShader;
            sfx = cgs.media.sfx_plasmaexp;
            mark = cgs.media.energyMarkShader;
            radius = 24i32 as libc::c_float
        }
        8 => {
            mod_0 = cgs.media.ringFlashModel;
            shader = cgs.media.plasmaExplosionShader;
            sfx = cgs.media.sfx_plasmaexp;
            mark = cgs.media.energyMarkShader;
            radius = 16i32 as libc::c_float
        }
        9 => {
            mod_0 = cgs.media.dishFlashModel;
            shader = cgs.media.bfgExplosionShader;
            sfx = cgs.media.sfx_rockexp;
            mark = cgs.media.burnMarkShader;
            radius = 32i32 as libc::c_float;
            isSprite = qtrue
        }
        3 => {
            mod_0 = cgs.media.bulletFlashModel;
            shader = cgs.media.bulletExplosionShader;
            mark = cgs.media.bulletMarkShader;
            sfx = 0i32;
            radius = 4i32 as libc::c_float
        }
        2 => {
            mod_0 = cgs.media.bulletFlashModel;
            shader = cgs.media.bulletExplosionShader;
            mark = cgs.media.bulletMarkShader;
            r = rand() & 3i32;
            if r == 0i32 {
                sfx = cgs.media.sfx_ric1
            } else if r == 1i32 {
                sfx = cgs.media.sfx_ric2
            } else {
                sfx = cgs.media.sfx_ric3
            }
            radius = 8i32 as libc::c_float
        }
        6 | _ => {
            r = rand() & 3i32;
            if r < 2i32 {
                sfx = cgs.media.sfx_lghit2
            } else if r == 2i32 {
                sfx = cgs.media.sfx_lghit1
            } else {
                sfx = cgs.media.sfx_lghit3
            }
            mark = cgs.media.holeMarkShader;
            radius = 12i32 as libc::c_float
        }
    }
    if 0 != sfx {
        trap_S_StartSound(
            origin,
            (1i32 << 10i32) - 2i32,
            CHAN_AUTO as libc::c_int,
            sfx,
        );
    }
    if 0 != mod_0 {
        le = CG_MakeExplosion(origin, dir, mod_0, shader, duration, isSprite);
        (*le).light = light;
        (*le).lightColor[0usize] = lightColor[0usize];
        (*le).lightColor[1usize] = lightColor[1usize];
        (*le).lightColor[2usize] = lightColor[2usize];
        if weapon == WP_RAILGUN as libc::c_int {
            (*le).color[0usize] = cgs.clientinfo[clientNum as usize].color1[0usize];
            (*le).color[1usize] = cgs.clientinfo[clientNum as usize].color1[1usize];
            (*le).color[2usize] = cgs.clientinfo[clientNum as usize].color1[2usize];
            (*le).refEntity.shaderRGBA[0usize] =
                ((*le).color[0usize] * 0xffi32 as libc::c_float) as byte;
            (*le).refEntity.shaderRGBA[1usize] =
                ((*le).color[1usize] * 0xffi32 as libc::c_float) as byte;
            (*le).refEntity.shaderRGBA[2usize] =
                ((*le).color[2usize] * 0xffi32 as libc::c_float) as byte;
            (*le).refEntity.shaderRGBA[3usize] = 0xffi32 as byte
        }
    }
    alphaFade = (mark == cgs.media.energyMarkShader) as libc::c_int as qboolean;
    if weapon == WP_RAILGUN as libc::c_int {
        let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
        color = cgs.clientinfo[clientNum as usize].color1.as_mut_ptr();
        CG_ImpactMark(
            mark,
            origin as *const vec_t,
            dir as *const vec_t,
            (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                * 360i32 as libc::c_float,
            *color.offset(0isize),
            *color.offset(1isize),
            *color.offset(2isize),
            1i32 as libc::c_float,
            alphaFade,
            radius,
            qfalse,
        );
    } else {
        CG_ImpactMark(
            mark,
            origin as *const vec_t,
            dir as *const vec_t,
            (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
                * 360i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            1i32 as libc::c_float,
            alphaFade,
            radius,
            qfalse,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_MissileHitPlayer(
    mut weapon: libc::c_int,
    mut origin: *mut vec_t,
    mut dir: *mut vec_t,
    mut entityNum: libc::c_int,
) {
    CG_Bleed(origin, entityNum);
    match weapon {
        4 | 5 | 8 | 9 => {
            CG_MissileHitWall(weapon, 0i32, origin, dir, IMPACTSOUND_FLESH);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_ShotgunFire(mut es: *mut entityState_t) {
    let mut v: vec3_t = [0.; 3];
    let mut contents: libc::c_int = 0;
    v[0usize] = (*es).origin2[0usize] - (*es).pos.trBase[0usize];
    v[1usize] = (*es).origin2[1usize] - (*es).pos.trBase[1usize];
    v[2usize] = (*es).origin2[2usize] - (*es).pos.trBase[2usize];
    VectorNormalize(v.as_mut_ptr());
    v[0usize] = v[0usize] * 32i32 as libc::c_float;
    v[1usize] = v[1usize] * 32i32 as libc::c_float;
    v[2usize] = v[2usize] * 32i32 as libc::c_float;
    v[0usize] = (*es).pos.trBase[0usize] + v[0usize];
    v[1usize] = (*es).pos.trBase[1usize] + v[1usize];
    v[2usize] = (*es).pos.trBase[2usize] + v[2usize];
    if cgs.glconfig.hardwareType as libc::c_uint != GLHW_RAGEPRO as libc::c_int as libc::c_uint {
        let mut up: vec3_t = [0.; 3];
        contents = CG_PointContents((*es).pos.trBase.as_mut_ptr() as *const vec_t, 0i32);
        if 0 == contents & 32i32 {
            up[0usize] = 0i32 as vec_t;
            up[1usize] = 0i32 as vec_t;
            up[2usize] = 8i32 as vec_t;
            CG_SmokePuff(
                v.as_mut_ptr() as *const vec_t,
                up.as_mut_ptr() as *const vec_t,
                32i32 as libc::c_float,
                1i32 as libc::c_float,
                1i32 as libc::c_float,
                1i32 as libc::c_float,
                0.33f32,
                900i32 as libc::c_float,
                cg.time,
                0i32,
                LEF_PUFF_DONT_SCALE as libc::c_int,
                cgs.media.shotgunSmokePuffShader,
            );
        }
    }
    CG_ShotgunPattern(
        (*es).pos.trBase.as_mut_ptr(),
        (*es).origin2.as_mut_ptr(),
        (*es).eventParm,
        (*es).otherEntityNum,
    );
}
/*
================
CG_ShotgunPattern

Perform the same traces the server did to locate the
hit splashes
================
*/
unsafe extern "C" fn CG_ShotgunPattern(
    mut origin: *mut vec_t,
    mut origin2: *mut vec_t,
    mut seed: libc::c_int,
    mut otherEntNum: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut end: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    VectorNormalize2(origin2 as *const vec_t, forward.as_mut_ptr());
    PerpendicularVector(right.as_mut_ptr(), forward.as_mut_ptr() as *const vec_t);
    CrossProduct(
        forward.as_mut_ptr() as *const vec_t,
        right.as_mut_ptr() as *const vec_t,
        up.as_mut_ptr(),
    );
    i = 0i32;
    while i < 11i32 {
        r = Q_crandom(&mut seed) * 700i32 as libc::c_float * 16i32 as libc::c_float;
        u = Q_crandom(&mut seed) * 700i32 as libc::c_float * 16i32 as libc::c_float;
        end[0usize] = *origin.offset(0isize) + forward[0usize] * (8192i32 * 16i32) as libc::c_float;
        end[1usize] = *origin.offset(1isize) + forward[1usize] * (8192i32 * 16i32) as libc::c_float;
        end[2usize] = *origin.offset(2isize) + forward[2usize] * (8192i32 * 16i32) as libc::c_float;
        end[0usize] = end[0usize] + right[0usize] * r;
        end[1usize] = end[1usize] + right[1usize] * r;
        end[2usize] = end[2usize] + right[2usize] * r;
        end[0usize] = end[0usize] + up[0usize] * u;
        end[1usize] = end[1usize] + up[1usize] * u;
        end[2usize] = end[2usize] + up[2usize] * u;
        CG_ShotgunPellet(origin, end.as_mut_ptr(), otherEntNum);
        i += 1
    }
}
/*
============================================================================

SHOTGUN TRACING

============================================================================
*/
/*
================
CG_ShotgunPellet
================
*/
unsafe extern "C" fn CG_ShotgunPellet(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut skipNum: libc::c_int,
) {
    let mut tr: trace_t = trace_t {
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
    let mut sourceContentType: libc::c_int = 0;
    let mut destContentType: libc::c_int = 0;
    CG_Trace(
        &mut tr,
        start as *const vec_t,
        0 as *const vec_t,
        0 as *const vec_t,
        end as *const vec_t,
        skipNum,
        1i32 | 0x2000000i32 | 0x4000000i32,
    );
    sourceContentType = CG_PointContents(start as *const vec_t, 0i32);
    destContentType = CG_PointContents(tr.endpos.as_mut_ptr() as *const vec_t, 0i32);
    if sourceContentType == destContentType {
        if 0 != sourceContentType & 32i32 {
            CG_BubbleTrail(start, tr.endpos.as_mut_ptr(), 32i32 as libc::c_float);
        }
    } else if 0 != sourceContentType & 32i32 {
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
        trap_CM_BoxTrace(
            &mut trace,
            end as *const vec_t,
            start as *const vec_t,
            0 as *const vec_t,
            0 as *const vec_t,
            0i32,
            32i32,
        );
        CG_BubbleTrail(start, trace.endpos.as_mut_ptr(), 32i32 as libc::c_float);
    } else if 0 != destContentType & 32i32 {
        let mut trace_0: trace_t = trace_t {
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
        trap_CM_BoxTrace(
            &mut trace_0,
            start as *const vec_t,
            end as *const vec_t,
            0 as *const vec_t,
            0 as *const vec_t,
            0i32,
            32i32,
        );
        CG_BubbleTrail(
            tr.endpos.as_mut_ptr(),
            trace_0.endpos.as_mut_ptr(),
            32i32 as libc::c_float,
        );
    }
    if 0 != tr.surfaceFlags & 0x10i32 {
        return;
    }
    if cg_entities[tr.entityNum as usize].currentState.eType == ET_PLAYER as libc::c_int {
        CG_MissileHitPlayer(
            WP_SHOTGUN as libc::c_int,
            tr.endpos.as_mut_ptr(),
            tr.plane.normal.as_mut_ptr(),
            tr.entityNum,
        );
    } else {
        if 0 != tr.surfaceFlags & 0x10i32 {
            return;
        }
        if 0 != tr.surfaceFlags & 0x1000i32 {
            CG_MissileHitWall(
                WP_SHOTGUN as libc::c_int,
                0i32,
                tr.endpos.as_mut_ptr(),
                tr.plane.normal.as_mut_ptr(),
                IMPACTSOUND_METAL,
            );
        } else {
            CG_MissileHitWall(
                WP_SHOTGUN as libc::c_int,
                0i32,
                tr.endpos.as_mut_ptr(),
                tr.plane.normal.as_mut_ptr(),
                IMPACTSOUND_DEFAULT,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_Bullet(
    mut end: *mut vec_t,
    mut sourceEntityNum: libc::c_int,
    mut normal: *mut vec_t,
    mut flesh: qboolean,
    mut fleshEntityNum: libc::c_int,
) {
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
    let mut sourceContentType: libc::c_int = 0;
    let mut destContentType: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    if sourceEntityNum >= 0i32 && cg_tracerChance.value > 0i32 as libc::c_float {
        if 0 != CG_CalcMuzzlePoint(sourceEntityNum, start.as_mut_ptr()) as u64 {
            sourceContentType = CG_PointContents(start.as_mut_ptr() as *const vec_t, 0i32);
            destContentType = CG_PointContents(end as *const vec_t, 0i32);
            if sourceContentType == destContentType && 0 != sourceContentType & 32i32 {
                CG_BubbleTrail(start.as_mut_ptr(), end, 32i32 as libc::c_float);
            } else if 0 != sourceContentType & 32i32 {
                trap_CM_BoxTrace(
                    &mut trace,
                    end as *const vec_t,
                    start.as_mut_ptr() as *const vec_t,
                    0 as *const vec_t,
                    0 as *const vec_t,
                    0i32,
                    32i32,
                );
                CG_BubbleTrail(
                    start.as_mut_ptr(),
                    trace.endpos.as_mut_ptr(),
                    32i32 as libc::c_float,
                );
            } else if 0 != destContentType & 32i32 {
                trap_CM_BoxTrace(
                    &mut trace,
                    start.as_mut_ptr() as *const vec_t,
                    end as *const vec_t,
                    0 as *const vec_t,
                    0 as *const vec_t,
                    0i32,
                    32i32,
                );
                CG_BubbleTrail(trace.endpos.as_mut_ptr(), end, 32i32 as libc::c_float);
            }
            if ((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                < cg_tracerChance.value
            {
                CG_Tracer(start.as_mut_ptr(), end);
            }
        }
    }
    if 0 != flesh as u64 {
        CG_Bleed(end, fleshEntityNum);
    } else {
        CG_MissileHitWall(
            WP_MACHINEGUN as libc::c_int,
            0i32,
            end,
            normal,
            IMPACTSOUND_DEFAULT,
        );
    };
}
/*
============================================================================

BULLETS

============================================================================
*/
/*
===============
CG_Tracer
===============
*/
#[no_mangle]
pub unsafe extern "C" fn CG_Tracer(mut source: *mut vec_t, mut dest: *mut vec_t) {
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut verts: [polyVert_t; 4] = [polyVert_t {
        xyz: [0.; 3],
        st: [0.; 2],
        modulate: [0; 4],
    }; 4];
    let mut line: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut begin: libc::c_float = 0.;
    let mut end: libc::c_float = 0.;
    let mut start: vec3_t = [0.; 3];
    let mut finish: vec3_t = [0.; 3];
    let mut midpoint: vec3_t = [0.; 3];
    forward[0usize] = *dest.offset(0isize) - *source.offset(0isize);
    forward[1usize] = *dest.offset(1isize) - *source.offset(1isize);
    forward[2usize] = *dest.offset(2isize) - *source.offset(2isize);
    len = VectorNormalize(forward.as_mut_ptr());
    if len < 100i32 as libc::c_float {
        return;
    }
    begin = 50i32 as libc::c_float
        + (rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float
            * (len - 60i32 as libc::c_float);
    end = begin + cg_tracerLength.value;
    if end > len {
        end = len
    }
    start[0usize] = *source.offset(0isize) + forward[0usize] * begin;
    start[1usize] = *source.offset(1isize) + forward[1usize] * begin;
    start[2usize] = *source.offset(2isize) + forward[2usize] * begin;
    finish[0usize] = *source.offset(0isize) + forward[0usize] * end;
    finish[1usize] = *source.offset(1isize) + forward[1usize] * end;
    finish[2usize] = *source.offset(2isize) + forward[2usize] * end;
    line[0usize] = forward[0usize] * cg.refdef.viewaxis[1usize][0usize]
        + forward[1usize] * cg.refdef.viewaxis[1usize][1usize]
        + forward[2usize] * cg.refdef.viewaxis[1usize][2usize];
    line[1usize] = forward[0usize] * cg.refdef.viewaxis[2usize][0usize]
        + forward[1usize] * cg.refdef.viewaxis[2usize][1usize]
        + forward[2usize] * cg.refdef.viewaxis[2usize][2usize];
    right[0usize] = cg.refdef.viewaxis[1usize][0usize] * line[1usize];
    right[1usize] = cg.refdef.viewaxis[1usize][1usize] * line[1usize];
    right[2usize] = cg.refdef.viewaxis[1usize][2usize] * line[1usize];
    right[0usize] = right[0usize] + cg.refdef.viewaxis[2usize][0usize] * -line[0usize];
    right[1usize] = right[1usize] + cg.refdef.viewaxis[2usize][1usize] * -line[0usize];
    right[2usize] = right[2usize] + cg.refdef.viewaxis[2usize][2usize] * -line[0usize];
    VectorNormalize(right.as_mut_ptr());
    verts[0usize].xyz[0usize] = finish[0usize] + right[0usize] * cg_tracerWidth.value;
    verts[0usize].xyz[1usize] = finish[1usize] + right[1usize] * cg_tracerWidth.value;
    verts[0usize].xyz[2usize] = finish[2usize] + right[2usize] * cg_tracerWidth.value;
    verts[0usize].st[0usize] = 0i32 as libc::c_float;
    verts[0usize].st[1usize] = 1i32 as libc::c_float;
    verts[0usize].modulate[0usize] = 255i32 as byte;
    verts[0usize].modulate[1usize] = 255i32 as byte;
    verts[0usize].modulate[2usize] = 255i32 as byte;
    verts[0usize].modulate[3usize] = 255i32 as byte;
    verts[1usize].xyz[0usize] = finish[0usize] + right[0usize] * -cg_tracerWidth.value;
    verts[1usize].xyz[1usize] = finish[1usize] + right[1usize] * -cg_tracerWidth.value;
    verts[1usize].xyz[2usize] = finish[2usize] + right[2usize] * -cg_tracerWidth.value;
    verts[1usize].st[0usize] = 1i32 as libc::c_float;
    verts[1usize].st[1usize] = 0i32 as libc::c_float;
    verts[1usize].modulate[0usize] = 255i32 as byte;
    verts[1usize].modulate[1usize] = 255i32 as byte;
    verts[1usize].modulate[2usize] = 255i32 as byte;
    verts[1usize].modulate[3usize] = 255i32 as byte;
    verts[2usize].xyz[0usize] = start[0usize] + right[0usize] * -cg_tracerWidth.value;
    verts[2usize].xyz[1usize] = start[1usize] + right[1usize] * -cg_tracerWidth.value;
    verts[2usize].xyz[2usize] = start[2usize] + right[2usize] * -cg_tracerWidth.value;
    verts[2usize].st[0usize] = 1i32 as libc::c_float;
    verts[2usize].st[1usize] = 1i32 as libc::c_float;
    verts[2usize].modulate[0usize] = 255i32 as byte;
    verts[2usize].modulate[1usize] = 255i32 as byte;
    verts[2usize].modulate[2usize] = 255i32 as byte;
    verts[2usize].modulate[3usize] = 255i32 as byte;
    verts[3usize].xyz[0usize] = start[0usize] + right[0usize] * cg_tracerWidth.value;
    verts[3usize].xyz[1usize] = start[1usize] + right[1usize] * cg_tracerWidth.value;
    verts[3usize].xyz[2usize] = start[2usize] + right[2usize] * cg_tracerWidth.value;
    verts[3usize].st[0usize] = 0i32 as libc::c_float;
    verts[3usize].st[1usize] = 0i32 as libc::c_float;
    verts[3usize].modulate[0usize] = 255i32 as byte;
    verts[3usize].modulate[1usize] = 255i32 as byte;
    verts[3usize].modulate[2usize] = 255i32 as byte;
    verts[3usize].modulate[3usize] = 255i32 as byte;
    trap_R_AddPolyToScene(cgs.media.tracerShader, 4i32, verts.as_mut_ptr());
    midpoint[0usize] = ((start[0usize] + finish[0usize]) as libc::c_double * 0.5f64) as vec_t;
    midpoint[1usize] = ((start[1usize] + finish[1usize]) as libc::c_double * 0.5f64) as vec_t;
    midpoint[2usize] = ((start[2usize] + finish[2usize]) as libc::c_double * 0.5f64) as vec_t;
    trap_S_StartSound(
        midpoint.as_mut_ptr(),
        (1i32 << 10i32) - 2i32,
        CHAN_AUTO as libc::c_int,
        cgs.media.tracerSound,
    );
}
/*
======================
CG_CalcMuzzlePoint
======================
*/
unsafe extern "C" fn CG_CalcMuzzlePoint(
    mut entityNum: libc::c_int,
    mut muzzle: *mut vec_t,
) -> qboolean {
    let mut forward: vec3_t = [0.; 3];
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut anim: libc::c_int = 0;
    if entityNum == (*cg.snap).ps.clientNum {
        *muzzle.offset(0isize) = (*cg.snap).ps.origin[0usize];
        *muzzle.offset(1isize) = (*cg.snap).ps.origin[1usize];
        *muzzle.offset(2isize) = (*cg.snap).ps.origin[2usize];
        let ref mut fresh0 = *muzzle.offset(2isize);
        *fresh0 += (*cg.snap).ps.viewheight as libc::c_float;
        AngleVectors(
            (*cg.snap).ps.viewangles.as_mut_ptr() as *const vec_t,
            forward.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        *muzzle.offset(0isize) = *muzzle.offset(0isize) + forward[0usize] * 14i32 as libc::c_float;
        *muzzle.offset(1isize) = *muzzle.offset(1isize) + forward[1usize] * 14i32 as libc::c_float;
        *muzzle.offset(2isize) = *muzzle.offset(2isize) + forward[2usize] * 14i32 as libc::c_float;
        return qtrue;
    }
    cent = &mut cg_entities[entityNum as usize] as *mut centity_t;
    if 0 == (*cent).currentValid as u64 {
        return qfalse;
    }
    *muzzle.offset(0isize) = (*cent).currentState.pos.trBase[0usize];
    *muzzle.offset(1isize) = (*cent).currentState.pos.trBase[1usize];
    *muzzle.offset(2isize) = (*cent).currentState.pos.trBase[2usize];
    AngleVectors(
        (*cent).currentState.apos.trBase.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    anim = (*cent).currentState.legsAnim & !128i32;
    if anim == LEGS_WALKCR as libc::c_int || anim == LEGS_IDLECR as libc::c_int {
        let ref mut fresh1 = *muzzle.offset(2isize);
        *fresh1 += 12i32 as libc::c_float
    } else {
        let ref mut fresh2 = *muzzle.offset(2isize);
        *fresh2 += 26i32 as libc::c_float
    }
    *muzzle.offset(0isize) = *muzzle.offset(0isize) + forward[0usize] * 14i32 as libc::c_float;
    *muzzle.offset(1isize) = *muzzle.offset(1isize) + forward[1usize] * 14i32 as libc::c_float;
    *muzzle.offset(2isize) = *muzzle.offset(2isize) + forward[2usize] * 14i32 as libc::c_float;
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CG_RailTrail(
    mut ci: *mut clientInfo_t,
    mut start: *mut vec_t,
    mut end: *mut vec_t,
) {
    let mut axis: [vec3_t; 36] = [[0.; 3]; 36];
    let mut move_0: vec3_t = [0.; 3];
    let mut move2: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    let mut le: *mut localEntity_t = 0 as *mut localEntity_t;
    let mut re: *mut refEntity_t = 0 as *mut refEntity_t;
    let ref mut fresh3 = *start.offset(2isize);
    *fresh3 -= 4i32 as libc::c_float;
    le = CG_AllocLocalEntity();
    re = &mut (*le).refEntity;
    (*le).leType = LE_FADE_RGB;
    (*le).startTime = cg.time;
    (*le).endTime = (cg.time as libc::c_float + cg_railTrailTime.value) as libc::c_int;
    (*le).lifeRate =
        (1.0f64 / ((*le).endTime - (*le).startTime) as libc::c_double) as libc::c_float;
    (*re).shaderTime = cg.time as libc::c_float / 1000.0f32;
    (*re).reType = RT_RAIL_CORE;
    (*re).customShader = cgs.media.railCoreShader;
    (*re).origin[0usize] = *start.offset(0isize);
    (*re).origin[1usize] = *start.offset(1isize);
    (*re).origin[2usize] = *start.offset(2isize);
    (*re).oldorigin[0usize] = *end.offset(0isize);
    (*re).oldorigin[1usize] = *end.offset(1isize);
    (*re).oldorigin[2usize] = *end.offset(2isize);
    (*re).shaderRGBA[0usize] = ((*ci).color1[0usize] * 255i32 as libc::c_float) as byte;
    (*re).shaderRGBA[1usize] = ((*ci).color1[1usize] * 255i32 as libc::c_float) as byte;
    (*re).shaderRGBA[2usize] = ((*ci).color1[2usize] * 255i32 as libc::c_float) as byte;
    (*re).shaderRGBA[3usize] = 255i32 as byte;
    (*le).color[0usize] = ((*ci).color1[0usize] as libc::c_double * 0.75f64) as libc::c_float;
    (*le).color[1usize] = ((*ci).color1[1usize] as libc::c_double * 0.75f64) as libc::c_float;
    (*le).color[2usize] = ((*ci).color1[2usize] as libc::c_double * 0.75f64) as libc::c_float;
    (*le).color[3usize] = 1.0f32;
    AxisClear((*re).axis.as_mut_ptr());
    if 0 != cg_oldRail.integer {
        (*re).origin[2usize] -= 8i32 as libc::c_float;
        (*re).oldorigin[2usize] -= 8i32 as libc::c_float;
        return;
    }
    move_0[0usize] = *start.offset(0isize);
    move_0[1usize] = *start.offset(1isize);
    move_0[2usize] = *start.offset(2isize);
    vec[0usize] = *end.offset(0isize) - *start.offset(0isize);
    vec[1usize] = *end.offset(1isize) - *start.offset(1isize);
    vec[2usize] = *end.offset(2isize) - *start.offset(2isize);
    len = VectorNormalize(vec.as_mut_ptr());
    PerpendicularVector(temp.as_mut_ptr(), vec.as_mut_ptr() as *const vec_t);
    i = 0i32;
    while i < 36i32 {
        RotatePointAroundVector(
            axis[i as usize].as_mut_ptr(),
            vec.as_mut_ptr() as *const vec_t,
            temp.as_mut_ptr() as *const vec_t,
            (i * 10i32) as libc::c_float,
        );
        i += 1
    }
    move_0[0usize] = move_0[0usize] + vec[0usize] * 20i32 as libc::c_float;
    move_0[1usize] = move_0[1usize] + vec[1usize] * 20i32 as libc::c_float;
    move_0[2usize] = move_0[2usize] + vec[2usize] * 20i32 as libc::c_float;
    vec[0usize] = vec[0usize] * 5i32 as libc::c_float;
    vec[1usize] = vec[1usize] * 5i32 as libc::c_float;
    vec[2usize] = vec[2usize] * 5i32 as libc::c_float;
    skip = -1i32;
    j = 18i32;
    i = 0i32;
    while (i as libc::c_float) < len {
        if i != skip {
            skip = i + 5i32;
            le = CG_AllocLocalEntity();
            re = &mut (*le).refEntity;
            (*le).leFlags = LEF_PUFF_DONT_SCALE as libc::c_int;
            (*le).leType = LE_MOVE_SCALE_FADE;
            (*le).startTime = cg.time;
            (*le).endTime = cg.time + (i >> 1i32) + 600i32;
            (*le).lifeRate =
                (1.0f64 / ((*le).endTime - (*le).startTime) as libc::c_double) as libc::c_float;
            (*re).shaderTime = cg.time as libc::c_float / 1000.0f32;
            (*re).reType = RT_SPRITE;
            (*re).radius = 1.1f32;
            (*re).customShader = cgs.media.railRingsShader;
            (*re).shaderRGBA[0usize] = ((*ci).color2[0usize] * 255i32 as libc::c_float) as byte;
            (*re).shaderRGBA[1usize] = ((*ci).color2[1usize] * 255i32 as libc::c_float) as byte;
            (*re).shaderRGBA[2usize] = ((*ci).color2[2usize] * 255i32 as libc::c_float) as byte;
            (*re).shaderRGBA[3usize] = 255i32 as byte;
            (*le).color[0usize] =
                ((*ci).color2[0usize] as libc::c_double * 0.75f64) as libc::c_float;
            (*le).color[1usize] =
                ((*ci).color2[1usize] as libc::c_double * 0.75f64) as libc::c_float;
            (*le).color[2usize] =
                ((*ci).color2[2usize] as libc::c_double * 0.75f64) as libc::c_float;
            (*le).color[3usize] = 1.0f32;
            (*le).pos.trType = TR_LINEAR;
            (*le).pos.trTime = cg.time;
            move2[0usize] = move_0[0usize];
            move2[1usize] = move_0[1usize];
            move2[2usize] = move_0[2usize];
            move2[0usize] = move2[0usize] + axis[j as usize][0usize] * 4i32 as libc::c_float;
            move2[1usize] = move2[1usize] + axis[j as usize][1usize] * 4i32 as libc::c_float;
            move2[2usize] = move2[2usize] + axis[j as usize][2usize] * 4i32 as libc::c_float;
            (*le).pos.trBase[0usize] = move2[0usize];
            (*le).pos.trBase[1usize] = move2[1usize];
            (*le).pos.trBase[2usize] = move2[2usize];
            (*le).pos.trDelta[0usize] = axis[j as usize][0usize] * 6i32 as libc::c_float;
            (*le).pos.trDelta[1usize] = axis[j as usize][1usize] * 6i32 as libc::c_float;
            (*le).pos.trDelta[2usize] = axis[j as usize][2usize] * 6i32 as libc::c_float
        }
        move_0[0usize] = move_0[0usize] + vec[0usize];
        move_0[1usize] = move_0[1usize] + vec[1usize];
        move_0[2usize] = move_0[2usize] + vec[2usize];
        j = (j + 1i32) % 36i32;
        i += 5i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn CG_AddViewWeapon(mut ps: *mut playerState_t) {
    let mut hand: refEntity_t = refEntity_t {
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
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut ci: *mut clientInfo_t = 0 as *mut clientInfo_t;
    let mut fovOffset: libc::c_float = 0.;
    let mut angles: vec3_t = [0.; 3];
    let mut weapon: *mut weaponInfo_t = 0 as *mut weaponInfo_t;
    if (*ps).persistant[PERS_TEAM as libc::c_int as usize] == TEAM_SPECTATOR as libc::c_int {
        return;
    }
    if (*ps).pm_type == PM_INTERMISSION as libc::c_int {
        return;
    }
    if 0 != cg.renderingThirdPerson as u64 {
        return;
    }
    if 0 == cg_drawGun.integer {
        let mut origin: vec3_t = [0.; 3];
        if 0 != cg.predictedPlayerState.eFlags & 0x100i32 {
            origin[0usize] = cg.refdef.vieworg[0usize];
            origin[1usize] = cg.refdef.vieworg[1usize];
            origin[2usize] = cg.refdef.vieworg[2usize];
            origin[0usize] =
                origin[0usize] + cg.refdef.viewaxis[2usize][0usize] * -8i32 as libc::c_float;
            origin[1usize] =
                origin[1usize] + cg.refdef.viewaxis[2usize][1usize] * -8i32 as libc::c_float;
            origin[2usize] =
                origin[2usize] + cg.refdef.viewaxis[2usize][2usize] * -8i32 as libc::c_float;
            CG_LightningBolt(
                &mut cg_entities[(*ps).clientNum as usize],
                origin.as_mut_ptr(),
            );
        }
        return;
    }
    if 0 != cg.testGun as u64 {
        return;
    }
    if cg_fov.integer > 90i32 {
        fovOffset = (-0.2f64 * (cg_fov.integer - 90i32) as libc::c_double) as libc::c_float
    } else {
        fovOffset = 0i32 as libc::c_float
    }
    cent = &mut cg.predictedPlayerEntity;
    CG_RegisterWeapon((*ps).weapon);
    weapon = &mut cg_weapons[(*ps).weapon as usize] as *mut weaponInfo_t;
    memset(
        &mut hand as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    CG_CalculateWeaponPosition(hand.origin.as_mut_ptr(), angles.as_mut_ptr());
    hand.origin[0usize] = hand.origin[0usize] + cg.refdef.viewaxis[0usize][0usize] * cg_gun_x.value;
    hand.origin[1usize] = hand.origin[1usize] + cg.refdef.viewaxis[0usize][1usize] * cg_gun_x.value;
    hand.origin[2usize] = hand.origin[2usize] + cg.refdef.viewaxis[0usize][2usize] * cg_gun_x.value;
    hand.origin[0usize] = hand.origin[0usize] + cg.refdef.viewaxis[1usize][0usize] * cg_gun_y.value;
    hand.origin[1usize] = hand.origin[1usize] + cg.refdef.viewaxis[1usize][1usize] * cg_gun_y.value;
    hand.origin[2usize] = hand.origin[2usize] + cg.refdef.viewaxis[1usize][2usize] * cg_gun_y.value;
    hand.origin[0usize] =
        hand.origin[0usize] + cg.refdef.viewaxis[2usize][0usize] * (cg_gun_z.value + fovOffset);
    hand.origin[1usize] =
        hand.origin[1usize] + cg.refdef.viewaxis[2usize][1usize] * (cg_gun_z.value + fovOffset);
    hand.origin[2usize] =
        hand.origin[2usize] + cg.refdef.viewaxis[2usize][2usize] * (cg_gun_z.value + fovOffset);
    AnglesToAxis(angles.as_mut_ptr() as *const vec_t, hand.axis.as_mut_ptr());
    if 0 != cg_gun_frame.integer {
        hand.oldframe = cg_gun_frame.integer;
        hand.frame = hand.oldframe;
        hand.backlerp = 0i32 as libc::c_float
    } else {
        ci = &mut cgs.clientinfo[(*cent).currentState.clientNum as usize] as *mut clientInfo_t;
        hand.frame = CG_MapTorsoToWeaponFrame(ci, (*cent).pe.torso.frame);
        hand.oldframe = CG_MapTorsoToWeaponFrame(ci, (*cent).pe.torso.oldFrame);
        hand.backlerp = (*cent).pe.torso.backlerp
    }
    hand.hModel = (*weapon).handsModel;
    hand.renderfx = 0x8i32 | 0x4i32 | 0x1i32;
    CG_AddPlayerWeapon(
        &mut hand,
        ps,
        &mut cg.predictedPlayerEntity,
        (*ps).persistant[PERS_TEAM as libc::c_int as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_AddPlayerWeapon(
    mut parent: *mut refEntity_t,
    mut ps: *mut playerState_t,
    mut cent: *mut centity_t,
    mut team: libc::c_int,
) {
    let mut gun: refEntity_t = refEntity_t {
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
    let mut barrel: refEntity_t = refEntity_t {
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
    let mut flash: refEntity_t = refEntity_t {
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
    let mut angles: vec3_t = [0.; 3];
    let mut weaponNum: weapon_t = WP_NONE;
    let mut weapon: *mut weaponInfo_t = 0 as *mut weaponInfo_t;
    let mut nonPredictedCent: *mut centity_t = 0 as *mut centity_t;
    let mut lerped: orientation_t = orientation_t {
        origin: [0.; 3],
        axis: [[0.; 3]; 3],
    };
    weaponNum = (*cent).currentState.weapon as weapon_t;
    CG_RegisterWeapon(weaponNum as libc::c_int);
    weapon = &mut cg_weapons[weaponNum as usize] as *mut weaponInfo_t;
    memset(
        &mut gun as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    gun.lightingOrigin[0usize] = (*parent).lightingOrigin[0usize];
    gun.lightingOrigin[1usize] = (*parent).lightingOrigin[1usize];
    gun.lightingOrigin[2usize] = (*parent).lightingOrigin[2usize];
    gun.shadowPlane = (*parent).shadowPlane;
    gun.renderfx = (*parent).renderfx;
    if weaponNum as libc::c_uint == WP_RAILGUN as libc::c_int as libc::c_uint {
        let mut ci: *mut clientInfo_t =
            &mut cgs.clientinfo[(*cent).currentState.clientNum as usize] as *mut clientInfo_t;
        if (*cent).pe.railFireTime + 1500i32 > cg.time {
            let mut scale: libc::c_int = 255i32 * (cg.time - (*cent).pe.railFireTime) / 1500i32;
            gun.shaderRGBA[0usize] = ((*ci).c1RGBA[0usize] as libc::c_int * scale >> 8i32) as byte;
            gun.shaderRGBA[1usize] = ((*ci).c1RGBA[1usize] as libc::c_int * scale >> 8i32) as byte;
            gun.shaderRGBA[2usize] = ((*ci).c1RGBA[2usize] as libc::c_int * scale >> 8i32) as byte;
            gun.shaderRGBA[3usize] = 255i32 as byte
        } else {
            gun.shaderRGBA[0usize] = (*ci).c1RGBA[0usize];
            gun.shaderRGBA[1usize] = (*ci).c1RGBA[1usize];
            gun.shaderRGBA[2usize] = (*ci).c1RGBA[2usize];
            gun.shaderRGBA[3usize] = (*ci).c1RGBA[3usize]
        }
    }
    gun.hModel = (*weapon).weaponModel;
    if 0 == gun.hModel {
        return;
    }
    if ps.is_null() {
        (*cent).pe.lightningFiring = qfalse as libc::c_int;
        if 0 != (*cent).currentState.eFlags & 0x100i32 && 0 != (*weapon).firingSound {
            trap_S_AddLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
                vec3_origin.as_mut_ptr() as *const vec_t,
                (*weapon).firingSound,
            );
            (*cent).pe.lightningFiring = qtrue as libc::c_int
        } else if 0 != (*weapon).readySound {
            trap_S_AddLoopingSound(
                (*cent).currentState.number,
                (*cent).lerpOrigin.as_mut_ptr() as *const vec_t,
                vec3_origin.as_mut_ptr() as *const vec_t,
                (*weapon).readySound,
            );
        }
    }
    trap_R_LerpTag(
        &mut lerped,
        (*parent).hModel,
        (*parent).oldframe,
        (*parent).frame,
        (1.0f64 - (*parent).backlerp as libc::c_double) as libc::c_float,
        b"tag_weapon\x00" as *const u8 as *const libc::c_char,
    );
    gun.origin[0usize] = (*parent).origin[0usize];
    gun.origin[1usize] = (*parent).origin[1usize];
    gun.origin[2usize] = (*parent).origin[2usize];
    gun.origin[0usize] =
        gun.origin[0usize] + (*parent).axis[0usize][0usize] * lerped.origin[0usize];
    gun.origin[1usize] =
        gun.origin[1usize] + (*parent).axis[0usize][1usize] * lerped.origin[0usize];
    gun.origin[2usize] =
        gun.origin[2usize] + (*parent).axis[0usize][2usize] * lerped.origin[0usize];
    if !ps.is_null() && cg_drawGun.integer == 2i32 {
        gun.origin[0usize] =
            gun.origin[0usize] + (*parent).axis[1usize][0usize] * -lerped.origin[1usize];
        gun.origin[1usize] =
            gun.origin[1usize] + (*parent).axis[1usize][1usize] * -lerped.origin[1usize];
        gun.origin[2usize] =
            gun.origin[2usize] + (*parent).axis[1usize][2usize] * -lerped.origin[1usize]
    } else if ps.is_null() || cg_drawGun.integer != 3i32 {
        gun.origin[0usize] =
            gun.origin[0usize] + (*parent).axis[1usize][0usize] * lerped.origin[1usize];
        gun.origin[1usize] =
            gun.origin[1usize] + (*parent).axis[1usize][1usize] * lerped.origin[1usize];
        gun.origin[2usize] =
            gun.origin[2usize] + (*parent).axis[1usize][2usize] * lerped.origin[1usize]
    }
    gun.origin[0usize] =
        gun.origin[0usize] + (*parent).axis[2usize][0usize] * lerped.origin[2usize];
    gun.origin[1usize] =
        gun.origin[1usize] + (*parent).axis[2usize][1usize] * lerped.origin[2usize];
    gun.origin[2usize] =
        gun.origin[2usize] + (*parent).axis[2usize][2usize] * lerped.origin[2usize];
    MatrixMultiply(
        lerped.axis.as_mut_ptr(),
        (*parent).axis.as_mut_ptr(),
        gun.axis.as_mut_ptr(),
    );
    gun.backlerp = (*parent).backlerp;
    CG_AddWeaponWithPowerups(&mut gun, (*cent).currentState.powerups);
    if 0 != (*weapon).barrelModel {
        memset(
            &mut barrel as *mut refEntity_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
        );
        barrel.lightingOrigin[0usize] = (*parent).lightingOrigin[0usize];
        barrel.lightingOrigin[1usize] = (*parent).lightingOrigin[1usize];
        barrel.lightingOrigin[2usize] = (*parent).lightingOrigin[2usize];
        barrel.shadowPlane = (*parent).shadowPlane;
        barrel.renderfx = (*parent).renderfx;
        barrel.hModel = (*weapon).barrelModel;
        angles[1usize] = 0i32 as vec_t;
        angles[0usize] = 0i32 as vec_t;
        angles[2usize] = CG_MachinegunSpinAngle(cent);
        AnglesToAxis(
            angles.as_mut_ptr() as *const vec_t,
            barrel.axis.as_mut_ptr(),
        );
        CG_PositionRotatedEntityOnTag(
            &mut barrel,
            &mut gun,
            (*weapon).weaponModel,
            b"tag_barrel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        CG_AddWeaponWithPowerups(&mut barrel, (*cent).currentState.powerups);
    }
    nonPredictedCent = &mut cg_entities[(*cent).currentState.clientNum as usize] as *mut centity_t;
    if nonPredictedCent.wrapping_offset_from(cg_entities.as_mut_ptr()) as libc::c_long
        != (*cent).currentState.clientNum as libc::c_long
    {
        nonPredictedCent = cent
    }
    if !((weaponNum as libc::c_uint == WP_LIGHTNING as libc::c_int as libc::c_uint
        || weaponNum as libc::c_uint == WP_GAUNTLET as libc::c_int as libc::c_uint
        || weaponNum as libc::c_uint == WP_GRAPPLING_HOOK as libc::c_int as libc::c_uint)
        && 0 != (*nonPredictedCent).currentState.eFlags & 0x100i32)
    {
        if cg.time - (*cent).muzzleFlashTime > 20i32 {
            return;
        }
    }
    memset(
        &mut flash as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    flash.lightingOrigin[0usize] = (*parent).lightingOrigin[0usize];
    flash.lightingOrigin[1usize] = (*parent).lightingOrigin[1usize];
    flash.lightingOrigin[2usize] = (*parent).lightingOrigin[2usize];
    flash.shadowPlane = (*parent).shadowPlane;
    flash.renderfx = (*parent).renderfx;
    flash.hModel = (*weapon).flashModel;
    if 0 == flash.hModel {
        return;
    }
    angles[1usize] = 0i32 as vec_t;
    angles[0usize] = 0i32 as vec_t;
    angles[2usize] = (2.0f64
        * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float) as libc::c_double
            - 0.5f64)
        * 10i32 as libc::c_double) as vec_t;
    AnglesToAxis(angles.as_mut_ptr() as *const vec_t, flash.axis.as_mut_ptr());
    if weaponNum as libc::c_uint == WP_RAILGUN as libc::c_int as libc::c_uint {
        let mut ci_0: *mut clientInfo_t = 0 as *mut clientInfo_t;
        ci_0 = &mut cgs.clientinfo[(*cent).currentState.clientNum as usize] as *mut clientInfo_t;
        flash.shaderRGBA[0usize] = (255i32 as libc::c_float * (*ci_0).color1[0usize]) as byte;
        flash.shaderRGBA[1usize] = (255i32 as libc::c_float * (*ci_0).color1[1usize]) as byte;
        flash.shaderRGBA[2usize] = (255i32 as libc::c_float * (*ci_0).color1[2usize]) as byte
    }
    CG_PositionRotatedEntityOnTag(
        &mut flash,
        &mut gun,
        (*weapon).weaponModel,
        b"tag_flash\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    trap_R_AddRefEntityToScene(&mut flash);
    if !ps.is_null()
        || 0 != cg.renderingThirdPerson as libc::c_uint
        || (*cent).currentState.number != cg.predictedPlayerState.clientNum
    {
        CG_LightningBolt(nonPredictedCent, flash.origin.as_mut_ptr());
        if 0. != (*weapon).flashDlightColor[0usize]
            || 0. != (*weapon).flashDlightColor[1usize]
            || 0. != (*weapon).flashDlightColor[2usize]
        {
            trap_R_AddLightToScene(
                flash.origin.as_mut_ptr() as *const vec_t,
                (300i32 + (rand() & 31i32)) as libc::c_float,
                (*weapon).flashDlightColor[0usize],
                (*weapon).flashDlightColor[1usize],
                (*weapon).flashDlightColor[2usize],
            );
        }
    };
}
/*
===============
CG_LightningBolt

Origin will be the exact tag point, which is slightly
different than the muzzle point used for determining hits.
The cent should be the non-predicted cent if it is from the player,
so the endpoint will reflect the simulated strike (lagging the predicted
angle)
===============
*/
unsafe extern "C" fn CG_LightningBolt(mut cent: *mut centity_t, mut origin: *mut vec_t) {
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
    let mut beam: refEntity_t = refEntity_t {
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
    let mut forward: vec3_t = [0.; 3];
    let mut muzzlePoint: vec3_t = [0.; 3];
    let mut endPoint: vec3_t = [0.; 3];
    let mut anim: libc::c_int = 0;
    if (*cent).currentState.weapon != WP_LIGHTNING as libc::c_int {
        return;
    }
    memset(
        &mut beam as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    if (*cent).currentState.number == cg.predictedPlayerState.clientNum
        && cg_trueLightning.value != 0i32 as libc::c_float
    {
        let mut angle: vec3_t = [0.; 3];
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < 3i32 {
            let mut a: libc::c_float =
                (*cent).lerpAngles[i as usize] - cg.refdefViewAngles[i as usize];
            if a > 180i32 as libc::c_float {
                a -= 360i32 as libc::c_float
            }
            if a < -180i32 as libc::c_float {
                a += 360i32 as libc::c_float
            }
            angle[i as usize] = (cg.refdefViewAngles[i as usize] as libc::c_double
                + a as libc::c_double * (1.0f64 - cg_trueLightning.value as libc::c_double))
                as vec_t;
            if angle[i as usize] < 0i32 as libc::c_float {
                angle[i as usize] += 360i32 as libc::c_float
            }
            if angle[i as usize] > 360i32 as libc::c_float {
                angle[i as usize] -= 360i32 as libc::c_float
            }
            i += 1
        }
        AngleVectors(
            angle.as_mut_ptr() as *const vec_t,
            forward.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        muzzlePoint[0usize] = (*cent).lerpOrigin[0usize];
        muzzlePoint[1usize] = (*cent).lerpOrigin[1usize];
        muzzlePoint[2usize] = (*cent).lerpOrigin[2usize]
    } else {
        AngleVectors(
            (*cent).lerpAngles.as_mut_ptr() as *const vec_t,
            forward.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        muzzlePoint[0usize] = (*cent).lerpOrigin[0usize];
        muzzlePoint[1usize] = (*cent).lerpOrigin[1usize];
        muzzlePoint[2usize] = (*cent).lerpOrigin[2usize]
    }
    anim = (*cent).currentState.legsAnim & !128i32;
    if anim == LEGS_WALKCR as libc::c_int || anim == LEGS_IDLECR as libc::c_int {
        muzzlePoint[2usize] += 12i32 as libc::c_float
    } else {
        muzzlePoint[2usize] += 26i32 as libc::c_float
    }
    muzzlePoint[0usize] = muzzlePoint[0usize] + forward[0usize] * 14i32 as libc::c_float;
    muzzlePoint[1usize] = muzzlePoint[1usize] + forward[1usize] * 14i32 as libc::c_float;
    muzzlePoint[2usize] = muzzlePoint[2usize] + forward[2usize] * 14i32 as libc::c_float;
    endPoint[0usize] = muzzlePoint[0usize] + forward[0usize] * 768i32 as libc::c_float;
    endPoint[1usize] = muzzlePoint[1usize] + forward[1usize] * 768i32 as libc::c_float;
    endPoint[2usize] = muzzlePoint[2usize] + forward[2usize] * 768i32 as libc::c_float;
    CG_Trace(
        &mut trace,
        muzzlePoint.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        endPoint.as_mut_ptr() as *const vec_t,
        (*cent).currentState.number,
        1i32 | 0x2000000i32 | 0x4000000i32,
    );
    beam.oldorigin[0usize] = trace.endpos[0usize];
    beam.oldorigin[1usize] = trace.endpos[1usize];
    beam.oldorigin[2usize] = trace.endpos[2usize];
    beam.origin[0usize] = *origin.offset(0isize);
    beam.origin[1usize] = *origin.offset(1isize);
    beam.origin[2usize] = *origin.offset(2isize);
    beam.reType = RT_LIGHTNING;
    beam.customShader = cgs.media.lightningShader;
    trap_R_AddRefEntityToScene(&mut beam);
    if (trace.fraction as libc::c_double) < 1.0f64 {
        let mut angles: vec3_t = [0.; 3];
        let mut dir: vec3_t = [0.; 3];
        dir[0usize] = beam.oldorigin[0usize] - beam.origin[0usize];
        dir[1usize] = beam.oldorigin[1usize] - beam.origin[1usize];
        dir[2usize] = beam.oldorigin[2usize] - beam.origin[2usize];
        VectorNormalize(dir.as_mut_ptr());
        memset(
            &mut beam as *mut refEntity_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
        );
        beam.hModel = cgs.media.lightningExplosionModel;
        beam.origin[0usize] = trace.endpos[0usize] + dir[0usize] * -16i32 as libc::c_float;
        beam.origin[1usize] = trace.endpos[1usize] + dir[1usize] * -16i32 as libc::c_float;
        beam.origin[2usize] = trace.endpos[2usize] + dir[2usize] * -16i32 as libc::c_float;
        angles[0usize] = (rand() % 360i32) as vec_t;
        angles[1usize] = (rand() % 360i32) as vec_t;
        angles[2usize] = (rand() % 360i32) as vec_t;
        AnglesToAxis(angles.as_mut_ptr() as *const vec_t, beam.axis.as_mut_ptr());
        trap_R_AddRefEntityToScene(&mut beam);
    };
}
/*
========================
CG_AddWeaponWithPowerups
========================
*/
unsafe extern "C" fn CG_AddWeaponWithPowerups(
    mut gun: *mut refEntity_t,
    mut powerups: libc::c_int,
) {
    if 0 != powerups & 1i32 << PW_INVIS as libc::c_int {
        (*gun).customShader = cgs.media.invisShader;
        trap_R_AddRefEntityToScene(gun);
    } else {
        trap_R_AddRefEntityToScene(gun);
        if 0 != powerups & 1i32 << PW_BATTLESUIT as libc::c_int {
            (*gun).customShader = cgs.media.battleWeaponShader;
            trap_R_AddRefEntityToScene(gun);
        }
        if 0 != powerups & 1i32 << PW_QUAD as libc::c_int {
            (*gun).customShader = cgs.media.quadWeaponShader;
            trap_R_AddRefEntityToScene(gun);
        }
    };
}
/*

static void CG_LightningBolt( centity_t *cent, vec3_t origin ) {
    trace_t		trace;
    refEntity_t		beam;
    vec3_t			forward;
    vec3_t			muzzlePoint, endPoint;

    if ( cent->currentState.weapon != WP_LIGHTNING ) {
        return;
    }

    memset( &beam, 0, sizeof( beam ) );

    // find muzzle point for this frame
    VectorCopy( cent->lerpOrigin, muzzlePoint );
    AngleVectors( cent->lerpAngles, forward, NULL, NULL );

    // FIXME: crouch
    muzzlePoint[2] += DEFAULT_VIEWHEIGHT;

    VectorMA( muzzlePoint, 14, forward, muzzlePoint );

    // project forward by the lightning range
    VectorMA( muzzlePoint, LIGHTNING_RANGE, forward, endPoint );

    // see if it hit a wall
    CG_Trace( &trace, muzzlePoint, vec3_origin, vec3_origin, endPoint,
        cent->currentState.number, MASK_SHOT );

    // this is the endpoint
    VectorCopy( trace.endpos, beam.oldorigin );

    // use the provided origin, even though it may be slightly
    // different than the muzzle origin
    VectorCopy( origin, beam.origin );

    beam.reType = RT_LIGHTNING;
    beam.customShader = cgs.media.lightningShader;
    trap_R_AddRefEntityToScene( &beam );

    // add the impact flare if it hit something
    if ( trace.fraction < 1.0 ) {
        vec3_t	angles;
        vec3_t	dir;

        VectorSubtract( beam.oldorigin, beam.origin, dir );
        VectorNormalize( dir );

        memset( &beam, 0, sizeof( beam ) );
        beam.hModel = cgs.media.lightningExplosionModel;

        VectorMA( trace.endpos, -16, dir, beam.origin );

        // make a random orientation
        angles[0] = rand() % 360;
        angles[1] = rand() % 360;
        angles[2] = rand() % 360;
        AnglesToAxis( angles, beam.axis );
        trap_R_AddRefEntityToScene( &beam );
    }
}
*/
/*
======================
CG_MachinegunSpinAngle
======================
*/
unsafe extern "C" fn CG_MachinegunSpinAngle(mut cent: *mut centity_t) -> libc::c_float {
    let mut delta: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    delta = cg.time - (*cent).pe.barrelTime;
    if 0 != (*cent).pe.barrelSpinning as u64 {
        angle = ((*cent).pe.barrelAngle as libc::c_double + delta as libc::c_double * 0.9f64)
            as libc::c_float
    } else {
        if delta > 1000i32 {
            delta = 1000i32
        }
        speed = (0.5f64
            * (0.9f64
                + ((1000i32 - delta) as libc::c_float / 1000i32 as libc::c_float)
                    as libc::c_double)) as libc::c_float;
        angle = (*cent).pe.barrelAngle + delta as libc::c_float * speed
    }
    if (*cent).pe.barrelSpinning as libc::c_uint
        == (0 == (*cent).currentState.eFlags & 0x100i32) as libc::c_int as libc::c_uint
    {
        (*cent).pe.barrelTime = cg.time;
        (*cent).pe.barrelAngle = AngleMod(angle);
        (*cent).pe.barrelSpinning =
            (0 != (*cent).currentState.eFlags & 0x100i32) as libc::c_int as qboolean
    }
    return angle;
}
/*
========================================================================================

VIEW WEAPON

========================================================================================
*/
/*
=================
CG_MapTorsoToWeaponFrame

=================
*/
unsafe extern "C" fn CG_MapTorsoToWeaponFrame(
    mut ci: *mut clientInfo_t,
    mut frame: libc::c_int,
) -> libc::c_int {
    if frame >= (*ci).animations[TORSO_DROP as libc::c_int as usize].firstFrame
        && frame < (*ci).animations[TORSO_DROP as libc::c_int as usize].firstFrame + 9i32
    {
        return frame - (*ci).animations[TORSO_DROP as libc::c_int as usize].firstFrame + 6i32;
    }
    if frame >= (*ci).animations[TORSO_ATTACK as libc::c_int as usize].firstFrame
        && frame < (*ci).animations[TORSO_ATTACK as libc::c_int as usize].firstFrame + 6i32
    {
        return 1i32 + frame - (*ci).animations[TORSO_ATTACK as libc::c_int as usize].firstFrame;
    }
    if frame >= (*ci).animations[TORSO_ATTACK2 as libc::c_int as usize].firstFrame
        && frame < (*ci).animations[TORSO_ATTACK2 as libc::c_int as usize].firstFrame + 6i32
    {
        return 1i32 + frame - (*ci).animations[TORSO_ATTACK2 as libc::c_int as usize].firstFrame;
    }
    return 0i32;
}
/*
==============
CG_CalculateWeaponPosition
==============
*/
unsafe extern "C" fn CG_CalculateWeaponPosition(mut origin: *mut vec_t, mut angles: *mut vec_t) {
    let mut scale: libc::c_float = 0.;
    let mut delta: libc::c_int = 0;
    let mut fracsin: libc::c_float = 0.;
    *origin.offset(0isize) = cg.refdef.vieworg[0usize];
    *origin.offset(1isize) = cg.refdef.vieworg[1usize];
    *origin.offset(2isize) = cg.refdef.vieworg[2usize];
    *angles.offset(0isize) = cg.refdefViewAngles[0usize];
    *angles.offset(1isize) = cg.refdefViewAngles[1usize];
    *angles.offset(2isize) = cg.refdefViewAngles[2usize];
    if 0 != cg.bobcycle & 1i32 {
        scale = -cg.xyspeed
    } else {
        scale = cg.xyspeed
    }
    let ref mut fresh4 = *angles.offset(2isize);
    *fresh4 =
        (*fresh4 as libc::c_double + (scale * cg.bobfracsin) as libc::c_double * 0.005f64) as vec_t;
    let ref mut fresh5 = *angles.offset(1isize);
    *fresh5 =
        (*fresh5 as libc::c_double + (scale * cg.bobfracsin) as libc::c_double * 0.01f64) as vec_t;
    let ref mut fresh6 = *angles.offset(0isize);
    *fresh6 = (*fresh6 as libc::c_double
        + (cg.xyspeed * cg.bobfracsin) as libc::c_double * 0.005f64) as vec_t;
    delta = cg.time - cg.landTime;
    if delta < 150i32 {
        let ref mut fresh7 = *origin.offset(2isize);
        *fresh7 = (*fresh7 as libc::c_double
            + cg.landChange as libc::c_double * 0.25f64 * delta as libc::c_double
                / 150i32 as libc::c_double) as vec_t
    } else if delta < 150i32 + 300i32 {
        let ref mut fresh8 = *origin.offset(2isize);
        *fresh8 = (*fresh8 as libc::c_double
            + cg.landChange as libc::c_double
                * 0.25f64
                * (150i32 + 300i32 - delta) as libc::c_double
                / 300i32 as libc::c_double) as vec_t
    }
    scale = cg.xyspeed + 40i32 as libc::c_float;
    fracsin = sin(cg.time as libc::c_double * 0.001f64) as libc::c_float;
    let ref mut fresh9 = *angles.offset(2isize);
    *fresh9 = (*fresh9 as libc::c_double + (scale * fracsin) as libc::c_double * 0.01f64) as vec_t;
    let ref mut fresh10 = *angles.offset(1isize);
    *fresh10 =
        (*fresh10 as libc::c_double + (scale * fracsin) as libc::c_double * 0.01f64) as vec_t;
    let ref mut fresh11 = *angles.offset(0isize);
    *fresh11 =
        (*fresh11 as libc::c_double + (scale * fracsin) as libc::c_double * 0.01f64) as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn CG_DrawWeaponSelect() {
    let mut i: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    if cg.predictedPlayerState.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        return;
    }
    color = CG_FadeColor(cg.weaponSelectTime, 1400i32);
    if color.is_null() {
        return;
    }
    trap_R_SetColor(color);
    cg.itemPickupTime = 0i32;
    bits = (*cg.snap).ps.stats[STAT_WEAPONS as libc::c_int as usize];
    count = 0i32;
    i = 1i32;
    while i < 16i32 {
        if 0 != bits & 1i32 << i {
            count += 1
        }
        i += 1
    }
    x = 320i32 - count * 20i32;
    y = 380i32;
    i = 1i32;
    while i < 16i32 {
        if !(0 == bits & 1i32 << i) {
            CG_RegisterWeapon(i);
            CG_DrawPic(
                x as libc::c_float,
                y as libc::c_float,
                32i32 as libc::c_float,
                32i32 as libc::c_float,
                cg_weapons[i as usize].weaponIcon,
            );
            if i == cg.weaponSelect {
                CG_DrawPic(
                    (x - 4i32) as libc::c_float,
                    (y - 4i32) as libc::c_float,
                    40i32 as libc::c_float,
                    40i32 as libc::c_float,
                    cgs.media.selectShader,
                );
            }
            if 0 == (*cg.snap).ps.ammo[i as usize] {
                CG_DrawPic(
                    x as libc::c_float,
                    y as libc::c_float,
                    32i32 as libc::c_float,
                    32i32 as libc::c_float,
                    cgs.media.noammoShader,
                );
            }
            x += 40i32
        }
        i += 1
    }
    if !cg_weapons[cg.weaponSelect as usize].item.is_null() {
        name = (*cg_weapons[cg.weaponSelect as usize].item).pickup_name;
        if !name.is_null() {
            w = CG_DrawStrlen(name) * 16i32;
            x = (640i32 - w) / 2i32;
            CG_DrawBigStringColor(x, y - 22i32, name, color);
        }
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
// should this be in pmove?
#[no_mangle]
pub unsafe extern "C" fn CG_OutOfAmmoChange() {
    let mut i: libc::c_int = 0;
    cg.weaponSelectTime = cg.time;
    i = 16i32 - 1i32;
    while i > 0i32 {
        if 0 != CG_WeaponSelectable(i) as u64 {
            cg.weaponSelect = i;
            break;
        } else {
            i -= 1
        }
    }
}
