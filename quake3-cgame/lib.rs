#![feature(libc)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(ptr_wrapping_offset_from)]
#![feature(label_break_value)]
#![feature(const_raw_ptr_to_usize_cast)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]
#![feature(custom_attribute)]

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
extern crate libc;

#[path = "src/cg_consolecmds.rs"]
pub mod cg_consolecmds;
#[path = "src/cg_draw.rs"]
pub mod cg_draw;
#[path = "src/cg_drawtools.rs"]
pub mod cg_drawtools;
#[path = "src/cg_effects.rs"]
pub mod cg_effects;
#[path = "src/cg_ents.rs"]
pub mod cg_ents;
#[path = "src/cg_event.rs"]
pub mod cg_event;
#[path = "src/cg_info.rs"]
pub mod cg_info;
#[path = "src/cg_localents.rs"]
pub mod cg_localents;
#[path = "src/cg_main.rs"]
pub mod cg_main;
#[path = "src/cg_marks.rs"]
pub mod cg_marks;
#[path = "src/cg_particles.rs"]
pub mod cg_particles;
#[path = "src/cg_players.rs"]
pub mod cg_players;
#[path = "src/cg_playerstate.rs"]
pub mod cg_playerstate;
#[path = "src/cg_predict.rs"]
pub mod cg_predict;
#[path = "src/cg_scoreboard.rs"]
pub mod cg_scoreboard;
#[path = "src/cg_servercmds.rs"]
pub mod cg_servercmds;
#[path = "src/cg_snapshot.rs"]
pub mod cg_snapshot;
#[path = "src/cg_view.rs"]
pub mod cg_view;
#[path = "src/cg_weapons.rs"]
pub mod cg_weapons;
mod q_shared_h {
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
        cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail,
        cg_oldRocket, cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll,
        cg_scorePlum, cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats,
        cg_swingSpeed, cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly,
        cg_thirdPerson, cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale,
        cg_timescaleFadeEnd, cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength,
        cg_tracerWidth, cg_trueLightning, cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed,
        pmove_msec, CG_Argv, CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer,
        CG_LastAttacker, CG_StartMusic, CG_UpdateCvars,
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
        CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f,
        CG_OutOfAmmoChange, CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire,
        CG_Weapon_f,
    };
    extern "C" {

        #[no_mangle]
        pub fn Q_crandom(seed: *mut libc::c_int) -> libc::c_float;

        #[no_mangle]
        pub fn RotatePointAroundVector(
            dst: *mut vec_t,
            dir: *const vec_t,
            point: *const vec_t,
            degrees: libc::c_float,
        );
        // perpendicular vector could be replaced by this
        //int	PlaneTypeForNormal (vec3_t normal);

        #[no_mangle]
        pub fn COM_StripExtension(
            in_0: *const libc::c_char,
            out: *mut libc::c_char,
            destsize: libc::c_int,
        );

    }
    extern "C" {
        #[no_mangle]
        pub fn vectoangles(value1: *const vec_t, angles: *mut vec_t);

    // portable case insensitive compare

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    extern "C" {

        #[no_mangle]
        pub fn VectorNormalize2(v: *const vec_t, out: *mut vec_t) -> vec_t;

        // perpendicular vector could be replaced by this
        //int	PlaneTypeForNormal (vec3_t normal);
        #[no_mangle]
        pub fn MatrixMultiply(
            in1: *mut [libc::c_float; 3],
            in2: *mut [libc::c_float; 3],
            out: *mut [libc::c_float; 3],
        );

        #[no_mangle]
        pub fn PerpendicularVector(dst: *mut vec_t, src: *const vec_t);
    }
    extern "C" {
        #[no_mangle]
        pub static mut axisDefault: [vec3_t; 3];

        #[no_mangle]
        pub fn Q_random(seed: *mut libc::c_int) -> libc::c_float;

        #[no_mangle]
        pub fn AxisCopy(in_0: *mut vec3_t, out: *mut vec3_t);
        #[no_mangle]
        pub fn RotateAroundDirection(axis: *mut vec3_t, yaw: libc::c_float);
    }
    extern "C" {

        // buffer size safe library replacements

        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int, src: *const libc::c_char);
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_READ: fsMode_t = 0;
    extern "C" {

        #[no_mangle]
        pub fn AxisClear(axis: *mut vec3_t);

        // buffer size safe library replacements

        // strlen that discounts Quake color sequences
        #[no_mangle]
        pub fn Q_PrintStrlen(string: *const libc::c_char) -> libc::c_int;
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    //=============================================
    //
    // key / value info strings
    //

    }
    extern "C" {

        // buffer size safe library replacements

        // removes color sequences from string
        #[no_mangle]
        pub fn Q_CleanStr(string: *mut libc::c_char) -> *mut libc::c_char;
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    //=============================================
    //
    // key / value info strings
    //

    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct markFragment_t {
        pub firstPoint: libc::c_int,
        pub numPoints: libc::c_int,
    }
    extern "C" {
        #[no_mangle]
        pub fn Q_IsColorString(p: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub static mut g_color_table: [vec4_t; 8];
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct usercmd_s {
        pub serverTime: libc::c_int,
        pub angles: [libc::c_int; 3],
        pub buttons: libc::c_int,
        pub weapon: byte,
        pub forwardmove: libc::c_schar,
        pub rightmove: libc::c_schar,
        pub upmove: libc::c_schar,
    }
    pub type fsMode_t = libc::c_uint;
    pub const FS_WRITE: fsMode_t = 1;
    extern "C" {
        #[no_mangle]
        pub fn ByteToDir(b: libc::c_int, dir: *mut vec_t);

    // buffer size safe library replacements

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    //=============================================
    //
    // key / value info strings
    //

    }
    pub const FS_APPEND: fsMode_t = 2;
    extern "C" {

        #[no_mangle]
        pub fn LerpAngle(
            from: libc::c_float,
            to: libc::c_float,
            frac: libc::c_float,
        ) -> libc::c_float;
    }
    pub const CHAN_ITEM: unnamed = 4;
    pub const CHAN_LOCAL_SOUND: unnamed = 6;
    pub type cvarHandle_t = libc::c_int;
    extern "C" {
        #[no_mangle]
        pub static mut colorWhite: vec4_t;

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    extern "C" {

        #[no_mangle]
        pub fn AngleVectors(
            angles: *const vec_t,
            forward: *mut vec_t,
            right: *mut vec_t,
            up: *mut vec_t,
        );
    // buffer size safe library replacements

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub type cplane_t = cplane_s;
    pub const CHAN_LOCAL: unnamed = 1;
    pub type fileHandle_t = libc::c_int;
    extern "C" {
        #[no_mangle]
        pub static mut vec3_origin: vec3_t;
        #[no_mangle]
        pub fn VectorNormalize(v: *mut vec_t) -> vec_t;
        #[no_mangle]
        pub fn AnglesToAxis(angles: *const vec_t, axis: *mut vec3_t);
        #[no_mangle]
        pub fn AngleMod(a: libc::c_float) -> libc::c_float;
        #[no_mangle]
        pub fn AngleSubtract(a1: libc::c_float, a2: libc::c_float) -> libc::c_float;
        #[no_mangle]
        pub fn AnglesSubtract(v1: *mut vec_t, v2: *mut vec_t, v3: *mut vec_t);
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_sprintf(
            dest: *mut libc::c_char,
            size: libc::c_int,
            fmt: *const libc::c_char,
            ...
        ) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char, destsize: libc::c_int);
        //=============================================
        /*
        short	BigShort(short l);
        short	LittleShort(short l);
        int		BigLong (int l);
        int		LittleLong (int l);
        qint64  BigLong64 (qint64 l);
        qint64  LittleLong64 (qint64 l);
        float	BigFloat (const float *l);
        float	LittleFloat (const float *l);

        void	Swap_Init (void);
        */
        #[no_mangle]
        pub fn va(format: *mut libc::c_char, ...) -> *mut libc::c_char;
        //=============================================
        //
        // key / value info strings
        //
        #[no_mangle]
        pub fn Info_ValueForKey(
            s: *const libc::c_char,
            key: *const libc::c_char,
        ) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
    pub const CHAN_BODY: unnamed = 5;
    pub type qhandle_t = libc::c_int;
    pub const TR_LINEAR: trType_t = 2;
    pub const CHAN_AUTO: unnamed = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct orientation_t {
        pub origin: vec3_t,
        pub axis: [vec3_t; 3],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct trace_t {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub surfaceFlags: libc::c_int,
        pub contents: libc::c_int,
        pub entityNum: libc::c_int,
    }
    pub const CHAN_ANNOUNCER: unnamed = 7;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct trajectory_t {
        pub trType: trType_t,
        pub trTime: libc::c_int,
        pub trDuration: libc::c_int,
        pub trBase: vec3_t,
        pub trDelta: vec3_t,
    }
    pub type byte = libc::c_uchar;
    pub type vec_t = libc::c_float;
    pub type sfxHandle_t = libc::c_int;
    pub type usercmd_t = usercmd_s;
    pub type trType_t = libc::c_uint;
    pub type playerState_t = playerState_s;
    pub type vec3_t = [vec_t; 3];
    pub const TR_GRAVITY: trType_t = 5;
    pub const TR_STATIONARY: trType_t = 0;
    pub const qfalse: qboolean = 0;
    pub const TR_SINE: trType_t = 4;
    pub const qtrue: qboolean = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct gameState_t {
        pub stringOffsets: [libc::c_int; 1024],
        pub stringData: [libc::c_char; 16000],
        pub dataCount: libc::c_int,
    }
    pub type vec4_t = [vec_t; 4];
    pub type unnamed = libc::c_uint;
    pub const TR_LINEAR_STOP: trType_t = 3;
    pub const CHAN_VOICE: unnamed = 3;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cplane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: byte,
        pub signbits: byte,
        pub pad: [byte; 2],
    }
    pub type qboolean = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct playerState_s {
        pub commandTime: libc::c_int,
        pub pm_type: libc::c_int,
        pub bobCycle: libc::c_int,
        pub pm_flags: libc::c_int,
        pub pm_time: libc::c_int,
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub weaponTime: libc::c_int,
        pub gravity: libc::c_int,
        pub speed: libc::c_int,
        pub delta_angles: [libc::c_int; 3],
        pub groundEntityNum: libc::c_int,
        pub legsTimer: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoTimer: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub movementDir: libc::c_int,
        pub grapplePoint: vec3_t,
        pub eFlags: libc::c_int,
        pub eventSequence: libc::c_int,
        pub events: [libc::c_int; 2],
        pub eventParms: [libc::c_int; 2],
        pub externalEvent: libc::c_int,
        pub externalEventParm: libc::c_int,
        pub externalEventTime: libc::c_int,
        pub clientNum: libc::c_int,
        pub weapon: libc::c_int,
        pub weaponstate: libc::c_int,
        pub viewangles: vec3_t,
        pub viewheight: libc::c_int,
        pub damageEvent: libc::c_int,
        pub damageYaw: libc::c_int,
        pub damagePitch: libc::c_int,
        pub damageCount: libc::c_int,
        pub stats: [libc::c_int; 16],
        pub persistant: [libc::c_int; 16],
        pub powerups: [libc::c_int; 16],
        pub ammo: [libc::c_int; 16],
        pub generic1: libc::c_int,
        pub loopSound: libc::c_int,
        pub jumppad_ent: libc::c_int,
        pub ping: libc::c_int,
        pub pmove_framecount: libc::c_int,
        pub jumppad_frame: libc::c_int,
        pub entityEventSequence: libc::c_int,
    }
    pub const TR_INTERPOLATE: trType_t = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct entityState_s {
        pub number: libc::c_int,
        pub eType: libc::c_int,
        pub eFlags: libc::c_int,
        pub pos: trajectory_t,
        pub apos: trajectory_t,
        pub time: libc::c_int,
        pub time2: libc::c_int,
        pub origin: vec3_t,
        pub origin2: vec3_t,
        pub angles: vec3_t,
        pub angles2: vec3_t,
        pub otherEntityNum: libc::c_int,
        pub otherEntityNum2: libc::c_int,
        pub groundEntityNum: libc::c_int,
        pub constantLight: libc::c_int,
        pub loopSound: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub clientNum: libc::c_int,
        pub frame: libc::c_int,
        pub solid: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub generic1: libc::c_int,
    }
    pub type clipHandle_t = libc::c_int;
    pub type entityState_t = entityState_s;
    pub const CHAN_WEAPON: unnamed = 2;
    use libc;
}
mod cg_local_h {
    use bg_public_h::{animation_t, gametype_t, gender_t, gitem_t, team_t};
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
        cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail,
        cg_oldRocket, cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll,
        cg_scorePlum, cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats,
        cg_swingSpeed, cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly,
        cg_thirdPerson, cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale,
        cg_timescaleFadeEnd, cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength,
        cg_tracerWidth, cg_trueLightning, cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed,
        pmove_msec, CG_Argv, CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer,
        CG_LastAttacker, CG_StartMusic, CG_UpdateCvars,
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
        CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f,
        CG_OutOfAmmoChange, CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire,
        CG_Weapon_f,
    };
    use q_shared_h::{
        byte, clipHandle_t, entityState_t, fileHandle_t, fsMode_t, gameState_t, markFragment_t,
        orientation_t, playerState_t, qboolean, qhandle_t, sfxHandle_t, trace_t, trajectory_t,
        usercmd_t, vec3_t, vec_t, vmCvar_t,
    };
    use tr_types_h::{glconfig_t, polyVert_t, poly_t, refEntity_t, refdef_t, stereoFrame_t};
    extern "C" {
        //==============================================================================

        //extern	vmCvar_t		cg_pmove_fixed;

        // ServerCommand and ConsoleCommand parameter access

        // all media should be registered during level startup to prevent
        // hitches during gameplay

        // respatialize recalculates the volumes of sound as they should be heard by the
        // given entityNum and position
        #[no_mangle]
        pub fn trap_S_Respatialize(
            entityNum: libc::c_int,
            origin: *const vec_t,
            axis: *mut vec3_t,
            inwater: libc::c_int,
        );
        // normal sounds will have their volume dynamically changed as their entity
        // moves and the listener moves

        // a local sound is always played full volume

        // used for the weapon select and zoom
        #[no_mangle]
        pub fn trap_SetUserCmdValue(stateValue: libc::c_int, sensitivityScale: libc::c_float);

    //
    // cg_snapshot.c
    //

    // a scene is built up by calls to R_ClearScene and the various R_Add functions.
    // Nothing is drawn until R_RenderScene is called.

    }
    pub type unnamed_0 = libc::c_uint;
    extern "C" {
        //==============================================================================

        //
        // cg_predict.c
        //

        //
        // cg_events.c
        //

        //
        // cg_servercmds.c
        //

        // a snapshot get can fail if the snapshot (or the entties it holds) is so
        // old that it has fallen out of the client system queue
        #[no_mangle]
        pub fn trap_GetSnapshot(snapshotNumber: libc::c_int, snapshot: *mut snapshot_t)
            -> qboolean;
        //
        // cg_playerstate.c
        //

        // cgame will poll each frame to see if a newer snapshot has arrived
        // that it is interested in.  The time is returned separately so that
        // snapshot latency can be calculated.
        #[no_mangle]
        pub fn trap_GetCurrentSnapshotNumber(
            snapshotNumber: *mut libc::c_int,
            serverTime: *mut libc::c_int,
        );
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_S_StartBackgroundTrack(intro: *const libc::c_char, loop_0: *const libc::c_char);

        //
        // cg_marks.c
        //

        //
        // cg_localents.c
        //

        //
        // cg_info.c
        //

        //
        // cg_consolecmds.c
        //

        // console variable interaction
        #[no_mangle]
        pub fn trap_Cvar_Register(
            vmCvar: *mut vmCvar_t,
            varName: *const libc::c_char,
            defaultValue: *const libc::c_char,
            flags: libc::c_int,
        );

        // model collision
        #[no_mangle]
        pub fn trap_CM_LoadMap(mapname: *const libc::c_char);
        #[no_mangle]
        pub fn trap_CM_NumInlineModels() -> libc::c_int;

        #[no_mangle]
        pub fn trap_R_LoadWorldMap(mapname: *const libc::c_char);
        // all media should be registered during level startup to prevent
        // hitches during gameplay

        // a scene is built up by calls to R_ClearScene and the various R_Add functions.
        // Nothing is drawn until R_RenderScene is called.

        // The glconfig_t will not change during the life of a cgame.
        // If it needs to change, the entire cgame will be restarted, because
        // all the qhandle_t are then invalid.
        #[no_mangle]
        pub fn trap_GetGlconfig(glconfig: *mut glconfig_t);
    // the gamestate should be grabbed at startup, and whenever a
    // configstring changes

    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct markPoly_s {
        pub prevMark: *mut markPoly_s,
        pub nextMark: *mut markPoly_s,
        pub time: libc::c_int,
        pub markShader: qhandle_t,
        pub alphaFade: qboolean,
        pub color: [libc::c_float; 4],
        pub poly: poly_t,
        pub verts: [polyVert_t; 10],
    }
    pub type unnamed = libc::c_uint;
    pub type impactSound_t = libc::c_uint;
    extern "C" {
        //==============================================================================

        //
        // cg_main.c
        //

        //
        // cg_drawtools.c
        //

        #[no_mangle]
        pub fn trap_R_RenderScene(fd: *const refdef_t);

        // a scene is built up by calls to R_ClearScene and the various R_Add functions.
        // Nothing is drawn until R_RenderScene is called.
        #[no_mangle]
        pub fn trap_R_ClearScene();

        //
        // cg_scoreboard.c
        //

        // a local sound is always played full volume

        // milliseconds should only be used for performance tuning, never
        // for anything game related.  Get time from the CG_DrawActiveFrame parameter
        #[no_mangle]
        pub fn trap_Milliseconds() -> libc::c_int;

    // returns the most recent command number that can be passed to GetUserCmd
    // this will always be at least one higher than the number in the current
    // snapshot, and it may be quite a few higher if it is a fast computer on
    // a lagged connection

    }
    extern "C" {

        //extern	vmCvar_t		cg_pmove_fixed;

        //
        // cg_view.c
        //

        //
        // cg_weapons.c
        //

        // send a string to the server over the network
        #[no_mangle]
        pub fn trap_SendClientCommand(s: *const libc::c_char);
        #[no_mangle]
        pub fn trap_Args(buffer: *mut libc::c_char, bufferLength: libc::c_int);
        #[no_mangle]
        pub fn trap_Argv(n: libc::c_int, buffer: *mut libc::c_char, bufferLength: libc::c_int);
        // register a command name so the console can perform command completion.
        // FIXME: replace this with a normal console command "defineCommand"?
        #[no_mangle]
        pub fn trap_AddCommand(cmdName: *const libc::c_char);
    }
    pub const IMPACTSOUND_FLESH: impactSound_t = 2;
    pub const LEF_TUMBLE: unnamed_0 = 2;
    extern "C" {
        //==============================================================================

        //
        // cg_main.c
        //

        //
        // cg_draw.c, cg_newDraw.c
        //

        //
        // cg_marks.c
        //

        //
        // cg_localents.c
        //

        #[no_mangle]
        pub fn trap_R_RemapShader(
            oldShader: *const libc::c_char,
            newShader: *const libc::c_char,
            timeOffset: *const libc::c_char,
        );
        // ServerCommand and ConsoleCommand parameter access
        #[no_mangle]
        pub fn trap_Argc() -> libc::c_int;

        // a local sound is always played full volume
        #[no_mangle]
        pub fn trap_S_StartLocalSound(sfx: sfxHandle_t, channelNum: libc::c_int);
        #[no_mangle]
        pub fn trap_S_ClearLoopingSounds(killall: qboolean);

        // all media should be registered during level startup to prevent
        // hitches during gameplay

        // the gamestate should be grabbed at startup, and whenever a
        // configstring changes
        #[no_mangle]
        pub fn trap_GetGameState(gamestate: *mut gameState_t);
        // retrieve a text command from the server stream
        // the current snapshot will hold the number of the most recent command
        // qfalse can be returned if the client system handled the command
        // argc() / argv() can be used to examine the parameters of the command
        #[no_mangle]
        pub fn trap_GetServerCommand(serverCommandNumber: libc::c_int) -> qboolean;
    }
    pub const LEBS_BLOOD: leBounceSoundType_t = 1;
    extern "C" {
        //==============================================================================

        //
        // cg_main.c
        //

        // force a screen update, only used during gamestate load
        #[no_mangle]
        pub fn trap_UpdateScreen();

    }
    extern "C" {
        //==============================================================================

        // polys are intended for simple wall marks, not really for doing
        // significant construction

        // Returns the projection of a polygon onto the solid brushes in the world
        #[no_mangle]
        pub fn trap_CM_MarkFragments(
            numPoints: libc::c_int,
            points: *const vec3_t,
            projection: *const vec_t,
            maxPoints: libc::c_int,
            pointBuffer: *mut vec_t,
            maxFragments: libc::c_int,
            fragmentBuffer: *mut markFragment_t,
        ) -> libc::c_int;
    }
    extern "C" {
        //==============================================================================

        //
        // cg_main.c
        //

        #[no_mangle]
        pub fn trap_S_StopLoopingSound(entnum: libc::c_int);

    // normal sounds will have their volume dynamically changed as their entity
    // moves and the listener moves

    // should this be in pmove?

    //
    // cg_effects.c
    //

    //
    // cg_ents.c
    //

    }
    pub const LEF_SOUND2: unnamed_8 = 8;
    pub type unnamed_8 = libc::c_uint;
    pub const IMPACTSOUND_DEFAULT: impactSound_t = 0;
    pub const LE_SCALE_FADE: leType_t = 7;
    pub const LE_EXPLOSION: leType_t = 1;
    extern "C" {
        //==============================================================================

        #[no_mangle]
        pub fn trap_R_DrawStretchPic(
            x: libc::c_float,
            y: libc::c_float,
            w: libc::c_float,
            h: libc::c_float,
            s1: libc::c_float,
            t1: libc::c_float,
            s2: libc::c_float,
            t2: libc::c_float,
            hShader: qhandle_t,
        );
    }
    extern "C" {
        //==============================================================================

        // all media should be registered during level startup to prevent
        // hitches during gameplay

        #[no_mangle]
        pub fn trap_R_RegisterShader(name: *const libc::c_char) -> qhandle_t;

        //
        // cg_effects.c
        //

        #[no_mangle]
        pub fn trap_R_ModelBounds(model: clipHandle_t, mins: *mut vec_t, maxs: *mut vec_t);
        // normal sounds will have their volume dynamically changed as their entity
        // moves and the listener moves

        // polys are intended for simple wall marks, not really for doing
        // significant construction

        #[no_mangle]
        pub fn trap_R_SetColor(rgba: *const libc::c_float);
    }
    extern "C" {
        //==============================================================================

        //
        // cg_player.c
        //

        #[no_mangle]
        pub fn trap_S_UpdateEntityPosition(entityNum: libc::c_int, origin: *const vec_t);

        // normal sounds will have their volume dynamically changed as their entity
        // moves and the listener moves

        #[no_mangle]
        pub fn trap_S_AddRealLoopingSound(
            entityNum: libc::c_int,
            origin: *const vec_t,
            velocity: *const vec_t,
            sfx: sfxHandle_t,
        );
    }
    pub const LEF_SOUND1: unnamed = 4;
    pub const FOOTSTEP_NORMAL: footstep_t = 0;
    pub type leMarkType_t = libc::c_uint;
    extern "C" {
        //==============================================================================

        #[no_mangle]
        pub fn trap_CM_TransformedPointContents(
            p: *const vec_t,
            model: clipHandle_t,
            origin: *const vec_t,
            angles: *const vec_t,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_CM_InlineModel(index: libc::c_int) -> clipHandle_t;
        #[no_mangle]
        pub fn trap_CM_PointContents(p: *const vec_t, model: clipHandle_t) -> libc::c_int;
        #[no_mangle]
        pub fn trap_CM_TransformedBoxTrace(
            results: *mut trace_t,
            start: *const vec_t,
            end: *const vec_t,
            mins: *const vec_t,
            maxs: *const vec_t,
            model: clipHandle_t,
            brushmask: libc::c_int,
            origin: *const vec_t,
            angles: *const vec_t,
        );
        #[no_mangle]
        pub fn trap_CM_TempBoxModel(mins: *const vec_t, maxs: *const vec_t) -> clipHandle_t;

        #[no_mangle]
        pub fn trap_GetUserCmd(cmdNumber: libc::c_int, ucmd: *mut usercmd_t) -> qboolean;
        #[no_mangle]
        pub fn trap_Cvar_Update(vmCvar: *mut vmCvar_t);
        #[no_mangle]
        pub fn trap_Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
        // returns the most recent command number that can be passed to GetUserCmd
        // this will always be at least one higher than the number in the current
        // snapshot, and it may be quite a few higher if it is a fast computer on
        // a lagged connection
        #[no_mangle]
        pub fn trap_GetCurrentCmdNumber() -> libc::c_int;
    }
    pub const LEBS_BRASS: leBounceSoundType_t = 2;
    pub type unnamed_6 = libc::c_uint;
    pub const LEMT_BLOOD: leMarkType_t = 2;
    pub const LE_SPRITE_EXPLOSION: leType_t = 2;
    pub const IMPACTSOUND_METAL: impactSound_t = 1;
    pub const LE_SCOREPLUM: leType_t = 8;
    pub const LEBS_NONE: leBounceSoundType_t = 0;
    pub const LE_MOVE_SCALE_FADE: leType_t = 4;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cgMedia_t {
        pub charsetShader: qhandle_t,
        pub charsetProp: qhandle_t,
        pub charsetPropGlow: qhandle_t,
        pub charsetPropB: qhandle_t,
        pub whiteShader: qhandle_t,
        pub redFlagModel: qhandle_t,
        pub blueFlagModel: qhandle_t,
        pub neutralFlagModel: qhandle_t,
        pub redFlagShader: [qhandle_t; 3],
        pub blueFlagShader: [qhandle_t; 3],
        pub flagShader: [qhandle_t; 4],
        pub flagPoleModel: qhandle_t,
        pub flagFlapModel: qhandle_t,
        pub redFlagFlapSkin: qhandle_t,
        pub blueFlagFlapSkin: qhandle_t,
        pub neutralFlagFlapSkin: qhandle_t,
        pub redFlagBaseModel: qhandle_t,
        pub blueFlagBaseModel: qhandle_t,
        pub neutralFlagBaseModel: qhandle_t,
        pub armorModel: qhandle_t,
        pub armorIcon: qhandle_t,
        pub teamStatusBar: qhandle_t,
        pub deferShader: qhandle_t,
        pub gibAbdomen: qhandle_t,
        pub gibArm: qhandle_t,
        pub gibChest: qhandle_t,
        pub gibFist: qhandle_t,
        pub gibFoot: qhandle_t,
        pub gibForearm: qhandle_t,
        pub gibIntestine: qhandle_t,
        pub gibLeg: qhandle_t,
        pub gibSkull: qhandle_t,
        pub gibBrain: qhandle_t,
        pub smoke2: qhandle_t,
        pub machinegunBrassModel: qhandle_t,
        pub shotgunBrassModel: qhandle_t,
        pub railRingsShader: qhandle_t,
        pub railCoreShader: qhandle_t,
        pub lightningShader: qhandle_t,
        pub friendShader: qhandle_t,
        pub balloonShader: qhandle_t,
        pub connectionShader: qhandle_t,
        pub selectShader: qhandle_t,
        pub viewBloodShader: qhandle_t,
        pub tracerShader: qhandle_t,
        pub crosshairShader: [qhandle_t; 10],
        pub lagometerShader: qhandle_t,
        pub backTileShader: qhandle_t,
        pub noammoShader: qhandle_t,
        pub smokePuffShader: qhandle_t,
        pub smokePuffRageProShader: qhandle_t,
        pub shotgunSmokePuffShader: qhandle_t,
        pub plasmaBallShader: qhandle_t,
        pub waterBubbleShader: qhandle_t,
        pub bloodTrailShader: qhandle_t,
        pub numberShaders: [qhandle_t; 11],
        pub shadowMarkShader: qhandle_t,
        pub botSkillShaders: [qhandle_t; 5],
        pub wakeMarkShader: qhandle_t,
        pub bloodMarkShader: qhandle_t,
        pub bulletMarkShader: qhandle_t,
        pub burnMarkShader: qhandle_t,
        pub holeMarkShader: qhandle_t,
        pub energyMarkShader: qhandle_t,
        pub quadShader: qhandle_t,
        pub redQuadShader: qhandle_t,
        pub quadWeaponShader: qhandle_t,
        pub invisShader: qhandle_t,
        pub regenShader: qhandle_t,
        pub battleSuitShader: qhandle_t,
        pub battleWeaponShader: qhandle_t,
        pub hastePuffShader: qhandle_t,
        pub bulletFlashModel: qhandle_t,
        pub ringFlashModel: qhandle_t,
        pub dishFlashModel: qhandle_t,
        pub lightningExplosionModel: qhandle_t,
        pub railExplosionShader: qhandle_t,
        pub plasmaExplosionShader: qhandle_t,
        pub bulletExplosionShader: qhandle_t,
        pub rocketExplosionShader: qhandle_t,
        pub grenadeExplosionShader: qhandle_t,
        pub bfgExplosionShader: qhandle_t,
        pub bloodExplosionShader: qhandle_t,
        pub teleportEffectModel: qhandle_t,
        pub teleportEffectShader: qhandle_t,
        pub scoreboardName: qhandle_t,
        pub scoreboardPing: qhandle_t,
        pub scoreboardScore: qhandle_t,
        pub scoreboardTime: qhandle_t,
        pub medalImpressive: qhandle_t,
        pub medalExcellent: qhandle_t,
        pub medalGauntlet: qhandle_t,
        pub medalDefend: qhandle_t,
        pub medalAssist: qhandle_t,
        pub medalCapture: qhandle_t,
        pub quadSound: sfxHandle_t,
        pub tracerSound: sfxHandle_t,
        pub selectSound: sfxHandle_t,
        pub useNothingSound: sfxHandle_t,
        pub wearOffSound: sfxHandle_t,
        pub footsteps: [[sfxHandle_t; 4]; 7],
        pub sfx_lghit1: sfxHandle_t,
        pub sfx_lghit2: sfxHandle_t,
        pub sfx_lghit3: sfxHandle_t,
        pub sfx_ric1: sfxHandle_t,
        pub sfx_ric2: sfxHandle_t,
        pub sfx_ric3: sfxHandle_t,
        pub sfx_rockexp: sfxHandle_t,
        pub sfx_plasmaexp: sfxHandle_t,
        pub gibSound: sfxHandle_t,
        pub gibBounce1Sound: sfxHandle_t,
        pub gibBounce2Sound: sfxHandle_t,
        pub gibBounce3Sound: sfxHandle_t,
        pub teleInSound: sfxHandle_t,
        pub teleOutSound: sfxHandle_t,
        pub noAmmoSound: sfxHandle_t,
        pub respawnSound: sfxHandle_t,
        pub talkSound: sfxHandle_t,
        pub landSound: sfxHandle_t,
        pub fallSound: sfxHandle_t,
        pub jumpPadSound: sfxHandle_t,
        pub oneMinuteSound: sfxHandle_t,
        pub fiveMinuteSound: sfxHandle_t,
        pub suddenDeathSound: sfxHandle_t,
        pub threeFragSound: sfxHandle_t,
        pub twoFragSound: sfxHandle_t,
        pub oneFragSound: sfxHandle_t,
        pub hitSound: sfxHandle_t,
        pub hitSoundHighArmor: sfxHandle_t,
        pub hitSoundLowArmor: sfxHandle_t,
        pub hitTeamSound: sfxHandle_t,
        pub impressiveSound: sfxHandle_t,
        pub excellentSound: sfxHandle_t,
        pub deniedSound: sfxHandle_t,
        pub humiliationSound: sfxHandle_t,
        pub assistSound: sfxHandle_t,
        pub defendSound: sfxHandle_t,
        pub firstImpressiveSound: sfxHandle_t,
        pub firstExcellentSound: sfxHandle_t,
        pub firstHumiliationSound: sfxHandle_t,
        pub takenLeadSound: sfxHandle_t,
        pub tiedLeadSound: sfxHandle_t,
        pub lostLeadSound: sfxHandle_t,
        pub voteNow: sfxHandle_t,
        pub votePassed: sfxHandle_t,
        pub voteFailed: sfxHandle_t,
        pub watrInSound: sfxHandle_t,
        pub watrOutSound: sfxHandle_t,
        pub watrUnSound: sfxHandle_t,
        pub flightSound: sfxHandle_t,
        pub medkitSound: sfxHandle_t,
        pub captureAwardSound: sfxHandle_t,
        pub redScoredSound: sfxHandle_t,
        pub blueScoredSound: sfxHandle_t,
        pub redLeadsSound: sfxHandle_t,
        pub blueLeadsSound: sfxHandle_t,
        pub teamsTiedSound: sfxHandle_t,
        pub captureYourTeamSound: sfxHandle_t,
        pub captureOpponentSound: sfxHandle_t,
        pub returnYourTeamSound: sfxHandle_t,
        pub returnOpponentSound: sfxHandle_t,
        pub takenYourTeamSound: sfxHandle_t,
        pub takenOpponentSound: sfxHandle_t,
        pub redFlagReturnedSound: sfxHandle_t,
        pub blueFlagReturnedSound: sfxHandle_t,
        pub enemyTookYourFlagSound: sfxHandle_t,
        pub yourTeamTookEnemyFlagSound: sfxHandle_t,
        pub youHaveFlagSound: sfxHandle_t,
        pub holyShitSound: sfxHandle_t,
        pub count3Sound: sfxHandle_t,
        pub count2Sound: sfxHandle_t,
        pub count1Sound: sfxHandle_t,
        pub countFightSound: sfxHandle_t,
        pub countPrepareSound: sfxHandle_t,
        pub regenSound: sfxHandle_t,
        pub protectSound: sfxHandle_t,
        pub n_healthSound: sfxHandle_t,
        pub hgrenb1aSound: sfxHandle_t,
        pub hgrenb2aSound: sfxHandle_t,
        pub wstbimplSound: sfxHandle_t,
        pub wstbimpmSound: sfxHandle_t,
        pub wstbimpdSound: sfxHandle_t,
        pub wstbactvSound: sfxHandle_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct centity_s {
        pub currentState: entityState_t,
        pub nextState: entityState_t,
        pub interpolate: qboolean,
        pub currentValid: qboolean,
        pub muzzleFlashTime: libc::c_int,
        pub previousEvent: libc::c_int,
        pub teleportFlag: libc::c_int,
        pub trailTime: libc::c_int,
        pub dustTrailTime: libc::c_int,
        pub miscTime: libc::c_int,
        pub snapShotTime: libc::c_int,
        pub pe: playerEntity_t,
        pub errorTime: libc::c_int,
        pub errorOrigin: vec3_t,
        pub errorAngles: vec3_t,
        pub extrapolated: qboolean,
        pub rawOrigin: vec3_t,
        pub rawAngles: vec3_t,
        pub beamEnd: vec3_t,
        pub lerpOrigin: vec3_t,
        pub lerpAngles: vec3_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct localEntity_s {
        pub prev: *mut localEntity_s,
        pub next: *mut localEntity_s,
        pub leType: leType_t,
        pub leFlags: libc::c_int,
        pub startTime: libc::c_int,
        pub endTime: libc::c_int,
        pub fadeInTime: libc::c_int,
        pub lifeRate: libc::c_float,
        pub pos: trajectory_t,
        pub angles: trajectory_t,
        pub bounceFactor: libc::c_float,
        pub color: [libc::c_float; 4],
        pub radius: libc::c_float,
        pub light: libc::c_float,
        pub lightColor: vec3_t,
        pub leMarkType: leMarkType_t,
        pub leBounceSoundType: leBounceSoundType_t,
        pub refEntity: refEntity_t,
    }
    pub const LEF_PUFF_DONT_SCALE: unnamed_8 = 1;
    pub type weaponInfo_t = weaponInfo_s;
    pub const LE_FALL_SCALE_FADE: leType_t = 5;
    pub type markPoly_t = markPoly_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct itemInfo_t {
        pub registered: qboolean,
        pub models: [qhandle_t; 4],
        pub icon: qhandle_t,
    }
    pub type leBounceSoundType_t = libc::c_uint;
    pub const LEMT_NONE: leMarkType_t = 0;
    pub type footstep_t = libc::c_uint;
    extern "C" {
        //==============================================================================

        // normal sounds will have their volume dynamically changed as their entity
        // moves and the listener moves
        #[no_mangle]
        pub fn trap_S_StartSound(
            origin: *mut vec_t,
            entityNum: libc::c_int,
            entchannel: libc::c_int,
            sfx: sfxHandle_t,
        );
    //
    // cg_effects.c
    //

    }
    extern "C" {
        //==============================================================================

        //
        // cg_main.c
        //

        //
        // cg_effects.c
        //

        #[no_mangle]
        pub fn trap_R_AddLightToScene(
            org: *const vec_t,
            intensity: libc::c_float,
            r: libc::c_float,
            g: libc::c_float,
            b: libc::c_float,
        );
        #[no_mangle]
        pub fn trap_R_AddRefEntityToScene(re: *const refEntity_t);

        #[no_mangle]
        pub fn trap_S_AddLoopingSound(
            entityNum: libc::c_int,
            origin: *const vec_t,
            velocity: *const vec_t,
            sfx: sfxHandle_t,
        );

        // polys are intended for simple wall marks, not really for doing
        // significant construction
        #[no_mangle]
        pub fn trap_R_AddPolyToScene(
            hShader: qhandle_t,
            numVerts: libc::c_int,
            verts: *const polyVert_t,
        );
        #[no_mangle]
        pub fn trap_CM_BoxTrace(
            results: *mut trace_t,
            start: *const vec_t,
            end: *const vec_t,
            mins: *const vec_t,
            maxs: *const vec_t,
            model: clipHandle_t,
            brushmask: libc::c_int,
        );

        #[no_mangle]
        pub fn trap_S_RegisterSound(
            sample: *const libc::c_char,
            compressed: qboolean,
        ) -> sfxHandle_t;
        #[no_mangle]
        pub fn trap_R_LerpTag(
            tag: *mut orientation_t,
            mod_0: clipHandle_t,
            startFrame: libc::c_int,
            endFrame: libc::c_int,
            frac: libc::c_float,
            tagName: *const libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_R_RegisterShaderNoMip(name: *const libc::c_char) -> qhandle_t;
        // filesystem access
        // returns length of file
        #[no_mangle]
        pub fn trap_FS_FOpenFile(
            qpath: *const libc::c_char,
            f: *mut fileHandle_t,
            mode: fsMode_t,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn trap_FS_Read(buffer: *mut libc::c_void, len: libc::c_int, f: fileHandle_t);
        #[no_mangle]
        pub fn trap_R_RegisterSkin(name: *const libc::c_char) -> qhandle_t;
        // all media should be registered during level startup to prevent
        // hitches during gameplay
        #[no_mangle]
        pub fn trap_R_RegisterModel(name: *const libc::c_char) -> qhandle_t;
        #[no_mangle]
        pub fn trap_MemoryRemaining() -> libc::c_int;
        #[no_mangle]
        pub fn trap_Cvar_VariableStringBuffer(
            var_name: *const libc::c_char,
            buffer: *mut libc::c_char,
            bufsize: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_R_LightForPoint(
            point: *mut vec_t,
            ambientLight: *mut vec_t,
            directedLight: *mut vec_t,
            lightDir: *mut vec_t,
        ) -> libc::c_int;
    }
    pub type localEntity_t = localEntity_s;
    pub const FOOTSTEP_TOTAL: footstep_t = 7;
    pub const LEMT_BURN: leMarkType_t = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cgs_t {
        pub gameState: gameState_t,
        pub glconfig: glconfig_t,
        pub screenXScale: libc::c_float,
        pub screenYScale: libc::c_float,
        pub screenXBias: libc::c_float,
        pub serverCommandSequence: libc::c_int,
        pub processedSnapshotNum: libc::c_int,
        pub localServer: qboolean,
        pub gametype: gametype_t,
        pub dmflags: libc::c_int,
        pub teamflags: libc::c_int,
        pub fraglimit: libc::c_int,
        pub capturelimit: libc::c_int,
        pub timelimit: libc::c_int,
        pub maxclients: libc::c_int,
        pub mapname: [libc::c_char; 64],
        pub redTeam: [libc::c_char; 64],
        pub blueTeam: [libc::c_char; 64],
        pub voteTime: libc::c_int,
        pub voteYes: libc::c_int,
        pub voteNo: libc::c_int,
        pub voteModified: qboolean,
        pub voteString: [libc::c_char; 1024],
        pub teamVoteTime: [libc::c_int; 2],
        pub teamVoteYes: [libc::c_int; 2],
        pub teamVoteNo: [libc::c_int; 2],
        pub teamVoteModified: [qboolean; 2],
        pub teamVoteString: [[libc::c_char; 1024]; 2],
        pub levelStartTime: libc::c_int,
        pub scores1: libc::c_int,
        pub scores2: libc::c_int,
        pub redflag: libc::c_int,
        pub blueflag: libc::c_int,
        pub flagStatus: libc::c_int,
        pub newHud: qboolean,
        pub gameModels: [qhandle_t; 256],
        pub gameSounds: [sfxHandle_t; 256],
        pub numInlineModels: libc::c_int,
        pub inlineDrawModel: [qhandle_t; 256],
        pub inlineModelMidpoints: [vec3_t; 256],
        pub clientinfo: [clientInfo_t; 64],
        pub teamChatMsgs: [[libc::c_char; 241]; 8],
        pub teamChatMsgTimes: [libc::c_int; 8],
        pub teamChatPos: libc::c_int,
        pub teamLastChatPos: libc::c_int,
        pub cursorX: libc::c_int,
        pub cursorY: libc::c_int,
        pub eventHandling: qboolean,
        pub mouseCaptured: qboolean,
        pub sizingHud: qboolean,
        pub capturedItem: *mut libc::c_void,
        pub activeCursor: qhandle_t,
        pub currentOrder: libc::c_int,
        pub orderPending: qboolean,
        pub orderTime: libc::c_int,
        pub currentVoiceClient: libc::c_int,
        pub acceptOrderTime: libc::c_int,
        pub acceptTask: libc::c_int,
        pub acceptLeader: libc::c_int,
        pub acceptVoice: [libc::c_char; 32],
        pub media: cgMedia_t,
    }
    pub const LE_MARK: leType_t = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct playerEntity_t {
        pub legs: lerpFrame_t,
        pub torso: lerpFrame_t,
        pub flag: lerpFrame_t,
        pub painTime: libc::c_int,
        pub painDirection: libc::c_int,
        pub lightningFiring: libc::c_int,
        pub railFireTime: libc::c_int,
        pub barrelAngle: libc::c_float,
        pub barrelTime: libc::c_int,
        pub barrelSpinning: qboolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct weaponInfo_s {
        pub registered: qboolean,
        pub item: *mut gitem_t,
        pub handsModel: qhandle_t,
        pub weaponModel: qhandle_t,
        pub barrelModel: qhandle_t,
        pub flashModel: qhandle_t,
        pub weaponMidpoint: vec3_t,
        pub flashDlight: libc::c_float,
        pub flashDlightColor: vec3_t,
        pub flashSound: [sfxHandle_t; 4],
        pub weaponIcon: qhandle_t,
        pub ammoIcon: qhandle_t,
        pub ammoModel: qhandle_t,
        pub missileModel: qhandle_t,
        pub missileSound: sfxHandle_t,
        pub missileTrailFunc:
            Option<unsafe extern "C" fn(_: *mut centity_t, _: *const weaponInfo_s) -> ()>,
        pub missileDlight: libc::c_float,
        pub missileDlightColor: vec3_t,
        pub missileRenderfx: libc::c_int,
        pub ejectBrassFunc: Option<unsafe extern "C" fn(_: *mut centity_t) -> ()>,
        pub trailRadius: libc::c_float,
        pub wiTrailTime: libc::c_float,
        pub readySound: sfxHandle_t,
        pub firingSound: sfxHandle_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cg_t {
        pub clientFrame: libc::c_int,
        pub clientNum: libc::c_int,
        pub demoPlayback: qboolean,
        pub levelShot: qboolean,
        pub deferredPlayerLoading: libc::c_int,
        pub loading: qboolean,
        pub intermissionStarted: qboolean,
        pub latestSnapshotNum: libc::c_int,
        pub latestSnapshotTime: libc::c_int,
        pub snap: *mut snapshot_t,
        pub nextSnap: *mut snapshot_t,
        pub activeSnapshots: [snapshot_t; 2],
        pub frameInterpolation: libc::c_float,
        pub thisFrameTeleport: qboolean,
        pub nextFrameTeleport: qboolean,
        pub frametime: libc::c_int,
        pub time: libc::c_int,
        pub oldTime: libc::c_int,
        pub physicsTime: libc::c_int,
        pub timelimitWarnings: libc::c_int,
        pub fraglimitWarnings: libc::c_int,
        pub mapRestart: qboolean,
        pub renderingThirdPerson: qboolean,
        pub hyperspace: qboolean,
        pub predictedPlayerState: playerState_t,
        pub predictedPlayerEntity: centity_t,
        pub validPPS: qboolean,
        pub predictedErrorTime: libc::c_int,
        pub predictedError: vec3_t,
        pub eventSequence: libc::c_int,
        pub predictableEvents: [libc::c_int; 16],
        pub stepChange: libc::c_float,
        pub stepTime: libc::c_int,
        pub duckChange: libc::c_float,
        pub duckTime: libc::c_int,
        pub landChange: libc::c_float,
        pub landTime: libc::c_int,
        pub weaponSelect: libc::c_int,
        pub autoAngles: vec3_t,
        pub autoAxis: [vec3_t; 3],
        pub autoAnglesFast: vec3_t,
        pub autoAxisFast: [vec3_t; 3],
        pub refdef: refdef_t,
        pub refdefViewAngles: vec3_t,
        pub zoomed: qboolean,
        pub zoomTime: libc::c_int,
        pub zoomSensitivity: libc::c_float,
        pub infoScreenText: [libc::c_char; 1024],
        pub scoresRequestTime: libc::c_int,
        pub numScores: libc::c_int,
        pub selectedScore: libc::c_int,
        pub teamScores: [libc::c_int; 2],
        pub scores: [score_t; 64],
        pub showScores: qboolean,
        pub scoreBoardShowing: qboolean,
        pub scoreFadeTime: libc::c_int,
        pub killerName: [libc::c_char; 32],
        pub spectatorList: [libc::c_char; 1024],
        pub spectatorLen: libc::c_int,
        pub spectatorWidth: libc::c_float,
        pub spectatorTime: libc::c_int,
        pub spectatorPaintX: libc::c_int,
        pub spectatorPaintX2: libc::c_int,
        pub spectatorOffset: libc::c_int,
        pub spectatorPaintLen: libc::c_int,
        pub centerPrintTime: libc::c_int,
        pub centerPrintCharWidth: libc::c_int,
        pub centerPrintY: libc::c_int,
        pub centerPrint: [libc::c_char; 1024],
        pub centerPrintLines: libc::c_int,
        pub lowAmmoWarning: libc::c_int,
        pub crosshairClientNum: libc::c_int,
        pub crosshairClientTime: libc::c_int,
        pub powerupActive: libc::c_int,
        pub powerupTime: libc::c_int,
        pub attackerTime: libc::c_int,
        pub voiceTime: libc::c_int,
        pub rewardStack: libc::c_int,
        pub rewardTime: libc::c_int,
        pub rewardCount: [libc::c_int; 10],
        pub rewardShader: [qhandle_t; 10],
        pub rewardSound: [qhandle_t; 10],
        pub soundBufferIn: libc::c_int,
        pub soundBufferOut: libc::c_int,
        pub soundTime: libc::c_int,
        pub soundBuffer: [qhandle_t; 20],
        pub warmup: libc::c_int,
        pub warmupCount: libc::c_int,
        pub itemPickup: libc::c_int,
        pub itemPickupTime: libc::c_int,
        pub itemPickupBlendTime: libc::c_int,
        pub weaponSelectTime: libc::c_int,
        pub weaponAnimation: libc::c_int,
        pub weaponAnimationTime: libc::c_int,
        pub damageTime: libc::c_float,
        pub damageX: libc::c_float,
        pub damageY: libc::c_float,
        pub damageValue: libc::c_float,
        pub headYaw: libc::c_float,
        pub headEndPitch: libc::c_float,
        pub headEndYaw: libc::c_float,
        pub headEndTime: libc::c_int,
        pub headStartPitch: libc::c_float,
        pub headStartYaw: libc::c_float,
        pub headStartTime: libc::c_int,
        pub v_dmg_time: libc::c_float,
        pub v_dmg_pitch: libc::c_float,
        pub v_dmg_roll: libc::c_float,
        pub bobfracsin: libc::c_float,
        pub bobcycle: libc::c_int,
        pub xyspeed: libc::c_float,
        pub nextOrbitTime: libc::c_int,
        pub testModelEntity: refEntity_t,
        pub testModelName: [libc::c_char; 64],
        pub testGun: qboolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct score_t {
        pub client: libc::c_int,
        pub score: libc::c_int,
        pub ping: libc::c_int,
        pub time: libc::c_int,
        pub scoreFlags: libc::c_int,
        pub powerUps: libc::c_int,
        pub accuracy: libc::c_int,
        pub impressiveCount: libc::c_int,
        pub excellentCount: libc::c_int,
        pub guantletCount: libc::c_int,
        pub defendCount: libc::c_int,
        pub assistCount: libc::c_int,
        pub captures: libc::c_int,
        pub perfect: qboolean,
        pub team: libc::c_int,
    }
    pub const FOOTSTEP_BOOT: footstep_t = 1;
    pub const LE_FRAGMENT: leType_t = 3;
    pub const FOOTSTEP_METAL: footstep_t = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct lerpFrame_t {
        pub oldFrame: libc::c_int,
        pub oldFrameTime: libc::c_int,
        pub frame: libc::c_int,
        pub frameTime: libc::c_int,
        pub backlerp: libc::c_float,
        pub yawAngle: libc::c_float,
        pub yawing: qboolean,
        pub pitchAngle: libc::c_float,
        pub pitching: qboolean,
        pub animationNumber: libc::c_int,
        pub animation: *mut animation_t,
        pub animationTime: libc::c_int,
    }
    pub const FOOTSTEP_ENERGY: footstep_t = 4;
    pub type leType_t = libc::c_uint;
    pub const FOOTSTEP_MECH: footstep_t = 3;
    pub const LE_FADE_RGB: leType_t = 6;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientInfo_t {
        pub infoValid: qboolean,
        pub name: [libc::c_char; 64],
        pub team: team_t,
        pub botSkill: libc::c_int,
        pub color1: vec3_t,
        pub color2: vec3_t,
        pub c1RGBA: [byte; 4],
        pub c2RGBA: [byte; 4],
        pub score: libc::c_int,
        pub location: libc::c_int,
        pub health: libc::c_int,
        pub armor: libc::c_int,
        pub curWeapon: libc::c_int,
        pub handicap: libc::c_int,
        pub wins: libc::c_int,
        pub losses: libc::c_int,
        pub teamTask: libc::c_int,
        pub teamLeader: qboolean,
        pub powerups: libc::c_int,
        pub medkitUsageTime: libc::c_int,
        pub invulnerabilityStartTime: libc::c_int,
        pub invulnerabilityStopTime: libc::c_int,
        pub breathPuffTime: libc::c_int,
        pub modelName: [libc::c_char; 64],
        pub skinName: [libc::c_char; 64],
        pub headModelName: [libc::c_char; 64],
        pub headSkinName: [libc::c_char; 64],
        pub redTeam: [libc::c_char; 32],
        pub blueTeam: [libc::c_char; 32],
        pub deferred: qboolean,
        pub newAnims: qboolean,
        pub fixedlegs: qboolean,
        pub fixedtorso: qboolean,
        pub headOffset: vec3_t,
        pub footsteps: footstep_t,
        pub gender: gender_t,
        pub legsModel: qhandle_t,
        pub legsSkin: qhandle_t,
        pub torsoModel: qhandle_t,
        pub torsoSkin: qhandle_t,
        pub headModel: qhandle_t,
        pub headSkin: qhandle_t,
        pub modelIcon: qhandle_t,
        pub animations: [animation_t; 37],
        pub sounds: [sfxHandle_t; 32],
    }
    pub const FOOTSTEP_SPLASH: footstep_t = 6;
    pub type centity_t = centity_s;
    pub const FOOTSTEP_FLESH: footstep_t = 2;
    use libc;
}
mod cg_variadic_h {
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
        cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail,
        cg_oldRocket, cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll,
        cg_scorePlum, cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats,
        cg_swingSpeed, cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly,
        cg_thirdPerson, cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale,
        cg_timescaleFadeEnd, cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength,
        cg_tracerWidth, cg_trueLightning, cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed,
        pmove_msec, CG_Argv, CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer,
        CG_LastAttacker, CG_StartMusic, CG_UpdateCvars,
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
        CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f,
        CG_OutOfAmmoChange, CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire,
        CG_Weapon_f,
    };
    extern "C" {
        #[no_mangle]
        pub fn CG_Printf(msg: *const libc::c_char, ...);

    }
    extern "C" {
        #[no_mangle]
        pub fn CG_Error(msg: *const libc::c_char, ...) -> !;
    }
    use libc;
}
mod stdlib {
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
        cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail,
        cg_oldRocket, cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll,
        cg_scorePlum, cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats,
        cg_swingSpeed, cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly,
        cg_thirdPerson, cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale,
        cg_timescaleFadeEnd, cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength,
        cg_tracerWidth, cg_trueLightning, cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed,
        pmove_msec, CG_Argv, CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer,
        CG_LastAttacker, CG_StartMusic, CG_UpdateCvars,
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
        CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f,
        CG_OutOfAmmoChange, CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire,
        CG_Weapon_f,
    };
    extern "C" {
        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    }
    extern "C" {

        #[no_mangle]
        pub fn strncpy(
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    }
    extern "C" {
        #[no_mangle]
        pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    }
    extern "C" {

        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    }
    extern "C" {
        #[no_mangle]
        pub fn acos(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
    extern "C" {

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
    extern "C" {

        #[no_mangle]
        pub fn tan(_: libc::c_double) -> libc::c_double;
    }
    extern "C" {
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    }
    extern "C" {
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
    extern "C" {
        #[no_mangle]
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;

        #[no_mangle]
        pub fn rand() -> libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub fn cos(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn sin(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
    pub type intptr_t = libc::c_long;
    use libc;
}
mod cg_public_h {
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
        cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail,
        cg_oldRocket, cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll,
        cg_scorePlum, cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats,
        cg_swingSpeed, cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly,
        cg_thirdPerson, cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale,
        cg_timescaleFadeEnd, cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength,
        cg_tracerWidth, cg_trueLightning, cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed,
        pmove_msec, CG_Argv, CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer,
        CG_LastAttacker, CG_StartMusic, CG_UpdateCvars,
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
        CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f,
        CG_OutOfAmmoChange, CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire,
        CG_Weapon_f,
    };
    use q_shared_h::{byte, entityState_t, playerState_t};
    pub const CG_INIT: unnamed_1 = 0;
    pub const CG_SHUTDOWN: unnamed_1 = 1;
    pub const CG_EVENT_HANDLING: unnamed_1 = 8;
    pub const CG_MOUSE_EVENT: unnamed_1 = 7;
    pub const CG_DRAW_ACTIVE_FRAME: unnamed_1 = 3;
    pub const CG_CONSOLE_COMMAND: unnamed_1 = 2;
    pub const CG_CROSSHAIR_PLAYER: unnamed_1 = 4;
    pub const CG_KEY_EVENT: unnamed_1 = 6;
    pub const CG_LAST_ATTACKER: unnamed_1 = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct snapshot_t {
        pub snapFlags: libc::c_int,
        pub ping: libc::c_int,
        pub serverTime: libc::c_int,
        pub areamask: [byte; 32],
        pub ps: playerState_t,
        pub numEntities: libc::c_int,
        pub entities: [entityState_t; 256],
        pub numServerCommands: libc::c_int,
        pub serverCommandSequence: libc::c_int,
    }
    pub type unnamed_1 = libc::c_uint;
    use libc;
}
mod bg_public_h {
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
        cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail,
        cg_oldRocket, cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll,
        cg_scorePlum, cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats,
        cg_swingSpeed, cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly,
        cg_thirdPerson, cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale,
        cg_timescaleFadeEnd, cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength,
        cg_tracerWidth, cg_trueLightning, cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed,
        pmove_msec, CG_Argv, CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer,
        CG_LastAttacker, CG_StartMusic, CG_UpdateCvars,
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
        CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f,
        CG_OutOfAmmoChange, CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire,
        CG_Weapon_f,
    };
    use q_shared_h::{
        entityState_t, playerState_t, qboolean, trace_t, trajectory_t, usercmd_t, vec3_t, vec_t,
    };
    pub const GTS_BLUETEAM_TOOK_LEAD: unnamed_4 = 11;
    pub type holdable_t = libc::c_uint;
    pub const MOD_SHOTGUN: unnamed_6 = 1;
    pub const HI_MEDKIT: holdable_t = 2;
    pub const MOD_GRAPPLE: unnamed_6 = 23;
    extern "C" {
        // included in both the game dll and the client

        #[no_mangle]
        pub fn BG_FindItemForPowerup(pw: powerup_t) -> *mut gitem_t;
    }
    pub const EV_DEATH2: unnamed_3 = 58;
    pub const EV_DEATH3: unnamed_3 = 59;
    pub const HI_NUM_HOLDABLE: holdable_t = 6;
    pub const MOD_PLASMA: unnamed_6 = 8;
    pub const WEAPON_DROPPING: unnamed_1 = 2;
    pub const MOD_ROCKET: unnamed_6 = 6;
    pub const EV_POWERUP_REGEN: unnamed_3 = 63;
    pub const EV_MISSILE_HIT: unnamed_3 = 50;
    pub const EV_GENERAL_SOUND: unnamed_3 = 45;
    pub const MOD_GRENADE: unnamed_6 = 4;
    pub const MOD_GAUNTLET: unnamed_6 = 2;
    pub const MOD_ROCKET_SPLASH: unnamed_6 = 7;
    pub const HI_PORTAL: holdable_t = 4;
    pub type unnamed_6 = libc::c_uint;
    pub const EV_MISSILE_MISS_METAL: unnamed_3 = 52;
    pub const LEGS_JUMPB: unnamed_1 = 20;
    pub const GTS_RED_RETURN: unnamed_4 = 2;
    pub const EV_JUMP_PAD: unnamed_3 = 13;
    pub const MOD_UNKNOWN: unnamed_6 = 0;
    pub const EV_RAILTRAIL: unnamed_3 = 53;
    pub const MOD_TRIGGER_HURT: unnamed_6 = 22;
    pub const EV_TAUNT_PATROL: unnamed_3 = 82;
    pub const EV_GIB_PLAYER: unnamed_3 = 64;
    pub const ET_GRAPPLE: unnamed = 11;
    pub const EV_POWERUP_QUAD: unnamed_3 = 61;
    pub const EV_SCOREPLUM: unnamed_3 = 65;
    pub const MOD_BFG_SPLASH: unnamed_6 = 13;
    pub const GTS_BLUEOBELISK_ATTACKED: unnamed_4 = 7;
    pub const MOD_CRUSH: unnamed_6 = 17;
    pub const WEAPON_READY: unnamed_1 = 0;
    pub const EV_BULLET_HIT_FLESH: unnamed_3 = 48;
    pub const FLAG_STAND2RUN: unnamed_4 = 36;
    pub const EV_BULLET_HIT_WALL: unnamed_3 = 49;
    pub const GTS_TEAMS_ARE_TIED: unnamed_4 = 12;
    pub type powerup_t = libc::c_uint;
    pub const EV_USE_ITEM15: unnamed_3 = 39;
    pub const EV_NOAMMO: unnamed_3 = 21;
    extern "C" {
        // if a full pmove isn't done on the client, you can just update the angles
        #[no_mangle]
        pub fn PM_UpdateViewAngles(ps: *mut playerState_t, cmd: *const usercmd_t);
        #[no_mangle]
        pub fn Pmove(pmove: *mut pmove_t);
        // included in both the game dll and the client

        #[no_mangle]
        pub fn BG_CanItemBeGrabbed(
            gametype: libc::c_int,
            ent: *const entityState_t,
            ps: *const playerState_t,
        ) -> qboolean;

        #[no_mangle]
        pub fn BG_AddPredictableEventToPlayerstate(
            newEvent: libc::c_int,
            eventParm: libc::c_int,
            ps: *mut playerState_t,
        );
        #[no_mangle]
        pub fn BG_TouchJumpPad(ps: *mut playerState_t, jumppad: *mut entityState_t);
        #[no_mangle]
        pub fn BG_PlayerTouchesItem(
            ps: *mut playerState_t,
            item: *mut entityState_t,
            atTime: libc::c_int,
        ) -> qboolean;
    }
    pub const GTS_RED_CAPTURE: unnamed_4 = 0;
    pub const WP_RAILGUN: unnamed_2 = 7;
    pub const GTS_BLUETEAM_SCORED: unnamed_4 = 9;
    pub const EV_JUMP: unnamed_3 = 14;
    pub const GTS_REDOBELISK_ATTACKED: unnamed_4 = 6;
    pub const EV_DEBUG_LINE: unnamed_3 = 74;
    pub const MOD_PLASMA_SPLASH: unnamed_6 = 9;
    pub const WEAPON_FIRING: unnamed_1 = 3;
    pub const EV_STEP_8: unnamed_3 = 7;
    pub const MOD_SUICIDE: unnamed_6 = 20;
    pub const EV_ITEM_RESPAWN: unnamed_3 = 40;
    pub const EV_PROXIMITY_MINE_TRIGGER: unnamed_3 = 67;
    pub const GTS_BLUE_TAKEN: unnamed_4 = 5;
    pub const HI_INVULNERABILITY: holdable_t = 5;
    pub const GTS_REDTEAM_TOOK_LEAD: unnamed_4 = 10;
    pub const EV_STOPLOOPINGSOUND: unnamed_3 = 75;
    pub const EV_TAUNT_NO: unnamed_3 = 78;
    pub const EV_FOOTSTEP_METAL: unnamed_3 = 2;
    pub const LEGS_SWIM: unnamed_4 = 17;
    pub const TORSO_GESTURE: unnamed_4 = 6;
    pub const PM_NORMAL: unnamed_0 = 0;
    pub const MOD_BFG: unnamed_6 = 12;
    pub const EV_ITEM_PICKUP: unnamed_3 = 19;
    pub const ET_BEAM: unnamed_1 = 5;
    pub const LEGS_BACKWALK: unnamed_1 = 33;
    pub const MOD_GRENADE_SPLASH: unnamed_6 = 5;
    pub const GTS_BLUE_CAPTURE: unnamed_4 = 1;
    pub const EV_TAUNT: unnamed_3 = 76;
    pub const EV_FALL_FAR: unnamed_3 = 12;
    pub const BOTH_DEATH3: unnamed_1 = 4;
    pub const EV_USE_ITEM13: unnamed_3 = 37;
    pub const WP_NONE: unnamed_2 = 0;
    pub const EV_FALL_SHORT: unnamed_3 = 10;
    pub const EV_FOOTSTEP: unnamed_3 = 1;
    pub const PW_BATTLESUIT: unnamed_2 = 2;
    pub const PM_INTERMISSION: unnamed = 5;
    pub const LEGS_BACKCR: unnamed_4 = 32;
    pub const PW_FLIGHT: unnamed_2 = 6;
    pub const MOD_RAILGUN: unnamed_6 = 10;
    pub const MOD_WATER: unnamed_6 = 14;
    pub const GTS_BLUE_RETURN: unnamed_4 = 3;
    pub const LEGS_IDLECR: unnamed_4 = 23;
    pub const EV_POWERUP_BATTLESUIT: unnamed_3 = 62;
    pub const EV_FOOTSPLASH: unnamed_3 = 3;
    pub const ET_PORTAL: unnamed_4 = 6;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct pmove_t {
        pub ps: *mut playerState_t,
        pub cmd: usercmd_t,
        pub tracemask: libc::c_int,
        pub debugLevel: libc::c_int,
        pub noFootsteps: qboolean,
        pub gauntletHit: qboolean,
        pub framecount: libc::c_int,
        pub numtouch: libc::c_int,
        pub touchents: [libc::c_int; 32],
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub watertype: libc::c_int,
        pub waterlevel: libc::c_int,
        pub xyspeed: libc::c_float,
        pub pmove_fixed: libc::c_int,
        pub pmove_msec: libc::c_int,
        pub trace: Option<
            unsafe extern "C" fn(
                _: *mut trace_t,
                _: *const vec_t,
                _: *const vec_t,
                _: *const vec_t,
                _: *const vec_t,
                _: libc::c_int,
                _: libc::c_int,
            ) -> (),
        >,
        pub pointcontents:
            Option<unsafe extern "C" fn(_: *const vec_t, _: libc::c_int) -> libc::c_int>,
    }
    pub const GTS_KAMIKAZE: unnamed_4 = 13;
    pub const TORSO_STAND: unnamed_4 = 11;
    pub const IT_PERSISTANT_POWERUP: itemType_t = 7;
    pub const EV_OBELISKPAIN: unnamed_3 = 70;
    pub const WP_GAUNTLET: unnamed_2 = 1;
    pub const TORSO_NEGATIVE: unnamed_5 = 30;
    pub const EV_FIRE_WEAPON: unnamed_3 = 23;
    pub const WP_SHOTGUN: unnamed_2 = 3;
    pub const EV_USE_ITEM4: unnamed_3 = 28;
    pub const ET_SPEAKER: unnamed_2 = 7;
    pub const EV_GLOBAL_ITEM_PICKUP: unnamed_3 = 20;
    pub const EV_STEP_4: unnamed_3 = 6;
    pub const MOD_TELEFRAG: unnamed_6 = 18;
    pub const LEGS_TURN: unnamed_5 = 24;
    pub const ET_MOVER: unnamed_1 = 4;
    pub const EV_ITEM_POP: unnamed_3 = 41;
    pub const BOTH_DEATH2: unnamed_5 = 2;
    pub const WP_GRENADE_LAUNCHER: unnamed_2 = 4;
    extern "C" {
        // included in both the game dll and the client

        #[no_mangle]
        pub fn BG_FindItemForHoldable(pw: holdable_t) -> *mut gitem_t;

    }
    pub const EV_OBITUARY: unnamed_3 = 60;
    pub const LEGS_IDLE: unnamed_5 = 22;
    pub const STAT_CLIENTS_READY: unnamed_2 = 5;
    pub const EV_PROXIMITY_MINE_STICK: unnamed_3 = 66;
    pub type itemType_t = libc::c_uint;
    pub const TORSO_ATTACK2: unnamed_4 = 8;
    pub const EV_TAUNT_GETFLAG: unnamed_3 = 80;
    pub const EV_BULLET: unnamed_3 = 55;
    pub const LEGS_RUN: unnamed_5 = 15;
    pub const BOTH_DEATH1: unnamed_5 = 0;
    pub const ET_PUSH_TRIGGER: unnamed_4 = 8;
    pub const TORSO_GETFLAG: unnamed_5 = 25;
    pub type unnamed = libc::c_uint;
    pub const PW_SCOUT: unnamed_2 = 10;
    pub const IT_TEAM: itemType_t = 8;
    pub const MOD_SLIME: unnamed_6 = 15;
    pub const TORSO_FOLLOWME: unnamed_5 = 28;
    pub const PERS_ATTACKER: unnamed_3 = 6;
    pub const EV_STEP_16: unnamed_3 = 9;
    pub const PERS_HITS: unnamed_2 = 1;
    pub const TORSO_AFFIRMATIVE: unnamed_4 = 29;
    pub const LEGS_LANDB: unnamed_4 = 21;
    pub const GT_TEAM: gametype_t = 3;
    pub const IT_ARMOR: itemType_t = 3;
    pub const HI_KAMIKAZE: holdable_t = 3;
    pub const WP_MACHINEGUN: unnamed_2 = 2;
    pub const TORSO_GUARDBASE: unnamed_4 = 26;
    pub const TORSO_STAND2: unnamed_5 = 12;
    pub const ET_TEAM: unnamed = 12;
    pub const WEAPON_RAISING: unnamed_1 = 1;
    pub const PERS_GAUNTLET_FRAG_COUNT: unnamed_0 = 13;
    pub const EV_SHOTGUN: unnamed_3 = 54;
    pub const LEGS_LAND: unnamed_4 = 19;
    pub const MOD_FALLING: unnamed_6 = 19;
    pub const TORSO_PATROL: unnamed_5 = 27;
    pub const EV_SWIM: unnamed_3 = 5;
    pub const LEGS_JUMP: unnamed_4 = 18;
    pub const GTS_REDTEAM_SCORED: unnamed_4 = 8;
    pub const HI_TELEPORTER: holdable_t = 1;
    pub const PW_NEUTRALFLAG: unnamed_2 = 9;
    pub const WP_ROCKET_LAUNCHER: unnamed_0 = 5;
    pub const EV_TAUNT_YES: unnamed_3 = 77;
    pub const EV_KAMIKAZE: unnamed_3 = 68;
    pub const EV_INVUL_IMPACT: unnamed_3 = 71;
    pub const EV_FALL_MEDIUM: unnamed_3 = 11;
    pub const STAT_DEAD_YAW: unnamed_1 = 4;
    pub const STAT_ARMOR: unnamed_0 = 3;
    pub const GTS_RED_TAKEN: unnamed_4 = 4;
    pub const PERS_ASSIST_COUNT: unnamed_2 = 12;
    pub const PERS_ATTACKEE_ARMOR: unnamed_1 = 7;
    pub const EV_LIGHTNINGBOLT: unnamed_3 = 73;
    pub const ET_ITEM: unnamed_5 = 2;
    pub const MOD_MACHINEGUN: unnamed_6 = 3;
    pub const EV_FOOTWADE: unnamed_3 = 4;
    pub const EV_USE_ITEM11: unnamed_3 = 35;
    pub const IT_BAD: itemType_t = 0;
    pub const ET_TELEPORT_TRIGGER: unnamed_2 = 9;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct animation_s {
        pub firstFrame: libc::c_int,
        pub numFrames: libc::c_int,
        pub loopFrames: libc::c_int,
        pub frameLerp: libc::c_int,
        pub initialLerp: libc::c_int,
        pub reversed: libc::c_int,
        pub flipflop: libc::c_int,
    }
    pub const EV_CHANGE_WEAPON: unnamed_3 = 22;
    pub const ET_MISSILE: unnamed = 3;
    pub type gender_t = libc::c_uint;
    pub const EV_NONE: unnamed_3 = 0;
    pub const EV_GLOBAL_TEAM_SOUND: unnamed_3 = 47;
    pub const WP_NUM_WEAPONS: weapon_t = 11;
    extern "C" {
        // included in both the game dll and the client
        #[no_mangle]
        pub static mut bg_itemlist: [gitem_t; 0];
        #[no_mangle]
        pub static mut bg_numItems: libc::c_int;
        #[no_mangle]
        pub fn BG_EvaluateTrajectory(
            tr: *const trajectory_t,
            atTime: libc::c_int,
            result: *mut vec_t,
        );
        #[no_mangle]
        pub fn BG_EvaluateTrajectoryDelta(
            tr: *const trajectory_t,
            atTime: libc::c_int,
            result: *mut vec_t,
        );
        #[no_mangle]
        pub fn BG_PlayerStateToEntityState(
            ps: *mut playerState_t,
            s: *mut entityState_t,
            snap: qboolean,
        );
    }
    pub const TORSO_DROP: unnamed_4 = 9;
    pub const WP_PLASMAGUN: unnamed_2 = 8;
    pub const GT_FFA: gametype_t = 0;
    pub const EV_WATER_CLEAR: unnamed_3 = 18;
    pub const EV_WATER_TOUCH: unnamed_3 = 15;
    pub const PERS_DEFEND_COUNT: unnamed_2 = 11;
    pub const TORSO_RAISE: unnamed_5 = 10;
    pub const PERS_SCORE: unnamed = 0;
    pub const GT_SINGLE_PLAYER: gametype_t = 2;
    pub const PERS_IMPRESSIVE_COUNT: unnamed_2 = 9;
    pub const MAX_ANIMATIONS: unnamed_4 = 31;
    pub type unnamed_2 = libc::c_uint;
    pub const PERS_EXCELLENT_COUNT: unnamed_2 = 10;
    pub const PERS_PLAYEREVENTS: unnamed = 5;
    pub type animation_t = animation_s;
    pub const PW_HASTE: powerup_t = 3;
    pub const IT_WEAPON: itemType_t = 1;
    pub const IT_POWERUP: itemType_t = 5;
    pub const PERS_KILLED: unnamed = 8;
    pub const EV_USE_ITEM7: unnamed_3 = 31;
    pub const PERS_SPAWN_COUNT: unnamed_0 = 4;
    pub const PW_REGEN: unnamed_2 = 5;
    pub const LEGS_WALKCR: unnamed_4 = 13;
    pub const EV_GLOBAL_SOUND: unnamed_3 = 46;
    pub const TEAM_BLUE: team_t = 2;
    pub const GT_1FCTF: gametype_t = 5;
    pub type unnamed_7 = libc::c_uint;
    pub type unnamed_4 = libc::c_uint;
    pub const PW_BLUEFLAG: unnamed_0 = 8;
    pub type unnamed_5 = libc::c_uint;
    pub const MOD_TARGET_LASER: unnamed_6 = 21;
    pub const ET_PLAYER: unnamed_7 = 1;
    pub const TORSO_ATTACK: unnamed_1 = 7;
    pub const EV_USE_ITEM9: unnamed_3 = 33;
    pub const WP_BFG: weapon_t = 9;
    pub const FLAG_RUN: unnamed_4 = 34;
    pub const EV_USE_ITEM1: unnamed_3 = 25;
    pub const PW_DOUBLER: powerup_t = 12;
    pub const TEAM_FREE: team_t = 0;
    pub const HI_NONE: holdable_t = 0;
    pub const ET_GENERAL: unnamed = 0;
    pub const PW_INVIS: unnamed_3 = 4;
    pub const BOTH_DEAD1: unnamed_5 = 1;
    pub const IT_AMMO: itemType_t = 2;
    pub const EV_STEP_12: unnamed_3 = 8;
    pub const TEAM_RED: team_t = 1;
    pub const PM_SPINTERMISSION: unnamed = 6;
    pub const MAX_TOTALANIMATIONS: unnamed_1 = 37;
    pub const GT_HARVESTER: gametype_t = 7;
    pub const GT_OBELISK: gametype_t = 6;
    pub const GT_TOURNAMENT: gametype_t = 1;
    pub const EV_GRENADE_BOUNCE: unnamed_3 = 44;
    pub const GENDER_NEUTER: gender_t = 2;
    pub type unnamed_0 = libc::c_uint;
    pub const LEGS_WALK: unnamed_1 = 14;
    pub const IT_HEALTH: itemType_t = 4;
    pub const PERS_RANK: unnamed_0 = 2;
    pub type gitem_t = gitem_s;
    pub const EV_PLAYER_TELEPORT_IN: unnamed_3 = 42;
    pub const EV_USE_ITEM2: unnamed_3 = 26;
    pub const TEAM_NUM_TEAMS: team_t = 4;
    pub const PM_SPECTATOR: unnamed = 2;
    pub const EV_WATER_UNDER: unnamed_3 = 17;
    pub const PW_REDFLAG: unnamed_2 = 7;
    pub const STAT_MAX_HEALTH: unnamed_0 = 6;
    pub const EV_PLAYER_TELEPORT_OUT: unnamed_3 = 43;
    pub const PW_NONE: powerup_t = 0;
    pub type unnamed_1 = libc::c_uint;
    pub type weapon_t = libc::c_uint;
    pub const PERS_TEAM: unnamed_1 = 3;
    pub const EV_OBELISKEXPLODE: unnamed_3 = 69;
    pub const PW_GUARD: unnamed_2 = 11;
    pub const PM_FREEZE: unnamed = 4;
    pub const EV_TAUNT_GUARDBASE: unnamed_3 = 81;
    pub const EV_USE_ITEM3: unnamed_3 = 27;
    pub const PW_QUAD: powerup_t = 1;
    pub const EV_DEATH1: unnamed_3 = 57;
    pub const MOD_LIGHTNING: unnamed_6 = 11;
    pub const PW_AMMOREGEN: unnamed_2 = 13;
    pub const STAT_HEALTH: unnamed_1 = 0;
    pub const EV_MISSILE_MISS: unnamed_3 = 51;
    pub const GT_CTF: gametype_t = 4;
    pub const PW_NUM_POWERUPS: unnamed_2 = 15;
    pub const BOTH_DEAD3: unnamed_1 = 5;
    pub type gametype_t = libc::c_uint;
    pub const EV_JUICED: unnamed_3 = 72;
    pub const EV_TAUNT_FOLLOWME: unnamed_3 = 79;
    pub const EV_WATER_LEAVE: unnamed_3 = 16;
    pub const EV_USE_ITEM5: unnamed_3 = 29;
    pub const LEGS_BACK: unnamed_5 = 16;
    pub const STAT_WEAPONS: unnamed_1 = 2;
    pub const TEAM_SPECTATOR: team_t = 3;
    pub const EV_USE_ITEM6: unnamed_3 = 30;
    pub const FLAG_STAND: unnamed_5 = 35;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct gitem_s {
        pub classname: *mut libc::c_char,
        pub pickup_sound: *mut libc::c_char,
        pub world_model: [*mut libc::c_char; 4],
        pub icon: *mut libc::c_char,
        pub pickup_name: *mut libc::c_char,
        pub quantity: libc::c_int,
        pub giType: itemType_t,
        pub giTag: libc::c_int,
        pub precaches: *mut libc::c_char,
        pub sounds: *mut libc::c_char,
    }
    pub const ET_INVISIBLE: unnamed_4 = 10;
    pub const BOTH_DEAD2: unnamed_5 = 3;
    pub type team_t = libc::c_uint;
    pub const EV_USE_ITEM10: unnamed_3 = 34;
    pub const PERS_CAPTURES: unnamed = 14;
    pub const PM_NOCLIP: unnamed_0 = 1;
    pub const EV_USE_ITEM8: unnamed_3 = 32;
    pub const PW_INVULNERABILITY: unnamed_3 = 14;
    pub const EV_PAIN: unnamed_3 = 56;
    pub const GENDER_FEMALE: gender_t = 1;
    pub const EV_USE_ITEM0: unnamed_3 = 24;
    pub const GENDER_MALE: gender_t = 0;
    pub const EV_USE_ITEM12: unnamed_3 = 36;
    pub const EV_USE_ITEM14: unnamed_3 = 38;
    pub const PM_DEAD: unnamed_0 = 3;
    pub const STAT_HOLDABLE_ITEM: unnamed_1 = 1;
    pub const WP_LIGHTNING: unnamed_2 = 6;
    pub const IT_HOLDABLE: itemType_t = 6;
    pub const MOD_LAVA: unnamed_6 = 16;
    pub const WP_GRAPPLING_HOOK: unnamed_0 = 10;
    pub type unnamed_3 = libc::c_uint;
    pub const ET_EVENTS: unnamed_4 = 13;
    pub const GT_MAX_GAME_TYPE: gametype_t = 8;
    use libc;
}
mod tr_types_h {
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
        cg_noPlayerAnims, cg_noProjectileTrail, cg_nopredict, cg_oldPlasma, cg_oldRail,
        cg_oldRocket, cg_paused, cg_predictItems, cg_railTrailTime, cg_runpitch, cg_runroll,
        cg_scorePlum, cg_shadows, cg_showmiss, cg_simpleItems, cg_smoothClients, cg_stats,
        cg_swingSpeed, cg_synchronousClients, cg_teamChatHeight, cg_teamChatTime, cg_teamChatsOnly,
        cg_thirdPerson, cg_thirdPersonAngle, cg_thirdPersonRange, cg_timescale,
        cg_timescaleFadeEnd, cg_timescaleFadeSpeed, cg_tracerChance, cg_tracerLength,
        cg_tracerWidth, cg_trueLightning, cg_viewsize, cg_weapons, cg_zoomFov, cgs, pmove_fixed,
        pmove_msec, CG_Argv, CG_BuildSpectatorString, CG_ConfigString, CG_CrosshairPlayer,
        CG_LastAttacker, CG_StartMusic, CG_UpdateCvars,
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
        CG_GrappleTrail, CG_MissileHitPlayer, CG_MissileHitWall, CG_NextWeapon_f,
        CG_OutOfAmmoChange, CG_PrevWeapon_f, CG_RailTrail, CG_RegisterItemVisuals, CG_ShotgunFire,
        CG_Weapon_f,
    };
    use q_shared_h::{byte, qboolean, qhandle_t, vec3_t};
    pub const STEREO_CENTER: stereoFrame_t = 0;
    pub const STEREO_RIGHT: stereoFrame_t = 2;
    pub type poly_t = poly_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct poly_s {
        pub hShader: qhandle_t,
        pub numVerts: libc::c_int,
        pub verts: *mut polyVert_t,
    }
    pub const STEREO_LEFT: stereoFrame_t = 1;
    pub type stereoFrame_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct polyVert_t {
        pub xyz: vec3_t,
        pub st: [libc::c_float; 2],
        pub modulate: [byte; 4],
    }
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    pub type glHardwareType_t = libc::c_uint;
    pub type refEntityType_t = libc::c_uint;
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    pub const RT_LIGHTNING: refEntityType_t = 6;
    pub type glDriverType_t = libc::c_uint;
    pub const RT_MAX_REF_ENTITY_TYPE: refEntityType_t = 8;
    pub const RT_RAIL_CORE: refEntityType_t = 4;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refdef_t {
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub fov_x: libc::c_float,
        pub fov_y: libc::c_float,
        pub vieworg: vec3_t,
        pub viewaxis: [vec3_t; 3],
        pub time: libc::c_int,
        pub rdflags: libc::c_int,
        pub areamask: [byte; 32],
        pub text: [[libc::c_char; 32]; 8],
    }
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    pub const TC_S3TC: textureCompression_t = 1;
    pub const RT_MODEL: refEntityType_t = 0;
    pub const GLDRV_ICD: glDriverType_t = 0;
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refEntity_t {
        pub reType: refEntityType_t,
        pub renderfx: libc::c_int,
        pub hModel: qhandle_t,
        pub lightingOrigin: vec3_t,
        pub shadowPlane: libc::c_float,
        pub axis: [vec3_t; 3],
        pub nonNormalizedAxes: qboolean,
        pub origin: [libc::c_float; 3],
        pub frame: libc::c_int,
        pub oldorigin: [libc::c_float; 3],
        pub oldframe: libc::c_int,
        pub backlerp: libc::c_float,
        pub skinNum: libc::c_int,
        pub customSkin: qhandle_t,
        pub customShader: qhandle_t,
        pub shaderRGBA: [byte; 4],
        pub shaderTexCoord: [libc::c_float; 2],
        pub shaderTime: libc::c_float,
        pub radius: libc::c_float,
        pub rotation: libc::c_float,
    }
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    pub const RT_BEAM: refEntityType_t = 3;
    pub type textureCompression_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: libc::c_int,
        pub numTextureUnits: libc::c_int,
        pub colorBits: libc::c_int,
        pub depthBits: libc::c_int,
        pub stencilBits: libc::c_int,
        pub driverType: glDriverType_t,
        pub hardwareType: glHardwareType_t,
        pub deviceSupportsGamma: qboolean,
        pub textureCompression: textureCompression_t,
        pub textureEnvAddAvailable: qboolean,
        pub vidWidth: libc::c_int,
        pub vidHeight: libc::c_int,
        pub windowAspect: libc::c_float,
        pub displayFrequency: libc::c_int,
        pub isFullscreen: qboolean,
        pub stereoEnabled: qboolean,
        pub smpActive: qboolean,
    }
    pub const RT_SPRITE: refEntityType_t = 2;
    pub const RT_PORTALSURFACE: refEntityType_t = 7;
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    pub const RT_RAIL_RINGS: refEntityType_t = 5;
    pub const RT_POLY: refEntityType_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    use libc;
}
