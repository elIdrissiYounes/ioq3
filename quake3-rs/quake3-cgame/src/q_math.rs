use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
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
use q_shared_h::{
    byte, cplane_s, cplane_t, floatint_t, qboolean, qfalse, qtrue, vec3_t, vec4_t, vec_t,
};
use stdlib::{__assert_fail, acos, atan2, cos, fabs, memcpy, memset, sin, sqrt};

#[no_mangle]
pub static mut bytedirs: [vec3_t; 162] = [
    [-0.525731f32, 0.000000f32, 0.850651f32],
    [-0.442863f32, 0.238856f32, 0.864188f32],
    [-0.295242f32, 0.000000f32, 0.955423f32],
    [-0.309017f32, 0.500000f32, 0.809017f32],
    [-0.162460f32, 0.262866f32, 0.951056f32],
    [0.000000f32, 0.000000f32, 1.000000f32],
    [0.000000f32, 0.850651f32, 0.525731f32],
    [-0.147621f32, 0.716567f32, 0.681718f32],
    [0.147621f32, 0.716567f32, 0.681718f32],
    [0.000000f32, 0.525731f32, 0.850651f32],
    [0.309017f32, 0.500000f32, 0.809017f32],
    [0.525731f32, 0.000000f32, 0.850651f32],
    [0.295242f32, 0.000000f32, 0.955423f32],
    [0.442863f32, 0.238856f32, 0.864188f32],
    [0.162460f32, 0.262866f32, 0.951056f32],
    [-0.681718f32, 0.147621f32, 0.716567f32],
    [-0.809017f32, 0.309017f32, 0.500000f32],
    [-0.587785f32, 0.425325f32, 0.688191f32],
    [-0.850651f32, 0.525731f32, 0.000000f32],
    [-0.864188f32, 0.442863f32, 0.238856f32],
    [-0.716567f32, 0.681718f32, 0.147621f32],
    [-0.688191f32, 0.587785f32, 0.425325f32],
    [-0.500000f32, 0.809017f32, 0.309017f32],
    [-0.238856f32, 0.864188f32, 0.442863f32],
    [-0.425325f32, 0.688191f32, 0.587785f32],
    [-0.716567f32, 0.681718f32, -0.147621f32],
    [-0.500000f32, 0.809017f32, -0.309017f32],
    [-0.525731f32, 0.850651f32, 0.000000f32],
    [0.000000f32, 0.850651f32, -0.525731f32],
    [-0.238856f32, 0.864188f32, -0.442863f32],
    [0.000000f32, 0.955423f32, -0.295242f32],
    [-0.262866f32, 0.951056f32, -0.162460f32],
    [0.000000f32, 1.000000f32, 0.000000f32],
    [0.000000f32, 0.955423f32, 0.295242f32],
    [-0.262866f32, 0.951056f32, 0.162460f32],
    [0.238856f32, 0.864188f32, 0.442863f32],
    [0.262866f32, 0.951056f32, 0.162460f32],
    [0.500000f32, 0.809017f32, 0.309017f32],
    [0.238856f32, 0.864188f32, -0.442863f32],
    [0.262866f32, 0.951056f32, -0.162460f32],
    [0.500000f32, 0.809017f32, -0.309017f32],
    [0.850651f32, 0.525731f32, 0.000000f32],
    [0.716567f32, 0.681718f32, 0.147621f32],
    [0.716567f32, 0.681718f32, -0.147621f32],
    [0.525731f32, 0.850651f32, 0.000000f32],
    [0.425325f32, 0.688191f32, 0.587785f32],
    [0.864188f32, 0.442863f32, 0.238856f32],
    [0.688191f32, 0.587785f32, 0.425325f32],
    [0.809017f32, 0.309017f32, 0.500000f32],
    [0.681718f32, 0.147621f32, 0.716567f32],
    [0.587785f32, 0.425325f32, 0.688191f32],
    [0.955423f32, 0.295242f32, 0.000000f32],
    [1.000000f32, 0.000000f32, 0.000000f32],
    [0.951056f32, 0.162460f32, 0.262866f32],
    [0.850651f32, -0.525731f32, 0.000000f32],
    [0.955423f32, -0.295242f32, 0.000000f32],
    [0.864188f32, -0.442863f32, 0.238856f32],
    [0.951056f32, -0.162460f32, 0.262866f32],
    [0.809017f32, -0.309017f32, 0.500000f32],
    [0.681718f32, -0.147621f32, 0.716567f32],
    [0.850651f32, 0.000000f32, 0.525731f32],
    [0.864188f32, 0.442863f32, -0.238856f32],
    [0.809017f32, 0.309017f32, -0.500000f32],
    [0.951056f32, 0.162460f32, -0.262866f32],
    [0.525731f32, 0.000000f32, -0.850651f32],
    [0.681718f32, 0.147621f32, -0.716567f32],
    [0.681718f32, -0.147621f32, -0.716567f32],
    [0.850651f32, 0.000000f32, -0.525731f32],
    [0.809017f32, -0.309017f32, -0.500000f32],
    [0.864188f32, -0.442863f32, -0.238856f32],
    [0.951056f32, -0.162460f32, -0.262866f32],
    [0.147621f32, 0.716567f32, -0.681718f32],
    [0.309017f32, 0.500000f32, -0.809017f32],
    [0.425325f32, 0.688191f32, -0.587785f32],
    [0.442863f32, 0.238856f32, -0.864188f32],
    [0.587785f32, 0.425325f32, -0.688191f32],
    [0.688191f32, 0.587785f32, -0.425325f32],
    [-0.147621f32, 0.716567f32, -0.681718f32],
    [-0.309017f32, 0.500000f32, -0.809017f32],
    [0.000000f32, 0.525731f32, -0.850651f32],
    [-0.525731f32, 0.000000f32, -0.850651f32],
    [-0.442863f32, 0.238856f32, -0.864188f32],
    [-0.295242f32, 0.000000f32, -0.955423f32],
    [-0.162460f32, 0.262866f32, -0.951056f32],
    [0.000000f32, 0.000000f32, -1.000000f32],
    [0.295242f32, 0.000000f32, -0.955423f32],
    [0.162460f32, 0.262866f32, -0.951056f32],
    [-0.442863f32, -0.238856f32, -0.864188f32],
    [-0.309017f32, -0.500000f32, -0.809017f32],
    [-0.162460f32, -0.262866f32, -0.951056f32],
    [0.000000f32, -0.850651f32, -0.525731f32],
    [-0.147621f32, -0.716567f32, -0.681718f32],
    [0.147621f32, -0.716567f32, -0.681718f32],
    [0.000000f32, -0.525731f32, -0.850651f32],
    [0.309017f32, -0.500000f32, -0.809017f32],
    [0.442863f32, -0.238856f32, -0.864188f32],
    [0.162460f32, -0.262866f32, -0.951056f32],
    [0.238856f32, -0.864188f32, -0.442863f32],
    [0.500000f32, -0.809017f32, -0.309017f32],
    [0.425325f32, -0.688191f32, -0.587785f32],
    [0.716567f32, -0.681718f32, -0.147621f32],
    [0.688191f32, -0.587785f32, -0.425325f32],
    [0.587785f32, -0.425325f32, -0.688191f32],
    [0.000000f32, -0.955423f32, -0.295242f32],
    [0.000000f32, -1.000000f32, 0.000000f32],
    [0.262866f32, -0.951056f32, -0.162460f32],
    [0.000000f32, -0.850651f32, 0.525731f32],
    [0.000000f32, -0.955423f32, 0.295242f32],
    [0.238856f32, -0.864188f32, 0.442863f32],
    [0.262866f32, -0.951056f32, 0.162460f32],
    [0.500000f32, -0.809017f32, 0.309017f32],
    [0.716567f32, -0.681718f32, 0.147621f32],
    [0.525731f32, -0.850651f32, 0.000000f32],
    [-0.238856f32, -0.864188f32, -0.442863f32],
    [-0.500000f32, -0.809017f32, -0.309017f32],
    [-0.262866f32, -0.951056f32, -0.162460f32],
    [-0.850651f32, -0.525731f32, 0.000000f32],
    [-0.716567f32, -0.681718f32, -0.147621f32],
    [-0.716567f32, -0.681718f32, 0.147621f32],
    [-0.525731f32, -0.850651f32, 0.000000f32],
    [-0.500000f32, -0.809017f32, 0.309017f32],
    [-0.238856f32, -0.864188f32, 0.442863f32],
    [-0.262866f32, -0.951056f32, 0.162460f32],
    [-0.864188f32, -0.442863f32, 0.238856f32],
    [-0.809017f32, -0.309017f32, 0.500000f32],
    [-0.688191f32, -0.587785f32, 0.425325f32],
    [-0.681718f32, -0.147621f32, 0.716567f32],
    [-0.442863f32, -0.238856f32, 0.864188f32],
    [-0.587785f32, -0.425325f32, 0.688191f32],
    [-0.309017f32, -0.500000f32, 0.809017f32],
    [-0.147621f32, -0.716567f32, 0.681718f32],
    [-0.425325f32, -0.688191f32, 0.587785f32],
    [-0.162460f32, -0.262866f32, 0.951056f32],
    [0.442863f32, -0.238856f32, 0.864188f32],
    [0.162460f32, -0.262866f32, 0.951056f32],
    [0.309017f32, -0.500000f32, 0.809017f32],
    [0.147621f32, -0.716567f32, 0.681718f32],
    [0.000000f32, -0.525731f32, 0.850651f32],
    [0.425325f32, -0.688191f32, 0.587785f32],
    [0.587785f32, -0.425325f32, 0.688191f32],
    [0.688191f32, -0.587785f32, 0.425325f32],
    [-0.955423f32, 0.295242f32, 0.000000f32],
    [-0.951056f32, 0.162460f32, 0.262866f32],
    [-1.000000f32, 0.000000f32, 0.000000f32],
    [-0.850651f32, 0.000000f32, 0.525731f32],
    [-0.955423f32, -0.295242f32, 0.000000f32],
    [-0.951056f32, -0.162460f32, 0.262866f32],
    [-0.864188f32, 0.442863f32, -0.238856f32],
    [-0.951056f32, 0.162460f32, -0.262866f32],
    [-0.809017f32, 0.309017f32, -0.500000f32],
    [-0.864188f32, -0.442863f32, -0.238856f32],
    [-0.951056f32, -0.162460f32, -0.262866f32],
    [-0.809017f32, -0.309017f32, -0.500000f32],
    [-0.681718f32, 0.147621f32, -0.716567f32],
    [-0.681718f32, -0.147621f32, -0.716567f32],
    [-0.850651f32, 0.000000f32, -0.525731f32],
    [-0.688191f32, 0.587785f32, -0.425325f32],
    [-0.587785f32, 0.425325f32, -0.688191f32],
    [-0.425325f32, 0.688191f32, -0.587785f32],
    [-0.425325f32, -0.688191f32, -0.587785f32],
    [-0.587785f32, -0.425325f32, -0.688191f32],
    [-0.688191f32, -0.587785f32, -0.425325f32],
];
// all drawing is done to a 640*480 virtual screen size
// and will be automatically scaled to the real resolution
#[no_mangle]
pub static mut colorBlack: vec4_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorRed: vec4_t = [1i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorGreen: vec4_t = [0i32 as vec_t, 1i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorBlue: vec4_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorYellow: vec4_t = [1i32 as vec_t, 1i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorMagenta: vec4_t = [1i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorCyan: vec4_t = [0i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorWhite: vec4_t = [1i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t, 1i32 as vec_t];
#[no_mangle]
pub static mut colorLtGrey: vec4_t = [
    0.75f64 as vec_t,
    0.75f64 as vec_t,
    0.75f64 as vec_t,
    1i32 as vec_t,
];
#[no_mangle]
pub static mut colorMdGrey: vec4_t = [
    0.5f64 as vec_t,
    0.5f64 as vec_t,
    0.5f64 as vec_t,
    1i32 as vec_t,
];
#[no_mangle]
pub static mut colorDkGrey: vec4_t = [
    0.25f64 as vec_t,
    0.25f64 as vec_t,
    0.25f64 as vec_t,
    1i32 as vec_t,
];
#[no_mangle]
pub static mut g_color_table: [vec4_t; 8] = [
    [
        0.0f64 as vec_t,
        0.0f64 as vec_t,
        0.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
    [
        1.0f64 as vec_t,
        0.0f64 as vec_t,
        0.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
    [
        0.0f64 as vec_t,
        1.0f64 as vec_t,
        0.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
    [
        1.0f64 as vec_t,
        1.0f64 as vec_t,
        0.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
    [
        0.0f64 as vec_t,
        0.0f64 as vec_t,
        1.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
    [
        0.0f64 as vec_t,
        1.0f64 as vec_t,
        1.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
    [
        1.0f64 as vec_t,
        0.0f64 as vec_t,
        1.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
    [
        1.0f64 as vec_t,
        1.0f64 as vec_t,
        1.0f64 as vec_t,
        1.0f64 as vec_t,
    ],
];
#[no_mangle]
pub static mut vec3_origin: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t];
#[no_mangle]
pub static mut axisDefault: [vec3_t; 3] = [
    [1i32 as vec_t, 0i32 as vec_t, 0i32 as vec_t],
    [0i32 as vec_t, 1i32 as vec_t, 0i32 as vec_t],
    [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t],
];
#[no_mangle]
pub unsafe extern "C" fn Q_isnan(mut x: libc::c_float) -> libc::c_int {
    let mut fi: floatint_t = floatint_t { f: 0. };
    fi.f = x;
    fi.ui &= 0x7fffffffi32 as libc::c_uint;
    fi.ui = (0x7f800000i32 as libc::c_uint).wrapping_sub(fi.ui);
    return (fi.ui >> 31i32) as libc::c_int;
}
/*
// if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
// or write a mail to the ioq3 mailing list.
#else
  #define Q_ftol(v) ((long) (v))
  #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
  #define Q_SnapVector(vec) \
    do\
    {\
        vec3_t *temp = (vec);\
        \
        Q_round((*temp)[0]);\
        Q_round((*temp)[1]);\
        Q_round((*temp)[2]);\
    } while(0)
#endif
*/
#[no_mangle]
pub unsafe extern "C" fn Q_fabs(mut f: libc::c_float) -> libc::c_float {
    let mut fi: floatint_t = floatint_t { f: 0. };
    fi.f = f;
    fi.i &= 0x7fffffffi32;
    return fi.f;
}
// reciprocal square root
#[no_mangle]
pub unsafe extern "C" fn Q_rsqrt(mut number: libc::c_float) -> libc::c_float {
    let mut t: floatint_t = floatint_t { f: 0. };
    let mut x2: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let threehalfs: libc::c_float = 1.5f32;
    x2 = number * 0.5f32;
    t.f = number;
    t.i = 0x5f3759dfi32 - (t.i >> 1i32);
    y = t.f;
    y = y * (threehalfs - x2 * y * y);
    return y;
}
#[no_mangle]
pub unsafe extern "C" fn ClampChar(mut i: libc::c_int) -> libc::c_schar {
    if i < -128i32 {
        return -128i32 as libc::c_schar;
    }
    if i > 127i32 {
        return 127i32 as libc::c_schar;
    }
    return i as libc::c_schar;
}
#[no_mangle]
pub unsafe extern "C" fn ClampShort(mut i: libc::c_int) -> libc::c_short {
    if i < -32768i32 {
        return -32768i32 as libc::c_short;
    }
    if i > 0x7fffi32 {
        return 0x7fffi32 as libc::c_short;
    }
    return i as libc::c_short;
}
// this isn't a real cheap function to call!
#[no_mangle]
pub unsafe extern "C" fn DirToByte(mut dir: *mut vec_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut bestd: libc::c_float = 0.;
    if dir.is_null() {
        return 0i32;
    }
    bestd = 0i32 as libc::c_float;
    best = 0i32;
    i = 0i32;
    while i < 162i32 {
        d = *dir.offset(0isize) * bytedirs[i as usize][0usize]
            + *dir.offset(1isize) * bytedirs[i as usize][1usize]
            + *dir.offset(2isize) * bytedirs[i as usize][2usize];
        if d > bestd {
            bestd = d;
            best = i
        }
        i += 1
    }
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn ByteToDir(mut b: libc::c_int, mut dir: *mut vec_t) {
    if b < 0i32 || b >= 162i32 {
        *dir.offset(0isize) = vec3_origin[0usize];
        *dir.offset(1isize) = vec3_origin[1usize];
        *dir.offset(2isize) = vec3_origin[2usize];
        return;
    }
    *dir.offset(0isize) = bytedirs[b as usize][0usize];
    *dir.offset(1isize) = bytedirs[b as usize][1usize];
    *dir.offset(2isize) = bytedirs[b as usize][2usize];
}
// just in case you don't want to use the macros
#[no_mangle]
pub unsafe extern "C" fn _DotProduct(mut v1: *const vec_t, mut v2: *const vec_t) -> vec_t {
    return *v1.offset(0isize) * *v2.offset(0isize)
        + *v1.offset(1isize) * *v2.offset(1isize)
        + *v1.offset(2isize) * *v2.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn _VectorSubtract(
    mut veca: *const vec_t,
    mut vecb: *const vec_t,
    mut out: *mut vec_t,
) {
    *out.offset(0isize) = *veca.offset(0isize) - *vecb.offset(0isize);
    *out.offset(1isize) = *veca.offset(1isize) - *vecb.offset(1isize);
    *out.offset(2isize) = *veca.offset(2isize) - *vecb.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn _VectorAdd(
    mut veca: *const vec_t,
    mut vecb: *const vec_t,
    mut out: *mut vec_t,
) {
    *out.offset(0isize) = *veca.offset(0isize) + *vecb.offset(0isize);
    *out.offset(1isize) = *veca.offset(1isize) + *vecb.offset(1isize);
    *out.offset(2isize) = *veca.offset(2isize) + *vecb.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn _VectorCopy(mut in_0: *const vec_t, mut out: *mut vec_t) {
    *out.offset(0isize) = *in_0.offset(0isize);
    *out.offset(1isize) = *in_0.offset(1isize);
    *out.offset(2isize) = *in_0.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn _VectorScale(
    mut in_0: *const vec_t,
    mut scale: vec_t,
    mut out: *mut vec_t,
) {
    *out.offset(0isize) = *in_0.offset(0isize) * scale;
    *out.offset(1isize) = *in_0.offset(1isize) * scale;
    *out.offset(2isize) = *in_0.offset(2isize) * scale;
}
#[no_mangle]
pub unsafe extern "C" fn _VectorMA(
    mut veca: *const vec_t,
    mut scale: libc::c_float,
    mut vecb: *const vec_t,
    mut vecc: *mut vec_t,
) {
    *vecc.offset(0isize) = *veca.offset(0isize) + scale * *vecb.offset(0isize);
    *vecc.offset(1isize) = *veca.offset(1isize) + scale * *vecb.offset(1isize);
    *vecc.offset(2isize) = *veca.offset(2isize) + scale * *vecb.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn ColorBytes3(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    *(&mut i as *mut libc::c_uint as *mut byte).offset(0isize) =
        (r * 255i32 as libc::c_float) as byte;
    *(&mut i as *mut libc::c_uint as *mut byte).offset(1isize) =
        (g * 255i32 as libc::c_float) as byte;
    *(&mut i as *mut libc::c_uint as *mut byte).offset(2isize) =
        (b * 255i32 as libc::c_float) as byte;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn ColorBytes4(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut a: libc::c_float,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    *(&mut i as *mut libc::c_uint as *mut byte).offset(0isize) =
        (r * 255i32 as libc::c_float) as byte;
    *(&mut i as *mut libc::c_uint as *mut byte).offset(1isize) =
        (g * 255i32 as libc::c_float) as byte;
    *(&mut i as *mut libc::c_uint as *mut byte).offset(2isize) =
        (b * 255i32 as libc::c_float) as byte;
    *(&mut i as *mut libc::c_uint as *mut byte).offset(3isize) =
        (a * 255i32 as libc::c_float) as byte;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn NormalizeColor(
    mut in_0: *const vec_t,
    mut out: *mut vec_t,
) -> libc::c_float {
    let mut max: libc::c_float = 0.;
    max = *in_0.offset(0isize);
    if *in_0.offset(1isize) > max {
        max = *in_0.offset(1isize)
    }
    if *in_0.offset(2isize) > max {
        max = *in_0.offset(2isize)
    }
    if 0. == max {
        let ref mut fresh1 = *out.offset(1isize);
        let ref mut fresh0 = *out.offset(2isize);
        *fresh0 = 0i32 as vec_t;
        *fresh1 = *fresh0;
        *out.offset(0isize) = *fresh1
    } else {
        *out.offset(0isize) = *in_0.offset(0isize) / max;
        *out.offset(1isize) = *in_0.offset(1isize) / max;
        *out.offset(2isize) = *in_0.offset(2isize) / max
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn RadiusFromBounds(
    mut mins: *const vec_t,
    mut maxs: *const vec_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut corner: vec3_t = [0.; 3];
    let mut a: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
    i = 0i32;
    while i < 3i32 {
        a = fabs(*mins.offset(i as isize) as libc::c_double) as libc::c_float;
        b = fabs(*maxs.offset(i as isize) as libc::c_double) as libc::c_float;
        corner[i as usize] = if a > b { a } else { b };
        i += 1
    }
    return VectorLength(corner.as_mut_ptr() as *const vec_t);
}
unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn ClearBounds(mut mins: *mut vec_t, mut maxs: *mut vec_t) {
    let ref mut fresh3 = *mins.offset(1isize);
    let ref mut fresh2 = *mins.offset(2isize);
    *fresh2 = 99999i32 as vec_t;
    *fresh3 = *fresh2;
    *mins.offset(0isize) = *fresh3;
    let ref mut fresh5 = *maxs.offset(1isize);
    let ref mut fresh4 = *maxs.offset(2isize);
    *fresh4 = -99999i32 as vec_t;
    *fresh5 = *fresh4;
    *maxs.offset(0isize) = *fresh5;
}
#[no_mangle]
pub unsafe extern "C" fn AddPointToBounds(
    mut v: *const vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) {
    if *v.offset(0isize) < *mins.offset(0isize) {
        *mins.offset(0isize) = *v.offset(0isize)
    }
    if *v.offset(0isize) > *maxs.offset(0isize) {
        *maxs.offset(0isize) = *v.offset(0isize)
    }
    if *v.offset(1isize) < *mins.offset(1isize) {
        *mins.offset(1isize) = *v.offset(1isize)
    }
    if *v.offset(1isize) > *maxs.offset(1isize) {
        *maxs.offset(1isize) = *v.offset(1isize)
    }
    if *v.offset(2isize) < *mins.offset(2isize) {
        *mins.offset(2isize) = *v.offset(2isize)
    }
    if *v.offset(2isize) > *maxs.offset(2isize) {
        *maxs.offset(2isize) = *v.offset(2isize)
    };
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
// returns vector length
#[no_mangle]
pub unsafe extern "C" fn VectorNormalize(mut v: *mut vec_t) -> vec_t {
    // NOTE: TTimo - Apple G4 altivec source uses double?
    let mut length: libc::c_float = 0.;
    let mut ilength: libc::c_float = 0.;
    length = *v.offset(0isize) * *v.offset(0isize)
        + *v.offset(1isize) * *v.offset(1isize)
        + *v.offset(2isize) * *v.offset(2isize);
    if 0. != length {
        ilength = 1i32 as libc::c_float / sqrt(length as libc::c_double) as libc::c_float;
        length *= ilength;
        let ref mut fresh6 = *v.offset(0isize);
        *fresh6 *= ilength;
        let ref mut fresh7 = *v.offset(1isize);
        *fresh7 *= ilength;
        let ref mut fresh8 = *v.offset(2isize);
        *fresh8 *= ilength
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn VectorNormalize2(mut v: *const vec_t, mut out: *mut vec_t) -> vec_t {
    let mut length: libc::c_float = 0.;
    let mut ilength: libc::c_float = 0.;
    length = *v.offset(0isize) * *v.offset(0isize)
        + *v.offset(1isize) * *v.offset(1isize)
        + *v.offset(2isize) * *v.offset(2isize);
    if 0. != length {
        ilength = 1i32 as libc::c_float / sqrt(length as libc::c_double) as libc::c_float;
        length *= ilength;
        *out.offset(0isize) = *v.offset(0isize) * ilength;
        *out.offset(1isize) = *v.offset(1isize) * ilength;
        *out.offset(2isize) = *v.offset(2isize) * ilength
    } else {
        let ref mut fresh10 = *out.offset(1isize);
        let ref mut fresh9 = *out.offset(2isize);
        *fresh9 = 0i32 as vec_t;
        *fresh10 = *fresh9;
        *out.offset(0isize) = *fresh10
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn Vector4Scale(
    mut in_0: *const vec_t,
    mut scale: vec_t,
    mut out: *mut vec_t,
) {
    *out.offset(0isize) = *in_0.offset(0isize) * scale;
    *out.offset(1isize) = *in_0.offset(1isize) * scale;
    *out.offset(2isize) = *in_0.offset(2isize) * scale;
    *out.offset(3isize) = *in_0.offset(3isize) * scale;
}
#[no_mangle]
pub unsafe extern "C" fn VectorRotate(
    mut in_0: *mut vec_t,
    mut matrix: *mut vec3_t,
    mut out: *mut vec_t,
) {
    *out.offset(0isize) = *in_0.offset(0isize) * (*matrix.offset(0isize))[0usize]
        + *in_0.offset(1isize) * (*matrix.offset(0isize))[1usize]
        + *in_0.offset(2isize) * (*matrix.offset(0isize))[2usize];
    *out.offset(1isize) = *in_0.offset(0isize) * (*matrix.offset(1isize))[0usize]
        + *in_0.offset(1isize) * (*matrix.offset(1isize))[1usize]
        + *in_0.offset(2isize) * (*matrix.offset(1isize))[2usize];
    *out.offset(2isize) = *in_0.offset(0isize) * (*matrix.offset(2isize))[0usize]
        + *in_0.offset(1isize) * (*matrix.offset(2isize))[1usize]
        + *in_0.offset(2isize) * (*matrix.offset(2isize))[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn Q_log2(mut val: libc::c_int) -> libc::c_int {
    let mut answer: libc::c_int = 0;
    answer = 0i32;
    loop {
        val >>= 1i32;
        if !(val != 0i32) {
            break;
        }
        answer += 1
    }
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn Q_acos(mut c: libc::c_float) -> libc::c_float {
    let mut angle: libc::c_float = 0.;
    angle = acos(c as libc::c_double) as libc::c_float;
    if angle as libc::c_double > 3.14159265358979323846f64 {
        return 3.14159265358979323846f64 as libc::c_float;
    }
    if (angle as libc::c_double) < -3.14159265358979323846f64 {
        return 3.14159265358979323846f64 as libc::c_float;
    }
    return angle;
}
#[no_mangle]
pub unsafe extern "C" fn Q_rand(mut seed: *mut libc::c_int) -> libc::c_int {
    *seed = 69069u32
        .wrapping_mul(*seed as libc::c_uint)
        .wrapping_add(1u32) as libc::c_int;
    return *seed;
}
#[no_mangle]
pub unsafe extern "C" fn Q_random(mut seed: *mut libc::c_int) -> libc::c_float {
    return (Q_rand(seed) & 0xffffi32) as libc::c_float / 0x10000i32 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Q_crandom(mut seed: *mut libc::c_int) -> libc::c_float {
    return (2.0f64 * (Q_random(seed) as libc::c_double - 0.5f64)) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn vectoangles(mut value1: *const vec_t, mut angles: *mut vec_t) {
    let mut forward: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    if *value1.offset(1isize) == 0i32 as libc::c_float
        && *value1.offset(0isize) == 0i32 as libc::c_float
    {
        yaw = 0i32 as libc::c_float;
        if *value1.offset(2isize) > 0i32 as libc::c_float {
            pitch = 90i32 as libc::c_float
        } else {
            pitch = 270i32 as libc::c_float
        }
    } else {
        if 0. != *value1.offset(0isize) {
            yaw = (atan2(
                *value1.offset(1isize) as libc::c_double,
                *value1.offset(0isize) as libc::c_double,
            ) * 180i32 as libc::c_double
                / 3.14159265358979323846f64) as libc::c_float
        } else if *value1.offset(1isize) > 0i32 as libc::c_float {
            yaw = 90i32 as libc::c_float
        } else {
            yaw = 270i32 as libc::c_float
        }
        if yaw < 0i32 as libc::c_float {
            yaw += 360i32 as libc::c_float
        }
        forward = sqrt(
            (*value1.offset(0isize) * *value1.offset(0isize)
                + *value1.offset(1isize) * *value1.offset(1isize)) as libc::c_double,
        ) as libc::c_float;
        pitch = (atan2(
            *value1.offset(2isize) as libc::c_double,
            forward as libc::c_double,
        ) * 180i32 as libc::c_double
            / 3.14159265358979323846f64) as libc::c_float;
        if pitch < 0i32 as libc::c_float {
            pitch += 360i32 as libc::c_float
        }
    }
    *angles.offset(0isize) = -pitch;
    *angles.offset(1isize) = yaw;
    *angles.offset(2isize) = 0i32 as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn AnglesToAxis(mut angles: *const vec_t, mut axis: *mut vec3_t) {
    let mut right: vec3_t = [0.; 3];
    AngleVectors(
        angles,
        (*axis.offset(0isize)).as_mut_ptr(),
        right.as_mut_ptr(),
        (*axis.offset(2isize)).as_mut_ptr(),
    );
    (*axis.offset(1isize))[0usize] = vec3_origin[0usize] - right[0usize];
    (*axis.offset(1isize))[1usize] = vec3_origin[1usize] - right[1usize];
    (*axis.offset(1isize))[2usize] = vec3_origin[2usize] - right[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn AngleVectors(
    mut angles: *const vec_t,
    mut forward: *mut vec_t,
    mut right: *mut vec_t,
    mut up: *mut vec_t,
) {
    let mut angle: libc::c_float = 0.;
    static mut sr: libc::c_float = 0.;
    static mut sp: libc::c_float = 0.;
    static mut sy: libc::c_float = 0.;
    static mut cr: libc::c_float = 0.;
    static mut cp: libc::c_float = 0.;
    static mut cy: libc::c_float = 0.;
    angle = (*angles.offset(1isize) as libc::c_double
        * (3.14159265358979323846f64 * 2i32 as libc::c_double / 360i32 as libc::c_double))
        as libc::c_float;
    sy = sin(angle as libc::c_double) as libc::c_float;
    cy = cos(angle as libc::c_double) as libc::c_float;
    angle = (*angles.offset(0isize) as libc::c_double
        * (3.14159265358979323846f64 * 2i32 as libc::c_double / 360i32 as libc::c_double))
        as libc::c_float;
    sp = sin(angle as libc::c_double) as libc::c_float;
    cp = cos(angle as libc::c_double) as libc::c_float;
    angle = (*angles.offset(2isize) as libc::c_double
        * (3.14159265358979323846f64 * 2i32 as libc::c_double / 360i32 as libc::c_double))
        as libc::c_float;
    sr = sin(angle as libc::c_double) as libc::c_float;
    cr = cos(angle as libc::c_double) as libc::c_float;
    if !forward.is_null() {
        *forward.offset(0isize) = cp * cy;
        *forward.offset(1isize) = cp * sy;
        *forward.offset(2isize) = -sp
    }
    if !right.is_null() {
        *right.offset(0isize) =
            -1i32 as libc::c_float * sr * sp * cy + -1i32 as libc::c_float * cr * -sy;
        *right.offset(1isize) =
            -1i32 as libc::c_float * sr * sp * sy + -1i32 as libc::c_float * cr * cy;
        *right.offset(2isize) = -1i32 as libc::c_float * sr * cp
    }
    if !up.is_null() {
        *up.offset(0isize) = cr * sp * cy + -sr * -sy;
        *up.offset(1isize) = cr * sp * sy + -sr * cy;
        *up.offset(2isize) = cr * cp
    };
}
#[no_mangle]
pub unsafe extern "C" fn AxisClear(mut axis: *mut vec3_t) {
    (*axis.offset(0isize))[0usize] = 1i32 as vec_t;
    (*axis.offset(0isize))[1usize] = 0i32 as vec_t;
    (*axis.offset(0isize))[2usize] = 0i32 as vec_t;
    (*axis.offset(1isize))[0usize] = 0i32 as vec_t;
    (*axis.offset(1isize))[1usize] = 1i32 as vec_t;
    (*axis.offset(1isize))[2usize] = 0i32 as vec_t;
    (*axis.offset(2isize))[0usize] = 0i32 as vec_t;
    (*axis.offset(2isize))[1usize] = 0i32 as vec_t;
    (*axis.offset(2isize))[2usize] = 1i32 as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn AxisCopy(mut in_0: *mut vec3_t, mut out: *mut vec3_t) {
    (*out.offset(0isize))[0usize] = (*in_0.offset(0isize))[0usize];
    (*out.offset(0isize))[1usize] = (*in_0.offset(0isize))[1usize];
    (*out.offset(0isize))[2usize] = (*in_0.offset(0isize))[2usize];
    (*out.offset(1isize))[0usize] = (*in_0.offset(1isize))[0usize];
    (*out.offset(1isize))[1usize] = (*in_0.offset(1isize))[1usize];
    (*out.offset(1isize))[2usize] = (*in_0.offset(1isize))[2usize];
    (*out.offset(2isize))[0usize] = (*in_0.offset(2isize))[0usize];
    (*out.offset(2isize))[1usize] = (*in_0.offset(2isize))[1usize];
    (*out.offset(2isize))[2usize] = (*in_0.offset(2isize))[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn SetPlaneSignbits(mut out: *mut cplane_t) {
    let mut bits: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    bits = 0i32;
    j = 0i32;
    while j < 3i32 {
        if (*out).normal[j as usize] < 0i32 as libc::c_float {
            bits |= 1i32 << j
        }
        j += 1
    }
    (*out).signbits = bits as byte;
}
#[no_mangle]
pub unsafe extern "C" fn BoxOnPlaneSide(
    mut emins: *mut vec_t,
    mut emaxs: *mut vec_t,
    mut p: *mut cplane_s,
) -> libc::c_int {
    let mut dist: [libc::c_float; 2] = [0.; 2];
    let mut sides: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if ((*p).type_0 as libc::c_int) < 3i32 {
        if (*p).dist <= *emins.offset((*p).type_0 as isize) {
            return 1i32;
        }
        if (*p).dist >= *emaxs.offset((*p).type_0 as isize) {
            return 2i32;
        }
        return 3i32;
    }
    dist[1usize] = 0i32 as libc::c_float;
    dist[0usize] = dist[1usize];
    if ((*p).signbits as libc::c_int) < 8i32 {
        i = 0i32;
        while i < 3i32 {
            b = (*p).signbits as libc::c_int >> i & 1i32;
            dist[b as usize] += (*p).normal[i as usize] * *emaxs.offset(i as isize);
            dist[(0 == b) as libc::c_int as usize] +=
                (*p).normal[i as usize] * *emins.offset(i as isize);
            i += 1
        }
    }
    sides = 0i32;
    if dist[0usize] >= (*p).dist {
        sides = 1i32
    }
    if dist[1usize] < (*p).dist {
        sides |= 2i32
    }
    return sides;
}
#[no_mangle]
pub unsafe extern "C" fn BoundsIntersect(
    mut mins: *const vec_t,
    mut maxs: *const vec_t,
    mut mins2: *const vec_t,
    mut maxs2: *const vec_t,
) -> qboolean {
    if *maxs.offset(0isize) < *mins2.offset(0isize)
        || *maxs.offset(1isize) < *mins2.offset(1isize)
        || *maxs.offset(2isize) < *mins2.offset(2isize)
        || *mins.offset(0isize) > *maxs2.offset(0isize)
        || *mins.offset(1isize) > *maxs2.offset(1isize)
        || *mins.offset(2isize) > *maxs2.offset(2isize)
    {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn BoundsIntersectSphere(
    mut mins: *const vec_t,
    mut maxs: *const vec_t,
    mut origin: *const vec_t,
    mut radius: vec_t,
) -> qboolean {
    if *origin.offset(0isize) - radius > *maxs.offset(0isize)
        || *origin.offset(0isize) + radius < *mins.offset(0isize)
        || *origin.offset(1isize) - radius > *maxs.offset(1isize)
        || *origin.offset(1isize) + radius < *mins.offset(1isize)
        || *origin.offset(2isize) - radius > *maxs.offset(2isize)
        || *origin.offset(2isize) + radius < *mins.offset(2isize)
    {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn BoundsIntersectPoint(
    mut mins: *const vec_t,
    mut maxs: *const vec_t,
    mut origin: *const vec_t,
) -> qboolean {
    if *origin.offset(0isize) > *maxs.offset(0isize)
        || *origin.offset(0isize) < *mins.offset(0isize)
        || *origin.offset(1isize) > *maxs.offset(1isize)
        || *origin.offset(1isize) < *mins.offset(1isize)
        || *origin.offset(2isize) > *maxs.offset(2isize)
        || *origin.offset(2isize) < *mins.offset(2isize)
    {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn AngleMod(mut a: libc::c_float) -> libc::c_float {
    a = (360.0f64 / 65536i32 as libc::c_double
        * ((a as libc::c_double * (65536i32 as libc::c_double / 360.0f64)) as libc::c_int
            & 65535i32) as libc::c_double) as libc::c_float;
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn LerpAngle(
    mut from: libc::c_float,
    mut to: libc::c_float,
    mut frac: libc::c_float,
) -> libc::c_float {
    let mut a: libc::c_float = 0.;
    if to - from > 180i32 as libc::c_float {
        to -= 360i32 as libc::c_float
    }
    if to - from < -180i32 as libc::c_float {
        to += 360i32 as libc::c_float
    }
    a = from + frac * (to - from);
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn AngleSubtract(
    mut a1: libc::c_float,
    mut a2: libc::c_float,
) -> libc::c_float {
    let mut a: libc::c_float = 0.;
    a = a1 - a2;
    while a > 180i32 as libc::c_float {
        a -= 360i32 as libc::c_float
    }
    while a < -180i32 as libc::c_float {
        a += 360i32 as libc::c_float
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn AnglesSubtract(
    mut v1: *mut vec_t,
    mut v2: *mut vec_t,
    mut v3: *mut vec_t,
) {
    *v3.offset(0isize) = AngleSubtract(*v1.offset(0isize), *v2.offset(0isize));
    *v3.offset(1isize) = AngleSubtract(*v1.offset(1isize), *v2.offset(1isize));
    *v3.offset(2isize) = AngleSubtract(*v1.offset(2isize), *v2.offset(2isize));
}
#[no_mangle]
pub unsafe extern "C" fn AngleNormalize360(mut angle: libc::c_float) -> libc::c_float {
    return (360.0f64 / 65536i32 as libc::c_double
        * ((angle as libc::c_double * (65536i32 as libc::c_double / 360.0f64)) as libc::c_int
            & 65535i32) as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn AngleNormalize180(mut angle: libc::c_float) -> libc::c_float {
    angle = AngleNormalize360(angle);
    if angle as libc::c_double > 180.0f64 {
        angle = (angle as libc::c_double - 360.0f64) as libc::c_float
    }
    return angle;
}
#[no_mangle]
pub unsafe extern "C" fn AngleDelta(
    mut angle1: libc::c_float,
    mut angle2: libc::c_float,
) -> libc::c_float {
    return AngleNormalize180(angle1 - angle2);
}
#[no_mangle]
pub unsafe extern "C" fn PlaneFromPoints(
    mut plane: *mut vec_t,
    mut a: *const vec_t,
    mut b: *const vec_t,
    mut c: *const vec_t,
) -> qboolean {
    let mut d1: vec3_t = [0.; 3];
    let mut d2: vec3_t = [0.; 3];
    d1[0usize] = *b.offset(0isize) - *a.offset(0isize);
    d1[1usize] = *b.offset(1isize) - *a.offset(1isize);
    d1[2usize] = *b.offset(2isize) - *a.offset(2isize);
    d2[0usize] = *c.offset(0isize) - *a.offset(0isize);
    d2[1usize] = *c.offset(1isize) - *a.offset(1isize);
    d2[2usize] = *c.offset(2isize) - *a.offset(2isize);
    CrossProduct(
        d2.as_mut_ptr() as *const vec_t,
        d1.as_mut_ptr() as *const vec_t,
        plane,
    );
    if VectorNormalize(plane) == 0i32 as libc::c_float {
        return qfalse;
    }
    *plane.offset(3isize) = *a.offset(0isize) * *plane.offset(0isize)
        + *a.offset(1isize) * *plane.offset(1isize)
        + *a.offset(2isize) * *plane.offset(2isize);
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn ProjectPointOnPlane(
    mut dst: *mut vec_t,
    mut p: *const vec_t,
    mut normal: *const vec_t,
) {
    let mut d: libc::c_float = 0.;
    let mut n: vec3_t = [0.; 3];
    let mut inv_denom: libc::c_float = 0.;
    inv_denom = *normal.offset(0isize) * *normal.offset(0isize)
        + *normal.offset(1isize) * *normal.offset(1isize)
        + *normal.offset(2isize) * *normal.offset(2isize);
    if Q_fabs(inv_denom) != 0.0f32 {
    } else {
        __assert_fail(
            b"Q_fabs(inv_denom) != 0.0f\x00" as *const u8 as *const libc::c_char,
            b"code/qcommon/q_math.c\x00" as *const u8 as *const libc::c_char,
            450i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 64], &[libc::c_char; 64]>(
                b"void ProjectPointOnPlane(vec_t *, const vec_t *, const vec_t *)\x00",
            ))
            .as_ptr(),
        );
    }
    inv_denom = 1.0f32 / inv_denom;
    d = (*normal.offset(0isize) * *p.offset(0isize)
        + *normal.offset(1isize) * *p.offset(1isize)
        + *normal.offset(2isize) * *p.offset(2isize))
        * inv_denom;
    n[0usize] = *normal.offset(0isize) * inv_denom;
    n[1usize] = *normal.offset(1isize) * inv_denom;
    n[2usize] = *normal.offset(2isize) * inv_denom;
    *dst.offset(0isize) = *p.offset(0isize) - d * n[0usize];
    *dst.offset(1isize) = *p.offset(1isize) - d * n[1usize];
    *dst.offset(2isize) = *p.offset(2isize) - d * n[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn RotatePointAroundVector(
    mut dst: *mut vec_t,
    mut dir: *const vec_t,
    mut point: *const vec_t,
    mut degrees: libc::c_float,
) {
    let mut m: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut im: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut zrot: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut tmpmat: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut rot: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut i: libc::c_int = 0;
    let mut vr: vec3_t = [0.; 3];
    let mut vup: vec3_t = [0.; 3];
    let mut vf: vec3_t = [0.; 3];
    let mut rad: libc::c_float = 0.;
    vf[0usize] = *dir.offset(0isize);
    vf[1usize] = *dir.offset(1isize);
    vf[2usize] = *dir.offset(2isize);
    PerpendicularVector(vr.as_mut_ptr(), dir);
    CrossProduct(
        vr.as_mut_ptr() as *const vec_t,
        vf.as_mut_ptr() as *const vec_t,
        vup.as_mut_ptr(),
    );
    m[0usize][0usize] = vr[0usize];
    m[1usize][0usize] = vr[1usize];
    m[2usize][0usize] = vr[2usize];
    m[0usize][1usize] = vup[0usize];
    m[1usize][1usize] = vup[1usize];
    m[2usize][1usize] = vup[2usize];
    m[0usize][2usize] = vf[0usize];
    m[1usize][2usize] = vf[1usize];
    m[2usize][2usize] = vf[2usize];
    memcpy(
        im.as_mut_ptr() as *mut libc::c_void,
        m.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[[libc::c_float; 3]; 3]>() as libc::c_ulong,
    );
    im[0usize][1usize] = m[1usize][0usize];
    im[0usize][2usize] = m[2usize][0usize];
    im[1usize][0usize] = m[0usize][1usize];
    im[1usize][2usize] = m[2usize][1usize];
    im[2usize][0usize] = m[0usize][2usize];
    im[2usize][1usize] = m[1usize][2usize];
    memset(
        zrot.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[[libc::c_float; 3]; 3]>() as libc::c_ulong,
    );
    zrot[2usize][2usize] = 1.0f32;
    zrot[1usize][1usize] = zrot[2usize][2usize];
    zrot[0usize][0usize] = zrot[1usize][1usize];
    rad = (degrees as libc::c_double * 3.14159265358979323846f64 / 180.0f32 as libc::c_double)
        as libc::c_float;
    zrot[0usize][0usize] = cos(rad as libc::c_double) as libc::c_float;
    zrot[0usize][1usize] = sin(rad as libc::c_double) as libc::c_float;
    zrot[1usize][0usize] = -sin(rad as libc::c_double) as libc::c_float;
    zrot[1usize][1usize] = cos(rad as libc::c_double) as libc::c_float;
    MatrixMultiply(m.as_mut_ptr(), zrot.as_mut_ptr(), tmpmat.as_mut_ptr());
    MatrixMultiply(tmpmat.as_mut_ptr(), im.as_mut_ptr(), rot.as_mut_ptr());
    i = 0i32;
    while i < 3i32 {
        *dst.offset(i as isize) = rot[i as usize][0usize] * *point.offset(0isize)
            + rot[i as usize][1usize] * *point.offset(1isize)
            + rot[i as usize][2usize] * *point.offset(2isize);
        i += 1
    }
}
// perpendicular vector could be replaced by this
//int	PlaneTypeForNormal (vec3_t normal);
#[no_mangle]
pub unsafe extern "C" fn MatrixMultiply(
    mut in1: *mut [libc::c_float; 3],
    mut in2: *mut [libc::c_float; 3],
    mut out: *mut [libc::c_float; 3],
) {
    (*out.offset(0isize))[0usize] = (*in1.offset(0isize))[0usize] * (*in2.offset(0isize))[0usize]
        + (*in1.offset(0isize))[1usize] * (*in2.offset(1isize))[0usize]
        + (*in1.offset(0isize))[2usize] * (*in2.offset(2isize))[0usize];
    (*out.offset(0isize))[1usize] = (*in1.offset(0isize))[0usize] * (*in2.offset(0isize))[1usize]
        + (*in1.offset(0isize))[1usize] * (*in2.offset(1isize))[1usize]
        + (*in1.offset(0isize))[2usize] * (*in2.offset(2isize))[1usize];
    (*out.offset(0isize))[2usize] = (*in1.offset(0isize))[0usize] * (*in2.offset(0isize))[2usize]
        + (*in1.offset(0isize))[1usize] * (*in2.offset(1isize))[2usize]
        + (*in1.offset(0isize))[2usize] * (*in2.offset(2isize))[2usize];
    (*out.offset(1isize))[0usize] = (*in1.offset(1isize))[0usize] * (*in2.offset(0isize))[0usize]
        + (*in1.offset(1isize))[1usize] * (*in2.offset(1isize))[0usize]
        + (*in1.offset(1isize))[2usize] * (*in2.offset(2isize))[0usize];
    (*out.offset(1isize))[1usize] = (*in1.offset(1isize))[0usize] * (*in2.offset(0isize))[1usize]
        + (*in1.offset(1isize))[1usize] * (*in2.offset(1isize))[1usize]
        + (*in1.offset(1isize))[2usize] * (*in2.offset(2isize))[1usize];
    (*out.offset(1isize))[2usize] = (*in1.offset(1isize))[0usize] * (*in2.offset(0isize))[2usize]
        + (*in1.offset(1isize))[1usize] * (*in2.offset(1isize))[2usize]
        + (*in1.offset(1isize))[2usize] * (*in2.offset(2isize))[2usize];
    (*out.offset(2isize))[0usize] = (*in1.offset(2isize))[0usize] * (*in2.offset(0isize))[0usize]
        + (*in1.offset(2isize))[1usize] * (*in2.offset(1isize))[0usize]
        + (*in1.offset(2isize))[2usize] * (*in2.offset(2isize))[0usize];
    (*out.offset(2isize))[1usize] = (*in1.offset(2isize))[0usize] * (*in2.offset(0isize))[1usize]
        + (*in1.offset(2isize))[1usize] * (*in2.offset(1isize))[1usize]
        + (*in1.offset(2isize))[2usize] * (*in2.offset(2isize))[1usize];
    (*out.offset(2isize))[2usize] = (*in1.offset(2isize))[0usize] * (*in2.offset(0isize))[2usize]
        + (*in1.offset(2isize))[1usize] * (*in2.offset(1isize))[2usize]
        + (*in1.offset(2isize))[2usize] * (*in2.offset(2isize))[2usize];
}
#[no_mangle]
pub unsafe extern "C" fn PerpendicularVector(mut dst: *mut vec_t, mut src: *const vec_t) {
    let mut pos: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut minelem: libc::c_float = 1.0f32;
    let mut tempvec: vec3_t = [0.; 3];
    pos = 0i32;
    i = 0i32;
    while i < 3i32 {
        if fabs(*src.offset(i as isize) as libc::c_double) < minelem as libc::c_double {
            pos = i;
            minelem = fabs(*src.offset(i as isize) as libc::c_double) as libc::c_float
        }
        i += 1
    }
    tempvec[2usize] = 0.0f32;
    tempvec[1usize] = tempvec[2usize];
    tempvec[0usize] = tempvec[1usize];
    tempvec[pos as usize] = 1.0f32;
    ProjectPointOnPlane(dst, tempvec.as_mut_ptr() as *const vec_t, src);
    VectorNormalize(dst);
}
#[no_mangle]
pub unsafe extern "C" fn RotateAroundDirection(mut axis: *mut vec3_t, mut yaw: libc::c_float) {
    PerpendicularVector(
        (*axis.offset(1isize)).as_mut_ptr(),
        (*axis.offset(0isize)).as_mut_ptr() as *const vec_t,
    );
    if 0. != yaw {
        let mut temp: vec3_t = [0.; 3];
        temp[0usize] = (*axis.offset(1isize))[0usize];
        temp[1usize] = (*axis.offset(1isize))[1usize];
        temp[2usize] = (*axis.offset(1isize))[2usize];
        RotatePointAroundVector(
            (*axis.offset(1isize)).as_mut_ptr(),
            (*axis.offset(0isize)).as_mut_ptr() as *const vec_t,
            temp.as_mut_ptr() as *const vec_t,
            yaw,
        );
    }
    CrossProduct(
        (*axis.offset(0isize)).as_mut_ptr() as *const vec_t,
        (*axis.offset(1isize)).as_mut_ptr() as *const vec_t,
        (*axis.offset(2isize)).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn MakeNormalVectors(
    mut forward: *const vec_t,
    mut right: *mut vec_t,
    mut up: *mut vec_t,
) {
    let mut d: libc::c_float = 0.;
    *right.offset(1isize) = -*forward.offset(0isize);
    *right.offset(2isize) = *forward.offset(1isize);
    *right.offset(0isize) = *forward.offset(2isize);
    d = *right.offset(0isize) * *forward.offset(0isize)
        + *right.offset(1isize) * *forward.offset(1isize)
        + *right.offset(2isize) * *forward.offset(2isize);
    *right.offset(0isize) = *right.offset(0isize) + *forward.offset(0isize) * -d;
    *right.offset(1isize) = *right.offset(1isize) + *forward.offset(1isize) * -d;
    *right.offset(2isize) = *right.offset(2isize) + *forward.offset(2isize) * -d;
    VectorNormalize(right);
    CrossProduct(right as *const vec_t, forward, up);
}
