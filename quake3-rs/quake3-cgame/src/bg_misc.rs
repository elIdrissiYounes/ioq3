use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    gitem_s, gitem_t, holdable_t, itemType_t, powerup_t, unnamed_0, unnamed_1, unnamed_2,
    unnamed_3, unnamed_4, unnamed_5, unnamed_6, weapon_t, ET_BEAM, ET_EVENTS, ET_GENERAL,
    ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER,
    ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL,
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
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
    HI_INVULNERABILITY, HI_KAMIKAZE, HI_MEDKIT, HI_NONE, HI_NUM_HOLDABLE, HI_PORTAL, HI_TELEPORTER,
    IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM,
    IT_WEAPON, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES,
    PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS,
    PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT,
    PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION, PM_NOCLIP, PM_NORMAL, PM_SPECTATOR,
    PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT, PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD,
    PW_HASTE, PW_INVIS, PW_INVULNERABILITY, PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD,
    PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH,
    STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS,
    TEAM_RED, TEAM_SPECTATOR, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER,
    WP_LIGHTNING, WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN,
    WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
use libc;
use q_math::{
    axisDefault, colorWhite, g_color_table, vec3_origin, vectoangles, AngleMod, AngleNormalize180,
    AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis, AxisClear, AxisCopy, ByteToDir,
    LerpAngle, MatrixMultiply, PerpendicularVector, Q_crandom, Q_random, RotateAroundDirection,
    RotatePointAroundVector, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    entityState_s, entityState_t, playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t,
    trajectory_t, unnamed, vec3_t, vec_t, Com_Error, Com_Printf, Q_stricmp, ERR_DISCONNECT,
    ERR_DROP, ERR_FATAL, ERR_NEED_CD, ERR_SERVERDISCONNECT, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{cos, fabs, sin};

// included in both the game dll and the client
#[no_mangle]
pub static mut bg_itemlist: [gitem_t; 37] = [
    gitem_s {
        classname: 0 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: 0 as *const libc::c_char as *mut libc::c_char,
        world_model: [
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: 0 as *const libc::c_char as *mut libc::c_char,
        pickup_name: 0 as *const libc::c_char as *mut libc::c_char,
        quantity: 0i32,
        giType: IT_BAD,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_armor_shard\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/ar1_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/armor/shard.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/armor/shard_sphere.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconr_shard\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Armor Shard\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 5i32,
        giType: IT_ARMOR,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_armor_combat\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/misc/ar2_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/armor/armor_yel.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconr_yellow\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Armor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 50i32,
        giType: IT_ARMOR,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_armor_body\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/ar2_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/armor/armor_red.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconr_red\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Heavy Armor\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 100i32,
        giType: IT_ARMOR,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_health_small\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/items/s_health.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/health/small_cross.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/health/small_sphere.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconh_green\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"5 Health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 5i32,
        giType: IT_HEALTH,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/n_health.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/health/medium_cross.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/health/medium_sphere.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconh_yellow\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"25 Health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 25i32,
        giType: IT_HEALTH,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_health_large\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/items/l_health.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/health/large_cross.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/health/large_sphere.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconh_red\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"50 Health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 50i32,
        giType: IT_HEALTH,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_health_mega\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/m_health.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/health/mega_cross.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/health/mega_sphere.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconh_mega\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Mega Health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 100i32,
        giType: IT_HEALTH,
        giTag: 0i32,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/gauntlet/gauntlet.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 0i32,
        giType: IT_WEAPON,
        giTag: WP_GAUNTLET as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/shotgun/shotgun.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 10i32,
        giType: IT_WEAPON,
        giTag: WP_SHOTGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_machinegun\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/machinegun/machinegun.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_machinegun\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_name: b"Machinegun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 40i32,
        giType: IT_WEAPON,
        giTag: WP_MACHINEGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_grenadelauncher\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/grenadel/grenadel.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_grenade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Grenade Launcher\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        quantity: 10i32,
        giType: IT_WEAPON,
        giTag: WP_GRENADE_LAUNCHER as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"sound/weapons/grenade/hgrenb1a.wav sound/weapons/grenade/hgrenb2a.wav\x00"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_rocketlauncher\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/rocketl/rocketl.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Rocket Launcher\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        quantity: 10i32,
        giType: IT_WEAPON,
        giTag: WP_ROCKET_LAUNCHER as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_lightning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/lightning/lightning.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_lightning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Lightning Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 100i32,
        giType: IT_WEAPON,
        giTag: WP_LIGHTNING as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/railgun/railgun.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 10i32,
        giType: IT_WEAPON,
        giTag: WP_RAILGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_plasmagun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/plasma/plasma.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_plasma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Plasma Gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 50i32,
        giType: IT_WEAPON,
        giTag: WP_PLASMAGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/bfg/bfg.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"BFG10K\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 20i32,
        giType: IT_WEAPON,
        giTag: WP_BFG as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"weapon_grapplinghook\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/misc/w_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/weapons2/grapple/grapple.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconw_grapple\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Grappling Hook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 0i32,
        giType: IT_WEAPON,
        giTag: WP_GRAPPLING_HOOK as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_shells\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/shotgunam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Shells\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 10i32,
        giType: IT_AMMO,
        giTag: WP_SHOTGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_bullets\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/machinegunam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_machinegun\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_name: b"Bullets\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 50i32,
        giType: IT_AMMO,
        giTag: WP_MACHINEGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_grenades\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/grenadeam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_grenade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Grenades\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 5i32,
        giType: IT_AMMO,
        giTag: WP_GRENADE_LAUNCHER as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_cells\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/plasmaam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_plasma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Cells\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 30i32,
        giType: IT_AMMO,
        giTag: WP_PLASMAGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_lightning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/lightningam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_lightning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Lightning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 60i32,
        giType: IT_AMMO,
        giTag: WP_LIGHTNING as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_rockets\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/rocketam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Rockets\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 5i32,
        giType: IT_AMMO,
        giTag: WP_ROCKET_LAUNCHER as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_slugs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/railgunam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Slugs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 10i32,
        giType: IT_AMMO,
        giTag: WP_RAILGUN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"ammo_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/misc/am_pkup.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/ammo/bfgam.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/icona_bfg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Bfg Ammo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 15i32,
        giType: IT_AMMO,
        giTag: WP_BFG as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"holdable_teleporter\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: b"sound/items/holdable.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/holdable/teleporter.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/teleporter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Personal Teleporter\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        quantity: 60i32,
        giType: IT_HOLDABLE,
        giTag: HI_TELEPORTER as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"holdable_medkit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/holdable.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/holdable/medkit.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/holdable/medkit_sphere.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/medkit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Medkit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 60i32,
        giType: IT_HOLDABLE,
        giTag: HI_MEDKIT as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"sound/items/use_medkit.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_quad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/quaddamage.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/instant/quad.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/instant/quad_ring.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/quad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Quad Damage\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 30i32,
        giType: IT_POWERUP,
        giTag: PW_QUAD as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"sound/items/damage2.wav sound/items/damage3.wav\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_enviro\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/protect.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/instant/enviro.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/instant/enviro_ring.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/envirosuit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Battle Suit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 30i32,
        giType: IT_POWERUP,
        giTag: PW_BATTLESUIT as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"sound/items/airout.wav sound/items/protect3.wav\x00" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_haste\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/haste.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/instant/haste.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/instant/haste_ring.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/haste\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 30i32,
        giType: IT_POWERUP,
        giTag: PW_HASTE as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_invis\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/invisibility.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/instant/invis.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/instant/invis_ring.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/invis\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Invisibility\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 30i32,
        giType: IT_POWERUP,
        giTag: PW_INVIS as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_regen\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/regeneration.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/instant/regen.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/instant/regen_ring.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/regen\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Regeneration\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 30i32,
        giType: IT_POWERUP,
        giTag: PW_REGEN as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"sound/items/regen.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    },
    gitem_s {
        classname: b"item_flight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: b"sound/items/flight.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        world_model: [
            b"models/powerups/instant/flight.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"models/powerups/instant/flight_ring.md3\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/flight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Flight\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 60i32,
        giType: IT_POWERUP,
        giTag: PW_FLIGHT as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"sound/items/flight.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    },
    gitem_s {
        classname: b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: 0 as *const libc::c_char as *mut libc::c_char,
        world_model: [
            b"models/flags/r_flag.md3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconf_red1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Red Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 0i32,
        giType: IT_TEAM,
        giTag: PW_REDFLAG as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pickup_sound: 0 as *const libc::c_char as *mut libc::c_char,
        world_model: [
            b"models/flags/b_flag.md3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
            0 as *const libc::c_char as *mut libc::c_char,
        ],
        icon: b"icons/iconf_blu1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pickup_name: b"Blue Flag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        quantity: 0i32,
        giType: IT_TEAM,
        giTag: PW_BLUEFLAG as libc::c_int,
        precaches: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        sounds: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    },
    gitem_s {
        classname: 0 as *const libc::c_char as *mut libc::c_char,
        pickup_sound: 0 as *const libc::c_char as *mut libc::c_char,
        world_model: [0 as *const libc::c_char as *mut libc::c_char; 4],
        icon: 0 as *const libc::c_char as *mut libc::c_char,
        pickup_name: 0 as *const libc::c_char as *mut libc::c_char,
        quantity: 0,
        giType: IT_BAD,
        giTag: 0,
        precaches: 0 as *const libc::c_char as *mut libc::c_char,
        sounds: 0 as *const libc::c_char as *mut libc::c_char,
    },
];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut bg_numItems: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn BG_FindItem(mut pickupName: *const libc::c_char) -> *mut gitem_t {
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    it = bg_itemlist.as_mut_ptr().offset(1isize);
    while !(*it).classname.is_null() {
        if 0 == Q_stricmp((*it).pickup_name, pickupName) {
            return it;
        }
        it = it.offset(1isize)
    }
    return 0 as *mut gitem_t;
}
#[no_mangle]
pub unsafe extern "C" fn BG_FindItemForWeapon(mut weapon: weapon_t) -> *mut gitem_t {
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    it = bg_itemlist.as_mut_ptr().offset(1isize);
    while !(*it).classname.is_null() {
        if (*it).giType as libc::c_uint == IT_WEAPON as libc::c_int as libc::c_uint
            && (*it).giTag as libc::c_uint == weapon as libc::c_uint
        {
            return it;
        }
        it = it.offset(1isize)
    }
    Com_Error(
        ERR_DROP as libc::c_int,
        b"Couldn\'t find item for weapon %i\x00" as *const u8 as *const libc::c_char,
        weapon as libc::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BG_FindItemForPowerup(mut pw: powerup_t) -> *mut gitem_t {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < bg_numItems {
        if (bg_itemlist[i as usize].giType as libc::c_uint
            == IT_POWERUP as libc::c_int as libc::c_uint
            || bg_itemlist[i as usize].giType as libc::c_uint
                == IT_TEAM as libc::c_int as libc::c_uint
            || bg_itemlist[i as usize].giType as libc::c_uint
                == IT_PERSISTANT_POWERUP as libc::c_int as libc::c_uint)
            && bg_itemlist[i as usize].giTag as libc::c_uint == pw as libc::c_uint
        {
            return &mut bg_itemlist[i as usize] as *mut gitem_t;
        }
        i += 1
    }
    return 0 as *mut gitem_t;
}
#[no_mangle]
pub unsafe extern "C" fn BG_FindItemForHoldable(mut pw: holdable_t) -> *mut gitem_t {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < bg_numItems {
        if bg_itemlist[i as usize].giType as libc::c_uint
            == IT_HOLDABLE as libc::c_int as libc::c_uint
            && bg_itemlist[i as usize].giTag as libc::c_uint == pw as libc::c_uint
        {
            return &mut bg_itemlist[i as usize] as *mut gitem_t;
        }
        i += 1
    }
    Com_Error(
        ERR_DROP as libc::c_int,
        b"HoldableItem not found\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn BG_CanItemBeGrabbed(
    mut gametype: libc::c_int,
    mut ent: *const entityState_t,
    mut ps: *const playerState_t,
) -> qboolean {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    if (*ent).modelindex < 1i32 || (*ent).modelindex >= bg_numItems {
        Com_Error(
            ERR_DROP as libc::c_int,
            b"BG_CanItemBeGrabbed: index out of range\x00" as *const u8 as *const libc::c_char,
        );
    }
    item = &mut bg_itemlist[(*ent).modelindex as usize] as *mut gitem_t;
    match (*item).giType as libc::c_uint {
        1 => return qtrue,
        2 => {
            if (*ps).ammo[(*item).giTag as usize] >= 200i32 {
                return qfalse;
            }
            return qtrue;
        }
        3 => {
            if (*ps).stats[STAT_ARMOR as libc::c_int as usize]
                >= (*ps).stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
            {
                return qfalse;
            }
            return qtrue;
        }
        4 => {
            if (*item).quantity == 5i32 || (*item).quantity == 100i32 {
                if (*ps).stats[STAT_HEALTH as libc::c_int as usize]
                    >= (*ps).stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
                {
                    return qfalse;
                }
                return qtrue;
            }
            if (*ps).stats[STAT_HEALTH as libc::c_int as usize]
                >= (*ps).stats[STAT_MAX_HEALTH as libc::c_int as usize]
            {
                return qfalse;
            }
            return qtrue;
        }
        5 => return qtrue,
        8 => {
            if gametype == GT_CTF as libc::c_int {
                if (*ps).persistant[PERS_TEAM as libc::c_int as usize] == TEAM_RED as libc::c_int {
                    if (*item).giTag == PW_BLUEFLAG as libc::c_int
                        || (*item).giTag == PW_REDFLAG as libc::c_int && 0 != (*ent).modelindex2
                        || (*item).giTag == PW_REDFLAG as libc::c_int
                            && 0 != (*ps).powerups[PW_BLUEFLAG as libc::c_int as usize]
                    {
                        return qtrue;
                    }
                } else if (*ps).persistant[PERS_TEAM as libc::c_int as usize]
                    == TEAM_BLUE as libc::c_int
                {
                    if (*item).giTag == PW_REDFLAG as libc::c_int
                        || (*item).giTag == PW_BLUEFLAG as libc::c_int && 0 != (*ent).modelindex2
                        || (*item).giTag == PW_BLUEFLAG as libc::c_int
                            && 0 != (*ps).powerups[PW_REDFLAG as libc::c_int as usize]
                    {
                        return qtrue;
                    }
                }
            }
            return qfalse;
        }
        6 => {
            if 0 != (*ps).stats[STAT_HOLDABLE_ITEM as libc::c_int as usize] {
                return qfalse;
            }
            return qtrue;
        }
        0 => {
            Com_Error(
                ERR_DROP as libc::c_int,
                b"BG_CanItemBeGrabbed: IT_BAD\x00" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            Com_Printf(
                b"BG_CanItemBeGrabbed: unknown enum %d\n\x00" as *const u8 as *const libc::c_char,
                (*item).giType as libc::c_uint,
            );
        }
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn BG_EvaluateTrajectory(
    mut tr: *const trajectory_t,
    mut atTime: libc::c_int,
    mut result: *mut vec_t,
) {
    let mut deltaTime: libc::c_float = 0.;
    let mut phase: libc::c_float = 0.;
    match (*tr).trType as libc::c_uint {
        0 | 1 => {
            *result.offset(0isize) = (*tr).trBase[0usize];
            *result.offset(1isize) = (*tr).trBase[1usize];
            *result.offset(2isize) = (*tr).trBase[2usize]
        }
        2 => {
            deltaTime = ((atTime - (*tr).trTime) as libc::c_double * 0.001f64) as libc::c_float;
            *result.offset(0isize) = (*tr).trBase[0usize] + (*tr).trDelta[0usize] * deltaTime;
            *result.offset(1isize) = (*tr).trBase[1usize] + (*tr).trDelta[1usize] * deltaTime;
            *result.offset(2isize) = (*tr).trBase[2usize] + (*tr).trDelta[2usize] * deltaTime
        }
        4 => {
            deltaTime =
                (atTime - (*tr).trTime) as libc::c_float / (*tr).trDuration as libc::c_float;
            phase = sin(deltaTime as libc::c_double
                * 3.14159265358979323846f64
                * 2i32 as libc::c_double) as libc::c_float;
            *result.offset(0isize) = (*tr).trBase[0usize] + (*tr).trDelta[0usize] * phase;
            *result.offset(1isize) = (*tr).trBase[1usize] + (*tr).trDelta[1usize] * phase;
            *result.offset(2isize) = (*tr).trBase[2usize] + (*tr).trDelta[2usize] * phase
        }
        3 => {
            if atTime > (*tr).trTime + (*tr).trDuration {
                atTime = (*tr).trTime + (*tr).trDuration
            }
            deltaTime = ((atTime - (*tr).trTime) as libc::c_double * 0.001f64) as libc::c_float;
            if deltaTime < 0i32 as libc::c_float {
                deltaTime = 0i32 as libc::c_float
            }
            *result.offset(0isize) = (*tr).trBase[0usize] + (*tr).trDelta[0usize] * deltaTime;
            *result.offset(1isize) = (*tr).trBase[1usize] + (*tr).trDelta[1usize] * deltaTime;
            *result.offset(2isize) = (*tr).trBase[2usize] + (*tr).trDelta[2usize] * deltaTime
        }
        5 => {
            deltaTime = ((atTime - (*tr).trTime) as libc::c_double * 0.001f64) as libc::c_float;
            *result.offset(0isize) = (*tr).trBase[0usize] + (*tr).trDelta[0usize] * deltaTime;
            *result.offset(1isize) = (*tr).trBase[1usize] + (*tr).trDelta[1usize] * deltaTime;
            *result.offset(2isize) = (*tr).trBase[2usize] + (*tr).trDelta[2usize] * deltaTime;
            let ref mut fresh0 = *result.offset(2isize);
            *fresh0 = (*fresh0 as libc::c_double
                - 0.5f64
                    * 800i32 as libc::c_double
                    * deltaTime as libc::c_double
                    * deltaTime as libc::c_double) as vec_t
        }
        _ => {
            Com_Error(
                ERR_DROP as libc::c_int,
                b"BG_EvaluateTrajectory: unknown trType: %i\x00" as *const u8
                    as *const libc::c_char,
                (*tr).trType as libc::c_uint,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BG_EvaluateTrajectoryDelta(
    mut tr: *const trajectory_t,
    mut atTime: libc::c_int,
    mut result: *mut vec_t,
) {
    let mut deltaTime: libc::c_float = 0.;
    let mut phase: libc::c_float = 0.;
    match (*tr).trType as libc::c_uint {
        0 | 1 => {
            let ref mut fresh2 = *result.offset(1isize);
            let ref mut fresh1 = *result.offset(2isize);
            *fresh1 = 0i32 as vec_t;
            *fresh2 = *fresh1;
            *result.offset(0isize) = *fresh2
        }
        2 => {
            *result.offset(0isize) = (*tr).trDelta[0usize];
            *result.offset(1isize) = (*tr).trDelta[1usize];
            *result.offset(2isize) = (*tr).trDelta[2usize]
        }
        4 => {
            deltaTime =
                (atTime - (*tr).trTime) as libc::c_float / (*tr).trDuration as libc::c_float;
            phase = cos(deltaTime as libc::c_double
                * 3.14159265358979323846f64
                * 2i32 as libc::c_double) as libc::c_float;
            phase = (phase as libc::c_double * 0.5f64) as libc::c_float;
            *result.offset(0isize) = (*tr).trDelta[0usize] * phase;
            *result.offset(1isize) = (*tr).trDelta[1usize] * phase;
            *result.offset(2isize) = (*tr).trDelta[2usize] * phase
        }
        3 => {
            if atTime > (*tr).trTime + (*tr).trDuration {
                let ref mut fresh4 = *result.offset(1isize);
                let ref mut fresh3 = *result.offset(2isize);
                *fresh3 = 0i32 as vec_t;
                *fresh4 = *fresh3;
                *result.offset(0isize) = *fresh4;
                return;
            }
            *result.offset(0isize) = (*tr).trDelta[0usize];
            *result.offset(1isize) = (*tr).trDelta[1usize];
            *result.offset(2isize) = (*tr).trDelta[2usize]
        }
        5 => {
            deltaTime = ((atTime - (*tr).trTime) as libc::c_double * 0.001f64) as libc::c_float;
            *result.offset(0isize) = (*tr).trDelta[0usize];
            *result.offset(1isize) = (*tr).trDelta[1usize];
            *result.offset(2isize) = (*tr).trDelta[2usize];
            let ref mut fresh5 = *result.offset(2isize);
            *fresh5 -= 800i32 as libc::c_float * deltaTime
        }
        _ => {
            Com_Error(
                ERR_DROP as libc::c_int,
                b"BG_EvaluateTrajectoryDelta: unknown trType: %i\x00" as *const u8
                    as *const libc::c_char,
                (*tr).trType as libc::c_uint,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BG_AddPredictableEventToPlayerstate(
    mut newEvent: libc::c_int,
    mut eventParm: libc::c_int,
    mut ps: *mut playerState_t,
) {
    (*ps).events[((*ps).eventSequence & 2i32 - 1i32) as usize] = newEvent;
    (*ps).eventParms[((*ps).eventSequence & 2i32 - 1i32) as usize] = eventParm;
    (*ps).eventSequence += 1;
}
#[no_mangle]
pub unsafe extern "C" fn BG_TouchJumpPad(
    mut ps: *mut playerState_t,
    mut jumppad: *mut entityState_t,
) {
    let mut angles: vec3_t = [0.; 3];
    let mut p: libc::c_float = 0.;
    let mut effectNum: libc::c_int = 0;
    if (*ps).pm_type != PM_NORMAL as libc::c_int {
        return;
    }
    if 0 != (*ps).powerups[PW_FLIGHT as libc::c_int as usize] {
        return;
    }
    if (*ps).jumppad_ent != (*jumppad).number {
        vectoangles(
            (*jumppad).origin2.as_mut_ptr() as *const vec_t,
            angles.as_mut_ptr(),
        );
        p = fabs(AngleNormalize180(angles[0usize]) as libc::c_double) as libc::c_float;
        if p < 45i32 as libc::c_float {
            effectNum = 0i32
        } else {
            effectNum = 1i32
        }
        BG_AddPredictableEventToPlayerstate(EV_JUMP_PAD as libc::c_int, effectNum, ps);
    }
    (*ps).jumppad_ent = (*jumppad).number;
    (*ps).jumppad_frame = (*ps).pmove_framecount;
    (*ps).velocity[0usize] = (*jumppad).origin2[0usize];
    (*ps).velocity[1usize] = (*jumppad).origin2[1usize];
    (*ps).velocity[2usize] = (*jumppad).origin2[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn BG_PlayerStateToEntityState(
    mut ps: *mut playerState_t,
    mut s: *mut entityState_t,
    mut snap: qboolean,
) {
    let mut i: libc::c_int = 0;
    if (*ps).pm_type == PM_INTERMISSION as libc::c_int
        || (*ps).pm_type == PM_SPECTATOR as libc::c_int
    {
        (*s).eType = ET_INVISIBLE as libc::c_int
    } else if (*ps).stats[STAT_HEALTH as libc::c_int as usize] <= -40i32 {
        (*s).eType = ET_INVISIBLE as libc::c_int
    } else {
        (*s).eType = ET_PLAYER as libc::c_int
    }
    (*s).number = (*ps).clientNum;
    (*s).pos.trType = TR_INTERPOLATE;
    (*s).pos.trBase[0usize] = (*ps).origin[0usize];
    (*s).pos.trBase[1usize] = (*ps).origin[1usize];
    (*s).pos.trBase[2usize] = (*ps).origin[2usize];
    if 0 != snap as u64 {
        (*s).pos.trBase[0usize] = (*s).pos.trBase[0usize] as libc::c_int as vec_t;
        (*s).pos.trBase[1usize] = (*s).pos.trBase[1usize] as libc::c_int as vec_t;
        (*s).pos.trBase[2usize] = (*s).pos.trBase[2usize] as libc::c_int as vec_t
    }
    (*s).pos.trDelta[0usize] = (*ps).velocity[0usize];
    (*s).pos.trDelta[1usize] = (*ps).velocity[1usize];
    (*s).pos.trDelta[2usize] = (*ps).velocity[2usize];
    (*s).apos.trType = TR_INTERPOLATE;
    (*s).apos.trBase[0usize] = (*ps).viewangles[0usize];
    (*s).apos.trBase[1usize] = (*ps).viewangles[1usize];
    (*s).apos.trBase[2usize] = (*ps).viewangles[2usize];
    if 0 != snap as u64 {
        (*s).apos.trBase[0usize] = (*s).apos.trBase[0usize] as libc::c_int as vec_t;
        (*s).apos.trBase[1usize] = (*s).apos.trBase[1usize] as libc::c_int as vec_t;
        (*s).apos.trBase[2usize] = (*s).apos.trBase[2usize] as libc::c_int as vec_t
    }
    (*s).angles2[1usize] = (*ps).movementDir as vec_t;
    (*s).legsAnim = (*ps).legsAnim;
    (*s).torsoAnim = (*ps).torsoAnim;
    (*s).clientNum = (*ps).clientNum;
    (*s).eFlags = (*ps).eFlags;
    if (*ps).stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        (*s).eFlags |= 0x1i32
    } else {
        (*s).eFlags &= !0x1i32
    }
    if 0 != (*ps).externalEvent {
        (*s).event = (*ps).externalEvent;
        (*s).eventParm = (*ps).externalEventParm
    } else if (*ps).entityEventSequence < (*ps).eventSequence {
        let mut seq: libc::c_int = 0;
        if (*ps).entityEventSequence < (*ps).eventSequence - 2i32 {
            (*ps).entityEventSequence = (*ps).eventSequence - 2i32
        }
        seq = (*ps).entityEventSequence & 2i32 - 1i32;
        (*s).event = (*ps).events[seq as usize] | ((*ps).entityEventSequence & 3i32) << 8i32;
        (*s).eventParm = (*ps).eventParms[seq as usize];
        (*ps).entityEventSequence += 1
    }
    (*s).weapon = (*ps).weapon;
    (*s).groundEntityNum = (*ps).groundEntityNum;
    (*s).powerups = 0i32;
    i = 0i32;
    while i < 16i32 {
        if 0 != (*ps).powerups[i as usize] {
            (*s).powerups |= 1i32 << i
        }
        i += 1
    }
    (*s).loopSound = (*ps).loopSound;
    (*s).generic1 = (*ps).generic1;
}
#[no_mangle]
pub unsafe extern "C" fn BG_PlayerStateToEntityStateExtraPolate(
    mut ps: *mut playerState_t,
    mut s: *mut entityState_t,
    mut time: libc::c_int,
    mut snap: qboolean,
) {
    let mut i: libc::c_int = 0;
    if (*ps).pm_type == PM_INTERMISSION as libc::c_int
        || (*ps).pm_type == PM_SPECTATOR as libc::c_int
    {
        (*s).eType = ET_INVISIBLE as libc::c_int
    } else if (*ps).stats[STAT_HEALTH as libc::c_int as usize] <= -40i32 {
        (*s).eType = ET_INVISIBLE as libc::c_int
    } else {
        (*s).eType = ET_PLAYER as libc::c_int
    }
    (*s).number = (*ps).clientNum;
    (*s).pos.trType = TR_LINEAR_STOP;
    (*s).pos.trBase[0usize] = (*ps).origin[0usize];
    (*s).pos.trBase[1usize] = (*ps).origin[1usize];
    (*s).pos.trBase[2usize] = (*ps).origin[2usize];
    if 0 != snap as u64 {
        (*s).pos.trBase[0usize] = (*s).pos.trBase[0usize] as libc::c_int as vec_t;
        (*s).pos.trBase[1usize] = (*s).pos.trBase[1usize] as libc::c_int as vec_t;
        (*s).pos.trBase[2usize] = (*s).pos.trBase[2usize] as libc::c_int as vec_t
    }
    (*s).pos.trDelta[0usize] = (*ps).velocity[0usize];
    (*s).pos.trDelta[1usize] = (*ps).velocity[1usize];
    (*s).pos.trDelta[2usize] = (*ps).velocity[2usize];
    (*s).pos.trTime = time;
    (*s).pos.trDuration = 50i32;
    (*s).apos.trType = TR_INTERPOLATE;
    (*s).apos.trBase[0usize] = (*ps).viewangles[0usize];
    (*s).apos.trBase[1usize] = (*ps).viewangles[1usize];
    (*s).apos.trBase[2usize] = (*ps).viewangles[2usize];
    if 0 != snap as u64 {
        (*s).apos.trBase[0usize] = (*s).apos.trBase[0usize] as libc::c_int as vec_t;
        (*s).apos.trBase[1usize] = (*s).apos.trBase[1usize] as libc::c_int as vec_t;
        (*s).apos.trBase[2usize] = (*s).apos.trBase[2usize] as libc::c_int as vec_t
    }
    (*s).angles2[1usize] = (*ps).movementDir as vec_t;
    (*s).legsAnim = (*ps).legsAnim;
    (*s).torsoAnim = (*ps).torsoAnim;
    (*s).clientNum = (*ps).clientNum;
    (*s).eFlags = (*ps).eFlags;
    if (*ps).stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        (*s).eFlags |= 0x1i32
    } else {
        (*s).eFlags &= !0x1i32
    }
    if 0 != (*ps).externalEvent {
        (*s).event = (*ps).externalEvent;
        (*s).eventParm = (*ps).externalEventParm
    } else if (*ps).entityEventSequence < (*ps).eventSequence {
        let mut seq: libc::c_int = 0;
        if (*ps).entityEventSequence < (*ps).eventSequence - 2i32 {
            (*ps).entityEventSequence = (*ps).eventSequence - 2i32
        }
        seq = (*ps).entityEventSequence & 2i32 - 1i32;
        (*s).event = (*ps).events[seq as usize] | ((*ps).entityEventSequence & 3i32) << 8i32;
        (*s).eventParm = (*ps).eventParms[seq as usize];
        (*ps).entityEventSequence += 1
    }
    (*s).weapon = (*ps).weapon;
    (*s).groundEntityNum = (*ps).groundEntityNum;
    (*s).powerups = 0i32;
    i = 0i32;
    while i < 16i32 {
        if 0 != (*ps).powerups[i as usize] {
            (*s).powerups |= 1i32 << i
        }
        i += 1
    }
    (*s).loopSound = (*ps).loopSound;
    (*s).generic1 = (*ps).generic1;
}
#[no_mangle]
pub unsafe extern "C" fn BG_PlayerTouchesItem(
    mut ps: *mut playerState_t,
    mut item: *mut entityState_t,
    mut atTime: libc::c_int,
) -> qboolean {
    let mut origin: vec3_t = [0.; 3];
    BG_EvaluateTrajectory(&mut (*item).pos, atTime, origin.as_mut_ptr());
    if (*ps).origin[0usize] - origin[0usize] > 44i32 as libc::c_float
        || (*ps).origin[0usize] - origin[0usize] < -50i32 as libc::c_float
        || (*ps).origin[1usize] - origin[1usize] > 36i32 as libc::c_float
        || (*ps).origin[1usize] - origin[1usize] < -36i32 as libc::c_float
        || (*ps).origin[2usize] - origin[2usize] > 36i32 as libc::c_float
        || (*ps).origin[2usize] - origin[2usize] < -36i32 as libc::c_float
    {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub static mut eventnames: [*mut libc::c_char; 83] = [
    b"EV_NONE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FOOTSTEP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FOOTSTEP_METAL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FOOTSPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FOOTWADE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_SWIM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_STEP_4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_STEP_8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_STEP_12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_STEP_16\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FALL_SHORT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FALL_MEDIUM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FALL_FAR\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_JUMP_PAD\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_JUMP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_WATER_TOUCH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_WATER_LEAVE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_WATER_UNDER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_WATER_CLEAR\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_ITEM_PICKUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_GLOBAL_ITEM_PICKUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_NOAMMO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_CHANGE_WEAPON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_FIRE_WEAPON\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM10\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM11\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM14\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_USE_ITEM15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_ITEM_RESPAWN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_ITEM_POP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_PLAYER_TELEPORT_IN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_PLAYER_TELEPORT_OUT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_GRENADE_BOUNCE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_GENERAL_SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_GLOBAL_SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_GLOBAL_TEAM_SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_BULLET_HIT_FLESH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_BULLET_HIT_WALL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_MISSILE_HIT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_MISSILE_MISS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_MISSILE_MISS_METAL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_RAILTRAIL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_SHOTGUN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_BULLET\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_PAIN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_DEATH1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_DEATH2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_DEATH3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_OBITUARY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_POWERUP_QUAD\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_POWERUP_BATTLESUIT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_POWERUP_REGEN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_GIB_PLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_SCOREPLUM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_PROXIMITY_MINE_STICK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_PROXIMITY_MINE_TRIGGER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_KAMIKAZE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_OBELISKEXPLODE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_OBELISKPAIN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_INVUL_IMPACT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_JUICED\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_LIGHTNINGBOLT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_DEBUG_LINE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_STOPLOOPINGSOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_TAUNT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_TAUNT_YES\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_TAUNT_NO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_TAUNT_FOLLOWME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_TAUNT_GETFLAG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_TAUNT_GUARDBASE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"EV_TAUNT_PATROL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
unsafe extern "C" fn run_static_initializers() {
    bg_numItems = (::std::mem::size_of::<[gitem_t; 37]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<gitem_t>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
