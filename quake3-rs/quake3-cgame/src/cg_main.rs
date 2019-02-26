#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           libc,
           ptr_wrapping_offset_from)]
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItemForHoldable,
    BG_FindItemForPowerup, BG_PlayerStateToEntityState, BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{
    c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, PM_UpdateViewAngles, Pmove,
};
use bg_public_h::{
    animation_s, animation_t, gametype_t, gender_t, gitem_s, gitem_t, itemType_t, team_t, unnamed,
    unnamed_0, GENDER_FEMALE, GENDER_MALE, GENDER_NEUTER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR,
    IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON,
    PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT,
    PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED,
    PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, TEAM_BLUE, TEAM_FREE,
    TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK,
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
use cg_event::{CG_CheckEvents, CG_EntityEvent, CG_PainEvent, CG_PlaceString};
use cg_info::{CG_DrawInformation, CG_LoadingClient, CG_LoadingItem, CG_LoadingString};
use cg_local_h::{
    centity_s, centity_t, cgMedia_t, cg_t, cgs_t, clientInfo_t, footstep_t, itemInfo_t,
    lerpFrame_t, playerEntity_t, score_t, trap_Argv, trap_CM_LoadMap, trap_CM_NumInlineModels,
    trap_Cvar_Register, trap_Cvar_Set, trap_Cvar_Update, trap_Cvar_VariableStringBuffer,
    trap_GetGameState, trap_GetGlconfig, trap_R_ClearScene, trap_R_LoadWorldMap,
    trap_R_ModelBounds, trap_R_RegisterModel, trap_R_RegisterShader, trap_R_RegisterShaderNoMip,
    trap_S_ClearLoopingSounds, trap_S_RegisterSound, trap_S_StartBackgroundTrack, weaponInfo_s,
    weaponInfo_t, FOOTSTEP_BOOT, FOOTSTEP_ENERGY, FOOTSTEP_FLESH, FOOTSTEP_MECH, FOOTSTEP_METAL,
    FOOTSTEP_NORMAL, FOOTSTEP_SPLASH, FOOTSTEP_TOTAL,
};
use cg_localents::{CG_AddLocalEntities, CG_AllocLocalEntity, CG_InitLocalEntities};
use cg_marks::{CG_AddMarks, CG_ImpactMark, CG_InitMarkPolys};
use cg_particles::{CG_AddParticles, CG_ClearParticles, CG_ParticleExplosion};
use cg_players::{
    CG_AddRefEntityWithPowerups, CG_CustomSound, CG_LoadDeferredPlayers, CG_NewClientInfo,
    CG_Player, CG_ResetPlayerEntity,
};
use cg_playerstate::{CG_Respawn, CG_TransitionPlayerState};
use cg_predict::{CG_BuildSolidList, CG_PointContents, CG_PredictPlayerState, CG_Trace};
use cg_public_h::{
    snapshot_t, unnamed_1, CG_CONSOLE_COMMAND, CG_CROSSHAIR_PLAYER, CG_DRAW_ACTIVE_FRAME,
    CG_EVENT_HANDLING, CG_INIT, CG_KEY_EVENT, CG_LAST_ATTACKER, CG_MOUSE_EVENT, CG_SHUTDOWN,
};
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
    byte, clipHandle_t, cvarHandle_t, entityState_s, entityState_t, gameState_t, playerState_s,
    playerState_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, trType_t, trajectory_t, va,
    vec3_t, vec_t, vmCvar_t, COM_Parse, Com_sprintf, Q_strcat, Q_strncpyz, TR_GRAVITY,
    TR_INTERPOLATE, TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{atoi, intptr_t, memcpy, memset, strcmp, strlen};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    stereoFrame_t, textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D,
    GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING,
    RT_MAX_REF_ENTITY_TYPE, RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS,
    RT_SPRITE, STEREO_CENTER, STEREO_LEFT, STEREO_RIGHT, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};
extern crate libc;

//==============================================================================
#[no_mangle]
pub static mut cgs: cgs_t = cgs_t {
    gameState: gameState_t {
        stringOffsets: [0; 1024],
        stringData: [0; 16000],
        dataCount: 0,
    },
    glconfig: glconfig_t {
        renderer_string: [0; 1024],
        vendor_string: [0; 1024],
        version_string: [0; 1024],
        extensions_string: [0; 8192],
        maxTextureSize: 0,
        numTextureUnits: 0,
        colorBits: 0,
        depthBits: 0,
        stencilBits: 0,
        driverType: GLDRV_ICD,
        hardwareType: GLHW_GENERIC,
        deviceSupportsGamma: qfalse,
        textureCompression: TC_NONE,
        textureEnvAddAvailable: qfalse,
        vidWidth: 0,
        vidHeight: 0,
        windowAspect: 0.,
        displayFrequency: 0,
        isFullscreen: qfalse,
        stereoEnabled: qfalse,
        smpActive: qfalse,
    },
    screenXScale: 0.,
    screenYScale: 0.,
    screenXBias: 0.,
    serverCommandSequence: 0,
    processedSnapshotNum: 0,
    localServer: qfalse,
    gametype: GT_FFA,
    dmflags: 0,
    teamflags: 0,
    fraglimit: 0,
    capturelimit: 0,
    timelimit: 0,
    maxclients: 0,
    mapname: [0; 64],
    redTeam: [0; 64],
    blueTeam: [0; 64],
    voteTime: 0,
    voteYes: 0,
    voteNo: 0,
    voteModified: qfalse,
    voteString: [0; 1024],
    teamVoteTime: [0; 2],
    teamVoteYes: [0; 2],
    teamVoteNo: [0; 2],
    teamVoteModified: [qfalse; 2],
    teamVoteString: [[0; 1024]; 2],
    levelStartTime: 0,
    scores1: 0,
    scores2: 0,
    redflag: 0,
    blueflag: 0,
    flagStatus: 0,
    newHud: qfalse,
    gameModels: [0; 256],
    gameSounds: [0; 256],
    numInlineModels: 0,
    inlineDrawModel: [0; 256],
    inlineModelMidpoints: [[0.; 3]; 256],
    clientinfo: [clientInfo_t {
        infoValid: qfalse,
        name: [0; 64],
        team: TEAM_FREE,
        botSkill: 0,
        color1: [0.; 3],
        color2: [0.; 3],
        c1RGBA: [0; 4],
        c2RGBA: [0; 4],
        score: 0,
        location: 0,
        health: 0,
        armor: 0,
        curWeapon: 0,
        handicap: 0,
        wins: 0,
        losses: 0,
        teamTask: 0,
        teamLeader: qfalse,
        powerups: 0,
        medkitUsageTime: 0,
        invulnerabilityStartTime: 0,
        invulnerabilityStopTime: 0,
        breathPuffTime: 0,
        modelName: [0; 64],
        skinName: [0; 64],
        headModelName: [0; 64],
        headSkinName: [0; 64],
        redTeam: [0; 32],
        blueTeam: [0; 32],
        deferred: qfalse,
        newAnims: qfalse,
        fixedlegs: qfalse,
        fixedtorso: qfalse,
        headOffset: [0.; 3],
        footsteps: FOOTSTEP_NORMAL,
        gender: GENDER_MALE,
        legsModel: 0,
        legsSkin: 0,
        torsoModel: 0,
        torsoSkin: 0,
        headModel: 0,
        headSkin: 0,
        modelIcon: 0,
        animations: [animation_s {
            firstFrame: 0,
            numFrames: 0,
            loopFrames: 0,
            frameLerp: 0,
            initialLerp: 0,
            reversed: 0,
            flipflop: 0,
        }; 37],
        sounds: [0; 32],
    }; 64],
    teamChatMsgs: [[0; 241]; 8],
    teamChatMsgTimes: [0; 8],
    teamChatPos: 0,
    teamLastChatPos: 0,
    cursorX: 0,
    cursorY: 0,
    eventHandling: qfalse,
    mouseCaptured: qfalse,
    sizingHud: qfalse,
    capturedItem: 0 as *const libc::c_void as *mut libc::c_void,
    activeCursor: 0,
    currentOrder: 0,
    orderPending: qfalse,
    orderTime: 0,
    currentVoiceClient: 0,
    acceptOrderTime: 0,
    acceptTask: 0,
    acceptLeader: 0,
    acceptVoice: [0; 32],
    media: cgMedia_t {
        charsetShader: 0,
        charsetProp: 0,
        charsetPropGlow: 0,
        charsetPropB: 0,
        whiteShader: 0,
        redFlagModel: 0,
        blueFlagModel: 0,
        neutralFlagModel: 0,
        redFlagShader: [0; 3],
        blueFlagShader: [0; 3],
        flagShader: [0; 4],
        flagPoleModel: 0,
        flagFlapModel: 0,
        redFlagFlapSkin: 0,
        blueFlagFlapSkin: 0,
        neutralFlagFlapSkin: 0,
        redFlagBaseModel: 0,
        blueFlagBaseModel: 0,
        neutralFlagBaseModel: 0,
        armorModel: 0,
        armorIcon: 0,
        teamStatusBar: 0,
        deferShader: 0,
        gibAbdomen: 0,
        gibArm: 0,
        gibChest: 0,
        gibFist: 0,
        gibFoot: 0,
        gibForearm: 0,
        gibIntestine: 0,
        gibLeg: 0,
        gibSkull: 0,
        gibBrain: 0,
        smoke2: 0,
        machinegunBrassModel: 0,
        shotgunBrassModel: 0,
        railRingsShader: 0,
        railCoreShader: 0,
        lightningShader: 0,
        friendShader: 0,
        balloonShader: 0,
        connectionShader: 0,
        selectShader: 0,
        viewBloodShader: 0,
        tracerShader: 0,
        crosshairShader: [0; 10],
        lagometerShader: 0,
        backTileShader: 0,
        noammoShader: 0,
        smokePuffShader: 0,
        smokePuffRageProShader: 0,
        shotgunSmokePuffShader: 0,
        plasmaBallShader: 0,
        waterBubbleShader: 0,
        bloodTrailShader: 0,
        numberShaders: [0; 11],
        shadowMarkShader: 0,
        botSkillShaders: [0; 5],
        wakeMarkShader: 0,
        bloodMarkShader: 0,
        bulletMarkShader: 0,
        burnMarkShader: 0,
        holeMarkShader: 0,
        energyMarkShader: 0,
        quadShader: 0,
        redQuadShader: 0,
        quadWeaponShader: 0,
        invisShader: 0,
        regenShader: 0,
        battleSuitShader: 0,
        battleWeaponShader: 0,
        hastePuffShader: 0,
        bulletFlashModel: 0,
        ringFlashModel: 0,
        dishFlashModel: 0,
        lightningExplosionModel: 0,
        railExplosionShader: 0,
        plasmaExplosionShader: 0,
        bulletExplosionShader: 0,
        rocketExplosionShader: 0,
        grenadeExplosionShader: 0,
        bfgExplosionShader: 0,
        bloodExplosionShader: 0,
        teleportEffectModel: 0,
        teleportEffectShader: 0,
        scoreboardName: 0,
        scoreboardPing: 0,
        scoreboardScore: 0,
        scoreboardTime: 0,
        medalImpressive: 0,
        medalExcellent: 0,
        medalGauntlet: 0,
        medalDefend: 0,
        medalAssist: 0,
        medalCapture: 0,
        quadSound: 0,
        tracerSound: 0,
        selectSound: 0,
        useNothingSound: 0,
        wearOffSound: 0,
        footsteps: [[0; 4]; 7],
        sfx_lghit1: 0,
        sfx_lghit2: 0,
        sfx_lghit3: 0,
        sfx_ric1: 0,
        sfx_ric2: 0,
        sfx_ric3: 0,
        sfx_rockexp: 0,
        sfx_plasmaexp: 0,
        gibSound: 0,
        gibBounce1Sound: 0,
        gibBounce2Sound: 0,
        gibBounce3Sound: 0,
        teleInSound: 0,
        teleOutSound: 0,
        noAmmoSound: 0,
        respawnSound: 0,
        talkSound: 0,
        landSound: 0,
        fallSound: 0,
        jumpPadSound: 0,
        oneMinuteSound: 0,
        fiveMinuteSound: 0,
        suddenDeathSound: 0,
        threeFragSound: 0,
        twoFragSound: 0,
        oneFragSound: 0,
        hitSound: 0,
        hitSoundHighArmor: 0,
        hitSoundLowArmor: 0,
        hitTeamSound: 0,
        impressiveSound: 0,
        excellentSound: 0,
        deniedSound: 0,
        humiliationSound: 0,
        assistSound: 0,
        defendSound: 0,
        firstImpressiveSound: 0,
        firstExcellentSound: 0,
        firstHumiliationSound: 0,
        takenLeadSound: 0,
        tiedLeadSound: 0,
        lostLeadSound: 0,
        voteNow: 0,
        votePassed: 0,
        voteFailed: 0,
        watrInSound: 0,
        watrOutSound: 0,
        watrUnSound: 0,
        flightSound: 0,
        medkitSound: 0,
        captureAwardSound: 0,
        redScoredSound: 0,
        blueScoredSound: 0,
        redLeadsSound: 0,
        blueLeadsSound: 0,
        teamsTiedSound: 0,
        captureYourTeamSound: 0,
        captureOpponentSound: 0,
        returnYourTeamSound: 0,
        returnOpponentSound: 0,
        takenYourTeamSound: 0,
        takenOpponentSound: 0,
        redFlagReturnedSound: 0,
        blueFlagReturnedSound: 0,
        enemyTookYourFlagSound: 0,
        yourTeamTookEnemyFlagSound: 0,
        youHaveFlagSound: 0,
        holyShitSound: 0,
        count3Sound: 0,
        count2Sound: 0,
        count1Sound: 0,
        countFightSound: 0,
        countPrepareSound: 0,
        regenSound: 0,
        protectSound: 0,
        n_healthSound: 0,
        hgrenb1aSound: 0,
        hgrenb2aSound: 0,
        wstbimplSound: 0,
        wstbimpmSound: 0,
        wstbimpdSound: 0,
        wstbactvSound: 0,
    },
};
#[no_mangle]
pub static mut cg: cg_t = cg_t {
    clientFrame: 0,
    clientNum: 0,
    demoPlayback: qfalse,
    levelShot: qfalse,
    deferredPlayerLoading: 0,
    loading: qfalse,
    intermissionStarted: qfalse,
    latestSnapshotNum: 0,
    latestSnapshotTime: 0,
    snap: 0 as *const snapshot_t as *mut snapshot_t,
    nextSnap: 0 as *const snapshot_t as *mut snapshot_t,
    activeSnapshots: [snapshot_t {
        snapFlags: 0,
        ping: 0,
        serverTime: 0,
        areamask: [0; 32],
        ps: playerState_s {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        },
        numEntities: 0,
        entities: [entityState_s {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: trajectory_t {
                trType: TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: trajectory_t {
                trType: TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            time: 0,
            time2: 0,
            origin: [0.; 3],
            origin2: [0.; 3],
            angles: [0.; 3],
            angles2: [0.; 3],
            otherEntityNum: 0,
            otherEntityNum2: 0,
            groundEntityNum: 0,
            constantLight: 0,
            loopSound: 0,
            modelindex: 0,
            modelindex2: 0,
            clientNum: 0,
            frame: 0,
            solid: 0,
            event: 0,
            eventParm: 0,
            powerups: 0,
            weapon: 0,
            legsAnim: 0,
            torsoAnim: 0,
            generic1: 0,
        }; 256],
        numServerCommands: 0,
        serverCommandSequence: 0,
    }; 2],
    frameInterpolation: 0.,
    thisFrameTeleport: qfalse,
    nextFrameTeleport: qfalse,
    frametime: 0,
    time: 0,
    oldTime: 0,
    physicsTime: 0,
    timelimitWarnings: 0,
    fraglimitWarnings: 0,
    mapRestart: qfalse,
    renderingThirdPerson: qfalse,
    hyperspace: qfalse,
    predictedPlayerState: playerState_s {
        commandTime: 0,
        pm_type: 0,
        bobCycle: 0,
        pm_flags: 0,
        pm_time: 0,
        origin: [0.; 3],
        velocity: [0.; 3],
        weaponTime: 0,
        gravity: 0,
        speed: 0,
        delta_angles: [0; 3],
        groundEntityNum: 0,
        legsTimer: 0,
        legsAnim: 0,
        torsoTimer: 0,
        torsoAnim: 0,
        movementDir: 0,
        grapplePoint: [0.; 3],
        eFlags: 0,
        eventSequence: 0,
        events: [0; 2],
        eventParms: [0; 2],
        externalEvent: 0,
        externalEventParm: 0,
        externalEventTime: 0,
        clientNum: 0,
        weapon: 0,
        weaponstate: 0,
        viewangles: [0.; 3],
        viewheight: 0,
        damageEvent: 0,
        damageYaw: 0,
        damagePitch: 0,
        damageCount: 0,
        stats: [0; 16],
        persistant: [0; 16],
        powerups: [0; 16],
        ammo: [0; 16],
        generic1: 0,
        loopSound: 0,
        jumppad_ent: 0,
        ping: 0,
        pmove_framecount: 0,
        jumppad_frame: 0,
        entityEventSequence: 0,
    },
    predictedPlayerEntity: centity_s {
        currentState: entityState_s {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: trajectory_t {
                trType: TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: trajectory_t {
                trType: TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            time: 0,
            time2: 0,
            origin: [0.; 3],
            origin2: [0.; 3],
            angles: [0.; 3],
            angles2: [0.; 3],
            otherEntityNum: 0,
            otherEntityNum2: 0,
            groundEntityNum: 0,
            constantLight: 0,
            loopSound: 0,
            modelindex: 0,
            modelindex2: 0,
            clientNum: 0,
            frame: 0,
            solid: 0,
            event: 0,
            eventParm: 0,
            powerups: 0,
            weapon: 0,
            legsAnim: 0,
            torsoAnim: 0,
            generic1: 0,
        },
        nextState: entityState_s {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: trajectory_t {
                trType: TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: trajectory_t {
                trType: TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            time: 0,
            time2: 0,
            origin: [0.; 3],
            origin2: [0.; 3],
            angles: [0.; 3],
            angles2: [0.; 3],
            otherEntityNum: 0,
            otherEntityNum2: 0,
            groundEntityNum: 0,
            constantLight: 0,
            loopSound: 0,
            modelindex: 0,
            modelindex2: 0,
            clientNum: 0,
            frame: 0,
            solid: 0,
            event: 0,
            eventParm: 0,
            powerups: 0,
            weapon: 0,
            legsAnim: 0,
            torsoAnim: 0,
            generic1: 0,
        },
        interpolate: qfalse,
        currentValid: qfalse,
        muzzleFlashTime: 0,
        previousEvent: 0,
        teleportFlag: 0,
        trailTime: 0,
        dustTrailTime: 0,
        miscTime: 0,
        snapShotTime: 0,
        pe: playerEntity_t {
            legs: lerpFrame_t {
                oldFrame: 0,
                oldFrameTime: 0,
                frame: 0,
                frameTime: 0,
                backlerp: 0.,
                yawAngle: 0.,
                yawing: qfalse,
                pitchAngle: 0.,
                pitching: qfalse,
                animationNumber: 0,
                animation: 0 as *const animation_t as *mut animation_t,
                animationTime: 0,
            },
            torso: lerpFrame_t {
                oldFrame: 0,
                oldFrameTime: 0,
                frame: 0,
                frameTime: 0,
                backlerp: 0.,
                yawAngle: 0.,
                yawing: qfalse,
                pitchAngle: 0.,
                pitching: qfalse,
                animationNumber: 0,
                animation: 0 as *const animation_t as *mut animation_t,
                animationTime: 0,
            },
            flag: lerpFrame_t {
                oldFrame: 0,
                oldFrameTime: 0,
                frame: 0,
                frameTime: 0,
                backlerp: 0.,
                yawAngle: 0.,
                yawing: qfalse,
                pitchAngle: 0.,
                pitching: qfalse,
                animationNumber: 0,
                animation: 0 as *const animation_t as *mut animation_t,
                animationTime: 0,
            },
            painTime: 0,
            painDirection: 0,
            lightningFiring: 0,
            railFireTime: 0,
            barrelAngle: 0.,
            barrelTime: 0,
            barrelSpinning: qfalse,
        },
        errorTime: 0,
        errorOrigin: [0.; 3],
        errorAngles: [0.; 3],
        extrapolated: qfalse,
        rawOrigin: [0.; 3],
        rawAngles: [0.; 3],
        beamEnd: [0.; 3],
        lerpOrigin: [0.; 3],
        lerpAngles: [0.; 3],
    },
    validPPS: qfalse,
    predictedErrorTime: 0,
    predictedError: [0.; 3],
    eventSequence: 0,
    predictableEvents: [0; 16],
    stepChange: 0.,
    stepTime: 0,
    duckChange: 0.,
    duckTime: 0,
    landChange: 0.,
    landTime: 0,
    weaponSelect: 0,
    autoAngles: [0.; 3],
    autoAxis: [[0.; 3]; 3],
    autoAnglesFast: [0.; 3],
    autoAxisFast: [[0.; 3]; 3],
    refdef: refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        text: [[0; 32]; 8],
    },
    refdefViewAngles: [0.; 3],
    zoomed: qfalse,
    zoomTime: 0,
    zoomSensitivity: 0.,
    infoScreenText: [0; 1024],
    scoresRequestTime: 0,
    numScores: 0,
    selectedScore: 0,
    teamScores: [0; 2],
    scores: [score_t {
        client: 0,
        score: 0,
        ping: 0,
        time: 0,
        scoreFlags: 0,
        powerUps: 0,
        accuracy: 0,
        impressiveCount: 0,
        excellentCount: 0,
        guantletCount: 0,
        defendCount: 0,
        assistCount: 0,
        captures: 0,
        perfect: qfalse,
        team: 0,
    }; 64],
    showScores: qfalse,
    scoreBoardShowing: qfalse,
    scoreFadeTime: 0,
    killerName: [0; 32],
    spectatorList: [0; 1024],
    spectatorLen: 0,
    spectatorWidth: 0.,
    spectatorTime: 0,
    spectatorPaintX: 0,
    spectatorPaintX2: 0,
    spectatorOffset: 0,
    spectatorPaintLen: 0,
    centerPrintTime: 0,
    centerPrintCharWidth: 0,
    centerPrintY: 0,
    centerPrint: [0; 1024],
    centerPrintLines: 0,
    lowAmmoWarning: 0,
    crosshairClientNum: 0,
    crosshairClientTime: 0,
    powerupActive: 0,
    powerupTime: 0,
    attackerTime: 0,
    voiceTime: 0,
    rewardStack: 0,
    rewardTime: 0,
    rewardCount: [0; 10],
    rewardShader: [0; 10],
    rewardSound: [0; 10],
    soundBufferIn: 0,
    soundBufferOut: 0,
    soundTime: 0,
    soundBuffer: [0; 20],
    warmup: 0,
    warmupCount: 0,
    itemPickup: 0,
    itemPickupTime: 0,
    itemPickupBlendTime: 0,
    weaponSelectTime: 0,
    weaponAnimation: 0,
    weaponAnimationTime: 0,
    damageTime: 0.,
    damageX: 0.,
    damageY: 0.,
    damageValue: 0.,
    headYaw: 0.,
    headEndPitch: 0.,
    headEndYaw: 0.,
    headEndTime: 0,
    headStartPitch: 0.,
    headStartYaw: 0.,
    headStartTime: 0,
    v_dmg_time: 0.,
    v_dmg_pitch: 0.,
    v_dmg_roll: 0.,
    bobfracsin: 0.,
    bobcycle: 0,
    xyspeed: 0.,
    nextOrbitTime: 0,
    testModelEntity: refEntity_t {
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
    },
    testModelName: [0; 64],
    testGun: qfalse,
};
#[no_mangle]
pub static mut cg_entities: [centity_t; 1024] = [centity_s {
    currentState: entityState_s {
        number: 0,
        eType: 0,
        eFlags: 0,
        pos: trajectory_t {
            trType: TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        apos: trajectory_t {
            trType: TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        time: 0,
        time2: 0,
        origin: [0.; 3],
        origin2: [0.; 3],
        angles: [0.; 3],
        angles2: [0.; 3],
        otherEntityNum: 0,
        otherEntityNum2: 0,
        groundEntityNum: 0,
        constantLight: 0,
        loopSound: 0,
        modelindex: 0,
        modelindex2: 0,
        clientNum: 0,
        frame: 0,
        solid: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
        generic1: 0,
    },
    nextState: entityState_s {
        number: 0,
        eType: 0,
        eFlags: 0,
        pos: trajectory_t {
            trType: TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        apos: trajectory_t {
            trType: TR_STATIONARY,
            trTime: 0,
            trDuration: 0,
            trBase: [0.; 3],
            trDelta: [0.; 3],
        },
        time: 0,
        time2: 0,
        origin: [0.; 3],
        origin2: [0.; 3],
        angles: [0.; 3],
        angles2: [0.; 3],
        otherEntityNum: 0,
        otherEntityNum2: 0,
        groundEntityNum: 0,
        constantLight: 0,
        loopSound: 0,
        modelindex: 0,
        modelindex2: 0,
        clientNum: 0,
        frame: 0,
        solid: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
        generic1: 0,
    },
    interpolate: qfalse,
    currentValid: qfalse,
    muzzleFlashTime: 0,
    previousEvent: 0,
    teleportFlag: 0,
    trailTime: 0,
    dustTrailTime: 0,
    miscTime: 0,
    snapShotTime: 0,
    pe: playerEntity_t {
        legs: lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: qfalse,
            pitchAngle: 0.,
            pitching: qfalse,
            animationNumber: 0,
            animation: 0 as *const animation_t as *mut animation_t,
            animationTime: 0,
        },
        torso: lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: qfalse,
            pitchAngle: 0.,
            pitching: qfalse,
            animationNumber: 0,
            animation: 0 as *const animation_t as *mut animation_t,
            animationTime: 0,
        },
        flag: lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: qfalse,
            pitchAngle: 0.,
            pitching: qfalse,
            animationNumber: 0,
            animation: 0 as *const animation_t as *mut animation_t,
            animationTime: 0,
        },
        painTime: 0,
        painDirection: 0,
        lightningFiring: 0,
        railFireTime: 0,
        barrelAngle: 0.,
        barrelTime: 0,
        barrelSpinning: qfalse,
    },
    errorTime: 0,
    errorOrigin: [0.; 3],
    errorAngles: [0.; 3],
    extrapolated: qfalse,
    rawOrigin: [0.; 3],
    rawAngles: [0.; 3],
    beamEnd: [0.; 3],
    lerpOrigin: [0.; 3],
    lerpAngles: [0.; 3],
}; 1024];
#[no_mangle]
pub static mut cg_weapons: [weaponInfo_t; 16] = [weaponInfo_s {
    registered: qfalse,
    item: 0 as *const gitem_t as *mut gitem_t,
    handsModel: 0,
    weaponModel: 0,
    barrelModel: 0,
    flashModel: 0,
    weaponMidpoint: [0.; 3],
    flashDlight: 0.,
    flashDlightColor: [0.; 3],
    flashSound: [0; 4],
    weaponIcon: 0,
    ammoIcon: 0,
    ammoModel: 0,
    missileModel: 0,
    missileSound: 0,
    missileTrailFunc: None,
    missileDlight: 0.,
    missileDlightColor: [0.; 3],
    missileRenderfx: 0,
    ejectBrassFunc: None,
    trailRadius: 0.,
    wiTrailTime: 0.,
    readySound: 0,
    firingSound: 0,
}; 16];
#[no_mangle]
pub static mut cg_items: [itemInfo_t; 256] = [itemInfo_t {
    registered: qfalse,
    models: [0; 4],
    icon: 0,
}; 256];
#[no_mangle]
pub static mut cg_centertime: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_runpitch: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_runroll: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_bobup: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_bobpitch: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_bobroll: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_swingSpeed: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_shadows: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_gibs: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawTimer: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawFPS: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawSnapshot: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_draw3dIcons: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawIcons: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawAmmoWarning: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawCrosshair: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawCrosshairNames: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawRewards: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawTeamOverlay: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_teamOverlayUserinfo: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_crosshairX: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_crosshairY: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_crosshairSize: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_crosshairHealth: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawStatus: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_draw2D: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_animSpeed: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_debugAnim: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_debugPosition: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_debugEvents: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_railTrailTime: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_errorDecay: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_nopredict: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_noPlayerAnims: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_showmiss: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_footsteps: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_addMarks: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_brassTime: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_gun_frame: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_gun_x: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_gun_y: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_gun_z: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawGun: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_viewsize: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_tracerChance: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_tracerWidth: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_tracerLength: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_autoswitch: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_ignore: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_simpleItems: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_fov: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_zoomFov: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_thirdPersonRange: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_thirdPersonAngle: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_thirdPerson: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_lagometer: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawAttacker: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_synchronousClients: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_teamChatTime: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_teamChatHeight: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_stats: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_forceModel: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_buildScript: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_paused: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_blood: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_predictItems: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_deferPlayers: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_drawFriend: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_teamChatsOnly: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_scorePlum: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_smoothClients: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut pmove_fixed: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut pmove_msec: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//extern	vmCvar_t		cg_pmove_fixed;
#[no_mangle]
pub static mut cg_cameraOrbit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_cameraOrbitDelay: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_timescaleFadeEnd: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_timescaleFadeSpeed: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_timescale: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_cameraMode: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_noProjectileTrail: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_oldRail: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_oldRocket: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_oldPlasma: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut cg_trueLightning: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//
// cg_main.c
//
#[no_mangle]
pub unsafe extern "C" fn CG_ConfigString(mut index: libc::c_int) -> *const libc::c_char {
    if index < 0i32 || index >= 1024i32 {
        CG_Error(
            b"CG_ConfigString: bad index: %i\x00" as *const u8 as *const libc::c_char,
            index,
        );
    }
    return cgs
        .gameState
        .stringData
        .as_mut_ptr()
        .offset(cgs.gameState.stringOffsets[index as usize] as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CG_Argv(mut arg: libc::c_int) -> *const libc::c_char {
    static mut buffer: [libc::c_char; 1024] = [0; 1024];
    trap_Argv(
        arg,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    return buffer.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn CG_StartMusic() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parm1: [libc::c_char; 64] = [0; 64];
    let mut parm2: [libc::c_char; 64] = [0; 64];
    s = CG_ConfigString(2i32) as *mut libc::c_char;
    Q_strncpyz(
        parm1.as_mut_ptr(),
        COM_Parse(&mut s),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_strncpyz(
        parm2.as_mut_ptr(),
        COM_Parse(&mut s),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    trap_S_StartBackgroundTrack(parm1.as_mut_ptr(), parm2.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CG_UpdateCvars() {
    let mut i: libc::c_int = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    i = 0i32;
    cv = cvarTable.as_mut_ptr();
    while i < cvarTableSize {
        trap_Cvar_Update((*cv).vmCvar);
        i += 1;
        cv = cv.offset(1isize)
    }
    if drawTeamOverlayModificationCount != cg_drawTeamOverlay.modificationCount {
        drawTeamOverlayModificationCount = cg_drawTeamOverlay.modificationCount;
        if cg_drawTeamOverlay.integer > 0i32 {
            trap_Cvar_Set(
                b"teamoverlay\x00" as *const u8 as *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            trap_Cvar_Set(
                b"teamoverlay\x00" as *const u8 as *const libc::c_char,
                b"0\x00" as *const u8 as *const libc::c_char,
            );
        }
    }
    if forceModelModificationCount != cg_forceModel.modificationCount {
        forceModelModificationCount = cg_forceModel.modificationCount;
        CG_ForceModelChange();
    };
}
/*
===================
CG_ForceModelChange
===================
*/
unsafe extern "C" fn CG_ForceModelChange() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 64i32 {
        let mut clientInfo: *const libc::c_char = 0 as *const libc::c_char;
        clientInfo = CG_ConfigString(32i32 + 256i32 + 256i32 + i);
        if !(0 == *clientInfo.offset(0isize)) {
            CG_NewClientInfo(i);
        }
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
// cg_main.c -- initialization and primary entry point for cgame
#[no_mangle]
pub static mut forceModelModificationCount: libc::c_int = -1i32;
// Initialized in run_static_initializers
static mut cvarTableSize: libc::c_int = 0;
static mut cvarTable: [cvarTable_t; 83] = unsafe {
    [
        cvarTable_t {
            vmCvar: &cg_ignore as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_ignore\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_autoswitch as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_autoswitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawGun as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawGun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_zoomFov as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_zoomfov\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"22.5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_fov as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_fov\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"90\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_viewsize as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_viewsize\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"100\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_shadows as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_shadows\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_gibs as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_gibs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_draw2D as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_draw2D\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawStatus as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawStatus\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawTimer as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawTimer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawFPS as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawFPS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawSnapshot as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawSnapshot\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_draw3dIcons as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_draw3dIcons\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawIcons as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawIcons\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawAmmoWarning as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawAmmoWarning\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawAttacker as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawAttacker\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawCrosshair as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawCrosshair\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawCrosshairNames as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawCrosshairNames\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawRewards as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawRewards\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_crosshairSize as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_crosshairSize\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"24\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_crosshairHealth as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_crosshairHealth\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_crosshairX as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_crosshairX\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_crosshairY as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_crosshairY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_brassTime as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_brassTime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"2500\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_simpleItems as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_simpleItems\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_addMarks as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_marks\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_lagometer as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_lagometer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_railTrailTime as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_railTrailTime\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"400\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_gun_x as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_gunX\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_gun_y as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_gunY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_gun_z as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_gunZ\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_centertime as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_centertime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_runpitch as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_runpitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0.002\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_runroll as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_runroll\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0.005\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_bobup as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_bobup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0.005\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_bobpitch as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_bobpitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0.002\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_bobroll as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_bobroll\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0.002\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_swingSpeed as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_swingSpeed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0.3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_animSpeed as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_animspeed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_debugAnim as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_debuganim\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_debugPosition as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_debugposition\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_debugEvents as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_debugevents\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_errorDecay as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_errordecay\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"100\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_nopredict as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_nopredict\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_noPlayerAnims as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_noplayeranims\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_showmiss as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_showmiss\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_footsteps as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_footsteps\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_tracerChance as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_tracerchance\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0.4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_tracerWidth as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_tracerwidth\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_tracerLength as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_tracerlength\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"100\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_thirdPersonRange as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_thirdPersonRange\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"40\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_thirdPersonAngle as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_thirdPersonAngle\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_thirdPerson as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_thirdPerson\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_teamChatTime as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_teamChatTime\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"3000\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_teamChatHeight as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_teamChatHeight\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_forceModel as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_forceModel\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_predictItems as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_predictItems\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_deferPlayers as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_deferPlayers\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawTeamOverlay as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawTeamOverlay\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_teamOverlayUserinfo as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"teamoverlay\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x40i32 | 0x2i32,
        },
        cvarTable_t {
            vmCvar: &cg_stats as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_stats\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_drawFriend as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawFriend\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_teamChatsOnly as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_teamChatsOnly\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_buildScript as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"com_buildScript\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_paused as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cl_paused\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x40i32,
        },
        cvarTable_t {
            vmCvar: &cg_blood as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"com_blood\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_synchronousClients as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_synchronousClients\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x8i32,
        },
        cvarTable_t {
            vmCvar: &cg_cameraOrbit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_cameraOrbit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &cg_cameraOrbitDelay as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_cameraOrbitDelay\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"50\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_timescaleFadeEnd as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_timescaleFadeEnd\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_timescaleFadeSpeed as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_timescaleFadeSpeed\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_timescale as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"timescale\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
        cvarTable_t {
            vmCvar: &cg_scorePlum as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_scorePlums\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x2i32 | 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_smoothClients as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_smoothClients\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x2i32 | 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_cameraMode as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"com_cameraMode\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x200i32,
        },
        cvarTable_t {
            vmCvar: &pmove_fixed as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"pmove_fixed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x8i32,
        },
        cvarTable_t {
            vmCvar: &pmove_msec as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"pmove_msec\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x8i32,
        },
        cvarTable_t {
            vmCvar: &cg_noProjectileTrail as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_noProjectileTrail\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_oldRail as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_oldRail\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_oldRocket as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_oldRocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_oldPlasma as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_oldPlasma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &cg_trueLightning as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_trueLightning\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0.0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn CG_CrosshairPlayer() -> libc::c_int {
    if cg.time > cg.crosshairClientTime + 1000i32 {
        return -1i32;
    }
    return cg.crosshairClientNum;
}
#[no_mangle]
pub unsafe extern "C" fn CG_LastAttacker() -> libc::c_int {
    if 0 == cg.attackerTime {
        return -1i32;
    }
    return (*cg.snap).ps.persistant[PERS_ATTACKER as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn CG_KeyEvent(mut key: libc::c_int, mut down: qboolean) {}
#[no_mangle]
pub unsafe extern "C" fn CG_MouseEvent(mut x: libc::c_int, mut y: libc::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn CG_EventHandling(mut type_0: libc::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn CG_BuildSpectatorString() {
    let mut i: libc::c_int = 0;
    cg.spectatorList[0usize] = 0i32 as libc::c_char;
    i = 0i32;
    while i < 64i32 {
        if 0 != cgs.clientinfo[i as usize].infoValid as libc::c_uint
            && cgs.clientinfo[i as usize].team as libc::c_uint
                == TEAM_SPECTATOR as libc::c_int as libc::c_uint
        {
            Q_strcat(
                cg.spectatorList.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
                va(
                    b"%s     \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    cgs.clientinfo[i as usize].name.as_mut_ptr(),
                ),
            );
        }
        i += 1
    }
    i = strlen(cg.spectatorList.as_mut_ptr()) as libc::c_int;
    if i != cg.spectatorLen {
        cg.spectatorLen = i;
        cg.spectatorWidth = -1i32 as libc::c_float
    };
}
#[no_mangle]
pub unsafe extern "C" fn CG_Init(
    mut serverMessageNum: libc::c_int,
    mut serverCommandSequence: libc::c_int,
    mut clientNum: libc::c_int,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    memset(
        &mut cgs as *mut cgs_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cgs_t>() as libc::c_ulong,
    );
    memset(
        &mut cg as *mut cg_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cg_t>() as libc::c_ulong,
    );
    memset(
        cg_entities.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[centity_t; 1024]>() as libc::c_ulong,
    );
    memset(
        cg_weapons.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[weaponInfo_t; 16]>() as libc::c_ulong,
    );
    memset(
        cg_items.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[itemInfo_t; 256]>() as libc::c_ulong,
    );
    cg.clientNum = clientNum;
    cgs.processedSnapshotNum = serverMessageNum;
    cgs.serverCommandSequence = serverCommandSequence;
    cgs.media.charsetShader =
        trap_R_RegisterShader(b"gfx/2d/bigchars\x00" as *const u8 as *const libc::c_char);
    cgs.media.whiteShader = trap_R_RegisterShader(b"white\x00" as *const u8 as *const libc::c_char);
    cgs.media.charsetProp = trap_R_RegisterShaderNoMip(
        b"menu/art/font1_prop.tga\x00" as *const u8 as *const libc::c_char,
    );
    cgs.media.charsetPropGlow = trap_R_RegisterShaderNoMip(
        b"menu/art/font1_prop_glo.tga\x00" as *const u8 as *const libc::c_char,
    );
    cgs.media.charsetPropB = trap_R_RegisterShaderNoMip(
        b"menu/art/font2_prop.tga\x00" as *const u8 as *const libc::c_char,
    );
    CG_RegisterCvars();
    CG_InitConsoleCommands();
    cg.weaponSelect = WP_MACHINEGUN as libc::c_int;
    cgs.blueflag = -1i32;
    cgs.redflag = cgs.blueflag;
    cgs.flagStatus = -1i32;
    trap_GetGlconfig(&mut cgs.glconfig);
    cgs.screenXScale = (cgs.glconfig.vidWidth as libc::c_double / 640.0f64) as libc::c_float;
    cgs.screenYScale = (cgs.glconfig.vidHeight as libc::c_double / 480.0f64) as libc::c_float;
    trap_GetGameState(&mut cgs.gameState);
    s = CG_ConfigString(20i32);
    if 0 != strcmp(s, b"baseq3-1\x00" as *const u8 as *const libc::c_char) {
        CG_Error(
            b"Client/Server game mismatch: %s/%s\x00" as *const u8 as *const libc::c_char,
            b"baseq3-1\x00" as *const u8 as *const libc::c_char,
            s,
        );
    }
    s = CG_ConfigString(21i32);
    cgs.levelStartTime = atoi(s);
    CG_ParseServerinfo();
    CG_LoadingString(b"collision map\x00" as *const u8 as *const libc::c_char);
    trap_CM_LoadMap(cgs.mapname.as_mut_ptr());
    cg.loading = qtrue;
    CG_LoadingString(b"sounds\x00" as *const u8 as *const libc::c_char);
    CG_RegisterSounds();
    CG_LoadingString(b"graphics\x00" as *const u8 as *const libc::c_char);
    CG_RegisterGraphics();
    CG_LoadingString(b"clients\x00" as *const u8 as *const libc::c_char);
    CG_RegisterClients();
    cg.loading = qfalse;
    CG_InitLocalEntities();
    CG_InitMarkPolys();
    cg.infoScreenText[0usize] = 0i32 as libc::c_char;
    CG_SetConfigValues();
    CG_StartMusic();
    CG_LoadingString(b"\x00" as *const u8 as *const libc::c_char);
    CG_ShaderStateChanged();
    trap_S_ClearLoopingSounds(qtrue);
}
/*
===================
CG_RegisterClients
===================
*/
unsafe extern "C" fn CG_RegisterClients() {
    let mut i: libc::c_int = 0;
    CG_LoadingClient(cg.clientNum);
    CG_NewClientInfo(cg.clientNum);
    i = 0i32;
    while i < 64i32 {
        let mut clientInfo: *const libc::c_char = 0 as *const libc::c_char;
        if !(cg.clientNum == i) {
            clientInfo = CG_ConfigString(32i32 + 256i32 + 256i32 + i);
            if !(0 == *clientInfo.offset(0isize)) {
                CG_LoadingClient(i);
                CG_NewClientInfo(i);
            }
        }
        i += 1
    }
    CG_BuildSpectatorString();
}
//===================================================================================
/*
=================
CG_RegisterGraphics

This function may execute for a couple of minutes with a slow disk.
=================
*/
unsafe extern "C" fn CG_RegisterGraphics() {
    let mut i: libc::c_int = 0;
    let mut items: [libc::c_char; 257] = [0; 257];
    static mut sb_nums: [*mut libc::c_char; 11] = [
        b"gfx/2d/numbers/zero_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/one_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/two_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/three_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/four_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/five_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/six_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/seven_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/eight_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/nine_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"gfx/2d/numbers/minus_32b\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    memset(
        &mut cg.refdef as *mut refdef_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refdef_t>() as libc::c_ulong,
    );
    trap_R_ClearScene();
    CG_LoadingString(cgs.mapname.as_mut_ptr());
    trap_R_LoadWorldMap(cgs.mapname.as_mut_ptr());
    CG_LoadingString(b"game media\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < 11i32 {
        cgs.media.numberShaders[i as usize] = trap_R_RegisterShader(sb_nums[i as usize]);
        i += 1
    }
    cgs.media.botSkillShaders[0usize] =
        trap_R_RegisterShader(b"menu/art/skill1.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.botSkillShaders[1usize] =
        trap_R_RegisterShader(b"menu/art/skill2.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.botSkillShaders[2usize] =
        trap_R_RegisterShader(b"menu/art/skill3.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.botSkillShaders[3usize] =
        trap_R_RegisterShader(b"menu/art/skill4.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.botSkillShaders[4usize] =
        trap_R_RegisterShader(b"menu/art/skill5.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.viewBloodShader =
        trap_R_RegisterShader(b"viewBloodBlend\x00" as *const u8 as *const libc::c_char);
    cgs.media.deferShader =
        trap_R_RegisterShaderNoMip(b"gfx/2d/defer.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.scoreboardName =
        trap_R_RegisterShaderNoMip(b"menu/tab/name.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.scoreboardPing =
        trap_R_RegisterShaderNoMip(b"menu/tab/ping.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.scoreboardScore =
        trap_R_RegisterShaderNoMip(b"menu/tab/score.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.scoreboardTime =
        trap_R_RegisterShaderNoMip(b"menu/tab/time.tga\x00" as *const u8 as *const libc::c_char);
    cgs.media.smokePuffShader =
        trap_R_RegisterShader(b"smokePuff\x00" as *const u8 as *const libc::c_char);
    cgs.media.smokePuffRageProShader =
        trap_R_RegisterShader(b"smokePuffRagePro\x00" as *const u8 as *const libc::c_char);
    cgs.media.shotgunSmokePuffShader =
        trap_R_RegisterShader(b"shotgunSmokePuff\x00" as *const u8 as *const libc::c_char);
    cgs.media.plasmaBallShader =
        trap_R_RegisterShader(b"sprites/plasma1\x00" as *const u8 as *const libc::c_char);
    cgs.media.bloodTrailShader =
        trap_R_RegisterShader(b"bloodTrail\x00" as *const u8 as *const libc::c_char);
    cgs.media.lagometerShader =
        trap_R_RegisterShader(b"lagometer\x00" as *const u8 as *const libc::c_char);
    cgs.media.connectionShader =
        trap_R_RegisterShader(b"disconnected\x00" as *const u8 as *const libc::c_char);
    cgs.media.waterBubbleShader =
        trap_R_RegisterShader(b"waterBubble\x00" as *const u8 as *const libc::c_char);
    cgs.media.tracerShader =
        trap_R_RegisterShader(b"gfx/misc/tracer\x00" as *const u8 as *const libc::c_char);
    cgs.media.selectShader =
        trap_R_RegisterShader(b"gfx/2d/select\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < 10i32 {
        cgs.media.crosshairShader[i as usize] = trap_R_RegisterShader(va(
            b"gfx/2d/crosshair%c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            'a' as i32 + i,
        ));
        i += 1
    }
    cgs.media.backTileShader =
        trap_R_RegisterShader(b"gfx/2d/backtile\x00" as *const u8 as *const libc::c_char);
    cgs.media.noammoShader =
        trap_R_RegisterShader(b"icons/noammo\x00" as *const u8 as *const libc::c_char);
    cgs.media.quadShader =
        trap_R_RegisterShader(b"powerups/quad\x00" as *const u8 as *const libc::c_char);
    cgs.media.quadWeaponShader =
        trap_R_RegisterShader(b"powerups/quadWeapon\x00" as *const u8 as *const libc::c_char);
    cgs.media.battleSuitShader =
        trap_R_RegisterShader(b"powerups/battleSuit\x00" as *const u8 as *const libc::c_char);
    cgs.media.battleWeaponShader =
        trap_R_RegisterShader(b"powerups/battleWeapon\x00" as *const u8 as *const libc::c_char);
    cgs.media.invisShader =
        trap_R_RegisterShader(b"powerups/invisibility\x00" as *const u8 as *const libc::c_char);
    cgs.media.regenShader =
        trap_R_RegisterShader(b"powerups/regen\x00" as *const u8 as *const libc::c_char);
    cgs.media.hastePuffShader =
        trap_R_RegisterShader(b"hasteSmokePuff\x00" as *const u8 as *const libc::c_char);
    if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint
        || 0 != cg_buildScript.integer
    {
        cgs.media.redFlagModel = trap_R_RegisterModel(
            b"models/flags/r_flag.md3\x00" as *const u8 as *const libc::c_char,
        );
        cgs.media.blueFlagModel = trap_R_RegisterModel(
            b"models/flags/b_flag.md3\x00" as *const u8 as *const libc::c_char,
        );
        cgs.media.redFlagShader[0usize] =
            trap_R_RegisterShaderNoMip(b"icons/iconf_red1\x00" as *const u8 as *const libc::c_char);
        cgs.media.redFlagShader[1usize] =
            trap_R_RegisterShaderNoMip(b"icons/iconf_red2\x00" as *const u8 as *const libc::c_char);
        cgs.media.redFlagShader[2usize] =
            trap_R_RegisterShaderNoMip(b"icons/iconf_red3\x00" as *const u8 as *const libc::c_char);
        cgs.media.blueFlagShader[0usize] =
            trap_R_RegisterShaderNoMip(b"icons/iconf_blu1\x00" as *const u8 as *const libc::c_char);
        cgs.media.blueFlagShader[1usize] =
            trap_R_RegisterShaderNoMip(b"icons/iconf_blu2\x00" as *const u8 as *const libc::c_char);
        cgs.media.blueFlagShader[2usize] =
            trap_R_RegisterShaderNoMip(b"icons/iconf_blu3\x00" as *const u8 as *const libc::c_char)
    }
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
        || 0 != cg_buildScript.integer
    {
        cgs.media.friendShader =
            trap_R_RegisterShader(b"sprites/foe\x00" as *const u8 as *const libc::c_char);
        cgs.media.redQuadShader =
            trap_R_RegisterShader(b"powerups/blueflag\x00" as *const u8 as *const libc::c_char);
        cgs.media.teamStatusBar =
            trap_R_RegisterShader(b"gfx/2d/colorbar.tga\x00" as *const u8 as *const libc::c_char)
    }
    cgs.media.armorModel = trap_R_RegisterModel(
        b"models/powerups/armor/armor_yel.md3\x00" as *const u8 as *const libc::c_char,
    );
    cgs.media.armorIcon =
        trap_R_RegisterShaderNoMip(b"icons/iconr_yellow\x00" as *const u8 as *const libc::c_char);
    cgs.media.machinegunBrassModel = trap_R_RegisterModel(
        b"models/weapons2/shells/m_shell.md3\x00" as *const u8 as *const libc::c_char,
    );
    cgs.media.shotgunBrassModel = trap_R_RegisterModel(
        b"models/weapons2/shells/s_shell.md3\x00" as *const u8 as *const libc::c_char,
    );
    cgs.media.gibAbdomen =
        trap_R_RegisterModel(b"models/gibs/abdomen.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibArm =
        trap_R_RegisterModel(b"models/gibs/arm.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibChest =
        trap_R_RegisterModel(b"models/gibs/chest.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibFist =
        trap_R_RegisterModel(b"models/gibs/fist.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibFoot =
        trap_R_RegisterModel(b"models/gibs/foot.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibForearm =
        trap_R_RegisterModel(b"models/gibs/forearm.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibIntestine =
        trap_R_RegisterModel(b"models/gibs/intestine.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibLeg =
        trap_R_RegisterModel(b"models/gibs/leg.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibSkull =
        trap_R_RegisterModel(b"models/gibs/skull.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.gibBrain =
        trap_R_RegisterModel(b"models/gibs/brain.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.smoke2 = trap_R_RegisterModel(
        b"models/weapons2/shells/s_shell.md3\x00" as *const u8 as *const libc::c_char,
    );
    cgs.media.balloonShader =
        trap_R_RegisterShader(b"sprites/balloon3\x00" as *const u8 as *const libc::c_char);
    cgs.media.bloodExplosionShader =
        trap_R_RegisterShader(b"bloodExplosion\x00" as *const u8 as *const libc::c_char);
    cgs.media.bulletFlashModel =
        trap_R_RegisterModel(b"models/weaphits/bullet.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.ringFlashModel =
        trap_R_RegisterModel(b"models/weaphits/ring02.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.dishFlashModel =
        trap_R_RegisterModel(b"models/weaphits/boom01.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.teleportEffectModel =
        trap_R_RegisterModel(b"models/misc/telep.md3\x00" as *const u8 as *const libc::c_char);
    cgs.media.teleportEffectShader =
        trap_R_RegisterShader(b"teleportEffect\x00" as *const u8 as *const libc::c_char);
    cgs.media.medalImpressive =
        trap_R_RegisterShaderNoMip(b"medal_impressive\x00" as *const u8 as *const libc::c_char);
    cgs.media.medalExcellent =
        trap_R_RegisterShaderNoMip(b"medal_excellent\x00" as *const u8 as *const libc::c_char);
    cgs.media.medalGauntlet =
        trap_R_RegisterShaderNoMip(b"medal_gauntlet\x00" as *const u8 as *const libc::c_char);
    cgs.media.medalDefend =
        trap_R_RegisterShaderNoMip(b"medal_defend\x00" as *const u8 as *const libc::c_char);
    cgs.media.medalAssist =
        trap_R_RegisterShaderNoMip(b"medal_assist\x00" as *const u8 as *const libc::c_char);
    cgs.media.medalCapture =
        trap_R_RegisterShaderNoMip(b"medal_capture\x00" as *const u8 as *const libc::c_char);
    memset(
        cg_items.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[itemInfo_t; 256]>() as libc::c_ulong,
    );
    memset(
        cg_weapons.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[weaponInfo_t; 16]>() as libc::c_ulong,
    );
    Q_strncpyz(
        items.as_mut_ptr(),
        CG_ConfigString(27i32),
        ::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong as libc::c_int,
    );
    i = 1i32;
    while i < bg_numItems {
        if items[i as usize] as libc::c_int == '1' as i32 || 0 != cg_buildScript.integer {
            CG_LoadingItem(i);
            CG_RegisterItemVisuals(i);
        }
        i += 1
    }
    cgs.media.bulletMarkShader =
        trap_R_RegisterShader(b"gfx/damage/bullet_mrk\x00" as *const u8 as *const libc::c_char);
    cgs.media.burnMarkShader =
        trap_R_RegisterShader(b"gfx/damage/burn_med_mrk\x00" as *const u8 as *const libc::c_char);
    cgs.media.holeMarkShader =
        trap_R_RegisterShader(b"gfx/damage/hole_lg_mrk\x00" as *const u8 as *const libc::c_char);
    cgs.media.energyMarkShader =
        trap_R_RegisterShader(b"gfx/damage/plasma_mrk\x00" as *const u8 as *const libc::c_char);
    cgs.media.shadowMarkShader =
        trap_R_RegisterShader(b"markShadow\x00" as *const u8 as *const libc::c_char);
    cgs.media.wakeMarkShader =
        trap_R_RegisterShader(b"wake\x00" as *const u8 as *const libc::c_char);
    cgs.media.bloodMarkShader =
        trap_R_RegisterShader(b"bloodMark\x00" as *const u8 as *const libc::c_char);
    cgs.numInlineModels = trap_CM_NumInlineModels();
    i = 1i32;
    while i < cgs.numInlineModels {
        let mut name: [libc::c_char; 10] = [0; 10];
        let mut mins: vec3_t = [0.; 3];
        let mut maxs: vec3_t = [0.; 3];
        let mut j: libc::c_int = 0;
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as libc::c_int,
            b"*%i\x00" as *const u8 as *const libc::c_char,
            i,
        );
        cgs.inlineDrawModel[i as usize] = trap_R_RegisterModel(name.as_mut_ptr());
        trap_R_ModelBounds(
            cgs.inlineDrawModel[i as usize],
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        j = 0i32;
        while j < 3i32 {
            cgs.inlineModelMidpoints[i as usize][j as usize] = (mins[j as usize] as libc::c_double
                + 0.5f64 * (maxs[j as usize] - mins[j as usize]) as libc::c_double)
                as vec_t;
            j += 1
        }
        i += 1
    }
    i = 1i32;
    while i < 256i32 {
        let mut modelName: *const libc::c_char = 0 as *const libc::c_char;
        modelName = CG_ConfigString(32i32 + i);
        if 0 == *modelName.offset(0isize) {
            break;
        }
        cgs.gameModels[i as usize] = trap_R_RegisterModel(modelName);
        i += 1
    }
    CG_ClearParticles();
}
/*
=================
CG_RegisterSounds

called during a precache command
=================
*/
unsafe extern "C" fn CG_RegisterSounds() {
    let mut i: libc::c_int = 0;
    let mut items: [libc::c_char; 257] = [0; 257];
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut soundName: *const libc::c_char = 0 as *const libc::c_char;
    cgs.media.oneMinuteSound = trap_S_RegisterSound(
        b"sound/feedback/1_minute.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.fiveMinuteSound = trap_S_RegisterSound(
        b"sound/feedback/5_minute.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.suddenDeathSound = trap_S_RegisterSound(
        b"sound/feedback/sudden_death.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.oneFragSound = trap_S_RegisterSound(
        b"sound/feedback/1_frag.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.twoFragSound = trap_S_RegisterSound(
        b"sound/feedback/2_frags.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.threeFragSound = trap_S_RegisterSound(
        b"sound/feedback/3_frags.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.count3Sound = trap_S_RegisterSound(
        b"sound/feedback/three.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.count2Sound = trap_S_RegisterSound(
        b"sound/feedback/two.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.count1Sound = trap_S_RegisterSound(
        b"sound/feedback/one.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.countFightSound = trap_S_RegisterSound(
        b"sound/feedback/fight.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.countPrepareSound = trap_S_RegisterSound(
        b"sound/feedback/prepare.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    if cgs.gametype as libc::c_uint >= GT_TEAM as libc::c_int as libc::c_uint
        || 0 != cg_buildScript.integer
    {
        cgs.media.captureAwardSound = trap_S_RegisterSound(
            b"sound/teamplay/flagcapture_yourteam.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.redLeadsSound = trap_S_RegisterSound(
            b"sound/feedback/redleads.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.blueLeadsSound = trap_S_RegisterSound(
            b"sound/feedback/blueleads.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.teamsTiedSound = trap_S_RegisterSound(
            b"sound/feedback/teamstied.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.hitTeamSound = trap_S_RegisterSound(
            b"sound/feedback/hit_teammate.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.redScoredSound = trap_S_RegisterSound(
            b"sound/teamplay/voc_red_scores.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.blueScoredSound = trap_S_RegisterSound(
            b"sound/teamplay/voc_blue_scores.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.captureYourTeamSound = trap_S_RegisterSound(
            b"sound/teamplay/flagcapture_yourteam.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.captureOpponentSound = trap_S_RegisterSound(
            b"sound/teamplay/flagcapture_opponent.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.returnYourTeamSound = trap_S_RegisterSound(
            b"sound/teamplay/flagreturn_yourteam.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.returnOpponentSound = trap_S_RegisterSound(
            b"sound/teamplay/flagreturn_opponent.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.takenYourTeamSound = trap_S_RegisterSound(
            b"sound/teamplay/flagtaken_yourteam.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.takenOpponentSound = trap_S_RegisterSound(
            b"sound/teamplay/flagtaken_opponent.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        if cgs.gametype as libc::c_uint == GT_CTF as libc::c_int as libc::c_uint
            || 0 != cg_buildScript.integer
        {
            cgs.media.redFlagReturnedSound = trap_S_RegisterSound(
                b"sound/teamplay/voc_red_returned.wav\x00" as *const u8 as *const libc::c_char,
                qtrue,
            );
            cgs.media.blueFlagReturnedSound = trap_S_RegisterSound(
                b"sound/teamplay/voc_blue_returned.wav\x00" as *const u8 as *const libc::c_char,
                qtrue,
            );
            cgs.media.enemyTookYourFlagSound = trap_S_RegisterSound(
                b"sound/teamplay/voc_enemy_flag.wav\x00" as *const u8 as *const libc::c_char,
                qtrue,
            );
            cgs.media.yourTeamTookEnemyFlagSound = trap_S_RegisterSound(
                b"sound/teamplay/voc_team_flag.wav\x00" as *const u8 as *const libc::c_char,
                qtrue,
            )
        }
        cgs.media.youHaveFlagSound = trap_S_RegisterSound(
            b"sound/teamplay/voc_you_flag.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        );
        cgs.media.holyShitSound = trap_S_RegisterSound(
            b"sound/feedback/voc_holyshit.wav\x00" as *const u8 as *const libc::c_char,
            qtrue,
        )
    }
    cgs.media.tracerSound = trap_S_RegisterSound(
        b"sound/weapons/machinegun/buletby1.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.selectSound = trap_S_RegisterSound(
        b"sound/weapons/change.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.wearOffSound = trap_S_RegisterSound(
        b"sound/items/wearoff.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.useNothingSound = trap_S_RegisterSound(
        b"sound/items/use_nothing.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.gibSound = trap_S_RegisterSound(
        b"sound/player/gibsplt1.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.gibBounce1Sound = trap_S_RegisterSound(
        b"sound/player/gibimp1.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.gibBounce2Sound = trap_S_RegisterSound(
        b"sound/player/gibimp2.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.gibBounce3Sound = trap_S_RegisterSound(
        b"sound/player/gibimp3.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.teleInSound = trap_S_RegisterSound(
        b"sound/world/telein.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.teleOutSound = trap_S_RegisterSound(
        b"sound/world/teleout.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.respawnSound = trap_S_RegisterSound(
        b"sound/items/respawn1.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.noAmmoSound = trap_S_RegisterSound(
        b"sound/weapons/noammo.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.talkSound = trap_S_RegisterSound(
        b"sound/player/talk.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.landSound = trap_S_RegisterSound(
        b"sound/player/land1.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.hitSound = trap_S_RegisterSound(
        b"sound/feedback/hit.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.impressiveSound = trap_S_RegisterSound(
        b"sound/feedback/impressive.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.excellentSound = trap_S_RegisterSound(
        b"sound/feedback/excellent.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.deniedSound = trap_S_RegisterSound(
        b"sound/feedback/denied.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.humiliationSound = trap_S_RegisterSound(
        b"sound/feedback/humiliation.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.assistSound = trap_S_RegisterSound(
        b"sound/feedback/assist.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.defendSound = trap_S_RegisterSound(
        b"sound/feedback/defense.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.takenLeadSound = trap_S_RegisterSound(
        b"sound/feedback/takenlead.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.tiedLeadSound = trap_S_RegisterSound(
        b"sound/feedback/tiedlead.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.lostLeadSound = trap_S_RegisterSound(
        b"sound/feedback/lostlead.wav\x00" as *const u8 as *const libc::c_char,
        qtrue,
    );
    cgs.media.watrInSound = trap_S_RegisterSound(
        b"sound/player/watr_in.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.watrOutSound = trap_S_RegisterSound(
        b"sound/player/watr_out.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.watrUnSound = trap_S_RegisterSound(
        b"sound/player/watr_un.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.jumpPadSound = trap_S_RegisterSound(
        b"sound/world/jumppad.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    i = 0i32;
    while i < 4i32 {
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/player/footsteps/step%i.wav\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
        );
        cgs.media.footsteps[FOOTSTEP_NORMAL as libc::c_int as usize][i as usize] =
            trap_S_RegisterSound(name.as_mut_ptr(), qfalse);
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/player/footsteps/boot%i.wav\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
        );
        cgs.media.footsteps[FOOTSTEP_BOOT as libc::c_int as usize][i as usize] =
            trap_S_RegisterSound(name.as_mut_ptr(), qfalse);
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/player/footsteps/flesh%i.wav\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
        );
        cgs.media.footsteps[FOOTSTEP_FLESH as libc::c_int as usize][i as usize] =
            trap_S_RegisterSound(name.as_mut_ptr(), qfalse);
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/player/footsteps/mech%i.wav\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
        );
        cgs.media.footsteps[FOOTSTEP_MECH as libc::c_int as usize][i as usize] =
            trap_S_RegisterSound(name.as_mut_ptr(), qfalse);
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/player/footsteps/energy%i.wav\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
        );
        cgs.media.footsteps[FOOTSTEP_ENERGY as libc::c_int as usize][i as usize] =
            trap_S_RegisterSound(name.as_mut_ptr(), qfalse);
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/player/footsteps/splash%i.wav\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
        );
        cgs.media.footsteps[FOOTSTEP_SPLASH as libc::c_int as usize][i as usize] =
            trap_S_RegisterSound(name.as_mut_ptr(), qfalse);
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/player/footsteps/clank%i.wav\x00" as *const u8 as *const libc::c_char,
            i + 1i32,
        );
        cgs.media.footsteps[FOOTSTEP_METAL as libc::c_int as usize][i as usize] =
            trap_S_RegisterSound(name.as_mut_ptr(), qfalse);
        i += 1
    }
    Q_strncpyz(
        items.as_mut_ptr(),
        CG_ConfigString(27i32),
        ::std::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong as libc::c_int,
    );
    i = 1i32;
    while i < bg_numItems {
        CG_RegisterItemSounds(i);
        i += 1
    }
    i = 1i32;
    while i < 256i32 {
        soundName = CG_ConfigString(32i32 + 256i32 + i);
        if 0 == *soundName.offset(0isize) {
            break;
        }
        if !(*soundName.offset(0isize) as libc::c_int == '*' as i32) {
            // custom sound
            cgs.gameSounds[i as usize] = trap_S_RegisterSound(soundName, qfalse)
        }
        i += 1
    }
    cgs.media.flightSound = trap_S_RegisterSound(
        b"sound/items/flight.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.medkitSound = trap_S_RegisterSound(
        b"sound/items/use_medkit.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.quadSound = trap_S_RegisterSound(
        b"sound/items/damage3.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.sfx_ric1 = trap_S_RegisterSound(
        b"sound/weapons/machinegun/ric1.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.sfx_ric2 = trap_S_RegisterSound(
        b"sound/weapons/machinegun/ric2.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.sfx_ric3 = trap_S_RegisterSound(
        b"sound/weapons/machinegun/ric3.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.sfx_rockexp = trap_S_RegisterSound(
        b"sound/weapons/rocket/rocklx1a.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.sfx_plasmaexp = trap_S_RegisterSound(
        b"sound/weapons/plasma/plasmx1a.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.regenSound = trap_S_RegisterSound(
        b"sound/items/regen.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.protectSound = trap_S_RegisterSound(
        b"sound/items/protect3.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.n_healthSound = trap_S_RegisterSound(
        b"sound/items/n_health.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.hgrenb1aSound = trap_S_RegisterSound(
        b"sound/weapons/grenade/hgrenb1a.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    cgs.media.hgrenb2aSound = trap_S_RegisterSound(
        b"sound/weapons/grenade/hgrenb2a.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
}
//========================================================================
/*
=================
CG_RegisterItemSounds

The server says this item is used on this level
=================
*/
unsafe extern "C" fn CG_RegisterItemSounds(mut itemNum: libc::c_int) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut data: [libc::c_char; 64] = [0; 64];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    item = &mut *bg_itemlist.as_mut_ptr().offset(itemNum as isize) as *mut gitem_t;
    if !(*item).pickup_sound.is_null() {
        trap_S_RegisterSound((*item).pickup_sound, qfalse);
    }
    s = (*item).sounds;
    if s.is_null() || 0 == *s.offset(0isize) {
        return;
    }
    while 0 != *s {
        start = s;
        while 0 != *s as libc::c_int && *s as libc::c_int != ' ' as i32 {
            s = s.offset(1isize)
        }
        len = s.wrapping_offset_from(start) as libc::c_long as libc::c_int;
        if len >= 64i32 || len < 5i32 {
            CG_Error(
                b"PrecacheItem: %s has bad precache string\x00" as *const u8 as *const libc::c_char,
                (*item).classname,
            );
        }
        memcpy(
            data.as_mut_ptr() as *mut libc::c_void,
            start as *const libc::c_void,
            len as libc::c_ulong,
        );
        data[len as usize] = 0i32 as libc::c_char;
        if 0 != *s {
            s = s.offset(1isize)
        }
        if 0 == strcmp(
            data.as_mut_ptr().offset(len as isize).offset(-3isize),
            b"wav\x00" as *const u8 as *const libc::c_char,
        ) {
            trap_S_RegisterSound(data.as_mut_ptr(), qfalse);
        }
    }
}
/*
=================
CG_RegisterCvars
=================
*/
#[no_mangle]
pub unsafe extern "C" fn CG_RegisterCvars() {
    let mut i: libc::c_int = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    let mut var: [libc::c_char; 1024] = [0; 1024];
    i = 0i32;
    cv = cvarTable.as_mut_ptr();
    while i < cvarTableSize {
        trap_Cvar_Register(
            (*cv).vmCvar,
            (*cv).cvarName,
            (*cv).defaultString,
            (*cv).cvarFlags,
        );
        i += 1;
        cv = cv.offset(1isize)
    }
    trap_Cvar_VariableStringBuffer(
        b"sv_running\x00" as *const u8 as *const libc::c_char,
        var.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    cgs.localServer = atoi(var.as_mut_ptr()) as qboolean;
    forceModelModificationCount = cg_forceModel.modificationCount;
    trap_Cvar_Register(
        0 as *mut vmCvar_t,
        b"model\x00" as *const u8 as *const libc::c_char,
        b"sarge\x00" as *const u8 as *const libc::c_char,
        0x2i32 | 0x1i32,
    );
    trap_Cvar_Register(
        0 as *mut vmCvar_t,
        b"headmodel\x00" as *const u8 as *const libc::c_char,
        b"sarge\x00" as *const u8 as *const libc::c_char,
        0x2i32 | 0x1i32,
    );
    trap_Cvar_Register(
        0 as *mut vmCvar_t,
        b"team_model\x00" as *const u8 as *const libc::c_char,
        b"sarge\x00" as *const u8 as *const libc::c_char,
        0x2i32 | 0x1i32,
    );
    trap_Cvar_Register(
        0 as *mut vmCvar_t,
        b"team_headmodel\x00" as *const u8 as *const libc::c_char,
        b"sarge\x00" as *const u8 as *const libc::c_char,
        0x2i32 | 0x1i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CG_Shutdown() {}
/*
================
vmMain

This is the only way control passes into the module.
This must be the very first function compiled into the .q3vm file
================
*/
#[no_mangle]
pub unsafe extern "C" fn vmMain(
    mut command: libc::c_int,
    mut arg0: libc::c_int,
    mut arg1: libc::c_int,
    mut arg2: libc::c_int,
    mut arg3: libc::c_int,
    mut arg4: libc::c_int,
    mut arg5: libc::c_int,
    mut arg6: libc::c_int,
    mut arg7: libc::c_int,
    mut arg8: libc::c_int,
    mut arg9: libc::c_int,
    mut arg10: libc::c_int,
    mut arg11: libc::c_int,
) -> intptr_t {
    match command {
        0 => {
            CG_Init(arg0, arg1, arg2);
            return 0i32 as intptr_t;
        }
        1 => {
            CG_Shutdown();
            return 0i32 as intptr_t;
        }
        2 => return CG_ConsoleCommand() as intptr_t,
        3 => {
            CG_DrawActiveFrame(arg0, arg1 as stereoFrame_t, arg2 as qboolean);
            return 0i32 as intptr_t;
        }
        4 => return CG_CrosshairPlayer() as intptr_t,
        5 => return CG_LastAttacker() as intptr_t,
        6 => {
            CG_KeyEvent(arg0, arg1 as qboolean);
            return 0i32 as intptr_t;
        }
        7 => {
            CG_MouseEvent(arg0, arg1);
            return 0i32 as intptr_t;
        }
        8 => {
            CG_EventHandling(arg0);
            return 0i32 as intptr_t;
        }
        _ => {
            CG_Error(
                b"vmMain: unknown command %i\x00" as *const u8 as *const libc::c_char,
                command,
            );
        }
    };
}
#[no_mangle]
pub static mut cg_hudFiles: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
//vmCvar_t	cg_pmove_fixed;
#[no_mangle]
pub static mut cg_pmove_msec: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
unsafe extern "C" fn run_static_initializers() {
    cvarTableSize = (::std::mem::size_of::<[cvarTable_t; 83]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cvarTable_t>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvarTable_t {
    pub vmCvar: *mut vmCvar_t,
    pub cvarName: *mut libc::c_char,
    pub defaultString: *mut libc::c_char,
    pub cvarFlags: libc::c_int,
}
